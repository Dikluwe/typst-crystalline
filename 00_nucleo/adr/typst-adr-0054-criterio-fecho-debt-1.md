# ⚖️ ADR-0054: Critério de fecho de DEBT-1 inclui consumo integral

**Status**: `EM VIGOR`
**Data**: 2026-04-24
**Autor**: Humano + IA
**Diagnóstico prévio**:
[`00_nucleo/diagnosticos/diagnostico-shaping-passo-135.md`](../diagnosticos/diagnostico-shaping-passo-135.md)

---

## Contexto

**DEBT-1** foi aberto no Passo 30 com âmbito "StyleChain +
propriedades adicionais para paridade total com o sistema de
styles do original" (DEBT.md linha 48). Interpretação usada nos
Passos 30–134: captura suficiente.

Os passos **126–134** capturaram a lista canónica completa
(`text.weight`, `text.lang`, `par.leading`, `text.font` incluindo
simbólico e agregado). Pós-134 os campos estão em `StyleDelta`
mas **5 de 10 são inertes**: layout actual usa `TextStyle`
plano que cobre só 5 campos (`bold, italic, size, fill,
heading_level`). `#set text(weight: 700)` é capturado mas PDF
é idêntico a sem o `#set`.

**ADR-0033** (paridade funcional) lido literal inclui output,
não só input processing. O diagnóstico do Passo 135 formalizou
o gap.

Esta ADR **redefine o critério de fecho de DEBT-1** para
incluir consumo integral.

## Decisão

DEBT-1 só fecha quando:

1. **Cada propriedade de `StyleDelta`** tem consumer em
   layout/export (ou é explicitamente marcada como scope-out
   com ADR de suporte).
2. **Output PDF observacional** é equivalente ao vanilla para
   inputs equivalentes (dentro dos limites do perfil de
   paridade adoptado — ver secção "Perfil de paridade" abaixo).
3. **DEBT-52** (rastreador aberto em Passo 135) encerra.

Captura sem consumer é **estado intermédio**, não fecho.

### Perfil de paridade

Três perfis possíveis, da mais restrita à mais permissiva:

1. **Bit-perfect**: output PDF binariamente idêntico.
   **Inalcançável** (font embedding, compressão, IDs).
2. **Visual**: render idêntico pixel-a-pixel depois de
   rasterizar. **Parcialmente alcançável** — mas sem
   rustybuzz, shaping real (ligatures, kern, bidi) diverge.
3. **Observacional graded**: métricas observáveis equivalentes
   (tamanho, cor, peso, espaçamento) para inputs de teste
   documentados, **sem garantia de shaping features**.
   **Alcançável em 4-8 passos**.

**Escolha**: **perfil observacional graded**.

Rustybuzz + shaping completo fica **explicitamente fora** do
critério de fecho de DEBT-1 (documentado como DEBT-52 fase
D/E opcional). Se no futuro for integrado, DEBT-1 pode
"reabrir" o critério para "visual", ou criar DEBT separado.

## Alternativas consideradas

| Alternativa | Prós | Contras |
|-------------|------|---------|
| **(a)** Nota em ADR-0033 | Minimalista, não duplica | Enterra decisão de fecho sob princípio geral |
| **(b)** ADR-0054 dedicada ✓ | Formaliza mudança com precedência vanilla 0052/0053 | Mais uma ADR |
| **(c)** Só em DEBT-52 sem ADR | Zero ADR | Decisão perde peso; revisões futuras podem não a ver |

**Escolha**: (b). Mudança de critério de fecho de DEBT central
merece ADR própria. Precedente: ADR-0052 (Lang) e ADR-0053 (Font)
materializaram tipos como decisões próprias; esta materializa
critério como decisão própria.

## Consequências

### Positivas

- **Critério de fecho claro**: "capturou" deixa de ser
  suficiente. "Afectou output" torna-se obrigatório.
- **Base para DEBT-52**: o rastreador ganha âmbito claro.
- **Expectativa ajustada**: roadmap DEBT-1 expande
  significativamente. Fecho estimado ~4-8 passos adicionais
  (fase A/B/C do diagnóstico).
