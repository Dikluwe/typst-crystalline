//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/shell/watch.md
//! @prompt-hash 019b231f
//! @layer L3
//! @updated 2026-08-23

//! Filesystem observation and atomic artifact replacement for `typst watch`.

use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex, atomic::{AtomicBool, AtomicU64, Ordering}, mpsc};

use crate::pipeline::BundleFile;
use std::fs;
use std::hash::{Hash, Hasher};
use std::io;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::{Duration, SystemTime};

#[derive(Clone, Debug, PartialEq, Eq)]
struct Fingerprint {
    modified: Option<SystemTime>,
    len: u64,
    content_hash: u64,
}

fn fingerprint(path: &Path) -> Option<Fingerprint> {
    let metadata = fs::metadata(path).ok()?;
    let bytes = fs::read(path).ok()?;
    let mut hasher = DefaultHasher::new();
    bytes.hash(&mut hasher);
    Some(Fingerprint {
        modified: metadata.modified().ok(),
        len: metadata.len(),
        content_hash: hasher.finish(),
    })
}

/// Opaque baseline for a set of observed filesystem paths.
pub struct WatchSnapshot {
    entries: Vec<(PathBuf, Option<Fingerprint>)>,
}

/// Capability for finalizing a watch cycle after its baseline was captured.
pub struct ArmedWatch {
    snapshot: WatchSnapshot,
}

/// Captures the current state of each observed path exactly once.
pub fn snapshot(paths: &[PathBuf]) -> WatchSnapshot {
    WatchSnapshot {
        entries: paths.iter().map(|path| (path.clone(), fingerprint(path))).collect(),
    }
}

/// Captures the observed paths immediately and arms cycle finalization.
pub fn arm(paths: &[PathBuf]) -> ArmedWatch {
    ArmedWatch { snapshot: snapshot(paths) }
}

impl ArmedWatch {
    /// Atomically publishes staging and returns the already captured baseline.
    pub fn publish(
        self,
        staging: &Path,
        destination: &Path,
    ) -> io::Result<WatchSnapshot> {
        match fs::rename(staging, destination) {
            Ok(()) => Ok(self.snapshot),
            Err(error) => {
                let _ = fs::remove_file(staging);
                Err(error)
            }
        }
    }

    /// Publishes a complete directory after preserving the previous valid tree.
    pub fn publish_directory(
        self,
        staging: &Path,
        destination: &Path,
    ) -> io::Result<WatchSnapshot> {
        static NEXT_BACKUP: AtomicU64 = AtomicU64::new(0);
        let backup = destination.with_file_name(format!(
            ".{}.crystalline-backup-{}-{}",
            destination.file_name().and_then(|name| name.to_str()).unwrap_or("bundle"),
            std::process::id(),
            NEXT_BACKUP.fetch_add(1, Ordering::Relaxed),
        ));
        let had_destination = destination.exists();
        if had_destination {
            fs::rename(destination, &backup)?;
        }
        match fs::rename(staging, destination) {
            Ok(()) => {
                if had_destination {
                    let _ = fs::remove_dir_all(&backup);
                }
                Ok(self.snapshot)
            }
            Err(error) => {
                if had_destination {
                    let _ = fs::rename(&backup, destination);
                }
                let _ = fs::remove_dir_all(staging);
                Err(error)
            }
        }
    }

    /// Discards only the uncommitted bundle tree.
    pub fn abandon_directory(self, staging: &Path) -> WatchSnapshot {
        let _ = fs::remove_dir_all(staging);
        self.snapshot
    }

    /// Abandons staging best-effort and returns the already captured baseline.
    pub fn abandon(self, staging: &Path) -> WatchSnapshot {
        let _ = fs::remove_file(staging);
        self.snapshot
    }
}

/// Blocks until one of the snapshotted paths changes, appears, or disappears.
pub fn wait_for_change_since(snapshot: WatchSnapshot, interval: Duration) {
    loop {
        thread::sleep(interval);
        if snapshot
            .entries
            .iter()
            .any(|(path, previous)| fingerprint(path) != *previous)
        {
            return;
        }
    }
}

