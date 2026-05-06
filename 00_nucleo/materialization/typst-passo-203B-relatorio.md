# Relatório do passo P203B

**Data de execução**: 2026-05-05.
**Executor**: Claude Code (Opus 4.7).
**Spec**: `00_nucleo/materialization/typst-passo-203B.md`.
**Natureza**: pivot pós-P203A para lacuna #1 real (Figure
kind=None ↔ Introspector); correcção administrativa
embutida.
**Magnitude planeada**: M (S–M para feature + S
correcção).
**Magnitude real**: **S** (caminho re-fixado para
Caminho C — Confirmação sem alteração de código produção;
1 test adicionado + 3 ficheiros administrativos editados).

---

## §1 O que foi feito

### Output 1 — Inventário interno

Localização:
`00_nucleo/diagnosticos/typst-passo-203B-inventario.md`.

Conteúdo:
- §1 C1 inventário empírico (3 arms canónicos analisados).
- §2 Achado central — divergência face à premissa do spec.
- §3 C2 caminho fixado (Caminho C — Confirmação).
- §4 Plano de implementação literal.
- §5 Cláusulas C3-C9 instanciadas.
- §6 Referências.

Tamanho: ~10 KB.

### Output 2 — Test E2E (1 test consolidado)

Localização: `01_core/src/rules/introspect.rs` módulo
`tests`, função
`p203b_lacuna_1_e_1b_fecho_formal_4_casos`.

Cobre os 4 casos canónicos (per spec C3):
1. `kind=None`, sem caption.
2. `kind=None`, com caption.
3. `kind=Some("table")`, com caption.
4. `kind=Some("table")`, sem caption.

3 grupos de asserções:
- (a) `extract_payload` preserva `kind` literal +
  deriva `is_counted` correctamente.
- (b) `populate_intr_from_tag_start` aplica gate
  `is_counted` + default `unwrap_or("image")`.
- (c) Tags emitidas pelo walk preservam `kind`
  literalmente.

Resultado: ✅ test passa. Tests workspace: **1823 →
1824**. Linter: **0 violations**.

### Output 3 — Correcções administrativas

#### Edição 3a — `snapshot-2026-05-05.md` §7

Tabela de lacunas reescrita:
- #1 redefinida como "Figure kind=None ↔ Introspector
  (default counter)" — fechada estruturalmente
  P190H/P191C; **formalizada P203B**.
- #1b redefinida como "gate `is_counted` no caminho de
  população do counter Figure" — fechada estruturalmente
  P191C; **formalizada P203B**.
- #2 marcado como reservada / vazia (não "Counter at
  locations").
- Position deslocada para secção "concerns ortogonais
  não-catalogados".
- Bloco de cabeçalho documenta correcção P203B.

#### Edição 3b — `snapshot-2026-05-05.md` §13

Bloco "Lacunas residuais" reescrito:
- #1 e #1b — fechadas estruturalmente; formalizadas
  P203B.
- #2 — reservada / vazia.
- #3-#7 — fechadas no ciclo P156B-P200.

Bloco "Concerns ortogonais" adicionado:
- Position concrete coberta por ADR-0066 + M8.

Métrica de tests recalibrada: 1823 → **1824**.

#### Edição 3c — `typst-passo-201-auditoria-delta.md` §2

Bloco de correcção retroactiva adicionado no início:
> CORRECÇÃO RETROACTIVA aplicada por P203B (2026-05-05)

Conteúdo histórico preservado abaixo (não canónico).

#### Edição 3d — `historiograma-passos.md`

Pesquisa cirúrgica feita: nenhum match para "Position"
ou "Counter at locations" como nomenclatura de lacuna no
historiograma. **Sem edições necessárias**.

### Output 4 — este relatório

---

## §2 Tempo de execução

Sessão única; ordem grosseira:

- Leitura do spec P203B: ~2 min.
- C1 inventário empírico (greps + leituras das 3 arms):
  ~10 min.
- Análise da divergência face à premissa do spec: ~5 min.
- C2 re-fixado (Caminho C): ~3 min.
- Implementação do test E2E (1 test consolidado): ~10 min.
- Verificação cargo test + crystalline-lint: ~3 min.
- Correcções administrativas (snapshot §7 + §13;
  auditoria delta §2; verificação historiograma):
  ~10 min.
- Redacção do inventário interno (Output 1): ~10 min.
- Redacção deste relatório: ~5 min.

**Total**: ~60 min.

---

## §3 Caminho escolhido

**Caminho C — Confirmação (sem alteração de código)**.

