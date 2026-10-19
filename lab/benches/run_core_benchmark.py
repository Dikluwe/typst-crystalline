"""Build and verify private core benchmarks before allowing any measurements."""

import argparse
import json
import os
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[2]
TESTS = {
    "compiler::eval::observation::benchmarks::observation_microbenchmarks",
    "compiler::stdlib::registry::microbenchmarks::benchmarks::stdlib_microbenchmarks",
}


def executable_from_messages(messages):
    executable = None
    for line in messages.splitlines():
        message = json.loads(line)
        if (
            message.get("reason") != "compiler-artifact"
            or message.get("target", {}).get("name") != "typst_core"
            or message.get("profile", {}).get("test") is not True
        ):
            continue
        profile = message["profile"]
        if (
            profile.get("opt_level") not in {"1", "2", "3", "s", "z"}
            or profile.get("debug_assertions") is not False
        ):
            raise ValueError(
                "refusing core benchmark artifact: optimization must be enabled "
                "and debug assertions disabled"
            )
        path = message.get("executable")
        if not isinstance(path, str) or not Path(path).is_absolute():
            raise ValueError("missing absolute core test executable")
        if executable is not None:
            raise ValueError("ambiguous core test executables")
        executable = Path(path)
    if executable is None:
        raise ValueError("Cargo did not report a core test executable")
    return executable


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("test", choices=sorted(TESTS))
    parser.add_argument("--bench", action="store_true", help=argparse.SUPPRESS)
    parser.add_argument("--test", dest="smoke", action="store_true", help="verify runner without sampling")
    args = parser.parse_args()

    environment = os.environ.copy()
    # Empty encoded flags take precedence over RUSTFLAGS and Cargo config flags.
    # This prevents hidden -C options from contradicting the reported profile.
    environment["CARGO_ENCODED_RUSTFLAGS"] = ""
    environment["CARGO_TARGET_DIR"] = str(ROOT / "lab" / "target" / "benchmarks")
    build = subprocess.run(
        [
            environment.get("CARGO", "cargo"),
            "test", "--offline", "-p", "typst-core", "--release", "--lib",
            "--no-run", "--message-format=json",
        ],
        cwd=ROOT, env=environment, stdout=subprocess.PIPE, text=True, check=True,
    )
    executable = executable_from_messages(build.stdout)
    print(f"Verified optimized core executable: {executable}", file=sys.stderr)
    command = [str(executable), args.test, "--exact", "--ignored"]
    listing = subprocess.run(
        command + ["--list"], cwd=ROOT, capture_output=True, text=True, check=True,
    )
    # libtest reports success even when a filter selects no tests.
    if f"{args.test}: test" not in listing.stdout.splitlines():
        raise ValueError(f"ignored benchmark test not found: {args.test}")
    if args.smoke:
        # --test is a harness smoke check, not a timing run.
        print(f"Verified benchmark: {args.test} (no measurements)", file=sys.stderr)
    else:
        subprocess.run(command + ["--nocapture"], cwd=ROOT, check=True)


if __name__ == "__main__":
    try:
        main()
    except (ValueError, OSError, subprocess.CalledProcessError) as error:
        if isinstance(error, subprocess.CalledProcessError) and error.stdout:
            # Cargo JSON diagnostics would otherwise be hidden by stdout capture.
            for line in error.stdout.splitlines():
                try:
                    message = json.loads(line)
                except ValueError:
                    continue
                diagnostic = message.get("message", {})
                if message.get("reason") == "compiler-message" and diagnostic.get("level") == "error":
                    print(diagnostic.get("rendered", diagnostic.get("message")), file=sys.stderr)
        sys.exit(f"Core benchmark refused: {error}")
