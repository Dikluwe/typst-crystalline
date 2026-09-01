# Prompt L0 — `math/layout/cancel` — `MathCancel`
Hash do Código: 81399128

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/math/callback-realization.toml sha256:4bf17f1455eef032ab3e30ea038edabed721e8378b913aaecf2b544bf288a917

**Camada**: L1 · **Alvo**: `01_core/src/compiler/math/layout/cancel.rs`
**Origem**: fatiado de `math/layout/mod.rs` em **P909**, completando o padrão de fatiamento
iniciado em P314 (ADR-0104) para `frac`/`root`/`stretchy`/`assembly`/`matrix`/`cases`/
`delimited`. `layout_cancel` foi adicionado em **P296**, depois de P314, e nunca tinha sido
movido. Núcleo partilhado: ver `math/layout/_comum.md`.

---

## Histórico P296 — heurística minimal (REVOGADA por P1291)

O bloco abaixo preserva a origem do handler, mas não é contrato vigente. Os
scope-outs de `inverted`/`cross`/`angle`/`stroke` foram fechados pelo contrato
full P1291 preservado em P1292.

`MathCancel` — layout do `body` seguido de uma linha diagonal sobre a bbox. Heurística minimal
per ADR-0054 graded:
- Diagonal default (canto inferior-esquerdo `(0, h)` → canto superior-direito `(width, 0)`;
  "rising", ângulo padrão do vanilla).
- Sem `inverted`/`cross`/`angle`/`stroke` cosméticos — scope-out para passo futuro dedicado a
  `MathCancel`.

`ascent`/`descent`/`width` do `MathBox` resultante são os do `body` — a linha não afecta as
métricas de caixa (é um item adicional sobreposto, não expande a caixa).

**Critério histórico revogado**: `MathCancel { body }` → `MathBox` com os items do `body` mais um `FrameItem::Line`
diagonal de `(0, h)` a `(width, 0)`, sem alterar `width`/`ascent`/`descent` do `body`.

## P986 — a linha usa a convenção baseline-relativa (quarto caso da família)

**Medição** (achado §7.3 da auditoria 2026-08-06; doc canónico secção 10,
`cancel(a + b)`, 600dpi): no vanilla a diagonal cruza POR DENTRO do texto
(início 0.68pt abaixo do topo, fim 10.3pt abaixo — efeito de risco); no
cristalino ficava INTEIRA abaixo do texto (8.8pt→17.3pt — efeito de
sublinhado), deslocamento de ~8pt ≈ `ascent+descent` do corpo.

**Causa**: a linha era emitida em coords "topo do MathBox" (`(0, h)` →
`(width, 0)`), mas os items de um `MathBox` vivem na convenção
**baseline-relativa** (`y=0` = baseline própria, negativo para cima —
ADR-0123, mesma família de erro de P901/P906/P919/P972 — quarto caso). Em
coords baseline-relativas, `(0, h)`→`(width, 0)` lê-se "da baseline até
`h` abaixo dela" — exactamente o sublinhado observado.

**Vanilla** (`typst-layout/src/math/cancel.rs:43-45,108-115`): a linha é
construída com o ponto médio no CENTRO do frame do corpo e estende-se pela
diagonal do frame — canto inferior-esquerdo → canto superior-direito da
**tinta do corpo**. Em coords baseline-relativas do cristalino:
`start = (0, body.descent)`, `end = (body.width, −body.ascent)`.

**Correcção**: `layout_cancel` emite a `FrameItem::Line` com esses
endpoints; o texto do critério original acima ("de `(0, h)` a `(width, 0)`")
fica **revogado** — era a descrição da convenção errada. `width`/`ascent`/
`descent` do `MathBox` continuam os do `body` (a linha não expande a caixa —
igual ao vanilla, que sobrepõe o frame da linha).

**Critério**: para um corpo com `ascent`/`descent` conhecidos, a linha vai de
`(0, descent)` a `(width, −ascent)`; no documento canónico, a diagonal cruza
o texto (risco), com início/fim ≈ vanilla (0.68/10.3pt abaixo do topo).

