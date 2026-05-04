# Passo 197A — Instrução Claude Code

## Contexto mínimo

Typst Cristalino é re-implementação atómica do projecto
`typst/typst` em Rust com arquitectura camadas L0–L4.
Vanilla original está em quarentena em `lab/typst-original/`.
Cristalino vive em `01_core/`, `02_shell/`, `03_infra/`,
`04_wiring/`. ADRs em `00_nucleo/adr/`.

**Snapshot de partida** (a confirmar empiricamente em
sub-passo .A):

- Tests workspace 1.843 verdes; zero violations.
- M9 ✅ 11/11.
- M4-residual fechado funcionalmente (P188B).
- M5 incremental:
  - P189B: Outline migrado + 6 excepções declaradas.
  - P193B: sub-store `ResolvedLabelStore` aberto.
  - P194B: consumer C4 migrado.
  - P195B-E: walk arm Labelled migrado (E4
    estruturalmente fechada). ADR-0069 ACEITE.
  - P196B: walk arm Heading auto-toc migrado (E2
    fecha 3/4; E2-residuo declarado).
- DEBT M5-residual: 2 pré-requisitos restantes.
- Trait `Introspector`: 19 métodos.
- `TagIntrospector`: 8 sub-stores.
- `ElementPayload`: 11 variants.
- `ElementKind`: 9.
- 4 excepções activas + 1 residuo: E1, E2-residuo, E3,
  E5, E6.

P197 é **passo 5 da sequência §9 P189**: migrar walk arm
`Content::Figure` para emitir Tag em vez de mutar
`state.figure_numbers` directamente.

**Pattern ADR-0069 estabelecido** com 2 variantes
operacionais (P196 §10):
- **P195D variant** — target não-locatable: snapshot+
  find_map.
- **P196B variant** — content locatable: `emitted_loc`
  directo.

Figure é locatable (`is_locatable(Content::Figure) =
true` per P164) → variante P196B aplicável directamente.
**3ª aplicação concreta do pattern** — incerteza
arquitectural reduzida.

P197 fecha **E3** (Figure walk arm). Per P189B §5 E3:
- "Figure walk arm (2 mutações):
  `state.local_figure_counters.entry(...).or_insert(0)`,
  `state.figure_numbers.entry(...).or_default().push(...)`".

Sub-store correspondente:
- `figure_numbers` — popula `intr.figure_numbers` —
  **a confirmar empiricamente** (esperado: já existe
  como sub-store em `TagIntrospector` per P184B; ou
  reuso de `figure_label_numbers` per P168).
- `local_figure_counters` — sub-store dedicado **a
  determinar** em `.A`. Pode ser parte de
  `CounterRegistry` ou reuso de mecanismo existente.

**Cláusula gate substancial potencial**: cadeia E3
descrita em P189B §5 + §"Padrão de cadeia" como
"chained com E2". Walk arm Labelled (P195D) **lê**
`state.figure_numbers` durante walk para popular
`figure_label_numbers`. Migrar Figure mutation pode
introduzir ordem-dependência.

Material de partida verificado:

- `00_nucleo/materialization/typst-passo-196-relatorio-consolidado.md`
  §9 — pattern ADR-0069 disponível para P197.
- `00_nucleo/materialization/typst-passo-189-relatorio-consolidado.md`
  §5 E3 — descrição da excepção.
- `00_nucleo/adr/typst-adr-0069-post-recursion-tag-emission.md`
  — ACEITE; aplicabilidade futura registada (P197
  explicitamente).
- `00_nucleo/materialization/typst-passo-184-relatorio-consolidado.md`
  — P184B abriu sub-store `figure_numbers` ou similar
  (a confirmar).
- `00_nucleo/materialization/typst-passo-168-relatorio-consolidado.md`
  — P168 figure-ref via Introspector; reuso de
  `figure_label_numbers`.

P197A é o passo de diagnóstico. Magnitude esperada **S**
(replica P196A/P195A — pattern + variante já
estabelecidos).

---

