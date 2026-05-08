# Passo 206D — Cobertura corpus 36 + matriz consolidada

**Série**: 206 (sub-passo `D` = consolidação após P206C
helpers materializados).
**Tipo**: implementação de relatório (matriz +
cobertura).
**Magnitude planeada**: S–M.
**Pré-condição**: P206C concluído; helper L3
`query_helpers.rs` em `03_infra/`; helpers
`vanilla_invoke.rs` + `structural_compare.rs` em
`lab/parity/src/`; tests parameterizados em
`tests/structural_parity.rs`; tests workspace cristalino
1873 verdes; tests `lab/parity` 64 verdes; 0 violations;
ADR-0075 PROPOSTO em vigor com §P206C anotado ✅
MATERIALIZADO; **`P206C.div-1`** registada (CLI
subcomando deferred).
**Output**: 3 ficheiros (inventário + relatório +
alterações de código).

---

## §1 Propósito

Materializar matriz consolidada de paridade que integra
os dados produzidos por P206C nos relatórios existentes
de `lab/parity/`:

- **Estender `corpus_completo_p3`** ou test análogo para
  incluir vanilla side via helpers P206C.
- **Actualizar `ParityMatrix`** (em `lab/parity/src/`)
  para colunas `text_content` e `structural` populadas
  com dados empíricos.
- **Render matriz consolidada** para 36 entradas com
  match/diff/skip por categoria.
- **Sentinelas dedicadas** (2–3) para regressão
  detection.

P206D **não estende helpers P206C** — usa-os. Não
materializa CLI subcomando (deferred per
`P206C.div-1`). Não transita ADRs (P206E).

P206D respeita o padrão: começa com inventário empírico
antes de qualquer alteração.

---

## §2 Material de partida verificado em P206C

Antes de qualquer alteração, confirmar empíricamente:

- `03_infra/src/query_helpers.rs` exporta
  `query_to_summary`, `parse_selector`, `QuerySummary`,
  `ParsedSelector`, `SelectorKind`, `QueryError`.
- `lab/parity/src/vanilla_invoke.rs` exporta
  `run_typst_query`, `vanilla_cli_available`,
  `VanillaInvokeError`.
- `lab/parity/src/structural_compare.rs` exporta
  `compare_query_outputs`, `CompareResult`.
- `lab/parity/tests/structural_parity.rs` corpus test
  produz matriz markdown via `eprintln!` (per P206C
  D7).
- `lab/parity/src/report.rs` (ou similar) existe com
  `ParityMatrix` ou estrutura análoga — confirmar
  caminho exacto em C1.
- Cobertura empírica P206C: 23 INCLUDE + 13 SKIP (3
  pre-existing + 10 feature) sobre 36 ficheiros.

Sem isto, recuar para P206C.

---

## §3 Cláusulas de execução (sem condicionais)

### C1 — Inventário empírico inicial

Antes de tocar em código, listar literalmente:

1. **`ParityMatrix` actual** — confirmar:
   - Caminho real (`lab/parity/src/report.rs` ou
     similar).
   - Estrutura (struct fields, enums).
   - Colunas existentes (`parse_p1`, `eval_p2`,
     `layout_p3`, ...).
   - Como é renderizada (Display impl? markdown? JSON?).
2. **`corpus_completo_p3`** — confirmar:
   - Localização exacta (`tests/layout_parity.rs` ou
     `tests/p3_layout_parity.rs`).
   - Estrutura actual (loop sobre corpus +
     produce-side cristalino-only baseline per P204F P3
     mode).
   - Como é integrada com matriz.
3. **Estrutura `structural_parity.rs` (P206C)** —
   confirmar:
   - Como produz matriz markdown via `eprintln!`.
   - Que dados produz (cristalino summary + vanilla
     JSON + compare result).
   - Onde os dados ficam (apenas em test output ou
     também em ficheiro/struct?).
4. **Patterns de relatório** — confirmar:
   - Reports em `lab/parity/reports/` (per P206A A1)?
   - Que formatos existem?
   - Convenção de nomenclatura.
5. **Sentinelas existentes** — confirmar:
   - `lab/parity/` tem testes que falham se cobertura
     diminuir?
   - Threshold convention (ex: "≥X ficheiros INCLUDE"
     ou "≤Y ficheiros REGRESSÃO")?