## P1132q — comprimento e espessura default completos

**Medição antes da decisão** (secção 10, working tree não commitado,
2026-08-22): o cristalino emite a diagonal de x=103,723pt a x=127,785pt,
com espessura 0,500pt. O vanilla ratificado emite de x=102,502pt a
x=129,747pt, com espessura 0,550pt. A caixa e os glifos do corpo já
coincidem; a diferença está exclusivamente no traço.

A fonte confirma dois defaults de linguagem em `math/cancel.rs`: `length =
100% + 0.3em`, relativo ao comprimento da diagonal da caixa, e `stroke.thickness
= 0.05em`. A direção continua sendo a diagonal dinâmica da caixa. Portanto,
para `w = body.width`, `h = body.ascent + body.descent`, `d = hypot(w,h)` e
`extra = 0.3em`, a extensão total adicional é projetada nos dois eixos:
`extra_x = extra*w/d`, `extra_y = extra*h/d`; metade fica em cada ponta. Os
endpoints baseline-relativos são `(-extra_x/2, descent+extra_y/2)` e
`(w+extra_x/2, -ascent-extra_y/2)`. A espessura é `0.05em`. Não entram
coordenadas do PDF nem constantes em pt.

O contrato P986 de canto a canto fica assim especializado: ele descreve a
parcela de `100%`; o default público acrescenta `0.3em` simetricamente.

## P1292 — consumo já materializado; sem segundo runtime

### Medição anterior à decisão

O consumer cristalino em `01_core/src/compiler/math/layout/cancel.rs:26-64`
recebe somente `body` e assa comprimento, direção e espessura default. O
vanilla ratificado mede primeiro a caixa, resolve o ângulo e o comprimento e
insere a linha atrás ou à frente em
`lab/typst-original/crates/typst-layout/src/math/cancel.rs:14-147`.

### Contrato preservado

`layout_cancel` recebe `&MathCancelElem`, não uma lista paralela de escalares.
O corpo é disposto uma vez para obter `width`, `ascent` e `descent`. O
comprimento resolve `Rel<Length>` contra a diagonal dessa caixa; a reta fica
centrada na caixa e não altera suas métricas. `inverted` reflete a direção no
eixo vertical. `cross` desenha as duas direções opostas e prevalece sobre
`inverted`. `background=true` insere a(s) linha(s) antes dos items do corpo;
`false` as insere depois. `stroke=None` deriva espessura `0.05em` e paint do
texto ativo; `Some(stroke)` preserva paint, espessura, cap, join e dash que o
modelo de `FrameItem` suportar, sem rebaixar silenciosamente o observável.

`MathCancelAngle::Angle` usa o ângulo explícito em relação ao eixo vertical;
`Auto` usa a diagonal ascendente dinâmica. Para `MathCancelAngle::Func`, cada
linha consulta/registra uma request pura em
`compiler/math/layout/callbacks.md` depois da medição. Uma resolução selada
correspondente fornece o ângulo; sem ela, o default desenhado é estritamente
provisório e a request impede exportação até realização L3. `cross=true`
produz duas requests/chamadas, uma por linha, como o vanilla; ambas recebem o
mesmo default positivo, cada resultado é independente e somente a geometria da
segunda linha é refletida. `cross` força a primeira linha não invertida e a
segunda invertida, ignorando `inverted`; sem `cross`, `inverted` reflete a linha
única depois da resolução. O resultado não é reutilizado artificialmente. O
fallback final `Func => Auto` seria divergência de linguagem.

O despacho estático e exaustivo continua no owner
`compiler/math/layout/_comum.md`; este arquivo permanece a unidade dona de toda
a geometria de cancel conforme ADR-0109.

P1292 apenas expõe este mesmo elemento por `math.cancel` e completa sua
morfologia. Os bits de presença de argumentos são ignorados geometricamente;
nenhum segundo `MathCancelElem`, callback store, algoritmo de linha ou caminho
de fallback é autorizado.
