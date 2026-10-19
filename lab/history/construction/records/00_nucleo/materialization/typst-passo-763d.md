---
# P763d — Correcção: divergência de coordenadas em `circle` nativo e no canvas do `cetz`

> **Passo:** 763d
> **Data:** 2026-07-15
> **Foco:** P763c refutou a explicação "mecânica/anti-aliasing" de P763b com medição directa e encontrou uma causa real, com duas componentes distintas: (1) a primitiva nativa `circle` tem um deslocamento vertical próprio de ~13 pt, mesmo isolada, sem `cetz`; (2) o canvas do `cetz` amplifica isso porque o cristalino usa flip-Y global sem aplicar a translação local do canvas que o vanilla aplica, e o eixo Y dentro do canvas parece invertido (o círculo aparece no ponto final da linha, não no inicial). Este passo corrige as duas causas, separadamente, com validação isolada de cada uma antes de validar o caso combinado.
> **Tipo:** Sonda adicional (localizar o código exacto) + Implementação.
> **Tamanho:** L — mexe no renderizador de paths, que é caminho partilhado por outras primitivas de desenho; regra 5 do handoff (checklist de sub-layouts) aplica-se aqui adaptada a formas de desenho, não só a texto.
> **ADR-0108 EM VIGOR** — medir antes/depois de cada correcção separadamente, não só no caso combinado.
> **Dependências:** P763c (causa concreta identificada e medida).

---

## Parte A — Deslocamento próprio de `circle` nativo (~13 pt)

### Sonda — localizar a origem do deslocamento

```bash
grep -rn "fn.*circle\|Circle" 01_core/src/engine/layout/*.rs 01_core/src/entities/*.rs 2>/dev/null | grep -i circle
```

Comparar com o cálculo do vanilla:

```bash
grep -n "fn layout_circle\|origin\|center" lab/typst-original/crates/typst-library/src/visualize/shape.rs 2>/dev/null | head -30
```

Hipóteses a confirmar directamente (não assumir):
1. Origem do círculo calculada a partir do canto superior-esquerdo da bounding box vs a partir do centro — troca de convenção.
2. `radius` vs `diameter` — possível confusão de metade/dobro em algum ponto do cálculo (13 pt não é óbvio como fracção de 10 pt de raio do teste isolado — registar a relação exacta encontrada, ex: se for `stroke-width/2` mal aplicado, etc.).
3. Comparar com `rect`/`square` (que não mostraram o mesmo problema nos testes de baseline de P763b, AE=241) para confirmar que o bug é específico da forma circular, não do sistema de coordenadas geral.

### Implementação

Corrigir o cálculo de origem/centro em `01_core/src/engine/layout/` (caminho exacto a confirmar pela sonda), replicando a convenção do vanilla.

### Validação isolada

```bash
cat > /tmp/p763d-circle-isolado.typ <<'EOF'
#set page(width: 8cm, height: 4cm)
#circle(radius: 10pt)
EOF
./target/release/typst compile /tmp/p763d-circle-isolado.typ /tmp/p763d-circle-cristalino.pdf
lab/typst-original/target/release/typst compile /tmp/p763d-circle-isolado.typ /tmp/p763d-circle-vanilla.pdf
compare -metric AE /tmp/p763d-circle-vanilla.pdf /tmp/p763d-circle-cristalino.pdf /tmp/p763d-circle-diff.png
```

Esperado: AE ≈ mesmo baseline de `rect` (≈241 ou menos), não mais.

### Checklist de sub-layouts (regra 5, adaptada)

Repetir o mesmo teste de `circle` dentro de `grid`, `box`, `columns`, `place` — confirmar que a correcção não é válida só no caso solto:

```bash
for wrapper in grid box columns place; do
  echo "=== $wrapper ==="
  # documento de teste específico por wrapper, mesma métrica AE
done
```

---

## Parte B — Canvas do `cetz`: translação ausente e possível inversão de eixo Y

### Sonda — localizar como o cristalino aplica transformações de grupo/canvas

```bash
grep -rn "fn.*transform\|Transform\b" 01_core/src/engine/layout/*.rs 01_core/src/entities/*.rs 2>/dev/null | grep -iv "test" | head -40
```

Confirmar:
1. Onde a translação de um grupo posicionado (via `place()`, que é o mecanismo que `cetz` usa internamente para posicionar cada elemento do canvas) deveria ser aplicada, e por que está a ser omitida.
2. Se existe, de facto, uma inversão de eixo Y dentro do escopo do grupo (P763c observou que o círculo aparece no ponto final da linha em vez do inicial — isto é consistente com um sinal trocado em alguma das componentes Y da transformação acumulada).
3. Se a correcção da Parte A (deslocamento próprio do `circle`) já resolve parte desta amplificação, medir de novo **depois** de A estar corrigida, antes de investigar mais fundo em B — pode ser que B seja só A amplificado, não uma causa adicional independente.

### Implementação (só depois de medir se ainda é necessária após a Parte A)

Corrigir a aplicação de translação/transformação de grupo, replicando o comportamento do vanilla (`transform="1 0 0 1 tx ty"` local, em vez de só flip-Y global sem translação).

### Validação isolada

```bash
cat > /tmp/p763d-cetz-line.typ <<'EOF'
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
})
EOF
```

Medir AE só com `line` (P763c mediu 2848 isolado) — depois de A corrigido, medir de novo antes de decidir se B precisa de correcção separada.

---

## Validação combinada (documento original de P763b/P763c)

```bash
cat > /tmp/p763d-cetz-completo.typ <<'EOF'
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
EOF
./target/release/typst compile /tmp/p763d-cetz-completo.typ /tmp/p763d-completo-cristalino.pdf
lab/typst-original/target/release/typst compile /tmp/p763d-cetz-completo.typ /tmp/p763d-completo-vanilla.pdf
compare -metric AE /tmp/p763d-completo-vanilla.pdf /tmp/p763d-completo-cristalino.pdf /tmp/p763d-completo-diff.png
```

Meta: AE no mesmo patamar do baseline de `rect` (≈241), não necessariamente 0 — registar o valor real e só investigar mais se ainda estiver na mesma ordem de grandeza do bug original (milhares).

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Parte A: origem do deslocamento de `circle` isolado identificada por leitura de código, não suposição.
- [ ] Parte A: corrigida; AE isolado no mesmo patamar do baseline de `rect`.
- [ ] Parte A: checklist de sub-layouts (grid, box, columns, place) confirmada.
- [ ] Parte B: medida de novo depois de A — só implementada se a amplificação persistir independente de A.
- [ ] Se Parte B for necessária: translação de grupo corrigida; inversão de eixo Y confirmada e corrigida ou descartada com evidência.
- [ ] Caso combinado (documento original de P763b) medido com AE final registado.
- [ ] `cargo test --workspace` verde.
- [ ] `crystalline-lint .` zero violações.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p763d.md`, com AE antes/depois de cada parte, separadamente.

---

## Próximo passo

Se AE final ficar no patamar do baseline de `rect`: fechar definitivamente a linha de trabalho `cetz`/download de pacotes (P763–P763d). Se persistir alguma diferença de ordem de grandeza maior que o baseline: abrir P763e com o achado específico remanescente, não reabrir o âmbito já fechado deste passo.
