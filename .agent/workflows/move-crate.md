---
description: Como mover um crate entre camadas
---

# Move Crate Workflow

Siga este workflow quando precisar mover um crate de uma camada para outra.

⚠️ **ATENÇÃO**: Este é um workflow crítico que afeta a integridade do build!

## Passos

1. **Verifique se a movimentação é válida** verificando as dependências:
```bash
cargo tree -p typst-<nome> --invert
```

2. **Execute o git mv**:
```bash
git mv <camada_origem>/typst-<nome> <camada_destino>/
```

3. **Atualize `[workspace.members]`** no `./Cargo.toml`:
```toml
# Remova ou atualize a linha antiga
# Adicione o novo caminho
"<camada_destino>/typst-<nome>",
```

4. **Atualize `[workspace.dependencies]`** no `./Cargo.toml`:
```toml
typst-<nome> = { path = "<camada_destino>/typst-<nome>", version = "0.14.1" }
```

// turbo
5. **Valide o build**:
```bash
cargo check --workspace
```

6. **Commit as mudanças**:
```bash
git add -A
git commit -m "refactor: move typst-<nome> from <origem> to <destino>"
```

## Nota Importante

Os crates individuais usam `{ workspace = true }` para dependências, então geralmente NÃO precisam de alterações após a movimentação. Todo o roteamento é controlado pelo `Cargo.toml` raiz.
