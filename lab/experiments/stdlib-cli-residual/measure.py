#!/usr/bin/env python3
"""CLI-only nominal/A-A comparisons using the unchanged publication clock."""
import argparse
import csv
import importlib.util
import itertools
from pathlib import Path
import shutil
import sys

HERE = Path(__file__).resolve().parent
TASK_RAM = Path('/dev/shm/tekt-stdlib-cli-residual-20261106')


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


def variant_order(names, round_id, repeat, fixture_index, repeats):
    """Cover every permutation once per factorial-sized block, without RNG."""
    permutations = list(itertools.permutations(names))
    index = (round_id * repeats + repeat + fixture_index) % len(permutations)
    return permutations[index]


def prepare_shared_root(driver, options, argument):
    original = Path(argument).absolute()
    shared = original.resolve()
    task = TASK_RAM.resolve()
    if (original != shared or shared.is_symlink() or shared.exists()
            or not shared.is_relative_to(task) or shared == task
            or not shared.parent.is_dir()):
        raise ValueError('shared-input-root must be a new non-symlink task RAM child')
    first = shared.relative_to(task).parts[0].lower()
    if first.startswith(('frozen', 'baseline', 'reference')) or first == 'r':
        raise ValueError('shared-input-root may not use a reference directory')
    for other in (options.ram_dir, options.package_cache, options.fixtures):
        if shared.is_relative_to(other) or other.is_relative_to(shared):
            raise ValueError('shared input root must not overlap output, cache or fixtures')
    shutil.copytree(options.fixtures, shared)
    for fixture in options.fixture:
        source = shared / fixture
        source.write_bytes(source.read_bytes() + driver.EPOCH_ZERO)
    return shared


def use_shared_root(driver, shared):
    previous_child = driver.Child
    previous_validation = driver.validate_inherited_packages

    def child(command, cwd, env, cpu, log):
        redirected = list(command)
        relative = Path(command[2]).relative_to(cwd)
        redirected[2] = str(shared / relative)
        redirected[redirected.index('--root') + 1] = str(shared)
        return previous_child(redirected, shared, env, cpu, log)

    def validate(env, _cwd):
        return previous_validation(env, shared)

    driver.Child = child
    driver.validate_inherited_packages = validate


def main():
    extras = argparse.ArgumentParser(add_help=False)
    extras.add_argument('--shared-input-root')
    custom, remaining = extras.parse_known_args()
    driver = load('cli_residual_previous_measure', HERE.parent / 'stdlib-qualification/measure.py')
    driver.TASK_RAM = TASK_RAM
    saved_arguments = sys.argv
    try:
        sys.argv = [sys.argv[0]] + remaining
        options = driver.parse_options()
    finally:
        sys.argv = saved_arguments
    if not 2 <= len(options.variants) <= 4:
        raise ValueError('Use two to four nominal/A-A labels')
    # This investigation always retains stock data-home fonts. The inherited
    # package-directory refusal is still enforced before each process clock.
    options.inherit_data_home = True
    if custom.shared_input_root is not None:
        shared = prepare_shared_root(driver, options, custom.shared_input_root)
        use_shared_root(driver, shared)
    options.ram_dir.mkdir(exist_ok=False)
    (options.ram_dir / 'data-home').mkdir()
    names = list(options.variants)
    for index, fixture in enumerate(options.fixture):
        for variant in names:
            driver.run_process(options, variant, fixture, index, 'cli', -1, 0, warmup=True)
    with (options.ram_dir / 'measurements.csv').open('x', newline='') as handle:
        writer = csv.DictWriter(handle, fieldnames=driver.FIELDS)
        writer.writeheader()
        handle.flush()
        count = len(options.fixture)
        for round_id in range(options.rounds):
            for repeat in range(options.repeats):
                offset = (round_id * options.repeats + repeat) % count
                indices = list(range(count))[offset:] + list(range(count))[:offset]
                for index in indices:
                    for variant in variant_order(names, round_id, repeat, index, options.repeats):
                        rows = driver.run_process(options, variant, options.fixture[index], index,
                                                  'cli', round_id, repeat)
                        writer.writerows(rows)
                        handle.flush()
            print(f'Completed CLI round {round_id + 1}/{options.rounds}', flush=True)
    print(options.ram_dir / 'measurements.csv', flush=True)


if __name__ == '__main__':
    main()
