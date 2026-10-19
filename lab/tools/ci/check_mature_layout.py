#!/usr/bin/env python3
"""Check the repository's mature root/lab layout and active local links.

An optional exact exception ledger can be passed with ``--known-broken-links``.
It is a UTF-8 TSV with ``source-path<TAB>line<TAB>target`` rows relative to the
repository root. Comments start with ``#``. Exceptions are exact and stale rows
are errors, so the ledger cannot hide newly introduced or repaired links.
Inline-code path references and fenced code examples are not navigable Markdown
links and are intentionally excluded.
"""

from __future__ import annotations

import argparse
from dataclasses import dataclass
import json
import re
import sys
import tomllib
from pathlib import Path
from urllib.parse import unquote, urlsplit


OLD_ROOT_PATHS = (
    "benches",
    "tools",
    "target",
    ".typ",
    "00_nucleo/materialization",
    "00_nucleo/diagnosticos",
    "00_nucleo/corpus-docs",
)
PRODUCT_DIRS = ("01_core", "02_shell", "03_infra", "04_wiring")
INCLUDE_SCAN_DIRS = PRODUCT_DIRS + ("lab/benches",)
LINK_RE = re.compile(
    r"!?\[([^\]]*)\]\(\s*(<[^>]+>|(?:\\.|[^\s)])+)(?:\s+[^)]*)?\)",
    re.MULTILINE,
)
LINK_DEFINITION_RE = re.compile(
    r"^\s{0,3}\[[^\]]+\]:\s*(<[^>]+>|\S+)(?:\s+[^\n]*)?$",
    re.MULTILINE,
)
OWNER_RE = re.compile(r"^\s*//!?\s*@prompt\s+(\S+)\s*$")
PATH_ATTR_RE = re.compile(r"#\s*\[\s*path\s*=\s*\"([^\"]+\.rs)\"\s*\]")
INCLUDE_RE = re.compile(r"\binclude!\s*\(\s*\"([^\"]+\.rs)\"\s*\)")
INCLUDE_LITERAL_RE = re.compile(
    r'\b(include_str|include_bytes)\s*!\s*\(\s*(?P<literal>"(?:\\.|[^"\\])*"|r(?P<hashes>#{0,})"(?P<body>.*?)"(?P=hashes))\s*\)',
    re.DOTALL,
)
DYNAMIC_INCLUDE_RE = re.compile(r"\b(include_str|include_bytes)\s*!\s*\(")


@dataclass(frozen=True)
class Finding:
    code: str
    source: str
    message: str
    line: int | None = None

    def __str__(self) -> str:
        location = f"{self.source}:{self.line}" if self.line is not None else self.source
        return f"[{self.code}] {location}: {self.message}"


def _relative(root: Path, path: Path) -> str:
    try:
        return path.relative_to(root).as_posix()
    except ValueError:
        return path.as_posix()


def _strip_fenced_code(text: str) -> str:
    lines = text.splitlines(keepends=True)
    kept: list[str] = []
    fence_char: str | None = None
    fence_len = 0
    for line in lines:
        match = re.match(r"^\s*(`{3,}|~{3,})", line)
        if fence_char is None:
            if match:
                fence_char = match.group(1)[0]
                fence_len = len(match.group(1))
                kept.append("\n" if line.endswith("\n") else "")
            else:
                kept.append(line)
            continue
        if match and match.group(1)[0] == fence_char and len(match.group(1)) >= fence_len:
            fence_char = None
            fence_len = 0
        kept.append("\n" if line.endswith("\n") else "")
    return "".join(kept)


def _active_markdown(text: str) -> str:
    text = _strip_fenced_code(text)
    # A path written as inline code documents a literal reference; only a
    # Markdown link creates a navigable filesystem link for this check.
    return re.sub(r"(`+)(.+?)\1", lambda m: " " * len(m.group(0)), text, flags=re.DOTALL)


def _local_target(root: Path, source: Path, raw_target: str) -> Path | None:
    target = raw_target[1:-1] if raw_target.startswith("<") and raw_target.endswith(">") else raw_target
    target = target.replace("\\(", "(").replace("\\)", ")")
    parts = urlsplit(target)
    if parts.scheme or parts.netloc or not parts.path:
        return None
    path_text = unquote(parts.path)
    candidate = root / path_text.lstrip("/") if path_text.startswith("/") else source.parent / path_text
    return candidate


