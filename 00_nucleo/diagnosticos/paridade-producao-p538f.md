# Relatório de Paridade de Produção — P538f

**Data:** 2026-07-03  
**Passo:** 538f  
**Prompt L0:** `00_nucleo/prompts/engine/eval.md` (hash `2ef05cee`)  
**Dependências:** P538c (onde o bug foi encontrado), P538e (fallback de fonte)

## Objectivo

Investigar e corrigir o PDF corrompido produzido por `#for` com corpo longo dentro de `#set page(columns: 2)`.

## Sonda

### Reprodução

Documento de teste:

```typst
#set page(columns: 2)
#for i in range(0, 80) [
  #{i+1}. #lorem(20)
]
```

Resultado antes da correcção:

```
Internal Error: xref num 3 not found but needed, try to reconstruct
Syntax Error: Kid object (page 1) is wrong type (null)
Pages: 1
Page size: 0 x 0 pts
```

Inspecção do PDF revelou um catalog `/Pages` com `/Kids [3 0 R]`, mas o objecto 3 nunca foi escrito. O layout gerou zero páginas reais.

### Redução

Testes com `range(0, n)` para `n = 1, 2, 10, 20, 40, 80` mostraram que **qualquer `#for`** reproduz o problema, mesmo sem colunas e mesmo com corpo mínimo:

```typst
#for i in range(0, 2) [A]
```

Conclusão: o bug **não é específico de colunas nem do tamanho do corpo** — é do `#for` em geral.

### Causa raiz

Em `01_core/src/engine/eval/control_flow.rs:65`, `eval_for` avaliava o corpo de cada iteração mas **descartava o resultado**:

```rust
eval_expr(loop_expr.body(), scopes, ctx, engine)?;
```

O `eval_for` retornava `Ok(Value::None)` sempre. Quando o corpo do `for` é conteúdo markup `[...]`, esse conteúdo era perdido. O documento resultante estava vazio, e o exportador gerou uma estrutura de páginas inconsistente (Catalog aponta para página inexistente).

## Implementação

`eval_for` agora acumula os valores produzidos pelo corpo de cada iteração:

- `Value::Content(c)` → adiciona `c` à lista de partes.
- `Value::Str(s)` → converte para `Content::text` e adiciona.
- `Value::None` → ignorado (mantém compatibilidade com corpos vazios).
- Outros tipos → erro claro.

No final, se houver partes, retorna `Value::Content(Content::sequence(parts))`; senão, `Value::None`.

### Prompt L0

O L0 `00_nucleo/prompts/engine/eval.md` foi actualizado para refletir a semântica correcta de acumulação de conteúdo no `#for`:

> `Expr::ForLoop(loop)` → eval_for: iterable() (não iter()), pattern().bindings(), body(); cada iteração avalia o corpo e concatena os valores `Content`/`Str` produzidos numa `Content::sequence`; `Value::None` no corpo é ignorado; `Value::None` como iterable é iterável vazio.

Os hashes dos ficheiros L1 ligados a este L0 foram actualizados com `crystalline-lint --fix-hashes`.

## Ficheiros alterados

- `00_nucleo/prompts/engine/eval.md` — especificação do comportamento de `#for`.
- `01_core/src/engine/eval/control_flow.rs` — `eval_for` acumula conteúdo.
- `01_core/src/engine/eval/*.rs` — hashes do prompt actualizados pelo linter.
- `01_core/src/engine/eval/tests.rs` — teste `p538f_for_acumula_conteudo_do_corpo`.

## Testes automáticos

Comandos executados:

```bash
cargo test --workspace
crystalline-lint .
```

Resultado: todos os testes passam; linter limpo.

Teste novo:

- `p538f_for_acumula_conteudo_do_corpo` — verifica que `#for i in range(3) [A]` produz content com "A" e que o layout gera pelo menos uma página.

## Teste manual

Documento original de 80 iterações em colunas:

```bash
./target/release/typst /tmp/for-cols-test.typ /tmp/for-cols.pdf
pdfinfo /tmp/for-cols.pdf
```

Resultado:

```
Pages:           6
Page size:       595.28 x 841.89 pts (A4)
```

PDF válido, sem erros de xref.

## Sem regressões

- Testes de P538c (`p538c_set_page_columns_fluxo_continuo_*`) continuam a passar.
- Snapshot test `p307b_07_multi_feature` passa.
- Caso P534 (`#set text(font: "DejaVu Sans")` com texto misto) continua a funcionar.

## Conclusão

P538f está concluído. A causa raiz era o descarte do conteúdo do corpo do `#for` em `eval_for`. A correcção acumula esse conteúdo numa `Content::sequence`, tornando documentos com loops renderizáveis e produzindo PDFs válidos.
