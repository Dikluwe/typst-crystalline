#!/usr/bin/env python3
"""Paired CLI/watch measurements; every generated file stays in task RAM.

The CSV is incremental per completed process. Watch rows use process CPU-clock
deltas, while process_cpu_ns and process_peak_rss_bytes repeat the final wait4
totals for that process (they are not per-cycle values). Wall ends at a new,
stable PDF publication with EOF; it includes event/debounce latency. No source
or PDF extraction tool runs inside the measured intervals.
"""

import argparse
import csv
import os
from pathlib import Path
import resource
import select
import shutil
import signal
import subprocess
import time


TASK_RAM = Path('/dev/shm/tekt-outline-warm-20261106')
TIMEOUT = 60.0
SETTLE = 0.030
POLL = 0.001
STACK = 64 * 1024 * 1024
EPOCH_ZERO = b'\n#"TEKT_EPOCH_0"\n'
EPOCH_ONE = b'\n#"TEKT_EPOCH_1"\n'
FIELDS = [
    'round', 'repeat', 'variant', 'fixture', 'mode', 'cycle', 'process_id', 'case',
    'sample_epoch', 'wall_ns', 'cpu_ns', 'process_cpu_ns', 'process_user_ns',
    'process_system_ns', 'process_peak_rss_bytes', 'exit_code',
]


def child_setup(cpu):
    os.sched_setaffinity(0, {cpu})
    resource.setrlimit(resource.RLIMIT_STACK, (STACK, STACK))


def environment(temporary):
    result = {
        key: value for key, value in os.environ.items()
        if not key.startswith(('TYPST_', 'CRYSTALLINE_', 'CAT_PROBE',
                               'E_JOINT', 'TEKT_', 'CPU_PROFILE'))
        and key != 'LD_PRELOAD'
    }
    result.update(TMPDIR=str(temporary), RAYON_NUM_THREADS='1',
                  SOURCE_DATE_EPOCH='0', TZ='UTC', NO_COLOR='1')
    return result


def signature(stat):
    return stat.st_dev, stat.st_ino, stat.st_mtime_ns, stat.st_size


def published(path, previous):
    """Return a new, stable publication's identity, never an old/unlinked PDF."""
    try:
        before = signature(path.stat())
        if before == previous:
            return None
        with path.open('rb') as handle:
            opened = signature(os.fstat(handle.fileno()))
            if opened != before:
                return None
            handle.seek(max(0, opened[3] - 256))
            complete = handle.read().rstrip().endswith(b'%%EOF')
            after = signature(os.fstat(handle.fileno()))
        if complete and before == after == signature(path.stat()):
            return before
    except FileNotFoundError:
        pass
    return None


class Child:
    def __init__(self, command, directory, env, cpu, log):
        self.process = subprocess.Popen(
            command, cwd=directory, env=env, stdout=log, stderr=subprocess.STDOUT,
            start_new_session=True, preexec_fn=lambda: child_setup(cpu),
        )
        self.pidfd = None
        self.usage = None
        try:
            self.pidfd = os.pidfd_open(self.process.pid)
        except BaseException:
            self.abort()
            raise

    def ready(self, timeout=0):
        return bool(select.select([self.pidfd], [], [], timeout)[0])

    def cpu(self):
        return time.clock_gettime_ns((~self.process.pid << 3) | 2)

    def reap(self, timeout=TIMEOUT):
        if self.usage is not None:
            return self.usage
        if not self.ready(timeout):
            raise TimeoutError('Child did not finish within the process timeout')
        _, status, self.usage = os.wait4(self.process.pid, 0)
        self.process.returncode = os.waitstatus_to_exitcode(status)
        return self.usage

    def stop(self):
        if self.usage is not None:
            return self.usage
        if not self.ready():
            try:
                os.killpg(self.process.pid, signal.SIGTERM)
            except ProcessLookupError:
                pass
        return self.reap(timeout=5.0)

    def abort(self):
        if self.process.returncode is None:
            try:
                os.killpg(self.process.pid, signal.SIGKILL)
            except ProcessLookupError:
                pass
            if self.pidfd is None:
                # Only an error path: no measured row is published when pidfd
                # setup fails. Popen owns this fallback wait, avoiding zombies.
                self.process.wait(timeout=5.0)
            else:
                self.reap(timeout=5.0)

    def close(self):
        try:
            self.abort()
        finally:
            if self.pidfd is not None:
                os.close(self.pidfd)
                self.pidfd = None


def await_pdf(child, pdf, previous):
    deadline = time.monotonic() + TIMEOUT
    while True:
        if child.ready():
            raise RuntimeError('Watch exited before publishing the requested cycle')
        current = published(pdf, previous)
        if current is not None:
            return current, time.perf_counter_ns(), child.cpu()
        if time.monotonic() >= deadline:
            raise TimeoutError('Watch did not publish a complete new PDF within 60 seconds')
        time.sleep(POLL)


