---
# P687 — Cobertura completa de cores nomeadas no scope global

> **Passo:** 687
> **Data:** 2026-07-10
> **Foco:** P686 encontrou que `gray` não está ligado no scope global, apesar de ser um nome de cor comum. Dado que cores nomeadas são fundamentais para praticamente qualquer documento Typst, e não apenas para `cetz`, este passo confirma a lista completa de cores que o vanilla expõe como valores globais, e fecha qualquer gap encontrado de uma vez — não só `gray`.
> **Tipo:** Sonda mínima + Implementação.
> **Tamanho:** S-M.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P686 (onde `gray` foi encontrado em falta).

---

## Sonda mínima

### Confirmar a lista completa de cores nomeadas do vanilla

```bash
cat > /tmp/p687-cores.typ <<'EOF'
#black #gray #silver #white
#red #orange #yellow #green #blue #purple
#navy #aqua #teal #eastern #maroon #fuchsia #lime #olive #ostrich #pink
EOF
lab/typst-original/target/release/typst compile /tmp/p687-cores.typ /tmp/p687-vanilla.pdf
echo "Exit code: $?"
```

Se algum destes falhar, remover da lista e confirmar quais são reais — não assumir a lista, obtê-la da documentação oficial do Typst (referência de cores), não só por tentativa e erro.

### Confirmar o estado actual do cristalino

```bash
./target/release/typst /tmp/p687-cores.typ /tmp/p687-cristalino.pdf
echo "Exit code: $?"
```

Testar cada cor individualmente, se o documento completo falhar, para identificar exactamente quais faltam, não só a primeira.

### Critério de fecho da sonda mínima

- [ ] Lista oficial completa de cores nomeadas confirmada, via documentação, não só teste de tentativa e erro.
- [ ] Cada cor testada individualmente contra o cristalino, lista de faltas completa.

---

## Implementação

Adicionar todas as cores nomeadas em falta ao scope global, ligadas ao valor de cor correcto (confirmar os valores RGB/HSL exactos de cada uma contra o vanilla, não assumir).

### Critério de fecho da implementação

- [ ] Todas as cores da lista oficial ligadas no scope global.
- [ ] Valores exactos (RGB) de cada cor confirmados contra o vanilla.
- [ ] Cores já existentes antes deste passo sem regressão.

---

## Validação

```bash
./target/release/typst /tmp/p687-cores.typ /tmp/p687-depois.pdf
```

```bash
cat > /tmp/p687-comparar.typ <<'EOF'
#repr(gray) #repr(navy) #repr(eastern)
EOF
lab/typst-original/target/release/typst compile /tmp/p687-comparar.typ /tmp/p687-repr-vanilla.pdf
pdftotext /tmp/p687-repr-vanilla.pdf -
./target/release/typst /tmp/p687-comparar.typ /tmp/p687-repr-cristalino.pdf
pdftotext /tmp/p687-repr-cristalino.pdf -
```

Comparar os valores RGB exactos, não só se compilam sem erro.

### Confirmar `cetz` de novo

```bash
cat > /tmp/p687-cetz.typ <<'EOF'
#import "@preview/cetz:0.2.2": canvas, draw
#canvas({
  draw.line((0,0), (1,1))
})
EOF
./target/release/typst /tmp/p687-cetz.typ /tmp/p687-cetz.pdf
echo "Exit code: $?"
```

Dado que P686 já confirmou que o vanilla também falha neste documento mínimo (em `canvas.typ:24`), não esperar que o cristalino produza PDF completo — confirmar apenas que o erro de `gray` desapareceu, e que o próximo erro (se houver) é comparável ao ponto onde o vanilla também tropeça, não um erro anterior a esse ponto.

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Sonda mínima completa, lista oficial confirmada via documentação.
- [ ] Todas as cores em falta implementadas, com valores RGB exactos confirmados.
- [ ] `cetz` re-testado — erro de `gray` desaparecido; próximo ponto de falha comparado com o do vanilla.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p687.md`, com hash do commit.
