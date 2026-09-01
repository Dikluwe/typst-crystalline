# P1291 — recibo de implementação do runtime de callback de `math.cancel`

## Papel e fronteira

- Papel: Implementador B de `P1291.cancel-angle-runtime`.
- Regime: implementação após contrato e testes RED protegidos.
- Este recibo **não** é veredito final, não executa os gates finais e não alega isolamento técnico entre autores.
- Estado medido: working tree não commitado, sobre `HEAD 53d21c5a602f4045a769a0ab0c935baa5ecd3b88`, em `2026-08-31T16:54:49-03:00`.
- A árvore era compartilhada e continha alterações alheias. A implementação preservou-as e restringiu sua escrita aos consumers autorizados abaixo e a este recibo.

## Autoridade selada

- Selo: `00_nucleo/diagnosticos/p1291-callback-runtime-seal.json`, SHA-256 `59650dfc24435e73892ee1d4346b66438349a0b3c679bc63130eb2de87082179`.
- Adendo de eval de argumentos math: `00_nucleo/diagnosticos/p1291-callback-runtime-seal-amendment.json`, SHA-256 `67315b04bdb89e46ac975a7320d9816faa1d3e7edc073a8f50b94d37745c4ba2`.
- Prompt acrescentado pelo primeiro adendo: `00_nucleo/prompts/compiler/eval/math.md`, SHA-256 confirmado `4f6bdd8034aea86f8f11f748597eafd60102bdbf94223e339c42d14c1c75ad55`.
- Adendo da tabela fechada de field access: `00_nucleo/diagnosticos/p1291-callback-runtime-seal-amendment-2.json`, SHA-256 `5d413b18451375745dfa51c14443b9455cc2fae32f34ec6e15dec6cf723a2067`.
- Prompt acrescentado pelo segundo adendo: `00_nucleo/prompts/compiler/eval/bindings/field_access.md`, SHA-256 confirmado `9fa9cbb895be844016d16a93f08cc47d868720a4509fc3677a97ba7358ef987f`.

## Testes protegidos

| Arquivo | SHA-256 antes, selado | SHA-256 depois |
|---|---|---|
| `01_core/src/compiler/math/layout/tests.rs` | `b5c8098348a7ee7c50b3757f72655ed9c5aca7d3050fb844462aa2f47ac3f3be` | `b5c8098348a7ee7c50b3757f72655ed9c5aca7d3050fb844462aa2f47ac3f3be` |
| `04_wiring/tests/p1291_callback_runtime.rs` | `9ab09f8173a3d3444dfe07ac5d8b3742552bd4d64b72cc2f8afa9a08f4993735` | `9ab09f8173a3d3444dfe07ac5d8b3742552bd4d64b72cc2f8afa9a08f4993735` |

Não foi executado `--fix-hashes` e nenhum teste protegido foi editado.

## Implementação materializada

- `01_core/src/compiler/math/layout/callbacks.rs`: tipos puros de request/resolution, identidade preorder, transcript, store integral/stale, captura de styles/span e passagem `Pending`/`Complete` sem documento provisório.
- `01_core/src/compiler/math/layout/mod.rs`: integração do estado de callback e reserva preorder da ocorrência antes da descida no body.
- `01_core/src/compiler/math/layout/cancel.rs`: realização de `auto`, ângulo explícito e callback, `inverted`, `cross`, background e stroke no layout final.
- `01_core/src/compiler/layout/mod.rs` e `01_core/src/compiler/layout/equation.rs`: propagação do pass state até o layout matemático.
- `01_core/src/entities/elements/math_cancel.rs`, `01_core/src/entities/elements/mod.rs` e `01_core/src/entities/content.rs`: transporte de `MathCancelAngle::Func`, argumentos públicos necessários, span e preservação em mapeamentos.
- `01_core/src/compiler/stdlib/structural/math.rs`: parsing e validação dos argumentos públicos de `math.cancel`.
- `03_infra/src/pipeline.rs`: realização L3 entre tentativas e antes de P1159, reaplicada em relayouts, sem exportar um documento provisório.
- `01_core/src/compiler/eval/math.rs`: classificação L0-first de closure/contextual como valor de código em argumento math, sem executar o callback nessa fase.
- `01_core/src/compiler/eval/bindings/field_access.rs`: extensão exata do braço P792 `Value::Func("text")` para `text.size`, lendo `engine.styles.custom("text.size")` como `Value::Length` e usando `Length::pt(11.0)` por defeito; `lang` e demais campos permaneceram inalterados.

Não foi introduzido wrapper ou namespace em `callbacks.rs`; a alternativa de clonar/reescrever `Func` foi rejeitada conforme o segundo adendo.

## Trajetória RED → GREEN

1. O primeiro build focal do core atingiu os testes protegidos e falhou com 22 erros de API ausente, confirmando o RED do runtime.
2. Após materializar o runtime, o filtro P1291 do core ficou GREEN.
3. O primeiro black-box expôs que closure/contextual chegavam como conteúdo/identificador matemático; o primeiro adendo abriu somente `compiler/eval/math.rs`.
4. Após essa correção, dois dos três black-box passaram e `p1291_callback_runtime_observa_style_efetivo` falhou literalmente com `cannot access fields on type function`.
5. O fluxo foi localizado no braço P792 de field access; após o segundo adendo e a extensão fechada para `text.size`, os três black-box ficaram GREEN.

## Evidência final reproduzível

Comandos executados no estado não commitado identificado acima:

```text
cargo test -p typst-core p1291_ -- --nocapture
test result: ok. 18 passed; 0 failed; 0 ignored; 0 measured; 5329 filtered out
```

```text
cargo test -p typst-wiring --test p1291_callback_runtime -- --nocapture
p1291_callback_runtime_erro_preserva_span_da_chamada ... ok
p1291_callback_runtime_observa_style_efetivo ... ok
p1291_callback_runtime_altera_angulo_final ... ok
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

```text
git diff --check
exit 0; sem saída
```

Os builds emitiram warnings preexistentes/não fatais; nenhum alterou os resultados acima.

## Limitações e escopo excluído

- O gap de `vec` permanece **Unknown**, não foi implementado e não recebe crédito.
- Não houve ampliação para `underline` nem invenção de oráculo.
- Não foram executados os gates/vereditos finais da materialização segregada.
- A execução ocorreu em thread e filesystem compartilhados, portanto permanece a limitação selada `executed_without_technical_isolation_attestation`.

## Resultado do implementador

Testes focais do runtime de callback de `math.cancel`: **GREEN**. O veredito independente permanece reservado ao verificador/adversário.
