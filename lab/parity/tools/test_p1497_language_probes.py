"""Unit controls for the P1497 bilateral language-probe harness."""

import importlib.util
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch


HERE = Path(__file__).resolve().parent
SPEC = importlib.util.spec_from_file_location("p1497_language_probes", HERE / "p1497_language_probes.py")
probe = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = probe
SPEC.loader.exec_module(probe)


def obs(exit_code=0, **kwargs):
    return {"exit": exit_code, "error": None, "warnings": [], **kwargs}


class ClassificationTests(unittest.TestCase):
    def test_inventory_has_explicit_targets_and_controls(self):
        self.assertEqual(len(probe.CASES), 12)
        by_id = {case[0]: case for case in probe.CASES}
        self.assertEqual(by_id["fonts.empty-path"][4], ("fonts", "--font-path", "", "--ignore-system-fonts"))
        self.assertEqual(by_id["stroke.invalid-control"][3], "repr(stroke(42))")
        self.assertEqual(by_id["html.mathml"][2], "html")
        self.assertEqual(by_id["svg.foreign-object"][2], "pdf/svg-source")
        for name in ("fonts.no-path-control", "completions.bash-control", "stroke.named-control", "stroke.invalid-control", "html.plain-control", "svg.invalid-bytes-control"):
            self.assertIn(name, by_id)

    def test_empty_font_path_rejected_is_violation_even_if_control_matches(self):
        verdict, _ = probe.verdict(
            "fonts.empty-path", obs(families=["A", "B"]),
            obs(2, families=None, error="a value is required for '--font-path <DIR>'"),
        )
        self.assertEqual(verdict, "VIOLATED")
        verdict, _ = probe.verdict(
            "fonts.no-path-control", obs(families=["A", "B"]), obs(families=["B", "A"]),
        )
        self.assertEqual(verdict, "VIOLATED")
        verdict, _ = probe.verdict(
            "fonts.no-path-control", obs(families=["A", "B"]), obs(families=["A", "B"]),
        )
        self.assertEqual(verdict, "MATCH")

    def test_bash_match_is_limited_to_valid_registered_script(self):
        vanilla = obs(bash_syntax_valid=True, bash_registered=True, stdout_sha256="different-a")
        crystal = obs(bash_syntax_valid=True, bash_registered=True, stdout_sha256="different-b")
        verdict, reason = probe.verdict("completions.bash-control", vanilla, crystal)
        self.assertEqual(verdict, "MATCH")
        self.assertIn("content is not compared", reason)
        crystal["bash_syntax_valid"] = False
        self.assertEqual(probe.verdict("completions.bash-control", vanilla, crystal)[0], "VIOLATED")

    def test_nushell_missing_is_violation(self):
        vanilla = obs(nushell_entrypoint=True)
        crystal = obs(2, error="invalid value 'nushell' for '<SHELL>'")
        self.assertEqual(probe.verdict("completions.nushell", vanilla, crystal)[0], "VIOLATED")

    def test_stroke_roundtrip_and_named_control(self):
        vanilla = obs(value="stroke()")
        crystal = obs(1, value=None, error="stroke() não aceita argumentos posicionais")
        self.assertEqual(probe.verdict("stroke.default-roundtrip", vanilla, crystal)[0], "VIOLATED")
        self.assertEqual(probe.verdict("stroke.named-control", vanilla, vanilla)[0], "MATCH")
        same_failure = obs(1, value=None, error="same error")
        self.assertEqual(probe.verdict("stroke.default-roundtrip", same_failure, same_failure)[0], "VIOLATED")
        self.assertEqual(probe.verdict("stroke.explicit-roundtrip", same_failure, same_failure)[0], "VIOLATED")

    def test_invalid_stroke_requires_bilateral_rejection_without_diagnostic_credit(self):
        vanilla = obs(1, error="expected stroke input")
        crystal = obs(1, error="positional arguments unsupported")
        verdict, reason = probe.verdict("stroke.invalid-control", vanilla, crystal)
        self.assertEqual(verdict, "MATCH")
        self.assertIn("diagnostic wording is not compared", reason)
        self.assertEqual(probe.verdict("stroke.invalid-control", vanilla, obs(value="42"))[0], "VIOLATED")

    def test_mathml_is_disabled_only_for_known_scope_error(self):
        vanilla = obs(body_tokens=["<math>", "x", "</math>"], has_mathml=True)
        crystal = obs(1, error="HTML export does not yet support this content")
        self.assertEqual(probe.verdict("html.mathml", vanilla, crystal)[0], "DISABLED_BY_PROFILE")
        crystal["error"] = "some other error"
        self.assertEqual(probe.verdict("html.mathml", vanilla, crystal)[0], "VIOLATED")
        crystal = obs(body_tokens=vanilla["body_tokens"], has_mathml=True)
        with self.assertRaises(probe.ProbeError):
            probe.verdict("html.mathml", vanilla, crystal)

    def test_plain_html_compares_body_not_artifact_bytes(self):
        vanilla = obs(body_tokens=["<p>", "Hello", "</p>"], artifact_present=True, body_seen=True, artifact_sha256="a")
        crystal = obs(body_tokens=["<p>", "Hello", "</p>"], artifact_present=True, body_seen=True, artifact_sha256="b")
        verdict, reason = probe.verdict("html.plain-control", vanilla, crystal)
        self.assertEqual(verdict, "MATCH")
        self.assertIn("attributes and bytes not compared", reason)
        crystal["body_tokens"] = ["<p>", "Goodbye", "</p>"]
        self.assertEqual(probe.verdict("html.plain-control", vanilla, crystal)[0], "VIOLATED")

    def test_plain_html_requires_artifact_body_and_nonempty_tokens(self):
        good = obs(body_tokens=["<p>", "Hello", "</p>"], artifact_present=True, body_seen=True)
        for missing in (
            obs(body_tokens=[], artifact_present=False, body_seen=False),
            obs(body_tokens=[], artifact_present=True, body_seen=True),
            obs(body_tokens=["Hello"], artifact_present=True, body_seen=False),
        ):
            self.assertEqual(probe.verdict("html.plain-control", good, missing)[0], "VIOLATED")

    def test_svg_scope_out_does_not_equalize_different_errors(self):
        vanilla = obs(pdf_emitted=True, warnings=["image contains foreign object"])
        crystal = obs(1, error="argumento nomeado inesperado em image(): 'format'", pdf_emitted=False)
        verdict, reason = probe.verdict("svg.foreign-object", vanilla, crystal)
        self.assertEqual(verdict, "DISABLED_BY_PROFILE")
        self.assertIn("no warning/render parity credit", reason)
        invalid = obs(1, error="failed to parse SVG", pdf_emitted=False)
        self.assertEqual(probe.verdict("svg.invalid-bytes-control", invalid, crystal)[0], "DISABLED_BY_PROFILE")
        crystal["error"] = "a different decoder error"
        self.assertEqual(probe.verdict("svg.invalid-bytes-control", invalid, crystal)[0], "VIOLATED")

    def test_html_body_parser_ignores_head_but_keeps_structure(self):
        parser = probe.BodyTokens()
        parser.feed("<html><head><style>x</style></head><body><p>A <strong>B</strong></p></body></html>")
        self.assertEqual(parser.tokens, ["<p>", "A", "<strong>", "B", "</strong>", "</p>"])
        self.assertTrue(parser.body_seen)

    def test_cli_observation_keeps_hash_not_completion_body(self):
        result = subprocess.CompletedProcess(["typst", "completions", "bash"], 0, b"_typst() {\ncomplete -F _typst typst\n}\n", b"")
        with patch.object(probe.subprocess, "run", return_value=subprocess.CompletedProcess(["bash", "-n"], 0, b"", b"")):
            observation = probe.observe("completions.bash-control", result, None)
        self.assertIn("stdout_sha256", observation)
        self.assertNotIn("stdout", observation)
        self.assertTrue(observation["bash_registered"])

    def test_temporary_paths_do_not_change_diagnostic_hash(self):
        first = subprocess.CompletedProcess([], 1, b"", b"error: /tmp/one/probe.typ failed\n")
        second = subprocess.CompletedProcess([], 1, b"", b"error: /tmp/two/probe.typ failed\n")
        a = probe.observe("svg.foreign-object", first, Path("/tmp/one/out.pdf"))
        b = probe.observe("svg.foreign-object", second, Path("/tmp/two/out.pdf"))
        self.assertEqual(a["stderr_normalized_sha256"], b["stderr_normalized_sha256"])

    def test_compile_uses_temp_output_not_literal_dash(self):
        commands = []

        def fake_execute(argv, *, env):
            commands.append(argv)
            Path(argv[-1]).write_text("<html><body><p>Hello, parity.</p></body></html>", encoding="utf-8")
            return subprocess.CompletedProcess(argv, 0, b"", b"")

        with tempfile.TemporaryDirectory() as directory, patch.object(probe, "execute", side_effect=fake_execute):
            case = next(item for item in probe.CASES if item[0] == "html.plain-control")
            result = probe.run_case(case, Path("/bin/vanilla"), Path("/bin/crystalline"), Path(directory), {})
        self.assertEqual(result["status"], "MATCH")
        self.assertTrue(all(command[-1] != "-" for command in commands))
        self.assertEqual(result["crystalline"]["argv_template"][-2:], ["-o", "<temporary-output>"])

    def test_temporary_sources_use_explicit_tmp_root(self):
        with patch.object(probe.tempfile, "TemporaryDirectory") as temporary:
            marker = object()
            temporary.return_value = marker
            self.assertIs(probe.probe_scratch_dir(), marker)
            temporary.assert_called_once_with(prefix="p1497-language-", dir="/tmp")


if __name__ == "__main__":
    unittest.main()
