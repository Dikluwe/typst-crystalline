# Passo 207B — `query_labelled` trait method

**Série**: 207 (sub-passo `B` após P207A diagnóstico).
**Marco**: M9c (M9-completion).
**Tipo**: implementação focada (Bloco I — trait baixo
custo).
**Magnitude**: S (~1-2h).
**Pré-condição**: P207A concluído; ADR-0076 PROPOSTO;
`P207A.div-1` aprovado; tests 1873 verdes; 0 violations.
**Output**: 1 ficheiro (relatório curto combinando
achados + alterações de código + decisões).

---

## §1 Trabalho

Materializar `query_labelled` trait method (item 5 da
auditoria P207A).

Reuso de dados P207A (sem re-auditar):

- `Introspector` trait em
  `01_core/src/entities/introspector.rs:41-179`
  (@prompt-hash `918d279b`).
- `LabelRegistry` em
  `01_core/src/entities/label_registry.rs:23` (115L).
- L0 prompts existentes:
  - `00_nucleo/prompts/entities/introspector.md`.
  - `00_nucleo/prompts/entities/label_registry.md`.

---

## §2 Cláusulas (4)

### C1 — Verificação curta de pré-condições

Antes de tocar código, confirmar:

1. `LabelRegistry` ainda tem API minimal (sem
   iterator/all() público) — único sinal que P207A A4
   ainda é válido.
2. Trait `Introspector` ainda tem 20 métodos
   (P207A A1).

Se algum falhar, registar `P207B.div-N`. Caso contrário,
prosseguir directo.

### C2 — Materializar `query_labelled`

**No L0 prompt `introspector.md`** primeiro, depois no
código L1.

Edição L0 (`00_nucleo/prompts/entities/introspector.md`)
— adicionar entrada para método novo na lista dos
métodos do trait com:

- Assinatura: `fn query_labelled(&self) -> Vec<(Label, Location)>`.
- Semântica: retorna todos os labels registados com a
  respectiva location.
- Origem: P207B (M9c).
- Paralelo vanilla: `Introspector::query_labelled() -> EcoVec<Content>`
  (vanilla retorna `Content`; cristalino retorna handles
  `(Label, Location)` per ADR-0073/0074 design pattern).

Edição L0 (`00_nucleo/prompts/entities/label_registry.md`)
— adicionar entrada para método novo:

- Assinatura: `pub fn iter(&self) -> impl Iterator<Item = (&Label, &Location)>`.
- Razão: expor iteração ordenada do registry para
  consumers que precisem listar todos os labels.

Edição L1 (`01_core/src/entities/label_registry.rs`)
— adicionar `pub fn iter`.

Edição L1 (`01_core/src/entities/introspector.rs`)
— adicionar `fn query_labelled` ao trait + impl em
`TagIntrospector` delegando a `self.labels.iter()` +
collect com clones.

### C3 — Tests dedicados

Adicionar 3-4 tests em `introspector.rs::tests`:

- `p207b_query_labelled_vazio` — empty intr retorna
  empty Vec.
- `p207b_query_labelled_um_label` — 1 label registado;
  query retorna `[(label, location)]`.
- `p207b_query_labelled_multiplos_ordem_de_insercao` —
  ordem documentada (insertion order via
  HashMap não-determinístico → ajustar test conforme
  decisão).

### C4 — Verificação final

```
cargo test --workspace 2>&1 | tail -10
crystalline-lint .
crystalline-lint --fix-hashes . # sincronizar L0 hashes
```

Critério: 1876+ verdes (1873 + 3 P207B); 0 violations.

Anotar ADR-0076 §P207B: `✅ MATERIALIZADO {data}`.

---

## §3 Output

**1 ficheiro**:
`00_nucleo/materialization/typst-passo-207B-relatorio.md`.

Estrutura conciso (~3-5 KB):

- §1 O que foi feito (sumário 3-5 linhas).
- §2 Alterações em código (lista compacta de ficheiros
  + edição literal essencial).
- §3 Decisões (apenas as substantivas).
- §4 Métricas (tabela compacta).
- §5 Divergências (se `P207B.div-N` registadas).
- §6 Próximo sub-passo.

Eliminar:
- Confronto de hipóteses extensa (apenas registar
  surpresas, se houver).
- Cross-references extensas.
- Tempo de execução granular.
- Decisões durante a leitura se forem mecânicas.

---

## §4 Não-objectivos

- Multi-label semântica (item 7 P207A; é P207C).
- Refactor `LabelRegistry` → `MultiMap` (P207C).
- Page-aware trait methods (P207D-E).

---

## §5 Risco a evitar

`HashMap::iter` ordem não-determinística pode tornar
tests não-determinísticos. Decisão dentro de P207B:
- Ordenar por `Label` antes de retornar (estável,
  custo trivial).
- Ou aceitar não-determinismo e tests adaptam.

Decisão preferida: ordenar. Cristalino não tem
convenção contra; estabilidade é benefício para
consumers.
