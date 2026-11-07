"""Observable plugin identity regression across a real watch lifecycle.

Fresh CLI processes must print their own WASM payload. In one watch process,
replacing the module and changing the entry source must publish the new epoch
with the new payload. A stale A at Epoch 2 is a failing assertion, not accepted
as a baseline. Every fixture, PDF and log is created under --ram-dir.
"""

import argparse
import hashlib
import os
import pathlib
import re
import shutil
import subprocess
import sys
import tempfile
import time


def uleb(value):
    result = bytearray()
    while True:
        byte = value & 127
        value >>= 7
        result.append(byte | (128 if value else 0))
        if not value:
            return bytes(result)


def name(value):
    encoded = value.encode("utf-8")
    return uleb(len(encoded)) + encoded


def wasm(tag):
    """Real zero-argument run()->i32, sending one byte via typst_env."""
    assert tag in (b"A", b"B")
    output = bytearray(b"\x00asm\x01\x00\x00\x00")

    def section(number, payload):
        output.append(number)
        output.extend(uleb(len(payload)))
        output.extend(payload)

    # type 0: (i32,i32)->(); type 1: ()->i32
    section(1, bytes([2, 0x60, 2, 0x7F, 0x7F, 0, 0x60, 0, 1, 0x7F]))
    section(2, b"\x01" + name("typst_env")
            + name("wasm_minimal_protocol_send_result_to_host") + b"\x00\x00")
    section(3, b"\x01\x01")
    section(5, b"\x01\x00\x01")
    section(7, b"\x02" + name("memory") + b"\x02\x00"
            + name("run") + b"\x00\x01")
    # locals=0; const 0; const 1; call send; const 0 (success); end
    section(10, bytes([1, 10, 0, 0x41, 0, 0x41, 1, 0x10, 0, 0x41, 0, 0x0B]))
    section(11, bytes([1, 0, 0x41, 0, 0x0B, 1]) + tag)
    return bytes(output)


def source(epoch):
    return '#let p = plugin("changing.wasm")\n#str(p.run())\nEpoch ' + str(epoch) + "\n"


def replace_module(directory, tag):
    staged = directory / "changing.wasm.next"
    staged.write_bytes(wasm(tag))
    os.replace(staged, directory / "changing.wasm")


def normalized(text):
    return " ".join(text.split())


def assert_payload(text, tag, epoch, label):
    compact = normalized(text)
    marker = re.search(r"Epoch\s+" + str(epoch) + r"\b", compact)
    assert marker, (
        f"{label}: wrong epoch; extracted={compact!r}"
    )
    # PDF extraction may join adjacent runs with no separator. The fixture
    # contains exactly the plugin output before the epoch, so inspect that
    # prefix directly rather than rely on a word boundary after its byte.
    observed_tag = compact[:marker.start()].strip()
    assert observed_tag == tag, (
        f"{label}: expected payload {tag} at Epoch {epoch}, got {observed_tag!r}; "
        f"extracted={compact!r}"
    )


def complete_pdf_text(path, pdftotext, timeout):
    try:
        payload = path.read_bytes()
    except FileNotFoundError:
        return None
    if not payload.startswith(b"%PDF-") or not payload.rstrip().endswith(b"%%EOF"):
        return None
    result = subprocess.run(
        [pdftotext, "-", "-"], input=payload, capture_output=True,
        timeout=max(0.01, min(timeout, 5.0)),
    )
    if result.returncode != 0:
        return None
    return result.stdout.decode("utf-8", errors="replace"), hashlib.sha256(payload).hexdigest()


def wait_epoch(path, epoch, pdftotext, timeout, process=None, previous_digest=None):
    deadline = time.monotonic() + timeout
    last_text = None
    while time.monotonic() < deadline:
        if process is not None and process.poll() is not None:
            raise AssertionError(f"watch exited early with status {process.returncode}")
        remaining = deadline - time.monotonic()
        try:
            observed = complete_pdf_text(path, pdftotext, remaining)
        except subprocess.TimeoutExpired:
            observed = None
        if observed is not None:
            text, digest = observed
            last_text = normalized(text)
            current_epoch = re.search(r"Epoch\s+" + str(epoch) + r"\b", last_text)
            if current_epoch and digest != previous_digest:
                return text, digest
        time.sleep(min(0.05, max(0.0, deadline - time.monotonic())))
    raise AssertionError(f"timed out waiting for complete new Epoch {epoch} PDF; last_text={last_text!r}")


