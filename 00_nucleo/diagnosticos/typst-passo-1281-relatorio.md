# Relatório do Passo 1281 — infraestrutura de prova de paridade

## Veredito

**APROVADO.** A suíte voltou a ser autocontida, a matriz diferencial aplica as
features HTML simetricamente, falhas do harness não são aceites como paridade e os
fixtures obrigatórios estão no índice do Git.

Este relatório prova apenas o fechamento do Passo 1281. Não constitui alegação de
paridade geral do produto.

## Regime e segregação

Foi usado o protocolo completo de materialização segregada:

- autoria do contrato: agente `contrato_p1281`;
- implementação: agente raiz;
- ataques: agente `ataques_p1281`;
- verificação independente: agente `verificador_p1281`.

A segregação foi procedural. Não há atestação de isolamento técnico de leitura do
filesystem compartilhado.

## Baseline e proveniência

- Início da medição: `2026-08-29T23:00:22-03:00`.
- Fechamento da medição: `2026-08-29T23:29:28-03:00`.
- HEAD: `53d21c5a602f4045a769a0ab0c935baa5ecd3b88`.
- Vanilla normativo: upstream/main `a51e02804`.
- Binário vanilla: `/usr/local/bin/typst`.
- SHA-256 vanilla:
  `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
- Binário cristalino: `target/release/typst`, compilado do HEAD e working tree
  descritos neste relatório.
- SHA-256 cristalino:
  `0a4d71415b4e3ba47ed9c50f08b612a61fd14e1e538790ea4e51245d3019ce3f`.
- SHA-256 do passo executado:
  `2c2b5da9c2c683a9146481a1319a25b9416c97c78ee9c941b659bba701a10cb0`.

No baseline, `git diff HEAD --stat` estava vazio e existiam somente os documentos
de materialização P1281–P1287 não rastreados, pertencentes ao utilizador. Esses
documentos não foram alterados.

## Contrato selado

O contrato `C-P1281-v1` restringiu a mudança aos fixtures necessários, ao padrão de
ignore, ao L0 dono dos fixtures, aos testes e ao harness/matriz. Ficaram proibidas
mudanças em código produtivo, API pública, comportamento padrão, fase do pipeline,
tolerâncias semânticas e fixtures sem referência direta.

Foram auditados os L0 aplicáveis e os ADR-0107, ADR-0108, ADR-0127 e ADR-0129. A
alteração foi classificada como reparação interna de fixture/harness: fluxo contínuo
pela ADR-0127. O L0 `00_nucleo/prompts/infra/export-fixtures.md` foi atualizado antes
do consumer e ressellado.

## RED → GREEN

O RED inicial foi reproduzido com `cargo test --workspace --no-run`: havia 13
inclusões ausentes, correspondentes a 10 fontes `.typ` únicas e à fonte Noto Sans
usada por dois conjuntos de testes. Os testes novos do runner também demonstraram
RED para manifest sem `expected_state`, fixture ausente, assimetria HTML, dois lados
falhando em comparação de capacidade e artefato PNG malformado.

O GREEN foi obtido sem regenerar expectativas para esconder divergências. O primeiro
GREEN parcial revelou ainda o caminho de fonte obsoleto no caso P307b-09; ele foi
corrigido para a fonte versionada e o snapshot existente voltou a coincidir.

## Fixtures recuperados

As fontes diretamente exigidas por `03_infra/src/integration_tests.rs` foram
restauradas e adicionadas ao índice:

| Fixture | SHA-256 |
|---|---|
| `.typ/sec_04.typ` | `bb18aa86a321bb883353d0fd8b440a5d42f6e5a132358f99a010e95c107ed7eb` |
| `.typ/sec_07.typ` | `e7caadc858d220618ba1bd330ac88aa1490768e3e78728807bdeb5ca8d53c629` |
| `.typ/sec_09.typ` | `51069d1dc757c738a36199ab41cca3755c1713bd4358c0f10113b15b5ae88278` |
| `.typ/sec_10.typ` | `6f420a160bc1f144d7623deef6128a5858a66e51d160e8a1965789d38f31b994` |
| `.typ/sec_12.typ` | `6977f54eb0ff753f5cbdabd4453250262a0abf8bfd7d84b410c14c76a8f58013` |
| `.typ/sec_16.typ` | `921b69e5d5524bb02b7e92686b2c10dc6b73a21079a105ab3bf4e31f0e27c382` |
| `.typ/sec_17.typ` | `18404850471b7812d67c4f846e6fe17c31425c4c15fd5dea304a29f486099653` |
| `.typ/sec_18.typ` | `cfefb865975201b2d920701bbd7840cdfef2f656678b5de9821fa1adf0eeef2e` |
| `.typ/sec_22.typ` | `f92b47555012863e9250f54d0f6eacf12f92a1b9227587098dd57a8e9492b289` |
| `.typ/sec_27.typ` | `67d8004602b42501b0aced71cfb6a6fafa3a3e58af26da6b0241fa53bd23648c` |

A fonte `03_infra/fixtures/fonts/NotoSans-Regular.ttf` tem 556.216 bytes e
SHA-256
`2ec33f84606cbaa0a1a944488e14f97faf2f6a25ecdd8354f5358f06da13c7d9`.
Ela é byte-idêntica ao artefato do checkout Cargo de Krilla `7772dbe`. A licença
OFL-1.1 foi preservada em `03_infra/fixtures/fonts/LICENSE-NotoSans.md`, SHA-256
`ff36f3394f04c7642af3b0138ce75fbf9ac426df6e9f0f9882f1c54e2122d99e`. O texto é
o original do mesmo checkout, com os finais de linha `CR` normalizados para `LF`.

O `.gitignore` continua a ignorar o diretório `/.typ` por padrão, mas libera somente
os 10 fixtures obrigatórios. `git check-ignore --no-index` não os considera ignorados
e `git ls-files --error-unmatch` confirma os 12 novos artefatos no índice.

## Matriz diferencial canônica para P1282

Comando:

```sh
python3 lab/parity/matrix/runner.py \
  --vanilla /usr/local/bin/typst \
  --crystalline target/release/typst \
  --output /tmp/p1281-matrix-final.json