def load_link_ledger(path: Path | None) -> tuple[set[tuple[str, int, str]], list[Finding]]:
    if path is None:
        return set(), []
    entries: set[tuple[str, int, str]] = set()
    errors: list[Finding] = []
    try:
        lines = path.read_text(encoding="utf-8").splitlines()
    except OSError as error:
        return set(), [Finding("LINK_LEDGER_READ", path.as_posix(), str(error))]
    for number, raw in enumerate(lines, 1):
        if not raw.strip() or raw.lstrip().startswith("#"):
            continue
        fields = raw.split("\t")
        if len(fields) != 3 or not fields[1].isdigit() or int(fields[1]) < 1:
            errors.append(Finding("LINK_LEDGER_FORMAT", path.as_posix(), "expected source-path<TAB>line<TAB>target", number))
            continue
        entries.add((fields[0], int(fields[1]), fields[2]))
    return entries, errors


def _markdown_files(root: Path) -> list[Path]:
    files = set(root.glob("*.md"))
    files.update(
        path
        for path in (
            root / "00_nucleo" / "README.md",
            root / "lab" / "README.md",
            root / "lab" / "history" / "README.md",
        )
        if path.is_file()
    )
    directories = (
        root / "00_nucleo" / "adr",
        root / "00_nucleo" / "prompts",
        root / "00_nucleo" / "diagnostics",
        root / "lab" / "tools",
    )
    for directory in directories:
        if directory.exists():
            files.update(directory.rglob("*.md"))
    # AGENTS.md may be a symlink to CLAUDE.md; check its target once.
    return sorted({path.resolve() for path in files if path.is_file()})


def check_markdown_links(root: Path, ledger: set[tuple[str, int, str]]) -> tuple[list[Finding], set[tuple[str, int, str]]]:
    findings: list[Finding] = []
    consumed: set[tuple[str, int, str]] = set()
    for source in _markdown_files(root):
        try:
            text = source.read_text(encoding="utf-8")
        except (OSError, UnicodeError) as error:
            findings.append(Finding("MARKDOWN_READ", _relative(root, source), str(error)))
            continue
        active = _active_markdown(text)
        matches = [(match.start(), match.group(2)) for match in LINK_RE.finditer(active)]
        matches.extend((match.start(), match.group(1)) for match in LINK_DEFINITION_RE.finditer(active))
        for start, raw_target in matches:
            raw_target = raw_target.strip()
            candidate = _local_target(root, source, raw_target)
            if candidate is None:
                continue
            source_name = _relative(root, source)
            line = active.count("\n", 0, start) + 1
            target_text = raw_target[1:-1] if raw_target.startswith("<") and raw_target.endswith(">") else raw_target
            key = (source_name, line, target_text)
            try:
                candidate.resolve().relative_to(root)
            except ValueError:
                findings.append(Finding("MARKDOWN_LINK_ESCAPES_ROOT", source_name, f"local Markdown link escapes the repository: {target_text}", line))
                continue
            if candidate.exists():
                continue
            if key in ledger:
                consumed.add(key)
                continue
            findings.append(Finding("MARKDOWN_LINK_MISSING", source_name, f"local Markdown link does not resolve: {target_text}", line))
    return findings, consumed


def _has_lab_source_reference(root: Path, source: Path) -> list[str]:
    text = source.read_text(encoding="utf-8", errors="replace")
    references = []
    for pattern in (PATH_ATTR_RE, INCLUDE_RE):
        for match in pattern.finditer(text):
            candidate = (source.parent / match.group(1)).resolve()
            try:
                candidate.relative_to(root / "lab")
            except ValueError:
                continue
            references.append(match.group(1))
    return references


