# Passo P174 — Mecanismo de fixpoint (M7 sub-passo 1)

Início de M7 — fixpoint loop e convergência. Materializa
**infraestrutura sem clientes**: features que dependem de
fixpoint (`query()`, `here()`, `counter.at`, etc.) ficam para
P175+. P174 entrega apenas mecanismo.

**Pré-condição**: P173 concluído. Engine cascade disponível
em `from_tags`. `TagIntrospector` materializado e populado
em paralelo a `CounterStateLegacy`.

**Restrições**:
- Sem features stdlib novas (não adicionar `query`/`here`).
- Sem migração de consumers M5.
- API pública compatível: callers existentes funcionam sem
  mudança aparente (loop interno transparente, ou caller
  explícito conforme `.A`).
- Output observable não muda em documentos sem queries
  (caso comum). Snapshot tests passam inalterados.
- Determinismo preservado: dado mesmo input, fixpoint
  converge no mesmo número de iterações com mesmo
  resultado.

---

## Sub-passos

### .A Inventário e decisões arquitecturais

Decisão arquitectural maior. **Inventário cuidadoso
obrigatório.**

1. **Confirmar pipeline eval → walk → tags actual**:
   - `grep -rn "eval(\|walk(\|introspect_with_introspector(" 01_core/src/ 03_infra/src/ | head -50`.
   - Identificar:
     - Onde Content é construído via eval.
     - Quem chama `introspect_with_introspector`.
     - Se Layouter chama eval antes de walk, ou se eval
       é feito mais cedo (pipeline).
   - Diagrama do fluxo actual (notas internas, não
     ficheiro).

2. **Decisão: onde corre o loop de fixpoint**:

   **Opção LOOP_INTERNAL**: `introspect_with_introspector`
   contém o loop. Caller vê só resultado convergido.

   ```rust
   pub fn introspect_with_introspector_fixpoint(
       eval_root: &SyntaxRoot,  // ou Source
       engine: &mut Engine,
       ctx: &mut EvalContext,
   ) -> Result<(CounterStateLegacy, TagIntrospector), Error> {
       let mut introspector_prev = TagIntrospector::empty();
       for iter in 0..MAX_ITERATIONS {
           ctx.introspector = introspector_prev.clone();
           let content = eval(eval_root, engine, ctx)?;
           let mut state = CounterStateLegacy::new();
           let mut locator = Locator::new();
           let mut tags = Vec::new();
           walk(&content, &mut state, &mut locator, &mut tags, None);
           let introspector = from_tags(&tags, Some(engine), Some(ctx));
           if has_converged(&introspector, &introspector_prev) {
               return Ok((state, introspector));
           }
           introspector_prev = introspector;
       }
       Err(Error::FixpointDidNotConverge)
   }
   ```

   - **Vantagem**: caller não precisa de saber sobre
     iteração; transparente.
   - **Desvantagem**: muda significativamente API
     (recebe `&SyntaxRoot` em vez de `&Content`); eval
     fica controlado por `introspect`.

   **Opção LOOP_EXTERNAL**: caller (Layouter ou pipeline)
   faz loop explícito.

   ```rust
   // Caller:
   let mut introspector_prev = TagIntrospector::empty();
   for iter in 0..MAX_ITERATIONS {
       ctx.introspector = introspector_prev.clone();
       let content = eval(...)?;
       let (state, introspector) = introspect_with_introspector(&content, Some(engine), Some(ctx));
       if has_converged(&introspector, &introspector_prev) {
           // converged
           break;
       }
       introspector_prev = introspector;
   }
   ```

   - **Vantagem**: `introspect_with_introspector` mantém
     signatura (P173); separação clara de
     responsabilidades.
   - **Desvantagem**: caller precisa de implementar loop;
     se há múltiplos callers, lógica duplicada.

   Cláusula gate trivial: avaliar quem chama
   `introspect_with_introspector`. Se 1-2 callers
   centralizados (ex. só pipeline ou só Layouter):
   LOOP_EXTERNAL aceitável. Se múltiplos: LOOP_INTERNAL
   melhor.

   Sugestão prévia: **LOOP_EXTERNAL** com helper utilitário
   (função pública `run_fixpoint(...)`) que callers chamam.
   Caller decide se usa fixpoint ou só 1 iteração (modo
   compat).

