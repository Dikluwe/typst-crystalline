"""Shared path checks for quarantine-only overlays. Does not execute patches."""
from pathlib import Path

RAM = Path('/dev/shm/tekt-joint-witness-20261106')
HERE = Path(__file__).resolve().parent


def prepared_workspace(argument):
    original = Path(argument).absolute()
    workspace = original.resolve()
    if not RAM.is_dir() or not workspace.is_dir():
        raise ValueError('Exclusive RAM area and prepared workspace must exist')
    ram = RAM.resolve()
    if original != workspace or not workspace.is_relative_to(ram) or workspace == ram:
        raise ValueError('Only a separate, non-symlink child workspace in task RAM is allowed')
    first = workspace.relative_to(ram).parts[0]
    if first.lower().startswith(('frozen', 'baseline')) or first == 'R':
        raise ValueError('Frozen/reference workspace is immutable')
    return workspace


def checked(workspace, relative):
    path = workspace / relative
    if not path.resolve().is_relative_to(workspace):
        raise ValueError(f'Overlay escapes workspace: {relative}')
    current = path
    while current != workspace:
        if current.is_symlink():
            raise ValueError(f'Symlink in overlay path: {relative}')
        current = current.parent
    return path


def replace_once(text, old, new):
    if text.count(old) != 1:
        raise ValueError(f'Expected exactly one unmodified anchor: {old[:120]!r}')
    return text.replace(old, new, 1)


class Edits:
    def __init__(self, workspace):
        self.workspace = workspace
        self.contents = {}

    def replace(self, relative, old, new):
        path = checked(self.workspace, relative)
        text = self.contents[path] if path in self.contents else path.read_text()
        self.contents[path] = replace_once(text, old, new)

    def new(self, relative, content):
        path = checked(self.workspace, relative)
        if path.exists():
            raise ValueError(f'Overlay helper already exists: {relative}')
        self.contents[path] = content

    def publish(self):
        # Validate every path again before the first write. No build/test runs.
        for path in self.contents:
            checked(self.workspace, path.relative_to(self.workspace))
        for path, content in self.contents.items():
            path.write_text(content)
