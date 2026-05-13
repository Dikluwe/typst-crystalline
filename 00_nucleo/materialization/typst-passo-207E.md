# Passo 207E — Encerramento série P207

**Série**: 207 (sub-passo `E` final).
**Marco**: M9c (encerramento série P207).
**Tipo**: encerramento série + decisão de captura.
**Magnitude**: S (~30min-1h) documental puro **ou** M
(~2-3h) com captura — decidido em C1.
**Pré-condição**: P207D concluído; trait 26 métodos;
`PageStore` materializado; tests 1899 verdes; 0
violations; ADR-0076 PROPOSTO anotado §P207D ✅
MATERIALIZADO; surpresa `ecow` dev-dep → dep documentada.
**Output**: 1 ficheiro (relatório curto + transição
para P208).

---

## §1 Trabalho

Encerrar série P207 (4 sub-passos materializados: B + C
+ D + esta). Decisão de **captura no walk de layout**
para popular `PageStore::numberings`/`supplements`
fixada em C1 com base em evidência empírica.

Reuso de dados P207A + P207B + P207C + P207D:

- Trait com 26 métodos.
- `PageStore` em
  `01_core/src/entities/page_store.rs` com 6 métodos
  (per P207D §3).
- `PageStore::from_runtime` materializado mas Layouter
  não captura `numbering`/`supplement` (per P207D D5).
- Pre-injecção: 4 métodos page-aware retornam None.
- Pós-injecção sem captura: `pages` e `page` resolvem;
  `page_numbering`/`page_supplement` retornam None
  mesmo com `PageStore` injectado.

---

## §2 Cláusulas (4)

### C1 — Diagnóstico breve: decisão sobre captura

Antes de tocar código, inventário focado em **3
sub-secções**:

1. **Onde captura aconteceria**: identificar literalmente
   o ponto no walk do Layouter onde `numbering` e
   `supplement` ficariam disponíveis. Esperado:
   `Layouter::layout_page` ou método análogo. Verificar
   empíricamente que dados a função consome (set-rule
   `page` properties).
2. **Custo empírico**: estimar magnitude da captura. Se
   `numbering` e `supplement` vivem em set-rule context
   acessível, captura é trivial (~10-20 LOC); se
   exigem walk profundo ou novo state runtime, é M.
3. **Consumer real**: confirmar que zero consumers
   cristalinos invocam `page_numbering`/`page_supplement`
   (per P207A A11 + P207D C1.1). Se algum consumer
   emergiu, captura é justificada imediatamente.

Critério literal para C2:

- **Caminho 1 — encerramento documental puro** (~30
  min): se C1.3 confirmar zero consumers reais E
  C1.2 estimar captura ≥ M ou exigir refactor walk.
  Justificação: captura sem consumer é over-engineering;
  emerge naturalmente quando P208 `here()` desbloquear
  consumer.
- **Caminho 2 — encerramento + captura** (~2-3h): se
  C1.2 estimar captura trivial (~10-20 LOC) OU C1.3
  revelar consumer emergente.
- **Caminho 3 — captura parcial** (~1h): popular apenas
  `numberings` se trivial; deixar `supplements` deferred.

C2 fixa **uma** alternativa empírica.

### C2 — Materializar caminho fixado em C1

Se Caminho 1 (puro): saltar C3 implementação; ir a C4
encerramento ADR.

Se Caminho 2 (com captura): materializar:

- L0 update `00_nucleo/prompts/rules/layout/layouter.md`
  ou similar — documentar captura de page-meta no
  walk.
- L0 update `00_nucleo/prompts/entities/page_store.md`
  — actualizar §"Integração" para reflectir captura
  activa.
- L1 update `01_core/src/rules/layout/mod.rs` ou onde
  o walk acontecer — capturar `numbering` + `supplement`
  de set-rule context para `LayouterRuntimeState`.
- L1 update `LayouterRuntimeState` — adicionar fields
  `page_numberings: Vec<Option<EcoString>>`,
  `page_supplements: Vec<Option<Content>>` (ou similar
  baseado em C1.1).
- L1 update `Layouter::finish` — sealing pós-finish
  popula `PageStore::from_runtime` com Vecs reais (em
  vez de empty).

Se Caminho 3 (parcial): materializar apenas numberings.

### C3 — Tests (se C2 ≠ Caminho 1)

3-5 tests E2E:

- `p207e_page_numbering_real_pos_layout` — set-rule
  `page(numbering: "1")` propaga; `page_numbering(loc)`
  retorna `Some("1")`.
- `p207e_page_supplement_real_pos_layout` (se Caminho
  2).
- 1-2 tests de regressão (Caminho 1 None preservado em
  ausência de set-rule).

Se Caminho 1: zero tests novos.

### C4 — Encerramento série P207

Independente de C2:

**ADR-0076 §Plano de materialização**:
- §P207 transita "EM CURSO" → "✅ MATERIALIZADO
  ({data})".
- §P207E anotado com forma fixada em C2.
- Bloco "Série P207 — encerrada" adicionado com
  sumário literal dos 4 sub-passos (B + C + D + E) +
  métricas agregadas.

**ADR-0076 §Plano de materialização P208**:
- Confirmar P208 como próxima série (per
  `P207A.div-1`).

**Blueprint**:
- Marca `§3.0quater [P207E]` adicionada adjacente a
  `§3.0ter [P206E]` per pattern marca-por-fecho
  estabelecido em P204H/P205E/P206E.

```
cargo test --workspace 2>&1 | tail -10
crystalline-lint .
crystalline-lint --fix-hashes .
```

Critério: tests verdes (1899 + N onde N depende de
C2); 0 violations.

---

## §3 Output

1 ficheiro:
`00_nucleo/materialization/typst-passo-207E-relatorio.md`.

Estrutura conciso (~4-6 KB) com 7 §s:

- §1 O que foi feito (sumário 3-5 linhas).
- §2 Decisão de captura fixada (Caminho 1/2/3) com
  evidência empírica.
- §3 Alterações em código (se C2 ≠ Caminho 1).
- §4 Decisões substantivas.
- §5 Métricas (tabela compacta).
- §6 Encerramento série P207 (resumo 4 sub-passos +
  agregado).
- §7 Próximo sub-passo (P208).

---

## §4 Não-objectivos

- `here()` / `locate()` (P208).
- Selector enum extensions (P209).
- Outline configurável (P211).
- `query_count_before` (Q4 deferred).
- Encerramento M9c inteiro — apenas série P207 fecha;
  marco M9c continua com P208+.
- Transição ADR-0076 PROPOSTO → ACEITE (P212
  encerramento M9c).

---

## §5 Riscos a evitar

1. **Materializar captura sem consumer**: se C1.3
   mostrar zero consumers E C1.2 mostrar custo M+,
   Caminho 1 é honesto. Sem inflação.
2. **Inflar relatório consolidado**: P207E é
   encerramento de série, não consolidado de marco.
   Sem §s extensas; sumário literal 4 sub-passos.
3. **Confundir encerramento série vs marco**: P207E
   fecha série P207 (4 sub-passos); marco M9c
   continua. ADR-0076 mantém PROPOSTO até P212.
4. **Esquecer marca blueprint**: per pattern §3.0/
   §3.0bis/§3.0ter, P207E adiciona §3.0quater.