- **Perfil observacional graded** honesto: reconhece limites
  do stack actual (sem rustybuzz) sem esconder gap.
- **Precedente para futuros DEBTs fundamentais**: captura vs
  efeito é distinção explícita.

### Negativas

- **DEBT-1 permanece aberto mais 4-8 passos** (versus fecho
  imediato em 135 conforme roadmap original).
- **ADR-0033 ganha dimensão observável**: leitura mais estrita
  pode afectar critérios de outros DEBTs pendentes. Aceite.
- **DEBT-52 grande em escopo**: rastreador com muitos gaps.
  Mitigado: cada gap é passo dedicado S/M.

### Neutras

- **rustybuzz fica explicitamente fora de DEBT-1**: não é
  degradação nem escolha arquitectural — apenas delimitação
  realista do que 4-8 passos podem atingir.
- **TextStyle permanece como ponte**: DEBT-48 continua
  encerrado; extensão de TextStyle (fase A) é caminho menos
  resistência, não refactor fundacional.

## Plano de materialização (roadmap DEBT-52)

Resumido do diagnóstico 135 secção 4:

1. **Fase A (XS, 1 passo)**: estender `TextStyle` com
   `weight/tracking/leading/lang/font` + propagar via
   `From<&StyleChain>`.
2. **Fase B (S, 3 passos)**: consumers `tracking`, `leading`,
   `weight` faux-bold.
3. **Fase C (M, 4-5 passos)**: consumers `font` string/array,
   `lang` hyphenation, PDF font embedding.
4. **Fase D (opcional)**: font dict → ADR-0054bis autorizar
   `regex`.
5. **Fase E (opcional, escopo XL)**: rustybuzz integration —
   provavelmente série dedicada fora de DEBT-1.

DEBT-1 fecha quando **A + B + C** encerrarem. D/E não bloqueiam.

## Referências

- **DEBT-1** (DEBT.md) — alvo do critério.
- **DEBT-52** (aberto em 135) — rastreador de gaps.
- **ADR-0033** (paridade) — princípio geral.
- **ADR-0038** (StyleChain) — base estrutural.
- **ADR-0052** (Lang) / **ADR-0053** (Font) — precedentes.
- **Passo 135** diagnóstico.
- **DEBT-48** (ENCERRADO) — `TextStyle` como ponte aceite.
- **ADR-0082** (PROPOSTO P249) — formaliza pattern "Promoções
  reais scope-outs ADR-0054 graded" (4 critérios operacionais).

---

## §Promoções reais cumulativas (refino interno P249)

**Pós-P249**: o perfil graded permite **promoção real** de
scope-outs declarados (refino futuro per ADR-0054 graded
documentado em "Limitações conscientes" do passo de origem).

Tabela cumulativa pós-P250:

| # | Passo | Scope-out promovido | Origem (graded) |
|---|-------|---------------------|-----------------|
| 1 | P242 | `radius` (Block + Boxed) | P156G + P156H scope-out |
| 2 | P242 | `clip` (Block + Boxed) | P156G + P156H scope-out |
| 3 | P247 | `outset` semantic real (Block + Boxed) | P156G + P156H + P231 graded |
| 4 | P247 | `fill` (Block + Boxed) | P156G + P156H scope-out |
| 5 | P247 | `stroke` (Block + Boxed) | P156G + P156H scope-out |
| 6 | P248 | `Block.breakable` semantic real | P156G "semantic adiada" |
| 7 | P248 | `Boxed.height` overflow real | P156H "semantic adiada" |
| 8 | P248 | `TableCell.body` overflow clip implícito | P157B "ignorados em layout" |
| 9 | P250 | `Block.spacing` (cursor.y collapse) | P156G scope-out |
| 10 | P250 | `Block.above` (override spacing) | P156G scope-out |
| 11 | P250 | `Block.below` (override spacing) | P156G scope-out |
| 12 | P250 | `Block.sticky` (lookahead break) | P156G scope-out |
| 13 | P251 | `TableCell.body` overflow row break real cell-level | P157B "clip implícito P248" → row break γ-Items |
| 14 | P252 | `Boxed.stroke-overhang` semantic real (bounds Shape expandidos thickness/2 quando overhang=true) | P156H "rejeitado em native_box" → Stroke +1 field + activação Layouter |

