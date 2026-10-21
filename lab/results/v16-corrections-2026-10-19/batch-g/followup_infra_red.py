"""Capture semantic footer RED after public-facade-only harness correction."""
from pathlib import Path
import subprocess
P=Path(__file__).resolve().parent
subprocess.run(['python3',str(P/'reseal.py'),'--output',str(P/'reseal-followup-infra-harness')],check=True)
subprocess.run(['python3',str(P/'run_tests.py'),'--output','focused-followup-red-infra-2','--package','typst-infra','--expected-total','32','--expected-fail','batch_g_html_raw_footer_projection_has_no_unemitted_warning'],check=True)
