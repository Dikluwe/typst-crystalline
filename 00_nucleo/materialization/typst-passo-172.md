# Passo P172 — `Func` callback em `StateUpdate` via Resolution + Engine cascade (M9 sub-passo 4)

Quarta feature M9. Fecha pendência nova de P171: callbacks
`Func` em `state.update(key, fn)`.

**Decisão arquitectural tomada (post-gate P172.A inicial)**:
abordagem **Resolution** — `from_tags` ganha capacidade de
chamar `Func` via cascade de `&mut Engine` desde
`introspect_with_introspector`. Walk continua **puro e
determinístico** (P163 invariante preservado).

**Magnitude reconhecida**: L. Cascade afecta API de
`introspect_with_introspector` e ~38 call-sites de produção
em `03_infra`. API legacy `introspect()` preservada com
comportamento defensivo para Funcs.

**Pré-condição**: P171 concluído. `StateUpdate::Set(Value)`
funciona; pendência callback registada.

**Restrições**:
- Walk em `rules/introspect.rs::walk` **não modificado** —
  continua puro. Walk emite `StateUpdate::Func(fn)` literal.
- Resolution faz eval em `from_tags` apenas.
- API pública legacy `introspect()` preservada
  (comportamento defensivo: Funcs ignoradas ou registadas
  como erro).
- Determinismo do walk preservado (P163 invariante).
- Output observable não muda; snapshot tests passam
  inalterados.

---

## Sub-passos

### .A Inventário detalhado e decisões locais

A decisão arquitectural maior já está tomada (Resolution +
cascade). `.A` confirma detalhes locais.

1. **Confirmar `apply_func` API real**:
   - Localização: `01_core/src/rules/eval/closures.rs:59`
     (per gate report). Confirmar.
   - Assinatura exacta:
     ```rust
     pub(crate) fn apply_func(
         func: Func,
         args: Args,
         ctx: &mut EvalContext,
         engine: &mut Engine<'_>,
     ) -> SourceResult<Value>
     ```
   - Confirmar que `Args`, `EvalContext`, `Engine` são
     tipos públicos ou `pub(crate)` acessíveis a `from_tags`.

2. **Construção de `Args` para call de Func com 1 argumento**:
   - Vanilla: `state.update(key, fn)` chama `fn(current_value)`.
   - Cristalino: como construir `Args` com 1 valor?
     `grep -rn "Args::new\|Args::single\|Args::positional" 01_core/src/`.
   - Se há método trivial: registar.
   - Se não: criar helper `Args::positional(value)`.

3. **Decidir API de `from_tags` modificada**:

   Opções:

   - **A. `from_tags(tags, ctx, engine)`** — assinatura
     directa.
     ```rust
     pub fn from_tags(
         tags: &[Tag],
         ctx: &mut EvalContext,
         engine: &mut Engine,
     ) -> Result<TagIntrospector, EvalError>;
     ```

   - **B. Builder pattern**:
     ```rust
     IntrospectorBuilder::new()
         .with_tags(tags)
         .resolve_funcs(ctx, engine)?
         .build()
     ```

   - **C. Two-pass**: `from_tags(tags) → TagIntrospectorPartial`
     (Funcs não resolvidas) seguido de
     `partial.resolve(ctx, engine) → TagIntrospector`.

   Cláusula gate trivial: escolher **A** se simples, **C**
   se quisermos manter `from_tags` puro com phase de
   resolução opcional. Sugestão: **A** — minimal disruption,
   `from_tags` deixa de ser puro mas é pragmático.

4. **API legacy `introspect()` preservada**:

   Confirmar plano:
   - `introspect()` continua a chamar walk + `from_tags`
     mas SEM Engine.
   - `from_tags` em modo "sem Engine" precisa de mecanismo:
     - **Opção α**: `from_tags` aceita `Option<&mut Engine>`.
       Se `None`, Funcs ignoradas (defensive); só
       `StateUpdate::Set` resolve.
     - **Opção β**: duas funções — `from_tags(tags)` puro
       (ignora Funcs, devolve Introspector parcial) +
       `from_tags_with_engine(tags, ctx, engine)`.
   - Sugestão **α**: parâmetro opcional. Mais simples,
     menos duplicação.

5. **API de `introspect_with_introspector` modificada**:

   Antes:
   ```rust
   pub fn introspect_with_introspector(
       content: &Content,
   ) -> (CounterStateLegacy, TagIntrospector);
   ```

   Depois (Opção α):
   ```rust
   pub fn introspect_with_introspector(
       content: &Content,
       ctx: &mut EvalContext,
       engine: &mut Engine,
   ) -> Result<(CounterStateLegacy, TagIntrospector), EvalError>;
   ```

   Erros possíveis:
   - Func panica ou retorna erro durante eval.
   - `apply_func` reporta erro.

