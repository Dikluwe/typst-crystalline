# P1300 — receipt dos testes A/B RED

**Papel:** P5, autor independente dos testes permanentes
**Regime:** A/B segregado dentro do protocolo completo P1300
**Atestação:** executado sem atestação de isolamento técnico; o filesystem é
compartilhado, mas as capacidades de leitura e escrita foram restringidas por
allowlist e nenhuma implementação candidata existia durante o RED final.

## Entradas congeladas

- `00_nucleo/diagnosticos/p1300-manifest.json` — SHA-256
  `c3e946e1330ccab108dba2f9a438fd6a8d4c3a2af7fca3307842dbd7403feed8`;
- `00_nucleo/diagnosticos/p1300-contract-seal.json` — SHA-256
  `c8cb9cbfad604e6f5c638c63dfb1dc8c91a36f3bf6f50601624ed27de2439956`;
- `00_nucleo/prompts/compiler/eval/tests.md` — SHA-256
  `d4d6382d351e084869445fe58e1cb484e762d5b5dfc843b7974cd21149289104`;
- `01_core/src/compiler/eval/tests.rs` baseline — SHA-256
  `b6b87d6786874a39b3071c667476d062a4fc41a9298c6519d9434e0ae637df20`;
- `01_core/CLAUDE.md` — SHA-256
  `d358e69f77ceb88bfd7f64bbf64f90b88b6e97c0bac88566f0eb94315b48829f`;
- `HEAD` baseline e ainda corrente no RED final —
  `1f082370e59939de7b57992e137a9f74bfb6758f`.

O contexto começou fresco com a allowlist P5 do manifest. Após se provar que
`html` e `a11y-extras` não são Cargo features de `typst-core`, o coordenador
forneceu a nota canônica pré-P6
`00_nucleo/diagnosticos/p1300-p5-public-api-note.md`, SHA-256
`869adc77a4611965864dc103c418a1c63229f4340fb25cd0e7c720c86c50825f`.
Essa ampliação informacional contém somente as assinaturas públicas baseline
de `eval_expression_with_features`, `Features` e `Feature` e as quatro
construções runtime. Não abri os sources citados pela nota.

Por exigência operacional do ambiente, li também a skill
`tekt-materializacao-segregada`; para diagnosticar a tentativa inválida de
Cargo feature, consultei apenas Cargo manifests. Não li
`compiler/eval/mod.rs`, `compiler/stdlib/color.rs`, contrato, oracle, mutants,
runner, patch candidato nem outputs P6–P8. Não invoquei o vanilla.

## Testes materializados

O consumer test-only ficou com SHA-256
`41b7f7aea339e4e4de8ae33c1e3be9d2ada5799161d2bd500365468b9746606b`.
O header `@prompt-hash` foi deliberadamente mantido sem alteração para o
resselo mecânico de P7.

Os testes usam somente `MockWorld` puro e APIs test-only existentes. Cada caso
é executado nos quatro perfis runtime:

1. `Features::empty()` (`default`);
2. `Features::html()` (`html`);
3. `Features::empty()` + `Feature::A11yExtras` (`a11y`);
4. `Features::html()` + `Feature::A11yExtras` (`html+a11y`).

Foram criados seis testes negativos independentes: três aliases bare e três
fields sob `std`. Para cada perfil, eles congelam quantidade de diagnósticos,
mensagem exata, hints vazios, posição pública `(1, 0)` e byte range público da
expressão inteira. Um controle com nomes sentinela ausentes executa esses dois
ramos diagnósticos no baseline, totalizando oito observações negativas de
controle já GREEN.

Três controles positivos, também nos quatro perfis, preservam:

- `color.hsl`, `color.hsv` e `color.linear-rgb` como funções chamáveis, seus
  reprs de função e valor, o valor `Color`, `color.space` e a identidade do
  espaço;
- os cinco globals bare `rgb`, `luma`, `cmyk`, `oklab`, `oklch` como funções e
  chamadas que produzem `Color`;
- os mesmos cinco constructors sob `std`, como funções e chamadas que produzem
  `Color`.

Isso separa apagamento de rota/nativa, correção de apenas uma projeção e deriva
entre os quatro perfis.

## Execução RED final no baseline

Janela: `2026-09-03T20:03:01-03:00`–`2026-09-03T20:04:10-03:00`.

```text
CARGO_TARGET_DIR=/dev/shm/typst-crystalline-p1300-p5
RUSTFLAGS=-Awarnings
CARGO_TERM_COLOR=never
cargo test -p typst-core p1300 -- --test-threads=1
```

Resultado: exit `101`; compilação concluída; `10` testes executados, `4`
passaram, `6` falharam, `0` ignored, `0` measured e `5417` filtered out; tempo
do lote `0.25s` (`59.79s` de build reportado).

Passaram exatamente:

- `p1300_cinco_globals_bare_preservam_funcoes_e_chamadas`;
- `p1300_cinco_globals_std_preservam_funcoes_e_chamadas`;
- `p1300_controle_diagnosticos_negativos_exatos_nos_quatro_perfis`;
- `p1300_rotas_color_preservam_funcoes_chamadas_repr_e_space`.

Falharam exatamente, e somente, as seis expectativas negativas:

- `p1300_hsl_bare_unknown_variable`;
- `p1300_hsv_bare_unknown_variable`;
- `p1300_linear_rgb_bare_unknown_variable`;
- `p1300_std_hsl_missing_field`;
- `p1300_std_hsv_missing_field`;
- `p1300_std_linear_rgb_missing_field`.

Cada falha listou os quatro perfis
`["default", "html", "a11y", "html+a11y"]` como ainda disponíveis. Assim, o
RED prova separadamente os três aliases bare e os três aliases sob `std`, e
prova que o baseline os aceita em todos os quatro perfis. Não houve falha de
compilação, import, fixture, controle positivo ou contrato diagnóstico.

## Tentativa inválida excluída da evidência

Em `2026-09-03T19:55:41-03:00`, a tentativa
`cargo test -p typst-core --features html p1300 -- --test-threads=1` terminou
antes da compilação com exit `101` e a mensagem
`the package 'typst-core' does not contain this feature: html`. Ela demonstrou
somente que os perfis são runtime, não Cargo features, e não foi contada como
evidência P1300.

## Gates adicionais

- `cargo fmt --all -- --check` — exit `0`;
- `CARGO_TARGET_DIR=/dev/shm/typst-crystalline-p1300-p5 RUSTFLAGS=-Awarnings CARGO_TERM_COLOR=never cargo check -p typst-core --tests` — exit `0` (`32.00s`);
- `git diff --check` — exit `0`.

Não fiz staging, commit ou push. Escrevi somente
`01_core/src/compiler/eval/tests.rs` e este receipt.
