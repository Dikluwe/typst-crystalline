# Relatório do passo P206B

**Data de execução**: 2026-05-08.
**Executor**: Claude Code (Opus 4.7).
**Spec**: `00_nucleo/materialization/typst-passo-206B.md`.
**Natureza**: implementação focada (fix breaks + smoke
sentinel).
**Sub-passo `B` da série P206** — segundo de 5 (A–E).
**Magnitude planeada**: S.
**Magnitude real**: **S** (~25 min; 2 fixes triviais
+ 1 ficheiro novo de smoke + 1 ADR anotação; sem
refactor mid-execution; zero `P206B.div-N`).

---

## §1 O que foi feito

P206B reactivou o harness `lab/parity/` per Caminho A
fixado em P206A C1:

- **2 fixes triviais** confirmados em P206A A2 + P206B
  C1 aplicados literalmente.
- **Smoke sentinel** para detecção runtime de vanilla
  CLI adicionado.
- **`cargo check --all-targets`** confirmado verde.
- **`parity-runner` smoke** preservado funcional.
- **Tests workspace cristalino** invariantes (1860
  mantém-se exactamente).
- **ADR-0075 §P206B** anotada `✅ MATERIALIZADO
  2026-05-08`.

P206B **não invocou vanilla CLI em runtime para
comparação** — esse trabalho é P206C. P206B materializa
apenas a pré-condição (harness compila + vanilla CLI
detectável + subcomando `query` confirmado para
P206C).

### Output 1 — Inventário interno

Localização:
`00_nucleo/diagnosticos/typst-passo-206B-inventario.md`.

Conteúdo:
- §1 C1 inventário (5 sub-secções; 4 CONFIRMADO + 1
  AJUSTE NECESSÁRIO em C1.5).
- §2 C2 fix `layout_parity.rs` (3 mudanças
  coordenadas).
- §3 C3 fix `value_dto.rs` (2 mudanças).
- §4 C4 vanilla CLI smoke (estrutura + skip graceful
  + versão pinning).
- §5 7 decisões durante a leitura (D1–D7).
- §6 métricas.

Tamanho: ~12 KB.

### Output 2 — Relatório (este ficheiro)

### Output 3 — Alterações em código

#### Ficheiros modificados

- **`lab/parity/tests/layout_parity.rs`**:
  - Linha 29 (import): `use ...introspect::introspect`
    eliminado.
  - Linha 68: `let state = introspect(content);`
    removida.
  - Linha 69: `layout(content, state)` →
    `layout(content)`. +3L comentário inline a
    documentar migration P190I.
- **`lab/parity/src/value_dto.rs`**:
  - Linha 80-81: docstring "18 variants" → "19
    variants (P206B: +Location per P179)".
  - Linha 109+: arm `Value::Location(loc) =>
    ValueDTO::Other(format!("location:{loc:?}"))`
    adicionado. +4L comentário inline.

#### Ficheiros novos

- **`lab/parity/tests/vanilla_cli_smoke.rs`** (~75
  LOC):
  - 2 tests:
    - `p206b_vanilla_cli_disponivel_e_versao_compativel`
      — `typst --version` + match prefixo `0.14` +
      skip graceful via `eprintln!`.
    - `p206b_vanilla_cli_query_subcomando_existe` —
      `typst query --help` confirma subcomando para
      P206C.
  - `const VANILLA_EXPECTED_VERSION_PREFIX: &str =
    "0.14"` para tolerar micro-versões.

#### Ficheiros docs modificados

- **`00_nucleo/adr/typst-adr-0075-vanilla-integration.md`**:
  - §P206B (no Plano de materialização) anotado
    `✅ MATERIALIZADO 2026-05-08` + sumário literal.

---

## §2 Tempo de execução

~25 minutos efectivos:

- ~3 min: leitura da spec + setup TaskList + contexto.
- ~5 min: C1 inventário empírico (5 sub-secções; cargo
  check + smoke parity-runner + grep convenção).
- ~3 min: C2 fix `layout_parity.rs` (import + 2
  edições + comentário).
- ~3 min: C3 fix `value_dto.rs` (arm + docstring
  update).
- ~7 min: C4 escrita de `vanilla_cli_smoke.rs` (~75
  LOC + 2 tests + skip graceful).
- ~3 min: C5-C9 validação (cargo check; tests;
  parity-runner; tests workspace; lint).
- ~1 min: C10 anotação ADR-0075.
- ~5 min: outputs documentais (inventário + este
  relatório).

---

