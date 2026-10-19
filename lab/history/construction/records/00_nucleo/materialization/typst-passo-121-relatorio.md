# Passo 121 — Relatório (`--root DIR` flag)

**Data**: 2026-04-23
**Precondição**: Passo 120 encerrado; 1025 total tests; zero
violations; 51 ADRs activas.
**Natureza**: segunda flag funcional aplicando pattern ADR-0051.
**ADR tocada**: **ADR-0051** (sem anotação — preview em 120.B
já descrevia `--root` literal).

---

## Sumário

CLI ganha flag `--root DIR` com fallback
`input.parent() → "."`. Pattern ADR-0051 aplicado *tal qual*:
`Args` ganha `root: Option<PathBuf>`, `RunIntent` ganha
`root: PathBuf`, `resolve_root_with` é pura e testável.

L4 fica **mais thin**: `main.rs` deixa de calcular
`input.parent()` localmente; destruturation directa de
`RunIntent { input, output, root, colored }` — 5 linhas
removidas.

**811 L1 + 24 L2 (+3) + 186 L3 + 8 L4 (+1)** + 6 ignorados =
**1029 total** (+4 novos testes). Zero violations. **51 ADRs
activas** (+0).

---

## 121.A — Inventário vanilla + decisões

Inventário completo em
`00_nucleo/diagnosticos/inventario-root-flag-passo-121.md`.

### Vanilla (typst-cli args.rs:392-394 + world.rs:243-256)

```rust
#[clap(long = "root", env = "TYPST_ROOT", value_name = "DIR")]
pub root: Option<PathBuf>,

// Em run-time:
root.as_deref()
    .or_else(|| input_path.as_deref().and_then(|i| i.parent()))
    .unwrap_or(Path::new("."))
    .canonicalize()?
```

### Decisões

| Dimensão | Escolha | Justificação |
|----------|---------|--------------|
| Semântica | **(a)** localiza main + placeholder futuro | Alinha vanilla; virtualização é passo dedicado |
| Default | **(a)** `input.parent() → "."` | Exacto fallback vanilla |
| Env `TYPST_ROOT` | **Não** | clap feature `env` não declarada; defer |
| Canonicalização | **Não** (por agora) | I/O pertence a L3; `SystemWorld::new` já resolve |
| Short flag | **Nenhum** | `-r` reservado; alinha vanilla |

### Semântica honesta (limitação documentada)

Cristalino `SystemWorld` hoje ignora `--root` para resolução de
imports — usa `directory_of(current_file)`. `--root` nesta passo
apenas localiza o **main**. Virtualização de paths absolutos é
passo dedicado (registado como candidato futuro em 121.D).

---

## 121.B — ADR-0051

**Sem anotação.** A preview em ADR-0051 (Passo 120.B) já
descrevia exactamente `resolve_root_with` com o fallback
`input.parent() → "."`. Este passo é a primeira *aplicação* do
pattern — confirmação, não revisão.

Pattern ADR-0051 aplicado ponto a ponto:

- **P1** ✓ — `Args.root: Option<PathBuf>` com `#[arg(long = "root", value_name = "DIR")]`.
- **P2** ✓ — `parse()` chama `resolve_root_with(args.root.as_ref(), &args.input)`.
- **P3** ✓ — `RunIntent.root: PathBuf` (raw resolvido).
- **P4** ✓ — default derivado em L2 (`input.parent() → "."`).
- **P5** ✓ — validação profunda em L3 (`SystemWorld::new` falha se path inválido).
- **P6** ✓ — função pura `resolve_root_with` com 3 unit tests.

---

## 121.C — Implementação

### Ficheiros tocados

| Ficheiro | Mudança |
|----------|---------|
| `02_shell/src/cli.rs` | +`root` em `Args`; +`root` em `RunIntent`; +`resolve_root_with` (pura); +3 unit tests. |
| `04_wiring/src/main.rs` | Destructure `root` de `RunIntent`; removido cálculo local `input.parent()`. 5 linhas a menos. |
| `04_wiring/tests/cli.rs` | +1 teste `cli_root_explicito`. |
| `00_nucleo/prompts/shell/cli.md` | Documenta `--root`, `resolve_root_with` e 15 testes total (era 12). |
| `00_nucleo/prompts/wiring.md` | Uso actualizado; passo 121 listado; candidato futuro de virtualização. |

### `resolve_root_with` — função pura

```rust
pub fn resolve_root_with(
    root: Option<&PathBuf>,
    input: &Path,
) -> PathBuf {
    root.cloned()
        .or_else(|| {
            input
                .parent()
                .map(Path::to_path_buf)
                .filter(|p| !p.as_os_str().is_empty())
        })
        .unwrap_or_else(|| PathBuf::from("."))
}
```

**Edge case capturado**: `Path::parent()` de `"file.typ"`
devolve `Some("")` — filtro `!p.as_os_str().is_empty()` evita
que o fallback fique em string vazia e cai no default `.`.

### 3 testes unitários

1. `resolve_root_flag_vence_parent` — flag explícita ganha.
2. `resolve_root_sem_flag_usa_parent_do_input` — `/tmp/sub/file.typ` → `/tmp/sub`.
3. `resolve_root_sem_flag_e_sem_parent_usa_dot` — `file.typ` → `.`.

### L4 — mais thin

