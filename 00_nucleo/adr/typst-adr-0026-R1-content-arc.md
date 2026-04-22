# ⚖️ ADR-0026 (Revisão): Content cristalino — actualização de ADR-0026

**Status**: `IMPLEMENTADO`
**Revê**: ADR-0026
**Data**: 2026-03-29
**Motivação**: ADR-0029 (pureza física) e ADR-0030 (performance é domínio)

---

## O que a ADR-0026 original decidiu correctamente

- `Content` como enum linear — divergência intencional do original (vtable + proc macros)
- Paridade funcional (mesmo output de texto para o mesmo input) como critério
- Enum pode crescer linearmente sem vtable

Estas decisões mantêm-se.

---

## O que a ADR-0026 original adiou incorrectamente

A ADR-0026 original registou:

> "Se `Content` deve eventualmente ter `Arc` para clone O(1) de sequências
> grandes — registado como candidato em DEBT.md"

Sob a ADR-0030, esta formulação é incorrecta: "eventualmente ter `Arc`"
trata a performance como dívida opcional. Pelo contrário, `Arc` em
sequências pesadas é parte do comportamento correcto de L1 — não uma
optimização futura.

---

## Decisão de actualização

### `Content::Sequence` deve usar `Arc<[Content]>` ou equivalente

A representação actual `Sequence(Vec<Content>)` tem clone O(n). Em
percursos de layout onde a mesma sequência é visitada múltiplas vezes
(ou passada entre ramos de eval), o custo é proporcional ao tamanho
do documento.

A representação correcta:

```rust
// Opção A — Arc<[Content]> para sequências imutáveis após construção
Sequence(Arc<[Content]>),

// Opção B — EcoVec<Content> se disponível via ecow
Sequence(ecow::EcoVec<Content>),  // clone O(1) com copy-on-write

// Opção C — Box<[Content]> se a sequência nunca é partilhada
// (clone seria O(n) — não serve para este caso)
```

A escolha entre A e B depende do diagnóstico: se `Content` é partilhado
entre múltiplos proprietários durante eval(), usar Opção A. Se é sempre
propriedade única mas clonado ao passar entre funções, Opção B (EcoVec)
é mais adequada.

### Quando implementar

Não requer refactorização imediata. A mudança deve acontecer antes
do Passo 30 (StyleChain), quando o percurso de layout sobre `Content`
se tornará mais intensivo.

**Critério de implementação**: quando um teste de performance ou profiling
mostrar que clone de `Sequence` aparece no hot path, ou antes do Passo 30
— o que ocorrer primeiro.

### `Box<Content>` em variantes recursivas — mantido

Variantes como `Strong(Box<Content>)`, `Heading { body: Box<Content> }`
usam `Box` porque têm um único proprietário. Correcto e sem alteração.

---

## Actualização da entrada em DEBT.md

A entrada actual (se existir):
```
Content::Sequence: Vec<Content> tem clone O(n). Candidato a Arc.
```

Deve ser actualizada para:
```
Content::Sequence: migrar Vec<Content> para Arc<[Content]> ou EcoVec<Content>
antes do Passo 30 (StyleChain). Ver ADR-0026 revisão e ADR-0030.
Não é dívida opcional — é requisito de performance de domínio.
```

---

## Consequências

**Positivas**:
- Alinha `Content` com a definição de pureza física (ADR-0029).
- Remove ambiguidade sobre se `Arc` em `Content` é "dívida" ou "requisito".
- Estabelece o critério de implementação antes que o Passo 30 force a mudança.

**Negativas**:
- `Arc<[Content]>` perde `PartialEq` derivado (slices comparáveis, mas
  `Arc` compara por ponteiro por defeito). Requer implementação manual
  de `PartialEq` que compara o conteúdo — não o ponteiro.
- `EcoVec` pode não estar disponível se `ecow` não estiver já como dep.

**Neutras**:
- `Content::sequence()` (o construtor normalizador) não muda a interface —
  apenas a representação interna muda.

---

## Referências

- ADR-0026 original — decisões base mantidas (enum, divergência do original)
- ADR-0029 — Arc em campos de struct é permitido e exigido
- ADR-0030 — performance de RAM é domínio de L1
- DEBT-1 — StyleChain (Passo 30) — contexto onde a mudança se tornará urgente
