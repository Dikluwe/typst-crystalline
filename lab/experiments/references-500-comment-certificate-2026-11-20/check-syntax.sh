#!/usr/bin/env bash
set -euo pipefail
d="$(cd -- "$(dirname -- "$0")" && pwd)"
root="$(git -C "$d" rev-parse --show-toplevel)"
ram="$(mktemp -d /dev/shm/tekt-comment-syntax.XXXXXXXX)"
trap 'rm -rf -- "$ram"' EXIT
export TMPDIR="$ram"
python3 - "$d" "$ram" <<'PY'
from pathlib import Path
import sys
d, ram = map(Path, sys.argv[1:])
sys.path.insert(0, str(d))
from bench import CASE_STATES
proof=(d/"watch_probe.inc.rs").read_text().split("struct WatchProbeEntry")[0]
rows=[]
for case, states in CASE_STATES.items():
    paths=[]
    for state, source, _ in states:
        path=ram/f"{case}-{state}.typ"
        path.write_bytes(source)
        paths.append(f'("{state}", include_str!("{path.name}"))')
    rows.append(f'    run("{case}", &[' + ", ".join(paths) + ']);')
program=proof+r"""
use typst_core::entities::source::Source;
fn run(case: &str, revisions: &[(&str, &str)]) {
    let mut previous = Source::detached(revisions[0].1).unwrap();
    assert!(watch_probe_source_eligible(&previous));
    for &(name, text) in &revisions[1..] {
        let current = previous.reparse(text.to_owned()).unwrap();
        assert!(watch_probe_source_eligible(&current));
        let result = watch_probe_comments_match(&previous, &current);
        println!("{case} {name} match={result}");
        let expected = match (case, name) {
            ("regular", "comment" | "comment-2")
            | ("spans", "span-kept" | "span-eof-insert" | "span-eof-remove")
            | ("dependency", _) | ("raw", _) => true,
            _ => false,
        };
        assert_eq!(result, expected, "{case} {name}");
        if result {
            ranges_equal(&previous, &current);
        }
        watch_probe_spans(name, &previous, &current);
        previous = current;
    }
}
fn ranges_equal(previous: &Source, current: &Source) {
    fn visit(source: &Source, node: &SyntaxNode) {
        for child in node.children() {
            if !watch_probe_comment(child) {
                let range = source.span_byte_range(child.span()).unwrap();
                assert_eq!(range.len(), child.len());
                visit(source, child);
            }
        }
    }
    fn compare(previous: &Source, current: &Source, children: &[SyntaxNode]) {
        for node in children {
            if !watch_probe_comment(node) {
                let span = node.span();
                assert_eq!(previous.span_byte_range(span), current.span_byte_range(span));
                assert_eq!(previous.span_to_line_col(span), current.span_to_line_col(span));
                compare(previous, current, node.children().as_slice());
            }
        }
    }
    visit(previous, previous.root());
    visit(current, current.root());
    compare(previous, current, watch_probe_root_prefix(previous.root()));
}
fn main() {
    let raw = Source::detached("#raw(\"// texto\")").unwrap();
    let edited = raw.reparse("#raw(\"// outro\")".into()).unwrap();
    assert!(!watch_probe_comments_match(&raw, &edited));
    let broken = raw.reparse("#raw(\"// texto".into()).unwrap();
    assert!(!watch_probe_source_eligible(&broken));
""" + "\n".join(rows) + "\n}"
(ram/"proof.rs").write_text(program)
PY
deps="$root/lab/experiments/references-500-integration-2026-11-19/target-release/release/deps"
rlib=("$deps"/libtypst_core-*.rlib)
test "${#rlib[@]}" -eq 1
sha256sum "${rlib[0]}"
rustc --edition 2021 --extern "typst_core=${rlib[0]}" -L "dependency=$deps" "$ram/proof.rs" -o "$ram/proof"
"$ram/proof"
