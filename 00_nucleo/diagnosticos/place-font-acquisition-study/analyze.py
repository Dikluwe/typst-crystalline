from pathlib import Path
import json,statistics,subprocess,hashlib,datetime
from PIL import Image,ImageChops
R=Path(__file__).resolve().parent;ROOT=R.parents[2]
def save(p,a):p.write_text(json.dumps(a,indent=2)+'\n')
def median(v):return statistics.median(v)
controls=[];tables={};metrics=[]
for label,folder in [('Explicit font declaration',R),('Default font policy',R/'default-policy')]:
 samples=json.loads((folder/'samples.json').read_text());summary=json.loads((folder/'summary.json').read_text());table=[]
 for case in ['empty','shape','placed_shape','image','placed_image','text','placed_text']:
  cli={s['variant']:s for s in summary if s['kind']=='cli' and s['case']==case};probes=[s for s in samples if s['kind']=='probe' and s['case']==case and not s['preload']];preloads=[s for s in samples if s['kind']=='probe' and s['case']==case and s['preload']];first=[s['result']['rows'][0] for s in probes]
  record={'cohort':label,'case':case,'tekt_cli':cli['tekt'],'vanilla_cli':cli['vanilla'],'cold_pipeline_median_ms':median(x['pipeline_ms'] for x in first),'first_book_median_ms':median(x['book_ms'][0] if x['book_ms'] else 0 for x in first),'book_fraction_median':median(x['book_ms'][0]/x['pipeline_ms'] if x['book_ms'] else 0 for x in first),'book_calls_per_compile':sorted(set(x['book_calls'] for s in probes for x in s['result']['rows'])),'repeated_pipeline_median_ms':[median(s['result']['rows'][i]['pipeline_ms'] for s in probes) for i in [1,2]],'subsequent_book_max_ms':max([t for s in probes for x in s['result']['rows'][1:] for t in x['book_ms']] or [0]),'preload_plus_first_pipeline_median_ms':median(s['result']['preload_ms']+s['result']['rows'][0]['pipeline_ms'] for s in preloads),'probe_stability':[s for s in summary if s['kind']=='probe' and s['case']==case]}
  trace=json.loads((folder/'controls'/f'{case}-vanilla.trace.json').read_text());events=trace if isinstance(trace,list) else trace.get('traceEvents',[]);record['vanilla_discovery_begin_events']=sum(e.get('ph')=='B' and e.get('name')=='discover fonts' for e in events)
  table.append(f"| {case} | {cli['tekt']['distribution']['median_ms']:.2f} | {cli['vanilla']['distribution']['median_ms']:.2f} | {'stable' if cli['tekt']['stable'] and cli['vanilla']['stable'] else 'exploratory'} | {record['book_calls_per_compile']} | {record['vanilla_discovery_begin_events']} |")
  metrics.append(record)
  outputs={}
  for engine in ['tekt','vanilla']:
   pdf=folder/'controls'/f'{case}-{engine}.pdf';stem=folder/'controls'/f'{case}-{engine}-raster'
   subprocess.run(['pdftoppm','-r','72','-png','-singlefile',str(pdf),str(stem)],capture_output=True,check=True)
   image=Image.open(stem.with_suffix('.png')).convert('RGB');diff=ImageChops.difference(image,Image.new('RGB',image.size,'white')).convert('L').point(lambda x:255 if x>16 else 0)
   text=subprocess.check_output(['pdftotext',str(pdf),'-'],text=True);info=subprocess.check_output(['pdfinfo',str(pdf)],text=True);fonts=subprocess.check_output(['pdffonts',str(pdf)],text=True)
   outputs[engine]={'text':' '.join(text.split()),'page_fields':[x for x in info.splitlines() if x.startswith(('Pages:','Page size:'))],'nonwhite_bbox_72dpi':diff.getbbox(),'raster_sha256':hashlib.sha256(stem.with_suffix('.png').read_bytes()).hexdigest(),'pdffonts':fonts}
  equal=outputs['tekt']['text']==outputs['vanilla']['text'] and outputs['tekt']['page_fields']==outputs['vanilla']['page_fields'];controls.append({'cohort':label,'case':case,'same_text_and_page_fields':equal,'same_raster_bbox':outputs['tekt']['nonwhite_bbox_72dpi']==outputs['vanilla']['nonwhite_bbox_72dpi'],'outputs':outputs})
 tables[label]=table
