# Relatório P159 — Diagnóstico Bibliography + Cite

Passo arquitectural de diagnóstico precedendo materialização.
**Não materializa código**. **Terceira aplicação concreta de
ADR-0065 critério #5** (scope determinado por inventário) após
P157 e P158 — auto-validação cumulativa do ADR meta P156K com
diversidade cross-feature confirmada (P157 multi-passo divisão;
P158 subset selection; P159 par acoplado). **Décima quarta
aplicação consecutiva** do padrão diagnóstico-primeiro.

P159 é o **maior dos três passos Model Fase 2 reservados** (XL
declarado em ADR-0060). Diagnóstico foi particularmente
importante para decidir como dividir o trabalho.

---

## 1. Resumo do diagnóstico (síntese das 5 secções)

### 1.1 §1 — ADRs relevantes

- **ADR-0060** declara P159 **XL** com `hayagriva` autorizada
  via ADR-0062.
- **ADR-0062** é apenas **reserva documentada** em README ADRs —
  **NÃO existe como ficheiro**. Promoção exigida se hayagriva
  for integrada.
- **ADR-0017** Introspection runtime adiada **NÃO bloqueia**
  subset minimal — Cite resolve walk single-pass como counters
  figure (P75).
- Precedentes ADR-0024/0023/0057 cobrem autorização de crate
  externa em L1.

### 1.2 §2 — Estado factual em código

- **Bibliography + Cite completamente ausentes** em código
  cristalino (zero matches grep).
- **DEBT-55** documenta plano completo XL com hayagriva,
  Cargo.toml, ADR-0062, variants, stdlib, introspect, layout,
  tests.
- **Vanilla integra hayagriva profundamente**: bibliography.rs
  1226 linhas; `Bibliography` interno usa
  `Arc<ManuallyHash<IndexMap<Label, hayagriva::Entry, FxBuildHasher>>>`.
- Vanilla `BibliographyElem` campos: sources/title/full/style/lang/region.
- Vanilla `CiteElem` campos: key/supplement/form/style.

### 1.3 §3 — Scope com avaliação 3 estruturas

3 estruturas avaliadas:

- **Estrutura A multi-passo análogo a P157**: P159A bibliography +
  P159B cite + P159C hayagriva. Cada um M.
- **Estrutura B minimal análogo a P158**: par num único M+ sem
  hayagriva. Quebra granularidade.
- **Estrutura C diferimento total** per ADR-0054 graded.

**Recomendação adoptada**: **Estrutura A adaptada** — par
acoplado Bibliography+Cite num único passo **M+** sem hayagriva
(input cristalino literal Vec<BibEntry>); refinos futuros NÃO
reservados.

Justificação:
- Bibliography e Cite são **inseparáveis funcionalmente**.
- Paridade vanilla acopla-os semanticamente.
- Granularidade quebrada **honestamente registada** (M+ vs M)
  com precedente P156C par lógico.
- Hayagriva contornada com input literal.

### 1.4 §4 — Dependências bloqueantes

**Zero bloqueios hard** para subset minimal estrutura A
adaptada:
- hayagriva: contornada com Vec<BibEntry> literal.
- ADR-0017 Introspection: walk single-pass viável.
- ADR-0062: NÃO necessária neste sub-passo.
- DEBT-55: pré-condição contornada.

Bloqueios para refinos futuros (NÃO scope P159A):
- Integração hayagriva → ADR-0062 promovida.
- CSL parsing → hayagriva.
- Cross-document forward refs → ADR-0017 promovida.

### 1.5 §5 — Esboço de P159A

**Identificador**: P159A (precedente P157A + P158A).

**Tamanho**: M+ (par funcional acoplado).

**Subset concreto**:
- Tipo `BibEntry { key, author, title, year }` em
  `entities/bib_entry.rs` novo.
- Variants `Content::Bibliography { entries, title }` +
  `Content::Cite { key, supplement }`.
- Stdlib `native_bibliography` + `native_cite` em
  `stdlib/structural.rs`.
- Layout: Bibliography como lista; Cite como `[key]` placeholder.
- Introspect: Bibliography não consume counter; Cite walk
  single-pass.

