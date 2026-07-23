# Relatório — P877: Diagnóstico da regressão de performance deixada por P874–P876

**Data:** 2026-07-23T19:13:17-03:00  
**Commit:** `30122788891b32a423b0354b97eebdbf28ccf068`  
**Estado:** fechado (diagnóstico, sem correções)  
**L0s afetados:** nenhum (apenas medições e relatório)

---

## 1. Contexto

Após P874 (subsetting CFF), P875 (filtro de fallback por coverage) e P876 (cache/dedup de imagens), o benchmark P872 foi reproduzido. As sete razões cristalino/vanilla **pioraram** em vez de melhorar. Este passo existe para isolar as causas reais antes de qualquer nova correção.

---

## 2. Metodologia

- Build release do cristalino no commit acima.
- Vanilla: `lab/typst-original/target/release/typst` (0.15.0, 969087ec).
- `hyperfine 1.20.0`, `--warmup 1 --min-runs 10`.
- Documentos em `/tmp/p872-bench/` (os mesmos de P872).
- Isolamento por reversão temporária de partes específicas do código, medição, e reposição imediata.

---

## 3. Resultados do benchmark completo (estado atual)

| Cenário | P872 C/V | P876 C/V | Delta |
|---|---|---|---|
| 01-hello | 0.35× | 0.56× | +0.21× |
| 02-lorem | 0.41× | 0.62× | +0.21× |
| **03-images** | **16.26×** | **23.59×** | **+7.32×** |
| **04-math** | **22.36×** | **23.64×** | **+1.28×** |
| 05-tables | 0.38× | 0.58× | +0.20× |
| 06-long | 1.24× | 1.45× | +0.21× |
| 07-context | 0.44× | 0.65× | +0.22× |

A regressão uniforme de ~+0.21× nos cenários simples e a explosão em imagens exigem duas investigações separadas.

---

## 4. Investigação 1 — Regressão uniforme (+0.21× em hello, lorem, tables, context)

### Hipótese

P875 introduziu `Coverage` em `FontInfo` e `extract_coverage(&face)` é chamado incondicionalmente para cada face durante `pair_slots_with_book` / `load_system_fonts`. Isso adiciona um custo fixo de CPU por compilação, independentemente do documento usar fallback ou não.

### Localização no código

- `03_infra/src/fonts.rs:349` — `coverage: extract_coverage(&face)` dentro de `font_info_from_bytes`.
- `03_infra/src/fonts.rs:826` — `pair_slots_with_book` chama `font_info_from_bytes` para cada slot descoberto.
- `03_infra/src/fontdb.rs:48` — `load_system_fonts` também chama `font_info_from_bytes` para cada face do sistema.
- `03_infra/src/world.rs:278` — `with_fonts_and_system` e `with_system_fonts` invocam os caminhos acima no arranque.

### Medição de isolamento

Reversão temporária: `coverage: Coverage::new()` em vez de `extract_coverage(&face)`.

```text
=== 01-hello (coverage vazio) ===
Cristalino: ~103 ms  |  Vanilla: ~281 ms  →  cristalino 2.71× mais rápido

=== 03-images (coverage vazio) ===
Cristalino: ~104 ms  |  Vanilla: ~6.8 ms  →  razão 15.13× (vs 23.59× com coverage)
```

### Conclusão

A extração eager de `coverage` é responsável por **~55–60 ms** de custo fixo por compilação. Isso explica praticamente toda a regressão uniforme de +0.21×. A correção é tornar a extração de coverage **lazy** (só quando a fonte é candidata a fallback), ou cachear entre compilações.

---

## 5. Investigação 2 — "Explosão" de syscalls em imagens

### Observação inicial

`strace -c` de `03-images` mostrava 2334 `openat` e 2981 `readlink` no cristalino, contra ~20 `openat` no vanilla.

### Hipótese testada

A explosão seria específica do caminho de imagem (cache de bytes, cache de payload, ou deduplicação por `Arc::as_ptr`).

### Medição de controlo

`strace -c` de `01-hello` (sem imagens) no estado atual:

```text
openat:    ~2332
readlink:  ~2981
statx:     ~2453
close:     ~2332
```

Os mesmos números de syscalls aparecem em `01-hello`. A explosão **não é específica de imagens**.

### Causa identificada