/// Blocks until one of the observed paths changes, appears, or disappears.
pub fn wait_for_change(paths: &[PathBuf], interval: Duration) {
    wait_for_change_since(snapshot(paths), interval);
}

struct ServedFile {
    data: Vec<u8>,
    mime: &'static str,
    html: bool,
}

#[derive(Default)]
struct ServerState {
    files: HashMap<String, ServedFile>,
    subscribers: Vec<mpsc::Sender<()>>,
}

/// Loopback-only HTTP snapshot for watch HTML and bundle output.
pub struct BundleHttpServer {
    addr: SocketAddr,
    state: Arc<Mutex<ServerState>>,
    stop: Arc<AtomicBool>,
    thread: Option<thread::JoinHandle<()>>,
    live: bool,
}

impl BundleHttpServer {
    pub fn new(_title: &str, port: Option<u16>, live: bool) -> io::Result<Self> {
        let listener = match port {
            Some(port) => TcpListener::bind(("127.0.0.1", port))?,
            None => {
                let mut selected = None;
                for port in 3000..=3005 {
                    if let Ok(listener) = TcpListener::bind(("127.0.0.1", port)) {
                        selected = Some(listener);
                        break;
                    }
                }
                selected.ok_or_else(|| io::Error::new(
                    io::ErrorKind::AddrInUse,
                    "no available local port in 3000..=3005",
                ))?
            }
        };
        let addr = listener.local_addr()?;
        listener.set_nonblocking(true)?;
        let state = Arc::new(Mutex::new(ServerState::default()));
        let stop = Arc::new(AtomicBool::new(false));
        let state_for_thread = state.clone();
        let stop_for_thread = stop.clone();
        let thread = thread::spawn(move || {
            while !stop_for_thread.load(Ordering::Relaxed) {
                match listener.accept() {
                    Ok((stream, _)) => {
                        let state = state_for_thread.clone();
                        let stop = stop_for_thread.clone();
                        thread::spawn(move || {
                            let _ = serve_connection(stream, state, stop, live);
                        });
                    }
                    Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
                        thread::sleep(Duration::from_millis(20));
                    }
                    Err(_) => break,
                }
            }
        });
        Ok(Self { addr, state, stop, thread: Some(thread), live })
    }

    pub fn addr(&self) -> SocketAddr {
        self.addr
    }

    pub fn publish(&self, files: &[BundleFile]) {
        let mut state = self.state.lock().expect("bundle server state");
        state.files.clear();
        for file in files {
            state.files.insert(format!("/{}", file.path.trim_start_matches('/')), ServedFile {
                data: file.data.clone(),
                mime: mime_for_path(&file.path),
                html: file.is_html_document,
            });
        }
        state.subscribers.retain(|sender| sender.send(()).is_ok());
    }

    pub fn publish_html(&self, html: &str) {
        let mut state = self.state.lock().expect("bundle server state");
        state.files.clear();
        state.files.insert("/index.html".to_owned(), ServedFile {
            data: html.as_bytes().to_vec(),
            mime: "text/html; charset=utf-8",
            html: true,
        });
        state.subscribers.retain(|sender| sender.send(()).is_ok());
    }
}

impl Drop for BundleHttpServer {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

fn mime_for_path(path: &str) -> &'static str {
    match path.rsplit('.').next().unwrap_or("") {
        "html" | "htm" => "text/html; charset=utf-8",
        "css" => "text/css",
        "js" => "text/javascript",
        "svg" => "image/svg+xml",
        "pdf" => "application/pdf",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "json" => "application/json",
        "txt" => "text/plain; charset=utf-8",
        _ => "application/octet-stream",
    }
}

fn decode_url_path(raw: &str) -> Option<String> {
    let raw = raw.split('?').next()?;
    let bytes = raw.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%' {
            let hi = (bytes.get(index + 1)? as &u8).to_ascii_lowercase();
            let lo = (bytes.get(index + 2)? as &u8).to_ascii_lowercase();
            let digit = |value: u8| match value {
                b'0'..=b'9' => Some(value - b'0'),
                b'a'..=b'f' => Some(value - b'a' + 10),
                _ => None,
            };
            decoded.push(digit(hi)? * 16 + digit(lo)?);
            index += 3;
        } else {
            decoded.push(bytes[index]);
            index += 1;
        }
    }
    let decoded = String::from_utf8(decoded).ok()?;
    if !decoded.starts_with('/') || decoded.contains('\\') || decoded.contains('\0') {
        return None;
    }
    let mut parts = Vec::new();
    for part in decoded.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                parts.pop()?;
            }
            part => parts.push(part),
        }
    }
    let mut path = format!("/{}", parts.join("/"));
    if decoded.ends_with('/') && path != "/" {
        path.push('/');
    }
    Some(path)
}