Spec original previa Caminhos A (alinhar from_tags ao
walk) ou B (alinhar walk ao extract_payload), assumindo
desalinhamento empírico. C1 revelou que o desalinhamento
**não existe** — pós-P190H/P191C a arquitectura está
alinhada:

- Walk arm Figure puro (P190H eliminou mutações).
- `extract_payload` preserva `kind` literal + deriva
  `is_counted`.
- `populate_intr_from_tag_start` (não `from_tags::Figure`
  — esse arm não existe) aplica gate + default
  consistentemente.

C2 re-fixado per spec §6 (clausula explícita para casos
de divergência empírica).

**Caminho C** = formalização do fecho via test E2E + 
correcção administrativa. Zero código produção tocado.

---

## §4 Tests adicionados

| Test | Local | Resultado |
|------|-------|-----------|
| `p203b_lacuna_1_e_1b_fecho_formal_4_casos` | `introspect.rs` módulo `tests` | ✅ verde |

**Δ tests**: +1 (1823 → 1824).

---

## §5 Métricas

| Métrica | Pré-P203B | Pós-P203B | Δ |
|---------|-----------|-----------|---|
| Tests workspace | 1823 | **1824** | +1 |
| Crystalline-lint violations | 0 | 0 | = |
| LOC produção | (sem alteração) | (sem alteração) | 0 |
| LOC tests | +~120 (test consolidado) | — | +120 |
| LOC documental | (sem alteração) | +~100 (correcções administrativas) | +100 |
| Lacunas residuais | #1, #1b "Position" (incorrecto) | **0 lacunas residuais** | -2 (+ correcção nomenclatura) |
| ADRs | 70 | 70 | = |

---

## §6 Lacunas marcadas fechadas

| Lacuna | Fecho estrutural | Formalização |
|--------|------------------|--------------|
| **#1** (Figure kind=None ↔ Introspector) | P190H + P191C | **P203B** (test consolidado) |
| **#1b** (gate `is_counted` em populate) | P191C | **P203B** (test consolidado) |

Pós-P203B: **zero lacunas residuais formalmente
catalogadas** em `m1-lacunas-captura.md` ou P200
consolidado §7.

**Concerns ortogonais não-catalogados** (informativo;
não são lacunas):
- Position concrete (`position_of` stub) — adiada para
  M8 por ADR-0066.

---

## §7 Decisões tomadas durante a leitura

### 7.1 Re-fixar C2 com Caminho C

C1 revelou que walk arm Figure já é puro (P190H
eliminou). Spec previu este caso em §6 ("Em caso de
divergência empírica relevante face a P203A... re-fixar
C2 com os novos dados").

**Decisão**: introduzir Caminho C (Confirmação sem
alteração de código) como alternativa empírica aos
Caminhos A/B do spec.

### 7.2 Único test consolidado vs múltiplos tests

Spec C3 lista 4 casos. Escolhi **um test consolidado**
em vez de 4 tests separados porque:
- Casos partilham fixtures construtores (`Content::Figure
  {...}`).
- Asserções 3-grupo (extract_payload, populate_intr,
  Tags) são paralelas — agrupar dentro de um test
  reduz duplicação.
- Casos pré-existentes (linhas 1750, 2871, 2927, 2945)
  já cobrem 3 dos 4 casos isoladamente — caso 1
  (kind=None+sem caption) é o único novo.

Test consolidado serve como **formalização explícita**
do fecho de #1/#1b — sentinel test referenciando P203B
no comentário.

### 7.3 ADR-0067 e ADR-0073: não tocar

P203B foca-se em #1/#1b. ADRs 0067 (Attribute grammar
scoping; PROPOSTO) e potencial 0073 (M8 comemo) ficam
para outros passos. Spec §8 não-objectivos confirma
("Não promove ADR-0067... Não decide M8").

### 7.4 ADR para P203B: não criar (per C7)

Justificação: Caminho C é formalização administrativa.
Nenhum mecanismo novo. Padrão usado (gate is_counted
+ unwrap_or default) já coberto por ADR-0069/0070/0071
existentes.

### 7.5 Historiograma sem edições

Pesquisa `grep -n "Position\|Counter at locations"
historiograma-passos.md` retornou 0 matches para
nomenclatura errada. Apenas matches em "M4-residual" e
"auditor #1" que são conceitos diferentes (não
relacionados a Position-as-lacuna). **Sem edições
cirúrgicas necessárias**.

### 7.6 Localização dos ficheiros administrativos

Snapshot e auditoria delta encontrados em
`00_nucleo/diagnosticos/` (não em `00_nucleo/`
top-level como spec P202 escreveu). Reorganização
prévia consolidou todos os documentos em `diagnosticos/`.
Edições aplicadas nos paths actuais.

