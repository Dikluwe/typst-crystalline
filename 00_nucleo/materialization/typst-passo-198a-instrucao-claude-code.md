# Passo 198A — Instrução Claude Code

## Contexto mínimo

Typst Cristalino é re-implementação atómica do projecto
`typst/typst` em Rust com arquitectura camadas L0–L4.
Vanilla original está em quarentena em `lab/typst-original/`.
Cristalino vive em `01_core/`, `02_shell/`, `03_infra/`,
`04_wiring/`. ADRs em `00_nucleo/adr/`.

**Snapshot de partida** (a confirmar empiricamente em
sub-passo .A):

- Tests workspace 1.848 verdes; zero violations.
- M9 ✅ 11/11.
- M4-residual fechado funcionalmente (P188B).
- M5 incremental:
  - P189B: Outline migrado + 6 excepções declaradas.
  - P193B: sub-store `ResolvedLabelStore` aberto.
  - P194B: consumer C4 migrado.
  - P195B-E: walk arm Labelled migrado (E4
    estruturalmente fechada). ADR-0069 ACEITE.
  - P196B-C: walk arm Heading auto-toc migrado (E2 →
    E2-residuo).
  - P197B-C: walk arm Figure declarado fechado
    estruturalmente via cenário α (E3 fechada).
- DEBT M5-residual: 2 pré-requisitos restantes.
- Trait `Introspector`: 19 métodos.
- `TagIntrospector`: 8 sub-stores.
- `ElementPayload`: 11 variants.
- `ElementKind`: 9.
- 3 excepções activas + 1 residuo: E1, E2-residuo,
  E5, E6.

P198 é **passo 6 da sequência §9 P189**: migrar walks
**SetHeadingNumbering** (E5) + **CounterUpdate** (E6).
**Último passo da sequência §9** antes de M5 universal
fechar (excepto pré-requisitos paralelos —
`SetEquationNumbering` para E1; sub-store
`headings_for_toc` para E2-residuo).

**Pattern ADR-0069 com 3 variantes operacionais
consolidadas** (P197 §4 A7):

| Variante | Aplicação | Exigência |
|---|---|---|
| P195D | Target não-locatable | Snapshot+find_map |
| P196B | Content locatable | `emitted_loc` directo |
| P197B | Cenário α | Caminho Introspector já activo |

**Particularidade de P198**: 2 arms a migrar em série,
**potencialmente em variantes diferentes**. Per P197 §9
análise preliminar:

- **SetHeadingNumbering** (E5): provável **cenário α**
  (paralelo a Figure P197B). É locatable
  (`extract_payload` retorna `Some(StateUpdate)` per
  P182C); `from_tags` arm StateUpdate popula
  StateRegistry. Caminho Introspector pode já estar
  activo.

- **CounterUpdate** (E6): **incerto**. Per P189A
  inventário, não é locatable atualmente. Pode exigir:
  - Promoção a locatable (variante P196B) — refactor
    maior.
  - Cenário α se já houver mecanismo equivalente via
    StateUpdate Tag.
  - Aplicação concreta P195D-style (não-locatable) com
    snapshot+find_map.

P198 fecha **E5** completa + **E6** completa quando
ambos arms fecharem. Ordem dentro de P198 é decisão de
P198A.

Material de partida verificado:

- `00_nucleo/materialization/typst-passo-197-relatorio-consolidado.md`
  §9 — análise preliminar: SetHeadingNumbering candidato
  a cenário α; CounterUpdate incerto.
- `00_nucleo/adr/typst-adr-0069-post-recursion-tag-emission.md`
  — ACEITE.
- `00_nucleo/materialization/typst-passo-182-relatorio-consolidado.md`
  — P182C SetHeadingNumbering via StateUpdate;
  template para análise.
- `00_nucleo/materialization/typst-passo-189-relatorio-consolidado.md`
  §5 E5 + E6 — descrições das excepções (per P189B
  cadeia chained com E2; antes de P195+P196).