## Postura do auditor / executor

P197A é passo **L0-puro / diagnóstico-primeiro**, padrão
estabelecido em 18 aplicações.

- **Zero código tocado** em camadas cristalinas.
- **Zero testes** modificados.
- **Pode criar** ADR `PROPOSTO` — improvável (pattern
  ADR-0069 cobre).
- **Pode abrir DEBT** se trabalho identificado for
  adiado.
- **Não modifica** walk, `from_tags`, sub-stores,
  consumer — P197B+.

**Magnitude diagnóstico**: S. Decisões esperadas
seguem padrão P196A — pattern ADR-0069 + variante
P196B (locatable) já estabelecidos.

**Regra dos 2 eixos aplicável** confirmada para Figure
em P189A `.A` (eixo 1 snapshot final; eixo 2 sub-store
populado parcialmente per P184B/P168).

**Pattern post-recursion ADR-0069 disponível** —
reaproveitar variante P196B (Heading) directamente.

---

## Escopo

**Primário**: desenhar migração de walk arm
`Content::Figure` para emitir Tag em vez de mutar
`state.figure_numbers` + `state.local_figure_counters`
directamente.

**Confirmação**: validar inventário factual — forma
exacta do walk arm Figure, mutações actuais, interacção
com walk arm Labelled (cadeia E2-E3), sub-stores
existentes.

**Decisões a tomar** — 7 cláusulas:

1. **Forma do payload** — variant `ElementPayload::Figure`
   já existe? Se sim, expandir; se não, decidir entre:
   - **Opção 1** — Adicionar variant
     `ElementPayload::Figure { kind, number,
     supplement }` ou similar.
   - **Opção 2** — Reusar `ElementPayload::Labelled`
     (P195B) — improvável porque Figure não é label
     resolution; é counter step.
   - **Opção 3** — Variant existente `ElementPayload::Equation`
     (P186B) tem campo `counter_update` — pattern
     similar pode aplicar.

2. **Helper `compute_figure`**:
   - Análogo a `compute_labelled` (P195D) e
     `compute_heading_auto_toc` (P196B).
   - Replica lógica actual de geração de número Figure
     (provável: `state.figure_numbers["figure"]` step
     ou similar).

3. **Tratamento de `local_figure_counters`** vs
   `figure_numbers`:
   - 2 mutações distintas. Confirmar empiricamente
     em `.A`:
     - `local_figure_counters` é counter local a
       Figure (per kind?). Mutação 1.
     - `figure_numbers` é registo global. Mutação 2.
   - Decidir se ambas migram para mesmo Tag ou
     separadas.

4. **Cadeia E2-E3 (interacção com Labelled P195D)**:
   - Walk arm Labelled lê `state.figure_numbers`
     durante walk para popular `figure_label_numbers`.
   - Após P197, mutação `state.figure_numbers` migra
     para Tag → `from_tags` arm popula
     `intr.figure_numbers` (ou sub-store equivalente).
   - **Cláusula gate substancial**: se walk arm
     Labelled lê **estado mutado durante walk**
     (`state.figure_numbers`), e P197 redirecionar
     para Tag, pode quebrar caminho legacy intermédio.
     Auditor confirma empiricamente.

5. **Locator handling** — Figure é locatable (P164).
   Variante P196B aplicável: `emitted_loc` directo.

6. **Mutação legacy preservada** (write paralelo
   durante janela compat M5) — replica P195D/P196B
   pattern.

7. **Critério de fecho de P197** — walk arm Figure
   emite Tag pós-recursão; `from_tags` arm processa;
   sub-store correspondente populated em produção;
   tests E2E confirmam paridade observable. **E3
   fecha estruturalmente**. Funcionalmente em M6.

**Fora de escopo**:

- Migração walks SetHeadingNumbering + CounterUpdate
  (P198).
- `SetEquationNumbering` materialização.
- Sub-store `headings_for_toc` (lacuna #3 — passo
  dedicado paralelo).
- Eliminação `CounterStateLegacy` (P190/P200).

