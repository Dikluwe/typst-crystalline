---
# P752 — Usar métricas reais da fonte no cálculo de `cap_height`, não a aproximação `FixedMetrics`

> **Passo:** 752
> **Data:** 2026-07-14
> **Foco:** P750/P751 deixaram um resíduo consistente de ~0,2-0,3pt na posição da baseline, explicado por `FixedMetrics::cap_height` usar uma aproximação (`size * 0.7`) em vez da métrica real da fonte. O próprio P750 já confirmou que `FontBookMetrics`/`FallbackFontMetrics` (`03_infra/src/font_metrics.rs`) já lêem `ttf.capital_height()` real, com fallback para `ascender`. Este passo confirma se essas implementações reais já estão disponíveis no ponto onde `ensure_initial_baseline()` (P751) calcula o `cap_height`, e liga-as, em vez de continuar a usar `FixedMetrics` nesse caminho.
> **Tipo:** Sonda + Implementação.
> **Tamanho:** S-M — a infra-estrutura de métricas reais já existe (P750); a questão é se está acessível no ponto certo, ou se precisa de ser passada até lá.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P751 (onde `ensure_initial_baseline()` foi criado, actualmente a usar `FixedMetrics`), P750 (onde `FontBookMetrics`/`FallbackFontMetrics` com `ttf.capital_height()` real já foram construídos, para outro propósito).

---

## Sonda

### Confirmar exactamente que implementação de `FontMetrics` é usada em `ensure_initial_baseline()`

```bash
grep -n "fn ensure_initial_baseline\|FixedMetrics\|dyn FontMetrics\|impl FontMetrics" 01_core/src/rules/layout/mod.rs 01_core/src/rules/layout/cursor.rs
```

Confirmar se `ensure_initial_baseline()` recebe uma referência genérica a `dyn FontMetrics` (podendo então receber `FontBookMetrics` real), ou se está ligado directamente a `FixedMetrics` de forma rígida.

### Confirmar se a fonte real (necessária para `FontBookMetrics`) já está resolvida no ponto onde `ensure_initial_baseline()` corre

```bash
grep -n "fn ensure_initial_baseline" -A 30 01_core/src/rules/layout/mod.rs 01_core/src/rules/layout/cursor.rs
```

Confirmar se, no momento da primeira emissão de conteúdo real (texto, equação, etc.), a fonte já está resolvida (nome, ficheiro, `ttf_parser::Face` disponível) — se sim, `FontBookMetrics::cap_height` pode ser chamado directamente. Se a fonte ainda não estiver resolvida nesse ponto (por exemplo, a resolução de fonte acontece depois, durante o shaping), confirmar o que fazer.

### Confirmar por que `FixedMetrics` existe e é usado aqui, historicamente

```bash
grep -n "FixedMetrics" 01_core/src/rules/layout/*.rs 03_infra/src/*.rs | grep -v test
```

Confirmar todos os sítios onde `FixedMetrics` ainda é usado — se for um caminho de fallback genuíno (por exemplo, para quando a resolução de fonte real falha), ou se é usado por conveniência em locais onde a métrica real já estaria disponível mas nunca foi ligada.

### Critério de fecho da sonda

- [ ] Confirmado se `FontBookMetrics`/`FallbackFontMetrics` (métrica real) está acessível no ponto de `ensure_initial_baseline()`.
- [ ] Confirmado se a fonte já está resolvida nesse ponto do pipeline.
- [ ] Confirmados todos os usos actuais de `FixedMetrics`, distinguindo fallback genuíno de uso por conveniência.

---

## Implementação

Se a métrica real estiver acessível no ponto certo: trocar `FixedMetrics::cap_height` por `FontBookMetrics::cap_height`/`FallbackFontMetrics::cap_height` em `ensure_initial_baseline()`.

Se a fonte ainda não estiver resolvida nesse ponto: confirmar o caminho mais simples para resolvê-la mais cedo, ou para adiar `ensure_initial_baseline()` até ao ponto em que a fonte já esteja disponível (mesmo princípio já usado em P751 — não calcular antes de ter a informação certa).

### Critério de fecho da implementação

- [ ] `cap_height` na baseline inicial usa a métrica real da fonte, não a aproximação `size * 0.7`.
- [ ] Resíduo de ~0,2-0,3pt (P750/P751) eliminado ou reduzido a um valor explicado.
- [ ] `FixedMetrics` mantido como fallback genuíno, se confirmado necessário para algum caso (fonte não resolvível), não removido sem razão.

---

## Validação

```bash
cat > /tmp/p752-baseline.typ <<'EOF'
X
EOF
lab/typst-original/target/release/typst compile /tmp/p752-baseline.typ /tmp/p752-vanilla.pdf
mutool show /tmp/p752-vanilla.pdf 4 2>&1 | head -10
./target/release/typst /tmp/p752-baseline.typ /tmp/p752-depois.pdf
mutool show /tmp/p752-depois.pdf 4 2>&1 | head -10
```

Comparar com o vanilla — o resíduo deve aproximar-se de zero.

```bash
cargo test --workspace
crystalline-lint .
```

Correr o corpus completo com atenção a regressão silenciosa, dado que isto toca o cálculo de baseline usado em todo o texto.

### Repetir a reprodução final de `cetz`

```bash
cat > /tmp/p752-cetz.typ <<'EOF'
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
EOF
./target/release/typst /tmp/p752-cetz.typ /tmp/p752-cetz.pdf
mutool draw -o /tmp/p752-cetz.png -r 150 /tmp/p752-cetz.pdf
lab/typst-original/target/release/typst compile /tmp/p752-cetz.typ /tmp/p752-cetz-vanilla.pdf
mutool draw -o /tmp/p752-cetz-vanilla.png -r 150 /tmp/p752-cetz-vanilla.pdf
```

Comparar (`compare -metric AE`) — confirmar se o número de pixels diferentes (~3312, medido em P751) diminui.

---

## Critério de fecho do passo

- [ ] Sonda completa, disponibilidade da métrica real confirmada no ponto certo.
- [ ] Corrigido, testado com documento mínimo — resíduo eliminado ou reduzido a valor explicado.
- [ ] `FixedMetrics` mantido só onde genuinamente necessário.
- [ ] Sem regressão em `cargo test --workspace`, corpus completo verificado.
- [ ] `crystalline-lint .` limpo.
- [ ] `cetz` — número de pixels diferentes medido de novo, comparado com o valor anterior.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p752.md`, com hash do commit.