3. **Decisão: como detectar convergência**:

   **Opção HASH_TAGS**: compara hash de `Vec<Tag>` entre
   iterações.
   - `Vec<Tag>` é determinístico (P163 invariante).
   - Hash via mecanismo existente (`format!("{:?}", tags)`
     + SipHash, padrão P162).
   - Convergência: hash iter N == hash iter N-1.
   - **Vantagem**: simples; tags são serializáveis; já há
     infraestrutura.
   - **Desvantagem**: hash colision teórica (extremamente
     improvável para SipHash duplo).

   **Opção PARTIALEQ_INTROSPECTOR**: implementar `PartialEq`
   estrutural em `TagIntrospector`.
   - Sub-stores precisam ser comparáveis.
   - `LabelRegistry`, `CounterRegistry`, `MetadataStore`,
     `StateRegistry` — todos precisam `PartialEq`.
   - **Vantagem**: comparação directa, sem hash.
   - **Desvantagem**: complica `Value` (NaN issue
     conhecido); muitos sub-stores a tocar.

   **Opção HASH_INTROSPECTOR**: hash directo do introspector
   serializado.
   - Similar a HASH_TAGS mas no Introspector final.
   - Mesma fragilidade `format!("{:?}", ...)`.

   Cláusula gate trivial: escolher conforme custo.

   Sugestão prévia: **HASH_TAGS**. Tags são forma canónica
   da informação que vai virar Introspector; se tags são
   iguais, Introspector também é. Custo de
   PARTIALEQ_INTROSPECTOR é alto (toca em Value).

4. **Hard cap de iterações**:
   - Vanilla usa cap de 5 ou similar.
   - Cristalino: definir constante (sugestão **5**) com
     erro claro se excedido.
   - Erro tipo: `Error::FixpointDidNotConverge` ou similar.
     Verificar tipo existente para erros do pipeline.

5. **`EvalContext.introspector` field**:
   - Adicionar field `introspector: TagIntrospector` (não
     `&TagIntrospector` para evitar lifetime threading).
   - Default: `TagIntrospector::empty()`.
   - Read-only no eval (Funcs/stdlib lêem; não escrevem).
   - Confirmar que `TagIntrospector: Clone` (P165 já
     deveria ter; verificar).

6. **Confirmar `MAX_ITERATIONS` aceitável para tests**:
   - Tests de fixpoint vão precisar verificar:
     (a) Documento sem queries converge em **1 iter**.
     (b) Documento que muda Introspector entre iters
         converge em N iters.
     (c) Documento que oscila excede cap.
   - Hard cap de 5 dá margem para tests.

7. **Determinismo preservado**:
   - Dado mesmo input, fixpoint deve produzir mesmo
     resultado e mesmo número de iterações.
   - Walk já determinístico (P163). `from_tags` agora pode
     ser não-determinístico se Func eval depende de RNG/clock
     — restrição existente.
   - Documentar.

Output: notas internas + decisões registadas:
- Local do loop (LOOP_INTERNAL vs LOOP_EXTERNAL).
- Mecanismo de convergência (HASH_TAGS / PartialEq /
  HASH_INTROSPECTOR).
- Constante MAX_ITERATIONS.
- Forma de erro de não-convergência.

**Critério de saída e gate de decisão**:
- Se eval pipeline é claro (1-2 callers de
  `introspect_with_introspector`): prosseguir.
- Se eval é executado em sítios díspares ou tem forma
  estranha: cláusula gate trivial — adoptar abordagem
  mais simples para o caso.
- Se algum mecanismo (`apply_func`, eval) for
  incompatível com loop: gate substancial.

