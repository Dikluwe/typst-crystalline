---

# P470 — Marcadores configuráveis de `list`/`enum` + prefixo i18n de caption

> **Passo:** 470
> **Data:** 2026-06-26
> **Foco:** (1) Marcadores configuráveis de `list` e `enum`; (2) prefixo i18n de caption ("Figure" → "Figura").
> **Trilha:** 8 — Refinos de stdlib e tipos.
> **Tipo:** Materialização / Refacto mecânico.
> **Tamanho:** S–M (~30 min total; dois sub-itens intercaláveis).
> **ADR-0117 Cláusula 4:** Verificar `ListItem`/`EnumItem` (modelo D, P318) e `figure_supplement_for_lang` (P158B) antes de propor campo novo ou função nova.

---

## Contexto

O roteiro (Trilha 8, `roteiro-conclusao-typst-cristalino-atualizado.md`) lista dois itens pendentes de baixo risco adequados para este passo:

1. **Marcadores configuráveis de `list`/`enum`** — o cristalino tem `ListItem` e `EnumItem` (model D, P318) mas o marcador é fixo (`•` / `N.`). O Typst vanilla permite `#list(marker: "→", ...)` e `#enum(numbering: "a)", ...)`.
2. **Prefixo i18n de caption** — divergência declarada em P454: cristalino usa `"Figura"` (pt), vanilla usa `"Figure"` (en). A infraestrutura `figure_supplement_for_lang` (P158B) já existe mas não está conectada ao set rule de `lang`.

Ambos os itens são independentes. Podem ser executados em sequência ou em paralelo.

---

## ADR-0108 — Medir antes de decidir

| Pergunta | Resultado | Status |
|----------|-----------|--------|
| `ListItem` tem campo `marker`? | Não — `ListItemElem { body: Content }` apenas | ❌ |
| `EnumItem` tem campo `numbering`? | Não — `EnumItemElem { number: Option<u32>, body: Content }` | ❌ |
| `native_list` / `native_enum` aceitam argumentos de marcador? | Verificar `rules/stdlib/layout.rs` ou equivalente | 🟡 |
| `figure_supplement_for_lang` existe? | Sim — P158B em `rules/lang/figure_supplement.rs` | ✅ |
| `lang` propagado pela StyleChain ao layout de figura? | Verificar se `state.lang` chega ao arm de figura no layout | 🟡 |
| `CitationStyle::Numeric` é default após P468? | Sim | ✅ |
| Bloqueadores técnicos? | Nenhum conhecido | ✅ |

**Nota:** As duas sondas marcadas 🟡 devem ser feitas via `grep` antes de escrever código. Registar `file:line` nos resultados.

---

## Sub-item A — Marcadores configuráveis de `list`/`enum`

### A.1 — Contexto vanilla

O Typst vanilla suporta:

```typst
#list(marker: "→", item1, item2)
#list(marker: ([•], [–], [·]), item1, item2)   // marcadores por nível
#enum(numbering: "1.", item1, item2)
#enum(numbering: "a)", item1, item2)
#enum(numbering: "(i)", item1, item2)
```

O cristalino fixa `•` para `list` e `"N."` para `enum`. Este passo materializa o argumento `marker:` para `list` e `numbering:` para `enum` no subset minimal.

### A.2 — Entidade `ListMarker` (`entities/list_marker.rs` — novo)

```rust
/// Marcador de item de lista. Subset minimal: string fixa ou bullet padrão.
#[derive(Debug, Clone, PartialEq)]
pub enum ListMarker {
    /// Bullet padrão (`•`).
    Default,
    /// Marcador customizado (ex: `"→"`, `"-"`, `"*"`).
    Custom(EcoString),
}

impl Default for ListMarker {
    fn default() -> Self { Self::Default }
}

impl ListMarker {
    pub fn render(&self) -> &str {
        match self {
            Self::Default => "•",
            Self::Custom(s) => s.as_str(),
        }
    }
}
```

**Decisão:** Não implementar marcadores por nível (`([•], [–], [·])`) neste passo. Scope-out explícito.

### A.3 — `ListItemElem` ganha `marker: Option<ListMarker>`

**Ficheiro:** `entities/elements/list_item.rs`

```rust
pub struct ListItemElem {
    pub body:   Content,
    pub marker: Option<ListMarker>,  // NOVO; None = usar marcador do contexto
}
```

**Alternativa (preferida):** O marcador pode ser propriedade do contentor `list`, não do item. Verificar se existe `Content::List` ou se `ListItem` é o único nível. Se `list` não existe como elemento, adicionar campo ao `ListItemElem`; se existe, adicionar ao contentor.

