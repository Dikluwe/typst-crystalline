from pathlib import Path
import json,statistics,math,matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
R=Path(__file__).resolve().parents[1]
m=json.loads((R/'micro-summary.json').read_text());e=json.loads((R/'e2e-summary.json').read_text());p=json.loads((R/'phase-summary.json').read_text())
scanner={};ratios=[]
for key,row in m.items():
 if key.startswith('scanner_bench/'):
  rows=[]
  for x in row['rounds']:
   bm=json.loads((R/x['raw_sample']).with_name('benchmark.json').read_text());size=bm['throughput']['Bytes'];rows.append({'round':x['round'],'bytes':size,'distribution_ns_per_byte':{k:v/size if k!='n' else v for k,v in x['distribution_ns'].items()}})
  scanner[key]={'rounds':rows,'stable_absolute':row['stable_absolute']}
 if key.startswith('fonts/') and '/tekt_' in key:
  prefix=key.rsplit('/',1)[0];other=[k for k in m if k.startswith(prefix+'/vanilla_')]
  if other:
   assert len(other)==1;ratios.append({'tekt_case':key,'vanilla_case':other[0],'ratios_per_round':[a['distribution_ns']['mean']/b['distribution_ns']['mean'] for a,b in zip(row['rounds'],m[other[0]]['rounds'])],'both_stable':row['stable_absolute'] and m[other[0]]['stable_absolute']})
(R/'scanner-summary.json').write_text(json.dumps(scanner,indent=2)+'\n');(R/'font-comparisons.json').write_text(json.dumps(ratios,indent=2)+'\n')
cli_stability={}
for fixture,row in e.items():
 cli_stability[fixture]={}
 for engine in ['tekt','vanilla']:
  means=[row['per_round'][str(n)][engine]['mean'] for n in [1,2,3]];median=statistics.median(means);diff=100*abs(statistics.mean(means)-median)/median;spread=100*(max(means)-min(means))/median;cli_stability[fixture][engine]={'round_means_ms':means,'mean_median_difference_pct':diff,'range_pct':spread,'stable_absolute':diff<3 and spread<=5}
(R/'cli-stability.json').write_text(json.dumps(cli_stability,indent=2)+'\n')
fig,ax=plt.subplots(figsize=(10,6));colors=['#4477AA','#66CCEE','#228833','#BBBBBB'];keys=['eval_ms','stabilization_layout_ms','pdf_complete_ms','residual_ms'];labels=['Initial eval','Stabilization/layout incl. lazy acquisition','Complete PDF export','Residual']
for index,fixture in enumerate(p):
 for j,engine in enumerate(['tekt','vanilla']):
  y=index*2.5+j;left=0
  for key,color in zip(keys,colors):
   val=p[fixture]['engines'][engine][key]['mean'];ax.barh(y,val,left=left,color=color,label=labels[keys.index(key)] if index==0 and j==0 else None);left+=val
ax.set_yticks([i*2.5+j for i in range(len(p)) for j in [0,1]],[f'{name} / {engine}' for name in p for engine in ['Tekt','vanilla']]);ax.invert_yaxis();ax.set_xlabel('Mean time (ms), instrumented population only');ax.set_title('Broad phase envelopes after infra corrections');ax.legend(loc='lower right',fontsize=8);ax.grid(axis='x',alpha=.15);fig.tight_layout();fig.savefig(R/'phase-envelopes.png',dpi=160);fig.savefig(R/'phase-envelopes.svg');plt.close(fig)
print('Scanner normalized, descriptive font ratios and CLI stability recorded; phase figure generated.')
