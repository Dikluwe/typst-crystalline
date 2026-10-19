# Passo 117 — Micro-refactor de alinhamento: CLI para L2

**Série**: 117 (micro-refactor; correcção arquitectural das
ADRs 0047 e 0048).
**Precondição**: Passo 116 encerrado; 811 L1 + 207 L3 + 5 L4 + 6
ignorados; zero violations.
**ADRs aplicáveis**: ADR-0046 (CLI mínima), ADR-0047 (clap),
ADR-0048 (cores).
**ADR nova**: ADR-00NN "CLI vive em L2 — correcção de
ADRs-0047/0048" — `PROPOSTO` em 117.B, `EM VIGOR` em 117.E.

---

## Natureza deste passo

**Correcção arquitectural**. Os Passos 113, 115 e 116 puseram CLI
em L3 e L4. A definição fundacional do projecto (em
`typst-migracao-estado.md`) define L2 (`02_shell/`) como "CLI —
interface com utilizador". Esta definição foi ignorada por 4
passos consecutivos. Resultado: L3 tem `clap` como dep, L4 tem
argparsing inline, L2 está vazio.

Este passo corrige sem adicionar funcionalidade. Zero cores
novas, zero flags novas, zero mudança de UX. Apenas move código
entre camadas para respeitar a arquitectura.

---

## Objectivo

Ao fim do passo, cada responsabilidade vive na camada correcta:

| Camada | Conteúdo |
|--------|----------|
| **L2 (02_shell)** | `Args` com `#[derive(Parser)]`, `ColorWhen` enum, `resolve_colored_with`, `cli::parse() -> RunIntent`. Conhece `clap` e env vars. |
| **L3 (03_infra)** | `format_diagnostic(..., colored: bool)`, `drain_diagnostics_to_stderr(..., colored: bool)`, pipeline. **Não** conhece `clap`. **Não** conhece env vars. |
| **L4 (04_wiring)** | `main()` thin — chama `cli::parse()`, invoca pipeline L3, passa `colored` do `RunIntent`. ~20 linhas. |

Zero mudança observável para o utilizador externo. Testes 114
continuam a passar sem modificação.

---

## Decisões já tomadas

1. **L2 ganha conteúdo real** — primeira vez desde o Passo 0.
2. **`RunIntent` struct pura** produzida por L2, consumida por L4.
3. **L4 chama `cli::parse()`** helper em L2 — não usa
   `Args::parse()` directamente. Isolamento completo de `clap`
   em L2.
4. **`03_infra/Cargo.toml` perde dep `clap`**.
5. **`02_shell/Cargo.toml` ganha dep `clap`**.
6. **`format_diagnostic` e `drain_diagnostics_to_stderr`
   permanecem em L3** com assinatura `colored: bool`. Não mudam.
7. **Sem mudança em L1**.

## Decisão diferida (117.A)

8. **Localização do código em L2**:
   - `02_shell/src/cli.rs` novo módulo.
   - `02_shell/src/lib.rs` directo.
   Depende do tamanho do conteúdo e do estado actual do L2.
   Decidido em 117.A com base em inventário.

---

## Escopo

**Dentro**:
- `02_shell/src/` — código novo (move de L3 e L4).
- `02_shell/Cargo.toml` — dep `clap`.
- `03_infra/src/diagnostic_format.rs` — remover `ColorWhen`,
  `resolve_colored_with`, testes relacionados (movem para L2).
  Manter `format_diagnostic`, `drain_diagnostics_to_stderr`,
  testes de formatação.
- `03_infra/Cargo.toml` — remover dep `clap`.
- `04_wiring/src/main.rs` — remover `Args`, wrapper local.
  Chama `typst_shell::cli::parse()`. ~20 linhas finais.
- `04_wiring/Cargo.toml` — manter dep `clap`? Sim, porque `Parser`
  trait é usado via `Args::parse()` implicitamente. Confirmar:
  **se L4 só chama `typst_shell::cli::parse()`**, não precisa de
  `clap` directo. Remover.
- Prompts L0 afectados: `02_shell/prompts/shell.md` ganha
  conteúdo; `03_infra/prompts/diagnostic_format.md` perde
  ColorWhen; `04_wiring/prompts/wiring.md` actualizado.
- Testes: movem conforme o código.

