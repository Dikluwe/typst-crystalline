# Prompt L0 — `compiler/eval/font_dict` — parsing do argumento `text.font`
Hash do Código: b7ea0701

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/eval/font_dict.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/eval.md` (dono de `compiler/eval/rules.rs`)
**ADRs relevantes**: ADR-0107 (paridade língua), ADR-0108 (medir antes de decidir), ADR-0109 (atomização)
**Técnica**: parsing de dict com fallback legacy

---

## Contexto

Este nó contém o parsing do argumento `font` de `#set text(...)`. O valor pode ser:

- uma string ou array de strings (tratado noutro sítio);
- um dict no formato **named fields** do vanilla (`(family: "…", variant: "…", weight: "…", style: "…", fallback: true)`) — introduzido em P414;
- um dict no formato **legacy** cristalino (`("Name": "Regular")` ou `("Name": ("Regular", "Bold"))`) — introduzido em P407.

O nó é chamado pelo hub `rules.rs` durante o processamento do set-rule de `text.font`. Extraído de `compiler/eval/rules.rs` no Passo 1011 conforme ADR-0109 (atomização — forma B, free function no arquivo da unidade).

---

## Instrução

### 1. Contrato público

```rust
pub(crate) fn parse_font_dict_named_fields<'a>(
    dict_node: Dict<'a>,
    span: Span,
    scopes: &mut Scopes,
    ctx: &mut EvalContext,
    engine: &mut Engine,
) -> SourceResult<Vec<Value>>;

pub(crate) fn parse_font_dict_legacy<'a>(
    dict_node: Dict<'a>,
    span: Span,
    scopes: &mut Scopes,
    ctx: &mut EvalContext,
    engine: &mut Engine,
) -> SourceResult<Vec<Value>>;

pub(crate) fn variants_from_value(
    value: Value,
    span: Span,
) -> SourceResult<Vec<Value>>;
```

- `parse_font_dict_named_fields` — converte um dict named fields num único `Value::Dict` com as chaves `name`, `variants`, e opcionalmente `variant`, `weight`, `style`.
- `parse_font_dict_legacy` — converte um dict legado num array de `Value::Dict`, um por família, cada um com `name` e `variants`.
- `variants_from_value` — helper partilhado que valida que o valor é uma string ou array de strings e devolve `Vec<Value::Str>`.

As duas funções de parsing recebem `Scopes`, `EvalContext` e `Engine` porque os valores dos campos/values são expressões AST que têm de ser avaliadas via `eval_expr`. Mecanicamente, o nó **não** é `Value → Value` puro: `eval_expr` recebe `&mut EvalContext` e `&mut Engine` e pode propagar esses mutáveis para avaliação de funções/closures com efeito lateral (sink, counters, state, warnings, active_guards). A separação do nó é sustentada pelo Critério 3 (co-mudança histórica, confirmada no Passo 1009), não por pureza. O propósito do nó continua a ser a normalização da sintaxe de `text.font`, mas a classificação é *stateful*, como o resto do hub `rules.rs`.

### 2. Comportamento

Manter exatamente o comportamento actual:

- Named fields: rejeitar chaves que não sejam identificadores; validar tipos de `family`, `variant`, `weight`, `style`, `fallback`; exigir `family`; rejeitar campos desconhecidos.
- Legacy: aceitar chaves string, identificador ou `regex("…")`; valores string ou array de strings; rejeitar `DictItem::Spread`; rejeitar dict vazio.
- `fallback: false` não gera erro — a semântica de fonte única é respeitada pelo layout através da lista resultante.

### 3. Gatilhos de reabertura

- Novo formato de especificação de fonte no vanilla.
- Alteração ao conjunto de campos named fields suportados.
- Mudança de fase (eval ↔ layout) no processamento de `text.font`.

---

## Critérios de verificação

```
Dado `#set text(font: (family: "Foo", variant: "Bar"))` → dict named fields válido
Dado `#set text(font: ("Foo": "Bar", "Baz": ("Regular", "Bold")))` → array de dicts legacy válido
Dado dict vazio legacy → erro "font dict must not be empty"
Dado campo desconhecido em named fields → erro "unknown font dict field"
```

Aplicação final: `cargo build && crystalline-lint .` — zero violations.
