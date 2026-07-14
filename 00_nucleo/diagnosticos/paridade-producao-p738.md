# P738 — Sonda: `FlowEvent::Return` condicional no fim de `while`/`for`

## Proveniência da medição (regra 2026-07-05)

- **Commit base:** `b686d7d74550d65c84c2f304cbff28786f9cf971` ("P737: preenche hash do commit no relatório")
- **Estado:** working tree sem alterações de código neste passo (sonda pura; só o relatório e o achados são novos).
- **Hora das medições:** 2026-07-13 ~21:08 (-03)
- **Binário vanilla:** `lab/typst-original/target/release/typst`; **cristalino:** `./target/release/typst` (build de P737).

## O que o passo pedia confirmar

P729 encontrou por inspeção que o vanilla marca `FlowEvent::Return` como condicional no fim de `while`/`for` (`flow.rs:105-108,183-185`) e o cristalino só o faz em `if`/`else` (P635). Sem caso medido divergente — este passo tenta construir um.

## Medições

### Caso do passo (return simples dentro de for)

```typst
#let f() = {
  for i in (1, 2, 3) {
    if i == 2 { return "encontrado" }
  }
  "não encontrado"
}
#f()
```

Vanilla → `encontrado`; cristalino → `encontrado`. **Idênticos.**

### Leitura do mecanismo vanilla (o que "condicional" realmente faz)

O flag `conditional` em `FlowEvent::Return(span, value, conditional)` tem **um único consumidor** em todo o `typst-eval`: `warn_for_discarded_content` (`code.rs:413-430`, chamada em `code.rs:62`), que emite a warning "this return unconditionally discards the content before it" **apenas quando `conditional == false`** e o valor joined antes do return é `Content`.

Ou seja: "condicional" não afeta o valor devolvido nem o fluxo de execução — afeta **só** se essa warning dispara.

### Matriz de warning medida (o observável real)

| Caso | Vanilla | Cristalino |
|---|---|---|
| `{ [conteúdo] return "x" }` (return direto) | **warning** "this return unconditionally discards the content before it" | sem warning, exit 0 |
| `{ [conteúdo] for i in (1,2,3) { if i == 2 { return "x" } } }` | sem warning | sem warning, exit 0 |
| `{ [conteúdo] while true { return "x" } }` | sem warning | sem warning, exit 0 |

## Conclusão da sonda

1. **O achado de P729 não tem caso divergente alcançável.** Nos casos onde o flag vanilla muda o comportamento (return dentro de for/while após conteúdo), ambos os compiladores ficam em silêncio — o cristalino porque **não implementa a warning de todo** (grep: "unconditionally discards" ausente; o mecanismo `conditional` nem existe no flow cristalino).
2. A sonda encontrou a divergência **real e adjacente**: o cristalino não emite a warning "this return unconditionally discards the content before it" no caso de return incondicional após conteúdo (caso 1 da matriz). Pré-existente, não é o achado de P729 — registado como **achado novo** na lista de controlo.

## Decisão (conforme o passo)

Sem caso divergente para o achado P729 → **scope-out reforçado**, com a razão agora confirmada por tentativa real e pela leitura do consumidor do flag: o flag `conditional` só alimenta uma warning que o cristalino não implementa; marcar `conditional` em for/while sem a warning não mudaria nenhum comportamento observável.

Sem mudança de código → `cargo test --workspace` não reaplicado (validado em P737: 4755 passed); `crystalline-lint .` → 0 violations.

## Achados

- Item P729 ("Return não marcado como condicional em while/for") → **scope-out reforçado** com esta tentativa registada.
- Novo achado: warning "this return unconditionally discards the content before it" ausente no cristalino (vanilla emite em return incondicional após conteúdo; o hint extra sobre state/counter exige query de seletores — provável scope-out parcial quando implementada).
