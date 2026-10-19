from pathlib import Path
import gzip,json,hashlib,subprocess,datetime
out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();assert all(sha(p)==h for p,h in b['hashes'].items());m=json.load(gzip.open(out/'before-measurement.json.gz'));assert len(m['cases'])==36
p=Path('lab/typst-original/crates/typst-layout/src/flow/collect.rs');ref=subprocess.check_output(['git','show','9f2b6e8715237cb086899a42873660fe744622e8:crates/typst-layout/src/flow/collect.rs']);assert ref==p.read_bytes();(out/'reference.json').write_text(json.dumps(dict(path=str(p),sha256=sha(p),pin='9f2b6e8715237cb086899a42873660fe744622e8',equal=True),indent=2)+'\n')
owner=Path(b['owner']);owner.write_text(owner.read_text()+'''

## V16 — Source, corpo vazio e fronteira Par (2026-10-17)

### Medição antes da decisão

No HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não
commitado, `01_core/src/compiler/layout/paragraph.rs:340-363` reconhece Par
somente por Styled e classifica corpos vazios/texto vazio sem atravessar Source.
`01_core/src/compiler/layout/style_scope.rs:23-31` precisa desse reconhecimento
para fechar a linha anterior antes do estilo; `paragraph.rs:385-422` decide
avanço vazio/virtual top pelos outros dois predicados. O eval anexa Source a
ContentBlock em `01_core/src/compiler/eval/expression_dispatch.rs:49-55`.

`00_nucleo/diagnosticos/tekt-v16-paragraph-2026-10-17/before.json.gz` e
`before-measurement.json.gz` conservam UTC, HEAD/status/diffstat, hashes de
fontes/L0/configurações/binários/fonte tipográfica e 18 casos bilaterais (36
execuções) contra `9f2b6e8715237cb086899a42873660fe744622e8`. Com DejaVu Sans
12pt, página auto e margem 8pt, empty-before e source-empty põem AFTER em
8.000013pt versus 22.400001pt; empty-after mede altura 25.1172pt versus
39.51719pt; styled-empty põe AFTER em 38.717213pt versus 60.317183pt.
space-par também perde 14.4pt. Text vazio já coincide e precisa preservar seu
top-edge virtual. `lab/typst-original/crates/typst-layout/src/flow/collect.rs:100-101,164-190`
conserva a fronteira e o spacing de Par. A fonte foi comparada ao objeto Git do
pin (reference.json). Isso mede linguagem, não intenção ou equivalência de bytes.

### Decisão e prescrição exclusiva

Correção interna/de paridade, fluxo contínuo ADR-0127. A inferência de que Source
causa as perdas será refutada se a mudança não restaurar altura/espaçamento dos
casos de origem ou alterar controles materiais/opacos. O núcleo source-origin
já exige transparência; a mecânica do match não pertence à paridade.

Nos três predicados is_paragraph/is_empty_body/contains_empty_text, Source
recorre somente ao body. Preservar Styled e Sequence exatamente onde já eram
admitidos; não promover Sequence a Par, Strong/Emph/Located/Realized/ShowRecipe
ou unidade opaca a corpo vazio por esta revisão. Reconhecimento via Source
permite fechar a linha anterior antes de instalar o estilo do Par. Corpo vazio
com Source conserva a fronteira e spacing explícito; Text vazio continua a
reservar seu top-edge uma única vez. Não emitir tinta, reavaliar callbacks,
fabricar Location, mudar a primeira origem ou mover decisões de eval para layout.

Substituir cada fallback de Content pelos nomes explícitos do complemento
atual, mantendo false para os casos não aceitos. Grupos V19 compartilham uma
única política de recusa em cada predicado; manter as notas e a justificativa,
sem dezenas de braços idênticos nem supressão V16. Variante Content futura
exige decisão explícita por exaustividade Rust. Não reescrever como if-let ou
matches para ocultar os avisos. Demais funções, fórmulas, certificados, corpos
opacos e estados privados deste owner permanecem inalterados.

### Aceitação e pendências

Antes da implementação: testes funcionais RED de geometria/espaçamento e
restauração de estilo com Source e controles de Text vazio/material/objeto;
não contar falha de compilação ou leitura AST como RED funcional. Depois:
GREEN, repetição dos 18 casos com páginas/palavras/caixas e tolerância absoluta
0.002pt para arredondamento PDF (nunca bytes nem normalização de texto).
Nenhum caso previamente coincidente pode regredir. Sonda rustc isolada com os
três matches reais, todas as formas Content vigentes e payloads mínimos deve
compilar o controle e rejeitar uma nova variante com E0004 nos três locais.
Essa sonda mede cobertura estrutural, não execução completa do compilador.

Resselo somente deste owner/consumer; build, fmt, Clippy estrito/all-targets,
Rustdoc -D warnings/quatro índices, suíte workspace, V1/V5/V15/V26, linhagem
reversa e diff check. Diferencial SARIF remove exatamente três V16, admite
somente as três notas V19 prescritas e preserva os demais diagnósticos. Comandos
pesados sequenciais, cada gate em seu cgroup com MemoryMax=12GiB, swap zero,
MemoryHigh=10GiB e Cargo jobs=1; não atribuir cache agregado a um processo.

O corpus inicial com text(18pt) mediu quatro falhas independentes do constructor
antes de Par. Elas permanecem abertas em positional-size.json.gz e não são
resolvidas usando size nomeado nos controles. Par aninhado, outros envelopes,
HTML e demais limites anteriores não são certificados por esta rodada.
''')
(out/'prescription.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),owner=str(owner),sha256=sha(owner),mode='ADR0127 continuous internal parity correction'),indent=2)+'\n')