def totals(usage):
    user = round(usage.ru_utime * 1_000_000_000)
    system = round(usage.ru_stime * 1_000_000_000)
    return dict(process_cpu_ns=user + system, process_user_ns=user,
                process_system_ns=system,
                process_peak_rss_bytes=usage.ru_maxrss * 1024)


def run_process(options, variant, fixture, fixture_index, kind, round_id, repeat, warmup=False):
    prefix = 'warmup' if warmup else f'round-{round_id:03d}-repeat-{repeat:03d}'
    name = f'{prefix}-fixture-{fixture_index:03d}-{kind}-{variant}'
    case = options.ram_dir / name
    case.mkdir(exist_ok=False)
    inputs = case / 'inputs'
    shutil.copytree(options.fixtures, inputs)
    main = inputs / fixture
    initial_source = main.read_bytes()
    epoch_zero = initial_source + EPOCH_ZERO
    main.write_bytes(epoch_zero)
    pdf = case / 'output.pdf'
    temporary = case / 'tmp'
    temporary.mkdir()
    binary, policy = options.variants[variant]
    command = [str(binary), 'compile' if kind == 'cli' else 'watch', str(main), str(pdf),
               '--root', str(inputs), '--pdf-tagged=false'] + options.extra_arg
    child = None
    rows = []
    base = dict(round=round_id, repeat=repeat, variant=variant,
                fixture=fixture.as_posix(), case=name)
    with (case / 'diagnostics.log').open('wb') as log:
        try:
            epoch = time.time()
            started = time.perf_counter_ns()
            env = environment(temporary)
            env.pop('TEKT_JOINT_PHASE_FILE', None)
            env.pop('TEKT_JOINT_MODE', None)
            env['TEKT_JOINT_MODE'] = policy
            if options.probe or variant in options.probe_variant: env['TEKT_JOINT_PHASE_FILE'] = str(case / 'phases.tsv')
            child = Child(command, inputs, env, options.cpu, log)
            base['process_id'] = child.process.pid
            if kind == 'cli':
                usage = child.reap()
                ended = time.perf_counter_ns()
                if child.process.returncode != 0:
                    raise RuntimeError(f'CLI failed ({child.process.returncode}); see {case}')
                if published(pdf, None) is None:
                    raise RuntimeError(f'CLI did not publish a complete PDF; see {case}')
                rows.append(dict(base, mode='cli', cycle=0, sample_epoch=epoch,
                                 wall_ns=ended - started,
                                 cpu_ns=totals(usage)['process_cpu_ns']))
                shutil.copy2(pdf, case / 'cli.pdf')
            else:
                _, ended, cpu_ended = await_pdf(child, pdf, None)
                rows.append(dict(base, mode='startup', cycle=0, sample_epoch=epoch,
                                 wall_ns=ended - started, cpu_ns=cpu_ended))
                time.sleep(SETTLE)
                shutil.copy2(pdf, case / 'startup.pdf')
                current_marker = 0
                cycle = 0
                for _ in range(options.cycles):
                    next_marker = 1 - current_marker
                    for mode, marker in [('same_bytes', current_marker), ('main_edit', next_marker)]:
                        cycle += 1
                        data = initial_source + (EPOCH_ZERO if marker == 0 else EPOCH_ONE)
                        # Complete publication and settle precede the interval.
                        # same_bytes repeats the current marker; main_edit flips.
                        if child.ready():
                            raise RuntimeError(f'Watch exited between cycles; see {case}')
                        previous = published(pdf, None)
                        if previous is None:
                            raise RuntimeError(f'Previous PDF is unstable before source edit; see {case}')
                        cpu_started = child.cpu()
                        epoch = time.time()
                        started = time.perf_counter_ns()
                        main.write_bytes(data)
                        _, ended, cpu_ended = await_pdf(child, pdf, previous)
                        delta = cpu_ended - cpu_started
                        if delta < 0:
                            raise RuntimeError('Process CPU clock moved backwards')
                        rows.append(dict(base, mode=mode, cycle=cycle, sample_epoch=epoch,
                                         wall_ns=ended - started, cpu_ns=delta))
                        time.sleep(SETTLE)
                        shutil.copy2(pdf, case / f'cycle-{cycle:03d}-{mode}.pdf')
                    current_marker = next_marker
                usage = child.stop()
                if child.process.returncode not in (0, -signal.SIGTERM):
                    raise RuntimeError(f'Unexpected watch termination; see {case}')
                if sum(row['cpu_ns'] for row in rows) > totals(usage)['process_cpu_ns'] + 1_000_000:
                    raise RuntimeError('Cycle CPU sum exceeds wait4 total by more than 1 ms')
            for row in rows:
                row.update(totals(usage), exit_code=child.process.returncode)
        finally:
            if child is not None:
                child.close()
    return rows


