#!/usr/bin/env python3
"""Black-box oracles for P1285.

Freeze only from the ratified vanilla binary. Later checks compare JSON/YAML
as typed trees, raw output as bytes, and always keep stdout, stderr and exit
status separate. Unknown is a failing verdict.
"""

from __future__ import annotations

import argparse
import base64
import hashlib
import json
import math
import os
from pathlib import Path
import re
import subprocess
import sys
import tempfile
from dataclasses import dataclass
from datetime import datetime, timezone
from typing import Any, Callable

try:
    import yaml
except ImportError:  # Explicitly mapped to Unknown below.
    yaml = None


SOURCE_PIN = "a51e02804"
RATIFIED_BINARY = Path("/usr/local/bin/typst")
RATIFIED_BINARY_SHA256 = "7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8"
DEFAULT_BASELINE = Path(__file__).with_name("p1285-oracle-baseline.json")
QUERY_WARNING = "subcommand is deprecated"

QUERY_CORPUS = b'''= First <head-one>
#figure(rect(width: 10pt, height: 20pt, fill: red), caption: [Cap]) <fig-one>
$ x^2 $ <eq-one>
#metadata((name: "x", n: 2)) <meta-one>
'''
TWO_HEADINGS = b"= First\n= Second\n"


def text(value: str) -> dict[str, str]:
    return {"func": "text", "text": value}


COMPOSITE = {
    "a": 1,
    "b": [2, 3],
    "v": "version(0, 15, 1)",
    "t": text("Hi"),
    "n": None,
    "finite": 1.5,
    "symbol": "❤️",
    "bytes": "bytes(2)",
}

FALLBACKS = {
    "k-auto": "auto",
    "k-length": "12pt",
    "k-color": 'rgb("#ff4136")',
    "k-label": "<intro>",
    "k-datetime": "datetime(year: 2024, month: 2, day: 3)",
    "k-decimal": 'decimal("123.4500")',
    "k-duration": "duration(seconds: 2)",
    "k-func": "gcd",
    "k-type": "int",
    "k-gradient": (
        "gradient.linear((oklab(65.95%, 0.2, 0.108), 0%), "
        "(oklab(56.22%, -0.05, -0.17), 100%))"
    ),
    "k-tiling": "tiling((10pt, 10pt), ..)",
    "k-regex": 'regex("a+")',
    "k-selector": "heading.where(level: 1)",
    "k-version": "version(0, 15, 1)",
}

CONTENT = {
    "func": "sequence",
    "children": [
        {"func": "strong", "body": text("Hi")},
        {"func": "space"},
        text("there"),
    ],
}


@dataclass(frozen=True)
class Case:
    case_id: str
    args: tuple[str, ...]
    stdin: bytes = b""
    stdout_kind: str = "json"
    expected_exit: int = 0
    expected: Any = None
    stderr_contains: tuple[str, ...] = ()
    stderr_empty: bool = False
    validator: str | None = None


def query_case(
    case_id: str,
    selector: str,
    *extra: str,
    stdin: bytes = QUERY_CORPUS,
    kind: str = "json",
    expected: Any = None,
    validator: str | None = None,
) -> Case:
    return Case(
        case_id,
        ("query", "-", selector, *extra),
        stdin=stdin,
        stdout_kind=kind,
        expected=expected,
        stderr_contains=(QUERY_WARNING,),
        validator=validator,
    )


SHOW_EXPECTATIONS = {
    "show_literal_repeated": [text("foo"), text("foo")],
    "show_literal_partial": [text("foo")],
    "show_regex_repeated": [text("foo"), text("fxo")],
    "show_literal_dot": [text(".")],
    "show_regex_dot": [text("a"), text("."), text("b")],
    "show_regex_no_match": [],
    "show_literal_node_boundary": [],
}


def show_case(case_id: str, source: str) -> Case:
    return query_case(
        case_id,
        "metadata",
        "--field",
        "value",
        "--format",
        "json",
        stdin=source.encode(),
        expected=SHOW_EXPECTATIONS[case_id],
    )


