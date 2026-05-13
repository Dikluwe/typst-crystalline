# Relatório do passo P225 — Encerramento Fase 4 Layout candidata (documental; fecha série α "terminar Layout")

**Data**: 2026-05-13.
**Spec**: `00_nucleo/materialization/typst-passo-225.md`.
**Tipo**: passo documental puro (zero código tocado); anotações
cumulativas + recálculos documentais + blueprint marca cirúrgica.
**Magnitude planeada**: S (~30min-1h). **Magnitude real**: S
(~30min).
**Marco**: **fecho formal série α "terminar Layout"** — **segundo
encerramento de Fase Layout pós-M9c** (primeiro foi P221 Fase 3);
pattern emergente "encerramento Fase Layout pós-M9c" N=1 → **2
cumulativo formalizado** em §3.0terdecies.

---

## §1 O que foi feito

P225 fecha cirúrgicamente a série α "terminar Layout" cumulativa
materializada em P222+P223+P224 (Fase 4 Layout candidata 3/3
sub-passos). **0 ADRs transitam** (ADR-0061 já IMPLEMENTADO desde
P221 preservado; ADR-0066 mantém PROPOSTO per pattern emergente
N=1 "ADR PROPOSTO com materialização parcial graded"). **0 DEBTs
novos fechados em P225** (DEBT-37 §"Divergência" via P223
cumulativo; DEBT-34e via P224 cumulativo). **1 DEBT preservado
aberto per `P224.div-1`** (DEBT-34d refino algorítmico track
sizing distinto). Inventário 148 footnote ⁴⁶ consolidada;
blueprint §3.0terdecies marca + §2.1 Opção γ refresh **"89% (12
impl + 4 impl⁺ + 2 parcial)"** — divergência metodológica visual
vs real **fechada via materialização cumulativa**. **6
verificações C8 passam ✓**. Tests **2039 verdes preservados**; 0
violations; "Nothing to fix".

---

## §2 Auditoria pré-P225 (C1)

Verificação empírica antes de anotações:

| Critério | Esperado | Real |
|----------|----------|------|
| `cargo test --workspace` | 2039 verdes | **2039 verdes** ✓ (1750+242+24+2+21) |
| `crystalline-lint .` | 0 violations | **0 violations** ✓ |
| Content variants | 59 (GridHeader/Footer/Cell P224) | **59** ✓ |
| ADR-0061 status | IMPLEMENTADO (P221) | ✓ preservado |
| ADR-0066 status | PROPOSTO (pattern N=1) | ✓ preservado |
| ADR-0078 status | IMPLEMENTADO (P221) | ✓ preservado |
| DEBT-34e | ENCERRADO P224 | ✓ |
| DEBT-34d | EM ABERTO preservado | ✓ aberto (per `P224.div-1`) |

Estado factual P224 baseline confirmado integralmente; sem
divergência; sem `P225.div-1`.

---

## §3 ADR-0061 anotação final série α (C2)

Bloco `### P225 anotação — Encerramento série α "terminar Layout"
2026-05-13` adicionado após `### P224 anotação` em §"Aplicações
cumulativas":

- Trajectória completa Fase 4 (P222 + P223 + P224) listada.
- Cumulativo: 3 variants Content novos + 7 fields refino + 4
  stdlib novas + 2 refinadas + 1 helper promoção + 1 módulo L1
  novo + 2 DEBTs fechados + 1 preservado per `P224.div-1` + 52
  tests cumulativos.
- **8 patterns emergentes cumulativos consolidados** listados.
- Política "sem novas reservas" preservada (Fase 5 candidata
  identificada mas NÃO reservada).
- **Status ADR-0061 mantido IMPLEMENTADO** (Fase 4 candidata
  100% materializada per Opção α P221 §8; sem nova transição).
- Status ADR-0066 mantido PROPOSTO (pattern N=1 preservado).

---

## §4 DEBT.md actualização cumulativa (C3)

**DEBT-37 §"Divergência face ao vanilla"**: anotação histórica
consolidada documenta:
- Comentário original "quando float for adicionado, repor a
  restrição" reproduzido com prefixo `**[HISTÓRICO]**`.
