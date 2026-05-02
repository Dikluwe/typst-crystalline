# Relatório — Passo 182A

**Data**: 2026-05-02
**Passo**: P182A — diagnóstico-primeiro / L0-puro
**Escopo**: lacuna #4 (`is_numbering_active` / `numbering_active`).
**Resultado**: 6 cláusulas fechadas; plano P182B–P182F sem condicionais; ADR não criada; DEBT não aberto.

---

## 1. Sumário

P182A executou o protocolo de diagnóstico antes da implementação da lacuna #4. As 6 cláusulas obrigatórias (mecanismo, default, consumers, localização, API, fecho) estão fixadas com inputs verificáveis. Plano P182B–P182F é S-M total, sem cláusulas condicionais.

Decisões consolidadas:
- **Mecanismo**: M1 (reusar `StateRegistry` P171 com chave `numbering_active:heading`).
- **Default**: OFF (paridade vanilla `Option<Numbering>` + cristalino actual `unwrap_or(false)`).
- **Consumers Layouter migráveis**: 2 (`mod.rs:301` heading, `equation.rs:24` equation).
- **API**: A2 — `Introspector::is_numbering_active(&self, key: &str) -> bool`.
- **Fecho**: Opção 3 (infra + consumer migrado; legacy permanece até M6).
- **ADR/DEBT**: nenhum.

Magnitude estimada P181J §5 (S-M) **valida-se**. Plano P182B–P182F replica padrão P181F + P181G + P181I.

Surpresa do inventário: vanilla **não tem** `numbering_active` em lado algum. Lacuna #4 é divergência arquitectural cristalino, não feature ausente. Vanilla resolve via `Option<Numbering>` + StyleChain location-aware; cristalino tem boolean global por chave por falta de StyleChain. P182 mantém o boolean cristalino e migra-o para Introspector — não materializa StyleChain.

Surpresa adicional: variant é `Content::SetHeadingNumbering { active: bool }` (apenas booleano, hardcoded para "heading"), não `{ key, value }` como o texto da lacuna sugeria. Equation usa a mesma chave `numbering_active["equation"]` mas sem variant set rule equivalente.

---

## 2. Validação estado actual (sub-passo 182A.A)

12 itens inspeccionados em 2026-05-02 — todos confirmados. Detalhe completo em `diagnostico-numbering-active-passo-182a.md` §1. Síntese:

- Field legacy: `01_core/src/entities/counter_state_legacy.rs:34`.
- Helper legacy: `01_core/src/entities/counter_state_legacy.rs:101–103` retorna `unwrap_or(false)`.
- Variant: `01_core/src/entities/content.rs:176` — `SetHeadingNumbering { active: bool }`.
- Eval producer: `01_core/src/rules/eval/rules.rs:227`.
- Walk arm canonical write: `01_core/src/rules/introspect.rs:455–457`.
- Layout walk write paralelo: `01_core/src/rules/layout/counters.rs:11–13` + `01_core/src/rules/layout/mod.rs:313–315`.
- Consumer Layouter heading-arm: `01_core/src/rules/layout/mod.rs:301`.
- Consumer Layouter equation-arm: `01_core/src/rules/layout/equation.rs:24`.
- Leitura intra-walk resolved_text: `01_core/src/rules/introspect.rs:360`.
- Leitura intra-walk equation step: `01_core/src/rules/introspect.rs:378`.
- Copy-sites fixpoint: `01_core/src/rules/layout/mod.rs:1414, 1442`.

Desvios face ao texto da lacuna em `m1-lacunas-captura.md` registados (variant é `{ active: bool }`, não `{ key, value }`; consumers são 2 Layouter + 2 intra-walk, não 1 só).

---

## 3. Inventário vanilla (sub-passo 182A.B)

`grep -rn "numbering_active\|is_numbering_active" lab/typst-original/crates/` → **zero matches**.

