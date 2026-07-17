# typst-passo-310 — formalizar Opção A da auditoria IEEE 754 (P309)

**Tipo**: Passo de Execução (planeamento táctico)
**Sub-tipo**: documental — sem código L1 tocado
**Data**: 2026-05-20
**Magnitude**: S documental
**Pré-requisitos**: P309 fechado (`diagnostico-ieee754-passo-309.md`
publicado); decisão humana **Opção A** registada
**Materialização**: ADR nova + drift L0 duplo + anotação cumulativa
ADR-0033

---

## 1. Motivação

P309 catalogou 67 sítios L1 com política IEEE 754 dupla:
- `eval`/layout/operators propaga (paridade vanilla, Cat B).
- `stdlib/calc.rs` rejeita NaN+Inf via `guard_float` (divergência
  vanilla, Cat A).

P309 §7 enunciou 4 opções (A/B/C/D); §9 recomendou D primária.
**Decisão humana**: **Opção A — Conformidade IEEE 754 total (status
quo divergente)**.

Implicações registadas em P309 §7.1:
- Manter `guard_float` em todos os 11 sítios Cat A.
- Documentar politicamente em ADR nova.
- Refino P308 (`erf` com A&S + short-circuit ±0) mantém-se útil.
- Reverter zero código.
- Migração agregada libm (futuro passo M+) substitui `f64::*` por
  `libm::*` **mantendo** `guard_float`.
- Custo zero implementação. Custo alto paridade.

P310 materializa a decisão **apenas em documentação** — código fica
intocado.

---

## 2. Objectivo do passo

Produzir três artefactos documentais:

1. **ADR-0101** — `Excepção IEEE 754 em stdlib: rejeita NaN/Inf via
   guard_float` — formaliza Opção A. Status `EM VIGOR`.
2. **Drift L0** em `00_nucleo/prompts/engine/stdlib.md` +
   `00_nucleo/prompts/engine/eval.md` — anotar política e cross-ref a
   ADR-0101. Remove contradição declarativa entre os dois L0 por
   **clarificação** (não por mudança de código).
3. **Anotação cumulativa P310 em ADR-0033** (paridade observable) —
   secção `§Anotação cumulativa P310 — Excepção IEEE 754 stdlib`
   referencia ADR-0101 para preservar paridade observable como regra
   geral **com** excepção stdlib documentada.

Sem código tocado. Sem testes tocados. Sem variants/types/traits novos.

---

## 3. Não-objectivos

P310 **não**:

- Materializa Opção B/C/D (rejeitadas).
- Toca L1.
- Reverte P308.
- Toca testes.
- Migra DEBT-libm (ortogonal; futuro passo).
- Reforça Cat D (Float→Int saturating, color range checks) —
  preocupações P309 §9 §10.5/§10.6 ficam adiadas como futuros
  candidatos a reforço pontual, fora do escopo Opção A.
- Promove ADR-0101 a meta-ADR (não há generalização; é decisão
  política específica).
- Adiciona ADR de revogação ou modificação a ADR-0033 — apenas
  anotação cumulativa per ADR-0093 Pattern 2.

---

## 4. Ficheiros tocados (previsão)

### 4.1 — Novos

- `00_nucleo/adr/typst-adr-0101-ieee754-restricao-stdlib.md` —
  ADR nova.

### 4.2 — Modificados (drift L0 deliberado)

- `00_nucleo/prompts/engine/stdlib.md` — secção nova `§"Política IEEE
  754 — guard_float"` com cross-ref ADR-0101. Hash actual `d4c214e1`
  (pós-P308) → novo hash via `--fix-hashes`.
- `00_nucleo/prompts/engine/eval.md` — secção nova `§"Política IEEE
  754 — propagação silenciosa"` com cross-ref ADR-0101. Drift L0
  segundo prompt simultâneo (paralelo P306/P308 mas em prompt
  distinto).
