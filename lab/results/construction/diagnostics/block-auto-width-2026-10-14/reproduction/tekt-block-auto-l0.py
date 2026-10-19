from pathlib import Path
import json,re,hashlib
out=Path('00_nucleo/diagnosticos/block-auto-width-2026-10-14');owners=json.loads((out/'owners.json').read_text())
nucleus=Path('00_nucleo/prompts/_nuclei/layout/flow-frame-width.toml')
assert not nucleus.exists()
nucleus.write_text('''tekt = 1
kind = "nucleus"
id = "flow-frame-width"
title = "Extensão horizontal causal de frames automáticos"

[[claims]]
id = "causal-width"
level = "must"
statement = "A largura intrínseca do fluxo é acumulada pelos produtores de layout em seu referencial, antes de deslocamentos de alinhamento visual. Shapes usam dimensão resolvida, imagens usam target, linhas usam extensão intrínseca efetivamente emitida, containers usam alocação final. Não é bbox de tinta de todos os descendentes: Place/overlays não ampliam o fluxo."

[[claims]]
id = "scope"
level = "must"
statement = "Block isola a extensão do corpo e publica sua largura completa com inset uma vez ao pai. Subframe isola/restaura em LIFO e devolve uma testemunha privada de largura, separada do legado last_sub_frame_width. Checkpoint conserva ambos os estados. Não medir ou executar o corpo novamente para obter esta largura."

[[claims]]
id = "pending"
level = "must"
statement = "A largura automática final deve orientar backing, clip e raiz de pintura na mesma região. Fragmentação, realinhamento tardio e o referencial afim dos exporters precisam de validação própria; a instalação da testemunha de largura não certifica esses consumidores."
''')
pin=hashlib.sha256(nucleus.read_bytes()+b'\0TEKT-NUCLEUS-DEPS-V1\0').hexdigest()
responsibilities={
'mod':'Armazenar flow_frame_extent:(f64,f64) privado (origem,largura), inicial (0,0), e last_sub_frame_flow_width:f64 inicial zero. Não alterar last_sub_frame_width legado nem API pública.',
'block':'Isolar origem/largura ao entrar no corpo, obter largura Auto do fluxo emitido + inset; explícita prevalece. Usar a mesma largura final no clip, paint_root e backing. Restaurar extensão exterior e publicar a alocação do Block; helper privado record_flow_width_at acumula origem+largura relativa ao escopo. No caso RTL textual já certificado, mover a origem da raiz com a translação existente do conteúdo. Não usar tinta de Place ou segunda medição.',
'sub_frame':'Salvar/restaurar flow_frame_extent; iniciar origem do subframe e largura zero. Devolver largura do fluxo em last_sub_frame_flow_width privado antes do restauro. Preservar last_sub_frame_width legado e todos os retornos públicos.',
'checkpoint':'Capturar/restaurar flow_frame_extent e last_sub_frame_flow_width junto dos demais estados causais, inclusive rollback de tentativa rejeitada.',
'line_commit':'Antes do alinhamento/numeração visual, acumular extensão da linha emitida: limite visual e TextSeparator Literal, excluindo terminais Markup/Newline. Não usar cursor global como largura textual nem contar números na margem. Fracionários e caixas usam os carriers já expandidos.',
'shape':'Ao emitir Shape, publicar a dimensão horizontal resolvida na posição lógica, independentemente de fill/stroke/tiling. Não usar stroke/outset como avanço de fluxo.',
'image':'Publicar largura do target usado para layout (ou dimensão natural quando não há target), na posição lógica; cover overflow não amplia a largura causal.',
'placement':'Depois de layout_align, publicar a largura causal devolvida pelo subframe na origem lógica anterior à translação visual. layout_place permanece fora do fluxo: apenas a restauração do subframe conserva a extensão do pai.',
'stack':'Publicar a largura dos subframes compostos: máximo na composição vertical, soma com spacing na horizontal, usando a testemunha causal. Não contar Place como largura de filho. Preservar a política vertical existente nesta etapa.',
'columns':'Publicar a largura útil da alocação de Columns; não derivar largura global da tinta em apenas uma coluna.',
'grid/cell_emission':'Publicar a extensão horizontal das tracks posicionadas durante a emissão das linhas, sem confundir tamanho da célula com tinta interna ou stroke.',
 'tests':'Adicionar regressões de largura Auto em Shape, Empty, Place isolado e misto, Block aninhado/explícito, inline Box, Grid e Stack; backing e paint_root devem concordar. Cobrir espaço literal vs markup e isolamento do estado/rollback após a implementação privada. Não exigir paridade de altura/alinhamento ainda refutados nem relaxar valores existentes.'}
evidence='''\n\n## Largura causal de Block Auto — 2026-10-14

### Medição anterior à decisão

block-auto-width-2026-10-14/before/results.json.gz conserva HEAD
06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado,
UTC/diffstat/status/fontes/binários/argv e pin 9f2b6e871. O extrator inicial
procurou blue como #0000ff; blue-geometries-corrected.json lê a cor real
#0074d9 dos mesmos SVGs, sem nova execução. Em shape, Empty, Place-only,
Grid e Stack a largura do backing Tekt é160; pin54,14,14,68,68 respectivamente.
block.rs:222,416,453 usa largura disponível; line_commit.rs:753 perde cursor
no fechamento e sub_frame.rs:354 guarda apenas cursor terminal. Fonte pinada
flow/block.rs:145–166,281–290,339–363 distingue Auto de expansão.
É geometria da linguagem; estrutura da testemunha é mecânica interna.
A hipótese é refutada se Place contaminar largura, escopos vazarem, texto
literal perder avanço ou containers explícitos mudarem de dimensão.

### Responsabilidade exclusiva

'''
for n,p in owners.items():
 key=n.removeprefix('01_core/src/compiler/layout/').removesuffix('.rs');path=Path(p);s=path.read_text()
 pinline=f'- {nucleus} sha256:{pin}'
 m=re.search(r'^Núcleos Tekt:\n((?:- .*\n)+)',s,re.M)
 if m:
  lines=m.group(1).splitlines()+[pinline];s=s[:m.start(1)]+'\n'.join(sorted(lines))+'\n'+s[m.end(1):]
 else:
  lines=s.splitlines(True);lines.insert(2,'\nNúcleos Tekt:\n'+pinline+'\n');s=''.join(lines)
 s+=evidence+responsibilities[key]+'''\n\n### Aceitação e limites

Correção interna/de paridade ADR-0127: nenhum contrato público, default ou
fase novo; L0 primeiro, RED→GREEN, resselo restrito. A largura é uma parte
da correção B2b/T1–T3 já em andamento: altura Auto/backing, alinhamento tardio,
fracionários, fragmentação, outset e PDF Parent continuam com critérios próprios.
Não declarar render aprovado por testar apenas o carrier. Preservar controles
bilaterais completos e refutadores, suíte/build/doc/fmt/linhagem/inventários.
'''
 path.write_text(s)
print(pin)
