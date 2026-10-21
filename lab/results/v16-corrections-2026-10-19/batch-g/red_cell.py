"""Capture the measured Cell block-flow failures before its productive repair."""
from pathlib import Path
import subprocess

P = Path(__file__).resolve().parent
subprocess.run(['python3', str(P / 'reseal.py'), '--output', str(P / 'reseal-cell-before-implementation')], check=True)
subprocess.run(['python3', str(P / 'run_tests.py'), '--output', 'focused-cell-red-infra',
                '--package', 'typst-infra', '--expected-total', '36',
                '--expected-fail', 'batch_g_html_root_cell_uses_selected_block_flow',
                '--expected-fail', 'batch_g_html_sequence_cell_flushes_neighboring_paragraphs'], check=True)