def fresh_cli_control(binary, directory, tag, epoch, pdftotext, timeout, environment):
    directory.mkdir(exist_ok=True)
    replace_module(directory, tag.encode("ascii"))
    entry = directory / "main.typ"
    entry.write_text(source(epoch), encoding="utf-8")
    pdf = directory / "output.pdf"
    # Remove the prior PDF so a successful old output cannot satisfy a control.
    pdf.unlink(missing_ok=True)
    result = subprocess.run(
        [str(binary), "compile", str(entry), str(pdf)], cwd=directory,
        env=environment, capture_output=True, timeout=timeout,
    )
    (directory / f"cli-{tag}.stdout").write_bytes(result.stdout)
    (directory / f"cli-{tag}.stderr").write_bytes(result.stderr)
    assert result.returncode == 0, (
        f"fresh CLI {tag} failed status={result.returncode}: "
        + result.stderr.decode("utf-8", errors="replace")
    )
    text, _ = wait_epoch(pdf, epoch, pdftotext, timeout)
    assert_payload(text, tag, epoch, f"fresh CLI {tag}")
    print(f"control fresh_cli_{tag}=PASS text={normalized(text)!r}", flush=True)


def stop_watch(process):
    # Reap on every path, including assertions and timeout. Escalation is
    # bounded; these processes are created and owned exclusively by this test.
    if process.poll() is None:
        process.terminate()
    try:
        process.wait(timeout=5)
    except subprocess.TimeoutExpired:
        process.kill()
        process.wait(timeout=5)


def exercise(options):
    binary = pathlib.Path(options.binary)
    ram = pathlib.Path(options.ram_dir)
    assert binary.is_absolute() and binary.is_file(), "--binary must be an existing absolute binary path"
    assert ram.is_absolute() and ram.resolve().is_relative_to(pathlib.Path("/dev/shm")), "--ram-dir must be task RAM"
    assert ram.is_dir(), "C must create --ram-dir before running the test"
    pdftotext = options.pdftotext or shutil.which("pdftotext")
    assert pdftotext, "pdftotext is required to observe PDF text"
    work = pathlib.Path(tempfile.mkdtemp(prefix="watch-plugin-identity-", dir=ram))
    print(f"ram_fixtures={work}", flush=True)
    environment = os.environ.copy()
    environment["TMPDIR"] = str(work)
    # Keep controls in fresh OS processes while replacing the module at the
    # same path, then hold precisely one process for the watch scenario.
    controls = work / "controls"
    fresh_cli_control(binary, controls, "A", 101, pdftotext, options.timeout, environment)
    fresh_cli_control(binary, controls, "B", 102, pdftotext, options.timeout, environment)

    watched = work / "watch"
    watched.mkdir()
    replace_module(watched, b"A")
    entry = watched / "main.typ"
    entry.write_text(source(1), encoding="utf-8")
    pdf = watched / "output.pdf"
    log_path = watched / "watch.log"
    process = None
    try:
        with log_path.open("wb") as log:
            process = subprocess.Popen(
                [str(binary), "watch", str(entry), str(pdf)], cwd=watched,
                env=environment, stdout=log, stderr=subprocess.STDOUT,
            )
            try:
                initial_text, initial_digest = wait_epoch(
                    pdf, 1, pdftotext, options.timeout, process=process,
                )
                assert_payload(initial_text, "A", 1, "watch startup")
                print(f"control watch_startup_A=PASS text={normalized(initial_text)!r}", flush=True)
                replace_module(watched, b"B")
                # Editing the entry explicitly requests a fresh compile. A
                # different epoch proves the observed PDF belongs to that run.
                entry.write_text(source(2), encoding="utf-8")
                current_text, _ = wait_epoch(
                    pdf, 2, pdftotext, options.timeout, process=process,
                    previous_digest=initial_digest,
                )
                print(f"observation watch_epoch_2={normalized(current_text)!r}", flush=True)
                assert_payload(current_text, "B", 2, "watch after module replacement")
                print("watch_plugin_identity=PASS", flush=True)
            finally:
                stop_watch(process)
    except BaseException:
        if process is not None and process.poll() is None:
            stop_watch(process)
        if log_path.exists():
            print("watch_log_tail=" + log_path.read_text(encoding="utf-8", errors="replace")[-8000:], file=sys.stderr)
        raise


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", required=True)
    parser.add_argument("--ram-dir", required=True)
    parser.add_argument("--pdftotext")
    parser.add_argument("--timeout", type=float, default=120.0)
    options = parser.parse_args()
    if not 0 < options.timeout <= 600:
        parser.error("--timeout must be positive and at most 600 seconds per stage")
    try:
        exercise(options)
    except (AssertionError, OSError, subprocess.SubprocessError) as error:
        print(f"watch_plugin_identity=FAIL: {error}", file=sys.stderr)
        sys.exit(1)