### A.4 — `native_list` aceita `marker: Str`

**Ficheiro:** `rules/stdlib/layout.rs` (ou onde `list` está registado)

```rust
// Antes: native_list(items: Array) -> Content
// Depois: native_list(items: Array, marker: Option<Str>) -> Content
```

Ao construir cada `ListItem`, propagar `marker` como `ListMarker::Custom(s)`.

### A.5 — Layout renderiza marcador

**Ficheiro:** `rules/layout/mod.rs` (ou `rules/layout/list.rs`)

```rust
// No arm Content::ListItem:
let marker_str = item.marker
    .as_ref()
    .map(|m| m.render())
    .unwrap_or("•");
// Renderizar: "{marker_str} {body}"
```

### A.6 — Enum: `numbering:` como pattern string

O Typst vanilla usa patterns como `"1."`, `"a)"`, `"(i)"`. Subset minimal: suportar `"1."` (default), `"a)"`, `"A)"`, `"i)"`.

**Ficheiro:** `entities/enum_numbering.rs` (novo) ou inline em `EnumItemElem`.

```rust
pub enum EnumNumbering {
    Decimal,        // "1.", "2.", ...
    LowerAlpha,     // "a)", "b)", ...
    UpperAlpha,     // "A)", "B)", ...
    LowerRoman,     // "i)", "ii)", ...
    Custom(EcoString), // pattern livre (scope-out: apenas registo, sem parse completo)
}
```

