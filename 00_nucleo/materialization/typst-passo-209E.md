# Passo 209E — Encerramento série P209

**Série**: 209 (sub-passo `E` final).
**Marco**: M9c (encerramento série P209).
**Tipo**: encerramento série + decisão de transição
ADR-0077.
**Magnitude**: S (~30min-1h) documental puro.
**Pré-condição**: P209D concluído; série P209
materializada em 4 sub-passos (A diagnóstico + B Label/Location
+ C And/Or + D Regex/ADR-0077); Selector enum 6
variants; trait 26 métodos; tests 1935 verdes; 0
violations; ADR-0076 PROPOSTO; ADR-0077 PROPOSTO;
blueprint §3.0quinquies [P208D].
**Output**: 1 ficheiro (relatório curto encerramento +
transição P210).

---

## §1 Trabalho

Encerrar série P209. **2 decisões** fixadas em C1 com
base em evidência empírica:

- **ADR-0077 transição PROPOSTO → ACEITE em P209E vs
  P212**.
- **Caminho 1 (puro) vs Caminho 2 (`native_regex` Opção
  α reabrir)** análogo a P207E + P208D.

Reuso de dados toda a trajectória M9c:

- ADR-0077 PROPOSTO escrito em P209D com critério de
  validação documentado.
- Pattern "Caminho 1 anti-inflação" 6 aplicações
  consecutivas.
- Pattern marca-por-fecho blueprint (§3.0quater P207E
  + §3.0quinquies P208D).
- Pattern distinção encerramento-série vs encerramento-marco
  (P207E formalizado).

---

## §2 Cláusulas (4)

### C1 — Diagnóstico breve: 2 decisões

Antes de tocar código, inventário focado em **2
sub-secções**:

#### C1.1 — ADR-0077 transição

Comparar 2 caminhos:

- **Caminho A — Transitar PROPOSTO → ACEITE em P209E**:
  ADR-0077 é dep-específica e isolada (cobre `regex`
  em allowlist L1 + wrapper L1, não cobre o marco M9c
  inteiro). Critério de validação ADR-0077 §4 (tests
  + lint + build) verificado empíricamente em P209D.
  Sem dependência futura que possa invalidar.
- **Caminho B — Manter PROPOSTO até P212**: paralelo a
  ADR-0076 que cobre M9c inteiro e aguarda P212.
  Consistência temporal entre ADRs do mesmo marco.

Critério literal:

- ADR-0077 é **independente** de ADR-0076? Sim — cobre
  dep diferente, escopo distinto.
- ADR-0077 tem **risco de reversão** se passos futuros
  M9c revelarem necessidade de mudar? Improvável — dep
  está em allowlist + crate fixed em workspace; passos
  futuros podem adicionar `native_regex` Opção α mas
  isso reusa ADR-0077, não a contradiz.
- ADR-0023/ADR-0024 (deps anteriores L1) — qual
  pattern? Verificar empíricamente o ciclo PROPOSTO →
  ACEITE típico.

Hipótese provável: **Caminho A**. ADR-0077 é dep-específica
e independente; nada a esperar.

#### C1.2 — Caminho 1 (puro) vs Caminho 2 (reabrir
`native_regex` Opção α)

P209D C6 fixou Opção γ (deferred stdlib func). Avaliar:

- **Caminho 1 — Encerramento documental puro**: per
  pattern P207E/P208D. Sem `native_regex`. Magnitude
  S documental.
- **Caminho 2 — Materializar `native_regex` agora**:
  reabrir Opção α (`Value::Regex` variant) ou β
  (`Value::Other` catch-all). Magnitude M+.

Critério literal:

- Consumer real emergiu durante P209? Esperado: não.
- P210+ desbloqueia consumer imediato para `native_regex`?
  Counter/State (P210) não invoca regex; Outline (P211)
  não invoca regex. Improvável.
- Pattern emergente "Caminho 1 anti-inflação" — 6
  aplicações consecutivas; 7ª aplicação justificada se
  consumer ausente.

Hipótese provável: **Caminho 1**. 7ª aplicação do
pattern.

### C2 — Materializar decisões fixadas em C1

Independente de C1.1 e C1.2 escolhas:

**Anotações documentais**:

- `00_nucleo/adr/typst-adr-0076-introspector-completion.md`
  — §Plano de materialização:
  - Série P209 transita "EM CURSO" → "✅ MATERIALIZADO
    ({data})".
  - §P209E anotado com forma fixada em C1.
  - Bloco "Série P209 — encerrada" com sumário literal
    dos 5 sub-passos (A + B + C + D + E) + métricas
    agregadas.
