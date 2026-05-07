# Relatório do passo P204H

**Data de execução**: 2026-05-07.
**Executor**: Claude Code (Opus 4.7).
**Spec**: `00_nucleo/materialization/typst-passo-204H.md`.
**Natureza**: encerramento da série M8; passo
L0-puro / administrativo (sem alteração de código
produção).
**Sub-passo `H` da série M8** — sétimo de 7 (B-H) per
ADR-0073; encerramento.
**Magnitude planeada**: S documental.
**Magnitude real**: **S documental** (~30 min; 3 ficheiros
novos + 3 modificados; sem código produção).

---

## §1 O que foi feito

P204H encerrou a série P204 com:

1. Auditoria das 9 condições de validação de ADR-0073
   (C1) — 8 CUMPRIDAS, 1 PARCIAL (`P204F.div-1`).
2. Forma de fecho fixada (C2) — "estruturalmente
   fechado" (análogo a M7 P192B).
3. Caminho de resolução fixado (C3) — Caminho A
   (aceitar parcialmente).
4. Relatório consolidado da série (C4) — 13 secções.
5. ADR-0073 transitada PROPOSTO → ACEITE (C5) com bloco
   "Validação P204A–H" listando 9 condições.
6. ADR-0066 transitada ACEITE → SUPERSEDED-BY 0073 (C6)
   com bloco "Superseded em P204H per ADR-0073"
   preservando conteúdo histórico.
7. Blueprint anotado com marca cirúrgica [P204H] em §3.0
   (C7) — sem reescrita ampla.
8. Verificação final (C9) — 1852 tests verdes; 0
   violations.

### Output 1 — Inventário interno

Localização:
`00_nucleo/diagnosticos/typst-passo-204H-inventario.md`.

Conteúdo:
- §1 C1 auditoria (tabela 9 linhas com etiqueta + evidência).
- §2 C2 forma de fecho ("estruturalmente fechado") com
  justificação em 5 pontos.
- §3 C3 caminho de resolução (A) com critério aplicado +
  tensão registada explicitamente.
- §4 5 decisões durante a leitura (D1–D5).

Tamanho: ~7 KB.

### Output 2 — Relatório consolidado da série

Localização:
`00_nucleo/materialization/typst-passo-204-relatorio-consolidado.md`.

13 secções (per spec C4 com adição de §12 Cross-references
e §13 Resumo executivo; spec previa 11 secções):
1. Trajectória da série.
2. Divergências detectadas e absorvidas.
3. Outputs concretos por sub-passo.
4. Achados consolidados.
5. Métricas agregadas.
6. Divergências da série.
7. Padrão demonstrado.
8. Estado pós-série face ao snapshot 2026-05-05.
9. Convenções consolidadas pela série.
10. Não-objectivos respeitados.
11. Sugestão para próximo marco arquitectónico.
12. Cross-references.
13. Resumo executivo.

Tamanho: ~14 KB.

### Output 3 — Edições cirúrgicas

#### `00_nucleo/adr/typst-adr-0073-comemo-introspector.md`

- Linha 3 — Status `PROPOSTO` → `ACEITE` (com nota
  "estruturalmente fechado em P204H").
- Linha 4 — Validado `pendente` →
  `2026-05-07 (P204H)`.
- Linha 6 — Data `2026-05-06` →
  `2026-05-06 (PROPOSTO); 2026-05-07 (ACEITE)`.
- Linha 7 — Sub-passo `P204A (PROPOSTO)` →
  `P204A (PROPOSTO); P204H (ACEITE)`.
- §P204H plano de materialização anotado com
  `✅ MATERIALIZADO 2026-05-07` + sumário literal.
- Bloco novo `## Validação P204A–H` com tabela de 9
  condições + excepção registada.

#### `00_nucleo/adr/typst-adr-0066-introspection-runtime-adiada.md`

- Linha 3 — Status `**ACEITE** (com nota...)` →
  `**SUPERSEDED-BY 0073** (P204H 2026-05-07)`.
