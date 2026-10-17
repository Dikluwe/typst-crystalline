from pathlib import Path
import json,gzip,hashlib,datetime,base64,difflib
out=Path('00_nucleo/diagnosticos/place-origem-local-2026-10-14')
def read(n):
 with gzip.open(out/n,'rt') as f:return json.load(f)
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
before=read('before-source.json.gz');native=read('native-before.json.gz');candidate=read('candidate-final.json.gz');suite=json.loads((out/'workspace-result.json').read_text());checks=json.loads((out/'followup-summary.json').read_text());cmp=json.loads((out/'comparison.json').read_text())
assert suite['exit']==0 and suite['sources_unchanged'] and all(sha(n)==v['sha256'] for n,v in candidate['files'].items())
assert checks['clippy_after']==checks['clippy_before'] and checks['architecture']=={'warning':382,'note':1627}
original=before['files']|native|read('html-before.json.gz')['files']
patch=''.join(''.join(difflib.unified_diff(original.get(n,{}).get('text','').splitlines(True),v['text'].splitlines(True),fromfile=n,tofile=n)) for n,v in candidate['files'].items());(out/'current.patch').write_text(patch)
repro=out/'reproduction';repro.mkdir(exist_ok=True)
scripts = ['tekt-place-origin-'+n+'.py' for n in ['setup','tests','implement','probe','controls','validate','suite','compare','seal','pdf','report','finalize','baseline-replay','html']] + ['tekt-place-'+n+'.py' for n in ['align-origin','align-probe','auto-visible-probe','expanded-controls']]
for name in scripts:
 p=Path('/tmp')/name
 (repro/p.name).write_bytes(p.read_bytes())
(repro/'README.txt').write_text('Registros históricos de execução. Scripts de setup/tests/implement/finalize/align-origin/baseline-replay/seal não são idempotentes e NÃO devem ser reaplicados à árvore final. current.patch e candidate-final.json.gz são o estado autoritativo deste lote. Pequenas correções interativas de origem X e limpeza de avisos aparecem no patch final. Baseline-replay restaura fontes/owners exatos do início e confere a restauração do candidato; não altera outros arquivos. Os probes criam diretórios novos e verificam estabilidade. Os resultados gravam fontes, comandos, binários e saídas. O primeiro align-controls foi renderizado em 96dpi/zoom4: bbox impresso * 3/4 = pt. As sondas expanded corrigem a escala para72dpi/zoom4; corpus principal e PDFs usam4px/pt.\n')
entries=[]
for p in sorted(out.rglob('*')):
 if p.is_file() and p.name!='evidence.json.gz':entries.append(dict(path=str(p),sha256=sha(p),base64=base64.b64encode(p.read_bytes()).decode()))
