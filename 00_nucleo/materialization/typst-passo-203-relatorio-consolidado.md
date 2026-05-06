# Relatório Consolidado P203 — Encerramento da série

**Data**: 2026-05-05.
**Magnitude consolidada**: S agregada (3 sub-passos
documentais + 1 test).
**Estado**: P203 série completa — **série fechada**.
**Escopo**: P203A + P203B + P203C.
**Sub-passo de fecho**: P203C (este relatório).
**ADRs criadas**: 0 (nenhuma necessária).
**Lacunas formalmente fechadas**: #1 + #1b (formalização
estrutural pré-existente P190H/P191C).

---

## §1 Resumo executivo

P203 série fecha após 3 sub-passos documentais (A
diagnóstico-primeiro + B implementação Caminho C + C
encerramento).

A trajectória da série exibiu **duas instâncias
consecutivas de diagnóstico-primeiro a detectar e corrigir
premissas erradas em specs herdadas** — primeiro corrigindo
nomenclatura de lacunas (P203A `div-1`); depois corrigindo
afirmações arquitecturais pré-P190H sobre o estado actual
(P203B `div-1`).

Resultado empírico:
- **Lacunas #1 e #1b formalmente fechadas** (estruturalmente
  fechadas em P190H + P191C; formalizadas em P203B com
  test consolidado).
- **Lacuna #2 confirmada como reservada / vazia**.
- **Position concrete** confirmada como concern ortogonal
  coberto por **ADR-0066 + M8** — não é lacuna catalogada.
- **Pipeline pós-P190H/P191C alinhada**: walk arm Figure
  puro; população em `populate_intr_from_tag_start`
  (não em `from_tags`); default `unwrap_or("image")`
  + gate `is_counted` consistentes.
- **Snapshot 2026-05-05 §7, §13, §11, §2** corrigidos
  para reflectir nomenclatura empírica e tests
  actualizados.
- **Auditoria delta P201 §2** anotada com correcção
  retroactiva.
- **Tests workspace**: 1823 → 1824 (+1 P203B
  formalização).
- **Crystalline-lint**: 0 violations (mantido).

---

## §2 Sub-passos materializados

| Passo | Magnitude planeada | Magnitude real | Δ tests | Δ código produção | Outputs |
|-------|--------------------|----------------|---------|--------------------|---------|
| **P203A** | S–M | S–M | 0 | 0 | 3 ficheiros (auditoria + diagnóstico + relatório) |
| **P203B** | M | **S** | +1 | 0 | 2 ficheiros (inventário + relatório) + 1 test + 4 edições administrativas |
| **P203C** | S documental | S | 0 | 0 | 1 ficheiro (este consolidado) + 1 edição administrativa |
| **Total** | S–M agregado | S | **+1** | **0** | 6 documentos + 1 test + 5 edições administrativas |

### Outputs por sub-passo (referência aos ficheiros existentes)

**P203A** — `00_nucleo/diagnosticos/`:
- `typst-passo-203A-auditoria-position.md` (~19 KB).
- `typst-passo-203A-diagnostico.md` (~14 KB).

**P203A** — `00_nucleo/materialization/`:
- `typst-passo-203A-relatorio.md` (~10 KB).

**P203B** — `00_nucleo/diagnosticos/`:
- `typst-passo-203B-inventario.md` (~11 KB).

**P203B** — `00_nucleo/materialization/`:
- `typst-passo-203B-relatorio.md` (~12 KB).

**P203B** — código:
- `01_core/src/rules/introspect.rs` — test
  `p203b_lacuna_1_e_1b_fecho_formal_4_casos` adicionado
  (~120 LOC tests).

**P203B** — edições administrativas:
- `00_nucleo/diagnosticos/snapshot-2026-05-05.md` §7
  (tabela lacunas reescrita).
- `00_nucleo/diagnosticos/snapshot-2026-05-05.md` §13
  (resumo nova sessão reescrito).
- `00_nucleo/diagnosticos/typst-passo-201-auditoria-delta.md`
  §2 (anotação retroactiva).
- `00_nucleo/diagnosticos/historiograma-passos.md`
  (verificação cirúrgica — sem matches; sem edições).

**P203C** — `00_nucleo/materialization/`:
- `typst-passo-203-relatorio-consolidado.md` (este
  ficheiro).

