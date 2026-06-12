# Relatório P326 — Lote 11 (largura: Footnote, Shape) + carona C1-ter

**Pré-condição**: Lote 10 (P325) fechado — lint 0, suíte verde (typst-core 2653).
✅ Verificado.
**Commits**: `Passo 326 — carona de registro (C1-ter)` (tree limpo, só modelo) e
`Passo 326 — lote 11`.

---

## Carona C1-ter — skip-list por linha-do-grep (confirmada e estreada)

**Mecanismo:** a lista do grep (passo 1) vira **skip-list explícita por
`ficheiro:linha`** passada ao transformador de construções; qualquer site na
lista é **pulado** (não decide por heurística; obedece). A verificação
pós-passada da C1-bis permanece como rede de segurança. Gravado no modelo
(Fase B, passo mecânico, item 5). Como o transformador é **efémero** (script
`/tmp` por lote), o mecanismo mora no modelo, não no script. Commit `d9ab56709`;
zero código de produto.

**Estreia — funcionou:** na Fase B deste lote o transformador de construções
recebeu a skip-list (21 sites de padrão); **pulou todos** e converteu só
construções. A verificação pós-passada confirmou **interseção vazia** (zero
construtor-em-posição-de-padrão) — **1ª passada limpa do transformador de
construções em 5 lotes** (P321–P325 todas erraram). A prevenção funcionou.

**Achado separado (honestidade epistémica):** há **dois** transformadores
efémeros — o de *construções* (skip-list aplicada, OK) e o de *padrões*
(if-let/matches → `(e)`). O de padrões **não** honrou a skip-list e quebrou
patterns **aninhados** `Shape { kind: ShapeKind::Path(items), .. }` (perdeu o
binding `items`); corrigidos à mão com `let-else`. **Nota para futuros lotes
gravada no modelo: a skip-list deve cobrir ambos os transformadores.** A
C1-ter resolveu o elo que atacou; o elo do transformador-de-padrões fica
exposto para o próximo passo que o use.

---

## Lote 11 — largura crescente (2 variantes)

**Composição confirmada: `Footnote`(46) · `Shape`(57)** = ~103 sites
largura-modelo (89 raw) — na faixa-guia (por baixo).

### Forma de cada `…Elem`

| Variante | Campos | Locatável | `is_empty` | `map_*` | Hash |
|----------|--------|-----------|------------|---------|------|
| `FootnoteElem` | `body` | **não** | default false (não delega) | recurse / recurse | **derive** (`Content`) |
| `ShapeElem` | `kind`, `width?`, `height?`, `fill?`, `stroke?` | **não** | default false | **terminal / terminal** (leaf, `\|`-combinado) | **manual** (`Value`/`Color`/`Stroke`, f64) |

### Locatabilidade (o achado do lote)

**Ambas não-locatáveis** — e isto **contraria a hipótese do passo**: `Footnote`
*não* é locatável no estado atual (P295 Fase-1 marker-only, scope-out per
ADR-0054; a frente P295.X tornaria-a locatável, mas hoje não — confirmado em
`locatable.rs:167` e ausência de arm em `extract_payload.rs`). `Shape` é
geometria inerte, não-locatável. **Nenhuma absorção `element_kind`/`to_payload`
neste lote** — contraste com Lotes 9–10.

### Forma — `Shape` leaf, sem assimetria, sem dependência

- **`Shape` é leaf** (geometria pura, sem body de conteúdo): terminal em
  `map_content` E `map_text`, no arm `|`-combinado → `(_)`, **sem split**.
- **Sem assimetria** (contraste `Equation` L10); **sem dependência-Hash**
  (contraste `CitationForm` L9): `Shape` é manual por floats; `Footnote` deriva.
- `Shape.width/height` mantêm `Option<Box<Value>>` (precedente `Image` L7);
  `derive(PartialEq)` equivale ao `.as_deref()` do hub.

### C1+C1-bis+C1-ter — sites de padrão tratados à mão

Grep prévio: **27 sites** de padrão registados (skip-list). Pelo transformador
de construções: **21 pulados pela skip-list** (stdlib/mod.rs ×18, introspect.rs
×3), zero conversões indevidas. Tratados à mão:

- **Arms de produção** (`introspect.rs` materialize/walk Footnote; `layout/mod.rs`
  Footnote + 2× Shape; `layout/helpers.rs` 2× Shape): destruturações → `(e)`/
  aliases. `Footnote.body` passou a `Box::new(e.body.clone())` no buffer de
  rodapé (campo `Box<Content>`).
- **Patterns aninhados `Shape { kind: Path(items), .. }`** (10 em
  `stdlib/mod.rs`): o transformador de padrões perdeu o binding `items` →
  reconstruídos com `let crate::…::ShapeKind::Path(items) = &e.kind else
  { panic!… }`. `e.stroke.clone()` onde `result` é owned (move-out de `Arc`).
- **Construções `&Content::{Footnote,Shape} { … }`** (falso-positivo `&` do
  transformador): convertidas após drop da heurística `&` (a skip-list cobre
  os padrões, então o `&` deixou de precisar de heurística).

---

## Verificação e medições (ADR-0104)

- **`cargo build`**: limpo (lib 0 erros).
- **Suíte**: `RUST_MIN_STACK=33554432 cargo test --workspace` →
  **typst-core 2662 passed** (era 2653; **+9**), 0 failed. `typst-infra` 472,
  restantes verdes.
- **`content.rs`**: **5385 → 5395 linhas** (**+10**) — exceção ao "encolhe": os
  6 braços do hub encolheram, mas **2 construtores novos** (`shape` 5-param +
  `footnote`, inexistentes antes) somaram ~13 linhas. Trajetória desde P313
  (5782): −387 acumulado.
- **Parte atómica**: 2 módulos novos = **191 linhas** (`elements/{footnote,
  shape}.rs`).
- **`crystalline-lint --fix-hashes .`**: 0 drift; **`crystalline-lint .`**:
  **✓ No violations found**.
- Ressalva conhecida: stack default em `recursao_infinita_*` — não é regressão
  (`RUST_MIN_STACK=33554432`).

## Contabilidade (modelo atualizado)

- Migradas **55 → 57**; restantes element-shaped **~7 → ~5**.
- Conta de fecho: 57 + 4 `Set*` + 11 (DEBT-58) + 5 restantes = **77** ✓.

## Proposta do Lote 12 (decisão humana) — o bloco grid/table cell

Restam **5**: `TableCell`(32) · `Table`(41) · `GridCell`(47) · `Grid`(73) —
o **bloco grid/table cell** (~193 sites) — + `Figure`(89). Sequência em vigor:
**L12 = bloco (4 variantes), L13 = `Figure`** (fecha os element-shaped, dispara
o gatilho do DEBT-58).

**Atenção para a Fase A do L12** (dimensionar o checkpoint do P327):

- **Campos densos**: `GridCell`/`TableCell` têm ~10 campos cumulativos cada
  (body/x/y/colspan/rowspan/stroke/fill/align/inset/breakable); `Grid` ~10
  (columns/rows/cells/gutter/align/inset/header/footer/stroke/fill); `Table`
  ~5 (columns/rows/children/stroke/fill). Hash provável **manual** (geometria
  `Length`/`Color`/`Stroke`).
- **`|`-combinados com binding entre as 4**: inspecionar `map_content`/`map_text`/
  `eq`/`materialize`/`walk` — se houver arm combinado com binding, **split**
  (regra do preditor P320).
- **Contentores recursivos**: `Grid`/`Table` recursam em `cells`/`children`
  (Vec); `GridCell`/`TableCell` no `body` + `header`/`footer` (Grid/Table).
- **Locatabilidade**: todas não-locatáveis (confirmado em `locatable.rs` — bloco
  estrutural sem identidade observable); sem absorção.
- **~193 sites** é o maior lote do roteiro — a skip-list (ambos os
  transformadores) e a divisão por variante serão críticas.

## Fora de escopo (confirmado)

Lote 12 (bloco grid/table cell) e Lote 13 (`Figure`); DEBT-58 (gatilho dispara
quando `Figure` fechar — ainda não); F / `Set*` / 99.E; otimizações sugeridas
por medição (medir ≠ mexer).
