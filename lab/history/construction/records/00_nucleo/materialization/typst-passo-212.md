# Passo 212 — Encerramento M9c (marco)

**Série**: 212 (sub-passo único — encerramento marco).
**Marco**: **M9c (encerramento)**.
**Tipo**: encerramento marco minimal documental.
**Magnitude**: S (~30min-1h).
**Pré-condição**: P211A concluído; série P211 fechada
em 1 sub-passo; trait 26 métodos; Selector 6 variants;
stdlib funcs ~53; tests 1939 verdes; 0 violations;
ADR-0076 PROPOSTO anotado em todas as séries P207-P211;
ADR-0077 ACEITE; blueprint marcas §3.0quater a
§3.0septies (P211 sem marca per Caminho 1 puro fixado
em P211A); 5 séries M9c fechadas.
**Output**: 1 ficheiro (relatório curto encerramento
marco).

---

## §1 Trabalho

Encerrar marco M9c (M9-completion). Trabalho minimal
documental — paralelo em escala a encerramentos série
(P207E/P208D/P209E/P210C/P211A) mas escopo qualitativamente
distinto (cobre 5 séries cumulativas).

**Decisão central de P212**: auditar as 7 condições
§Plano de validação ADR-0076 + transitar PROPOSTO →
ACEITE se todas satisfeitas.

Reuso de dados toda a trajectória M9c:

- 5 séries fechadas (P207-P211).
- 9 aplicações cumulativas anti-inflação.
- 7 marcas blueprint cirúrgicas.
- ADR-0076 (M9c marco) PROPOSTO + ADR-0077 (regex L1)
  ACEITE.
- Pattern emergente "Caminho 3 honest subset" (1
  aplicação P210).
- Pattern emergente "série diagnóstico-only" (1
  aplicação P211).

---

## §2 Cláusulas (4)

### C1 — Auditoria das 7 condições ADR-0076

ADR-0076 §Plano de validação tem 7 condições para
transição PROPOSTO → ACEITE. C1 audita cada uma
empíricamente:

1. **Series P207-P211 todas materializadas** (esperado:
   ✓ — séries fechadas via P207E/P208D/P209E/P210C/P211A).
2. **Trait `Introspector` extensões consolidadas**
   (esperado: ✓ — 20 → 26 métodos; +6 série P207).
3. **Sub-stores L1 adicionados consolidados**
   (esperado: ✓ — `PageStore` P207D + `Regex` wrapper
   P209D).
4. **Stdlib funcs `here`/`locate`/`counter_step`
   registadas** (esperado: ✓ — +3 funcs séries P208/P210).
5. **`Selector` enum extensões** (esperado: ✓ — 1 → 6
   variants; +5 série P209).
6. **Tests workspace verdes** (esperado: ✓ — 1873 →
   1939; +66 cumulative).
7. **`crystalline-lint` 0 violations** (esperado: ✓ —
   preservado toda a trajectória).

Se todas as 7 condições satisfeitas: **fixar ACEITE em
C2**. Se alguma falhar: registar `P212.div-N` e fixar
fallback (manter PROPOSTO + identificar gap).

Hipótese provável: todas as 7 satisfeitas.

### C2 — Transição ADR-0076 PROPOSTO → ACEITE

Editar `00_nucleo/adr/typst-adr-0076-introspector-completion.md`:

- Status: **PROPOSTO → ACEITE 2026-05-12**.
- Data actualizada.
- Histórico anotado com:
  - Transição em P212.
  - Verificação empírica das 7 condições C1.
  - Cross-reference a relatório P212 (este).

ADR-0076 fecha como ACEITE; passa a referência
arquitectural estável.

### C3 — Marca blueprint final M9c

Editar `00_nucleo/diagnosticos/blueprint-projecto.md`:

**Decisão sobre forma da marca**:

- **Opção α — Marca §3.0octies cumulativa**: paralela a
  §3.0septies (P210C). Consistência com pattern
  marca-por-fecho.
- **Opção β — Section nova §3.1 ou similar**: distingue
  encerramento marco de encerramentos série. Mais
  estruturante mas inflação possível.
- **Opção γ — Marca §3.0octies + nota explícita
  "encerramento M9c marco"**: híbrida. Distingue na
  semântica sem criar section nova.

Critério: continuidade do pattern emergente.

Hipótese provável: **Opção γ** — §3.0octies marca com
nota "encerramento marco M9c (5 séries fechadas
cumulativamente)".

