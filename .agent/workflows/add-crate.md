---
description: Como adicionar um novo crate ao workspace
---

# Add New Crate Workflow

Siga este workflow ao criar um novo crate no projeto Typst.

## Passos

1. **Determine a camada correta** baseado na responsabilidade do crate:
   - `01_core/` - Funcionalidade fundamental do compilador
   - `02_shell/` - Interfaces de usuário (CLI, IDE)
   - `03_infra/` - Exportadores, ferramentas, infraestrutura
   - `04_wiring/` - Orquestração e fachadas

2. **Crie o crate na pasta correta**:
```bash
cd <camada>/
cargo new --lib typst-<nome>
```

3. **Atualize o workspace Cargo.toml raiz** (`./Cargo.toml`):

   a. Adicione em `[workspace.members]`:
   ```toml
   members = [
       # ... existing
       "<camada>/typst-<nome>",
   ]
   ```

   b. Adicione em `[workspace.dependencies]`:
   ```toml
   typst-<nome> = { path = "<camada>/typst-<nome>", version = "0.14.1" }
   ```

4. **Configure o Cargo.toml do novo crate** (`<camada>/typst-<nome>/Cargo.toml`):
```toml
[package]
name = "typst-<nome>"
version = { workspace = true }
edition = { workspace = true }
# ... outros campos workspace

[dependencies]
# Use workspace dependencies
typst-utils = { workspace = true }

[lints]
workspace = true
```

// turbo
5. **Verifique o build**:
```bash
cargo check --workspace
```

## Regras de Dependência

- Crates em `01_core/` NÃO devem depender de outras camadas
- Crates em `02_shell/` podem depender de `01_core/` e `04_wiring/`
- Crates em `03_infra/` podem depender de `01_core/`
- Crates em `04_wiring/` podem depender de qualquer camada
