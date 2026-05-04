# Relatório consolidado — Série P188

**Período**: 2026-05-04 (P188A diagnóstico + P188B implementação)
**Magnitude agregada**: S (sub-passo único de implementação após
diagnóstico)
**Estado**: ✅ Série fechada (A ✅ B ✅) — **M4-residual fechado funcionalmente**
**ADR vinculada**: nenhuma (replicação P187B)
**DEBT**: M4-residual vazio em prática

---

## §1 Resumo executivo

Migração C2 (equation counter) materializada em
`equation.rs:97`. **Último passo funcional de M4-residual.**
Replica padrão P187B com 2 diferenças:

1. Primitiva `flat_counter_at` (P185B) em vez de
   `formatted_counter_at` (P177).
2. `unwrap_or_else` em vez de `or_else` (legacy `get_flat`
   retorna `usize`, não `Option<usize>`).

```rust
let n = self.current_location
    .and_then(|loc| self.introspector.flat_counter_at("equation", loc))
    .unwrap_or_else(|| self.counter.get_flat("equation"));
```

**P188 é o primeiro consumer da série M4-residual onde
migração estrutural não traduz em mudança funcional em
produção.** Introspector path migra mas é **dormente em
produção** porque `Content::SetEquationNumbering` ausente
em cristalino (P186A §11.2). Fallback legacy `get_flat` é
caminho funcional **permanente** até equation set rule
materializar.

Δ tests cumulativo: **+3** (1805 → 1808) com **zero
regressões**. Output observable em produção **inalterado**
(diferente de P187B onde caminho funcional muda).

**M4-residual fechado funcionalmente**:
- C1 fechado em P187B (Introspector funcional).
- C2 fechado em P188B (Introspector dormente; fallback
  legacy permanente).
- C3 fechado em P184D (Introspector funcional).
- 6 outros via P181G/P182D/P184D/P186 estabelecidos.

DEBT M4-residual **vazio em prática**.

---

## §2 Sub-passos materializados

| Passo | Magnitude planeada | Magnitude real | Δ tests | L0s tocados |
|-------|---------------------|-----------------|---------|-------------|
| **P188A** | S (diagnóstico) | S | 0 | nenhum |
| **P188B** | S (agregado) | S | **+3** | `rules/layout.md` |
| **Total** | — | — | **+3** | 1 L0 produção |

P188B agregou em sub-passo único:
- `.B` migração consumer C2 em `equation.rs:97` + comentário
  inline obrigatório.
- `.C` actualização L0 `rules/layout.md` (nova secção C2 +
  estado dormente honestamente documentado).
- `.D` 3 tests E2E em `mod p188b_c2_equation_counter`.
- `.E` verificação estrutural (13/13).
- `.F` actualização nota DEBT M4-residual (in-line neste
  relatório).
- `.G` relatório consolidado P188 (este ficheiro).

---

## §3 Decisões arquiteturais

### 7 cláusulas P188A fechadas

| # | Cláusula | Decisão | Sub-passo |
|---|----------|---------|-----------|
| 1 | Forma da expressão | **Opção A** inline com `and_then(...).unwrap_or_else(legacy)` | P188B `.B` |
| 2 | `None` do Introspector | **Opção A** `unwrap_or_else` (caminho funcional permanente) | P188B `.B` |
| 3 | `None` do `current_location` | **Opção B** `and_then` defensivo | P188B `.B` |
| 4 | Receptor | **Cenário 1** — `self` é Layouter directamente | P188B `.B` |
| 5 | Forma de retorno | **`usize`** preservado | P188B `.B` |
| 6 | Documentação inline | **Opção A** comentário curto + cross-reference P186A §11.2 | P188B `.B` |
| 7 | Critério fecho | **Opção 3** — consumer + tests E2E + DEBT vazio em prática | P188B `.G` |

### Sem ADR — replicação de padrão

Substitution-with-fallback é padrão estabelecido P187B
(que já replicava P184D). Decisões registadas em P188A §2.

---

## §4 Achados não-triviais durante execução

### P188A §11.1 — Diferença de tipo legacy (`get_flat -> usize`)

`get_flat("equation")` retorna `usize` directamente, não
`Option<usize>`. Diferente de `format_hierarchical("heading")`
(P187B) que retorna `Option<String>`. Consequência sintáctica:
`unwrap_or_else` em vez de `or_else`. Diferença mecânica,
não conceptual.

### P188A §11.4 — `Content::SetEquationNumbering` ausente confirmado

