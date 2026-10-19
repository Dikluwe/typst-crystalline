# Retrospectivo metodológico — Passos P283-P299

**Data**: 2026-05-19
**Origem**: Passo 300 §3.A (caminho A consolidação documental pura).
**Tipo**: documento metodológico cumulativo, não-normativo.
**Conteúdo**: lições emergentes da sequência de 17 passos
materializadores P283-P299; magnitudes A.0.0; padrões cumulativos
adiados com justificações.

---

## 1. Sumário cumulativo P283-P299

**17 passos materializadores** (P283-P299) + **1 retrospectivo**
(P300) = **18 passos** sequência metodológica.

### 1.1 — Decomposição por categoria

| Categoria | Passos | Total |
|---|---|---|
| Cumulativos Style/Text | P288-P292 | 5 |
| Ortogonais (P293+) | P293, P294, P295, P299 | 4 |
| Cluster math | P296, P297, P298 | 3 |
| Extensões directas | P284 (decorations), P285 (Line.color), P286 (wrap-aware), P287 (smartquote), P283 (calc) | 5 |
| **Total materializadores** | | **17** |
| Retrospectivo | P300 | 1 |

### 1.2 — Magnitudes A.0.0 (template §8.7') P293-P299

```
P293: ████████░░ alta     ← H6 não-listada na spec
P294: ██████████ máxima   ← Spec inteira invalidada (vanilla q→c)
P295: ██░░░░░░░░ baixa    ← Linha tabela admin desactualizada
P296: ██████░░░░ média    ← Classificação inteira inválida
P297: ████████░░ alta     ← Wrapper vanilla inexistente (12 elementos)
P298: ████████░░ alta     ← Heurística limits-style já existia
P299: ███░░░░░░░ baixa    ← Precedente `calc` module directo
```

**Tendência**: não-monotónica; **flutuação saudável** documentada.

---

## 2. Lições metodológicas emergentes

### 2.1 — "Confirmação esperada" como categoria honesta (P295 §10; P297 §A.5')

P295 §10 inaugurou a distinção entre:
- **"Descoberta"**: A.0.0 revela hipótese não antecipada na spec.
- **"Confirmação esperada"**: A.0.0 confirma o que a spec
  antecipou.

**Valor da distinção**: evita interpretar magnitude baixa A.0.0
como "ritualismo" — confirmação esperada é **resultado legítimo**
mesmo quando spec é factualmente correcta.

**Aplicação**: P295 (baixa) e P299 (baixa) classificadas como
"confirmação esperada" em vez de degenerescência.

### 2.2 — "Flutuação saudável" magnitude A.0.0 (P299 §10)

Hipótese degenerescência §6.6 P295 hipotetizou: *"4 reaplicações
A.0.0 consecutivas com magnitude decrescente ou factual-modesta →
template degenera em ritual procedimental"*.

**Refutada empiricamente**: P296 (média) → P297 (alta) → P298
(alta) → P299 (baixa). Sequência **flutua**, não decresce
monotonicamente.

**Conclusão**: template §8.7' robusto perante flutuação; magnitudes
variadas reflectem **complexidade variável dos passos**, não
degenerescência do método.

### 2.3 — Subdivision decision A.0.0' (P299; reaplicada P300)

P299 inaugurou secção **A.0.0'** após A.0.0 — decisão explícita
sobre subdivisão de scope quando spec antecipa ambiguidade
(P299.A/B/C/D).

P300 reaplicou: A.0.0' decisão de caminho A/B/C/D/E.

