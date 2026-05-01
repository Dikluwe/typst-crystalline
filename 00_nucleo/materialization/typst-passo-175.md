# Passo P175 — `query(selector)` com Selector minimal (M9 sub-passo 5)

Quinta feature M9. **Primeira feature que usa fixpoint
(P174)**. Materializa:
1. Tipo `Selector` minimal (variant `Kind(ElementKind)`).
2. Stdlib `query(selector)` que consulta
   `ctx.introspector` durante eval.
3. Método `Introspector::query(selector)` no trait.
4. Entry point novo `introspect_to_fixpoint` que adopta
   `run_fixpoint` (P174).
5. Tests E2E que validam fixpoint convergência com query
   real.

**Resolve lacuna #7** (`has_outline`) via
`query(Selector::Kind(Outline))`.

**Pré-condição**: P174 concluído. `run_fixpoint`,
`compute_tags_hash`, `EvalContext.introspector` disponíveis.

**Restrições**:
- Walk em `rules/introspect.rs::walk` **NÃO modificado**.
- API pública existente preservada (`introspect()`,
  `introspect_with_introspector`).
- `kind_index` em `TagIntrospector` continua
  `HashMap<ElementKind, Vec<Location>>` — query retorna
  `Vec<Location>`, **não `Vec<Content>`**. Decisão
  arquitectural registada.
- Output observable não muda em snapshot tests existentes.

---

## Sub-passos

### .A Inventário e decisões locais

1. **Confirmar estado pós-P174**:
   - `run_fixpoint` em `rules/introspect/fixpoint.rs`.
   - `compute_tags_hash` em
     `rules/introspect/convergence.rs`.
   - `EvalContext.introspector: TagIntrospector` field.
   - `MAX_FIXPOINT_ITERATIONS = 5`.

2. **Confirmar `ElementKind` variants existentes**:
   - P171 levou variants para 6: Heading, Figure, Citation,
     Metadata, State, StateUpdate.
   - Confirmar que `Outline` está em ElementKind.
     - Vanilla: Outline tem ElementKind próprio? Ou é
       parte de Heading queries?
     - Cristalino: `grep -rn "Outline" 01_core/src/entities/element_kind.rs`.
     - Se ausente: adicionar `ElementKind::Outline`. Mas
       isto exige que walk emita tag para Outline — verificar
       se `Content::Outline` está em `extract_payload` arms
       como locatable.
     - Se Outline NÃO é locatable em M1: lacuna #7 resolve
       parcialmente. Documentar.

