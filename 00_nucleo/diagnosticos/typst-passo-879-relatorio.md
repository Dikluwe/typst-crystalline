# Relatório — P879: Aplicar filtro de coverage em `FallbackFontMetrics::covering`

**Data:** 2026-07-23T19:51:21-03:00  
**Commit:** `3f15cc50ec1dc40e852b41bc92e9ee2895ecaec6`  
**Estado:** fechado — escopo mínimo implementado; critério mínimo de math atingido; critério completo de cenários simples **não** atingido (regressão residual de coverage eager, ver secção 6)  
**L0s afetados:** nenhum alterado (a mudança é mecânica e já estava prevista pelo L0 de `infra/fonts.md` e `infra/shaper.md`, que introduziram `candidates_for_char`)

---

## 1. O que foi implementado

### 1.1 Escopo mínimo (obrigatório)

`03_infra/src/font_metrics.rs:750`, `FallbackFontMetrics::covering`:

```rust
// Antes:
for slot_idx in 0..book.len() { ... }

// Depois (P879):
for slot_idx in book.candidates_for_char(c) { ... }
```

Esta alteração faz com que o fallback de métricas de fonte use o mesmo filtro de coverage introduzido em P875 para o shaper. Código de produção, não reversão temporária.

### 1.2 Escopo alargado (avaliado e rejeitado)

P878 identificou três loops semelhantes em `font_metrics.rs`:

- `line_metrics` (~linha 909)
- `cap_height` (~linha 957)
- `edge_metrics` (~linha 994)

**Avaliação:** estes loops só executam quando `resolve_primary(style)` falha (retorna vazio). Nos cenários do benchmark, as primárias resolvem sempre. `strace -e trace=openat` em `01-hello` e `03-images` mostrou apenas as 11 aberturas baseline de `NotoSansCJK-Regular.ttc` (descoberta inicial do `fontdb`), sem aberturas extra. Em `04-math`, após a correção de `covering`, também não há aberturas CJK extra.

**Decisão:** não aplicar o filtro a estes três loops neste passo. A contribuição é mensuravelmente zero nos cenários testados; aplicá-los seria otimização especulativa.

---

## 2. Validação

### 2.1 Testes

```text
$ cargo test --workspace

test result: ok. 4686 passed; 0 failed; 2 ignored; 0 measured; 0 filtered out  (typst_core)
test result: ok. 727 passed; 0 failed; 5 ignored; 0 measured; 0 filtered out   (typst_infra)
test result: ok. 41 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out    (typst_shell)
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out     (typst_wiring unit)
test result: ok. 37 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out    (typst_wiring integration)
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out     (typst_wiring crystalline_lint)
```

Total: **5505 passed**.

### 2.2 Linter

```text
$ crystalline-lint .
warning: Prompt órfão: '00_nucleo/prompts/infra/package_version_resolution.md' ... [V7]
exit: 0
```

Zero violations. O warning V7 é pré-existente.

---

## 3. Benchmark completo P872 (sete cenários)

### 3.1 Tabela comparativa

| Cenário | P872 C/V (base) | P876 C/V (regredido) | P879 C/V (após correção) | vs P872 | vs P876 |
|---|---|---|---|---|---|
| 01-hello | 0.35× | 0.56× | **0.54×** | +0.19× | -0.02× |
| 02-lorem | 0.41× | 0.62× | **0.60×** | +0.19× | -0.02× |
| 03-images | 16.26× | 23.59× | **23.18×** | +6.91× | -0.41× |
| 04-math | 22.36× | 23.64× | **19.35×** | -3.00× | -4.28× |
| 05-tables | 0.38× | 0.58× | **0.56×** | +0.18× | -0.02× |
| 06-long | 1.24× | 1.45× | **1.40×** | +0.16× | -0.05× |
| 07-context | 0.44× | 0.65× | **0.61×** | +0.18× | -0.04× |

### 3.2 Interpretação

