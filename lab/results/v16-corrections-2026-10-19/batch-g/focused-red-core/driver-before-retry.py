"""Run the declared real semantic RED controls on unchanged productive bodies."""
from pathlib import Path
import json,subprocess
P=Path(__file__).resolve().parent
core=['batch_g_title_same_kind_compares_body','batch_g_html_frame_same_kind_compares_body','batch_g_html_same_kind_preserves_attrs_order_presence_and_body_state','batch_g_context_hub_equality_uses_owned_id_without_executing_closure','batch_g_context_language_is_opaque_and_preserves_original_identity','batch_g_context_language_reaches_suppressed_source_and_located_roots','batch_g_context_language_projects_nominal_container_children','batch_g_context_language_containers_and_operators_share_opaque_relation']
html=json.loads((P/'agents/html/tests-ready.json').read_text())
infra=[t['name'] for t in html['tests'] if t['expected_old_result']=='functional RED']
assert len(core)==8 and len(infra)==9
for package,name,total,failures in [('typst-core','focused-red-core',17,core),('typst-infra','focused-red-infra',27,infra)]:
    argv=['python3',str(P/'run_tests.py'),'--output',name,'--package',package,'--expected-total',str(total)]
    for failure in failures:argv+=['--expected-fail',failure]
    subprocess.run(argv,check=True)
