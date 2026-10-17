from pathlib import Path
import gzip,json,hashlib,datetime,subprocess
out=Path('00_nucleo/diagnosticos/paint-root-2026-10-14');stage=out/'observer-cleanup';stage.mkdir(exist_ok=True)
for p in list(out.glob('followup-*'))+[out/'candidate-final.json.gz',out/'pre-validation.json.gz',out/'seal.log']:
 (stage/('before-'+p.name)).write_bytes(p.read_bytes())
source=Path('01_core/src/compiler/layout/tests.rs');prompt=Path('00_nucleo/prompts/compiler/layout/tests.md')
def snap():return dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),files={str(p):dict(sha256=hashlib.sha256(p.read_bytes()).hexdigest(),text=p.read_text()) for p in [source,prompt]})
with gzip.open(stage/'before.json.gz','wt') as f:json.dump(snap(),f)
prompt.write_text(prompt.read_text()+'''\n\n## Observador exaustivo de raiz — 2026-10-14\n\nMedição anterior: paint-root-2026-10-14/observer-cleanup/before-followup-architecture.log\nregistra V16 em tests.rs:24369 e notas V19/V20 no mesmo observador novo.\nO curinga vazio ignora silenciosamente futuras variantes do domínio.\nNeste helper cfg(test), enumerar todas as variantes de FrameItem, mantendo\nrecursão individual por Semantic/TextReplacement/Link e coleta apenas de\nShape sólido. Separar o pattern aninhado de fill sem mudar os observáveis\nou expectativas. Novas variantes devem exigir decisão explícita na compilação.\nA suíte completa anterior não é atribuída à edição posterior do helper;\nreexecutar os quatro testes afetados e checks de compilação/linhagem.\n''')
s=source.read_text();start=s.index('mod paint_root_transport {');prefix=s[:start];body=s[start:]
old='''                FrameItem::Shape { pos, fill: Some(Paint::Solid(color)), .. } => {
                    let (x, y) = matrix.apply(pos.x.0, pos.y.0);
                    out.push(Observation {
                        color: color.clone(),
                        root,
                        position: Point { x: Pt(x), y: Pt(y) },
                    });
                }
                FrameItem::Semantic { items, .. }
                | FrameItem::TextReplacement { items, .. }
                | FrameItem::Link { items, .. } => observe(items, matrix, root, out),
                _ => {}
'''
new='''                FrameItem::Shape { pos, fill, .. } => {
                    if let Some(Paint::Solid(color)) = fill {
                        let (x, y) = matrix.apply(pos.x.0, pos.y.0);
                        out.push(Observation {
                            color: color.clone(),
                            root,
                            position: Point { x: Pt(x), y: Pt(y) },
                        });
                    }
                }
                FrameItem::Semantic { items, .. } => observe(items, matrix, root, out),
                FrameItem::TextReplacement { items, .. } => {
                    observe(items, matrix, root, out)
                }
                FrameItem::Link { items, .. } => observe(items, matrix, root, out),
                FrameItem::TextSeparator { .. } => {}
                FrameItem::Text { .. } => {}
                FrameItem::TextShaped { .. } => {}
                FrameItem::Line { .. } => {}
                FrameItem::Glyph { .. } => {}
                FrameItem::Image { .. } => {}
'''
assert body.count(old)==1;source.write_text(prefix+body.replace(old,new))
subprocess.run(['rustfmt','--edition','2021','--config','skip_children=true',str(source)],check=True)
subprocess.run(['python3','/tmp/tekt-paint-root-seal.py'],check=True)
with gzip.open(stage/'after.json.gz','wt') as f:json.dump(snap(),f)
cmd=['cargo','test','--offline','--locked','-p','typst-core','paint_root_transport','--','--test-threads=1']
start=datetime.datetime.now(datetime.timezone.utc).isoformat()
with (stage/'focused.log').open('w') as f:r=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT)
(stage/'result.json').write_text(json.dumps(dict(start=start,end=datetime.datetime.now(datetime.timezone.utc).isoformat(),command=cmd,exit=r.returncode),indent=2))
raise SystemExit(r.returncode)
