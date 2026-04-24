# Passo 115.A — Decisões (clap e escopo de flags)

**Data**: 2026-04-23

---

## Parte 1 — Versão de clap no vanilla

`lab/typst-original/Cargo.toml.original:47-49`:

```toml
clap = { version = "4.4", features = ["derive", "env", "wrap_help"] }
clap_complete = "4.2.1"
clap_mangen   = "0.2.10"
```

`lab/typst-original/crates/typst-cli/Cargo.toml`:

```toml
clap = { workspace = true, features = ["string"] }
```

- **Versão major**: `4.4`. Clap 4.x é actual.
- **Features vanilla**: `derive`, `env`, `wrap_help` (+ `string`
  em `typst-cli`).

### Decisão de versão

**`clap = { version = "4", features = ["derive"] }`** no workspace.

- Pin a major `4` (não `4.4`) — permite atualizações minor sem
  alterações. Consistente com estilo do workspace (e.g.
  `anyhow = "1"`, `thiserror = "2"`).
- Features extra (`env`, `wrap_help`, `string`) **adiadas** até
  aparecerem no código. Adicionar depois é trivial; adicionar
  agora sem uso é churn.

---

## Parte 2 — Idioma clap no vanilla

`lab/typst-original/crates/typst-cli/src/args.rs` (revisto no
Passo 112.C):

```rust
#[derive(Parser)]
#[command(name = "typst", version, about = "...", help_template = ...)]
pub struct CliArguments {
    #[command(subcommand)] pub command: Command,
    #[clap(long, default_value_t = ColorChoice::Auto, ...)] pub color: ColorChoice,
    #[clap(long, env = "TYPST_CERT")] pub cert: Option<PathBuf>,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Command {
    Compile(CompileCommand),    // alias "c"
    Watch(WatchCommand),        // alias "w"
    Init(InitCommand),
    Query(QueryCommand),
    Eval(EvalCommand),
    Fonts(FontsCommand),
    Update(UpdateCommand),
    Completions(CompletionsCommand),
    Info(InfoCommand),
}

pub struct CompileArgs {
    #[clap(value_parser = ..., value_hint = ValueHint::FilePath)]
    pub input: Input,           // positional; `-` para stdin

    #[clap(required_if_eq("input", "-"), value_parser = ..., value_hint = ...)]
    pub output: Option<Output>, // POSITIONAL (não `-o/--output`)

    #[arg(long = "format", short = 'f')]
    pub format: Option<OutputFormat>,
    // ... WorldArgs (--root, --font-path, ...)
    // ... ProcessArgs
}
```

### Observações

- **Vanilla usa positional `input output`** (não `-o/--output`).
  O positional `output` é `Option<>` com `required_if_eq("input",
  "-")` — obrigatório se input é stdin, opcional caso contrário
  (default deriva de input).
- **Format é `-f/--format`**, não `-o`.
- **Top-level tem subcomandos** — `typst compile <input> [output]`.

### Implicação para o cristalino

Para alinhar com vanilla em positional-first convention **sem**
materializar subcomandos:
- **Positional `input output`** — idem à CLI actual do 113.
- Sem `-o/--output`.
- Sem flag `-f/--format` (só PDF por agora).

Isto mantém migração pura — zero mudança de UX face ao 113.

---

## Parte 3 — Decisões

### Escopo: **(a) Mínimo** — migração pura

- `--help`, `--version` gratuitos via derive.
- Positional `input output` mantido exactamente como no 113.
- Sem flags funcionais novas neste passo.

Razões:
1. **Alinhamento com vanilla**: vanilla é positional-first; não
   expõe `-o`.
2. **Escopo contido**: (b) introduz `Option<PathBuf>` positional
   com default — complexidade desnecessária; (c) adiciona flags
   semânticas (`--root`, `--font-path`) que são passos dedicados.
3. **Preserva testes 114** sem modificação.

### Compatibilidade: **(A) Manter positional**

- Invocação `typst input.typ output.pdf` continua a funcionar.
- Tests 114 inalterados.
- Mudança interna (manual → clap) é invisível ao utilizador.

Razões:
1. **Zero impacto em utilizadores** da CLI 113.
2. **Tests 114 inalterados** — regressão garantida sem ajustes.
3. **Alinhamento vanilla**: positional é o idioma.

### Rejeitadas

- **(b) `-o/--output`**: divergiria de vanilla (que usa positional).
  Dupla aceitação (B) é confusa em clap; único-flag (C) migra
  tests.
- **(c) Subset útil (`--root`, `--font-path`)**: cada uma é passo
  dedicado. `--root` integra com `SystemWorld::new(root, main)`
  de maneira que precisa de decisão específica; `--font-path`
  toca `discover_fonts`/`with_fonts`. Fora do escopo.

### Matriz final

| Escopo | Compat | Escolhido? |
|--------|--------|:---:|
| **(a) Mínimo** | **(A) Manter positional** | **✓** |
| (b) Mínimo+`-o` | (A,B,C) | ✗ — divergiria de vanilla |
| (c) Subset útil | (A,C) | ✗ — cada flag é passo dedicado |

---

## Gate 115.A

**Não disparado**. Vanilla usa positional; cristalino alinha-se.
Decisão (a)+(A) preserva tests 114 intactos e mantém UX exacta
de 113.

Pronto para 115.B (ADR) e 115.C (implementação).

---

## Pontos confirmados

- **Versão clap**: `4` com features `derive`.
- **`[workspace.dependencies]`** já existe — adicionar clap é 1
  linha.
- **Features extra de vanilla** (`env`, `wrap_help`, `string`) —
  adiadas.
- **`version = true` em `#[command(...)]`** — lê versão do
  `Cargo.toml` que é `version.workspace = true` → `"0.1.0"`.
- **`typst --version`** output esperado: `typst 0.1.0`.
- **Testes 114**: nenhum ajuste necessário (compat A).
