# Passo 195A — Instrução Claude Code

## Contexto mínimo

Typst Cristalino é re-implementação atómica do projecto
`typst/typst` em Rust com arquitectura camadas L0–L4.
Vanilla original está em quarentena em `lab/typst-original/`.
Cristalino vive em `01_core/`, `02_shell/`, `03_infra/`,
`04_wiring/`. ADRs em `00_nucleo/adr/`.

**Snapshot de partida** (a confirmar empiricamente em
sub-passo .A):

- Tests workspace 1.825 verdes; zero violations.
- M9 ✅ 11/11.
- M4-residual fechado funcionalmente (P188B).
- M5 incremental (P189B): 1 arm migrado + 6 excepções.
- P193B abriu sub-store `ResolvedLabelStore`.
- P194B migrou consumer C4 com substitution-with-fallback.
- DEBT M5-residual: 2 pré-requisitos restantes.
- Trait `Introspector` 19 métodos.
- `TagIntrospector` 8 sub-stores.

P195 é **passo 3 da sequência §9 P189**: migrar walk arm
`Content::Labelled` para emitir Tag em vez de mutar
`state.resolved_labels` directamente.

**Primeira migração walk arm M5** — diferente de P193+P194
(infra + consumer). Trabalho mais arquitectural com
decisões reais a tomar.

P195 fecha **E4** (Labelled walk arm). E2 (Heading walk
arm auto-toc) **não fecha** em P195 — só residualmente em
P196. Razão: auto-toc do Heading também popula
`resolved_labels`; P196 trata isso.

**Aprendizado P186C/D crítico**: walk de introspect gateia
em `extract_payload(content).is_some()`, **não** em
`is_locatable(content)`. Se P195 escolher promoção a
locatable, ordem de implementação importa para evitar
dessincronização Locator.

Material de partida verificado:

- `00_nucleo/materialization/typst-passo-186-relatorio-consolidado.md`
  §4 — descoberta empírica do gating walk
  (`introspect.rs:329`); padrão pragmático auditor
  durante M4-residual.
- `00_nucleo/materialization/typst-passo-181-relatorio-consolidado.md`
  — promoção Bibliography a locatable; template para
  Opção 1.
- `00_nucleo/materialization/typst-passo-182-relatorio-consolidado.md`
  — `SetHeadingNumbering` via StateUpdate puro; template
  para Opção 2.
- `00_nucleo/materialization/typst-passo-194-relatorio-consolidado.md`
  §5 — estado temporário documentado; P195 activa
  Introspector path.
- `00_nucleo/materialization/typst-passo-189-relatorio-consolidado.md`
  §5 E4 — descrição da excepção: "Labelled walk arm (2
  mutações): `state.figure_label_numbers.insert(label,
  n)` + `state.resolved_labels.insert(label, text)`".

P195A é o passo de diagnóstico que precede a
implementação. Magnitude esperada **S** para diagnóstico;
implementação **S–M** dependendo de cláusula 1 (decisão
arquitectural sobre payload).

---

## Postura do auditor / executor

P195A é passo **L0-puro / diagnóstico-primeiro**, padrão
estabelecido em 16 aplicações.

- **Zero código tocado** em camadas cristalinas.
- **Zero testes** modificados.
- **Pode criar** ADR `PROPOSTO` — possível, conforme
  cláusula 1.
- **Pode abrir DEBT** se trabalho identificado for adiado.
- **Não modifica** walk, `from_tags`, sub-stores,
  consumer — P195B+.

**Magnitude diagnóstico**: S. Decisão central é cláusula
1 (forma do payload). Outras cláusulas seguem a partir
dela.

**Regra dos 2 eixos aplicável** confirmada em P189A `.A`
para Labelled — eixo 1 e eixo 2 ambos passam após
infraestrutura P193B.

**3 padrões pragmáticos do auditor** disponíveis (P186):
- Ajustar fixture de teste em vez de violar restrição.
- Violar restrição justificadamente quando spec é
  internamente inconsistente.
- Inlining em vez de chamar trait method para evitar
  circularidade estilística.

---

## Escopo

**Primário**: desenhar migração de walk arm
`Content::Labelled` para emitir Tag em vez de mutar
`state.resolved_labels` + `state.figure_label_numbers`
directamente.

