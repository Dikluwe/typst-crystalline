---
# P751 — `cursor_y` inicial fixado cedo demais, deve usar o estilo realmente activo

> **Passo:** 751
> **Data:** 2026-07-14
> **Foco:** P750 encontrou que `Layouter::new` fixa `cursor_y = margem + cap_height(initial_style.size)` usando um estilo por defeito, antes de processar qualquer `#set text(size: ...)` que apareça logo no início do documento. Isto significa que `#set text(size: 8pt)` seguido de texto ainda posiciona a primeira linha usando `cap_height` de 11pt (o tamanho por defeito), não 8pt. É a mesma classe de bug já corrigida em P711 para blocos `context` — um valor calculado cedo demais, com um estilo assumido, em vez de esperar pelo estilo realmente activo no ponto certo.
> **Tipo:** Sonda + Implementação. Mecanismo central de layout, mesma prioridade de P711.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.** **ADR-0114 EM VIGOR** — mesma categoria de mecanismo central já corrigida uma vez (P711); cuidado para não reintroduzir o mesmo tipo de bug de forma diferente.
> **Dependências:** P750 (onde a limitação foi encontrada e explicitamente deixada fora do âmbito), P711 (correcção irmã, mesmo padrão de bug — `StyleChain` fixado cedo demais).

---

## Sonda

### Confirmar exactamente onde e quando `cursor_y` é fixado hoje

```bash
grep -n "cursor_y = margin\|fn new" 01_core/src/engine/layout/mod.rs | head -10
```

Confirmar se `Layouter::new` recebe o `StyleChain` inicial como argumento fixo, ou se tem acesso ao conteúdo completo do documento (que lhe permitiria, em teoria, olhar à frente).

### Confirmar como o vanilla resolve isto

```bash
grep -n "fn new\|cursor\|first.*style\|initial.*style" lab/typst-original/crates/typst-layout/src/flow/mod.rs 2>/dev/null | head -20
```

Confirmar se o vanilla adia o cálculo da posição inicial até processar o primeiro elemento de conteúdo real (não fixa nada antes disso), ou se tem outro mecanismo. A hipótese mais provável, dado o padrão já visto em P711: o cursor não é um valor fixo calculado à partida, é derivado do `StyleChain` activo no momento em que a primeira linha é de facto colocada.

### Reproduzir o caso exacto

```bash
cat > /tmp/p751-tamanho-inicial.typ <<'EOF'
#set text(size: 8pt)
X
EOF
lab/typst-original/target/release/typst compile /tmp/p751-tamanho-inicial.typ /tmp/p751-vanilla.pdf
mutool show /tmp/p751-vanilla.pdf 4 2>&1 | head -10
./target/release/typst /tmp/p751-tamanho-inicial.typ /tmp/p751-cristalino.pdf
mutool show /tmp/p751-cristalino.pdf 4 2>&1 | head -10
```

Confirmar a diferença exacta antes de corrigir.

### Critério de fecho da sonda

- [ ] Confirmado exactamente onde `cursor_y` é fixado hoje, e com que estilo.
- [ ] Confirmado como o vanilla evita este problema (adiar o cálculo, não fixar antecipadamente).
- [ ] Caso reproduzido com números antes da correcção.

---

## Implementação

Adiar o cálculo de `cursor_y` inicial para o momento em que o primeiro conteúdo real é processado, usando o `StyleChain` activo nesse ponto (que já incorpora qualquer `#set` aplicado antes), em vez de calcular com um estilo assumido em `Layouter::new`. Seguir o mesmo padrão arquitectural já usado na correcção de P711 (acumular o `StyleChain`, não descartar/assumir).

### Critério de fecho da implementação

- [ ] `#set text(size: 8pt)` seguido de texto posiciona a primeira linha usando 8pt, não 11pt.
- [ ] Caso sem `#set` inicial (tamanho por defeito) sem regressão.
- [ ] `#set page(...)` a meio do documento (já corrigido por P750) sem regressão.
- [ ] Grids/formas (P748/P750, ajustados à baseline) sem regressão.

---

## Validação

```bash
./target/release/typst /tmp/p751-tamanho-inicial.typ /tmp/p751-depois.pdf
mutool show /tmp/p751-depois.pdf 4 2>&1 | head -10
```

Comparar com o vanilla já obtido na sonda — a diferença deve aproximar-se de zero (dentro da mesma margem residual de métricas de fonte já explicada por P750, ~0,28pt).

```bash
cargo test --workspace
crystalline-lint .
```

Dado o alcance (mecanismo central de layout, mesma categoria de P711), correr o corpus completo com atenção a regressão silenciosa.

### Repetir a reprodução final de `cetz`

```bash
cat > /tmp/p751-cetz.typ <<'EOF'
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
EOF
./target/release/typst /tmp/p751-cetz.typ /tmp/p751-cetz.pdf
mutool draw -o /tmp/p751-cetz.png -r 150 /tmp/p751-cetz.pdf
lab/typst-original/target/release/typst compile /tmp/p751-cetz.typ /tmp/p751-cetz-vanilla.pdf
mutool draw -o /tmp/p751-cetz-vanilla.png -r 150 /tmp/p751-cetz-vanilla.pdf
python3 /tmp/pngdiff.py /tmp/p751-cetz-vanilla.png /tmp/p751-cetz.png
```

---

## Critério de fecho do passo

- [ ] Sonda completa, mecanismo confirmado, causa exacta localizada.
- [ ] Corrigido: `cursor_y` inicial usa o estilo realmente activo, não um valor assumido.
- [ ] Casos de não-regressão testados (sem `#set` inicial, `#set page` a meio, grids/formas).
- [ ] Sem regressão em `cargo test --workspace`, corpus completo verificado.
- [ ] `crystalline-lint .` limpo.
- [ ] `cetz` sem regressão.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p751.md`, com hash do commit.
