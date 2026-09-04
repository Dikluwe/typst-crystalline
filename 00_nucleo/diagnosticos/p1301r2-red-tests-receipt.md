# P1301r2 — recibo dos testes RED independentes P5-r2

## Veredito

**RED válido.** O filtro P1301 compilou e terminou com `exit 101` exclusivamente
porque as duas suites negativas novas rejeitaram a mensagem cristalina antiga e o
span alargado. Os dois controlos positivos passaram.

Regime: protocolo Tekt completo, papel `P5-r2` / testador A/B independente.

Atestação: **executado sem atestação de isolamento técnico**.

Não foi lida a implementação candidata nem o owner produtivo
`field_access`; não foram lidos oracles, mutantes, runner, recibos protegidos,
artefactos P1301 v1, `00_nucleo/materialization/` ou `00_nucleo/context/`.

## Entradas congeladas e capacidades

- Prompt test-only:
  `00_nucleo/prompts/compiler/eval/tests.md`, SHA-256
  `a4ade7eda600891465c62c320d80b7c2182c30dbb5f456de210c4a823b3e609a`;
  identidade semântica normalizada
  `bf0d711419ebe33ce7bc7b54c6bd187798ef94c64747e6a50a0aa5674905924a`.
- Manifesto: `00_nucleo/diagnosticos/p1301r2-manifest.json`, SHA-256
  `1526dc81e08a36ce0153fe7639dd2558c220a85d44c4b1de7098a59c099a5712`.
- Contrato: `00_nucleo/diagnosticos/p1301r2-contract.json`, SHA-256
  `69e852d38636afd0ce996f10a64cbf0e6dc0be12dca453a65ee3e0d515c88fad`.
- Selo: `00_nucleo/diagnosticos/p1301r2-contract-seal.json`, SHA-256
  `760846d4be24e6422b3284ae0853bc7647114cdfa25a92909207b1b10de9dcca`,
  estado `SEALED`, veredito `PASS`, fase seguinte autorizada `P5-r2`.
- Consumer test-only recebido antes da edição:
  `01_core/src/compiler/eval/tests.rs`, SHA-256
  `a870b5870dc38ed2aa7292c6371f92e6de8490bc573fcf5228c8431359950327`,
  igual ao baseline pré-candidato do manifesto.
- Escritas concedidas e usadas: somente
  `01_core/src/compiler/eval/tests.rs` e este recibo.
- O header e o `@prompt-hash` não foram alterados. Os testes P1300 e as demais
  mudanças pré-existentes foram preservados.

## Cobertura materializada

Foram acrescentados quatro testes `p1301` e um helper test-only que compara,
sem normalização:

- mensagem vanilla exata, hints vazios, uma diagnóstica primária, zero
  diagnósticas laterais e span field-only para `std.hsl`, `std.hsv` e
  `std.linear_rgb` nos perfis `default`, `html`, `a11y` e `html+a11y`;
- os mesmos observáveis para `calc.nope`, `sym.nope` e `color.map.nope`;
- sucessos e kinds de `std.rgb`, `calc.abs`, `sym.arrow` e
  `color.map.turbo`;
- controlo não-`Module` de dicionário com mensagem exata, hints vazios,
  cardinalidade e span cristalino preservado `10..21`.

`rustfmt` foi executado somente sobre
`01_core/src/compiler/eval/tests.rs`.

## Execução RED reproduzível

Janela exata: `2026-09-03T22:17:10,539572540-03:00` a
`2026-09-03T22:18:09,114298384-03:00`.

Comando único, incluindo criação dos temporários e `target` em `/dev/shm`:

```bash
bash -lc 'date --iso-8601=ns; mkdir -p /dev/shm/p1301r2-tmp /dev/shm/p1301r2-target; TMPDIR=/dev/shm/p1301r2-tmp CARGO_TARGET_DIR=/dev/shm/p1301r2-target cargo test -p typst-core p1301 -- --nocapture; rc=$?; date --iso-8601=ns; exit $rc'
```

