# P1291 — recibo de saneamento dos testes históricos de nesting

## Escopo e isolamento

- Regime: Tekt A/B, sublote de correção de oráculos históricos.
- Executor: `/root/testador_a`.
- Escrita autorizada: somente os testes
  `p311b5_bb_cal_x_outer_wins` e
  `p311b5_upright_italic_x_outer_wins` em
  `01_core/src/compiler/math/layout/tests.rs`, mais este recibo.
- Produção, L0, headers e testes P1291 protegidos preexistentes não foram
  editados neste sublote.
- A afirmação fica restrita à morfologia dos dois nestings observados; não
  constitui equivalência funcional geral do layout matemático.

## Entradas e proveniência

- Instante posterior aos testes: `2026-08-31T11:52:55-03:00`.
- HEAD: `53d21c5a602f4045a769a0ab0c935baa5ecd3b88`.
- Árvore: não commitada; alterações alheias foram preservadas.
- Manifesto P1291: SHA-256
  `7ca6445f85cc697039849a6869a4fac62f9da0ca96c42dda56a52de8894d59e2`.
- L0 `compiler/stdlib/math_style.md`: SHA-256
  `fd33cd0c921eec632e122aa475aa28d845a1e6cb4345458bfe68a5e78c53390f`.
- L0 `compiler/math/layout/_comum.md`: SHA-256
  `a4760755ea54621fd5c44c24804665f31d821b1658d73fa544e25240ddc3dbee`.
- Testes após o saneamento, `compiler/math/layout/tests.rs`: SHA-256
  `8dd396a5a7a26e09e2bb09fb2d9c3d5178472f4d953503feb7bee6e3b61804c8`.

## Contradição removida

Os nomes, comentários e expectativas outer-wins foram substituídos por:

- `p311b5_bb_cal_x_inner_wins`: `bb(cal(x))` contém `𝓍` U+1D4CD e não
  contém `𝕩` U+1D569;
- `p311b5_upright_italic_x_inner_wins`: `upright(italic(x))` contém `𝑥`
  U+1D465 e não contém `x` upright.

Isto corresponde à medição P1291 e aos dois L0s vigentes: o setter mais
interno vence quando dois wrappers escrevem o mesmo eixo.

## RED anterior

O coordenador reproduziu o RED antes deste saneamento, já com a implementação
inner-wins integrada, usando os filtros históricos:

```text
cargo test -p typst-core p311b5_bb_cal_x_outer_wins -- --nocapture
cargo test -p typst-core p311b5_upright_italic_x_outer_wins -- --nocapture
```

Cada filtro executou o teste antigo e falhou porque o código produzia a
morfologia vanilla inner-wins enquanto a expectativa ainda exigia outer-wins:
Cal `𝓍` em vez de DoubleStruck `𝕩`, e italic `𝑥` em vez de `x` upright.
Este RED é evidência recebida do coordenador, não alegação de execução
independente pelo autor deste recibo.

## GREEN posterior

```text
cargo test -p typst-core p311b5_bb_cal_x_inner_wins -- --nocapture
```

Resultado: `exit 0`; `1 passed`, `0 failed`.

```text
cargo test -p typst-core p311b5_upright_italic_x_inner_wins -- --nocapture
```

Resultado: `exit 0`; `1 passed`, `0 failed`.

```text
cargo test -p typst-core p311b -- --nocapture
```

Resultado: `exit 0`; `31 passed`, `0 failed`. O filtro inclui os dois testes
históricos corrigidos, os guardas P1291 já protegidos e os testes P311b de
stdlib/layout.

`git diff --check -- 01_core/src/compiler/math/layout/tests.rs` terminou sem
saída (`exit 0`).

## Veredito

GREEN para este fragmento observável: os dois oráculos históricos agora
aceitam inner-wins e rejeitam a morfologia outer-wins antiga. Execução
segregada no papel de saneamento de testes; o RED anterior é atestado pelo
coordenador e explicitamente distinguido das execuções GREEN deste agente.
