# Relatório do passo P204E

**Data de execução**: 2026-05-06.
**Executor**: Claude Code (Opus 4.7).
**Spec**: `00_nucleo/materialization/typst-passo-204E.md`.
**Natureza**: implementação trivial — wrapper L4 sobre
`comemo::evict`.
**Sub-passo `E` da série M8** — quarto de 7 (B-H) per
ADR-0073.
**Magnitude planeada**: S.
**Magnitude real**: **S** (~30 min; 1 ficheiro novo +
1 main.rs mod + 1 Cargo.toml dep + 1 L0 prompt + 1 ADR
anotação).

---

## §1 O que foi feito

P204E expôs `comemo::evict` via wrapper cristalino em L4
wiring per ADR-0073 (política de invalidação
tracking-based intra-compilation + `evict()` exposed para
callers).

### Output 1 — Inventário interno

Localização:
`00_nucleo/diagnosticos/typst-passo-204E-inventario.md`.

Conteúdo:
- §1 C1 inventário (5 sub-secções).
- §2 C2 forma fixada (passthrough).
- §3 alterações literais aplicadas.
- §4-§5 verificações + decisões.
- §6 métricas.
- §7 critério de fecho.
- §8 referências.

Tamanho: ~10 KB.

### Output 2 — Alterações em código

5 ficheiros modificados/criados:

#### Production (04_wiring)

1. **NOVO** `04_wiring/src/eviction.rs` — Wrapper
   passthrough `pub fn crystalline_evict(max_age: usize)`
   + 2 sentinel tests.
2. `04_wiring/src/main.rs` — `mod eviction;` declaração.
3. `04_wiring/Cargo.toml` — `comemo = { workspace = true }`
   dependency.

#### L0 prompt

4. **NOVO** `00_nucleo/prompts/wiring/eviction.md` — Prompt
   L0 formal per CLAUDE.md Protocolo de Nucleação. Hash
   `7ac7b48b` (auto-syncronizado via `--fix-hashes`).

#### ADR

5. `00_nucleo/adr/typst-adr-0073-comemo-introspector.md`
   — Anotação cirúrgica em "P204E — `crystalline_evict()`
   wrapper" com `✅ MATERIALIZADO 2026-05-06`.

### Output 3 — este relatório

---

## §2 Tempo de execução

Sessão única; ordem grosseira:

- Leitura do spec P204E: ~2 min.
- C1 inventário (`04_wiring/` estrutura, comemo API,
  vanilla pattern): ~5 min.
- C2 decisão passthrough: ~1 min.
- C3 implementação (eviction.rs + main.rs mod +
  Cargo.toml + L0 prompt): ~10 min.
- C4 doc-comment com ADR-0073 + P204E references: ~3 min.
- C5 sentinels (2): ~3 min.
- C6 compilação + resolução `dead_code` warning: ~3 min.
- `--fix-hashes` para V5 PromptDrift inicial: ~1 min.
- C9 ADR-0073 anotação cirúrgica: ~2 min.
- Inventário interno (Output 1): ~10 min.
- Este relatório: ~3 min.

**Total**: ~45 min.

---

## §3 Métricas

| Métrica | Pré-P204E | Pós-P204E | Δ |
|---------|-----------|-----------|---|
| Tests workspace | 1836 | **1838** | +2 |
| Crystalline-lint violations | 0 | 0 | = |
| LOC produção | baseline | +~20 | +20 |
| LOC tests | baseline | +~15 | +15 |
| L0 prompts novos | — | 1 | +1 |
| Cargo.toml deps novas | — | 1 (`comemo` em 04_wiring) | +1 |
| Ficheiros modificados/criados | — | 5 | — |

---

## §4 Decisões tomadas durante a leitura

### 4.1 04_wiring é binary-only — sem `lib.rs`

C1 detectou que `04_wiring` tem apenas `main.rs`, sem
`lib.rs`. Spec mencionou `lib.rs` como opção. Decisão:
criar `eviction.rs` como módulo no binary crate (não
converter para lib + bin).

### 4.2 `pub fn` + `#[allow(dead_code)]`

