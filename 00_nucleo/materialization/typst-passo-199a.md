# Passo 199A — Instrução Claude Code

## Contexto mínimo

Typst Cristalino é re-implementação atómica do projecto
`typst/typst` em Rust com arquitectura camadas L0–L4.
Vanilla original está em quarentena em `lab/typst-original/`.
Cristalino vive em `01_core/`, `02_shell/`, `03_infra/`,
`04_wiring/`. ADRs em `00_nucleo/adr/`.

**Snapshot de partida** (a confirmar empiricamente em
sub-passo .A):

- Tests workspace 1.859 verdes; zero violations.
- M9 ✅ 11/11.
- M4-residual fechado funcionalmente (P188B).
- M5 incremental:
  - P189B: Outline migrado + 6 excepções declaradas.
  - P193B: sub-store `ResolvedLabelStore`.
  - P194B: consumer C4 migrado.
  - P195B-E: walk arm Labelled (E4 fechada).
  - P196B-C: walk arm Heading auto-toc (E2 → E2-residuo).
  - P197B-C: walk arm Figure (E3 fechada — cenário α).
  - P198B-D: walks SetHeadingNumbering + CounterUpdate
    (E5 + E6 fechadas).
  - **Sequência §9 P189 cumprida na totalidade**.
- DEBT M5-residual: 2 pré-requisitos paralelos restantes.
- Trait `Introspector`: 19 métodos.
- `TagIntrospector`: 8 sub-stores.
- `ElementPayload`: 12 variants (após P198C).
- `ElementKind`: 10 (após P198C).
- 1 excepção activa + 1 residuo: **E1**, E2-residuo.
- Pattern ADR-0069 com 4 variantes operacionais
  consolidadas (P195D, P196B, cenário α, cenário
  β-promote).

P199 é **passo paralelo fora série §9 P189** —
materializa `Content::SetEquationNumbering` para fechar
**E1** (Reserva 1 desde P189B).

**Pré-requisito M5 universal**: P199 + passo paralelo
sub-store `headings_for_toc` (fecha E2-residuo). Após
ambos fecharem, M5 universal completo desbloqueia M6
(P190A reescrita do zero — eliminação `CounterStateLegacy`).

**Material de partida** verificado:

- `00_nucleo/materialization/typst-passo-198-relatorio-consolidado.md`
  §9 — `SetEquationNumbering` identificada como
  pré-requisito paralelo; magnitude M esperada
  (analogia directa com SetHeadingNumbering P182C).
- `00_nucleo/materialization/typst-passo-182-relatorio-consolidado.md`
  — P182C SetHeadingNumbering via StateUpdate; **template
  primário para análise de P199**.
- `00_nucleo/materialization/typst-passo-198b-relatorio.md`
  — P198B declaração formal cenário α SetHeadingNumbering.
- `00_nucleo/materialization/typst-passo-189-relatorio-consolidado.md`
  §5 E1 — descrição da Reserva 1.
- `00_nucleo/adr/typst-adr-0069-post-recursion-tag-emission.md`
  — ACEITE.

P199A é o passo de diagnóstico. Magnitude esperada **S**
(diagnóstico). Implementação P199B+ depende criticamente
de cláusula 1 (forma da materialização) e cláusula 2
(scope do parser).

---

## Postura do auditor / executor

P199A é passo **L0-puro / diagnóstico-primeiro**, padrão
estabelecido em 20 aplicações.

- **Zero código tocado** em camadas cristalinas.
- **Zero testes** modificados.
- **Pode criar** ADR `PROPOSTO` — improvável (analogia
  directa com SetHeadingNumbering P182C cobre).
- **Pode abrir DEBT** se trabalho identificado for
  adiado.
- **Não modifica** walk, `from_tags`, sub-stores,
  consumer — P199B+.

**Magnitude diagnóstico**: S. Decisões esperadas seguem
analogia directa com P182C.

**Regra dos 2 eixos aplicável** confirmada:
- Eixo 1: snapshot final (consumer Equation rendering
  lê após walk completo).
- Eixo 2: sub-store `intr.state` (StateRegistry) já
  existe — populated em produção via Tag::StateUpdate
  pattern.

**Distinção crítica face a P198B**:
- P198B: declaração formal cenário α porque variant
  `Content::SetHeadingNumbering` **já existia**;
  caminho Introspector já activo desde P182C; refactor
  estilístico mínimo.