P198A é o passo de diagnóstico. Magnitude esperada **S**
(diagnóstico). P198B+ depende de cláusula 1 e 2
(variante por arm).

---

## Postura do auditor / executor

P198A é passo **L0-puro / diagnóstico-primeiro**, padrão
estabelecido em 19 aplicações.

- **Zero código tocado** em camadas cristalinas.
- **Zero testes** modificados.
- **Pode criar** ADR `PROPOSTO` — improvável (3
  variantes ADR-0069 cobrem).
- **Pode abrir DEBT** se trabalho identificado for
  adiado.
- **Não modifica** walk, `from_tags`, sub-stores,
  consumer — P198B+.

**Magnitude diagnóstico**: S. Decisões expandidas
porque há 2 arms a auditar empiricamente.

**Regra dos 2 eixos aplicável** a cada arm
separadamente — eixo 1 + eixo 2 confirmados em P189A
para ambos.

**3 variantes operacionais ADR-0069 disponíveis** —
auditor decide empiricamente qual aplica a cada arm.

---

## Escopo

**Primário**: desenhar migração de **walks
SetHeadingNumbering** (E5) e **CounterUpdate** (E6).
Cada um pode estar em variante diferente:
- Variante P195D (não-locatable + snapshot+find_map).
- Variante P196B (locatable + `emitted_loc` directo).
- Variante P197B (cenário α — caminho Introspector já
  activo; refactor estilístico).

**Confirmação**: validar inventário factual para cada
arm — forma exacta, mutações, locatable/não, sub-store
correspondente, consumers downstream, interacção com
outros arms.

**Decisões a tomar** — 9 cláusulas (mais que passos
anteriores porque há 2 arms):

1. **Variante para SetHeadingNumbering (E5)** —
   α/β/γ ou nova:
   - Análise preliminar P197 §9: provável cenário α
     (paralelo a Figure P197B).
   - Confirmar empiricamente em `.A`.

2. **Variante para CounterUpdate (E6)**:
   - Estado actual incerto.
   - Confirmar `is_locatable`, `extract_payload`,
     `from_tags` arm em `.A`.

3. **Helper(s) privado(s)** — análogos a `compute_*`
   da família ADR-0069:
   - `compute_set_heading_numbering` (se aplicável).
   - `compute_counter_update` (se aplicável).
   - Ou nenhum (cenário α puro pode dispensar).

4. **Ordem de execução dos 2 arms em P198B+**:
   - Sub-passo único agregado se ambos cenário α.
   - Sub-passos separados se variantes diferentes.

5. **Cadeia E5/E6 com outros arms** — análise das
   dependências:
   - Walk arm Heading lê
     `state.is_numbering_active("heading")` durante
     walk para auto-toc (per P195A §11.5 + P196A
     §2.5). Mutação legacy
     `state.numbering_active.insert("heading", ...)`
     pelo arm SetHeadingNumbering precisa de continuar
     activa — paralelo legacy preservado.
   - `compute_labelled` P195D pode ler counter
     mutados via CounterUpdate — verificar.

6. **Interacção com `Content::SetEquationNumbering`
   (E1)**:
   - E1 é independente de P198.
   - Mas se CounterUpdate cobrir Equation counter,
     pode parcialmente desbloquear E1.
   - Verificar empiricamente.

7. **Mutação legacy preservada** — replica padrão
   P195D/P196B/P197B (write paralelo M5).

8. **Critério de fecho de P198** — E5 fecha + E6 fecha
   ambos estruturalmente. M5 universal **NÃO fecha**
   ainda (E1 + E2-residuo + lacuna #3 +
   `SetEquationNumbering` ainda).

9. **Granularidade** — 1 sub-passo agregado vs
   sub-passos por arm:
   - Se ambos cenário α: 1 sub-passo (similar a P197B
     mas com 2 helpers).
   - Se variantes diferentes: sub-passos separados.

**Fora de escopo**:

- `SetEquationNumbering` materialização (passo
  independente paralelo — fecha E1).
- Sub-store `headings_for_toc` (lacuna #3 — passo
  dedicado paralelo — fecha E2-residuo).
