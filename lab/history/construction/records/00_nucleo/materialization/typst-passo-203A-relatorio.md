# Relatório do passo P203A

**Data de execução**: 2026-05-05.
**Executor**: Claude Code (Opus 4.7).
**Spec**: `00_nucleo/materialization/typst-passo-203A.md`.
**Natureza**: diagnóstico-primeiro (zero código tocado) +
auditoria empírica. **36ª aplicação consecutiva** do padrão
diagnóstico-primeiro.
**Magnitude planeada**: S–M.
**Magnitude real**: S–M (volume audit ≈ 10 cláusulas;
diagnóstico 12 cláusulas).

---

## §1 O que foi feito

P203A produziu **3 ficheiros** (sem código tocado):

### Output 1 — Auditoria empírica

Localização:
`00_nucleo/diagnosticos/typst-passo-203A-auditoria-position.md`.

Conteúdo:
- §1 Estado de partida verificado (3 confirmações + 1
  divergência crítica).
- §2 Cláusulas A1-A10 com etiqueta + evidência.
- §3 Resumo dos achados.
- §4 Divergência registada (`P203A.div-1`).
- §5 Observações adicionais.
- §6 Referências.

Tamanho: ~13 KB.

### Output 2 — Diagnóstico

Localização:
`00_nucleo/diagnosticos/typst-passo-203A-diagnostico.md`.

Conteúdo:
- §1 Sumário das decisões (tabela 11 cláusulas).
- §2-§12 detalhe C1-C11 instanciadas.
- §13 Critério de progressão.
- §14 Referências.

Tamanho: ~10 KB.

### Output 3 — este relatório

Localização:
`00_nucleo/materialization/typst-passo-203A-relatorio.md`.

---

## §2 Tempo de execução

Sessão única; ordem grosseira:

- Leitura do spec P203A: ~3 min.
- Auditoria A1-A4 (Position type, vanilla, consumers,
  TagIntrospector): ~5 min.
- Auditoria A5-A7 (walk fn, Layouter, vanilla pipeline):
  ~7 min.
- Auditoria A8 (lacuna #1b — descoberta da divergência
  crítica): ~10 min (cross-reference m1-lacunas-captura
  + P200 consolidado §7 + P201 auditoria delta).
- Auditoria A9-A10 (tests, corpus): ~3 min.
- Redacção da auditoria (Output 1): ~10 min.
- Análise do impacto da divergência + redacção do
  diagnóstico (Output 2): ~12 min.
- Redacção deste relatório: ~5 min.

**Total**: ~55 min.

---

## §3 Decisões tomadas durante a leitura

### 3.1 Descoberta crítica em A8 — divergência empírica

A auditoria A8 revelou que **a premissa central de P203A
está empíricamente errada**:

- Spec P203A §1: "P203 endereça lacunas #1 (Position) e
  #1b (Position-related)..."
- Realidade empírica (P200 consolidado §7 +
  m1-lacunas-captura.md):
  - #1 = Figure kind=None ↔ Introspector
  - #1b = from_tags arm Figure sem gate `is_counted`
  - #2 = reservada (vazia)

**Origem do erro** (cadeia de propagação):
1. P201 auditoria delta §2 (escrita por mim, P201)
   atribuiu "Position" a #1/#1b/#2 sem cross-check com
   `m1-lacunas-captura.md` ou P200 consolidado §7.
2. P202 reconciliação reificou a interpretação no
   snapshot 2026-05-05.
3. P203A spec herdou.

**Decisão (per spec P203A §6)**: registar `P203A.div-1`;
ramificar.

### 3.2 Posição do diagnóstico — não decidir M8 nem promover ADR

Per spec §8 não-objectivos, P203A:
- Não toca em código.
- Não cria tipo Position.
- Não adiciona sub-store.
- Não promove ADRs.
- Não pré-define sub-passos.

**Decisão**: completei C1-C11 com valores concretos
(diagnóstico §2-§12) **mas com a observação que P203B+ não
deveria prosseguir como Position-focused**. Esta é uma
recomendação, não uma decisão.

### 3.3 Honestidade sobre redundância com M8

**Decisão**: documentar explicitamente em C9 que P203
(Position-focused) é redundante com M8. Razões:
- ADR-0066 (ACEITE P192B) já adia Position runtime para
  M8.
- M8 introduz comemo + paridade vanilla — naturalmente
  cobre Position.
- Materializar antes seria trabalho duplicado.

Esta honestidade é cumprida per spec §7 ("Honestidade
sobre dead code, gate dormente, magnitude real" —
princípio aplicado).

### 3.4 Plano `*B+` — recomendação de pivot

**Decisão**: em vez de pré-fixar P203B-G como Position
materialização, apresentei **4 opções α-δ não-vinculativas**
para a decisão humana sobre P203B:

- α: pivot para lacuna #1 real (Figure).
- β: pivot para lacuna #1b real (from_tags Figure gate).
- γ: aceitar redundância; avançar para M8 (recomendado).
- δ: materializar Position mesmo assim (ignorar zero
  pressure).

Esta forma respeita o princípio diagnóstico-primeiro —
plano `*B+` emerge do diagnóstico, não do spec.

### 3.5 Não criar ADR dedicada

Per C10 do diagnóstico:
- ADR-0066 já cobre Position estratégicamente.
- M8 criará ADR-0073 (ou similar) que cobrirá Position
  como parte de paridade vanilla.
- Criar ADR dedicada para P203 (Position-focused) seria
  duplicação prematura.

**Decisão**: não criar ADR. Se Opção δ for escolhida,
revisar ADR-0066 (não criar nova).

