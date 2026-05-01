# Passo P178 — `ElementKind::Outline` cascade (lacuna #7 fecha)

Refino arquitectural — não é feature stdlib. Adiciona
`Outline` como kind locatable, completando lacuna #7
parcialmente resolvida em P175 (`query("outline")`
retornava 0 porque `ElementKind::Outline` não existia).

**Diferença face a P169 (Metadata)**: NÃO adiciona variant
novo a `Content` — `Content::Outline` já existe. P178
apenas modifica:
1. Adiciona `ElementKind::Outline`.
2. Adiciona `ElementPayload::Outline`.
3. Modifica `is_locatable` arm de `Outline => false` para
   `Outline => true`.
4. Adiciona `extract_payload` arm para `Content::Outline`.
5. Adiciona `from_tags` arm para popular
   `kind_index[Outline]`.

Sem cascade em ~9 sítios — outros arms exhaustive já têm
decisão tomada para `Content::Outline`.

**Pré-condição**: P177 concluído. `Selector::Kind`,
`Introspector::query`, stdlib `query` disponíveis (P175).

**Restrições**:
- Walk em `rules/introspect.rs::walk` **NÃO modificado**.
- `Content::Outline` variant **NÃO modificado** — apenas
  altera comportamento de funções que matcham sobre ele.
- API pública existente preservada.
- Output observable não muda; snapshot tests passam
  inalterados.
- Sem feature stdlib nova — `query("outline")` continua a
  funcionar via mecanismo P175 mas agora com resultados
  reais.

---

## Sub-passos

### .A Inventário

1. **Confirmar `Content::Outline` variant existente**:
   - `grep -n "Outline" 01_core/src/entities/content.rs`.
   - Identificar campos exactos: provavelmente
     `Content::Outline { ... }` com algumas opções.
     Vanilla tem `depth: Option<NonZeroUsize>`, `title:
     Content`, e mais. Cristalino: confirmar.
   - Se variant é unit (`Content::Outline`): payload
     também unit. Se tem campos: decidir quais capturar.

2. **Confirmar `is_locatable` arm actual**:
   - `01_core/src/rules/introspect/locatable.rs`.
   - `Content::Outline { .. } => false` ou agrupado em
     or-pattern com outros não-locatable.
   - Localizar onde mudar.

3. **Confirmar `extract_payload` fall-through actual**:
   - `01_core/src/rules/introspect/extract_payload.rs`.
   - `_ => None` (P164 confirmou).
   - Localizar onde inserir arm novo.

4. **Confirmar `ElementKind` variants actuais**:
   - P171 levou para 6 (Heading, Figure, Citation,
     Metadata, State, StateUpdate). Confirmar.
   - Adicionar `Outline` (variant 7).

5. **Confirmar `ElementPayload` variants actuais**:
   - 6 paralelos a `ElementKind`. Adicionar `Outline`
     variant.

6. **Decisão sobre campos em `ElementPayload::Outline`**:

   - **Opção α** — sem campos: `ElementPayload::Outline {}`
     ou `ElementPayload::Outline`.
     - Suficiente para `query("outline").len() > 0`.
     - Sem necessidade de capturar info do Outline real.
     - Simplest.

   - **Opção β** — com `depth: Option<usize>`:
     - Permite consumers futuros filtrarem outlines por
       profundidade.
     - Mais informação preservada.
     - Custo: minimal.

   - **Opção γ** — com mais campos (title, etc.):
     - Vanilla rich.
     - Cascade em testes e construções.

   Sugestão: **α** em P178. Refino para β/γ futuro se
   consumer real precisar.

7. **`from_tags` arm para Outline**:
   - Popular `kind_index[ElementKind::Outline]` com
     Location.
   - Padrão idêntico a Heading, Figure, Citation arms.

8. **stdlib `query` aceita "outline"**:
   - P175 stdlib `query(kind_str)` parseia string para
     `ElementKind`. Confirmar que parser aceita "outline".
   - Helper `ElementKind::from_name(&str)` existe (P175 .A
     reportou). Adicionar arm "outline" => ElementKind::Outline.

9. **Layouter arm para `Content::Outline`**:
   - P175 não tocou Layouter para Outline porque Outline
     já tinha arm próprio (renderização). Confirmar que
     adicionar tag Tag::Start/End para Outline não muda
     output observable — Layouter renderiza Content::Outline
     normalmente; tag emitida em paralelo é descartada por
     Layouter.

10. **Lacuna #7**:
    - `m1-lacunas-captura.md` documenta lacuna #7 como
      "Parcial em P175". P178 fecha — actualizar para
      "✅ Resolvida em P178".

Output: notas internas + decisões registadas:
- Forma de `Content::Outline` (campos).
- Forma de `ElementPayload::Outline` (α/β/γ).

