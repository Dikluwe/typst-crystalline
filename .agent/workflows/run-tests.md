---
description: Rodar suíte de testes do projeto
---

# Run Tests Workflow

Workflow para executar os diferentes níveis de testes do projeto Typst.

## Testes Rápidos (Unitários)

// turbo
1. Executar testes unitários de todos os crates:
```bash
cargo test --workspace --lib
```

## Testes Completos

2. Executar todos os testes incluindo integração:
```bash
cargo test --workspace
```

## Testes de Crate Específico

3. Testar apenas um crate:
```bash
cargo test -p typst-<nome>
```

## Testes de Fuzzing

4. Executar fuzz testing (requer nightly):
```bash
cd 03_infra/fuzz
cargo +nightly fuzz run parse
```

## Atualizar Snapshots de Referência

5. Se testes de renderização falharem e as mudanças forem intencionais:
```bash
cd tests
cargo run -- --update
```

## Notas

- Testes de renderização comparam com imagens em `tests/ref/`
- Alguns testes requerem fontes específicas (typst-assets)
- Fuzz tests são para encontrar crashes, não para CI regular