**Marco P250**: Block A.4 COMPLETO 10/10 scope-outs originais
P156G fechados cumulativamente (incluindo breakable contado
como elemento original).

**Marco P251**: Categoria C.2 Fase 5 Layout activada parcialmente
(cell-level row break via γ-Items); multi-region completo
(column flow DEBT-56) continua diferido. Pattern "Slice frame
items at height" N=1 inaugurado.

**Marco P252**: **Boxed A.4 COMPLETO 6/6** — segundo variant
Content com 100% scope-outs originais P156H fechados
cumulativamente (após Block P250 10/10). Pattern "Refactor
cross-cutting entity primitivo" N=1 inaugurado.

**Divergência consciente P252**: construtor Rust low-level
`Stroke { paint, thickness, overhang: false }` é divergência
consciente face vanilla default `true`. Paridade user-facing
restaurada via stdlib `extract_stroke` helper (defaults
`overhang: true` para inputs Length/Color atalhos + Dict sem
chave explícita). Justificações cumulativas: backward compat
literal estrita pré-P252 (~42 construtores literais
preservados); anti-inflação 44ª (defaults zero-impact
construtor low-level paridade pattern P247 fill).

**Padrão metodológico de promoção formalizado em ADR-0082
PROPOSTO** (P249 administrativo XS): 4 critérios operacionais
(storage prévio + consumer Layouter graded + paridade vanilla
referência + backward compat literal).

**ADR-0054 status `EM VIGOR` preservado** — refino interno
secção nova apenas; não reaberta nem revogada. ADR-0082
formaliza metodologia downstream sem alterar perfil graded.

---

## Anotação cumulativa P266 — Cobertura Text empírica confirmada (Fase A audit; primeiro consumo directo ADR-0084 + 0085)

**Data**: 2026-05-15.

P266 Fase A audit confirmou cobertura empírica Text módulo
~86% ponderado linear (~92% ponderado com bonus implementado⁺).
**Primeiro consumo directo formal ADR-0084 + ADR-0085** pós-P260
formalização — validação retrospectiva via exercício real
num módulo grande.

### Cobertura empírica Text Fase A (40 entradas)

| Estado | Audit P266 |
|--------|------------|
| implementado | 21/40 (52%) |
| implementado⁺ | **11/40 (28%)** |
| parcial | 1/40 (3%) |
| ausente | 5/40 (12%) |
| promoção arquitectural (Strong/Emph via Styled) | 2/40 (5%) |
| TOTAL | 40 |

**Fechados literais**: 34/40 = **85%**.
**Ponderada linear**: **86.25%**.
**Ponderada com bonus implementado⁺**: **91.75%**.

### Promoções implementado⁺ detectadas (+10 vs pré-audit)

Consumers reais materializados em P128/P137/P139/P144/P155:
- A.8 tracking PDF Tc emit (P137).
- A.9 leading line_height (P128).
- A.12 lang hyphenation + smart-quotes (P144 + P155).
- B.1 Text + StyleChain Layouter consumer.
- B.10 Smart-quotes lang-aware (P155).
- D.2 Hyphenation greedy (P144 consumer).
- E.1 Faux-bold (P139 consumer).
- E.2 Tracking PDF (P137 consumer).
- E.3 Leading (P128 consumer).
- E.4 Hyphenation greedy break (P144 consumer cursor).

### Pendências preservadas pós-P266

1. **Shaping completo rustybuzz** — preservado (sem DEBT
   dedicada; era ref P266 spec mas DEBT-53 está ENCERRADO
   P206E para outro tópico). Candidato XL futuro per ADR-0054
   §"granularidade gradual"; sem DEBT novo per política P158.
2. **C.5 Variant-aware font selection** — `FontVariant::default()`
   literal em resolve_font; substitui faux-bold P139 onde
   font-file dedicado existe. Candidato **P267 Opção 1**
   (M; ADR-0055bis ou ADR-0089).
