"""Read-only raw measurements of the authorized Array<-Bytes prerequisite."""
import datetime, hashlib, json, pathlib, subprocess, sys, time

def digest(path):
    return hashlib.sha256(pathlib.Path(path).read_bytes()).hexdigest()

def run(argv):
    start = datetime.datetime.now(datetime.timezone.utc).isoformat()
    clock = time.monotonic()
    result = subprocess.run(argv, capture_output=True, text=True, timeout=30)
    return dict(argv=argv, utc=start, seconds=time.monotonic()-clock,
                exit=result.returncode, stdout=result.stdout, stderr=result.stderr)

sources = ["01_core/src/compiler/eval/call_dispatch.rs",
           "lab/typst-original/crates/typst-library/src/foundations/array.rs"]
cases = [
    'array(bytes(()))', 'array(bytes((0, 1, 127, 128, 255)))',
    '{ let convert = array; convert(bytes((255, 0))) }',
    'array(..arguments(bytes((3, 2, 1))))',
    'array(bytes((1,)), 2)', 'array(bytes((1,)), other: 2)',
    'array(value: bytes((1,)))', 'array()', 'array(1)',
    'array(version(1, 2))', 'array((1, 2))',
    'array(bytes((1,)), panic("argument-evaluated"))',
]
report = dict(schema="raw-prerequisite-measurement-not-independent-verdict",
              head=run(["git", "rev-parse", "HEAD"]),
              diff_stat=run(["git", "diff", "HEAD", "--stat"]),
              status=run(["git", "status", "--short", "--untracked-files=no"]),
              sources={path:digest(path) for path in sources}, results=[])
for binary in sys.argv[1:]:
    binary_pin = digest(binary)
    for expression in cases:
        row = run([binary, "eval", expression])
        row["binary_sha256"] = binary_pin
        report["results"].append(row)
report["source_pins_unchanged"] = all(digest(p)==h for p,h in report["sources"].items())
print(json.dumps(report, ensure_ascii=False, indent=2))
