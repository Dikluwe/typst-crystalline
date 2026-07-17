---

# P471 — `Value::Symbol` + parâmetros de `highlight` + `sub`/`super` size

> **Passo:** 471
> **Data:** 2026-06-26
> **Foco:** (1) `Symbol` como tipo de domínio e `Value::Symbol`; (2) parâmetros configuráveis de `highlight`; (3) argumento `size:` em `sub`/`super`.
> **Trilha:** 8 — Refinos de stdlib e tipos.
> **Tipo:** Materialização / Refacto mecânico.
> **Tamanho:** S–M (~35 min total; três sub-itens independentes).
> **ADR-0117 Cláusula 4:** `Symbol` marcado `não-iniciado` no mapa de migração (`typst_library::foundations::symbol`). `sub`/`super` e `highlight` têm struct no modelo D (P319/P448/P449). Verificar antes de propor campo novo.

---

## Contexto

O roteiro (Trilha 8) lista três itens pendentes após P470:

1. **`Symbol` refinado / `Value::Symbol`** — `typst_library::foundations::symbol` está `não-iniciado` (0%). O vanilla usa `Symbol` como valor para caracteres simbólicos acessíveis via notação de ponto (`sym.arrow.r`, `sym.eq.not`). O cristalino não tem `Value::Symbol` nem módulo `sym`.
2. **Parâmetros de `highlight`** — `HighlightElem` (P449) tem `body` e `fill: Option<Color>`. O vanilla também aceita `radius: Length` (cantos arredondados) e `extent: Length` (extensão horizontal). Apenas `fill` foi materializado.
3. **`size:` em `sub`/`super`** — `SubElem`/`SuperElem` (P448) têm `body` e `baseline: Option<Length>`. O vanilla aceita `size: Length` (tamanho do script). Apenas `baseline` foi materializado.

Os três itens são independentes entre si.

---

## ADR-0108 — Medir antes de decidir

| Pergunta | Resultado | Status |
|----------|-----------|--------|
| `Value::Symbol` existe? | Não — comentado em `value.rs` como variante futura | ❌ |
| Módulo `sym` existe no stdlib? | Verificar `rules/stdlib/mod.rs` | 🟡 |
| `HighlightElem` tem campo `radius`? | Verificar `entities/elements/highlight.rs` | 🟡 |
| `HighlightElem` tem campo `extent`? | Verificar idem | 🟡 |
| `native_highlight` aceita `radius:`/`extent:`? | Verificar `stdlib/text.rs` | 🟡 |
| Layout de highlight usa `radius`? | Verificar `layout/highlight.rs` ou arm no `layout/mod.rs` | 🟡 |
| `SubElem`/`SuperElem` têm campo `size`? | Verificar `entities/elements/sub.rs` e `super.rs` | 🟡 |
| `native_sub`/`native_super` aceitam `size:`? | Verificar `stdlib/text.rs` | 🟡 |
| Bloqueadores técnicos? | Nenhum conhecido | ✅ |

**Todas as sondas marcadas 🟡 devem ser executadas via `grep` com `file:line` antes de escrever código.**

---

## Sub-item A — `Value::Symbol`

### A.1 — Contexto vanilla

O vanilla define `Symbol` como valor de runtime com acesso por notação de ponto:

```typst
sym.arrow.r     // →  U+2192
sym.eq.not      // ≠  U+2260
sym.alpha       // α  U+03B1
#sym.times      // ×  U+00D7
```

`Symbol` no vanilla é uma struct com um nome canónico e uma tabela de modificadores (`Modifier`) que selecionam variantes Unicode. Complexidade total: 23 itens R + 41 boilerplate no mapa de migração.

### A.2 — Decisão de subset minimal

O cristalino não vai implementar a tabela de modificadores completa do vanilla neste passo. O subset minimal cobre:

1. Tipo de domínio `Symbol` — armazena um `char` e um nome canónico.
2. `Value::Symbol(Symbol)` — variante no enum `Value`.
3. Módulo `sym` no stdlib — mapa estático de `name → char` para os ~50 símbolos mais comuns.
4. Acesso por campo: `sym.arrow` → `Value::Symbol(Symbol { ch: '→', name: "arrow" })`.
5. `repr` de `Value::Symbol` → `"symbol(\"→\")"` ou simplesmente `"→"`.
6. Conversão implícita `Symbol → Content::Text` no layout (um `Symbol` usado em markup produz o char correspondente).

**Divergência declarada:** o vanilla suporta modificadores encadeados (`sym.arrow.r.double`) via `Modifier` struct. O cristalino implementa apenas nomes simples e compostos pré-definidos (`sym.arrow_r`, ou entrada direta `sym.arrow.r` como lookup flat). ADR a documentar.

