# Prompt L0 — `compiler/stdlib/text/regex` — `regex` (MOVIDO)
Hash do Código: n/a (redirecionamento histórico — código movido para `foundations/str.rs`)

> **⚠️ Nota de redirecionamento — Passo 1032**
> Este nó foi **movido** de `text/regex.rs` para `foundations/str.rs`.
> O prompt L0 vigente é agora `00_nucleo/prompts/compiler/stdlib/foundations/str.md`.
> Este ficheiro permanece apenas como registo histórico da decisão do Passo 1022.

**Camada**: L1
**Ficheiro alvo**: ~~`01_core/src/compiler/stdlib/text/regex.rs`~~ →
`01_core/src/compiler/stdlib/foundations/str.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/stdlib/foundations.md`
**Origem**: Passo 1022 (nascimento em `text/regex.rs`); Passo 1032 (absorção pelo
 domínio `foundations`).
**Convenções partilhadas**: `00_nucleo/prompts/compiler/stdlib/_comum.md`
**Vanilla**: o tipo `Regex` e a função `regex(pattern)` vivem em
`foundations/str.rs:1017` do vanilla. A colocação cristalina agora espelha essa
fronteira.

**Fronteira medida**: nó de uma nativa só. Nenhum commit liga `regex` a outra nativa de
`text` fora de lotes transversais. Critério 3 em vácuo; critério 4 aponta para fora de
`text/`.

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
