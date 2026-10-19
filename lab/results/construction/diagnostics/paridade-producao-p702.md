# Paridade Produção — P702 — `Func::with(...)` (aplicação parcial de argumentos)

**Data:** 2026-07-11
**Passo:** `00_nucleo/materialization/typst-passo-702.md`
**Hash do commit (implementação):** `7b6c3c415`.
**HEAD base:** `760101de7` (fim de P701, detached HEAD).
**Estado:** ÂMBITO DE P702 FECHADO — `.with()` implementado, testado, paridade vanilla confirmada (incluindo o caso medido a pedido, `f.with(...).sub_func`). `cetz` progride bem mais longe (chega a invocar o plugin WASM real), mas bloqueia num gap novo e não relacionado (`rgb()` com 1 argumento string/hex).

---

## 1. Proveniência das medições

- Working tree no momento da medição: ficheiros de código+L0 de P702 (ver
  §4) + este relatório. `cargo test --workspace` corrido antes (herdado de
  P701: 3736 passed) e depois (3747 passed — +11 testes novos) da
  implementação.
- `~/.cache/typst/packages/preview/cetz/0.5.2` idêntico às sessões
  anteriores.

---

## 2. Sonda — confirmado contra o vanilla, com `file:line`

- **Estrutura interna**: `foundations/func.rs:149-159` (`FuncInner::With(Arc<(Func, Args)>)`),
  `:359-362` (fusão na chamada: `args.items = pre.items.chain(new.items)`),
  `:380-394` (`#[scope] impl Func { pub fn with(...) }`).
- **Comportamento confirmado com documento `.typ` real** (`lab/typst-original`):
  - `calc.round.with(digits: 2)(3.14159)` → `3.14`.
  - Closure posicional: `g(a,b,c)=a+b+c; g.with(1,2)(3)` → `6`.
  - Closure nomeado: `h(a,named:10)=a+named; h.with(named:20)(5)` → `25`.
  - Encadeamento posicional: `f.with(1).with(2)(3)` → `6`.
  - Encadeamento nomeado: `h.with(x:100).with(y:200)(1)` → `301`.
  - Named num parâmetro posicional falha — `f.with(a: 1)` onde `f(a,b,c)`
    são todos posicionais → `error: the argument 'a' is positional`
    (comportamento correcto do binding de parâmetros, não de `.with()`).
- **Estado cristalino antes do fix**: `error: esta função não tem campos`
  em todos os casos acima (reprodução exacta do bloqueio isolado por P701).
- **Medição extra pedida explicitamente** (não estava nos critérios originais
  do passo, mas o utilizador pediu para confirmar antes de decidir
  scope-out): `table.with(columns: 2).cell` — sub-função de namespace
  acedida através de uma função parcialmente aplicada. **Vanilla suporta**
  (`type(t)` e `type(t.cell)` → `function`, `function`), confirmado por
  `foundations/func.rs:269-277` (`Func::scope()`, equivalente vanilla de
  `namespace()`): braço explícito `FuncInner::With(with) => with.0.scope()`.
  **Decisão corrigida a partir desta medição**: `namespace()` delega através
  de `With` no cristalino (não fica `None` como um scope-out não verificado
  teria ficado).

---

## 3. Implementação

### 3.1 `FuncRepr::With(Arc<(Func, Args)>)`

Nova variante em `01_core/src/entities/func.rs`. `Func::with(self, args)`
constrói `Func(Arc::new(FuncRepr::With(Arc::new((self, args)))))`.

### 3.2 Delegação nos métodos exaustivos de `Func`

- `name()` ⇒ `w.0.name()` (delega, paridade vanilla).
- `native_fn_addr()` ⇒ `None` (não é fn-ptr directo).
- `namespace()` ⇒ `w.0.namespace()` (delega — medido, §2 acima; **não**
  ficou no wildcard `_ => None` porque a medição confirmou que devia
  delegar).

### 3.3 Fusão de `Args` e despacho (`apply_func`, `rules/eval/closures.rs`)

Novo braço `FuncRepr::With(w) => apply_func(inner.clone(), merge_with_args(pre, args), ...)`
— recursivo, resolve encadeamento sem lógica extra. `merge_with_args`:
posicionais `pre.items ++ new.items` (paridade vanilla exacta); nomeados
`pre.named` sobreposto por `new.named` (decisão por defeito, não exercitada
pelo vanilla — documentada em `entities/func.md`).

### 3.4 Intercepção sintáctica (`eval_func_call`, `rules/eval/closures.rs`)

Novo bloco, mesmo padrão de P417 (`where`)/P423 (`or`/`and`)/P504
(`within`)/P466 (métodos de colecção)/P506 (`state`/`counter`): se o callee
é `FieldAccess` com campo `"with"` e o alvo avalia para `Value::Func`,
devolve `Value::Func(target.with(args))` sem invocar. Não intercepta para
não-`Func` — confirmado com teste de regressão (dict com chave `"with"`
continua a funcionar por field access normal).

### 3.5 Testes (11 novos)

- `01_core/src/entities/func.rs` (4): delegação de nome, `native_fn_addr`
  `None`, delegação de namespace (reproduz `table.with(...).cell` ao nível
  de `Func`), encadeamento preserva a função original.