3. **C.6 Font subsetting PDF** — TTF complete embedded.
   Candidato **P267 Opção 2** (M-L; ADR-0056).
4. **D.4 Shaping rustybuzz** + **D.5 Bidi RTL** — scope-out
   ADR-0054 graded preservado.
5. **B.7 Content::Link parcial** — refino qualitativo
   (PDF annotation futuro).
6. **B.9 Content::Parbreak** — promoção arquitectural implícita
   via parser whitespace duplo (não variant Content explícito;
   paridade vanilla ParbreakElem delegada ao layouter cursor/
   spacing).

### Achados inesperados

- **+10 promoções implementado⁺** vs pré-audit (consumers
  reais P128/P137/P139/P144/P155 não documentados pré).
- **Parbreak ausente como variant** (era "a confirmar";
  confirmado emergente do parser).
- **Strong/Emph promoção arquitectural** P101 ADR-0038/0039
  (variant explícito → `Content::Styled` wrapper).
- **Spec P266 referência DEBT-53 errada** — DEBT-53 está
  ENCERRADO P206E para "Integração pipeline vanilla lab/parity"
  (não shaping). Shaping rustybuzz preservado scope-out
  ADR-0054 §"granularidade gradual" sem DEBT formal dedicada.
- **10 fields StyleDelta** (não 12 esperados pré-audit) —
  bold/italic/size/fill/heading_level/weight/tracking/leading/
  lang/font.

### Status ADR-0054 preservado literal

Status `EM VIGOR` preservado. Esta anotação documenta cobertura
empírica P266 + validação ADR-0084/0085 sem reabrir nem revogar
ADR-0054.

### Subpadrões cumulativos pós-P266

- **"Auditoria condicional" N=5 → N=6 cumulativo** (P192A +
  P255 + P257 + P258 + P259 + **P266**) — **primeiro consumo
  directo formal pós-P260**.
- **"Diagnóstico imutável precedente à acção" N=6 → N=7
  cumulativo** (P255-259 audit Fase A N=4 + P262 + P264 + **P266**;
  Note: contagem ajustada — P262/P264 foram diagnósticos
  vanilla; P266 é audit Fase A formal).
- **"Cobertura empírica > citada"** confirmada N=3 (P257 +
  P258 + **P266**; vs P259 que foi -8 a -13pp).
- **"Hipótese auditável Text padrão Color/Model"** confirmada
  empíricamente: pré-audit ~52% citado vs ~86% empírico
  (Δ +34pp).

Cross-references:
- `00_nucleo/diagnosticos/diagnostico-text-fase-a-passo-266.md`
  — diagnóstico imutável P266.A.
- ADR-0084 + ADR-0085 (P260 formalização; primeiro consumo
  directo P266).
- `00_nucleo/prompts/entities/style_chain.md` — secção
  cumulativa P266 anotada.
- P259 — Visualize Fase A (último audit pré-formalização P260;
  template literal directo P266).

---

## Anotação cumulativa P268.1 — Divergência arquitectural Conic PDF industry-aligned

**Data**: 2026-05-15.

P268.1 — divergência arquitectural Conic PDF Type 4 cristalino vs
estratégia vanilla actual desconhecida (krilla `SweepGradient`
interno opaco; Typst original pré-krilla era Type 6 Coons per blog
2023) formalizada via ADR-0090 EM VIGOR; cluster Gradient PDF
mantém-se 3/3 em estratégia conservadora industry-aligned
(Cairo/Inkscape/Typst original precedent — todos família mesh).
ADR-0018 preservado.

Status `EM VIGOR` preservado literal. Esta anotação documenta que o
cluster Gradient PDF cristalino — embora divergente intra-família
mesh (Type 4 vs Type 6 Typst original / Type 6-7 Cairo) — está
alinhado com industry mesh-based standard; perfil graded ADR-0054
mantém-se válido para Visualize/Gradient porque a divergência é
estratégica (simplicidade implementação Type 4 vs Type 6;
compatibilidade reader) e não simplificação per se.

Cross-references:
- ADR-0090 — Gradient Conic PDF strategy Type 4 vs Type 1
  (EM VIGOR P268.1).