### .B Adicionar `EvalContext.introspector` field

1. L0 (se existe) `00_nucleo/prompts/.../eval_context.md`
   ou similar.
2. L1 do EvalContext: adicionar field
   `introspector: TagIntrospector`.
   - Inicializar em construtor com `TagIntrospector::empty()`.
   - Field público ou getter/setter — escolher conforme
     padrão.
3. Tests:
   - Construir EvalContext, verificar
     `ctx.introspector.query_first(Heading) == None`.

**Critério de saída**:
- `cargo check` passa.
- Tests novos passam.
- Linter passa.

### .C Mecanismo de convergência

Apenas se `.A` escolheu HASH_TAGS (sugestão).

1. L0+L1 helper em
   `01_core/src/rules/introspect/convergence.rs` (ou
   módulo similar):
   - L0: `00_nucleo/prompts/rules/introspect/convergence.md`.
   - Função `pub fn tags_have_converged(prev: &[Tag], curr: &[Tag]) -> bool`
     ou método `Vec<Tag>::convergence_hash() -> u128`.
   - Implementação: hash via `format!("{:?}", tags)` +
     SipHash duplo (padrão P162).

2. Tests:
   - Tags vazias convergem (hash igual).
   - Tags idênticas convergem.
   - Tags diferentes não convergem.

3. Update L0 `00_nucleo/prompts/rules/introspect.md` para
   documentar uso de convergence em loop futuro.

Se PartialEq escolhido (alternativa), implementar em
sub-stores em vez de função utilitária. Custo maior;
documentar.

**Critério de saída**:
- `cargo check` passa.
- Tests passam.
- Linter passa.

### .D Função `run_fixpoint` (LOOP_EXTERNAL)

Apenas se `.A` escolheu LOOP_EXTERNAL.

1. L0 `00_nucleo/prompts/rules/introspect/fixpoint.md`.
2. L1 `01_core/src/rules/introspect/fixpoint.rs`:
   ```rust
   pub const MAX_FIXPOINT_ITERATIONS: usize = 5;

   pub enum FixpointError {
       NotConverged,
       Eval(SourceError),
   }

   pub fn run_fixpoint<F>(
       engine: &mut Engine,
       ctx: &mut EvalContext,
       mut eval_step: F,
   ) -> Result<(CounterStateLegacy, TagIntrospector), FixpointError>
   where
       F: FnMut(&mut Engine, &mut EvalContext) -> Result<Content, SourceError>,
   {
       let mut prev_introspector = TagIntrospector::empty();
       let mut prev_tags_hash: Option<u128> = None;
       for iteration in 0..MAX_FIXPOINT_ITERATIONS {
           ctx.introspector = prev_introspector.clone();
           let content = eval_step(engine, ctx).map_err(FixpointError::Eval)?;

           let mut state = CounterStateLegacy::new();
           let mut locator = Locator::new();
           let mut tags = Vec::new();
           walk(&content, &mut state, &mut locator, &mut tags, None);

           let curr_tags_hash = compute_tags_hash(&tags);
           let introspector = from_tags(&tags, Some(engine), Some(ctx));

           if let Some(prev_hash) = prev_tags_hash {
               if prev_hash == curr_tags_hash {
                   // converged
                   return Ok((state, introspector));
               }
           }

           prev_tags_hash = Some(curr_tags_hash);
           prev_introspector = introspector;
       }
       Err(FixpointError::NotConverged)
   }
   ```

   - **Nota**: closure `eval_step` deixa caller decidir como
     evaluar. Caller passa closure que faz parse + eval da
     forma que faz sentido para si.
   - Loop sai quando convergem **dois** ciclos consecutivos
     (iter N+1 igual a iter N). Primeira iter nunca pode
     convergir (não há prev).

3. Update L0 correspondente.

