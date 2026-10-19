# Passo 122.A — Inventário vanilla + decisões de `--font-path`

**Data**: 2026-04-23

---

## Parte 1 — Vanilla (`lab/typst-original/crates/typst-cli/src/args.rs:466-472`)

```rust
#[clap(
    long = "font-path",
    env = "TYPST_FONT_PATHS",
    value_name = "DIR",
    value_delimiter = ENV_PATH_SEP,
)]
pub font_paths: Vec<PathBuf>,

/// Ensures system fonts won't be searched, unless explicitly included via
/// `--font-path`.
#[arg(long, env = "TYPST_IGNORE_SYSTEM_FONTS")]
pub ignore_system_fonts: bool,
```

- Long `--font-path`, sem short.
- `value_name = "DIR"`.
- **Env var `TYPST_FONT_PATHS`**.
- **Separador de sistema** (`:` Unix / `;` Windows) — `value_delimiter`,
  **não** `ArgAction::Append`.
- Companheiro: `--ignore-system-fonts` + env `TYPST_IGNORE_SYSTEM_FONTS`.

Também inclui sub-comando `fonts` que lista fontes descobertas.

---

## Parte 2 — `SystemWorld::new` (`03_infra/src/world.rs:101-139`)

```rust
pub fn new(root: impl Into<PathBuf>, main: impl AsRef<Path>)
    -> Result<Self, SystemWorldError>
{ /* ... */ }

pub fn with_fonts(mut self, font_slots: Vec<FontSlot>) -> Self { /* ... */ }
```

**Cenário (α)** do spec 122 — construtor separado do builder de fontes.
`.with_fonts(Vec<FontSlot>)` já existe. Zero mudança em L3.

---

## Parte 3 — `discover_fonts` (`03_infra/src/fonts.rs:51-61`)

```rust
pub fn discover_fonts(font_paths: &[PathBuf]) -> Vec<FontSlot> {
    let mut slots = Vec::new();
    for path in font_paths {
        if path.is_dir() {
            discover_in_dir(path, &mut slots);
        } else if is_font_file(path) {
            push_slots(path, &mut slots);
        }
    }
    slots
}
```

**Comportamento path inválido**: `is_dir()` devolve `false` para path
inexistente; `is_font_file()` só testa a extensão. Se path inexistente
sem extensão de fonte → silent skip. Se path inexistente com extensão
`.ttf/.otf/...` → slot criado; `FontSlot::get()` retorna `None` ao tentar
ler bytes (também silent).

→ **Cenário (a)** do spec: L3 ignora silenciosamente. Panic-free.

`discover_in_dir` também ignora erros de `read_dir` (`let Ok(entries) = ... else { return }`). Robust.

---

## Parte 4 — L4 actual (`04_wiring/src/main.rs`)

```rust
let world = match SystemWorld::new(&root, &main_path) {
    Ok(w) => w,
    // ...
};
```

**NÃO chama `.with_fonts(...)`**. Font slots ficam em `Vec::new()`
inicializado pelo construtor. Portanto hoje `typst` compila sem
descobrir fontes — FontBook vazio; layout usa fallback interno
(provavelmente ineficaz para glyphs reais).

Este passo é a **primeira ligação** do binário a fontes reais.

---

## Parte 5 — Testes L4 existentes (`04_wiring/tests/cli.rs`)

Grep por `font`:
- Linha 65: `"#set text(font: \"Arial\")"` — teste aproveita font
  missing para disparar warning.
- Linhas 82–83: assertion de stderr contém `"font"`.
- Linha 157: comentário.

**Zero testes usam `--font-path`**. Testes novos são aditivos — não
afectam nenhum existente.

---

## Decisões

| Dimensão | Escolha | Razão |
|----------|---------|-------|
| Clap action | **`ArgAction::Append`** (repetível) | Spec 122; mais simples que `value_delimiter` platform-specific |
| Env `TYPST_FONT_PATHS` | **Defer** | Coerente com 121 (`TYPST_ROOT`); requer feature `env` do clap |
| `--ignore-system-fonts` | **Fora de escopo** | Escopo deste passo é apenas `--font-path` |
| Helper L2 `resolve_font_paths_with` | **Não criar** | Passagem directa `args.font_paths`; sem lógica real (spec aceita P6 como opcional) |
| Assinatura `SystemWorld::new` | **Não mudar** | Cenário (α); usar `.with_fonts` builder existente |
| `discover_fonts` | **Chamar em L4** | I/O filesystem; L4 já compõe L3 |
| Comportamento path inválido | **Silent skip** (status quo L3) | Panic-free; alinha Unix convention |

### Divergência vs vanilla

- **Repetir flag** (`--font-path /a --font-path /b`) em vez de
  `--font-path /a:/b`. Mais idiomático em CLI moderna. Documentar
  em `--help` como "May be repeated".
- **Sem env var**: deferido. Utilizador pode fazer shell alias.
- **Sem `--ignore-system-fonts`**: hoje cristalino não varre system
  fonts (empty default). Flag seria no-op.

### ADR-0051

Preview em 120.B mencionou literalmente `--font-path DIR` (repetível,
`Vec<PathBuf>` raw). Este passo executa preview 1:1. **Sem anotação**.

### Tamanho estimado

**Pequeno**. Cenário (α) + sem helper L2 + L3 intacto:
- L2: +3 linhas em `Args`, +1 linha em `RunIntent`, +1 linha em `parse()`.
- L4: destructure + chamada a `discover_fonts` + `.with_fonts(...)` (+3 linhas).
- L4 tests: 3 testes novos.
- Prompts: 2 ficheiros (cli.md + wiring.md).
