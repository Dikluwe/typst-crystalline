# Prompt L0 — `stdlib/foundations` — hub de reexportação
Hash do Código: 2b77985d

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/foundations/mod.rs`
**Origem**: Passo 1032 — fatiamento de `foundations.rs` monolítico conforme
`auditar-fatiamento.md`. P1140.1-A cria unidades próprias para `regex` e
`selector`.
P1077: remoção da extensão `len` global para paridade estrita com o Typst oficial.
**ADRs**: ADR-0037 (coesão por domínio), ADR-0107 (paridade linguagem),
ADR-0109 (atomização forma B), ADR-0127 (gate de L0).
**Convenções partilhadas**: ver `00_nucleo/prompts/compiler/stdlib/_comum.md`.

---

## 1. Visão geral

O módulo `foundations` é um **hub de reexportação** sem lógica de negócio.
A lógica vive nos nós abaixo; o hub apenas declara os submódulos e reexporta
as funções para manter compatibilidade com os consumidores existentes
(`crate::compiler::stdlib::native_*`).

## 2. Nós

| Nó | Ficheiro | Funções |
|---|---|---|
| `ty` | `foundations/ty.rs` | `native_type` |
| `repr` | `foundations/repr.rs` | `native_repr` |
| `str` | `foundations/str.rs` | `native_str`, `native_str_from_unicode` |
| `regex` | `foundations/regex_constructor.rs` | `native_regex` |
| `cast` | `foundations/cast.rs` | `native_int`, `native_float`, `native_range`, `native_bytes`, `native_datetime`, `native_symbol` |
| `datetime` | `foundations/datetime.rs` | fields estáticos `today`, `display`, `year`, `month`, `weekday`, `day`, `hour`, `minute`, `second`, `ordinal` |
| `int` | `foundations/int.rs` | `int_type_field`, signum, bitwise e conversões bytes |
| `color` | `foundations/color.rs` | `native_rgb`, `native_luma`, `native_oklab`, `native_oklch`, `native_linear_rgb`, `native_cmyk`, `native_hsl`, `native_hsv` |
| `query` | `foundations/query.rs` | `native_metadata`, `native_query`, `native_locate`, `native_here`, `native_target` |
| `selector` | `foundations/selector.rs` | `native_selector`, parser partilhado de selectors |

> **Nota P1077**: O nó `len` (`len.rs` / `native_len`) foi **removido** no Passo 1077 (Achado #12 do P1031). A linguagem Typst oficial não possui a função global `len` (emite erro `unknown variable`), possuindo exclusivamente o método `.len()` sobre strings (contando bytes UTF-8), arrays e dicionários.

## 3. Restrições estruturais

- O hub **não contém lógica de negócio** — só `pub mod` e `pub use`.
- Cada nó tem o seu próprio prompt L0 em
  `00_nucleo/prompts/compiler/stdlib/foundations/<nó>.md`.
- A visibilidade e as assinaturas das funções são preservadas.

## P1146 — atomização da superfície de `datetime`

A medição do scope ratificado encontrou dez fields além do constructor. O
constructor permanece em `cast`; fields e sua semântica formam o nó coeso
`foundations/datetime`. O hub apenas declara/reexporta o nó e não implementa
calendário, formatting ou acesso ao World.

## 4. Critérios de verificação

- `cargo build -p typst-core` sem erros.
- `cargo test -p typst-core` mantém a contagem de `#[test]` do hub original.
- `crate::compiler::stdlib::native_*` continua a resolver para a função
  correspondente.

## P1140.2 — `repr` de valores `label`

Medição vanilla: `repr(label("x")) == "<x>"` e
`repr(label("a b")) == "label(\"a b\")"`. O cristalino anterior só exercia
a primeira forma via sintaxe literal. `compiler/eval/repr.rs`, cujo L0 vigente
é este hub, deve escolher `<nome>` apenas quando o nome satisfaz a gramática
de identificador literal de label; caso contrário, usa `label(<repr string>)`.
A função de validade é pura e tem um único dono, sem regex duplicada.

## P1140.3-A — `repr` de raiz matemática

**Medição anterior à decisão:** no vanilla ratificado,
`repr(math.sqrt([x])) == "root(radicand: [x])"`. O cristalino anterior emitia
`"sqrt([x])"`, confundindo a função produtora com a morfologia do elemento.

`Content::MathRoot` representa-se como elemento `root`: sem índice,
`root(radicand: <repr>)`; com índice mantém provisoriamente a forma vigente
`root(<index>, <radicand>)` até medição específica. Esta correção afeta apenas
`repr`; `plain_text` continua `sqrt(...)`/`root(i, ...)` e o layout não muda.