- P199: variant `Content::SetEquationNumbering`
  **não existe ainda**. P199 **adiciona variant** —
  cenário diferente. Após adição, declarar fechada
  estruturalmente (cenário α por construção desde a
  materialização).

---

## Escopo

**Primário**: materializar `Content::SetEquationNumbering`
seguindo template P182C (SetHeadingNumbering).

**Confirmação**: validar inventário factual — variant
ausente, walk arm Equation lê
`state.is_numbering_active("equation")`, sub-store
StateRegistry pronto.

**Decisões a tomar** — 7 cláusulas:

1. **Forma da materialização** — variant nova
   `Content::SetEquationNumbering { active: bool }`:
   - **Opção α** (preferida): replica literal de
     `Content::SetHeadingNumbering` (P182C).
   - **Opção β**: alternativa com forma diferente
     (improvável — analogia directa esperada).

2. **Scope do parser**:
   - **Opção α**: P199 cobre apenas materialização
     interna (variant + walk arm + extract_payload).
     Construção do `Content::SetEquationNumbering` fica
     restrita a tests / programaticamente. Parser
     sintáctico (`#set equation(numbering: ...)`) fora
     de escopo.
   - **Opção β**: P199 cobre parser também. Magnitude
     L+. Improvável.

3. **`is_locatable` + `extract_payload` arms** — replica
   P182C:
   - `is_locatable(Content::SetEquationNumbering) =
     true`.
   - `extract_payload(Content::SetEquationNumbering)`
     retorna `Some(ElementPayload::StateUpdate { key:
     "numbering_active:equation", update:
     Set(Bool(active)) })`.

4. **Walk arm** — replica literal P182C:
   - `state.numbering_active.insert("equation".to_string(),
     *active)`.
   - Comentário inline indicando cenário α por
     construção (caminho Introspector activa
     imediatamente).

5. **Reuso `from_tags` arm StateUpdate** (P171) — sem
   modificação porque key canónica
   `numbering_active:equation` é processada pela arm
   genérica:
   - Confirmar empiricamente que arm StateUpdate
     não tem hardcoded keys (genérica para qualquer
     `numbering_active:*`).

6. **Cadeia E1 ↔ helpers** — mutação legacy preservada:
   - Walk arm Equation (per P186) lê
     `state.is_numbering_active("equation")` durante
     walk para gating.
   - `compute_labelled` Equation arm (P195D) lê
     `state.is_numbering_active("equation")` ou
     `state.get_flat("equation")`.
   - **Decisão obrigatória**: preservar mutação legacy
     `numbering_active.insert` durante janela compat
     M5.

7. **Critério de fecho de P199** — `Content::SetEquationNumbering`
   materializado; variant em enum; walk arm + arm
   `extract_payload` + `is_locatable` arm activos;
   `from_tags` arm StateUpdate processa em produção
   sem modificação; tests E2E confirmam paridade
   observable + activação Introspector path; **E1
   fecha estruturalmente**.

**Fora de escopo**:

- Parser sintáctico para `#set equation(numbering:
  ...)` (per cláusula 2 Opção α).
- Sub-store `headings_for_toc` — passo paralelo
  independente (fecha E2-residuo).
- Eliminação `CounterStateLegacy` (P190A — M6).
- Migração de consumers Equation além do que P186
  cobre.

---

## Critérios objectivos

### O1 — Inputs verificáveis

`grep -rn "SetHeadingNumbering" 01_core/src/` para
mapear template P182C. `grep -rn "Content::Equation\|is_numbering_active.*equation"
01_core/src/` para mapear consumers Equation que
precisam de `state.numbering_active["equation"]`.

### O2 — Alternativas

Cláusula 2 (scope parser) tem 2 opções; restantes
cláusulas têm analogia directa com P182C.

### O3 — Critério de escolha

Replicação literal de P182C reduz incerteza
arquitectural a zero.

### O4 — Magnitude

P199 implementação **M agregada**:
- 1 sub-passo único agregado: variant + 3 arms +
  comentário + L0 + tests E2E + relatório.
- Alternativa: 2 sub-passos (B materialização + C
  consolidado) per padrão P196B+P196C / P197B+P197C.

### O5 — Reversibilidade

Reversível por construção (write paralelo legacy
preservado). Desfazer P199 envolve remover variant +
arms + reverter tests.

---

## Critérios qualitativos

### Q1 — Consistência com padrão estabelecido

P199 replica P182C (SetHeadingNumbering). Sem invenção
arquitectural nova. Pattern ADR-0069 cenário α aplicável
por construção (caminho Introspector activa
imediatamente após materialização).

