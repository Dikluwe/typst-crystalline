// Linux-only instrumentation injected into a disposable RAM compiler.
mod lab_line_commit {
    use std::{alloc::{GlobalAlloc, Layout, System}, cell::{Cell, RefCell}};
    #[derive(Clone, Copy, Default)]
    struct Counts { alloc: u64, realloc: u64, bytes: u64 }
    impl Counts {
        fn minus(self, other: Self) -> Self {
            Self { alloc: self.alloc-other.alloc, realloc: self.realloc-other.realloc,
                   bytes: self.bytes-other.bytes }
        }
    }
    #[derive(Clone, Copy, Default)]
    struct Row { calls: u64, ns: u64, alloc: u64, realloc: u64, bytes: u64, hits: u64, builds: u64, edge_queries_saved: u64 }
    thread_local! {
        static ENABLED: bool = std::env::var_os("TEKT_LINE_CENSUS").is_some();
        static REUSE: bool = std::env::var_os("TEKT_LINE_REUSE").is_some();
        static DEPTH: Cell<u32> = const { Cell::new(0) };
        static CURRENT: Cell<Counts> = const { Cell::new(Counts { alloc:0,realloc:0,bytes:0 }) };
        static CHILD_NS: Cell<u64> = const { Cell::new(0) };
        static CHILD: Cell<Counts> = const { Cell::new(Counts { alloc:0,realloc:0,bytes:0 }) };
        static ROWS: RefCell<[Row; 129]> = RefCell::new([Row::default();129]);
    }
    struct Allocator;
    #[global_allocator]
    static ALLOCATOR: Allocator = Allocator;
    fn request(bytes: usize, realloc: bool) {
        if DEPTH.try_with(Cell::get).unwrap_or(0) != 0 {
            CURRENT.with(|c| {
                let mut v = c.get();
                v.bytes += bytes as u64;
                v.alloc += u64::from(!realloc);
                v.realloc += u64::from(realloc);
                c.set(v);
            });
        }
    }
    unsafe impl GlobalAlloc for Allocator {
        unsafe fn alloc(&self, l: Layout) -> *mut u8 { request(l.size(), false); unsafe { System.alloc(l) } }
        unsafe fn alloc_zeroed(&self, l: Layout) -> *mut u8 { request(l.size(), false); unsafe { System.alloc_zeroed(l) } }
        unsafe fn realloc(&self, p: *mut u8, l: Layout, n: usize) -> *mut u8 { request(n, true); unsafe { System.realloc(p,l,n) } }
        unsafe fn dealloc(&self, p: *mut u8, l: Layout) { unsafe { System.dealloc(p,l) } }
    }
    #[repr(C)]
    struct Timespec { seconds: i64, nanos: i64 }
    unsafe extern "C" { fn clock_gettime(clock: i32, out: *mut Timespec) -> i32; }
    fn cpu_ns() -> u64 {
        let mut t = Timespec { seconds: 0, nanos: 0 };
        assert_eq!(unsafe { clock_gettime(3, &mut t) }, 0);
        t.seconds as u64 * 1_000_000_000 + t.nanos as u64
    }
    pub(super) struct Probe { start: u64, children: u64, counts: Counts, child_counts: Counts }
    pub(super) fn enter() -> Option<Probe> {
        if !ENABLED.with(|x| *x) { return None; }
        DEPTH.with(|d| d.set(d.get()+1));
        Some(Probe { start: cpu_ns(), children: CHILD_NS.with(Cell::get), counts: CURRENT.with(Cell::get),
                     child_counts: CHILD.with(Cell::get) })
    }
    impl Probe {
        pub(super) fn finish(self, page: usize) {
            let elapsed = cpu_ns()-self.start;
            let child_ns = CHILD_NS.with(|c| c.replace(self.children + elapsed));
            let inclusive = CURRENT.with(Cell::get).minus(self.counts);
            let child = CHILD.with(|c| {
                let old = c.get();
                c.set(Counts { alloc:self.child_counts.alloc+inclusive.alloc,
                    realloc:self.child_counts.realloc+inclusive.realloc,
                    bytes:self.child_counts.bytes+inclusive.bytes });
                old.minus(self.child_counts)
            });
            DEPTH.with(|d| d.set(d.get()-1));
            let own = inclusive.minus(child);
            ROWS.with(|rows| {
                let mut all = rows.borrow_mut();
                let r = &mut all[page.min(128)];
                r.calls += 1; r.ns += elapsed.saturating_sub(child_ns-self.children);
                r.alloc += own.alloc; r.realloc += own.realloc; r.bytes += own.bytes;
            });
        }
    }
    #[allow(dead_code)]
    pub(super) fn reuse_enabled() -> bool { REUSE.with(|v| *v) }
    #[allow(dead_code)]
    pub(super) fn reused(page: usize, hit: bool, count: usize) {
        if ENABLED.with(|v| *v) {
            ROWS.with(|rows| {
                let mut rows = rows.borrow_mut();
                rows[page.min(128)].hits += u64::from(hit);
                rows[page.min(128)].builds += u64::from(!hit);
                if hit { rows[page.min(128)].edge_queries_saved += count as u64; }
            });
        }
    }
    pub(in crate::compiler::layout) fn dump(scope: &str) {
        if !ENABLED.with(|x| *x) { return; }
        ROWS.with(|rows| {
            let all = rows.replace([Row::default();129]);
            let values: Vec<_> = all.iter().enumerate().filter(|(_,r)| r.calls != 0)
                .map(|(p,r)| (p, r.calls,r.ns,r.alloc,r.realloc,r.bytes,r.hits,r.builds,r.edge_queries_saved)).collect();
            eprintln!("lab-line-commit-{scope} {values:?}");
        });
    }
}