- `00_nucleo/adr/typst-adr-0033-paridade-funcional-vanilla.md` —
  anotação cumulativa P310 anexada (§"Anotação cumulativa P310 —
  Excepção IEEE 754 stdlib"); status `EM VIGOR` preservado literal
  (paralelo P266 §"Anotação cumulativa P266"  etc.).

### 4.3 — Propagados mecanicamente (linter `--fix-hashes`)

- 11× `01_core/src/engine/stdlib/*.rs` — `@prompt-hash` actualizado de
  `d4c214e1` para novo hash de `stdlib.md`.
- N× ficheiros L1 que consomem `eval.md` (a determinar via grep
  `@prompt 00_nucleo/prompts/engine/eval.md`).

### 4.4 — Não tocados

- `entities/content.rs` — hash inalterado (**27º consecutivo
  esperado**).
- `export/*` — bit-exact preservado (**3º consecutivo pós-P307**).
- Demais L0, ADRs, L1.

---

## 5. Estrutura proposta da ADR-0101

```
# ⚖️ ADR-0101: Excepção IEEE 754 em stdlib — rejeita NaN/Inf via guard_float

**Status**: EM VIGOR
**Data**: 2026-05-20
**Passo promotor**: P309 (diagnóstico transversal) + P310 (formalização)
**Categoria**: Arquitectural / Política numérica
**Cross-ref**: ADR-0033 (paridade observable; excepção documentada),
              ADR-0018 (DEBT-libm; ortogonal mas relacionado)

## Contexto
[divergência declarativa eval.md vs stdlib.md; 67 sítios P309;
 12 funções calc com `guard_float`; 9 divergências vs vanilla]

## Decisão
Política IEEE 754 cristalina é **categorial**:
- eval/layout/operators: IEEE 754 puro (paridade vanilla).
- stdlib funções matemáticas escalares: `guard_float` rejeita
  NaN+Inf no resultado (divergência consciente vanilla).
- conversões Float→Int (Cat D): saturating implícito Rust 1.45+
  (paralelo vanilla via libm).
- color constructors (Cat D): aceita range f32 sem validação
  (divergência tipada vanilla; aceita scope-out).

## Divergências catalogadas vs vanilla
[tabela 9 funções: sin/cos/tan/atan/atan2/sinh/cosh/tanh/asinh +
 sqrt/exp/pow/erf — todas onde cristalino rejeita Inf+NaN e
 vanilla deixa passar / rejeita só NaN no result]

## Racional
1. Erros explícitos > silent NaN/Inf no output PDF.
2. Reverter custaria ~5-8 testes refactor + ADR justification.
3. Refino P308 erf (A&S + short-circuits ±∞/±0) mantém-se útil.
4. Consistência interna stdlib (todas as 12 funções rejeitam).

## Não-decisões
- Cat D Float→Int Err não é parte da Opção A (P309 §10.6 adia).
- Color range check não é parte da Opção A (P309 §10.5 adia).
- DEBT-libm ortogonal (ADR-0018 vigente; migração futura mantém
  `guard_float`).

## Consequências
[positivas, negativas, neutras]

## Alternativas consideradas
[Opção B/C/D rejeitadas — racional curto]

## Referências
[P309 diagnóstico; P308 erf; ADR-0033 paridade; ADR-0018 DEBT-libm]
```

Magnitude estimada: ~200 linhas (paralelo ADR-0033 / ADR-0098).

---

## 6. Estrutura proposta drift L0

### 6.1 — `stdlib.md` — secção nova após §"Helpers Internos"

```markdown
## Política IEEE 754 — `guard_float` (ADR-0101)

`guard_float(f)` rejeita NaN e Inf no **resultado** de funções
matemáticas escalares (`calc.pow`, `calc.sqrt`, trig, hiperbólicas,
log, exp, root, norm, atan2). `calc.erf` rejeita NaN no input + 
faz short-circuit em ±∞/±0.

**Política transversal cristalina**:
- `eval`/layout/operators: **IEEE 754 puro** (paridade vanilla).
- `stdlib` funções matemáticas: **rejeita NaN+Inf** (divergência
  consciente vanilla; ADR-0101).

Esta divergência é **categórica**, não acidental. Vanilla `sin/cos/
tan/.../erf` retornam `f64` transparente; cristalino encerra em
`SourceResult<Value>` via `guard_float`. Racional completo em
ADR-0101.

Ver `00_nucleo/diagnosticos/diagnostico-ieee754-passo-309.md` para
catálogo completo de 67 sítios e comparação vanilla.
```

### 6.2 — `eval.md` — secção nova (ou estende §"Float")

```markdown
## Política IEEE 754 — propagação silenciosa (ADR-0101)

Operações binárias (`eval_binary_op`) e unárias (`eval_unary_op`)
sobre `Value::Float` propagam IEEE 754 silenciosamente:
- `5.0 / 0.5e-200 = Float(Inf)` sem erro.
- `0.0 / 0.0 = Err("cannot divide by zero")` (caso especial divisor
  zero literal).
- `0.0 * f64::INFINITY = Float(NaN)` sem erro.

**Não invoca `guard_float`** — esse helper é exclusivo de
`stdlib/calc.rs` per ADR-0101 (divergência categorial entre eval
permissivo e stdlib restritivo).

Paridade vanilla `foundations/ops.rs` total para este sítio.
```

---

## 7. Anotação cumulativa ADR-0033

Inserir após última anotação cumulativa existente (P273):

```markdown
## Anotação cumulativa P310 — Excepção IEEE 754 stdlib

**Data**: 2026-05-20.

P310 formaliza **excepção paridade observable em stdlib funções
matemáticas escalares** via ADR-0101 EM VIGOR. Cristalino rejeita
NaN+Inf no resultado de 12 funções `calc.*` onde vanilla deixa
passar (sin/cos/tan/.../erf). Catálogo completo em P309 diagnóstico.

Status `EM VIGOR` ADR-0033 preservado literal. Esta anotação
documenta **excepção categorial** stdlib sem revogar regra geral
paridade observable (eval/layout/output PDF preservam paridade
literal).

Sub-padrão "Excepção categorial documentada via ADR dedicada N=1
inaugural" — candidato observação cumulativa futura.

Cross-references:
- ADR-0101 — IEEE 754 restricao stdlib (criada P310).
- P309 — diagnóstico transversal IEEE 754 (catálogo 67 sítios).
- P308 — `calc.erf` divergência local que motivou auditoria.
```

Paralelo absoluto a anotações cumulativas P266/P268.1/.../P273
(ADR-0054, ADR-0083, etc.).

---

## 8. Granularidade — fixada

P310 é **passo único documental**. Justificação:

- Custo zero código.
- 3 artefactos documentais (ADR nova + drift L0 duplo + anotação ADR
  existente) — coesos por escopo (Opção A IEEE 754).
- Sem decisão arquitectural intercalada (decisão já tomada em P309).
- Precedente: P271 (meta-formalização sub-padrões ADR-0093+0094) —
  passo administrativo XS-S documental.

---

## 9. Protocolo de Nucleação — sequência prevista

| Fase | Acção | Quem |
|---|---|---|
| 1 | Plano | Humano + IA (este doc) |
| 2 | IA redige ADR-0101 + drift L0 + anotação ADR-0033 | IA |
| 3 | Humano grava artefactos + `crystalline-lint --fix-hashes .` | Humano |
| 4 | IA escreve testes | n/a — sem código L1 tocado |
| 5 | IA escreve implementação | n/a |
| 6 | Linhagem + `crystalline-lint` zero violations | IA valida |

**Trava arquitectural**: Fase 3 humano (grava + fix-hashes) precede
Fase 6 validação. Como não há código L1 tocado, Fase 4-5 são n/a.

---

## 10. Critérios de fecho

P310 está fechado quando:

- [ ] **Fase 2**: ADR-0101 publicada em `00_nucleo/adr/` com status
      `EM VIGOR`.
- [ ] **Fase 2**: `stdlib.md` actualizado com secção §"Política IEEE
      754" + cross-ref ADR-0101.
- [ ] **Fase 2**: `eval.md` actualizado com secção §"Política IEEE
      754 — propagação silenciosa" + cross-ref ADR-0101.
- [ ] **Fase 2**: ADR-0033 anotada cumulativamente com secção P310.
- [ ] **Fase 3**: Hashes propagados; `crystalline-lint .` zero
      violations.
- [ ] **Fase 6**: Validação final `crystalline-lint .` clean.
- [ ] Relatório P310 escrito após fecho.

Invariantes a preservar:

- [ ] `entities/content.rs` hash inalterado (**27º consecutivo**)
- [ ] `export/*` snapshots inalterados (**3º consecutivo pós-P307**)
- [ ] Cobertura calc 41/41 inalterada
- [ ] **0 ADRs meta novas** (**17ª consecutiva**) — ADR-0101 é
      arquitectural/política, não meta-metodológica
- [ ] Testes inalterados — 2 409 verdes preservados
- [ ] Zero código L1 tocado

---

## 11. Sub-padrões observados

### 11.1 — "Decisão pós-diagnóstico transversal" — N=1 inaugural

P309 → P310 sequência: diagnóstico amplo (67 sítios) → 4 opções →
decisão humana → materialização documental.

Paralelo histórico: P156B → P156G (Layout diagnóstico → decisão).
Pattern recorrente mas não formalizado; N=1 cumulativo neste sub-
padrão específico (decisão IEEE 754).

### 11.2 — "Drift L0 múltiplo em prompts distintos" — N=1 inaugural

P306/P308 fizeram drift L0 em **stdlib.md apenas** (1 prompt).
P310 inaugura drift L0 simultâneo em **2 prompts distintos**
(`stdlib.md` + `eval.md`) com cross-ref recíproco a uma ADR comum.

Candidato observação cumulativa futura. Limiar tentativo N=3 longe;
adiamento natural.

### 11.3 — "Anotação cumulativa ADR pré-existente via P{N}" — N=10+
cumulativo

ADR-0054 acumulou ~9 anotações P249-P273. ADR-0083/0091 idem.
P310 adiciona N+1 a ADR-0033 — pattern maduro per ADR-0093
Pattern 2.

---

## 12. Sem decisões adiadas a este passo

Per P309 §10 decisões adiadas explícitas, **nenhuma é parte de P310**:

1. ~~Adoptar Opção A/B/C/D~~ — **decidida em pré-P310** (escolha
   humana Opção A).
2. ~~Reverter `erf(NaN) → Err`~~ — não aplicável Opção A.
3. ~~Migração agregada libm~~ — ortogonal (ADR-0018 vigente).
4. ~~ADR-IEEE754-conformidade~~ — **materializa em P310 como ADR-0101**.
5. ~~Range check em `oklab/...`~~ — adiada para futuro passo de reforço
   pontual.
6. ~~Float→Int guard~~ — idem.
7. ~~Uniformização linguística mensagens~~ — ortogonal; adiada.

---

## 13. Próxima acção

**Aguardar arranque** (utilizador confirmou Opção A; nenhuma decisão
adicional pendente para este passo). IA pode prosseguir directamente
para Fase 2 (redacção dos 3 artefactos documentais).

Se houver dúvida sobre escolhas internas (e.g. número da ADR, posição
da anotação em ADR-0033, formulação exacta de prosa L0), o utilizador
deve sinalizá-las **antes** de Fase 2; caso contrário a IA decide
seguindo precedentes documentados (ADR-0101, anotação no final de
ADR-0033, prosa neutra factual).
