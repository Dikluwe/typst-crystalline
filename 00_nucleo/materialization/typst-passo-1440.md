# Passo 1440 — Materializar `math.overbracket` com identidade e geometria de spreader

**Estado:** PROPOSTA L0 — AGUARDA CONFIRMAÇÃO HUMANA (ADR-0127)
**Data da medição:** 2026-09-18T06:33:18-03:00
**Commit medido:** `fbd4f07dabc0e526d61b0fe14b1abe673fcf5032`
**Árvore medida:** limpa (`git status --short` e `git diff HEAD --stat` sem saída)
**Regime:** `executed_without_isolation_attestation`

## 1. Próxima unidade nominal

O inventário residual do Passo 1422 ordena, depois de `math.overbrace`,
`math.overbracket`, `math.overline`, `math.overparen`, `math.overshell`,
`math.primes`, `math.root`, `math.round`, `math.stretch`, `math.underbrace`,
`math.underbracket`, `math.underparen` e `math.undershell`. P1439 fechou
`overbrace`; portanto a próxima rota nominal é `math.overbracket`. `math.abs`,
deliberadamente não retomada pela sequência recente, continua residual e não
é declarada fechada.

## 2. Medição antes da decisão

### 2.1 Fonte vanilla ratificada

No pin `586e1bd43`, `math/underover.rs` declara `OverbracketElem` com
`body: Content` obrigatório e posicional e `annotation: Option<Content>`
opcional e posicional. `math/ir/resolve.rs` resolve o body, cria o glifo
U+23B4 (`⎴`) com classe `Diacritic`, stretch horizontal de 100% e shortfall
zero, e o combina como acento superior com largura exata. O item resultante
herda a classe matemática do body. Quando há anotação, ela é adicionada como
script superior.

Hashes SHA-256 das fontes medidas:

| Fonte | SHA-256 |
|---|---|
| `lab/typst-original/crates/typst-library/src/math/underover.rs` | `49fe33135766b83c16d27f84c51bd8f867cd5f3f790b075242e8043661a7e298` |
| `lab/typst-original/crates/typst-library/src/math/ir/resolve.rs` | `115d775641509a755a1b7f4bd6da26cc8502a09e0e23e303d38d4a7cd76e0112` |

### 2.2 Observáveis públicos medidos no vanilla

Com `/usr/local/bin/typst` ratificado (`typst 0.15.1 (586e1bd4)`):

- `repr(math.overbracket)` produz `"overbracket"`;
- `repr(math.overbracket([x]))` produz `overbracket(body: [x])`;
- `repr(math.overbracket([x], [n]))` produz
  `overbracket(body: [x], annotation: [n])`;
- omissão e presença explícita diferem: `overbracket([x], none)` conserva
  `annotation: none` no `repr`;
- `none` e string convertem para conteúdo no body; nesting e spread canônico
  conservam identidade;
- a forma bare `$ overbracket(x) $` produz a mesma identidade;
- ausência do body, terceiro positional, `body:`, `annotation:` e named
  desconhecido falham; inteiro no body é rejeitado e inteiro como segundo
  positional produz `unexpected argument`.

### 2.3 Estado cristalino medido

A forma qualificada inexiste. A forma bare partilha um braço genérico que
silencia named e positional excedente, baixa diretamente para
`MathUnderover` e perde a identidade: os casos medidos aparecem como
`xover("⎴")` e `xover("⎴")over(n)`. A geometria física madura e o padrão
P1439 já existem em `layout/underover.rs`; o defeito é de contrato,
morfologia e fronteira de argumentos, não ausência dessa mecânica.

Hashes SHA-256 dos consumers antes da proposta:

