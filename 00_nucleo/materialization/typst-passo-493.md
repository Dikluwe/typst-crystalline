---

# P493 — Materialização de Field Access em Coleções (D3)

> **Passo:** 493
> **Data:** 2026-06-29
> **Foco:** Fechar empiricamente os 3 gaps de field access em coleções identificados no diagnóstico P490 (Grupo D3). Não declarar conclusão — medir antes e depois de cada sub-tarefa.
> **Tipo:** Implementação M-size com validação via bateria P490.
> **Tamanho:** M (~45 min de implementação + 15 min de validação).
> **ADR-0075 ACEITE** — mecanismo de comparação via `typst query --format json` vs `Introspector::query_*`.
> **ADR-0054 ACEITE** — graded parity: MATCH / DIFF / ERRO_DESCRITIVO / PANIC / AUSENTE.
> **Dependências:** P491 (Lote D2 fechado), P492 (D4/D5 fechado), P466 (array methods), P247 (stroke), P393/P473 (show-regex).

---

## Metodologia

Para cada sub-tarefa D3a–D3c, implementar o field access, correr a bateria P490 (20 ficheiros), e comparar o output estrutural contra o baseline do diagnóstico.

```bash
# Baseline P492 (antes de tocar código):
cargo test --test structural_parity p492_cores_predefinidas_text_e_stroke

# Após cada sub-tarefa:
cargo test --test structural_parity p493_<subtarefa>
```

Classificar cada resultado: `MATCH` | `DIFF` | `ERRO_DESCRITIVO` | `PANIC` | `AUSENTE`.

---

## Categoria 1 — `arr.dedup()` (D3a)

### 1.1 — Estado baseline (P490 / P492)

```typst
// test-array.typ
#let arr = (3, 1, 4, 1, 5, 9, 2, 6)
#arr.dedup()
```

- **Vanilla:** `ok((3, 1, 4, 5, 9, 2, 6))` — remove duplicados adjacentes.
- **Cristalino (pré-493):** `ERRO_DESCRITIVO` — field access `dedup` não suportado em array.
- **Classificação P490:** **DIFF**

### 1.2 — Diagnóstico de causa raiz

P466 materializou métodos de array (`sorted`, `enumerate`, `zip`, `flatten`, `fold`), mas `dedup()`, `chunks(n)` e `windows(n)` ficaram como gap residual. O field access em array (`arr.dedup`) requer:

1. **H1:** O eval de field access (`value.field`) já existe para structs (`Content`, `Dict`), mas não para `Array`.
2. **H2:** O parser já reconhece `arr.dedup` como field access, mas o eval não tem handler para `Array` + field name.
3. **H3:** O método `dedup` não existe como `Value::Func` no scope de array.

**Verificação rápida:**
```bash
rg -n "field_access\|FieldAccess" src/eval/ --type rs
rg -n "dedup\|chunks\|windows" src/stdlib/ --type rs
```

### 1.3 — Implementação

**Arquivo alvo:** `src/eval/field_access.rs` (ou equivalente) e `src/stdlib/array.rs`

**Mudança 1 (eval):** Adicionar handler de field access para `Value::Array`:

```rust
Value::Array(arr) => match field.as_str() {
    "len" => Ok(Value::Int(arr.len() as i64)),
    "first" => Ok(arr.first().cloned().unwrap_or(Value::None)),
    "last" => Ok(arr.last().cloned().unwrap_or(Value::None)),
    "dedup" => Ok(Value::Func(array_dedup)),
    "chunks" => Ok(Value::Func(array_chunks)),
    "windows" => Ok(Value::Func(array_windows)),
    // ... métodos já existentes (sorted, enumerate, zip, etc.)
    _ => Err("array does not have field '{}'"),
},
```

**Mudança 2 (stdlib):** Implementar `array_dedup` como `Value::Func`:

```rust
fn array_dedup(args: Args) -> SourceResult<Value> {
    let arr = args.expect::<Array>("self")?;
    let mut result = Vec::new();
    for item in arr.iter() {
        if result.last() != Some(item) {
            result.push(item.clone());
        }
    }
    Ok(Value::Array(result))
}
```

### 1.4 — Validação