- `00_nucleo/adr/typst-adr-0077-regex-l1.md` —
  conforme C1.1:
  - Se Caminho A: estado PROPOSTO → ACEITE +
    `Histórico` anotado.
  - Se Caminho B: estado mantém PROPOSTO; anotação
    "aguarda P212".
- `00_nucleo/diagnosticos/blueprint-projecto.md` —
  §3.0sexies marca adicionada (paralelo a §3.0quinquies
  [P208D]).

**Se C1.2 = Caminho 2** (improvável): materializar
`native_regex` stdlib func + Value::Regex variant ou
Value::Other catch-all. Magnitude M+ — sub-spec extensa.

### C3 — Verificação final

```
cargo test --workspace 2>&1 | tail -10
crystalline-lint .
crystalline-lint --fix-hashes .
```

Critério: tests verdes (1935 + N onde N depende de C2);
0 violations.

### C4 — Encerramento série P209 (resumo agregado)

ADR-0076 §Plano de materialização ganha bloco
"Agregado série P209" com:

- Sumário 5 sub-passos (A + B + C + D + E).
- Métricas Δ série (variants +5, query arms +5, tests
  +28, L0 prompts novos 2, L1 ficheiros novos 1,
  ADRs novas 1, allowlist deps +1).
- Patterns formalizados (Caminho 1 anti-inflação 6→7
  aplicações; recursive Hash funciona transparentemente;
  Opção c Rust API only para compósitos).

---

## §3 Output

1 ficheiro:
`00_nucleo/materialization/typst-passo-209E-relatorio.md`.

Estrutura (~4-6 KB) com 7 §s padrão (paralelo a P207E /
P208D):

- §1 O que foi feito (sumário 3-5 linhas).
- §2 Decisões fixadas em C1 (com evidência empírica).
- §3 Alterações documentais (anotações ADR + blueprint).
- §4 Decisões substantivas.
- §5 Métricas (compacta).
- §6 Encerramento série P209 (sumário 5 sub-passos +
  agregado).
- §7 Próximo sub-passo (P210).

---

## §4 Não-objectivos

- Counter/State extras Q1β (P210).
- Outline configurável (P211).
- Encerramento M9c inteiro (P212).
- Transição ADR-0076 PROPOSTO → ACEITE (P212).
- Materialização `native_regex` se C1.2 = Caminho 1
  (deferred).
- `Value::Regex` variant em `Value` enum (deferred).
- `Selector::Where`/`Before`/`After`/`Within` (fora
  roadmap M9c).

---

## §5 Riscos a evitar

1. **Forçar Caminho 2 por completude vanilla**: P209D
   C6 Opção γ fixou anti-inflação. P209E mantém. Per
   `P205A.div-1` divergência arquitectónica legítima.
2. **Inflar encerramento série**: P209E é encerramento
   de série, não consolidado M9c. Sumário literal 5
   sub-passos.
3. **Confundir encerramento série vs marco**: P209E
   fecha série P209; M9c continua. ADR-0076 mantém
   PROPOSTO até P212 independente da decisão C1.1
   sobre ADR-0077.
4. **Esquecer marca blueprint**: pattern §3.0/3.0bis/
   3.0ter/3.0quater/3.0quinquies — P209E adiciona
   §3.0sexies.
5. **ADR-0077 transição prematura sem critério**: se
   C1.1 = Caminho A, transição precisa de critério
   explícito (tests + lint + build verificados). Não
   é decisão estética.
6. **Esquecer regra empírica P207B §5**: confirmar
   pela última vez na série P209 que regra **não foi
   accionada** — Selector extension não toca trait
   Introspector. Trait mantém 26 métodos. Útil para
   consolidação no agregado série.

---

## §6 Hipótese provável

C1.1 fixará **Caminho A** (ADR-0077 PROPOSTO → ACEITE).
ADR é dep-específica e independente; critério §4
verificado empíricamente em P209D; sem dependência
futura que possa invalidar.

C1.2 fixará **Caminho 1** (encerramento documental
puro). 7ª aplicação consecutiva do pattern "Caminho 1
anti-inflação" — princípio operacional consolidado.

Trabalho documental:
- ADR-0076 anotado (série P209 fechada + agregado).
- ADR-0077 PROPOSTO → ACEITE (se A) ou anotação
  "aguarda P212" (se B).
- Blueprint §3.0sexies marca.
- Relatório.

Magnitude S (~30min-1h documental).

Mas é hipótese, não decisão. C1 fixa-se empíricamente.
