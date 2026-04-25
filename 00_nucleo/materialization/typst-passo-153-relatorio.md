# Passo 153 — Relatório (P2 cristalino-only baseline; `ValueDTO` + matriz semantic)

**Data**: 2026-04-25
**Natureza**: passo **substantivo** em `lab/parity/` (workspace
separado fora do cristalino). **Zero código tocado em L1, L2,
L3, L4 cristalino**. **Zero ADRs criadas**. **Zero DEBTs
criados / fechados / actualizados**. **Tests cristalino**:
1113 inalterados.
**Precondição**: Passo 152 encerrado; DEBT-54 com plano
refinado; baseline P3 P150 preservado; 12 DEBTs abertos.

---

## 1. Sumário executivo

Passo 153 entregou a **infraestrutura de medição P2** em
`lab/parity/`:

- `lab/parity/src/value_dto.rs` — `ValueDTO` neutro com 20
  variants em ordem alfabética; `from_cristalino()` cobre os
  18 variants do `Value` cristalino sem panic;
  `from_vanilla_stub()` placeholder; `compare()` semântico.
- `lab/parity/tests/eval_parity.rs` — harness sem `assert!`
  global; itera corpus semantic; eval cristalino via
  `SystemWorld` + `eval_to_module_with_sink`; extrai
  `__resultado__` do scope; converte para `ValueDTO`.
- `lab/parity/corpus/semantic/` — **10 ficheiros novos** com
  metadata `.typ.toml` (bool, int, float, str, array, dict,
  if, len, type, closure).
- `lab/parity/reports/latest.md` — re-renderizado com matriz
  P2 ao lado de P3 (P150 preservada).
- `lab/parity/reports/history/2026-04-25-passo-153.md` —
  cópia histórica; cópias 150 e 151 preservadas.

**Matriz P2 produzida**: **10/10 ficheiros eval ok** em
cristalino. Coluna `value_equality` vs vanilla = `N/A`
(DEBT-54 → DEBT-53).

**Tests `lab/parity`**: `corpus_semantic_p2` corre limpo.

**Reformulação 7 da série paridade** confirmada (148 → 149 →
150 → 151 → 152 → **153** P2 baseline).

---

## 2. Inventário pré-materialização (sub-passo 153.1)

### 2.1 — API necessária em `typst-core`

| Símbolo | Localização | Disponibilidade |
|---------|-------------|-----------------|
| `Module::scope() -> &Scope` | `01_core/src/entities/module.rs:52` | ✓ |
| `Scope::get(name) -> Option<&Value>` | `01_core/src/entities/scope.rs:60` | ✓ |
| `Func::name() -> Option<&str>` | `01_core/src/entities/func.rs:107` | ✓ |
| `Module::name() -> &str` | `01_core/src/entities/module.rs:48` | ✓ |
| `Content::plain_text() -> String` | `01_core/src/entities/content.rs:376` | ✓ |
| `eval_to_module_with_sink(world, source)` | `03_infra/src/pipeline.rs` | ✓ (já em uso pelo P150) |

**Sem accessor novo necessário** em L1; toda a API exigida
já existe.

### 2.2 — Pipeline do test harness