**Confirmação**: validar inventário factual — forma
exacta do walk arm Labelled, conteúdo da Tag, arms já
existentes em `from_tags` e `extract_payload`.

**Decisões a tomar** — 7 cláusulas:

1. **Forma do payload — decisão arquitectural central**:
   - **Opção 1** — Promover Labelled a locatable kind
     (replica P186 Equation): adicionar
     `ElementPayload::Labelled { label, resolved_text,
     figure_number }` + `ElementKind::Labelled` +
     `is_locatable(Content::Labelled) = true` +
     `extract_payload` arm + `from_tags` arm.
   - **Opção 2** — Usar mecanismo `ElementPayload::StateUpdate`
     existente (replica P182C SetHeadingNumbering): walk
     emite múltiplas StateUpdate Tags por Labelled
     (uma para cada field afectado).
   - **Opção 3** — Mistura: nova variant `ElementPayload::Labelled`
     mas **não** locatable (sem `is_locatable=true` nem
     `ElementKind::Labelled`); apenas `extract_payload`
     produz payload.

2. **Janela de invariante quebrada (Opção 1 apenas)** —
   per aprendizado P186C/D, se promoção a locatable for
   escolhida, ordem importa. Sub-passos B+ devem aplicar
   sequência segura per P186 (corrected ordering).

3. **Tratamento dos 2 fields mutados** —
   `figure_label_numbers` (sub-store já existente per
   P168) + `resolved_labels` (sub-store novo P193B). Como
   o payload da Tag carrega ambos? Estrutura única ou
   2 Tags?

4. **Auto-label vs explicit label** — `resolved_labels`
   é populated pelo Heading auto-toc (E2) **e** pelo
   Labelled explicit (E4). P195 fecha apenas E4
   (Labelled). E2 fica para P196. Confirmar que walk
   arm Labelled é independente do walk arm Heading
   auto-toc.

5. **Conteúdo da Tag — `resolved_text`** — texto
   resolvido depende do conteúdo do Labelled. Como
   computar? Walk arm já tem o texto (per E4 mutação
   actual), apenas redirecciona para Tag em vez de
   state.

6. **`figure_label_numbers` populate** — sub-store já
   existe (per P168 figure-ref); como o walk arm Labelled
   actual popula? Mantém via `from_tags` ou nova
   primitiva?

7. **Critério de fecho de P195** — walk arm Labelled
   puro (sem mutação directa de state); `from_tags`
   popula sub-stores via Tag; tests E2E confirmam paridade
   observable + activação Introspector path para C4 (P194)
   em produção.

**Fora de escopo**:

- Migração walk arm Heading (P196).
- Migração walk arm Figure (P197).
- Migração walks SetHeadingNumbering + CounterUpdate
  (P198).
