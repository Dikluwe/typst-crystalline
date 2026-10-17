from pathlib import Path
import gzip,json,hashlib,datetime,subprocess
out=Path('00_nucleo/diagnosticos/tiling-cobertura-2026-10-14')
src=Path('01_core/src/compiler/layout/tiling.rs');owner=Path('00_nucleo/prompts/compiler/layout/tiling.md')
r=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),files={str(p):dict(text=p.read_text(),sha256=hashlib.sha256(p.read_bytes()).hexdigest()) for p in [src,owner]},workspace={str(p):hashlib.sha256(p.read_bytes()).hexdigest() for top in ['01_core','02_shell','03_infra','04_wiring','benches'] for p in Path(top).rglob('*.rs')})
with gzip.open(out/'before-state.json.gz','wt') as f:json.dump(r,f)
with owner.open('a') as f:f.write('''

## Célula geométrica e cobertura da repetição — 2026-10-14

### Medição anterior à decisão

`00_nucleo/diagnosticos/tiling-cobertura-2026-10-14/before/results.json.gz`
conserva fontes/owners, HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b,
working tree não commitado com diffstat, binários e UTC. No canvas inteiro
Poppler 144 dpi: empty pinta 3840 pixels Tekt/0 vanilla; empty-dense
40000/0; dense 40000/10000; self e offset-control 0/960. O pin vanilla
é 9f2b6e8715237cb086899a42873660fe744622e8. O controle large-offset
não pinta em nenhum binário; isto não legitima perder periodicidade.
A fonte deste owner:35 usa a origem de baseline do helper; :47 usa
last_sub_frame_width, que é apenas avanço final de cursor (sub_frame.rs:350),
não a largura de um shape em bloco; :57 rejeita conteúdo sem itens e :76
usa um quadrado centrado na origem, não o inverso do recorte transformado.
São hipóteses causais a discriminar por RED e recoleta, não paridade já provada.

### Decisão e responsabilidade exclusiva

Conservar a materialização privada na fase layout e os FrameItems existentes.
Usar o helper isolado com origem geométrica; texto adquire sua própria baseline.
Na dimensão automática, medir a largura dos itens retornados com o helper
line_content_right existente e conservar também o avanço final; não executar
body novamente. Altura usa a extensão devolvida pelo sub-layout. Dimensões
explícitas inválidas são rejeitadas antes de executar o body.

Com tamanho/pitch/offset/ângulo válidos, body sem itens é um preenchimento
transparente materializado com sucesso (Some grupo vazio), nunca None/fallback
preto. Não construir uma grelha vazia só para atingir o limite de células.

A grelha cobre o inverso dos quatro cantos do recorte alvo pela transformação
de placement, incluindo o tamanho da célula nas bordas. Reduzir deslocamentos
periódicos pelo pitch antes de combiná-los, evitando saturação de índices por
offsets grandes. Isto conserva a fase de self/parent e offset antes de ângulo;
não ratifica todo o referencial parent/rotação como paridade bilateral.
Verificar finitude e spans antes de conversões/loops e manter o máximo de
10000 células efetivamente candidatas. O limite residual continua Unknown,
não sucesso de render nem prova de produção; não elevá-lo para esconder erro.

### Aceitação e limites

RED→GREEN para transparência, geometria da célula, size:auto de shape em
bloco, padrão denso abaixo do limite real, cobertura parent deslocado/rotacionado
e periodicidade de offset grande. Controles preservam recorte de cada célula,
ordem de pintura, layout único e rejeição de entradas inválidas. Recolher o
corpus bilateral e validar build, workspace, formato e linhagem. Stroke ainda
não transporta corpo materializado ao exporter; esta correção não o encerra.
Sem campo/assinatura pública nova, default ou mudança de fase (ADR-0127).
''')
