# Prompt L0 — `compiler/stdlib/structural/lists` — nativas de lista
Hash do Código: 8926c761

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/structural/lists.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/stdlib/structural.md` — dono de
`structural/mod.rs` e da história acumulada destas nativas (marcos P69…P962). Este L0
especifica **a superfície do nó**; o detalhe por marco vive no pai.
**Convenções partilhadas**: `00_nucleo/prompts/compiler/stdlib/_comum.md`
**Vanilla**: `typst-library/src/model/{list,enum,terms}.rs`. A co-mudança agrega-as: P494-P495 e P505 movem `native_list` e `native_enum` sempre em conjunto.

---

## Contexto

As três formas de lista da língua. `list` e `enum` partilham a totalidade dos parâmetros
de indentação (`indent`, `body-indent`, `tight`) e a mesma validação; `terms` é a forma
por pares chave→descrição.

## Instrução

| Nativa | Assinatura |
|---|---|
| `list` | `list(..items, marker: ?, marker-align: ?, indent: ?, body-indent: ?, tight: ?)` |
| `enum` | `enum(..items, numbering: ?, start: ?, indent: ?, body-indent: ?, tight: ?)` |
| `terms` | `terms(named: descrição, …)` — a **ordem dos argumentos nomeados é preservada** |

Validação partilhada (P505): `indent`/`body-indent` têm de ser comprimentos; `tight` tem
de ser booleano; nome nomeado desconhecido é erro. `enum` propaga `start` para a
numeração.

## Restrições Estruturais

- L1 puro.
- `terms` depende da ordem de inserção dos nomeados — não reordenar nem usar mapa não
  ordenado.

## Critérios de Verificação

```
- a
- b                          → list com 2 itens, sem marker
+ a
+ b                          → enum com 2 itens
#enum(start: 3)[a]           → numeração começa em 3
#list(indent: 1)             → Err (não é comprimento)
#list(tight: 1)              → Err (não é bool)
#list(zz: 1)                 → Err (nomeado desconhecido)
#enum(numbering: 1)          → Err
#list()                      → lista vazia
```
