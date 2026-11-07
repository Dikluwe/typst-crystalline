#!/usr/bin/env python3
"""Apply the unchanged nominal overlay only to a prepared new RAM workspace."""
import importlib.util
from pathlib import Path
import sys

HERE = Path(__file__).resolve().parent
RAM = Path('/dev/shm/tekt-stdlib-cli-residual-20261106')


def load(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    saved = sys.dont_write_bytecode
    sys.dont_write_bytecode = True
    try:
        spec.loader.exec_module(module)
    finally:
        sys.dont_write_bytecode = saved
    return module


def main():
    previous = load('cli_residual_previous_apply', HERE.parent / 'stdlib-qualification/apply.py')
    previous.RAM = RAM
    previous.main()


if __name__ == '__main__':
    main()