`grep -rn "SetEquationNumbering" 01_core/src/` retorna zero
hits em produção. Apenas referências em comentários
(`equation.rs:25-29`, `element_payload.rs:113`,
`locatable.rs:55`). Confirmação empírica de P186A §11.2.

### P188A §11.5 — Primeira migração com Introspector dormente

P188 é o **primeiro caso da série M4-residual** onde
migração estrutural não traduz em mudança funcional em
produção. Comparação:

| Caso | Introspector em produção | Caminho funcional |
|------|---------------------------|-------------------|
| C3 Figure (P184D) | activo | Introspector |
| C1 Heading prefix (P187B) | activo | Introspector |
| **C2 Equation counter (P188B)** | **dormente** | **fallback legacy permanente** |

### P188A §11.6 — Documentação obrigatória em 4 pontos

Honestidade sobre estado dormente exigiu documentação em:
1. ✅ Comentário inline em `equation.rs:97-104` (P188B `.B`).
2. ✅ Secção em L0 `rules/layout.md` "C2 equation counter
   migrado (P188B)" com sub-secção "Estado dormente em
   produção" (P188B `.C`).
3. ✅ Test `c2_equation_counter_via_fallback_legacy_caso_producao`
   em `mod p188b_c2_equation_counter` (P188B `.D`).
4. ✅ Esta secção §5 do relatório consolidado (P188B `.G`).

Sem estes 4 pontos, leitores futuros poderiam assumir
incorrectamente que P188 fecha completamente C2
funcionalmente.

### P188B `.D.2` — Test caso central produção empiricamente confirma estado dormente

Test `c2_equation_counter_via_fallback_legacy_caso_producao`
exercita o cenário real de produção (sem
`Content::StateUpdate` para `numbering_active:equation`).
Asserções:
- `intr.flat_counter_at("equation", loc)` retorna `None`
  para todas as locations.
- `unwrap_or_else` cai em `get_flat` legacy.
- `plain_text` contém numerações correctas via fallback.

Empiricamente confirma que cenário de produção real é
coberto pelo fallback legacy.

---

## §5 Estado dormente (secção dedicada)

### Diferença observable

P188 é o **primeiro consumer onde migração estrutural não
traduz em mudança funcional em produção**.

Em produção real:
- `Content::SetEquationNumbering` ausente → walk não emite
  Tag para state `numbering_active:equation`.
- State em TagIntrospector permanece sem entry para essa
  key.
- Gate em `from_tags` arm Equation (P186E) bloqueia.
- `intr.counters` permanece sem entry para chave `"equation"`.
- `flat_counter_at("equation", *)` retorna sempre `None`.
- `unwrap_or_else` cai sempre em `get_flat` legacy.

Caminho funcional em produção: **fallback legacy permanente**.

### Razão

Documentado em P186A §11.2:

> `Content::SetEquationNumbering` não existe em cristalino.
> Em runtime real, state `numbering_active:equation`
> nunca é populado. Gate em P186E nunca dispara → counter
> introspector permanece vazio.

Confirmado empiricamente em P188A §11.4 (zero hits em
grep).

### Trabalho identificado fora série

**`Content::SetEquationNumbering` materialização** — passo
dedicado quando equation set rule for prioridade. Inclui:
- Adicionar variant `Content::SetEquationNumbering { active }`
  ao Content enum.
- Arm em `extract_payload` que produz `ElementPayload::StateUpdate
  { key: "numbering_active:equation", update: Set(Bool(active)) }`
  (replica padrão P182C `SetHeadingNumbering`).
- Arm no eval que emite o variant.
- Tests E2E.

Após esse passo:
- State é populado via tag StateUpdate.
- Gate em P186E dispara → counter introspector populado.
- `flat_counter_at` retorna `Some(n)` em produção.
- Caminho Introspector activa-se automaticamente sem
  alteração ao código P188B.

### Janela compat M6

**Não fecha** para Equation enquanto
`Content::SetEquationNumbering` não materializar — diferente
de C1 (que pode fechar imediatamente quando F1 retomar).

Após equation set rule:
- `CounterStateLegacy.flat["equation"]` removido.
- `step_flat("equation")` em walk arm legacy
  (`introspect.rs:377-382`) removido.
- `get_flat("equation")` no fallback C2 removido.
- `numbering_active.insert("equation", ...)` em testes
  removido.

---

## §6 Estado final M9 e M5/M4