Rodar `test-array.typ` contra vanilla e cristalino. Esperado:
- `arr.dedup()` → **MATCH** ((3, 1, 4, 5, 9, 2, 6))
- `arr.chunks(3)` → **MATCH** se testado ((3, 1, 4), (1, 5, 9), (2, 6))
- `arr.windows(3)` → **MATCH** se testado ((3, 1, 4), (1, 4, 1), ...)

---

## Categoria 2 — `table.header` / `table.footer` (D3b)

### 2.1 — Estado baseline (P490 / P492)

```typst
// test-table.typ
#table(
  columns: (1fr, 1fr, 1fr),
  table.header[Nome][Idade][Cidade],
  [Ana], [25], [Lisboa],
)
```

- **Vanilla:** `ok(1)` — table com header renderiza.
- **Cristalino (pré-493):** `ERRO_DESCRITIVO` — `table.header` não reconhecido; field access em função `table` não suportado.
- **Classificação P490:** **DIFF**

### 2.2 — Diagnóstico de causa raiz

No vanilla, `table` é uma função que também expõe sub-funções (`table.header`, `table.footer`, `table.cell`, `table.hline`, `table.vline`) via field access na própria função. Isso é um padrão especial: **função com namespace anexado**.

**Hipóteses:**
1. **H1:** O cristalino trata `table` como `Value::Func` simples, sem campos anexados.
2. **H2:** `table.header` existe como função separada mas não está ligada ao namespace `table`.
3. **H3:** O parser não reconhece `table.header` como field access em função.

**Verificação rápida:**
```bash
rg -n "table.header\|table.footer" src/ --type rs --type md
rg -n "Func.*field\|field.*Func" src/eval/ --type rs
```

### 2.3 — Implementação

**Arquivo alvo:** `src/eval/field_access.rs` e `src/stdlib/table.rs`

**Mudança 1 (eval):** Adicionar handler de field access para `Value::Func` quando a função tem namespace anexado:

```rust
Value::Func(func) => {
    if let Some(namespace) = func.namespace() {
        match namespace.get(field.as_str()) {
            Some(value) => Ok(value.clone()),
            None => Err("function does not have field '{}'"),
        }
    } else {
        Err("function does not have fields")
    }
},
```

**Mudança 2 (stdlib):** Construir `table` como função com namespace:

```rust
let mut table_namespace = Scope::new();
table_namespace.define("header", Value::Func(native_table_header));
table_namespace.define("footer", Value::Func(native_table_footer));
table_namespace.define("cell", Value::Func(native_table_cell));
// ...

let table_func = Value::Func(Func::with_namespace(
    native_table,
    table_namespace,
));
```

**Mudança 3 (stdlib):** Implementar `native_table_header`:

```rust
fn native_table_header(args: Args) -> SourceResult<Value> {
    // Recebe células posicionais, retorna Content::TableHeader ou similar
    let cells = args.expect_positional::<Content>()?;
    Ok(Value::Content(Content::TableHeader { cells }))
}
```

**Nota:** Se o modelo de `Content` do cristalino não tem `TableHeader`/`TableFooter` variants, adicioná-los em `entities/content.md` / `src/model/content.rs`.

### 2.4 — Validação

Rodar `test-table.typ` contra vanilla e cristalino. Esperado:
- `table.header[Nome][Idade][Cidade]` → **MATCH** (table compila com header)
- `table.footer[Total][2][—]` → **MATCH** se testado
- `table(...)` sem header → **MATCH** (não-regressão)

---

## Categoria 3 — `show heading.where(level: 1, outlined: true)` multi-field (D3c)

### 3.1 — Estado baseline (P490 / P492)

```typst
// test-show-where-multi.typ
#show heading.where(level: 1, outlined: true): it => upper(it.body)
= Heading nível 1
```

- **Vanilla:** `ok(1)` — heading renderiza em maiúsculas.
- **Cristalino (pré-493):** `ERRO_DESCRITIVO` — `heading.where` com múltiplos campos não suportado; scope-out P474.
- **Classificação P490:** **DIFF**

### 3.2 — Diagnóstico de causa raiz

P474 declarou multi-field `where` como scope-out. O P490 confirmou que o erro é descritivo (não PANIC). Para fechar este gap, é necessário:

1. O parser de `Selector::Where` já aceita `base: Box<Selector>, field: EcoString, value: Value` (documentado em `entities/selector.md:72`).
2. O gap é que `heading.where(...)` com múltiplos args nomeados não cria um `Selector::Where` encadeado (`Where { base: Where { base: Kind(Heading), field: "level", value: 1 }, field: "outlined", value: true }`).

**Hipóteses:**
1. **H1:** O parser de `where` só aceita 1 arg nomeado e rejeita múltiplos.
2. **H2:** O parser aceita múltiplos mas o eval não encadeia os `Where`.
3. **H3:** O eval de `Selector::Where` com múltiplos campos não faz AND lógico.

**Verificação rápida:**
```bash
rg -n "Where.*field\|where.*parse" src/ --type rs
rg -n "heading.where\|Selector::Where" src/ --type rs
```

### 3.3 — Implementação

**Arquivo alvo:** `src/eval/selector.rs` (ou `src/parse/selector.rs`)

**Mudança:** Se o parser já aceita múltiplos args nomeados em `where`, o eval deve encadear os `Where`:

```rust
// No eval de heading.where(...)
let mut selector = Selector::Kind(Heading);
for (field, value) in args.named.iter() {
    selector = Selector::Where {
        base: Box::new(selector),
        field: field.clone(),
        value: value.clone(),
    };
}
```

Se o parser rejeita múltiplos args, ajustar o parser para aceitar `IndexMap<EcoString, Value>` em vez de um único par.

**Nota:** P474 scope-out pode ter sido por complexidade do parser. Se for o caso, P493c pode ser S-size (só ajustar o eval) ou M-size (parser + eval).

### 3.4 — Validação

Rodar `test-show-where-multi.typ` contra vanilla e cristalino. Esperado:
- `heading.where(level: 1, outlined: true)` → **MATCH** (show-rule aplica)
- `heading.where(level: 1)` → **MATCH** (não-regressão, single-field)
- `heading.where(level: 2)` → **MATCH** (não aplica, heading nível 1 sem alteração)

---

## Formato do relatório de resultados

Para cada sub-tarefa, produzir uma linha:

```
| 493a arr.dedup() | Vanilla: ok((3,1,4,5,9,2,6)) | Cristalino: ok((3,1,4,5,9,2,6)) | MATCH | chunks/windows como colateral |
| 493b table.header | Vanilla: ok(1) | Cristalino: ok(1) | MATCH | namespace anexado em func |
| 493c heading.where(multi) | Vanilla: ok(1) | Cristalino: ok(1) | MATCH | Where encadeado, scope-out P474 revogado |
```

Colunas: `sub-tarefa | vanilla | cristalino | classificação | notas`

**Classificações:**
- `MATCH` — output estruturalmente equivalente.
- `DIFF` — ambos produzem resultado mas diferente (investigar).
- `ERRO_DESCRITIVO` — cristalino retorna erro com mensagem clara (aceitável se scope-out).
- `PANIC` — cristalino crasha (bug prioritário, deve ser zero).
- `AUSENTE` — funcionalidade não reconhecida.

---

## Testes de não-regressão

Após as 3 sub-tarefas, rodar a bateria completa P490 (20 ficheiros):

| Ficheiro | Selector | Vanilla | Cristalino pré-493 | Esperado pós-493 | Δ |
|---|---|---|---|---|---|
| test-array.typ | `metadata` | ok(0) | ERRO_DESCRITIVO | ok(0) | **DIFF → MATCH** |
| test-table.typ | `table` | ok(1) | ERRO_DESCRITIVO | ok(1) | **DIFF → MATCH** |
| test-show-where-multi.typ | `heading` | ok(1) | ERRO_DESCRITIVO | ok(1) | **DIFF → MATCH** |
| test-stroke-sides.typ | `heading` | ok(0) | ok(0) | ok(0) | MATCH (P492) |
| test-show-regex.typ | `heading` | ok(0) | ok(0) | ok(0) | MATCH (P492) |
| test-calc.typ | `metadata` | ok(0) | ok(0) | ok(0) | MATCH (P491) |
| test-str.typ | `metadata` | ok(0) | ok(0) | ok(0) | MATCH (P491) |
| test-dict.typ | `metadata` | ok(0) | ok(0) | ok(0) | MATCH (P491) |
| (outros 12) | — | — | — | — | preservados |