**Fora**:
- Funcionalidade nova.
- Mudança de UX.
- Subcomandos.
- Flags adicionais.
- Mudança em L1.

---

## Sub-passos

### 117.A — Inventário

**Parte 1 — Estado actual de L2**:

1. `view` em `02_shell/`. Listar estrutura: ficheiros, Cargo.toml,
   lib.rs.
2. `view` em `02_shell/src/lib.rs`. Registar:
   - Linhas existentes (provavelmente só header).
   - Se há módulos declarados.
   - Se há qualquer dep externa em Cargo.toml.
3. `view` em `02_shell/Cargo.toml`. Confirmar deps actuais.

**Parte 2 — Escopo da migração**:

1. `grep` em `03_infra/src/diagnostic_format.rs` por `ColorWhen`,
   `resolve_colored_with`, testes `resolve_colored_*`. Listar.
2. `grep` em `04_wiring/src/main.rs` por `Args`, `resolve_colored`
   (wrapper thin). Listar.
3. `grep` em ambos por `use clap`, `clap::`. Registar todos os
   imports.

**Parte 3 — Tamanho estimado**:

Contar linhas a mover:

```
ColorWhen enum + docstrings:     ~8 linhas
resolve_colored_with (pura):     ~10 linhas
testes resolve_colored_*:        ~40 linhas (6 testes)
struct Args:                     ~12 linhas
resolve_colored wrapper (L4):    ~10 linhas
TOTAL para L2:                   ~80 linhas
```

**Parte 4 — Decisão de localização em L2**:

Com base em Parte 1:

- **`02_shell/src/cli.rs`** se o conteúdo é ≥ 50 linhas. Evita
  inflar `lib.rs`.
- **`02_shell/src/lib.rs` directo** se conteúdo é < 50 linhas e
  L2 não tem outros módulos.

Estimativa: ~80 linhas. Provavelmente `cli.rs` dedicado.

**Escrever** em `00_nucleo/diagnosticos/inventario-l2-refactor-passo-117.md`:

```
L2 actual:
  lib.rs: N linhas
  Cargo.toml deps: [...]
  módulos: [...] (provavelmente nenhum)

Migração:
  De L3 (diagnostic_format.rs): ColorWhen + resolve_colored_with + testes
  De L4 (main.rs): Args + resolve_colored wrapper
  Total: ~80 linhas

Localização escolhida: 02_shell/src/cli.rs dedicado
  Razão: conteúdo > 50 linhas, primeiro módulo de L2
```

**Gate 117.A**: se o inventário revelar que `02_shell` tem
conteúdo útil pré-existente que conflita com este passo, parar
e reportar. (Improvável — relatório de continuidade e Passo 0
indicam L2 como stub.)

### 117.B — ADR nova

Criar `00_nucleo/adr/typst-adr-00NN-cli-em-l2.md` com `PROPOSTO`.

Conteúdo:

- **Contexto**: Passos 113, 115, 116 colocaram CLI em L3 e L4,
  ignorando a definição fundacional de L2 em
  `typst-migracao-estado.md`. V12 do linter disparou no Passo
  116 mas a correcção moveu para L3 em vez de L2 (resistência
  menor). Este passo corrige.
- **Decisão**:
  - `02_shell` ganha conteúdo real: `Args`, `ColorWhen`,
    `resolve_colored_with`, `cli::parse() -> RunIntent`.
  - `RunIntent` struct pura com `input`, `output`, `colored` (e
    futuros campos à medida que flags forem adicionadas).
  - L3 mantém funções burras sobre `bool`; perde `clap` como dep.
  - L4 fica ~20 linhas, chama `cli::parse()` sem conhecer `clap`.
- **Relação com ADRs 0047 e 0048**:
  - ADR-0047 declarou "clap adicionado a 04_wiring". Corrige-se:
    clap em 02_shell.
  - ADR-0048 declarou "ColorWhen em L3". Corrige-se: ColorWhen
    em L2.
  - ADR-0048 declarou "L4 envolve L3". Corrige-se: L2 envolve,
    L4 compõe.
  - ADRs 0047 e 0048 ficam com nota de "corrigido pela ADR-00NN
    neste ponto". Não revogadas — mantêm contexto histórico;
    mas decisões específicas são substituídas.
