#!/usr/bin/env bash
set -euo pipefail
d="$(cd "$(dirname "$0")" && pwd)"
root="$(git -C "$d" rev-parse --show-toplevel)"
ram="$(mktemp -d /dev/shm/tekt-inert-proof.XXXXXXXX)"
trap 'rm -rf -- "$ram"' EXIT
export TEKT_LAB_WORK="$ram" TMPDIR="$ram"
python3 - "$d" "$ram" <<'PY'
from pathlib import Path
import sys
d,ram=map(Path,sys.argv[1:])
sys.path.insert(0,str(d))
from bench import CASE_STATES
proof=(d/"watch_probe.inc.rs").read_text().split("use std::sync::{Arc, atomic")[0]
rows=[]
for kind, states in CASE_STATES.items():
    base=ram/f"{kind}.typ"
    base.write_bytes(states[0][1])
    wanted=str(kind in ("regular","strict") or kind.startswith("family-")).lower()
    rows.append(f'    let source = Source::detached(include_str!("{base.name}")).unwrap();')
    rows.append(f'    assert_eq!(inert_source(&source), {wanted}, "{kind}");')
    if kind == "regular" or kind.startswith("family-"):
        for i in (1,2):
            path=ram/f"{kind}-{i}.typ"
            path.write_bytes(states[i][1])
            rows.append(f'    let other = source.reparse(include_str!("{path.name}").into()).unwrap();')
            rows.append('    assert!(inert_matches(&source, &other));')
            rows.append('    for node in watch_probe_tail(&source).0 { verify_range(&source, &other, node); }')
support="""
fn verify_range(before: &Source, after: &Source, node: &SyntaxNode) {
    if !watch_probe_comment(node) {
        assert_eq!(before.span_byte_range(node.span()), after.span_byte_range(node.span()));
        assert_eq!(before.span_to_line_col(node.span()), after.span_to_line_col(node.span()));
        for child in node.children() { verify_range(before, after, child); }
    }
}
"""
(ram/"check.rs").write_text("#![allow(dead_code)]\nuse typst_core::entities::{source::Source,layout_types::PagedDocument};\n"+proof+support+"\nfn main(){\n"+"\n".join(rows)+"\n}")
PY
deps="$root/lab/experiments/references-500-integration-2026-11-19/target-release/release/deps"
rlib=("$deps"/libtypst_core-*.rlib)
sha256sum "${rlib[0]}"
rustc --edition 2021 --extern "typst_core=${rlib[0]}" -L "dependency=$deps" "$ram/check.rs" -o "$ram/check"
"$ram/check"
printf '%s\n' "Classificador e resolução dos spans passaram."
python3 - "$d" <<'PY'
import hashlib,re,subprocess,sys
from pathlib import Path
sys.path.insert(0,sys.argv[1])
from bench import *
EXES["control"] = D/"control.elf"
def normalized(pdf):
    raw=subprocess.check_output(["qpdf","--qdf","--object-streams=disable",str(pdf),"-"])
    raw=re.sub(rb"(<xmpMM:(?:Instance|Document)ID>)[A-Za-z0-9+/=]+(</xmpMM:(?:Instance|Document)ID>)",
               lambda m:m[1]+b"x"*(len(m[0])-len(m[1])-len(m[2]))+m[2],raw)
    return hashlib.sha256(re.sub(rb"/ID \[<[0-9a-f]+><[0-9a-f]+>\]",b"/ID []",raw)).hexdigest()
for kind, states in CASE_STATES.items():
    if kind.startswith("family-") or kind in ("regular","unclosed","whole"):
        selections=states[:3]
        case=WORK/"fresh"/kind
        runner.run_fresh(case,"control",selections)
        hashes=[normalized(case/(st+"-fresh.pdf")) for st,_,_ in selections]
        print(kind, hashes, flush=True)
        if kind!="unclosed":
            assert len(set(hashes))==(2 if kind=="whole" else 1),kind
PY
