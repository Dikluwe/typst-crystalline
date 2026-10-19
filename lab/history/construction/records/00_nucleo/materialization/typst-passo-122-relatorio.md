# Passo 122 — Relatório (`--font-path DIR` flag; fecho preview ADR-0051)

**Data**: 2026-04-23
**Precondição**: Passo 121 encerrado; 1029 total tests; zero
violations; 51 ADRs activas.
**Natureza**: terceira flag funcional; **fecha preview original
ADR-0051**.
**ADR tocada**: **ADR-0051** (sem anotação — preview 120.B
mencionava literalmente `--font-path`).

---

## Sumário

CLI ganha flag `--font-path DIR` repetível via
`ArgAction::Append`. `RunIntent.font_paths: Vec<PathBuf>`
passa directo para L4, que agora chama
`typst_infra::fonts::discover_fonts(&font_paths)` +
`.with_fonts(slots)` no `SystemWorld`.

**L3 intacto**. `SystemWorld::new` e `discover_fonts` já tinham
a assinatura certa (cenário α do spec 122.A). Este passo apenas
liga.

**Descoberta colateral**: L4 **nunca** chamava `with_fonts`
antes. Este passo é a primeira ligação real do binário a fontes
descobertas — utilizadores agora podem passar `--font-path` e a
flag tem efeito concreto.

**811 L1 + 24 L2 + 186 L3 + 11 L4 (+3)** + 6 ignorados =
**1032 total** (+3 novos testes). Zero violations. **51 ADRs
activas** (+0).

---

## 122.A — Inventário

Completo em
`00_nucleo/diagnosticos/inventario-font-path-passo-122.md`.

### Vanilla (typst-cli args.rs:466-472)

```rust
#[clap(
    long = "font-path",
    env = "TYPST_FONT_PATHS",
    value_name = "DIR",
    value_delimiter = ENV_PATH_SEP,
)]
pub font_paths: Vec<PathBuf>,
```

Usa **`value_delimiter` com separador de sistema** (`:` Unix /
`;` Windows). Tem env var e companheiro `--ignore-system-fonts`.

### Cristalino pré-122

- `SystemWorld::new(root, main)` + `.with_fonts(Vec<FontSlot>)` —
  **cenário (α)**.
- `discover_fonts(&[PathBuf]) -> Vec<FontSlot>` já existia; path
  inválido → silent skip (**cenário (a)**).
- L4 `main.rs` **nunca** chamava `with_fonts` — FontBook vazio.

### Decisões

| Dimensão | Escolha | Divergência vanilla |
|----------|---------|--------------------|
| Clap action | `ArgAction::Append` (repetível) | Sim (vanilla usa delimiter) |
| Env `TYPST_FONT_PATHS` | **Defer** | Sim (coerente com 121) |
| `--ignore-system-fonts` | **Fora escopo** | Sim (cristalino não varre system fonts hoje) |
| Helper L2 `resolve_font_paths_with` | **Não criar** | N/A |
| Semântica path inválido | Silent skip | Igual (ambos) |

---

## 122.B — ADR-0051

**Sem anotação.** Preview em 120.B mencionava `--font-path DIR`
(repetível, `Vec<PathBuf>`) exactamente como executado. Pattern
P1–P6 aplicado:

- **P1** ✓ `Args.font_paths: Vec<PathBuf>` com `ArgAction::Append`.
- **P2** ✓ `parse()` move `args.font_paths` → `RunIntent`.
- **P3** ✓ `RunIntent.font_paths: Vec<PathBuf>`.
- **P4** ✓ Default: `Vec::new()` (clap trivial).
- **P5** ✓ Validação profunda em L3 (`discover_fonts` faz I/O).
- **P6** ✗ — sem helper. Passagem directa sem lógica não precisa
  de testabilidade. Aceitável pelo spec ("helpers L2 triviais:
  preferir passagem directa. P6 é sobre testabilidade, não
  obrigação").

---

## 122.C — Implementação

### Ficheiros tocados

| Ficheiro | Mudança |
|----------|---------|
| `02_shell/src/cli.rs` | +`font_paths` em `Args` (ArgAction::Append); +`font_paths` em `RunIntent`; move em `parse()`. |
| `04_wiring/src/main.rs` | +import `discover_fonts`; destructure `font_paths`; `.with_fonts(discover_fonts(&font_paths))` após `SystemWorld::new`. |
| `04_wiring/tests/cli.rs` | +3 testes: explícito, repetível, inexistente. |
| `00_nucleo/prompts/shell/cli.md` | `Args`+`RunIntent` actualizados; nota sobre divergência Append vs delimiter. |
| `00_nucleo/prompts/wiring.md` | Uso, pipeline (passo 3 novo), escopo futuro. |
| **L3** | **intacto**. |

### `Args` — fragmento novo

```rust
/// Additional directories to search for fonts. May be repeated.
/// Invalid paths are silently skipped by the font discoverer.
#[arg(
    long = "font-path",
    value_name = "DIR",
    action = clap::ArgAction::Append,
)]
font_paths: Vec<PathBuf>,
```

### `parse()` — fragmento novo

```rust
RunIntent {
    input: args.input,
    output,
    root,
    font_paths: args.font_paths,  // move directo
    colored,
}
```

### L4 `main.rs` — diff

```rust
// ANTES:
let RunIntent { input, output, root, colored } = cli::parse();
// ...
let world = match SystemWorld::new(&root, &main_path) {
    Ok(w) => w,
    Err(e) => { /* ... */ }
};

// DEPOIS:
use typst_infra::fonts::discover_fonts;
// ...
let RunIntent { input, output, root, font_paths, colored } = cli::parse();
// ...
let font_slots = discover_fonts(&font_paths);
let world = match SystemWorld::new(&root, &main_path) {
    Ok(w) => w.with_fonts(font_slots),
    Err(e) => { /* ... */ }
};
```

3 linhas novas em L4. Helper `discover_fonts` é função
`typst_infra::fonts::discover_fonts` — L4 compõe L3 (OK V12: L4
não cria tipos, só chama funções L3).

---

## 122.D — Tests L4

### 3 testes novos

```rust
#[test] fn cli_font_path_explicito() { ... }
#[test] fn cli_font_path_repetivel() { ... }
#[test] fn cli_font_path_inexistente_nao_falha() { ... }
```

### Comportamento path inválido

Confirmado: `discover_fonts(&["/path/que/nao/existe"])` não
falha, não emite warning — slot inválido filtrado por
`is_dir() == false && is_font_file() == false`. Teste
`cli_font_path_inexistente_nao_falha` documenta.

---

## 122.E — Encerramento

### `cargo test --workspace`

```
test result: ok. 811 passed ...       (L1 inalterado)
test result: ok. 186 passed, 6 ignored (L3 inalterado)
test result: ok. 24 passed  ...       (L2 inalterado — sem helper)
test result: ok. 11 passed  ...       (L4 +3: font_path_*)
```

### `crystalline-lint .`

```
✓ No violations found
```

### Manual

```bash
$ typst --help | grep -B1 -A2 -- "--font-path"
      --font-path <DIR>
          Additional directories to search for fonts. May be
          repeated. Invalid paths are silently skipped by the
          font discoverer

$ typst /tmp/p122/file.typ \
    --font-path /tmp \
    --font-path /usr/share/fonts \
    --font-path /path/que/nao/existe/xyz \
    -o /tmp/p122/out.pdf
$ ls -la /tmp/p122/out.pdf   # 979 bytes ✓
```

Três paths (dois válidos, um inválido) — binário compila sem
erro, paths inválidos silent-skipped em L3.

---

## Números finais

| Métrica | Antes (Passo 121) | Depois |
|---------|------:|-------:|
| L1 tests | 811 | 811 |
| L2 tests | 24 | 24 |
| L3 tests | 186 | 186 |
| L4 tests | 8 | **11** (+3) |
| **Total** | **1029** | **1032** (+3) |
| Ignorados | 6 | 6 |
| Violations | 0 | 0 |
| ADRs activas | 51 | 51 |
| DEBTs abertos | 11 | 11 |

---

## Limitações aceites

1. **Sem env `TYPST_FONT_PATHS`**: vanilla tem; cristalino defere
   (igual a `TYPST_ROOT` no 121).
2. **Sem `--ignore-system-fonts`**: cristalino não varre system
   fonts hoje. Flag seria no-op; reintroduz quando sistema existir.
3. **Separador `:` não suportado**: `--font-path /a:/b` trata
   `/a:/b` como um único path. Documentação diz "May be repeated" —
   utilizador aprende rápido. Divergência deliberada.
4. **Sem validação L2**: L2 não verifica existência; passa raw a
   L3. Conforme P5 de ADR-0051. `discover_fonts` é silent para
   paths bad — sem feedback ao utilizador. Candidato futuro: Sink
   warn para "font-path DIR not found".
5. **Duplicados não deduplicados**: `--font-path /a --font-path /a`
   produz 2 varreduras. Trivial em L2; não priorizado.

---

## Lições

1. **L4 vazia de fonts era bug silencioso**: pre-122, L4
   compilava sem nunca chamar `.with_fonts()`. FontBook vazio
   escapava porque pipeline tem fallback interno para glyphs.
   Só tocar `--font-path` expôs. Valor colateral do passo:
   descobrir que a flag também activa um path que estava morto.

2. **Cenário (α) do spec validou-se**: constructor + builder
   separados (`new` + `with_fonts`) é API que escala — novo
   passo não mexeu `new`. Contraste com linguagens que forçam
   todos os params num único constructor.

3. **`ArgAction::Append` vs `value_delimiter`**: vanilla escolhe
   delimiter por portabilidade Unix (`PATH`-like). Cristalino
   escolhe Append por simplicidade. Ambos válidos; trade-off
   é UX shell script antigo vs UX moderna. Documentado.

4. **Helper L2 opcional é feature**: spec deixou `P6` como
   opcional. Neste passo `resolve_font_paths_with` seria
   `args.font_paths.to_vec()` — zero valor. Preferir move
   directo. ADR-0051 aguenta flags sem helpers quando pipeline
   não tem lógica.

5. **L3 já suporta**: não ter de tocar L3 para executar a flag é
   sinal de que a API de L3 foi bem dimensionada. `discover_fonts`
   feito em passos anteriores para suportar exactamente este
   caso — pay-off chegou 1-2 passos depois.

6. **ADR-0051 preview literal = zero divergência**: trilhos
   desenhados em 120 acertaram em 121 e 122 sem anotação. Três
   flags em três passos cumprem o pattern exactamente. Preview
   detalhado paga-se.

---

## Estado pós-Passo 122 — Preview ADR-0051 fechado

| Flag | Passo | Pattern aplicado |
|------|------:|:----------------:|
| `-o/--output` | 120 | P1-P6 completo |
| `--root DIR` | 121 | P1-P6 completo |
| `--font-path DIR` | 122 | P1-P5 (sem helper L2) |

### CLI final

```bash
$ typst --help
Typst compiler (crystalline)

Usage: typst [OPTIONS] <INPUT> [OUTPUT]

Arguments:
  <INPUT>   Input .typ file
  [OUTPUT]  Output PDF file (positional) ...

Options:
  -o, --output <FILE>      Output PDF file ...
      --root <DIR>         Project root directory ...
      --font-path <DIR>    Additional directories to search for fonts. May be repeated ...
      --color <COLOR>      When to use coloured diagnostics [default: auto]
  -h, --help               Print help
  -V, --version            Print version
```

### Trabalho futuro

1. **Env vars** (`TYPST_ROOT`, `TYPST_FONT_PATHS`) — feature
   `env` de clap; consistente entre flags.
2. **`--ignore-system-fonts`** — quando system font discovery
   existir (hoje `discover_fonts` só varre paths explícitos).
3. **System font discovery** — `fontdb` ou equivalente em L3.
   Passo dedicado; expande `fonts.rs`.
4. **`-f/--format`** — export PNG/SVG/HTML. Abre 4ª flag e
   novas ADRs (exports múltiplos).
5. **Subcomandos** (`compile`, `watch`, `query`, `fonts`) —
   passo grande; revê topologia de L2.
6. **Warning amigável para `--font-path` inexistente** — L3
   emite via Sink; hoje é silent. Pequeno, útil.
7. **Virtualização de imports** (do 121) — complementar a
   `--root` semântico completo.
