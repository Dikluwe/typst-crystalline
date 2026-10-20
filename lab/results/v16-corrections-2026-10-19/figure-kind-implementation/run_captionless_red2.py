from pathlib import Path
import subprocess
p=Path(__file__).resolve().parent
commands=[['python3',str(p/'run_tests.py'),'--output',str(p/'captionless-block-red2-infra'),'--package','typst-infra','--filter','figure_captionless_default_block_','--red'],['python3',str(p/'run_tests.py'),'--output',str(p/'captionless-block-red2-core'),'--filter','figure_reference_visual_projection_keeps_dormant_template_only_in_queried_original','--red']]
results=[subprocess.run(c).returncode for c in commands]
assert results==[0,0],results
