# Passo 963 — Relatório (sup lateral de base esticada: `is_text_like` exclui `extended_shape`)

**Data**: 2026-08-04
**Estado da árvore**: commit base `eb080abc4` (P962); alterações deste passo por cima.

---

## 1. Fase A — por que P959 só corrigiu um lado

A leitura da auditoria ("limite superior ~5× errado, mediana 11.39pt vs
2.37pt") foi reproduzida com o mesmo método (gap de tinta acima de cada
operador grande no documento): mediana cristalina **11.39pt** — o número
exacto da auditoria. A decomposição mostrou que os casos grandes NÃO eram
limites empilhados (o caminho de P959, correcto) mas **scripts laterais de
integrais esticadas em Display**: `$ integral_0^1 $` isolado — sup do
cristalino ~2.8pt ABAIXO da baseline da base; vanilla ~12.9pt ACIMA.

**Causa**: `compute_script_shifts` derivava `is_text_like` do **Content**
(`matches!(base, MathIdent|MathText)`) — verdadeiro para `∫`. No vanilla
(`fragment/mod.rs:129-135`), `is_text_like` de um glifo é
`!extended_shape`: uma base ESTICADA (variante de Display/assembly —
`FrameItem::Glyph` no cristalino) não é text-like, e o termo
`base_ascent − superscript_baseline_drop_max` aplica-se ao `shift_up` (e
`base_descent + subscript_baseline_drop_min` ao `shift_down`). Com o termo
zero por engano, o sup lateral ficava em `sup_shift_up` (~4.4pt) em vez de
~13.3pt. P959 (limites empilhados) estava e está correcta — era um braço
diferente de `attach.rs`.

Constantes medidas (NewCMMath-Book, du): SuperscriptShiftUp=363,
SuperscriptBaselineDropMax=250, SubscriptBaselineDropMin=200,
SubscriptShiftDown=247; `integral.v1` ink +1361/−861du. (Nos testes de L1
os valores vêm do `MathConstants::fallback()` — sup_drop_max=250,
sub_drop_min=50 — a forma é o que está em teste.)

## 2. Fase B — implementação (TDD directo)

- L0: `attach.md` §P963.
- Testes RED (em `p952op_tests`, stub com ink injectada da v1):
  `p963_integral_display_sup_lateral_usa_drop_term` (sup a 13.332pt acima;
  sub a 10.932pt abaixo — com as constantes do fallback) e a guarda
  `p963_base_texto_sup_sem_drop_term` (base text-like mantém o drop a
  zero). RED confirmado nos dois (5.664pt obtido antes da correcção).
- Correcção (`attach.rs`): `is_text_like` passa a ser falso quando a caixa
  da base contém `FrameItem::Glyph` (variante esticada/assembly).
- Suite: `cargo test --workspace` — **5706 testes, 0 falhas**.
- `crystalline-lint .`: zero violations (só o V7 órfão pré-existente).

## 3. Fase C — revalidação

- **Ocorrências** (mesmo método da auditoria, doc de 30 secções): sup —
  mediana **2.39pt** (era 11.39) vs vanilla **2.37pt**; 19 casos tight em
  1.9-2.4pt nos DOIS lados; a cauda (11-22pt) é mis-pairing
  cross-equação do instrumento, presente com a mesma forma no vanilla
  (que chega a 20.9). Sub — mediana **3.56pt** vs vanilla 3.55 (P959
  preservado, sem regressão).
- `compare.py` (secções 4/15/17/25/29): sec 15: 3.63 → **2.53**, sec 25:
  1.48 → **1.32**; med|dy| ≈ 0 mantido.
- Benchmark: ver tabela (hyperfine, "antes" = release pós-P962; JSONs em
  `tools/perf/results/p963-canonical/`).

| Cenário | antes (ms) | depois (ms) | ratio |
|---|---|---|---|
| 01-hello | 89.90 | 88.42 | 0.984 |
| 02-lorem | 107.23 | 107.40 | 1.002 |
| 03-images | 96.10 | 96.16 | 1.001 |
| 04-math | 122.32 | 120.70 | 0.987 |
| 05-tables | 91.24 | 92.90 | 1.018 |
| 06-long | 295.92 | 296.06 | 1.000 |
| 07-context | 127.98 | 131.58 | 1.028 |

Ratio médio **1.003** — sem regressão.
