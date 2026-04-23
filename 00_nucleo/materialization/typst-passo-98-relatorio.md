# Passo 98 — Relatório de encerramento (ADR-0036 para `EvalContext`)

**Data**: 2026-04-23
**Precondição**: Passo 97 encerrado, DEBT-47 fechado, 764 L1 + 174 L3 testes,
zero violations.
**ADR aplicável**: ADR-0036 (atomização progressiva, Regra 1 — dependências
declaradas na assinatura).

---

## Sumário

`current_file` e `figure_numbering` removidos de `EvalContext`. Ambos
passam agora como parâmetros explícitos nas ~27 funções `eval_*` e
no ABI de `NativeFunc`. **Encerramento da aplicação da ADR-0036 a
`EvalContext`** — a struct tem exactamente 4 campos, todos Regra 4
com comentário justificativo.

Zero regressão funcional: **764 L1 + 174 L3 + 6 ignorados** inalterados.
`crystalline-lint .` → zero violations.

---

## 98.A — Inventário

Inventário detalhado em
`00_nucleo/diagnosticos/inventario-evalcontext-passo-98.md`.

### Leitores directos, escritores, cadeia transitiva

| Campo | LD | W | LT | Profundidade |
|-------|---:|--:|---:|-------------:|
| `current_file` | 2 | 1 padrão save/set/restore | ~25 | 4 níveis |
| `figure_numbering` | 2 | 1 site único | ~25 | 4 níveis |

Ambos bem abaixo dos gates (40 LTs, 6 níveis) → **Opção A** (extracção
individual) viável.

### Tipo da origem de `figure_numbering`

- Origem: `Value::Str(s)` onde `s: EcoString` (do AST).
- Lifetime disponível: nenhum (owned).
- **Decisão**: `&mut Option<String>` nas funções `eval_*` (permite
  read/write), `Option<&str>` via `.as_deref()` no ABI de `NativeFunc`
  (read-only em native functions).

---

## 98.B + 98.C — Execução

### Assinatura antes/depois (exemplo `eval_markup`)

**Antes**:

```rust
fn eval_markup<'r>(
    node: &SyntaxNode,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext<'_>,
    route: Tracked<'r, Route<'r>>,
    styles: &mut StyleChain,
    show_rules: &mut Arc<[ShowRule]>,
    active_guards: &mut Vec<RuleId>,
) -> SourceResult<Value>
```

**Depois**:

```rust
fn eval_markup<'r>(
    node: &SyntaxNode,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext<'_>,
    route: Tracked<'r, Route<'r>>,
    styles: &mut StyleChain,
    show_rules: &mut Arc<[ShowRule]>,
    active_guards: &mut Vec<RuleId>,
    current_file: FileId,
    figure_numbering: &mut Option<String>,
) -> SourceResult<Value>
```

### Contagem de parâmetros

- Antes: 6 parâmetros explícitos além de `ctx` → 7 total.
- Depois: 8 parâmetros explícitos além de `ctx` → **9 total**.
- Crescimento previsto pelo spec: de 6 → 8 (spec subestima por 1;
  o 7º era `ctx`).

### NativeFunc ABI

**Antes**:

```rust
pub call: fn(&mut EvalContext<'_>, &Args) -> SourceResult<Value>
```

**Depois**:

```rust
pub call: fn(
    &mut EvalContext<'_>,
    &Args,
    FileId,              // current_file
    Option<&str>,        // figure_numbering
) -> SourceResult<Value>
```

56 funções nativas em `stdlib/` actualizadas — maioria com
`_current_file` e `_figure_numbering` (ignorados). `native_image`
usa `current_file` para `ctx.world.read_bytes`. `native_figure` usa
`figure_numbering` para propagar a numeração activa no `Content::Figure`.

### Padrão save/set/restore eliminado

**Antes** (em `eval_module_include`):

```rust
let saved_file = ctx.current_file;
ctx.current_file = src_id;
let result = eval_markup(...);
ctx.current_file = saved_file;
result
```

**Depois**:

```rust
let child_current_file = src_id;
eval_markup(..., child_current_file, figure_numbering)
```

Elimina o bug latente: se `eval_markup` retornasse `Err` via `?` (no
padrão antigo, isto não acontecia por pura sorte — os statements são
puros assignments, não `?`-expressions — mas o padrão era frágil), o
`ctx.current_file` ficaria corrompido. No padrão novo, o `current_file`
do chamador nunca é tocado.

### Write de `figure_numbering`

**Antes** (em `eval_set_rule`, caso `target == "figure"`):

```rust
ctx.figure_numbering = new_numbering.clone();
```

**Depois**:

```rust
*figure_numbering = new_numbering.clone();
```

O `&mut Option<String>` propaga-se para cima; callers de `eval_set_rule`
vêem a mutação na sua própria variável local.

---

## 98.D — Verificação estrutural

```bash
$ grep -rn "ctx\.current_file\|ctx\.figure_numbering\|self\.current_file\|self\.figure_numbering" 01_core/src/
(zero matches)

$ grep -n "pub.*:" 01_core/src/rules/eval/mod.rs | head
pub struct EvalContext<'w> {
    pub world: &'w dyn World,
    pub loop_iterations: usize,
    pub max_loop_iterations: usize,
    pub next_rule_id: RuleId,
}
```

Exactamente 4 campos, todos Regra 4 com comentário.

### Testes

- `cargo test --workspace`: **764 L1 + 174 L3 + 6 ignorados** ✓
- `crystalline-lint .`: **zero violations** ✓

---

## Nota sobre o fecho da ADR-0036 para `EvalContext`

A `EvalContext` agora contém **apenas** campos que cumprem a Regra 4
(dados globais ao eval, independentes do fluxo de controlo):

| Campo | Regra 4 |
|-------|---------|
| `world: &'w dyn World` | Handle externo — fonte de I/O |
| `loop_iterations: usize` | Contador monotónico anti-loop-bomb |
| `max_loop_iterations: usize` | Limite estático de configuração |
| `next_rule_id: RuleId` | Alocador monotónico de IDs de ShowRule |

Histórico da atomização:

| Passo | O que saiu do contexto | Como passou a ser passado |
|-------|-----------------------|---------------------------|
| 92 | `route` (ex-`Vec<FileId>`) | `Tracked<'r, Route<'r>>` |
| 94 | `styles` (ex-`StyleChain`) | `&mut StyleChain` |
| 95 | `show_rules` + `active_guards` | `&mut Arc<[ShowRule]>` + `&mut Vec<RuleId>` |
| **98** | **`current_file` + `figure_numbering`** | **`FileId` + `&mut Option<String>`** |

ADR-0036 aplicada end-to-end ao `EvalContext`. Próximos trabalhos
candidatos de atomização (fora do escopo deste passo) ficam reservados
para materialização futura de `Engine<'a>` (agregador vanilla).

---

## Observações

### Contagem de parâmetros crescente

9 parâmetros nas funções `eval_*` (além de `ctx`) é, em absoluto, muito.
Em termos ergonómicos, a função assinatura é longa:

```rust
eval_markup(root, &mut scopes, &mut ctx, route.track(),
            &mut styles, &mut show_rules, &mut active_guards,
            current_file, &mut figure_numbering)
```

Cada call site repete as 9 variáveis. Isto valida o mérito futuro de um
struct agregador — o candidato "Materializar `Engine<'a>`" do relatório
de continuidade ganha agora evidência empírica: **a ADR-0036 atomizou
até ao limite, e o próximo passo lógico é reagrupar num vehicle
estrutural (não num super-contexto mutável)**.

### ABI de NativeFunc crescente

Passar 2 novos parâmetros por todas as funções nativas (mesmo as que
não precisam) é ruído. Alternativas consideradas:

- **Agregador transiente** (`NativeCallContext`) que agrupe os
  parâmetros — adiado para futura materialização `Engine<'a>`.
- **Variantes de NativeFunc** (com vs. sem acesso a file/numbering) —
  complica enum + dispatch; descartado.

Por agora, os 54 `_current_file` / `_figure_numbering` prefixados
custam apenas 2 identificadores por assinatura — aceitável até a
materialização trazer o vehicle.

### Nenhum DEBT novo aberto

Nenhum call site revelou acoplamento implícito impossível de resolver
neste passo. A extracção foi mecânica (apesar de extensa em número de
sites tocados).

---

## Decisão

**ADR-0036 aplicada a `EvalContext` — ENCERRADA (Passo 98).**

`EvalContext` tem 4 campos, todos Regra 4 com comentário justificativo.
Zero `ctx.current_file`, `ctx.figure_numbering`, `self.current_file`,
`self.figure_numbering` no workspace.

Próximos candidatos de trabalho:

1. **Materialização de `Engine<'a>`** — agrupar `current_file`,
   `figure_numbering`, `route`, `styles`, `show_rules`, `active_guards`
   num vehicle estrutural passável como unidade. Agora com evidência
   empírica de ergonomia.
2. **Materializar dependências folha** (`Style`, `LazyHash`,
   `Introspection`) — desbloqueia `Styles` e `Sink`.
3. **Materializar `Traced` completo** — stub do world_types hoje, sem
   uso funcional.