### A.3 — Tipo de domínio `Symbol` (`entities/symbol.rs` — novo)

```rust
/// Símbolo Unicode nomeado. Subset minimal P471.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Symbol {
    pub ch:   char,
    pub name: EcoString,
}

impl Symbol {
    pub fn new(ch: char, name: impl Into<EcoString>) -> Self {
        Self { ch, name: name.into() }
    }
}
```

**Decisão:** sem `Modifier` neste passo. Scope-out explícito.

### A.4 — `Value::Symbol` (`entities/value.rs`)

Adicionar variante ao enum `Value`:

```rust
pub enum Value {
    // ... variantes existentes
    Symbol(Symbol),   // NOVO
}
```

Impacto nos match exaustivos de `Value`:
- `type_name()` → `"symbol"`
- `repr()` → o char como string (`"→"`)
- `From<Symbol> for Value`

### A.5 — Módulo `sym` no stdlib

**Ficheiro:** `rules/stdlib/sym.rs` (novo) + registo em `rules/stdlib/mod.rs`

Tabela estática de ~50 símbolos prioritários (subset do vanilla `sym` module):

```rust
pub static SYM_TABLE: &[(&str, char)] = &[
    ("arrow",       '→'),
    ("arrow.l",     '←'),
    ("arrow.r",     '→'),
    ("arrow.t",     '↑'),
    ("arrow.b",     '↓'),
    ("arrow.lr",    '↔'),
    ("eq",          '='),
    ("eq.not",      '≠'),
    ("lt",          '<'),
    ("gt",          '>'),
    ("lt.eq",       '≤'),
    ("gt.eq",       '≥'),
    ("plus",        '+'),
    ("minus",       '−'),
    ("times",       '×'),
    ("div",         '÷'),
    ("dot",         '·'),
    ("dots",        '…'),
    ("alpha",       'α'),
    ("beta",        'β'),
    ("gamma",       'γ'),
    ("delta",       'δ'),
    ("pi",          'π'),
    ("sigma",       'σ'),
    ("omega",       'ω'),
    ("infinity",    '∞'),
    ("sum",         '∑'),
    ("product",     '∏'),
    ("integral",    '∫'),
    ("sqrt",        '√'),
    ("in",          '∈'),
    ("not.in",      '∉'),
    ("subset",      '⊂'),
    ("supset",      '⊃'),
    ("union",       '∪'),
    ("sect",        '∩'),
    ("and",         '∧'),
    ("or",          '∨'),
    ("not",         '¬'),
    ("forall",      '∀'),
    ("exists",      '∃'),
    ("dagger",      '†'),
    ("star",        '⋆'),
    ("bullet",      '•'),
    ("diamond",     '◇'),
    ("circle",      '○'),
    ("square",      '□'),
    ("copyright",   '©'),
    ("trademark",   '™'),
    ("registered",  '®'),
];
```

Registo no scope como módulo `sym` com cada entrada acessível como campo.

### A.6 — Acesso no eval

`sym.arrow` em eval: o eval de `FieldAccess` sobre `Value::Module(sym_module)` devolve `Value::Symbol(Symbol { ch: '→', name: "arrow" })`.

Alternativa mais simples: `sym` é um `Dict` cujos valores são `Value::Symbol`. A decisão entre `Module` e `Dict` depende do que é menos disruptivo no eval existente — a sonda deve verificar.

### A.7 — Conversão `Symbol → Content` no layout

No layouter, um `Value::Symbol` ou `Content::Text` com o char diretamente. A conversão deve acontecer ao nível do eval quando um `Symbol` é usado em contexto de markup:

```rust
// Em eval markup, ao encontrar Value::Symbol(s):
Content::Text(EcoString::from(s.ch))
```

---

## Sub-item B — Parâmetros de `highlight`

### B.1 — Estado atual

`HighlightElem` (P449) tem:

```rust
pub struct HighlightElem {
    pub body: Content,
    pub fill: Option<Color>,
}
```

O vanilla também suporta `radius: Length` (cantos arredondados da caixa de destaque) e `extent: Length` (extensão horizontal para além do body).

### B.2 — `HighlightElem` ganha `radius` e `extent`

**Ficheiro:** `entities/elements/highlight.rs`

```rust
pub struct HighlightElem {
    pub body:   Content,
    pub fill:   Option<Color>,
    pub radius: Option<Length>,   // NOVO — cantos arredondados; None = 0pt
    pub extent: Option<Length>,   // NOVO — extensão horizontal; None = 0pt
}
```

Retrocompatibilidade: construtor existente `Content::highlight(body, fill)` define `radius: None, extent: None`.

### B.3 — `native_highlight` alargada

**Ficheiro:** `rules/stdlib/text.rs`