6. **Inventário de call-sites afectados**:

   `grep -rn "introspect_with_introspector(" 01_core/src/ 03_infra/src/`:
   - Production em `03_infra`: P167 reportou ~10 call-sites
     externos a 01_core. Recontar — decisões P166-P171
     podem ter alterado.
   - Tests: P166-P171 adicionaram tests que chamam
     `introspect_with_introspector` directamente.

   Para cada call-site, identificar se Engine + EvalContext
   estão disponíveis no escopo:
   - Production em `03_infra`: provavelmente sim (Engine é
     construído no pipeline).
   - Tests: provavelmente não — precisam construir Engine
     mock ou helper.

7. **Helper de Engine para tests**:

   `grep -rn "Engine::new\|MockEngine\|TestEngine" 01_core/src/ 03_infra/src/`:
   - Se há helper: usar.
   - Se não: criar `#[cfg(test)] fn make_test_engine() -> Engine` em local apropriado.

8. **`Func` derives**: confirmar `Clone` derivado (per gate
   report). Confirmar que `Eq` falha (esperado — `Func` tem
   closures internas). Aplicar workaround estabelecido em
   P169/P171: Hash via `format!("{:?}", ...)`, `Eq` marker.

Output: notas internas + decisões registadas:
- API forma escolhida (A/B/C em ponto 3).
- API legacy mecanismo (α/β em ponto 4).
- Lista de call-sites externos.
- Helper de Engine para tests (existe ou criar).

**Critério de saída e gate de decisão**:
- Se `apply_func` API confirmada e Engine acessível em
  call-sites externos: prosseguir.
- Se algum call-site externo não tem Engine acessível:
  cláusula gate trivial — propagar Engine pelo caller mais
  acima ou identificar local de construção.
- Se helper de Engine não existe e construir é trabalho
  substancial: gate substancial. Reabrir.

### .B Adicionar variant `Func` ao `StateUpdate`

Trabalho mecânico.

1. L0 `entities/state_update.md`:
   - Adicionar `Func(Func)` variant.
   - Documentar semântica: callback que recebe valor actual,
     retorna novo valor. Resolução em `from_tags` via
     Engine.

2. L1 `entities/state_update.rs`:
   - Adicionar variant `Func(Func)`.
   - Aplicar workarounds Hash/Eq se necessário (provável,
     conforme P169 padrão).
   - Tests co-localizados: hash determinístico via Debug
     format.

3. L0+L1 `entities/element_payload.md`/`.rs`:
   - `ElementPayload::StateUpdate { key, update }` agora
     pode carregar `update: StateUpdate::Func(...)`.
   - Sem mudança estrutural; apenas garantia que enum
     estendido propaga.

**Critério de saída**:
- `cargo check` passa.
- Tests novos passam.
- Linter passa.

### .C Modificar `from_tags` para aceitar Engine + EvalContext

1. Decidir entre Opção α (parâmetro opcional) ou β (duas
   funções) conforme decidido em `.A`. Sugestão neste
   passo: **α**.

2. L0 `rules/introspect/from_tags.md`:
   - Documentar nova assinatura:
     ```
     pub fn from_tags(
         tags: &[Tag],
         engine: Option<&mut Engine>,
         ctx: Option<&mut EvalContext>,
     ) -> TagIntrospector;
     ```
   - Comportamento:
     - Se `engine.is_none()` ou `ctx.is_none()`:
       comportamento defensivo — `StateUpdate::Func` é
       ignorada (registry não actualiza).
     - Se ambos `Some`: ao processar
       `ElementPayload::StateUpdate { key, update: Func(fn) }`,
       chama `apply_func` com valor actual; regista `Set`
       resolvido.

3. L1 `rules/introspect/from_tags.rs`:
   - Modificar signatura.
   - Match expandido sobre `update`:
     ```rust
     match update {
         StateUpdate::Set(value) => {
             state_registry.update(key.clone(), (**value).clone(), *loc);
         }
         StateUpdate::Func(fn_value) => {
             match (engine.as_deref_mut(), ctx.as_deref_mut()) {
                 (Some(eng), Some(ctx)) => {
                     let current = state_registry.value_at(key, *loc).cloned();
                     match current {
                         Some(curr) => {
                             let args = Args::positional(curr);
                             match apply_func(fn_value.clone(), args, ctx, eng) {
                                 Ok(new_value) => state_registry.update(key.clone(), new_value, *loc),
                                 Err(_) => {
                                     // erro defensive: registar diagnóstico se infraestrutura existir
                                     // adiar para refino
                                 }
                             }
                         }
                         None => {
                             // sem init prévio — defensive ignore (P171 padrão)
                         }
                     }
                 }
                 _ => {
                     // engine/ctx ausentes — defensive ignore
                 }
             }
         }
     }
     ```
   - Adaptar exactos nomes (Args::positional, etc.) conforme
     `.A.2`.