save(R/'attribution-summary.json',metrics);save(R/'output-controls.json',controls)
report='''# Place font acquisition investigation

## Evidence before conclusion

Two declaration-policy cohorts were measured at HEAD `17f13b1df`, with the exact incoming working tree, UTC times, machine and toolchain in `place-font-acquisition-study/provenance.json`. No production code, L0, ADR or benchmark was changed. Release profiles, commands and binary identities are recorded in the artifact receipts and dataset. The pinned vanilla executable matches SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`, pin `9f2b6e8715237cb086899a42873660fe744622e8`, tree `6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4`; its previous build recipe receipt is referenced by hash.

The first cohort explicitly declares Libertinus Serif in both engines. The captured first demand is `eval_set_rule`, before placement. Tekt checks missing font families at `01_core/src/compiler/eval/rules.rs:3322`; vanilla performs the corresponding check at `lab/typst-original/crates/typst-library/src/text/mod.rs:1578`. Vanilla discovery appears even for empty/nontext inputs. Removing this check blindly would change observable warnings, rather than merely optimize geometry.

The second cohort removes only that declaration. The first placed-shape and placed-image demand is now `layout_sub_frame → vertical_metrics → resolve_primary → World::book`, captured in the control stderr. The source anchors are `01_core/src/compiler/layout/sub_frame.rs:139`, `03_infra/src/font_metrics/fallback.rs:186`, and `03_infra/src/font_metrics/resolution.rs:56`. The compensating lookup is at `01_core/src/compiler/layout/placement.rs:326`. Standalone empty, shape and image controls have zero decorated book calls. Vanilla traces have no discovery event for either standalone or placed nontext controls, but do discover fonts for text.

This establishes a font dependency caused by the current Tekt subframe-origin convention for these nontext placements. It does not establish an upstream intention or require copying vanilla execution mechanics. Vanilla constructs a placed frame through `lab/typst-original/crates/typst-layout/src/flow/collect.rs:645`; its per-child CachedCell is a separate reuse mechanism, not a disk catalog.

## Plain CLI results

Each CLI cell is the median of twelve admitted subprocess samples, including startup, setup, parsing, pipeline, export, output I/O and destruction. Each cohort has three rounds and four samples per case/engine. Pair order alternates. Do not subtract instrumented book intervals from these medians.

Explicit font declaration:

| Fixture | Tekt ms | Vanilla ms | Timing classification | Tekt book calls per compile | Vanilla discovery events in control |
| --- | ---: | ---: | --- | --- | ---: |
'''+ '\n'.join(tables['Explicit font declaration'])+'''

Default font policy:

| Fixture | Tekt ms | Vanilla ms | Timing classification | Tekt book calls per compile | Vanilla discovery events in control |
| --- | ---: | ---: | --- | --- | ---: |
'''+ '\n'.join(tables['Default font policy'])+'''

All fourteen explicit-declaration CLI cohorts passed the predefined timing criteria. None of the default-policy timing cohorts passed; their values are exploratory observations, not a stable speed ratio or promised optimization gain. Both cohort collections passed source/input/font/executable fingerprints and the monitored competing-process gate. Admission and timing stability are separate questions. Full distributions and round medians remain in each `summary.json`; rejected stability is not hidden by pooling cohorts or selective repetitions.

## Compatible attribution and reuse

The decorator's first-book interval is nested within the same public compile-to-PDF interval. It includes complete catalog acquisition, not isolated filesystem, cmap or shaping cost. `attribution-summary.json` records per-sample-derived fractions, cold pipeline distributions, first access, later accesses and preload-plus-pipeline totals. Setup/source acquisition and output write/destruction are outside this diagnostic pipeline interval. It must not be treated as equivalent to a plain CLI boundary.

For default-policy placed geometry, the catalog's first acquisition dominates the observed instrumented pipeline interval. The numerical fraction and elapsed values are descriptive because timing stability failed. Repeated compilations on the same world retain book accessor calls but do not repeat the large first-access interval. Catalog ownership is already once per world (`03_infra/src/world/fonts.rs:104`), not once per accessor. Other world/process caches also survive, so the whole warm compilation gain is not assigned to fonts. A public decorator does not count every possible internal catalog access or filesystem operation.

Explicit preloading moves acquisition outside the pipeline interval; its cost is retained and summed with the first pipeline call in the same sample. A lower preloaded pipeline time is not an end-to-end improvement. No disk persistence, cross-process reuse or production optimization was implemented. No vanilla same-world runtime experiment was run here; its LazyLock retention and reset behavior are source evidence, while the CLI traces are measured evidence for fresh processes.

## Output controls and limits

`output-controls.json` preserves extracted text, page fields, PDF font listings, raster hashes and nonwhite pixel bounding boxes at 72dpi. The controls expose existing differences: empty pages are A4 in Tekt instead of the requested 160 x 180pt; placed images have ink bounds (11, 0, 92, 74) instead of (11, 13, 92, 53); placed text has bounds (11, 5, 64, 13) instead of (11, 12, 64, 20). These differences recur in both declaration cohorts. Twelve of fourteen controls match extracted text/page fields; ten match raster ink bounds. These are bounded controls, not global geometry/parity or PDF byte identity. Empty-page, placed-image and placed-text timings must therefore not be used as successful comparisons of equivalent output, even where their timing distribution is stable. Placed-shape, standalone shape/image and ordinary text pass these bounded output controls.

Inspection finds `translate_frame_item` reconstructing images with `clip_rect: None` at `01_core/src/compiler/layout/helpers.rs:196`. This is a concrete candidate for the lost image clip; a RED-to-GREEN preservation test should establish causality before attributing the whole discrepancy to it. The text placement offset and empty-page configuration also need dedicated correctness controls. Raster differences do not independently identify their causes. Default text controls resolve Libertinus Serif Regular in both exports; available project/system fonts are fingerprinted. No claim of identical internal catalog ordering is made.

The monitor polls every 100ms for known build/compiler/benchmark processes. It cannot exclude shorter processes, desktop load, thermal/boost effects or all scheduler interference. The default-policy distributions failed stability despite no detected competing build/test/benchmark. File caches were warm, no OS caches were dropped, and this host's discovered font universe is part of the experiment. First-book time is not transferable to another machine or font installation without measurement.

## Next controlled repair

The candidate is lazy typography for a subframe's coordinate origin, with matching compensation in its callers. Changing only the initialization or only Place compensation would shift output. First retain the reproduced page-size, image-clip and placed-text differences as correctness regressions. Do not use a coordinate-origin optimization to hide them. Before implementation, establish RED controls with a rejecting FontMetrics provider for nontext placement while retaining text baselines. Cover nested Place/Align/Group, floats, decorations, auto dimensions and page transitions; preserve physical/logical alignment metadata rebasing. Update the private owners first, keep public signatures and product defaults, and run the existing geometry/layout suite. Then measure a separately archived before/after pair under a stable host interval, using these same fixtures. Accept avoided catalog demand independently from a quantified latency claim.

The explicit font-family validation path is not the target of that repair. Subsequent candidates include repeated face parsing after world reuse, but this study does not count or attribute those costs. This investigation identifies a bounded avoidable dependency, not a general proof that all font handling is rebuilt constantly.
'''
(R.parent/'place-font-acquisition-study.md').write_text(report)
print('Output controls',len(controls),'text/page matches',sum(x['same_text_and_page_fields'] for x in controls),'bbox matches',sum(x['same_raster_bbox'] for x in controls))
print('Mismatches',[(x['cohort'],x['case'],x['outputs']['tekt']['nonwhite_bbox_72dpi'],x['outputs']['vanilla']['nonwhite_bbox_72dpi']) for x in controls if not x['same_raster_bbox']])
