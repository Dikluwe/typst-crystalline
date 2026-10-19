from pathlib import Path
import json,gzip,hashlib,datetime,base64,difflib
out=Path('00_nucleo/diagnosticos/block-regiao-2026-10-14');sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
def read(folder):
 with gzip.open(out/folder,'rt') as f:return json.load(f)
def compare(folder):
 r=read(folder+'/results.json.gz');pairs={}
 for row in r['rows']:pairs.setdefault(row['case'],{})[row['binary']]=row
 return dict(cases=len(pairs),matches=sum(v['tekt'].get('blue_bbox')==v['vanilla'].get('blue_bbox') and v['tekt'].get('exit',0)==0 and v['vanilla'].get('exit',0)==0 for v in pairs.values()),differences={n:{k:v.get('blue_bbox') for k,v in p.items()} for n,p in pairs.items() if p['tekt'].get('blue_bbox')!=p['vanilla'].get('blue_bbox')})
pre=read('before-source.json.gz');final=read('candidate-final.json.gz');checks=json.loads((out/'followup-summary.json').read_text());suite=json.loads((out/'workspace-result.json').read_text());assert suite['exit']==0 and suite['sources_unchanged'];assert all(sha(n)==v['sha256'] for n,v in final['files'].items());assert checks['clippy_after']==181 and checks['architecture']=={'warning':382,'note':1627}
cmp={name:compare(name) for name in ['after-first','after-final','extra-first','extra-final','pdf-final','causal-before']};(out/'comparison.json').write_text(json.dumps(cmp,indent=2))
(out/'current.patch').write_text(''.join(''.join(difflib.unified_diff(pre['files'].get(n,{}).get('text','').splitlines(True),v['text'].splitlines(True),fromfile=n,tofile=n)) for n,v in final['files'].items()))
repro=out/'reproduction';repro.mkdir(exist_ok=True)
for name in ['setup','tests','implement','probe','extra','causal','seal','validate','suite','pdf','open-pdf','report']:
 p=Path('/tmp/tekt-block-region-'+name+'.py');(repro/p.name).write_bytes(p.read_bytes())
(repro/'README.txt').write_text('Scripts históricos não idempotentes. Não reaplicar setup/tests/implement/seal à árvore corrigida. current.patch conserva o delta exato e candidate-final.json.gz o estado final. O reparo de compilação, o teste de percentuais e a extensão de checkpoint constam do patch. causal-before usou o binário de first-candidate.json.gz enquanto percentuais/checkpoint já eram alterados na fonte: ele conserva o hash do binário, e suas conclusões se limitam àquela primeira implementação. after-final, extra-final e PDFs usam o candidato final selado. before do corpus principal está em place-fluxo-2026-10-14/block-regions-before.\n')
entries=[]
for p in sorted(out.rglob('*')):
 if p.is_file() and p.name!='evidence.json.gz':entries.append(dict(path=str(p),sha256=sha(p),base64=base64.b64encode(p.read_bytes()).decode()))