**Granularidade quebrada N=13 → M+** honestamente registada
(precedente P156C par lógico pad+hide M+).

**Risco médio**: 2 variants em paralelo + tipo novo em entities;
**quebra esperada padrão "preservação hash content.rs"** (sétimo
passo consecutivo termina; P159A será o oitavo passo onde hash
muda).

---

## 2. Decisão final de scope para P159A

**Estrutura A adaptada — par acoplado num único passo M+**.

Características:
- 1 tipo novo (`BibEntry`).
- 2 variants Content novos (Bibliography + Cite par funcional
  acoplado).
- 2 stdlib funcs novas (naming flat per padrão P157B).
- Layout placeholder (paridade vanilla mínima per ADR-0054
  graded).
- Tests ~15-20.
- Granularidade quebrada N=13 → M+ honestamente registada.

**Refinos futuros NÃO reservados** per política P158:
- Integração hayagriva (ADR-0062 a promover).
- CSL parsing.
- Form variants Normal/Prose/etc.
- Numbering schemes.
- Cross-document forward refs (ADR-0017 a promover).

---

## 3. Dependências identificadas a tratar antes de P159A

**Zero pré-requisitos hard**. P159A pode iniciar sem trabalho
prévio.

Notas operacionais:
- ADR-0062 mantém-se como reserva sem ficheiro — não criar em
  P159 nem em P159A (subset minimal não usa hayagriva).
- DEBT-55 mantém-se aberto após P159A (refinos futuros pendem
  de hayagriva integration).
- Política "sem novas reservas" preservada — passo seguinte a
  P159A decidido sequencialmente.

---

## 4. Análise de risco (padrão N=12 → 13; passo diagnóstico)

P159 é **passo diagnóstico** sem alteração de código. **Décima
terceira aplicação consecutiva** de §análise de risco
(P156F-P158A + P159) preservando precedente.

### 4.1 Riscos identificados

| Risco | Avaliação | Mitigação aplicada |
|-------|-----------|---------------------|
| ADR-0062 não cobrir hayagriva como esperado | Materializou como confirmado: NÃO existe ficheiro | Documentado em §1.2; P159A subset minimal contorna sem hayagriva |
| DEBT-55 ter scope diferente | Não materializou — DEBT-55 documentou plano completo XL com hayagriva, alinhado com expectativa | Conteúdo completo citado em §2.3 |
| Bibliography/Cite parcialmente implementados em cristalino com features custom | Não materializou — completamente ausentes | Documentado em §2.1 (zero matches grep) |
| ADR-0017 ser bloqueador hard de cite() | Não materializou — walk single-pass viável paridade counters figure | Documentado em §1.3 e §3 |
| hayagriva ser dependência runtime obrigatória per ADR-0033 paridade | Não materializou — paridade observable mínima aceite per ADR-0054 graded com placeholder render | Estrutura A adaptada usa input literal |
| Subset "bibliography + cite" forçar quebra de granularidade | Materializou — estrutura A adaptada quebra N=13 → M+ | Documentado honestamente em §3 e §5 com precedente P156C |

### 4.2 Riscos avaliados mas não materializados

| Risco | Avaliação inicial | Razão de não-materialização |
|-------|-------------------|----------------------------|
| 3 estruturas (A/B/C) requerem sub-decisão complexa | Médio | Inventário .1 + .2 forneceu factualidade suficiente — recomendação clara em §3.5 |
| Recomendação Estrutura A original quebrar paridade vanilla | Baixo | Adaptação "par acoplado num único passo" preserva paridade funcional aceitável |

### 4.3 Riscos não-aplicáveis

- **Refactor de código**: zero (passo puramente documental).
- **Quebra de contrato API**: zero.
- **Drift de hashes L0/L1**: zero.

### 4.4 Conclusão de risco