---

## Critérios objectivos

### O1 — Inputs verificáveis

`grep -rn "Content::Figure" 01_core/src/`. Para cláusula
1, confirmar se variant `ElementPayload::Figure` já
existe. Para cláusula 3, confirmar mutações exactas e
sub-stores correspondentes.

### O2 — Alternativas

Cláusula 1 tem 2-3 opções dependendo de auditoria.
Cláusula 3 tem 2 opções (mesmo Tag vs separadas).

### O3 — Critério de escolha

Pattern ADR-0069 + variante P196B já estabelecidos.
Replicação directa preferida.

### O4 — Magnitude

P197 implementação **M agregada** (replica P196B):
- 1 sub-passo único agregado: walk arm + helper +
  tests E2E + L0 + relatório. Magnitude M.
- Alternativa: 2 sub-passos (B walk arm + C consolidado)
  per padrão P196B+P196C.

### O5 — Reversibilidade

Reversível por construção (write paralelo legacy
preservado).

---

## Critérios qualitativos

### Q1 — Consistência com padrão estabelecido

Variante P196B replica:
- Pattern post-recursion (ADR-0069).
- Helper privado para computação.
- `emitted_loc` directo (Figure locatable).
- Mutação legacy paralela durante janela compat.
- Tests E2E paridade observable + activação.

Diferença esperada vs P196B: payload pode ser nova
variant (Opção 1) se Figure tem semântica distinta.

### Q2 — Honestidade de magnitude

P197A diagnóstico é S. P197B+ implementação:
- Provável 2 sub-passos (B + C) per padrão P196.
- Magnitude M agregada.

Total agregado: ~80 LOC walk arm + ~40 LOC helper +
~150 LOC tests + relatório consolidado ≈ M.

### Q3 — Cobertura sem regressão

P197 mantém output observable preservado:
- Mutação legacy 2 mutações continuam activas até P197B
  modificar.
- Após P197: mutações redireccionam para Tag write
  paralelo; legacy preservada.
- Consumer downstream (Layouter Figure rendering,
  outline) pode receber dados via Introspector ou
  legacy — depende de auditoria.

**Risco moderado**: cadeia E2-E3 — walk arm Labelled lê
`state.figure_numbers`. Se ordem mudar, paridade pode
quebrar. Auditor decide empiricamente em `.A`.

### Q4 — E3 fecha (versus E2 que fechou parcialmente)

E3 só tem 2 mutações (vs 4 de E2). Se ambas mutações
têm sub-store destino:
- E3 fecha **estruturalmente** completa em P197 —
  diferente de E2 que ficou com residuo.
- Inversão observable parcial: figure numbering via
  Introspector path activa.

### Q5 — Granularidade

Conforme padrão P196:

| Sub-passo | Escopo | Magnitude |
|-----------|--------|-----------|
| `.B` | Walk arm Figure + helper + tests E2E + L0 | M |
| `.C` | Relatório consolidado P197 + actualização DEBT | S |

---

## Sub-passos de P197A

### Sub-passo 197A.A — Validação do estado actual

Auditor confirma empiricamente:

1. Confirmar walk arm `Content::Figure` actual:
   - `01_core/src/rules/introspect.rs` — localizar arm.
   - Mutações empíricas (per P189B §5 E3):
     - `state.local_figure_counters.entry(...).or_insert(0)`
       seguido de step.
     - `state.figure_numbers.entry(...).or_default().push(...)`.
   - Confirmar linhas exactas + contexto.
   - Identificar **condição** (provável:
     `is_counted` ou `numbering.is_some()`).

2. Confirmar `Content::Figure` variant:
   - `01_core/src/entities/content.rs` — localizar.
   - Campos: `kind`, `numbering`, `caption`, `body`,
     possivelmente outros.
   - Confirmar empiricamente.

3. Confirmar `is_locatable(Content::Figure)`:
   - Esperado: `true` (P164 promoção).
   - `emitted_loc` em scope do arm.

