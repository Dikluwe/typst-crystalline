# Relatório P327 — Lote 12 (bloco grid/table cell) + carona C1-quater

**Pré-condição**: Lote 11 (P326) fechado — lint 0, suíte verde (typst-core 2662).
✅ Verificado.
**Commits**: `Passo 327 — carona de registro (C1-quater)` (tree limpo, só modelo)
e `Passo 327 — lote 12`.

---

## Carona C1-quater — skip-list em ambos os transformadores (confirmada)

A nota do P326 ("a skip-list deve cobrir ambos os transformadores") virou
**regra mecânica** (item 6 do passo mecânico): a mesma skip-list por
`ficheiro:linha` é entregue aos **dois** transformadores (construções e padrões);
nenhum decide por heurística. E **padrões aninhados** (destruturação interna além
do `Content::X` de topo) são **sempre manuais por classe** (`let <Pat> = … else`),
mesmo fora da skip-list. Commit `19ca79947`; zero código.

**Estreia neste lote:** a skip-list (73 sites) foi entregue aos dois
transformadores. A verificação pós-passada (rede de segurança) confirmou
**interseção vazia** em todas as 4 variantes (zero construtor-em-posição-de-padrão).
Os padrões aninhados que apareceram — tuple if-let `(Content::Grid, Content::Table)`
e (do P326) `Shape { kind: Path(items) }` — foram tratados à mão por classe.

---

## Lote 12 — bloco grid/table cell (4 variantes, ~193 sites)

**Composição confirmada: `TableCell`(32) · `Table`(41) · `GridCell`(47) ·
`Grid`(73)** — o maior lote do roteiro. **Modo: bloco inteiro** (decisão do dono)
com **validação intermediária** após cada variante.

### A válvula — resultado da inspeção (o ponto-chave)

- **Não disparou no hub**: cada uma das 4 tem o seu **próprio arm** em todos os
  matches; `GridCell`/`TableCell` são gémeas em campos mas **arms separados** —
  zero `|`-combinado com binding entre as 4 em `content.rs`.
- **Um combinado fora do hub**: `layout/grid.rs` tinha
  `Content::GridCell {…} | Content::TableCell {…}` (binding partilhado) —
  **dividido** (após migração, os `Elem` têm tipos distintos e não partilham
  or-pattern; split permanente, custo +1 arm, negligível).
- **Custo ≈ largura, sem inflação** → bloco rodado inteiro (válvula não forçou 2+2).

### Forma das 4

| Variante | Campos | Locatável | `is_empty` | `map_*` | Hash |
|----------|--------|-----------|------------|---------|------|
| `TableCell` | 10 (body + 9 cosm.) | não | `body.is_empty()` | recurse body | **manual** |
| `GridCell` | 10 (**gémea** de TableCell) | não | `body.is_empty()` | recurse body | **manual** |
| `Table` | 5 (cols/rows/children/stroke/fill) | não | `children.is_empty()` | recurse children (Vec) | **manual** |
| `Grid` | 10 (+ cells/header/footer) | não | `cells.is_empty()` | recurse cells+header+footer | **manual** |

**Todas não-locatáveis** (reconfirmado). **Interação Lote 5**: `Grid.header/footer`
(antes `Option<Box<Content>>`, agora `Option<Content>`) contêm `Content` que pode
ser `GridHeader`/`GridFooter` (já `Arc<…Elem>`) — `map_*` recursam transparente.

### Validação intermediária (regra deste lote, pelo tamanho)

| Após variante | typst-core | build |
|---|---|---|
| TableCell | 2666 | ✅ |
| Table | 2670 | ✅ |
| GridCell | 2674 | ✅ |
| Grid | 2678 | ✅ |

### C1…C1-quater — sites tratados à mão (zero conversões indevidas)

Skip-list: **73 sites** registados, entregue aos dois transformadores. Por
variante: ~5–18 padrões pulados pela skip-list em `stdlib/mod.rs`. Tratados à mão:

- **Arms de produção** (`introspect.rs` materialize/walk; `layout/{mod,grid,
  grid_placement}.rs`; `stdlib/{structural,layout}.rs` construções): destruturações
  → `(e)`/aliases; **construções densas** via `Arc::new(Elem{…})` com caminho
  qualificado (o construtor só cobre 5 de 10 campos — não serve para preservar
  cosméticos; daí `..(**e).clone()` em materialize).
- **`|`-combinado `GridCell | TableCell`** (`layout/grid.rs`): dividido à mão.
- **Tuple if-let `(Grid, Table)`** (`stdlib/mod.rs`): aninhado — à mão, em duas
  fases (Table na sua migração, Grid na dele).
- **Match-arms `Ok(…TableCell/GridCell {…})`** e **Arc move-outs**
  (`e.stroke.unwrap()` → `.clone().unwrap()`): à mão.
- **Achado de tooling**: o transformador de **construções** assumia
  "construtor-cobre-todos-os-campos" — falso para variantes densas. Adaptado para
  emitir `Content::X(Arc::new(Elem{…}))` (Arc-wrap), não `Content::x(…)`.

### Verificação pós-passada (rede de segurança C1-bis)

**Interseção vazia** nas 4 variantes — nenhum construtor (`table_cell`/`table`/
`grid_cell`/`grid`) em posição de padrão. A skip-list (C1-quater) preveniu; a
verificação confirmou. Nenhuma 5ª reincidência.

---

## Medições (ADR-0104)

- **`cargo build`**: limpo (lib 0 erros).
- **Suíte**: `RUST_MIN_STACK=33554432 cargo test --workspace` →
  **typst-core 2678 passed** (era 2662; **+16**), 0 failed. `typst-infra` 472,
  restantes verdes.
- **`content.rs`**: **5395 → 5157 linhas** (**−238** — o maior encolhimento do
  roteiro; os arms das 4 eram muito verbosos). Trajetória desde P313 (5782):
  **−625 acumulado**.
- **Parte atómica**: 4 módulos novos = **549 linhas** (`elements/{table_cell,
  table,grid_cell,grid}.rs`).
- **`crystalline-lint --fix-hashes .`**: 0 drift; **`crystalline-lint .`**:
  **✓ No violations found**.
- Ressalva conhecida: stack default em `recursao_infinita_*` — não é regressão
  (`RUST_MIN_STACK=33554432`).

## Contabilidade (modelo atualizado)

- Migradas **57 → 61**; restantes element-shaped **~5 → 1** (`Figure`).
- Conta de fecho: 61 + 4 `Set*` + 11 (DEBT-58) + 1 restante = **77** ✓.

## Proposta do Lote 13 (decisão humana) — `Figure`, o último

Resta **`Figure`(89)** — a mais larga, **o último element-shaped**. **Aviso**: ao
fechar `Figure`, **dispara o gatilho do DEBT-58** (triagem dos primitivos de AST
— `MathSequence`/`MathText`/`MathIdent`/`Sequence`/`Empty`/`Block` + `Space` +
wrappers `Styled`/`Boxed`/`Labelled` + `Text`). A triagem é **conversa de
desenho, não lote**: o prompt do P328 fecha `Figure`; o passo seguinte abre a
triagem do DEBT-58.

`Figure` tem ~4 campos (`body`/`caption`/`kind`/`numbering`) e **é locatável**
(M1, junto de Heading/Cite) — absorve `element_kind`/`to_payload` (precedente
Heading); a Fase A do P328 confirma campos e locatabilidade.

## Fora de escopo (confirmado)

Lote 13 (`Figure`); DEBT-58 (gatilho dispara quando `Figure` fechar — a triagem
é o passo seguinte ao L13); F / `Set*` / 99.E; otimizações sugeridas por medição
(medir ≠ mexer).
