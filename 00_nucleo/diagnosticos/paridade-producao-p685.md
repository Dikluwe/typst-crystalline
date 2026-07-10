# P685 — Tipos como valores de primeira classe (`type(x) == length`)

> **Passo:** 685
> **Data:** 2026-07-10
> **Commit base:** `f5f23be49aa15d55df1a4a4cb29217d0fb8a978c` (P684)
> **Commit deste trabalho:** `__P685_COMMIT__` (preenchido no 2.º commit)
> **Hora da medição (UTC):** 2026-07-10T19:08:04Z
> **Vanilla de referência:** `typst 0.15.0 (969087ec)`
> **Estado do repositório no momento da medição:** working tree com os 19 ficheiros listados em §3 modificados (não commitados) + untracked pré-existentes (`materialization/`, `adr/`, `temp_p*/`, `perf.data`) que **não** entraram neste passo.

---

## 1. Resumo executivo

P683 encontrou que `cetz` usa `type(x) == length` — tratando nomes de tipo
(`length`, `ratio`, `int`, …) como **valores** comparáveis directamente. O
cristalino só tinha `type()` como função (a devolver uma **string** com o nome
do tipo); os nomes de tipo não existiam como valores no scope.

Neste passo:

1. **Sonda** mediu na fonte (`lab/typst-original`, 0.15.0) o que `type(x)`
   devolve e o que `#(int)` produz: `type(1)` é o **valor-tipo** `int` (não a
   string `"int"`); `repr(int) = int`; `type(int) = type`; construtores
   (`int`/`float`/`str`/`type`) são **chamáveis**, os restantes tipos não.
2. **Implementação**: nova variante `Value::Type(Type)` (enum `Type` `Copy`);
   `type(x)` passa a devolver `Value::Type(v.type_of())`; os nomes de tipo são
   registados no scope global como `Value::Type`. `int`/`float`/`str`/`type`
   ficam valores-tipo **chamáveis** (despachados para o construtor nativo no
   call-path), preservando `int("5")`, `str(5)`, `int.min`, `str.from-unicode`.
3. **Resultado**: `type(x) == length` (e equivalentes) funciona por igualdade
   directa; sombreamento por `#let length = 5` funciona; paridade de **valores**
   com o vanilla confirmada por diff normalizado.
4. **cetz** avança: o bloqueio de tipos desapareceu; o próximo bloqueio é
   `include: ficheiro não encontrado: /src/process.typ` (caminho absoluto de
   pacote não resolvido) — registado com honestidade, sem assumir resolução.

`cargo test --workspace` **4352 passed / 0 failed**; `crystalline-lint .`
**✓ No violations found**.

---

## 2. Classificação (ADR-0107 / ADR-0108)

Medido **antes** de decidir (sondas §4; `file:line` do código em §6):

- **`type(x)` devolver um valor-tipo** e **`type(x) == length`** — **morfologia /
  semântica da linguagem** (paridade): é a forma do conteúdo enquanto objecto
  da linguagem.
- **Lista de nomes de tipo expostos** e **quais são chamáveis** — **semântica**
  (paridade), medida: `bool(1)`/`array(1,2)`/`dictionary(a:1)` →
  "type X does not have a constructor"; `int("5")`/`str(5)`/`float("3.5")` → OK.
- **Representação interna** (`Value::Type` vs `Value::Str`, `Type` `Copy`,
  `type_name()` para mensagens) — **mecânica** (diverge de propósito, P329).
- **Espaçamento e smart quotes** no PDF de validação (§8) — **mecânica de
  render** (pré-existente, fora de escopo).
- **Mensagem de erro** "type bool does not have a constructor" (cristalino) vs
  "type boolean …" (vanilla) — **observável mecânico** (divergência menor; débito
  §7).

Inferência marcada: assumi que `int`/`float`/`str` devem ser **o mesmo valor**
que `type(1)` para `type(1) == int` ser `true`. Refutação possível: se o vanilla
tivesse `int` como função e `type(1)` como valor distinto mas igualável por
coerção — a sonda refuta isso (`type(1) == int` é `true` por identidade, e
`repr(int) = int` sem aspas).

---

## 3. Proveniência da medição

- **Commit base:** `f5f23be49aa15d55df1a4a4cb29217d0fb8a978c`.
- **Hora (UTC):** 2026-07-10T19:08:04Z.
- **`git diff HEAD --stat`** (19 ficheiros, +659 / −75):

