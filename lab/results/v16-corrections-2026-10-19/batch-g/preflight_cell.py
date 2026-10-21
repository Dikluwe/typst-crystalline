"""Check the unchanged HTML policy before repeating the full final gates."""
from pathlib import Path
import subprocess

P = Path(__file__).resolve().parent
subprocess.run(['python3', str(P / 'build.py'), '--output', str(P / 'candidate-cell-build')], check=True)
subprocess.run(['python3', str(P / 'agents/html/replay_cli.py'),
                '--product', 'lab/target/product/debug/typst',
                '--build-receipt', str(P / 'candidate-cell-build/receipt.json'),
                '--output', str(P / 'html-cell-green')], check=True)