- ADR-0089 — Gradient Conic-only L1+stdlib (anotação cumulativa
  P268.1 cross-reference ADR-0090).
- P268 — PDF Conic Type 4 Gouraud materializado.
- P268.2 (futuro) — refino adaptive N hybrid; spec dedicada.

---

## Anotação cumulativa P268.2 — Refino adaptive N hybrid 1+2 (cluster Gradient PDF industry-grade)

**Data**: 2026-05-15.

P268.2 — refino adaptive N hybrid 1+2 materializa qualidade visual
Type 4 Gouraud sem mudar estratégia ADR-0090; cluster Gradient PDF
qualitativamente industry-grade. Perfil graded DEBT-1 preservado
(refino é optimização local, não simplificação).

Status `EM VIGOR` preservado literal. Esta anotação documenta que o
cluster Gradient PDF cristalino — após P268.2 — apresenta qualidade
visual industry-grade (banding eliminado em casos extremos via
adaptive N hybrid 1+2 calibrado para Oklab canónico). Perfil graded
ADR-0054 mantém-se válido porque refino paramétrico é optimização
local; ADR-0090 (estratégia Type 4) intocada.

Cross-references:
- ADR-0089 — Gradient Conic-only L1+stdlib (anotação cumulativa
  P268.2 com fórmula completa + factor_delta=256.0 calibrado).
- ADR-0090 — Type 4 Gouraud strategy (preservada literal por P268.2;
  só parâmetro N refinado).
- `00_nucleo/diagnosticos/diagnostico-adaptive-n-passo-268-2.md` —
  diagnóstico imutável P268.2.A (sexto consumo directo de fonte;
  primeiro consumo de literatura técnica perceptual).
- P268 — PDF Conic Type 4 Gouraud N=32 fixo (precedente refinado).
- P268.1 — ADR-0090 EM VIGOR (preservada literal por P268.2).

---

## Anotação cumulativa P269 — Gradient Radial focal_* activado (cluster extensão completa)

**Data**: 2026-05-15.

P269 — cluster Gradient Radial focal_* materializado L1+stdlib+PDF
(focal_center + focal_radius activados); ADR-0088 §"Scope-outs
documentados" §focal_* revogado parcialmente. Perfil graded DEBT-1
preservado (activação per ADR explícita; defaults preservam P264
zero regressão).

Status `EM VIGOR` preservado literal. Esta anotação documenta que
o cluster Gradient cristalino — pós-P269 — cobre Radial com 5
campos materializados (stops + center + radius + focal_center +
focal_radius) vs 3 campos P264. Cluster Gradient agora tem todas
3 variants principais (Linear/Radial/Conic) com features completas
L1+stdlib+PDF para gradient real-world workflows.

Sub-padrão "ADR scope-out revogado parcialmente" N=1 → **N=2** (P267
Conic + **P269 focal_***); candidato meta-formalização futura
se N≥3.

Cross-references:
- ADR-0088 — Gradient Radial-only (anotação cumulativa P269 com
  fórmula completa + defaults + validações stdlib portadas).
- `00_nucleo/diagnosticos/diagnostico-gradient-focal-passo-269.md`
  — diagnóstico imutável P269.A (sétimo consumo directo de fonte
  vanilla).
