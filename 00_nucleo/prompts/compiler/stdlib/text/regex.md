# Prompt L0 — `compiler/stdlib/text/regex` — `regex`
Hash do Código: efaf44d1

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/text/regex.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/stdlib/text.md` — dono de `text/mod.rs`
e da história por marco (`6e29fbaba`, `87bc1c64d`). Este L0 especifica **a superfície do
nó**.
**Convenções partilhadas**: `00_nucleo/prompts/compiler/stdlib/_comum.md`
**Vanilla**: **não** há ficheiro homólogo em `text/` — o tipo vive em
`foundations/str.rs:1017` (`pub struct Regex(regex::Regex)`) e a função é o construtor do
tipo. No cristalino a nativa nasceu em `stdlib/text.rs` com o suporte de
`#show regex(...)` (`6e29fbaba`, 2026-06-22) e fica neste nó por continuidade de história,
não por correspondência vanilla.

**Fronteira medida**: nó de uma nativa só. Nenhum commit liga `regex` a outra nativa de
`text` fora de lotes transversais (`87bc1c64d` `Value::Symbol`/`StyleDelta`, `04eda8179`
span de `Args`, `0661aef91` `cargo fmt` global, renames) — o único cluster restante,
`lorem`+`regex`+`smartquote` em `6e29fbaba`, é o commit que **criou** `regex` dentro de um
lote com outras matérias. Critério 3 em vácuo; critério 4 aponta para fora de `text/`.

> **Divergência registada, não fechada**: o lugar certo deste nó é o domínio
> `foundations`, junto do tipo. Movê-lo agora arrastaria `stdlib/foundations.rs` (89 KB,
> ainda monolítico) para dentro deste fatiamento. Fica como **candidato explícito do
> fatiamento de `foundations`**: quem o fatiar deve absorver este nó ou declarar por que
> não. Não é deriva silenciosa — é uma fronteira conhecida, adiada com dono.

---

## Contexto

`regex(pattern)` compila uma pattern e devolve-a como valor de primeira classe
(`Value::Regex`). Não produz conteúdo nem estilo: é um **construtor de tipo**, e a única
razão de estar em `text/` é histórica. Os consumidores são o matching de show rules
(`#show regex("..."): ...`) e as operações de string que aceitam padrões.

## Instrução

`regex(pattern)` — 1 posicional `Str`; **zero** nomeados.

- `Str` → `Regex::new(pattern)`; sucesso → `Value::Regex(re)`.
- Pattern inválida → `regex inválida: {erro do motor}`. A mensagem do motor entra
  verbatim: é o observável que diz ao autor do documento onde está o erro na pattern.
- Tipo errado → `regex() espera string, recebeu {tipo}`; contagem errada →
  `regex() requer 1 argumento (pattern)`; qualquer nomeado → erro de `expect_no_named`.
- A compilação é **eager**, na avaliação da chamada — não preguiçosa no primeiro match.
  Uma pattern inválida falha no ponto onde foi escrita, não onde é usada.

## Restrições Estruturais

- L1 puro. `_ctx`/`_world`/`_current_file` são pass-through — nunca usados.
- `Regex` é a entidade L1 `entities/regex.rs` (wrapper do motor); este nó não conhece o
  motor directamente e não expõe as suas opções (case-insensitive, multiline) como
  argumentos — quem as quiser usa a sintaxe `(?i)` da própria pattern, como no vanilla.
- Sem cache de compilação: um `static`/`OnceLock` de patterns compiladas violaria V13.

## Critérios de Verificação

```
#regex("a+")            → Value::Regex compilada
#regex("[")             → Err "regex inválida: ..." (mensagem do motor verbatim)
#regex(1)               → Err "regex() espera string, recebeu integer"
#regex()                → Err "regex() requer 1 argumento (pattern)"
#regex("a", "b")        → Err "regex() requer 1 argumento (pattern)"
#regex("a", foo: 1)     → Err (nomeado inesperado)
#show regex("\d+"): it => strong(it)  → matching aplica-se aos runs que casam
```
