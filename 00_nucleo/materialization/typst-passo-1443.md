# Passo 1443 — Materializar `math.overshell` com identidade e spreader superior

**Estado:** MATERIALIZADO E SELADO
**Data da medição:** 2026-09-18T11:16:14-03:00
**Commit medido:** `13f6a76a5a9392644920346f15043323f012805b`
**Árvore medida:** limpa (`git status --short` e `git diff HEAD --stat` sem saída)
**Regime:** `executed_without_isolation_attestation`

## 1. Próxima unidade nominal

O inventário residual do Passo 1422 ordena, depois de `math.overparen`,
`math.overshell`, `math.primes`, `math.root`, `math.round`, `math.stretch`,
`math.underbrace`, `math.underbracket`, `math.underparen` e
`math.undershell`. P1442 fechou `overparen`; portanto a próxima rota nominal
é `math.overshell`. `math.abs`, deliberadamente não retomada pela sequência
recente, continua residual e não é declarada fechada.

## 2. Medição antes da decisão

### 2.1 Fonte vanilla ratificada

No pin `586e1bd43`, `math/underover.rs` declara `OvershellElem` com
`body: Content` obrigatório e `annotation: Option<Content>` opcional e
posicional. `math/ir/resolve.rs:1398-1414` baixa a identidade para o spreader
U+23E0 (`⏠`) acima do body; `resolve_underoverspreader` marca o glifo como
`Diacritic`, aplica stretch horizontal de 100% com shortfall zero, conserva a
classe do body e, quando presente, adiciona annotation como script superior.

Hashes SHA-256 das fontes medidas:

| Fonte | SHA-256 |
|---|---|
| `lab/typst-original/crates/typst-library/src/math/underover.rs` | `49fe33135766b83c16d27f84c51bd8f867cd5f3f790b075242e8043661a7e298` |
| `lab/typst-original/crates/typst-library/src/math/ir/resolve.rs` | `115d775641509a755a1b7f4bd6da26cc8502a09e0e23e303d38d4a7cd76e0112` |

### 2.2 Observáveis públicos medidos no vanilla

Com o build ratificado `lab/typst-original/target/release/typst`:

- `repr(math.overshell)` produz `"overshell"`;
- uma chamada body-only produz `overshell(body: [x])`;
- uma annotation produz `overshell(body: [x], annotation: [n])`;
- omissão e presença explícita diferem: `overshell([x], none)` conserva
  `annotation: none` no `repr`;
- `none` e string convertem para conteúdo no body; spread canônico conserva
  identidade e campos;
- a forma bare `$ overshell(x, n) $` produz a mesma identidade e os mesmos
  campos da forma qualificada;
- ausência do body, terceiro positional, `body:`, `annotation:`, named
  desconhecido e inteiro no body são rejeitados; inteiro como segundo
  positional também é `unexpected argument`.

### 2.3 Estado cristalino medido

A forma qualificada inexiste. A forma bare produz `overshell(x)` e, com
annotation, `overshell(x","n)`: morfologia opaca que não preserva o elemento
público nem os seus campos. O carrier `MathUnderover` e o layout de spreaders
já realizam stretch, attachment, estilo de script e composição, mas não podem
provar a identidade `overshell` nem distinguir annotation omitida de
`annotation: none`.

Hashes SHA-256 dos consumers antes da proposta:

| Consumer | SHA-256 |
|---|---|
| `01_core/src/entities/elements/math_underover.rs` | `f6611daecff4eaa24c90f86960cdbe856f29e58560095f4a17a421c0d09542e9` |
| `01_core/src/entities/content.rs` | `057c3ac1298bdbf634c14f57cec72bd432f49f2cbac182a539f8deba6d2971e8` |
| `01_core/src/compiler/stdlib/structural/math.rs` | `638eb62a39c6e86cba5342723ea450d742faef2675acf6246ddbc81e305ab02f` |
| `01_core/src/compiler/eval/math.rs` | `be7b73bce116f69fcbb927f2f9a958857501ee428ef8e79e0954c72012eab4a3` |
| `01_core/src/compiler/eval/repr.rs` | `7ba64ffc39899a8c48a30446ce011d0dd5ec9eb341cc4724c9715296754b82b4` |
| `01_core/src/compiler/math/layout/underover.rs` | `f653fe053679faf8b88c0200e4b89aaca5dda011db0322b1d6152363c1f8b3e6` |
| `01_core/src/compiler/math/layout/mod.rs` | `fd4bb03ff2b4d85e04478cbc733de248c2bda1af1fc4561d259f9324a76d10aa` |
| `01_core/src/compiler/math/layout/spacing.rs` | `2c0332a68fc85433b6f2e9ef7be79f55badba4cf181ef55d82801a0034df6e2e` |
| `01_core/src/compiler/eval/mod.rs` | `3a8159f23597fd966ab5aa038a0776d4612e66e1616baed3eeb9b83963c9a5e7` |
| `01_core/src/compiler/introspect.rs` | `2b60d7dc5a06edfc9f12c859382c11a82555b11f75f4c247fe57a5c533608462` |
| `01_core/src/compiler/introspect/locatable.rs` | `cb69f1bff30ff0333f20f691d7ceb74ee15f9d82f4438e4c0e51cb6b233eb601` |
| `01_core/src/compiler/layout/mod.rs` | `ed0a79cabdf62ee60497ac117048fba245e4eabe8010c498ec34bc0bfde390fb` |
| `03_infra/src/query_helpers.rs` | `e29ac844362fc322a6c5b33712dc34daedadfb9477a5f98228af7a7b24f3d763` |

