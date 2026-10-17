# Commit do trabalho e re-sync vanilla — 2026-10-17

O trabalho anterior foi consolidado no commit local
`e2d7f6cdc93c5f6b948ca80167411d390ccc714d`. A referência de linguagem foi atualizada de
`9f2b6e8715237cb086899a42873660fe744622e8` para
[`bd5ebec285505889e8ff1d4f68804ab147a699f3`](https://github.com/typst/typst/commit/bd5ebec285505889e8ff1d4f68804ab147a699f3).
A consulta oficial foi repetida antes do fechamento; HEAD e main continuam nesse pin.
O pedido humano autoriza commit e atualização da referência; não houve push ou release.

## Proveniência e escopo

Estado medido: HEAD `e2d7f6cdc93c5f6b948ca80167411d390ccc714d` com working tree não commitado
na atualização. UTC, lista exata de alterações, diffstat, hashes, comandos e
identidade dos executáveis estão em `validation-state.json.gz`,
`workspace/workspace-pre.json.gz` e no pacote `evidence.json.gz`.
`precommit-state.json.gz` registra o estado anterior à consolidação.
Fechamento do relatório: 2026-10-17T21:26:43.601420+00:00.

- Upstream oficial: `https://github.com/typst/typst.git`, branch main observada e depois fixada.
- Commit: `bd5ebec285505889e8ff1d4f68804ab147a699f3`; árvore: `cac836c07e4dec2a89219370428292393a3216b5`.
- Binário: `lab/typst-original/target/release/typst`.
- SHA-256: `9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.
- CLI auxiliar: `typst 0.15.1 (bd5ebec2)`.
- `sys.version` medido diretamente em ambos os produtos: `version(0, 15, 1)`.
- `PARITY_VERSION` e os contratos de produção permanecem com os mesmos valores.

O intervalo upstream contém 20 commits. A cópia
quarentenada foi atualizada por diff Git entre os pins e conferida por hash de blob.
O único arquivo upstream diferente é `Cargo.lock`, por causa do overlay local
preexistente. Os dois arquivos extras pertencem a `p1140-inventory`; a revisão
emitida pelo extrator foi atualizada e seu executável reconstruído. Os hashes do
overlay, receita e lockfile estão no recibo. O binário antigo P1489 foi preservado
localmente em `lab/typst-original/target/release/typst-9f2b6e871-p1489`.
O verificador usado pelo CI aceitou localmente um checkout independente e limpo
do novo commit/árvore. Os workflows remotos não foram executados nesta etapa.

Atualizações: pin e árvore em `tools/ci/build_oracle.py`, checkout do oráculo no CI,
fonte em `lab/typst-original`, referência vigente em CLAUDE/README e documentação
operacional, runner/schema/manifest da matriz, merge e extrator do inventário,
smoke vanilla e SHA das sondas. O builder do oráculo passou a um job.
A mensagem do harness p1293 foi atualizada com L0 primeiro e resselo restrito;
comparadores, esperados e tolerâncias não foram relaxados. Não houve alteração
funcional produtiva no compilador para acompanhar mecanicamente o vanilla.

Medições e propostas históricas conservaram seus pins. O manifesto da matriz
identifica a autoridade nova e mantém explícita a proveniência anterior de cada
caso. Antes de implementar qualquer diferença futura, reauditar o L0 proprietário
com a nova fonte conforme ADR-0107/0108/0127/0129.

## Validação local

Suíte workspace: **8932 aprovados, 0 falhas,
7 ignorados**, com fixtures em `/dev/shm`, um job, uma thread de
teste e o oráculo novo verificado. Os hashes de fontes/L0 permaneceram congelados.
Smoke Rust separado do vanilla: 2 aprovados. Testes Python: 16 de CI, 85 da matriz
e 25 do inventário aprovados. Os quatro índices Rustdoc foram confirmados.

| Gate | Exit |
|---|---:|
| build | 0 |
| workspace | 0 |
| vanilla-smoke-build | 0 |
| vanilla-smoke | 0 |
| fmt | 0 |
| clippy | 0 |
| rustdoc | 0 |
| lineage | 0 |
| reverse | 0 |
| architecture | 1 |
| diff | 0 |

O exit 1 de architecture preserva os **106 V16** preexistentes; também foram
observadas 1099 notas V19 e 873 notas V20, sem alteração dessas contagens.
O gate arquitetural global continua bloqueado por esses V16. Linhagem estrita
V1/V5/V15/V26 e verificação reversa passaram; isso não equivale a arquitetura
inteiramente aprovada nem autoriza release.

## Matriz bilateral contra o pin novo

Cada perfil avaliou 43 casos e executou os casos ativos nas duas ordens. Resultados são locais
aos observáveis declarados de cada caso; não constituem percentual de paridade
global. DISABLED_BY_PROFILE não recebe crédito de paridade. Igualdade de bytes
ou da mecânica Rust não é requisito de linguagem (ADR-0107).

| Perfil | MATCH | DIFFERENCE | VIOLATED | DISABLED_BY_PROFILE | UNKNOWN |
|---|---:|---:|---:|---:|---:|
| default | 27 | 2 | 0 | 14 | 0 |
| html | 34 | 2 | 0 | 7 | 0 |
| a11y-extras | 28 | 2 | 0 | 13 | 0 |
| html+a11y | 35 | 2 | 0 | 6 | 0 |
| bundle | 31 | 2 | 0 | 10 | 0 |
| html+bundle | 40 | 2 | 0 | 1 | 0 |

Todos os perfis corresponderam às expectativas anteriores ao re-sync, com
UNKNOWN=0 e VIOLATED=0. A inversão de ordem foi idêntica nos seis perfis.
`matrix-expectations-integrity.json` confirma que casos, comandos, comparadores
e esperados são idênticos aos do checkpoint anterior.

Os dois DIFFERENCE permanecem registrados em todos os perfis:

- `P1138-X-002`: PNG com dimensões coincidentes, 11 de 2.005.644 pixels
  diferentes e variação máxima de um canal. O comparador de pixels continua
  classificando DIFFERENCE; a medição não demonstra defeito semântico.
- `P1138-X-003`: página, texto e geometria PDF coincidentes no observável do
  caso; nome interno de fonte `LibertinusSerif-Regular` versus
  `LibertinusSerif-Regular-Identity-H`. Não se exige copiar a mecânica Rust
  ou os bytes do PDF para obter paridade de linguagem.

Essas duas diferenças de formato não foram convertidas artificialmente em
MATCH. Os estados se referem somente a esta amostra e aos seus comparadores.


Sondas adicionais, com o mesmo binário novo:

- symbols: `{"EXTENSION": 36, "MATCH": 146, "TEXT_MATCH_CARRIER_DIFF": 16}`.
- language-probes: `{"DISABLED_BY_PROFILE": 3, "MATCH": 9, "VIOLATED": 0}`.

Nos 16 TEXT_MATCH_CARRIER_DIFF, o glifo coincide, mas a observação pública
expõe `symbol` no vanilla e `text` no cristalino. É a pendência de morfologia
SY1 já registrada; esses casos não recebem MATCH integral nem são descartados
como igualdade mecânica Rust. O exit 0 do auxiliar não fecha essa diferença.
As 36 extensões são conservadas e não contam como ganho de paridade.

O inventário vanilla foi emitido novamente nos seis perfis, com revisão e hash
do produto verificados. Isso valida a identidade do lado vanilla; a comparação
global de cobertura do projeto continua dependente de inventários bilaterais
frescos, sem transportar percentuais históricos.

## Recursos e limites

Jobs pesados sequenciais, `CARGO_BUILD_JOBS=1`, MemoryHigh=10 GiB,
MemoryMax=12 GiB, MemorySwapMax=0. Pico medido dos cgroups desses jobs:
**10742022144 bytes (10.004 GiB)**. Não houve OOM nem atingimento de
MemoryMax; a validação acionou o controle MemoryHigh, registrado nos contadores.
Esse pico inclui o uso contabilizado pelo cgroup, não somente RSS de rustc.
Recursos completos por comando estão no pacote de evidências.

## Pendências e entrega

O delta upstream inclui igualdade com LooselySpanned, bibliografia com
hanging-indent, rastros de erro, correção de densidade de imagens e ajustes de
HTML/PDF/fontes. O diff de fonte os identifica como candidatos a sondas próprias;
não prova automaticamente divergência nem ganho de paridade no cristalino.
As frentes de paridade e os 106 V16 já abertos continuam nas etapas próprias.
O objetivo amplo permanece em pausa após esta entrega delimitada.

Artefatos: `oracle-build.json.gz` (recibo), `evidence.json.gz` (resultados e logs),
`validation-state.json.gz` (estado e hashes) e `manifest.sha256` (integridade).
