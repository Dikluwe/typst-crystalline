# Relatório — Passo 975 (fecho da pendência P952 §6.4: medianas horizontais nas secções 4/25/28)

**Data:** 2026-08-05
**Proveniência**: HEAD no início = `77e921dcb` (P974). Referência vanilla
recompilada neste passo (`/usr/local/bin/typst` 0.15.1, 9dfd3a08) →
`temp/p975-vanilla.pdf`. Cristalino medido em `temp/p975-crys.pdf`
(antes) e `temp/p975-crys2.pdf` (depois do fix de IC). Benchmark:
`temp/p975/typst-antes` = release de P974.

## Fase A — estado actual e decomposição

Re-medição (`compare.py --secoes 4,25,28`, JSON em `temp/p975.json`):
sec 4 med|dx| 0.662 · sec 25 2.422 · sec 28 1.625 (234 flagged). As
medianas cruas não fecham nada — como o passo previa, a prova tem de ser
por decomposição com medição directa. Ordenando os pares flagged por
posição x, o padrão revelador: **dx assinado cresce linearmente com x
dentro de cada linha** — não é deslocamento de origem, é um **defice
cumulativo por glifo**. A caça à causa, por medição directa de casos
mínimos, encontrou **dois drivers reais** (e dois artefactos):

### Driver 1 — variantes de script `ssty` (`.st`/`.sts`) não aplicadas [→ P977]

O vanilla aplica `feat("ssty", 1|2)` por nível MathSize
(`text/mod.rs:1457-1460`); NewCMMath tem AlternateSubst GSUB com
`.st`/`.sts` para 1094 glifos. Medição exacta (`$ K_n $`, sub a 7.7pt):
cristalino **4.62pt** (advance do glifo base, 600du) vs vanilla
**5.44pt** (advance de `u1D45B.st`, 706du) — a largura renderizada de
cada lado bate ao centésimo com o advance da tabela correspondente.
Acumula ~0.8pt por glifo de script, daí o gradiente. **Não corrigido
neste passo** — requer métrica + render + subsetting (mapa char→gid do
export é style-cego; ToUnicode/widths por gid) com blast radius em todos
os glifos de script do documento ⇒ passo próprio com protocolo de dois
agentes: **`typst-passo-977.md`** (spec completa já medida).

### Driver 2 — advance de glifo math sem italics correction [CORRIGIDO aqui]

`$ tau(G) $`: o vanilla posiciona `(` 1.12pt mais à direita — exactamente
IC(𝜏)=102du a 11pt (medido na fonte; IC(𝑓)=90du). Vanilla
`update_glyph` (`fragment/glyph.rs:204-211`): `x_advance +=
italics_correction` para glifos singulares não-esticados. O cristalino
usava o advance puro do `hmtx`. Cada letra itálica math ficava ~IC mais
estreita — acumula em todas as equações com `f(x)`, `𝜏(𝐺)`, etc.

**Correcção** (fluxo contínuo ADR-0127 — comportamento interno de
métricas, sem mudar assinaturas): `FallbackFontMetrics::advance` e
`FontBookMetrics::advance` somam a IC quando `style.math && 1 carácter`;
`AdvanceWidthKey` ganhou `math: bool` (a chave não distinguia os
contextos). L0: `infra/font_metrics.md` §P975.

**Armadilha apanhada durante a validação** (e corrigida no mesmo passo):
o teste de layout directo passava mas o PDF continuava errado —
`fix_line_positions` (P582) revertia a posição porque para itens math o
`w_real − w_est` passou a ser a IC **por desenho** (o glifo desenhado
mantém o advance puro; a IC é espaço de layout). Itens `style.math`
deixam de contribuir para a reconciliação (continuam a receber shifts de
prosa na mesma linha). L0: `infra/shaper.md` §P975. Lição reforçada: a
asserção tem de ser **depois da pipeline completa** — o teste de
regressão principal (`p975_pipeline_preserva_ic_no_posicionamento`)
corre layout→bidi→shape→fix e mede no fim.