4. Tests co-localizados:
   - Func que adiciona 1: state init=0 + duas updates Func
     `x => x + 1` → final = 2.
   - Func sem init: defensive ignore.
   - `from_tags(&tags, None, None)`: Funcs ignoradas, só
     Sets aplicados.
   - `from_tags(&tags, Some(...), Some(...))`: Funcs
     aplicadas correctamente.

**Critério de saída**:
- `cargo check` passa.
- Tests novos passam.
- Linter passa.

### .D Modificar `introspect_with_introspector` para propagar

1. L1 `01_core/src/rules/introspect.rs`:

   Antes:
   ```rust
   pub fn introspect_with_introspector(
       content: &Content,
   ) -> (CounterStateLegacy, TagIntrospector) {
       let mut state = CounterStateLegacy::new();
       let mut locator = Locator::new();
       let mut tags = Vec::new();
       walk(content, &mut state, &mut locator, &mut tags, None);
       let introspector = from_tags(&tags);
       (state, introspector)
   }
   ```

   Depois (Opção α):
   ```rust
   pub fn introspect_with_introspector(
       content: &Content,
       engine: Option<&mut Engine>,
       ctx: Option<&mut EvalContext>,
   ) -> (CounterStateLegacy, TagIntrospector) {
       let mut state = CounterStateLegacy::new();
       let mut locator = Locator::new();
       let mut tags = Vec::new();
       walk(content, &mut state, &mut locator, &mut tags, None);
       let introspector = from_tags(&tags, engine, ctx);
       (state, introspector)
   }
   ```

2. `pub fn introspect()` legacy: continua wrapper SEM
   Engine:
   ```rust
   pub fn introspect(content: &Content) -> CounterStateLegacy {
       let (state, _) = introspect_with_introspector(content, None, None);
       state
   }
   ```
   Funcs em `state.update` serão ignoradas defensively.
   Comportamento documentado.

3. Update L0 `rules/introspect.md`:
   - Documentar nova assinatura.
   - Documentar comportamento defensivo de `introspect()`
     legacy.

**Critério de saída**:
- `cargo check` passa em 01_core.
- Tests internos passam.

### .E Adaptar call-sites

1. Para cada call-site identificado em `.A.6`:
   - Production em `03_infra`: passar `Some(&mut engine)` +
     `Some(&mut ctx)`.
     - Se Engine não está acessível no escopo, cláusula
       gate trivial: refactor caller para receber/criar.
   - Tests existentes (P166, P171): se chamada original
     era `introspect_with_introspector(&content)`, mudar
     para `introspect_with_introspector(&content, None, None)`.
     Funcs em testes existentes são raras — apenas testes
     novos em `.F` é que precisam de Engine real.

2. `grep` final para verificar zero call-sites antigos.

**Critério de saída**:
- `cargo check --workspace` passa.
- Todos os tests existentes passam.
- Linter passa.

### .F Stdlib `state_update_with` (callback variant)

1. Estender stdlib registry:
   - `state_update(key, value)` (P171, mantém-se).
   - Adicionar `state_update_with(key, fn)` que retorna
     `Content::StateUpdate { key, update: Func(fn) }`.
   - Cláusula gate trivial sobre nome: `state_update_with`
     vs `state_update_fn` vs detecção polimórfica em
     `state_update`.

2. Tests stdlib:
   - `state_update_with("x", fn)` produz Content correcto
     com Func variant.

**Critério de saída**:
- `cargo check` passa.
- Tests passam.
- Linter passa.

### .G Tests E2E

1. **Documento com state + update via Func**:
   - `state("counter", 0)` + heading + `state_update_with("counter", x => x + 1)` + heading + `state_update_with("counter", x => x * 10)`.
   - Construir Engine via helper `.A.7`.
   - Chamar `introspect_with_introspector(&content, Some(&mut engine), Some(&mut ctx))`.
   - `introspector.state_value("counter", loc_h1)` → `Some(0)`.
   - `introspector.state_value("counter", loc_h2)` → `Some(1)`.
   - `introspector.state_final_value("counter")` → `Some(10)` (`1 * 10 = 10`, não 20).

2. **Func sem init prévio**: defensive ignore.

3. **Engine ausente, Func presente**: `introspect()` legacy
   sobre mesmo Content. Funcs ignoradas. State final só
   reflecte Sets (zero updates aplicados se só houver Funcs)
   ou init.

4. **Determinismo**: dois chamadas a
   `introspect_with_introspector` sobre mesmo Content + mesmo
   Engine produzem mesmo resultado.

5. **State invisível em layout** (regressão P171).

**Critério de saída**:
- 5 tests E2E passam.
- Linter passa.