3. **Decisão sobre tipo de retorno de query**:

   - **Opção LOCATIONS**: `query(selector) -> Vec<Location>`.
     - Simples; `kind_index` já tem isto.
     - Caller que quer Content tem que navegar
       Location→Content separadamente (mecanismo não
       existe).
     - Suficiente para casos como
       `query(Outline).is_empty()` (lacuna #7).
     - Sugestão.

   - **Opção CONTENT**: `query(selector) -> Vec<Content>`.
     - Vanilla compat.
     - Exige guardar Content em `TagIntrospector` (P165
       não fez — só Location).
     - Refactor maior: `kind_index` muda para
       `HashMap<ElementKind, Vec<(Location, Content)>>`.
     - Memória extra; afecta clone de Introspector que
       fixpoint faz por iteração.
     - Adiar.

   - **Opção HYBRID**: `query(selector) -> Vec<Location>`
     em M9 inicial; método separado
     `resolve_to_content(loc) -> Option<Content>` quando
     necessário.
     - Compromisso. Mas precisa de mapa
       Location→Content que ainda não existe.

   Escolha sugerida: **LOCATIONS** em P175. Refino para
   CONTENT em passo futuro se consumer real precisar.

4. **Forma de `Selector`**:
   - Enum minimal:
     ```rust
     pub enum Selector {
         Kind(ElementKind),
     }
     ```
   - Variants futuros (vanilla): `Where(...)`, `And(...)`,
     `Or(...)`, `Label(Label)`. **Adiar todos.** P175 só
     inclui `Kind`.
   - Tipo público em `entities/selector.rs`.

5. **Stdlib `query(selector)` forma**:
   - Vanilla: `query(selector)` — selector é literal
     `heading`, `figure`, etc.
   - Cristalino: confirmar como stdlib aceita Selector como
     argumento. Pode requerer:
     - `Value::Selector(Selector)` variant em `Value`
       enum (provavelmente ausente).
     - Ou: stdlib aceita `Value::Type(...)` que mapeia
       para Selector::Kind.
     - Ou: forma simplificada — `query("heading")` aceita
       string.
   - Cláusula gate trivial: forma minimal que funciona.
     Sugestão: `query(kind_str: Str)` que internamente
     parseia para `Selector::Kind`. Adiar `Selector`
     value type para refino.

6. **Adopção de `run_fixpoint`**:

   Decisão de quem adopta:

   - **Opção 1: Entry point novo `introspect_to_fixpoint`**.
     - `pub fn introspect_to_fixpoint<F>(eval_step: F, engine, ctx) -> Result<...>`.
     - Adopção é opt-in; existentes não migram.
     - Sugestão.

   - **Opção 2: `introspect_with_introspector` adopta**.
     - Quebra signature P173.
     - Rejeitada (confirmação P174).

   - **Opção 3: Layouter adopta**.
     - Layouter adopta `run_fixpoint`. Mas Layouter
       actual recebe Content já evaluado — não tem closure
       de eval para passar.
     - Maior refactor.
     - Adiar.

   Escolha: **Opção 1** — entry point novo opt-in.

7. **Mecanismo de tests para query + fixpoint**:
   - Tests precisam construir Engine + EvalContext (helper
     `with_engine!` de P173).
   - Closure de eval em tests: provavelmente Content
     programático (não parse + eval) — closure retorna
     Content fixo que contém `query()` calls.
   - Mas como simular `query()` em Content programático?
     Precisa stdlib mecanismo de eval Func que faz
     query, ou test pode invocar `ctx.introspector.query(...)`
     directamente sem stdlib.
   - Cláusula gate trivial: tests simplificados que
     verificam mecanismo, não a stdlib completa.

Output: notas internas + decisões registadas:
- ElementKind::Outline existe ou criado.
- Tipo de retorno de query (sugestão LOCATIONS).
- Forma de Selector (minimal Kind variant).
- Adopção de fixpoint (Entry point novo).

**Critério de saída e gate de decisão**:
- Se `Outline` ausente de ElementKind e adicionar requer
  variants em Content novos: gate substancial. Reabrir.
- Se `Value::Selector` é exigido pela stdlib forma: gate
  trivial — adoptar forma alternativa.
- Senão prosseguir.

### .B Criar tipo `Selector`

1. L0 `00_nucleo/prompts/entities/selector.md`:
   - Cabeçalho com hash em branco.
   - Camada L1, ficheiro alvo
     `01_core/src/entities/selector.rs`.
   - ADRs: ADR-0033, ADR-0066.
   - Origem vanilla:
     `lab/typst-original/.../foundations/selector.rs`
     (forma minimal — variants extra adiados).
   - Restrições:
     - `pub enum Selector { Kind(ElementKind) }`.
     - `Clone, PartialEq, Eq, Hash`.
   - Critérios:
     - `Selector::Kind(Heading) == Selector::Kind(Heading)`.
     - `Selector::Kind(Heading) != Selector::Kind(Figure)`.

2. L1 `01_core/src/entities/selector.rs`:
   - Cabeçalho `@prompt`.
   - Implementação.
   - Tests co-localizados (3 mínimos: igualdade, hash,
     match).

3. Update `entities/mod.rs`: re-export `Selector`.

**Critério de saída**:
- `cargo check` passa.
- Tests novos passam.
- Linter passa.

### .C Estender `Introspector` trait com `query`

1. L0 `entities/introspector.md`:
   - Adicionar método `query(&self, selector: &Selector) -> Vec<Location>`.
   - Documentar comportamento:
     - `Selector::Kind(kind)` → retorna
       `kind_index[kind].clone()` ou `Vec::new()` se
       ausente.
     - Ordem preserva ordem de inserção (cronológica via
       Location).

2. L1 `entities/introspector.rs`:
   - Adicionar método ao trait + impl em `TagIntrospector`.
   - Tests co-localizados:
     - `Introspector` vazio → `query` retorna `Vec::new()`.
     - Após popular com 2 headings → `query(Selector::Kind(Heading))`
       retorna 2 Locations na ordem.
     - Selector com kind sem entries → `Vec::new()`.

3. Sem mudança em `from_tags` — `kind_index` já é
   populado em P165.

**Critério de saída**:
- `cargo check` passa.
- Tests novos passam.
- Linter passa.

### .D Stdlib `query(kind_str)`

Forma minimal sugerida em `.A.5`. Adaptar conforme decisão.

1. Identificar stdlib registry (P169 usou `make_stdlib`).
2. Adicionar função stdlib `query(kind_str: Str) -> Vec<Value>`
   (ou forma equivalente):
   - Parseia `kind_str` para `ElementKind` via `from_str`
     ou similar (criar helper se ausente).
   - Constrói `Selector::Kind(kind)`.
   - Consulta `ctx.introspector.query(&selector)`.
   - Mapeia `Vec<Location>` para `Vec<Value>` (provavelmente
     `Vec<Value::Location>` se existe; ou `Vec<Value::Int>`
     com Location internal id).

3. **`Value::Location` variant**: confirmar se existe em
   cristalino. Se não:
   - **Opção α**: adicionar `Value::Location(Location)`.
     Trabalho cascade médio.
   - **Opção β**: stdlib retorna número de matches
     (`usize`) em vez de Locations. Suficiente para lacuna
     #7 (`query(Outline).len() > 0` é resposta a
     `has_outline`).
   - Sugestão: **β** em P175. Refino para α se consumer
     precisar.