## 3. Classificação anti-deriva

- Identidade `overshell`, assinatura/casts, presença explícita, `repr`, classe
  herdada e geometria observável são linguagem (ADR-0107).
- `AccentItem`, `ScriptsItem`, a árvore IR e a redução para
  `MathUnderover` são mecânica; podem divergir se os observáveis forem
  preservados.
- Inferência: stretch de 100% e shortfall zero tornam U+23E0 um spreader que
  cobre exatamente a largura-alvo do body antes da composição da annotation.
  Refutação: witness vanilla em que o casco fique deliberadamente aquém do
  body ou a annotation redefina a largura-alvo do glifo.
- Reutilizar apenas o carrier genérico preservaria parte da geometria, mas
  continuaria apagando identidade, campos e presença — equivalência mecânica
  insuficiente para paridade de linguagem.

## 4. Decisão L0 proposta

1. Adicionar `MathOvershellElem { body, annotation, annotation_explicit }` ao
   owner coeso `entities/elements/math_underover.rs`; não criar Prompt órfão.
2. Adicionar `Content::MathOvershell(Arc<MathOvershellElem>)`. O bit de
   presença é morfológico e nunca é inferido do valor convertido.
3. Registrar `math.overshell`; a forma bare delega ao mesmo builder. Exigir
   body posicional e no máximo uma annotation posicional, reproduzindo casts,
   diagnostics, spread e avaliação única.
4. Projetar `overshell(body: ..., annotation?: ...)`; emitir annotation só
   quando fornecida e conservar explicitamente `none`.
5. O owner de layout reduz localmente a variante à composição madura: body
   com U+23E0 acima e, se presente, annotation superior. A free function fica
   em `layout/underover.rs`; o match central permanece magro (ADR-0109).
6. A classe esquerda/direita herda recursivamente o body. O spreader
   permanece diacrítico, estica com shortfall zero e não troca a classe.
7. Reconstruções transformam body e annotation, preservam presença e
   identidade. `ObservationEq` compara os três campos.
8. A variante é não-locatable e terminal nos walkers introspectivos e helpers
   aproximados L3; o walker estrutural L1 continua dono dos filhos.

As outras oito rotas que sucedem `overshell` no inventário e `math.abs`
permanecem fora de escopo.

## 5. Gate ADR-0127

Esta proposta cria elemento e variante públicos e corrige o comportamento por
defeito da rota bare. Portanto este commit contém apenas passo e L0 e **PARA**.
Testes, código e resselo começam somente após confirmação humana explícita.

## 6. Execução após confirmação

1. Criar oráculos RED para binding qualificado/bare, aridade, casts, presença,
   `repr`, nesting, classe herdada, lowering U+23E0 e walkers.
2. Confirmar RED cristalino e GREEN vanilla onde aplicável.
3. Implementar carrier, builders, dispatch e layout na forma B.
4. Atacar mutantes: apagar identidade/presença, aceitar named/extra, trocar
   U+23E0, inverter posição, perder classe e deixar annotation substituir o
   spreader.
5. Exigir score `1.0`; resselo somente após V15/V26.
6. Validar `cargo fmt --check`, `cargo build --workspace`, suítes
   focal/completa, `git diff --check` e `crystalline-lint .`, registrando
   proveniência.

## 7. Confirmação humana

O dono confirmou explicitamente o gate ADR-0127 em 2026-09-18 após o commit
de proposta `d616cace141bf64f02137ed002dadce019aff0fa`. A cadeia RED→GREEN,
ataques, resselo e veredito está autorizada sem ampliar o escopo acima.

## 8. Materialização e veredito

Medição final em `2026-09-18T11:43:59-03:00`, sobre working tree não
commitado derivado do commit de proposta
`d616cace141bf64f02137ed002dadce019aff0fa`; o estado exato está descrito
por `git diff HEAD --stat` e pelos artefatos `p1443-*` deste passo.

- RED atribuível: nove erros de compilação pela ausência do carrier, variant
  e helpers congelados.
- GREEN focal: 4 passados, 0 falhados.
- Ataques: 12 famílias em ordens direta, repetida e inversa; 36/36 mortos,
  score 1,0 e zero `Unknown`.
- Paridade pública positiva: função qualificada, casts, presença causal,
  `repr`, nesting, forma bare pelo seu body, classe herdada e lowering U+23E0
  preservados.
- Nos sete negativos, status e mensagem primária coincidem com o vanilla. A
  ancoragem de span e o hint de `body:` continuam na limitação diagnóstica
  global do carrier simplificado e não recebem crédito de equivalência
  integral neste passo.
- Gates: `cargo fmt --check`, `git diff --check`, `cargo build --workspace`
  e `crystalline-lint .` passaram; V5/V15/V26 = 0.
- Suíte integral: 5875 passados e os mesmos 8 falhados históricos, sem falha
  atribuível a P1443.

Veredito limitado: `Preserved` para o fragmento `math.overshell` descrito;
não declara equivalência Typst geral, paridade integral de apresentação dos
diagnósticos nem fecha as rotas residuais de P1422.