```
00_nucleo/prompts/entities/value.md           | 130 ++++++++++++++--
00_nucleo/prompts/rules/eval.md               |  30 +++-
00_nucleo/prompts/rules/eval/field-access.md  |  40 ++++-
00_nucleo/prompts/rules/stdlib/foundations.md |  46 ++++--
01_core/src/entities/value.rs                 | 211 +++++++++++++++++++++++++-
01_core/src/rules/eval/bibliography.rs        |   2 +-    (só @prompt-hash)
01_core/src/rules/eval/bindings.rs            |  27 +++-
01_core/src/rules/eval/closures.rs            |  26 +++-
01_core/src/rules/eval/control_flow.rs        |   2 +-    (só @prompt-hash)
01_core/src/rules/eval/flow.rs                |   2 +-    (só @prompt-hash)
01_core/src/rules/eval/markup.rs              |   2 +-    (só @prompt-hash)
01_core/src/rules/eval/math.rs                |   2 +-    (só @prompt-hash)
01_core/src/rules/eval/mod.rs                 |  57 ++++---
01_core/src/rules/eval/modules.rs             |   2 +-    (só @prompt-hash)
01_core/src/rules/eval/repr.rs                |   2 +
01_core/src/rules/eval/rules.rs               |   2 +-    (só @prompt-hash)
01_core/src/rules/eval/tests.rs               | 137 ++++++++++++++++-
01_core/src/rules/stdlib/foundations.rs       |   7 +-
01_core/src/rules/stdlib/mod.rs               |   7 +-
```

> Os ficheiros marcados "(só @prompt-hash)" mudaram **apenas** o hash do L0
> `eval.md` (propagado por `crystalline-lint --fix-hashes`), sem alteração de
> lógica.

- **Contagem de testes:** `cargo test --workspace` → 3687 (core) + 606 + 28 + 2
  + 27 + 2 = **4352 passed, 0 failed**.

---

## 4. Sonda (medido na fonte vanilla 0.15.0)

`pdftotext` do documento de sonda (`temp_p685/sonda-tipos.typ`,
`sonda-construtores.typ`, `sonda-edge.typ`):

**`type(v)` e `repr` dos valores-tipo:**

| expressão | vanilla |
|---|---|
| `type(1)` | `int` |
| `type(1.0)` | `float` |
| `type(1pt)` / `type(1em)` | `length` |
| `type(1fr)` | `fraction` |
| `type(50%)` | `ratio` |
| `type(1deg)` | `angle` |
| `type("x")` | `str` |
| `type(true)` | `bool` |
| `type(())` | `array` |
| `type((:))` | `dictionary` |
| `type(int)` / `type(length)` / `type(type)` | `type` |
| `repr(int)` / `repr(length)` / `repr(type)` | `int` / `length` / `type` (sem aspas) |

**Igualdade:** `type(1) == int`→`true`; `type(1pt) == length`→`true`;
`type(50%) == ratio`→`true`; `length == ratio`→`false`; `int == float`→`false`.

**Construtores vs tipos vs funções:**

- `type(int)`=`type(str)`=`type(bool)`=`type(array)`=`type(length)`=`type(type)`= **`type`**.
- `type(rgb)`=`type(repr)`= **`function`**; `type(rgb) == function`→`true`;
  `repr(function)`=`function`.
- Chamáveis: `int("5")=5`, `str(5)="5"`, `float("3.5")=3.5`.
- **Não** chamáveis: `bool(1)`→"type boolean does not have a constructor";
  `array(1,2)`→erro; `dictionary(a:1)`→erro.
- `int` sozinho renderiza `int`; `int("7")`=7 — o **mesmo** valor é tipo e
  construtor.

**Edge cases:**

- `type(50% + 1pt)`=`length` e `type(50% + 1pt) == length`→`true` →
  **comprimento relativo é `length`** (não um tipo próprio); `repr(relative)`→erro
  (não há binding `relative`).
- `type(none)`=`none`, `type(auto)`=`auto` (tipos internos; `none`/`auto` são
  keywords, não bindings).
- Mais tipos com binding: `color`, `gradient`, `stroke`, `bytes`, `decimal`,
  `duration`, `version`, `symbol`, `module`, `datetime`, `regex`, `label`,
  `selector`, `alignment`, `direction`, `location` (`repr` devolve o nome).

**Conclusão da sonda:** existe um meta-valor `type`; cada tipo primitivo tem um
**singleton** no scope; `type(x)` devolve esse singleton; a igualdade é por
identidade; alguns singletons são chamáveis (construtores).

