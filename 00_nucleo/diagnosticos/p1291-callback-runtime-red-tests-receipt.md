# Recibo RED independente — P1291.cancel-angle-runtime

## Papel, escopo e isolamento

- Papel: Testador A, autor dos testes antes da implementação candidata.
- Regime: testes independentes derivados somente dos Prompts L0 protegidos e do selo humano `p1291-callback-runtime-seal.json`; ficheiros de testes preexistentes foram lidos somente para convenções de harness.
- Estado de isolamento: `executed_without_technical_isolation_attestation`. O turno partilha thread/filesystem com os demais papéis, portanto este recibo não alega isolamento técnico integral.
- Escrita limitada a:
  - `01_core/src/compiler/math/layout/tests.rs`;
  - `04_wiring/tests/p1291_callback_runtime.rs`;
  - este recibo.
- Nenhum código produtivo, Prompt L0, Núcleo, manifesto ou selo foi alterado pelo Testador A.

## Proveniência da execução

- HEAD: `53d21c5a602f4045a769a0ab0c935baa5ecd3b88`.
- Instante do snapshot de estado: `2026-08-31T16:08:50-03:00`.
- Working tree: não commitada e partilhada; `git status --short` continha 344 entradas nesse instante.
- SHA-256 da saída literal de `git status --short`: `7e7a9d0c2c9525f786963eb3990a0b6971ab7a4525ac34239a0260e65d52d62a`.
- Estado dos ficheiros de teste no snapshot: `M 01_core/src/compiler/math/layout/tests.rs` e `?? 04_wiring/tests/p1291_callback_runtime.rs`. As demais entradas pertenciam à árvore partilhada e não foram assumidas nem revertidas.

## Revalidação do selo

Uma leitura inicial defasada apresentou para `compiler/layout.md` um hash com prefixo `599…`. Antes de escrever os testes, a leitura foi repetida diretamente no filesystem real com o path absoluto:

```text
$ sha256sum /repos/Antigravity/typst-crystalline/00_nucleo/prompts/compiler/layout.md
f4bd917d3c8256392359a44ba462f48c84ad2a976acebabbad9aa57a36ac865c  /repos/Antigravity/typst-crystalline/00_nucleo/prompts/compiler/layout.md
```

O `mtime` observado foi `2026-08-31 15:28:47.695534839 -0300`, anterior ao selo. A revalidação dos dez inputs protegidos coincidiu integralmente com o selo:

| Input protegido | SHA-256 observado |
|---|---|
| `compiler/math/layout/callbacks.md` | `a305478928d785027e521cc9f2d7dbcedf6895fd8207d3501b595a97d32601dd` |
| `compiler/math/layout/_comum.md` | `3b5eb1149957ed04f95b289d0a1f526ab7da491e160c80fce943c2906390a99e` |
| `compiler/layout.md` | `f4bd917d3c8256392359a44ba462f48c84ad2a976acebabbad9aa57a36ac865c` |
| `compiler/layout/equation.md` | `d38a2ef130e4b878a33b841aaa15ba38c93ac0456b8d22689557dd46eca749ec` |
| `compiler/math/layout/cancel.md` | `7f6820bbe2de268ab31b7fec4c855a6766e9ce02b9087c163d3e405362a7b969` |
| `compiler/math/layout/vec.md` | `17e0b016019812bebe4fc5fe902051fc2d808174557f32571f6620457ffa32ea` |
| `entities/elements/math_cancel.md` | `97b23542c82daa0d144189d26a5bbc43be26605834609a18046318eaee263334` |
| `entities/content.md` | `e766b0e65403438ac22d204ecb51ab9163ab6ee262c93e1890755b9a7f7c79f5` |
| `compiler/stdlib/structural/math.md` | `94ae011a5ded72ffec4b8468f7b496cdb6a33079eaade67241230b6e79a2812f` |
| `infra/pipeline.md` | `28fb5f18d4f6d454de32cb6120c0ac2919d802a5d1aa85261b466872d632df3b` |

SHA-256 do selo consumido: `59650dfc24435e73892ee1d4346b66438349a0b3c679bc63130eb2de87082179`.

## Cobertura autorada

| Obrigação | Teste independente |
|---|---|
| `Pending` não transporta `PagedDocument` | `p1291_pending_nao_transporta_paged_document` |
| identidade por equation + occurrence + line e reserva pre-order | `p1291_identity_preorder_e_cross_duas_chamadas` |
| store parcial ou stale não pode completar | `p1291_store_parcial_ou_stale_nunca_completa` |
| `cross` realiza duas chamadas distinguíveis por line | `p1291_identity_preorder_e_cross_duas_chamadas` |
| preservação lexical e `TextStyle` efetivo | `p1291_request_preserva_chain_lexical_e_style_math_efetivo` e `p1291_callback_runtime_observa_style_efetivo` |
| erro preserva o span da chamada e não exporta documento | `p1291_callback_runtime_erro_preserva_span_da_chamada` |
| callback runtime altera o ângulo final | `p1291_callback_runtime_altera_angulo_final` |
| gap percentual de `vec` contra região efetiva | `Unknown`: não foi criado teste. Os inputs protegidos não expõem um construtor/harness tipo-seguro suficiente para isolar esse cálculo sem inventar API, e um oráculo black-box de região exigiria baseline adicional não selado. Não recebe crédito GREEN. |

## Evidência RED válida

Teste black-box realmente selecionado:

```text
$ cargo test -p typst-wiring --test p1291_callback_runtime p1291_callback_runtime_altera_angulo_final -- --exact --nocapture
running 1 test
test p1291_callback_runtime_altera_angulo_final ... FAILED

error: cancel(): argumento nomeado 'angle' não suportado em P296 (length/inverted/cross/angle/stroke scope-out per ADR-0054 graded)

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 2 filtered out
exit status: 101
```

Esse RED é comportamental: o caminho público ainda rejeita justamente o argumento `angle` necessário à callback runtime.

Teste focal de contrato selecionado:

```text
$ cargo test -p typst-core p1291_pending_nao_transporta_paged_document -- --nocapture
error[E0432]: unresolved import `crate::entities::elements::math_cancel::MathCancelAngle`
error[E0432/E0433]: could not find `callbacks` in `super`
exit status: 101
```

Esse segundo RED confirma que os tipos de contrato selados ainda não existem na candidata; ele não substitui o RED comportamental acima.

## Integridade dos testes protegidos

- `01_core/src/compiler/math/layout/tests.rs`: `b5c8098348a7ee7c50b3757f72655ed9c5aca7d3050fb844462aa2f47ac3f3be`.
- `04_wiring/tests/p1291_callback_runtime.rs`: `9ab09f8173a3d3444dfe07ac5d8b3742552bd4d64b72cc2f8afa9a08f4993735`.
- `git diff --check -- 01_core/src/compiler/math/layout/tests.rs 04_wiring/tests/p1291_callback_runtime.rs`: sucesso, sem saída.

## Veredito desta fase

**RED estabelecido.** A implementação deve ser feita por outro papel; o Testador A parou sem corrigir produção e sem converter os três casos gated em alegações GREEN.