- Eliminação `CounterStateLegacy` (P190/P200).

---

## Critérios objectivos

### O1 — Inputs verificáveis

`grep -rn "Content::SetHeadingNumbering" 01_core/src/`.
`grep -rn "Content::CounterUpdate" 01_core/src/`. Para
cada arm, confirmar mutações exactas, locatable
status, payload existente, consumer downstream.

### O2 — Alternativas

Para cada arm, 3 variantes possíveis (α/β/γ). Total
combinações: 3 × 3 = 9 estados teóricos. Auditoria
empírica reduz a 1 estado actual por arm.

### O3 — Critério de escolha

Variante α (cenário α) preferível se aplicável (menor
custo). Variante β (locatable) próxima se locatable
disponível. Variante γ (não-locatable) último recurso.

### O4 — Magnitude

P198 implementação depende empiricamente:
- Ambos cenário α: **S/M agregado** (similar a P197).
- 1 cenário α + 1 P196B: **M agregado**.
- Ambos P195D ou P196B: **M-L agregado** (similar ou
  maior que P195).

### O5 — Reversibilidade

Reversível por construção (write paralelo legacy
preservado).

---

## Critérios qualitativos

### Q1 — Consistência com padrão estabelecido

Cada arm replica variante apropriada. Pattern ADR-0069
+ 3 variantes consolidadas reduzem incerteza.

### Q2 — Honestidade de magnitude

P198A diagnóstico é S. P198B+ implementação:
- Pode variar entre S/M e M-L conforme variantes.
- Auditor declara magnitude empírica em `.A`.

### Q3 — Cobertura sem regressão

Mutação legacy preservada per padrão. Cadeia E5/E6 com
outros arms (auto-toc Heading) preservada.

### Q4 — Excepções fecham completamente

E5 e E6 fecham **estruturalmente** após P198 (não
residuais). Diferente de E2 (que ficou com residuo).
Razão: ambos têm sub-store destino claro
(StateRegistry P182 ou CounterRegistry P184B).

### Q5 — Granularidade

Conforme cláusula 9. 1 sub-passo agregado a 4
sub-passos separados.

---

## Sub-passos de P198A

### Sub-passo 198A.A — Validação do estado actual

Auditor confirma empiricamente:

#### Para `Content::SetHeadingNumbering` (E5)

1. Localizar walk arm:
   - `01_core/src/rules/introspect.rs` — match arm
     `Content::SetHeadingNumbering`.
   - Identificar mutações legacy.

2. Confirmar variant `Content::SetHeadingNumbering`:
   - `01_core/src/entities/content.rs`.
   - Campos: `active: bool` ou similar.

3. Confirmar `is_locatable(SetHeadingNumbering)`:
   - Per P182C aprendizado: arm StateUpdate via
     `extract_payload` per P182C.
   - Verificar empiricamente.

4. Confirmar `extract_payload(SetHeadingNumbering)`:
   - Esperado: retorna `Some(ElementPayload::StateUpdate
     { key, update })` (P182C pattern).

5. Confirmar `from_tags` arm StateUpdate:
   - Popula `intr.state` (StateRegistry P182).
   - Replica P171 pattern.

6. Confirmar mutação legacy
   `state.numbering_active.insert(...)` paralela:
   - Walk arm muta legacy E também emite Tag (write
     paralelo).
   - Per P195A §11.5: mutação legacy preservada.

7. Confirmar consumer downstream:
   - Walk arm Heading lê
     `state.is_numbering_active("heading")` em
     `compute_heading_auto_toc` (P196B helper).
   - Layouter Heading rendering provavelmente também.

8. Aplicar regra dos 2 eixos:
   - Eixo 1: snapshot final (consumer Heading lê após
     SetHeadingNumbering processada).
   - Eixo 2: `intr.state["numbering_active:heading"]`
     populated em produção via P182C.