- P264 — Radial L1+stdlib (precedente directo extendido).
- P265 — PDF Radial /ShadingType 3 (template emit extendido).
- P267 — Conic activado (precedente "ADR scope-out revogado
  parcialmente" N=1).

---

## Anotação cumulativa P270 — Gradient ColorSpace runtime cross-variant L1+stdlib

**Data**: 2026-05-17.

P270 — cluster Gradient extensão ColorSpace runtime cross-variant
activado L1+stdlib (3 variants × 8 spaces); ADR-0083 §"ColorSpace
runtime" scope-out revogado parcialmente; perfil graded DEBT-1
preservado (activação per ADR explícita; defaults Oklab preservam
P262/P264/P267 zero regressão bit-exact).

Status `EM VIGOR` preservado literal. Esta anotação documenta que o
cluster Gradient L1+stdlib cristalino — pós-P270 — cobre **3 variants
× 8 spaces** materializados (24 combinações cross-variant × space).
L3 emit refactor adiado P270.1 + P270.2; cluster L1+stdlib feature-
complete.

Sub-padrão "ADR scope-out revogado parcialmente" N=2 → **N=3 cumulativo**
(P267 Conic + P269 focal_* + **P270 ColorSpace**). **Atinge limiar
formalização clara**; candidato meta-formalização futura.

Cross-references:
- ADR-0091 — Gradient ColorSpace runtime + CMYK strategy (criada
  PROPOSTO+IMPLEMENTADO P270).
- ADR-0083 — Color paridade (anotada cumulativa P270; §ColorSpace
  runtime revogado parcialmente).
- ADR-0087/0088/0089/0090 — Variant strategies (anotadas cumulativa
  P270; preservadas em estratégia).
- `00_nucleo/diagnosticos/diagnostico-gradient-space-passo-270.md`
  — diagnóstico imutável P270.A (oitavo consumo directo de fonte
  vanilla).
- P262/P264/P267 — Linear/Radial/Conic L1+stdlib Oklab hardcoded
  (precedentes directos extendidos).
- P269 — Radial focal_* activated (preservado; campo space adicional
  cross-variant).

---

## Anotação cumulativa P270.1 — Gradient L3 emit multi-space materializado (7/8 spaces)

**Data**: 2026-05-17.

P270.1 — cluster Gradient L3 emit feature-complete **7/8 spaces** (Oklab/
Oklch/sRGB/Luma/LinearRGB/HSL/HSV); CMYK último P270.2; perfil graded
DEBT-1 preservado (refino L3 sem mudar estratégia ADR-0087/0088/0089/
0090).

Status `EM VIGOR` preservado literal. Esta anotação documenta que o
cluster Gradient cristalino — pós-P270.1 — cobre 3 variants × 7 spaces
em L3 emit completamente (21 combinações materializadas); CMYK
adicional sub-óptimo via pipeline natural CMYK→sRGB (P270.2 fecha
com `/DeviceCMYK` directo).

Descoberta arquitectural P270.1.A: **P270 já passou L3 multi-space
implicitamente** via `<variant>.sample(t)` dispatcher P270. P270.1 é
maioritariamente cosmético (rename helpers + docs + tests).

Cross-references:
- ADR-0091 §"Anotação cumulativa P270.1" — fórmula completa Op B
  uniforme materializada.
- ADR-0087/0088/0089/0090 — Variant strategies anotadas cumulativa
  P270.1 (preservadas; helpers L3 renomeados; body literal preserved).
- `00_nucleo/diagnosticos/diagnostico-l3-multispace-passo-270-1.md`
  — diagnóstico imutável P270.1.A (nono consumo directo de fonte
  vanilla).
- P270 — Gradient ColorSpace runtime L1+stdlib (precedente directo
  materializado em L3).
- P263/P265/P268/P268.2 — L3 templates (helpers renomeados; body
  preservado).

---

## Anotação cumulativa P270.2 — Gradient L3 emit CMYK directo (cluster Linear+Radial 8/8; Conic CMYK preserved)

**Data**: 2026-05-17.

P270.2 — cluster Gradient L3 emit CMYK directo materializado
para Linear+Radial via `/ColorSpace /DeviceCMYK` + Function
4-component. Conic CMYK scope-out preserved (§A.8 Cenário B
diagnóstico); candidato futuro P-Gradient-Conic-CMYK. ADR-0083
§"DeviceCMYK PDF" revogação **parcial** P270.2; perfil graded
DEBT-1 preservado (refino L3 sem mudar estratégia ADR-0087/0088/
0089/0090).

Status `EM VIGOR` preservado literal. Esta anotação documenta que
o cluster Gradient cristalino — pós-P270.2 — cobre:
- Linear: 8/8 spaces (P270.1 + P270.2 CMYK directo).
- Radial: 8/8 spaces (P270.1 + P270.2 CMYK directo;
  focal_* P269 preservados).
- Conic: 7/8 spaces full + CMYK fallback sub-óptimo
  (candidato P-Gradient-Conic-CMYK futuro).

**Sub-padrão "ADR scope-out revogado parcialmente"** N=3 → **N=4
cumulativo limiar formalização clara** (P267 Conic + P269 focal_*
+ P270 ColorSpace + **P270.2 DeviceCMYK** parcial). Candidato
meta-ADR futura.

Bug vanilla #4422 resolvido por construção (cristalino emit
`/DeviceCMYK` correcto).