## §3 Métricas

| Métrica | Valor |
|---------|-------|
| Tests workspace cristalino antes | 1860 |
| Tests workspace cristalino depois | **1860** (∆ 0 — P206B é lab/parity quarentena) |
| Tests lab/parity antes | 52 |
| Tests lab/parity depois | **54** (+2 vanilla_cli_smoke novos) |
| Tests P206B novos | 2 (sentinelas runtime para vanilla CLI) |
| Linter violations | 0 (sem alteração) |
| Linter warnings | 0 (sem alteração; warnings pré-existentes em frame_dto.rs preservados) |
| Ficheiros código modificados | 2 (`tests/layout_parity.rs`; `src/value_dto.rs`) |
| Ficheiros código novos | 1 (`tests/vanilla_cli_smoke.rs`) |
| Ficheiros docs novos | 2 (inventário + este relatório) |
| Ficheiros docs modificados | 1 (ADR-0075 §P206B anotação) |
| LOC novas (código) | ~75 (smoke) + ~5 (comentários) = ~80 |
| LOC removidas | 3 (2 fix lines + 1 import) |
| Cargo deps adicionados | 0 |
| Refactor mid-execution | 0 |
| `P206B.div-N` registadas | 0 |

### Tests por crate (workspace cristalino)

- `typst_core` unit: 1584 (sem alteração).
- `typst_infra` unit: 24 (sem alteração).
- `typst_shell` unit: 21 (sem alteração).
- `typst_wiring` unit: 2 (sem alteração).
- Integration tests: 229 (sem alteração).
- **Total workspace**: 1860 (sem alteração).

### Tests lab/parity (quarentena, não contado em workspace cristalino)

- `parse_parity` (P1, corpus_completo_p1): 50 passed.
- `eval_parity` (P2, corpus_completo_p2): 1 passed.
- `layout_parity` (P3, corpus_completo_p3): 1 passed.
- `vanilla_cli_smoke` (P206B novo): **2 passed**.
- **Total lab/parity**: 54 passed; 0 failed.

---

## §4 Decisões

### D1 — `Value::Location` mapeado para `Other`, não variant próprio

Spec C3 fixa "1 line arm" mínimo. Adicionar variant
`ValueDTO::Location` exigiria editar enum + tests
PartialEq + serialização. `Other(format!("location:{loc:?}"))`
respeita convenção catch-all docstring (l.65-67) e
mantém o fix minimal. P206C/D pode estender se vanilla
integration produzir output Location-shaped que exija
diff fino — deferred.

### D2 — Smoke test em `tests/`, não helper em `src/`

Vanilla CLI smoke é **sentinela executável** —
`cargo test --test vanilla_cli_smoke` confirma estado
em runtime. Helper em `src/` seria invocado mas não
testado isoladamente. Spec C4 deu liberdade entre os
dois; `tests/` aligna com pattern de sentinelas
existentes.

### D3 — 2 tests no smoke (não 1)

Decidi 2 tests por separação de concerns:
- Disponibilidade + versão check.
- Subcomando query (pré-condição P206C).

Custo marginal: ~30 LOC duplicados em pattern
`match...output()`. Benefício: falha granular — sabe-se
se regressão é em disponibilidade ou em subcomando.

### D4 — Comentário inline em `layout_parity.rs:67-71`

Migration P190I de signature outdated é WHY não-óbvio
sem comment. Per CLAUDE.md "comentários só quando WHY
é não-óbvio": migration deliberada vs delete por engano
exige documentação inline para auditor futuro.

### D5 — Docstring de 18 → 19 variants é continuação, não inflação

Spec C3 fixa "1 line arm". Mas docstring linhas 80-81
afirma "Cobre os 18 variants" — desfazendo verdade
empírica pós-fix. Ajuste docstring é parte do mesmo
fix; não é inflação independente.

### D6 — Sem `P206B.div-N` registado

P206A A2 reportou 2 breaks; P206B C1.3 auditou
exhaustivamente via `cargo check --all-targets` e
confirmou apenas E0061 + E0004. Sem divergência. Spec
§8 risco "assumir 2 breaks únicos sem verificar":
evitado por verificação empírica.

### D7 — Skip graceful via `eprintln!` (não panic; não
ignored)

