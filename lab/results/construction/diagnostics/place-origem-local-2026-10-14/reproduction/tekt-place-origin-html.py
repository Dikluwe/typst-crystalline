from pathlib import Path
import re,json,gzip,hashlib,datetime
out=Path('00_nucleo/diagnosticos/place-origem-local-2026-10-14');owners=json.loads((out/'owners.json').read_text());sources=['01_core/src/compiler/layout/'+n+'.rs' for n in ['context_block','html_frame']];added={n:re.search(r'^//! @prompt (.+)$',Path(n).read_text(),re.M).group(1) for n in sources}
with gzip.open(out/'html-before.json.gz','wt') as f:json.dump(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),files={n:dict(text=Path(n).read_text(),sha256=hashlib.sha256(Path(n).read_bytes()).hexdigest()) for n in sources+list(added.values())}),f)
owners.update(added);(out/'owners.json').write_text(json.dumps(owners,indent=2))
entry='- 00_nucleo/prompts/_nuclei/place-coordinate-origin.toml sha256:2baf472bb540640d571ae618aa218e5f2b166839a2070f8a4a907ec65d7ded20'
for src,owner in added.items():
 p=Path(owner);s=p.read_text();m=re.search(r'Núcleos Tekt:\n((?:- [^\n]+\n)+)',s);assert m;s=s[:m.start(1)]+'\n'.join(sorted(set(m.group(1).splitlines()+[entry])))+'\n'+s[m.end(1):]
 s+='''
## Origem geométrica da ilha e do carrier — 2026-10-14

### Medição antes da decisão

place-origem-local-2026-10-14/html-failures.log reproduz quatro falhas P1493 existentes: aqui() publica y11.2 em vez de20 dentro de Place(Box HTML), diferença de ascender8.8 em FixedMetrics. O entrypoint html_frame.rs:219 compõe em baseline legada; context_block.rs:16–22 subtrai ascender ativo mesmo quando Place/Box usam origem geométrica0. Os testes de pipeline preservam explicitamente x30/y20, nenhuma página e fixpoint do mesmo target. O estado anterior está em html-before.json.gz; workspace-interrupted conserva a primeira ocorrência e o estado integral. Captura e desenho devem usar a mesma geometria, como exige o núcleo coordinates; não é motivo para converter HTML em pipeline paginado nem mudar ADR-0128.

### Responsabilidade exclusiva

'''
 if src.endswith('html_frame.rs'):s+='Compor a ilha intrínseca pelo helper de origem geométrica. Texto instala a própria baseline; a captura continua antes do crop visual, e carriers são removidos depois da projeção. Não alterar target, feature, fixpoint, PageStore, assinaturas ou política de posições.\n'
 else:s+='No subframe, normalizar o cursor pelo ascent efetivamente instalado da linha (line_assumed_ascent), ou sub_frame_origin_y se não houver ascent instalado; com baseline inicial pendente, não subtrair fonte inexistente. Fora de subframe, preservar a fórmula legada com métricas ativas. Não executar closure, avançar cursor nem mudar a captura paginada. Isso substitui a subtração incondicional de ascender quando o body usa origem geométrica.\n'
 s+='\nCorreção interna ADR-0127 para preservar o contrato HTML já vigente. As expectativas P1493 permanecem iguais. Validar todos os testes p1493, contexto/HTML e suíte completa, além dos controles PDF/SVG e gates de linhagem; sem mudança pública ou de fase.\n';p.write_text(s)
p=Path(sources[0]);s=p.read_text();a=s.index('        let (ascender, _) =');b=s.index('        layouter\n',a);s=s[:a]+'''        let origin_y = if layouter.is_sub_frame {
            if layouter.initial_baseline_pending {
                0.0
            } else {
                layouter.line_assumed_ascent.unwrap_or(layouter.sub_frame_origin_y)
            }
        } else {
            layouter.metrics.vertical_metrics(layouter.style.size, &layouter.style).0 .0
        };
        let point = Point {
            x: layouter.regions.current.cursor_x,
            y: layouter.regions.current.cursor_y - crate::entities::layout_types::Pt(origin_y),
        };
'''+s[b:];p.write_text(s)
p=Path(sources[1]);s=p.read_text();old='let (height, mut items, _, _, _) = layouter.layout_sub_frame(';assert s.count(old)==1;s=s.replace(old,'let (height, mut items, _, _, _) = layouter.layout_sub_frame_at_geometric_origin(');p.write_text(s)
