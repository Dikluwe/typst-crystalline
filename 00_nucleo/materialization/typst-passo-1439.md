# Passo 1439 — Materializar `math.overbrace` com identidade e geometria de spreader

**Estado:** PROPOSTA L0 — AGUARDA CONFIRMAÇÃO HUMANA (ADR-0127)  
**Data da medição:** 2026-09-17T22:46:49-03:00  
**Commit medido:** `243fa2784675bd1eb3710cc4fb03b2b6f4e24b92`  
**Árvore medida:** limpa (`git status --short` e `git diff HEAD --stat` sem saída)  
**Regime:** `executed_without_isolation_attestation`

## 1. Próxima unidade nominal

O inventário residual do Passo 1422 ordena, depois de `math.norm`,
`math.overbrace`, `math.overbracket`, `math.overline`, `math.overparen`,
`math.overshell`, `math.primes`, `math.root`, `math.round`, `math.stretch`,
`math.underbrace`, `math.underbracket`, `math.underparen` e
`math.undershell`. Portanto a próxima rota nominal é `math.overbrace`.
`math.abs`, deliberadamente não retomada pela sequência dos passos recentes,
continua residual e não é declarada fechada.

## 2. Medição antes da decisão

### 2.1 Fonte vanilla ratificada

No pin `586e1bd43`, `math/underover.rs` declara `OverbraceElem` com
`body: Content` obrigatório e posicional e `annotation: Option<Content>`
opcional e posicional. `math/ir/resolve.rs` resolve o body, cria o glifo
U+23DE (`⏞`) com classe `Diacritic`, stretch horizontal de 100% e shortfall
zero, e o combina como acento superior com largura exata. O item resultante
herda a classe matemática do body. Quando há anotação, ela é adicionada como
script superior.

Hashes SHA-256 das fontes medidas:

| Fonte | SHA-256 |
|---|---|
| `lab/typst-original/crates/typst-library/src/math/underover.rs` | `49fe33135766b83c16d27f84c51bd8f867cd5f3f790b075242e8043661a7e298` |
| `lab/typst-original/crates/typst-library/src/math/ir/resolve.rs` | `115d775641509a755a1b7f4bd6da26cc8502a09e0e23e303d38d4a7cd76e0112` |

### 2.2 Observáveis públicos medidos no vanilla

Com `/usr/local/bin/typst` ratificado:

- `repr(math.overbrace)` produz `"overbrace"`;
- `repr(math.overbrace([x]))` produz `overbrace(body: [x])`;
- `repr(math.overbrace([x], [n]))` produz
  `overbrace(body: [x], annotation: [n])`;
- omissão e presença explícita diferem: `overbrace([x], none)` conserva
  `annotation: none` no `repr`;
- `none` e string convertem para conteúdo no body; nesting e spread canônico
  conservam identidade;
- a forma bare `$ overbrace(x) $` produz a mesma identidade;
- ausência do body, terceiro positional, `body:`, `annotation:` e named
  desconhecido falham; inteiro no body é rejeitado e inteiro como segundo
  positional produz o diagnóstico vanilla `unexpected argument`.

### 2.3 Estado cristalino medido

A forma qualificada inexiste. A forma bare partilha um braço genérico que
silencia named e positional excedente, baixa diretamente para
`MathUnderover` e perde a identidade: os casos medidos aparecem como
`xover("⏞")` e `xover("⏞")over(n)`. A geometria física madura já existe em
`layout/underover.rs`, inclusive variantes/assemblies horizontais, attachment,
largura exata e script da anotação; o defeito é de contrato, morfologia e
fronteira de argumentos, não ausência dessa mecânica.

Hashes SHA-256 dos consumers antes da proposta:

