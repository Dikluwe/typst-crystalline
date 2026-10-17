from pathlib import Path
import re,json,gzip,hashlib,datetime,subprocess
out=Path('00_nucleo/diagnosticos/place-fluxo-2026-10-14');out.mkdir(exist_ok=True)
names=['mod','checkpoint','sub_frame','place','line_commit','shape','block','placement','grid/cell_emission','raw','v_space','parbreak','region_progress','tests'];sources=['01_core/src/compiler/layout/'+n+'.rs' for n in names];owners={n:re.search(r'^//! @prompt (.+)$',Path(n).read_text(),re.M).group(1) for n in sources};paths=sources+list(owners.values());sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
r=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True),status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),files={n:dict(text=Path(n).read_text(),sha256=sha(n)) for n in paths})
with gzip.open(out/'before-source.json.gz','wt') as f:json.dump(r,f)
(out/'owners.json').write_text(json.dumps(owners,indent=2))
p=Path('00_nucleo/prompts/_nuclei/place-flow-boundary.toml');assert not p.exists();p.write_text('''tekt = 1
kind = "nucleus"
id = "place-flow-boundary"
title = "Fronteira física causal para Place fora do fluxo"

[[claims]]
id = "causal-end"
level = "must"
statement = "Place sem alinhamento vertical explícito usa a baseline da linha pendente ou a última fronteira física de fluxo certificada. A fronteira exclui preparação da próxima baseline e margens fracas; não é o máximo dos bboxes, pois overlays não consomem fluxo. Top/Bottom/Horizon explícitos mantêm a região."

[[claims]]
id = "witness"
level = "must"
statement = "O certificado privado guarda fim físico e cursor preparado no instante da produção. Só é reutilizado quando esse cursor ainda corresponde ao estado corrente; alteração desconhecida invalida a evidência, sem adivinhar um fim. Parbreak pode preservar o fim ao preparar margem; VSpace acrescenta seu avanço físico."

[[claims]]
id = "isolation"
level = "must"
statement = "Subframe inicia sem certificado e restaura o do pai em LIFO; checkpoint captura/restaura; nova página/coluna/configuração limpa o certificado. Place não altera a fronteira do pai. Não mudar contratos públicos, pipeline ou política de margens dos produtores."
''');pin=hashlib.sha256(p.read_bytes()+b'\0TEKT-NUCLEUS-DEPS-V1\0').hexdigest();entry=f'- {p} sha256:{pin}'
roles={
'mod':'Armazenar pub(super) flow_placement_end:Option<(f64,f64)> (fim físico,cursor preparado), inicial None. install_page_config limpa a evidência.',
'checkpoint':'Capturar/restaurar flow_placement_end sem alterar demais dados do sufixo.',
'sub_frame':'Salvar/take flow_placement_end antes do body e restaurar no retorno, para que body de Place não consuma o fluxo do pai.',
'place':'Possuir métodos privados para registrar um fim com cursor corrente e consultar somente certificado cuja testemunha de cursor ainda coincide. O adapter não flutuante usa linha pendente primeiro, depois fim certificado, somente quando VAlign ausente; aplica a mesma delta a itens/decoração/fixups. Não faz scan de tinta nem flush antecipado. Floats não mudam.',
'line_commit':'Após estabilizar e drenar linha causal, registrar baseline_y + line_descent junto do cursor já preparado para a próxima linha. Flush sem linha causal não cria fronteira.',
'shape':'Ao concluir a forma, registrar shape_base + resolved_h, depois de atualizar o cursor; excluir below e cap-height preparatório. Preservar desenho e margens existentes.',
'block':'Isolar a fronteira anterior ao entrar no body; conservar o fim físico produzido antes dos avanços de inset/below/leading. Ao sair, registrar fim da região explicitamente alocada quando há altura (start_y + outset_top + altura + outset_bottom), ou fim causal do body mais inset inferior/outset; sem alterar a política de desenho ou cursor legado nesta etapa. O ramo multipágina deve usar a fronteira da região sobrevivente, nunca o start_y da página anterior.',
'placement':'Ao concluir Align, registrar target_y+sub_h como fim físico, com cursor preparado atual; não capturar a tinta de Place nem usar limite de página como fim do body.',
'grid/cell_emission':'Após concluir cada row e avançar cursor, registrar row_start_y+row_h. Não importar a fronteira local dos bodies de células.',
'raw':'Registrar end físico calculado pelo owner Raw depois de instalar seus estados de abaixo/cursor, sem incluir a margem abaixo.',
'v_space':'Depois do flush existente, conservar fronteira certificada (ou cursor se não houver), somar amount e registrar com cursor após avanço. Não incluir leading hipotético de linha fechada no fim físico.',
'parbreak':'Preservar o fim físico certificado pelo flush enquanto prepara spacing/estado de bloco; revalidar a testemunha no cursor novo sem somar margem fraca ao fim.',
'region_progress':'Limpar flow_placement_end em reset_closed_region_flow, tanto página como coluna, sem mover ownership de transição.',
 'tests':'Adicionar regressões root/Box após Linebreak/Parbreak, Shape, Block vazio/textual/fixo/auto, Grid, Align e VSpace; preservar Top explícito, overlays sucessivos, página nova e aninhamento. Usar coordenadas de linguagem e métricas injetadas, nunca redefinir esperados pelo resultado.'}
