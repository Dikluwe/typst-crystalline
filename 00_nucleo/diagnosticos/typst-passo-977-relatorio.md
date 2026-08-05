# Relatório — Passo 977 (variantes ssty `.st`/`.sts` em scripts math)

**Data:** 2026-08-05
**Proveniência**: HEAD no início = `ec7707624` (P976). Benchmark:
`temp/p977/typst-antes` = release de P975.

## Fase A — pontos de injecção (mapeados antes de escrever código)

O desenho saiu mais simples que o temido na spec: o texto math **já passa
pelo shaper** (`shape_item` cobre `Text` com `style.font.is_some()` —
confirmado por comportamento em P975), logo a substituição no render não
precisa de cirurgia no subsetting — os glifos shaped entram pelo mecanismo
existente. Dois lados a mudar, em concordância (lição P772o):

1. **Shaper** (`shaper.rs::try_shape`): `rustybuzz::shape` era chamado com
   features vazias e script por detecção. Descoberta-chave da Fase A: em
   NewCMMath a `ssty` é a feature #10, registada **só no script OpenType
   `math`** (o script `latn` tem [1,2,4-9]); o vanilla shape texto math
   com `buffer.set_script(math)` (`typst-layout/src/math/shaping.rs:200`).
   Sem o script certo, a feature nem sequer é considerada — confirmado por
   sonda directa ao rustybuzz (gid inalterado sem o script; substituído
   com ele).
2. **Métricas** (`font_metrics.rs`): `advance`/`text_ink_bounds`/
   `text_ink_bounds_signed` ganham a substituição ssty por leitura directa
   da GSUB via ttf-parser (`gsub::AlternateSubstitution` —
   `features.find("ssty") → lookups → coverage → alternate_sets[i]
   [level−1]`), aplicada quando `style.math && math_size ∈ {Script,
   ScriptScript}`. `AdvanceWidthKey` ganhou `math_size`. Interacção com
   P975: a IC lê-se do glifo **substituído** (os `.st` medidos não têm
   entradas de IC — o termo desaparece em scripts, como no vanilla).

Sem mudança de assinaturas (o `style` já carrega `math_size`, P945) —
fluxo contínuo ADR-0127 (correcção de paridade), sem gate.

## Fase B — implementação e testes

Testes de integração (fontes reais; RED confirmado nos dois primeiros
antes da implementação):

- `p977_subscript_usa_variante_st` — sub de `$ K_n $`: 4.620 → **5.436pt**
  (advance `.st` = 706du a 7.7pt, medido na fonte).
- `p977_scriptscript_usa_variante_sts` — asserção no **glyph id**
  (u1D467.sts = gid 5784): desacopla do achado adjacente abaixo.
- `p977_base_size_e_prosa_inalterados` — guardas (math em corpo e prosa
  ficam no glifo base).

**Achado adjacente registado** (não corrigido aqui — fora do escopo):
`attach.rs:44-45` calcula os tamanhos de script com o factor plano
×`script_percent` — o scope-out de P945 ("o factor NÃO muda neste passo")
que P970 já corrigiu para o índice de raiz. Medido: sub-sub a 5.39pt
(7.7×0.7) em vez de 5.5pt (11×0.5 absoluto do vanilla). Fica para passo
próprio (mesma classe de P970 — factor por nível em attach).

Suite completa: **5751 testes, 0 falhas** (+3; a sonda rustybuzz usada na
Fase A foi removida depois de cumprir o papel).

## Fase C — Revalidação

- **Casos da Fase A de P975**: sub de `$ K_n $` a **5.44pt = vanilla**;
  cluster `k=1` de `∑_(k=1)^n` a **15.05pt = vanilla** (era 13.85);
  `$ sum_(k=1)^n k $` e `$ K_n, C_n, P_n, Q_n $` medidos palavra a
  palavra — todas as posições a Δ≤0.2pt do vanilla excepto um residual de
  0.93pt no último elemento da linha K_n…Q_n (provável interacção
  IC×attach em `Q_n` — registado, sub-ponto).
- **compare.py sec 4/25/28** (a pendência de P952 §6.4):
  - sec 4: med|dx| 0.662 → 0.000 (P975) → **0.000**; flagged 78 → 53.
  - sec 25: 2.422 → 1.432 → **0.616**; flagged 104 → 88.
  - sec 28: 1.625 → 1.040 → **0.000**; flagged 52 → 33.
  - Total flagged: 234 → 195 → **174**. O remanescente: letras de
    heading (P978), pares de numeração/artefactos de fusão (ferramenta).
- **Benchmark** (`benchmark-p977-canonical.py`, 7 cenários,
  `tools/perf/results/p977-canonical/`): 01-hello 1.008 · 02-lorem 0.993 ·
  03-images 0.996 · 04-math 0.991 · 05-tables 0.990 · 06-long 1.004 ·
  07-context 1.006 — rácio médio **0.998**, zero regressão (a leitura GSUB
  por glifo fica coberta pelas caches de advance/shaping).
- **Linter**: resselo dos ficheiros tocados; `crystalline-lint .` →
  0 violations (só o V7 órfão pré-existente).

## Resultado

- Scripts math renderizados com as variantes `.st`/`.sts` desenhadas para
  o efeito — paridade tipográfica com o vanilla (medida ao centésimo).
- A pendência de P952 §6.4 fica essencialmente fechada: duas das três
  secções a med|dx| 0.000; a terceira a 0.616 com o remanescente já com
  dono (P978/artefactos).
- Residual registado: factor plano de tamanho em `attach.rs` (scope-out
  P945) — candidato natural a P979+.
- Benchmark sem regressão; suíte verde; linter limpo.