| Consumer | SHA-256 |
|---|---|
| `01_core/src/entities/elements/math_underover.rs` | `aab62f0cd57bb9b26fbf3e3bb1f8a47f55c48709a9e5ccec75168db399f99d87` |
| `01_core/src/entities/content.rs` | `d3dc465cf1fccb0d9bf684cb5d54943db2a9ce40b993a1697e50b450cce0c7c0` |
| `01_core/src/compiler/stdlib/structural/math.rs` | `e1bb64a8a01cba71c8db94a96f6e66d7aa5d630b2142a64a24eda243e565c8c8` |
| `01_core/src/compiler/eval/math.rs` | `40ff287322e6a6cb84e1aedde6105a38c7c780bde1786d97da88fc051203dfc3` |
| `01_core/src/compiler/eval/repr.rs` | `7cf7cd7ba4a5a3518b144729844f3cf0a1a114cd87ae7b0dc2f1858fc38928ac` |
| `01_core/src/compiler/math/layout/underover.rs` | `e6a42dae479791e681a6b76a96f797d594017191441a752f7b02fd02872f1f8a` |
| `01_core/src/compiler/math/layout/mod.rs` | `b0918b833751eeec2fdca73aabc4cb1c649af9a7c43b22e3bcbb42dc5c72c51f` |
| `01_core/src/compiler/math/layout/spacing.rs` | `fd64cf5721f04acea87e066c3ea0434b4c6c462cad2b070937d3f5d3b0820011` |
| `01_core/src/compiler/eval/mod.rs` | `024ef1e0cd4135cd5585f3bfc9fb4dba8a1bf4a2055193f6434e8fd6e6827565` |
| `01_core/src/compiler/introspect.rs` | `cc2f282b60f56a7e9c3262f0e30e79b148f5dcf39b9c7cc9ebcb48275ed22ede` |
| `01_core/src/compiler/introspect/locatable.rs` | `fc469853c3db3a5cacbfbe4baf949451dc1fb25f852f9d11290c6f62094a4a2b` |
| `01_core/src/compiler/layout/mod.rs` | `df76b306f99b85041bfa6e278f5544e62ef453121ada333b245be671ffd2b6f6` |
| `03_infra/src/query_helpers.rs` | `a2f393a64f7e2efba1aca767b86ef7d7d7b3e99f0658bb622994b45bc2f49548` |

## 3. Classificação anti-deriva

- Identidade `overbrace`, assinatura/casts, presença explícita, `repr`, classe
  herdada e geometria observável são linguagem (ADR-0107).
- `AccentItem`, `ScriptsItem`, a árvore IR e a redução cristalina para
  `MathUnderover` são mecânica; podem divergir desde que os observáveis sejam
  preservados.
- Inferência: shortfall zero + largura exata expressam que a chave deve cobrir
  exatamente o corpo antes da centralização da anotação. Refutação: witness
  vanilla em que a chave fique deliberadamente aquém do corpo ou a anotação
  determine a largura-alvo da chave.
- Reutilizar somente o carrier genérico não basta: ele não distingue a função
  pública nem omissão de `annotation` de `annotation: none`.

## 4. Decisão L0 proposta

1. Adicionar `MathOverbraceElem { body, annotation, annotation_explicit }` ao
   owner coeso `entities/elements/math_underover.rs`; não criar Prompt órfão.
2. Adicionar `Content::MathOverbrace(Arc<MathOverbraceElem>)`. O bit de
   presença é morfológico e nunca é inferido do valor convertido.
3. Registrar `math.overbrace`; a forma bare delega ao mesmo builder. Exigir um
   body posicional e no máximo uma anotação posicional, reproduzindo casts e
   diagnósticos medidos, spread e avaliação única.
4. Projetar `overbrace(body: ..., annotation?: ...)`; emitir `annotation`
   somente quando fornecida e conservar explicitamente `none`.
5. O layout owner reduz localmente a variante à composição madura equivalente:
   body com U+23DE acima e, se presente, anotação superior. A free function
   fica em `layout/underover.rs`; o match central permanece magro (ADR-0109).
6. A classe esquerda/direita herda recursivamente o body. A chave permanece
   diacrítica, estica horizontalmente com shortfall zero e não troca a classe.
7. Reconstruções transformam body e anotação, preservam presença e identidade.
   `ObservationEq` compara os três campos.
8. A variante é não-locatable e terminal nos walkers introspectivos e helpers
   aproximados L3; o walker estrutural L1 continua dono dos filhos.

As outras 12 rotas que sucedem `overbrace` no inventário e `math.abs`
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
   U+23DE, inverter posição, perder classe herdada e deixar de esticar.
5. Exigir score `1.0` para os mutantes definidos; resselo somente após V15/V26.
6. Validar `cargo fmt --check`, `cargo build --workspace`, suítes focal/completa,
   `git diff --check` e `crystalline-lint .`, registrando proveniência.
