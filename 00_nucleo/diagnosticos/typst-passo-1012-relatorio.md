# Passo 1012 — Relatório final

**Data**: 2026-08-12  
**Commit de base**: `358434c52` (P1010/P1011: adicionar relatórios de execução)  
**Ficheiros alterados**:

- `01_core/src/compiler/eval/mod.rs` — declaração do novo submódulo `call_dispatch`
- `01_core/src/compiler/eval/closures.rs` — reduzido a criação/ aplicação de closures
- `01_core/src/compiler/eval/call_dispatch.rs` — **novo nó** com dispatch de chamadas
- `00_nucleo/prompts/compiler/eval/closures.md` — **L0 do nó remodelado**
- `00_nucleo/prompts/compiler/eval/call_dispatch.md` — **L0 do novo nó**
- `00_nucleo/prompts/compiler/eval/font_dict.md` — L0 corrigido (classificação mecânica)
- `00_nucleo/prompts/compiler/eval.md` — secção de submódulos atomizados actualizada
- `01_core/src/compiler/eval/bindings.rs`, `rules.rs`, `introspect/from_tags.rs` — imports de `apply_func`/`eval_args`/`eval_func_call` actualizados de `closures` para `call_dispatch`
- `01_core/src/compiler/stdlib/collections.rs`, `counter.rs`, `numbering.rs`, `state.rs` — imports actualizados de `apply_func`/`eval_args`/`eval_func_call` de `closures` para `call_dispatch`
- Filhos de `eval.md` em `01_core/src/compiler/eval/` — resselo de `@prompt-hash` via `crystalline-lint --fix-hashes`

---

## Resumo

### Parte 1 — `font_dict` é *stateful*, não declarativo

O Passo 1011 classificou `font_dict` como "declarativo" por propósito ("transforma sintaxe em dicionário normalizado"). A verificação mecânica do Critério 2 (P1002) mostra que o nó **não** é `Value → Value` puro: as duas funções de topo recebem `&mut EvalContext` e `&mut Engine` e propagam esses mutáveis para `eval_expr`, que pode avaliar closures/funções com efeito lateral (sink, counters, state, warnings, active_guards).

Pontos de contacto reais com `Engine`/`EvalContext` em `compiler/eval/font_dict.rs`:

- `parse_font_dict_named_fields`: linha 55 — `eval_expr(named.expr(), scopes, ctx, engine)?`
- `parse_font_dict_legacy`: linha 180 — `eval_expr(Expr::FuncCall(call), scopes, ctx, engine)?`
- `parse_font_dict_legacy`: linha 197 — `eval_expr(keyed.expr(), scopes, ctx, engine)?`
- `parse_font_dict_legacy`: linha 206 — `eval_expr(named_item.expr(), scopes, ctx, engine)?`

**Resultado**: **B** — o nó é *stateful*. O L0 de `font_dict.md` foi corrigido para não afirmar "declarativo"; a separação do nó continua sustentada pelo Critério 3 (co-mudança histórica, confirmada no Passo 1009) e pelo propósito de normalização de sintaxe, mas a classificação de pureza é a do resto do hub `rules.rs`.

### Parte 2 — Fatiamento de `compiler/eval/closures.rs`

#### Inventário (Fase A)

Funções públicas antes no monólito `closures.rs`:

- `eval_func_call` — avalia uma chamada sintáctica (`Expr::FuncCall`).
- `eval_args` — avalia a lista de argumentos AST.
- `apply_func` — aplica `Func` (closure/native/plugin/element/with).
- `eval_closure_expr` — constrói um `Value::Func` a partir de uma closure AST.
- `apply_closure` — aplica uma closure a argumentos.
- `trace_call` — adiciona `Tracepoint::Call` a erros quando apropriado.
- `merge_with_args` — funde args pré-ligados por `.with(...)`.
- `call_plugin` — aplica export de plugin WASM.
- `eval_location_method` — resolve métodos de `Value::Location`.

#### Aplicação dos 4 critérios (Fase B)

1. **Isolamento de teste**: `merge_with_args` é pura (`Args`/`Args` → `Args`). `call_plugin` precisa de `PluginFunc`. As restantes precisam de `Engine`/`Scopes`/`EvalContext` reais (dispatch, aplicação de closure, eval de args).
2. **Pureza vs estado**: maioritariamente stateful. `eval_func_call`, `apply_func`, `trace_call`, `call_plugin`, `eval_location_method`, `eval_closure_expr`, `apply_closure` tocam `Engine`/`ctx`. `merge_with_args` é a única pura entre as privadas.
3. **Co-mudança histórica**: o monólito `closures.rs` acumulava interceptações de method calls (P417/P423/P504/P506/P707/P710/P712/P792/P815/P829-B/P846/P685) ao lado da mecânica de closures (P708/P724/P733/P504). As interceptações mudam quando surgem novos métodos especiais; o binding de closures muda quando a semântica de parâmetros/captura evolui. Co-mudança distinta.
4. **Correspondência vanilla**: o cristalino misturava `typst_eval::call` (chamadas gerais + intercepções) e `typst_library::foundations::func` (aplicação de closures). O corte separa essas duas fronteiras: `call_dispatch.rs` fica com o eval/call do vanilla; `closures.rs` fica com a aplicação de closures.

#### Decisão de corte (Fase C)

- **`compiler/eval/call_dispatch.rs`** — dispatch de chamadas: `eval_func_call`, `apply_func`, `eval_args`, `trace_call`, `merge_with_args`, `call_plugin`, `eval_location_method`, e todos os blocos de intercepção de method calls (P417/P423/P504/P506/P707/P710/P712/P792/P829-B/P815/P846/P685).
- **`compiler/eval/closures.rs`** reduzido — criação e aplicação de closures: `eval_closure_expr`, `apply_closure`.

A **Opção B da ADR-0109** foi mantida: o dispatcher fica magro e delega em free functions no arquivo da unidade. Não houve alteração de contratos públicos fora da criação do novo módulo.

---

## Validação

```
cargo test --workspace
```

Resultado:

- typst-core: 4963 passed
- typst-infra: 787 passed
- typst-shell: 41 passed
- benches: 2 passed
- wiring: 37 passed
- crystalline_lint: 2 passed

Total: **5834 tests passed**, 0 failed.

```
crystalline-lint .
```

Zero erros. Dois warnings V7 pré-existentes (`auditar-spec.md`, `package_version_resolution.md`), nada relacionado com esta mudança.

```
crystalline-lint --fix-hashes .
```

Nenhum drift restante.

---

## Notas para passos futuros

- O hub `eval/mod.rs` mantém a coordenação geral do eval; `rules.rs` continua como hub de set/show-rules e realização. `closures.rs` e `call_dispatch.rs` são agora unidades autónomas dentro da camada de eval.
- A classificação mecânica de `font_dict` como *stateful* deve ser usada como referência para futuros nós que recebam `Engine`/`EvalContext`: a presença do contexto mutável não é meramente cosmética — `eval_expr` pode propagar efeitos laterais.
- Próximo candidato da ordem do P1008: `bindings.rs` (já confirmado em P1008/P1010) ou `stdlib::structural`, conforme prioridade definida no passo seguinte.
