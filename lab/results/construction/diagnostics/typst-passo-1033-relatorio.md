# Passo 1033 — Relatório: `--version` do binário reflecte o commit compilado

> **Estado: COMPLETO.** `02_shell/build.rs` corrigido para reexecutar quando `HEAD`/refs git mudam.
> **Build:** `cargo build --release -p typst-wiring` ok.
> **Testes:** `cargo test --workspace` ok (~5855 passed, 0 failed).
> **Lint:** `crystalline-lint .` → 0 violations.

---

## Problema

`02_shell/build.rs` só declarava `cargo:rerun-if-env-changed=TYPST_COMMIT_SHA`. O Cargo só
reexecuta o script quando essa variável de ambiente muda; mudanças no HEAD git não a
provocam. Resultado: após `git commit`/`git checkout`, um `cargo build` incremental podia
manter o hash do commit anterior em `--version`.

Reproduzido no Passo 1031: build em HEAD `4f64e4e69` imprimiu `typst 0.15.0 (0f8487b9)` —
hash errado.

## Causa

A lógica de `build.rs` copiada do vanilla (`lab/typst-original/crates/typst-utils/build.rs`)
assume que `TYPST_COMMIT_SHA` é definida externamente em builds de release. No fluxo de
passos do cristalino, onde rebuilds frequentes em HEADs diferentes são normais, essa
premissa não segura o hash.

## Correcção

Adicionadas em `02_shell/build.rs` instruções de rerun para os ficheiros/directórios git que
determinam o HEAD:

```rust
println!("cargo:rerun-if-changed=../.git/HEAD");
println!("cargo:rerun-if-changed=../.git/refs");
```

Isto é aditivo: não muda o output do binário, só força o Cargo a reexecutar `build.rs` quando
o HEAD muda.

## Validação

### Antes da correcção

```text
$ ./target/release/typst --version
typst 0.15.0 (0f8487b9)   # hash errado, de um HEAD anterior
```

### Após a correcção

1. Build no HEAD `29a75805c`:

```text
$ cargo build --release -p typst-wiring
$ ./target/release/typst --version
typst 0.15.0 (29a75805c)
```

2. Criar commit vazio para mudar o HEAD (mesma árvore), rebuild sem `cargo clean`:

```text
$ git commit --allow-empty -m 'P1033: teste de versionamento do build'
$ cargo build --release -p typst-wiring
$ ./target/release/typst --version
typst 0.15.0 (99992561a)   # hash do novo commit
```

3. Remover o commit de teste e rebuildar no HEAD real:

```text
$ git reset --soft HEAD~1
$ cargo build --release -p typst-wiring
$ ./target/release/typst --version
typst 0.15.0 (29a75805c)   # volta ao HEAD actual
```

## Ficheiros alterados

- `02_shell/build.rs` — adicionados `cargo:rerun-if-changed=../.git/HEAD` e
  `cargo:rerun-if-changed=../.git/refs`.
- `AGENTS.md` e `CLAUDE.md` — actualizada a secção "Referência de paridade — qual binário,
  qual fonte" para incluir a quarta armadilha sobre `--version` preso a HEAD antigo.

## Nota de proveniência

Até todos os binários em uso serem produzidos por rebuilds limpos pós-P1033, a proveniência
de medições continua a usar **HEAD + estado da árvore**, nunca `--version` sozinho.
