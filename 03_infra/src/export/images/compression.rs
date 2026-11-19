//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/images/compression.md
//! @prompt-hash 75c2002f
//! @layer L3
//! @updated 2026-11-12

use flate2::{write::ZlibEncoder, Compression};
use std::collections::VecDeque;
use std::io::Write;
use std::sync::{Mutex, OnceLock};

const MAX_ENTRIES: usize = 64;
const MAX_BYTES: usize = 2 * 1024 * 1024;
struct Entry {
    input: Box<[u8]>,
    output: Box<[u8]>,
}
#[derive(Default)]
struct Cache {
    entries: VecDeque<Entry>,
    bytes: usize,
}
impl Cache {
    fn get(&mut self, data: &[u8]) -> Option<Vec<u8>> {
        let index = self.entries.iter().position(|entry| entry.input.as_ref() == data)?;
        let entry = self.entries.remove(index)?;
        let output = entry.output.to_vec();
        self.entries.push_back(entry);
        Some(output)
    }
    fn insert(&mut self, data: &[u8], output: &[u8]) {
        let Some(bytes) = data.len().checked_add(output.len()) else { return };
        if bytes > MAX_BYTES
            || self.entries.iter().any(|entry| entry.input.as_ref() == data)
        {
            return;
        }
        while self.entries.len() >= MAX_ENTRIES || self.bytes > MAX_BYTES - bytes {
            let Some(entry) = self.entries.pop_front() else { return };
            self.bytes -= entry.input.len() + entry.output.len();
        }
        self.entries.push_back(Entry {
            input: data.to_vec().into_boxed_slice(),
            output: output.to_vec().into_boxed_slice(),
        });
        self.bytes += bytes;
    }
}
static CACHE: OnceLock<Mutex<Cache>> = OnceLock::new();

fn fresh(data: &[u8]) -> Result<Vec<u8>, String> {
    let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
    enc.write_all(data).map_err(|e| e.to_string())?;
    enc.finish().map_err(|e| e.to_string())
}
fn cached(
    cache: &Mutex<Cache>,
    data: &[u8],
    compute: impl FnOnce(&[u8]) -> Result<Vec<u8>, String>,
) -> Result<Vec<u8>, String> {
    if data.len() > MAX_BYTES {
        return compute(data);
    }
    if let Ok(mut retained) = cache.lock() {
        if let Some(output) = retained.get(data) {
            return Ok(output);
        }
    }
    let output = compute(data)?;
    if let Ok(mut retained) = cache.lock() {
        retained.insert(data, &output);
    }
    Ok(output)
}
pub(super) fn compress(data: &[u8]) -> Result<Vec<u8>, String> {
    cached(CACHE.get_or_init(|| Mutex::new(Cache::default())), data, fresh)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::io::Read;
    fn decoded(data: &[u8]) -> Vec<u8> {
        let mut out = Vec::new();
        flate2::read::ZlibDecoder::new(data).read_to_end(&mut out).unwrap();
        out
    }
    fn observed(cache: &Mutex<Cache>, data: &[u8], calls: &Cell<usize>) -> Vec<u8> {
        cached(cache, data, |data| {
            calls.set(calls.get() + 1);
            fresh(data)
        })
        .unwrap()
    }
    #[test]
    fn compression_reuse_exact_inputs_abba_and_empty() {
        let cache = Mutex::new(Cache::default());
        let calls = Cell::new(0);
        for data in [
            &b"alpha"[..],
            &b"alphb"[..],
            &b"alphb"[..],
            &b"alpha"[..],
            &b""[..],
            &b""[..],
        ] {
            let out = observed(&cache, data, &calls);
            assert_eq!(decoded(&out), data);
        }
        assert_eq!(calls.get(), 3, "only three distinct immutable inputs");
    }
    #[test]
    fn compression_reuse_is_owned_on_input_and_output() {
        let cache = Mutex::new(Cache::default());
        let calls = Cell::new(0);
        let mut input = b"alpha".to_vec();
        let mut out = observed(&cache, &input, &calls);
        input[0] = b'z';
        out.fill(0);
        assert_eq!(decoded(&observed(&cache, b"alpha", &calls)), b"alpha");
        assert_eq!(decoded(&observed(&cache, &input, &calls)), b"zlpha");
        assert_eq!(calls.get(), 2);
    }
    #[test]
    fn compression_reuse_failure_is_not_retained() {
        let cache = Mutex::new(Cache::default());
        let calls = Cell::new(0);
        assert_eq!(cached(&cache, b"x", |_| Err("retry".into())), Err("retry".into()));
        for _ in 0..2 {
            assert_eq!(decoded(&observed(&cache, b"x", &calls)), b"x");
        }
        assert_eq!(calls.get(), 1);
    }
    #[test]
    fn compression_reuse_entry_bound_and_recent_use() {
        let cache = Mutex::new(Cache::default());
        let calls = Cell::new(0);
        for n in 0..MAX_ENTRIES {
            observed(&cache, &(n as u64).to_le_bytes(), &calls);
        }
        observed(&cache, &0u64.to_le_bytes(), &calls);
        observed(&cache, &(MAX_ENTRIES as u64).to_le_bytes(), &calls);
        observed(&cache, &0u64.to_le_bytes(), &calls);
        assert_eq!(calls.get(), MAX_ENTRIES + 1, "recent entry survives eviction");
        observed(&cache, &1u64.to_le_bytes(), &calls);
        assert_eq!(calls.get(), MAX_ENTRIES + 2);
    }
    #[test]
    fn compression_reuse_byte_bound_and_oversized_bypass() {
        let cache = Mutex::new(Cache::default());
        let calls = Cell::new(0);
        let size = MAX_BYTES / 3;
        for n in 0..3 {
            observed(&cache, &vec![n; size], &calls);
        }
        observed(&cache, &vec![2; size], &calls);
        assert_eq!(calls.get(), 3);
        observed(&cache, &vec![0; size], &calls);
        assert_eq!(calls.get(), 4, "compressed bytes also count toward budget");
        let oversized = vec![7; MAX_BYTES + 1];
        for _ in 0..2 {
            assert_eq!(decoded(&observed(&cache, &oversized, &calls)), oversized);
        }
        assert_eq!(calls.get(), 6);
    }
    #[test]
    fn compression_reuse_poison_falls_back() {
        let cache = std::sync::Arc::new(Mutex::new(Cache::default()));
        let other = cache.clone();
        let _ = std::thread::spawn(move || {
            let _guard = other.lock().unwrap();
            panic!("poison control")
        })
        .join();
        let calls = Cell::new(0);
        for _ in 0..2 {
            assert_eq!(decoded(&observed(&cache, b"intact", &calls)), b"intact");
        }
        assert_eq!(calls.get(), 2);
    }
    #[test]
    fn compression_reuse_concurrent_requests_keep_payloads() {
        let cache = std::sync::Arc::new(Mutex::new(Cache::default()));
        std::thread::scope(|scope| {
            for n in 0..8u8 {
                let cache = cache.clone();
                scope.spawn(move || {
                    for _ in 0..10 {
                        let data = vec![n % 2; 4096];
                        assert_eq!(decoded(&cached(&cache, &data, fresh).unwrap()), data);
                    }
                });
            }
        });
        let calls = Cell::new(0);
        for n in 0..2 {
            assert_eq!(decoded(&observed(&cache, &vec![n; 4096], &calls)), vec![n; 4096]);
        }
        assert_eq!(calls.get(), 0, "completed concurrent results remain reusable");
    }
}
