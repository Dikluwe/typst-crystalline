# Relatório P159B — Diagnóstico amplo expansão série 159 + tecto realista Model

Passo arquitectural de diagnóstico amplo (M-) precedendo
materialização. **Não materializa código**. Análogo estrutural
a **P156B** (diagnóstico Layout amplo). **Quarta aplicação
concreta de ADR-0065 critério #5** (scope determinado por
inventário) com **diversidade ampliada multi-feature** —
P157/P158/P159 inventariaram uma feature cada; P159B inventaria
**todas as expansões pendentes da família 159 + outros refinos
Model** simultaneamente. **Décima sexta aplicação consecutiva**
do padrão diagnóstico-primeiro.

---

## 1. Resumo do diagnóstico (síntese das 6 secções)

### 1.1 §1 — ADRs/DEBTs por família

- **Família 159 (Bibliography + Cite)**: DEBT-55 plano
  documentado tem 6/10 itens ainda pendentes pós-P159A
  (ADR-0062 criar, Cargo.toml hayagriva, pipeline introspect
  cross-document, render layout completo CSL). ADR-0062 é
  reserva sem ficheiro. ADR-0017 bloqueia cite cross-document
  forward refs mas walk single-pass viável.
- **Família 158 (figure-kinds)**: subset máximo P158 §3.3
  (supplement automático por lang) materializável sem
  dependência cruzada. ADR-0041 show selectors `figure.where(...)`
  scope-out per ADR-0054 graded.
- **Família 157 (table foundations)**: refinos requerem
  cross-módulo (DEBT-34e Layout placement; cells Stroke/Paint
  Layout+entities).
- **Outros Model**: promoção ADR-0060 R1 + L0 prompt content.md
  + ADR meta saturação (administrativos XS).

### 1.2 §2 — Inventário código pendente

- **Família 159**: BibEntry fields adicionais
  (volume/pages/journal/etc.); Cite.form (Normal/Prose/Author/
  Year); Cite.style (depende hayagriva); Bibliography.style
  (depende hayagriva); Bibliography.full (depende ADR-0017);
  Bibliography.lang/region (i18n).
- **Família 158**: supplement automático por lang
  (reuso padrão P155 quotes); refactor `kind: String →
  Option<String>` (XS); show selectors (cross-módulo).
- **Família 157**: cells refinos (Stroke/Paint cross-módulo);
  algoritmo placement Grid (DEBT-34e cross-módulo); repeat
  (DEBT-56 cross-módulo).
- **Outros Model**: footnote (scope-out humano); document/
  title/asset (Fase 3 ADR-0060 divergência); list/enum/
  caption/link/par (`parcial`).

### 1.3 §3 — Matriz de dependências cruzadas

5 categorias:
- **A — Introspection runtime (ADR-0017)**: cite cross-doc
  refs, numbering autor-ano (hard); validação `key ∈ entries`
  (soft).
- **B — Refactor multi-region (DEBT-56)**: TableHeader/Footer
  repeat; Bibliography paginada.
- **C — Crate externa (hayagriva)**: CSL parsing/styles;
  Bibliography parsing externo; Cite.style override.
- **D — ADR pendente promoção**: ADR-0062 hayagriva; ADR-0017
  Introspection.
- **E — Outro módulo**: placement Grid (Layout); cells refinos
  (Layout+entities); show selectors (Rules/show); Bibliography
  paginada (Layout).

### 1.4 §4 — Tecto realista de Model

**Estado actual**: cobertura Model agregada **50%** (11/22);
cobertura ampla 24/22 entradas com valor cumulativo.

**Tecto Model puro estimado** (Bloco A — 5 refinos sem
dependência cruzada hard):
- Cobertura agregada **50% → ~55-60%** (alcançável com 3-4
  sub-passos M aplicados a Bloco A).
- Cobertura ampla cresce ligeiramente.
- Cobertura arquitectural mantém-se ~82% (sem novos variants).

**Tecto Model + hayagriva** (Bloco A + B):
- Cobertura agregada **~55-60% → ~68%** (paridade ADR-0060
  declarado).

**Tecto pós-Bloco C** (cross-módulo): difícil estimar; depende
prioridade humana entre Model/Layout/Introspection.

