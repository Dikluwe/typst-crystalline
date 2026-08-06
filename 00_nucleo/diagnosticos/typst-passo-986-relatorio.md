# Relatório — Passo 986: linha do `cancel` virou sublinhado (convenção de baseline)

**Estado do código das medições**: HEAD `b28631034` (P985) + alterações deste
passo. Commit final no fim.
**Gate ADR-0127**: não aplicável — correcção de paridade interna (fluxo
contínuo: L0 primeiro + resselo).

## Fase A — causa confirmada

Reprodução no doc canónico (`cancel(a + b)`, secção 10): vanilla — diagonal
cruza POR DENTRO do texto (banda de tinta única de 9.24pt); cristalino —
linha INTEIRA abaixo do texto (banda combinada de 16.2pt: texto + linha
deslocada ~8pt). Visual: risco vs sublinhado.

**Causa** (`cancel.rs:32-34`): a `FrameItem::Line` era emitida em coords
**topo-relativas** (`(0, h)` → `(width, 0)`, `h = ascent+descent`), mas os
items de um `MathBox` vivem na convenção **baseline-relativa** (`y=0` =
baseline própria, ADR-0123). Lido nessa convenção, o segmento ia "da
baseline até `h` abaixo dela" — exactamente o sublinhado observado.
**Quarto caso** da família de erro de convenção de baseline desta frente
(P901 radical, P906/P919, P972).

**Vanilla** (`typst-layout/src/math/cancel.rs:43-45,108-115`): a linha é
construída com o ponto médio no centro do frame do corpo e estende-se pela
diagonal do frame — canto inferior-esquerdo → canto superior-direito da
tinta. Equivalente baseline-relativo: `start = (0, body.descent)`,
`end = (body.width, −body.ascent)`.

**Varredura (Fase A.2 / C.3 do passo)**: produtores de `FrameItem::Line` no
lado matemático são 3 — `cancel.rs` (este bug), `frac.rs:125` (barra de
fracção) e `root.rs:171` (overline do radical) — estes dois já
baseline-correctos desde P905/P901 e revalidados visualmente em todas as
rondas desde então. Os restantes (`mod.rs` ×4) são transformadores
(`offset_item` etc.), não produtores. Os produtores do lado de texto
(`decorations.rs`, `footnote_flush.rs`, `link.rs`, `equation.rs`) vivem na
convenção de frames de página (pipeline de regiões), validada pelo render de
texto — `std.strike` medido igual ao vanilla (8.52pt = 8.52pt). **Decisão:
não há varredura ampla a fazer** — a frente matemática fica coberta por esta
correcção; a frente de texto não partilha a convenção de `MathBox`.

## Fase B — TDD

L0 primeiro: `cancel.md` §P986 (o critério original "`(0, h)` → `(width, 0)`"
fica revogado — era a descrição da convenção errada).

RED confirmado: `p986_cancel_linha_cruza_texto_convencao_baseline` falhou com
`start.y = 8.400` (= `h`, topo-relativo) vs esperado `descent = 0.0`. Mais
`p986_cancel_linha_nao_expande_caixa` (regressão do contrato P296 — já
passava, fica como guarda).

Implementação: 2 linhas em `cancel.rs` (endpoints baseline-relativos).

GREEN: **5781 testes, 0 falhas**. `crystalline-lint .`: 0 violations (só o V7
órfão pré-existente).

## Fase C — Revalidação

Doc canónico: banda de tinta do `cancel(a+b)` passou de 16.2pt (texto + linha
deslocada) para **8.64pt** — linha dentro da banda do texto, como o vanilla
(9.24pt; a diferença de 0.6pt vem dos endpoints do vanilla irem até aos
cantos do frame, que inclui bearings). Confirmação visual a 600dpi: diagonal
cruzando `a+b` de canto inferior-esquerdo a superior-direito — efeito de
risco, idêntico ao vanilla (`temp/p984/sec10c2-1.png` vs `sec10v-1.png`).
`std.strike` inalterado (8.52pt = vanilla).

Benchmark canónico (`benchmark-p986-canonical.py`, antes = release P985):
ratios 0.990–1.024, **média 1.005 — sem regressão**.