9. Decisão preliminar variante:
   - Se cenário α: caminho Introspector já activo,
     refactor estilístico opcional.
   - Se P196B: emit Tag pós-recursão (mas é leaf
     content, não há recursão).
   - Provável: **cenário α** (similar a Figure P197B
     — caminho activo desde P182C).

#### Para `Content::CounterUpdate` (E6)

10. Localizar walk arm:
    - `01_core/src/rules/introspect.rs` — match arm
      `Content::CounterUpdate`.
    - Identificar mutações legacy (per P189B §5 E6: 3
      caminhos `step_hierarchical`, `step_flat`,
      `update_flat`).

11. Confirmar variant `Content::CounterUpdate`:
    - Campos: `key`, `action` ou similar.

12. Confirmar `is_locatable(CounterUpdate)`:
    - **Crítico**. Se locatable: variante P196B.
    - Se não: variante P195D ou cenário α.

13. Confirmar `extract_payload(CounterUpdate)`:
    - Pode existir ou não.
    - Se existe: arm emite Tag via mecanismo padrão.

14. Confirmar `from_tags` arm correspondente:
    - Pode haver `ElementPayload::CounterUpdate` ou
      reuso de StateUpdate.

15. Confirmar consumer downstream:
    - Walk arm Heading auto-toc, Equation, Figure
      podem ler counter mutados.
    - `compute_labelled` P195D Equation arm lê
      `state.get_flat("equation")` — pode ser afectado
      por CounterUpdate(equation).

16. Aplicar regra dos 2 eixos:
    - Eixo 1: snapshot final?
    - Eixo 2: sub-store equivalente populated?

17. Decisão preliminar variante:
    - **Incerta**. Auditor decide empiricamente após
      `.A.10–.A.16`.

#### Confirmações cross-cutting

18. Confirmar tests existentes que cobrem E5 e E6:
    - Sentinelas P189B (per `.D` ponto 3 do P189B).

19. Confirmar L0 `rules/introspect.md`:
    - Identificar onde adicionar entradas para arms
      migrados.

Output: tabela com item + estado verificado por arm.

**Critério de saída**:
- Walks SetHeadingNumbering + CounterUpdate localizados.
- Variant + is_locatable + extract_payload + from_tags
  estado confirmado para cada.
- Cadeia com outros arms identificada.
- Variante operacional preliminar identificada para
  cada arm.

### Sub-passo 198A.B — Decisão cláusula 1 (SetHeadingNumbering variante)

Conforme `.A.1–.A.9`:

**Sugestão preliminar**: cenário α (paralelo a Figure
P197B). Caminho Introspector activo desde P182C
(StateUpdate + StateRegistry).

Acções esperadas em P198B:
- Helper estilístico opcional `compute_set_heading_numbering`
  (consistência com família ADR-0069).
- Ou refactor mínimo sem helper se já é trivial.
- Declaração formal em L0 que E5 fecha estruturalmente.
- 5 tests sentinela cenário α.
- Mutação legacy preservada.

Output: variante fixada per `.A`.

### Sub-passo 198A.C — Decisão cláusula 2 (CounterUpdate variante)

Conforme `.A.10–.A.17`:

3 cenários possíveis:

**Cenário α — caminho Introspector já activo**: análogo
a SetHeadingNumbering. Se `extract_payload` já existe
e `from_tags` popula CounterRegistry/StateRegistry,
refactor estilístico.

**Cenário β — promover a locatable**: variante P196B.
Adicionar arm `is_locatable=true`, `extract_payload`
arm, `from_tags` arm. Maior trabalho. Tag pós-recursão
via `emitted_loc`.

**Cenário γ — pattern P195D**: variante non-locatable
com snapshot+find_map. Se nem cenário α nem
promovido, aplicar pattern post-recursion via
snapshot.

Output: variante fixada per `.A`.

### Sub-passo 198A.D — Decisão cláusula 3 (helpers)

Conforme cláusulas 1+2:

- Se ambos cenário α: 0-2 helpers estilísticos
  opcionais.
- Se P196B/P195D para 1+: 1-2 helpers funcionais
  obrigatórios.

