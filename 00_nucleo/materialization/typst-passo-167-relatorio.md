# Relatório P167 — Inventário de consumers `CounterStateLegacy` (M5 sub-passo 1)

Executado em 2026-04-30. Passo documental — sem código tocado.

## Resumo

- Inventário completo dos consumers de `CounterStateLegacy` em `01_core/src/rules/` e `03_infra/src/`. Identificados **6 consumers de production** + 1 test.
- Mapeamento campo-a-campo legacy → `TagIntrospector`. Resultado: **0 mapeamentos Total**, vários Parcial, vários Lacuna.
- 4 lacunas novas detectadas e adicionadas a `m1-lacunas-captura.md` (numbering_active, format_hierarchical hierárquico, bib_entries/bib_numbers, has_outline).
- Decisão para P168: migrar **subset apenas do figure-ref em `references.rs::layout_ref`** — único caso Parcial viável com caveats geríveis.
- Diagnóstico produzido em `00_nucleo/diagnosticos/inventario-consumers-counter-state-legacy.md`.

## Verificações .E

| # | Critério | Estado |
|---|----------|--------|
| 1 | `cargo check --workspace` passa (sem código tocado) | ✅ |
| 2 | `cargo test --workspace` passa sem mudança de contagem | ✅ **1 593** (igual a P166 = 1 593, sem Δ) |
| 3 | `crystalline-lint`: zero violations (sem L0/L1 modificados) | ✅ |
| 4 | Diagnóstico `inventario-consumers-counter-state-legacy.md` existe com 4 secções | ✅ Leitores + Mapeamento + Migrabilidade + Escolha |
| 5 | `m1-lacunas-captura.md` actualizado com lacunas novas | ✅ +4 entradas (#4–#7) |
| 6 | Nenhum L0 modificado | ✅ |

## Resumo numérico do inventário

### Consumers identificados

| # | Consumer | Categoria | Migrabilidade |
|---|----------|-----------|---------------|
| 1 | `Layouter::layout` (`layout/mod.rs`) | production, central | Bloqueada |
| 2 | `layout_ref` + `layout_labelled` (`references.rs`) | production | **Parcial** (figure-ref viável) |
| 3 | `layout_outline` (`outline.rs`) | production | Bloqueada (lacuna #3) |
| 4 | `counter_helpers` (`counters.rs`) | production | Bloqueada (mutação) |
| 5 | `layout_equation` (`equation.rs`) | production | Bloqueada (Equation não é payload kind) |
| 6 | `integration_tests` (`03_infra/.../integration_tests.rs:1116`) | test | Parcial |

**Distribuição**: 0 Total / 2 Parcial / 4 Bloqueada / 1 N/A (test).

### Lacunas

3 originais (P163 em `m1-lacunas-captura.md`) + 4 novas (P167):

| # | Lacuna | Origem |
|---|--------|--------|
| 1 | `figure.kind` None vs "image" default | P163 |
| 2 | Auto-labels só em state | P163 |
| 3 | Body frozen em state vs hash em tags | P163 |
| 4 | `is_numbering_active` / `numbering_active` | **P167** |
| 5 | `format_hierarchical` hierárquico vs `CounterRegistry` flat | **P167** |
| 6 | `bib_entries` / `bib_numbers` | **P167** |
| 7 | `has_outline` | **P167** |

## Consumer escolhido para P168

**`layout_ref` em `01_core/src/rules/layout/references.rs`, subset apenas para o caso de figure-ref** (`references.rs:35`).

**Justificação** (per regras `.D`):
1. Total não disponível → cláusula gate trivial; Parcial admitido.
2. Entre Parciais: production beats test → `layout_ref` > `integration_tests`.
3. Menor número de call-sites: 1 (apenas a arm figura-ref).
4. Menor superfície downstream: função folha; só consumida pelo walk arm `Content::Ref`.

**Riscos identificados** (passados a P168):

1. **Filtro figura-numerada-captioned**: `figure_label_numbers` legacy só inclui figuras com `numbering+caption`; `kind_index[Figure]` inclui todas. P168 decide entre 3 caminhos (A: from_tags filtra; B: query filtra — não viável sem payload mais rico; C: campo `is_counted` em `ElementPayload::Figure`).
2. **Adicionar field `introspector: TagIntrospector` ao `Layouter`**: requer toque em `Layouter::new` (mod.rs:144) e em `pub fn layout()` (mod.rs:1325). Custo médio.
3. **Lacuna #1 aplica-se directamente**: o filtro figura-numerada-captioned é o instrumento que resolve a divergência kind=None vs "image" para este caso.

**Tamanho estimado P168**: M (médio).

## Estado pós-passo

- **P167 concluído**.
- **P168 desbloqueado** — primeira migração real do figure-ref em `layout_ref`. Decisão entre caminhos A/B/C para o filtro figura-numerada-captioned fica para P168 .A.
- **Layouter como consumer central permanece bloqueado** — exigirá expansão substancial do Introspector antes de migração total. Provavelmente M9 (`MetadataStore`, `numbering_active`, hierarquia em CounterRegistry, bib state) é pré-requisito.

## Pendências cumulativas (M1+M2+M3+M4+M5)

- **7 lacunas** em `m1-lacunas-captura.md` (3 P163 + 4 P167).
- Reshape opcional `CounterUpdate::Step` → `Step(usize)`.
- Refino opcional de `hash_content`.
- `Position` ainda não materializado (M5/M9).
- `comemo::track` deferido (M7+).
- `CounterKey` enum vanilla deferido (M9).
- `MetadataStore` adiado (M9).

P167 não introduziu novas pendências de código — apenas documentou as existentes.