| Consumer | SHA-256 |
|---|---|
| `01_core/src/entities/elements/math_underover.rs` | `aad48fc246eebf39ee57f2eb225edeb8d6025e1636442f1b040bac6e7d392599` |
| `01_core/src/entities/content.rs` | `f0c48afb31f1dbea27405c3c4a5645de8ffb473a9321a31f6dce50cdf1ff66ca` |
| `01_core/src/compiler/stdlib/structural/math.rs` | `074575e14911e391c692aac77a0d9e6b468ec391cb8c338cdba96c05a8c4f9e2` |
| `01_core/src/compiler/eval/math.rs` | `803e7b1fa1c7142eb0f89a1b1d1365bcee058db32d29fd38013f2b77696abb12` |
| `01_core/src/compiler/eval/repr.rs` | `de30619511362a10cf1a408afbb311c029b9618566eaa71719d23e4d53c19f7f` |
| `01_core/src/compiler/math/layout/underover.rs` | `4d79a9538ed42b03c8d55510dbc6d84e2622579378163b70b209b2217f86b942` |
| `01_core/src/compiler/math/layout/mod.rs` | `294cc82d85de3304e2a4f800fdf31eefc5936f8bf3bad56aa0f446821e39c3dc` |
| `01_core/src/compiler/math/layout/spacing.rs` | `314a2fce9c93a118d11ae0191ce79810e36bc24d6df512c7ef61b71ad7860865` |
| `01_core/src/compiler/eval/mod.rs` | `2357b67ff95028ba0bf70692539570e4caefbb5535955e025fc9090ce7e09a73` |
| `01_core/src/compiler/introspect.rs` | `77f42b1917bfc645c8a4d017b3ed808da9e90cf7b4b069c6aab5ec6694f564fc` |
| `01_core/src/compiler/introspect/locatable.rs` | `2d7f6ac7f23534dd3316948eb69efffb128cf31b816a2b63a159b566d1011b5c` |
| `01_core/src/compiler/layout/mod.rs` | `353562495b0de2698bde5100060303acdab0c3d6cc1937cc5e0ca70bb5ccfceb` |
| `03_infra/src/query_helpers.rs` | `6986396512345530ac1b06f46a7734559e40fecc98e25aebdb170d75133d1838` |

## 3. Classificação anti-deriva

- Identidade `overbracket`, assinatura/casts, presença explícita, `repr`,
  classe herdada e geometria observável são linguagem (ADR-0107).
- `AccentItem`, `ScriptsItem`, a árvore IR e a redução cristalina para
  `MathUnderover` são mecânica; podem divergir se os observáveis forem
  preservados.
- Inferência: shortfall zero + largura exata expressam que a barra horizontal
  deve cobrir exatamente o body antes da centralização da anotação. Refutação:
  witness vanilla em que a barra fique deliberadamente aquém do body ou a
  anotação determine sua largura-alvo.
- Reutilizar somente o carrier genérico não basta: ele não distingue a função
  pública nem omissão de `annotation` de `annotation: none`.

## 4. Decisão L0 proposta

1. Adicionar `MathOverbracketElem { body, annotation, annotation_explicit }`
   ao owner coeso `entities/elements/math_underover.rs`; não criar Prompt
   órfão. A identidade pública permanece distinta de `MathOverbraceElem`.
2. Adicionar `Content::MathOverbracket(Arc<MathOverbracketElem>)`. O bit de
   presença é morfológico e nunca é inferido do valor convertido.
3. Registrar `math.overbracket`; a forma bare delega ao mesmo builder. Exigir
   um body posicional e no máximo uma anotação posicional, reproduzindo casts,
   diagnósticos medidos, spread e avaliação única.
4. Projetar `overbracket(body: ..., annotation?: ...)`; emitir `annotation`
   somente quando fornecida e conservar explicitamente `none`.
5. O layout owner reduz localmente a variante à composição madura equivalente:
   body com U+23B4 acima e, se presente, anotação superior. A free function
   fica em `layout/underover.rs`; o match central permanece magro (ADR-0109).
6. A classe esquerda/direita herda recursivamente o body. A barra permanece
   diacrítica, estica horizontalmente com shortfall zero e não troca a classe.
7. Reconstruções transformam body e anotação, preservam presença e identidade.
   `ObservationEq` compara os três campos.
8. A variante é não-locatable e terminal nos walkers introspectivos e helpers
   aproximados L3; o walker estrutural L1 continua dono dos filhos.

As outras 11 rotas que sucedem `overbracket` no inventário e `math.abs`
permanecem fora de escopo.

## 5. Gate ADR-0127

Esta proposta cria elemento e variante públicos e corrige o comportamento por
defeito da rota bare. Portanto este commit contém apenas passo e L0 e **PARA**.
Testes, código e resselo começam somente após confirmação humana explícita.

## 6. Execução após confirmação

1. Criar oráculos RED para binding qualificado/bare, aridade, casts, presença,
   `repr`, nesting, classe herdada, geometria e walkers.
2. Confirmar RED cristalino e GREEN vanilla onde aplicável.
3. Implementar carrier, builders, dispatch e layout na forma B.
4. Atacar mutantes: apagar identidade/presença, aceitar named/extra, trocar
   U+23B4, inverter posição, perder classe herdada e deixar de esticar.
5. Exigir score `1.0` para os mutantes definidos; resselo somente após V15/V26.
6. Validar `cargo fmt --check`, `cargo build --workspace`, suítes focal/completa,
   `git diff --check` e `crystalline-lint .`, registrando proveniência.