**P203C** — edição administrativa:
- `00_nucleo/diagnosticos/snapshot-2026-05-05.md` §2 e
  §11 (tests 1823 → 1824 com nota P203B).

---

## §3 Trajectória da série (sequência de decisões e descobertas)

### 3.1 Premissa inicial herdada

P201 auditoria delta §2 (escrita em P201) atribuiu
"Position", "Position-related" e "Counter at locations"
às lacunas #1, #1b, #2. P202 reificou no snapshot
2026-05-05 §7. Spec P203A herdou.

**Origem do erro**: P201 não fez cross-check com
`m1-lacunas-captura.md` (canónica) ou P200 consolidado
§7. Erro propagou.

### 3.2 Detecção em P203A — `P203A.div-1`

P203A executou auditoria empírica A1-A10. **A8 detectou
divergência crítica**:

- Definição canónica P200 consolidado §7 + m1-lacunas-captura.md:
  - #1 = Figure kind=None ↔ Introspector
  - #1b = from_tags arm Figure sem gate `is_counted`
  - #2 = reservada / vazia
- Definição em P201 auditoria delta §2 / P202 snapshot §7:
  - #1 = Position
  - #1b = Position-related
  - #2 = Counter at locations

**As duas atribuições são incompatíveis**. Position é um
concern real (stub `position_of() -> Option<()>`) mas
**não é catalogada como lacuna**. ADR-0066 cobre o
adiamento até M8.

### 3.3 P203A diagnóstico — 4 opções não-vinculativas

Per spec P203A §6 ("ramificar"), P203A diagnóstico
apresentou 4 opções para humano decidir P203B:

- **α** — pivot para lacuna #1 real (Figure).
- **β** — pivot para lacuna #1b real (from_tags Figure
  gate).
- **γ** — aceitar redundância; avançar para M8
  (recomendada — Position é redundante com M8).
- **δ** — materializar Position mesmo assim
  (L cross-modular; sem pressure empírica).

**Humano escolheu α** (pivot para Figure kind=None).

### 3.4 P203B spec — premissa pré-P190H

Spec P203B §1 declarou:

> Walk arm usa `kind.as_deref().unwrap_or("image")` para
> contador (default fallback). from_tags arm para Figure
> ou usa o mesmo default ou omite o gate `is_counted`,
> criando divergência.

### 3.5 Detecção em P203B — `P203B.div-1`

P203B C1 inventário empírico revelou:

- **Walk arm Figure está PURO** desde P190H (M6 fechou).
  Apenas desce em body + caption. **Não usa `kind` nem
  `numbering`**. **Não aplica `unwrap_or("image")`**.
- **Não há `from_tags::Figure` arm**. População é durante
  walk via `populate_intr_from_tag_start` (helper do
  P191B/C — ADR-0071).
- `populate_intr_from_tag_start` aplica gate `is_counted`
  + default `unwrap_or("image")` consistentemente.
- `extract_payload` preserva `kind` literal + deriva
  `is_counted: numbering.is_some() && caption.is_some()`.

**Conclusão**: arquitectura pós-P190H/P191C **já está
alinhada**. Não há desalinhamento para corrigir.

### 3.6 P203B C2 re-fixado — Caminho C