---

## §4 Magnitude calibrada

**Magnitude planeada (spec)**: S-M.
**Magnitude real (P203A)**: S-M (≈55 min).

**Magnitude estimada caso P203 prossiga** (per C7 do
diagnóstico):
- Opção α (Figure lacuna #1): S-M.
- Opção β (Figure lacuna #1b): S.
- Opção γ (avançar para M8): magnitude P203 = 0
  (encerra série).
- Opção δ (Position materialização): **L cross-modular**.

---

## §5 Recomendação de número ADR

**Decisão (per C10)**: **nenhuma ADR criada por P203A**.

Caso humano escolha:
- Opção α/β: ADR provavelmente desnecessária (são
  resoluções de divergências catalogadas; sem decisão
  arquitectural nova).
- Opção γ: ADR fica para M8 (provavelmente ADR-0073 ou
  ADR-0074).
- Opção δ: revisar ADR-0066 (não criar nova).

---

## §6 Sugestão para próximo sub-passo (não-vinculativa)

Per spec §10 ("execução"), P203A pode preparar `P203B`
mas não pré-define a sua forma.

**Recomendação**: **Opção γ** (aceitar redundância;
avançar para M8) ou **Opção α** (pivot para lacuna #1
Figure real).

**Justificação**:
- γ é a opção mais alinhada com o trabalho útil cumulativo
  — M8 cobre o concern naturalmente.
- α é a opção mais alinhada com "fechar lacunas
  catalogadas" — endereçar a lacuna #1 real (não a
  inventada por P201 auditoria delta).
- δ tem custo alto (L cross-modular) sem desbloquear nada
  empírico.
- β tem escopo muito pequeno; pode ser combinada com α
  como sub-trabalho.

**Próximas decisões estratégicas**:

1. Humano lê P203A diagnóstico §9 (4 opções).
2. Humano decide entre α/β/γ/δ.
3. Spec P203B (ou P204) é redigido com base na escolha.
4. P204 administrativo pode corrigir snapshot
   2026-05-05 + auditoria delta P201 §2 quanto à
   nomenclatura de lacunas (separadamente da decisão
   sobre Position).

**P203A reporta. Não decide.**

---

## §7 Critério de progressão respeitado

Per spec §6, P203A está concluído quando:

- [x] A1-A10 todos com etiqueta CONFIRMADO ou DIVERGÊNCIA
  registada.
- [x] C1-C11 instanciadas com valores concretos.
- [x] Magnitude calibrada (C7: L se Opção δ; ~0 se γ).
- [x] Plano `*B+` sem condicionais (recomendação não-
  vinculativa γ ou α; sem `if`).
- [x] ADR dedicada decidida (C10: não criar).

**Divergência empírica relevante** (`P203A.div-1`):
- Registada em auditoria §4.
- Decisão escolhida: **ramificar** — registar
  recomendação de pivot e correcção administrativa
  futura.

---

## §8 Não-objectivos respeitados

Per spec §8, P203A não:

- [x] Não tocou em código.
- [x] Não criou tipo `Position`.
- [x] Não adicionou sub-store `positions`.
- [x] Não modificou `position_of`.
- [x] Não promoveu ADRs.
- [x] Não pré-definiu sub-passos `*B+` (apenas opções
  não-vinculativas para humano decidir).
- [x] Não decidiu M8.

---

## §9 Achados resumo

| Achado | Implicação |
|--------|-----------|
| Tipo `Position` ausente em L1 | Trabalho potencial para P203 caso prossiga |
| Vanilla tem `DocumentPosition`/`PagedPosition`/`HtmlPosition` | Réplica disponível como referência |
| 0 consumers `position_of` em produção | Materializar não desbloqueia nada |
| 9 sub-stores TagIntrospector confirmados | Posição lógica para `positions` se decidido |
| Walk-time NÃO PODE calcular Position (A5) | Walk-time puro impossibilitado |
| Layouter tem informação suficiente (A6) | Layouter feedback é único caminho viável |
| Vanilla pipeline é POST-LAYOUT (A7) | Cristalino single-pass divergiria intencionalmente |
| **Lacunas #1/#1b/#2 NÃO são Position** | **Premissa de P203A errada** |
| 2 tests stub apenas (A9) | Migration custo trivial em testes |
| Zero corpus pressure (A10) | Zero motivação empírica para materializar agora |
| ADR-0066 já adia Position M8 | Trabalho redundante com M8 |

---

## §10 Notas operacionais

### 10.1 Spec P203A respeitou `*A` discipline

P203A spec foi bem desenhada como diagnóstico-primeiro:
- Auditoria precede diagnóstico.
- Cláusulas C-decisão fixadas com base em A-empírico.
- Sem condicionais.
- Plano `*B+` emerge, não pré-definido.

A descoberta da divergência (premissa errada) é
precisamente o tipo de output que diagnóstico-primeiro
existe para detectar. **Pattern funcionou**.

### 10.2 Contraste com P201/P202

P201/P202 foram administrativos. P203A é diagnóstico de
feature — escopo focado, volume de leitura menor (≈10
greps + 5 reads vs ≈100 ficheiros em P201).

### 10.3 Correcção retroactiva sugerida (não bloqueante)

`P203A.div-1` documenta erro propagado em P201 auditoria
delta + P202 snapshot. **Correcção é trabalho
administrativo separado** (P204 ou similar). Não bloqueia
P203B.

Sugestão: humano pode decidir incluir correcção da
nomenclatura de lacunas no mesmo sub-passo que pivota
P203B (Opção α/β) ou separadamente.

---

**Fim do relatório P203A.**