CASES: tuple[Case, ...] = (
    Case("eval_help_formats", ("eval", "--help"), stdout_kind="help-eval", expected={"default": "json", "formats": ["json", "yaml", "raw"]}, stderr_empty=True),
    Case("query_help_formats", ("query", "--help"), stdout_kind="help-query", expected={"default": "json", "formats": ["json", "yaml"]}, stderr_empty=True),
    Case(
        "eval_composite_json",
        ("eval", '(a: 1, b: (2, 3), v: sys.version, t: [Hi], n: none, finite: 1.5, symbol: emoji.heart, bytes: bytes((65, 66)))', "--format", "json"),
        expected=COMPOSITE,
        stderr_empty=True,
    ),
    Case(
        "eval_composite_yaml",
        ("eval", '(a: 1, b: (2, 3), v: sys.version, t: [Hi], n: none, finite: 1.5, symbol: emoji.heart, bytes: bytes((65, 66)))', "--format", "yaml"),
        stdout_kind="yaml",
        expected=COMPOSITE,
        stderr_empty=True,
    ),
    Case(
        "eval_composite_yaml_pretty",
        ("eval", '(a: 1, b: (2, 3), v: sys.version, t: [Hi], n: none, finite: 1.5, symbol: emoji.heart, bytes: bytes((65, 66)))', "--format", "yaml", "--pretty"),
        stdout_kind="yaml",
        expected=COMPOSITE,
        stderr_empty=True,
    ),
    Case(
        "eval_fallbacks_json",
        (
            "eval",
            '(k-auto: auto, k-length: 12pt, k-color: rgb("#ff4136"), k-label: <intro>, k-datetime: datetime(year: 2024, month: 2, day: 3), k-decimal: decimal("123.4500"), k-duration: duration(seconds: 2), k-func: calc.gcd, k-type: int, k-gradient: gradient.linear(red, blue), k-tiling: tiling([x], size: (10pt, 10pt)), k-regex: regex("a+"), k-selector: selector(heading.where(level: 1)), k-version: sys.version)',
            "--format",
            "json",
        ),
        expected=FALLBACKS,
        stderr_empty=True,
    ),
    Case("eval_content_json", ("eval", "[*Hi* there]", "--format", "json"), expected=CONTENT, stderr_empty=True),
    Case("eval_nonfinite_json", ("eval", "calc.inf", "--format", "json"), expected=None, stderr_empty=True),
    Case("eval_nonfinite_yaml", ("eval", "calc.inf", "--format", "yaml"), stdout_kind="yaml", expected=math.inf, stderr_empty=True),
    Case("eval_raw_string", ("eval", '"A\\nB"', "--format", "raw"), stdout_kind="raw", expected=b"A\nB", stderr_empty=True),
    Case("eval_raw_bytes", ("eval", "bytes((0, 10, 255, 65))", "--format", "raw"), stdout_kind="raw", expected=b"\x00\x0a\xffA", stderr_empty=True),
    Case("eval_raw_version_rejected", ("eval", "sys.version", "--format", "raw"), stdout_kind="empty", expected_exit=1, stderr_contains=("cannot print version in raw format", "only supports strings and bytes")),
    Case("eval_invalid_format", ("eval", "1", "--format", "toml"), stdout_kind="empty", expected_exit=2, stderr_contains=("invalid value", "toml", "json", "yaml", "raw")),
    Case("eval_invalid_expression", ("eval", "(", "--format", "json"), stdout_kind="empty", expected_exit=1, stderr_contains=("unclosed delimiter",)),
    Case("query_invalid_raw", ("query", "-", "heading", "--format", "raw"), stdout_kind="empty", expected_exit=2, stderr_contains=("invalid value", "raw", "json", "yaml")),
    query_case("query_heading_json", "heading", "--format", "json", validator="heading"),
    query_case("query_figure_json", "figure", "--format", "json", validator="figure"),
    query_case("query_figure_yaml", "figure", "--format", "yaml", kind="yaml", validator="figure"),
    query_case("query_equation_json", "math.equation", "--format", "json", validator="equation"),
    query_case("query_metadata_json", "metadata", "--format", "json", validator="metadata"),
    query_case("query_metadata_yaml", "metadata", "--format", "yaml", "--pretty", kind="yaml", validator="metadata"),
    query_case("query_label_figure_json", "<fig-one>", "--format", "json", validator="figure"),
    query_case("query_figure_field_body", "figure", "--field", "body", "--format", "json", expected=[{"func": "rect", "width": "0% + 10pt", "height": "0% + 20pt", "fill": 'rgb("#ff4136")'}]),
    query_case("query_metadata_field_one", "metadata", "--field", "value", "--one", "--format", "json", expected={"name": "x", "n": 2}),
    query_case("query_heading_level_one", "heading", "--field", "level", "--one", "--format", "json", expected=1),
    query_case("query_heading_body_order", "heading", "--field", "body", "--format", "json", stdin=TWO_HEADINGS, expected=[text("First"), text("Second")]),
    query_case("query_missing_field_list", "figure", "--field", "does-not-exist", "--format", "json", expected=[]),
    Case("query_missing_field_one", ("query", "-", "figure", "--field", "does-not-exist", "--one", "--format", "json"), stdin=QUERY_CORPUS, stdout_kind="empty", expected_exit=1, stderr_contains=("no such field found for element",)),
    Case("query_one_zero", ("query", "-", "heading", "--one", "--format", "json"), stdin=b"Plain text\n", stdout_kind="empty", expected_exit=1, stderr_contains=("expected exactly one element, found 0",)),
    Case("query_one_many", ("query", "-", "heading", "--one", "--format", "json"), stdin=TWO_HEADINGS, stdout_kind="empty", expected_exit=1, stderr_contains=("expected exactly one element, found 2",)),
    show_case("show_literal_repeated", '#show "foo": it => metadata(it)\nfoo bar foo\n'),
    show_case("show_literal_partial", '#show "foo": it => metadata(it)\nxfooY\n'),
    show_case("show_regex_repeated", '#show regex("f.o"): it => metadata(it)\nfoo fxo bar\n'),
    show_case("show_literal_dot", '#show ".": it => metadata(it)\na.b\n'),
    show_case("show_regex_dot", '#show regex("."): it => metadata(it)\na.b\n'),
    show_case("show_regex_no_match", '#show regex("z+"): it => metadata(it)\nfoo fxo\n'),
    show_case("show_literal_node_boundary", '#show "foobar": it => metadata(it)\nfoo#strong[bar]\n'),
    Case("selector_empty_text", ("eval", 'selector("")', "--format", "json"), stdout_kind="empty", expected_exit=1, stderr_contains=("text selector is empty",)),
    Case("selector_empty_regex", ("eval", 'selector(regex(""))', "--format", "json"), stdout_kind="empty", expected_exit=1, stderr_contains=("regex selector is empty",)),
    Case("selector_regex_matches_empty", ("eval", 'selector(regex("a*"))', "--format", "json"), stdout_kind="empty", expected_exit=1, stderr_contains=("regex matches empty text",)),
    Case("query_text_not_locatable", ("query", "-", '"foo"', "--format", "json"), stdin=b"foo\n", stdout_kind="empty", expected_exit=1, stderr_contains=("text is not locatable",)),
    Case("query_regex_not_locatable", ("query", "-", 'regex("foo")', "--format", "json"), stdin=b"foo\n", stdout_kind="empty", expected_exit=1, stderr_contains=("text is not locatable",)),
)


