"""Summarize inclusive profile evidence; never treat overlapping shares as exclusive."""
from pathlib import Path
import gzip,json,collections,statistics,datetime
D=Path(__file__).resolve().parent
cpu=json.load(gzip.open(D/'cpu-style-current.json.gz','rt'));alloc=json.load(gzip.open(D/'stacks-current.json.gz','rt'))
needles={'font_select':'FontBook::select_pattern','primary_resolution':'::resolve_primary::','text_edges':'::text_edges::','vertical_metrics':'::vertical_metrics::','style_scope':'style_scope::layout_styled','style_collapse':'StyleChain::collapse','content_mapping':'::map_content_inner','table_change_proof':'TableElem::map_content_if_changed','cell_change_proof':'TableCellElem::map_content_if_changed','collect_context':'collect_context_blocks_into','realize_document':'realize::realize_document','cell_defaults':'table_cells::with_defaults','initial_layout':'initial_layout_independent'}
summary={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'scope':'inclusive stack presence, nearest project frame is not exclusive time; x excludes external DSOs; deterministic 1/64 allocation sample is not random and may alias call order','cpu':{},'allocations':{}}
for f in sorted({r['fixture'] for r in cpu['runs']}):
 counts=collections.Counter();first=collections.Counter();leaf=collections.Counter();per=[];total=0
 for row in cpu['runs']:
  if row['fixture']!=f:continue
  local=collections.Counter()
  for line in row['stacks']:
   frames=[cpu['symbols'][pc] for pc in line.split()];total+=1
   for k,needle in needles.items():
    if any(needle in frame for frame in frames):counts[k]+=1;local[k]+=1
   project=next((fr for fr in frames if fr.startswith(('typst_core::','typst_infra::','typst_shell::','typst_wiring::'))),'[no project frame]');first[project]+=1
  per.append({'rep':row['rep'],'total':len(row['stacks']),'anchors':dict(local)})
 summary['cpu'][f]={'total':total,'inclusive':{k:{'samples':counts[k],'fraction':counts[k]/total} for k in needles},'nearest_project_frame':first.most_common(25),'per_run':per}
for row in alloc['samples']:
 counts=collections.Counter();nearest=collections.Counter();requested=collections.Counter()
 for line in row['stacks']:
  tokens=line.split();frames=[alloc['symbols'][pc] for pc in tokens[2:]]
  for k,needle in needles.items():
   if any(needle in fr for fr in frames):counts[k]+=1;requested[k]+=int(tokens[1])
  project=next((fr for fr in frames if fr.startswith(('typst_core::','typst_infra::','typst_shell::','typst_wiring::'))),'[no project frame]');nearest[project]+=1
 summary['allocations'][f"{row['fixture']}-{row['rep']}"]={'header':row['header'],'sampled_count':len(row['stacks']),'inclusive':dict(counts),'sampled_requested_bytes_inclusive':dict(requested),'nearest_project_frame':nearest.most_common(25)}
(D/'analysis.json').write_text(json.dumps(summary,ensure_ascii=False,indent=2)+'\n')
for name,c in summary['cpu'].items():
 print(name,'samples',c['total'],'font',round(c['inclusive']['font_select']['fraction']*100,1),'%', 'resolve-primary',round(c['inclusive']['primary_resolution']['fraction']*100,1),'%', 'generic-map',round(c['inclusive']['content_mapping']['fraction']*100,1),'%')
 print(' nearest',c['nearest_project_frame'][:6])
for name,c in summary['allocations'].items():
 if name.endswith('-0'):print(name,c['header'],c['inclusive'],'nearest',c['nearest_project_frame'][:8])