---

## 5. Estado anterior do cristalino (medido)

- `native_type` (`01_core/src/rules/stdlib/foundations.rs:24`) devolvia
  `Value::Str(v.type_name().into())` — uma **string**. Logo `type(1) == int`
  era `Value::Str("int") == <int inexistente>` → erro "unknown variable: int".
- `int`/`float`/`str` (`eval/mod.rs:971–986`) eram `Value::Func` construtores
  (com `native_with_namespace` para `int.min`/`int.max`/`str.from-unicode`).
- `length`/`ratio`/`angle`/… **não** estavam no scope.
- Não existia `Value::Type`.

---

## 6. Implementação

**`01_core/src/entities/value.rs`**

- Enum `Type` (`#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]`) com 33
  variantes unitárias (nomes alinhados a `type_name()`); métodos
  `Type::name()` e `Type::is_callable()` (`Int`/`Float`/`Str`/`Type`).
- Variante `Value::Type(Type)`.
- `type_name()`: `Self::Type(_) => "type"`.
- `Value::type_of(&self) -> Type` (mapeamento total; `Value::Relative` →
  `Type::Length`; `Value::Type(_)` → `Type::Type`).
- `impl From<Type> for Value`.
- Igualdade de tipos vem "de graça" do `#[derive(PartialEq)]` já existente em
  `Value` — `type(1) == int` não precisa de braço em `eval_binary_op`.
- Testes unitários: `type_name_coincide`, `type_of_mapeia_variantes`,
  `type_equality_por_identidade`, `type_is_callable`, `from_type_para_value`.

**`01_core/src/rules/stdlib/foundations.rs`**

- `native_type` (`:24`): `[v] => Ok(Value::Type(v.type_of()))` (era
  `Value::Str(v.type_name().into())`).

**`01_core/src/rules/eval/repr.rs`**

- `repr_value`: braço `Value::Type(t) => t.name().to_string()`
  (`repr(int) == "int"`, `repr(type) == "type"`).

**`01_core/src/rules/eval/mod.rs` (`make_stdlib`)**

- `type` → `Value::Type(Type::Type)`.
- `int`/`float`/`str` → `Value::Type(Type::Int/Float/Str)` (removidos os
  `Func::native_with_namespace` e os imports locais `native_int`/`native_float`/
  `native_str`/`native_str_from_unicode`/`native_type` de `make_stdlib`).
- Novos bindings tipo (sem colisão): `bool`, `length`, `ratio`, `angle`,
  `fraction`, `array`, `dictionary`, `function`, `content`, `arguments`,
  `module`, `datetime`, `bytes`, `symbol`, `alignment`, `direction`, `location`.

**`01_core/src/rules/eval/closures.rs` (call-path)**

- Braço `Value::Type(t)` em `eval_func_call`: `Int`→`native_int`,
  `Float`→`native_float`, `Str`→`native_str`, `Type`→`native_type`
  (com `engine.world`/`engine.current_file`, replicando `apply_func`); restantes
  → erro `type {name} does not have a constructor`.

**`01_core/src/rules/eval/bindings.rs` (`eval_field_access`)**

- Braço `Value::Type(t)`: `int.min`/`int.max` → `i64::MIN`/`MAX`;
  `str.from-unicode` → `Value::Func(native_str_from_unicode)`; restantes →
  erro. Substitui o `Func::native_with_namespace` anterior de `int`/`str`.

**L0 actualizados** (Trava Arquitetural; hashes propagados por
`crystalline-lint --fix-hashes .`): `00_nucleo/prompts/entities/value.md`,
`rules/stdlib/foundations.md`, `rules/eval.md` (§P685),
`rules/eval/field-access.md` (§12).

---

## 7. Débitos e divergências conhecidas

1. **Tipos que colidem com função/módulo (não convertidos).** `color`,
   `gradient`, `stroke`, `regex`, `tiling`, `decimal`, `duration`, `version`,
   `label`, `state`, `counter`, `selector` já estavam no scope como função ou
   módulo (`color.rgb(...)`, `gradient.linear(...)`, `regex(...)`,
   `decimal(...)`, `state(...)`, …). Convertê-los para `Value::Type` quebraria
   esses usos. Para esses, `type(x) == color` permanece `false`. **Não**
   bloqueia `cetz` (que compara com `length`/`ratio`/`angle`/`int`/`float`).
   Reconciliação (módulo/função ⇔ `Value::Type`) fica para passo futuro.

