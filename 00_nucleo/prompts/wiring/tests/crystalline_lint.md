# Prompt L0 — `wiring/tests/crystalline_lint` — whitelist type-level
Hash do Código: 687b9f29


**Camada**: L4
**Ficheiro alvo**: `04_wiring/tests/crystalline_lint.rs`
**Criado em**: 2026-08-26 (P1198; individualização de `wiring.md`)
**ADRs**: ADR-0129

---

## Medição antes da decisão

Esta suíte não executa o binário `typst`. Ela cria projetos cristalinos
mínimos e invoca `crystalline-lint` pelo PATH para verificar exclusivamente a
whitelist type-level V14. Portanto, não compartilha os observáveis CLI de
`main.rs` e não consome o núcleo `wiring/cli-observables`.

## Responsabilidade

Validar em integração que `[l1_allowed_external.<crate>].types` distingue
tipos autorizados e proibidos de uma crate externa já autorizada.

O fixture mínimo declara L0/L1, gramática Rust, porta `entities`, severidade
V14 e allowlist de `ecow` com `EcoString` e `EcoVec`.

## Cenários

Um source L1 com `use ecow::EcoMap;` deve produzir output contendo `V14` e o
pacote normativo `'ecow'`. A mensagem atual do linter identifica o pacote
externo, não repete o path integral do item proibido. Um source L1 com
`use ecow::EcoString;` não deve produzir `V14`.

Os testes combinam stdout e stderr porque a posição do relatório entre streams
é detalhe do executável do linter, não contrato desta suíte.

## Harness

Cada projeto temporário inclui PID no nome, recria os diretórios esperados pelo
walker de prompts, escreve `crystalline.toml` mínimo e remove a árvore ao fim.
O binário é resolvido como `crystalline-lint` no PATH.

## Critérios de verificação

- `cargo test -p typst-wiring --test crystalline_lint` passa os dois cenários.
- O caso proibido contém V14 e o pacote `ecow` na mensagem.
- O caso autorizado permanece sem V14.
- A alteração de linhagem do P1198 não modifica corpos Rust.

## Histórico de revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-06-24 | P440 — integração da whitelist type-level V14 | `tests/crystalline_lint.rs` |
| 2026-08-26 | P1198 — owner independente, sem núcleo de observáveis CLI | `wiring/tests/crystalline_lint.md`, `tests/crystalline_lint.rs` |
| 2026-08-26 | P1198 RED→GREEN — oracle textual alinhado à mensagem normativa por pacote do linter atual | `wiring/tests/crystalline_lint.md`, `tests/crystalline_lint.rs` |