4. Update L0 stdlib (se existe).

5. Tests:
   - `query("heading")` em iter 1 (introspector vazio) →
     `0` (ou `[]` conforme retorno).
   - `query("heading")` em iter 2 (após walk produzir tags)
     → número correcto.

**Critério de saída**:
- `cargo check` passa.
- Tests novos passam.
- Linter passa.

### .E Entry point `introspect_to_fixpoint`

1. L0 (extensão de
   `00_nucleo/prompts/rules/introspect/fixpoint.md`):
   - Documentar `introspect_to_fixpoint`.

2. L1 em `rules/introspect/fixpoint.rs`:
   ```rust
   pub fn introspect_to_fixpoint<F>(
       eval_step: F,
       engine: &mut Engine,
       ctx: &mut EvalContext,
   ) -> Result<(CounterStateLegacy, TagIntrospector), FixpointError>
   where
       F: FnMut(&mut Engine, &mut EvalContext) -> Result<Content, Vec<SourceDiagnostic>>,
   {
       run_fixpoint(engine, ctx, eval_step)
   }
   ```

   Wrapper directo sobre `run_fixpoint` mas com nome
   semanticamente claro (entry point para introspection
   com fixpoint).

3. Update L0.

**Critério de saída**:
- `cargo check` passa.
- Linter passa.

### .F Tests E2E com fixpoint real

1. **`p175_query_em_doc_estavel_converge`**:
   Closure de eval que retorna Content fixo com 2 headings.
   `introspect_to_fixpoint(closure, engine, ctx)`.
   Verifica:
   - Resultado `Ok(...)` em ≤ MAX_FIXPOINT_ITERATIONS.
   - `introspector.query(Selector::Kind(Heading)).len() == 2`.

2. **`p175_query_evolui_entre_iters_e_converge`**:
   Closure que constrói Content baseado em
   `ctx.introspector.query(...)`. Por exemplo:
   - Iter 1: `ctx.introspector` vazio →
     `query("heading").len() == 0` → Content sem texto
     adicional.
   - Iter 2: `ctx.introspector` populado →
     `query("heading").len() == 2` → Content adicional
     "Total: 2".
   - Iter 3: Content novo regista headings? Não — só lê.
     Convergência em iter 3.
   Verifica que `ctx.introspector` reflecte iter anterior.

3. **`p175_query_via_stdlib`** (se forma stdlib viável):
   Simular `query("heading")` durante eval. Verifica
   resultado correcto.

4. **Lacuna #7 resolvida**:
   `p175_has_outline_via_query`:
   - Doc com Outline → `query("outline")` retorna ≥ 1.
   - Doc sem Outline → `query("outline")` retorna 0.
   - **Apenas se `ElementKind::Outline` existe**.
     Senão, adiar test.

**Critério de saída**:
- 3-4 tests E2E passam.
- Linter passa.

### .G Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace` — todos os tests passam.
   Δ vs P174 baseline (1655). Estimativa: +10 a +15.
3. `crystalline-lint`: zero violations.
4. `Selector` tipo existe.
5. `Introspector::query(selector)` no trait.
6. `TagIntrospector` impl de query.
7. Stdlib `query(...)` registada (forma decidida em `.A`).
8. `introspect_to_fixpoint` entry point novo.
9. Walk **NÃO modificado**.
10. `introspect()` legacy preservada (não usa fixpoint).
11. `introspect_with_introspector` preservada (não usa
    fixpoint — opt-in via novo entry point).
12. **Lacuna #7 resolvida** (se Outline está em ElementKind):
    `m1-lacunas-captura.md` actualizado.
