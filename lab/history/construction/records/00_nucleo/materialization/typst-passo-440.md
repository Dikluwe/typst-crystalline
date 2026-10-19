# P440 — Fecho de débito: DEBT-43 (linter whitelist type-level)

---

## Contexto

**DEBT-43** está aberto desde o Passo 89 (2026-04-26). O `crystalline.toml` usa whitelist crate-level para externos autorizados em L1: se uma crate tem ao menos um tipo autorizado por ADR, **qualquer** tipo dessa crate passa o linter — mesmo tipos cujo uso não foi autorizado.

Exemplo concreto (Passo 87):
- ADR-0024 autorizou `ecow::EcoString` para `Value::Str`.
- ADR-0035 autorizou `ecow::EcoVec`.
- `ecow::EcoMap` ou `ecow::EcoArc` passariam silenciosamente se usados.

A disciplina é humana (revisão de código), não automática. Este passo fecha a lacuna de enforcement.

> **Nota:** este passo toca **dois repositórios**: `crystalline-lint` (binário guardião) e `typst-cristalino` (configuração). Como `crystalline-lint` é projecto separado, o P440 foca na **configuração deste repositório**, assumindo que o binário já aceita o novo formato (ou que a alteração no binário é feita em passo dedicado nesse projecto).

---

## ADR-0108 — Medir antes de decidir

**FASE A.0 — Sonda:**

| Pergunta | Resultado | Status |
|----------|-----------|--------|
| `crystalline.toml` aceita whitelist type-level hoje? | Não — formato é array de crates (`["ecow", "indexmap", ...]`) | ❌ |
| `crystalline-lint` binário suporta type-level? | Não confirmado — requer alteração no projecto separado | ❌ |
| Existe ADR que proíba type-level? | Não — ADR-0032 proíbe `unsafe`, não fala de linter | ✅ |
| Quantas crates autorizadas? | ~8 (`ecow`, `indexmap`, `rustc-hash`, `hypher`, `comemo`, `ttf-parser`, `bib_csl`, `hayagriva`) | — |
| Bloqueadores? | `crystalline-lint` binário é pré-requisito técnico | ❌ |

**Reclassificação:** S-M (~1-2h; toca `crystalline.toml` + teste de violação negativa + documentação; **pré-requisito**: `crystalline-lint` binário atualizado).

---

## ADR-0107 — Paridade linguagem

O contrato é **arquitetural**: o `crystalline-lint` deve rejeitar automaticamente tipos não autorizados de crates autorizadas. Exemplo: `use ecow::EcoMap` → violation, porque apenas `EcoString` e `EcoVec` estão listados.

---

## ADR-0109 — Atomização forma B

**Opções analisadas:**

| Opção | Formato `crystalline.toml` | Toques | Risco |
|-------|---------------------------|--------|-------|
| α | `[l1_allowed_external.ecow] types = ["EcoString", "EcoVec"]` | ~15 LOC no parser do linter; migração TOML mecânica | Médio — requer binário atualizado |
| β | `[l1_allowed_external] ecow = { types = ["EcoString", "EcoVec"] }` | Similar a α; sintaxe TOML inline table | Médio — idem |
| γ | Manter crate-level + adicionar teste de violação negativa no CI | Zero alteração no binário; teste Rust que tenta compilar `ecow::EcoMap` e espera falha | Baixo — mas não é enforcement automático no linter |

**Decisão: Opção α** — formato TOML por seção. É a mais explícita e alinha com a proposta original do DEBT-43. Assume que `crystalline-lint` binário será atualizado (ou já foi) para interpretar o novo formato.

**Toques pontuais:**
1. `crystalline.toml` — migrar de array para formato por seção:
   ```toml
   [l1_allowed_external.ecow]
   types = ["EcoString", "EcoVec"]

   [l1_allowed_external.indexmap]
   types = ["IndexMap"]

   [l1_allowed_external.rustc-hash]
   types = ["FxHashMap"]

   [l1_allowed_external.hypher]
   types = ["hypher"]  # ou função específica

   [l1_allowed_external.comemo]
   types = ["Tracked", "TrackedMut", "Validate"]

   [l1_allowed_external.ttf-parser]
   types = ["Face", "TableDirectory"]

   [l1_allowed_external.hayagriva]
   types = ["Entry", "IndependentStyle"]  # ou equivalente

   [l1_allowed_external.bib_csl]
   types = ["..."]  # ajustar conforme uso real
   ```
2. `crystalline-lint` binário — garantir que parser aceita novo formato (passo no projecto separado, ou verificar se já aceita).
3. Teste de violação negativa — criar ficheiro Rust temporário em `tests/` que usa `ecow::EcoMap` e espera falha do linter (ou teste unitário no `crystalline-lint` se projecto separado).
4. `DEBT.md` — reclassificar DEBT-43 como FECHADO (P440).

---

## Decisões arquiteturais

| Decisão | Opção escolhida | Justificativa |
|---------|----------------|---------------|
| Formato TOML | Seção por crate (`[l1_allowed_external.ecow]`) | Paridade com proposta original DEBT-43; legível; extensível |
| Migração | Uma crate de cada vez, começando por `ecow` (maior risco) | Atomização; permite validar incrementalmente |
| Teste de violação | `tests/crystalline_lint_type_level.rs` (ou similar) | Garante que tipo não listado é rejeitado |
| `crystalline-lint` binário | Pré-requisito; assumir atualizado ou atualizar em passo dedicado | DEBT-43 menciona explicitamente que trabalho no binário é pré-requisito |

---

## Scope-out explícito

- Atualização do `crystalline-lint` binário — se ainda não suporta o formato, é trabalho de passo dedicado no projecto separado (mencionado em DEBT-43).
- Whitelist de funções/métodos individuais — fora do escopo; só tipos.
- Whitelist de macros — fora do escopo.

---

## Critério de fecho

- [ ] `crystalline.toml` migrado para formato type-level (pelo menos `ecow` como prova de conceito).
- [ ] Tipo não autorizado da mesma crate (`ecow::EcoMap`) reportado como violação em teste.
- [ ] Tipo autorizado (`ecow::EcoString`) continua a passar.
- [ ] `DEBT.md` reclassificado: DEBT-43 **FECHADO (P440)**.
- [ ] `cargo test --workspace` verde (zero código funcional de L1 modificado).
- [ ] `crystalline-lint` zero violations (ou teste de violação negativa passa).

---

**Próximo passo:** Com DEBT-43 fechado, resta apenas **DEBT-42** (`get_unchecked` bloqueado por benchmark — aguarda infra de benchmarking). O inventário de débitos estará **quase limpo**. Indique se quer ajustar o escopo do P440.