Conteúdo da marca:
- M9c encerramento confirmado.
- ADR-0076 ACEITE 2026-05-12.
- 5 séries fechadas (P207-P211).
- Métricas cumulativas (Δ marco).
- 9 aplicações anti-inflação cumulativas.
- Patterns emergentes formalizados.
- Estado pós-M9c: trajectória aberta para próximo marco
  (M10? F4? — fora de escopo P212).

### C4 — Verificação final

```
cargo test --workspace 2>&1 | tail -10
crystalline-lint .
crystalline-lint --fix-hashes .
```

Critério: tests 1939 verdes (Δ 0); 0 violations.

---

## §3 Output

1 ficheiro:
`00_nucleo/materialization/typst-passo-212-relatorio.md`.

Estrutura (~5-7 KB) com 8 §s:

- §1 O que foi feito (sumário 3-5 linhas).
- §2 Auditoria 7 condições ADR-0076 (tabela compacta).
- §3 ADR-0076 transição (estado pré/pós; histórico
  anotado).
- §4 Blueprint marca §3.0octies (forma fixada em C3).
- §5 Decisões substantivas.
- §6 Métricas cumulativas M9c (tabela Δ marco).
- §7 Encerramento M9c — sumário literal 5 séries +
  patterns emergentes.
- §8 Próximo passo (fora de P212; recomendação para
  próxima sessão).

---

## §4 Não-objectivos

- Materializar trabalho deferred (display/get,
  Content::Context, walk advance, page-meta capture,
  outline configurável, etc.) — todos com critério de
  reabertura documentado.
- Próximo marco arquitectónico (M10? F4? — fora
  escopo P212).
- Resolver divergência outline-toc principal (out of
  M9c).
- Re-auditar pre-M9c trabalho (ADRs anteriores
  intactas).
- ADR-0077 transição (já ACEITE em P209E).
- Re-escrita blueprint ampla (preserva pattern
  marca-por-fecho).

---

## §5 Riscos a evitar

1. **Inflar para relatório consolidado estilo M7/M8**:
   humano fixou variante minimal. Manter paralelo em
   escala a encerramentos série. Sumário literal,
   não narrativa.
2. **Esquecer auditoria empírica das 7 condições**: C1
   exige verificação real, não assumpção. Cada
   condição com evidência registada.
3. **Transição ACEITE prematura sem auditoria**: C1
   deve preceder C2. Pattern paralelo a ADR-0077 em
   P209E (que verificou 8 critérios antes de transição).
4. **Confundir encerramento marco vs encerramento série**:
   P212 cobre 5 séries cumulativas. Métricas, patterns,
   marca blueprint têm escopo distinto.
5. **Marca blueprint inflada**: §3.0octies preserva
   pattern minimal de marca-por-fecho. Não reescrever
   blueprint amplo.
6. **Deferreds documentados explicitamente preservados**:
   relatório P212 deve listar deferreds M9c com critério
   de reabertura para auditoria futura. Esses são:
   - P205D `SealedLabelPages`.
   - P207E captura page-meta.
   - P208B walk advance + P208D `Content::Context` block.
   - P209D `native_regex` stdlib func.
   - P210 `counter.display` + `state.get` here-aware.
   - P211 outline configurável.
   - `Selector::Where` (Q2=γ).
   - `Selector::Before/After/Within` (fora roadmap).
   - `query_count_before` (Q4=β).

---

## §6 Hipótese provável

C1 satisfará todas as 7 condições — pattern consistente
M9c em todas as séries.

C2 fixará transição ACEITE 2026-05-12.

C3 fixará Opção γ (§3.0octies marca com nota
"encerramento marco M9c").

C4 reportará tests/lint preservados.

Custo real: S (~30-45min documental). Sem código
tocado.

Mas é hipótese, não decisão. C1-C4 fixam-se empíricamente.

---

## §7 Particularidade P212

P212 é estruturalmente único na trajectória M9c:

- **Único sub-passo de encerramento marco** (vs
  encerramentos série anteriores).
- **Transita ADR de marco** (ADR-0076 PROPOSTO → ACEITE)
  — paralelo histórico a ADRs anteriores transitadas em
  encerramentos marco (M5/M6/M7/M8).
- **Estabelece referência arquitectural estável** —
  pós-ACEITE, ADR-0076 é base de auditoria + planeamento
  para marcos futuros.

Por isso C5 §3 risco 3 ("transição prematura") é
relevante — auditoria empírica das 7 condições é
condição **necessária** para ACEITE, não formalidade.