### M9 (counter-feature) — inalterado: 11/11

P188 não introduz feature M9 nova. Migração de consumer C2.

### M5/M4 (read-site migration) — **8/12** (era 7/12)

C2 fechado **estruturalmente** em P188B. Read-sites
migrados:
- C3 figure-arm (P184D) ✓
- C1 heading-prefix (P187B) ✓
- **C2 equation-counter (P188B) ✓ — NEW** (estrutural;
  Introspector dormente em produção)
- 5 outros via P181G/P182D/P184D/P186 estabelecidos.

Restantes 4/12 são **fora-de-escopo M4-residual**:
- TOC (resolved labels durante walk).
- Fixpoint side-channels.
- Outros casos pendentes em P183E não corrido.

### Trait `Introspector` — 18 métodos (inalterado)

### Layouter — sem mudança em fields

### `equation.rs:97` — consumer migrado

Antes P188: `let n = self.counter.get_flat("equation")`.
Após P188: substitution-with-fallback location-aware
(Introspector primeiro com `unwrap_or_else` para legacy).

---

## §7 Estado final lacunas

Inalterado em P188. As lacunas catalogadas até P187
permanecem na mesma situação.

---

## §8 Pendências cumulativas + DEBT M4-residual

### DEBT M4-residual — **vazio em prática** após P188B

**Antes de P187**: notas preventivas mencionavam DEBT
cobrindo C1+C2.

**Após P187B**: DEBT cobre apenas C2.

**Após P188B**: DEBT M4-residual **vazio em prática**.
- C1 fechado (Introspector funcional).
- C2 fechado estruturalmente (Introspector dormente;
  fallback legacy permanente).

**P183F formal pode ser dispensado** (DEBT vazio antes de
abrir formalmente). Decisão:
- Cenário X — abrir e arquivar imediatamente como histórico.
- Cenário Y — dispensar abertura formal; nota cumulativa
  basta.

Decisão fica para passo subsequente (M5 P189 ou trabalho
posterior).

### Trabalho identificado fora de série

- **`Content::SetEquationNumbering` materialização** —
  activa caminho Introspector para C2 em produção; permite
  janela compat M6 abrir para Equation.
- **F1 retomar (M6 janela compat)** — para Heading +
  Figure (C1 + C3) pode abrir imediatamente; para Equation
  (C2) depende de equation set rule.

---

## §9 Próximos passos sugeridos

### M4-residual fechado — segue M5

1. **P189 — M5 walk puro**: começar fase M5 (próxima
   evolução pós-M4-residual). Eliminar mutações state
   legacy durante walk de introspect (objectivo P163
   originalmente).

### Trabalho identificado fora de escopo M4-residual

2. **`Content::SetEquationNumbering`** — passo dedicado
   quando equation set rule for prioridade.
3. **C4 (resolved label TOC)** — pendente em P183E não
   corrido; pode ser revisitado em M5 ou posteriormente.
4. **F1 retomar (M6 janela compat)** — eliminar
   `CounterStateLegacy` para C1 + C3 imediatamente; para
   C2 após `Content::SetEquationNumbering`.

---

## §10 Conclusão

P188 fechou em 2 sub-passos (A diagnóstico + B
implementação agregada) com magnitude correctamente
estimada (S em ambos). Replicação literal de P187B com
diferença sintáctica (`unwrap_or_else` por tipo) e
honestidade documental obrigatória sobre estado dormente.

Achados centrais:
- **C2 fechado estruturalmente** com Introspector dormente
  em produção. Diferença explícita face a P184D Figure e
  P187B C1 (ambos funcionais).
- **Documentação obrigatória em 4 pontos** materializada
  (comentário inline, L0, test, relatório consolidado).
- **Test caso central produção** confirma empiricamente
  que fallback legacy é o caminho activo.
- **M4-residual fechado funcionalmente** após P188B.
- **DEBT M4-residual vazio em prática**.

Diferença chave face P187 (C1): em P187, Introspector é
**caminho funcional** porque `Content::SetHeadingNumbering`
existe (P182C). Em P188, Introspector é **dormente** porque
`Content::SetEquationNumbering` não existe ainda.

P188 termina como **fim funcional de M4-residual**. M5
(P189) começa próximo. **58 passos executados** após
P188B.

Padrão diagnóstico-primeiro mantido — 13/13 acertaram a
magnitude planeada ±1 nível
(P131A/132A/140A/148/154A/181A/182A/183A/184A/185A/186A/
187A/188A).