4. Tests directos da função:
   - Closure que retorna sempre mesmo Content → converge
     em 2 iter.
   - Closure que retorna Content diferente cada iter →
     erro `NotConverged`.
   - Closure que erro de eval → erro `Eval(_)`.

**Critério de saída**:
- `cargo check` passa.
- Tests novos passam.
- Linter passa.

### .E Tests E2E

1. Tests em `rules/introspect/fixpoint.rs::tests` ou
   módulo dedicado:

   - **`fixpoint_converge_em_uma_iter_para_doc_sem_queries`**:
     closure retorna Content fixo (sem stdlib que dependa
     de Introspector). Loop deve convergir em 2 iter (1
     produção + 1 confirmação). Tags hash idêntico.

   - **`fixpoint_excede_cap_oscilatorio`**:
     closure retorna Content que oscila (depende de
     `iteration` interno, simulando dependência mal
     comportada). Loop excede cap, retorna `NotConverged`.

   - **`fixpoint_propaga_erro_eval`**:
     closure retorna `Err(SourceError::...)`. Loop devolve
     `Eval(_)` no primeiro tick.

   - **`introspector_da_iter_anterior_em_evalctx`**:
     verificar que ctx.introspector é actualizado entre
     iterações (closure inspecciona e regista; teste
     verifica que iter 2 vê introspector populado por
     iter 1).

2. Snapshot tests existentes não devem mudar — caller
   actual (Layouter, pipeline) ainda não usa
   `run_fixpoint`. Adopção é P175+.

**Critério de saída**:
- 4 tests E2E passam.
- Linter passa.

### .F Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace` — todos os tests passam.
   Δ vs P173 baseline (1646). Estimativa: +10 a +15 tests.
3. `crystalline-lint`: zero violations.
4. `EvalContext.introspector` existe (default empty).
5. `run_fixpoint` (ou nome equivalente) existe se
   LOOP_EXTERNAL escolhido.
6. Mecanismo de convergência (HASH_TAGS ou PartialEq)
   funciona — tests confirmam.
7. `MAX_FIXPOINT_ITERATIONS` definida (sugestão 5).
8. Erro `FixpointError` (ou tipo equivalente) tem variant
   `NotConverged`.
9. Walk em `introspect.rs::walk` **NÃO modificado**.
10. `introspect_with_introspector` signature **inalterada**
    (P173 forma preservada).
11. **Sem features stdlib novas**: `query`, `here`,
    `counter.at`, `locate` continuam ausentes.
12. **Caller actual não usa fixpoint ainda**: mecanismo
    está disponível mas pipeline existente continua a
    fazer 1 iter (P175+ adopta).
13. Snapshot tests de paridade ADR-0033 passam inalterados.
14. Linter passa em verificação final.

### .G Encerramento

Escrever
`00_nucleo/materialization/typst-passo-174-relatorio.md` com:

- Resumo: mecanismo de fixpoint materializado
  (loop + convergência + EvalContext.introspector); sem
  clientes ainda.
- Confirmação de cada verificação `.F`.
- Hashes finais de L0s novos/modificados.
- Decisões registadas em `.A`:
  - Local do loop escolhido.
  - Mecanismo de convergência escolhido.
  - MAX_FIXPOINT_ITERATIONS valor.
- Δ tests vs baseline P173.
- **Estado de M7**: 1/N sub-passos. Mecanismo entregue;
  features ficam para P175+.
- **Estado de M9**: 4/11 features (P174 não conta como
  feature stdlib).
- Pendências cumulativas + actualização.
- Estado pós-passo: P174 concluído. P175 desbloqueado —
  primeira feature que usa fixpoint
  (`query()`/`here()`/`counter.at`).

---

## Critério de conclusão

Todas em conjunto:

1. `.A` produziu inventário sem disparar gate substancial.
2. `EvalContext.introspector` field existe.
3. Mecanismo de convergência implementado.
4. `run_fixpoint` (ou equivalente) implementado se
   LOOP_EXTERNAL.