fn serve_connection(
    mut stream: TcpStream,
    state: Arc<Mutex<ServerState>>,
    stop: Arc<AtomicBool>,
    live: bool,
) -> io::Result<()> {
    stream.set_read_timeout(Some(Duration::from_secs(5)))?;
    let mut request = [0u8; 8192];
    let size = stream.read(&mut request)?;
    let line = String::from_utf8_lossy(&request[..size]);
    let raw_path = line.split_whitespace().nth(1).unwrap_or("/");
    let Some(path) = decode_url_path(raw_path) else {
        return respond(&mut stream, "404 Not Found", "text/plain", b"Not Found", None);
    };
    if path == "/__events" {
        stream.write_all(b"HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nCache-Control: no-cache\r\nConnection: keep-alive\r\n\r\n: connected\n\n")?;
        let (sender, receiver) = mpsc::channel();
        state.lock().expect("bundle server state").subscribers.push(sender);
        while !stop.load(Ordering::Relaxed) {
            match receiver.recv_timeout(Duration::from_secs(1)) {
                Ok(()) => stream.write_all(b"event: reload\ndata: now\n\n")?,
                Err(mpsc::RecvTimeoutError::Timeout) => {}
                Err(mpsc::RecvTimeoutError::Disconnected) => break,
            }
        }
        return Ok(());
    }
    let state = state.lock().expect("bundle server state");
    let lookup = if path == "/" {
        "/index.html".to_owned()
    } else if path.ends_with('/') {
        format!("{}index.html", path)
    } else {
        path.clone()
    };
    if let Some(file) = state.files.get(&lookup) {
        let data = if live && file.html {
            inject_reload(&file.data)
        } else {
            file.data.clone()
        };
        return respond(&mut stream, "200 OK", file.mime, &data, None);
    }
    if !path.ends_with('/') && state.files.contains_key(&format!("{path}/index.html")) {
        return respond(&mut stream, "301 Moved Permanently", "text/plain", b"", Some(&format!("{path}/")));
    }
    respond(&mut stream, "404 Not Found", "text/plain", b"Not Found", None)
}

fn inject_reload(html: &[u8]) -> Vec<u8> {
    const SCRIPT: &str = r#"<script>new EventSource("/__events").addEventListener("reload",()=>location.reload())</script>"#;
    let text = String::from_utf8_lossy(html);
    let insertion = text.find("</body>").unwrap_or(text.len());
    let mut output = Vec::with_capacity(html.len() + SCRIPT.len());
    output.extend_from_slice(&text.as_bytes()[..insertion]);
    output.extend_from_slice(SCRIPT.as_bytes());
    output.extend_from_slice(&text.as_bytes()[insertion..]);
    output
}

fn respond(
    stream: &mut TcpStream,
    status: &str,
    mime: &str,
    body: &[u8],
    location: Option<&str>,
) -> io::Result<()> {
    write!(stream, "HTTP/1.1 {status}\r\nContent-Type: {mime}\r\nContent-Length: {}\r\n", body.len())?;
    if let Some(location) = location {
        write!(stream, "Location: {location}\r\n")?;
    }
    stream.write_all(b"Connection: close\r\n\r\n")?;
    stream.write_all(body)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_content_change_even_when_length_is_stable() {
        let path = std::env::temp_dir()
            .join(format!("crystalline-watch-fingerprint-{}", std::process::id()));
        fs::write(&path, "one").unwrap();
        let before = fingerprint(&path);
        fs::write(&path, "two").unwrap();
        assert_ne!(before, fingerprint(&path));
        let _ = fs::remove_file(path);
    }
}