### Q2 — Honestidade de magnitude

P199A diagnóstico é S. Implementação:
- P199B: M (variant + arms + tests).
- P199C: S puro (relatório consolidado).

Total agregado: M.

### Q3 — Cobertura sem regressão

Output observable preservado:
- Variant nova é adição retrocompatível (sem campo
  removido).
- Walk arm + arms novos não afectam código existente.
- Mutação legacy `numbering_active.insert("equation")`
  paralela com Tag emit.
- Consumer Equation (P186) lê legacy durante walk;
  preservado.
- `compute_labelled` Equation arm (P195D) lê legacy;
  preservado.

### Q4 — E1 fecha estruturalmente

E1 fecha **estruturalmente completa** após P199
(análoga a E5 P198B):
- Variant existe.
- Walk arm muta legacy + emite Tag (write paralelo).
- Caminho Introspector activa via `from_tags` arm
  StateUpdate (P171).
- Consumers podem optar por ler via Introspector
  (substitution-with-fallback) — fora de escopo de
  P199.

Cleanup orgânico em M6 (P190A) quando consumers
Equation migrarem para Introspector path.

### Q5 — Granularidade

Conforme padrão P195/P196/P197/P198:

| Sub-passo | Escopo | Magnitude |
|---|---|---|
| `.B` | Variant + 3 arms + walk arm + comentário + tests E2E + L0 | M |
| `.C` | Relatório consolidado P199 + actualização DEBT | S |

---

## Sub-passos de P199A

### Sub-passo 199A.A — Validação do estado actual

Auditor confirma empiricamente:

1. Confirmar `Content::SetHeadingNumbering` em
   `01_core/src/entities/content.rs` (template):
   - Forma exacta da variant (campo `active: bool` ou
     similar).
   - Localizar todos os arms relacionados
     (`is_locatable`, `extract_payload`, walk arm
     `introspect.rs`).

2. Confirmar variant `Content::SetEquationNumbering`
   **não existe**:
   - `grep -rn "SetEquationNumbering" 01_core/src/`
     deve retornar zero (excepto possivelmente em
     comentários referindo Reserva 1).
   - Se existir parcial implementação histórica:
     identificar e decidir migração.

3. Confirmar arm `is_locatable` template P182C:
   - `01_core/src/rules/introspect/locatable.rs` — arm
     `Content::SetHeadingNumbering { .. } => true`.
   - Onde adicionar arm análogo.

4. Confirmar arm `extract_payload` template P182C:
   - Forma de retorno `Some(ElementPayload::StateUpdate
     { key, update })`.
   - Chave canónica `numbering_active:heading` em
     P182C.
   - Equivalente para P199: `numbering_active:equation`.

5. Confirmar walk arm SetHeadingNumbering em
   `introspect.rs`:
   - Mutação `state.numbering_active.insert("heading",
     *active)`.
   - Equivalente para P199:
     `state.numbering_active.insert("equation",
     *active)`.

6. Confirmar `from_tags` arm StateUpdate (P171):
   - Genérica para qualquer key.
   - Não tem hardcoded `"heading"`.
   - Processa `numbering_active:equation`
     transparentemente.

7. Confirmar consumer downstream Equation:
   - Walk arm Equation (per P186): localizar leitura
     `state.is_numbering_active("equation")` durante
     walk para gating.
   - `compute_labelled` Equation arm (P195D):
     localizar leitura state.
   - **Mutação legacy
     `numbering_active.insert("equation")`
     obrigatória** durante janela compat M5.

8. Confirmar tests existentes:
   - Sentinela E1 P189B (per `.D` ponto 3 do P189B).
   - Tests P171/P182C que cobrem StateUpdate flow.
   - Identificar quais devem manter-se inalterados.

9. Confirmar L0 alvos:
   - `entities/content.md` (variant nova).
   - `rules/introspect.md` (arms novos + walk arm).

10. Aplicar regra dos 2 eixos:
    - Eixo 1: snapshot final (consumer Equation lê
      após walk completo).
    - Eixo 2: `intr.state["numbering_active:equation"]`
      populated em produção via Tag::StateUpdate
      genérica.

11. Confirmar parser scope:
    - `grep -rn "set equation" 01_core/src/` para
      identificar se há parser sintáctico actual.
    - Se ausente: P199 só cobre materialização interna
      (cláusula 2 Opção α).
    - Se presente: cláusula 2 pode exigir Opção β.

Output: tabela com item + estado verificado.

