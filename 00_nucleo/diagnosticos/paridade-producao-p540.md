# Relatório de Paridade de Produção — P540

**Data:** 2026-07-03  
**Passo:** 540  
**Prompt L0:** `00_nucleo/prompts/engine/eval.md` (hash `2b4597d5`)  
**Dependências:** P538f (`eval_for` acumula conteúdo), P538i (onde o problema foi evitado)

## Objectivo

Suportar destructuring de tuplos em `#for`, permitindo `#for (i, x) in items.enumerate()`.

## Sonda

1. **Parser**: aceita `(i, x)` no `#for` — erro era "unknown variable: x" e não
   erro de sintaxe, portanto a AST é válida.
2. **`eval_for`**: após P538f, acumulava conteúdo mas ainda só definia a
   primeira variável do padrão (`bindings.first()`).
3. **`enumerate()`**: testado isoladamente; devolve tuplos `((0, "a"), (1, "b"), (2, "c"))`.

Conclusão da sonda: o problema está isolado em `eval_for`, que não sabe
atribuir múltiplas variáveis de um tuplo.

## Implementação

`01_core/src/engine/eval/control_flow.rs:76`:

- Se `bindings.len() == 1`, comportamento anterior (atribuir o item inteiro).
- Se `bindings.len() > 1`, cada item do iterável deve ser `Value::Array`;
  verifica que o tamanho coincide com o número de bindings e atribui
  posicionalmente.
- Se `bindings.is_empty()`, não define nada (pattern `_` ou vazio).

Exemplo de atribuição para `#for (i, x) in items.enumerate()`:

```text
item = (0, "um")  →  i = 0, x = "um"
item = (1, "dois") → i = 1, x = "dois"
```

## Prompt L0

`00_nucleo/prompts/engine/eval.md` actualizado para mencionar destructuring de
tuplo em `#for`. Hashes dos ficheiros L1 actualizados via
`crystalline-lint --fix-hashes`.

## Testes

### Teste unitário

`01_core/src/engine/eval/tests.rs` — `p540_for_destructuring_tuplo`:

```typst
#let items = ("um", "dois", "três")
#for (i, x) in items.enumerate() [#str(i) #x]
```

Verifica que o content contém "um", "dois", "três" e os índices 0, 1, 2.

### Teste de corpus

`lab/parity/corpus/p538i/for-with-counter.typ` reescrito para usar
`enumerate()`:

```typst
#let items = ("A", "B", "C")
#for (i, x) in items.enumerate() [
  #str(i+1). #x
]
```

A bateria `structural_parity.rs` verifica que o texto extraído contém A/B/C
e os índices 1/2/3.

### Teste manual

```bash
./target/release/typst /tmp/for-destructure.typ /tmp/fd.pdf
pdftotext /tmp/fd.pdf -
```

Cristalino (usando `#str(i+1). #x`):

```
1 . um
2 . dois
3 . três
```

Vanilla (usando `#{i+1}. #x`):

```
1. um 2. dois 3. três
```

**Nota**: o cristalino não suporta interpolação `#{expr}` dentro de markup;
o teste manual usou `#str(i+1)` como equivalente. O destructuring em si
funciona; a diferença de layout (quebras de linha) é um problema de layout
separado.

### Índice inicial

`items.enumerate()` começa em 0 tanto no cristalino como no vanilla.

## Validação

```bash
cargo test --workspace
cargo test --test structural_parity  # lab/parity
crystalline-lint .
```

Resultado: todos passam; linter limpo.

## Ficheiros alterados

- `01_core/src/engine/eval/control_flow.rs` — destructuring em `eval_for`.
- `01_core/src/engine/eval/tests.rs` — teste `p540_for_destructuring_tuplo`.
- `00_nucleo/prompts/engine/eval.md` — L0 actualizado.
- `01_core/src/engine/eval/*.rs` — hashes actualizados.
- `lab/parity/corpus/p538i/for-with-counter.typ` — usa `enumerate()`.
- `lab/parity/tests/structural_parity.rs` — assert ajustado aos novos valores.
- `00_nucleo/diagnosticos/paridade-producao-p540.md` — este relatório.

## Conclusão

P540 está concluído. `#for (i, x) in items.enumerate()` funciona no
cristalino. O teste de corpus de P538i foi actualizado para usar a forma
original prevista.