```
native_highlight(body, fill:?, radius:?, extent:?)
```

- `radius:` aceita `Length` ou `none`. `None` = sem cantos arredondados.
- `extent:` aceita `Length` ou `none`. `None` = sem extensão.
- Named args desconhecidos → erro descritivo.

### B.4 — Layout usa `radius` e `extent`

**Ficheiro:** `engine/layout/highlight.rs` ou arm no `layout/mod.rs`

O layout de highlight atualmente emite um `FrameItem::Shape` (retângulo preenchido) ou série de `FrameItem` equivalentes. Com `radius`:

```rust
let shape_kind = if let Some(r) = e.radius {
    ShapeKind::RoundedRect { radii: Corners::uniform(r) }
} else {
    ShapeKind::Rect
};
```

Com `extent`, alargar o retângulo em `extent_pt` para cada lado horizontalmente.

**Scope-out:** `extent` vertical (top/bottom) — apenas horizontal neste passo.

---

## Sub-item C — `size:` em `sub`/`super`

### C.1 — Estado atual

`SubElem`/`SuperElem` (P448) têm:

```rust
pub struct SubElem {
    pub body:     Content,
    pub baseline: Option<Length>,
}

pub struct SuperElem {
    pub body:     Content,
    pub baseline: Option<Length>,
}
```

O vanilla aceita `size: Length` para controlar o tamanho do script (default: ~65% do body, já fixo no layouter atual).

### C.2 — Adicionar `size: Option<Length>`

**Ficheiros:** `entities/elements/sub.rs`, `entities/elements/super.rs`

```rust
pub struct SubElem {
    pub body:     Content,
    pub baseline: Option<Length>,
    pub size:     Option<Length>,   // NOVO — None = 65% do font-size atual
}
```

Retrocompatibilidade: construtor existente define `size: None`.

### C.3 — `native_sub`/`native_super` alargadas

**Ficheiro:** `rules/stdlib/text.rs`

```
native_sub(body, baseline:?, size:?)
native_super(body, baseline:?, size:?)
```

### C.4 — Layout aplica `size`

**Ficheiro:** arm de `SubElem`/`SuperElem` no layout

```rust
let script_size = e.size
    .map(|s| s.resolve_pt(font_size))
    .unwrap_or(font_size * 0.65);
// Usar script_size ao fazer layout do body
```

---

## Tests

### Sub-item A

- **L1:** `Symbol::new('→', "arrow").ch == '→'`.
- **L1:** `Value::Symbol(Symbol::new('α', "alpha")).type_name() == "symbol"`.
- **L1:** `repr(Value::Symbol(...))` → `"→"` ou `"symbol(\"→\")"`.
- **L1:** Lookup `sym.arrow` → `Symbol { ch: '→', ... }`.
- **L1:** Lookup `sym.eq.not` → `Symbol { ch: '≠', ... }`.
- **L1:** Lookup `sym.inexistente` → `None` ou erro.
- **L2:** `Symbol` em contexto de markup produz `Content::Text("→")` no eval.
- **L2:** `repr(sym.alpha)` → `"α"`.

### Sub-item B

- **L1:** `HighlightElem` com `radius: Some(Length::pt(3.0))` preserva field em `map_content`.
- **L2:** `native_highlight(body, radius: 3pt)` produz `HighlightElem` com `radius: Some(3pt)`.
- **L2:** Layout com `radius: None` emite `ShapeKind::Rect`.
- **L2:** Layout com `radius: Some(r)` emite `ShapeKind::RoundedRect`.
- **L2:** `extent: Some(2pt)` alarga o retângulo 2pt para cada lado.

### Sub-item C

- **L1:** `SubElem` com `size: Some(Length::pt(8.0))` preserva field em `map_content`.
- **L2:** `native_sub(body, size: 8pt)` produz `SubElem` com `size: Some(8pt)`.
- **L2:** Layout de `SubElem` com `size: None` usa 65% do font-size.
- **L2:** Layout de `SubElem` com `size: Some(8pt)` usa 8pt.

---

## Spec L0

### Novos

- `entities/symbol.md` — `Symbol` struct, campos, construtores.
- `rules/stdlib/sym.md` — tabela `SYM_TABLE`, acesso por campo, divergência de modificadores.

### Actualizados

- `entities/value.md` — `Value::Symbol`, `type_name`, `repr`, `From<Symbol>`. Remover `Symbol` da lista de variantes futuras.
- `entities/elements/highlight.md` — campos `radius` e `extent`.
- `entities/elements/sub.md` — campo `size`.
- `entities/elements/super.md` (se ficheiro próprio existir) — campo `size`.
- `rules/stdlib/text.md` — `native_highlight`, `native_sub`, `native_super` alargadas.
- `engine/layout_figure.md` ou `layout/mod.md` — arm de highlight com radius/extent.

