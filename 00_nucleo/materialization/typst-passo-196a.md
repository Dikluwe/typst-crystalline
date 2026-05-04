# Passo 196A — Instrução Claude Code

## Contexto mínimo

Typst Cristalino é re-implementação atómica do projecto
`typst/typst` em Rust com arquitectura camadas L0–L4.
Vanilla original está em quarentena em `lab/typst-original/`.
Cristalino vive em `01_core/`, `02_shell/`, `03_infra/`,
`04_wiring/`. ADRs em `00_nucleo/adr/`.

**Snapshot de partida** (a confirmar empiricamente em
sub-passo .A):

- Tests workspace 1.838 verdes; zero violations.
- M9 ✅ 11/11.
- M4-residual fechado funcionalmente (P188B).
- M5 incremental:
  - P189B: Outline migrado + 6 excepções declaradas.
  - P193B: sub-store `ResolvedLabelStore` aberto.
  - P194B: consumer C4 migrado com
    substitution-with-fallback.
  - P195B-E: walk arm Labelled migrado;
    **ADR-0069 ACEITE**; E4 estruturalmente fechada.
- DEBT M5-residual: 2 pré-requisitos restantes.
- Trait `Introspector`: 19 métodos.
- `TagIntrospector`: 8 sub-stores.
- `ElementPayload`: 11 variants.
- `ElementKind`: 9 (inalterado — ADR-0069 bypass
  locatable).
- 5 excepções activas: E1, E2, E3, E5, E6.

P196 é **passo 4 da sequência §9 P189**: migrar walk arm
`Content::Heading` auto-toc para emitir Tag em vez de
mutar `state.resolved_labels[auto-toc-N]` directamente.

**Pattern ADR-0069 já estabelecido** (P195D primeira
aplicação) — reduz incerteza arquitectural. P196 é
**segunda aplicação concreta do pattern**.

P196 fecha **E2 residualmente**. E2 tem 4 mutações (per
P189B §5):
- `state.step_hierarchical("heading", *level)`
- `state.auto_label_counter += 1`
- `state.resolved_labels.insert(auto_label, ...)`
- `state.headings_for_toc.push(...)`

