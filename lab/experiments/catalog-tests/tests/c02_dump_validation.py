"""Black-box tests for C's catalog probe dump consumer.

Run this test driver with --validator, --ram-dir and repeated --known-scope.
The validator CLI accepts --input PATH --mode timing|query|off
--purpose counts|performance plus repeated --known-scope LABEL. Its exit status
is zero exactly when the input is admissible for that use. This tests a
consumer of the public TSV protocol, without importing the instrumented code.

All generated fixtures live below the explicitly supplied RAM task directory.
The optional --real-dump is checked separately as a real producer observation.
"""

import argparse
import pathlib
import subprocess
import sys
import tempfile
import unittest


OPTIONS = None


def cycle(rows, pid=17, thread="ThreadId(1)"):
    return "cycle\tpid=" + str(pid) + "\tthread=" + thread + "\n" + "".join(
        row + "\n" for row in rows
    ) + "end\n"


def scope(label, count=1, ns=1):
    return f"scope\t{label}\t{count}\t{ns}"


def query(count=1):
    # Escaped Debug values contain no literal TSV separator. Consumers must
    # retain the complete fields; this protocol does not make Debug into JSON.
    return (
        'query\t["G1", "nested\\tlabel\\n\\\"quoted\\\""]'
        '\tstatx\t"/fonts/name\\twith\\nnewline\\\\slash.otf"'
        f"\tOk\t{count}"
    )


class DumpConsumerTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.ram = pathlib.Path(OPTIONS.ram_dir).resolve()
        if not cls.ram.is_relative_to(pathlib.Path("/dev/shm")):
            raise RuntimeError("--ram-dir must be inside this task's /dev/shm directory")
        if not cls.ram.is_dir():
            raise RuntimeError("C must create --ram-dir before running the tests")
        cls.temp = tempfile.TemporaryDirectory(prefix="c02-consumer-tests-", dir=cls.ram)
        cls.root = pathlib.Path(cls.temp.name)
        cls.labels = list(dict.fromkeys(OPTIONS.known_scope))
        if len(set(cls.labels)) < 2:
            raise RuntimeError("supply at least two distinct production --known-scope labels")
        cls.sequence = 0

    @classmethod
    def tearDownClass(cls):
        cls.temp.cleanup()

    def validate(self, text, *, mode="timing", purpose="counts", accepted=True):
        type(self).sequence += 1
        path = self.root / f"fixture-{self.sequence}.tsv"
        path.write_text(text, encoding="utf-8")
        return self.validate_path(path, mode=mode, purpose=purpose, accepted=accepted)

    def validate_path(self, path, *, mode, purpose, accepted):
        command = [
            sys.executable, OPTIONS.validator, "--input", str(path),
            "--mode", mode, "--purpose", purpose,
        ]
        for label in self.labels:
            command.extend(["--known-scope", label])
        result = subprocess.run(command, capture_output=True, text=True, timeout=30)
        self.assertEqual(
            result.returncode == 0, accepted,
            f"consumer acceptance mismatch; expected accepted={accepted}; "
            f"status={result.returncode}\nstdout={result.stdout}\nstderr={result.stderr}",
        )
        return result

    def test_control_single_timing_acquisition(self):
        self.validate(cycle([scope(self.labels[0])]), purpose="performance")

    def test_control_zero_nanoseconds_and_integer_upper_bounds(self):
        self.validate(cycle([
            scope(self.labels[0], 1, 0),
            scope(self.labels[1], 2**64 - 1, 2**128 - 1),
        ]))

    def test_control_multiple_blocks_may_repeat_scopes(self):
        self.validate(
            cycle([scope(self.labels[0], 2, 100)], pid=17)
            + cycle([scope(self.labels[0], 3, 150)], pid=18)
            + cycle([scope(self.labels[0], 1, 50)], pid=17),
            purpose="performance",
        )

    def test_control_inclusive_scope_times_are_not_a_partition(self):
        # These scopes may be parent and child. They are not additive and
        # must not be rejected for summing above the parent's inclusive time.
        self.validate(cycle([
            scope(self.labels[0], 1, 100), scope(self.labels[1], 1, 90),
        ]), purpose="performance")

    def test_control_escaped_debug_fields_in_query_mode(self):
        self.validate(cycle([scope(self.labels[0], 1, 0), query(2**64 - 1)]), mode="query")

    def test_query_mode_is_not_performance_evidence(self):
        for ns in [0, 500]:
            with self.subTest(ns=ns):
                self.validate(cycle([scope(self.labels[0], 1, ns), query()]),
                              mode="query", purpose="performance", accepted=False)

    def test_reject_empty_or_truncated_dump(self):
        good = cycle([scope(self.labels[0], 1, 10)])
        for text in ["", "cycle", "cycle\tpid=17\tthread=", good[:-4],
                     good + "cycle\tpid=18\tthread=ThreadId(2)\n",
                     "cycle\tpid=17\tthread=ThreadId(1)\nscope\t"]:
            with self.subTest(text=text):
                self.validate(text, accepted=False)

    def test_reject_interleaved_or_unframed_lines(self):
        for text in [
            scope(self.labels[0]) + "\n",
            "end\n",
            "cycle\tpid=17\tthread=ThreadId(1)\n"
            + "cycle\tpid=18\tthread=ThreadId(2)\nend\nend\n",
            cycle([scope(self.labels[0])]) + scope(self.labels[1]) + "\n",
            cycle(["unknown_record\tvalue"]),
            cycle([scope(self.labels[0])]).replace("end\n", "end\textra\n"),
        ]:
            with self.subTest(text=text):
                self.validate(text, accepted=False)

    def test_reject_duplicate_scope_within_one_acquisition(self):
        for second in [scope(self.labels[0]), scope(self.labels[0], 7, 900)]:
            with self.subTest(second=second):
                self.validate(cycle([scope(self.labels[0]), second]), accepted=False)

    def test_reject_unknown_scope_label(self):
        unknown = "__catalog_test_unknown_scope__"
        self.assertNotIn(unknown, self.labels)
        self.validate(cycle([scope(unknown)]), accepted=False)

    def test_reject_scope_integer_negative_overflow_and_noninteger(self):
        for count in ["0", "-1", str(2**64), "1.5", "NaN", "", "1e3"]:
            with self.subTest(count=count):
                self.validate(cycle([scope(self.labels[0], count, 1)]), accepted=False)
        for ns in ["-1", str(2**128), "1.5", "NaN", "", "1e3"]:
            with self.subTest(ns=ns):
                self.validate(cycle([scope(self.labels[0], 1, ns)]), accepted=False)

    def test_reject_query_count_negative_overflow_and_noninteger(self):
        for count in ["0", "-1", str(2**64), "1.5", "NaN", "", "1e3"]:
            with self.subTest(count=count):
                self.validate(cycle([query(count)]), mode="query", accepted=False)

    def test_reject_literal_tab_in_debug_field_or_wrong_arity(self):
        escaped = query()
        for row in [
            escaped.replace("\\tlabel", "\tlabel"),
            escaped.replace("\\twith", "\twith"),
            escaped + "\textra",
            "query\tstack\tstatx\tpath\tOk",
            scope(self.labels[0]) + "\textra",
            "scope\t" + self.labels[0] + "\t1",
        ]:
            with self.subTest(row=row):
                self.validate(cycle([row]), mode="query", accepted=False)

    def test_reject_malformed_cycle_header(self):
        for header in [
            "cycle\t17\tthread=ThreadId(1)",
            "cycle\tpid=no-number\tthread=ThreadId(1)",
            "cycle\tpid=0\tthread=ThreadId(1)",
            "cycle\tpid=-1\tthread=ThreadId(1)",
            "cycle\tpid=17", "cycle\tpid=17\tthread=",
            "cycle\tpid=17\tthread=ThreadId(1)\textra",
        ]:
            with self.subTest(header=header):
                self.validate(header + "\n" + scope(self.labels[0]) + "\nend\n", accepted=False)

    def test_reject_acquisition_without_mode_required_records(self):
        self.validate(cycle([]), accepted=False)
        self.validate(cycle([query()]), mode="timing", accepted=False)
        self.validate(cycle([scope(self.labels[0])]), mode="query", accepted=False)

    def test_reject_empty_query_debug_fields(self):
        self.validate(cycle(["query\t\tstatx\tpath\tOk\t1"]),
                      mode="query", accepted=False)
        self.validate(cycle(["query\tstack\tstatx\t\tOk\t1"]),
                      mode="query", accepted=False)

    def test_off_consumer_accepts_only_absent_file(self):
        missing = self.root / "off-must-stay-absent.tsv"
        self.assertFalse(missing.exists())
        self.validate_path(missing, mode="off", purpose="counts", accepted=True)
        self.validate("", mode="off", accepted=False)
        self.validate(cycle([scope(self.labels[0])]), mode="off", accepted=False)

    def test_real_producer_dump_when_supplied(self):
        if OPTIONS.real_dump is None:
            self.skipTest("no real producer dump supplied; fixtures alone do not certify producer")
        self.validate_path(pathlib.Path(OPTIONS.real_dump), mode=OPTIONS.real_mode,
                           purpose="counts", accepted=True)
        if OPTIONS.real_mode == "query":
            self.validate_path(pathlib.Path(OPTIONS.real_dump), mode="query",
                               purpose="performance", accepted=False)

    def test_off_probe_leaves_no_file_when_supplied(self):
        if OPTIONS.off_path is None:
            self.skipTest("no Off probe path supplied; consumer fixtures do not certify Off")
        path = pathlib.Path(OPTIONS.off_path)
        self.assertFalse(path.exists(), f"Off probe created output: {path}")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--validator", required=True)
    parser.add_argument("--ram-dir", required=True)
    parser.add_argument("--known-scope", action="append", required=True)
    parser.add_argument("--real-dump")
    parser.add_argument("--real-mode", choices=["timing", "query"], default="timing")
    parser.add_argument("--off-path")
    OPTIONS, remaining = parser.parse_known_args()
    unittest.main(argv=[sys.argv[0], *remaining])