- `SetEquationNumbering` materialização.
- Sub-store `headings_for_toc` (lacuna #3).
- Eliminação `CounterStateLegacy` (M6).

---

## Critérios objectivos

### O1 — Inputs verificáveis

`grep -rn "Content::Labelled" 01_core/src/`. Para cláusula
1, confirmar shape de `ElementPayload` actual + impacto
de adicionar variant. Para cláusula 5, confirmar como
walk arm computa `resolved_text` actualmente.

### O2 — Alternativas

Cláusula 1 tem 3 opções com trade-offs distintos.
Cláusula 3 tem 2-3 opções (estrutura única vs múltiplas
Tags).

### O3 — Critério de escolha

- Opção 1 (promoção locatable) replica P186 Equation —
  estrutura padrão, mas mais código.
- Opção 2 (StateUpdate) replica P182C — menos código,
  mas múltiplas Tags por Labelled.
- Opção 3 (variant não-locatable) compromisso.

Critério dominante: simplicidade do downstream
(consumer C4 já existe e usa `resolved_label_for` —
funciona com qualquer das 3 opções).

### O4 — Magnitude

P195 implementação S–M conforme cláusula 1:
- Opção 1: M (mais código + ordem cuidada per P186).
- Opção 2: S (menos código).
- Opção 3: S–M.

### O5 — Reversibilidade

Todas as opções reversíveis. Opção 1 mais difícil de
reverter (mais sítios afectados).

---

## Critérios qualitativos

### Q1 — Consistência com padrão estabelecido

Opção 1 replica P186 Equation literalmente.
Opção 2 replica P182C SetHeadingNumbering.
Opção 3 não tem precedente directo.

### Q2 — Honestidade de magnitude

P195A diagnóstico é S. P195B+ implementação:
- Opção 1: ~150-250 LOC + tests E2E ≈ M.
- Opção 2: ~80-150 LOC + tests ≈ S.
- Opção 3: ~100-200 LOC ≈ S–M.

Auditor decide empiricamente em `.A`.

### Q3 — Cobertura sem regressão

P195 mantém output observable preservado:
- Walks legacy E2 (Heading) continua a popular
  `state.resolved_labels` para auto-toc (até P196).
- Walks legacy E3 (Figure) continua a popular
  `state.figure_label_numbers` (até P197).
- Após P195, **2 caminhos populam** sub-stores Introspector:
  walk arm Labelled (Tag) e legacy via E2/E3 não
  migrados.

Tests E2E confirmam paridade.

### Q4 — Estado activação caminho Introspector

Após P195, **caminho Introspector activa parcialmente**:
- `intr.resolved_labels` populated **só para Labelled
  explicit** (E4 fechou).
- `intr.resolved_labels` **NÃO populated** para Heading
  auto-toc (E2 ainda activa) — fica para P196.

Consumer C4 (P194) começa a receber `Some(text)` para
labels explicit. **Inversão observable parcial**.

### Q5 — Granularidade

P195 sub-passo único agregado se Opção 2 ou 3 escolhida.
**Série multi-passo** (B-F similar a P186) se Opção 1
escolhida — ordem importa per P186C/D.

---

## Sub-passos de P195A

### Sub-passo 195A.A — Validação do estado actual

Auditor confirma empiricamente:

1. Confirmar walk arm `Content::Labelled` actual:
   - `01_core/src/rules/introspect.rs` — localizar arm.
   - Mutações empíricas (per P189B §5 E4):
     - `state.figure_label_numbers.insert(label, n)`.
     - `state.resolved_labels.insert(label, text)`.
   - Confirmar linha exacta + contexto.

2. Confirmar `Content::Labelled` variant:
   - `01_core/src/entities/content.rs` — localizar variant.
   - Campos: `label`, `body` (esperado).
   - Confirmar empiricamente.

3. Confirmar `ElementPayload` actual:
   - 10 variants após P186B (incl. Equation).
   - Identificar onde adicionar variant Labelled (Opção
     1) ou se reusar StateUpdate (Opção 2).

4. Confirmar `ElementKind` actual:
   - 9 variants após P186B (incl. Equation).
   - Identificar onde adicionar Labelled (Opção 1).

5. Confirmar `is_locatable` actual:
   - Localizar arm `Content::Labelled` actual (esperado:
     `false`).
   - Para Opção 1: passa a `true`.

6. Confirmar `extract_payload` actual:
   - Localizar arm `Content::Labelled` actual (esperado:
     catch-all → `None`).
   - Para Opção 1: arm específico que produz payload.

7. Confirmar `from_tags` actual:
   - Match exhaustivo per P186B descoberta.
   - Adição de variant força arm explícito.

8. Confirmar consumer C4 (P194B):
   - `references.rs:53-57` consulta
     `intr.resolved_label_for(target)`.
   - Após P195, path activa para labels explicit.

9. Confirmar `resolved_text` computation:
   - Como walk arm actual computa o texto resolvido?
   - É baseado em `body` do Labelled? Em outro estado?
   - Identificar para preservar semântica.

10. Confirmar `figure_label_numbers` populate actual:
    - Sub-store já existente (P168).
    - Como walk arm Labelled lê
      `state.figure_numbers["figure"]` para popular?
    - Confirmar fluxo.

11. Aplicar regra dos 2 eixos:
    - **Eixo 1**: consumer C4 precisa do valor "durante
      walk" ou snapshot final?
      - Per P194: snapshot final (consumer lê após walk
        completo).
    - **Eixo 2**: sub-store `resolved_labels` populated
      em produção?
      - Após P195: sim (parcial — apenas explicit
        labels). Após P196: completo.

Output: tabela com item + estado + linhas exactas para
cada inventário.

**Critério de saída**:
- Walk arm Labelled localizado com mutações exactas.
- `Content::Labelled` variant confirmado.
- ElementPayload/Kind/is_locatable/extract_payload/from_tags
  estado actual confirmado.
- `resolved_text` computation entendida.

### Sub-passo 195A.B — Decisão cláusula 1 (forma do payload)

**Opção 1 — Promover Labelled a locatable kind**:

Vantagens:
- Replica P186 Equation literalmente — padrão
  estabelecido.
- `kind_index[Labelled]` permite query por kind (útil
  para introspecção futura).
- `extract_payload` arm carrega payload rico.

Desvantagens:
- Mais código (5 sítios afectados:
  ElementPayload/ElementKind/is_locatable/extract_payload/from_tags).
- Aplica aprendizado P186C/D — ordem cuidada para evitar
  janela invariante quebrada.
- Magnitude M.

**Opção 2 — Usar `ElementPayload::StateUpdate`**:

Vantagens:
- Replica P182C SetHeadingNumbering — padrão estabelecido.
- Menos código (apenas walk arm + `from_tags`
  StateUpdate arm já cobre).
- Magnitude S.

Desvantagens:
- Múltiplas Tags por Labelled (uma por field afectado).
- `StateUpdate` é genérico; não captura semântica
  específica de Labelled.
- Sub-store `resolved_labels` precisa de chave especial
  para encontrar o valor (ex.: `"resolved_label:{label}"`).

**Opção 3 — Variant não-locatable**:

Vantagens:
- `ElementPayload::Labelled` permite payload rico.
- Não exige `is_locatable=true` (sem janela invariante
  quebrada).
- Magnitude S–M.

Desvantagens:
- Não tem precedente directo em cristalino.
- Decisão arquitectural nova — provável ADR.

Critério de escolha:
- Opção 1: maior consistência com padrão M4-residual
  (P186 Equation é precedente recente).
- Opção 2: menor custo, mas não-natural.
- Opção 3: novidade arquitectural — ADR.

Sugestão preliminar: **Opção 1** (replica P186). Auditor
decide empiricamente após `.A`.

Output: decisão fixada com justificação literal.

### Sub-passo 195A.C — Decisão cláusula 2 (ordem para Opção 1)

**Aplicável apenas se cláusula 1 = Opção 1**.

Per P186C/D corrected ordering:
1. **Sub-passo P195B**: adicionar variants
   `ElementPayload::Labelled { ... }` +
   `ElementKind::Labelled` (sem efeito ainda; replica
   P186B).
2. **Sub-passo P195C**: adicionar arm em `extract_payload`
   produzindo `Some(...)` (ainda `is_locatable=false`;
   arm latente; replica P186C corrected).
3. **Sub-passo P195D**: activar `is_locatable=true` +
   ajustar tests P185D se necessário (replica P186D
   corrected).
4. **Sub-passo P195E**: estender `from_tags` arm Labelled
   (que já existe se cláusula 3 Opção A escolhida) com
   populate completo dos sub-stores (replica P186E).
5. **Sub-passo P195F**: tests E2E + relatório consolidado.

**Aplicável apenas se cláusula 1 = Opção 2 ou 3**:
sequência mais simples — passo único agregado.

Output: ordem fixada conforme cláusula 1.

### Sub-passo 195A.D — Decisão cláusula 3 (estrutura do payload)

Walk arm Labelled muta 2 fields. Como Tag carrega ambos?

**Opção α** — Estrutura única `ElementPayload::Labelled
{ label, resolved_text, figure_number }`:
- 1 Tag por Labelled.
- Payload carrega tudo o que `from_tags` precisa.
- `figure_number` é `Option<usize>` (não-figure → None).

**Opção β** — 2 Tags separadas
(`ElementPayload::ResolvedLabel { label, text }` +
`ElementPayload::FigureLabel { label, n }`):
- Mais granular.
- `from_tags` arm cada uma popula seu sub-store.

**Opção γ** — Aproveita `StateUpdate` para `resolved_labels`
e mantém `figure_label_numbers` em arm separado:
- Mistura de mecanismos.

Sugestão: **Opção α** (estrutura única) — replica P186
ElementPayload Equation que carrega múltiplos fields
(`block`, `counter_update`).

Output: decisão fixada.

### Sub-passo 195A.E — Decisão cláusula 4 (auto-label vs explicit)

Confirmar empiricamente em `.A` que:
- Walk arm Labelled é **independente** do walk arm
  Heading auto-toc.
- P195 migra apenas Labelled; E2 (Heading auto-toc) fica
  activo.
- Após P195, sub-store `resolved_labels` populated apenas
  parcialmente (só explicit labels).

**Risco**: se walk arm Labelled lê de
`state.resolved_labels` durante walk para o seu próprio
trabalho (improvável), migração introduz ciclo.

Output: independência confirmada empiricamente.

### Sub-passo 195A.F — Decisão cláusula 5 (resolved_text computation)

Per `.A.9`, walk arm actual computa `resolved_text` baseado
em algo (provável: `body` do Labelled + estado de
counters).

Para Opção 1 + α:
- `extract_payload(Content::Labelled { label, body })`
  precisa de computar `resolved_text`.
- Mas `extract_payload` é função pura (não tem acesso a
  state) — não pode replicar a lógica de walk.

**Cláusula gate substancial potencial**: se computation
de `resolved_text` exige state, Opção 1 fica difícil.
Auditor decide empiricamente.

Possível solução: payload carrega `body` em vez de
`resolved_text`; `from_tags` arm computa `resolved_text`
no momento de populate (com acesso a estado parcial via
sub-stores construídos até esse ponto).

Output: decisão empírica per `.A.9`.

### Sub-passo 195A.G — Decisão cláusula 6 (figure_label_numbers populate)

Per `.A.10`:
- Walk arm Labelled lê `state.figure_numbers["figure"]`
  durante walk para popular `figure_label_numbers`.
- Após P195, walk arm emite Tag → `from_tags` precisa
  ler `intr.counters["figure"]` para popular
  `figure_label_numbers`.
- Mas `intr.counters["figure"]` é populated por
  `from_tags` arm Figure (P184B) — pode estar populated
  parcialmente até este ponto?

**Cláusula gate substancial potencial**: ordem dos arms
em `from_tags` importa. Auditor confirma empiricamente.

Output: ordem `from_tags` confirmada.

### Sub-passo 195A.H — Decisão cláusula 7 (critério de fecho)

P195 fecha quando:
- Walk arm `Content::Labelled` puro (sem mutação directa
  de state).
- `from_tags` arm Labelled popula:
  - `intr.resolved_labels` (sub-store P193B).
  - `intr.figure_label_numbers` (sub-store P168).
- E4 fecha (excepção P189B).
- Tests E2E confirmam paridade observable.
- Consumer C4 (P194) começa a receber `Some(text)` para
  labels explicit em produção real (inversão observable
  parcial).

E2 (Heading auto-toc) **NÃO fecha em P195** — só
residualmente em P196.

Output: critério literal verificável.

### Sub-passo 195A.I — Validação do plano de sub-passos

Tabela esperada conforme cláusula 1:

**Se Opção 1 (locatable)** — série B–F (replica P186):

| Sub-passo | Escopo | Magnitude |
|-----------|--------|-----------|
| `.B` | Adicionar variants `ElementPayload::Labelled` + `ElementKind::Labelled` + L0s + tests | S |
| `.C` | Adicionar arm `extract_payload` + L0 (`is_locatable` ainda false) | S |
| `.D` | Activar `is_locatable=true` + ajuste tests P185D | trivial |
| `.E` | Estender `from_tags` arm Labelled (substituir stub no-op P186-style por arm completo) | S |
| `.F` | Tests E2E + relatório consolidado P195 | S |

**Se Opção 2 (StateUpdate)** — passo único:

| Sub-passo | Escopo | Magnitude |
|-----------|--------|-----------|
| `.B` | Walk arm emite StateUpdate Tags + `from_tags` ajustes + tests E2E + L0 + relatório | S |

**Se Opção 3 (variant não-locatable)** — 2 sub-passos:

| Sub-passo | Escopo | Magnitude |
|-----------|--------|-----------|
| `.B` | Adicionar variant `ElementPayload::Labelled` (não-locatable) + L0 + ADR `PROPOSTO` | S |
| `.C` | Walk arm + `from_tags` + tests + relatório | S |

Output: tabela final conforme cláusula 1.

### Sub-passo 195A.J — ADR

Avaliar:

- **Opção 1** (locatable): replica P186 Equation; **não
  ADR**.
- **Opção 2** (StateUpdate): replica P182C; **não ADR**.
- **Opção 3** (variant não-locatable): novidade
  arquitectural; **ADR `PROPOSTO`** registando padrão
  novo.

Conclusão: depende de cláusula 1.

### Sub-passo 195A.K — DEBT

P195 fecha **E4** (Labelled walk arm).

DEBT M5-residual após P195B:
- Antes: 2 pré-requisitos restantes
  (`headings_for_toc`, `SetEquationNumbering`).
- Após: 2 pré-requisitos restantes (P195 fecha E4 mas
  não avança pré-requisitos — esses são para
  destrancar excepções; E4 é uma excepção em si).
- **Excepções E1, E2, E3, E5, E6 continuam activas**.
  E4 fechada.

**Cenário B continua** (sem DEBT formal aberto).

Output: estado actualizado.

### Sub-passo 195A.L — Outputs

Produzir 3 ficheiros (padrão P181A–P194A):

1. **`00_nucleo/diagnosticos/diagnostico-walk-labelled-passo-195a.md`**
   — diagnóstico com 8 secções:
   - §1 Validação estado actual.
   - §2 Decisões cláusula 1–7 (formato O1–O5).
   - §3 Plano de sub-passos sem condicionais (conforme
     cláusula 1).
   - §4 Magnitude consolidada.
   - §5 ADR avaliação.
   - §6 DEBT avaliação.
   - §7 Janela invariante quebrada (se Opção 1) — plano
     de mitigação per P186C/D corrected.
   - §8 Próximo sub-passo (P195B com escopo concreto).

2. **`00_nucleo/materialization/typst-passo-195a-relatorio.md`**
   — relatório com 14 secções (padrão P181A/etc.).

3. **ADR `PROPOSTO`** apenas se cláusula 1 = Opção 3.

---

## Restrições

- **Zero código tocado** em qualquer ficheiro fora de
  `00_nucleo/`.
- **Zero testes** modificados.
- **Não criar reservas** de identificadores.
- **Não modificar walk** — P195B+.
- **Não tocar `from_tags`** — P195B+.
- **Não modificar trait `Introspector`** — P185B fechou.
- **Não modificar `TagIntrospector`** — P193B fechou.
- **Não modificar consumer C4** — P194B fechou.
- **Não migrar walk arm Heading** — P196.
- **Não migrar walk arm Figure** — P197.
- **Não inflar linguagem**: sem "patamar", "limiar",
  "consolidação", "deriva", "subpadrão", "cumulativo",
  "cross-domínio", "paridade observable" como bandeira
  retórica.
- **Aplicar regra dos 2 eixos** a auditoria empírica.
- **Aplicar aprendizado P186C/D** se Opção 1 escolhida —
  ordem corrigida obrigatória.
- **Sem cláusulas condicionais nos sub-passos `.B`+ do
  plano**.

---

## Critério de conclusão

- Diagnóstico em
  `00_nucleo/diagnosticos/diagnostico-walk-labelled-passo-195a.md`
  com 8 secções produzido.
- Relatório em
  `00_nucleo/materialization/typst-passo-195a-relatorio.md`
  com 14 secções produzido.
- 7 cláusulas fechadas com decisão literal.
- Plano de sub-passos sem condicionais (1, 2 ou 5
  sub-passos conforme cláusula 1).
- Magnitude consolidada confirmada.
- Critério de fecho P195 fixado.
- ADR avaliada (criada se Opção 3).
- DEBT M5-residual estado registado.
- Janela invariante quebrada plano de mitigação (se
  Opção 1).
- Aprendizado P186C/D aplicado (se Opção 1).
- Regra dos 2 eixos aplicada empiricamente.
- Nenhum ficheiro de cristalino tocado.
- Tests workspace 1.825 inalterados.
- `crystalline-lint .` zero violations.

P195A é instrumento. Migração concreta de walk arm
Labelled começa em P195B+.
