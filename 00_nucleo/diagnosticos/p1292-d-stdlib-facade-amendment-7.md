# P1292 — amendment-7: fachada stdlib para `native_flush`

**Papel:** autor segregado do contrato + auditor de ownership

**Manifest autorizado:**
`30f5ba462a11a554f060cd58b4e600a07ccb4a2abb16bcd890dbd38a5d81a2d6`

**Predecessor v7:** canônico
`415d8abb1cd5df02fcc08fd2ea90c2d423b67751ef0a09818fb8cc511b225244`,
seal `dc59cefb3aa50cbf38d38b4452aacdddf3b5eeb5a1484d9281206cdc17c65bcc`.

**Proveniência:** `HEAD`
`0eb39f8ecb48930515f2cadb6a378450855b5a72`, working tree não commitada.
Medição em `2026-09-01T02:39:39-03:00`; `git diff HEAD --stat` registrou
37 arquivos, 1.727 inserções e 332 remoções. Após o L0, em
`2026-09-01T02:40:01-03:00`, registrou 38 arquivos, 1.762 inserções e 332
remoções.

## Baselines medidos

```text
00_nucleo/prompts/compiler/stdlib/_comum.md
29aee6d1d5413c0023e6feeb3151bc4642705a556313cdf8f6bac86d92291d7b

01_core/src/compiler/stdlib/mod.rs
02d53b1588b008c295e2dcab0adc0e6ad2e8604d9972c5fa0b2503d340826f3f

00_nucleo/prompts/compiler/stdlib/layout.md
6ff688ec12444582ec9ec87bb7b432019ff568882fbb1c66170eb47367e62dc4
```

O baseline produtivo declara `mod layout` privado. `stdlib/mod.rs` é a fachada
registrada e já reexporta nativas do owner layout para consumidores externos
ao submódulo, mas a lista omite `native_flush`. O preflight D registrado no
manifest provou que `compiler/eval/mod.rs` consome pela fachada
`compiler::stdlib::*`; portanto acessar diretamente o submódulo privado não é
um caminho válido.

`stdlib/layout.md` já especifica integralmente `native_flush`: zero argumentos,
retorno `Content::flush()`, sem efeito de layout. Logo o problema não é um
owner produtivo ausente nem uma nova função; é uma dependência de fachada que
o grafo P1292 de 22 L0s omitiu.

## Decisão e ownership 1:1

O owner `_comum.md` passa a exigir exclusivamente:

```rust
pub(crate) use crate::compiler::stdlib::layout::native_flush;
```

Matriz 1:1:

| Prompt | Consumer | Obrigação |
|---|---|---|
| `compiler/stdlib/layout.md` | `compiler/stdlib/layout.rs` | implementação/construção de `native_flush` |
| `compiler/stdlib/_comum.md` | `compiler/stdlib/mod.rs` | reexport interno mínimo da fachada |

O hub não recebe lógica de negócio, wrapper, lookup, cast ou validação. Não
torna `layout` público, não usa wildcard e não promove `native_flush` à API
Rust externa. A função continua com um único owner produtivo; o novo 23º L0
legitima somente a linha de wiring interna do consumer distinto, conforme
ADR-0129.

Vetores e resultados públicos A-D permanecem inalterados. Em particular,
namespace `place.flush`, argumentos, erros, `repr`, entidade e efeito de fluxo
continuam nos owners anteriores. A correção é interna e segue fluxo contínuo
ADR-0127.

Refutadores: lógica no hub; reexport `pub` externo; mudança em `layout.rs`;
segundo constructor/implementação; acesso direto do consumer ao submódulo
privado; ou alteração de qualquer superfície D. Qualquer deles invalida o
amendment em vez de ampliar silenciosamente o owner.

## Delta L0 e paragem

Único L0 alterado:

```text
00_nucleo/prompts/compiler/stdlib/_comum.md
SHA-256 3d009e06bd08828518297aed8a9b3e5bc580b32a6bcbad65a4b4554aa94f1d1e
consumer 01_core/src/compiler/stdlib/mod.rs
```

Nenhum código, teste, oráculo, ataque ou veredito foi editado. `--fix-hashes`
não foi usado. O implementador recebe somente o reexport `pub(crate)` e a
atualização de linhagem do consumer; qualquer outro delta exige parar.
**PARAGEM.**
