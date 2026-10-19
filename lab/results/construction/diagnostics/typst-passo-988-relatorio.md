# Relatório — Passo 988: `⎵_⎵` sem gap corrigido + acento sem compensar itálico

**Estado do código das medições**: Parte A sobre HEAD `b5e84354c` (P987);
Parte B sobre HEAD `30f7389f8` (P989) — Parte A já commitada entretanto.
Commits finais no fim de cada parte.
**Gate ADR-0127**: Parte A não aplicável (correcção de fórmula interna,
caminho de assembly já documentado). Parte B **parou** para confirmação do
dono (contrato público: método novo `top_accent_attach` no trait
`FontMetrics`) — **aprovado em 2026-08-06** (ver `accent.md` §P988-B: "o
fallback `(w+IC)/2` não reproduz o valor — x itálico tem IC ausente mas
TopAccent=287du").

---

## Parte A — `⎵_⎵` com o mesmo bug que P985 corrigiu em `⏟`

### Fase A — causa
Achado §8.3 da auditoria (2026-08-06; bandas a 600dpi no doc canónico):
`underbracket(a+b+c)` — gap conteúdo↔suporte cristalino 9.84pt vs vanilla
0.84pt (`overbracket` já correcto: 0.72=0.72). P985 corrigiu
`emit_horizontal_variant` (variante única), mas `⎵` de NewCMMath-Book só tem
variantes até 2986du; `a+b+c` mede ~3850du → cai no caminho de **assembly
horizontal**, irmão separado que replicava o bug pré-P985:
`layout_assembly_horizontal` terminava com `vertical_metrics` da fonte
(ascent 0.8em, descent 0) em vez da tinta real das peças.

### Fase B — TDD
L0 primeiro (`assembly.md` §P988). Correcção: `ascent`/`descent` da caixa
passam a ser a união da tinta real das peças (`FontMetrics::glyph_ink_bounds`,
valores COM SINAL — `up` negativo quando a tinta fica toda abaixo da
baseline, caso das peças de `⎵`), não mais `vertical_metrics`. Posicionamento
das peças inalterado.

GREEN: 5786 testes, 0 falhas. `crystalline-lint .`: 0 violations.

### Fase C — Revalidação
`underbracket(a+b+c)`: gap 9.84pt → 0.96pt (vanilla 0.84pt) — mesma família
de correcção de P985, agora cobrindo o caminho de assembly.

**Commit**: `9ec038a1c` — "fix(math): assembly horizontal com métricas da
tinta das peças — Passo 988 (Parte A)".

---

## Parte B — acento não desloca para compensar inclinação do itálico

### Fase A — causa
Achado §8.4 da auditoria: vanilla desloca `hat(x)` ~0.5pt para a direita do
centro da caixa do "x" — compensação de itálico. Cristalino centrava
exactamente no centro geométrico (`dx = (base.width − accent.width)/2`), sem
compensação. Mecanismo real do vanilla (`accent.rs:37-52` +
`fragment/glyph.rs:222-224`): `accent_x = base_attach − accent_attach`, onde
cada `attach` vem de `TopAccentAttachment` da tabela MATH (fallback
`(advance+IC)/2` fora da cobertura). **Decisão pendente identificada**: isto
exige um método novo no trait `FontMetrics` (mudança de contrato público) —
parado para gate ADR-0127.

**Gate**: aprovado pelo dono em 2026-08-06. Justificação registada em
`accent.md` §P988-B: o fallback puramente geométrico `(w+IC)/2` não
reproduz o valor real da tabela — medição concreta: x itálico matemático
(U+1D44E) tem `IC` ausente mas `TopAccentAttachment` = 287du (não
equivalentes).

### Fase B — TDD
L0 primeiro (`accent.md` §P988-B, hash resselado). Implementação:
- `engine/layout/metrics.rs` — `FontMetrics::top_accent_attach(c, size,
  style) -> Option<Pt>`, default `None` (métricas sintéticas → caller cai na
  centragem simples pré-P988, comportamento inalterado).
- `03_infra/src/font_metrics.rs` — implementação em `FontBookMetrics` e
  `FallbackFontMetrics`: valor da tabela `MathTopAccentAttachment` na
  cobertura; fallback `(advance+IC)/2` fora dela; `None` se o char não
  existir na face. `FallbackFontMetrics` usa a mesma resolução de face de
  `text_ink_bounds` (`resolve_primary_with_math_fallback`+`covering`) e
  aplica variante ssty (P977).
- `01_core/src/engine/math/layout/accent.rs` — `dx = base_attach −
  accent_attach` para bases/acentos de 1 carácter (via
  `FontMetrics::top_accent_attach`); bases multi-carácter ou sem dado de
  tabela caem no fallback `largura/2` (degenera para a centragem simples,
  o fallback do vanilla para fragmentos compostos, `fragment/mod.rs:149`).

Testes novos: `p988b_top_accent_attach_fonte_real` (L3, fonte real —
x itálico 287du≈3.157pt a 11pt, hat 250du≈2.75pt a 11pt),
`p988b_acento_deslocado_por_top_accent_attachment` (dx=0.444pt com attaches
sintéticos 3.444/3.0), `p988b_sem_attach_cai_na_centragem_simples` (guarda de
regressão — sem dados de tabela, dx=0.0 = centragem pré-P988).

GREEN: 5709 testes (`typst-core` 4922 + `typst-infra` 787), 0 falhas.
`crystalline-lint .`: 0 violations (só V7 órfão pré-existente, não
relacionado).

### Fase C — Revalidação
Validado nesta sessão em conjunto com o Passo 990 (mesma árvore, commit
seguinte): benchmark canónico de 7 cenários rodado sobre a árvore combinada
P988-B+P990 (`benchmark-p990-canonical.py`, antes=`30f7389f8`) — ratio médio
1.006, sem regressão sistemática (detalhe em
`typst-passo-990-relatorio.md`). Não foi isolado um benchmark só de P988-B;
o efeito esperado (deslocamento horizontal de ~0.5pt em acentos) é
localizado e não é o tipo de mudança que historicamente move os 7 cenários
canónicos (nenhum é math-pesado o suficiente para acentos dominarem o
tempo).

**Commit**: `59d2a2f3d` — "fix(math): centragem horizontal de acento via
TopAccentAttachment — Passo 988 (Parte B)".