def check_product_lab_imports(root: Path) -> list[Finding]:
    findings: list[Finding] = []
    for crate in PRODUCT_DIRS:
        crate_root = root / crate
        for source in crate_root.rglob("*.rs"):
            for reference in _has_lab_source_reference(root, source):
                findings.append(Finding("PRODUCT_IMPORTS_LAB", _relative(root, source), f"Rust source inclusion resolves into lab/: {reference}"))
        manifest = crate_root / "Cargo.toml"
        if not manifest.exists():
            continue
        try:
            data = tomllib.loads(manifest.read_text(encoding="utf-8"))
        except (OSError, tomllib.TOMLDecodeError) as error:
            findings.append(Finding("CARGO_MANIFEST_READ", _relative(root, manifest), str(error)))
            continue
        def inspect_paths(value: object, key_path: str = "") -> None:
            if not isinstance(value, dict):
                return
            dependency_path = value.get("path")
            if isinstance(dependency_path, str):
                resolved = (manifest.parent / dependency_path).resolve()
                try:
                    resolved.relative_to(root / "lab")
                except ValueError:
                    pass
                else:
                    findings.append(Finding("PRODUCT_IMPORTS_LAB", _relative(root, manifest), f"{key_path} path resolves into lab/: {dependency_path}"))
            for key, child in value.items():
                if isinstance(child, dict):
                    inspect_paths(child, f"{key_path}.{key}" if key_path else key)

        inspect_paths(data)
    return findings


def check_compile_time_include_paths(root: Path) -> list[Finding]:
    """Check direct literal include_str!/include_bytes! paths relative to each Rust file.

    Macro expressions such as concat!(env!(...)) are deliberately not expanded:
    they are not reported as verified. The CLI reports how many such paths were
    skipped so a passing static check cannot imply that dynamic paths were checked.
    """
    findings: list[Finding] = []
    for source_dir in INCLUDE_SCAN_DIRS:
        for source in (root / source_dir).rglob("*.rs"):
            try:
                text = source.read_text(encoding="utf-8", errors="replace")
            except OSError as error:
                findings.append(Finding("INCLUDE_SOURCE_READ", _relative(root, source), str(error)))
                continue
            for match in INCLUDE_LITERAL_RE.finditer(text):
                macro = match.group(1)
                encoded_path = match.group("literal")
                line = text.count("\n", 0, match.start()) + 1
                if encoded_path.startswith("r"):
                    relative_path = match.group("body")
                else:
                    try:
                        relative_path = json.loads(encoded_path)
                    except json.JSONDecodeError:
                        findings.append(Finding("INCLUDE_PATH_LITERAL", _relative(root, source), f"cannot decode {macro}! path literal", line))
                        continue
                candidate = (source.parent / relative_path).resolve()
                try:
                    candidate.relative_to(root)
                except ValueError:
                    findings.append(Finding("INCLUDE_PATH_ESCAPES_ROOT", _relative(root, source), f"{macro}! path escapes the repository: {relative_path}", line))
                    continue
                if not candidate.is_file():
                    findings.append(Finding("INCLUDE_PATH_MISSING", _relative(root, source), f"{macro}! path does not resolve to a file: {relative_path}", line))
    return findings


def count_dynamic_include_paths(root: Path) -> int:
    """Count non-literal include macros that this path-only check cannot verify."""
    count = 0
    for source_dir in INCLUDE_SCAN_DIRS:
        for source in (root / source_dir).rglob("*.rs"):
            text = source.read_text(encoding="utf-8", errors="replace")
            literal_spans = [match.span() for match in INCLUDE_LITERAL_RE.finditer(text)]
            for match in DYNAMIC_INCLUDE_RE.finditer(text):
                if not any(start <= match.start() < end for start, end in literal_spans):
                    count += 1
    return count


def check_owner_paths(root: Path) -> list[Finding]:
    findings: list[Finding] = []
    owners: dict[str, list[str]] = {}
    for crate in PRODUCT_DIRS:
        for source in sorted((root / crate).rglob("*.rs")):
            try:
                lines = source.read_text(encoding="utf-8", errors="replace").splitlines()[:40]
            except OSError as error:
                findings.append(Finding("OWNER_SOURCE_READ", _relative(root, source), str(error)))
                continue
            for line in lines:
                match = OWNER_RE.match(line)
                if not match:
                    continue
                owner = match.group(1)
                if not owner.startswith("00_nucleo/prompts/") or ".." in Path(owner).parts or Path(owner).is_absolute():
                    findings.append(Finding("OWNER_PATH_NONCANONICAL", _relative(root, source), f"owner must use canonical 00_nucleo/prompts path: {owner}"))
                    continue
                if not (root / owner).is_file():
                    findings.append(Finding("OWNER_PATH_MISSING", _relative(root, source), f"owner prompt does not exist: {owner}"))
                    continue
                owners.setdefault(owner, []).append(_relative(root, source))
                break
    for owner, consumers in sorted(owners.items()):
        if len(consumers) > 1:
            findings.append(Finding("OWNER_DUPLICATE", owner, f"prompt has multiple productive consumers: {', '.join(consumers)}"))
    return findings


