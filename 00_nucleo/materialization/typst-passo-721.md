---
# P721 — Corrigir `repr_value`: repr Typst, não Debug do Rust

> **Passo:** 721
> **Data:** 2026-07-10
> **Foco:** `repr_value` (`rules/eval/repr.rs:59-66`) formata `Length`/`Ratio`/`Angle`/`Color`/`Stroke`/`Align` (entre outros) com `{:?}` do Rust quando embutidos directamente em markup via `#expr`, em vez do repr Typst (`"6pt"`, `"50%"`, etc.). Encontrado em P710, reconfirmado em P711, prometido como "próximo passo" duas vezes e não retomado desde então — esta lista de controlo existe precisamente para não deixar isto acontecer de novo. Prioridade alta.
> **Tipo:** Sonda + Implementação.
> **Tamanho:** M — vários tipos a corrigir, cada um com formato próprio.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P710/P711 (onde foi encontrado, duas vezes, sem nunca ser fechado).

---

## Sonda

### Confirmar o alcance completo do problema

```bash
cat > /tmp/p721-repr.typ <<'EOF'
#(6pt)
#(50%)
#(45deg)
#(rgb("#ff0000"))
#(stroke(paint: red, thickness: 2pt))
#(left)
#(1fr)
EOF
lab/typst-original/target/release/typst compile /tmp/p721-repr.typ /tmp/p721-vanilla.pdf
pdftotext /tmp/p721-vanilla.pdf -
```

Confirmar o formato exacto de cada tipo, não assumir.

### Confirmar o estado actual do cristalino

```bash
./target/release/typst /tmp/p721-repr.typ /tmp/p721-cristalino.pdf
pdftotext /tmp/p721-cristalino.pdf -
```

### Localizar o código exacto

```bash
grep -n "fn repr_value" 01_core/src/engine/eval/repr.rs
```

Confirmar quais tipos já têm braço próprio no `repr_value`, e quais caem no fallback `{:?}`.

### Critério de fecho da sonda

- [ ] Formato exacto de cada tipo confirmado contra o vanilla.
- [ ] Lista completa de tipos afectados pelo fallback `{:?}` confirmada, com `file:line`.

---

## Implementação

Adicionar braços próprios em `repr_value` para cada tipo confirmado pela sonda, produzindo o repr Typst correcto em vez do fallback `{:?}`.

### Critério de fecho da implementação

- [ ] Cada tipo confirmado pela sonda produz o repr correcto.
- [ ] Tipos já correctos antes deste passo sem regressão.

---

## Validação

```bash
./target/release/typst /tmp/p721-repr.typ /tmp/p721-depois.pdf
pdftotext /tmp/p721-depois.pdf -
```

Comparar com o vanilla já obtido na sonda.

```bash
cargo test --workspace
crystalline-lint .
```

### Confirmar que os documentos de P710-720 que mostravam o bug agora mostram o formato certo

```bash
cat > /tmp/p721-regressao.typ <<'EOF'
#set text(size: 12pt)
#context [#(10em).to-absolute()]
EOF
./target/release/typst /tmp/p721-regressao.typ /tmp/p721-regressao.pdf
pdftotext /tmp/p721-regressao.pdf -
```

Deve mostrar `"120pt"`, não `Length { abs: Abs(120.0), em: 0.0 }`.

---

## Critério de fecho do passo

- [ ] Sonda completa, alcance confirmado.
- [ ] Todos os tipos corrigidos, testados contra o vanilla.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p721.md`, com hash do commit.
- [ ] Item marcado como fechado em `achados-adiados-cetz.md`, com o número deste passo.
