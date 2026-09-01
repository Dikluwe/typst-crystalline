# P1292 — amendment-1 de dependências L0 exaustivas

**Papel:** Autor do contrato + auditor de ownership
**Manifest autorizado:** `0a4894f3088ff5d124d346ce677292db24c2241ecce26d657880b33db752d931`
**Instante da medição:** `2026-08-31T23:34:16-03:00`
**HEAD:** `0eb39f8ecb48930515f2cadb6a378450855b5a72`
**Estado:** working tree não commitado; os paths exatos constam de
`git status --short --untracked-files=all` no recibo de implementação A e
foram rechecados nesta sequência antes da primeira escrita.

## Causa e bridge imutável

O preflight do lote B encontrou três consumers exaustivos cujos owners não
estavam no grafo L0 congelado: `compiler/math/layout/spacing.rs`,
`compiler/introspect.rs` e `infra/query_helpers.rs`. O manifest atual autoriza
somente seus três prompts, este diagnóstico e o resselo contratual.

O contrato predecessor permanece identificável por:

| Evidência anterior | SHA-256 |
|---|---|
| contrato canônico P1292 v1 | `18a987cedb0808cab4473a5f578b1cd336e1830595cb694cd9d31cfbd1fa0554` |
| arquivo do selo v1 | `7359b85cda7cdaf85f12eeabb2725ee04452676b2448ffebaa32b8a4a635edbe` |
| recibo pre-gate imutável | `c0c9ded039181be699ad387f1e89368fd56a72c976ba5d0859f528769ef91966` |
| lotes públicos A–D canônicos | `c2caa85f8258ef2bc6d983256029e0ac2c5690456415af568fede473cc9813bb` |

O amendment não muda nenhum vetor positivo, negativo ou limítrofe A–D, nem
seus resultados de aceitação. Ele fecha somente obrigações de consumers
exaustivos antes de retomar B/C/D.

## Medição anterior à decisão

### Cristalino

- `spacing.rs:29-175,184-221`: fallback de classe `Normal`; refinamentos de
  borda só para `MathDelimited`, wrappers/sequências e `MathMatrix`.
- `introspect.rs:568-596,1151-1174,1687-1713`: as três listas exaustivas
  (`materialize_time`, `classify_unreferencable_body`, `walk`) já tornam a
  família math materializada terminal, mas não podiam nomear B/C/D.
- `query_helpers.rs:274-346,349-424`: `has_any_text`/`count_variant` devolvem
  `false`/`0` para a família math terminal e só descem em contentores
  explicitamente listados.

### L0s P1292 já selados

- `entities/elements/math_underline.md` e `compiler/math/layout/underline.md`:
  identidade math própria, não-locatável, body único e propriedades
  matemáticas relevantes conservadas.
- `entities/elements/math_vec.md` e `compiler/math/layout/vec.md`: identidade
  própria, delimitadores opcionais e ausência representada sem glifo.
- `entities/elements/flush.md` e `compiler/layout/flush.md`: sentinela
  não-locatável, sem texto/item/cursor próprio, cujo único efeito é realizar o
  prefixo de floats no owner de layout.

### Vanilla ratificado `a51e02804`

| Fonte e linhas | SHA-256 | Medição |
|---|---|---|
| `typst-library/src/math/ir/item.rs:118-145,570-603,834-856` | `273dbf55cb714ad7e40aa13c4648293260eee8d2818001952fbf21f9a4180b3a` | `LineItem` herda `base.raw_class`; `FencedItem` expõe Opening/Closing somente na borda presente |
| `typst-library/src/math/ir/resolve.rs:198-211,1002-1025,1164-1189,1253-1261` | `115d775641509a755a1b7f4bd6da26cc8502a09e0e23e303d38d4a7cd76e0112` | underline resolve como line sobre body; vec resolve como fenced com delimitadores opcionais |
| `typst-library/src/math/underover.rs:4-14` | `49fe33135766b83c16d27f84c51bd8f867cd5f3f790b075242e8043661a7e298` | underline math é elemento `Mathy` body-only |
| `typst-library/src/math/matrix.rs:18-68` | `fb6ed72f7fac4bb5489170059b9aa0f919beb0b1a23bf4e862d4cff52905120f` | vec é elemento `Mathy` distinto com delimitadores opcionais |
| `typst-library/src/layout/place.rs:179-213` | `ec64092c3f09428aa5d5c2e43a35849cfa2e062947d8d662066a4cdfc3541579` | `FlushElem` é zero-field sob o namespace de place |
| `typst-layout/src/flow/collect.rs:79-91` | `0baa7e20f1b2e1bfb5d7901ce24f90073fd53648281f4b41c25b090d43318b11` | flush vira sentinela `Child::Flush`, não conteúdo visual |

