from pathlib import Path
import json,datetime,hashlib,shutil,subprocess
out=Path('00_nucleo/diagnosticos/clippy-locais-2026-10-16');first=out/'first-validation';first.mkdir(exist_ok=False)
for p in out.glob('validation*'):
 if p.is_file():shutil.move(str(p),first/p.name)
shutil.move(str(out/'clippy-inventory.json'),first/'clippy-inventory.json')
p=Path('00_nucleo/prompts/infra/export/tests/mod.md');sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),owner_before=sha(p),source_before=sha('03_infra/src/export/tests/mod.rs'),evidence='first-validation/validation-architecture.log',issue='Novo V16 no helper emitted_bbox: wildcard com None descarta variantes de FrameItem')
p.write_text(p.read_text()+'''\n\n### Exaustividade do observador bbox — correção após o primeiro gate\n\nMedição antes da decisão: o primeiro gate completo desta etapa, em `00_nucleo/diagnosticos/clippy-locais-2026-10-16/first-validation/validation-architecture.log`, identificou V16 em export/tests/mod.rs:6092 no novo emitted_bbox, `_ => None`. Clippy já havia medido 79 avisos; o gate barrou fechamento por aumentar V16 de 327 para 328. O enum FrameItem vigente em entities/layout_types.rs:373–547 foi conferido antes da correção.\n\nO helper deve enumerar todas as variantes explicitamente: Shape observa seu bbox; Group/Link/Semantic/TextReplacement percorrem os filhos; TextSeparator/Text/TextShaped/Line/Glyph/Image devolvem None porque não possuem parent_bbox_at_emit nem filhos. Não usar wildcard para variantes futuras: o compilador deve exigir uma decisão quando o enum crescer. Preservar a fixture Block/Shape, os asserts de bbox/PDF e todas as outras correções desta etapa. L0 primeiro, resselo apenas deste owner/consumer, revalidar o gate Clippy e toda a suíte/linhagem sem elevar baseline ou suprimir V16.\n''')
record['owner_updated_before_code']=sha(p)
p=Path('03_infra/src/export/tests/mod.rs');s=p.read_text();old='''            | FrameItem::TextReplacement { items, .. } => emitted_bbox(items),
            _ => None,
''';new='''            | FrameItem::TextReplacement { items, .. } => emitted_bbox(items),
            FrameItem::TextSeparator { .. }
            | FrameItem::Text { .. }
            | FrameItem::TextShaped { .. }
            | FrameItem::Line { .. }
            | FrameItem::Glyph { .. }
            | FrameItem::Image { .. } => None,
'''
assert s.count(old)==1;p.write_text(s.replace(old,new,1));subprocess.run(['rustfmt','--edition','2021','--config','skip_children=true',str(p)],check=True)
record.update(source_after=sha(p),utc_end=datetime.datetime.now(datetime.timezone.utc).isoformat())
(out/'exhaustiveness-repair.json').write_text(json.dumps(record,ensure_ascii=False,indent=2)+'\n')
# Keep the durable recipe consistent for replay from the recorded baseline.
p=out/'workers/materialize.py';s=p.read_text();assert s.count(old)==1;p.write_text(s.replace(old,new,1))
p=out/'workers/validate.py';s=p.read_text();anchor="    ('fmt',['cargo','fmt','--check','--all'])";assert anchor in s;s=s.replace(anchor,"    ('strict-clippy',['python3','00_nucleo/diagnosticos/clippy-locais-2026-10-16/workers/gate.py','green']),('fmt',['cargo','fmt','--check','--all'])");p.write_text(s)
p=out/'workers/finalize.py';s=p.read_text().replace("len(validation['commands'])==9","len(validation['commands'])==10");p.write_text(s)
print('Primeiro resultado preservado; L0 e helper corrigidos sem supressão; novo gate inclui Clippy estrito.')
