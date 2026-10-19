---
# P599 — Completar a validação de P598

> **Passo:** 599
> **Data:** 2026-07-05
> **Foco:** P598 só correu `cargo test -p typst-core`, não `cargo test --workspace`, e nunca mencionou `crystalline-lint .`. As duas alterações de P598 (`layout_types.rs`, `set_page.rs`) são consumidas por outros crates. Este passo completa a validação que faltou, antes de aceitar P598 como fechado.
> **Tipo:** Verificação directa.
> **Tamanho:** XS.
> **ADR-0108 EM VIGOR.** A validação completa não é opcional nem substituível por uma parte dela.
> **Dependências:** P598 (onde a validação ficou incompleta).

---

## Verificação

```bash
cargo test --workspace
crystalline-lint .
```

### Critério de fecho

- [ ] Resultado completo de `cargo test --workspace`, todos os crates, não só `typst-core`.
- [ ] Resultado de `crystalline-lint .`.
- [ ] Se houver alguma falha em `typst-infra` ou noutro crate consumidor de `layout_types`/`set_page`: investigada e corrigida, ou registada como achado novo, não ignorada.

---

## Se tudo passar

Confirmar P598 como realmente fechado, com a validação completa desta vez, não com a parcial que tinha.

## Se algo falhar

Não é para desfazer a correcção de P598 — a fórmula está confirmada com o código fonte do vanilla, isso não muda. É para encontrar o que quebrou noutro crate, com a mesma disciplina já estabelecida nesta sequência inteira: sonda, causa, correcção, não suposição.

---

## Critério de fecho do passo

- [ ] `cargo test --workspace` completo, com resultado real, não parcial.
- [ ] `crystalline-lint .` corrido e registado.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p599.md`, com hash do commit.