Vanilla CLI smoke usa `eprintln!` + `return` quando
binário ausente em PATH. Test **completa com sucesso**.
Alternativas rejeitadas:
- `panic!`: viola convenção lab/parity ("paridade é
  medição, não verificação"); falharia CI sem vanilla.
- `#[ignore]` attribute: requer manual flag para
  executar; não é skip dinâmico.

`eprintln!` + `return` é pattern coerente com
`tests/layout_parity.rs:138` (que reporta falhas mas
não panics).

---

## §5 Confronto de hipóteses do passo

A spec listou hipóteses específicas em §8 e §9:

| Hipótese | Resultado |
|----------|-----------|
| §8: "lab/parity/tests/ pode ter mais de 2 breaks" | **REJEITADA empíricamente** — apenas E0061 + E0004 confirmados via cargo check |
| §8: "vanilla CLI smoke fragile (hardcoded path/versão)" | **EVITADO** — versão match prefixo `0.14`; path via `Command::new("typst")` resolvido pelo PATH; skip graceful |
| §8: "assumir 2 breaks únicos sem verificar" | **EVITADO** — C1.3 verificação exhaustiva |
| §9: "fixes são triviais (1-line)" | **CONFIRMADA** — 2 fixes de 1-line cada (com remoção de import + docstring update como continuação coerente) |

4 de 4 hipóteses resolvidas pela auditoria empírica.

---

## §6 Sugestão para próximo sub-passo

P206B fechado per C12 com todos os critérios cumpridos:

- ✓ C1 inventário completo (5 sub-secções; 4 CONFIRMADO
  + 1 AJUSTE em C1.5).
- ✓ C2 `layout_parity.rs:69` fixado (3 mudanças).
- ✓ C3 `value_dto.rs:83` fixado (2 mudanças).
- ✓ C4 vanilla CLI smoke sentinel (2 tests; ~75 LOC).
- ✓ C5 `cargo check lab/parity --all-targets` verde.
- ✓ C6 `parity-runner` smoke preservado (`✓ Paridade
  confirmada (13 bytes)`).
- ✓ C7 vanilla CLI smoke verde (2/2 passed).
- ✓ C8 tests workspace cristalino 1860 mantém-se.
- ✓ C9 linter 0 violations preservadas.
- ✓ C10 ADR-0075 §P206B anotada.
- ✓ Inventário registado.
- ✓ Relatório escrito (este ficheiro).

**Próximo sub-passo**: **P206C — Comparação estrutural
via typst query** (per ADR-0075 plano de materialização).

P206C é magnitude M (~1-1.5h):

- Helper `lab/parity/src/vanilla_invoke.rs` —
  invocação vanilla CLI via `std::process::Command`.
- Helper `lab/parity/src/structural_compare.rs` —
  comparação JSON via `serde_json`.
- Cristalino test helper para serializar
  `Introspector::query_*` em JSON análogo.
- Tests: ≥5 unit tests para comparação estrutural
  (introspection corpus P204F).
- Outputs: 3 ficheiros (inventário + relatório +
  alterações de código).

Pré-condição confirmada por P206B:
- Vanilla CLI 0.14.x detectável (smoke test passa).
- Subcomando `query` disponível (smoke test
  específico passa).
- Harness lab/parity compila com `cargo check
  --all-targets`.

---

## §7 Cross-references

- **Spec**: `00_nucleo/materialization/typst-passo-206B.md`.
- **Outputs P206B**:
  - `00_nucleo/diagnosticos/typst-passo-206B-inventario.md`.
- **ADR**:
  `00_nucleo/adr/typst-adr-0075-vanilla-integration.md`
  (§P206B ✅ MATERIALIZADO 2026-05-08).
- **Predecessores**:
  - P206A (diagnóstico-primeiro de vanilla integration).
  - P205E (F3 ACEITE final).
  - P204H (M8 estruturalmente fechado; cond 9 PARCIAL
    será fechada em P206E).
- **Sucessor planeado**: P206C (vanilla CLI invocation
  + comparação estrutural via typst query JSON).
- **Pendências endereçadas**:
  - P204F.div-1: 2 breaks fixados; harness reactivado.
  - DEBT-53: progresso material (vanilla CLI
    detectável; pré-condição P206C cumprida).
  - DEBT-54: progresso documental (smoke confirma
    pre-built CLI funciona; workspace setup
    desnecessário).
- **Vanilla typst v0.14.2**:
  - Path dep: `lab/typst-original/crates/typst-syntax`.
  - Binary: `/usr/local/bin/typst v0.14.2 (b33de9de)`
    (smoke test confirma).
- **Pattern referência**: P204B — diagnóstico→
  implementação directa em sub-passo `*B` após `*A`
  diagnóstico-primeiro.