Em binary crate, `pub fn` não é consumido external; cargo
emite `dead_code` warning. Decisão: aplicar
`#[allow(dead_code)]` com comentário explicativo
("P204E expõe API para integração CLI / watch mode
futura").

### 4.3 Wrapper passthrough (não policy)

C2 fixou passthrough. Justificação: vanilla expõe
`comemo::evict` directamente sem wrapper (per A9).
Cristalino aplica wrapper para **simetria nominal**
(`crystalline_evict` vs `comemo::evict`). Sem policy
adicional.

### 4.4 2 sentinels (não 1)

Spec recomendou 1; implementado 2:
- `p204e_crystalline_evict_existe` — chama com `0`.
- `p204e_crystalline_evict_aceita_max_age_parametro` —
  chama com `10` e `usize::MAX`.

Cobre signature + runtime smoke separadamente.

### 4.5 L0 prompt em `wiring/` subdirectory

Spec não pré-fixou caminho do L0. Decisão: seguir padrão
de `entities/` (subdirectory para sub-módulos). Criado
`00_nucleo/prompts/wiring/eviction.md`. Próximos módulos
L4 podem usar mesma convenção.

---

## §5 Sugestão para próximo sub-passo (não-vinculativa)

**P204F — Corpus paridade reduzido** (per ADR-0073 plano
de materialização).

Trabalho concreto P204F:
1. 5-7 ficheiros .typ novos em `lab/parity/corpus/` cobrindo
   features de introspection:
   - `outline-toc.typ` (TOC).
   - `counter-heading.typ` (counter heading).
   - `figure-ref.typ` (figure ref).
   - `equation-ref.typ` (equation ref).
   - `cite-bibliography.typ` (bibliography + cite).
   - (Opcional) `here-locate.typ`.
   - (Opcional) `query-metadata.typ`.
2. Para cada ficheiro `.toml` de expectativa.
3. Validação cristalino == vanilla via parity tests.

Magnitude esperada: **M**.

Pré-condição cumprida por P204E:
- Wrapper `crystalline_evict` exposto ✅.
- ADR-0073 plano §P204E marcado ✅.

---

## §6 Critério de progressão respeitado

Per spec §3 C10, P204E está concluído quando:

- [x] C1 inventário completo (5 sub-secções).
- [x] C2 forma fixada (passthrough).
- [x] C3 edição aplicada (4 ficheiros).
- [x] C4 documentação inline (doc-comment com
  cross-references).
- [x] C5 sentinela adicionada (2).
- [x] C6 compilação verde.
- [x] C7 tests workspace verdes (1838).
- [x] C8 linter 0 violations (após `--fix-hashes`).
- [x] C9 ADR-0073 anotada (P204E ✅ materializado).
- [x] Inventário registado (Output 1).
- [x] Relatório escrito (este).

**Sem `P204E.div-N`** registadas.

---

## §7 Não-objectivos respeitados

Per spec §7, P204E não:

- [x] Não adicionou ficheiros ao corpus de paridade
  (P204F).
- [x] Não adicionou benchmarks (P204G).
- [x] Não transitou ADR-0073 para ACEITE (P204H).
- [x] Não transitou ADR-0066 para superseded (P204H).
- [x] Não criou ADR nova.
- [x] Não integrou `crystalline_evict` em CLI ou em
  fluxo automatizado de compilação.
- [x] Não tocou em consumers de `Introspector`,
  Layouter, ou qualquer trait L1.
- [x] Não modificou trait `Introspector` ou impl
  `TagIntrospector`.

---

## §8 Achados resumo

| Achado | Implicação |
|--------|-----------|
| 04_wiring é binary-only sem lib.rs | Wrapper como módulo dentro do binary; `#[allow(dead_code)]` |
| comemo::evict signature: `(max_age: usize)` simples | Passthrough trivial 1-linha |
| Vanilla usa `comemo::evict(10)` directamente em CLI watch | Convenção vanilla replicada nominalmente |
| `--fix-hashes` syncroniza placeholder hash automaticamente | Workflow eficiente para novos L0 prompts |

---

## §9 Notas operacionais

### 9.1 Magnitude S confirmada

Spec planeou S; real foi S (~45 min). Sub-passo mais
simples de M8 série como antecipado em P204A C12.

### 9.2 Diagnóstico-primeiro funcionou

C1 detectou:
- 04_wiring binary-only (não lib + bin).
- comemo não em deps actuais.
- Vanilla pattern (sem wrapper).

Resoluções aplicadas in-flight sem `P204E.div-N`.

### 9.3 Trabalho útil cumulativo

P204A + P204B + P204C + P204D + P204E juntos:
- Auditoria empírica (16 cláusulas).
- Diagnóstico (14 cláusulas).
- ADR-0073 PROPOSTO + 4 sub-passos materializados.
- Trait `Introspector` trackable + Send + Sync.
- 3 Hash impls (Value, BibEntry, Content).
- Layouter migração para Tracked (com `'a`).
- Position concrete materializada.
- `crystalline_evict` wrapper exposto.
- 9 sentinel tests activos (3 P204B + 2 P204C +
  2 P204D + 2 P204E).

P204F (corpus paridade) pode iniciar imediatamente.

### 9.4 Paridade vanilla nominal

`crystalline_evict` é wrapper nominal (1-linha) que
mantém naming consistency cristalino. Funcionalidade
idêntica a `comemo::evict`. ADR-0033 (paridade observable)
preservada.

---

**Fim do relatório P204E.**