**Testes** (integração, fontes reais): `p975_advance_math_inclui_
italics_correction` (RED: 4.807 → GREEN: 5.929pt = 437+102du),
`p975_texto_multicaracter_math_sem_ic` (guarda: "sin" sem IC),
`p975_pipeline_preserva_ic_no_posicionamento` (RED antes do fix do
shaper: revertia para 4.806; GREEN depois). Suite: **5748 testes,
0 falhas** (+3).

**End-to-end**: `$ alpha(G) + tau(G) $` — `(` a 5.929pt após 𝜏
(vanilla: 5.929); `$ tau(G) $` renderiza 𝜏 e (𝐺) separados como o
vanilla (posições iguais a 0.002pt).

### Artefactos e achados adjacentes (veredictos registados)

- **(c) Artefactos de emparelhamento**: palavras fundidas num lado e
  partidas no outro (`𝜏(𝐺)` vs `𝜏`+`(𝐺)` — antes do fix; `𝑓(𝑥)d𝑥` vs
  `𝑓(𝑥)`+`d𝑥`) — compare.py emparelha por texto; limitação conhecida
  (P948/P949).
- **Escala de headings** [→ P978]: os maiores |dx| residuais na sec 28
  são **letras de headings** — `heading_scale` usa 2.0/1.667/1.333
  (vanilla: 1.4/1.2/1.0 em relativo ao tamanho corrente) e **perde-se por
  completo com `#set text(...)`** (headings do doc a 11pt vs 13.2pt no
  vanilla — mesmo ficheiro de fonte, medido por Tf no content stream).
  Fora do escopo math ⇒ `typst-passo-978.md` (spec escrita).
- **Numeração de equações em página auto-width**: o vanilla coloca o
  número logo após o conteúdo (a linha colapsa para a largura do
  conteúdo); o cristalino alinha à direita da largura computada ((34) a
  489.85 vs ~305-332). Divergência real mas **não entra na massa flagged**
  (os pares de números não são emparelhados pela ferramenta) — registada
  aqui para investigação futura, sem passo ainda.
- **Altura da página auto** (6666.6 vs 6882.75pt — o cristalino empacota
  216pt mais curto na vertical): divergência pré-existente de leading,
  fora do escopo; note-se que o compare.py lida com y por secção, logo
  não contamina os dx.

## Fase C — Revalidação

`compare.py --secoes 4,25,28` depois do fix de IC (sem ssty ainda):

| sec | med\|dx\| (compare.py) antes → depois | flagged antes → depois |
|---|---|---|
| 4 | 0.662 → **0.000** | 78 → 57 |
| 25 | 2.422 → **1.432** | 104 → 89 |
| 28 | 1.625 → **1.040** | 52 → 49 |

Total flagged: 234 → 195. O remanescente é dominado pelo driver 1 (ssty
— os top-|dx| actuais são glifos de linhas densas em scripts, o padrão do
gradiente) e por artefactos de fusão/emparelhamento; os pares de letras
de heading ficam para P978.

**Benchmark** (`benchmark-p975-canonical.py`, 7 cenários,
`tools/perf/results/p975-canonical/`): 01-hello 1.014 · 02-lorem 0.992 ·
03-images 1.000 · 04-math 0.981 · 05-tables 0.995 · 06-long 0.982 ·
07-context 0.991 — rácio médio **0.994**, zero regressão (a leitura
extra da tabela MATH por glifo math singular fica coberta pela cache de
P677, cuja chave ganhou `math`).

**Linter**: resselo dos ficheiros tocados; `crystalline-lint .` →
0 violations (só o V7 órfão pré-existente).

## Resultado

- Pendência P952 §6.4 **fechada por prova directa**: o padrão (gradiente
  cumulativo) tem duas causas reais medidas ao centésimo — ssty (→ P977)
  e IC no advance (corrigida aqui) — mais artefactos de emparelhamento
  classificados. Não é mais "atribuição estatística".
- Sec 4 a med|dx| 0.000; secções 25/28 melhoram ~40% e o restante fica
  com dono (P977/P978).
- Achados adjacentes registados com spec: headings (P978) e numeração de
  equações em auto-width (nota).
- Benchmark sem regressão; suíte verde; linter limpo.