- **Alternativas rejeitadas**:
  - **Manter em L3/L4 e aceitar V12**: contradiz definição
    fundacional. Qualquer passo futuro que queira respeitar L2
    enfrenta a mesma questão.
  - **Revogar ADR-0047/0048 completamente**: agressivo demais.
    Decisões base (usar clap, cores ANSI, paleta) são correctas;
    só a camada estava errada.
- **Sem mudança de funcionalidade**: utilizador vê exactamente
  o mesmo output, mesmos comandos, mesmos exit codes.

Promover em 117.E.

### 117.C — Implementação

Ordem obrigatória. Compilação pode partir a meio; aceitável
enquanto o passo não acaba.

**117.C.1 — Criar `02_shell/src/cli.rs` (se decidido)**:

Conteúdo:

```rust
//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/shell/cli.md
//! @prompt-hash <novo>
//! @layer L2
//! @updated 2026-04-23

use std::path::PathBuf;
use clap::Parser;

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum ColorWhen {
    /// Cores activas se stderr é terminal e `NO_COLOR` ausente.
    Auto,
    /// Cores sempre activas, mesmo em pipe.
    Always,
    /// Cores sempre desactivadas.
    Never,
}

#[derive(Parser, Debug)]
#[command(
    name = "typst",
    version,
    about = "Typst compiler (crystalline)"
)]
struct Args {
    /// Input .typ file.
    input: PathBuf,
    /// Output PDF file.
    output: PathBuf,
    /// When to use coloured diagnostics.
    #[arg(long = "color", value_enum, default_value_t = ColorWhen::Auto)]
    color: ColorWhen,
}

/// Intenção de execução pura — output de L2 para L4.
///
/// Criada em L2 depois de traduzir argumentos + env vars + isatty.
/// L4 consome sem conhecer clap ou env vars.
#[derive(Debug)]
pub struct RunIntent {
    pub input: PathBuf,
    pub output: PathBuf,
    pub colored: bool,
}

/// Ponto de entrada público da CLI.
///
/// Parses arguments, resolves colored output, returns RunIntent.
/// Em erro de args, termina o processo (clap::Parser::parse).
pub fn parse() -> RunIntent {
    let args = Args::parse();
    let colored = resolve_colored(&args.color);
    RunIntent {
        input: args.input,
        output: args.output,
        colored,
    }
}

fn resolve_colored(choice: &ColorWhen) -> bool {
    let no_color = std::env::var_os("NO_COLOR").is_some();
    let isatty = std::io::IsTerminal::is_terminal(&std::io::stderr());
    resolve_colored_with(choice, no_color, isatty)
}

/// Função pura: decide colored conforme choice, env, isatty.
///
/// Separada para testabilidade — sem acesso a env ou isatty.
pub fn resolve_colored_with(choice: &ColorWhen, no_color: bool, isatty: bool) -> bool {
    match choice {
        ColorWhen::Never  => false,
        ColorWhen::Always => true,
        ColorWhen::Auto   => !no_color && isatty,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn never_sempre_falso() {
        assert_eq!(resolve_colored_with(&ColorWhen::Never, false, true), false);
        assert_eq!(resolve_colored_with(&ColorWhen::Never, true, true), false);
    }
    
    #[test]
    fn always_sempre_verdadeiro() {
        assert_eq!(resolve_colored_with(&ColorWhen::Always, false, false), true);
        assert_eq!(resolve_colored_with(&ColorWhen::Always, true, false), true);
    }
    
    #[test]
    fn auto_com_tty_sem_no_color() {
        assert_eq!(resolve_colored_with(&ColorWhen::Auto, false, true), true);
    }
    
    #[test]
    fn auto_sem_tty() {
        assert_eq!(resolve_colored_with(&ColorWhen::Auto, false, false), false);
    }
    
    #[test]
    fn auto_com_no_color() {
        assert_eq!(resolve_colored_with(&ColorWhen::Auto, true, true), false);
    }
    
    #[test]
    fn auto_no_color_e_sem_tty() {
        assert_eq!(resolve_colored_with(&ColorWhen::Auto, true, false), false);
    }
}
```

**117.C.2 — Actualizar `02_shell/src/lib.rs`**:

```rust
//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/shell.md
//! @prompt-hash <mantido>
//! @layer L2
//! @updated 2026-04-23

pub mod cli;
```

**117.C.3 — Actualizar `02_shell/Cargo.toml`**:

```toml
[dependencies]
typst-core = { path = "../01_core" }
anyhow     = { workspace = true }
clap       = { workspace = true }
```

**117.C.4 — Remover de L3**:

Em `03_infra/src/diagnostic_format.rs`:

- Remover enum `ColorWhen`.
- Remover função `resolve_colored_with`.
- Remover testes `resolve_colored_*`.
- Manter: constantes ANSI, `format_diagnostic`,
  `drain_diagnostics_to_stderr`, testes de formatação
  (`format_diagnostic_*`).
- Remover `use clap::...` se existe.

Em `03_infra/Cargo.toml`:

- Remover `clap = { workspace = true }`.

**117.C.5 — Actualizar L4**:

Em `04_wiring/src/main.rs`:

```rust
//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/wiring.md
//! @prompt-hash <novo>
//! @layer L4
//! @updated 2026-04-23

use std::fs;
use std::process::ExitCode;
use typst_shell::cli::{self, RunIntent};
use typst_infra::diagnostic_format::drain_diagnostics_to_stderr;
use typst_infra::pipeline::compile_to_pdf_bytes;
use typst_infra::system_world::SystemWorld;

fn main() -> ExitCode {
    let RunIntent { input, output, colored } = cli::parse();
    
    let root = input.parent().unwrap_or_else(|| std::path::Path::new(".")).to_path_buf();
    let world = match SystemWorld::new(&root, &input) {
        Ok(w) => w,
        Err(e) => {
            eprintln!("error: {}", e);
            return ExitCode::from(2);
        }
    };
    
    let source = world.source(world.main()).expect("main source should load");
    let source_path = input.display().to_string();
    
    let (result, warnings) = compile_to_pdf_bytes(&world, &source);
    drain_diagnostics_to_stderr(&warnings, &source, &source_path, colored);
    
    match result {
        Ok(pdf_bytes) => match fs::write(&output, pdf_bytes) {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("error: failed to write output: {}", e);
                ExitCode::from(2)
            }
        },
        Err(errors) => {
            drain_diagnostics_to_stderr(&errors, &source, &source_path, colored);
            ExitCode::from(1)
        }
    }
}
```

Em `04_wiring/Cargo.toml`:

- Remover `clap = { workspace = true }` (se não é usado
  directamente).
- Manter deps de typst-core, typst-shell, typst-infra, anyhow.

Verificar antes de remover: `cargo build -p typst-wiring` compila
sem `clap` directo.

**117.C.6 — Prompts L0**:

Criar `00_nucleo/prompts/shell/cli.md` — descreve o módulo,
`Args`, `ColorWhen`, `RunIntent`, `parse`, `resolve_colored_with`.

Actualizar `00_nucleo/prompts/shell.md` — aponta para
sub-prompt `cli.md`.

Actualizar `00_nucleo/prompts/infra/diagnostic_format.md` —
remove referências a `ColorWhen` (movido para L2).

Actualizar `00_nucleo/prompts/wiring.md` — descreve L4 como
composição thin que consome `cli::parse()`.

`crystalline-lint --fix-hashes .` para actualizar hashes.

### 117.D — Testes

1. **Testes `resolve_colored_*` movem para L2**: o código do teste
   é idêntico, só muda o módulo onde vivem.
2. **Testes de L3**: `format_diagnostic_*` continuam em L3, inalterados.
3. **Testes L4 (114)**: verificar que passam. Provável: sim, porque
   o binário tem mesmo comportamento.
4. **Novo teste em L2**: opcional, `parse()` unit test é complicado
   (requer mutar argv global). `resolve_colored_with` cobre
   testabilidade suficiente.

Contagens esperadas:

- L1: 811 (inalterado).
- L2: **0 → N** (6 testes de `resolve_colored_*` movem para cá).
- L3: 207 → **207 − 6 = 201** (perde os 6 testes movidos).
- L4: 5 (inalterado).

Total testes workspace: 811 + 6 + 201 + 5 = **1023**. Antes:
811 + 207 + 5 = 1023. **Igual** — redistribuição, não mudança de
total.

### 117.E — Encerramento

1. `cargo build --release` passa.
2. `cargo test --workspace` passa, total inalterado (1023),
   distribuição diferente.
3. `crystalline-lint .` zero violations (V12 não dispara — L3 sem
   `clap`; L4 thin).
