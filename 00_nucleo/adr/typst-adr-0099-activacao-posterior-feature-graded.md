# ADR-0099 — Activação posterior de feature graded como padrão de pendência

**Status**: IMPLEMENTADO (desde P285; formalizado em P289)
**Data**: 2026-05-19
**Passo promotor**: P289 (`P-style-weight-variant`)
**Categoria**: Meta-arquitectural / processo

---

## Contexto

Vários passos da migração cristalina deixam features **parseadas
mas inertes** — código que recebe e armazena dados (atributos via
`#set`, campos em variants existentes) mas cujo consumer ainda não
existe ou não está activo. Tipicamente isto acontece quando:

1. **Variant pré-existente ganhou campo novo** sem consumer (ex:
   `stroke: Option<Color>` em `Content::Underline` adicionado em
   P284 mas inerte até P285).
2. **Field em struct existente** está populado por parse mas
   consumer não lê em ponto relevante (ex: `StyleDelta.lang` em
   P130/P144 mas sem variant `Style::Lang` correspondente até P288).
3. **Feature ausente paralela a markup pré-existente** que requer
   primitivas L1 novas (ex: função stdlib smartquote ausente apesar
   de markup `"foo"` activo desde P155 até P287).
4. **Consumer single-path** que requer extensão para caso novo (ex:
   consumer single-line em P284 → wrap-aware em P286).

Estes casos partilham padrão estrutural comum: **a pendência é
resolúvel adicionando fontes de entrada ou consumers sem refactor
de fundo nem quebra de caminhos pré-existentes**. Atingido **limiar
histórico N=5 aplicações cumulativas** (critério empírico ADR-0065
para formalização de padrão meta).

---

## Decisão

Formaliza-se o padrão **"Activação posterior de feature graded como
padrão de pendência"** como meta-processual perene em cristalino:

> **Regra**: pendências graded registadas em passos anteriores
> (features parseadas-mas-inertes; variants ausentes para fields
> activos; consumers single-path; features ausentes paralelas a
> caminhos existentes) **devem ser resolvidas por extensão minimalista
> sem refactor de caminho pré-existente**.
>
> O passo activador adiciona apenas:
> - 2ª fonte de entrada (variant novo, função stdlib, hook
>   condicional), ou
> - Consumer novo / extensão de consumer existente, ou
> - Refino cirúrgico de consumer existente para cobrir caso novo.
>
> **Não** revisita o passo originador. **Não** quebra bit-exact do
> caminho pré-existente. Backward-compat é critério **duro**.

**Consequência operacional**: cada activação produz **patch
arquitectural sucinto** (~30-100 LOC L1, zero ou minimal L3) com
testes que **cobrem ambos os caminhos pré-existente e novo**.

**Consequência testável**: testes pré-passo continuam verdes
bit-exact; testes novos exercitam exclusivamente o caminho activado.

---

## 5 aplicações cumulativas (justificação do limiar N=5)

### N=1: P285 §8.3 — `stroke` em decorações P284 activado

P284 introduziu `Content::Underline/Strike/Overline` com campo
`stroke: Option<Color>` parseado mas inerte (consumer Layouter
ignorava porque `FrameItem::Line` não tinha `color`). **P285**
adicionou `FrameItem::Line.color: Option<Color>` + emit `RG`
condicional + consumer Layouter actualizado (`stroke.or(style.fill)`).
P284 não foi revisitado; bit-exact preservado em call-sites
pré-P285.

### N=2: P286 §8.1 — Consumer P284 wrap-aware activado

P284 consumer assumia single-line. **P286** estendeu para wrap-aware
via campo opcional `decoration_lines_collector` no Layouter + hook
condicional em `flush_line`. P284 single-line path preservado por
construção (collector None → algoritmo P284 original).

### N=3: P287 §8.2 — Função stdlib smartquote materializada

Markup `"foo"` ↔ smart-quotes activo desde P155 mas função stdlib
`#smartquote(...)` ausente. **P287** adicionou
`Content::SmartQuote { double }` leaf + `native_smartquote` stdlib +
consumer Layouter. P155 `eval_markup` path preservado bit-exact
(estados independentes per diagnóstico §A.3 opção γ′).

### N=4: P288 §8.2 — `Style::Lang(Lang)` variant activado

`StyleDelta.lang` parseado desde P130/P131B/P144 mas sem variant
`Style::Lang` correspondente (assimetria Tabela B.3 vs B.4).
**P288** adicionou `Style::Lang(Lang)` ao enum + arm em
`StyleChain::push_styles` (2ª fonte de entrada). Parse-driven
`eval_set_rule` path preservado bit-exact.

### N=5: P289 §A.4.3 — `Style::Weight(u16)` variant activado