**Risco residual: muito baixo**. Padrão "passo diagnóstico
documental + scope determinado por inventário (ADR-0065 #5) +
política sem novas reservas" replica tratamento bem-sucedido de
P157/P158.

**Auto-validação ADR-0065 critério #5 atinge 3 aplicações
concretas com diversidade máxima**:
- P157: divisão M+ → 3xM (table foundations).
- P158: subset selection (figure-kinds minimal vs máximo).
- P159: par acoplado num único passo (Bibliography+Cite).

Critério #5 demonstra **flexibilidade cross-feature** —
aplica-se tanto a divisão quanto a selecção quanto a acoplamento.

---

## 5. Verificações (numeradas per spec do passo)

| # | Verificação | Resultado |
|---|-------------|-----------|
| 1 | Diagnóstico produzido com 5 secções | **✓** `diagnostico-bibliography-cite-passo-159.md` (5 secções §1-§5 com avaliação 3 estruturas em §3) |
| 2 | ADR-0060 §"Decisão" sobre Bibliography lida e resumida em §1 | **✓** §1.1; subset declarado XL com hayagriva citado literalmente |
| 3 | ADR-0062 lida e estado confirmado em §1 | **✓** §1.2; **NÃO existe como ficheiro** — confirmação factual |
| 4 | ADR-0017 lida e impacto em cite() determinado em §1 | **✓** §1.3; NÃO bloqueia subset minimal; bloqueia apenas cross-document forward refs |
| 5 | Estado de Content::Bibliography/Cite determinado factualmente em §2 | **✓** §2.1; **completamente ausentes** (zero matches grep) |
| 6 | DEBT-55 conteúdo completo documentado em §2 | **✓** §2.3 com plano XL completo + critério fecho + notas |
| 7 | Subset concreto com avaliação 3 estruturas e recomendação | **✓** §3.2/§3.3/§3.4 estruturas A/B/C; §3.5 recomendação Estrutura A adaptada |
| 8 | Dependências bloqueantes em §4 | **✓** zero bloqueios hard documentados; refinos futuros bloqueios listados |
| 9 | Esboço de P159A em §5 | **✓** identificador, tamanho M+, subset, sub-passos, granularidade quebrada honestamente registada |
| 10 | Sem novas reservas criadas | **✓** política P158 preservada — refinos pós-P159A NÃO reservados |
| 11 | ADR-0061 §"Aplicações cumulativas" actualizada com linha P159 | **✓** linha P159 com slope "—"; padrões inventariar primeiro N=12 → 13; §análise risco N=12 → 13 |
| 12 | `crystalline-lint`: zero violations | **✓ No violations found** |
| 13 | Sem alteração de hashes — `entities/content.rs` mantém `ec58d849` (oitavo passo consecutivo) | **✓** zero código modificado; hash preservado P156L → P159 — **oitavo passo consecutivo** |

---

## 6. Confirmação: ADR-0065 critério #5 terceira aplicação concreta

P159 é **terceira aplicação concreta** de ADR-0065 critério #5
(scope: atributos a incluir/diferir per ADR-0054 graded) após
P157 e P158.

Auto-validação cumulativa do ADR meta P156K — **diversidade
máxima cross-feature**:

| Passo | Tipo de aplicação critério #5 |
|-------|-------------------------------|
| P157 | **Divisão multi-passo** — table foundations M+ → 3xM (P157A/B/C) |
| P158 | **Subset selection** — figure-kinds minimal §3.2 vs máximo §3.3 vs intermédio §3.4 |
| **P159** | **Par acoplado num único passo** — Bibliography+Cite M+ par funcional inseparável |

**Padrão emergente confirmado**: critério #5 demonstra
flexibilidade — aplica-se a divisão, selecção E acoplamento.
ADR-0065 ganha utilidade empírica em 3 dimensões distintas sem
nova ADR.

---

## 7. Estado pós-P159

- **Cobertura Layout**: **78%** (inalterada — escopo Model
  documental).
- **Cobertura Model agregada**: ~50% (inalterada — passo
  documental).
- **Cobertura arquitectural total**: **80%** (inalterada).
- **Variants Content**: **56** (inalterada).
- **Stdlib funcs**: **46** (inalterada).
- **Tests**: **1147** typst-core lib (inalterada). Workspace:
  **1407** (inalterada).
- **Lint**: zero violations.
- **DEBTs**: zero criados ou fechados (DEBT-55 mantém-se aberto;
  pré-condição hayagriva contornada para subset minimal).
- **ADR-0061** §"Aplicações cumulativas": tabela slope ganha
  linha P159 (slope "—"); padrões N actualizados.
- **README ADRs**: entrada P159 adicionada antes de P158A.
- **Reservas pré-existentes**: ADR-0062 mantida (NÃO reforçada;
  política P158 preservada).
- **Hash `content.rs`**: `ec58d849` (preservado — **oitavo
  passo consecutivo** P156L → P159).
- **Total ADRs**: **63** (inalterado; ADR-0062 continua reserva
  sem ficheiro).

### 7.1 Próxima decisão

**P159A** redigido como spec separada com base no diagnóstico
§5. Ponto de validação humana antes de redigir spec.

Se P159A for aprovado:
- Materializa par acoplado Bibliography+Cite minimal.
- Cadência granular **quebrada** N=13 → M+ honestamente.
- Refino qualitativo de Model — primeiro caso de "par acoplado
  num único passo" pós-P156C.
- **Quebra padrão "preservação hash content.rs"** (sétimo passo
  consecutivo termina; novos variants no enum).

Se P159A for redirigido:
- Estrutura B (minimal num único M+) ou C (diferimento) ficam
  como alternativas documentadas.

Outras direcções pendentes (sem reservas reforçadas):
- Continuar refino figure-kinds (supplement por lang; NÃO
  reservado).
- Continuar Fase 3 Layout (columns/colbreak — DEBT-56).
- Footnote area.
- Atacar Introspection (17%; mais fraco).
- Promover ADR-0061 a IMPLEMENTADO.
- Promover `extract_length` a helper público (N=7 patamar
  forte).
- Fechar DEBT-34e e DEBT-56 (refactor multi-region L+).
- Promover ADR-0060 a R1 com confirmação Fase 2 sub-passo 3
  fechado.
- ADR meta XS de "ADR-0064 caso completion" (saturação atingida
  P157C).
- Promover ADR-0062 a IMPLEMENTADO (passo administrativo;
  precondição hayagriva integration).
- Criar ADR-0062 como ficheiro PROPOSTO (decisão prévia a
  hayagriva integration; passo administrativo XS).

---

## 8. Fechamento

P159 fecha como **passo diagnóstico documental** — terceiro
diagnóstico Model Fase 2 (P157 table foundations; P158
figure-kinds; P159 bibliography+cite). **Auto-validação
ADR-0065 critério #5 atinge maturidade** com 3 aplicações
concretas em 3 dimensões distintas (divisão / selecção /
acoplamento).

**Decisão crítica**: Estrutura A adaptada (par acoplado num
único passo M+ sem hayagriva). Granularidade quebrada
honestamente registada com precedente P156C.

**Política "sem novas reservas" preservada** (P158 estabeleceu;
P158A respeitou; P159 respeita) — refinos pós-P159A
(hayagriva, CSL, form variants, numbering schemes, cross-document
forward refs) permanecem candidatos NÃO-reservados.

**Padrões pós-P159**:
- Granularidade N=13 (inalterada — diagnóstico).
- Inventariar primeiro N=12 → **13** (terceira aplicação concreta
  critério #5).
- §análise risco N=12 → **13** (passo diagnóstico baixo risco;
  XL declarado torna inventário particularmente importante).
- **Hash `content.rs` preservado oitavo passo consecutivo**
  (P156L → P159) — padrão "estabilidade de contrato L0"
  fortalece-se; **quebra esperada em P159A**.

**ADR-0060 mantém `IMPLEMENTADO`** (P159 lê, não modifica).
**ADR-0061 mantém `PROPOSTO`** (Layout não tocado).
**ADR-0062 mantém-se reserva sem ficheiro** (não promovida
neste passo).

**Pausa natural após P159 — diagnóstico estruturado completo
para o maior dos três passos Model Fase 2 reservados; recomendação
informada de Estrutura A adaptada com granularidade quebrada
honestamente; padrão "sem novas reservas" preservado. Decisão
humana sobre P159A (ou outras 11 candidatas documentadas) tem
máxima informação acumulada.**