### 1.5 §5 — Sequência candidata sub-passos

**Bloco A** (5 refinos puramente Model, ordenados):

| Ord | Identificador | Refino | Tamanho | Hash content.rs | Tests Δ |
|---:|:-------------|--------|---------|:---------------:|--------:|
| 1 | **P158B** | Supplement automático por lang figure | M | preservado | +12-15 |
| 2 | **P159C** | Cite.form variants | M | quebrado (variant Cite) | +10-15 |
| 3 | **P159D** | BibEntry fields adicionais | S+ | preservado | +5-8 |
| 4 | **P158C** ou **P159E** | Refactor `kind: String → Option<String>` | XS | quebrado (Figure) | +2-4 |
| 5 | **P159F** | Numbering numérico simples Bibliography | M | preservado | +10-15 |

**Bloco B** (5 refinos com hayagriva ADR-0062): pré-requisito
ADR-0062 promovida; CSL parsing → styles APA/IEEE/MLA/Chicago.

**Bloco C** (cross-módulo): NÃO materializáveis em Model puro.

### 1.6 §6 — Recomendação concreta

**Recomendação primária**: **P158B — Supplement automático
por lang em figure**.

Justificação:
1. Sem dependência cruzada hard.
2. Reuso de padrão consolidado: `localize_quotes(lang)` em
   `rules/lang/quotes.rs` (P155).
3. Hash content.rs preservado.
4. Tamanho M preserva cadência granular.
5. Funcionalidade visível ("Figura"/"Figure"/"Abbildung").

**Recomendação secundária**: **P159D BibEntry fields adicionais**
(S+; refino struct sem variant).

**Recomendação terciária**: passo administrativo XS (criar
ADR-0062 PROPOSTO + actualizar L0 prompt content.md).

---

## 2. Decisão final de scope para passo seguinte

**Recomendação**: **P158B — Supplement automático por lang em
figure**.

Características:
- Refino comportamental análogo a P158A.
- Helper novo `figure_supplement_for_lang(kind, lang) -> String`
  em `rules/lang/figure_supplement.rs` (módulo novo seguindo
  padrão `quotes.rs` P155).
- Modificação em `introspect.rs` para usar supplement no formato
  do label resolvido.
- Mapeamento mínimo: pt/en/de/fr/es/it para "image" e "table"
  e "raw" (~3 keys × 6 langs = 18 strings).
- Tests ~12-15.
- Granularidade preservada N=15.

---

## 3. Listagem completa de candidatos Bloco A (informação para validação humana)

### 3.1 P158B — Supplement automático por lang figure (M)

- **Refino**: `infer_kind_from_body` produz kind (image/table/
  raw); novo helper `lookup_supplement(kind, lang)` produz
  prefix localizado.
- **Sem dependência cruzada hard**.
- **Hash `content.rs` preservado** (módulo novo + modificação
  introspect).
- **Tests Δ**: +12-15.
- **Granularidade**: M preservado.

### 3.2 P159C — Cite.form variants (M)

- **Refino**: enum `CitationForm { Normal, Prose, Author, Year }`
  + field `form: Option<CitationForm>` em `Content::Cite`.
- **Sem dependência cruzada hard**.
- **Hash `content.rs` quebrado** (variant Cite expande field).
- **ADR-0064 Caso A** aplicado para form (`Smart<CitationForm>`
  vanilla → `Option<CitationForm>` cristalino) — patamar Caso A
  cresce N=5 → 6.
- **Layout placeholder**: render diferente por form (Normal
  `[key]`; Prose `Author (Year)`; Author `Author`; Year `Year`).
- **Tests Δ**: +10-15.
- **Granularidade**: M preservado.

### 3.3 P159D — BibEntry fields adicionais (S+)

- **Refino**: expandir `BibEntry` struct com fields adicionais
  `volume/pages/journal/publisher` (4 fields adicionais
  comuns).
- **Sem dependência cruzada hard**.
- **Hash `content.rs` preservado** (struct extensão; não toca
  Content enum).
- **Tipo entity refino** — quebra padrão "estabilidade hash
  entities" mas só do `bib_entry.rs`, não content.rs.
