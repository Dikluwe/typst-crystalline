# Inventário `Content::Strong` / `Content::Emph` — Passo 101.A

Data: 2026-04-23.

Total de ocorrências: **28** em 7 ficheiros.

---

## Por categoria

### E — Enum

| Ficheiro | Linha | Variante |
|----------|------:|----------|
| `entities/content.rs` | ~51 | `Strong(Box<Content>)` |
| `entities/content.rs` | ~53 | `Emph(Box<Content>)` |

### C — Construção / literal

| Ficheiro | Linha | Uso |
|----------|------:|-----|
| `rules/eval/markup.rs` | 45 | `Content::strong(body)` |
| `rules/eval/markup.rs` | 63 | `Content::emph(body)` |
| `rules/stdlib/structural.rs` | 36 | `Content::Strong(Box::new(body))` |
| `rules/stdlib/structural.rs` | 51 | `Content::Emph(Box::new(body))` |
| `rules/introspect.rs` | 49-50 | `Content::Strong(..)` / `Content::Emph(..)` (em match arm — recria) |
| `rules/layout/tests.rs` | 122, 135, 163 | `Content::strong(...)` / `Content::emph(...)` |
| `entities/content.rs` (tests) | 1014, 1031, 1065 | construção directa em tests |

### M — Match arms

| Ficheiro | Linha | Contexto |
|----------|------:|----------|
| `entities/content.rs` | 450-451 | PartialEq |
| `entities/content.rs` | 552-553 | map_content |
| `entities/content.rs` | 702-703 | map_text |
| `entities/content.rs` | — | plain_text, is_empty (via fallback `_ => false`) |
| `rules/layout/mod.rs` | 257-268 | arm `Content::Strong` + arm `Content::Emph` em layout_content |
| `rules/eval/rules.rs` | 80-81 | selector show rule: `(Content::Strong(_), NodeKind::Strong)` |
| `rules/introspect.rs` | 49-50 | materialize_time |
| `rules/introspect.rs` | 255 | walk: `Content::Strong(body) \| Content::Emph(body) => walk(body, state)` |
| `entities/content.rs` (tests) | 1070 | inspect via `Content::Strong(body)` pattern |

### T — Testes

| Ficheiro | Linha | Tipo de dependência |
|----------|------:|---------------------|
| `entities/content.rs` (tests) | 842, 847, 1014, 1031, 1065, 1070 | construção + inspecção |
| `rules/layout/tests.rs` | 122, 135, 163 | construção |
| `rules/introspect.rs` (tests) | 593 | construção |

**Total M**: ~10 arm blocks.

---

## Desafios identificados

### 1. Show rule selector em `eval/rules.rs`

O match actual liga `Content::Strong(_)` a `NodeKind::Strong`:

```rust
match (&content, kind) {
    | (Content::Strong(_), NodeKind::Strong)
    | (Content::Emph(_),   NodeKind::Emph)
    ...
```

É como `#show strong: it => ...` identifica alvos. Após 101, o
critério passa a ser: `Content::Styled(body, styles)` onde `styles`
contém `Style::Bold(true)` → match `NodeKind::Strong` (e `Italic(true)`
→ `NodeKind::Emph`).

Precisa de regra mais cuidadosa — styles pode ter múltiplos estilos;
a política de match deve ser "contém pelo menos Bold/Italic activo".

### 2. `Content::strong(...)` / `emph(...)` construtores

8 call sites. Serão substituídos por factory helper:

```rust
impl Content {
    pub fn strong(body: Content) -> Self {
        Self::Styled(Box::new(body),
                     Styles::from_iter([Style::Bold(true)]))
    }
    pub fn emph(body: Content) -> Self {
        Self::Styled(Box::new(body),
                     Styles::from_iter([Style::Italic(true)]))
    }
}
```

Com esta redefinição dos construtores, a maioria dos call sites
continuam a compilar. Semântica preservada.

### 3. `introspect.rs` test construção

`Content::Strong(Box::new(Content::text("Negrito")))` → substituir
por `Content::strong(Content::text("Negrito"))` via construtor redefinido.

### 4. PartialEq, map_content, map_text

Perdem os arms dedicados. `Content::Styled(ba, sa) == Content::Styled(bb, sb)`
já cobre o caso (adicionado no Passo 99). Nenhuma acção adicional.

### 5. Layout tests

3 construções via `Content::strong(...)` / `Content::emph(...)` — como
manter o construtor (redefinido para emitir `Styled`), os tests
continuam a compilar e a resultar no mesmo comportamento visual.

---

## Recomendação

Procedimento seguro:

1. **Manter os construtores** `Content::strong(body)` e `Content::emph(body)`
   **mas redefinir** para emitir `Content::Styled(.., Styles::[Bold/Italic])`.
   — Isto mantém a API pública estável; 8 call sites não mudam.

2. Actualizar stdlib `native_strong`/`native_emph` para chamar
   `Content::strong()` em vez de `Content::Strong(Box::new(...))`.

3. Remover arms dedicados em `layout`, `introspect`, `map_content`,
   `map_text`, `PartialEq`.

4. Actualizar selector em `eval/rules.rs`:
   - `Content::Styled(body, styles)` com `Bold(true)` → `NodeKind::Strong`.
   - `Content::Styled(body, styles)` com `Italic(true)` → `NodeKind::Emph`.
   
   (Estratégia: função helper `styles_contains_bold(&Styles) -> bool`.)

5. **Remover** variantes `Strong(Box<Content>)` e `Emph(Box<Content>)`
   do enum.

6. Actualizar tests que usam `matches!(..., Content::Strong(_))` para
   o novo padrão.

**Factor de risco**: M=10 arms é manejável (limite é 30). Paridade
funcional preservada via redefinição dos construtores.

Decisão: **Proceder em sequência 101.B → 101.E**.