```rust
// Antes (5 linhas):
let RunIntent { input, output, colored } = cli::parse();
let root = input
    .parent()
    .filter(|p| !p.as_os_str().is_empty())
    .map(Path::to_path_buf)
    .unwrap_or_else(|| PathBuf::from("."));

// Depois (1 linha):
let RunIntent { input, output, root, colored } = cli::parse();
```

`use std::path::Path` já não precisa; só `PathBuf` ficou.

---

## 121.D — Validação

### `cargo test --workspace`

```
test result: ok. 811 passed ...   (L1 inalterado)
test result: ok. 186 passed ...   (L3 inalterado)
test result: ok. 24 passed  ...   (L2 +3: resolve_root_*)
test result: ok. 8 passed   ...   (L4 +1: cli_root_explicito)
```

### `crystalline-lint .`

```
✓ No violations found
```

### Manual (`--help` + comportamento)

```bash
$ typst --help | grep -A1 -- "--root"
      --root <DIR>
          Project root directory. Used to locate the main file and
          (in future) virtualize imports. Defaults to the parent
          directory of `input`, or `.` if input has no parent
```

**Cenário explícito**:
```bash
$ cd /tmp/proj && typst main.typ --root /tmp/proj -o /tmp/out.pdf
$ ls -la /tmp/out.pdf   # 978 bytes ✓
```

**Cenário default**:
```bash
$ typst /tmp/file.typ
$ ls -la /tmp/file.pdf  # 940 bytes ✓
```

### Candidato futuro registado

**Imports cross-file virtualization**: hoje `SystemWorld`
resolve imports contra `directory_of(current_file)`, não
`root`. Paths absolutos `/__project__/foo.typ` (ou similar)
não mapeados. Passo dedicado quando necessário — exige extender
`SystemWorld` para aceitar prefixo virtual. Não bloqueia
ficheiros single-file actuais.

---

## 121.E — Encerramento

### Números finais

| Métrica | Antes (Passo 120) | Depois |
|---------|------:|-------:|
| L1 tests | 811 | 811 |
| L2 tests | 21 | **24** (+3) |
| L3 tests | 186 | 186 |
| L4 tests | 7 | **8** (+1) |
| **Total** | **1025** | **1029** (+4) |
| Ignorados | 6 | 6 |
| Violations | 0 | 0 |
| ADRs activas | 51 | 51 |
| DEBTs abertos | 11 | 11 |

---

## Lições

1. **ADR abrangente paga-se já**: ADR-0051 escrita no Passo 120
   previu `--root` literal. Implementação deste passo ficou a
   mapear 1:1 — zero invenção de pattern. Confirma valor da
   regra "ADR descreve a família, passos executam membros".

2. **L4 fica realmente thin**: destructure directa
   `RunIntent { input, output, root, colored }` ≡ L2 resolve,
   L4 passa. `use std::path::Path` deixou de ser necessário —
   sinal físico de que a lógica saiu.

3. **Filter de empty path é hack subtil**: `Path::parent()` de
   `"file.typ"` = `Some("")` é surpreendente e não óbvio dos
   docs do stdlib. Unit test `resolve_root_sem_flag_e_sem_parent_usa_dot`
   bloqueia regressão — exactamente o tipo de invariante que só
   se descobre com teste.

4. **Virtualização é escopo separado**: implementar `--root`
   semântico completo (com `/__project__/` virtualization)
   dobrava o passo. Decisão honesta: flag ganha hoje, semântica
   completa em passo dedicado. Documentação de `--help` diz
   "in future virtualize imports" — não engana utilizador.

5. **Env `TYPST_ROOT` deferido sem culpa**: clap feature `env`
   não estava declarada; adicionar era 1 linha mas tocar
   workspace-level deps por um efeito cosmético viola
   parcimónia. Futuros utilizadores que queiram env podem
   exportar shell alias ou pedir.

---

## Estado pós-Passo 121

### CLI final

```bash
$ typst --help
Typst compiler (crystalline)

Usage: typst [OPTIONS] <INPUT> [OUTPUT]

Arguments:
  <INPUT>   Input .typ file
  [OUTPUT]  Output PDF file (positional). Defaults to input with `.pdf` ...

Options:
  -o, --output <FILE>   Output PDF file. Alternative to positional ...
      --root <DIR>      Project root directory ... Defaults to parent of input, or `.`.
      --color <COLOR>   When to use coloured diagnostics [default: auto]
  -h, --help            Print help
  -V, --version         Print version
```

### Pattern ADR-0051 — estado de execução

| Flag | Passo | Estado |
|------|------:|:------:|
| `-o/--output` | 120 | ✓ done |
| `--root DIR` | 121 | ✓ done |
| `--font-path DIR` (repetível) | — | candidato |
| `-f/--format` | — | candidato (depende de PNG/SVG/HTML) |

### Trabalho futuro identificado

1. **`--font-path DIR` (repetível)** — pattern P1-P6. Requer
   decidir se `Vec<PathBuf>` passa raw para L3 (P5) ou já
   canonicalizado em L2. Favorece raw.
2. **Virtualização de imports** — `SystemWorld` aceita
   `virtual_prefix: Option<&str>`; paths absolutos dentro do
   root mapeiam. Sem bloqueio para flow single-file.
3. **`--root canonicalize` em L3** — reportar erro amigável
   se path inválido, não esperar pela falha tardia de leitura.
4. **Env `TYPST_ROOT`** — se utilizador pedir; adicionar clap
   feature `env` no workspace.
