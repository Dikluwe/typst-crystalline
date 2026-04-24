# Passo 129.A — Inventário `text.weight` simbólico (DEBT-1 subset)

**Data**: 2026-04-24

---

## Parte 1 — Vanilla `FontWeight` cast

**Ficheiro**: `lab/typst-original/crates/typst-library/src/text/font/variant.rs:149-180`.

Macro `cast!` declara:
- **Write side** (u16 → string): os 9 canónicos mapeiam para nome;
  valores não-canónicos serializam como número via `to_number()`.
- **Read side from `i64`**: `from_number(v.clamp(0, u16::MAX as i64) as u16)`.
- **Read side from 9 strings**: `"thin" => Self::THIN`, etc. —
  exactamente os 9 nomes da tabela do 129.

**Comportamento em nome inválido**: cast! macro — strings não
listadas produzem **erro de cast** com mensagem listing
valid options.

**Sem aliases** (ex: não há `"normal"` → `"regular"`).

## Parte 2 — Arm "weight" actual (pós-Passo 126)

**Ficheiro**: `01_core/src/rules/eval/rules.rs:297-306`.

```rust
"weight" => {
    if let Value::Int(n) = val {
        if let Ok(w) = u16::try_from(n) {
            delta.weight = Some(w);
        }
    }
}
```

Apenas `Value::Int`. Tipo errado silent skip (pattern DEBT-1 XS).

## Parte 3 — `FontWeight` em L1

**Ficheiro**: `01_core/src/entities/font_book.rs:29-59`.

```rust
pub struct FontWeight(pub u16);

impl FontWeight {
    pub const THIN:       Self = Self(100);
    pub const EXTRALIGHT: Self = Self(200);
    pub const LIGHT:      Self = Self(300);
    pub const REGULAR:    Self = Self(400);
    pub const MEDIUM:     Self = Self(500);
    pub const SEMIBOLD:   Self = Self(600);
    pub const BOLD:       Self = Self(700);
    pub const EXTRABOLD:  Self = Self(800);
    pub const BLACK:      Self = Self(900);

    pub fn from_number(weight: u16) -> Self;
    pub fn to_number(self) -> u16;
    pub fn distance(self, other: Self) -> u16;
}
```

**Tipo semântico natural já existe**. 9 constantes com os valores
canónicos esperados. Helper `from_name(&str) -> Option<Self>` cabe
em `impl FontWeight` — ADR-0037 (coesão por domínio).

## Parte 4 — DEBT-49 L3

Input pós-Passo 126: `"#set text(font: \"A\", lang: \"pt\", stroke: 1pt)"`.
`weight` **não aparece** (removido no 126 — weight agora é conhecido).
Sem rotação.

---

## Decisões

| Dimensão | Escolha | Razão |
|----------|---------|-------|
| Localização helper | **(b)** — `impl FontWeight { pub fn from_name }` em `font_book.rs` | Módulo tipográfico já existe com constantes; ADR-0037 coesão por domínio |
| `StyleDelta.weight` | **mantém `Option<u16>`** | Decisão 126 preservada; arm faz `.to_number()` no caminho simbólico |
| Nome desconhecido | **silencioso** (sem warning) | Pattern DEBT-1 XS (coerente com tipo errado em outros arms) |
| Aliases | **não suportar** | Vanilla não tem; escopo estrito 9 nomes |
| ADR-0038 | **anotar curta** | Documenta pattern "helper em tipo semântico L1" como variante do DEBT-1 XS |

### Divergência vanilla

**Nome inválido**:
- **Vanilla**: erro de cast com mensagem.
- **Cristalino**: silent skip (captura falha → `delta.weight = None`).

Categoria ADR-0033: **semântica** (comportamento observável
diverge). Aceita porque:
1. Coerente com pattern DEBT-1 XS (tipo errado silencioso).
2. Tornar em erro exigiria passar por canal de diagnostics
   para valores no eval — hoje arms de `#set` não emitem
   erros fatais, só warnings.
3. Quando `eval_set_text` adoptar canal de validação
   semântica (passo dedicado), `FontWeight::from_name` já
   está disponível para reutilizar.

## Gate 129.A

**Passa**. Arquivos esperados: 2 em L1:
- `entities/font_book.rs` (+ helper ~15 linhas).
- `rules/eval/rules.rs` (+ import + 3 linhas no arm).

Tests +3. Zero ripple L3/L4.

---

## Pattern variante registado

**"Helper simbólico em tipo semântico L1"**:
1. Tipo semântico (`FontWeight`, `FontStretch`, `Color`...)
   já existe em L1 com constantes.
2. Adicionar método `from_name(&str) -> Option<Self>`.
3. Arm no eval delega: `if let Some(x) = T::from_name(s) { delta.<prop> = Some(x.to_number()); }`.
4. Nome desconhecido → silent skip (coerente DEBT-1 XS).

Aplicável a futuras propriedades simbólicas: `font-stretch`
(`"normal"`/`"condensed"`/...), `style` (`"italic"`/`"oblique"`/
`"normal"`), etc.
