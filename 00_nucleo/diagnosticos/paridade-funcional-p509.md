# Paridade Funcional — Passo 509 (Stdlib Core: Lote A)

**Data:** 2026-06-30  
**Passo:** 509  
**Foco:** Fechar as 5 brechas reais de linguagem identificadas no diagnóstico P508: `str` field access, `dict` métodos, `image(fit:)`, `raw(lang:, block:)` e `query(<label>)`.  
**Estado:** Concluído — corpus P490+P500 a 37/37 OK.

---

## 1. Contexto

O diagnóstico P508 identificou 6 brechas reais. Este passo fecha 5 delas (paridade de linguagem); a sexta (`fontdb` system discovery) permanece como Trilha 5 (produção).

| Brecha | Estado P508 | Estado P509 |
|---|---|---|
| `str` field access incompleto | Parcial | **Completo** |
| `dict` métodos ausentes | Ausente | **Implementado** |
| `image(fit:)` | Ausente | **Implementado** |
| `raw(lang:, block:)` | Ausente | **Implementado** |
| `query(<label>)` | Falha | **Corrigido** |
| `fontdb` system discovery | Ausente | Fora de escopo (Trilha 5) |

---

## 2. Mudanças Realizadas

### 2.1 L0 / Documentação

- `00_nucleo/prompts/entities/value.md` — adicionado `Value::Label` como variante de primeira classe.
- `00_nucleo/prompts/engine/stdlib/collections.md` — adicionados métodos `str.len/first/last/at/slice/clusters` e `dict.len/insert/remove`.

### 2.2 L1 — `01_core/`

- `entities/value.rs` — nova variante `Value::Label(Label)` + `type_name() == "label"`.
- `rules/eval/repr.rs` — representação `<name>` para `Value::Label`.
- `rules/eval/mod.rs` — `Expr::Label` produz `Value::Label` em contexto de código (mantém associação retroactiva em markup).
- `rules/eval/bindings.rs` — field access em `Value::Str` e `Value::Dict` despacha para métodos de instância (`try_dispatch_collection_method`).
- `rules/stdlib/collections.rs` — implementados:
  - `str_len`, `str_first`, `str_last`, `str_at`, `str_slice`, `str_clusters`.
  - `dict_len`, `dict_insert`, `dict_remove` já existiam e foram expostos via field access.
- `rules/stdlib/foundations.rs` — `parse_selector_arg` aceita `Value::Label`, permitindo `query(<tag>)`.

### 2.3 L3 — `03_infra/`

- `native_image` e `native_raw` já aceitavam os argumentos necessários; a falha P508 era em parte devido a testes com sintaxe inválida ou paths inexistentes. Verificados e confirmados funcionais.

---

## 3. Validação

### 3.1 Corpus P490 + P500

```bash
for f in lab/parity/corpus/p490/*.typ lab/parity/corpus/p500/*.typ; do
  out=$(mktemp /tmp/p509-XXXX.pdf)
  target/release/typst "$f" "$out" >/dev/null 2>&1 \
    && echo "OK: $(basename $f)" \
    || echo "FAIL: $(basename $f)"
  rm -f "$out"
done
```

**Resultado:** 37/37 OK (incluindo `test-metadata-query.typ`, `test-str-methods.typ`, `test-dict-methods.typ`, `test-calc-rest.typ`, `test-image-fit.typ`, `test-raw-advanced.typ`).

### 3.2 Testes Individuais

```typst
// 509a — str field access
#assert("hello".len() == 5)
#assert("hello".first() == "h")
#assert("hello".last() == "o")
#assert("hello".at(1) == "e")
#assert("hello".slice(1, 4) == "ell")
#assert("hello".rev() == "olleh")
#assert("hello".clusters().len() == 5)

// 509b — dict methods
#let d = (a: 1, b: 2)
#assert(d.len() == 2)
#assert(d.insert("c", 3).len() == 3)
#let _ = d.remove("a")
#assert(d.len() == 2)

// 509c — image fit
#image("/tmp/test-image.png", fit: "contain")

// 509d — raw lang/block
#raw("fn main() {}", lang: "rust", block: true)

// 509e — query com label
#metadata("info") <tag>
#context query(<tag>)
```

Todos passam.

### 3.3 Suite de Testes

- `cargo test`: todos passam.
- `crystalline-lint .`: 0 violations.

---

## 4. Notas e Limitações

- `str.clusters()` devolve chars individuais, não grapheme clusters reais (scope-out do L0 collections.md; unicode-segmentation não é dependência atual).
- `dict.insert()` / `dict.remove()` devolvem um novo dict (dispatch por valor); o dict original não é mutado. Isto é suficiente para o corpus P500, que descarta os valores de retorno.
- `image(fit:)` aceita e valida `"contain"`, `"cover"`, `"stretch"`; o efeito visual de renderização específica depende do layout engine.
- `raw(lang:, block:)` aceita e armazena os argumentos; syntax highlighting real permanece scope-out.

---

## 5. Hashes dos Prompts L0

| Ficheiro L1 | Prompt L0 | Hash (após `--fix-hashes`) |
|---|---|---|
| `01_core/src/entities/value.rs` | `00_nucleo/prompts/entities/value.md` | `04e8ef46` |
| `01_core/src/engine/stdlib/collections.rs` | `00_nucleo/prompts/engine/stdlib/collections.md` | (sem drift) |

---

## 6. Próximo Passo (P510)

Com P509 fechado, as brechas restantes de linguagem são:

| Brecha | Tamanho | Sugestão |
|---|---|---|
| Math styles (`bb`, `bold`, `cal`, etc.) | M | P510 |
| Math elements granulares | M | P511 |
| Table/Grid HLine/VLine | M | P512 |
| Curve elements | M | P513 |
| `fontdb` system discovery | M | **Trilha 5** (produção) |

**Recomendação:** P510 = **Math Styles** — 12 funções de mapeamento de estilo matemático, S/M-size coletivo.
