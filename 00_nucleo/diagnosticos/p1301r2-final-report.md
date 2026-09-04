# P1301 revisão 2 — relatório terminal P8-r2

## Veredito

`P1301R2_CERTIFIED`

Regime Tekt completo, **executado sem atestação de isolamento técnico**.
O certificado limita-se ao fragmento observável selado: diagnósticos de field
ausente em `Module`, controles positivos, duas construções opacas e a
sentinela cristalina não-`Module`. Não declara equivalência funcional geral.

## Resultado terminal

Os nove artefactos congelados do selo re-hasharam sem desvio. O runner r2 foi
reexecutado no self-check e no gate discriminatório: `12/12` mutantes válidos
mortos, mutation score `1.0`, zero sobreviventes, zero mutantes inválidos e
zero operações não suportadas. Em cada ordem/repetição o controle nominal
permaneceu `20 Preserved / 0 Violated / 2 Unknown / 0 FAIL`; os dois `Unknown`
são exatamente os opacos previamente declarados, com
`UNSUPPORTED_DIAGNOSTIC_CARDINALITY`. A sentinela
`C23-dictionary-crystalline-span-sentinel` permaneceu separada e `Preserved`
em bytes `10..21`.

O candidato foi recompilado em release num `target` novo sob `/dev/shm` e o
runner selado foi chamado para `normal` e `inverted` dentro do mesmo processo
Python. Ambas as ordens deram `20 Preserved / 2 Unknown / 0 Violated / 0
FAIL`; os vinte casos requeridos tiveram zero `Violated`/`Unknown`, os dois
opacos ficaram exatamente `Unknown`, a sentinela ficou separada e
`Preserved`, e o vetor canónico por `case_id` foi idêntico. Nenhum argv usou
`--diagnostic-format` ou `--target`.

## Linhagem e separação

Os dois L0 preservaram os digests semânticos congelados. A reconstrução dos
full hashes anteriores confirmou que o delta de cada prompt foi somente a
linha 2 `Hash do Código`. O recibo de resselo documenta uma única execução de
escrita; depois do reparo test-only, o dry-run voltou a responder `Nothing to
fix`. Os headers finais são exatos, e V15/V26 confirmam ownership `1:1` e
integridade dos núcleos.

O reparo `candidate-repair-1` alterou somente expectativas antigas do helper
P1300 em `tests.rs`, alinhando-as à obrigação P1301 já congelada (`global` e
span somente do field). Não mudou L0, contrato, selo ou consumer produtivo e
não exigiu nova derivação.

A calibração v1 permanece apenas evidência de falha do harness. O runner v1
tem SHA-256 `30989db6…`, o runner r2 `622a97b8…`, e o runner r2 não referencia
nem carrega contrato, selo ou runner v1. O release recompilado produziu o
mesmo hash de conteúdo `ab61ce31…` do candidato transitório v1, mas foi
construído do estado corrente num diretório novo; nenhum artefacto v1 foi
reutilizado.

## Gates finais

- `cargo test -p typst-core p1300`: `10 passed`, `0 failed`, exit `0`.
- `cargo test -p typst-core p1301`: `4 passed`, `0 failed`, exit `0`.
- `cargo build`: exit `0`.
- `crystalline-lint .`: exit `0`; `236` warnings e `1132` infos consultivos,
  sem linhas de erro.
- `crystalline-lint --fail-on warning --checks v3,v4,v5,v13,v14,v15,v26 .`:
  `✓ No violations found`, exit `0`.
- `crystalline-lint --fix-hashes --dry-run .`: `Nothing to fix`, exit `0`.
- `git diff --check`: sem output, exit `0`.

Os warnings Rust e V16/outros avisos do lint geral são dívida preexistente e
não bloqueiam os gates exigidos; o recorte arquitetural com warning fatal
ficou limpo.

## P1300 desbloqueado

O veredito predecessor `P1300_BLOCKED_IMPLEMENTATION` identificou precisamente
a divergência de mensagem/span de `Module` que P1301 reabriu. Com a correção
P1301r2, as regressões P1300 agora passam `10/10` e a matriz pública selada
passa nas duas ordens. Assim, o bloqueio observado por P1300 foi resolvido sob
esta nova cadeia; o recibo P1300 histórico não é retroativamente reescrito.

## Artefactos

- Recibo: `00_nucleo/diagnosticos/p1301r2-verification-receipt.json`, SHA-256
  `9b7bcdefdb65ecca2a1d1cccc3fe6dea958328c494e47a52f31c8dc0645c5e0d`.
- Certificado: `00_nucleo/diagnosticos/p1301r2-certificate.json`, SHA-256
  `3e78d10bc0d45843371ba8a607ce26a7d3bb43c28c711312e01731c26e053b4a`.
- `git status --short` final: SHA-256
  `1715eb1ae1eae4c923a88358de66e33df73ac017f209584ea08ec06b520b3573`.

HEAD verificado: `1f082370e59939de7b57992e137a9f74bfb6758f`. Working tree
não commitada; imediatamente antes destes outputs havia `8` ficheiros
tracked modificados e `65` paths untracked. Os três paths acima são toda a
contribuição de escrita de P8-r2.
