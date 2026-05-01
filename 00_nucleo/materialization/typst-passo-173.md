# Passo P173 — Cascade Engine + eval real em `from_tags` (M9 sub-passo 5)

**Continuação de P172 com escopo correctivo.** P172 entregou
superfície tipológica (`StateUpdate::Func` variant + stdlib
`state_update_with`) mas adiou eval real para stub silencioso.
Decisão pós-P172: repor — implementar cascade Engine que P172
spec original especificava.

P173 NÃO duplica trabalho. Variant e stdlib já existem.
P173 acrescenta apenas:
1. Cascade `Engine` + `EvalContext` desde
   `introspect_with_introspector` até `from_tags`.
2. Eval real de `Func` em `from_tags` via `apply_func`.
3. **Limpeza dos tests do stub P172** que codificam
   comportamento errado.

**Pré-condição**: P172 concluído (stub). `StateUpdate::Func`
variant existe; `state_update_with` registada; `apply_func`
disponível em `01_core/src/rules/eval/closures.rs`.

**Restrições**:
- Walk em `rules/introspect.rs::walk` **NÃO modificado** —
  continua puro (P163 invariante preservado).
- API pública legacy `introspect()` preservada
  (comportamento defensivo: Funcs ignoradas quando Engine
  ausente).
- Determinismo do walk preservado.
- Output observable não muda; snapshot tests passam
  inalterados.
- Tests do stub P172 **explicitamente removidos ou
  reescritos** — comportamento "Func ignorada
  silenciosamente" deixa de ser invariante.

---

## Sub-passos

### .A Inventário detalhado e decisão sobre API

Reverificar (não confiar em P172):

1. **Confirmar estado actual**:
   - `StateUpdate::Func(Func)` variant existe em
     `01_core/src/entities/state_update.rs`.
   - Stdlib `state_update_with(key, fn)` em
     `01_core/src/rules/stdlib*.rs`.
   - `from_tags` em
     `01_core/src/rules/introspect/from_tags.rs` tem arm
     `StateUpdate::Func(_)` que é no-op (per P172
     relatório).
   - Tests stub identificados:
     - `func_variant_e_silenciosamente_ignorada_em_from_tags`
       — codifica invariante incorrecto. **REMOVER**.
     - `func_variant_e_invisivel_em_layout` — testa que
       layout não renderiza Func. Continua válido. Manter.
     - `set_continua_a_funcionar_apos_func_variant` —
       regressão Set. Manter.

2. **Confirmar `apply_func` API real**:
   - `01_core/src/rules/eval/closures.rs:59` (per gate
     report P172.A).
   - Assinatura:
     ```rust
     pub(crate) fn apply_func(
         func: Func,
         args: Args,
         ctx: &mut EvalContext,
         engine: &mut Engine<'_>,
     ) -> SourceResult<Value>
     ```
   - Confirmar visibilidade (pub(crate) deve ser acessível
     a `from_tags` se forem mesmo crate). Se `from_tags` é
     no mesmo crate (`01_core`): OK. Confirmar.

3. **Construção de `Args` com 1 valor**:
   - `grep -rn "Args::positional\|Args::single\|Args::new" 01_core/src/`.
   - Se há helper directo: usar.
   - Se não: criar helper minimal `Args::positional(value: Value) -> Args`.

4. **Decisão sobre obrigatório vs opcional**:

   | Critério | `Option<&mut Engine>` | `&mut Engine` obrigatório |
   |----------|------------------------|---------------------------|
   | API legacy `introspect()` preserva | ✓ (passa None) | ✗ — quebra ou wrapper precisa criar Engine vazio |
   | Funcs em modo legacy | Defensive ignore | Erro / panic |
   | Call-sites externos a adaptar | ~38 (mas trivial: passar None) | ~38 (precisam Engine real ou mock) |
   | Pureza de API | Inferior (Option) | Superior |
   | Cláusula match em from_tags | 4 ramos (engine S/N × current S/N) | 2 ramos (current S/N) |

   Sugestão: **Option** — preserva API legacy com custo
   pequeno (1 ramo extra de match). Magnitude controlada.

   Cláusula gate trivial: confirmar em `.A` se Engine está
   acessível na maioria dos 38 call-sites externos. Se sim:
   ambos viáveis. Se não: Option é a escolha.

