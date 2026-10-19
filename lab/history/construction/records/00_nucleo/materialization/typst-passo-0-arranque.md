# Passo 0 — Arranque da migração Typst

## Contexto

Este é o passo inicial da migração do Typst para a Arquitetura
Cristalina. Ler ADR-0001 antes de executar qualquer tarefa.

O objectivo deste passo é apenas criar a estrutura base —
nenhum código Typst é movido ainda.

---

## Pré-condições

- Estamos no branch de migração (ex: `cristalino/migration`)
- O código Typst original está na raiz do repositório (crates/, cli/, etc.)
- `crystalline-lint` está instalado e acessível no PATH

Verificar:
```bash
crystalline-lint --version
git branch --show-current
ls crates/    # deve listar typst-syntax, typst-eval, etc.
```

---

## Tarefa 1 — Criar estrutura de directórios

```bash
# Camadas cristalinas
mkdir -p 00_nucleo/{prompts,adr}
mkdir -p 01_core/{entities,contracts,rules}
mkdir -p 02_shell
mkdir -p 03_infra
mkdir -p 04_wiring

# lab — destino do código original
mkdir -p lab/typst-original
```

---

## Tarefa 2 — Mover código original para lab/

```bash
# Mover tudo excepto o que vai ficar na raiz
mv crates/      lab/typst-original/
mv cli/         lab/typst-original/
mv tests/       lab/typst-original/
mv docs/        lab/typst-original/
mv assets/      lab/typst-original/

# Ficheiros de configuração do workspace original
mv Cargo.toml   lab/typst-original/Cargo.toml.original
# NÃO mover: .git/, .github/, LICENSE, NOTICE, README.md
```

**Verificar após mover:**
```bash
ls lab/typst-original/
# deve conter: crates/ cli/ tests/ docs/ assets/ Cargo.toml.original
```

---

## Tarefa 3 — Criar Cargo.toml do workspace cristalino

Criar `Cargo.toml` na raiz:

```toml
[workspace]
resolver = "2"
members = [
    "01_core",
    "02_shell",
    "03_infra",
    "04_wiring",
    # lab/typst-original não é membro do workspace cristalino
    # compile separadamente se necessário para testes de paridade
]

[workspace.package]
version     = "0.1.0"
edition     = "2021"
authors     = ["Typst GmbH <hi@typst.app>"]
license     = "Apache-2.0"
repository  = "https://github.com/[SEU_FORK]/typst"

[workspace.dependencies]
# Será preenchido conforme as crates cristalinas forem criadas
thiserror = "2"
comemo    = "0.4"   # autorizado em L1 — ver ADR-0001
anyhow    = "1"
```

---

## Tarefa 4 — Criar Cargo.toml das crates cristalinas (esqueletos)

Cada camada é uma crate separada. Criar esqueletos com `lib.rs` mínimo.

**`01_core/Cargo.toml`:**
```toml
[package]
name        = "typst-core"
description = "Pure domain logic for the Typst compiler"
version.workspace    = true
edition.workspace    = true
authors.workspace    = true
license.workspace    = true
repository.workspace = true

[dependencies]
thiserror = { workspace = true }
comemo    = { workspace = true }
```

**`01_core/src/lib.rs`:**
```rust
//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core.md
//! @prompt-hash 00000000
//! @layer L1
//! @updated 2026-03-22

// Módulos serão adicionados durante a migração
```

**`02_shell/Cargo.toml`:**
```toml
[package]
name        = "typst-shell"
description = "CLI and formatters for the Typst compiler"
version.workspace    = true
edition.workspace    = true
authors.workspace    = true
license.workspace    = true
repository.workspace = true

[dependencies]
typst-core = { path = "../01_core" }
anyhow     = { workspace = true }
```

**`02_shell/src/lib.rs`:**
```rust
//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/shell.md
//! @prompt-hash 00000000
//! @layer L2
//! @updated 2026-03-22
```

**`03_infra/Cargo.toml`:**
```toml
[package]
name        = "typst-infra"
description = "I/O implementations for the Typst compiler"
version.workspace    = true
edition.workspace    = true
authors.workspace    = true
license.workspace    = true
repository.workspace = true

[dependencies]
typst-core = { path = "../01_core" }
thiserror  = { workspace = true }
```

**`03_infra/src/lib.rs`:**
```rust
//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra.md
//! @prompt-hash 00000000
//! @layer L3
//! @updated 2026-03-22
```