**Critério de saída e gate de decisão**:
- Se `Content::Outline` é unit ou com campos simples:
  prosseguir.
- Se `Content::Outline` tem campos complexos (e.g.
  `Content` recursivo): cláusula gate trivial — capturar
  só campos simples.
- Senão prosseguir.

### .B Estender `ElementKind`

1. L0 `00_nucleo/prompts/entities/element_kind.md`:
   - Adicionar `Outline` variant à lista.

2. L1 `01_core/src/entities/element_kind.rs`:
   - Adicionar `Outline` variant.
   - Tests co-localizados:
     - Igualdade `Outline == Outline`.
     - `Outline != Heading`.
     - Hash determinístico.

3. Verificar se `ElementKind::from_name(&str)` (helper
   P175) está completo:
   - Arm `"outline" => Some(ElementKind::Outline)`.
   - Tests: `from_name("outline") == Some(Outline)`.

**Critério de saída**:
- `cargo check` passa.
- Tests novos passam.
- Linter passa.

### .C Estender `ElementPayload`

1. L0 `00_nucleo/prompts/entities/element_payload.md`:
   - Adicionar `Outline` variant (forma decidida em `.A.6`).
   - Sugestão: `Outline` (sem campos, equivalente
     `ElementPayload::Outline`).

2. L1 `01_core/src/entities/element_payload.rs`:
   - Adicionar variant.
   - Hash manual (P169 padrão) cobre automaticamente.
   - Tests co-localizados.

**Critério de saída**:
- `cargo check` passa.
- Tests novos passam.
- Linter passa.

### .D Modificar `is_locatable`

1. L0 `00_nucleo/prompts/rules/introspect/locatable.md`:
   - Documentar que `Outline` agora é locatable.

2. L1 `01_core/src/rules/introspect/locatable.rs`:
   - Mudar arm `Outline => false` para `Outline => true`
     (ou retirar de or-pattern não-locatable e adicionar
     a arm `=> true`).
   - Match continua exaustivo (P164 invariante).

3. Tests co-localizados:
   - `is_locatable(&Content::Outline {..})` retorna
     `true`.
   - Invariante `is_locatable(c) == extract_payload(c).is_some()`
     verificado para Outline.

**Critério de saída**:
- `cargo check` passa.
- Tests novos passam.
- Linter passa.

### .E Adicionar arm a `extract_payload`

1. L0 `00_nucleo/prompts/rules/introspect/extract_payload.md`:
   - Adicionar entrada para `Content::Outline`.

2. L1 `01_core/src/rules/introspect/extract_payload.rs`:
   - Adicionar arm antes do `_ => None`:
     ```rust
     Content::Outline { .. } => Some(ElementPayload::Outline),
     ```
   - Adaptar campos conforme `.A.1`.

3. Tests co-localizados:
   - `extract_payload(&Content::Outline {..})` retorna
     `Some(ElementPayload::Outline)`.

**Critério de saída**:
- `cargo check` passa.
- Tests novos passam.
- Linter passa.

### .F Adicionar arm a `from_tags`

1. L0 `00_nucleo/prompts/rules/introspect/from_tags.md`:
   - Documentar arm Outline.

2. L1 `01_core/src/rules/introspect/from_tags.rs`:
   - Adicionar arm:
     ```rust
     ElementPayload::Outline => {
         kind_index.entry(ElementKind::Outline).or_default().push(*loc);
     }
     ```
   - Match exaustivo continua válido.

3. Tests co-localizados:
   - Tag stream com Outline → `kind_index[Outline]`
     populado com Location.

**Critério de saída**:
- `cargo check` passa.
- Tests novos passam.
- Linter passa.

### .G Tests E2E

1. **`p178_outline_locatable_e_indexavel`**:
   Walk sobre Content com 1 Outline → tags incluem
   Tag::Start/End para Outline. Introspector tem
   `kind_index[Outline]` com 1 Location.

2. **`p178_query_outline_retorna_count_correcto`**:
   Doc com 1 Outline. Stdlib `query("outline")` →
   `Value::Int(1)`. Doc sem Outline → `Value::Int(0)`.
   Lacuna #7 fechada — verificável.

3. **`p178_outline_invisivel_em_layout_paridade`**:
   Snapshot test confirma que adicionar tag para Outline
   não muda output observable.

4. **`p178_outline_via_introspect_to_fixpoint`**:
   `introspect_to_fixpoint` com Content que contém Outline
   → introspector reflecte presence.

**Critério de saída**:
- 4 tests E2E passam.
- Linter passa.

### .H Lacuna #7 fechada