## Matriz 1:1 e decisão

| Consumer produtivo | Prompt owner 1:1 | Obrigação acrescentada |
|---|---|---|
| `01_core/src/compiler/math/layout/spacing.rs` | `prompts/compiler/math/layout/spacing.md` | `MathUnderline` conserva `lclass/rclass` do body; `MathVec` expõe `Opening`/`Closing` por delimitador e `Normal` na borda ausente; `Flush` é `Normal/Normal` neutro |
| `01_core/src/compiler/introspect.rs` | `prompts/compiler/introspect.md` | materialize/classify/walk tratam B/C como terminais math e D como sentinela terminal; nenhum é locatável nem produz efeito de introspecção |
| `03_infra/src/query_helpers.rs` | `prompts/infra/query-helpers.md` | braços terminais de `has_any_text` e `count_variant` devolvem `false`/`0` para B/C/D, sem descer no payload |

`classify_unreferencable_body` usa `UnreferencableKind::Text` apenas como
morfologia da mensagem de erro de label não referenciável; isso não converte
B/C/D em texto. O pre-check de `count_variant` que conta a própria predicate
permanece; nenhum selector aproximado existente usa B/C/D como predicate.

Classificação ADR-0107/0108: classe de borda, ausência de Location/tag/efeito e
ausência de falso match de query são observáveis de linguagem; a organização
dos `match` é mecânica. A terminalidade de introspect/query e a neutralidade de
Flush no classificador são inferências a partir dos L0s P1292 e das medições
vanilla, não alegações de igualdade de IR.

Refutação explícita: reabrir o contrato se (a) underline perder a classe de um
body explicitamente classificado, (b) vec não expuser as bordas delimitadas ou
classificar uma borda ausente como delimiter, (c) inserir flush sem floats
mudar o espaçamento matemático, (d) B/C/D adquirirem identidade locatável ou
efeitos em counters/labels/query, ou (e) o helper aproximado passar a selecionar
texto interno de elementos math.

## Hashes L0

| Prompt | Antes | Depois |
|---|---|---|
| `compiler/math/layout/spacing.md` | `0ef99e014944e69b76d99ab4dfa959f1000d9cf91aed207e886ebb7abf859420` | `33f370600d7b22bf7c205e4844498e5d9851a855daa2b18d43d7a26c2f88cb39` |
| `compiler/introspect.md` | `b1dac49c5aa2c5b4998b4363401f5c88f355181521a89b1fdfd50bffac1abe2d` | `6d794502e69e1643c258889302e8adcfd35769193ad0b08acd3808cc972f794f` |
| `infra/query-helpers.md` | `42debc22810cf3eebf16185d91543cedc36be4e179221f5b6d15b8f2ff6c6705` | `ceb77f9accd160a90bae7f335d561beef5ddec19b1c43ba416f9a6b6114883a4` |

Os headers `Hash do Código` permanecem sem resselo: os consumers ainda não
implementam o amendment e `--fix-hashes` é proibido nesta sequência.

## Gates

Antes e depois da redação:

```text
crystalline-lint . --checks v15,v26 --fail-on warning
✓ No violations found
exit 0
```

`git diff --check` sobre os três L0s não produziu saída. Whitespace terminal é
validado novamente em conjunto com selo/recibo ao final.

**PARAGEM CONTRATUAL:** este papel não escreve código, testes, ataques ou
veredito. B/C/D só retomam após resselo e reconhecimento causal pelo papel
seguinte.
