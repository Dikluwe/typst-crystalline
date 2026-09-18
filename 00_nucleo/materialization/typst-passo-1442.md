# Passo 1442 — Materializar `math.overparen` com identidade e spreader superior

**Estado:** PROPOSTA L0 — AGUARDA CONFIRMAÇÃO HUMANA (ADR-0127)
**Data da medição:** 2026-09-18T10:26:30-03:00
**Commit medido:** `2eb37dc53d32cb9e73f942cd63a02cfc7af2bd66`
**Árvore medida:** limpa (`git status --short` e `git diff HEAD --stat` sem saída)
**Regime:** `executed_without_isolation_attestation`

## 1. Próxima unidade nominal

O inventário residual do Passo 1422 ordena, depois de `math.overline`,
`math.overparen`, `math.overshell`, `math.primes`, `math.root`, `math.round`,
`math.stretch`, `math.underbrace`, `math.underbracket`, `math.underparen` e
`math.undershell`. P1441 fechou `overline`; portanto a próxima rota nominal é
`math.overparen`. `math.abs`, deliberadamente não retomada pela sequência
recente, continua residual e não é declarada fechada.

## 2. Medição antes da decisão

### 2.1 Fonte vanilla ratificada

No pin `586e1bd43`, `math/underover.rs` declara `OverparenElem` com
`body: Content` obrigatório e `annotation: Option<Content>` opcional e
posicional. `math/ir/resolve.rs:1364-1377` baixa a identidade para o spreader
U+23DC (`⏜`) acima do body; `resolve_underoverspreader` marca o glifo como
`Diacritic`, aplica stretch horizontal de 100% com shortfall zero, conserva a
classe do body e, quando presente, adiciona annotation como script superior.

Hashes SHA-256 das fontes medidas:

| Fonte | SHA-256 |
|---|---|
| `lab/typst-original/crates/typst-library/src/math/underover.rs` | `49fe33135766b83c16d27f84c51bd8f867cd5f3f790b075242e8043661a7e298` |
| `lab/typst-original/crates/typst-library/src/math/ir/resolve.rs` | `115d775641509a755a1b7f4bd6da26cc8502a09e0e23e303d38d4a7cd76e0112` |

### 2.2 Observáveis públicos medidos no vanilla

Com `/usr/local/bin/typst` ratificado (`typst 0.15.1 (586e1bd4)`):

- `repr(math.overparen)` produz `"overparen"`;
- uma chamada body-only produz `overparen(body: [x])`;
- uma annotation produz `overparen(body: [x], annotation: [n])`;
- omissão e presença explícita diferem: `overparen([x], none)` conserva
  `annotation: none` no `repr`;
- `none` e string convertem para conteúdo no body; spread canônico e nesting
  conservam identidade;
- a forma bare `$ overparen(x, n) $` produz a mesma identidade e os mesmos
  campos da forma qualificada;
- ausência do body, terceiro positional, `body:`, `annotation:`, named
  desconhecido e inteiro no body são rejeitados; inteiro como segundo
  positional também é `unexpected argument`.

### 2.3 Estado cristalino medido

A forma qualificada inexiste. A forma bare produz `overparen(x)` e, com
annotation, `overparen(x","n)`: uma morfologia opaca que não preserva o
elemento público nem os seus campos. O carrier `MathUnderover` e o layout de
spreaders já realizam stretch, attachment, estilo de script e composição,
mas não podem provar a identidade `overparen` nem distinguir annotation
omitida de `annotation: none`.

Hashes SHA-256 dos consumers antes da proposta:

| Consumer | SHA-256 |
|---|---|
| `01_core/src/entities/elements/math_underover.rs` | `0e8df38be37deca0830083203eb1ef6695198d4f1f2755fad78cbd541e4abd80` |
| `01_core/src/entities/content.rs` | `ac61c09dd4b9267bb8bb0b9060457c41f5f95f1ad40d3f62bac4747a65cee17c` |
| `01_core/src/compiler/stdlib/structural/math.rs` | `1fcaace2118b429f96984bc1f3e92e10ca4839f624280871c57c208cab222c87` |
| `01_core/src/compiler/eval/math.rs` | `d54bc969aae1cc87074ba9752f8d957b5ae30dbcd44d3164d9ee26118d3a983f` |
| `01_core/src/compiler/eval/repr.rs` | `1b099896180e3b236695e7b2e384c4a3a9f2e6aea1f4e7ecf37a73c27677556f` |
| `01_core/src/compiler/math/layout/underover.rs` | `4f6c670428f88b0f7727bb024ac563772292751a86fc1ff228d4962c180d91ca` |
| `01_core/src/compiler/math/layout/mod.rs` | `e15db218d73e099e0573116dc36118e69343b78a98e5071b3166750b10bd6647` |
| `01_core/src/compiler/math/layout/spacing.rs` | `9b217514c4054310fa9b336ddbbae1a78dfc292efb424f4fe2d652acc4794c88` |
| `01_core/src/compiler/eval/mod.rs` | `40a217608625a3a99c1753b8a9ef2289f5f1489a38188c13bf36448df202935c` |
| `01_core/src/compiler/introspect.rs` | `dbbf03eaa1f3719e48ae708eee7de618b8efc99ca3b99ce9c38fb6e286edcb97` |
| `01_core/src/compiler/introspect/locatable.rs` | `feb09c4c84a41829545bd2b771d8fe93edceaf79b051bf5dca80aaa2772fc44d` |
| `01_core/src/compiler/layout/mod.rs` | `0af4825fbba692cbb3d0e252bacd10da04eb06b60d411174386fafbe08e6cbd0` |
| `03_infra/src/query_helpers.rs` | `b8c086f393bee8412027a28d2080baca9c52bb13663255d89c71a47d3622e92d` |

## 3. Classificação anti-deriva

- Identidade `overparen`, assinatura/casts, presença explícita, `repr`, classe
  herdada e geometria observável são linguagem (ADR-0107).
- `AccentItem`, `ScriptsItem`, a árvore IR e a redução para
  `MathUnderover` são mecânica; podem divergir se os observáveis forem
  preservados.
- Inferência: stretch de 100% e shortfall zero tornam U+23DC um spreader que
  cobre exatamente a largura-alvo do body antes da composição da annotation.
  Refutação: witness vanilla em que o parêntesis fique deliberadamente aquém
  do body ou a annotation redefina a largura-alvo do glifo.
- Reutilizar apenas o carrier genérico preservaria parte da geometria, mas
  continuaria apagando identidade, campos e presença — equivalência mecânica
  insuficiente para paridade de linguagem.

## 4. Decisão L0 proposta

1. Adicionar `MathOverparenElem { body, annotation, annotation_explicit }` ao
   owner coeso `entities/elements/math_underover.rs`; não criar Prompt órfão.
2. Adicionar `Content::MathOverparen(Arc<MathOverparenElem>)`. O bit de
   presença é morfológico e nunca é inferido do valor convertido.
3. Registrar `math.overparen`; a forma bare delega ao mesmo builder. Exigir
   body posicional e no máximo uma annotation posicional, reproduzindo casts,
   diagnostics, spread e avaliação única.
4. Projetar `overparen(body: ..., annotation?: ...)`; emitir annotation só
   quando fornecida e conservar explicitamente `none`.
5. O owner de layout reduz localmente a variante à composição madura: body
   com U+23DC acima e, se presente, annotation superior. A free function fica
   em `layout/underover.rs`; o match central permanece magro (ADR-0109).
6. A classe esquerda/direita herda recursivamente o body. O spreader
   permanece diacrítico, estica com shortfall zero e não troca a classe.
7. Reconstruções transformam body e annotation, preservam presença e
   identidade. `ObservationEq` compara os três campos.
8. A variante é não-locatable e terminal nos walkers introspectivos e helpers
   aproximados L3; o walker estrutural L1 continua dono dos filhos.

As outras nove rotas que sucedem `overparen` no inventário e `math.abs`
permanecem fora de escopo.

## 5. Gate ADR-0127

Esta proposta cria elemento e variante públicos e corrige o comportamento por
defeito da rota bare. Portanto este commit contém apenas passo e L0 e **PARA**.
Testes, código e resselo começam somente após confirmação humana explícita.

## 6. Execução após confirmação

1. Criar oráculos RED para binding qualificado/bare, aridade, casts, presença,
   `repr`, nesting, classe herdada, lowering U+23DC e walkers.
2. Confirmar RED cristalino e GREEN vanilla onde aplicável.
3. Implementar carrier, builders, dispatch e layout na forma B.
4. Atacar mutantes: apagar identidade/presença, aceitar named/extra, trocar
   U+23DC, inverter posição, perder stretch/classe e deixar annotation alterar
   a largura-alvo.
5. Exigir score `1.0` para os mutantes definidos; resselo somente após
   V15/V26.
6. Validar `cargo fmt --check`, `cargo build --workspace`, suítes
   focal/completa, `git diff --check` e `crystalline-lint .`, registrando
   proveniência.

