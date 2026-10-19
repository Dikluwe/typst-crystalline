---
# P739 — Quatro itens pequenos e independentes: `hline`/`vline` `stroke: none`, `line(end:)`, formatação de `Float`, `Length / Float` com NaN

> **Passo:** 739
> **Data:** 2026-07-10
> **Foco:** Os quatro últimos itens de baixa prioridade da lista de controlo, todos pequenos e sem dependência entre si. Agrupados num só passo por serem independentes, não porque estejam relacionados — cada um tem a sua própria sonda e implementação, e cada um pode ser adiado individualmente se a sonda revelar mais complexidade do que esperado.
> **Tipo:** Sonda + Implementação, quatro sub-partes independentes.
> **Tamanho:** M no total (cada sub-parte é XS-S).
> **ADR-0108 EM VIGOR.**
> **Dependências:** P726 (`hline`/`vline`, `line(end:)`), P713 (formatação de `Float`), P725 (saneamento de NaN em `Mul`, a espelhar em `Div`).

---

## Parte A — `hline`/`vline` com `stroke: none`

### Sonda

```bash
cat > /tmp/p739a-hline.typ <<'EOF'
#table(
  columns: 2,
  table.hline(stroke: none),
  [a], [b],
)
EOF
lab/typst-original/target/release/typst compile /tmp/p739a-hline.typ /tmp/p739a-vanilla.pdf
echo "Exit code: $?"
./target/release/typst /tmp/p739a-hline.typ /tmp/p739a-cristalino.pdf
echo "Exit code: $?"
```

Confirmar se a correcção exige mesmo `Option<Stroke>` na entidade (P726 já apontou isto), ou se há um atalho mais simples.

### Implementação

Se o custo confirmado for razoável: alterar `GridHLineElem`/`TableHLineElem` (+vlines) para `Option<Stroke>`, com salto no render quando `None`.

### Critério de fecho da Parte A

- [ ] Sonda confirma o custo real (não estimado).
- [ ] Implementado se o custo for razoável, ou scope-out reforçado se não for.

---

## Parte B — `line(end:)`

### Sonda

```bash
cat > /tmp/p739b-line-end.typ <<'EOF'
#line(start: (0pt, 0pt), end: (50pt, 50pt))
EOF
lab/typst-original/target/release/typst compile /tmp/p739b-line-end.typ /tmp/p739b-vanilla.pdf
./target/release/typst /tmp/p739b-line-end.typ /tmp/p739b-cristalino.pdf
echo "Exit code: $?"
```

### Implementação

Adicionar `end:` como argumento nomeado alternativo a `angle:`/`length:` em `native_line`.

### Critério de fecho da Parte B

- [ ] `line(end:)` funciona, testado com diff de pixels contra o vanilla.

---

## Parte C — Formatação de `Float`

### Sonda

```bash
cat > /tmp/p739c-float.typ <<'EOF'
#(4/2)
#(1.0)
#(2.5)
EOF
lab/typst-original/target/release/typst compile /tmp/p739c-float.typ /tmp/p739c-vanilla.pdf
pdftotext /tmp/p739c-vanilla.pdf -
./target/release/typst /tmp/p739c-float.typ /tmp/p739c-cristalino.pdf
pdftotext /tmp/p739c-cristalino.pdf -
```

Confirmar a regra exacta do vanilla (provavelmente: inteiros exactos mostram-se sem `.0`, mas `1.0` explícito também mostra sem `.0` — confirmar isto com cuidado, pode ser mais subtil do que parece).

### Implementação

Corrigir `repr_value`/formatação de display para `Float`, seguindo a regra confirmada.

### Critério de fecho da Parte C

- [ ] Regra exacta confirmada e implementada.

---

## Parte D — `Length / Float` com NaN

### Sonda

```bash
cat > /tmp/p739d-nan.typ <<'EOF'
#(1pt / float.nan)
EOF
lab/typst-original/target/release/typst compile /tmp/p739d-nan.typ /tmp/p739d-vanilla.pdf 2>&1
```

Confirmar se isto é sequer alcançável por sintaxe de utilizador (P725 registou que não era, para `Mul`) — se `float.nan` não existir como valor acessível, confirmar de outra forma (por exemplo, `0.0/0.0`).

### Implementação

Se alcançável: espelhar o saneamento de NaN já feito em `Mul` (P725) para `Div`.

### Critério de fecho da Parte D

- [ ] Confirmado se é alcançável; implementado se sim, scope-out confirmado se não.

---

## Validação final (todas as partes)

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Cada uma das quatro partes tratada individualmente — implementada ou scope-out confirmado com razão medida.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p739.md`, com hash do commit.
- [ ] Cada item actualizado individualmente em `achados-adiados-cetz.md`.