4. Confirmar `extract_payload(Content::Figure)`:
   - Esperado: já existe arm (variant
     `ElementPayload::Figure` em uso ou similar per
     P184B).
   - **Crítico**: se já existe payload, P197 expande
     ou redefine?

5. Confirmar variant `ElementPayload::Figure` actual:
   - Verificar empiricamente em
     `01_core/src/entities/element_payload.rs`.
   - Campos actuais.
   - **Cláusula 1 depende criticamente disto**:
     - Se variant existe + cobre semântica nova:
       reuso (mais simples).
     - Se variant existe + falta campo: expansão.
     - Se variant não existe: adicionar.

6. Confirmar `from_tags` arm Figure (P184B?):
   - Localizar arm que processa tags Figure.
   - Verificar se já popula `intr.figure_numbers` ou
     similar.
   - Identificar gap entre populate actual e populate
     necessário pós-P197.

7. Confirmar sub-stores existentes para Figure:
   - `intr.figure_numbers` — esperado existir per
     P184B.
   - `intr.figure_label_numbers` — populated por P168
     + P195D (write paralelo redundante per P195A
     §11.3).
   - **`intr.local_figure_counters`** — provavelmente
     **NÃO existe**. Decidir: criar ou ignorar (per
     uso real downstream).

8. Confirmar consumer downstream de
   `state.figure_numbers`:
   - `grep -rn "figure_numbers" 01_core/src/`.
   - Identificar consumers em Layouter (Figure
     rendering) e/ou outros.
   - Aplicar regra dos 2 eixos:
     - Eixo 1: consumer precisa do valor "durante
       walk" ou snapshot final?
     - Eixo 2: sub-store equivalente populated em
       produção?

9. Confirmar consumer downstream de
   `state.local_figure_counters`:
   - `grep -rn "local_figure_counters" 01_core/src/`.
   - Pode ser apenas walk-internal (sem consumer
     downstream) — caso em que migração é trivial.
   - Ou pode ter consumers que exigem sub-store.

10. **Cláusula gate substancial — cadeia E2-E3
    interacção com Labelled (P195D)**:
    - Walk arm Labelled (`introspect.rs:Content::Labelled`):
      como lê `state.figure_numbers`?
    - `compute_labelled` (P195D) tem match arm
      `Content::Figure { ... }` per P195A §11.6 —
      lê durante walk via state.
    - Após P197: walk arm Figure muta legacy +
      emite Tag. Labelled lê legacy (continua a
      funcionar). Cadeia preservada.
    - **Risco**: se P197 remover mutação legacy
      (não preservar write paralelo), Labelled
      quebra. **Decisão obrigatória**: preservar
      write paralelo per padrão P195D/P196B.

11. Confirmar tests existentes:
    - Sentinela E3 P189B (per `.D` ponto 3 do
      P189B).
    - Tests P184B/P168 que cobrem figure
      numbering.
    - Identificar quais devem manter-se inalterados
      ou ser adaptados.

12. Aplicar regra dos 2 eixos a Figure:
    - **Eixo 1**: snapshot final (consumer Layouter
      lê após walk completo).
    - **Eixo 2**: sub-store `figure_numbers` populated?
      Confirmar via `.A.6`.

Output: tabela com item + estado verificado.

**Critério de saída**:
- Walk arm Figure localizado com 2 mutações exactas.
- Variant `ElementPayload::Figure` estado actual
  confirmado.
- Sub-stores Figure existentes inventariados.
- Consumers downstream identificados.
- Cadeia E2-E3 confirmada empiricamente.

### Sub-passo 197A.B — Decisão cláusula 1 (forma do payload)

Conforme `.A.5`:

**Cenário α — variant `ElementPayload::Figure` já
existe e cobre semântica**:
- Reuso directo. Sem mudança ao enum.
- Magnitude reduzida.

**Cenário β — variant existe mas falta campo**:
- Expandir variant adicionando campo
  (ex.: `local_counter`).
- Tests existentes podem regridir (variant ABI
  changed).