2. **`ratio` vs comprimento relativo (pré-existente, P469).** O cristalino
   modela `50%` como `Value::Relative`, que mapeia para `Type::Length`; logo
   `type(50%) == length` (não `ratio`). O vanilla distingue `50%` (`ratio`) de
   `50% + 1pt` (`length`). Divergência de modelação pré-existente; o teste de
   ratio foi substituído por `type(1deg) == angle` (sem ambiguidade). Fora de
   escopo P685.

3. **Mensagem de erro de construtor.** Cristalino: "type **bool** does not have
   a constructor"; vanilla: "type **boolean** …" (usa o *title*). Divergência
   menor num observable mecânico; não afecta `cetz`.

4. **Espaçamento e smart quotes no PDF** (§8) — mecânica de render pré-existente,
   não introduzida por P685.

---

## 8. Validação

### 8.1 Testes e lint

- `cargo test -p typst-core` → **3687 passed / 0 failed** (inclui 19 testes
  novos P685: `p685_type_eq_*`, `p685_type_of_type`, `p685_type_of_func`,
  `p685_type_distinct`, `p685_repr_*`, `p685_shadow_*`, `p685_int_callable`,
  `p685_str_callable`, `p685_float_callable`, `p685_int_min_max_fields`,
  `p685_str_from_unicode_field`, `p685_bool_not_callable`,
  `p685_length_not_callable`).
- `cargo test --workspace` → **4352 passed / 0 failed**.
- `crystalline-lint --fix-hashes .` + `crystalline-lint .` →
  **✓ No violations found**.

### 8.2 Paridade cristalino vs vanilla (valores)

Documento `temp_p685/paridade.typ` compilado com ambos; `pdftotext` normalizado
(whitespace colapsado, `• `→`•`) → **valores idênticos** em todas as linhas:

- `type(1)==int`→`true`, `type(1.0)==float`→`true`, `type(1pt)==length`→`true`,
  `type("s")==str`→`true`, `type(())==array`→`true`, `type((:))==dictionary`→`true`,
  `type(true)==bool`→`true`, `type(1deg)==angle`→`true`, `type(1fr)==fraction`→`true`.
- `type(int)==type`→`true`, `type(rgb)==function`→`true`, `length==ratio`→`false`.
- `repr(int)`=`int`, …, `repr(type(1pt))`=`length`; `(int)`=`int`.
- `int("5")=5`, `str(5)="5"`, `float("3.5")=3.5`.
- Sombreamento: `#let length = 5` → `length`=`5`, `type(length)==int`→`true`.
- `int.max`=`9223372036854775807`, `str.from-unicode(97)`=`a`.

Diferenças residuais: **apenas** smart quotes (`"` vs `"`) e espaçamento de
layout — mecânica de render pré-existente (§7.4).

### 8.3 `cetz` re-testado (honestidade)

```
#import "@preview/cetz:0.2.2": canvas, draw
#canvas({ draw.line((0,0), (1,1)) })
```

- **Antes de P685 (P683):** bloqueio em `type(x) == length` (nomes de tipo não
  eram valores).
- **Depois de P685:** o bloqueio de tipos desapareceu; o próximo erro é
  `cetz.typ:<detached>: error: include: ficheiro não encontrado: /src/process.typ`
  — resolução de caminho **absoluto** `/src/process.typ` dentro do pacote
  (root-relative `include`/`import`). PDF não gerado. Registado sem assumir que
  `cetz` está resolvido: **não está** — avançou um bloqueio.

---

## 9. Critério de fecho

- [x] Sonda mínima completa; lista de tipos e o que `#(int)` produz confirmados.
- [x] Tipos expostos como valores; `type(x) == length` (e equivalentes) testado
      contra o vanilla.
- [x] `type()` sem regressão (devolve valor-tipo; `repr(type(x))` preserva o nome).
- [x] Sombreamento por variável do utilizador testado (`#let length = 5`).
- [x] `cetz` re-testado; próximo estado registado com honestidade.
- [x] `cargo test --workspace` sem regressão (4352 passed).
- [x] `crystalline-lint .` limpo.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p685.md` com hash
      do commit (preenchido no 2.º commit).

---

## 10. Nota de forma

Trabalho feito em `temp_p685/` (scratch, apagado no fim). O commit inclui
**apenas** os 19 ficheiros listados em §3 (L0 + código) e este relatório; os
untracked pré-existentes (`materialization/`, `adr/`, `temp_p*/`, `perf.data`)
não foram adicionados.