def parse_options():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--variant', action='append', required=True, help='NAME=ELF:0|2 (original binary also receives mode0)')
    parser.add_argument('--probe', action='store_true')
    parser.add_argument('--probe-variant', action='append', default=[])
    parser.add_argument('--fixtures', type=Path, required=True)
    parser.add_argument('--fixture', type=Path, action='append', required=True)
    parser.add_argument('--ram-dir', type=Path, required=True)
    parser.add_argument('--rounds', type=int, default=10)
    parser.add_argument('--cycles', type=int, default=6)
    parser.add_argument('--cpu', type=int, default=0)
    parser.add_argument('--extra-arg', action='append', default=[])
    parser.add_argument('--repeats', type=int, default=2)
    options = parser.parse_args()
    if options.ram_dir.absolute() != options.ram_dir.resolve():
        parser.error('ram-dir may not use symlinks or lexical escapes')
    if options.ram_dir.is_symlink():
        parser.error('ram-dir may not name an existing symlink')
    for field in ('fixtures', 'ram_dir'):
        setattr(options, field, getattr(options, field).resolve())
    if (not 1 <= options.rounds <= 1000 or not 1 <= options.repeats <= 1000
            or not 1 <= options.cycles <= 1000):
        parser.error('rounds, repeats and cycles must be in 1..1000')
    task = TASK_RAM.resolve()
    if (TASK_RAM.is_symlink() or not task.is_dir()
            or not task.is_relative_to(Path('/dev/shm').resolve())):
        parser.error('Exclusive task RAM directory must exist; no disk fallback')
    if not options.ram_dir.is_relative_to(task) or options.ram_dir == task:
        parser.error('ram-dir must be a new exclusive child of the task RAM area')
    first = options.ram_dir.relative_to(task).parts[0]
    if first.lower().startswith(('frozen', 'baseline', 'reference')) or first == 'R':
        parser.error('Reference directories cannot contain measurement output')
    if options.ram_dir.exists() or options.ram_dir.is_symlink():
        parser.error('ram-dir already exists; previous results are never overwritten')
    if not options.ram_dir.parent.is_dir():
        parser.error('ram-dir parent must already exist')
    options.variants = {}
    for entry in options.variant:
        name, rest = entry.split('=', 1)
        binary, policy = rest.rsplit(':', 1)
        if policy not in ('0', '2') or not name.isalnum() or name in options.variants:
            parser.error('Invalid or duplicate variant')
        options.variants[name] = (Path(binary).resolve(), policy)
    for binary, _ in options.variants.values():
        if not binary.is_file() or not os.access(binary, os.X_OK):
            parser.error(f'Executable not available: {binary}')
    if not options.fixtures.is_dir():
        parser.error('fixtures must be an existing self-contained tree')
    if (options.ram_dir.is_relative_to(options.fixtures)
            or options.fixtures.is_relative_to(options.ram_dir)):
        parser.error('Fixtures and output RAM area may not contain each other')
    if any(path.is_symlink() for path in options.fixtures.rglob('*')):
        parser.error('Use self-contained fixture trees without symlinks')
    normalized = []
    for fixture in options.fixture:
        if fixture.is_absolute() or '..' in fixture.parts:
            parser.error('Each fixture must be a relative path without ..')
        source = (options.fixtures / fixture).resolve()
        if not source.is_relative_to(options.fixtures) or not source.is_file():
            parser.error(f'Fixture not found inside tree: {fixture}')
        normalized.append(source.relative_to(options.fixtures))
    if len(set(normalized)) != len(normalized):
        parser.error('Duplicate fixture arguments are ambiguous for paired analysis')
    options.fixture = normalized
    if options.cpu not in os.sched_getaffinity(0):
        parser.error(f'Requested CPU {options.cpu} is not in the available affinity set')
    for variant in options.probe_variant:
        if variant not in options.variants:
            parser.error(f'Unknown probe variant: {variant}')
    return options


def main():
    options = parse_options()
    options.ram_dir.mkdir(exist_ok=False)
    variants = list(options.variants)
    for index, fixture in enumerate(options.fixture):
        for kind in ('cli', 'watch'):
            for variant in variants[index % 2:] + variants[:index % 2]:
                run_process(options, variant, fixture, index, kind, -1, 0, warmup=True)
    with (options.ram_dir / 'measurements.csv').open('x', newline='') as handle:
        writer = csv.DictWriter(handle, fieldnames=FIELDS)
        writer.writeheader()
        handle.flush()
        count = len(options.fixture)
        for round_id in range(options.rounds):
            for repeat in range(options.repeats):
                offset = (round_id + repeat) % count
                order = list(range(count))[offset:] + list(range(count))[:offset]
                for index in order:
                    kinds = ['cli', 'watch']
                    if (round_id + repeat) % 2:
                        kinds.reverse()
                    for kind_index, kind in enumerate(kinds):
                        pair = variants[:]
                        shift = (round_id * options.repeats + repeat + index + kind_index) % len(pair)
                        pair = pair[shift:] + pair[:shift]
                        if round_id % 2: pair.reverse()
                        for variant in pair:
                            rows = run_process(options, variant, options.fixture[index],
                                               index, kind, round_id, repeat)
                            writer.writerows(rows)
                            handle.flush()
            print(f'Completed round {round_id + 1}/{options.rounds}', flush=True)
    print(options.ram_dir / 'measurements.csv', flush=True)


if __name__ == '__main__':
    main()