5. MAX_FIXPOINT_ITERATIONS + erro de não-convergência.
6. Walk **NÃO modificado**.
7. `introspect_with_introspector` signature **inalterada**.
8. Tests E2E confirmam: convergência em 1-2 iter para doc
   sem queries; erro NotConverged para oscilatório;
   propagação de erros eval; introspector actualizado
   entre iterações.
9. Sem features stdlib novas em P174.
10. Verificações `.F` 1-14 passam.
11. Relatório `.G` escrito.
12. Output observable não muda em snapshot tests.
13. Caller actual não usa fixpoint ainda (compat).

---

## O que pode sair errado

- **Eval pipeline mais complicado que esperado** (`.A.1`):
  documento de eval pode envolver state interno difícil
  de iterar. Cláusula gate trivial: adoptar closure
  `FnMut` que encapsula complexidade do caller.
- **`TagIntrospector::Clone` ausente ou caro**: Clone
  necessário para passar Introspector da iter anterior
  ao EvalContext novo. P165 deve ter `Clone` derivado.
  Se não: adicionar; verificar custo (sub-stores podem
  ser grandes).
- **Hash de tags via `format!("{:?}", ...)` colide**:
  improvável (SipHash duplo), mas teórico. Se algum
  test detectar: investigar; pode forçar PartialEq.
- **Loop converge mas resultado errado**: convergência
  detecta apenas ausência de mudança entre iter; não
  garante correctness. Tests verificam apenas mecanismo;
  semântica é P175+ quando features reais existem.
- **`EvalContext.introspector` field cria lifetime
  cascade**: se EvalContext já tem lifetimes complexos,
  adicionar `TagIntrospector` (owned) pode ajudar.
- **Integração com Layouter futura é não-trivial**:
  Layouter actual chama `introspect_with_introspector`
  com Content já evaluado. Para usar fixpoint, Layouter
  precisará de receber `eval_step` como input ou ter
  lógica de iteração própria. P175+ resolve.
- **Closure `FnMut` cria problemas de borrow**: se
  `eval_step` precisa de borrowed state externo, lifetimes
  podem complicar. Pode precisar `Box<dyn FnMut...>` ou
  similar.
- **Tests precisam construir Content programaticamente**:
  closures de teste não fazem parse real. Construir
  `Content` directo (sem eval) cobre teste de mecanismo.
  Documentar que tests semânticos vêm em P175+.
- **Linter divergência com vários ficheiros tocados**:
  ajustar conforme erro reportado.

---

## Notas operacionais

- **Tamanho**: M. Sem features stdlib; trabalho concentra-se
  em loop + convergência + 1 field novo em EvalContext.
  Pode crescer para M-L se LOOP_INTERNAL for escolhido
  (refactor maior de `introspect_with_introspector`).
- **Pré-condição P175**: feature que usa fixpoint pode ser
  escrita. Candidatas: `here()` (mais simples — ler
  current_location do EvalContext, mas precisa também de
  current_location, separadamente), `query()` (mais
  complexa).
- **Cláusula gate trivial**: aplicável a decisões locais
  (forma de erro, posicionamento de helpers).
- **Optimização early-exit adiada**: vanilla detecta
  estaticamente se documento usa queries; se não, salta
  iter extra. Cristalino P174 não faz — sempre executa
  pelo menos 2 iter para confirmar convergência. Refino
  futuro.
- **Custo de fixpoint**: 2× eval no caso típico (1 produção
  + 1 confirmação). Docs com queries que mudam: até
  MAX_ITERATIONS× eval. Sem comemo, sem memoização real.
  Documentar como limitação conhecida; refino M7+ com
  comemo.
- **Determinismo**: walk e from_tags determinísticos (P163);
  eval determinístico (Funcs sem side effects, vanilla
  proibition). Fixpoint herda determinismo.
- **Sem mudança observable em P174**: caller actual não
  usa fixpoint. Snapshot tests inalterados. Mecanismo
  entrega valor apenas em P175+.
