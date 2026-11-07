//! Independent gate contracts: observable closure order, refusal and publication.
use joint_font_witness::establish;
use std::cell::{Cell, RefCell};

#[derive(Clone, Debug, PartialEq, Eq)]
struct Policy(&'static str);
#[derive(Debug)]
struct Prepared(u32);
#[derive(Debug, PartialEq, Eq)]
struct Catalog(u32);

#[test]
fn establish_preserves_gate_order_and_passes_the_original_values() {
    let log = RefCell::new(Vec::new());
    let result = establish(
        || { log.borrow_mut().push("discover"); Some((Policy("loaded-policy"), Prepared(17))) },
        |p| { log.borrow_mut().push("g2"); assert_eq!(p.0, 17); true },
        |p| { log.borrow_mut().push("key"); assert_eq!(p.0, 17); p.0 + 10 },
        |p| { log.borrow_mut().push("build"); Catalog(p.0 + 100) },
        |key| { log.borrow_mut().push("g3"); assert_eq!(*key, 27); true },
        |policy| { log.borrow_mut().push("policy"); assert_eq!(policy, &Policy("loaded-policy")); true },
    );
    assert_eq!(result, Some((Policy("loaded-policy"), 27, Catalog(117))));
    assert_eq!(*log.borrow(), ["discover", "g2", "key", "build", "g3", "policy"]);
}

#[test]
fn missing_discovery_short_circuits_every_later_operation() {
    let log = RefCell::new(Vec::new());
    let result = establish(
        || { log.borrow_mut().push("discover"); None::<(Policy, Prepared)> },
        |_| panic!("g2 must not run"),
        |_| -> u32 { panic!("key must not run") },
        |_| -> Catalog { panic!("build must not run") },
        |_| panic!("g3 must not run"),
        |_| panic!("policy must not run"),
    );
    assert!(result.is_none());
    assert_eq!(*log.borrow(), ["discover"]);
}

#[test]
fn every_failed_gate_short_circuits_without_publishing_a_partial_catalog() {
    for rejected in ["g2", "g3", "policy"] {
        let log = RefCell::new(Vec::new());
        let published = RefCell::new((Policy("old"), 5u32, Catalog(5)));
        let result = establish(
            || { log.borrow_mut().push("discover"); Some((Policy("new"), Prepared(9))) },
            |_| { log.borrow_mut().push("g2"); rejected != "g2" },
            |p| { log.borrow_mut().push("key"); p.0 },
            |p| { log.borrow_mut().push("build"); Catalog(p.0) },
            |_| { log.borrow_mut().push("g3"); rejected != "g3" },
            |_| { log.borrow_mut().push("policy"); rejected != "policy" },
        );
        assert!(result.is_none(), "{rejected}: refusing a gate must return None");
        if let Some(candidate) = result { *published.borrow_mut() = candidate; }
        assert_eq!(*published.borrow(), (Policy("old"), 5, Catalog(5)));
        let expected: &[&str] = match rejected {
            "g2" => &["discover", "g2"],
            "g3" => &["discover", "g2", "key", "build", "g3"],
            _ => &["discover", "g2", "key", "build", "g3", "policy"],
        };
        assert_eq!(log.borrow().as_slice(), expected, "{rejected}");
    }
}

#[test]
fn post_build_gate_sees_mutation_and_does_not_call_policy_after_refusal() {
    let generation = Cell::new(1u32);
    let policy_calls = Cell::new(0);
    let result = establish(
        || Some((Policy("policy"), Prepared(generation.get()))),
        |p| p.0 == generation.get(),
        |p| p.0,
        |p| { generation.set(2); Catalog(p.0) },
        |key| *key == generation.get(),
        |_| { policy_calls.set(policy_calls.get() + 1); true },
    );
    assert!(result.is_none());
    assert_eq!(policy_calls.get(), 0);
}