**DIFFs restantes esperados:** 3 - 3 = **0** (todos os DIFFs do P490 resolvidos).  
**PANICs esperados:** **0** (preservado).  
**AUSENTEs esperados:** 6 (preservados, não são alvo deste passo).

---

## Critério de fecho

- [ ] 493a implementado: `arr.dedup()`, `arr.chunks(n)`, `arr.windows(n)` funcionam.
- [ ] 493b implementado: `table.header`, `table.footer`, `table.cell` funcionam (namespace anexado em func).
- [ ] 493c implementado: `heading.where(level: 1, outlined: true)` funciona (Where encadeado).
- [ ] 3 testes unitários novos passam (`p493_array_dedup`, `p493_table_header`, `p493_heading_where_multi`).
- [ ] Bateria P490 completa: 3 DIFFs de D3 viraram MATCH.
- [ ] DIFFs restantes: **0** (todos os DIFFs do P490 resolvidos).
- [ ] PANICs: 0 (preservado).
- [ ] AUSENTEs: 6 (preservados, não regressão).
- [ ] Documentação atualizada (`entities/selector.md` se P474 revogado; `rules/stdlib/table.md` com header/footer; `rules/stdlib/array.md` com dedup/chunks/windows).
- [ ] ADR-0107 checklist atualizado (3 itens marcados implementado).
- [ ] Sentinela `p493_field_access_colecoes` adicionada em `lab/parity/tests/structural_parity.rs`.
- [ ] `00_nucleo/diagnosticos/paridade-funcional-p493.md` produzido com tabela de resultados.

---

## Próximo passo (P494)

Com P493 fechado, todos os DIFFs do P490 estão resolvidos. Os gaps restantes são apenas AUSENTEs (selectors não registrados):

| Grupo | Gap | Tamanho | Recomendação |
|---|---|---|---|
| D1 | Selectores ausentes (`list`, `enum`, `par`, `link`, `raw`, `quote`, `footnote`) | M-size | P494 = expansão `parse_selector` |

**Recomendação:** P494 = **D1 (selectors ausentes)** — M-size, pode ser split em sub-tarefas:
- P494a: `list`, `enum`, `par` (3 elementos de layout de texto)
- P494b: `link`, `raw`, `quote`, `footnote` (4 elementos de conteúdo inline)

Alternativa: P494 = **audit de cobertura stdlib** — verificar se há outros gaps não detectados pela bateria P490 (ex: `calc` args nomeados restantes, `str` métodos, `dict` métodos).

---

## A. Apêndice — Referência rápida dos gaps D3

```typst
// 493a: array dedup/chunks/windows
#let arr = (3, 1, 4, 1, 5, 9, 2, 6)
#assert(arr.dedup() == (3, 1, 4, 5, 9, 2, 6))
#assert(arr.chunks(3) == ((3, 1, 4), (1, 5, 9), (2, 6)))
#assert(arr.windows(3) == ((3, 1, 4), (1, 4, 1), (4, 1, 5), (1, 5, 9), (5, 9, 2), (9, 2, 6)))

// 493b: table header/footer
#table(
  columns: (1fr, 1fr, 1fr),
  table.header[Nome][Idade][Cidade],
  [Ana], [25], [Lisboa],
  table.footer[Total][1][—],
)

// 493c: show where multi-field
#show heading.where(level: 1, outlined: true): it => upper(it.body)
= Heading nível 1
```

---

## B. Apêndice — Estrutura de namespace anexado (referência)

No vanilla, as seguintes funções têm namespaces anexados:

| Função | Campos do namespace |
|---|---|
| `table` | `header`, `footer`, `cell`, `hline`, `vline` |
| `list` | `item` |
| `enum` | `item` |
| `quote` | `attribution` (não, é arg nomeado) |
| `footnote` | `entry` (não, é arg nomeado) |

O cristalino já pode ter `list.item` e `enum.item` implementados (verificar). O gap D3b foca em `table.header`/`table.footer` como caso representativo do padrão namespace-anexado.
