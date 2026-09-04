# P1301 — recibo RED test-only independente (P5)

## Identidade, fronteira e isolamento

- Papel: testador A/B independente P5.
- Regime: protocolo completo, executado sem atestação de isolamento técnico.
- Limitação: a segregação foi lógica num working tree e filesystem compartilhados; este recibo não alega isolamento de processo, de checkout ou de alterações concorrentes.
- Não foram lidos `01_core/src/compiler/eval/bindings/field_access.rs`, a implementação candidata, o oracle privado nem os mutants.
- Não foi necessária nenhuma extensão de leitura a assinaturas públicas em `eval/mod.rs` ou entities: os helpers e tipos requeridos já estavam visíveis no consumer test-only autorizado.
- `repr(std)` permaneceu estritamente fora do teste e não foi executado, comparado nem usado como critério.
- Ficheiros editados por P5: somente `01_core/src/compiler/eval/tests.rs` e este recibo, sempre via `apply_patch`.

## Inputs selados revalidados

Antes da autoria do teste foram recalculados os SHA-256:

| Artefacto | SHA-256 observado |
|---|---|
| `AGENTS.md` | `bc50c0c6d54c0e301a5fe3c5c5869dbeef8fdf624096b0fa20185122c64d7da0` |
| `00_nucleo/diagnosticos/p1301-manifest.json` | `074fabeda17f741f064fe1d561ad6a78ad81e45654861825586664a9646f05a8` |
| `00_nucleo/prompts/compiler/eval/tests.md` | `a4ade7eda600891465c62c320d80b7c2182c30dbb5f456de210c4a823b3e609a` |
| `00_nucleo/diagnosticos/p1301-contract.json` | `2d8d7a141d63c13473681abdefa24ed9370e03925824a9cae74ca26305ea85f5` |
| `00_nucleo/diagnosticos/p1301-contract-seal.json` | `658fe879eaf7dce5b0a6372f686e376e7cc652f621d298c817628b03e16f82fb` |
| `01_core/src/compiler/eval/tests.rs` pré-candidato | `a870b5870dc38ed2aa7292c6371f92e6de8490bc573fcf5228c8431359950327` |

Todos coincidiram com o manifesto, o contrato e a encomenda de P5. O consumer test-only final usado na medição RED tem SHA-256 `84840c4ae59e5d97ddc7672c1abf06ead1e32ffae61a0d499f7ba566d9225fee` e header `@prompt-hash a4ade7ed`.

## Teste materializado

O filtro `p1301` adiciona oito testes:

1. `std.hsl`, `std.hsv` e `std.linear_rgb` exigem, em `default`, `html`, `a11y` e `html+a11y`, respectivamente, `module `global` does not contain `<field>``; hints vazios; e spans somente do field `14..17`, `14..17` e `14..24`.
2. `calc.nope`, `sym.nope` e `color.map.nope` exercem a regra categorial com nomes públicos `calc`, `sym` e `map`, mensagens vanilla exatas e spans `15..19`, `14..18` e `20..24`.
3. Um controle positivo preserva valor/kind em `std.rgb`, `calc.abs`, `sym.arrow` e `color.map.turbo`.
4. Um sentinela não-`Module` preserva mensagem, hints vazios e o whole-span cristalino `10..21` de `repr(type((a: 1).nope))`.

O helper P1300 preexistente foi retificado para que os testes plain `std.hsl`, `std.hsv` e `std.linear_rgb` também exijam o nome `global`, a mensagem vanilla com backticks e somente o field no span; os negativos bare continuam inalterados.

## Proveniência exata da medição aceita