```

Resultado: exit `0`, 19 casos, sendo 17 `MATCH` e 2 `DIFF` esperados; nenhuma
expectativa falhou. O JSON da execução tem SHA-256
`dc0ddc511958ccd80777df70cd0293c03aa7a731e47b89d67011538c4941d6eb`.

Features por lado nos casos HTML:

| Caso | Vanilla | Cristalino |
|---|---|---|
| `P1137-X-002` | `--features html` | `--features html` |
| `P1138-X-004` | `--features html` | `--features html` |

A expectativa obsoleta de `P1138-L-001` passou de `DIFF` para `MATCH` somente após
a medição observar a mesma geometria nos dois lados. As divergências que permanecem
são `P1138-X-002` (raster PNG) e `P1138-X-003` (observáveis PDF).

O runner agora exige `expected_state`, valida existência e confinamento das fontes,
valida simetria das features HTML, registra SHA-256 dos binários e separa:

- diferença real do produto: `DIFF` com classe de linguagem/formato;
- falha de ferramenta externa ou do oráculo: `ERROR`/`HARNESS_DEFECT` ou
  `BASELINE_DEFECT`;
- sucesso real de ambos os lados: elegível a `MATCH`.

## Ataques e mutation score

O conjunto adversarial desenhou 14 mutações: fixture ausente/ignorado, perda dos
canários SVG, troca ou truncamento da fonte/licença, assimetria HTML nos dois sentidos,
expectativa stale, ausência de `expected_state`, falsa igualdade quando ambos os lados
falham, artefato malformado, ferramenta externa ausente, identidade binária fraca e
dependência da ordem dos casos. Todas foram rejeitadas pelos gates combinados:
**14/14 = 1,0**.

O verificador independente repetiu o subconjunto executável selecionado sem consultar
expectativas internas e obteve **8/8 = 1,0**.

## Gates finais

| Gate | Resultado |
|---|---|
| `cargo test --workspace --quiet` | exit `0`; 6.340 testes passaram, zero falhas; 3 ignorados |
| testes `sec_*` | 15/15 |
| canário SVG | 1/1 |
| snapshots P307b | 9/9 |
| `python3 -m unittest lab/parity/matrix/test_runner.py` | 26/26 |
| `python3 lab/parity/matrix/runner.py --validate-only` | 19 casos válidos |
| matriz diferencial canônica | 17 `MATCH`, 2 `DIFF`, exit `0` |
| `cargo build --workspace --bin typst` | exit `0` |
| `cargo build --workspace --release --bin typst` | exit `0` |
| `crystalline-lint --checks v5,v15,v26 --fail-on warning .` | zero violações, exit `0` |
| `crystalline-lint .` | exit `0`; apenas avisos/informações preexistentes |
| `cargo fmt --all -- --check` | exit `0` |
| `git diff --check` | exit `0` |

O verificador independente emitiu **APROVADO** depois de confirmar os 12 novos
artefatos como adições no índice. Os artefatos temporários da matriz ficaram em
`/tmp`; nenhum cache ou resultado de build foi acrescentado ao repositório.