with gzip.open(out/'evidence.json.gz','wt') as f:json.dump(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),entries=entries),f)
with gzip.open(out/'evidence.json.gz','rt') as f:archive=json.load(f)
assert all(hashlib.sha256(base64.b64decode(x['base64'])).hexdigest()==x['sha256'] for x in archive['entries'])
text=f'''# Place/Align: origem de composição local — etapa A — 2026-10-14

Implementada a origem geométrica única de corpos de Place não flutuante e Align,
a distinção entre alinhamento vertical omitido e Top explícito, e a conservação
da origem horizontal de Align. O corpo não recebe um strut de fonte artificial.
O lote é parcial: a etapa B ainda precisa corrigir âncoras após fluxo comitado e
inset de Block. Remover a compensação acidental aumenta o erro em dois controles
que já falhavam (Box após Linebreak/Shape); esses casos não estão encerrados.

## Proveniência

HEAD `{before['head'].strip()}`, working tree não commitado. before-source.json.gz,
native-before.json.gz, red-state.json.gz, red-origin-x-state.json.gz,
candidate-final.json.gz e workspace-pre.json.gz conservam fontes, hashes,
status/diffstat e UTC. A suíte ocorreu de {suite['start']} a {suite['end']};
os hashes permaneceram estáveis durante execução e verificações. Não houve
commit, push, PR, release ou publicação.

Oráculo: vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`, árvore
`6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4`, receita P1489, binário
SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
Cada corpus guarda comandos, fontes, saídas, UTC e SHA-256 de ambos os binários.
baseline-replay.json comprova a reprodução do estado anterior e a restauração
integral dos sete pares de fontes/owners do candidato.

## Medição antes da decisão

placement.rs vetava origem geométrica em subframe, célula e linha pendente;
place.rs acrescentava top-edge à baseline do pai, inclusive para Top explícito;
stdlib/layout/place.rs convertia omissão em Some(Top). Align também subtraía
ascender de um Box cuja composição já começava em zero. Dois erros podiam se
compensar, como documentam green-attempt1/place-attempt2.

O corpus principal tem {cmp['before']['successful_pairs']} pares compilados e dois
controles de rejeição de scope Parent sem float. Posições/bboxes e pixels
coincidentes: {cmp['before']['bbox_matches']} antes, {cmp['after']['bbox_matches']}
depois. Casos corrigidos incluem Box fixo/auto/inset/aninhado, Place de Box,
Bottom/Horizon, tamanhos de fonte e Place depois de texto ainda na linha.
Os controles de Grid, Table e Align(Box) continuam coincidentes.
Top explícito após texto passa a ficar no topo da região, confirmado em extra-after.

O corpus adicional ampliado tem nove casos, de {cmp['baseline-expanded']['bbox_matches']}
para {cmp['final-expanded']['bbox_matches']} bboxes coincidentes. Ele identifica
Place diretamente em Align com x0.25pt em vez de20.25pt: conservar line_start_x
corrige a origem perdida. Para Align bottom em página auto, o texto sobe
indevidamente no estado anterior (bbox y12.75pt); o candidato e o vanilla
coincidem em20pt. Os casos com dy=-5 tornam o marcador visível; os primeiros
controles com dy30 fora do canvas não contam como prova de posição.

Nos PDFs finais, {cmp['pdf-final']['bbox_matches']}/{cmp['pdf-final']['cases']}
pares têm bbox coincidente do marcador e {cmp['pdf-final']['same_pixels']} têm
pixels integrais idênticos. Os 26 PDFs passaram qpdf. A amostra Box aninhado
foi inspecionada visualmente. Isso valida esses controles, não igualdade de
bytes nem paridade universal do documento/exportador.

## Contrato e implementação

- Nove owners 1:1 atualizados antes de código; núcleo place-coordinate-origin
  pinado com `{hashlib.sha256(Path('00_nucleo/prompts/_nuclei/place-coordinate-origin.toml').read_bytes()+bytes([0])+b'TEKT-NUCLEUS-DEPS-V1'+bytes([0])).hexdigest()}`.
- `Layouter.sub_frame_origin_y` é pub(super), guardado/restaurado por subframe e
  checkpoint. Preserva a origem de recomposição dos callers legados sem inferi-la
  da fonte corrente. Nenhum campo/assinatura pública foi acrescentado.
- Place e Align compõem bodies na origem geométrica; translações de itens,
  decoração e fixups seguem a mesma referência. Place em célula usa a origem
  local esperada pelo rebase; no eixo X usa a referência existente do subframe.
- O construtor preserva v=None para omissão. Place segue a baseline pendente
  somente nesse caso; Top explícito usa a região. Floats mantêm seu caminho.
- A ilha HTML intrínseca usa origem geométrica; seu carrier de contexto conserva
  o cursor físico do subframe e recebe a mesma composição dos demais itens.
  Não subtrai ascender antes do caller que já faz o rebase.
- ADR-0127: correções internas/de paridade em fluxo contínuo. O pacote público
  de tiling permanece pendente, sem implementação ou alteração de candidatos.

## Testes e qualidade

Três testes geométricos falharam antes da implementação; o controle de célula/
Align inicialmente passava e impediu aceitar uma compensação incorreta. O teste
horizontal teve RED separado: x7 em vez de17. Seis testes novos cobrem origem,
fonte, aninhamento, VAlign, omissão/Top, células e restauração/checkpoint.

Cinco expectativas antigas confundiam a baseline de FrameItem::Text com o topo
do frame. P898 Place raiz já somava top-edge; P908 aninhados e dois P898 Align
não somavam. Foram retificadas após sondas e leitura de flow/distribute.rs:703–715,
preservando altura, finitude e dy. A lacuna de espaçamento após Block não foi
reclassificada como correta. As falhas e o patch das expectativas estão arquivados.

A primeira suíte completa terminou com status143, causa não estabelecida, antes
de um resultado final; o log revelou quatro regressões P1493 de posição HTML.
A correção do carrier e da entrada da ilha preservou todas essas expectativas.
Uma tentativa intermediária também revelou dupla subtração em Transform/Link;
esses controles permaneceram inalterados. As tentativas estão preservadas em
workspace-interrupted e html-failures/html-green-attempt1. A suíte final foi
executada isoladamente. Os PDFs foram conferidos antes do ajuste estritamente
HTML; essa revisão foi validada pelos testes de HTML e pela suíte final.

Suíte completa: **{suite['passed']} passaram, {suite['failed']} falharam,
{suite['ignored']} ignorados**. Build offline/locked, rustdoc -D warnings,
all-target check, fmt e diff check passaram. Linhagem V1/V5/V15/V26 estrita e
reverse lineage passaram; resselo restrito aos nove pares.

A primeira rodada de qualidade encontrou uma closure desnecessária e um wildcard
silencioso no auxiliar de projeção do teste; ambos foram removidos. Inventário
final Clippy: {checks['clippy_after']} avisos, sem aumento. Linter completo:
zero erros, {checks['architecture']['warning']} warnings e
{checks['architecture']['note']} notes. Isso não é zero violations nem aprovação
do gate CI Clippy -D warnings. CI remoto e release continuam sem certificação.

## Pendências medidas e limite da etapa

- Place após Linebreak/Parbreak/Shape ainda ancora na região, em vez do fim causal
  do fluxo. Em Box, y depois de Linebreak passou de24.75 para15pt (vanilla22.25);
  depois de Shape, de24.75 para15pt (vanilla27). São casos antes parcialmente
  compensados pelo strut removido; a etapa B é necessária antes de fechar Place.
- Block com inset3 mantém Place em y15pt, enquanto vanilla pede18pt.
- Align horizon após Block alto tem bbox y220pt contra233.25; falta margem de
  fluxo13.2pt. Rotate do controle tem diferença correlata de altura/posição.
- Mistura texto/Shape em Align conserva diferença residual de geometria;
  comparações completas estão em comparison.json, sem crédito de paridade.

As causas confirmadas neste lote estão no código de coordenadas/omissão, mas os
L0 e cinco expectativas de teste também precisaram de correção. Não há levantamento
estatístico completo que autorize dizer qual categoria domina o projeto inteiro.

## Evidências

[evidence.json.gz](place-origem-local-2026-10-14/evidence.json.gz) contém
{len(entries)} entradas com caminho, SHA-256 e bytes, relidas e verificadas.
SHA-256 do arquivo: `{sha(out/'evidence.json.gz')}`.
Inclui snapshots, diffstat, patch, testes RED/GREEN, validações, SARIF, sondas,
PDF/SVG/PNG e scripts históricos. Os scripts de mutação não são comandos para
reaplicar sobre a árvore atual.
'''
out.with_suffix('.md').write_text(text)
print(json.dumps(dict(report=str(out.with_suffix('.md')),archive_entries=len(entries),archive_sha256=sha(out/'evidence.json.gz'),suite={k:suite[k] for k in ['start','end','passed','failed','ignored','sources_unchanged']},checks=checks),indent=2))