- Fecho via P223 (Decisão 3 Opção α restauração) detalhado:
  campo `float: bool` adicionado a `Content::Place`; restrição
  vanilla `scope: Parent + float: true` restaurada em
  `native_place`; 1 test pre-existente adaptado intencionalmente.
- Pattern emergente "fecho de divergência documentada via
  refino" N=1 inaugurado P223 registado.

**DEBT-34d**: nota `P224.div-1` consolidada adicionada ao título
+ corpo:
- Estado pós-P224 explicitamente: **aberto preservado per
  `P224.div-1`**.
- Distinção factual material entre DEBT-34d (track sizing
  greediness — algoritmo Auto vs Fraction) e DEBT-34e
  (placement colspan/rowspan via `grid_placement.rs` P224.C).
- Pattern emergente "divergência factual material registada
  como `Pxxx.div-N`" N=1 → 2 cumulativo formalizado em P225
  (P215.div-1 + P224.div-1).
- Refino candidato Fase 5 Layout NÃO-reservada per política
  P158.

**Saldo DEBTs cumulativo Layout Fase 3+4**:
- Pré-P221: 14 abertos.
- Pós-P221: 13 abertos (DEBT-56 fechou).
- Pós-P224: 12 abertos (DEBT-34e fechou).
- **Pós-P225: 12 abertos** (preservado; P225 documental puro
  sem fechos novos).

---

## §5 Inventário 148 consolidação (C4)

**§A.5 Layout** — 3 entradas com footnote ⁴⁶ anotada:
- `place(...)` (linha 137): `implementado⁺ ⁵ ⁴⁴ ⁴⁶`.
- `grid(...)` (linha 141): `implementado⁺ ⁵ ⁴⁵ ⁴⁶`.
- `measure(body)` (linha 151): `implementado⁺ ⁴³ ⁴⁶`.

Tabela A.5 footnotes: `⁴⁰ ⁴¹ ⁴² ⁴³ ⁴⁴ ⁴⁵` → `⁴⁰ ⁴¹ ⁴² ⁴³ ⁴⁴ ⁴⁵
⁴⁶`. Distribuição preservada **`12/4/2/0/0 = 18`** (P225
documental).

Total user-facing footnotes: `⁴⁰ ⁴¹ ⁴² ⁴³ ⁴⁴ ⁴⁵` → `⁴⁰ ⁴¹ ⁴² ⁴³
⁴⁴ ⁴⁵ ⁴⁶`. Distribuição preservada **`68/27/24/20/2 = 141`**.
Cobertura: `(68+27)/141 ≈ **67%**` preservada.

**Cobertura Layout per metodologia**: `(12+4)/18 = **89%**` real
(preservado P224; paridade visual histórica Opção γ §2.1
**refrescada** pós-P225 para "89% (12 impl + 4 impl⁺ + 2 parcial)"
— divergência metodológica visual vs real **fechada via
materialização cumulativa**).

**Footnote ⁴⁶ adicionada** (~95 linhas) documentando:
- Trajectória completa Fase 4 (P222+P223+P224).
- Cumulativo material (3 variants + 7 fields + 4 stdlib + 2
  refinadas + 1 helper + 1 módulo + 2 DEBTs fechados + 1
  preservado per `P224.div-1` + 52 tests).
- 3 reclassificações §A.5 parcial → impl⁺.
- Distribuição §A.5 final `12/4/2/0/0 = 18` (zero ausentes
  preservado).