### .H Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace` — todos os tests passam.
   Δ vs P171 baseline (1634).
3. `crystalline-lint`: zero violations.
4. `StateUpdate::Func(Func)` variant existe.
5. `from_tags` aceita `Option<&mut Engine>` + `Option<&mut EvalContext>`.
6. `introspect_with_introspector` propaga Engine.
7. `introspect()` legacy preservada com comportamento
   defensivo documentado.
8. Stdlib `state_update_with` (ou forma escolhida)
   registada.
9. Walk em `introspect.rs::walk` **NÃO modificado** —
   continua puro (P163 invariante preservado).
10. Determinismo do walk verificado em test E2E.
11. Snapshot tests de paridade ADR-0033 passam inalterados.
12. Linter passa em verificação final.

### .I Encerramento

Escrever
`00_nucleo/materialization/typst-passo-172-relatorio.md` com:

- Resumo: Resolution implementada; callback `Func`
  funcionando via Engine cascade.
- Confirmação de cada verificação `.H`.
- Hashes finais de L0s modificados.
- Decisões registadas em `.A`:
  - API form de `from_tags` (A/B/C).
  - API legacy mechanism (α/β).
  - Helper de Engine para tests.
- Δ tests vs baseline P171.
- **Estado de M9**: 4/11 features materializadas.
- **Pendência fechada**: `Func` callback em `StateUpdate`.
- Pendências cumulativas + actualização.
- Estado pós-passo: P172 concluído. P173 desbloqueado.

---

## Critério de conclusão

Todas em conjunto:

1. `.A` produziu inventário sem disparar gate substancial.
2. `StateUpdate::Func(Func)` existe.
3. `from_tags` aceita Engine + EvalContext via `Option<>`.
4. `introspect_with_introspector` propaga.
5. `introspect()` legacy preservada (Funcs defensive ignore).
6. Walk **NÃO modificado**.
7. Stdlib aceita callback.
8. Tests E2E passam (5 testes).
9. Verificações `.H` 1-12 passam.
10. Relatório `.I` escrito.
11. Output observable não muda.
12. M9 4/11 features concluída.
13. Pendência "Func callback em StateUpdate" fechada.

---

## O que pode sair errado

- **`apply_func` API ligeiramente diferente do reportado**:
  cláusula gate trivial. Adaptar.
- **`Args::positional` não existe**: criar helper.
  Documentar.
- **`Func` derives Hash/Eq problemáticos**: aplicar
  workaround `format!("{:?}", ...)` (padrão P169/P171).
- **Engine não acessível em algum call-site externo**:
  refactor caller para obter Engine. Cascade adicional.
  Cláusula gate trivial; documentar.
- **Helper de Engine para tests não existe e construir é
  pesado**: gate substancial. Reabrir. Possível resolução:
  `MockEngine` minimal que satisfaz `apply_func` para
  Funcs simples.
- **`from_tags` torna-se complicado de testar**: agora tem
  parâmetros opcionais e múltiplos caminhos. Tests
  precisam cobrir 4 combinações (engine S/N x ctx S/N).
- **Erro de Func eval não tem destino claro**: actualmente
  ignorado defensively. Pendência: infraestrutura de
  diagnostics para reportar erros de Func sem panic.
  Adiar para refino.
- **Determinismo de Func falha**: se Func tem side effects
  ou depende de RNG/clock. Vanilla proíbe. Documentar
  restrição em L0.
- **Linter divergência com 5+ ficheiros tocados**: ajustar.

---

## Notas operacionais

- **Tamanho**: L. Maior que P171, comparável a P165 com
  cascade adicional. Decomposto em 9 sub-passos.
- **Walk preservado puro**: invariante P163 mantido.
  Resolution localiza eval em `from_tags` apenas.
- **API legacy preservada**: `introspect()` continua a
  funcionar sem Engine. Comportamento defensivo (Funcs
  ignoradas) é coerente com P171 ("init/update sem init
  → ignore").
- **`Option<&mut Engine>` é trade-off**: funciona, mas
  alguns argumentariam que Engine deveria ser sempre
  obrigatório (purificação). Cristalino aceita o trade-off
  por preservar API legacy.
- **Pré-condição P173**: feature seguinte M9. Engine agora
  disponível em `from_tags` — features futuras (query,
  here, counter.at) podem usar mesma cascade. Investimento
  paga.
- **Pendência fecha**: P171 deixou `Func` callback como
  pendência. P172 fecha. Pendências cumulativas reduzem
  por 1.
- **Cláusula gate trivial**: aplicável a decisões locais
  (forma de Args::positional, nome de stdlib func).
- **Cascade reconhecida**: ~38 call-sites externos
  potencialmente afectados. Maioria provavelmente é
  trivial (passar `None, None` ou Engine que já existe).
  Casos não-triviais documentados durante `.E`.
