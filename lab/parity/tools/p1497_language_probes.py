#!/usr/bin/env python3
"""P1497: bilateral language probes, with explicit target and feature profile.

The observations are deliberately narrower than whole-product parity. In
particular, Bash completion is checked for a registered, valid script, not
byte equality; HTML/CSS and SVG rendering get no parity credit while the
crystalline target/profile intentionally does not support them.
"""

from __future__ import annotations

import argparse
from datetime import datetime, timezone
import hashlib
from html.parser import HTMLParser
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile


ROOT = Path(__file__).resolve().parents[3]
VANILLA_SHA256 = "22897f93c7c2c8589cf5913b0ce735f1c01ba711c70f55c7a350f7c4b96c0b34"
MATH_FIXTURE = ROOT / "lab/parity/matrix/fixtures/math-match.typ"
PLAIN_FIXTURE = ROOT / "lab/parity/matrix/fixtures/plain.typ"
SVG_XML = (
    '<svg xmlns="http://www.w3.org/2000/svg" width="8" height="8">'
    '<foreignObject width="8" height="8"><div xmlns="http://www.w3.org/1999/xhtml">'
    'X</div></foreignObject></svg>'
)
SVG_SOURCE = f'#image(bytes({json.dumps(SVG_XML)}), format: "svg")\n'
INVALID_SVG_SOURCE = '#image(bytes("not-an-image"), format: "svg")\n'

CASES = (
    ("fonts.empty-path", "cli", "default", "fonts", ("fonts", "--font-path", "", "--ignore-system-fonts")),
    ("fonts.no-path-control", "cli", "default", "fonts", ("fonts", "--ignore-system-fonts")),
    ("completions.nushell", "cli", "default", "nushell", ("completions", "nushell")),
    ("completions.bash-control", "cli", "default", "bash", ("completions", "bash")),
    ("stroke.default-roundtrip", "eval", "default", "repr(stroke(eval(repr(stroke()))))", ()),
    ("stroke.explicit-roundtrip", "eval", "default", "repr(stroke(eval(repr(stroke(thickness: 2pt, paint: red)))))", ()),
    ("stroke.named-control", "eval", "default", "repr(stroke(thickness: 2pt, paint: red))", ()),
    ("stroke.invalid-control", "eval", "default", "repr(stroke(42))", ()),
    ("html.mathml", "compile", "html", "mathml", ()),
    ("html.plain-control", "compile", "html", "plain", ()),
    ("svg.foreign-object", "compile", "pdf/svg-source", "svg", ()),
    ("svg.invalid-bytes-control", "compile", "pdf/svg-source", "svg-invalid", ()),
)


class ProbeError(Exception):
    """A measurement cannot safely be interpreted."""