---

## Scope-out explícito

- **Modificadores encadeados de `Symbol`** (`sym.arrow.r.double`) — requer `Modifier` struct. Futuro. Divergência declarada em ADR (nova).
- **Tabela completa de `sym`** — o vanilla tem centenas de símbolos. Subset de ~50 neste passo.
- **`sym.emoji.*`** — fora do subset P471.
- **`extent` vertical de `highlight`** — apenas horizontal neste passo.
- **`highlight` em contexto multi-linha** — a extensão aplica-se por linha (paridade P284 wrap-aware). Se o body fluir para linha nova, o comportamento é best-effort.
- **`sub`/`super` com `size` relativo** (`65%`) — apenas `Length` absoluto ou `Em` neste passo; `Relative` (P469) não é conectado aqui.
- **`Value::Symbol` em `match` de `StyleChain`** — sem impacto em set rules neste passo.

---

## ADR necessária

**ADR nova (modificadores `Symbol`):** documentar a divergência entre o cristalino (lookup flat, `sym.eq.not` como entrada única) e o vanilla (modificadores encadeados). Tamanho XS; pode ser escrita inline no relatório e formalizada a seguir.

---

## Critério de fecho

- [ ] Sondas: `Value::Symbol`, `HighlightElem`, `SubElem`/`SuperElem`, módulo `sym` — todos com `file:line`.
- [ ] `Symbol` struct implementada em `entities/symbol.rs`.
- [ ] `Value::Symbol(Symbol)` adicionada ao enum `Value`.
- [ ] `type_name()`, `repr()`, `From<Symbol>` para `Value::Symbol`.
- [ ] `SYM_TABLE` com ~50 entradas em `rules/stdlib/sym.rs`.
- [ ] Módulo `sym` registado no scope; acesso por campo funcional.
- [ ] Eval de `Symbol` em markup produz `Content::Text`.
- [ ] `HighlightElem` tem campos `radius: Option<Length>` e `extent: Option<Length>`.
- [ ] `native_highlight` aceita `radius:` e `extent:`.
- [ ] Layout de highlight usa `radius` (Rect vs RoundedRect) e `extent`.
- [ ] `SubElem` e `SuperElem` têm campo `size: Option<Length>`.
- [ ] `native_sub` e `native_super` aceitam `size:`.
- [ ] Layout de sub/super aplica `size`.
- [ ] 16+ testes verdes (8 sub-A + 5 sub-B + 4 sub-C).
- [ ] Spec L0 actualizada/criada (8 ficheiros).
- [ ] ADR de divergência de modificadores `Symbol` redigida.
- [ ] `cargo test --workspace` verde; `crystalline-lint` zero violations.
- [ ] **Trilha 8: 5/8 completo** (repr P465 + métodos P466 + Relative P469 + marcadores/i18n P470 + Symbol/highlight/sub-super P471).

---

## Próximo passo (Trilha 8 termina ou pivot)

Itens restantes de Trilha 8 após P471:

- **P472** — `pad`/`corners`/`sides` inset modeling refinado (se ainda pendente após P242/P243) — verificar estado real antes de propor.
- **P472** — Pivot para Trilha 6: back-references + `ibid`/`op. cit.` (S-M, ~30 min).
- **P472** — Pivot para Trilha 4: `Value::Gradient` tipo real (M, ~35 min).
- **P472** — Pivot para Trilha 7: `columns`/`colbreak` (L, ~60+ min).

Com P471, Trilha 8 fica em 5/8. Os 3 itens restantes são pequenos ou requerem sonda de estado real antes de decidir se ainda há trabalho.

---

## Estado pós-P470 (para referência)

| Passo | Descrição | Estado | Trilha |
|-------|-----------|--------|--------|
| P465 | `repr()` completo | ✅ FECHADO | 8 |
| P466 | Métodos array/dict/str | ✅ FECHADO | 8 |
| P467 | Sonda `Selector::Where` | ✅ FECHADO | 3 |
| P468 | Estilos numéricos `[1]`, `[2]` | ✅ FECHADO | 6 |
| P469 | `Value::Relative` (`Rel<Length>`) | ✅ FECHADO | 8 |
| P470 | Marcadores list/enum + i18n caption | ✅ FECHADO | 8 |
| **P471** | `Symbol` + `highlight` params + `sub`/`super` size | 🔄 EM PREPARAÇÃO | 8 |

**Trilha 1: COMPLETA.**
**Trilha 2: COMPLETA.**
**Trilha 3: 1/3 completo.**
**Trilha 8: 4/8 completo** (pré-P471).
**Inventário de débitos: LIMPO.**
