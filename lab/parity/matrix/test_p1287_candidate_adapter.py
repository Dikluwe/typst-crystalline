import importlib.util
from pathlib import Path
import unittest


HERE = Path(__file__).resolve().parent
SPEC = importlib.util.spec_from_file_location(
    "p1287_candidate_adapter", HERE / "p1287_candidate_adapter.py"
)
ADAPTER = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(ADAPTER)


class CandidateAdapterTests(unittest.TestCase):
    def test_compile_removes_only_unsupported_measurement_options(self):
        args = [
            "compile", "--creation-timestamp", "0", "--jobs", "1",
            "--diagnostic-format", "short", "--ppi", "144",
            "--format", "png", "in.typ", "out.png",
        ]
        self.assertEqual(
            ADAPTER.adapt(args),
            ["compile", "--format", "png", "in.typ", "out.png"],
        )

    def test_eval_removes_diagnostic_format_but_preserves_expression(self):
        self.assertEqual(
            ADAPTER.adapt(["eval", "--diagnostic-format", "short", "1 + 2"]),
            ["eval", "1 + 2"],
        )

    def test_query_is_not_emulated(self):
        args = ["query", "--diagnostic-format", "short", "in.typ", "heading"]
        self.assertEqual(ADAPTER.adapt(args), ["query", "in.typ", "heading"])

    def test_unknown_options_are_preserved(self):
        self.assertEqual(
            ADAPTER.adapt(["compile", "--features", "html", "in.typ", "out.html"]),
            ["compile", "--features", "html", "in.typ", "out.html"],
        )


if __name__ == "__main__":
    unittest.main()