Per P189B §5 + P189A §11.2, sub-store `headings_for_toc`
**ainda não existe** (lacuna #3) — 4ª mutação **não
fecha** em P196.

**P196 fecha 3 das 4 mutações de E2** — auto-toc
generation populando `intr.resolved_labels` para chave
`auto-toc-N`. Mutações `headings_for_toc` ficam
excepcionadas; passo dedicado fecha lacuna #3
posteriormente.

Material de partida verificado:

- `00_nucleo/materialization/typst-passo-195-relatorio-consolidado.md`
  §9 — pattern ADR-0069 disponível para P196.
- `00_nucleo/adr/typst-adr-0069-post-recursion-tag-emission.md`
  — ACEITE em P195E; aplicabilidade futura registada
  (P196/P197/P198).
- `00_nucleo/materialization/typst-passo-189-relatorio-consolidado.md`
  §5 E2 — descrição da excepção: 4 mutações listadas.

P196A é o passo de diagnóstico que precede a
implementação. Magnitude esperada **S** (replica P195A
com pattern ADR-0069 já estabelecido).

---

## Postura do auditor / executor

P196A é passo **L0-puro / diagnóstico-primeiro**, padrão
estabelecido em 17 aplicações.

- **Zero código tocado** em camadas cristalinas.
- **Zero testes** modificados.
- **Pode criar** ADR `PROPOSTO` — improvável (pattern
  ADR-0069 cobre).
- **Pode abrir DEBT** se trabalho identificado for
  adiado.
- **Não modifica** walk, `from_tags`, sub-stores,
  consumer — P196B+.

**Magnitude diagnóstico**: S. Decisões esperadas
seguem padrão P195A — pattern ADR-0069 reduz incerteza.

**Regra dos 2 eixos aplicável** confirmada para Heading
auto-toc em P189A `.A`.

**Pattern post-recursion ADR-0069 disponível** —
reaproveitar diretamente sem decisão arquitectural nova.

---

## Escopo

**Primário**: desenhar migração de walk arm
`Content::Heading` (auto-toc generation) para emitir Tag
em vez de mutar `state.resolved_labels[auto-toc-N]`
directamente.

**Confirmação**: validar inventário factual — forma
exacta do walk arm Heading, mutações actuais,
condições para auto-toc, interacção com walk arm
Labelled.

**Decisões a tomar** — 7 cláusulas:

1. **Forma do payload**:
   - **Opção 1** — Reusar `ElementPayload::Labelled`
     (P195B) com chave `auto-toc-N` no field `label`.
     Sem variant nova.
   - **Opção 2** — Variant nova
     `ElementPayload::HeadingAutoToc { auto_label,
     resolved_text, level }` distinta de Labelled.

2. **Helper `compute_heading_auto_toc`**:
   - Análogo a `compute_labelled` (P195D).
   - Replica lógica actual de geração de texto auto-toc
     (provável: `state.format_hierarchical("heading")`
     ou similar).

3. **Tratamento da chave `auto_label`**:
   - Forma exacta da chave (`Label("auto-toc-{N}")`?
     `Label::auto(N)`?). Confirmar empiricamente em
     `.A`.

4. **`headings_for_toc` mutation residual**:
   - Sub-store ausente (lacuna #3). Decisão:
     **manter como excepção residual** com cross-reference
     a passo dedicado para abrir sub-store
     `headings_for_toc`.

5. **Locator handling** — pattern P195D snapshot+find_map
   reuso de Location aplicável a Heading? Heading é
   locatable — diferente de Labelled.

   **Cláusula gate substancial potencial**: se Heading é
   locatable, walk Locator avança para Heading.
   Auto-toc tag emitida pós-recursão precisa de Location
   sincronizada com Layouter. Auditor decide
   empiricamente.

6. **Mutação legacy preservada** (write paralelo
   durante janela compat M5) — replica P195D pattern.

7. **Critério de fecho de P196** — walk arm Heading
   emite Tag auto-toc; `from_tags` arm processa;
   sub-store `intr.resolved_labels` populated para
   chave `auto-toc-N`; tests E2E confirmam paridade
   observable + activação Introspector path para C4
   em produção (auto-toc labels). E2 fecha 3 das 4
   mutações; mutação `headings_for_toc.push` continua
   activa.

**Fora de escopo**:

- Migração walk arm Figure (P197).
- Migração walks SetHeadingNumbering + CounterUpdate
  (P198).
- `SetEquationNumbering` materialização.
- Sub-store `headings_for_toc` (lacuna #3 — passo
  dedicado paralelo).
- Eliminação `CounterStateLegacy` (P190).

---

## Critérios objectivos

### O1 — Inputs verificáveis

`grep -rn "Content::Heading" 01_core/src/`. Para
cláusula 1, confirmar se `ElementPayload::Labelled`
(P195B) cobre auto-toc semanticamente. Para cláusula
2, confirmar lógica actual de geração de texto
auto-toc.

### O2 — Alternativas

Cláusula 1 tem 2 opções. Cláusulas 2-7 derivam de
cláusula 1.

### O3 — Critério de escolha

Pattern ADR-0069 já estabelecido. Reuso de variant
existente (Opção 1) preferível se semântica encaixa.
Variant nova (Opção 2) só se semântica de auto-toc é
genuinamente distinta de Labelled explicit.

### O4 — Magnitude

P196 implementação S–M:
- Opção 1 (reuso variant): **S** — replica P195C+P195D.
- Opção 2 (variant nova): **M** — adicionar variant +
  arm em `from_tags` + walk arm + tests.

### O5 — Reversibilidade

Reversível por construção. Se variant nova for adicionada
e depois decidir-se reuso, eliminação de variant é
trivial.

---

## Critérios qualitativos

### Q1 — Consistência com padrão estabelecido

P195D estabeleceu pattern. P196 replica:
- Pattern post-recursion (ADR-0069).
- Helper privado para computação.
- Mutação legacy paralela durante janela compat.
- Tests E2E de paridade observable + activação.

Diferença esperada vs P195D: Heading é **locatable**
(`is_locatable=true` para Content::Heading). Walk arm
Labelled era não-locatable. Implicações para Locator
handling (cláusula 5).

### Q2 — Honestidade de magnitude

P196A diagnóstico é S. P196B+ implementação:
- Opção 1: 1-2 sub-passos, magnitude S–M.
- Opção 2: 3-4 sub-passos, magnitude M.

Total agregado: ~50-150 LOC + tests + ADR (se Opção 2).

### Q3 — Cobertura sem regressão

P196 mantém output observable preservado:
- Mutação legacy 4 mutações continuam activas até P196B
  modificar 3 delas.
- Após P196: 3 das 4 mutações redirecionam para Tag;
  4ª (`headings_for_toc.push`) permanece como excepção
  residual.
- Consumer C4 (P194B) começa a receber `Some(text)`
  para auto-toc labels após P196 — segunda inversão
  observable real.

### Q4 — E2 fecha 3 das 4 mutações; residual em lacuna #3

Após P196:
- `state.step_hierarchical("heading", *level)`:
  redireccionada via Tag (auto-toc count agora gerido
  via Introspector).
- `state.auto_label_counter += 1`: redireccionada
  paralelamente (para gerar auto_label key).
- `state.resolved_labels.insert(auto_label, ...)`:
  redireccionada via Tag (pattern ADR-0069).
- `state.headings_for_toc.push(...)`: **continua
  activa** (lacuna #3 sub-store ausente).

E2 não fecha completamente em P196 — fica E2-residual
com 1 mutação activa. Documentação obrigatória per
padrão P189B (4 pontos).

### Q5 — Granularidade

Conforme cláusula 1:

**Se Opção 1** (reuso `ElementPayload::Labelled`):

| Sub-passo | Escopo | Magnitude |
|-----------|--------|-----------|
| `.B` | Walk arm Heading emite Tag (auto-toc) + helper `compute_heading_auto_toc` + tests E2E + L0 + relatório | M |
| `.C` | Relatório consolidado P196 | S |

**Se Opção 2** (variant nova):

| Sub-passo | Escopo | Magnitude |
|-----------|--------|-----------|
| `.B` | Variant nova + L0 + tests + stub no-op | S |
| `.C` | `from_tags` arm + tests | S |
| `.D` | Walk arm + helper + tests E2E | M |
| `.E` | Relatório consolidado P196 | S |

---

## Sub-passos de P196A

### Sub-passo 196A.A — Validação do estado actual

Auditor confirma empiricamente:

1. Confirmar walk arm `Content::Heading` actual:
   - `01_core/src/rules/introspect.rs` — localizar arm.
   - Mutações empíricas (per P189B §5 E2):
     - `state.step_hierarchical("heading", *level)`.
     - `state.auto_label_counter += 1`.
     - `state.resolved_labels.insert(auto_label, ...)`.
     - `state.headings_for_toc.push(...)`.
   - Confirmar linha exacta + contexto.
   - Identificar **condição** para auto-toc (provável:
     `state.is_numbering_active("heading")` + `level
     <= max_depth_toc` ou similar).

2. Confirmar `Content::Heading` variant:
   - `01_core/src/entities/content.rs` — localizar.
   - Campos: `level`, `body`, `label` (esperado).
   - Confirmar `label: Option<Label>`.

3. Confirmar `is_locatable(Content::Heading)`:
   - Esperado: `true` (P165 promoção).
   - Walk Locator avança para Heading.

4. Confirmar geração `auto_label`:
   - Forma exacta (per `.A.1` mutação 2 e 3).
   - Provável: `Label(format!("auto-toc-{}", state.auto_label_counter))`
     ou similar.

5. Confirmar geração `resolved_text` para auto-toc:
   - Provável: `state.format_hierarchical("heading")`
     com prefixo "Secção" ou similar.
   - Confirmar empiricamente.

6. Confirmar `ElementPayload::Labelled` (P195B):
   - Campos: `label`, `resolved_text`, `figure_number`.
   - Verificar se cobre semanticamente auto-toc:
     - `label`: pode receber `auto-toc-N`.
     - `resolved_text`: texto auto-toc.
     - `figure_number`: `None` para auto-toc Heading.
   - **Resposta provável**: cobre — auto-toc é caso de
     resolved label sem figure number.

7. Confirmar walk arm Labelled (P195D):
   - Identificar como interage com Heading se Labelled
     wrap Heading: `Content::Labelled { target:
     Heading, label: explicit }`.
   - Após P195D: explicit label populated via Tag
     Labelled.
   - Após P196: auto-toc label populated via Tag
     Labelled (se Opção 1) ou Tag HeadingAutoToc (se
     Opção 2).

8. Confirmar sub-store `intr.resolved_labels` (P193B):
   - Aceita qualquer `Label` — `auto-toc-N` é válido.
   - Sem necessidade de adaptar sub-store.

9. Confirmar consumer C4 (P194B) inalterado:
   - `references.rs:53-67` consulta
     `intr.resolved_label_for(target)`.
   - Após P196 activa para auto-toc labels também.

10. Confirmar tests existentes:
    - Sentinela E2 P189B (per `.D` ponto 3 do P189B).
    - Identificar quais devem manter-se inalterados.

11. Confirmar lacuna #3 (`headings_for_toc`):
    - `grep -rn "headings_for_toc" 01_core/src/`.
    - Sub-store **NÃO existe**. Mutação 4 da E2
      continua activa após P196.
    - Cross-reference a passo dedicado para abrir
      sub-store.

12. Aplicar regra dos 2 eixos:
    - **Eixo 1**: consumer C4 precisa do valor
      "durante walk" ou snapshot final?
      - Per P194: snapshot final.
    - **Eixo 2**: sub-store `resolved_labels` populated
      em produção?
      - Após P195D: parcial (explicit labels).
      - Após P196: completo (auto-toc + explicit).

Output: tabela com item + estado verificado.

**Critério de saída**:
- Walk arm Heading localizado com 4 mutações exactas.
- Geração `auto_label` + `resolved_text` entendida.
- `is_locatable(Heading)` confirmado.
- `ElementPayload::Labelled` cobre semanticamente
  auto-toc (cláusula 1 candidata Opção 1).
- Lacuna #3 confirmada sem sub-store.

### Sub-passo 196A.B — Decisão cláusula 1 (forma do payload)

**Opção 1 — Reusar `ElementPayload::Labelled`**:

Vantagens:
- Sem variant nova; sem ADR nova.
- `from_tags` arm Labelled (P195C) já popula
  `intr.resolved_labels` — funciona para `auto-toc-N`
  directamente.
- Magnitude S — apenas walk arm modification + helper.
- Replicação literal de pattern P195D.

Desvantagens:
- Semântica menos explícita — auto-toc e explicit
  ambos via mesmo variant. Disambiguation via key
  prefix `auto-toc-N`.

**Opção 2 — Variant nova `HeadingAutoToc`**:

Vantagens:
- Semântica explícita: auto-toc distinto de Labelled.
- Permite payload mais rico (level, position, etc.).

Desvantagens:
- Variant nova + ADR nova (provavelmente).
- Magnitude M.
- Sem benefício real face a Opção 1 (sub-store
  `intr.resolved_labels` é mesmo).

Critério de escolha: simplicidade. Opção 1 é
preferível se semântica encaixa.

Sugestão preliminar: **Opção 1** (reuso). Auditor
confirma empiricamente em `.A.6`.

Output: decisão fixada com justificação literal.

### Sub-passo 196A.C — Decisão cláusula 2 (helper)

Análogo a `compute_labelled` (P195D):

```
fn compute_heading_auto_toc(
    state: &CounterStateLegacy,
    level: usize,
) -> (Option<Label>, Option<String>) {
    if !state.is_numbering_active("heading") {
        return (None, None);
    }
    if !auto_toc_eligible(level) {
        return (None, None);
    }
    let auto_label = Label(format!("auto-toc-{}", state.auto_label_counter));
    let resolved   = state.format_hierarchical("heading")
        .map(|n| format!("Secção {}", n));
    (Some(auto_label), resolved)
}
```

Forma exacta replica lógica actual. Auditor confirma
em `.A.5`.

Output: esquema do helper fixado.

### Sub-passo 196A.D — Decisão cláusula 3 (chave `auto_label`)

Confirmar empiricamente em `.A.4` forma exacta.
Provável: `Label(format!("auto-toc-{}", N))`.

Output: forma fixada.

### Sub-passo 196A.E — Decisão cláusula 4 (`headings_for_toc` residual)

Mutação 4 da E2 continua activa após P196 — sub-store
`headings_for_toc` não existe (lacuna #3).

**Decisão**: manter como **excepção residual E2-resíduo**.
Documentação 4 pontos:
1. Comentário inline no walk arm.
2. L0 `rules/introspect.md` secção "Excepções M5".
3. Test sentinela cobre que mutação continua funcional.
4. Secção em P196 consolidado §"E2 residual".

Cross-reference a passo dedicado para abrir sub-store
`headings_for_toc`.

Output: decisão fixada.

### Sub-passo 196A.F — Decisão cláusula 5 (Locator handling)

Heading é locatable — diferente de Labelled (P195D).
Implicações:

**Sub-cláusula 5.1**: walk Locator avança 1 quando
processa Content::Heading (per P165). Layouter Locator
também avança 1 (gating `is_locatable=true`).
Sequências sincronizadas — sem necessidade de reuso de
Location.

**Sub-cláusula 5.2**: Tag auto-toc emitida pós-recursão
do body. Location obtida via:
- Opção (a): reuso de Location já alocada para
  Heading (per pattern P186 Equation locatable).
- Opção (b): snapshot+find_map per P195D — mas
  redundante porque Heading já é locatable.

Sugestão: **Opção (a)** — reusar Location alocada para
Heading. Mais simples e semanticamente correcto.

**Cláusula gate substancial potencial**: se walk arm
Heading actual já emite Tag (per locatable P165), Tag
auto-toc precisa de coexistir com Tag Heading. Auditor
verifica empiricamente em `.A.1`.

Output: decisão fixada após verificação `.A`.

### Sub-passo 196A.G — Decisão cláusula 6 (mutação legacy preservada)

Replica P195D pattern. Walk arm muta legacy + emite
Tag. Cleanup orgânico em M6 (P190).

**Excepção**: mutação `state.headings_for_toc.push(...)`
**continua sem Tag equivalent** porque sub-store não
existe. Per cláusula 4.

Output: decisão fixada — 3 mutações ganham write
paralelo Tag; 1 continua sem Tag.

### Sub-passo 196A.H — Decisão cláusula 7 (critério de fecho)

P196 fecha quando:
- Walk arm Heading emite Tag auto-toc pós-recursão.
- `from_tags` arm processa Tag → popula
  `intr.resolved_labels[auto-toc-N]`.
- E2 fecha **3 das 4 mutações** estruturalmente.
- 4ª mutação (`headings_for_toc.push`) documentada
  como E2-residual com cross-reference a lacuna #3.
- Tests E2E confirmam paridade observable + activação
  Introspector path para C4 em produção (auto-toc
  labels).
- Consumer C4 (P194B) começa a receber `Some(text)`
  para auto-toc labels.

E2-resíduo (`headings_for_toc.push`) **NÃO fecha em
P196** — fica para passo dedicado abrir sub-store.

Output: critério literal verificável.

### Sub-passo 196A.I — Validação do plano de sub-passos

Conforme cláusula 1:

**Se Opção 1** — 2 sub-passos:

| Sub-passo | Escopo | Magnitude |
|-----------|--------|-----------|
| `.B` | Walk arm Heading + helper `compute_heading_auto_toc` + tests E2E + L0 | M |
| `.C` | Relatório consolidado P196 + actualização DEBT | S |

Total agregado: ~30 LOC walk arm + ~50 LOC helper +
~150 LOC tests + edits L0 + relatório consolidado ≈
**M agregado**.

**Se Opção 2** — 4 sub-passos (raro, esperado).

Output: tabela final conforme cláusula 1.

### Sub-passo 196A.J — ADR

Avaliar:

- **Opção 1** (reuso): pattern ADR-0069 já cobre. **Não
  ADR**.
- **Opção 2** (variant nova): **ADR PROPOSTO** (rara).

Conclusão esperada: **não cria ADR**.

### Sub-passo 196A.K — DEBT

P196 fecha **E2 estruturalmente** (3 das 4 mutações).
1 mutação (`headings_for_toc.push`) fica como
**E2-resíduo**.

DEBT M5-residual após P196B+:
- Antes: 5 excepções activas (E1, E2, E3, E5, E6); 2
  pré-requisitos restantes.
- Após: 4 excepções + 1 resíduo (E1, E2-residuo, E3,
  E5, E6); 2 pré-requisitos restantes.

**Cenário B continua** (sem DEBT formal aberto).

Output: estado actualizado.

### Sub-passo 196A.L — Outputs

Produzir 3 ficheiros (padrão P181A–P195A):

1. **`00_nucleo/diagnosticos/diagnostico-walk-heading-passo-196a.md`**
   — diagnóstico com 8 secções:
   - §1 Validação estado actual.
   - §2 Decisões cláusula 1–7 (formato O1–O5).
   - §3 Plano de sub-passos sem condicionais (conforme
     cláusula 1).
   - §4 Magnitude consolidada.
   - §5 ADR avaliação.
   - §6 DEBT avaliação (E2-residuo registado).
   - §7 E2-residuo: documentação 4 pontos para
     `headings_for_toc.push`.
   - §8 Próximo sub-passo (P196B com escopo concreto).

2. **`00_nucleo/materialization/typst-passo-196a-relatorio.md`**
   — relatório com 14 secções (padrão P181A/etc.).

3. **Sem ADR e sem DEBT formal esperados**.

---

## Restrições

- **Zero código tocado** em qualquer ficheiro fora de
  `00_nucleo/`.
- **Zero testes** modificados.
- **Não criar reservas** de identificadores.
- **Não modificar walk** — P196B+.
- **Não tocar `from_tags`** — P196B+.
- **Não modificar trait `Introspector`** — P185B fechou.
- **Não modificar `TagIntrospector`** — P193B fechou.
- **Não modificar consumer C4** — P194B fechou.
- **Não migrar walk arm Figure** — P197.
- **Não abrir sub-store `headings_for_toc`** — passo
  dedicado paralelo.
- **Não inflar linguagem**: sem "patamar", "limiar",
  "consolidação", "deriva", "subpadrão", "cumulativo",
  "cross-domínio", "paridade observable" como bandeira
  retórica.
- **Aplicar regra dos 2 eixos** a auditoria empírica.
- **Reaproveitar pattern ADR-0069** sem decisão
  arquitectural nova.
- **Sem cláusulas condicionais nos sub-passos `.B`+ do
  plano**.

---

## Critério de conclusão

- Diagnóstico em
  `00_nucleo/diagnosticos/diagnostico-walk-heading-passo-196a.md`
  com 8 secções produzido.
- Relatório em
  `00_nucleo/materialization/typst-passo-196a-relatorio.md`
  com 14 secções produzido.
- 7 cláusulas fechadas com decisão literal.
- Plano de sub-passos sem condicionais (2 ou 4
  sub-passos conforme cláusula 1).
- Magnitude consolidada confirmada (provável M
  agregado).
- Critério de fecho P196 fixado.
- ADR avaliada (esperado: não criada).
- DEBT M5-residual estado registado (E2-resíduo
  catalogado).
- Documentação 4 pontos de E2-resíduo planeada para
  P196B.
- Regra dos 2 eixos aplicada empiricamente.
- Pattern ADR-0069 reaproveitado.
- Nenhum ficheiro de cristalino tocado.
- Tests workspace 1.838 inalterados.
- `crystalline-lint .` zero violations.

P196A é instrumento. Migração concreta de walk arm
Heading começa em P196B+.