**Critério de saída**:
- Variant `Content::SetEquationNumbering` confirmada
  ausente.
- Template P182C totalmente mapeado.
- Cadeia E1 com consumers Equation identificada.
- Mutação legacy obrigatória estabelecida.

### Sub-passo 199A.B — Decisão cláusula 1 (forma da materialização)

Conforme `.A.1`:

**Opção α** (preferida) — variant
`Content::SetEquationNumbering { active: bool }`
literal a `SetHeadingNumbering`.

Sugestão preliminar fixada: Opção α.

Output: forma fixada.

### Sub-passo 199A.C — Decisão cláusula 2 (scope do parser)

Conforme `.A.11`:

**Opção α** (preferida) — P199 cobre apenas
materialização interna. Variant disponível
programaticamente para tests + uso futuro. Parser
sintáctico fica para passo separado (provável M6+).

**Opção β** — P199 cobre parser também. Magnitude L+.
Sai do escopo M5.

Decisão preliminar: Opção α.

Output: scope fixado.

### Sub-passo 199A.D — Decisão cláusula 3 (`is_locatable` + `extract_payload`)

Replica P182C literal:

```
// is_locatable.rs
Content::SetEquationNumbering { .. } => true,

// extract_payload.rs
Content::SetEquationNumbering { active } => Some(
    ElementPayload::StateUpdate {
        key:    "numbering_active:equation".to_string(),
        update: StateUpdate::Set(Value::Bool(*active)),
    },
),
```

Forma exacta fica para Claude Code conforme convenção
do projecto e API real (per `.A.4`).

Output: arms fixados.

### Sub-passo 199A.E — Decisão cláusula 4 (walk arm)

Replica P182C literal com chave `equation`:

```
Content::SetEquationNumbering { active } => {
    // P199B — E1 fechada estruturalmente (cenário α
    // por construção).
    //
    // Caminho Introspector activado por construção
    // desde a materialização: extract_payload →
    // ElementPayload::StateUpdate sob chave
    // numbering_active:equation → from_tags arm
    // StateUpdate (P171) popula StateRegistry.
    //
    // Mutação legacy preservada como write paralelo
    // M5: walk arm Equation (P186) + compute_labelled
    // Equation arm (P195D) lêem
    // state.is_numbering_active("equation") durante
    // walk para gating + format. Cleanup orgânico em
    // M6.
    state.numbering_active.insert("equation".to_string(), *active);
}
```

Output: walk arm fixado.

### Sub-passo 199A.F — Decisão cláusula 5 (reuso `from_tags` arm)

Per `.A.6`:

Confirmação obrigatória — arm StateUpdate (P171) é
genérica. Sem modificação necessária.

**Cláusula gate substancial**: se arm StateUpdate tem
hardcoded `"heading"` (improvável mas possível),
P199 exige adição de caso `"equation"`.

Decisão preliminar: sem modificação esperada.

Output: decisão fixada — sem modificação a
`from_tags`.

### Sub-passo 199A.G — Decisão cláusula 6 (cadeia E1 ↔ helpers)

Per `.A.7`:

- Walk arm Equation (P186) e `compute_labelled` Equation
  arm (P195D) lêem `state.is_numbering_active("equation")`
  + `state.get_flat("equation")` durante walk.
- **Decisão obrigatória**: preservar mutação legacy
  `numbering_active.insert("equation")` per padrão
  P195D/P196B/P198B.

Output: decisão fixada — write paralelo preservado.

### Sub-passo 199A.H — Decisão cláusula 7 (critério de fecho)

P199 fecha quando:
- Variant `Content::SetEquationNumbering` adicionada.
- 3 arms novos: `is_locatable`, `extract_payload`,
  walk arm.
- `from_tags` arm StateUpdate processa em produção
  sem modificação.
- Tests E2E confirmam paridade observable + activação
  Introspector path.
- **E1 fecha estruturalmente completa** (cenário α por
  construção).
- Mutação legacy preservada.

M5 universal **NÃO fecha** ainda — sub-store
`headings_for_toc` (E2-residuo) ainda activo. Passo
paralelo restante.

Output: critério literal verificável.

### Sub-passo 199A.I — Validação do plano de sub-passos

| Sub-passo | Escopo | Magnitude |
|---|---|---|
| `.B` | Variant + 3 arms + walk arm + comentário inline + tests E2E + L0 | M |
| `.C` | Relatório consolidado P199 + actualização DEBT M5-residual | S |