- **Tests Δ**: +5-8.
- **Granularidade**: S+ preservada.

### 3.4 P158C ou P159E — Refactor kind: String → Option<String> (XS)

- **Refino cosmetic**: refactor kind para Option (Caso A
  estrito); benefício marginal sem hayagriva.
- **Sem dependência cruzada hard**.
- **Hash `content.rs` quebrado** (Figure variant refino).
- **Tests Δ**: +2-4.
- **Recomendação baixa prioridade** — benefício marginal.

### 3.5 P159F — Numbering numérico simples Bibliography (M)

- **Refino**: counter local de bib entries; render `[1]`/`[2]`/
  `[3]` em vez de `[key]`.
- **Sem dependência cruzada hard** (counter local single-pass).
- **Hash `content.rs` preservado** (refino layout + introspect).
- **Tests Δ**: +10-15.
- **Granularidade**: M preservado.

---

## 4. Listagem Bloco B — bloqueadores ADR-0062 (informação)

| Item | Pré-requisitos |
|------|----------------|
| **ADR-0062 XS** Criar PROPOSTO | — (administrativo) |
| **P159G** Cargo.toml + crystalline.toml hayagriva | ADR-0062 |
| **P159H** hayagriva integration minimal (parsing entries) | P159G |
| **P159I** CSL styles APA simples | P159H |
| **P159J** CSL styles adicionais (IEEE/MLA/Chicago) | P159I |

**Bloco B pode iniciar com criação ADR-0062** (administrativo
XS) **a qualquer momento** — não bloqueia Bloco A.

---

## 5. Listagem Bloco C — cross-módulo (NÃO materializáveis em Model puro)

| Item | Bloqueador |
|------|-----------|
| Algoritmo placement Grid | DEBT-34e (Layout) |
| Repeat header/footer real | DEBT-56 (Layout multi-region) |
| Cite cross-document forward refs | ADR-0017 (Introspection) |
| Show selectors `figure.where(kind:)` | Refactor show rules |
| Cells refinos com Stroke/Paint | Layout + entities Stroke/Paint |
| Bibliography paginada | DEBT-56 (Layout) |

---

## 6. Análise de risco (padrão N=14 → 15; passo diagnóstico amplo)

P159B é **passo diagnóstico amplo** (M-) sem alteração de
código. **Décima quinta aplicação consecutiva** de §análise de
risco preservando precedente.

### 6.1 Riscos identificados

| Risco | Avaliação | Mitigação aplicada |
|-------|-----------|---------------------|
| DEBT-55 ter scope diferente do esperado | Não materializou — DEBT-55 documenta plano completo XL alinhado com expectativa | Conteúdo completo citado em §1.1 |
| ADR-0062 ter conteúdo concreto não visto antes | Não materializou — ADR-0062 confirmada como reserva sem ficheiro (paridade P159) | Documentado em §1.1 |
| ADR-0017 detalhes revelarem mais features Model dependentes | Parcial — confirmação que cite walk single-pass viável | Documentado em §3.1 categoria A |
| DEBT-34e detalhes serem partilhados com outros containers | Baixo — DEBT-34e isolado a Grid | Documentado em §1.3 |
| Bloco A ficar vazio | Não materializou — Bloco A tem 5 candidatos | §5.1 lista completa |

### 6.2 Riscos avaliados mas não materializados

| Risco | Avaliação inicial | Razão de não-materialização |
|-------|-------------------|----------------------------|
| Diagnóstico ser tão amplo que perde foco | Médio | Estrutura por família + matriz dependências mantém clarity |
| Recomendação §6 ser ambígua | Baixo | P158B identificado claramente como recomendação primária |

### 6.3 Riscos não-aplicáveis

- **Refactor de código**: zero (passo puramente documental).
- **Quebra de contrato API**: zero.
- **Drift de hashes**: zero.

### 6.4 Conclusão de risco

