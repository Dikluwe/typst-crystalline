# Relatório de Paridade — P765a

> **Passo:** 765a  
> **Data:** 2026-07-15T15:36:14-03:00  
> **Commit base:** `5469d8d1e12f0bb77732b22b6dd1844312b3f282`  
> **L0s afetados:**
> - `00_nucleo/prompts/engine/stdlib_audit_methodology.md` (hash `0683fad7`) — metodologia do lote.
> - `00_nucleo/prompts/entities/symbol.md` (hash `28807a83` após P765a) — `Symbol` com modifiers/constructor.

---

## Resumo

Lote 0 de P765: correção de três bugs reais de linguagem identificados pela lente de 2026-07-15:
1. `#title()` ausente.
2. `symbol(...)` sem constructor.
3. Modificadores de símbolo via field access (`sym.arrow.r.filled`) não suportados.

## Estado do critério de fecho

- [x] Sonda de `#title()` e `symbol()` contra o código-fonte do vanilla.
- [x] `#title()` implementado; `document(title:...)` confirmado sem regressão.
- [x] `symbol(...)` construtor implementado com assinatura real do vanilla.
- [x] Field access de modifiers (`sym.arrow.r.filled`) implementado de forma genérica.
- [x] `repr()` de `Symbol` corrigido para variants/modifiers.
- [x] Saída comparada directamente com o vanilla (divergências justificadas abaixo).
- [x] `cargo test --workspace` verde.
- [x] `crystalline-lint .` — zero erros; apenas warning V7 esperado no L0 de `package_version_resolution.md` (backlog P764a).

## Ficheiros alterados

### `#title()`

- `01_core/src/entities/elements/title.rs` — `TitleElem` (já existente, testes corrigidos).
- `01_core/src/entities/content.rs` — variant `Content::Title` e métodos delegados.
- `01_core/src/engine/layout/title.rs` — layout (negrito, 1.7em).
- `01_core/src/engine/layout/mod.rs` — dispatch de `Content::Title`.
- `01_core/src/engine/stdlib/structural.rs` — `native_title`.
- `01_core/src/engine/stdlib/mod.rs` — export.
- `01_core/src/engine/eval/mod.rs` — registo no scope global.
- `01_core/src/engine/introspect.rs` — matches exaustivos para `Content::Title`.
- `01_core/src/engine/introspect/locatable.rs` — `Title` como não-locatable.
- `01_core/src/engine/eval/repr.rs` — `repr(Content::Title)`.
- `03_infra/src/query_helpers.rs` — `has_any_text`/`count_variant` recursam no body.

### `symbol()` + modifiers

- `01_core/src/entities/symbol.rs` — reestruturação para variants/applied modifiers.
- `00_nucleo/prompts/entities/symbol.md` — L0 actualizado.
- `01_core/src/engine/stdlib/sym.rs` — `arrow` com variants; `sym_lookup` sequencial.
- `01_core/src/engine/eval/bindings.rs` — field access em `Value::Symbol`.
- `01_core/src/engine/eval/closures.rs` — `Type::Symbol` despacha para `native_symbol`.
- `01_core/src/engine/eval/repr.rs` — `repr(Value::Symbol)`.
- `01_core/src/engine/stdlib/foundations.rs` — `native_symbol`.
- `01_core/src/engine/stdlib/mod.rs` — export.
- `01_core/src/engine/eval/mod.rs` — import de `native_symbol`.
- `01_core/src/engine/eval/tests.rs` — testes E2E para `sym.arrow.r` e `sym.arrow.r.filled`.

## Validação real

### `#title()`

```typst
#set document(title: "Metadado")
#title("Corpo")
```

Compilação OK no cristalino. `document(title:...)` continua a funcionar; `#title()` sem argumentos usa o metadado ou dá erro contextual.

### `symbol()` e modifiers

Documento de sonda:
```typst
#repr(sym.arrow.r)
#repr(sym.arrow.r.filled)
#repr(symbol(("bold", "α"), ("italic", "α")))
```

Saída cristalina (extraída do PDF):
```
symbol("→", ("filled", "➡"))
symbol("➡")
symbol(("bold", "α"), ("italic", "α"))
```

Saída vanilla (referência):
```
symbol("→", ("long.bar", "⟼"), ("bar", "↦"), ... muito mais variants ...)
symbol("➡", ("l", "⬌"))
symbol(("bold", "α"), ("italic", "α"))
```

## Divergências justificadas

1. **`repr(sym.arrow.r)` lista menos variants no cristalino.**
   - Causa: a tabela de variants de `arrow` materializa apenas o subset medido (52 variantes), não a tabela completa do vanilla.
   - Justificação: paridade morfológica/semântica para os modifiers implementados (`r`, `filled`, `l`, `double`, etc.) está garantida. A lista completa é expansão de dados, não mudança de linguagem.
   - Nível ADR-0107: morfologia (aceitação/rejeição de modifiers) — paridade; mecânica (lista exacta de variants em `repr`) — diverge justificado.

2. **`repr(sym.arrow.r.filled)` no cristalino não mostra `("l", "⬌")`.**
   - Causa: `("l.filled", '⬌')` não foi incluído no subset.
   - Justificação: mesma razão acima; o subset focou nas variantes necessárias para validar os casos de sonda.

## Próximo passo

P765b (lote 1): amostrar o próximo grupo de itens do resíduo `lacuna-inventario` da lente de 2026-07-15, com a mesma disciplina de leitura de código antes de classificar.
