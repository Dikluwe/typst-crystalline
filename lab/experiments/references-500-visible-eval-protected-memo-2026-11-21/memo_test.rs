mod lab {
    const MAX_TOTAL: usize = 8 * 1024 * 1024;
    type Value = u32;

    struct Key {
        id: u32,
        charge: usize,
    }

    impl Key {
        fn matches(&self, other: &Self) -> bool {
            self.id == other.id
        }
    }

    include!("memo.inc.rs");

    fn key(id: u32, charge: usize) -> Key {
        Key { id, charge }
    }

    #[test]
    fn old_entries_make_it_to_their_consultation() {
        let mut memo = Memo::default();
        memo.begin(true, 8);
        memo.store(key(2, 4), 2);
        memo.store(key(3, 4), 3);
        memo.finish();

        memo.begin(true, 8);
        assert_eq!(memo.lookup(&key(1, 4)), None);
        memo.store(key(1, 4), 1);
        assert_eq!(memo.lookup(&key(2, 4)), Some(2));
        assert_eq!(memo.lookup(&key(3, 4)), Some(3));
        assert_eq!(memo.report.evicted_during, 0);
        memo.finish();
        assert_eq!(memo.report.evicted_after, 1);
        assert_eq!(memo.charged, 4);
    }

    #[test]
    fn consultation_releases_the_entry_for_eviction() {
        let mut memo = Memo::default();
        memo.begin(true, 8);
        memo.store(key(2, 4), 2);
        memo.store(key(3, 4), 3);
        memo.finish();

        memo.begin(true, 8);
        assert_eq!(memo.lookup(&key(2, 4)), Some(2));
        memo.store(key(1, 4), 1);
        assert_eq!(memo.lookup(&key(2, 4)), None);
        assert_eq!(memo.lookup(&key(3, 4)), Some(3));
        assert_eq!(memo.lookup(&key(1, 4)), Some(1));
        memo.finish();
        assert_eq!(memo.report.evicted_during, 1);
        assert_eq!(memo.report.evicted_after, 0);
        assert_eq!(memo.charged, 8);
    }

    #[test]
    fn a_denied_insertion_does_not_partially_evict() {
        let mut memo = Memo::default();
        memo.begin(true, 12);
        for i in 1..=3 {
            memo.store(key(i, 4), i);
        }
        memo.finish();

        memo.begin(true, 12);
        assert_eq!(memo.lookup(&key(1, 4)), Some(1));
        memo.store(key(4, 9), 4);
        assert_eq!(memo.report.deferred, 1);
        assert_eq!(memo.report.evicted_during, 0);
        assert_eq!(memo.lookup(&key(1, 4)), Some(1));
        assert_eq!(memo.lookup(&key(2, 4)), Some(2));
        assert_eq!(memo.lookup(&key(3, 4)), Some(3));
        memo.finish();
        assert!(memo.charged <= 12);
    }
}
