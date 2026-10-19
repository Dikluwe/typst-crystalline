# Relatório P320 — Lote 5 (quebras/espaços + grid/table header/footer) + carona

**Pré-condição**: Lote 4 (P319) fechado — lint 0, suíte verde. ✅ Verificado.
**Dois commits isoláveis**: `Passo 320 — carona de registro` (1º, tree limpo) e
`Passo 320 — lote 5`.

---

## Carona de registro (commit próprio)

- **Correção do preditor — arms `|`-combinados (achado P319)**, adicionada à
  seção de previsão de custo do modelo: variante que partilha um braço de
  `match` **com binding** com variantes não migradas custa mais que a largura
  sugere (o `Arc` tipado força separar o braço); inspecionar os `|` que
  envolvem o LOTE antes de estimar (greps registados). **Casos sem binding não
  custam extra.**
- **Contabilidade**: registo do **Lote 5 como união dos propostos 5+6** (9
  variantes), no estilo do registo de união P314→P315.

---

## Lote 5 — quebras/espaços + grid/table header/footer (união 5+6)

**Composição (confirmada): 9 variantes** element-shaped, ordem por largura
crescente: `GridFooter`(7) · `GridHeader`(7) · `TableFooter`(10) ·
`TableHeader`(11) · `Linebreak`(11) · `Colbreak`(12) · `VSpace`(14) ·
`HSpace`(18) · `Pagebreak`(22) = **112 sites**.

**Locatabilidade**: confirmado **nenhuma é locatável**. **Fora do lote** (como
mandado): `Space` (triagem DEBT-58) e o bloco `TableCell`/`Table`/`GridCell`/
`Grid` (lote próprio, ~193 sites — registado na Contabilidade).

**Três formas na mesma família:**

- **Contentores (body + `repeat`)** — Grid/Table Header/Footer: `map_*`
  **recursam** no body, preservam `repeat`; **`is_empty` delega ao body**.
- **Espaços (`amount: Length`, `weak`)** — `HSpace`/`VSpace`: terminais em
  `map_*`; **`is_empty` = `amount.is_zero()`**; **`Hash` manual via Debug**
  (`Length`/`f64`, precedente Lote 4) + teste relacional de hash.
- **Comandos unit/leaf** — `Linebreak` (unit), `Colbreak`, `Pagebreak`:
  terminais; `Colbreak`/`Pagebreak` nunca vazios.

### Achado do checkpoint — `Hash` de `Parity` (dependência do lote)

`Pagebreak { weak, to: Option<Parity> }` e **`Parity` não implementava `Hash`**
(derivava `Debug, Clone, Copy, PartialEq, Eq`). Decisão do dono (opção a):
como `Parity` é enum **`Copy+Eq` sem floats** (confirmado: `{ Even, Odd }`), o
`Hash` canónico é trivialmente correto → **adicionado `Hash` ao derive de
`Parity`** (`entities/parity.rs`), e `PagebreakElem` deriva `Hash` normalmente.
**Não é conserto oportunista — é dependência directa do lote** (registado aqui;
`parity.md` L0 não pinava derives, nada a actualizar lá). Regra geral gravada no
modelo: Debug-hash só quando o `Hash` canónico é inseguro (f64/`Length`); tipo
`Copy+Eq` sem floats recebe o `derive`. `HSpace`/`VSpace` mantêm Debug-hash
(`Length`).

### Verificação dos `|`-combinados (carona aplicada a si própria)

A inspeção dos `|` envolvendo as 9 deu **só casos sem binding** (`Linebreak`
combinado com `MathAlignPoint(_)` em `matches!`/terminal de `math/layout` e
`layout/mod.rs`) → custo = largura, **sem separação extra**. (Contraste P319,
onde `Underline|Strike|Overline` partilhavam braço com binding.)

### Medições (métrica ADR-0104) — vs preditor

| medição | valor | comando |
|---|---|---|
| `content.rs` antes/depois | **5639 → 5603 (−36)** | `wc -l`; diff `+143 / −179` |
| parte atómica (9 módulos novos, c/ testes) | **748 linhas** | `wc -l elements/{…}.rs` |
| suíte typst-core | **2533 → 2566 (+33)** | só os 33 testes unitários novos |
| `crystalline-lint .` | **0 violations** | gate binário (inclui `parity.rs`) |

**Trajetória `content.rs`**: 5782 (P313) → 5735 (L2) → 5700 (L3) → 5639 (L4) →
**5603** (L5). Custo-por-módulo: HSpace/VSpace 99 (com `impl Hash` + teste),
Grid/Table H/F 88, Pagebreak 75, Colbreak 66, Linebreak 57. **Preditor
validado**: custo ∝ largura (112 sites; bulk de construções/matches em
`stdlib/mod.rs` e `layout/tests.rs`).

**Plano de toque (realidade)**: `content.rs` hub (6 matches → dispatch;
contentores recursam, leaves no bloco terminal), `layout/mod.rs` (handlers de
grid/table/spacing/breaks), `introspect.rs`/`locatable.rs` (materialize + walk +
matches exaustivos), `stdlib/layout.rs`+`stdlib/structural.rs` (construção via
construtores), `math/layout` (Linebreak nos `matches!`), `stdlib/mod.rs`+
`layout/tests.rs`+content.rs tests (construções e matches). **Nenhuma asserção
existente alterada.**

### Contabilidade atualizada (item obrigatório do relatório)

Migradas **22 → 31**; element-shaped restantes **~40 → ~31**; estimativa
**4–6 lotes** restantes. O bloco grid/table cell (`TableCell`/`GridCell`/
`Table`/`Grid`, ~193 sites) fica para lote próprio.

### Proposta do Lote 6 (do mapa C3 — decisão humana)

Por largura crescente: `Raw`(9) · `Align`(11) · `CounterDisplay`(15) ·
`Metadata`(16) · `Image`(17) · `Hide`(18) · `Repeat`(18) · `Quote`(20) · … —
ou um lote temático (ex.: **state/counter**: `State`/`StateDisplay`/`StateUpdate`/
`CounterUpdate`/`CounterDisplay`/`CounterDisplayCallback`). O bloco grid/table
cell é candidato a lote próprio (pesado).

---

## Encerramento

- `git log --oneline` (dois commits isoláveis): `Passo 320 — lote 5` ·
  `Passo 320 — carona de registro`.
- `crystalline-lint .` → **0 violations**. `git status` limpo (só cruft `lab/`).
- Caveat conhecida: stack default estoura em `recursao_infinita_*` — não é
  regressão (`RUST_MIN_STACK=33554432`).

## Fora de escopo (confirmado)

Lotes 6+ e o bloco grid/table cell (instância do modelo); DEBT-58 (gatilho não
disparou); F / `Set*` / 99.E; otimizações sugeridas por medição.