Cross-references:
- ADR-0091 §"Anotação cumulativa P270.2" — fórmula completa
  Cenário B.
- ADR-0083 §"Anotação cumulativa P270.2" — revogação parcial
  §DeviceCMYK PDF.
- ADR-0087/0088/0089/0090 — Variant strategies anotadas cumulativa
  P270.2.
- `00_nucleo/diagnosticos/diagnostico-l3-cmyk-passo-270-2.md`
  — diagnóstico imutável P270.2.A (décimo consumo directo de
  fonte vanilla).
- P270.1 — L3 emit 7 spaces (precedente directo refinado).
- P263/P265 — L3 templates Linear+Radial (estendidos com CMYK
  branch dual).
- typst/typst issue #4422 — CMYK gradient bug vanilla causa raiz.

---

## Anotação cumulativa P270.3 — Infra-estrutura Type 6 Coons Patch Mesh (preparação cluster 24/24)

**Data**: 2026-05-17.

P270.3 materializa **infra-estrutura Type 6 Coons Patch Mesh** como
estratégia adicional Conic L3 emit (preparação CMYK P270.4 via
ADR-0092 EM VIGOR). Cluster Gradient ganha **industry-aligned
mesh-based** para conic CMYK (Cairo/Inkscape/Typst original Type 6
precedent).

Perfil graded DEBT-1 preservado (P270.3 adiciona infra-estrutura
sem mudar estratégia ADR-0090 Type 4 RGB; ADR-0089 2 emit paths
coexistem).

**Primeiro caso "2 estratégias L3 emit coexistem para mesmo variant"**
em cristalino — Conic ganha Type 4 Gouraud (RGB; ADR-0090 preserved)
+ Type 6 Coons (CMYK preparation; ADR-0092 novo).

Sub-padrão "ADR scope-out revogado parcialmente" N=4 → **N=5
cumulativo limiar formalização clara muito ultrapassado** —
candidato meta-ADR URGENTE.

Status `EM VIGOR` preservado literal. Ver ADR-0092 EM VIGOR.

Cross-references:
- ADR-0092 — Conic Coons Patches (criada PROPOSTO+IMPLEMENTADO P270.3).
- ADR-0090 §"Anotação cumulativa P270.3" — Type 6 scope-out revogado
  parcialmente.
- ADR-0089 §"Anotação cumulativa P270.3" — 2 emit paths Conic.
- ADR-0091 §"Anotação cumulativa P270.3" — preparação P270.4.
- `00_nucleo/diagnosticos/diagnostico-conic-coons-passo-270-3.md`
  — diagnóstico imutável (décimo primeiro consumo directo de fonte).
- Industry research: Cairo Igalia 2020 + Typst blog 2023 + W3C
  Workshop 2021 + Stanislaw Adaszewski + ISO 32000-1 §7.5.7.4.

---

## Anotação cumulativa P270.4 — Cluster Gradient L1+stdlib+L3 emit feature-complete 24/24 absoluto (fecho cluster série P270)

**Data**: 2026-05-17.

P270.4 — **cluster Gradient L1+stdlib+L3 emit feature-complete absoluto
24/24** (3 variants × 8 spaces). Marco arquitectural máximo do cluster
Color (ADR-0083) + Gradient (ADR-0087/0088/0089). Perfil graded
DEBT-1 §"Color paridade vanilla" agora cobre 24/24 combinações
user-facing L1+L3.

### Série P270 completa pós-P270.4