for src,owner in owners.items():
 f=Path(owner);s=f.read_text();m=re.search(r'Núcleos Tekt:\n((?:- [^\n]+\n)+)',s)
 if m:s=s[:m.start(1)]+'\n'.join(sorted(set(m.group(1).splitlines()+[entry])))+'\n'+s[m.end(1):]
 else:
  i=s.index('\n',s.index('Hash do Código:'));s=s[:i+1]+'\nNúcleos Tekt:\n'+entry+'\n'+s[i+1:]
 name=src[len('01_core/src/compiler/layout/'):-3]
 s+='''
## Place após fluxo comitado — etapa B1 — 2026-10-14

### Medição antes da decisão

`00_nucleo/diagnosticos/place-fluxo-2026-10-14/before/results.json.gz` registra HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, diffstat/UTC/fontes/binários/argv e pin vanilla 9f2b6e8715237cb086899a42873660fe744622e8. Nos 24 pares root/Box após 12 prefixos, o marcador diverge em todos; Linebreak/Parbreak precisam y22.25pt, Shape y27pt, Block/Grid/VSpace20pt precisam y35pt, mas Tekt usa15pt. Em Rotate há também sobreposição visual, defeito independente. line_commit.rs:721–733 prepara próxima baseline; shape.rs:196–206 agrega below/cap; block.rs:465 conserva fronteira e margem separadas; place.rs:91–99 só reconhece linha ainda pendente. Fonte pinada flow/distribute.rs:703–715 usa offset físico de flow para Place sem eixo explícito. Isso é geometria de linguagem, não exigência de estrutura Rust. A hipótese é refutada por margem fraca, sobreposição, página nova, rollback ou vazamento entre subframes.

### Responsabilidade exclusiva

'''+roles[name]+'''

### Aceitação e extensão pendente

Correção interna ADR-0127: L0 primeiro, RED→GREEN e resselo; sem campo público, flag/default ou fase nova. Este adendo autoriza somente o estado privado necessário a esse contrato, supersedendo proibições de novo estado limitadas a extrações anteriores. Guardar/restaurar em LIFO é obrigatório. Revalidar suíte completa, HTML/contexto, PDF/SVG, build/doc/fmt/check, linhagem e inventário de avisos.
B1 completa a âncora após produtores certificados (linha, forma, Block, Align, Grid, Raw, VSpace), sem substituir o objetivo de corrigir todo o sistema. B2 completa região de Block/inset; B3 resolve assentamento tardio de linha pendente com inline Box/Transform, e audita demais produtores (Heading/List/Stack/Equation/Columns) antes de certificar Place universal. Testemunha inválida conserva a rota legada e é pendência, não crédito de paridade. O gate público de tiling continua pendente.
''';f.write_text(s)
print(pin)
