"""Summaries of admitted populations only; no benchmark or production writes."""
from pathlib import Path
import json,statistics,collections,importlib.util,subprocess,re
R=Path(__file__).resolve().parent;ROOT=R.parents[2]
spec=importlib.util.spec_from_file_location('sealed_stats',ROOT/'00_nucleo/diagnosticos/performance-baseline/analyze.py');old=importlib.util.module_from_spec(spec);spec.loader.exec_module(old)
def save(name,data):(R/name).write_text(json.dumps(data,indent=2)+'\n')
def main():
 micro=collections.defaultdict(list);cli=[];valid=[]
 for block in sorted((R/'blocks').glob('*/a*')):
  receipt=json.loads((block/'receipt.json').read_text())
  if not receipt['valid']:continue
  valid.append(str(block.relative_to(R)))
  for p in block.rglob('new/sample.json'):
   sample=json.loads(p.read_text());bm=json.loads((p.parent/'benchmark.json').read_text());kind='fonts' if block.parent.name.startswith('font-') else p.relative_to(block).parts[0];key=kind+'/'+bm['full_id'];values=[t/i for t,i in zip(sample['times'],sample['iters'])];micro[key].append({'round':block.parent.name,'distribution_ns':old.distribution(values),'raw_sample':str(p.relative_to(R)),'estimates':json.loads((p.parent/'estimates.json').read_text()),'sampling_mode':sample.get('sampling_mode')})
  for p in block.rglob('samples.json'):cli.extend(json.loads(p.read_text()))
 summary={}
 for key,rows in micro.items():
  assert len(rows)==3,(key,len(rows));means=[x['distribution_ns']['mean'] for x in rows];med=statistics.median(means);diff=100*abs(statistics.mean(means)-med)/med;spread=100*(max(means)-min(means))/med;summary[key]={'rounds':rows,'mean_round_ns':statistics.mean(means),'mean_median_difference_pct':diff,'round_range_pct':spread,'stable_absolute':diff<3 and spread<=5}
 save('micro-summary.json',summary)
 plain={};phases={};phase_rows=[]
 for fixture in sorted({r['fixture'] for r in cli}):
  rows=[r for r in cli if r['fixture']==fixture and not r['instrumented']];engines={engine:old.distribution([x['wall_ms'] for x in rows if x['engine']==engine]) for engine in ['tekt','vanilla']};paired=[]
  for group in sorted({(x['round'],x['pair']) for x in rows}):
   pair={x['engine']:x for x in rows if (x['round'],x['pair'])==group};paired.append({'round':group[0],'pair':group[1],'delta_ms':pair['tekt']['wall_ms']-pair['vanilla']['wall_ms'],'ratio':pair['tekt']['wall_ms']/pair['vanilla']['wall_ms']})
  plain[fixture]={'engines':engines,'paired':paired,'paired_delta_ms':old.distribution([x['delta_ms'] for x in paired]),'paired_ratio':old.distribution([x['ratio'] for x in paired]),'per_round':{str(n):{engine:old.distribution([x['wall_ms'] for x in rows if x['round']==n and x['engine']==engine]) for engine in ['tekt','vanilla']} for n in [1,2,3]}}
  traced=[r for r in cli if r['fixture']==fixture and r['instrumented']]
  for row in traced:
   if row['engine']=='tekt':
    t=json.loads((R/row['trace']).read_text());b={'eval_ms':t['eval_ms'],'stabilization_layout_ms':t['introspect_ms']+t['expand_context_ms']+t['layout_ms']+t['shape_ms'],'pdf_complete_ms':t['subset_ms']+t['render_ms']}
   else:
    t=old.trace_summary(R/row['trace']);b={'eval_ms':t['eval_initial_ms'],'stabilization_layout_ms':t['layout_stabilization_ms'],'pdf_complete_ms':t['pdf_export_ms']}
   b['residual_ms']=row['wall_ms']-sum(b.values());phase_rows.append(row|{'broad':b,'detail':t})
  data=[x for x in phase_rows if x['fixture']==fixture];engines={engine:{k:old.distribution([x['broad'][k] for x in data if x['engine']==engine]) for k in data[0]['broad']} for engine in ['tekt','vanilla']};delta={k:engines['tekt'][k]['mean']-engines['vanilla'][k]['mean'] for k in data[0]['broad']};wall=statistics.mean([x['wall_ms'] for x in data if x['engine']=='tekt'])-statistics.mean([x['wall_ms'] for x in data if x['engine']=='vanilla']);assert abs(sum(delta.values())-wall)<1e-6;phases[fixture]={'engines':engines,'mean_delta_ms':delta,'mean_wall_delta_ms':wall}
 save('e2e-summary.json',plain);save('phase-samples.json',phase_rows);save('phase-summary.json',phases);save('accepted-blocks.json',valid)
 # Output controls inspect one plain pair per fixture and round; no global parity claim.
 controls=[]
 for fixture in plain:
  for n in [1,2,3]:
   pair=[x for x in cli if x['fixture']==fixture and x['round']==n and x['pair']==0 and not x['instrumented']];outputs={}
   for row in pair:
    p=R/row['output'];info=subprocess.check_output(['pdfinfo',str(p)],text=True);text=subprocess.check_output(['pdftotext',str(p),'-'],text=True);fields={k:re.search(r'(?m)^'+k+r':\s*(.*)$',info).group(1) for k in ['Pages','Page size']};outputs[row['engine']]={'fields':fields,'normalized_text':' '.join(text.split())}
   fields_equal=outputs['tekt']['fields']==outputs['vanilla']['fields'];text_equal=outputs['tekt']['normalized_text']==outputs['vanilla']['normalized_text'];controls.append({'fixture':fixture,'round':n,'outputs':outputs,'page_fields_equal':fields_equal,'text_equal':text_equal})
 save('output-controls.json',controls)
 print('Cases',len(summary),'stable',sum(x['stable_absolute'] for x in summary.values()),'CLI fixtures',len(plain),'all sampled output controls',all(x['page_fields_equal'] and x['text_equal'] for x in controls))
if __name__=='__main__':main()