---

## §8 Achado registado — `P203B.div-1`

**Divergência**: spec P203B §1 e §3 declaram que walk
arm usa `kind.as_deref().unwrap_or("image")` e que há
`from_tags` arm Figure. Empíricamente:
- Walk arm Figure puro (P190H eliminou em 2026-05-05).
- Não há `from_tags::Figure` arm — população acontece
  durante walk via `populate_intr_from_tag_start`
  (P191B/C ADR-0071).

Spec premissa reflectia arquitectura **pré-P190H**.
Pós-P190H/P191C alinhamento já existe.

**Decisão (per spec §6)**: re-executar C1 com valores
actuais; re-fixar C2 com novos dados (Caminho C).
Trabalho útil = formalização do fecho via test +
correcção administrativa.

---

## §9 Critério de progressão respeitado

Per spec §6 + §C9, P203B está concluído quando:

- [x] Tests workspace verdes (1824; +1 face baseline
  1823).
- [x] Crystalline-lint 0 violations.
- [x] Walk e populate produzem mapping idêntico para
  os 4 casos (test `p203b_lacuna_1_e_1b_fecho_formal_4_casos`
  passa).
- [x] Snapshot §7 e §13 reescritos.
- [x] Auditoria delta P201 §2 anotada.
- [x] Historiograma corrigido cirurgicamente (verificado
  sem matches relevantes; sem edições necessárias).
- [x] Lacunas #1/#1b marcadas fechadas (estruturalmente
  P190H/P191C; formalizadas P203B).

**Divergência empírica** `P203B.div-1`:
- Registada em §8 deste relatório e §2 do inventário.
- Decisão: re-fixar C2 (Caminho C) per spec §6.

---

## §10 Não-objectivos respeitados

Per spec §8, P203B não:

- [x] Não materializou Position concrete (concern
  ADR-0066 + M8).
- [x] Não decidiu M8.
- [x] Não promoveu ADR-0067.
- [x] Não tocou em outros consumers além dos 3 arms
  (walk / extract_payload / populate_intr) e os tests
  — na realidade tocou apenas em **tests** (zero código
  produção).
- [x] Não reescreveu o historiograma inteiro
  (verificação cirúrgica; sem edições).
- [x] Não reescreveu auditoria delta P201 — anotação
  retroactiva apenas.
- [x] Não criou sub-passos `*C+`.

---

## §11 Sugestão para próximo passo (não-vinculativa)

P203 série está estructuralmente completa após P203B —
**zero lacunas residuais formalmente catalogadas**.

Próximas decisões estratégicas (humano decide):

### 11.1 P203C — encerrar série P203 (recomendado)

Relatório consolidado P203 (P203A + P203B). Trabalho
S documental.

### 11.2 P204A — diagnóstico de M8 (caminho default
snapshot §13)

Pré-condição cumprida (M5+M6+M7 fechados; lacunas
catalogadas zeradas). M8 introduz comemo
+ paridade vanilla + cobre Position concrete
naturalmente.

### 11.3 P204A — promoção formal de ADR-0067

Se humano quiser materializar pattern attribute-grammar
(precisa de propriedade alvo concreta) antes de M8.

### 11.4 P204A — F3 completo (refactor 21 fields
ortogonais Layouter)

Trabalho ortogonal a M8; pode ser feito antes ou depois.

### 11.5 Pausa estratégica

Validar saída cristalino vs vanilla em corpus actual
(sem comemo) antes de M8.

**P203B reporta. Não decide.**

---

## §12 Notas operacionais

### 12.1 Diagnóstico-primeiro funcionou

P203A detectou que P203 baseado em premissa errada
(Position) — recomendou pivot para lacuna #1 real
(Figure). P203B começou com C1 inventário empírico —
detectou nova divergência (premissa do spec P203B
sobre walk arm). Ambos os passos demonstram que
"empírico precede afirmação herdada" (per spec §10).

### 12.2 Trabalho útil cumulativo

P203A + P203B juntos:
- Confirmaram empíricamente que lacunas #1/#1b já
  estavam estructuralmente fechadas pós-P190H/P191C.
- Formalizaram o fecho via test consolidado.
- Corrigiram nomenclatura errada nos 3 documentos
  administrativos.
- Reposicionaram Position como concern ortogonal
  (ADR-0066 + M8), não lacuna catalogada.

### 12.3 Magnitude real vs planeada

Spec P203B planeou M (S-M para feature + S correcção).
Real: S (zero código produção; 1 test + 3 edições
administrativas). Diferença reflecte que o trabalho
empírico já estava feito — P203B é formalização, não
implementação.

---

**Fim do relatório P203B.**