- Cobertura Layout: 78% Fase 3 → 89% pós-Fase 4.
- 9 patterns emergentes cumulativos (incluindo "encerramento
  Fase Layout pós-M9c" N=1 → 2 formalizado em P225).
- Política "sem novas reservas" preservada.

**Decisão Opção γ §"Footnotes"**: **manter ⁴³ + ⁴⁴ + ⁴⁵ + ⁴⁶**
em conjunto (paridade P221 que manteve ⁴⁰ + ⁴¹ + ⁴² histórico
preservado per P204H+ "histórico textual preservado"). Reduz
inflação acumulativa mas preserva rastreabilidade P222-P224
incremental.

---

## §6 Blueprint §2.1 Opção γ refresh + §3.0terdecies (C5)

`00_nucleo/diagnosticos/blueprint-projecto.md` editado em 2
sítios:

**§2.1 linha Layout** (Opção γ refresh):

Antes:
```
| Layout ⁽ᴾ²¹⁴⁾⁽ᴾ²²¹⁾ | 78% (12 impl + 5 parcial) | quase total — Fase 3 fechada estructuralmente P221 ... |
```

Depois:
```
| Layout ⁽ᴾ²¹⁴⁾⁽ᴾ²²¹⁾⁽ᴾ²²⁵⁾ | **89% (12 impl + 4 impl⁺ + 2 parcial)** | quase total — **Fase 4 candidata fechada formalmente P225** (série α "terminar Layout" cumprida 3/3 sub-passos cumulativos P222+P223+P224; DEBT-37 §"Divergência" + DEBT-34e fechadas; DEBT-34d preservado per `P224.div-1`; ADR-0061 IMPLEMENTADO desde P221 preservado); refinos cosméticos stroke/fill + Auto track sizing DEBT-34d + per-cell GridCell atributos + consumer geometric integration + flow real Place float Fase 5 candidata NÃO-reservada |
```

Opção γ refresh: divergência metodológica visual vs real **fechada
via materialização cumulativa** — 89% real per metodologia coincide
com 89% per paridade visual histórica refrescada. Distribuição
"12 impl + 4 impl⁺ + 2 parcial" reflecte estado real pós-Fase 4.

**§3.0terdecies Marca de actualização — [P225] Encerramento Fase
4 Layout candidata (série α "terminar Layout" fechada formalmente)**
adicionada após §3.0duodecies (P221) e antes de §3.1:

- Distinção qualitativa face a §3.0duodecies P221 documentada.
- Trajectória completa Fase 4 (P222+P223+P224) listada.
- Mudanças factuais cumulativas detalhadas (3 variants + 7 fields
  + 4 stdlib + 2 refinadas + 1 helper + 1 módulo + 2 DEBTs +
  1 preservado + 52 tests + zero ausentes preservado).
- **9 patterns emergentes formalizados** (incluindo "encerramento
  Fase Layout pós-M9c" N=1 → 2 formalizado).
- Política "sem novas reservas" preservada.
- Estado pós-P225 + trajectória aberta documentados.
- **13 sub-passos cumulativos pós-M9c P213-P225** consolidados
  como referência arquitectural.

---

## §7 Resultados verificação (C6+C7+C8)

| Critério | Esperado | Real |
|----------|----------|------|
| `cargo test --workspace` | 2039 preservado | **2039 verdes** ✓ (zero código tocado) |
| `crystalline-lint .` | 0 violations | **0 violations** ✓ |
| `crystalline-lint --fix-hashes` | "Nothing to fix" | **"Nothing to fix"** ✓ (L0 prompts não tocados) |
| ADR-0061 status IMPLEMENTADO | preservado | ✓ |
| DEBT-37 §"Divergência" anotação fechada | sim | ✓ (P223 + P225 consolidação) |
| DEBT-34e ENCERRADO | preservado | ✓ P224 |
| DEBT-34d EM ABERTO preservado P224.div-1 | sim | ✓ |
| Inventário 148 footnote ⁴⁶ | adicionada | ✓ (~95 linhas) |
| Blueprint §3.0terdecies + §2.1 P225 | adicionados | ✓ marca + Opção γ refresh |

**Auditoria cumulativa final** (6 verificações C8):
1. ✓ ADR-0061 status IMPLEMENTADO preservado.
2. ✓ DEBT-37 divergência fechada (anotação P223 + P225
   consolidação).
3. ✓ DEBT-34e ENCERRADO (P224 anotado).
4. ✓ DEBT-34d EM ABERTO preservado per `P224.div-1`.
5. ✓ Inventário 148 footnote ⁴⁶ P225 consolidada.
6. ✓ Blueprint §3.0terdecies + §2.1 marcador P225.

C9 README ADRs **opcional** — skip per política minimalista P225
(spec C9 marcou como opcional; entry administrativo não-crítico).

---

## §8 Próximo trabalho

P225 fecha série α "terminar Layout" formalmente. **Layout em
estado terminal estructural reconhecido oficialmente**. Decisão
humana sobre próxima sessão:

| Caminho | Trabalho | Magnitude | Prioridade subjectiva |
|---------|----------|-----------|------------------------|
| **Caminho 1** | **ADR meta documental "L0 minimal para refactors" N=7** (Caminho 4 candidato P221 §8 reforçado; passo administrativo XS paridade P160A/P213/P214 — N=7 patamar empírico muito sólido) | XS (~30min) | alta (consolidação metodológica; pattern fortemente consolidado) |
| **Caminho 2** | ADR meta documental "Field armazenado semantic adiada" N=5 (paridade Caminho 1 mas para pattern semântico-arquitectural) | XS (~30min) | média |
| **Caminho 3** | Fase 5 Layout candidata sub-passo 1 (stroke/fill cosméticos OU Auto track sizing DEBT-34d OU per-cell GridCell atributos OU consumer geometric integration P224.C OU flow real Place float OU Opção A multi-region) | varia M-L | baixa-média (Layout 89% já alto; Fase 5 introduz refinos cosméticos ou algorítmicos profundos NÃO-reservados per P158) |
| **Caminho 4** | Pivot outro módulo (Visualize 54%; Text features 52%; Markup 78%; Model 50%; Foundations 67%; let/set/show 62%) | varia | média (Visualize + Text + Model são candidatos óbvios; mais espaço para ganho) |
| **Caminho 5** | Adiar Layout completo; outro objectivo arquitectural | varia | baixa |

**Recomendação subjectiva**: **Caminho 1 (ADR meta L0 minimal
N=7)** OU **Caminho 4 (pivot Visualize/Text/Model)** dependendo
de prioridade:
- Se humano quer **consolidar metodologia** pós-fecho Layout
  ambicioso: Caminho 1 (passo administrativo limpo; N=7
  patamar sólido).
- Se humano quer **continuar materialização**: Caminho 4
  (Visualize 54% ou Text 52% são candidatos com mais espaço
  ganho que Layout 89%).

**Decisão humana fica em aberto literal**. P225 não compromete
trabalho subsequente per política "sem novas reservas" P158.

**Estado pós-P225**:
- **Fase 3 Layout fechada** (P221) ✓.
- **Fase 4 Layout candidata fechada formalmente** (P225) ✓.
- **Série α "terminar Layout" 3/3 sub-passos cumpridos** (Opção
  α P221 §8 100% materializada).
- Cobertura Layout per metodologia: **89% real** (paridade
  visual Opção γ refrescada).
- Cobertura user-facing total: **67%** (+2pp cumulativo
  pós-M9c).
- Tests workspace: 1939 (pre-M9c) → **2039 verdes** (+100
  cumulativo pós-M9c P213-P225).
- ADRs: distribuição preservada P221 (PROPOSTO 11;
  IMPLEMENTADO 21).
- DEBTs abertos: 14 (pre-M9c) → **12** (-2 cumulativo).
- 18 aplicações cumulativas anti-inflação pós-P205D.
- **Pattern emergente "encerramento Fase Layout pós-M9c"
  N=1 → 2 formalizado em §3.0terdecies P225** (P221 Fase 3;
  P225 Fase 4). Reusável Fase 5 candidata futura ou Model
  Fase 3 candidata.
- **Pattern "L0 minimal para refactors" N=6 → 7 consolidado**
  — patamar empírico **muito sólido**; Caminho 1 candidato.
- **Pattern "Field armazenado semantic adiada" N=4 → 5
  consolidado** — Caminho 2 candidato.
- **Pattern "divergência factual material `Pxxx.div-N`"
  N=1 → 2 cumulativo formalizado P225** (P215.div-1;
  P224.div-1). Pattern de honestidade arquitectural.
- **13 sub-passos cumulativos pós-M9c P213-P225** consolidados
  (P213+P214 recálculos; P215 diagnóstico; P216A+B refactor;
  P217-P220 série Fase 3 sub-fase b; P221 fecho Fase 3;
  P222-P224 série α Fase 4; **P225 fecho Fase 4**).
- **Marco M9c preservado** como referência arquitectural
  estável + Layout em estado terminal estructural reconhecido
  oficialmente.
- **Fase 5 Layout candidata NÃO-reservada** per política P158
  (refinos cosméticos stroke/fill; per-cell GridCell atributos;
  Auto track sizing DEBT-34d; consumer geometric integration
  P224.C; flow real Place float; Opção A multi-region para
  columns/colbreak).