| Passo | Materialização | Marco |
|---|---|---|
| P270 | L1+stdlib `space: ColorSpace` cross-variant (3 variants × 8 spaces) | ADR-0091 PROPOSTO+IMPLEMENTADO |
| P270.1 | L3 emit 7 spaces RGB-family + perceptual (sRGB/LinearRGB/Luma/Oklab/Oklch/HSL/HSV) | helpers renomeados `multispace_sample_stops_*` |
| P270.2 | L3 emit CMYK Linear+Radial `/DeviceCMYK` | Bug #4422 resolvido Linear+Radial |
| P270.3 | Coons RGB infra-estrutura (flag opt-in default OFF; ADR-0092 PROPOSTO+IMPLEMENTADO) | precedente 2 emit paths Conic |
| **P270.4** | **Coons CMYK activação opt-in flag ON; `/ShadingType 6 /DeviceCMYK`** | **Cluster 24/24 absoluto** |

### Cluster Gradient L3 emit pós-P270.4 final

| Variant | 7 RGB-family + perceptual | CMYK |
|---|---|---|
| Linear | P270.1 ✓ `/DeviceRGB` (Function 3-comp) | P270.2 ✓ `/DeviceCMYK` (Function 4-comp) |
| Radial | P270.1 ✓ `/DeviceRGB` (focal_* P269) | P270.2 ✓ `/DeviceCMYK` |
| Conic | P268+P268.2 ✓ `/ShadingType 4` Gouraud | **P270.4 ✓** `/ShadingType 6` Coons |

**24 combinações user-facing materializadas** (3 variants × 8 spaces)
em L1 + stdlib + L3 PDF emit completamente.

### Sub-padrão "ADR scope-out revogado parcialmente" N=5 → N=6 cumulativo

**Limiar formalização clara ainda mais ultrapassado** — meta-ADR
URGENTE FINAL. Pattern consolidado claro:
- N=1 P267 (ADR-0088 §Conic).
- N=2 P269 (ADR-0088 §focal_*).
- N=3 P270 (ADR-0083 §ColorSpace runtime).
- N=4 P270.2 (ADR-0083 §DeviceCMYK Linear+Radial).
- N=5 P270.3 (ADR-0090 §Type 6 Coons).
- **N=6 P270.4** (ADR-0091 §Conic CMYK scope-out + ADR-0083
  §DeviceCMYK definitivo).

Candidato meta-ADR formalização futura paridade P260 ADR-0084/0085
(que formalizaram outros sub-padrões em N=5-6 cumulativo).

Status `EM VIGOR` preservado literal.

Cross-references:
- ADR-0092 §"Anotação cumulativa P270.4" — fórmula completa
  activação opt-in flag ON.
- ADR-0091 §"Anotação cumulativa P270.4" — revogação final §Conic
  CMYK scope-out.
- ADR-0083 §"Anotação cumulativa P270.4" — revogação final
  §DeviceCMYK PDF (3 variants × CMYK).
- ADR-0089 §"Anotação cumulativa P270.4" — Conic 2 emit paths
  ambos activos.
- `00_nucleo/diagnosticos/diagnostico-conic-coons-cmyk-passo-270-4.md`
  — diagnóstico imutável (décimo segundo consumo directo de fonte).
- Cluster Color (P257) cobertura final 100% estrutural + 100% L3
  PDF emit.

## Anotação cumulativa P271 — Meta-formalização sub-padrões metodológicos

P271 — meta-formalização sub-padrões metodológicos via **ADR-0093 +
ADR-0094 EM VIGOR** (passo administrativo XS; paridade P260
ADR-0084/0085 + P268.1 ADR-0090).

Perfil graded DEBT-1 reforçado por mecanismos operacionais
documentados:

- **Caps explícitos hard/soft** → ADR-0094 §"Pattern 1".
- **Reutilização literal helpers cross-passos** → ADR-0094 §"Pattern 2".
- **Industry research proactiva** → ADR-0094 §"Pattern 3".
- **Evolução ADRs via scope-out parcial + anotação cumulativa** →
  ADR-0093 §"Pattern 1 + Pattern 2".

Sub-padrão "Anotação cumulativa em vez de ADR nova" N=10 cumulativo;
anotações desta ADR (P269/P270/P270.1/P270.2/P270.3/P270.4/P271)
demonstram o pattern em larga escala.

Status `EM VIGOR` preservado literal. Ver ADR-0093 + ADR-0094 EM
VIGOR.