6. **Custos estimados das alternativas C2** — análise:
   - Caminho A (estender `corpus_completo_p3`):
     magnitude.
   - Caminho B (matriz dedicada como struct +
     renderer): magnitude.
   - Caminho C (consolidação via reports/ ficheiro):
     magnitude.

Output: 6 sub-secções com etiqueta CONFIRMADO ou
**AJUSTE NECESSÁRIO**.

### C2 — Forma da matriz consolidada

Decisão fixada com base em C1:

- **Caminho A — estender `corpus_completo_p3`** — test
  existente ganha vanilla side via helpers P206C;
  matriz expandida com colunas `text_content` /
  `structural`.
- **Caminho B — matriz dedicada** — struct
  `ParityMatrix` (ou similar) ganha colunas; renderer
  produz markdown consolidado.
- **Caminho C — reports/ ficheiro** — output em
  `lab/parity/reports/` com matriz markdown versionada.

Critério: simplicidade + reuso de patterns existentes.
Caminho A é favorito se `corpus_completo_p3` já tem
estrutura adequada. Caminho B é favorito se há
`ParityMatrix` separado renderizável.

C2 fixa **uma** alternativa.

### C3 — Estrutura concreta da matriz

Com base em C2 + C1.1, fixar:

- Colunas: `parse_p1`, `eval_p2`, `layout_p3`,
  `text_content`, `structural` (5 dimensões).
- Linhas: 36 ficheiros do corpus.
- Cells: enum `MatchStatus` (Match / Diff / Skip /
  NotApplicable / Error).
- Render formato (markdown table preferido por
  legibilidade humana).

C3 fixa estrutura literal.

### C4 — Categorias de Skip explicitamente documentadas

Per P206C C6 + `P206C.div-1`:

- 3 SKIP-pre-existing (`code/let.typ`, `code/set.typ`,
  `markup/error.typ`).
- 10 SKIP-feature (corpus `semantic/*` fora-de-escopo
  introspection).
- 13 INCLUDE com diffs documentados (3 divergências
  arquitectónicas conhecidas: `equation`,
  `cite-bibliography`, `outline-toc`).

C4 produz manifest literal (em `lab/parity/SKIPS.md` ou
similar; decide em C1).

### C5 — Sentinelas dedicadas

Adicionar 2–3 sentinelas:

- **`p206d_corpus_cobertura_minima`** — falha se
  N_INCLUDE < threshold (ex: 20).
- **`p206d_matriz_renderizavel`** — produz output
  esperado sem panic.
- **`p206d_skips_documentados`** (opcional) — confirma
  que cada SKIP tem entrada em manifest.

Localização: `lab/parity/tests/` (quarentena).

### C6 — Render para auditor humano

Decisão sobre formato de output:

- **Markdown table** — legível humano + diff-friendly.
- **JSON** — maquinalmente diff-friendly mas menos
  legível.
- **Ambos** — markdown primário + JSON secundário.

C6 fixa **uma**.

### C7 — Compilação

```
cargo check --manifest-path lab/parity --all-targets
cargo build --workspace
```

Critério: ambos verdes.

### C8 — Tests workspace cristalino

```
cargo test --workspace 2>&1 | tail -10
```

Critério: 1873 mantém-se. P206D é puramente lab/parity
quarentena.

### C9 — Tests `lab/parity`

```
cargo test --manifest-path lab/parity --all-targets
```

Critério: 64 + N tests verdes (N = sentinelas P206D
adicionadas).

### C10 — Linter

```
crystalline-lint .
```

Critério: 0 violations.

### C11 — Documentação ADR-0075

ADR-0075 mantém PROPOSTO. Anotação cirúrgica em §P206D
com `✅ MATERIALIZADO` + sumário (1–2 linhas).

### C12 — Critério de fecho de P206D

P206D concluído quando:

- C1 inventário completo (6 sub-secções).
- C2 caminho fixado com justificação.
- C3 estrutura concreta da matriz.
- C4 categorias de Skip documentadas.
- C5 sentinelas dedicadas (mínimo 2).
- C6 formato de output fixado.
- C7 compilação verde.
- C8 tests workspace mantidos.
- C9 tests lab/parity verdes.
- C10 linter 0 violations.
- C11 ADR-0075 anotada.
- Inventário registado.
- Relatório escrito.

### C13 — Sem cláusulas condicionais