- Linha 4 — Data atualizada para incluir transição
  P204H.
- Bloco novo no início (após cabeçalho):
  `## Superseded em P204H per ADR-0073 ACEITE 2026-05-07`
  com 7 bullets dos sub-passos P204B–G + nota de
  preservação histórica.

#### `00_nucleo/diagnosticos/blueprint-projecto.md`

- §3.0 inserido (~25 linhas) antes de §3.1 (que mantém a
  marca temporal 2026-04-25 preservada). §3.0 regista
  marca cirúrgica `[P204H]` com:
  - Estado de cada milestone (M5–M9).
  - **M8 estruturalmente fechado em P204H 2026-05-07**.
  - Trajectória de tests 1145 → 1852.
  - Referência ao consolidado da série.
  - Nota explícita de que reescrita ampla é
    fora-de-escopo (per spec §7).

### Output 4 — Relatório do passo (este ficheiro)

Análogo aos relatórios individuais P204B–G — específico
do sub-passo P204H, distinto do consolidado da série.

---

## §2 Tempo de execução

~30 minutos efectivos:

- ~5 min: leitura da spec + setup TaskList.
- ~10 min: C1 auditoria empírica das 9 condições
  (verificação contra ADR-0073 plano de validação +
  evidência empírica em ficheiros de cada sub-passo).
- ~3 min: C2 + C3 fixação de etiqueta + caminho.
- ~2 min: redacção do inventário (Output 1).
- ~10 min: redacção do consolidado da série (Output 2).
- ~3 min: edições cirúrgicas em ADR-0073, ADR-0066,
  blueprint (Output 3).
- ~2 min: redacção deste relatório (Output 4) +
  verificação final (C9).

---

## §3 Decisões

### D1 — Etiqueta "estruturalmente fechado" preferida sobre "fechado completo"

P204H §8 advertia explicitamente contra inflar para
"fechado completo" sob pressão da forma das transições.
A clarificação inicial fixou transições "completas"
(ACEITE final + SUPERSEDED-BY), mas isso não exige
"fechado completo". Uma ADR pode ser ACEITE com excepção
documentada (como ADR-0066 foi ACEITE com nota
"intermediário" em P192B). Aplicámos o mesmo padrão:
ADR-0073 ACEITE com bloco "Validação P204A–H" listando
8 CUMPRIDAS + 1 PARCIAL.

### D2 — Caminho A (aceitar parcialmente) sobre Caminho R (recuar)

Critério: condição 9 PARCIAL é **justificável** (DEBT-53/54
pre-existing P151/P152, não criado por M8) e **não
bloqueante** (M8 mantém valor estrutural sem ela). Recuar
adiaria fecho de M8 indefinidamente — está fora do
controle de M8 endereçar DEBTs herdados. Caminho A
preserva integridade arquitectural ao explicitar a
excepção sem inflar.

### D3 — Blueprint marca cirúrgica vs reescrita ampla

Spec §7 não-objectivos: "Não reescreve historiograma
inteiro." e "Toca em código produção." (no toca, mas
spec implica escopo limitado). Blueprint datado
2026-04-25 com 1145 tests está obsoleto, mas
reescrevê-lo seria sub-passo dedicado (sugerido em §11
"higiene documental"). P204H aplica apenas marca §3.0
com data 2026-05-07 + estado de cada milestone +
trajectória — ~25 linhas. Conteúdo histórico abaixo
preservado.

### D4 — Localização do blueprint em `diagnosticos/`

Spec C7 sugere
`00_nucleo/projecto/blueprint-projecto.md`. Realidade
empírica: blueprint vive em
`00_nucleo/diagnosticos/blueprint-projecto.md`. Edição
cirúrgica aplicada na localização real. P204H §3 §3.0
**não** propõe mover o ficheiro (fora-de-escopo).

### D5 — Tests excederam estimativa sem regressão

