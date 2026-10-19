from pathlib import Path
import gzip,json,hashlib,base64,subprocess,datetime,shutil
out=Path('00_nucleo/diagnosticos/tiling-cobertura-2026-10-14')
def read(name):
 p=out/name
 if p.suffix=='.gz':
  with gzip.open(p,'rt') as f:return json.load(f)
 return json.loads(p.read_text())
before=read('before-state.json.gz');after=read('after/results.json.gz');suite=read('workspace-result.json');validation=read('followup-summary.json');cmp=read('render-comparison.json.gz');extra=read('extra-comparison.json.gz')
assert suite['exit']==0 and suite['sources_unchanged']
assert all(hashlib.sha256(Path(p).read_bytes()).hexdigest()==v['sha256'] for p,v in read('candidate-final.json.gz')['files'].items())
changed=[p for p,h in before['workspace'].items() if hashlib.sha256(Path(p).read_bytes()).hexdigest()!=h];assert changed==['01_core/src/compiler/layout/tiling.rs'],changed
receipt=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),changed_rust=changed,head=subprocess.check_output(['git','rev-parse','HEAD'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),sources_stable=True)
with gzip.open(out/'final-receipt.json.gz','wt') as f:json.dump(receipt,f)
Path(str(out)+'.md').write_text(f'''# Tiling: célula geométrica, transparência e cobertura — 2026-10-14

## Proveniência e âmbito

HEAD `{receipt['head'].strip()}`, working tree não commitado. before-state,
before/results, before-expanded/results, red-state, candidate-final,
pre-validation, workspace-pre e final-receipt conservam fontes, owners,
hashes, UTC e diffstat do estado correspondente. A comparação é contra
vanilla pinado 9f2b6e8715237cb086899a42873660fe744622e8, árvore/receita P1489,
binário SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea.

Apenas `01_core/src/compiler/layout/tiling.rs` e seu owner
`00_nucleo/prompts/compiler/layout/tiling.md` foram alterados neste lote.
O recibo compara todos os Rust das camadas e benches com o estado inicial;
o restante do working tree preexistente foi preservado. Não houve alteração
de API, fase de pipeline, default, dependência, pin CI ou snapshots de PDF.

## Evidência causal e correção

O helper anterior inicializava o corpo da célula numa baseline. O teste
FixedMetrics mediu a forma em y=8,8 pt dentro de uma célula de 4 pt; o
recorte descartava a tinta. O layout agora usa a origem geométrica isolada
já existente, com baseline adquirida apenas quando há texto. A largura
automática considera a extensão dos itens retornados, além do avanço
final do cursor, para não transformar um shape de bloco em largura zero.
O body continua sendo layoutado uma vez; as cópias são visuais.

Uma célula explicitamente dimensionada e vazia retorna um grupo vazio
bem-sucedido. Antes, None encaminhava o caso ao fallback preto. Dimensões
inválidas são rejeitadas antes do sub-layout; offset/ângulo/pitch continuam
validados. Não se constrói grelha de milhares de células sem conteúdo.

A repetição usa o inverso dos cantos do recorte pela transformação real,
em lugar do quadrado baseado na diagonal ao redor da origem. O limite
existente de 10000 candidatos é preservado e não certifica casos maiores.
Fases de offset e posição são reduzidas pelo pitch antes de se combinarem,
e os índices são validados antes da conversão inteira. A redução conserva
periodicidade; não é autorização para mudar o referencial parent/rotação.

L0 foi atualizado primeiro, seguido de seis testes funcionalmente RED,
implementação e GREEN. red-fixture-compile.log registra um erro inicial do
fixture ao tentar comparar FrameItem sem PartialEq; não é RED comportamental.
O teste corrigido compara geometria. red.log e red-state preservam as seis
falhas reais. green.log registra 36 testes filtrados de tiling passando.
Os selos foram reparados apenas no par autorizado, em staging verificável.

## Render antes/depois

Recoleta {after['start']} a {after['end']}; binário Tekt SHA-256
`{after['binaries']['tekt']['sha256']}`. Fontes e binários ficaram estáveis.
Canvas inteiro a 144 dpi, limiar de tinta >32 por canal contra branco:

| Caso | Antes Tekt | Depois Tekt | Vanilla |
|---|---:|---:|---:|
| vazio explícito | 3840 | 0 | 0 |
| vazio denso | 40000 | 0 | 0 |
| denso com motivo | 40000 | 10000 | 10000 |
| self | 0 | 960 | 960 |
| spacing | 0 | 640 | 640 |
| auto shape | 3840 | 3840 | 3840 |

A contagem auto shape escondia o erro: antes os pixels eram pretos;
agora são vermelhos (255,65,54) e a imagem coincide integralmente com o
vanilla. Os histogramas estão em extra-comparison. Igualdade de contagem
nos demais casos não significa paridade visual: render-comparison conserva
todas as diferenças bilaterais, sem recorte ou tolerância criada para aprovar.
Os dois casos vazios também coincidem integralmente com o controle.

No offset de 10^12 pt, múltiplo do pitch 8 pt, o Tekt agora produz os mesmos
pixels do seu controle com offset zero. O vanilla pinado não pinta naquele
caso. Preserva-se a periodicidade; a perda do controle não foi copiada como
requisito. Não se declara superioridade geral do Tekt nem se atribui causa
interna ao vanilla sem investigação. A inspeção visual das duas imagens
rotacionadas confirma padrões visíveis, mas diferenças permanecem.

## Limites abertos e próxima frente

- O padrão parent tem fase diferente do controle, inclusive dentro de Place.
  Usar a posição local não demonstra ancoragem no referencial composto do pai.
- Rotação apresenta diferenças de geometria/cobertura; parent-rotated e
  self-rotated não estão certificados como paridade. Antialias também difere
  nas células pequenas; coincidência de contagem não o fecha.
- Em extra/over-limit, o corpo de 200×100 pt com pitch 1 pt ainda cai em preto:
  80000 pixels Tekt contra 20000 do padrão no vanilla. Ele excede o limite
  real; a correção de cobertura não remove esse limite de materialização.
- Em extra/stroke, o contorno continua sólido (4096 pixels contra 1024 do
  motivo). Shape só chama materialize_fill; stream/shape mantém to_color
  para o contorno. A modelagem atual não transporta a célula resolvida para
  esse consumidor. Não adicionar silenciosamente uma entidade pública
  ResolvedTiling: o L0 vigente a exclui e uma mudança pública exige ADR-0127.
- extra/text-auto continua com diferença de texto/geométrica; ellipse passa
  a desenhar o padrão, mas igualdade de tinta não certifica todo o export.
- Body vazio com size:auto inválido, targets/texto fora do contrato, regras
  de preenchimento dos clips e a implementação completa de padrões continuam
  sujeitos a investigação. Nada neste lote fecha o objetivo global.

## Validação

Suíte completa de {suite['start']} a {suite['end']}:
**{suite['passed']} passaram, {suite['failed']} falharam, {suite['ignored']} ignorados**,
com fontes/owners estáveis, oráculo pinado, linter CI e temporários em /dev/shm.
Build offline locked, rustdoc -D warnings, check all-targets, fmt, linhagem
estrita V1/V5/V15/V26, reversa e diff-check passaram. Clippy é inventário
com --cap-lints warn: {validation['clippy_after']} ocorrências, sem delta;
não se afirma aprovação de um gate -D warnings. Linter completo: zero erros,
{validation['architecture']['warning']} warnings e {validation['architecture']['note']} notas;
não é zero violations. {len(cmp['qpdf'])+len(extra['extra_checks'])} PDFs passaram no qpdf.

Scripts e recibos estão no arquivo evidence.json.gz com conteúdo e SHA-256
por entrada, relido e verificado após gravar. O objetivo permanece ativo;
CI remoto, release e demais débitos da auditoria não foram certificados.
''')
for p in Path('/tmp').glob('tekt-tiling-coverage-*.py'):
 dst=out/'scripts'/p.name;dst.parent.mkdir(exist_ok=True);shutil.copyfile(p,dst)
entries=[]
for p in sorted(out.rglob('*')):
 if p.is_file() and p.name!='evidence.json.gz':
  raw=p.read_bytes();entries.append(dict(path=str(p.relative_to(out)),sha256=hashlib.sha256(raw).hexdigest(),base64=base64.b64encode(raw).decode()))
with gzip.open(out/'evidence.json.gz','wt') as f:json.dump(entries,f)
with gzip.open(out/'evidence.json.gz','rt') as f:check=json.load(f)
for e in check:assert hashlib.sha256(base64.b64decode(e['base64'])).hexdigest()==e['sha256']
print('archive verified',len(check));print('report',str(out)+'.md')
