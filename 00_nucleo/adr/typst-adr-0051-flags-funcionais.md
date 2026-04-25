# ⚖️ ADR-0051: Flags funcionais em L2 — pattern e primeira flag (`-o`)

**Status**: `EM VIGOR`
**Complementa**: ADR-0047 (clap), ADR-0049 (CLI em L2),
ADR-0050 (formatter em L2).
**Validado**: Passo 120.E.
**Data**: 2026-04-23
**Autor**: Passo 120
**Abrangente**: estabelece **pattern** para flags funcionais
futuras (`-o`, `--root`, `--font-path`, …), executando só `-o`
neste passo.

---

## Contexto

A CLI pós-Passo 119 tem apenas flags "meta":

- `--color=auto|always|never` (Passo 116).
- `--help` e `--version` (Passo 115).

Zero flags **funcionais** (que afectem comportamento de
compilação). Utilizadores reais esperam `-o`, `--root`,
`--font-path` e afins — padrão gcc/clang/rustc.

Este passo inicia a série com `-o/--output` e estabelece o
pattern que os próximos (`--root`, `--font-path`, etc.) seguirão.

---

## Decisão — Pattern para flags funcionais

### P1 — L2 define em `Args` com derive clap

Cada flag é um campo de `Args` em `02_shell/src/cli.rs`. Tipo:

- Valor único: `Option<T>` (para default derivado) ou `T` (com
  `default_value_t`).
- Repetível (`--font-path DIR` múltiplas vezes): `Vec<T>`.

### P2 — L2 converte raw → resolvido em `parse() -> RunIntent`

`parse()` chama helpers `resolve_xxx(args) -> Valor` que:

- Aplicam defaults (ex: `input.with_extension("pdf")`).
- Escolhem precedência (ex: flag > positional > default).
- Convertem tipos (ex: `Vec<PathBuf>` de entrada → tipo domínio).

### P3 — `RunIntent` cresce com campos prontos

Cada flag resolvida é um campo em `RunIntent`. L4 consome sem
conhecer `clap` ou defaults.

### P4 — Defaults em L2, não em L4

L2 é tradutor completo. L4 só compõe.

### P5 — Validação profunda em L3/L4

L2 **não faz I/O**. Verificar se um path existe, se uma fonte é
válida, se um directório é acessível — fica em L3 (descoberta,
carga) ou L4 (erro de composição). L2 só aceita argumentos e
resolve defaults.

### P6 — Function `resolve_xxx_with` pura, testável

Quando a lógica de resolução não é trivial, extrair para função
pura `resolve_xxx_with(...)` testável sem clap mock, sem env
mutation. Wrapper `resolve_xxx(args)` lê inputs reais e chama
a pura.

**Precedente**: `resolve_colored_with(choice, no_color, is_tty)`
em `cli.rs` (Passo 116).

---

## Decisão específica para `-o/--output`

Após inventário do vanilla (120.A):

### Forma: **(c) Positional + `-o` sinónimo + default derivado**

```rust
struct Args {
    /// Input .typ file.
    input: PathBuf,

    /// Output PDF file (positional). Defaults to input with .pdf
    /// extension.
    output: Option<PathBuf>,

    /// Output PDF file (alternative). Takes precedence over
    /// positional if both provided.
    #[arg(short = 'o', long = "output")]
    output_flag: Option<PathBuf>,

    #[arg(long = "color", value_enum, default_value_t = ColorWhen::Auto)]
    color: ColorWhen,
}
```

**Regras de precedência**:

1. `-o/--output` flag (se presente) vence.
2. Positional `output` (se presente).
3. Default: `input.with_extension("pdf")`.

### Razões da escolha

1. **Vanilla usa positional** — mantém paridade.
2. **`-o` é convenção moderna** — gcc/clang/rustc/cargo usam.
3. **Tests 114 intactos** — zero migração.
4. **Default derivado** — UX mínimo friction (`typst in.typ`
   compila para `in.pdf`).

### Código de resolução

```rust
fn resolve_output(args: &Args) -> PathBuf {
    args.output_flag
        .clone()
        .or_else(|| args.output.clone())
        .unwrap_or_else(|| args.input.with_extension("pdf"))
}
```