`StyleDelta.weight` parseado desde P126/P129 com consumer
faux-bold P139 activo, mas sem variant `Style::Weight` correspondente
(continuação da assimetria P288). **P289** adicionou `Style::Weight(u16)`
+ arm em `push_styles`. Consumer faux-bold P139 reusado sem
alteração — **primeira aplicação directa de ADR-0098** (single
source of truth) cumulativa com este padrão.

---

## Alternativas consideradas

### (a) Refactor cross-cluster (status quo pré-ADR)

Cada activação refactora simultaneamente caminho pré-existente +
caminho novo. **Rejeitada** por gerar diffs grandes e risco de
regressão bit-exact.

### (b) Single-path refactor (substituição em vez de adição)

Substituir caminho pré-existente pelo novo (e.g. P288 reescreveria
`eval_set_rule` para emitir `Content::Styled` em vez de escrever
directamente em `delta`). **Rejeitada** por quebrar bit-exact e
arquitectura paralela do parse vanilla.

### (c) Não formalizar (continuar implícito)

Padrão emerge naturalmente mas sem ADR. Risco: passos futuros podem
quebrar a invariante por desconhecimento (e.g. propor refactor de
caminho pré-existente quando extensão minimalista cobriria). Limiar
N=5 atingido — formalização legítima per ADR-0065.

---

## Consequências

### Imediatas (já materializadas)

1. **Pendências graded registadas em passos** são candidatos
   naturais a activação posterior. Diagnósticos de passos seguintes
   devem inspeccionar pendências em §"Pendências relacionadas" + §7
   "Risco residual" de passos anteriores.
2. **2ª fonte de entrada** é mecanismo preferido sobre refactor de
   1ª fonte. Match exaustivo em cascade (`push_styles`, walkers)
   defende contra omissão por construção.
3. **Patch arquitectural sucinto** é métrica de aderência ao padrão.
   Activações >100 LOC L1 ou que tocam L3 devem ser examinadas para
   garantir que não estão a violar a regra.

### Futuras (operacionais)

1. **Fase A de passos futuros** deve incluir secção "Reaplicação do
   padrão N=X" se aplicável. P289 §A.4.3 estabelece template:
   tabela com N citações cumulativas + decisão promoção condicional.
2. **Limite N≥5 para promoção ADR meta paralela** — quando o padrão
   ganha 5 aplicações distinas, considerar formalização (paralelo
   ADR-0098 N=5 → ADR-0099 N=5).
3. **Anti-padrão P273.17 §0 vigente** — promover apenas se gatilho
   disparar genuinamente. Promoção mecânica viola anti-padrão.

### Sinergia com ADR-0098

ADR-0098 ("Single source of truth como invariante anti-bug") e
ADR-0099 ("Activação posterior") são **complementares**:

- ADR-0098 garante que **estrutura de emit é estável** (reuso L1/L3
  pré-existente).
- ADR-0099 garante que **estrutura de consumer é estável** (extensão
  minimalista de caminhos pré-existentes).

Juntas, definem o **paradigma cirúrgico** dos passos pós-P281:
modificações localizadas + bit-exact backward-compat + testes
cumulativos.

### Não-objectivos

1. **Não** proibir refactor cross-cluster. Features genuinamente
   novas (e.g. nova categoria de Content; novo emit operator)
   motivam refactor maior. A regra aplica-se a **pendências graded
   pre-existentes**, não a features novas em si.
2. **Não** impor sequência mecânica. Cada pendência tem timing
   próprio — activação não é obrigação automática.

---

## Status

`IMPLEMENTADO` desde **P285** (1ª aplicação `stroke` em decorações);
**formalizado em P289** (limiar histórico N=5 atingido empiricamente
via passos P285/P286/P287/P288/P289).

## Cross-references

- **ADR-0065** — inventariar-primeiro + critério N≥5 para promoção
  meta-ADR.
- **ADR-0098** — Single source of truth como invariante anti-bug
  (formalizada em P288; complementar a esta ADR).
- **ADR-0093** — Meta-metodologia evolução ADRs (justifica
  formalização incremental).
- **ADR-0094** — Meta-operacional specs.
- **P284** — Origem da pendência stroke (resolvida P285).
- **P285 §8.3** — Aplicação N=1.
- **P286 §8.1** — Aplicação N=2.
- **P287 §8.2** — Aplicação N=3.
- **P288 §8.2** — Aplicação N=4.
- **P289 §A.4.3** — Aplicação N=5 (formalização).
- **P273.17 §0** — Anti-padrão over-formalização (justifica
  condicional + apenas 1 ADR meta por passo).

---

*ADR-0099 formaliza "Activação posterior de feature graded como
padrão de pendência" como meta-processual perene em cristalino, com
base em 5 aplicações cumulativas (P285-P289) empiricamente confirmadas.
Complementar a ADR-0098: ADR-0098 garante estrutura de emit estável;
ADR-0099 garante estrutura de consumer estável. Juntas definem o
paradigma cirúrgico pós-P281. Promoção legítima per ADR-0065 critério
N≥5 + ADR-0093 política de formalização incremental.*