def check_layout(root: Path, known_broken_links: Path | None = None) -> list[Finding]:
    root = root.resolve()
    findings: list[Finding] = []

    for old_path in OLD_ROOT_PATHS:
        if (root / old_path).exists():
            findings.append(Finding("OLD_ROOT_PATH_PRESENT", old_path, "mature layout requires this path to be absent from the repository root"))

    agents = root / "AGENTS.md"
    if not agents.is_symlink():
        findings.append(Finding("AGENTS_LINK", "AGENTS.md", "must be a relative symlink to the repository's CLAUDE.md"))
    else:
        link = Path(agents.readlink())
        if link.is_absolute() or agents.resolve() != (root / "CLAUDE.md").resolve() or not agents.resolve().is_file():
            findings.append(Finding("AGENTS_LINK", "AGENTS.md", "must be a portable relative symlink resolving to CLAUDE.md"))

    config_path = root / ".cargo" / "config.toml"
    try:
        config = tomllib.loads(config_path.read_text(encoding="utf-8"))
        if config.get("build", {}).get("target-dir") != "lab/target/product":
            findings.append(Finding("CARGO_TARGET_DIR", ".cargo/config.toml", "[build].target-dir must equal lab/target/product"))
    except (OSError, tomllib.TOMLDecodeError) as error:
        findings.append(Finding("CARGO_CONFIG_READ", _relative(root, config_path), str(error)))

    workspace_path = root / "Cargo.toml"
    bench_manifest = root / "lab" / "benches" / "Cargo.toml"
    try:
        workspace = tomllib.loads(workspace_path.read_text(encoding="utf-8")).get("workspace", {})
        members = workspace.get("members", [])
        if "lab/benches" not in members or "benches" in members:
            findings.append(Finding("CARGO_BENCH_MEMBER", "Cargo.toml", "workspace must include lab/benches and exclude root benches"))
        package = tomllib.loads(bench_manifest.read_text(encoding="utf-8")).get("package", {})
        if package.get("workspace") != "../..":
            findings.append(Finding("CARGO_BENCH_WORKSPACE", "lab/benches/Cargo.toml", "package.workspace must explicitly point to ../.."))
    except (OSError, tomllib.TOMLDecodeError) as error:
        findings.append(Finding("CARGO_WORKSPACE_READ", _relative(root, workspace_path), str(error)))

    findings.extend(check_owner_paths(root))
    findings.extend(check_product_lab_imports(root))
    findings.extend(check_compile_time_include_paths(root))

    ledger, ledger_errors = load_link_ledger(known_broken_links)
    findings.extend(ledger_errors)
    link_findings, consumed = check_markdown_links(root, ledger)
    findings.extend(link_findings)
    for entry in sorted(ledger - consumed):
        source, line, target = entry
        findings.append(Finding("LINK_LEDGER_STALE", source, f"allowlisted link is no longer a broken local link: {target}", line))
    return findings


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("root", nargs="?", type=Path, default=Path(__file__).resolve().parents[3])
    parser.add_argument("--known-broken-links", type=Path, help="exact TSV exceptions for pre-existing broken active Markdown links")
    args = parser.parse_args(argv)
    findings = check_layout(args.root, args.known_broken_links)
    for finding in findings:
        print(f"ERROR {finding}", file=sys.stderr)
    if findings:
        print(f"mature layout check failed: {len(findings)} finding(s)", file=sys.stderr)
        return 1
    unchecked = count_dynamic_include_paths(args.root.resolve())
    if unchecked:
        print(f"mature layout check passed; {unchecked} dynamic include path(s) were not statically verified")
    else:
        print("mature layout check passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
