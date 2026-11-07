"""Black-box validation of the allocator probe's explicit self-test.

This checks instrumentation with known GlobalAlloc operations. It does not
implement an alternate allocator or assert product performance. The producer
must use its ordinary process-global getters and report snapshots after the
four operations, without resetting live state between intervals.
"""

import argparse
import math
import os
import pathlib
import re
import subprocess
import sys


EXPECTED = (
    ("allocator.zeroed", 1, 0, 0, 128, 0, 128),
    ("allocator.grow", 0, 0, 1, 192, 128, 64),
    ("allocator.shrink", 0, 0, 1, 64, 192, -128),
    ("allocator.free", 0, 1, 0, 0, 64, -64),
)
FIELDS = (
    "allocs", "deallocs", "reallocs", "requested_bytes", "released_bytes",
    "live_before", "live_after", "live_delta",
    "process_peak_before", "process_peak_after",
)


def integer(value, field, label):
    grammar = r"-?[0-9]+" if field == "live_delta" else r"[0-9]+"
    assert re.fullmatch(grammar, value), (
        f"{label}: {field} must be a decimal integer, got {value!r}"
    )
    return int(value)


def validate(stdout):
    labels = {row[0] for row in EXPECTED}
    observed = {}
    sequence = []
    controls = []
    for line in stdout.splitlines():
        fields = line.split("\t")
        if fields[0] == "alloc":
            assert len(fields) >= 3, f"malformed allocation record: {line!r}"
            label = fields[1]
            if label not in labels:
                continue
            assert label not in observed, f"duplicate allocation interval: {label}"
            raw = {}
            for item in fields[2:]:
                key, separator, value = item.partition("=")
                assert separator and key, f"malformed allocation field: {item!r}"
                assert key not in raw, f"{label}: duplicate field {key}"
                raw[key] = value
            missing = set(FIELDS) - raw.keys()
            assert not missing, f"{label}: missing fields {sorted(missing)}"
            observed[label] = {key: integer(raw[key], key, label) for key in FIELDS}
            sequence.append(label)
        elif line.split()[:1] == ["allocator_control"]:
            controls.append(line.split()[1:])

    assert sequence == [row[0] for row in EXPECTED], (
        f"missing or out-of-order allocator intervals: {sequence!r}"
    )
    assert len(controls) == 1, f"expected one allocator_control record, found {len(controls)}"
    control_fields = {}
    for item in controls[0]:
        key, separator, value = item.partition("=")
        assert separator and key, f"malformed allocator_control field: {item!r}"
        assert key not in control_fields, f"duplicate allocator_control field: {key}"
        control_fields[key] = value
    assert control_fields.get("zero_valid") == "true", "alloc_zeroed did not preserve zero initialization"
    assert control_fields.get("prefix_valid") == "true", "realloc did not preserve the existing prefix"

    baseline = observed[EXPECTED[0][0]]["live_before"]
    assert baseline > 0, "live state must be the existing process-global state, not a zeroed interval gauge"
    previous = None
    for label, allocs, deallocs, reallocs, requested, released, delta in EXPECTED:
        row = observed[label]
        expected_fields = {
            "allocs": allocs, "deallocs": deallocs, "reallocs": reallocs,
            "requested_bytes": requested, "released_bytes": released, "live_delta": delta,
        }
        for field, expected in expected_fields.items():
            assert row[field] == expected, (
                f"{label}: {field} expected {expected}, observed {row[field]}"
            )
        assert row["live_after"] - row["live_before"] == delta, (
            f"{label}: live_after-live_before disagrees with explicit operation delta"
        )
        assert row["requested_bytes"] - row["released_bytes"] == delta, (
            f"{label}: requested/released balance disagrees with live delta"
        )
        assert row["process_peak_after"] >= row["process_peak_before"], (
            f"{label}: process-global peak decreased within interval"
        )
        assert row["process_peak_before"] >= row["live_before"], (
            f"{label}: prior process peak is below prior live bytes"
        )
        assert row["process_peak_after"] >= row["live_after"], (
            f"{label}: process peak is below current live bytes"
        )
        if previous is not None:
            assert row["live_before"] == previous["live_after"], (
                f"{label}: live snapshots are discontinuous between explicit operations"
            )
            assert row["process_peak_before"] == previous["process_peak_after"], (
                f"{label}: process peak snapshots are discontinuous between explicit operations"
            )
        previous = row

    assert sum(row["live_delta"] for row in observed.values()) == 0, "self-test leaked live bytes"
    assert sum(row["requested_bytes"] for row in observed.values()) == sum(
        row["released_bytes"] for row in observed.values()
    ), "total requested and released bytes do not balance"
    assert previous["live_after"] == baseline, "free did not restore the original global live state"
    return baseline, previous["process_peak_after"]


def main(options):
    binary = pathlib.Path(options.binary)
    assert binary.is_absolute() and binary.is_file(), "--binary must be an existing absolute path"
    assert os.access(binary, os.X_OK), "--binary must be executable"
    result = subprocess.run(
        [str(binary), "allocator-selftest"], capture_output=True, text=True,
        timeout=options.timeout,
    )
    assert result.returncode == 0, (
        f"allocator-selftest exited {result.returncode}\nstdout={result.stdout}\nstderr={result.stderr}"
    )
    try:
        baseline, peak = validate(result.stdout)
    except AssertionError:
        print("producer_stdout=" + result.stdout, file=sys.stderr)
        print("producer_stderr=" + result.stderr, file=sys.stderr)
        raise
    print(f"allocator_validation=PASS intervals=4 live_baseline={baseline} process_peak_after={peak}")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", required=True)
    parser.add_argument("--timeout", type=float, default=30.0)
    options = parser.parse_args()
    if not math.isfinite(options.timeout) or options.timeout <= 0:
        parser.error("--timeout must be a positive finite duration")
    try:
        main(options)
    except (AssertionError, OSError, subprocess.SubprocessError) as error:
        print(f"allocator_validation=FAIL: {error}", file=sys.stderr)
        sys.exit(1)
