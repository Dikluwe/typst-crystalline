---
# P732 — Fallback de stroke default em `polygon` (mesmo defeito de `curve`, P727)

> **Passo:** 732
> **Data:** 2026-07-10
> **Foco:** `native_polygon` (`shapes.rs:348-351`) tem o mesmo defeito que `native_curve` tinha antes de P727 — não aplica o fallback `Smart::Auto` do vanilla (sem fill nem stroke → stroke preto 1pt; com fill sem stroke → sem stroke). Encontrado por inspecção de código durante P727, nunca confirmado com um caso medido de pixels.
> **Tipo:** Sonda mínima + Implementação. Correcção pequena, mesmo padrão já estabelecido.
> **Tamanho:** S.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P727 (onde o mesmo defeito foi corrigido em `curve`, padrão a replicar).

---

## Sonda mínima

### Confirmar o bug com um caso medido de pixels, não só por inspecção de código

```bash
cat > /tmp/p732-polygon.typ <<'EOF'
#polygon((0pt, 0pt), (50pt, 0pt), (25pt, 40pt))
EOF
lab/typst-original/target/release/typst compile /tmp/p732-polygon.typ /tmp/p732-vanilla.pdf
mutool draw -o /tmp/p732-vanilla.png -r 150 /tmp/p732-vanilla.pdf
./target/release/typst /tmp/p732-polygon.typ /tmp/p732-cristalino.pdf
mutool draw -o /tmp/p732-cristalino.png -r 150 /tmp/p732-cristalino.pdf
```

Confirmar com contagem de pixels não-brancos (mesmo método já usado em P727) se o cristalino produz página em branco ou uma forma visível mas com defeito diferente.

### Confirmar o caso "com fill, sem stroke"

```bash
cat > /tmp/p732-polygon-fill.typ <<'EOF'
#polygon(fill: red, (0pt, 0pt), (50pt, 0pt), (25pt, 40pt))
EOF
lab/typst-original/target/release/typst compile /tmp/p732-polygon-fill.typ /tmp/p732-fill-vanilla.pdf
./target/release/typst /tmp/p732-polygon-fill.typ /tmp/p732-fill-cristalino.pdf
```

### Critério de fecho da sonda mínima

- [ ] Bug confirmado com contagem de pixels, não só inspecção de código.
- [ ] Caso "com fill, sem stroke" confirmado.

---

## Implementação

Aplicar exactamente o mesmo fallback já implementado em `native_curve` (P727) a `native_polygon`.

### Critério de fecho da implementação

- [ ] `polygon` sem fill nem stroke produz stroke preto 1pt, testado com diff de pixels.
- [ ] `polygon` com fill sem stroke produz sem stroke, testado.
- [ ] `polygon` com stroke explícito sem regressão.

---

## Validação

```bash
./target/release/typst /tmp/p732-polygon.typ /tmp/p732-depois.pdf
mutool draw -o /tmp/p732-depois.png -r 150 /tmp/p732-depois.pdf
```

Diff de pixels contra o vanilla.

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Sonda mínima completa, com diff de pixels.
- [ ] Implementado e testado.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p732.md`, com hash do commit.
- [ ] Item marcado como fechado em `achados-adiados-cetz.md`.