Per spec §6 ("Em caso de divergência empírica relevante,
re-executar C1 com valores actuais; re-fixar C2 com novos
dados"):

- Caminhos A/B do spec rejeitados (premissa errada).
- **Caminho C — Confirmação (sem alteração de código)**
  introduzido. Trabalho concreto: 1 test consolidado +
  3 correcções administrativas.

### 3.7 P203B execução — Caminho C

- Test `p203b_lacuna_1_e_1b_fecho_formal_4_casos`
  adicionado (4 casos × 3 grupos de asserções).
- Snapshot §7 e §13 reescritos (lacunas #1/#1b
  fechadas; Position deslocada para concerns ortogonais).
- Auditoria delta P201 §2 anotada com correcção
  retroactiva.
- Historiograma cirurgicamente verificado (sem matches
  para Position-as-lacuna; sem edições necessárias).

### 3.8 P203C — encerramento

Este passo. Agrega P203A + P203B + actualiza snapshot
§2 e §11 (tests 1823 → 1824).

---

## §4 Achados consolidados

### 4.1 Lacunas #1 e #1b — fechadas

Lacuna | Tópico (definição empírica) | Fecho estrutural | Formalização
:--:|---|---|---
**#1** | `figure.kind` literal em tags vs colapsado em counter (default `"image"`) | P190H + P191C | P203B (test `p203b_lacuna_1_e_1b_fecho_formal_4_casos`)
**#1b** | gate `is_counted` no caminho de população do counter Figure | P191C (`populate_intr_from_tag_start`) | P203B (mesmo test, asserção (b))

### 4.2 Lacuna #2 — confirmada reservada

P200 consolidado §7 declarou "reservada"; P203 confirmou
empíricamente — não há trabalho remanescente sob esta
numeração.

### 4.3 Position — concern ortogonal coberto por M8

- Stub `position_of() -> Option<()>` real.
- 0 consumers em produção (apenas 2 tests stub).
- 0 corpus pressure (lab/parity sem casos).
- ADR-0066 (Introspection runtime adiada — ACEITE em
  P192B com nota "intermediário até M8") cobre o
  adiamento estratégico.
- Vanilla pipeline para Position é **post-layout** (per
  P203A A7); cristalino single-pass via `runtime` é
  alternativa viável, **mas seria redundante com M8
  paridade vanilla**.

**Conclusão**: Position concrete fica para M8.

### 4.4 Pipeline pós-P190H/P191C — confirmada alinhada

- Walk arm Figure puro (introspect.rs:866-890).
- `extract_payload` Figure preserva kind literal +
  deriva is_counted (extract_payload.rs:27-34).
- `populate_intr_from_tag_start` Figure aplica
  default + gate (introspect.rs:527-559).
- `compute_labelled` Figure arm aplica mesmo default
  para reads (introspect.rs:399-419).
- **Sem `from_tags::Figure` arm** — população durante
  walk per ADR-0071.

### 4.5 Snapshot 2026-05-05 reconciliado

- §2 (cobertura empírica): tests 1823 → **1824** (P203C).
- §7 (lacunas): nomenclatura empírica correcta;
  zero residuais formalmente catalogadas (P203B).
- §11 (métricas): tests 1824 (P203C).
- §13 (resumo nova sessão): bloco lacunas reescrito;
  bloco "concerns ortogonais" adicionado (P203B).

### 4.6 Auditoria delta P201 §2 — anotada

Bloco de correcção retroactiva no início da secção
(P203B). Conteúdo histórico preservado abaixo
(não-canónico).

---

## §5 Métricas agregadas

### 5.1 Pré-P203 vs Pós-P203

| Métrica | Pré-P203 | Pós-P203 | Δ |
|---------|----------|----------|---|
| Tests workspace | 1823 | **1824** | +1 |
| Crystalline-lint violations | 0 | **0** | = |
| LOC produção | (sem alteração) | (sem alteração) | **0** |
| LOC tests | baseline | +~120 | +120 |
| LOC documental | baseline | +~100 | +100 |
| ADRs total | 70 | **70** | = |
| Lacunas residuais formalmente catalogadas | 2 (com nomenclatura errada) | **0** | -2 |
| Marcos fechados | M1-M7, M9, F1 (9) | M1-M7, M9, F1 (9) | = |

### 5.2 Δ por sub-passo

| Sub-passo | Δ tests | Δ LOC produção | Δ LOC tests | Δ LOC documental |
|-----------|---------|----------------|-------------|------------------|
| P203A | 0 | 0 | 0 | +~43 KB (3 ficheiros) |
| P203B | +1 | 0 | +~120 | +~23 KB (2 ficheiros + edições) |
| P203C | 0 | 0 | 0 | +~10 KB (este consolidado + edição §2/§11) |

---

## §6 Divergências da série

| Divergência | Detectada em | Causa | Resolução |
|---|---|---|---|
| **`P203A.div-1`** | A8 P203A auditoria | Lacunas #1/#1b/#2 não são Position; erro propagado de P201 → P202 → spec P203A | Pivot α (decisão humana — pivot para lacuna #1 real Figure) |
| **`P203B.div-1`** | C1 P203B inventário | Premissa do spec sobre walk arm e from_tags arm reflectia arquitectura pré-P190H; pós-P190H/P191C arquitectura já alinhada | Caminho C — Confirmação sem alteração de código (re-fixação per spec §6) |

Ambas as divergências foram registadas, justificadas
e resolvidas dentro da própria série, sem necessidade de
P204 administrativo correctivo separado.

---

## §7 Padrão demonstrado

A série P203 é a **1ª aplicação completa documentada do
padrão "diagnóstico-primeiro a detectar premissa errada
em spec herdada"** em duas instâncias consecutivas:

### Instância 1 — P203A `div-1`

Spec P203A herdou nomenclatura "Position" das lacunas
#1/#1b/#2 directamente de P201 auditoria delta + P202
snapshot. Auditoria empírica A8 cross-checkou contra
fontes canónicas (`m1-lacunas-captura.md` + P200
consolidado §7) e detectou que a atribuição era
empíricamente errada.

### Instância 2 — P203B `div-1`

Spec P203B foi escrita assumindo arquitectura pré-P190H
(walk arm Figure usa `unwrap_or("image")`; `from_tags`
arm Figure existe). Inventário empírico C1 detectou
que pós-P190H/P191C a arquitectura é diferente:
walk arm puro; população via `populate_intr_from_tag_start`;
default + gate consistentes.

### Pattern formalizado

**Mesmo sub-passos `*B+` começam com inventário empírico**
— não escapam à regra de empírico-precede-afirmação por
terem prefixo de "implementação".

A spec P203B §10 foi explícita sobre isto:
> Pattern: cada passo de feature começa com inventário
> empírico do estado actual, não com afirmações herdadas
> de specs anteriores. Sub-passo `*B` não escapa a esta
> regra.

P203B C1 cumpriu literalmente. Sem a disciplina, ambos
os erros teriam propagado para código (Caminho A/B
implementaria mudanças desnecessárias num sistema já
alinhado).

---

## §8 Estado pós-série face ao snapshot 2026-05-05

### 8.1 Comparação directa

| Secção | Estado pré-P203 | Estado pós-P203 |
|--------|-----------------|-----------------|
| §2 (cobertura) | 1823 tests | **1824 tests** (P203C) |
| §7 (lacunas) | nomenclatura "Position" errada | **nomenclatura empírica correcta** (P203B); 0 residuais |
| §8 (M8 escopo) | inalterado | inalterado (Position remanesce coberta) |
| §11 (métricas) | 1823 tests | **1824 tests** (P203C) |
| §13 (resumo) | "Lacunas #1/#1b residuais" | **"0 lacunas; concerns ortogonais incluem Position"** (P203B) |

### 8.2 Marcos arquitectónicos

Inalterados:
- M1 (P163) ✅
- M2 (P164) ✅
- M3 (P165) ✅
- M3 location-aware (P185E) ✅
- M4 (P166) ✅
- M4-residual (P188B) ✅
- M5 incremental (P189B) ✅
- M5 universal (P200B) ✅
- M6 (P190I) ✅
- M7 estruturalmente (P192B) ✅
- M9 11/11 (P182F) ✅
- F1 (P190I) ✅
- F3 parcial (P190I)
- M8 (comemo) — pendente

### 8.3 Lacunas

- #1, #1b — formalmente fechadas (P203B).
- #2 — reservada / vazia.
- #3-#7 — fechadas no ciclo P156B-P200.

**Zero lacunas residuais formalmente catalogadas
pós-P203**.

### 8.4 ADRs do ciclo M5/M6/M7

Inalteradas:
- 6 ACEITES estritas (0066, 0068, 0069, 0070, 0071, 0072).
- 2 EM VIGOR (0064, 0065).
- 3 PROPOSTAS (0061, 0062, 0067).
- Slot 0063 vazio.

---

## §9 Convenções consolidadas pela série P203

### 9.1 Mesmo sub-passos `*B+` começam com inventário empírico

Formalizado na convenção a registar no historiograma
quando aplicável. Spec P203B §10 explicitou; P203B C1
cumpriu literalmente; P203B `div-1` foi detectada por
isso.

### 9.2 Caminhos de spec podem ser re-fixados

Per cláusula explícita do spec ("em caso de divergência
empírica relevante, re-executar C1, re-fixar C2"). Não
é falha do passo — é mecanismo previsto.

### 9.3 Localização canónica dos documentos

- `00_nucleo/diagnosticos/` — diagnósticos, snapshots,
  auditorias delta, inventários, historiograma.
- `00_nucleo/materialization/` — specs, relatórios,
  consolidados.

Specs futuras devem usar paths reais (P202 spec
escreveu paths não-existentes).

### 9.4 Correcção retroactiva preserva histórico

Documentos não são reescritos retroactivamente. Anotação
de correcção colocada como bloco no início da secção
afectada, preservando conteúdo histórico abaixo como
não-canónico.

### 9.5 Caminho C (Confirmação) emergiu como categoria

Para casos onde inventário revela alinhamento já
existente, a opção válida é **formalizar o fecho via
test + correcção administrativa**, sem alteração de
código produção. Caminho C completa Caminhos A/B
existentes.

---

## §10 Não-objectivos respeitados

- [x] P203 não materializou Position concrete.
- [x] P203 não decidiu M8.
- [x] P203 não promoveu ADR-0067.
- [x] P203 não criou ADRs novas.
- [x] P203 não reescreveu historiograma inteiro
  (verificação cirúrgica revelou 0 matches).
- [x] P203 não reescreveu auditoria delta P201 — anotação
  retroactiva apenas.
- [x] P203 não modificou relatórios de passos anteriores
  retroactivamente.

---

## §11 Sugestão para próximo passo (não-vinculativa)

P203 série fechada. Pré-condição arquitectónica
**totalmente cumprida** para M8: M5+M6+M7 estruturalmente
fechados; zero lacunas residuais formalmente catalogadas;
baseline reconciliado.

### 11.1 P204A — diagnóstico de M8 (caminho default snapshot §13)

**Recomendado**. Pré-condição totalmente cumprida.
Trabalho concreto esperado:
1. ADR dedicada (ADR-0073 ou similar).
2. `#[comemo::track]` em trait `Introspector` (20 métodos).
3. Queries location-aware re-emitidas com tracking
   granular.
4. Re-walks parciais via invalidação cross-iteration.
5. Validação saída cristalino == vanilla (snapshot tests
   em corpus de paridade).
6. Performance comparável.

Magnitude esperada: **L cross-modular** (similar a M6).

### 11.2 P204A — promoção formal de ADR-0067

Trabalho pré-M8 condicional a haver propriedade alvo
concreta (ex: `numbering_active`) materializada com
pattern attribute-grammar. Magnitude provavelmente M.
Não é pré-requisito de M8.

### 11.3 P204A — F3 completo

Refactor 21 fields ortogonais Layouter. Ortogonal a M8
(pode ser feito antes ou depois).

### 11.4 Pausa estratégica

Validar saída cristalino vs vanilla em corpus actual
(sem comemo) antes de M8. Pode revelar gaps específicos
que M8 deve cobrir.

**P203 reporta. Não decide.**

---

## §12 Linhagem

- **Trabalho útil cumulativo**: P203 + correcção
  retroactiva fecharam administrativamente as últimas
  duas lacunas formalmente catalogadas (#1, #1b).
- **Pattern emergente**: "Caminho C — Confirmação sem
  alteração de código" — registada como variante
  válida de P203B.
- **Pattern formalizado**: "mesmo `*B+` começam com
  inventário empírico" — explicitado em spec P203B §10
  e cumprido em P203B C1.
- **Pre-condição arquitectónica para M8**: cumprida
  totalmente após P203 — M5+M6+M7+M9 fechados;
  baseline empírico reconciliado.

---

## §13 Achado arquitectural significativo

**P203 confirma empíricamente que a arquitectura
pós-P190H/P191C já implementa correctamente a separação
de responsabilidades para Figure introspection**:

- Tag preserva kind literal (`Option<String>`); nenhum
  default aplicado em momento de tag emission.
- Counter store usa default canónico `"image"` quando
  kind=None, aplicado consistentemente em populate +
  reads.
- Gate `is_counted` (`numbering.is_some() && caption.is_some()`)
  determina se figura conta para numeração;
  aplicado uma vez em `extract_payload` + lido no
  populate.

**Não há desalinhamento empírico entre walk e populate**
porque walk arm Figure é puro (P190H) e a população é
durante walk via `populate_intr_from_tag_start`
(P191B/C ADR-0071).

A lacuna #1 catalogada (figure.kind literal em tags vs
colapsado em state) **resolveu-se pela eliminação do
state em P190I** — o branch "state" deixou de existir;
o branch "tag" preserva literal; consumers usam default
canónico de forma consistente.

P203 documenta este achado sem alterar a arquitectura.
Próximas decisões arquitecturais (M8) podem assumir
este alinhamento como pré-condição cumprida.

---

**Fim do consolidado P203. Série P203 fechada.**
