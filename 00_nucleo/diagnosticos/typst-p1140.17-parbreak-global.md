# Diagnóstico P1140.17 — função global `parbreak()`

**Data:** 2026-08-24  
**Estado:** fechado  
**Passo:** `00_nucleo/materialization/typst-passo-1140.17.md`

## Resultado

O binding público `parbreak` foi restaurado sem criar uma nova semântica de
parágrafo. A função nativa atomizada retorna a variante `Content::Parbreak` que
já era produzida pela sintaxe de linha vazia e tratada pelo layout existente.

No binário release da working tree:

```text
repr(type(parbreak))       → "function"
repr(parbreak())           → "parbreak()"
repr(parbreak(1))          → error: unexpected argument
repr(parbreak(foo: true))  → error: unexpected argument: foo
```

## Medição e decisão

A fonte ratificada em
`lab/typst-original/crates/typst-library/src/model/par.rs:697-728` declara o
elemento sem campos. A medição vanilla confirmou função, lista vazia de
parâmetros, representação `parbreak()` e rejeição de argumentos. A leitura do
cristalino confirmou que a lacuna era o constructor/binding público, não o
algoritmo de parágrafo.

O dono confirmou o gate ADR-0127 depois da atualização inicial dos L0s. Durante
o RED→GREEN, um segundo RED revelou também a divergência de `repr`; os L0s de
representação e conteúdo foram atualizados antes da correção correspondente.

Hashes finais normalizados:

- `compiler/stdlib/structural/par.md`: `848b794d`;
- `compiler/stdlib/structural.md`: `50c093bc`;
- `compiler/eval.md`: `2bb85e70`;
- `compiler/stdlib/foundations/repr.md`: `d7574fca`;
- `entities/content.md`: `f17c8a34`.

## Implementação atomizada

`native_parbreak` reside em
`01_core/src/compiler/stdlib/structural/par.rs`, ao lado da função `par`, dona
da mesma unidade vanilla. O hub estrutural apenas reexporta e `make_stdlib`
registra a função. Não houve campo novo em `Content`, mudança de pipeline,
alteração no colapso de quebras ou despacho dinâmico.

## RED→GREEN e validação

O primeiro teste RED falhou com `unknown variable parbreak`. Após registrar o
binding, o segundo RED encontrou `parbreak` em vez de `parbreak()`. Com os L0s
atualizados e ambas as correções aplicadas:

- testes focados de binding, representação e argumentos: aprovados;
- `cargo test -p typst-core --lib`: 5.162 aprovados, zero falhas;
- `cargo build --workspace`: aprovado;
- `cargo build --release --bin typst`: aprovado;
- `crystalline-lint .`: exit 0, sem violations bloqueantes.

## Superfície pública pós-implementação

O inventário reproduzível está em
`00_nucleo/diagnosticos/superficie-linguagem-p1140.17.json`:

| Classe | P1140.16 | P1140.17 | Delta |
|---|---:|---:|---:|
| `MATCH` | 799 | 799 | 0 |
| `UNVERIFIED_METADATA` | 176 | 177 | +1 |
| `MISSING_BINDING` | 4 | 3 | −1 |
| `MISSING_MEMBER` | 1.155 | 1.155 | 0 |
| `EXTRA_BINDING` | 45 | 45 | 0 |

`parbreak` passou para `UNVERIFIED_METADATA` porque o instrumento cristalino
confirma presença e kind, mas não expõe metadados genéricos de parâmetros. O
probe público, registrado em
`00_nucleo/diagnosticos/superficie-linguagem-p1140.17-probes.json`, passou e
elevou o total de equivalências de 15/28 para 16/28.

## Proveniência

- HEAD: `45b547073d7686cdd5d3e3030c82de3e22ec395f`;
- estado: working tree não commitada;
- hora final: `2026-08-24T13:31:30-03:00`;
- `git diff HEAD --stat`: `83 files changed, 734 insertions(+), 505 deletions(-)`;
- vanilla: catálogo do build ratificado `upstream/main a51e02804` reutilizado de
  P1140.16;
- inventário cristalino: binário de inspeção compilado da mesma working tree;
- probe: `target/release/typst` reconstruído às `2026-08-24T13:29:41-03:00`.

Nenhum número acima fecha uma decisão fora desse estado de árvore.