**Decisão:** `EnumNumbering` como tipo separado em ficheiro próprio (sub-padrão #14, N cumulativo).

---

## Sub-item B — Prefixo i18n de caption

### B.1 — Estado atual

`figure_supplement_for_lang(kind, lang)` existe em `rules/lang/figure_supplement.rs` (P158B). Mas a divergência declarada em P454 indica que o cristalino usa `"Figura"` independentemente do `lang` configurado.

**Sonda necessária:** verificar se `state.lang` (ou `StyleChain lang`) é consultado no arm de figura do layouter ao escrever o prefixo de caption.

### B.2 — Ligação ao layouter

**Ficheiro:** `rules/layout/mod.rs` (ou `rules/layout/figure.rs`)

Localizar o sítio onde o prefixo `"Figura"` / `"Figure"` é construído. Substituir string fixa por:

```rust
let lang = state.lang.unwrap_or(Lang::Portuguese);  // default cristalino
let supplement = figure_supplement_for_lang(figure.kind.as_deref(), lang);
// Renderizar: "{supplement} {counter}" em vez de "Figura {counter}"
```

### B.3 — `set text(lang: "en")` muda o prefixo

Após a ligação, o seguinte deve funcionar:

```typst
#set text(lang: "en")
#figure(image("foto.png"), caption: [Descrição])
// Deve renderizar: "Figure 1" (não "Figura 1")
```

**Nota:** Se `lang` não está a chegar ao arm de figura via `state`, esta é a correção principal. Se a infra já existe mas o call está ausente, é uma linha de código.

### B.4 — Remover divergência declarada

Após P470, remover a anotação de divergência de paridade do inventário de cobertura referente ao prefixo de caption.

---

## Tests

### Sub-item A

- **L1:** `ListMarker::Default` renderiza `"•"`; `ListMarker::Custom("→")` renderiza `"→"`.
- **L1:** `EnumNumbering::LowerAlpha` formata número 1 como `"a)"`, 2 como `"b)"`.
- **L2:** `native_list` com `marker: "→"` produz `ListItem` com `ListMarker::Custom("→")`.
- **L2:** Layout de `ListItem` com marcador customizado renderiza `"→ texto"`.
- **L2:** `native_enum` com `numbering: "a)"` renderiza `"a) primeiro"`, `"b) segundo"`.

### Sub-item B

- **L2:** Documento com `#set text(lang: "en")` + figura → prefixo `"Figure"`.
- **L2:** Documento sem `set text(lang:)` → prefixo default (verificar qual é o default correto: `"Figure"` para paridade vanilla, ou manter `"Figura"` como cristalino pt).
- **L2:** `figure_supplement_for_lang(None, Lang::English)` → `"Figure"`.
- **L2:** `figure_supplement_for_lang(None, Lang::Portuguese)` → `"Figura"`.

**Nota sobre default:** Decidir antes de implementar se o default sem `set text(lang:)` é `"Figure"` (paridade vanilla) ou `"Figura"` (cristalino atual). Se mudar para `"Figure"`, atualizar os testes de regressão existentes de figura.

---

## Spec L0

- `entities/list_marker.md` — **NOVO** — `ListMarker` enum.
- `entities/enum_numbering.md` — **NOVO** — `EnumNumbering` enum.
- `entities/elements/list_item.md` — M — campo `marker`.
- `entities/elements/enum_item.md` — M — campo `numbering`.
- `rules/stdlib/structural.md` ou `layout.md` — M — `marker:` e `numbering:` em `list`/`enum`.
- `rules/layout/figure.md` ou `mod.md` — M — ligação `figure_supplement_for_lang`.

---

## Scope-out explícito

- **Marcadores por nível** (`([•], [–], [·])`) — requer infraestrutura de nível de aninhamento. Futuro.
- **Pattern `numbering` completo** (parse de `"(1a)"`, offsets, etc.) — apenas subset `"1."`, `"a)"`, `"A)"`, `"i)"` neste passo.
- **`LowerRoman` / `UpperRoman` completo** — apenas `"i)"` = 1-12 simples (sem algoritmo geral de romano).
- **Marcadores de `list` herdados da StyleChain** — neste passo, `marker` só por argumento direto de `native_list`.
- **`table` caption supplement** — i18n de tabela separado (futuro, mas infra é a mesma).
- **`ref` supplement i18n** — P462 scope-out; continua diferido.

---

## Critério de fecho

- [ ] Sonda A: localizar `native_list`/`native_enum` e registar `file:line`.
- [ ] Sonda B: localizar onde prefixo de caption é construído e registar `file:line`.
- [ ] `ListMarker` enum implementado em `entities/list_marker.rs`.
- [ ] `EnumNumbering` enum implementado em `entities/enum_numbering.rs`.
- [ ] `ListItemElem` ou contentor `list` tem campo `marker`.
- [ ] `native_list` aceita `marker: Option<Str>`; `native_enum` aceita `numbering: Option<Str>`.
- [ ] Layout renderiza marcador customizado.
- [ ] Layout de `enum` respeita `numbering`.
- [ ] `figure_supplement_for_lang` ligado ao arm de figura no layouter.
- [ ] Default de lang para caption definido e documentado.
- [ ] 9+ testes verdes (5 sub-A + 4 sub-B).
- [ ] Spec L0 atualizada (6 ficheiros).
- [ ] Divergência de paridade do prefixo de caption removida do inventário (se default mudou para `"Figure"`).
- [ ] `cargo test --workspace` verde; `crystalline-lint` zero violations.
- [ ] **Trilha 8: 4/8 completo** (repr P465 + métodos P466 + Relative P469 + marcadores/i18n P470).

---

## Próximo passo (Trilha 8 continua ou pivot)

Itens restantes de Trilha 8 após P470:

- **P471** — `Symbol` refinado / `Value::Symbol` (S, ~15 min)
- **P471** — Parâmetros configuráveis de `sub`/`super`/`highlight`/decorações — offset, extent, size (S cada)
- **P471** — `pad`/`corners`/`sides` inset modeling refinado (se ainda pendente após P242/P243)
- **P471** — Pivot para Trilha 6: back-references + `ibid`/`op. cit.` (S-M, ~30 min)
- **P471** — Pivot para Trilha 4: `Value::Gradient` tipo real (M, ~35 min)

---

## Estado pós-P469 (para referência)

| Passo | Descrição | Estado | Trilha |
|-------|-----------|--------|--------|
| P444–P449 | Decorações de texto | ✅ FECHADO | 8 |
| P450 | Bibliography `.bib` de disco | ✅ FECHADO | 6 (Fase 1) |
| P451–P459 | Numeração completa | ✅ FECHADO | 1 |
| P460–P463 | Referências cruzadas + PDF /GoTo | ✅ FECHADO | 2 |
| P464 | Cleanup Label/Labelled | ✅ FECHADO | — |
| P465 | `repr()` completo | ✅ FECHADO | 8 |
| P466 | Métodos array/dict/str | ✅ FECHADO | 8 |
| P467 | Sonda `Selector::Where` | ✅ FECHADO | 3 |
| P468 | Estilos numéricos `[1]`, `[2]` | ✅ FECHADO | 6 |
| P469 | `Value::Relative` (`Rel<Length>`) | ✅ FECHADO | 8 |
| **P470** | Marcadores list/enum + i18n caption | 🔄 EM PREPARAÇÃO | 8 |

**Trilha 1: COMPLETA.**
**Trilha 2: COMPLETA.**
**Trilha 3: 1/3 completo.**
**Trilha 8: 3/8 completo** (pré-P470).
**Inventário de débitos: LIMPO.**
