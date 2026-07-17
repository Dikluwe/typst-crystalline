---
# P735 — Namespaces `emoji` e `pdf` ausentes

> **Passo:** 735
> **Data:** 2026-07-10
> **Foco:** P731 confirmou que `emoji` e `pdf` não existem no scope global do cristalino ("unknown variable"), enquanto o vanilla os expõe como `module`. Sem consumidor conhecido em `cetz`, mas é uma lacuna genuína de linguagem.
> **Tipo:** Sonda + Implementação.
> **Tamanho:** M — depende do conteúdo de cada namespace.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P731 (onde a ausência foi confirmada), P709 (`std`, mesmo padrão de `Value::Module`).

---

## Sonda

### Confirmar o conteúdo completo de cada namespace no vanilla

```bash
cat > /tmp/p735-emoji.typ <<'EOF'
#emoji.face
#emoji.grinning
EOF
lab/typst-original/target/release/typst compile /tmp/p735-emoji.typ /tmp/p735-emoji-vanilla.pdf
```

```bash
grep -n "pub fn\|pub const" lab/typst-original/crates/typst-library/src/foundations/pdf.rs 2>/dev/null | head -20
grep -rn "\"emoji\"\|\"pdf\"" lab/typst-original/crates/typst-library/src/lib.rs | head -10
```

Confirmar: `emoji` é uma colecção de símbolos (semelhante a `sym`, já implementado)? `pdf` é um módulo com funções específicas de PDF (tags, metadados)? Confirmar o tamanho real de cada um antes de decidir implementar tudo de uma vez ou faseado.

### Critério de fecho da sonda

- [ ] Conteúdo completo de `emoji` confirmado (provavelmente grande — símbolos emoji).
- [ ] Conteúdo completo de `pdf` confirmado.
- [ ] Decisão registada: implementar ambos neste passo, ou faseado (com razão medida, não estimada).

---

## Implementação

Implementar os namespaces confirmados, seguindo o padrão já estabelecido para `sym` (P471) e `std`/`calc`/`sys`/`math` (P709/P731) — `Value::Module`.

### Critério de fecho da implementação

- [ ] Namespaces implementados conforme decidido pela sonda.
- [ ] `type(emoji)`/`type(pdf)` devolvem `module`, igual ao vanilla.

---

## Validação

```bash
./target/release/typst /tmp/p735-emoji.typ /tmp/p735-emoji-depois.pdf
```

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Sonda completa, conteúdo confirmado.
- [ ] Implementado e testado.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p735.md`, com hash do commit.
- [ ] Item marcado como fechado em `achados-adiados-cetz.md`.