```
src   = String com `#let __resultado__ = <expr>` + outros lets opcionais
world = SystemWorld::new(tempdir, "main.typ")
source = world.source(world.main())
module = eval_to_module_with_sink(world, source).0?
value  = module.scope().get("__resultado__")?.clone()
dto    = ValueDTO::from_cristalino(&value)
```

### 2.3 — Inventário do `Value` cristalino (18 variants)

Confirmado em `01_core/src/entities/value.rs:18-83`:

| Variant | Forma |
|---------|-------|
| `None`, `Auto` | singleton |
| `Bool(bool)` | trivial |
| `Int(i64)`, `Float(f64)`, `Fraction(f64)` | numéricos |
| `Str(EcoString)` | EcoString (clone O(1)) |
| `Array(Vec<Value>)` | recurse |
| `Dict(IndexMap<EcoString, Value>)` | preserva ordem |
| `Module(Module)` | nome via `.name()` |
| `Datetime(Datetime)` | format Debug |
| `Func(Func)` | nome via `.name()` |
| `Content(Content)` | plain_text via `.plain_text()` |
| `Length(Length)`, `Ratio(Ratio)`, `Angle(Angle)`, `Color(Color)`, `Align(Align2D)` | format Debug |

Variants comentados (futuros, ADR-0017):
`Relative`, `Gradient`, `Tiling`, `Symbol`, `Version`,
`Bytes`, `Decimal`, `Duration`, `Styles`, `Args` (ADR-0059),
`Type` (ADR-0058), `Dyn`. **Total 18 + ~12 ausentes
estratégicamente**.

---

## 3. `value_dto.rs` — assinatura final

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum ValueDTO {
    Align(String), Angle(String), Array(Vec<ValueDTO>), Auto,
    Bool(bool), Color(String), Content(String),
    Datetime(String), Dict(Vec<(String, ValueDTO)>),
    Float(u64),       // bits via to_bits()
    Fraction(u64),
    Func(String),     // "<closure>" para sem-nome
    Int(i64), Length(String), Module(String), None,
    Other(String),    // catch-all (vanilla::Args/Bytes/Decimal/...)
    Ratio(String),
    Str(String),
    Type(String),     // ADR-0058 — vazio em cristalino
}

impl ValueDTO {
    pub fn from_cristalino(v: &typst_core::entities::value::Value) -> Self;
    pub fn from_vanilla_stub() -> Self;  // DEBT-54
    pub fn compare(&self, other: &Self) -> ValueComparison;
    pub fn type_name(&self) -> &'static str;
}

pub enum ValueComparison {
    Equal,
    Differ { crist: String, vanilla: String },
}
```

**Notas**:

- **20 variants** em ordem alfabética (16 + 4 cobre-tudo:
  `Other`, `Type`, `Fraction`, `Align`).
- `Float` e `Fraction` armazenam bits via `f64::to_bits()`
  (per `definicoes.md` §P2 "bits idênticos (NaN inclusive)").
- `Func`: `f.name().unwrap_or("<closure>")`.
- `Module`: `m.name().to_string()`.
- `Content`: `c.plain_text()` — `String`.
- `Length`/`Ratio`/`Angle`/`Color`/`Align`/`Datetime`:
  `format!("{:?}", ...)` (Debug repr; suficiente para
  comparação cristalino-only; vanilla pode divergir e
  `ValueDTO::Other` capturará).

---

## 4. `tests/eval_parity.rs` — estratégia + resultados

**Estratégia**:

1. Lê corpus em `lab/parity/corpus/semantic/*.typ` ordenado
   por nome.
2. Para cada ficheiro:
   - Cria tempdir; escreve source em `main.typ`.
   - `SystemWorld::new(tempdir, "main.typ")`.
   - `eval_to_module_with_sink(world, source)` →
     `(SourceResult<Module>, warnings)`.
   - Em sucesso: `module.scope().get("__resultado__")` →
     `Option<&Value>` → `.clone()` → `ValueDTO::from_cristalino`.
   - Conta `eval_ok` / `eval_fail` / `no_resultado`.
3. Re-renderiza `latest.md` com matriz P2 + P3 preservada
   (P150).
4. Escreve `history/2026-04-25-passo-153.md` (cópia idêntica).

**Sem `assert!` global** (consistente com P150). Falhas
individuais entram na matriz como `eval_fail` ou
`no_resultado`.

**Corrida**:

```
cd lab/parity && cargo test --test eval_parity
test corpus_semantic_p2 ... ok
```

Resultado:

| Métrica | Valor |
|---------|------:|
| Total ficheiros | 10 |
| Eval ok | **10** |
| Eval falhou | 0 |
| Sem `__resultado__` | 0 |
| `value_equality` vs vanilla | N/A |

---

## 5. Corpus semantic — lista final + diagnósticos

10 ficheiros em `lab/parity/corpus/semantic/`:

| Ficheiro | Resultado cristalino | Notas |
|----------|----------------------|-------|
| `array-literal.typ` | `array ([3])` | `(1, 2, 3)` literal |
| `bool-true.typ` | `bool (true)` | trivial |
| `closure-aplicada.typ` | `int (15)` | f(7,8) → 15; **diagnóstico ADR-0059** |
| `condicional.typ` | `str ("sim")` | `if 2 > 1 { "sim" } ...` |
| `dict-literal.typ` | `none (None)` | **surpresa**: `(a: 1, b: 2)` retorna `None` no scope (informação útil) |
| `float-divisao.typ` | `float (0x4004000000000000)` | `10.0/4.0` = 2.5 (bits) |
| `funcao-builtin.typ` | `int (3)` | `len((1,2,3))` |
| `int-aritmetica.typ` | `int (5)` | `2 + 3` |
| `string-concat.typ` | `str ("ab")` | `"a" + "b"` |
| `tipo-inspeccao.typ` | `str ("int")` | **diagnóstico ADR-0058** |

### 5.1 — Diagnósticos canários

- **`tipo-inspeccao.typ`** (ADR-0058): cristalino devolve
  `Value::Str("int")` via `native_type`. Pós-DEBT-54: vanilla
  devolverá `Value::Type(Type)` rico → `ValueDTO::Type("int")`.
  Comparação revelará a divergência.
- **`closure-aplicada.typ`** (ADR-0059): closure cristalina
  recebe `&Args` como parâmetro (não embrulhado em
  `Value::Args`). Pós-DEBT-54: vanilla pode embrulhar `Args`
  em `Value::Args(Args)` em alguns contextos (a confirmar).

### 5.2 — Surpresa empírica: `dict-literal.typ`

`#let __resultado__ = (a: 1, b: 2)` produz `None` em
cristalino. Hipóteses:

1. Sintaxe de dict literal em `let` cristalino exige forma
   diferente (e.g., `dict(a: 1, b: 2)` ou `{ a: 1, b: 2 }`).
2. Parser interpreta `(a: 1, b: 2)` como argumentos nomeados
   (Args), não dict literal.
3. Há regressão / feature ausente parcialmente em eval.

Não-bloqueante para P153 — registado na matriz como
informação. Investigação fica para passo posterior se
priorizado. ADR-0034 perfil cobre: pode ser divergência
estrutural aceite ou bug a corrigir; classificação requer
inspecção dedicada.

---

## 6. Matriz primeira (cópia integral de `latest.md` pós-153)

```
# Paridade — Passo 153 (2026-04-25)

**Matriz multi-nível** (P2 + P3 cristalino-only baseline).
Vanilla integration depende de **DEBT-54** → fecho **DEBT-53**.

## Matriz P2 (Eval, Passo 153)

| Categoria | Total | Eval ok (crist) | Eval falhou | sem `__resultado__` | value_equality (vs vanilla) |
|-----------|------:|----------------:|------------:|--------------------:|:---------------------------:|
| semantic  |    10 |          10/ 10 |           0 |                  0 |                         N/A |
| **Total** |   **10** |     **10/ 10** |       **0** |              **0** |                         N/A |

### Detalhes (cristalino)

```
  array-literal.typ → array ([3])
  bool-true.typ → bool (true)
  closure-aplicada.typ → int (15)
  condicional.typ → str ("sim")
  dict-literal.typ → none (None)
  float-divisao.typ → float (0x4004000000000000)
  funcao-builtin.typ → int (3)
  int-aritmetica.typ → int (5)
  string-concat.typ → str ("ab")
  tipo-inspeccao.typ → str ("int")
```

---

## Matriz P3 (Layout, Passo 150 — preservada)

[matriz P150: 19/19 compila; vanilla N/A]
```

(Cópia integral guardada em
`lab/parity/reports/history/2026-04-25-passo-153.md`.)

---

## 7. §9 dos documentos de paridade actualizado

Novo item 6 (P153 implementado) acrescentado; itens 7/8/9
renumerados:

| # | Antes | Depois |
|---|-------|--------|
| 5 | Passo 152 (refino DEBT-54, P152) | mantém |
| 6 | Passo 153+ (P2) | **Passo 153 = implementado** |
| 7 | Passo dedicado DEBT-54 | renumerado para item 8 |
| 8 | Decisão corpus | renumerado para item 9 |
| 6→7 | (insertado) Passo 154+ (P4 textual) | novo |

Texto final reflecte que P153 está completo; P154+ = P4;
P155+ = passo dedicado para DEBT-54.

---

## 8. Próximo passo

Sequência recomendada (decisão humana entre eles):

1. **P154 — P4 cristalino-only baseline** (`pdf_compare.rs`
   textual). Estende cobertura observacional ao quarto
   nível; mantém estratégia cristalino-only até DEBT-54
   fechar. Estimativa: similar a P150/P153.
2. **Passo dedicado para DEBT-54** — setup vanilla workspace
   (~3-4h por estimativa pós-152). Após fechar, novo passo
   materializa vanilla integration em `lab/parity` e popula
   colunas vanilla nas matrizes P2 + P3.
3. **Sub-investigação de `dict-literal.typ`** — passo curto
   se priorizado (verificar se é feature ausente em
   cristalino eval ou bug do test).

---

## 9. Verificação final

| Item | Estado |
|------|--------|
| `lab/parity/src/value_dto.rs` materializado (20 variants) | ✅ |
| `from_cristalino` cobre os 18 variants `Value` cristalino sem panic | ✅ |
| `from_vanilla_stub` placeholder | ✅ |
| `compare()` implementado (PartialEq derivado + Debug repr) | ✅ |
| `lab/parity/tests/eval_parity.rs` corre o corpus semantic | ✅ (`cargo test --test eval_parity` → ok) |
| Subdir `lab/parity/corpus/semantic/` com 10 ficheiros + `.typ.toml` adjacentes | ✅ |
| `lab/parity/reports/latest.md` re-renderizado com matriz P2 + P3 preservada | ✅ |
| `lab/parity/reports/history/2026-04-25-passo-153.md` produzido | ✅ |
| Cópias 150 e 151 preservadas em `history/` | ✅ |
| §9 dos documentos de paridade renumerado | ✅ |
| Nenhum ficheiro tocado em L1/L2/L3/L4 cristalino | ✅ |
| Nenhuma ADR criada / revogada / revisada | ✅ |
| Nenhum DEBT criado / fechado / actualizado | ✅ |
| `crystalline-lint .` zero violations | ✅ |
| `cargo test --workspace --lib`: 1113 inalterado | ✅ |
| `cd lab/parity && cargo test --test eval_parity` corre | ✅ |
| `cd lab/parity && cargo test --test layout_parity` continua a correr (P150 baseline) | ✅ |
| Matriz P2 entrega: 10/10 ficheiros eval ok em cristalino | ✅ |
| Diagnósticos canários `tipo-inspeccao.typ` (ADR-0058) e `closure-aplicada.typ` (ADR-0059) presentes no corpus | ✅ |
| Surpresa empírica (`dict-literal.typ → None`) registada para investigação posterior | ✅ |

**Pós-153**: utilizador tem **2 níveis de paridade
materializados** (P2 + P3) com cobertura observacional
cristalino-only. **Falta P4** + vanilla integration via
DEBT-54 → DEBT-53. A partir do P150 + P153, a coluna
"compila/eval cristalino" tem números reais; as colunas
vanilla esperam DEBT-54 para serem preenchidas.

**Reformulação 7 da série paridade** confirma o padrão:
inventário (148) → arqueologia (149) → P3 baseline (150) →
investigação DEBT-53 (151) → refino DEBT-54 (152) → **P2
baseline (153)**. Cada passo entrega um pedaço do puzzle;
nenhum entrega tudo. A pergunta "em que paridade estamos?"
tem **resposta tripla** desde P150:
1. **Cobertura declarada** (148+149): 54% user-facing, 72%
   arquitectural.
2. **Cobertura observacional cristalino-only** (150+153):
   19/19 P3 + 10/10 P2 compilam/evaluam sem panic.
3. **Paridade observacional vs vanilla**: N/A — pendente de
   DEBT-54 → DEBT-53.