Total agregado: ~50 LOC produção + ~120 LOC tests E2E
+ ~60 LOC L0 + relatório consolidado ≈ **M agregado**.

Output: tabela final.

### Sub-passo 199A.J — ADR

Avaliar:

- Pattern ADR-0069 cenário α aplicável por construção.
- Sem decisão arquitectural nova esperada.
- Analogia directa com P182C.

Conclusão esperada: **não cria ADR**.

### Sub-passo 199A.K — DEBT

P199 fecha **E1 estruturalmente completa**.

DEBT M5-residual após P199B+:
- Antes: 1 excepção activa + 1 residuo (E1,
  E2-residuo).
- Após: **0 excepções activas + 1 residuo**
  (E2-residuo).
- 1 pré-requisito restante (sub-store
  `headings_for_toc` — fecha E2-residuo).

**Marco arquitectural**: M5 universal a 1 passo do
fecho após P199 fechar.

**Cenário B continua** (sem DEBT formal aberto).

Output: estado actualizado.

### Sub-passo 199A.L — Outputs

Produzir 3 ficheiros (padrão P181A–P198A):

1. **`00_nucleo/diagnosticos/diagnostico-set-equation-numbering-passo-199a.md`**
   — diagnóstico com 8 secções:
   - §1 Validação estado actual.
   - §2 Decisões cláusula 1–7 (formato O1–O5).
   - §3 Plano de sub-passos sem condicionais.
   - §4 Magnitude consolidada.
   - §5 ADR avaliação.
   - §6 DEBT avaliação.
   - §7 Cadeia E1 com consumers Equation — análise
     empírica.
   - §8 Próximo sub-passo (P199B com escopo concreto).

2. **`00_nucleo/materialization/typst-passo-199a-relatorio.md`**
   — relatório com 14 secções (padrão P181A/etc.).

3. **Sem ADR e sem DEBT formal esperados**.

---

## Restrições

- **Zero código tocado** em qualquer ficheiro fora de
  `00_nucleo/`.
- **Zero testes** modificados.
- **Não criar reservas** de identificadores.
- **Não modificar walk** — P199B+.
- **Não tocar `from_tags`** — P199B+.
- **Não modificar trait `Introspector`** — P185B fechou.
- **Não modificar `TagIntrospector`** — P193B fechou.
- **Não modificar consumer C3 ou C4** — P184D/P194B
  fecharam.
- **Não materializar parser sintáctico** — fora de
  escopo per cláusula 2.
- **Não abrir sub-store `headings_for_toc`** — passo
  paralelo independente.
- **Não materializar P190A** — aguarda M5 universal
  fechar.
- **Não inflar linguagem**: sem "patamar", "limiar",
  "consolidação", "deriva", "subpadrão", "cumulativo",
  "cross-domínio", "paridade observable" como bandeira
  retórica.
- **Aplicar regra dos 2 eixos** a Equation
  empiricamente.
- **Reaproveitar template P182C** literalmente.
- **Sem cláusulas condicionais nos sub-passos `.B`+ do
  plano**.

---

## Critério de conclusão

- Diagnóstico em
  `00_nucleo/diagnosticos/diagnostico-set-equation-numbering-passo-199a.md`
  com 8 secções produzido.
- Relatório em
  `00_nucleo/materialization/typst-passo-199a-relatorio.md`
  com 14 secções produzido.
- 7 cláusulas fechadas com decisão literal.
- Plano de sub-passos sem condicionais (2 sub-passos
  B + C).
- Magnitude consolidada confirmada (M agregado).
- Critério de fecho P199 fixado.
- ADR avaliada (esperado: não criada).
- DEBT M5-residual estado registado.
- Cadeia E1 com consumers Equation analisada
  empiricamente.
- Variant `Content::SetEquationNumbering` confirmada
  ausente.
- Template P182C totalmente mapeado.
- Regra dos 2 eixos aplicada empiricamente.
- Pattern ADR-0069 cenário α aplicável por construção
  documentado.
- Nenhum ficheiro de cristalino tocado.
- Tests workspace 1.859 inalterados.
- `crystalline-lint .` zero violations.

P199A é instrumento. Materialização concreta de
`Content::SetEquationNumbering` começa em P199B.

**Após P199 série fechar**: M5 universal está a **1
pré-requisito do fecho** — sub-store `headings_for_toc`
fecha E2-residuo. Após esse passo paralelo, M5 universal
completo desbloqueia M6 (P190A reescrita do zero —
eliminação `CounterStateLegacy`).