def sha256_bytes(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as source:
        for chunk in iter(lambda: source.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def utc_now() -> str:
    return datetime.now(timezone.utc).isoformat(timespec="seconds")


def git_state() -> dict:
    def read(*args: str) -> str:
        result = subprocess.run(
            ("git", *args), cwd=ROOT, capture_output=True, text=True, check=False,
        )
        if result.returncode != 0:
            raise ProbeError(f"git {' '.join(args)} failed: {result.stderr.strip()}")
        return result.stdout.rstrip("\n")

    return {
        "head": read("rev-parse", "HEAD"),
        "status_short": read("status", "--short", "--untracked-files=all").splitlines(),
        "diff_head_stat": read("diff", "HEAD", "--stat").splitlines(),
    }


def stable_environment() -> dict[str, str]:
    env = os.environ.copy()
    for key in ("TYPST_FONT_PATHS", "TYPST_IGNORE_SYSTEM_FONTS", "TYPST_IGNORE_EMBEDDED_FONTS"):
        env.pop(key, None)
    env.update(NO_COLOR="1", LC_ALL="C")
    return env


def probe_scratch_dir():
    """Keep generated sources and exports outside the repository, even with TMPDIR set."""
    return tempfile.TemporaryDirectory(prefix="p1497-language-", dir="/tmp")


def execute(argv: list[str], *, env: dict[str, str]) -> subprocess.CompletedProcess[bytes]:
    try:
        return subprocess.run(argv, cwd=ROOT, env=env, capture_output=True, timeout=45, check=False)
    except (OSError, subprocess.TimeoutExpired) as error:
        raise ProbeError(f"cannot execute {argv[0]}: {error}") from error


def diagnostic(stderr: str, kind: str) -> str | None:
    prefix = f"{kind}: "
    return next((line[len(prefix):] for line in stderr.splitlines() if line.startswith(prefix)), None)


class BodyTokens(HTMLParser):
    def __init__(self) -> None:
        super().__init__(convert_charrefs=True)
        self.depth = 0
        self.body_seen = False
        self.tokens: list[str] = []

    def handle_starttag(self, tag: str, attrs: list[tuple[str, str | None]]) -> None:
        if tag == "body":
            self.depth = 1
            self.body_seen = True
        elif self.depth:
            self.tokens.append(f"<{tag}>")
            self.depth += 1

    def handle_endtag(self, tag: str) -> None:
        if self.depth and tag == "body" and self.depth == 1:
            self.depth = 0
        elif self.depth:
            self.tokens.append(f"</{tag}>")
            self.depth -= 1

    def handle_data(self, data: str) -> None:
        if self.depth and data.strip():
            self.tokens.append(" ".join(data.split()))


def observe(case_id: str, completed: subprocess.CompletedProcess[bytes], output: Path | None) -> dict:
    stdout = completed.stdout.decode("utf-8", errors="replace")
    stderr = completed.stderr.decode("utf-8", errors="replace")
    # Temporary source/output paths are incidental and change on every run.
    stderr_for_hash = (
        completed.stderr.replace(str(output.parent).encode(), b"<TEMP>")
        if output else completed.stderr
    )
    result: dict = {
        "exit": completed.returncode,
        "stdout_sha256": sha256_bytes(completed.stdout),
        "stderr_normalized_sha256": sha256_bytes(stderr_for_hash),
        "error": diagnostic(stderr, "error"),
        "warnings": [line[len("warning: "):] for line in stderr.splitlines() if line.startswith("warning: ")],
    }
    if case_id.startswith("fonts."):
        result["families"] = sorted(set(stdout.splitlines())) if completed.returncode == 0 else None
    elif case_id.startswith("completions."):
        result["line_count"] = len(stdout.splitlines())
        if case_id.endswith("bash-control") and completed.returncode == 0:
            syntax = subprocess.run(("bash", "-n"), input=completed.stdout, capture_output=True, check=False)
            result["bash_syntax_valid"] = syntax.returncode == 0
            result["bash_registered"] = "complete -F _typst" in stdout and "_typst() {" in stdout
        elif case_id.endswith("nushell") and completed.returncode == 0:
            result["nushell_entrypoint"] = "module completions {" in stdout and "export extern typst" in stdout
    elif case_id.startswith("stroke."):
        if completed.returncode == 0:
            try:
                result["value"] = json.loads(stdout)
            except json.JSONDecodeError as error:
                raise ProbeError(f"{case_id}: eval did not return JSON") from error
        else:
            result["value"] = None
    elif case_id.startswith("html."):
        html = output.read_text(encoding="utf-8") if output and output.is_file() else ""
        parser = BodyTokens()
        parser.feed(html)
        result["artifact_present"] = bool(html)
        result["body_seen"] = parser.body_seen
        result["body_tokens"] = parser.tokens if completed.returncode == 0 else None
        result["has_mathml"] = "<math>" in parser.tokens if completed.returncode == 0 else None
        result["artifact_sha256"] = sha256_bytes(html.encode("utf-8")) if html else None
    elif case_id.startswith("svg."):
        result["pdf_emitted"] = bool(output and output.is_file() and output.stat().st_size)
    return result


def verdict(case_id: str, vanilla: dict, crystal: dict) -> tuple[str, str]:
    ve, ce = vanilla["exit"], crystal["exit"]
    if case_id.startswith("fonts."):
        good = ve == ce == 0 and vanilla["families"] == crystal["families"]
        return ("MATCH", "same font-family set") if good else ("VIOLATED", "font CLI acceptance or family set differs")
    if case_id == "completions.bash-control":
        good = ve == ce == 0 and all(
            side.get("bash_syntax_valid") and side.get("bash_registered") for side in (vanilla, crystal)
        )
        return ("MATCH", "both generate registered, syntactically valid Bash; content is not compared") if good else ("VIOLATED", "Bash completion generation failed")
    if case_id == "completions.nushell":
        good = ve == ce == 0 and vanilla.get("nushell_entrypoint") and crystal.get("nushell_entrypoint")
        return ("MATCH", "both expose Nushell entrypoint; completion content is not compared") if good else ("VIOLATED", "Nushell completion acceptance differs")
    if case_id == "stroke.invalid-control":
        if ve != 0 and ce != 0:
            return "MATCH", "both reject invalid stroke input; diagnostic wording is not compared"
        return "VIOLATED", "invalid stroke input was accepted by at least one product"
    if case_id.startswith("stroke."):
        if ve == ce == 0 and vanilla["value"] == crystal["value"] and vanilla["warnings"] == crystal["warnings"]:
            return "MATCH", "same language-level repr value and warnings"
        return "VIOLATED", "positive stroke probe requires successful evaluation with matching repr and warnings"
    if case_id == "html.plain-control":
        good = (
            ve == ce == 0
            and all(side.get("artifact_present") and side.get("body_seen") and side.get("body_tokens") for side in (vanilla, crystal))
            and vanilla["body_tokens"] == crystal["body_tokens"]
        )
        return ("MATCH", "same body tags and text; attributes and bytes not compared") if good else ("VIOLATED", "plain HTML artifact, body, tags, or text differs")
    if case_id == "html.mathml":
        if ve != 0 or not vanilla.get("has_mathml"):
            raise ProbeError("vanilla MathML oracle did not emit MathML")
        if ce != 0 and crystal["error"] == "HTML export does not yet support this content":
            return "DISABLED_BY_PROFILE", "MathML/CSS excluded by infra/export/html.md:37-38; no CSS parity credit"
        if ce == 0:
            raise ProbeError("MathML enabled: body/CSS rendering needs a new semantic observer")
        return "VIOLATED", "HTML failed for a reason outside the declared MathML scope-out"
    if case_id.startswith("svg."):
        disabled = ce != 0 and crystal["error"] in (
            "argumento nomeado inesperado em image(): 'format'",
            "SVG images are not supported yet",
        )
        if case_id == "svg.foreign-object":
            if ve != 0 or not vanilla["pdf_emitted"] or "image contains foreign object" not in vanilla["warnings"]:
                raise ProbeError("vanilla SVG oracle did not render and warn as expected")
            if disabled:
                return "DISABLED_BY_PROFILE", "SVG-source support is DEBT-67; no warning/render parity credit"
            if ce == 0:
                raise ProbeError("SVG enabled: semantic rendering needs a new observer")
            return "VIOLATED", "SVG rejected outside the declared scope-out"
        if ve == 0 or vanilla["error"] is None:
            raise ProbeError("invalid SVG control did not fail in vanilla")
        if disabled:
            return "DISABLED_BY_PROFILE", "invalid SVG cannot compare decoder errors while SVG source is disabled"
        if ce != 0 and crystal["error"] == vanilla["error"]:
            return "MATCH", "same invalid input and exact primary error"
        return "VIOLATED", "invalid SVG acceptance or exact primary error differs"
    raise ProbeError(f"unknown case: {case_id}")


def run_case(spec: tuple, vanilla_bin: Path, crystal_bin: Path, temp: Path, env: dict[str, str]) -> dict:
    case_id, kind, profile, source, args = spec
    fixture = MATH_FIXTURE if source == "mathml" else PLAIN_FIXTURE
    source_text = SVG_SOURCE if source == "svg" else INVALID_SVG_SOURCE
    source_file = temp / f"{case_id}.typ"
    if case_id.startswith("svg."):
        source_file.write_text(source_text, encoding="utf-8")
    sides = {}
    for side, binary in (("vanilla", vanilla_bin), ("crystalline", crystal_bin)):
        output = temp / f"{case_id}-{side}.{'html' if profile == 'html' else 'pdf'}"
        if kind == "cli":
            command = [str(binary), *args]
            template = [binary.name, *args]
            output = None
        elif kind == "eval":
            command = [str(binary), "eval", source, "--format", "json"]
            template = [binary.name, "eval", source, "--format", "json"]
            output = None
        else:
            input_path = fixture if profile == "html" else source_file
            command = [str(binary), "compile"]
            if profile == "html":
                command += ["--features", "html", "--format", "html"]
            else:
                command += ["--format", "pdf"]
            command += [str(input_path)]
            if side == "vanilla":
                command += [str(output)]
            else:
                command += ["-o", str(output)]
            template = [binary.name, *command[1:]]
            template = ["<temporary-source>" if item == str(source_file) else "<temporary-output>" if item == str(output) else item for item in template]
        completed = execute(command, env=env)
        sides[side] = observe(case_id, completed, output)
        sides[side]["argv_template"] = template
    status, reason = verdict(case_id, sides["vanilla"], sides["crystalline"])
    return {
        "id": case_id,
        "target": "html" if profile == "html" else "pdf" if profile == "pdf/svg-source" else kind,
        "profile": profile,
        "required_features": ["html"] if profile == "html" else [],
        "source": str(fixture.relative_to(ROOT)) if profile == "html" else source_text if profile == "pdf/svg-source" else source,
        "source_sha256": sha256_bytes(source_text.encode("utf-8")) if profile == "pdf/svg-source" else None,
        "status": status,
        "reason": reason,
        **sides,
    }


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--vanilla", type=Path, default=ROOT / "lab/typst-original/target/release/typst")
    parser.add_argument("--crystalline", type=Path, default=ROOT / "target/release/typst")
    args = parser.parse_args(argv)
    started = utc_now()
    try:
        vanilla_bin, crystal_bin = args.vanilla.resolve(), args.crystalline.resolve()
        before = git_state()
        hashes = {"vanilla": sha256_file(vanilla_bin), "crystalline": sha256_file(crystal_bin)}
        if hashes["vanilla"] != VANILLA_SHA256:
            raise ProbeError("vanilla binary is not the ratified 64c5e8c0 build (2026-10-26)")
        env = stable_environment()
        with probe_scratch_dir() as directory:
            cases = [run_case(spec, vanilla_bin, crystal_bin, Path(directory), env) for spec in CASES]
        after = git_state()
        if hashes != {"vanilla": sha256_file(vanilla_bin), "crystalline": sha256_file(crystal_bin)}:
            raise ProbeError("a binary changed during the probe")
        if before != after:
            raise ProbeError("working tree changed during the probe; rerun on a stable snapshot")
        counts = {status: sum(case["status"] == status for case in cases) for status in ("MATCH", "VIOLATED", "DISABLED_BY_PROFILE")}
        print(json.dumps({
            "schema": "p1497-language-probes-v1", "started_utc": started, "finished_utc": utc_now(),
            "provenance": {"git": before, "binary_sha256": hashes, "environment_overrides": {"NO_COLOR": "1", "LC_ALL": "C", "TYPST_FONT_PATHS": None, "TYPST_IGNORE_SYSTEM_FONTS": None, "TYPST_IGNORE_EMBEDDED_FONTS": None}},
            "counts": counts, "cases": cases,
        }, ensure_ascii=False, sort_keys=True))
        return 1 if counts["VIOLATED"] else 0
    except (OSError, ProbeError) as error:
        print(json.dumps({"schema": "p1497-language-probes-v1", "error": str(error), "started_utc": started, "finished_utc": utc_now()}, ensure_ascii=False, sort_keys=True))
        return 2


if __name__ == "__main__":
    sys.exit(main())