Resultado exato: `4` testes executados; `2 passed`; `2 failed`; `0 ignored`;
`0 measured`; `5427 filtered out`; build concluído; `exit 101`.

Passaram:

- `p1301_lookups_de_modulo_existentes_preservam_valor_e_kind`;
- `p1301_controle_nao_module_dicionario_preserva_span_total`.

Falharam por expectativas semânticas novas:

- `p1301_std_aliases_mensagem_vanilla_e_span_field_only_nos_quatro_perfis`;
- `p1301_modulos_independentes_mensagem_vanilla_e_span_field_only`.

Testemunhas de mensagem e span:

- `std.hsl`: esperado ``module `global` does not contain `hsl` `` e
  `14..17`; observado `module 'std' does not contain field "hsl"` e
  `10..17`;
- `std.hsv`: esperado ``module `global` does not contain `hsv` `` e
  `14..17`; observado `module 'std' does not contain field "hsv"` e
  `10..17`;
- `std.linear_rgb`: esperado
  ``module `global` does not contain `linear_rgb` `` e `14..24`;
  observado `module 'std' does not contain field "linear_rgb"` e
  `10..24`;
- `calc.nope`: esperado ``module `calc` does not contain `nope` `` e
  `15..19`; observado `module 'calc' does not contain field "nope"` e
  `10..19`;
- `sym.nope`: esperado ``module `sym` does not contain `nope` `` e
  `14..18`; observado `module 'sym' does not contain field "nope"` e
  `10..18`;
- `color.map.nope`: esperado ``module `map` does not contain `nope` `` e
  `20..24`; observado `module 'map' does not contain field "nope"` e
  `10..24`.

As três testemunhas `std` repetiram-se identicamente nos quatro perfis. Em
todos os `15` casos `Module`, a cardinalidade primária foi `1`, a lateral foi
`0` e os hints foram `[]`; por isso somente mensagem e span aparecem como
mismatches. O controlo de dicionário também preservou essas cardinalidades,
hints `[]` e span `10..21`.

Houve duas tentativas preparatórias que não integram este veredito: a primeira
expôs um erro de empréstimo exclusivamente no novo assertion test-only, depois
corrigido; a segunda separou indevidamente a criação do `TMPDIR` da execução
e falhou ao encontrar o diretório efémero. Nenhuma foi classificada como RED.

## Proveniência da medição

`HEAD`:

```text
1f082370e59939de7b57992e137a9f74bfb6758f
```

Working tree não commitada. `git diff HEAD --stat`, recolhido imediatamente
após o run e antes de criar este recibo:

```text
 00_nucleo/prompts/compiler/eval.md                 |  39 ++-
 .../prompts/compiler/eval/bindings/field_access.md |  72 ++++
 00_nucleo/prompts/compiler/eval/tests.md           |  90 ++++-
 00_nucleo/prompts/compiler/stdlib/color.md         |  46 ++-
 01_core/src/compiler/eval/mod.rs                   |  14 +-
 01_core/src/compiler/eval/tests.rs                 | 363 ++++++++++++++++++++-
 01_core/src/compiler/stdlib/color.rs               |   6 +-
 7 files changed, 610 insertions(+), 20 deletions(-)
```

Esse stat inclui mudanças pré-existentes do utilizador. A contribuição
P5-r2 limita-se ao bloco P1301 acrescentado ao consumer test-only. O SHA-256
do consumer após os testes e antes deste recibo é
`6f2a17e97ae812728d8829ff3b69776970cfa02c09efe5c8e7e6720a82e429c0`.

## Limite do veredito

Este recibo prova somente que os testes independentes discriminam o baseline
pré-candidato no fragmento observável selado. Não certifica implementação,
GREEN, paridade CLI bilateral geral, lineage ressellada ou equivalência
funcional fora desse fragmento.