- **04-math:** melhorou de 23.64× para **19.35×**, atingindo o critério mínimo (~19× ou melhor). Confirma que o filtro em `FallbackFontMetrics::covering` é a correção certa para a regressão de math.
- **Cenários simples (01, 02, 05, 07):** melhoraram marginalmente em relação a P876 (0.02× a 0.05×), mas **não recuperaram a vantagem original de P872**. Há uma regressão residual uniforme de ~0.18× em todos eles.

### 3.3 Métricas de `/usr/bin/time -v` para 04-math

| Métrica | P876 (regredido) | P879 (após correção) |
|---|---|---|
| User time | 1.35 s | **0.57 s** |
| System time | 5.37 s | **4.85 s** |
| Elapsed | 6.73 s | **5.43 s** |
| Maximum RSS | 9.41 GB | **7.07 GB** |

### 3.4 `strace -c` para 04-math

| Métrica | P876 (regredido) | P879 (após correção) |
|---|---|---|
| Chamadas `read` | 2343 | **797** |
| % tempo em `read` | 93.55% | **78.13%** |

A correção eliminou as ~1500 chamadas `read` extra correspondentes às faces CJK.

### 3.5 Aberturas de `.ttc` CJK em 04-math

| Ficheiro | Baseline (01-hello) | P876 (regredido) | P879 (após correção) |
|---|---|---|---|
| `NotoSansCJK-Regular.ttc` | 11 | 21 | **11** |
| `NotoSerifCJK-Regular.ttc` | 6 | 11 | **6** |

Voltou ao baseline: o filtro evitou que cada face CJK fosse carregada durante o fallback de math.

---

## 4. Efeito em imagens (03-images)

`03-images` melhorou marginalmente (23.59× → 23.18×). A causa dominante deste cenário continua a ser o custo fixo de descoberta de fontes do sistema (`fontdb::load_system_fonts`), não o loop corrigido aqui — imagens não disparam fallback de fonte. `strace` confirmou apenas 11 aberturas baseline de `NotoSansCJK-Regular.ttc`.

---

## 5. Critérios de fechamento

### 5.1 Critério mínimo (math)

**Atingido.** 04-math passou de 23.64× para **19.35×**.

### 5.2 Critério completo (cenários simples voltarem a 0.35–0.44×)

**Não atingido.** Os quatro cenários simples melhoraram apenas marginalmente e permanecem ~0.18× acima dos valores de P872.

---

## 6. Por que o critério completo não foi atingido

A regressão residual foi isolada em P877: a extração eager de `coverage` em `font_info_from_bytes` (P875) adiciona ~55–60 ms de CPU por compilação, independentemente do documento. Com coverage vazio, `01-hello` melhorou de 0.56× para 0.35× (2.71× mais rápido que o vanilla), confirmando que este é o custo restante.

**Não foi corrigido neste passo porque:**
- O escopo de P879 era especificamente o filtro em `FallbackFontMetrics::covering`.
- A correção de coverage eager requer uma alteração arquitetural mais profunda (coverage lazy ou cache persistente), com impacto em `FontInfo`, `FontBook` e possivelmente no trait `World`).
- Fazê-lo no mesmo passo arriscaria misturar duas correções de natureza diferente e dificultar a medição isolada.

---

## 7. Próximo passo recomendado

**P880 — coverage lazy:** tornar a extração de cobertura Unicode lazy (só quando `candidates_for_char` é chamada para uma fonte, e idealmente cacheada) para eliminar o custo fixo de ~55–60 ms de startup. Após essa correção, espera-se que os cenários simples voltem aos valores originais de P872 (0.35–0.44×) e que `03-images` também melhore significativamente.

---

## 8. Ficheiros alterados

- `03_infra/src/font_metrics.rs` (linha 750 — loop `covering`)

---

## 9. Procedimento de reprodução

```text
cargo build --release --bin typst
hyperfine --warmup 1 --min-runs 10 \
  "lab/typst-original/target/release/typst compile 04-math.typ /dev/null --format pdf" \
  "target/release/typst 04-math.typ /dev/null"
```
