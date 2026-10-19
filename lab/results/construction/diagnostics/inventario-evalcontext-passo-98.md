# Inventário `EvalContext.current_file` e `EvalContext.figure_numbering` — Passo 98.A

Data: 2026-04-23.

---

## `current_file: FileId`

### Leitores directos (LD)

| Ficheiro | Linha | Função | Uso |
|----------|------:|--------|-----|
| `rules/eval/modules.rs` | 57 | `eval_module_include` | `ctx.world.include_source(ctx.current_file, &path)` |
| `rules/stdlib/figure_image.rs` | 93 | `native_image` | `ctx.world.read_bytes(ctx.current_file, &path)` |

**Total LD**: 2 sites, 2 funções, 2 ficheiros.

### Escritores (W)

| Ficheiro | Linha | Função | Escopo |
|----------|------:|--------|--------|
| `rules/eval/modules.rs` | 78-79 | `eval_module_include` | `save_cur = ctx.current_file; ctx.current_file = src_id;` |
| `rules/eval/modules.rs` | 81 | `eval_module_include` | `ctx.current_file = saved_file;` (restore) |

**Total W**: 1 padrão save/set/restore num único ficheiro (eval_module_include).

**Vulnerabilidade**: o restore é `ctx.current_file = saved_file;` **depois** de `let result = eval_markup(...);` — se `eval_markup` retornar `Err` via `?`, o `ctx.current_file` **não fica corrompido** neste caso porque não há `?` entre save e restore (ambos são assignments puros). Mesmo assim, o padrão novo (variável local) elimina qualquer risco equivalente.

### Leitores transitivos (LT)

Qualquer `eval_*` que chame `apply_func` (todos os que avaliam funções) ou
`eval_module_include` torna-se LT. Como `eval_expr` despacha para quase
todos os braços, a cadeia é larga.

- `apply_func` (closures.rs) — dispatcher de Func → **chama NativeFunc.call** → `native_image` lê.
- `eval_func_call` (closures.rs) — chama `apply_func`.
- `eval_expr` (mod.rs) — dispatcher central, chama `eval_func_call` no arm `FuncCall`.
- `eval_markup`, `eval_code`, `eval_math`, `eval_expr_item`, etc. — consumidores do dispatcher.

**LT estimado**: ~25 funções `eval_*`. Abaixo do gate de 40.

### Profundidade máxima da cadeia

`eval_expr` → `eval_func_call` → `apply_func` → `(native_image.call)(ctx, ...)` → `ctx.world.read_bytes(current_file, path)` = **4 níveis**.

Também: `eval_expr` → `eval_module_include` → `include_source(current_file, path)` = **3 níveis**.

**Máximo: 4 níveis**, abaixo do gate de 6.

---

## `figure_numbering: Option<String>`

### Leitores directos (LD)

| Ficheiro | Linha | Função | Uso |
|----------|------:|--------|-----|
| `rules/eval/rules.rs` | 227 | `eval_set_rule` | `let mut new_numbering = ctx.figure_numbering.clone();` |
| `rules/stdlib/figure_image.rs` | 53 | `native_figure` | `let numbering = ctx.figure_numbering.clone();` |

**Total LD**: 2 sites, 2 funções, 2 ficheiros.

### Escritores (W)

| Ficheiro | Linha | Função | Escopo |
|----------|------:|--------|--------|
| `rules/eval/rules.rs` | 240 | `eval_set_rule` | `ctx.figure_numbering = new_numbering.clone();` (após `#set figure(numbering: ...)`) |

**Total W**: 1 site único em eval_set_rule (caso `target == "figure"` do dispatcher de set rules).

### Leitores transitivos (LT)

Mesma estrutura que `current_file` — qualquer `eval_*` que chame `apply_func`
(para atingir `native_figure`) ou `eval_set_rule` torna-se LT.

**LT estimado**: ~25 funções, igual a `current_file`.

### Profundidade máxima da cadeia

- `eval_expr` → `eval_func_call` → `apply_func` → `(native_figure.call)(...)` = **4 níveis** (leitor).
- `eval_expr` → `eval_set_rule` = **2 níveis** (leitor + escritor).

**Máximo: 4 níveis**, abaixo do gate de 6.

### Tipo da origem

- Origem do valor: `Value::Str(s)` onde `s: EcoString` (proveniente do AST
  via parsing de `#set figure(numbering: "1")`).
- Conversão actual: `s.to_string()` → `Option<String>` owned.
- **Lifetime disponível na origem**: nenhum (EcoString é owned, sem `'w` nem
  `'static` directo).

### Decisão de passagem

- **`figure_numbering: &mut Option<String>`** para funções `eval_*` que
  precisam de ler **ou** escrever. `eval_set_rule` é o único W — modifica
  via `*figure_numbering = new_numbering`.
- **`figure_numbering: Option<&str>`** para `native_figure` (read-only), via
  `.as_deref()` no call site do `apply_func`.
- O clone de `String` curta de numeração (tipicamente `"1"` ou `"a"`) é
  desprezável — a operação não está em caminho quente.

---

## Constraint: ABI das funções nativas (`NativeFunc.call`)

Actualmente:

```rust
pub call: fn(&mut EvalContext<'_>, &Args) -> SourceResult<Value>
```

Se removermos os dois campos de `EvalContext`, `native_image` e `native_figure`
perdem o acesso. Como o ABI é consumido por **56 funções nativas** em
`stdlib/`, mudá-lo é invasivo mas mecânico.

**Decisão**: adicionar os dois novos parâmetros ao ABI:

```rust
pub call: fn(
    &mut EvalContext<'_>,
    &Args,
    FileId,              // current_file
    Option<&str>,        // figure_numbering
) -> SourceResult<Value>
```

- Para `native_image` e `native_figure`: usam os novos params.
- Para as 54 funções restantes: `_current_file: FileId, _figure_numbering: Option<&str>` (ignoradas).

O call site único (`apply_func` em `eval/closures.rs`) passa os valores
recebidos das funções `eval_*`.

---

## Gate de decisão

- LT current_file: ~25 funções — **≤ 40 OK**.
- LT figure_numbering: ~25 funções — **≤ 40 OK**.
- Profundidade: 4 — **≤ 6 OK**.
- Tipo figure_numbering: owned (`Option<String>`) sem lifetime — passagem
  mutável `&mut Option<String>` para escrita, immutable `Option<&str>` para
  native read-only.

**Decisão: prosseguir com Opção A** (extracção individual de ambos os campos).

**Trabalho extra não previsto pelo spec**: modificar ABI das funções nativas
(mudança mecânica em 56 funções). Registado como observação operacional, não
como DEBT — é consequência directa da extracção.
