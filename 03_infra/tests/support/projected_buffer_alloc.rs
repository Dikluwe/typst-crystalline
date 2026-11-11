//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/tests/support/projected_buffer_alloc.md
//! @prompt-hash 63840640
//! @layer L3
//! @updated 2026-11-10

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;
thread_local! { static OWNED:Cell<bool>=const {Cell::new(false)}; }
fn owner() -> bool {
    OWNED.try_with(Cell::get).unwrap_or(false)
}
use std::sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering},
    Mutex,
};
const LIMIT: usize = 131_072;
#[derive(Clone, Copy)]
struct Event {
    kind: u8,
    old: usize,
    new: usize,
    size: usize,
}
const EMPTY: Event = Event { kind: 0, old: 0, new: 0, size: 0 };
struct Trace {
    events: [Event; LIMIT],
    len: usize,
    overflow: bool,
}
static TRACE: Mutex<Trace> =
    Mutex::new(Trace { events: [EMPTY; LIMIT], len: 0, overflow: false });
static ACTIVE: AtomicBool = AtomicBool::new(false);
static ATTEMPTS: AtomicUsize = AtomicUsize::new(0);
static FAIL_NTH: AtomicUsize = AtomicUsize::new(0);
static FAILED: AtomicBool = AtomicBool::new(false);
pub struct TraceAlloc;
fn reject() -> bool {
    if !ACTIVE.load(Ordering::Relaxed) || !owner() {
        return false;
    }
    let n = ATTEMPTS.fetch_add(1, Ordering::Relaxed) + 1;
    if n == FAIL_NTH.load(Ordering::Relaxed) {
        FAILED.store(true, Ordering::Relaxed);
        true
    } else {
        false
    }
}
fn record(e: Event) {
    if !ACTIVE.load(Ordering::Relaxed) || !owner() {
        return;
    }
    let mut t = TRACE.lock().unwrap();
    if t.len < LIMIT {
        let i = t.len;
        t.events[i] = e;
        t.len += 1;
    } else {
        t.overflow = true;
    }
}
// SAFETY: Every successful operation delegates the original pointer, layout and
// size to System. The recorder stores numeric addresses only; it never reads or
// writes allocated memory. Explicit failure returns null only to the TLS owner.
unsafe impl GlobalAlloc for TraceAlloc {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        if reject() {
            return std::ptr::null_mut();
        }
        let p = unsafe { System.alloc(l) };
        if !p.is_null() {
            record(Event { kind: 1, old: 0, new: p as usize, size: l.size() });
        }
        p
    }
    unsafe fn alloc_zeroed(&self, l: Layout) -> *mut u8 {
        if reject() {
            return std::ptr::null_mut();
        }
        let p = unsafe { System.alloc_zeroed(l) };
        if !p.is_null() {
            record(Event { kind: 1, old: 0, new: p as usize, size: l.size() });
        }
        p
    }
    unsafe fn realloc(&self, p: *mut u8, l: Layout, n: usize) -> *mut u8 {
        if reject() {
            return std::ptr::null_mut();
        }
        let q = unsafe { System.realloc(p, l, n) };
        if !q.is_null() {
            record(Event { kind: 2, old: p as usize, new: q as usize, size: n });
        }
        q
    }
    unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
        record(Event { kind: 3, old: p as usize, new: 0, size: l.size() });
        unsafe { System.dealloc(p, l) }
    }
}
#[derive(Clone, Copy)]
pub struct Snapshot {
    len: usize,
    pub failed: bool,
    pub overflow: bool,
}
static SERIAL: Mutex<()> = Mutex::new(());
pub fn serial() -> std::sync::MutexGuard<'static, ()> {
    SERIAL.lock().unwrap_or_else(|e| e.into_inner())
}
pub struct Window;
impl Drop for Window {
    fn drop(&mut self) {
        OWNED.with(|o| o.set(false));
        ACTIVE.store(false, Ordering::Relaxed);
    }
}
pub fn start(fail_nth: usize) -> Window {
    ACTIVE.store(false, Ordering::Relaxed);
    {
        let mut t = TRACE.lock().unwrap();
        t.len = 0;
        t.overflow = false;
    }
    ATTEMPTS.store(0, Ordering::Relaxed);
    FAIL_NTH.store(fail_nth, Ordering::Relaxed);
    FAILED.store(false, Ordering::Relaxed);
    OWNED.with(|o| o.set(true));
    ACTIVE.store(true, Ordering::Relaxed);
    Window
}
pub fn stop(window: Window) -> Snapshot {
    drop(window);
    let t = TRACE.lock().unwrap();
    Snapshot {
        len: t.len,
        failed: FAILED.load(Ordering::Relaxed),
        overflow: t.overflow,
    }
}
impl Snapshot {
    /// Follow reallocations backwards until this lifetime's initial allocation.
    /// The initial allocation terminates traversal, even if older freed memory reused
    /// the same numeric address. This does not count unrelated Reader/plan buffers.
    pub fn lineage(self, final_ptr: *const u8) -> (usize, usize) {
        let mut ptr = final_ptr as usize;
        let mut growths = 0;
        let mut origin = None;
        {
            let t = TRACE.lock().unwrap();
            for e in t.events[..self.len].iter().rev() {
                if e.kind == 2 && e.new == ptr {
                    growths += 1;
                    ptr = e.old;
                } else if e.kind == 1 && e.new == ptr {
                    origin = Some(e.size);
                    break;
                }
            }
        }
        (growths, origin.expect("returned buffer's initial allocation must be traced"))
    }
    pub fn max_request(self) -> usize {
        let t = TRACE.lock().unwrap();
        t.events[..self.len]
            .iter()
            .filter(|e| e.kind == 1 || e.kind == 2)
            .map(|e| e.size)
            .max()
            .unwrap_or(0)
    }
}