5. **Inventário de call-sites afectados**:

   `grep -rn "introspect_with_introspector(" 01_core/src/ 03_infra/src/`:
   - Production em `03_infra` (P166 reportou ~10; P167
     re-confirmou; P168 adicionou via Layouter
     `layout_with_introspector`).
   - Tests em `01_core` e `03_infra`.

   Para cada call-site:
   - Production: identificar se `Engine` e `EvalContext`
     estão no escopo (provavelmente sim).
   - Tests: maioria provavelmente passa `None, None` —
     trivial.

6. **Helper Engine para tests novos**:

   Tests P173 novos (eval real) precisam construir Engine
   funcional. Pode requerer:
   - World mock + route mock.
   - Sink (typst.sink Sink).
   - Outros campos de Engine.

   `grep -rn "Engine::new\|Engine {" 01_core/src/ 03_infra/src/`:
   - Identificar onde Engine é construído em produção.
   - Se há test helper existente: usar.
   - Se não: criar `make_test_engine() -> Engine` em
     local apropriado (tests de `from_tags` ou
     `01_core/src/rules/introspect/test_helpers.rs`).

7. **Confirmar `Func` derives**:
   - `Clone` derivado (P172 confirmou).
   - `apply_func` aceita `Func` por valor (não `&Func`)?
     Confirmar — se sim, `func.clone()` em chamada.

Output: notas internas + decisões registadas:
- API forma: Option vs obrigatório (sugestão Option).
- Lista de call-sites externos com Engine acessível ou
  não.
- Helper Engine para tests (existe ou criar).
- Visibilidade de `apply_func` confirmada.

**Critério de saída e gate de decisão**:
- Se `apply_func` acessível e Engine construível em tests:
  prosseguir.
- Se `apply_func` é `pub(crate)` em crate diferente:
  ajustar visibilidade ou expor wrapper. Cláusula gate
  trivial.
- Se construir Engine em tests é trabalho substancial:
  gate substancial. Reabrir.
- Senão, prosseguir.

### .B Modificar `from_tags` para aceitar Engine + EvalContext

1. L0 `00_nucleo/prompts/rules/introspect/from_tags.md`:
   - Documentar nova assinatura:
     ```rust
     pub fn from_tags(
         tags: &[Tag],
         engine: Option<&mut Engine>,
         ctx: Option<&mut EvalContext>,
     ) -> TagIntrospector;
     ```
   - Comportamento:
     - `engine.is_none()` ou `ctx.is_none()`:
       `StateUpdate::Func` ignorada (defensive — coerente
       com P171 padrão).
     - Ambos `Some`: ao processar
       `ElementPayload::StateUpdate { key, update: Func(fn) }`,
       chamar `apply_func` com valor actual.
   - Documentar fluxo eval:
     - Consultar `state_registry.value_at(key, location)`.
     - Se `None`: defensive ignore.
     - Se `Some(curr)`: construir `Args` com `curr`; chamar
       `apply_func`; em `Ok(new_value)` → `update`; em
       `Err(_)` → defensive ignore (refino futuro:
       diagnostics).

2. L1 `01_core/src/rules/introspect/from_tags.rs`:
   - Modificar signatura.
   - Match expandido sobre `update`:
     ```rust
     match update {
         StateUpdate::Set(value) => {
             state_registry.update(key.clone(), (**value).clone(), *loc);
         }
         StateUpdate::Func(fn_value) => {
             if let (Some(eng), Some(c)) = (engine.as_deref_mut(), ctx.as_deref_mut()) {
                 if let Some(curr) = state_registry.value_at(key, *loc).cloned() {
                     let args = Args::positional(curr);
                     match apply_func(fn_value.clone(), args, c, eng) {
                         Ok(new_value) => {
                             state_registry.update(key.clone(), new_value, *loc);
                         }
                         Err(_) => {
                             // defensive ignore — refino futuro
                         }
                     }
                 }
                 // sem init prévio — defensive ignore (P171 padrão)
             }
             // engine/ctx ausentes — defensive ignore
         }
     }
     ```
   - Adaptar nomes exactos conforme `.A.3` (`Args::positional`
     ou helper criado).

3. **REMOVER test stub**:
   - `func_variant_e_silenciosamente_ignorada_em_from_tags`
     em `from_tags.rs::tests` (ou onde estiver). Test
     codifica invariante incorrecto. Remover.
   - Verificar que outros tests do stub se mantêm (tests
     P172 que apenas verificam construção do variant,
     não comportamento eval, continuam válidos).

