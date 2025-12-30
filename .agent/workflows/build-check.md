---
description: Verificar integridade do build após modificações
---

# Build Check Workflow

Após qualquer modificação de código Rust no projeto, execute este workflow para garantir a integridade do build.

## Passos

1. Verificar tipos e dependências:
```bash
cargo check --workspace
```

2. Verificar formatação:
```bash
cargo fmt --check
```

// turbo
3. Verificar lints (opcional mas recomendado):
```bash
cargo clippy --workspace --all-targets
```

## Interpretando Resultados

- **"manifest not found"**: Caminho incorreto em `Cargo.toml` - verifique `[workspace.members]` ou `[workspace.dependencies]`
- **Dependência cíclica**: Verifique se a direção do fluxo de dependências está correta (01→02/03→04)
- **Unresolved import**: Crate movido mas path não atualizado