A descoberta de fontes do sistema via `fontdb::Database::load_system_fonts()` (`03_infra/src/fontdb.rs:30`) percorre todos os ficheiros de fontes do sistema em cada execução. O vanilla usa fontconfig/threading e faz muito menos I/O de arranque.

### Conclusão

A diferença de tempo entre cristalino e vanilla em `03-images` tem duas componentes:
1. Custo fixo de descoberta de fontes do sistema (~90–100 ms de syscall time).
2. Custo da extração eager de coverage (~55–60 ms).

Mesmo com coverage vazio, `03-images` continua ~15× mais lento que o vanilla, quase todo por causa do I/O de fontes do sistema. O cache de imagem (P876) funcionou estruturalmente (PDF caiu de 95.9 KB para 8.4 KB), mas não ataca o gargalo de tempo.

---

## 6. Investigação 3 — Por que 04-math não melhorou

### Hipóteses testadas

1. **Custo da extração de coverage?** Não — coverage vazio não mudou o tempo (23.44× vs 23.64×).
2. **Custo do subsetting CFF (P874)?** Não — subsetting desativado não mudou o tempo (23.34× vs 23.64×).

### Medição direta

`--timings-json` do cristalino para `04-math`:

```json
{
  "eval_ms": 5.521,
  "introspect_ms": 0.668,
  "expand_context_ms": 0.554,
  "layout_ms": 5709.760,
  "shape_ms": 36.651,
  "subset_ms": 0.000,
  "render_ms": 2.920,
  "total_ms": 5756.073
}
```

### Conclusão

**O gargalo de 04-math é o layout matemático: 5.71 s dos 5.76 s totais.** Subsetting e coverage são irrelevantes para este cenário. O layout de equações matemáticas no cristalino tem uma complexidade superlinear ou recálculo repetido que precisa de investigação própria (provavelmente no módulo `engine/layout/math` ou no posicionamento de sub/superscripts).

---

## 7. Síntese das causas

| Problema | Onde | Evidência | Correção futura |
|---|---|---|---|
| Regressão uniforme (+0.21×) | `font_info_from_bytes` → `extract_coverage` | 55–60 ms recuperados com coverage vazio | Tornar coverage lazy ou cachear entre runs |
| Imagens ainda 15–24× mais lentas | `fontdb::load_system_fonts()` | 2334 `openat`/`readlink` mesmo em `01-hello`; vanilla faz ~20 | Cachear descoberta de fontes do sistema, ou lazy discovery |
| Math ainda 22–24× mais lento | Layout matemático (`layout_ms` = 5.7 s) | Subsetting/coverage irrelevantes; `--timings-json` aponta layout | Investigar/otimizar `engine/layout/math` |

---

## 8. O que NÃO foi a causa

- **Duplicação de imagens (P873/P876):** corrigida estruturalmente; o PDF de `03-images` caiu de 95.9 KB para 8.4 KB. Não é o gargalo de tempo.
- **Subsetting CFF (P874):** reduziu o PDF de `04-math` de 1.8 MB para 157 KB; não afeta o tempo.
- **Filtro de fallback por coverage (P875):** funciona mecanicamente, mas a extração eager introduziu regressão de startup e não ataca o gargalo de math.

---

## 9. Recomendações para o próximo passo

Três frentes independentes:

1. **P878a — Coverage lazy:** mover `extract_coverage` para ser calculado só quando `FontBook::candidates_for_char` é chamado, ou cachear os resultados entre compilações.
2. **P878b — Cache de descoberta de fontes:** persistir ou lazy-carregar o resultado de `load_system_fonts()` para não repetir os ~2300 `openat` em cada compilação.
3. **P878c — Layout math:** perfilar `engine/layout/math` para identificar por que 100 equações simples consomem 5.7 s.

A ordem sugerida é (1) → (2) → (3), porque (1) e (2) são regressões introduzidas por P875/P874 e devem ser estabilizadas antes de atacar (3), que é um problema de paridade funcional mais profundo.

---

## 10. Procedimento de medição para validação futura

Qualquer correção futura deve reproduzir:

```text
hyperfine --warmup 1 --min-runs 10 \
  "lab/typst-original/target/release/typst compile 04-math.typ /dev/null --format pdf" \
  "target/release/typst 04-math.typ /dev/null"
```

E reportar a razão C/V. O critério de fecho é a razão cair de ~23× para perto de 1× (ou pelo menos demonstrar melhoria significativa e explicada).