Output: helpers fixados.

### Sub-passo 198A.E — Decisão cláusula 4 (ordem dos arms)

**Opção α** — Sub-passo agregado único P198B (ambos
cenário α): 1 sub-passo + relatório consolidado.

**Opção β** — Sub-passos separados:
- P198B: SetHeadingNumbering.
- P198C: CounterUpdate.
- P198D: relatório consolidado.

Critério: Opção α se variantes idênticas + magnitude S.
Opção β se variantes diferentes ou magnitude
divergente.

Output: ordem fixada per `.A.B/.C/.D`.

### Sub-passo 198A.F — Decisão cláusula 5 (cadeia E5/E6)

Per `.A.7` e `.A.15`:

- E5 ↔ Heading auto-toc (P196B): mutação legacy
  `numbering_active` lida por `compute_heading_auto_toc`.
- E6 ↔ Heading auto-toc / Equation / Figure: counter
  mutations lidos por `compute_*` helpers.

**Decisão obrigatória**: preservar mutações legacy
durante janela compat M5. Cleanup orgânico em M6
quando consumers (helpers `compute_*`) migrarem para
sub-stores.

Output: decisão fixada — mutação legacy paralela
obrigatória.

### Sub-passo 198A.G — Decisão cláusula 6 (interacção com E1)

Per `.A.6`:

E1 (Equation) bloqueada por
`Content::SetEquationNumbering` ausente. Independente
de P198.

**Excepção**: se `Content::CounterUpdate` cobrir
counter Equation (`step_flat("equation")`), pode haver
interacção. Verificar empiricamente em `.A.10`.

Output: decisão fixada — sem trabalho em E1 dentro de
P198.

### Sub-passo 198A.H — Decisão cláusula 7 (mutação legacy)

Replica P195D/P196B/P197B. Write paralelo M5
preservado. Cleanup orgânico em M6.

Output: decisão fixada.

### Sub-passo 198A.I — Decisão cláusula 8 (critério de fecho)

P198 fecha quando:
- Walk arm SetHeadingNumbering migrado/declarado
  (E5 fecha estruturalmente).
- Walk arm CounterUpdate migrado/declarado (E6 fecha
  estruturalmente).
- Tests E2E confirmam paridade observable + activação
  Introspector path em produção.
- Mutação legacy preservada (write paralelo M5).

M5 universal **NÃO fecha** ainda. Pré-requisitos
restantes:
- E1 ↔ `SetEquationNumbering` materialização (passo
  paralelo).
- E2-residuo ↔ sub-store `intr.headings_for_toc`
  (passo paralelo).

Output: critério literal verificável.

### Sub-passo 198A.J — Decisão cláusula 9 (plano P198B+)

Conforme cláusula 4:

**Se Opção α**:

| Sub-passo | Escopo | Magnitude |
|---|---|---|
| `.B` | Walk arms ambos + helper(s) + L0 + tests E2E | M |
| `.C` | Relatório consolidado P198 + DEBT actualizada | S |

**Se Opção β**:

| Sub-passo | Escopo | Magnitude |
|---|---|---|
| `.B` | Walk arm SetHeadingNumbering + helper + L0 + tests | S/M |
| `.C` | Walk arm CounterUpdate + helper + L0 + tests | S/M ou M |
| `.D` | Relatório consolidado P198 + DEBT actualizada | S |

Output: tabela final.

### Sub-passo 198A.K — ADR

Avaliar:

- 3 variantes ADR-0069 cobrem.
- Sem decisão arquitectural nova esperada.

Conclusão: **não cria ADR**.

**Excepção**: se CounterUpdate exigir variante nova
não documentada nas 3 actuais (improvável), ADR
PROPOSTO.

### Sub-passo 198A.L — DEBT

P198 fecha **E5 e E6 estruturalmente**.

DEBT M5-residual após P198B+:
- Antes: 3 excepções activas + 1 residuo (E1,
  E2-residuo, E5, E6).
- Após: **1 excepção activa + 1 residuo** (E1,
  E2-residuo).