**Pattern emergente**: separa **descoberta empírica** (A.0.0) de
**escolha arquitectural** (A.0.0') que depende dessa descoberta.

### 2.4 — Cross-variant interaction (P298)

P296/P297 criaram variants com handlers independentes. P298
modificou `attach.rs is_limits` para reconhecer `MathOp` —
**1.ª vez** que um variant afecta o layout de outro no cluster
math.

**Sub-padrão N=1 inaugural** — registado mas não promovido.

### 2.5 — Module namespaced via SSoT (P299)

P283 inaugurou `make_calc_module()` (calc). P299 reaplicou com
`make_math_module()` (math). **Sub-padrão N=2 cumulativo** —
emergente.

42 operadores math pré-definidos registados via `Content::MathOp`
como Single Source of Truth.

### 2.6 — Refutação significativa via inspecção vanilla (P294/P297)

Casos onde inspecção literal de `lab/typst-original/` invalidou
toda a estrutura proposta pela spec:

- **P294**: vanilla converte quadratic→cubic em construct-time;
  spec propunha `QuadraticTo` variant + emit novo PDF operator.
- **P297**: vanilla fragmenta `underover` em 12 elementos separados;
  spec propunha wrapper unificado.

**Lição**: inspecção vanilla **antes** de materialização economiza
trabalho que seria descartado.

### 2.7 — Anti-padrão over-formalização rigorosamente honrado

P273.17 §0 estabelece: **uma ADR meta por passo no máximo;
preferível zero**.

**Estado P293-P300 (8 passos cumulativos)**: **0 ADRs meta
promovidas**, apesar de múltiplos candidatos genuínos:

| Passo | Candidato | Veredicto |
|---|---|---|
| P293 | §8.7' inaugural N=1 | adiado |
| P294 | §8.3 N=6 candidato | adiado |
| P295 | §8.7' N=3 + §8.3 N=7 | ambos adiados |
| P296 | §8.7' N=4 + §8.3 N=8 | ambos adiados |
| P297 | §8.7' N=5 + §8.3 N=9 + "variant rico" N=5 | 3 adiados |
| P298 | §8.7' N=6 + §8.3 N=10 + sub-cluster N=3 | 3 adiados (P298 §6.8) |
| P299 | §8.7' N=7 + 3 sub-padrões | 4 adiados |
| **P300** | **§8.7' N=7 maduro + §8.3 N=11** | **Ambos adiados conservadoramente** |

**Padrão**: candidatos amadurecem mas **não há pressa para
formalizar**. P273.17 §0 confirma-se em 8 passos consecutivos.

---

## 3. Padrões cumulativos pós-P300

### 3.1 — §8.7' "A.0.0 inventário Fase A" (N=7 maduro adiado)

**Estado**: aplicação cumulativa robusta; 7 magnitudes documentadas;
valor empírico claro em P294/P297/P298.

**Adiado P300 por**:
- Funciona organicamente sem formalização.
- Risco ADR prescritiva vs prática discricionária.
- Preferência conservadora P273.17 §0.

**Reavaliação**: P301+ se atingir N=8 com valor empírico novo
inequívoco.

### 3.2 — §8.3 "refutação pragmática" (N=11 descritivo adiado)

**Estado**: 11 refutações cumulativas documentadas.

**Adiado P300 por**: padrão **descritivo** (observação sobre o
sistema), não **prescritivo** (princípio operacional). ADR teria
valor marginal.

### 3.3 — §8.6 "A.5' anti-reflexão" (N=10 instrumental interno)

**Estado**: 10 reaplicações P291-P300; ferramenta metodológica
interna.

**Não promovido**: função utilitária; formalização não necessária.

### 3.4 — "Variant rico com cosméticos opcionais" (N=1 genuíno)

**Estado**: P297 `MathUnderover` Option `Box<Content>` estrutural —
primeira qualificação genuína após refutações de P156G/H/I (bool
defaults).

**Adiado**: N=1 abaixo de limiar tentativo N≥3.

### 3.5 — Sub-padrão "cluster math handler dedicado" (N=3 ambíguo)

**Estado**: P296 (2 handlers) + P297 (1 handler) + P298 (1 handler
trivial) = N=3 quantitativo; qualitativo questionável (P298
trivial).

**Adiado**: ambiguidade qualitativa.

### 3.6 — Sub-padrão "cross-variant interaction" (N=1 inaugural)

**Estado**: P298 `MathOp.limits` afecta `MathAttach is_limits`.

**Insuficiente**: aguarda N≥3.

### 3.7 — Sub-padrão "module namespaced" (N=2 cumulativo)

**Estado**: P283 `make_calc_module` + P299 `make_math_module`.

**Insuficiente**: aguarda N=3.

### 3.8 — Sub-padrão "operadores pré-definidos via SSoT" (N=1)

**Estado**: P299 42 operadores via `Content::MathOp`.

**Insuficiente**: aguarda N≥3.

---

## 4. Bugs latentes descobertos durante P283-P299

### 4.1 — NBSP em SmartQuote consumer (P287 → P290)

P287 SmartQuote inicial usava `layout_content(Content::Text(...))`
que invocava `split_whitespace()` — eliminando NBSP (não-breaking
space).

**Descoberto P290** (lang fr ↔ FR NBSP requirements). **Fixed**:
substituído por `layout_word(glyph)` que preserva NBSP.

**Lição**: testes regression bit-exact obrigatórios capturam
bugs colaterais de refinos cumulativos.

---

## 5. ADR-0098 "single source of truth" — N=17 cumulativo

Hash `export.rs 66cb8ac3` preservado bit-exact pelos **17 passos
consecutivos** P282-P300. Invariante robusta sobre 17 features
distintas:

| Passo | Feature | Modo de preservação |
|---|---|---|
| P282 | Refactor pipeline stream-builders | Refactor sem mudança bit-exact |
| P285 | Line.color | Helper `line_rg_prefix` colapso bit-exact |
| P286 | wrap-aware decorations | Per-line geometry sem alterar emit |
| P287 | SmartQuote | Variant + consumer via FrameItem::Text standard |
| P288-P292 | Style cumulativos | StyleDelta projection via TextStyle |
| P293 | curve cubic | Emit já existia (P277) |
| P294 | curve quadratic | Conversão q→c em construct-time |
| P295 | footnote Fase 1 | Marker [N] como Content::text |
| P296 | accent/cancel | FrameItem::Text/Line standard |
| P297 | underover | Empilhamento via offset_item |
| P298 | op | Trivial delegate; cross-variant via attach.rs (sem emit) |
| P299 | math module | SSoT MathOp; emit reusa P298 |
| **P300** | **retrospectivo** | **Zero alterações código** |

**Lição**: pattern "feature via FrameItem standard + variant no
domínio puro" é arquitecturalmente robusto.

---

## 6. ADR-0099 "activação posterior" — N=16 cumulativo

P293-P299 reaplicações documentadas; P300 herda contagem (sem
materialização).

---

## 7. Decisão final P300: caminho A

**0 promoções ADR meta** apesar de §8.7' N=7 maduro candidato.

**Razão**: critério estrito spec §1.1 — "promoção só se gatilho
**inequivocamente** dispara". §8.7' é maduro mas a formalização
traz overhead (rigidez vs discrição) sem ganho claro.

**P300 não é número redondo glorificado** — é **consolidação
honesta** com preservação do anti-padrão over-formalização P273.17
§0.

---

## 8. Próximos passos

P301+ continua a série com frentes pendentes catalogadas (vide
`frentes-pendentes-pos-p299.md`).

**Padrões metodológicos preservados como ferramentas**:
- §8.7' A.0.0 template (aplicação discricionária per passo).
- §8.3 refutação pragmática (observação sistémica).
- A.0.0' subdivision decision (quando ambiguidade arquitectural).
- "Confirmação esperada" categoria (magnitude baixa legítima).
- "Flutuação saudável" magnitude (sequências variadas).

**Anti-padrão P273.17 §0**: rigorosamente honrado por **8 passos
consecutivos** P293-P300. **0 ADRs meta promovidas** num período de
materialização significativa.

---

## Fecho

P283-P299 (17 passos materializadores + P300 retrospectivo) =
**série metodológica completa de 18 passos** documentada
cumulativamente.

**Marco P300**: 1.º passo não-materializador da série; caminho A
escolhido honestamente; anti-padrão rigorosamente honrado.

Reset arquitectural para P301+ com frentes pendentes claramente
catalogadas e padrões cumulativos preservados como ferramentas
discricionárias.