with gzip.open(out/'evidence.json.gz','wt') as f:json.dump(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),entries=entries),f)
with gzip.open(out/'evidence.json.gz','rt') as f:r=json.load(f)
assert all(hashlib.sha256(base64.b64decode(x['base64'])).hexdigest()==x['sha256'] for x in r['entries'])
report=f'''# Block: região interna, percentuais e recorte — B2a — 2026-10-14

Corrigidos origem/dimensões de Place dentro de Block, insets dos dois lados,
percentuais de Block aninhado e recorte de containers sem avanço de fluxo.
A altura automática é resolvida pelo fluxo certificado do próprio corpo;
Place sozinho não faz o Block crescer. Este lote não fecha fragmentação,
outset, produtores sem fronteira certificada nem a frente geral de produção.

## Proveniência

HEAD `{pre['head'].strip()}`, working tree não commitado. before-source,
red-state, first-candidate, percent-red-state, candidate-final e workspace-pre
preservam UTC, hashes, fontes, status e git diff HEAD --stat. Suíte completa:
{suite['start']} → {suite['end']}; fontes estáveis durante testes e validação.
Sem commit, push, PR, publicação ou release.

Referência vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`, árvore
`6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4`, receita P1489, binário SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
Cada probe conserva argv, fonte, saídas, UTC e SHA-256 dos binários; versão
textual não identifica o oráculo. O corpus principal antes do reparo está em
place-fluxo-2026-10-14/block-regions-before/results.json.gz e no arquivo de
outras evidências daquele lote (SHA-256
`c4ea0404d7753e3452ea5d6b9b7c81304726c9d4819a817c3f62399de20a4dc9`).

## Evidência e mudança

block.rs ignorava inset.right, tratava largura declarada como largura do body,
usava avanço zero do Place como altura de clip e agrupava filhos absolutos
sem converter sua origem. placement.rs não tinha região própria de Block.
A fonte pinada flow/block.rs:66–89 e :223–234 força tamanho interno, cresce
inset e só depois recorta; :278–280 desconta inset da região. A correção segue
geometria da linguagem, sem copiar estrutura de dados ou algoritmos upstream.

Agora Block instala uma região interna privada, com origem após inset e largura
igual ao total menos as duas bordas. Place a resolve antes de compor seu corpo;
o eixo vertical omitido aceita a fronteira de fluxo, enquanto Top/Bottom/Horizon
usam a região. A passagem explícita da origem elimina o empréstimo de margin.top
e a segunda translação do adapter. Subframe isola/restaura a região e os segmentos
de decoração; checkpoint preserva ambos; página/coluna/configuração limpa o estado.

No Block sobrevivente, fixups Y auto são resolvidos antes de encapsular o body.
A decoração associada recebe a mesma delta. Clip usa dimensões alocadas/causais
do container e converte os filhos para coordenadas locais uma vez. Os caminhos
horizontais pendentes acompanham Group e inserção do backing. Percentuais de
Block descendente usam a área interna do pai, não a dimensão global mutada.

Oito pares L0/consumer 1:1 foram atualizados antes do código, com resselo restrito.
Núcleo block-place-region, pin
`77c7d31bf3f7276cc9b4d1b2951634724a6d0fb4352a0b45c76e6f3775b512dc`.
Correção interna/de paridade ADR-0127; nenhuma entidade pública, flag, modo ou
fase foi alterada. Os L0 nomeiam B2b e B3 como etapas ainda incompletas.

## Validação

O corpus principal tem 24 pares root/Box: apenas 2 bboxes coincidiam antes;
**{cmp['after-final']['matches']}/24 coincidem agora**. Inset3/top passou de
(20,15) para (20,18); Bottom+Right passou de (71,37) para (62,44). Com clip, o
marcador reaparece e é cortado na borda correta. Texto pendente e Top explícito
continuam distintos, sem baseline artificial sobre formas.

Nos 24 pares PDF finais, **{cmp['pdf-final']['matches']}/24 bboxes coincidem**;
os 48 PDFs passaram qpdf. A comparação é geométrica, não igualdade de bytes.
O corpus extra deliberadamente adversarial tem {cmp['extra-final']['matches']}/24
coincidências; percentuais aninhados foram refutados na primeira implementação
(root(44,33), Box(42,38), esperado(42,34)), ganharam RED próprio e agora coincidem.
Os demais casos desse corpus permanecem falhas; não foram diluídos no principal.

Três regressões iniciais e a de percentuais foram observadas em RED e passaram
em GREEN. O controle de checkpoint existente também foi fortalecido. Os 15 testes
de place_origin_regressions passaram. A primeira suíte de layout, anterior ao
quarto teste, teve 1169 aprovados e 3 ignorados. Houve uma tentativa intermediária
com erro de compilação no posicionamento do registro de decoração; foi corrigida
antes do GREEN e permanece nos logs.

A primeira tentativa de suíte completa terminou com status 143, sem resultado
final e sem processos de teste remanescentes na inspeção do host. A causa não
foi estabelecida; seus logs foram preservados e ela não conta como aprovação.

Suíte completa final: **{suite['passed']} passaram, {suite['failed']} falharam,
{suite['ignored']} ignorados**. Build offline/locked, rustdoc -D warnings,
all-target check, fmt, diff check, V1/V5/V15/V26 estritos e reverse lineage passaram.
Clippy permanece em 181 avisos no inventário --cap-lints warn. O linter completo
registra zero erros, 382 warnings e 1627 notes. Isso não aprova o gate Clippy
-D warnings nem significa zero violations. CI remoto/release continua não certificado.

## Falhas abertas que orientam a próxima etapa

- Overflow de Shape em Block baixo na raiz: o PDF comprovou duas páginas (a
  primeira vazia) no Tekt contra uma página recortada no pin. Em Box, esse controle
  coincide. Não se trata simplesmente de marcador ausente na primeira imagem.
- Outset ainda avança o fluxo e desloca a origem do body, além de não expandir o
  clip corretamente. O marcador após outset4 fica em y43, contra y35 do pin.
- PDF e SVG divergem entre si com Block(outset) sem fill/stroke: PDF pinta o
  backing invisível em azul quando o marcador seguinte é pintado. Inspeção visual
  de open-pdf/root-after-outset-tekt-1.png confirma o retângulo extra. draw.rs:513–517
  e :922–926 deixam o ramo sem fill/stroke vazio antes de Q; a hipótese causal é
  um caminho não encerrado que participa da pintura seguinte. Requer RED→GREEN
  próprio nos percursos top/local, sem confundir essa falha L3 com geometria L1.
- Sublinhado nos controles azul/auto/fixo termina 0.5pt abaixo do pin na rasterização
  a 4px/pt; o lote não certifica sua espessura.
- A sonda causal adicional de oito casos, feita com o primeiro binário deste lote,
  confirma Stack invisível sob clip auto e divergências de List/Heading aninhados.
  É evidência para B2b/B3, não resultado do candidato final nem crédito de paridade.
- Baseline tardia com inline Box/Transform, demais produtores de fluxo, transporte
  público de tiling pendente e demais itens do objetivo permanecem abertos.

Essas diferenças são observações de código/integração e contratos que precisam
ser atualizados com evidência; não sustentam uma porcentagem código versus prompts
para o projeto inteiro. Divergências Tekt anteriormente justificadas continuam
preservadas. O objetivo completo permanece ativo.

## Evidências preservadas

[evidence.json.gz](block-regiao-2026-10-14/evidence.json.gz) contém {len(entries)}
entradas com caminho, SHA-256 e bytes base64, relidas e verificadas. SHA-256:
`{sha(out/'evidence.json.gz')}`. Inclui manifestos, patch, logs RED/GREEN,
validação, suíte, SARIF, SVG/PDF/PNG e scripts históricos. Não reaplicar scripts
de mutação ao estado corrigido.
'''
out.with_suffix('.md').write_text(report)
print(json.dumps(dict(report=str(out.with_suffix('.md')),entries=len(entries),archive_sha256=sha(out/'evidence.json.gz'),suite={k:suite[k] for k in ['passed','failed','ignored','sources_unchanged']},comparisons=cmp),indent=2))