4. Validação manual: binário comporta-se exactamente como no Passo
   116 (cores em tty, sem cores em pipe, `--color` funciona,
   `NO_COLOR` respeitado).
5. ADR-00NN promovida.
6. ADRs 0047 e 0048 ganham nota de "corrigido parcialmente pela
   ADR-00NN — camada muda, decisões base mantidas".
7. Relatório `typst-passo-117-relatorio.md`:
   - Decisão de localização (cli.rs ou lib.rs).
   - Linhas movidas exactas.
   - Diff de deps (clap sai de L3/L4, entra em L2).
   - Linhas finais de `04_wiring/src/main.rs`.
   - Limitações aceites.

---

## Critério de conclusão

Todas em conjunto:

1. Inventário 117.A escrito.
2. ADR-00NN criada e promovida.
3. `02_shell` tem conteúdo real (cli module ou em lib.rs).
4. `ColorWhen`, `resolve_colored_with`, `Args`, `RunIntent`,
   `parse()` em L2.
5. `03_infra` não tem `clap` em Cargo.toml.
6. `04_wiring/src/main.rs` ≤ 35 linhas úteis (excluindo header).
7. `04_wiring/Cargo.toml` sem `clap` directo (excepto se necessário).
8. Prompts L0 actualizados com hashes corretos.
9. `cargo build --release` passa.
10. `cargo test --workspace` passa com contagem total inalterada.
11. `crystalline-lint .` zero violations.
12. Validação manual: comportamento externo inalterado.
13. Relatório 117.E escrito.

---

## O que pode sair errado

- **`cargo test` falha durante a migração**: compilação pode
  partir a meio. Aceitar — correr `cargo check -p <crate>` após
  cada sub-passo. No fim do 117.C.5, tudo deve compilar.
- **L4 ainda precisa de `clap` directamente**: se algum uso
  sobreviver (`use clap::Parser`), cargo clippy avisa. Remover ou
  documentar. Objectivo: L4 fica cego a `clap`.
- **`V12` dispara em L4**: não deveria — L4 fica thin. Se disparar,
  o código migrado não era só `Args`/`resolve_*` — havia lógica
  implícita que foi esquecida. Investigar.
- **Prompts hash desactualizado**: depois de mover código, os
  hashes de `shell.md`, `diagnostic_format.md`, `wiring.md` podem
  estar out of sync. `--fix-hashes` resolve. Se um hash aparece
  órfão (prompt ficou vazio), remover o prompt ou manter com
  nota.
- **Test count diverge**: se alguém criou teste novo em L3 entre
  Passos 116 e 117 que não estava esperado, a contagem muda.
  Inventário 117.A confirma base.
- **ADRs 0047/0048 "corrigidas" vs "revogadas"**: escolha é
  parcial. Decisões base (clap, cores, paleta) mantêm-se; decisão
  de camada muda. Se a auditoria DEBT futura considerar ADR "parcialmente
  corrigida" confusa, revogar pode ser mais limpo. Aceitar
  correcção parcial neste passo.
- **`typst-shell` torna-se dep pesada**: traz `clap`. Mas `typst-shell`
  já é dep de `typst-wiring`, que é o binário final. Zero mudança
  no tamanho do binário.
- **`Args::parse()` vs `clap::Parser::parse()`**: em L2, `Args::parse()`
  funciona via trait `Parser` em scope (`use clap::Parser`). Não
  re-exportar `Parser` em `lib.rs` — fica interno.

---

## Notas operacionais

- Este é **primeiro passo que corrige ADRs anteriores**. Padrão
  novo para o projecto: ADR que corrige decisão de camada sem
  revogar decisão funcional.
- L4 fica com **15-25 linhas de main(). Qualquer linha adicional no
  futuro deve justificar-se; se L4 crescer, é sinal de que lógica
  escapou para lá.
- L2 é agora sítio activo. Próximos flags de CLI (`-o`, `--root`,
  `--font-path`) vão aqui — não para L4, não para L3. O padrão
  fica estabelecido.
- `crystalline-lint` é fonte de verdade para violations de
  camada. V12 continua a ser o detector — este passo deixa L4
  limpo para que V12 não dispare mesmo para flags futuras.
- Se em 117.A o inventário revelar que L2 já tinha algum
  conteúdo (improvável), integrar em vez de sobrescrever. O
  cli.rs/lib.rs pode coexistir com outros módulos futuros.