C1 produz dados. C2 fixa **uma** alternativa. C3–C11
executam decisões fixas.

---

## §4 Outputs concretos

### Ficheiro 1 — Inventário interno

Localização:
`00_nucleo/diagnosticos/typst-passo-206D-inventario.md`.

Conteúdo:
- §1 C1 — inventário (6 sub-secções).
- §2 C2 — caminho fixado.
- §3 C3 — estrutura matriz.
- §4 C4 — categorias Skip.
- §5 Decisões durante a leitura.

### Ficheiro 2 — Relatório

Localização:
`00_nucleo/materialization/typst-passo-206D-relatorio.md`.

Conteúdo:
- O que foi feito.
- Caminho escolhido.
- Tempo de execução.
- Métricas (tests pre/post; LOC delta).
- Decisões.
- Sugestão para próximo sub-passo (P206E).

### Ficheiro 3 — Alterações em código

Não é ficheiro discreto. Conjunto de:

- (Caminho A) `corpus_completo_p3` test estendido +
  matriz inline.
- (Caminho B) `lab/parity/src/report.rs` actualizado +
  `corpus_completo_p3` invoca renderer.
- (Caminho C) `lab/parity/reports/parity-matrix.md`
  (novo) + test que regenera + diff-check.
- Manifest `SKIPS.md` (per C4).
- Sentinelas (per C5).
- Anotação cirúrgica em ADR-0075.

---

## §5 Critério de progressão para P206E

P206D fechado quando C12 cumprido.

Em caso de divergência empírica relevante, registar em
`P206D.div-N` e:

- Resolver dentro de P206D (preferido).
- Recuar para P206A re-fixar C2 (improvável).

P206E só começa quando P206D fechado.

---

## §6 Convenções mantidas

- Sem condicionais estruturais.
- 3 outputs.
- Inventário empírico antes de implementação.
- Localização canónica.
- Sem inflação retórica.
- `lab/parity/` é quarentena — não invade workspace
  cristalino.

---

## §7 Não-objectivos

P206D não:

- Estende helpers P206C (já fixados).
- Materializa CLI subcomando (deferred per
  `P206C.div-1`; pós-P206 dedicado).
- Estende `Selector` enum em L1.
- Cria ficheiros corpus novos.
- Fixa divergências arquitectónicas documentadas em
  P206C (`equation` namespace, `cite-bibliography`,
  `outline-toc`).
- Transita ADR-0075 para ACEITE (P206E).
- Transita ADR-0073 para "completo final" (P206E).
- Adiciona expectations vanilla nas companions
  `.typ.toml` (P206E condicional).
- Toca em código produção workspace cristalino.

---

## §8 Erro a não repetir

Da série P204+P205+P206A/B/C — pattern empírico:
inventário antes de decisão; honestidade sobre
divergências.

Risco específico de P206D: **inflar matriz para
demonstrar "cobertura completa" quando dados empíricos
de P206C mostram limitações claras** (3 divergências
arquitectónicas + 13 SKIPs justificados). Matriz deve
**reflectir realidade empírica**, não inflar para
parecer completa.

Outro risco: **duplicar lógica de renderização**. P206C
produziu matriz markdown via `eprintln!` em
`structural_parity.rs`. P206D pode reutilizar pattern
ou refactorizar para renderer dedicado. Decidir em C2
empíricamente, não por preferência estética.

Hipótese mais provável: C2 = Caminho B (matriz dedicada
como struct + renderer). `ParityMatrix` provavelmente
já existe em `lab/parity/src/report.rs` (per P206A A1)
e estendê-lo com colunas novas é trabalho cirúrgico.
Caminho A inflaciona `corpus_completo_p3` que tem foco
P3 layout. Caminho C cria ficheiro que precisa
diff-check em CI — overhead.

Mas é hipótese, não decisão. C2 fixa-se com base em
C1.

---

## §9 Particularidade — execução

P206D é trabalho de relatório:

- Modificação de `ParityMatrix` ou test análogo (~50–
  100 LOC).
- Manifest `SKIPS.md` (~30–50 linhas markdown).
- 2–3 sentinelas (~30–60 LOC).
- Anotação ADR.

Volume baixo a médio. Magnitude S–M.

Recomendado pela sessão actual (Opus, com bash_tool)
se houver disponibilidade — P206D é trabalho mais
documental que de código. Caso contrário, Claude Code
segue padrão habitual.
