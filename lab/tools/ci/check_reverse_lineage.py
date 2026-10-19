#!/usr/bin/env python3
"""Read-only reverse seal gate. Forward seals, ownership and nuclei use the linter."""
import hashlib
from pathlib import Path
import re
import sys
import tomllib


def check_pair(root: Path, source: Path) -> list[str]:
    try:
        root = root.resolve()
        relative = source.relative_to(root)
        if any((root / parent).is_symlink() for parent in [relative, *relative.parents]):
            return [f'{relative}: source symlink is not accepted']
        raw = source.read_bytes()
        header = []
        for line in raw.splitlines(keepends=True):
            if not line.startswith(b'//!'):
                break
            header.append(line)
        owners = [line[12:].strip().decode('utf-8') for line in header if line.startswith(b'//! @prompt ')]
        seals = [line for line in header if line.startswith(b'//! @prompt-hash ')]
        if len(owners) != 1 or len(seals) != 1:
            return [f'{relative}: expected one owner and one forward seal in header']
        if not re.fullmatch(rb'//! @prompt-hash [0-9a-f]{8}\r?\n?', seals[0]):
            return [f'{relative}: malformed forward seal']
        owner = Path(owners[0])
        if owner.is_absolute() or '..' in owner.parts or not owners[0].startswith('00_nucleo/prompts/') or '_nuclei' in owner.parts:
            return [f'{relative}: owner must be a confined materializable L0']
        if any((root / parent).is_symlink() for parent in [owner, *owner.parents]):
            return [f'{relative}: owner symlink is not accepted']
        preamble = []
        for line in (root / owner).read_text(encoding='utf-8').splitlines():
            if not line.strip() or line.lstrip().startswith(('```', '~~~')):
                break
            preamble.append(line)
        metadata = [line for line in preamble if line.startswith('Hash do Código:')]
        if len(metadata) != 1 or not re.fullmatch(r'Hash do Código: [0-9a-f]{8}', metadata[0]):
            return [f'{owner}: expected one canonical reverse seal in preamble']
        # Only the leading seal is excluded; similarly named body text remains hashed.
        digest = hashlib.sha256(raw.replace(seals[0], b'', 1)).hexdigest()[:8]
        if metadata[0] != f'Hash do Código: {digest}':
            return [f'{relative}: reverse hash mismatch in {owner}: expected {digest}']
        return []
    except (OSError, UnicodeError, ValueError) as error:
        return [f'{source}: {error}']


def main() -> int:
    root = Path(__file__).resolve().parents[3]
    config = tomllib.loads((root / 'crystalline.toml').read_text())
    excluded = {Path(p) for p in config.get('excluded_files', {}).values()}
    errors = []
    checked = 0
    for layer in ('L1', 'L2', 'L3', 'L4'):
        directory = root / config['layers'][layer]
        if not directory.is_dir():
            errors.append(f'missing layer: {directory}')
            continue
        for source in sorted(directory.rglob('*.rs')):
            if source.relative_to(root) in excluded:
                continue
            checked += 1
            errors.extend(check_pair(root, source))
    for error in errors:
        print(error, file=sys.stderr)
    print(f'Reverse lineage: {checked} consumers, {len(errors)} errors')
    return 1 if errors or checked == 0 else 0


if __name__ == '__main__':
    sys.exit(main())