4. Tests novos em `from_tags::tests`:
   - **`func_eval_aplica_callback_com_engine`**: state init
     0 + StateUpdate::Func que adiciona 1, com Engine
     fornecido → final value 1.
   - **`func_eval_sem_init_e_defensive_ignore`**: sem init
     prévio + Func update → registry inalterado.
   - **`func_eval_sem_engine_e_defensive_ignore`**: state
     init + Func update + `from_tags(_, None, None)` →
     final value = init (Func não aplicada).
   - **`func_eval_sequencia_aplica_em_ordem`**: state init
     0 + duas Funcs (`x => x + 1`, `x => x * 10`) → final
     value 10 (não 20: `(0+1)*10`).

**Critério de saída**:
- `cargo check` passa.
- 4 tests novos passam.
- Test stub removido.
- Linter passa.

### .C Modificar `introspect_with_introspector` para propagar

1. L1 `01_core/src/rules/introspect.rs`:

   Antes (estado pós-P172):
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

   Depois:
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
   Engine. Adapta para nova signatura:
   ```rust
   pub fn introspect(content: &Content) -> CounterStateLegacy {
       let (state, _) = introspect_with_introspector(content, None, None);
       state
   }
   ```
   Funcs em `state.update` aplicadas via API legacy ficam
   ignoradas defensively. Comportamento documentado.

3. Update L0 `00_nucleo/prompts/rules/introspect.md`:
   - Documentar nova assinatura.
   - Documentar comportamento defensivo de `introspect()`
     legacy.

**Critério de saída**:
- `cargo check` passa em 01_core.
- Tests internos passam.

### .D Adaptar call-sites

1. Para cada call-site identificado em `.A.5`:

   **Tests** (maioria):
   - Mudar `introspect_with_introspector(&content)` para
     `introspect_with_introspector(&content, None, None)`.
   - Maioria dos tests não usa Funcs — passar `None, None`
     é correcto.

   **Production em `03_infra`**:
   - Se Engine está acessível no escopo:
     `introspect_with_introspector(&content, Some(&mut engine), Some(&mut ctx))`.
   - Se Engine NÃO está acessível: cláusula gate trivial.
     Refactor caller para receber Engine, ou passar `None`
     com decisão documentada (Funcs ignoradas neste path).

2. `grep` final: confirmar zero call-sites com signatura
   antiga.

3. **Identificar e adaptar `Layouter::layout_with_introspector`**:
   - P168 criou este entry point. Confirmar se chama
     `introspect_with_introspector`.
   - Se sim: adaptar para passar Engine se disponível, ou
     `None`.

**Critério de saída**:
- `cargo check --workspace` passa.
- Todos os tests existentes passam.
- Linter passa.

### .E Tests E2E

1. **Helper Engine para tests** (criado em `.A.6` se ausente):
   - `fn make_test_engine() -> Engine` minimal mas funcional
     para `apply_func` com Funcs simples.

2. Tests E2E em local apropriado (`rules/introspect.rs` ou
   módulo dedicado):

   - **`p173_func_callback_e2e`**: documento com
     `state("counter", 0)` + heading + `state_update_with("counter", x => x + 1)` + heading + `state_update_with("counter", x => x * 10)`.
     Construir Engine via helper. Chamar
     `introspect_with_introspector(&content, Some(&mut engine), Some(&mut ctx))`.
     - `introspector.state_value("counter", loc_h1)` →
       `Some(Value::Int(0))`.
     - `introspector.state_value("counter", loc_h2)` →
       `Some(Value::Int(1))`.
     - `introspector.state_final_value("counter")` →
       `Some(Value::Int(10))`.

   - **`p173_func_legacy_introspect_ignora`**: mesmo
     documento mas chamada via `introspect()` legacy.
     State `numbering_active` ou similar inalterado por
     Funcs. Final value reflecte só init.

   - **`p173_determinismo_func_eval`**: dois chamadas a
     `introspect_with_introspector` sobre mesmo Content +
     mesmo Engine produzem mesmo resultado.

   - **`p173_state_invisivel_em_layout`**: regressão P171.

**Critério de saída**:
- 4 tests E2E passam.
- Linter passa.

### .F Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace` — todos os tests passam.
   Δ vs P172 baseline (1640).
   Δ esperado: +7 a +10 (tests novos em `from_tags` e E2E,
   menos test stub removido).
3. `crystalline-lint`: zero violations.
4. `from_tags` aceita `Option<&mut Engine>` + `Option<&mut EvalContext>`.
5. `introspect_with_introspector` propaga.
6. `introspect()` legacy preservada (comportamento
   defensivo).
7. **Test stub `func_variant_e_silenciosamente_ignorada_em_from_tags` REMOVIDO**.
8. Eval real verificado por test E2E
   `p173_func_callback_e2e` com valores concretos.
