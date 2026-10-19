"""Recreate the standalone phase figure from accepted instrumented summaries."""
import json
from pathlib import Path
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
import numpy as np
OUT=Path(__file__).resolve().parent
p=json.loads((OUT/'phase-summary.json').read_text())
labels={'context.typ':'Context','image.typ':'Image','pipeline-single-font.typ':'Hello','shapes.typ':'Shapes','text-dense.typ':'Dense text'}
phases=[('eval_initial_ms','Initial eval','#4575b4'),('layout_stabilization_ms','Layout / stabilization','#e9a342'),('pdf_export_ms','Complete PDF export','#8064a2'),('unattributed_ms','Outside phase counters','#a7adb5')]
fig,ax=plt.subplots(figsize=(11.8,5.4));positions=np.arange(len(labels))*2.8
for engine,offset in [('tekt',-.47),('vanilla',.47)]:
    base=np.zeros(len(labels))
    for key,label,color in phases:
        values=np.array([p[name]['engine'][engine]['broad'][key]['mean'] for name in labels])
        ax.bar(positions+offset,values,bottom=base,width=.8,color=color,label=label if engine=='tekt' else None)
        base+=values
    for x,total in zip(positions+offset,base):ax.text(x,total+4,f'{total:.1f}',ha='center',fontsize=9,color='#303740')
ax.set_xticks([x+offset for x in positions for offset in [-.47,.47]],['Tekt','Vanilla']*len(labels),fontsize=9)
for x,label in zip(positions,labels.values()):ax.text(x,-.14,label,transform=ax.get_xaxis_transform(),ha='center',fontsize=11,fontweight='medium')
ax.set_ylabel('Mean process wall time (ms)');ax.set_ylim(0,340)
ax.set_title('Recorded phase envelopes and residual',loc='left',fontsize=15,pad=46)
ax.legend(loc='upper left',bbox_to_anchor=(0,1.13),ncol=2,frameon=False,fontsize=9)
ax.spines[['top','right']].set_visible(False);ax.set_axisbelow(True);ax.grid(axis='y',color='#e8ebef')
fig.text(.09,.04,'Instrumented population: 18 pairs per fixture. Phase boundaries differ; residual is not an isolated startup timer.',fontsize=9,color='#545c66')
fig.subplots_adjust(left=.09,right=.98,top=.77,bottom=.23)
fig.savefig(OUT/'phase-envelopes.png',dpi=180,facecolor='white');fig.savefig(OUT/'phase-envelopes.svg',facecolor='white')
