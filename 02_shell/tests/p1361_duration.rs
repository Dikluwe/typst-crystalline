//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/shell/tests/p1361-duration.md
//! @prompt-hash 909752fd
//! @layer L2
//! @updated 2026-09-13

use std::collections::BTreeMap;
use std::time::Duration;

use typst_shell::duration::{format_duration, format_watch_line, WatchStatus};

#[derive(Clone, Copy, Debug)]
struct Case {
    id: &'static str,
    duration: Duration,
    expected: &'static str,
}

const CASES: [Case; 22] = [
    Case {
        id: "P01",
        duration: Duration::new(1_000_000, 0),
        expected: "11 d 13 h 46 min",
    },
    Case {
        id: "P02",
        duration: Duration::new(86_400, 0),
        expected: "1 d",
    },
    Case {
        id: "P03",
        duration: Duration::new(3_600, 0),
        expected: "1 h",
    },
    Case {
        id: "P04",
        duration: Duration::new(3_840, 0),
        expected: "1 h 4 min",
    },
    Case {
        id: "P05",
        duration: Duration::new(364, 770_000_000),
        expected: "6 min 4 s",
    },
    Case {
        id: "P06",
        duration: Duration::new(264, 776_000_000),
        expected: "4 min 24.78 s",
    },
    Case {
        id: "P07",
        duration: Duration::new(3, 0),
        expected: "3 s",
    },
    Case {
        id: "P08",
        duration: Duration::new(2, 849_200_000),
        expected: "2.85 s",
    },
    Case {
        id: "P09",
        duration: Duration::new(1, 0),
        expected: "1 s",
    },
    Case {
        id: "P10",
        duration: Duration::new(0, 999_000_000),
        expected: "999 ms",
    },
    Case {
        id: "P11",
        duration: Duration::new(0, 1_000_000),
        expected: "1 ms",
    },
    Case {
        id: "P12",
        duration: Duration::new(0, 294_816_000),
        expected: "294.82 ms",
    },
    Case {
        id: "P13",
        duration: Duration::new(0, 999_000),
        expected: "999 µs",
    },
    Case {
        id: "P14",
        duration: Duration::new(0, 734_000),
        expected: "734 µs",
    },
    Case {
        id: "P15",
        duration: Duration::new(0, 1_000),
        expected: "1 µs",
    },
    Case {
        id: "P16",
        duration: Duration::new(0, 999),
        expected: "999 ns",
    },
    Case {
        id: "P17",
        duration: Duration::new(0, 1),
        expected: "1 ns",
    },
    Case {
        id: "P18",
        duration: Duration::new(0, 0),
        expected: "0 s",
    },
    Case {
        id: "B01",
        duration: Duration::new(299, 999_000_000),
        expected: "4 min 60 s",
    },
    Case {
        id: "B02",
        duration: Duration::new(300, 0),
        expected: "5 min 0 s",
    },
    Case {
        id: "B03",
        duration: Duration::new(86_340, 0),
        expected: "23 h 59 min",
    },
    Case {
        id: "B04",
        duration: Duration::new(300, 6_000_000),
        expected: "5 min 0 s",
    },
];

fn observe<'a>(
    cases: impl IntoIterator<Item = &'a Case>,
) -> BTreeMap<&'static str, String> {
    cases
        .into_iter()
        .map(|case| {
            let rendered = format_duration(case.duration).to_string();
            assert_eq!(rendered, case.expected, "case {}", case.id);
            (case.id, rendered)
        })
        .collect()
}

#[test]
fn p1361_duration_exact_vector_is_deterministic_and_boundary_safe() {
    let expected = observe(CASES.iter());

    for _ in 0..3 {
        assert_eq!(observe(CASES.iter()), expected);
        assert_eq!(observe(CASES.iter().rev()), expected);
    }

    let boundary_controls = [
        ("P16", "999 ns", "P15", "1 µs"),
        ("P13", "999 µs", "P11", "1 ms"),
        ("P10", "999 ms", "P09", "1 s"),
        ("B01", "4 min 60 s", "B02", "5 min 0 s"),
        ("B03", "23 h 59 min", "P02", "1 d"),
        ("B04", "5 min 0 s", "B02", "5 min 0 s"),
    ];

    for (lower_id, lower, upper_id, upper) in boundary_controls {
        assert_eq!(expected.get(lower_id).map(String::as_str), Some(lower));
        assert_eq!(expected.get(upper_id).map(String::as_str), Some(upper));
    }
}

#[test]
fn p1362_watch_status_messages_are_exact_and_error_has_no_duration() {
    let elapsed = Duration::new(2, 849_200_000);

    assert_eq!(WatchStatus::Compiling.message(), "compiling ...");
    assert_eq!(
        WatchStatus::Success(elapsed).message(),
        "compiled successfully in 2.85 s"
    );
    assert_eq!(
        WatchStatus::PartialSuccess(elapsed).message(),
        "compiled with warnings in 2.85 s"
    );

    let error = WatchStatus::Error.message();
    assert_eq!(error, "compiled with errors");
    assert!(!error.contains("2.85 s"));
    assert!(!error.contains(" in "));
}

#[test]
fn p1363_watch_line_pads_timestamp_and_preserves_all_status_bodies() {
    let elapsed = Duration::new(2, 849_200_000);
    let cases = [
        (WatchStatus::Compiling, "[03:04:05] compiling ..."),
        (WatchStatus::Success(elapsed), "[03:04:05] compiled successfully in 2.85 s"),
        (
            WatchStatus::PartialSuccess(elapsed),
            "[03:04:05] compiled with warnings in 2.85 s",
        ),
        (WatchStatus::Error, "[03:04:05] compiled with errors"),
    ];

    for (status, expected) in cases {
        let line = format_watch_line(3, 4, 5, &status);

        assert_eq!(line, expected);
        assert!(line.starts_with("[03:04:05] "));
        assert_eq!(line.matches("] ").count(), 1);
        assert_eq!(&line[11..], status.message());
        assert!(!line.contains(['\r', '\n']));
    }
}