9. Walk em `introspect.rs::walk` **NÃO modificado** —
   continua puro.
10. Determinismo verificado.
11. Snapshot tests de paridade ADR-0033 passam inalterados.
12. Linter passa em verificação final.

### .G Encerramento

Escrever
`00_nucleo/materialization/typst-passo-173-relatorio.md` com:

- Resumo: cascade Engine implementada; eval real
  funcionando; pendência P171/P172 fechada.
- Confirmação de cada verificação `.F`.
- Hashes finais de L0s modificados.
- Decisões registadas em `.A`:
  - API form (Option escolhida).
  - Helper Engine criado ou existente.
  - Call-sites adaptados (lista).
- Δ tests vs baseline P172.
- **Estado de M9**: 4/11 features completas (P172 stub
  fica retroactivamente como parcial; P173 completa).
- **Pendência fechada**: `Func` callback em `StateUpdate`
  via Engine cascade.
- Pendências cumulativas + actualização.
- Estado pós-passo: P173 concluído. P174 desbloqueado.

---

## Critério de conclusão

Todas em conjunto:

1. `.A` produziu inventário sem disparar gate substancial.
2. `from_tags` aceita Engine + EvalContext.
3. `introspect_with_introspector` propaga.
4. `introspect()` legacy preservada (defensive ignore).
5. Walk **NÃO modificado**.
6. Eval real verificado com valores concretos:
   `state_update_with("k", x => x + 1)` com init 0 produz
   final value 1.
7. Test stub removido.
8. Verificações `.F` 1-12 passam.
9. Relatório `.G` escrito.
10. Output observable não muda.
11. M9 4/11 features completas (P172+P173 conjunto fecha
    feature `state_update_with(key, fn)`).
12. Pendência "Func callback em StateUpdate" fechada.

---

## O que pode sair errado

- **`apply_func` em crate diferente com visibilidade
  insuficiente**: cláusula gate trivial. Ajustar
  visibilidade ou expor wrapper público em crate de
  introspect.
- **`Args::positional` ou equivalente não existe**: criar
  helper minimal. Documentar.
- **Engine em tests pesado de construir**: gate
  substancial possível. Resolução: `MockEngine` minimal
  que satisfaz `apply_func` para Funcs simples.
- **`Func::call` requer args específicos
  (kwargs/spread/etc.)**: provavelmente `Args::positional`
  cobre caso simples. Para Funcs com signaturas complexas:
  adiar ou simplificar tests.
- **Erro de Func eval sem destino claro**: actualmente
  ignorado defensively. Pendência: infraestrutura de
  diagnostics. Adiar para refino.
- **Determinismo de Func falha**: vanilla proíbe Funcs com
  side effects. Documentar restrição em L0 `state_update.md`.
- **Test stub remoção quebra outros tests**: improvável,
  mas verificar. Se algum outro test depende do stub
  comportamento, investigar.
- **Call-sites externos sem Engine acessível**: passar
  `None, None` é fallback aceitável (Funcs ignoradas).
  Documentar como decisão local em `.D`.
- **Linter divergência com vários ficheiros tocados**:
  ajustar conforme erro reportado.

---

## Notas operacionais

- **Tamanho**: M-L. Menor que P172 spec original porque
  variant + stdlib já existem. Trabalho concentra-se em
  cascade + eval + cleanup.
- **Pré-condição P174**: feature seguinte M9. Engine
  agora disponível em `from_tags` — features futuras
  (`query()`, `here()`, `counter.at`) podem usar mesma
  cascade. Investimento P173 paga aqui.
- **Cláusula gate trivial**: aplicável a decisões locais
  (forma de Args, helper Engine, visibility de
  `apply_func`).
- **Tests do stub removidos explicitamente**: razão
  documentada — codificavam invariante incorrecto.
  Continuar a aceitar codificaria comportamento errado
  como esperado.
- **Comportamento defensivo legacy**: `introspect()` sem
  Engine continua a funcionar; Funcs ignoradas. Este
  comportamento é intencional, não bug. Documentar no
  L0 para evitar futura confusão.
- **Pendência fecha agora genuinamente**: P172 deixou
  pendência aberta (apenas mudou nome). P173 fecha eval
  real. Lista cumulativa reduz por 1.
- **Trabalho NÃO duplica P172**: variant `Func`, stdlib
  `state_update_with`, e tests do construtor já existem.
  P173 só toca em `from_tags`, `introspect_with_introspector`,
  call-sites, e tests de eval.
