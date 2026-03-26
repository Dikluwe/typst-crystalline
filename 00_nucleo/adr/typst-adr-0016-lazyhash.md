# ⚖️ ADR-0016: `typst_utils::LazyHash` → removido de L1

**Status**: `IMPLEMENTADO`
**Data**: 2026-03-25

---

## Contexto

`source.rs` do original usa `Arc<LazyHash<SourceInner>>` como
representação interna de `Source`. `LazyHash<T>` é um wrapper de
`typst_utils` que:

1. Armazena um `T` com um lock interno para o hash
2. Calcula `hash128(value)` na primeira invocação de `Hash::hash()`
   e cacheia o resultado
3. Implementa `PartialEq`/`Eq` baseado no hash (não field-by-field)

O objectivo é evitar recomputação de hash em comparações frequentes —
optimização relevante quando `comemo` rastreia `Source` em ciclos
de compilação incremental.

---

## Problema

`typst_utils::LazyHash` não está coberto pelos ADRs 0006–0015.
Adicioná-lo a `[l1_allowed_external]` criaria uma dependência de L1
num mecanismo de performance de infraestrutura.

`LazyHash` não é um conceito de domínio — é uma optimização de
hashing para uso com `comemo`. O domínio de `Source` é:
"ficheiro de texto com CST associada". O mecanismo de hash é
infraestrutura.

---

## Análise

| Opção | Descrição | Consequência |
|-------|-----------|--------------|
| A — Remover LazyHash | `Arc<SourceInner>` directamente; sem `Hash`/`Eq` em `Source` | Hash recomputado quando necessário; `comemo` resolvido no Passo 10 |
| B — Autorizar typst_utils | Adicionar a `[l1_allowed_external]` | L1 dependente de crate de infraestrutura |
| C — Hash em construção | Adicionar campo `content_hash: u64` a `SourceInner` | Hash correcto e puro; custo único na construção |

---

## Decisão

**Opção A — Remover LazyHash de L1.**

`Source` em L1 usa `Arc<SourceInner>` sem `LazyHash`. `Hash` e
`PartialEq` não são derivados em `Source` neste passo — os testes
do Passo 5 não os requerem.

Quando `comemo` precisar de rastrear `Source` (Passo 10),
a decisão será entre Opção C (campo de hash) ou delegar o
hashing ao wrapper de comemo em L3.

`typst_utils` **não entra** em `[l1_allowed_external]`.

---

## Consequências

**Positivas**: L1 sem dependência de crate de infraestrutura;
`Source` é um tipo de domínio puro.

**Negativas**: Sem `Hash`/`Eq` em `Source` por enquanto — limitação
aceitável até Passo 10 onde `comemo` é isolado em L3 (ADR-0001).

**Neutras**: A interface pública de `Source` (métodos `new`,
`detached`, `id`, `text`, `root`, `len_bytes`) não é afectada.

---

## Referências

- ADR-0001 — isolamento de `comemo` em L3 no Passo 10
- ADR-0005 — `Source` stub e contrato de `World`
- Diagnóstico Passo 5 — `LazyHash` encontrado em `source.rs`