**`04_wiring/Cargo.toml`:**
```toml
[package]
name        = "typst-wiring"
description = "Composition root for the Typst compiler"
version.workspace    = true
edition.workspace    = true
authors.workspace    = true
license.workspace    = true
repository.workspace = true

[[bin]]
name = "typst"
path = "src/main.rs"

[dependencies]
typst-core  = { path = "../01_core" }
typst-shell = { path = "../02_shell" }
typst-infra = { path = "../03_infra" }
anyhow      = { workspace = true }
```

**`04_wiring/src/main.rs`:**
```rust
//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/wiring.md
//! @prompt-hash 00000000
//! @layer L4
//! @updated 2026-03-22

fn main() {
    // Composição será implementada durante a migração
    println!("typst cristalino — em migração");
}
```

---

## Tarefa 5 — Criar crystalline.toml

Criar `crystalline.toml` na raiz com o conteúdo do ADR-0001
(secção "crystalline.toml inicial").

---

## Tarefa 6 — Criar ADR-0001

Copiar o ficheiro `typst-adr-0001-estrategia-migracao.md` para
`00_nucleo/adr/0001-estrategia-migracao.md`.

Criar `00_nucleo/adr/.gitkeep` se o directório ficar vazio.

---

## Tarefa 7 — Verificar build inicial

```bash
cargo build
```

Esperado: compila com zero erros. As crates cristalinas são
esqueletos vazios — devem compilar trivialmente.

---

## Tarefa 8 — Verificar linter inicial

```bash
crystalline-lint .
```

Esperado no estado actual:
- V1 (MissingPromptHeader): vai disparar para os `lib.rs` e `main.rs`
  que têm `@prompt-hash 00000000` — isto é esperado; os prompts
  ainda não existem
- V7 (OrphanPrompt): não vai disparar porque não há prompts ainda
- V8 (AlienFile): **não deve disparar** — todos os directórios
  estão mapeados em `[layers]` ou `[excluded]`

Se V8 disparar para `lab/typst-original/`, verificar se `lab`
está correctamente mapeado em `[layers]` como `lab = "lab"`.

**Se V8 disparar para qualquer ficheiro em `lab/`:**
O código em `lab/` é quarentena — V8 não deve disparar para ele
porque `lab` está mapeado. Se disparar, é um bug de configuração
no `crystalline.toml`.

**Se V8 disparar para ficheiros na raiz** (ex: `LICENSE`, `README.md`):
Adicionar a `[excluded_files]`:
```toml
[excluded_files]
license = "LICENSE"
notice  = "NOTICE"
readme  = "README.md"
```

---

## Tarefa 9 — Corrigir hashes

```bash
crystalline-lint --fix-hashes .
```

Isto vai tentar resolver os `@prompt-hash 00000000`. Como os prompts
ainda não existem, vai reportar entradas não corrigíveis — esperado.
Criar os prompts mínimos antes de correr este comando:

```bash
# Prompts mínimos para os lib.rs
echo "# Core — typst-core\n\nEm migração." > 00_nucleo/prompts/core.md
echo "# Shell — typst-shell\n\nEm migração." > 00_nucleo/prompts/shell.md
echo "# Infra — typst-infra\n\nEm migração." > 00_nucleo/prompts/infra.md
echo "# Wiring — typst-wiring\n\nEm migração." > 00_nucleo/prompts/wiring.md

crystalline-lint --fix-hashes .
crystalline-lint .
```

---

## Estado esperado ao fim do Passo 0

```
typst/
├── 00_nucleo/
│   ├── adr/
│   │   └── 0001-estrategia-migracao.md
│   └── prompts/
│       ├── core.md
│       ├── shell.md
│       ├── infra.md
│       └── wiring.md
├── 01_core/src/lib.rs
├── 02_shell/src/lib.rs
├── 03_infra/src/lib.rs
├── 04_wiring/src/main.rs
├── lab/
│   └── typst-original/
│       ├── crates/
│       ├── cli/
│       ├── tests/
│       └── ...
├── Cargo.toml          ← workspace cristalino
└── crystalline.toml
```

```bash
cargo build             # ✓ zero erros
crystalline-lint .      # violations apenas de V1 (hashes a corrigir)
                        # zero V8 — estrutura de camadas correcta
```

Quando este estado estiver estável, avançar para o Passo 1:
migração dos tipos de domínio de `typst-syntax` para `01_core/entities/`.