**Cenário γ — variant não existe**:
- Adicionar variant nova.
- Replica P195B (P195 introduziu Labelled).
- Magnitude S extra para variant + L0 + tests.

Sugestão preliminar: cenário α esperado per P184B.
Auditor confirma empiricamente.

Output: cenário identificado + decisão fixada.

### Sub-passo 197A.C — Decisão cláusula 2 (helper)

Análogo a `compute_labelled` (P195D) e
`compute_heading_auto_toc` (P196B):

```
fn compute_figure(
    state:      &CounterStateLegacy,
    kind:       &Option<String>,
    numbering:  &Option<...>,
    is_counted: bool,
) -> Option<usize> {
    if !is_counted { return None; }
    let key = kind.clone().unwrap_or_else(|| "figure".into());
    let n = state.figure_numbers
        .entry(key)
        .or_default()
        .len() + 1;
    Some(n)
}
```

Forma exacta replica lógica actual. Auditor confirma
em `.A.1`.

**Caso edge**: se Figure não é counted (numbering
None), retorna `None`. Sem Tag emitida (similar a
P196B numbering inactivo).

Output: esquema do helper fixado.

### Sub-passo 197A.D — Decisão cláusula 3 (`local_figure_counters`)

Per `.A.7` + `.A.9`:
- Se `local_figure_counters` tem consumer downstream:
  precisa de sub-store dedicado em P197.
- Se é apenas walk-internal (sem consumer): mutação
  legacy preservada como write paralelo; sem necessidade
  de sub-store novo.

Sugestão preliminar: walk-internal (simplesmente
auxiliar para gerar `n` de Figure). Mutação legacy
preserva-se sem necessidade de sub-store novo.

Decisão final em `.A`.

Output: decisão fixada após confirmação empírica.

### Sub-passo 197A.E — Decisão cláusula 4 (cadeia E2-E3)

Per `.A.10`:
- Walk arm Labelled (P195D) lê
  `state.figure_numbers` via `compute_labelled`.
- P197 deve **preservar mutação legacy** —
  write paralelo durante janela compat M5.
- Sem mudança a walk arm Labelled — continua a
  funcionar via legacy.
- Após M6 cleanup (P190/P200): se Labelled migrar
  para ler do sub-store em vez de legacy, cadeia
  fecha completamente.

**Decisão obrigatória**: preservar write paralelo
mutação legacy. Não-negociável.

Output: decisão fixada.

### Sub-passo 197A.F — Decisão cláusula 5 (Locator handling)

Variante P196B aplicável (Figure locatable):
- `emitted_loc` directo do walk top.
- Tag pós-recursão reusa essa Location.
- Sem snapshot+find_map.

Output: decisão fixada — variante P196B.

### Sub-passo 197A.G — Decisão cláusula 6 (mutação legacy preservada)

Replica P195D/P196B pattern. Walk arm muta legacy +
emite Tag (write paralelo). Cleanup orgânico em M6.

Output: decisão fixada.

### Sub-passo 197A.H — Decisão cláusula 7 (critério de fecho)

P197 fecha quando:
- Walk arm Figure emite Tag pós-recursão.
- `from_tags` arm processa Tag → popula sub-store
  apropriado.
- E3 fecha **estruturalmente** completa (vs E2 que
  ficou com residuo).
- Tests E2E confirmam paridade observable + activação
  Introspector path em produção.
- Consumer downstream começa a receber dados via
  Introspector path.

Output: critério literal verificável.

### Sub-passo 197A.I — Validação do plano de sub-passos

| Sub-passo | Escopo | Magnitude |
|-----------|--------|-----------|
| `.B` | Walk arm Figure + helper `compute_figure` + tests E2E + L0 + (variant nova se cenário γ) | M |
| `.C` | Relatório consolidado P197 + actualização DEBT M5-residual | S |

Total agregado: ~80 LOC walk arm + ~40 LOC helper +
~150 LOC tests + edits L0 + relatório consolidado ≈
**M agregado**.