`RunIntent.output: PathBuf` (sempre resolvido). L4 recebe valor
pronto.

---

## Preview de flags próximas (não executadas neste passo)

### `--root DIR`

**Função**: override da raiz do projecto (para imports com path
absolutos).

**Pattern** (antecipado):
```rust
#[arg(long = "root", value_name = "DIR")]
root: Option<PathBuf>,
```

`resolve_root(args) -> PathBuf`:
```rust
args.root.clone()
    .or_else(|| args.input.parent().map(Path::to_path_buf))
    .unwrap_or_else(|| PathBuf::from("."))
```

L4 passa `runtime.root` directo para `SystemWorld::new(root, main)`.

### `--font-path DIR` (repetível)

**Função**: directórios adicionais de fontes.

**Pattern**:
```rust
#[arg(long = "font-path", value_name = "DIR", action = ArgAction::Append)]
font_paths: Vec<PathBuf>,
```

`RunIntent.font_paths: Vec<PathBuf>` — passa raw. L4 chama
`discover_fonts(&runtime.font_paths)` + `SystemWorld::with_fonts`.

**L2 não valida** se paths existem — L3 (`discover_fonts`)
faz isso: paths inválidos geram warnings, não errors.

---

## Alternativas rejeitadas

### R-1 — Defaults em L4

Rejeitada. L4 fica menos thin; perde razão de existência de
`RunIntent` como intenção resolvida.

### R-2 — Validação de paths em L2

Rejeitada. L2 pós-117 não faz I/O (só lê env var). Validar
existência de paths requer filesystem. L3/L4 detecta.

### R-3 — Forma (a) só flag (eliminar positional)

Rejeitada (para `-o`). Diverge de vanilla; quebra tests 114.
Forma (c) preserva positional + adiciona flag.

### R-4 — Forma (b) dupla aceitação ambígua

Rejeitada. Sem regra de precedência clara, utilizador pode
confundir-se. Forma (c) especifica "flag vence" explicitamente.

### R-5 — Campos `Args` com `pub` para L4 consumir

Rejeitada. L4 não deve conhecer `Args` — só `RunIntent`. Manter
`Args` privado em L2 honra encapsulamento (ADR-0049).

---

## Limitações aceites

1. **Sem `-o -` para stdout**: `-` como path literal, não como
   stdout. Utilizador redirige via shell.
2. **Sem validação de paths em L2**: ex: `-o /nao/existe/out.pdf`
   só falha em `fs::write` em L4.
3. **Overwrite silencioso**: se output existe, sobrescreve. Sem
   `--force` / `--no-clobber`. Convenção gcc.
4. **Short `-o` só para output**: `-r` (--root), `-f` (--format)
   não têm short por enquanto — evita conflito com subcomandos
   futuros que podem querer shorts curtos.

---

## Consequências

### Positivas

1. **UX moderna**: `typst input.typ -o output.pdf` funciona como
   gcc/clang/cargo.
2. **Default derivado**: `typst input.typ` compila para `input.pdf`
   — UX mínima.
3. **Tests 114 intactos**: positional continua a funcionar; zero
   migração.
4. **Pattern estabelecido**: `--root` e `--font-path` (passos
   futuros) seguem o mesmo modelo.
5. **`resolve_output` testável**: função pura, 2-3 testes unit
   cobrem precedência + default.

### Negativas

1. **API dupla**: positional + flag. Utilizador pode confundir.
   Mitigado por docstring clara em `--help`.
2. **Campo `output_flag`** no `Args` struct: nome interno divorciado
   do nome clap (`--output`). Necessário para evitar conflito com
   campo positional `output`. Cosmético.

### Neutras

1. **`RunIntent.output: PathBuf`** (agora sempre resolvido) —
   L4 não muda nada; o tipo já era `PathBuf`.
2. **Pipeline de compilação** intacto. Só muda como `output` é
   determinado.

---

## Aplicação

Implementado no Passo 120.C — ver
`00_nucleo/materialization/typst-passo-120-relatorio.md`.

Flags futuras (`--root`, `--font-path`) em passos dedicados,
aplicando este mesmo pattern.

ADR promovida a **EM VIGOR** em 120.E.