**Risco residual: muito baixo**. Padrão "passo diagnóstico
documental amplo + scope multi-feature determinado por
inventário (ADR-0065 #5) + política sem novas reservas"
estabelece precedente novo (P156B foi diagnóstico amplo Layout;
P159B é diagnóstico amplo Model — ambos análogos).

**Auto-validação ADR-0065 critério #5 atinge 4 aplicações
concretas com diversidade máxima**:
- P157: divisão multi-passo single feature.
- P158: subset selection single feature.
- P159: par acoplado single feature.
- **P159B: scope multi-feature simultâneo** (família 159 +
  158 + 157 + outros).

---

## 7. Verificações (numeradas per spec do passo)

| # | Verificação | Resultado |
|---|-------------|-----------|
| 1 | Diagnóstico produzido com 6 secções | **✓** `diagnostico-expansao-159-passo-159b.md` (6 secções §1-§6) |
| 2 | Mapa ADR/DEBT por família documentado em §1 | **✓** 3 famílias + outros Model com ADRs/DEBTs detalhados |
| 3 | Inventário código pendente factual em §2 | **✓** campos diferidos vanilla por família documentados |
| 4 | Matriz dependências cruzadas em §3 com 5 categorias | **✓** A Introspection / B multi-region / C crate externa / D ADR pendente / E outro módulo |
| 5 | Análise tecto Model em §4 com estimativas | **✓** 50% → ~55-60% (Bloco A); → ~68% (+ hayagriva); cross-módulo difícil |
| 6 | Sequência candidata em §5 com Bloco A populado | **✓** 5 candidatos Bloco A ordenados + Bloco B/C documentados |
| 7 | Recomendação concreta em §6 | **✓** P158B Supplement por lang figure (recomendação primária); secundária + terciária listadas |
| 8 | Sem novas reservas criadas | **✓** política P158 preservada — recomendações são para validação humana |
| 9 | ADR-0061 §"Aplicações cumulativas" actualizada | **✓** linha P159B com slope "—"; padrões inventariar primeiro N=14 → 15; §análise risco N=14 → 15 |
| 10 | `crystalline-lint`: zero violations | **✓ No violations found** |
| 11 | Sem alteração de hashes — `entities/content.rs` mantém `ec58d849` (décimo passo consecutivo) | **✓** zero código modificado; **décimo passo consecutivo** P156L → P159B |

---

## 8. Confirmação: ADR-0065 critério #5 quarta aplicação concreta com diversidade multi-feature

P159B é **quarta aplicação concreta** de ADR-0065 critério #5
(scope) após P157/P158/P159.

**Tipos de aplicação acumulados**:

| Passo | Tipo aplicação critério #5 | Scope |
|-------|----------------------------|-------|
| P157 | Divisão multi-passo | Single feature (table foundations) |
| P158 | Subset selection | Single feature (figure-kinds) |
| P159 | Par acoplado | Single feature (Bibliography+Cite) |
| **P159B** | **Multi-feature scope amplo** | **Múltiplas famílias 159+158+157+outros simultâneas** |

**Diferenciador P159B**: primeira aplicação multi-feature do
critério #5. Padrão consolida-se em 4 modalidades distintas —
critério #5 demonstra flexibilidade em **divisão / selecção /
acoplamento / scope amplo**.

ADR meta P156K continua a ganhar evidência empírica sem nova ADR.

---

## 9. Estado pós-P159B

- **Cobertura Layout**: **78%** (inalterada — escopo Model
  documental amplo).
- **Cobertura Model agregada**: ~50% (inalterada — passo
  documental).
- **Cobertura arquitectural total**: **82%** (inalterada).
- **Variants Content**: **58** (inalterada).
- **Stdlib funcs**: **48** (inalterada).
- **Tests**: **1174** typst-core lib (inalterada). Workspace:
  **1434** (inalterada).
- **Lint**: zero violations.
- **DEBTs**: zero criados ou fechados.
- **ADR-0061** §"Aplicações cumulativas": tabela slope ganha
  linha P159B (slope "—"); padrões N actualizados.
- **README ADRs**: entrada P159B adicionada antes de P159A.
- **Reservas pré-existentes**: ADR-0062 mantida (não
  reforçada).
- **Hash `content.rs`**: `ec58d849` (preservado — **décimo
  passo consecutivo** P156L → P159B).
- **Total ADRs**: **63** (inalterado).

### 9.1 Decisão crítica registada — Tecto Model

**Tecto Model puro vs pós-resolver dependências**:

| Cenário | Cobertura agregada Model | Esforço | Próxima direcção |
|---------|:------------------------:|:-------:|------------------|
| Pós-Bloco A apenas | ~55-60% | 3-4 sub-passos M | Decidir hayagriva ou cross-módulo |
| Pós-Bloco A + Bloco B (hayagriva) | ~68% (paridade ADR-0060) | +5-8 sub-passos M-XL | Refinos restantes Layout/Introspection |
| Pós-Bloco C (cross-módulo) | difícil estimar | refactor extenso | Saturação |

**Recomendação operacional**: **executar Bloco A primeiro** —
informação acumulada após cada sub-passo melhora decisão sobre
Bloco B/C.

---

## 10. Decisão pós-P159B (validação humana)

Per §6 do diagnóstico:

**Validação humana de §6 (recomendação)**:
- ✓ Aprovar P158B (recomendação primária) → redigir spec.
- ↗ Redirigir para P159D (recomendação secundária) → redigir
  spec.
- ↗ Redirigir para passo administrativo XS (criar ADR-0062
  PROPOSTO ou actualizar L0 content.md) → redigir spec.
- ✗ Mudar de módulo (Introspection ou Layout) → redigir P160
  como diagnóstico do novo módulo.

**Outras direcções pendentes** (sem reservas reforçadas):
- Continuar Fase 3 Layout (columns/colbreak — DEBT-56).
- Footnote area.
- Atacar Introspection (17%; mais fraco).
- Promover ADR-0061 a IMPLEMENTADO.
- Promover `extract_length` a helper público (N=7 patamar
  forte).
- Fechar DEBT-34e + DEBT-56 (refactor multi-region L+).
- Promover ADR-0060 a R1 com confirmação Fase 2 fechada.
- ADR meta XS de "ADR-0064 caso completion" (saturação atingida
  P157C).

ADR-0060 mantém-se `IMPLEMENTADO`. ADR-0061 mantém-se
`PROPOSTO`. ADR-0062 mantém-se reserva sem ficheiro.

---

## 11. Fechamento

P159B fecha como **passo diagnóstico amplo** (M-) — quarto
diagnóstico Model (P157 table; P158 figure-kinds; P159
bibliography+cite; **P159B amplo família 159 + outros refinos
Model**). **Auto-validação ADR-0065 critério #5 atinge 4
modalidades** (divisão / selecção / acoplamento / scope amplo
multi-feature).

**Decisão crítica registada**: tecto Model puro estimado
**~55-60%** (alcançável com 5 sub-passos Bloco A); +hayagriva
**~68%** (paridade ADR-0060 declarado).

**Recomendação primária**: **P158B Supplement automático por
lang em figure** — sem dependência cruzada hard; reuso padrão
P155 quotes; hash content.rs preservado; funcionalidade visível.

**Política "sem novas reservas" preservada** — recomendações
§5/§6 são para validação humana, não compromissos.

**Padrões pós-P159B**:
- Granularidade N=14 (inalterada — diagnóstico).
- Inventariar primeiro N=14 → **15** (quarta aplicação concreta
  critério #5 com diversidade multi-feature).
- §análise risco N=14 → **15** (passo diagnóstico amplo baixo
  risco; M-).
- **Hash content.rs preservado décimo passo consecutivo**
  (P156L → P159B) — padrão estabilidade L0 do content
  fortalece-se cumulativamente.

**ADR-0060 mantém `IMPLEMENTADO`** (P159B lê, não modifica).
**ADR-0061 mantém `PROPOSTO`** (Layout não tocado).
**ADR-0062 mantém-se reserva sem ficheiro** (não promovida).

**Pausa natural após P159B — diagnóstico estruturado completo
para todas as expansões pendentes da família 159 + outros
refinos Model; tecto Model documentado com estimativas
informadas; recomendação primária P158B identificada com
justificação multi-critério; política "sem novas reservas"
preservada. Decisão humana sobre próxima direcção tem máxima
informação acumulada — informação útil mesmo se mudar de
módulo, porque torna a decisão informada em vez de arbitrária.**