Output: tabela final.

### Sub-passo 197A.J — ADR

Avaliar:

- Pattern ADR-0069 reusado (cenário α/β).
- Sem decisão arquitectural nova esperada.

Conclusão esperada: **não cria ADR**.

**Excepção**: se cenário γ (variant nova) e decisão
arquitectural não-trivial sobre semântica Figure
emergir, ADR PROPOSTO. Improvável.

### Sub-passo 197A.K — DEBT

P197 fecha **E3 estruturalmente completa** (não residuo
como E2).

DEBT M5-residual após P197B+:
- Antes: 4 excepções activas + 1 residuo (E1,
  E2-residuo, E3, E5, E6); 2 pré-requisitos restantes.
- Após: **3 excepções activas + 1 residuo** (E1,
  E2-residuo, E5, E6); 2 pré-requisitos restantes.

**Cenário B continua** (sem DEBT formal aberto).

Output: estado actualizado.

### Sub-passo 197A.L — Outputs

Produzir 3 ficheiros (padrão P181A–P196A):

1. **`00_nucleo/diagnosticos/diagnostico-walk-figure-passo-197a.md`**
   — diagnóstico com 8 secções:
   - §1 Validação estado actual.
   - §2 Decisões cláusula 1–7 (formato O1–O5).
   - §3 Plano de sub-passos sem condicionais.
   - §4 Magnitude consolidada.
   - §5 ADR avaliação.
   - §6 DEBT avaliação.
   - §7 Cadeia E2-E3 (interacção com Labelled
     P195D) — análise empírica.
   - §8 Próximo sub-passo (P197B com escopo concreto).

2. **`00_nucleo/materialization/typst-passo-197a-relatorio.md`**
   — relatório com 14 secções (padrão P181A/etc.).

3. **Sem ADR e sem DEBT formal esperados**.

---

## Restrições

- **Zero código tocado** em qualquer ficheiro fora de
  `00_nucleo/`.
- **Zero testes** modificados.
- **Não criar reservas** de identificadores.
- **Não modificar walk** — P197B+.
- **Não tocar `from_tags`** — P197B+.
- **Não modificar trait `Introspector`** — P185B fechou.
- **Não modificar `TagIntrospector`** — P193B fechou.
- **Não modificar consumer C4** — P194B fechou.
- **Não migrar walks SetHeadingNumbering +
  CounterUpdate** — P198.
- **Não inflar linguagem**: sem "patamar", "limiar",
  "consolidação", "deriva", "subpadrão", "cumulativo",
  "cross-domínio", "paridade observable" como bandeira
  retórica.
- **Aplicar regra dos 2 eixos** a auditoria empírica.
- **Reaproveitar pattern ADR-0069 + variante P196B**
  (locatable). Sem decisão arquitectural nova esperada.
- **Sem cláusulas condicionais nos sub-passos `.B`+ do
  plano**.

---

## Critério de conclusão

- Diagnóstico em
  `00_nucleo/diagnosticos/diagnostico-walk-figure-passo-197a.md`
  com 8 secções produzido.
- Relatório em
  `00_nucleo/materialization/typst-passo-197a-relatorio.md`
  com 14 secções produzido.
- 7 cláusulas fechadas com decisão literal.
- Plano de sub-passos sem condicionais (2 sub-passos B
  + C).
- Magnitude consolidada confirmada (M agregado).
- Critério de fecho P197 fixado.
- ADR avaliada (esperado: não criada).
- DEBT M5-residual estado registado.
- Cadeia E2-E3 analisada empiricamente.
- Variant `ElementPayload::Figure` estado actual
  identificado (cenário α/β/γ).
- Regra dos 2 eixos aplicada empiricamente.
- Pattern ADR-0069 + variante P196B reaproveitados.
- Nenhum ficheiro de cristalino tocado.
- Tests workspace 1.843 inalterados.
- `crystalline-lint .` zero violations.

P197A é instrumento. Migração concreta de walk arm
Figure começa em P197B.