- `01_core/src/engine/eval/tests.rs` (7): os 5 casos da sonda (nativa+named,
  closure posicional, closure nomeado, 2 encadeamentos) + sub-função via
  namespace através de `with` + regressão dict-com-chave-"with".

---

## 4. Ficheiros tocados

- **L0**: `00_nucleo/prompts/entities/func.md` (nova variante `With` +
  interface + testes canónicos + scope-out de colisão de nomeados),
  `00_nucleo/prompts/engine/eval.md` (§P702, intercepção em `eval_func_call`).
- **Código com lógica nova**: `01_core/src/entities/func.rs`,
  `01_core/src/engine/eval/closures.rs`, `01_core/src/engine/eval/tests.rs`.
- **Só hash realinhado** (`crystalline-lint --fix-hashes`, mesma L0
  partilhada `rules/eval.md`): `bibliography.rs`, `control_flow.rs`,
  `flow.rs`, `markup.rs`, `math.rs`, `mod.rs`, `modules.rs`, `rules.rs` —
  sem mudança de lógica nestes 8 ficheiros.

---

## 5. Validação — âmbito de P702 confirmado

Reexecutados os 3 documentos `.typ` da sonda (release build):

- `calc.round.with(digits:2)(3.14159)` / closure posicional / closure
  nomeado → `3.14`, `6`, `25` — **idêntico ao vanilla**.
- Encadeamento posicional / nomeado → `6`, `301` — **idêntico ao vanilla**.
- `type(table.with(columns:2))` / `type(table.with(columns:2).cell)` →
  `function`, `function` — **idêntico ao vanilla**.

- `cargo test --workspace` → **3747 passed**, 0 failed (3736 de P701 + 11
  novos de P702); `typst-infra` inalterado (626/5).
- `crystalline-lint .` → 0 violations (11 ficheiros com drift V5 realinhados
  de uma vez — `--fix-hashes` cobre todos os ficheiros que partilham a L0
  editada).

---

## 6. Repetição da reprodução de P700/P701 — progresso real, novo bloqueio

```
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
```

**Progresso mensurável**: a compilação agora demora **~7.3s** (antes: falha
instantânea em `matrix.typ:8`) — sinal de que o eval avança muito mais
fundo na árvore de `cetz`, incluindo, provavelmente, invocações reais ao
plugin WASM (`cetz_core.wasm`) via `call_wasm`/`cbor.encode` (P701).

**Novo bloqueio**: `error: rgb() requer 3 ou 4 Int, recebeu 1 args`. Isolado
a `src/lib/palette.typ` (paletes de cor pré-definidas de `cetz`):
```
"cc0000", "d3d7cf", "555753").map(rgb)
...
range(90, 40, step: -12).map(v => luma(v * 1%))
```
`.map(rgb)` chama `rgb("cc0000")`/`rgb("#FF0000")` — **forma de 1 argumento
string/hex**, que `native_rgb` (`01_core/src/engine/stdlib/foundations.rs`)
não suporta (só aceita 3 ou 4 `Int`). Confirmado standalone:
```
#rgb("#FF0000")
```
→ vanilla: exit 0. Cristalino: `error: rgb() requer 3 ou 4 Int, recebeu 1
args`.

**Não é o mesmo bloqueio de P700/P701** — `.with()` está confirmadamente
resolvido (§5); este é um gap novo e não relacionado, num construtor de cor
diferente. Mesma disciplina: registar, não forçar correcção improvisada fora
do âmbito de P702.

### Próximo passo sugerido (P703, não iniciado)

1. Confirmar no vanilla as formas de `rgb()` além de 3/4 Int — pelo menos
   hex string (`rgb("#RRGGBB")`, `rgb("RRGGBBAA")` sem `#`?) e possivelmente
   grayscale de 1 componente.
2. L0 para `native_rgb` (`00_nucleo/prompts/engine/stdlib/foundations.md`,
   já tem secção própria — precisa de extensão, não de ficheiro novo).
3. Reexecutar a reprodução deste §6 como critério de fecho; se `cetz`
   avançar mais, repetir de novo (mesmo padrão desta cadeia).

---

## 7. Estado da cadeia P678–702

Progresso real: plugin WASM real (P699/P699b/P700), `cbor.encode`/`cbor(bytes)`
(P701), e agora `.with()` (P702) — todos fechados e testados. `cetz` ainda
não renderiza, mas a compilação chega visivelmente mais longe a cada passo
(tempo de execução ~7.3s nesta sessão vs. falha instantânea antes). Pausada
aqui, com P703 sugerido e a causa exacta já isolada
(`palette.typ`, `rgb()` de 1 argumento).

## 8. Critério de fecho do passo

- [x] Sonda completa, estrutura e comportamento de `.with()` confirmados
      contra o vanilla (`file:line`), incluindo o caso extra medido a
      pedido (`f.with(...).sub_func`).
- [x] Implementado para nativas (com/sem namespace), closures, e
      encadeamento — testado.
- [x] Mecanismo já existente (`it.with(...)` em elementos, distinto) sem
      regressão — não tocado, mecanismo diferente.
- [x] `cetz` re-testado — **progresso real, próximo bloqueio identificado e
      isolado** (`rgb()` 1-arg), não sucesso completo.
- [x] Sem regressão em `cargo test --workspace` (3747 passed).
- [x] `crystalline-lint .` limpo.
- [x] Relatório com resultado exacto (este ficheiro).
