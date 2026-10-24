"""Contract tests for the diagnostic symbol classifier, independent of binaries."""
import importlib.util
from pathlib import Path
import unittest


ROOT = Path(__file__).resolve().parents[3]
SPEC = importlib.util.spec_from_file_location(
    'symbols_probe', ROOT / 'lab/parity/tools/p1495_symbols_probe.py')
probe = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(probe)

# Literal language witnesses: do not derive expectations from the probe's tables.
LEGACY = {
    'join': '⨝', 'join.r': '⟖', 'join.l': '⟕', 'join.l.r': '⟗',
    'gt.tri': '⊳', 'gt.tri.eq': '⊵', 'gt.tri.eq.not': '⋭', 'gt.tri.not': '⋫',
    'lt.tri': '⊲', 'lt.tri.eq': '⊴', 'lt.tri.eq.not': '⋬', 'lt.tri.not': '⋪',
}
JOIN_WARNING = '`join` is deprecated, use `bowtie.big` instead'


def result(value=None, *, exit=0, error=None, warnings=()):
    return {'exit': exit, 'value': value, 'error': error, 'warnings': list(warnings)}


def absent_legacy(path):
    error = 'unknown variable `join`' if path.startswith('join') else 'unknown symbol modifier'
    return result(exit=1, error=error)


def native_legacy(path, glyph):
    return result({'func': 'symbol', 'text': glyph},
                  warnings=[JOIN_WARNING] if path.startswith('join') else [])


class LegacySymbolGateTests(unittest.TestCase):
    def classify(self, path, current, oracle=None):
        return probe.verdict('bare-extension', 'math-bare.' + path,
                             absent_legacy(path) if oracle is None else oracle, current)

    def test_all_twelve_native_legacy_symbols_remain_extensions(self):
        for path, glyph in LEGACY.items():
            with self.subTest(path=path):
                self.assertEqual(self.classify(path, native_legacy(path, glyph)), 'EXTENSION')

    def test_equal_glyph_in_text_content_is_identity_drift(self):
        for path, glyph in LEGACY.items():
            current = native_legacy(path, glyph)
            current['value']['func'] = 'text'
            with self.subTest(path=path):
                self.assertEqual(self.classify(path, current), 'EXTENSION_DRIFT')

    def test_wrong_missing_and_malformed_symbol_payloads_still_fail(self):
        for path, glyph in LEGACY.items():
            for payload in [
                {'func': 'symbol', 'text': '?'}, {'func': 'symbol'},
                {'text': glyph}, {'func': None, 'text': glyph},
                {'func': 'equation', 'text': glyph}, glyph, None, {},
            ]:
                current = native_legacy(path, glyph)
                current['value'] = payload
                with self.subTest(path=path, payload=payload):
                    self.assertEqual(self.classify(path, current), 'EXTENSION_DRIFT')

    def test_missing_extra_and_changed_deprecation_warnings_still_fail(self):
        for path, glyph in LEGACY.items():
            invalid = [[], ['wrong warning'], [JOIN_WARNING, JOIN_WARNING]] if path.startswith('join') else [[JOIN_WARNING], ['wrong warning']]
            for warnings in invalid:
                current = native_legacy(path, glyph)
                current['warnings'] = warnings
                with self.subTest(path=path, warnings=warnings):
                    self.assertEqual(self.classify(path, current), 'EXTENSION_WARNING_RED')

    def test_missing_or_failed_product_is_not_an_extension(self):
        for current in [result(exit=1, error='failure'), result(exit=None),
                        result({'func': 'symbol', 'text': '⨝'}, error='failure')]:
            with self.subTest(current=current):
                self.assertEqual(self.classify('join', current), 'EXTENSION_MISSING')

    def test_oracle_absence_is_required_and_never_relabeled_as_parity(self):
        for path, glyph in LEGACY.items():
            wrong_oracles = [result({'func': 'symbol', 'text': glyph}),
                             result(exit=None), result(exit=1, error='other error'),
                             result(exit=1, error=absent_legacy(path)['error'], warnings=['unexpected'])]
            for oracle in wrong_oracles:
                with self.subTest(path=path, oracle=oracle):
                    self.assertEqual(self.classify(path, native_legacy(path, glyph), oracle), 'BASELINE_DEFECT')

    def test_qualified_extension_representation_and_namespace_warning_policy(self):
        for namespace, warnings in [('sym', [JOIN_WARNING]), ('math', [])]:
            oracle = result(exit=1, error=f'module `{namespace}` does not contain `join`')
            value = 'symbol("⨝", ("r", "⟖"), ("l", "⟕"), ("l.r", "⟗"))'
            with self.subTest(namespace=namespace):
                self.assertEqual(probe.verdict('extension', namespace + '.join', oracle,
                                              result(value, warnings=warnings)), 'EXTENSION')
                self.assertEqual(probe.verdict('extension', namespace + '.join', oracle,
                                              result('wrong', warnings=warnings)), 'EXTENSION_DRIFT')

    def test_regular_bare_parity_keeps_full_native_carrier_comparison(self):
        oracle = result({'func': 'symbol', 'text': 'α'})
        self.assertEqual(probe.verdict('bare', 'math-bare.alpha', oracle, oracle), 'MATCH')
        text = result({'func': 'text', 'text': 'α'})
        self.assertEqual(probe.verdict('bare', 'math-bare.alpha', oracle, text), 'TEXT_MATCH_CARRIER_DIFF')
        wrong = result({'func': 'symbol', 'text': 'β'})
        self.assertEqual(probe.verdict('bare', 'math-bare.alpha', oracle, wrong), 'RED')


if __name__ == '__main__':
    unittest.main()