- Comando: `CARGO_TARGET_DIR=/dev/shm/typst-crystalline-p1301-p5 RUSTFLAGS=-Awarnings CARGO_TERM_COLOR=never cargo test -p typst-core p1301 -- --test-threads=1`
- Início: `2026-09-03T21:27:49.752314356-03:00`.
- Fim: `2026-09-03T21:28:46.779023845-03:00`.
- Duração wall-clock derivada: `57.026709489 s`; Cargo reportou `56.81 s` de build e `0.13 s` do filtro.
- `HEAD`: `1f082370e59939de7b57992e137a9f74bfb6758f`.
- Estado: working tree não commitado.
- SHA-256 de `git status --short --untracked-files=all`: `74dbe74d07965ed3e8f4811d633358501c215380c86e0f7a65f8ec371179cb43`.
- SHA-256 de `git diff HEAD --stat`: `1b69907997fce6a62636e22d3c07172d97a5a72b26ef2aa8f548d7b158904b28`.
- `git diff HEAD --stat` nesse estado: `7 files changed, 636 insertions(+), 20 deletions(-)`:
  - `00_nucleo/prompts/compiler/eval.md` (`39` linhas no stat);
  - `00_nucleo/prompts/compiler/eval/bindings/field_access.md` (`72`);
  - `00_nucleo/prompts/compiler/eval/tests.md` (`90`);
  - `00_nucleo/prompts/compiler/stdlib/color.md` (`46`);
  - `01_core/src/compiler/eval/mod.rs` (`14`);
  - `01_core/src/compiler/eval/tests.rs` (`389`);
  - `01_core/src/compiler/stdlib/color.rs` (`6`).
- O status continha ainda artefactos diagnósticos untracked P1299/P1300/P1301 e os passos P1299/P1300; o hash de status acima fixa a lista completa sem alegar autoria de P5.

Uma primeira tentativa de compilação, entre `2026-09-03T21:25:37.630509039-03:00` e `2026-09-03T21:26:13.895945831-03:00`, encontrou apenas incompatibilidade test-only `Option<(u32,u32)>` versus `Option<(integer,usize)>` no novo helper. Ela não foi classificada como RED. A conversão test-only foi corrigida antes da medição aceita acima.

## Resultado RED

O comando aceito terminou com exit code `101` e partção:

- `8` testes executados;
- `2 passed`;
- `6 failed`;
- `0 ignored`;
- `0 measured`;
- `5427 filtered out`.

Controles verdes:

- `p1301_lookups_module_existentes_preservam_kind_publico`;
- `p1301_dictionary_missing_key_preserva_span_da_expressao_inteira`.

Falhas exatas, todas e somente nas expectativas novas de `Module`:

| Teste | Perfis | Message observada | Message esperada | Span observado | Span esperado |
|---|---|---|---|---|---|
| `p1301_std_hsl_usa_global_e_span_do_field_nos_quatro_perfis` | os 4 | `module 'std' does not contain field "hsl"` | `module `global` does not contain `hsl`` | `10..17` | `14..17` |
| `p1301_std_hsv_usa_global_e_span_do_field_nos_quatro_perfis` | os 4 | `module 'std' does not contain field "hsv"` | `module `global` does not contain `hsv`` | `10..17` | `14..17` |
| `p1301_std_linear_rgb_usa_global_e_span_do_field_nos_quatro_perfis` | os 4 | `module 'std' does not contain field "linear_rgb"` | `module `global` does not contain `linear_rgb`` | `10..24` | `14..24` |
| `p1301_calc_nope_exerce_categoria_module` | default | `module 'calc' does not contain field "nope"` | `module `calc` does not contain `nope`` | `10..19` | `15..19` |
| `p1301_sym_nope_exerce_categoria_module` | default | `module 'sym' does not contain field "nope"` | `module `sym` does not contain `nope`` | `10..18` | `14..18` |
| `p1301_color_map_nope_exerce_modulo_aninhado` | default | `module 'map' does not contain field "nope"` | `module `map` does not contain `nope`` | `10..24` | `20..24` |

Em todos os doze subcasos `std.*` e nos três controles categoriais, os hints permaneceram vazios e não houve diagnostics laterais; portanto não existiu falha adicional fora de message, span e line/column derivados do mesmo span. O padrão mata mensagem antiga, whole-span, nome `std`, whitelist dos três aliases e regra restrita a um único perfil, enquanto os dois controles verdes impedem quebra de lookup existente e generalização a dicionário.

## Formatação e veredito

`rustfmt --edition 2021 --check 01_core/src/compiler/eval/tests.rs` passou antes do rerun aceito. A própria compilação e execução filtrada constituem o check proporcional do consumer test-only; nenhum check fora do escopo foi executado.

**Veredito P5:** `P1301_RED_CONFIRMED`. O baseline pré-candidato falha exclusivamente nos novos observáveis categóricos de `Module`; todos os controles do filtro permanecem verdes. Este recibo não certifica implementação nem GREEN.