P204A estimou +10 a +23 tests; real é +28. Não é
regressão (todos verdes); é cobertura mais densa que
o estimado. Detalhe registado em D1 do inventário e em
§5.1 do consolidado.

### D6 — Bloco "Superseded" em ADR-0066 preserva histórico

Padrão P201/P202: ADRs supersededed mantêm conteúdo
histórico (não revogadas). ADR-0066 ganha bloco no
topo explicando que a sua promessa (adopção comemo em
M8) foi cumprida pela materialização de ADR-0073, mas
o conteúdo abaixo (validação empírica P192A,
hash-based convergence intermédia, etc.) permanece
literalmente. Referência cruzada para ADR-0073 e
consolidado da série.

---

## §4 Tensão consciente registada

A spec P204H §2 alertou para tensão entre dois inputs:

1. "Forma do fecho decidida no inventário" — pode resultar
   em "estruturalmente fechado" (não "completo").
2. "Transições completas" — ACEITE final + SUPERSEDED-BY.

P204H resolveu **honestamente** via Caminho A:

- ADR-0073 transita ACEITE final, mas com bloco
  "Validação P204A–H" listando excepção (condição 9
  PARCIAL).
- ADR-0066 transita SUPERSEDED-BY 0073 porque a sua
  promessa estrutural foi cumprida.
- Blueprint regista M8 como "estruturalmente fechado",
  não "fechado completo".

Resultado: forma de fecho honesta + transições completas
+ excepção documentada. Não é compromisso entre
extremos; é coerência empírica.

---

## §5 Sugestão para próximo passo

P204H fechado per C10 com todos os critérios cumpridos:

- ✓ C1 auditoria das 9 condições completa.
- ✓ C2 forma de fecho fixada ("estruturalmente fechado").
- ✓ C3 caminho de resolução fixado (A).
- ✓ C4 relatório consolidado escrito.
- ✓ C5 ADR-0073 transitada (PROPOSTO → ACEITE).
- ✓ C6 ADR-0066 transitada (ACEITE → SUPERSEDED-BY 0073).
- ✓ C7 blueprint actualizado (marca cirúrgica [P204H]).
- ✓ C9 verificação final (1852 verdes; 0 violations).
- ✓ Inventário registado.

Série P204 fechada. M8 estruturalmente fechado.

**Sugestões para próximo passo** (per consolidado §11):

1. **Sub-passo `P204I` ou similar para `P204F.div-1`** —
   investigar e fechar condição 9 (vanilla integration).
2. **Próximo marco arquitectónico** — F3 / Layout Fase X /
   Model Fase 2 / Introspection user-facing.
3. **Optimizações pós-M8** — re-walks parciais; benchmarks
   vanilla observable; CLI watch mode.
4. **Higiene documental** — actualização ampla do
   blueprint; reorganização de DEBTs.
5. **Pausa estratégica** — consolidação antes de novo
   investimento.

P204H **não** decide. Reporta. Humano escolhe.

---

## §6 Cross-references

- **Spec**: `00_nucleo/materialization/typst-passo-204H.md`.
- **Inventário**:
  `00_nucleo/diagnosticos/typst-passo-204H-inventario.md`.
- **Consolidado**:
  `00_nucleo/materialization/typst-passo-204-relatorio-consolidado.md`.
- **ADRs**:
  - `00_nucleo/adr/typst-adr-0073-comemo-introspector.md`
    (ACEITE 2026-05-07; bloco "Validação P204A–H").
  - `00_nucleo/adr/typst-adr-0066-introspection-runtime-adiada.md`
    (SUPERSEDED-BY 0073; bloco "Superseded em P204H").
- **Blueprint**:
  `00_nucleo/diagnosticos/blueprint-projecto.md` §3.0
  (marca [P204H]).
- **Sub-passos predecessores**: P204A (diagnóstico),
  P204B (track applied), P204C (Layouter Tracked),
  P204D (Position concrete), P204E (crystalline_evict),
  P204F (corpus paridade), P204G (measurements).