13. Snapshot tests ADR-0033 verdes.
14. Linter passa final.

### .H Encerramento

Escrever
`00_nucleo/materialization/typst-passo-175-relatorio.md` com:

- Resumo: `query(selector)` materializado; primeira feature
  fixpoint adopt; lacuna #7 resolvida.
- Confirmação de cada verificação `.G`.
- Hashes finais de L0s novos/modificados.
- Decisões registadas em `.A`:
  - Tipo de retorno (LOCATIONS).
  - Forma de Selector (minimal).
  - Adopção fixpoint (entry point novo).
  - `Value::Location` α vs β (sugestão β).
  - Outline em ElementKind ou ausente.
- Δ tests vs baseline P174.
- **Estado de M9**: 5/11 features.
- **Estado de M7**: 2/N sub-passos (mecanismo + primeiro
  cliente).
- **Lacuna #7 fechada**: ✅ resolvida via `query`.
- Pendências cumulativas + actualização.
- Estado pós-passo: P175 concluído. P176 desbloqueado.

---

## Critério de conclusão

Todas em conjunto:

1. `.A` produziu inventário sem disparar gate substancial.
2. `Selector` enum minimal materializado.
3. `Introspector::query` no trait.
4. Stdlib `query(...)` registada.
5. `introspect_to_fixpoint` entry point novo.
6. Tests E2E confirmam fixpoint funciona com query real.
7. Lacuna #7 fechada (se Outline existe em ElementKind).
8. Verificações `.G` 1-14 passam.
9. Relatório `.H` escrito.
10. Output observable não muda.
11. M9 5/11 features.
12. M7 2/N sub-passos.

---

## O que pode sair errado

- **`ElementKind::Outline` ausente**: lacuna #7 não fecha
  totalmente em P175. Documentar como pendência. Adicionar
  Outline em passo futuro requer Content variant + arms
  cascade — não trivial.
- **`Value::Location` ausente e cascade alta para criar**:
  fallback Opção β (retornar `usize` count). Aceitável para
  validar mecanismo; refino futuro.
- **Stdlib forma não suporta selector como string**: cláusula
  gate trivial. Adoptar alternativa (e.g.
  `query_heading()`, `query_figure()` como funcs separadas).
- **Tests E2E com fixpoint são complicados**: closure de
  eval programática pode não simular bem o fluxo real.
  Tests podem ser parciais. Documentar limitações.
- **`from_tags` no fixpoint loop precisa Engine**:
  `run_fixpoint` (P174) chama `from_tags(&tags, Some(engine), Some(ctx))`?
  Verificar implementação P174 — se não passa Engine,
  Funcs em loop são ignoradas (defensive). P175 precisa
  ou: `run_fixpoint` adapta-se, ou P175 modifica para
  passar Engine. Cláusula gate trivial.
- **Convergência em casos com query é diferente**: query
  introduce dependência circular natural. Iter 1: query
  vazio; iter 2: query populado mas Content depende de
  query; iter 3: confirma. Tests precisam verificar
  número de iter correcto.
- **`HashMap<ElementKind, Vec<Location>>` itera não-determinística
  em query**: P165 usou `HashMap`. Para query retornar
  ordem consistente, precisa BTreeMap ou ordenação.
  Verificar; ajustar se necessário.
- **Linter divergência**: ajustar conforme erro.

---

## Notas operacionais

- **Tamanho**: M. Selector + query method + stdlib + entry
  point + tests E2E. Maior que P170 (refactor isolado),
  comparável a P171 sem cascade Engine.
- **Pré-condição P176**: feature seguinte M9. Candidatas
  pós-P175: `here()` (precisa current_location), `counter.at`
  (capitaliza P170 + LabelRegistry), `counter.final`,
  `locate(callback)`.
- **Cláusula gate trivial**: aplicável a forma de stdlib,
  retorno LOCATIONS vs CONTENT, value type para Selector.
- **Lacuna #7 fechada parcial ou totalmente** dependendo
  de Outline em ElementKind. Documentar estado real no
  relatório.
- **`introspect_to_fixpoint` é opt-in**: nenhum consumer
  existente migra. Layouter, pipeline, `introspect()` legacy
  não mudam. Migração é trabalho separado em passos
  futuros.
- **Magnitude controlada**: P175 valida ciclo "feature usa
  fixpoint" sem refactor de consumers. Se mecanismo
  funcionar bem, features seguintes (counter.at, here)
  reutilizam padrão.