Update `00_nucleo/diagnosticos/m1-lacunas-captura.md`:
- Lacuna #7 marcada como "✅ Resolvida em P178".
- Documentar mecanismo: `ElementKind::Outline` adicionado;
  `query("outline")` retorna count correcto.

### .I Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace` — todos os tests passam.
   Δ vs P177 baseline (1689). Estimativa: +8 a +12.
3. `crystalline-lint`: zero violations.
4. `ElementKind::Outline` existe.
5. `ElementPayload::Outline` existe.
6. `is_locatable(&Content::Outline {..}) == true`.
7. `extract_payload(&Content::Outline {..})` retorna
   `Some`.
8. `from_tags` arm para Outline popula `kind_index`.
9. `ElementKind::from_name("outline")` retorna
   `Some(Outline)`.
10. Walk em `introspect.rs::walk` **NÃO modificado**.
11. `Content::Outline` variant **NÃO modificado**.
12. Lacuna #7 actualizada em `m1-lacunas-captura.md`.
13. Snapshot tests ADR-0033 verdes.
14. Linter passa final.

### .J Encerramento

Escrever
`00_nucleo/materialization/typst-passo-178-relatorio.md` com:

- Resumo: `ElementKind::Outline` cascade aplicado; lacuna
  #7 fechada.
- Confirmação de cada verificação `.I`.
- Hashes finais de L0s modificados.
- Decisões registadas em `.A`:
  - Forma de `ElementPayload::Outline` (α/β/γ).
  - Campos capturados de `Content::Outline`.
- Δ tests vs baseline P177.
- **Estado de M9**: 8/11 features (Outline conta como
  feature, mesmo sendo refino arquitectural).
- **Lacuna #7 fechada**: ✅.
- Pendências cumulativas + actualização.
- Estado pós-passo: P178 concluído. P179 desbloqueado.

---

## Critério de conclusão

Todas em conjunto:

1. `.A` produziu inventário sem disparar gate substancial.
2. `ElementKind::Outline` adicionado.
3. `ElementPayload::Outline` adicionado.
4. `is_locatable` retorna `true` para Outline.
5. `extract_payload` retorna `Some` para Outline.
6. `from_tags` indexa Outline em `kind_index`.
7. Stdlib `query("outline")` retorna count correcto.
8. Lacuna #7 marcada como resolvida.
9. Verificações `.I` 1-14 passam.
10. Relatório `.J` escrito.
11. Output observable não muda.
12. M9 8/11 features.

---

## O que pode sair errado

- **`Content::Outline` tem forma complexa**: campos com
  Content recursivo dificultam captura. Cláusula gate
  trivial: capturar apenas campos simples ou nenhum.
- **`is_locatable` está em or-pattern**: mudar `Outline`
  de or-pattern não-locatable para arm `=> true` requer
  cuidado. Compilador guia.
- **Snapshot tests detectam mudança observable**:
  improvável. Outline já tinha tag emitida? Não, P162
  estabeleceu walk emite tag apenas para is_locatable.
  Adicionar tag pode mudar `Vec<Tag>` mas não output
  visível (Layouter ignora tags).
- **Layouter renderização de Outline conflita**:
  improvável — P162 walk emite tag em paralelo a
  CounterStateLegacy, sem afectar walk arms de Layouter.
- **`from_name("outline")` já existia mas com
  comportamento diferente**: P175 helper. Verificar arm
  actual e adicionar/modificar conforme necessário.
- **Tests de paridade falham**: se algum test legacy
  esperava `query("outline") == 0`, agora pode falhar.
  Investigar e ajustar.
- **Linter divergência**: ajustar conforme erro.

---

## Notas operacionais

- **Tamanho**: S. Sem variant Content novo, sem cascade
  ~9 arms, sem entry point novo. Trabalho concentra-se
  em ElementKind + ElementPayload variants + 2 arms
  modificados/adicionados.
- **Pré-condição P179**: feature seguinte M9. Restantes:
  - `here()` (M, com pré-requisitos).
  - `locate(callback)` (depende de Position).
  - Bib state (lacuna #6, magnitude desconhecida).
  - `query` upgrade (refino).
- **Cláusula gate trivial**: aplicável a forma de
  `ElementPayload::Outline` (campos a capturar).
- **Refino arquitectural, não feature stdlib**: P178 não
  adiciona stdlib func nova — P175 `query("outline")` já
  existia mas retornava 0 porque kind ausente. P178 muda
  comportamento via cascade.
- **Padrão "adicionar locatable kind"**: replicável para
  outros Content variants se consumer real precisar
  (e.g. Equation lacuna). Documentar como template
  reutilizável.
- **Magnitude S confirmada**: P176 e P177 entregaram em
  S sem L0 modificado em alguns casos. P178 modifica L0s
  mas é trabalho minimal.