Mecanismo equivalente vanilla:
- `HeadingElem.numbering: Option<Numbering>` em `lab/typst-original/crates/typst-library/src/model/heading.rs:134`.
- `EquationElem.numbering: Option<Numbering>` em `lab/typst-original/crates/typst-library/src/math/equation.rs:67`.
- Resolução via StyleChain (`self.numbering.get_ref(styles)`); `Some(_)` → activa, `None` → inactiva.
- Default: `None` (numeração OFF).
- Granularidade: por elemento via StyleChain hierárquica (escopo léxico).

Cristalino diverge: HashMap global por chave, "última escrita ganha". P182 não corrige a divergência — mantém a forma cristalina e expõe-na via Introspector.

---

## 4. Decisões cláusula 1–6 (sub-passos 182A.C–H)

Síntese; detalhe O1–O5 em `diagnostico-numbering-active-passo-182a.md` §3.

| # | Cláusula | Decisão | Magnitude | Reversibilidade |
|---|----------|---------|-----------|-----------------|
| 1 | Mecanismo | **M1** — reusar `StateRegistry` P171 com chave `numbering_active:heading`; `extract_payload` arm `Content::SetHeadingNumbering` produz `ElementPayload::StateUpdate { key, update: Set(Value::Bool(active)) }` | trivial | alta |
| 2 | Default value | **OFF** | trivial | alta |
| 3 | Consumers | **2 Layouter** (`mod.rs:301`, `equation.rs:24`); 2 leituras intra-walk excluídas | trivial | alta |
| 4 | Localização exacta | tabela em diagnóstico §3 cláusula 4 (substitution-with-fallback P168) | trivial | alta |
| 5 | Forma da API | **A2** — `Introspector::is_numbering_active(&self, key: &str) -> bool`; impl delega a `state.final_value` + match `Value::Bool(true)` | trivial | alta |
| 6 | Critério de fecho | **Opção 3** — infra pronta + consumer migrado; legacy permanece até M6 (simétrico com lacuna #6 P181) | trivial | alta |

---

## 5. Plano de sub-passos (sub-passo 182A.I)

| Sub-passo | Escopo | Magnitude | Depende |
|-----------|--------|-----------|---------|
| `P182B` | Trait method `is_numbering_active(&self, key: &str) -> bool` em `01_core/src/entities/introspector.rs`; impl em `TagIntrospector`; tests unitários | S | — |
| `P182C` | `extract_payload` arm `Content::SetHeadingNumbering` produz `ElementPayload::StateUpdate { key: "numbering_active:heading", update: StateUpdate::Set(Value::Bool(active)) }`; walk arm legacy continua paralelo (M6) | S | `P182B` |
| `P182D` | Layouter consumers (`layout/mod.rs:301`, `layout/equation.rs:24`) migram para `self.introspector.is_numbering_active(key) \|\| self.counter.is_numbering_active(key)` (substitution-with-fallback P168) | S | `P182B`, `P182C` |
| `P182E` | Tests E2E confirmam paridade pipeline completo via Introspector e via state legacy (heading + equation) | S | `P182D` |
| `P182F` | Lacuna #4 marcada fechada em `m1-lacunas-captura.md` + relatório P182X | S | `P182E` |

Sem cláusulas condicionais. Sem gates "FULL vs INVENTORY_ONLY".

---

## 6. Magnitude consolidada

**S-M**. 5 sub-passos S, todos triviais individualmente. Total agregado ~ 1/3 de P181 (a metade-bibliography teve 9 sub-passos `.A`–`.I`; P182 colapsa em 5 porque infra P171 já cobre os equivalentes a P181B/C/D/E).

Estimativa P181J §5 (S-M) **mantém-se válida**.

---

## 7. ADR — avaliação (sub-passo 182A.J)

Não criada. Justificação literal: M1 reusa P171 sem extensão de enum (zero novo `ElementPayload` variant) e A2 replica P181F sem nova arquitectura. Detalhe em diagnóstico §6.

---

## 8. DEBT — avaliação

Não aberto. Pendências M6 são as canónicas (legacy field, write canonical, write paralelo, copy-sites, leituras intra-walk) — todas já implícitas no critério "Opção 3" e em F1 (auditoria-fresh-projecto). Detalhe em diagnóstico §7.

---

## 9. Plano de materialização (P182B+)

P182B (próximo) — trait method + impl + 5 tests unitários. Magnitude S. Sem dependência.

P182C — `extract_payload` arm. Magnitude S. Depende P182B.

P182D — migração 2 consumers Layouter via substitution-with-fallback. Magnitude S. Depende P182B + P182C.

P182E — testes E2E. Magnitude S. Depende P182D.

P182F — fecho da lacuna #4 + relatório consolidado. Magnitude S. Depende P182E.

Total: 5 sub-passos S; cumulativo S-M.

---

## 10. ADR

Não produzido (cf. §7).

---

## 11. DEBTs

Não aberto (cf. §8).

---

## 12. `m1-lacunas-captura.md` actualizado

Linha 62–69 (entrada lacuna #4) actualizada para reflectir decisões P182A:

```
**P182A decisões fixadas**: 6 cláusulas resolvidas (link para
`diagnostico-numbering-active-passo-182a.md`). Mecanismo: M1
(reusar StateRegistry P171). Default: OFF. Consumers: 2 Layouter
(mod.rs:301 + equation.rs:24); 2 leituras intra-walk excluídas.
API: A2 (helper `Introspector::is_numbering_active(key) -> bool`).
Critério de fecho: Opção 3 (infra + consumer migrado; legacy
permanece até M6, simétrico com #6). Plano P182B–P182F (5
sub-passos S, magnitude S-M total) sem condicionais. Próximo:
P182B (trait method + impl + tests).
```

Tabela §Resumo (linha 127) actualizada de "Adiar — M9 ou passo dedicado" para "P182A decisões fixadas; plano P182B–P182F (S-M) em curso".

---

## 13. Próximo passo

**P182B** — trait method `is_numbering_active`.

Escopo concreto:
1. Adicionar `fn is_numbering_active(&self, key: &str) -> bool;` ao trait `Introspector` em `01_core/src/entities/introspector.rs:104` (após `bib_number_for_key`).
2. Adicionar impl em `impl Introspector for TagIntrospector`:
   ```rust
   fn is_numbering_active(&self, key: &str) -> bool {
       matches!(self.state.final_value(key), Some(Value::Bool(true)))
   }
   ```
3. 5 tests unitários em `mod tests`:
   - vazio devolve false;
   - apply Set Bool(true) → retorna true;
   - keys distintas isoladas;
   - Bool(false) retorna false;
   - non-Bool value retorna false.
4. `cargo test --workspace --lib`: 1.738 verdes + 5 novos.
5. `crystalline-lint .`: zero violations.

Escopo NÃO inclui: `extract_payload` (P182C), Layouter consumers (P182D), tests E2E (P182E), fecho lacuna (P182F).

---

## 14. Verificação final

| Item | Estado |
|------|--------|
| Diagnóstico `00_nucleo/diagnosticos/diagnostico-numbering-active-passo-182a.md` (8 secções) produzido | ✅ |
| Relatório `00_nucleo/materialization/typst-passo-182a-relatorio.md` (14 secções) produzido | ✅ |
| 6 cláusulas fechadas com decisão literal | ✅ (M1, OFF, 2 consumers, tabela 4 linhas, A2, Opção 3) |
| Plano sub-passos sem condicionais (escopo + magnitude + dependência) | ✅ (5 sub-passos B–F) |
| `m1-lacunas-captura.md` actualizado | ✅ (entrada lacuna #4 + tabela resumo) |
| Magnitude consolidada | ✅ (S-M) |
| Critério de fecho lacuna #4 fixado em palavras verificáveis | ✅ (Opção 3, 3 critérios literais em diagnóstico §3 cláusula 6) |
| ADR avaliada | ✅ (não criada — justificação literal §7) |
| DEBT avaliada | ✅ (não aberto — justificação literal §8) |
| Nenhum ficheiro em `01_core/`, `02_shell/`, `03_infra/`, `04_wiring/` tocado | ✅ (zero modificações fora de `00_nucleo/`) |
| `cargo test --workspace --lib` 1.738 inalterados | ✅ (não correu — código não tocado) |
| `crystalline-lint .` zero violations | ✅ (não correu — código não tocado) |