class HarnessUnknown(RuntimeError):
    pass


@dataclass
class Observation:
    exit_code: int | None
    stdout: bytes
    stderr: bytes
    runtime_error: str | None = None


def sha256_bytes(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def clean_environment() -> dict[str, str]:
    keep = ("HOME", "PATH", "XDG_CACHE_HOME", "XDG_DATA_HOME", "FONTCONFIG_FILE", "FONTCONFIG_PATH")
    env = {key: os.environ[key] for key in keep if key in os.environ}
    env.update({"NO_COLOR": "1", "TERM": "dumb", "LANG": "C.UTF-8", "LC_ALL": "C.UTF-8"})
    return env


def run_case(binary: Path, case: Case) -> Observation:
    try:
        proc = subprocess.run(
            [str(binary), "--color", "never", *case.args],
            input=case.stdin,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            env=clean_environment(),
            timeout=30,
            check=False,
        )
        return Observation(proc.returncode, proc.stdout, proc.stderr)
    except (OSError, subprocess.TimeoutExpired) as exc:
        return Observation(None, b"", b"", f"{type(exc).__name__}: {exc}")


def no_duplicate_object(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    result: dict[str, Any] = {}
    for key, value in pairs:
        if key in result:
            raise ValueError(f"duplicate JSON key: {key}")
        result[key] = value
    return result


def extract_help_formats(raw: bytes) -> dict[str, Any]:
    value = raw.decode("utf-8")
    lines = value.splitlines()
    start = next((i for i, line in enumerate(lines) if "--format <FORMAT>" in line), None)
    if start is None:
        raise ValueError("help has no --format <FORMAT> block")
    block: list[str] = []
    for line in lines[start + 1 :]:
        if re.match(r"^(?: {2}-\w| {6}--\w)", line):
            break
        block.append(line.strip())
    joined = "\n".join(block)
    default_match = re.search(r"\[default: ([^\]]+)\]", joined)
    formats: list[str] = []
    inline = re.search(r"\[possible values: ([^\]]+)\]", joined)
    if inline:
        formats = [part.strip() for part in inline.group(1).split(",")]
    else:
        for line in block:
            match = re.match(r"- ([a-z0-9-]+)(?::|$)", line)
            if match:
                formats.append(match.group(1))
    if default_match is None or not formats:
        raise ValueError("could not parse format default/values from help")
    return {"default": default_match.group(1), "formats": formats}


def semantic_stdout(case: Case, raw: bytes) -> Any:
    if case.stdout_kind == "raw":
        return {"base64": base64.b64encode(raw).decode("ascii")}
    if case.stdout_kind == "empty":
        return {"base64": base64.b64encode(raw).decode("ascii")}
    if case.stdout_kind.startswith("help-"):
        return extract_help_formats(raw)
    if not raw.endswith(b"\n"):
        raise ValueError("structured stdout is not newline-terminated")
    decoded = raw.decode("utf-8")
    if case.stdout_kind == "json":
        return json.loads(decoded, object_pairs_hook=no_duplicate_object)
    if case.stdout_kind == "yaml":
        if yaml is None:
            raise HarnessUnknown("PyYAML is not installed")
        return yaml.safe_load(decoded)
    raise HarnessUnknown(f"unknown stdout kind {case.stdout_kind}")


def semantic_jsonable(value: Any) -> Any:
    if isinstance(value, float):
        if math.isnan(value):
            return {"$float": "nan"}
        if math.isinf(value):
            return {"$float": "+inf" if value > 0 else "-inf"}
        return value
    if isinstance(value, dict):
        return {key: semantic_jsonable(item) for key, item in sorted(value.items())}
    if isinstance(value, list):
        return [semantic_jsonable(item) for item in value]
    return value


def validate_shape(name: str, value: Any) -> list[str]:
    issues: list[str] = []
    if not isinstance(value, list) or len(value) != 1 or not isinstance(value[0], dict):
        return [f"{name}: expected one object"]
    item = value[0]
    expected = {
        "heading": ("heading", "<head-one>", "body"),
        "figure": ("figure", "<fig-one>", "body"),
        "equation": ("equation", "<eq-one>", "body"),
        "metadata": ("metadata", "<meta-one>", "value"),
    }[name]
    if item.get("func") != expected[0]:
        issues.append(f"{name}: func is {item.get('func')!r}, expected {expected[0]!r}")
    if item.get("label") != expected[1]:
        issues.append(f"{name}: label is {item.get('label')!r}, expected {expected[1]!r}")
    if expected[2] not in item:
        issues.append(f"{name}: missing public field {expected[2]!r}")
    if name == "heading" and item.get("body") != text("First"):
        issues.append("heading: body morphology differs")
    if name == "figure" and not isinstance(item.get("body"), dict):
        issues.append("figure: body is not structured content")
    if name == "equation" and not isinstance(item.get("body"), dict):
        issues.append("equation: body is not structured content")
    if name == "metadata" and item.get("value") != {"name": "x", "n": 2}:
        issues.append("metadata: value is not the structured dict")
    if item.get("func") == "label":
        issues.append(f"{name}: label transport leaked as func=label")
    return issues


def judge(case: Case, observation: Observation) -> tuple[str, Any, list[str]]:
    if observation.runtime_error:
        return "Unknown", None, [observation.runtime_error]
    issues: list[str] = []
    if observation.exit_code != case.expected_exit:
        issues.append(f"exit {observation.exit_code}, expected {case.expected_exit}")
    try:
        normalized = semantic_stdout(case, observation.stdout)
    except HarnessUnknown as exc:
        return "Unknown", None, [str(exc)]
    except (UnicodeDecodeError, ValueError, json.JSONDecodeError) as exc:
        return "Violated", None, [f"stdout parse: {exc}"]
    if case.stdout_kind == "empty" and observation.stdout != b"":
        issues.append("stdout is not empty")
    elif case.stdout_kind == "raw" and observation.stdout != case.expected:
        issues.append("raw stdout bytes differ")
    elif case.stdout_kind not in ("raw", "empty") and case.expected is not None:
        if semantic_jsonable(normalized) != semantic_jsonable(case.expected):
            issues.append("semantic stdout differs from the contract")
    if case.validator:
        issues.extend(validate_shape(case.validator, normalized))
    try:
        stderr_text = observation.stderr.decode("utf-8")
    except UnicodeDecodeError as exc:
        issues.append(f"stderr is not UTF-8: {exc}")
        stderr_text = ""
    if case.stderr_empty and observation.stderr:
        issues.append("stderr is not empty")
    for fragment in case.stderr_contains:
        if fragment not in stderr_text:
            issues.append(f"stderr lacks {fragment!r}")
    if case.expected_exit == 0 and "error:" in stderr_text.lower():
        issues.append("successful case emitted an error diagnostic")
    if case.expected_exit == 0 and "supports headings only" in stderr_text:
        issues.append("heading-only serializer sentinel remains")
    return ("Preserved" if not issues else "Violated"), normalized, issues


def observation_record(case: Case, obs: Observation, normalized: Any) -> dict[str, Any]:
    return {
        "args": list(case.args),
        "stdin_sha256": sha256_bytes(case.stdin),
        "stdout_kind": case.stdout_kind,
        "exit_code": obs.exit_code,
        "stdout_sha256": sha256_bytes(obs.stdout),
        "stderr_sha256": sha256_bytes(obs.stderr),
        "stdout_base64": base64.b64encode(obs.stdout).decode("ascii"),
        "stderr_base64": base64.b64encode(obs.stderr).decode("ascii"),
        "semantic_stdout": semantic_jsonable(normalized),
    }


def check_relations(observations: dict[str, tuple[Observation, Any]]) -> list[str]:
    issues: list[str] = []
    semantic_groups = (
        ("eval_composite_json", "eval_composite_yaml", "eval_composite_yaml_pretty"),
        ("query_figure_json", "query_figure_yaml", "query_label_figure_json"),
        ("query_metadata_json", "query_metadata_yaml"),
    )
    for group in semantic_groups:
        values = [semantic_jsonable(observations[item][1]) for item in group]
        if any(value != values[0] for value in values[1:]):
            issues.append(f"semantic relation differs: {', '.join(group)}")
    yaml_plain = observations["eval_composite_yaml"][0].stdout
    yaml_pretty = observations["eval_composite_yaml_pretty"][0].stdout
    if yaml_plain != yaml_pretty:
        issues.append("--pretty changed YAML bytes")
    return issues


def load_baseline(path: Path, suite_sha: str) -> dict[str, Any]:
    try:
        data = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as exc:
        raise HarnessUnknown(f"cannot load baseline: {exc}") from exc
    if data.get("status") != "FROZEN":
        raise HarnessUnknown("baseline status is not FROZEN")
    if data.get("source_pin") != SOURCE_PIN:
        raise HarnessUnknown("baseline source pin differs")
    if data.get("suite_sha256") != suite_sha:
        raise HarnessUnknown("suite changed after baseline freeze")
    return data


def write_json_atomic(path: Path, value: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    payload = json.dumps(value, ensure_ascii=False, indent=2, sort_keys=True, allow_nan=False) + "\n"
    with tempfile.NamedTemporaryFile("w", encoding="utf-8", dir=path.parent, delete=False) as handle:
        handle.write(payload)
        temp_path = Path(handle.name)
    os.replace(temp_path, path)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--typst", type=Path, required=True, help="binary to exercise")
    parser.add_argument("--baseline", type=Path, default=DEFAULT_BASELINE)
    parser.add_argument("--freeze", action="store_true", help="freeze the ratified vanilla baseline")
    args = parser.parse_args()

    suite_path = Path(__file__).resolve()
    suite_sha = sha256_file(suite_path)
    binary = args.typst.resolve()
    if not binary.is_file():
        print(f"P1285 ORACLES: Unknown: binary not found: {binary}", file=sys.stderr)
        return 2
    binary_sha = sha256_file(binary)
    if args.freeze and (binary != RATIFIED_BINARY or binary_sha != RATIFIED_BINARY_SHA256):
        print("P1285 ORACLES: Unknown: freeze requires the pinned /usr/local/bin/typst", file=sys.stderr)
        return 2

    baseline: dict[str, Any] | None = None
    if not args.freeze:
        try:
            baseline = load_baseline(args.baseline, suite_sha)
        except HarnessUnknown as exc:
            print(f"P1285 ORACLES: Unknown: {exc}", file=sys.stderr)
            return 2

    first: dict[str, tuple[Observation, Any]] = {}
    statuses: dict[str, dict[str, Any]] = {}
    for pass_index, ordered in enumerate((CASES, tuple(reversed(CASES))), start=1):
        for case in ordered:
            obs = run_case(binary, case)
            verdict, normalized, issues = judge(case, obs)
            if pass_index == 1:
                first[case.case_id] = (obs, normalized)
                statuses[case.case_id] = {"verdict": verdict, "issues": issues}
                if baseline is not None and verdict == "Preserved":
                    frozen = baseline.get("cases", {}).get(case.case_id)
                    if frozen is None:
                        statuses[case.case_id] = {"verdict": "Unknown", "issues": ["case absent from baseline"]}
                    else:
                        if obs.exit_code != frozen.get("exit_code"):
                            statuses[case.case_id]["issues"].append("exit differs from baseline")
                        if semantic_jsonable(normalized) != frozen.get("semantic_stdout"):
                            statuses[case.case_id]["issues"].append("semantic stdout differs from baseline")
                        if statuses[case.case_id]["issues"]:
                            statuses[case.case_id]["verdict"] = "Violated"
            else:
                prior_obs, prior_normalized = first[case.case_id]
                if obs.runtime_error:
                    statuses[case.case_id] = {"verdict": "Unknown", "issues": [obs.runtime_error]}
                elif (
                    obs.exit_code != prior_obs.exit_code
                    or obs.stdout != prior_obs.stdout
                    or obs.stderr != prior_obs.stderr
                    or semantic_jsonable(normalized) != semantic_jsonable(prior_normalized)
                ):
                    statuses[case.case_id]["verdict"] = "Violated"
                    statuses[case.case_id]["issues"].append("second reversed-order run was not deterministic")

    relation_issues = check_relations(first)
    preserved = sum(item["verdict"] == "Preserved" for item in statuses.values())
    violated = sum(item["verdict"] == "Violated" for item in statuses.values())
    unknown = sum(item["verdict"] == "Unknown" for item in statuses.values())

    if args.freeze and violated == 0 and unknown == 0 and not relation_issues:
        snapshot = {
            "schema": 1,
            "status": "FROZEN",
            "source_pin": SOURCE_PIN,
            "created_at_utc": datetime.now(timezone.utc).isoformat(),
            "binary_path": str(binary),
            "binary_sha256": binary_sha,
            "suite_sha256": suite_sha,
            "policy": {
                "unknown_is_success": False,
                "structured_comparison": "semantic typed tree",
                "raw_comparison": "exact bytes",
                "streams": "stdout/stderr/exit separate",
                "repetition": "two passes; second in reverse order",
            },
            "cases": {
                case.case_id: observation_record(case, first[case.case_id][0], first[case.case_id][1])
                for case in CASES
            },
        }
        write_json_atomic(args.baseline, snapshot)
        print(f"P1285 ORACLES: FROZEN ({preserved}/{len(CASES)} Preserved)")
        print(f"baseline_sha256={sha256_file(args.baseline)}")
        print(f"suite_sha256={suite_sha}")
        print(f"binary_sha256={binary_sha}")
        return 0

    for case_id, item in statuses.items():
        if item["verdict"] != "Preserved":
            print(f"{item['verdict']} {case_id}: {'; '.join(item['issues'])}", file=sys.stderr)
    for issue in relation_issues:
        print(f"Violated relation: {issue}", file=sys.stderr)
    final = "Preserved" if violated == 0 and unknown == 0 and not relation_issues else "FAILED"
    print(f"P1285 ORACLES: {final} (Preserved={preserved}, Violated={violated}, Unknown={unknown})")
    return 0 if final == "Preserved" else 1


if __name__ == "__main__":
    raise SystemExit(main())
