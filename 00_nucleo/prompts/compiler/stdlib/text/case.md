# Prompt L0 — `compiler/stdlib/text/case` — `upper`, `lower`, `replace`
Hash do Código: 4382da9f

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/text/case.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/stdlib/text.md` — dono de `text/mod.rs`.
Este L0 especifica **a superfície do nó**.
**Convenções partilhadas**: `00_nucleo/prompts/compiler/stdlib/_comum.md`
**Vanilla**: `typst-library/src/text/case.rs` (`upper`/`lower`); `replace` vive em
`foundations/str.rs` no vanilla — no cristalino agrega-se aqui porque partilha o motor
`map_text` e a história: `d2faea55d`, `fdd39b892` e `1aea00338` (2026-04-23) movem as três
sempre juntas e sozinhas.

> **Correcção de deriva**: estas três nativas estavam materializadas desde `d2faea55d` mas
> **nunca especificadas** em L0 (a nota "deriva (F4)" do L0 pai reconhecia a lacuna sem
> a fechar). Este ficheiro fecha-a.

---

## Contexto

As três nativas que transformam texto **sem introduzir elemento novo**: aceitam
`Str` ou `Content` e devolvem o mesmo tipo transformado. O motor comum é
`Content::map_text(&mut F)`, que percorre os nós de texto do conteúdo e aplica a
closure a cada um, preservando a estrutura à volta.

## Instrução

| Nativa | Assinatura | Devolve |
|---|---|---|
| `upper` | `upper(str \| content)` | mesmo tipo, texto em maiúsculas |
| `lower` | `lower(str \| content)` | mesmo tipo, texto em minúsculas |
| `replace` | `replace(fonte, padrão, substituição, count: ?)` | mesmo tipo da fonte |

- `upper`/`lower`: exactamente 1 posicional, zero nomeados. `Str` → `Value::Str`
  transformado; `Content` → `map_text` com `to_uppercase`/`to_lowercase`.
- `replace`: 3 posicionais (`fonte: Str | Content`, `padrão: Str`, `substituição: Str`)
  + nomeado opcional `count: Int`. Qualquer outro nomeado → erro.
- **Padrão vazio é rejeitado** — `replacen("", …)` entraria em ciclo infinito.
- **`count` é global ao documento, não por nó de texto**: o contador vive numa closure
  `FnMut` capturada por `map_text`, portanto o orçamento de substituições decresce à
  medida que os nós são visitados e esgota-se uma vez só. Quando chega a zero, os nós
  restantes passam intactos. Sem `count`, substitui todas as ocorrências.

## Restrições Estruturais

- L1 puro. `_ctx`/`_world`/`_current_file` são pass-through — recebidos pela assinatura
  uniforme das nativas e nunca usados.
- O estado de `count` é **local à chamada** (variável capturada), nunca `static` —
  L1 proíbe estado global mutável (V13).
- `replace` não aceita `Regex` como padrão neste nó (só `Str`); a substituição por
  padrão regex é o eixo de `#show regex(...)`, servido pelo nó `regex`.

## Critérios de Verificação

```
#upper("ab")                      → Str("AB")
#upper[a *b*]                     → Content com texto em maiúsculas, ênfase preservada
#upper(1)                         → Err "upper() espera string ou content, recebeu integer"
#upper()                          → Err "upper() requer 1 argumento"
#upper("a", "b")                  → Err
#lower("AB")                      → Str("ab")
#replace("aaa", "a", "b")         → Str("bbb")
#replace("aaa", "a", "b", count: 2) → Str("bba")
#replace([a a], "a", "b", count: 1) → só a primeira ocorrência do documento inteiro
#replace("a", "", "b")            → Err (padrão vazio)
#replace("a", "b")                → Err (requer 3 posicionais)
#replace("a", "b", "c", foo: 1)   → Err (nomeado inesperado)
```
