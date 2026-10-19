from pathlib import Path
import tempfile
import unittest

from check_mature_layout import check_layout, expected_owner_path


class MatureLayoutTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        for directory in (
            ".cargo", "00_nucleo/adr", "00_nucleo/prompts/core", "01_core/src",
            "00_nucleo/diagnostics", "02_shell/src", "03_infra/src", "04_wiring/src",
            "lab/benches", "lab/tools/ci", "lab/history/construction/records", "lab/results",
        ):
            (self.root / directory).mkdir(parents=True, exist_ok=True)
        (self.root / "CLAUDE.md").write_text("# instructions\n")
        (self.root / "AGENTS.md").symlink_to("CLAUDE.md")
        (self.root / ".cargo/config.toml").write_text('[build]\ntarget-dir = "lab/target/product"\n')
        (self.root / "Cargo.toml").write_text('[workspace]\nmembers = ["01_core", "lab/benches"]\n')
        (self.root / "lab/benches/Cargo.toml").write_text('[package]\nworkspace = "../.."\n')
        (self.root / "crystalline.toml").write_text("[excluded_files]\n")
        (self.root / "00_nucleo/prompts/core/unit.md").write_text("# Unit\n")
        (self.root / "01_core/src/unit.rs").write_text("//! @prompt 00_nucleo/prompts/core/unit.md\nfn unit() {}\n")
        (self.root / "README.md").write_text("[instructions](CLAUDE.md)\n")

    def test_valid_mature_layout_passes(self):
        self.assertEqual(check_layout(self.root), [])

    def test_old_root_path_and_bad_benchmark_membership_are_detected(self):
        (self.root / "tools").mkdir()
        (self.root / "Cargo.toml").write_text('[workspace]\nmembers = ["benches"]\n')
        (self.root / "lab/benches/Cargo.toml").write_text('[package]\nworkspace = ".."\n')
        codes = {finding.code for finding in check_layout(self.root)}
        self.assertIn("OLD_ROOT_PATH_PRESENT", codes)
        self.assertIn("CARGO_BENCH_MEMBER", codes)
        self.assertIn("CARGO_BENCH_WORKSPACE", codes)

    def test_missing_active_markdown_link_fails_but_code_references_are_ignored(self):
        (self.root / "README.md").write_text(
            "[missing](does-not-exist.md)\n\n"
            "Inline ` [also-missing](inline.md) ` stays literal.\n\n"
            "```md\n[example](fenced.md)\n```\n"
        )
        findings = check_layout(self.root)
        links = [finding for finding in findings if finding.code == "MARKDOWN_LINK_MISSING"]
        self.assertEqual(len(links), 1)
        self.assertIn("does-not-exist.md", links[0].message)
        (self.root / "README.md").write_text("![missing image](missing.png)\n")
        self.assertIn("MARKDOWN_LINK_MISSING", {finding.code for finding in check_layout(self.root)})
        (self.root / "README.md").write_text("[missing-ref]: missing-ref.md\n")
        self.assertIn("MARKDOWN_LINK_MISSING", {finding.code for finding in check_layout(self.root)})

    def test_markdown_link_cannot_escape_repository(self):
        (self.root / "README.md").write_text("[outside](../outside.md)\n")
        self.assertIn("MARKDOWN_LINK_ESCAPES_ROOT", {finding.code for finding in check_layout(self.root)})

    def test_duplicate_prompt_owner_is_reported(self):
        (self.root / "02_shell/src/other.rs").write_text(
            "//! @prompt 00_nucleo/prompts/core/unit.md\nfn other() {}\n"
        )
        findings = check_layout(self.root)
        self.assertIn("OWNER_DUPLICATE", {finding.code for finding in findings})

    def test_product_rust_source_cannot_include_lab_source(self):
        (self.root / "lab/benches/source.rs").write_text("fn fixture() {}\n")
        (self.root / "01_core/src/illicit.rs").write_text(
            '#[path = "../../lab/benches/source.rs"]\nmod fixture;\n'
        )
        findings = check_layout(self.root)
        self.assertIn("PRODUCT_IMPORTS_LAB", {finding.code for finding in findings})

    def test_include_literal_paths_must_exist_and_lab_fixtures_are_allowed(self):
        fixture = self.root / "lab/corpus/legacy-sections/sec_04.typ"
        fixture.parent.mkdir(parents=True)
        fixture.write_text("= Legacy section\n")
        source = self.root / "03_infra/src/includes.rs"
        source.write_text(
            'const FIXTURE: &str = include_str!("../../lab/corpus/legacy-sections/sec_04.typ");\n'
            'const FIXTURE_BYTES: &[u8] = include_bytes!(r#"../../lab/corpus/legacy-sections/sec_04.typ"#);\n'
            'const ABSENT: &[u8] = include_bytes!("missing.bin");\n'
        )
        findings = check_layout(self.root)
        include_findings = [finding for finding in findings if finding.code.startswith("INCLUDE_PATH_")]
        self.assertEqual(len(include_findings), 1)
        self.assertEqual(include_findings[0].code, "INCLUDE_PATH_MISSING")
        self.assertIn("missing.bin", include_findings[0].message)

    def test_missing_benchmark_fixture_include_is_reported(self):
        source = self.root / "lab/benches/pdf_builder_bench.rs"
        source.write_text(
            'const IMAGE: &[u8] = include_bytes!("../../03_infra/fixtures/p307b/sources/missing.jpg");\n'
        )
        findings = check_layout(self.root)
        include_findings = [finding for finding in findings if finding.code == "INCLUDE_PATH_MISSING"]
        self.assertEqual(len(include_findings), 1)
        self.assertEqual(include_findings[0].source, "lab/benches/pdf_builder_bench.rs")
        self.assertIn("missing.jpg", include_findings[0].message)

    def test_dynamic_include_is_not_treated_as_a_verified_literal(self):
        from check_mature_layout import count_dynamic_include_paths

        source = self.root / "03_infra/src/dynamic.rs"
        source.write_text(
            'const GENERATED: &str = include_str!(concat!(env!("OUT_DIR"), "/generated.txt"));\n'
        )
        self.assertEqual(count_dynamic_include_paths(self.root), 1)
        self.assertNotIn("INCLUDE_PATH_MISSING", {finding.code for finding in check_layout(self.root)})

    def test_active_links_cover_nucleo_diagnostics_and_lab_tool_docs_only(self):
        scoped = {
            "00_nucleo/README.md": "nucleo",
            "00_nucleo/diagnostics/current.md": "diagnostic",
            "lab/README.md": "lab",
            "lab/history/README.md": "history",
            "lab/tools/ci/README.md": "tools",
        }
        for relative, label in scoped.items():
            path = self.root / relative
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(f"[{label}](missing-{label}.md)\n")
        (self.root / "lab/history/construction/records/old.md").write_text("[old](missing-old.md)\n")
        (self.root / "lab/results/old.md").write_text("[result](missing-result.md)\n")

        findings = [finding for finding in check_layout(self.root) if finding.code == "MARKDOWN_LINK_MISSING"]
        sources = {finding.source for finding in findings}
        self.assertEqual(sources, set(scoped))

    def test_exact_link_ledger_exception_is_consumed_and_stale_rows_fail(self):
        (self.root / "README.md").write_text("[missing](missing.md)\n")
        ledger = self.root / "known-links.tsv"
        ledger.write_text("README.md\t1\tmissing.md\n")
        self.assertEqual(check_layout(self.root, ledger), [])
        (self.root / "README.md").write_text("[fixed](CLAUDE.md)\n")
        self.assertIn("LINK_LEDGER_STALE", {finding.code for finding in check_layout(self.root, ledger)})

    def test_prompt_owner_must_resolve_and_use_canonical_path(self):
        source = self.root / "01_core/src/unit.rs"
        source.write_text("//! @prompt missing.md\nfn unit() {}\n")
        self.assertIn("OWNER_PATH_NONCANONICAL", {finding.code for finding in check_layout(self.root)})

    def test_owner_layout_preserves_module_test_and_crate_root_structure(self):
        pairs = {
            "01_core/src/compiler/layout/mod.rs": "core/compiler/layout/mod.md",
            "01_core/src/lib.rs": "core/lib.md",
            "02_shell/build.rs": "shell/build.md",
            "03_infra/tests/font_loading.rs": "infra/tests/font_loading.md",
            "04_wiring/src/main.rs": "wiring/main.md",
            "01_core/src/compiler/src/unit.rs": "core/compiler/src/unit.md",
        }
        for source, owner in pairs.items():
            with self.subTest(source=source):
                self.assertEqual(expected_owner_path(Path(source)), Path("00_nucleo/prompts", owner))

    def test_owner_layout_rejects_wrong_layer_and_retained_leading_src(self):
        source = self.root / "01_core/src/unit.rs"
        for owner in ("infra/unit.md", "core/src/unit.md"):
            with self.subTest(owner=owner):
                prompt = self.root / "00_nucleo/prompts" / owner
                prompt.parent.mkdir(parents=True, exist_ok=True)
                prompt.write_text("# Unit\n")
                source.write_text(f"//! @prompt 00_nucleo/prompts/{owner}\nfn unit() {{}}\n")
                findings = check_layout(self.root)
                self.assertIn("OWNER_LAYOUT_MISMATCH", {finding.code for finding in findings})
                self.assertNotIn("OWNER_PATH_MISSING", {finding.code for finding in findings})

    def test_missing_or_duplicate_owner_header_cannot_bypass_layout(self):
        source = self.root / "01_core/src/unit.rs"
        for header in ("", "//! @prompt 00_nucleo/prompts/core/unit.md\n" * 2):
            source.write_text(header + "fn unit() {}\n")
            self.assertIn("OWNER_HEADER_COUNT", {finding.code for finding in check_layout(self.root)})

    def test_excluded_files_do_not_require_prompt_ownership(self):
        source = self.root / "04_wiring/tests/p1286_contract.rs"
        source.parent.mkdir(parents=True)
        source.write_text("fn test() {}\n")
        (self.root / "crystalline.toml").write_text(
            '[excluded_files]\ncontract = "04_wiring/tests/p1286_contract.rs"\n'
        )
        self.assertEqual(check_layout(self.root), [])
        source.rename(source.with_name("other.rs"))
        self.assertIn("OWNER_HEADER_COUNT", {finding.code for finding in check_layout(self.root)})

    def test_step_filenames_fail_only_for_active_consumers_and_their_owners(self):
        for name in ("p1405_outline_indented", "passo-537b-set-page-columns"):
            source = self.root / "01_core/src" / f"{name}.rs"
            prompt = self.root / "00_nucleo/prompts/core" / f"{name}.md"
            source.write_text(f"//! @prompt 00_nucleo/prompts/core/{name}.md\nfn unit() {{}}\n")
            prompt.write_text("# Unit\n")
            codes = {finding.code for finding in check_layout(self.root)}
            self.assertIn("OWNER_SOURCE_STEP_NAME", codes)
            self.assertIn("OWNER_PROMPT_STEP_NAME", codes)
            source.unlink()
            prompt.unlink()
        (self.root / "lab/history/construction/records/p1405.md").write_text("# Historical step\n")
        (self.root / "lab/benches/p1405_fixture.rs").write_text("// fixture\n")
        nucleus = self.root / "00_nucleo/prompts/_nuclei/path-identity.toml"
        nucleus.parent.mkdir()
        nucleus.write_text('tekt = 1\nkind = "nucleus"\n')
        self.assertEqual(check_layout(self.root), [])

    def test_src_omission_collision_is_reported(self):
        (self.root / "01_core/unit.rs").write_text(
            "//! @prompt 00_nucleo/prompts/core/unit.md\nfn unit() {}\n"
        )
        self.assertIn("OWNER_LAYOUT_COLLISION", {finding.code for finding in check_layout(self.root)})


if __name__ == "__main__":
    unittest.main()