- 2 pré-requisitos restantes (inalterado).

**M5 universal estado**:
- 4 dos 6 arms declarados M5 fechados estruturalmente
  (P189B + P195D + P196B + P197B + **P198**).
- E1 + E2-residuo restantes — pré-requisitos paralelos.

**Cenário B continua** (sem DEBT formal aberto).

Output: estado actualizado.

### Sub-passo 198A.M — Outputs

Produzir 3 ficheiros (padrão P181A–P197A):

1. **`00_nucleo/diagnosticos/diagnostico-walks-set-counter-passo-198a.md`**
   — diagnóstico com 8 secções:
   - §1 Validação estado actual (por arm).
   - §2 Decisões cláusula 1–9.
   - §3 Plano de sub-passos sem condicionais.
   - §4 Magnitude consolidada.
   - §5 ADR avaliação.
   - §6 DEBT avaliação.
   - §7 Cadeia E5/E6 com outros arms — análise
     empírica.
   - §8 Próximo sub-passo (P198B com escopo concreto).

2. **`00_nucleo/materialization/typst-passo-198a-relatorio.md`**
   — relatório com 14 secções (padrão P181A/etc.).

3. **Sem ADR e sem DEBT formal esperados**.

---

## Restrições

- **Zero código tocado** em qualquer ficheiro fora de
  `00_nucleo/`.
- **Zero testes** modificados.
- **Não criar reservas** de identificadores.
- **Não modificar walk** — P198B+.
- **Não tocar `from_tags`** — P198B+.
- **Não modificar trait `Introspector`** — P185B fechou.
- **Não modificar `TagIntrospector`** — P193B fechou.
- **Não modificar consumer C3 ou C4** — P184D/P194B
  fecharam.
- **Não materializar `SetEquationNumbering`** — passo
  paralelo independente.
- **Não abrir sub-store `headings_for_toc`** — passo
  dedicado paralelo.
- **Não inflar linguagem**: sem "patamar", "limiar",
  "consolidação", "deriva", "subpadrão", "cumulativo",
  "cross-domínio", "paridade observable" como bandeira
  retórica.
- **Aplicar regra dos 2 eixos** a cada arm
  empiricamente.
- **Reaproveitar pattern ADR-0069 + 3 variantes
  operacionais** sem decisão arquitectural nova.
- **Sem cláusulas condicionais nos sub-passos `.B`+ do
  plano**.

---

## Critério de conclusão

- Diagnóstico em
  `00_nucleo/diagnosticos/diagnostico-walks-set-counter-passo-198a.md`
  com 8 secções produzido.
- Relatório em
  `00_nucleo/materialization/typst-passo-198a-relatorio.md`
  com 14 secções produzido.
- 9 cláusulas fechadas com decisão literal.
- Plano de sub-passos sem condicionais (2 ou 3
  sub-passos conforme cláusula 4).
- Magnitude consolidada confirmada empiricamente.
- Critério de fecho P198 fixado (E5+E6 estruturalmente
  fechadas).
- ADR avaliada (esperado: não criada).
- DEBT M5-residual estado registado (1 excepção activa
  + 1 residuo após P198B+).
- Cadeia E5/E6 com outros arms analisada empiricamente.
- Variantes operacionais identificadas para cada arm
  (α/β/γ).
- Regra dos 2 eixos aplicada empiricamente para cada
  arm.
- Pattern ADR-0069 reaproveitado.
- Nenhum ficheiro de cristalino tocado.
- Tests workspace 1.848 inalterados.
- `crystalline-lint .` zero violations.

P198A é instrumento. Migração concreta de walks
SetHeadingNumbering + CounterUpdate começa em P198B+.

**Após P198 série fechar**: M5 universal está a 1
pré-requisito de fechar estruturalmente
(`SetEquationNumbering` materializado fecha E1; passo
paralelo `headings_for_toc` fecha E2-residuo). M5
universal completo desbloqueia P190/P200 (M6 —
eliminação `CounterStateLegacy`).
