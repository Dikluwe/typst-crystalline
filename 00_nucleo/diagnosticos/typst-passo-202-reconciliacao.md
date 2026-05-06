# Registo de reconciliação P202

**Data**: 2026-05-05.
**Executor**: Claude Code (LLM externa, modelo Opus 4.7).
**Spec**: `00_nucleo/materialization/typst-passo-202.md`.
**Auditoria de origem**:
`00_nucleo/diagnosticos/typst-passo-201-auditoria-delta.md`.
**Snapshot reconciliado**:
`00_nucleo/snapshot-2026-05-05.md`.
**Snapshot pré-P202 (reconstrução conceptual)**:
`00_nucleo/snapshot-2026-05-05.pre-P202.md`.

**Escopo**: 4 divergências detectadas em §1 C6 da auditoria
delta + 5 notas de auditor em §8.

---

## §1 Tabela resumo — questão → decisão → alteração

| # | Questão | Estado anterior | Estado real | Decisão P202 | Alteração no snapshot |
|---|---------|-----------------|-------------|--------------|----------------------|
| **D1** | 7 ADRs ACEITES no ciclo | "7" (P192 §10) | 6 ACEITES estritas + 2 EM VIGOR + 3 PROPOSTAS | Adoptar 6 ACEITES estritas; clarificar interpretação | §4 reescrita com lista explícita por categoria |
| **D2** | 1.802 tests | 1.802 | 1823 verdes (+21) | Adoptar valor empírico | §2, §11, §13 actualizadas |
| **D7** | 33 aplicações diag-1º | 33 | ~35 (range 33-35) | Adoptar range "~35 (33-35 plausível)" | §11 actualizado com range |
| **D11** | Layouter 19 fields | 19 (sem counter) | 22 (sem counter) | Adoptar 22 fields | §5 reescrita com lista 22 fields; §8 ortogonais 18→21 |
| **N8.1** | ADR-0067 ACEITE em consolidados vs PROPOSTO em ficheiro | ACEITE (consolidados P190/P192) | PROPOSTO (ficheiro 00_nucleo/adr/typst-adr-0067-*.md linha 3) | Manter PROPOSTO; correcção retroactiva | §4 ADR-0067 listada como PROPOSTO; correcção registada |
| **N8.2** | ADR-0063 slot vazio | "outra crate reservada" | slot conceptual; sem ficheiro | Apenas documentar; **não criar ficheiro** | §4 ADR-0063 explicitamente "slot conceptual vazio" |
| **N8.3** | Datação P157-P160 ausente | cabeçalhos sem data | inferíveis entre 2026-04-26 e 2026-04-30 | Aceitar ausência; documentar janela inferida | snapshot não modifica relatórios anteriores; janela inferida no historiograma §1.2 |
| **N8.4** | Datação anómala P190/P192 vs P195-P200 | datação relatórios em ordem inversa à dependência | "ordem de publicação ≠ ordem de dependência" | Aceitar e documentar a distinção | §3 marcos reflectem ordem de dependência semântica; auditoria delta §8.4 absorve |
| **N8.5** | Relatórios individuais ausentes (P190B-H, P183B, P160B, P181J) | absorvidos por consolidados | mesmo | Formalizar convenção de absorção | §10 convenções activas inclui "Pattern absorção pelo consolidado" |

---

## §2 Detalhe — D1 (ADRs ACEITES no ciclo)

### Estado anterior (citado)

P192 consolidado §10 (linha 253): "7 ADRs ACEITES no ciclo
M5/M6/M7" listando 0066, 0067, 0068, 0069, 0070, 0071,
0072.

P192B spec linha 57: "7 ADRs ACEITES no ciclo M5/M6/M7
(0066, 0067, 0068, 0069, 0070, 0071, 0072)".

### Estado real (verificado)

Inspecção dos ficheiros canónicos
`00_nucleo/adr/typst-adr-006[1-9]*.md` e
`00_nucleo/adr/typst-adr-007[0-2]*.md`:

- **ACEITE estrito** (status `ACEITE` no ficheiro): 6
  ADRs — 0066, 0068, 0069, 0070, 0071, 0072.
- **EM VIGOR** (status `EM VIGOR` no ficheiro): 2 ADRs —
  0064, 0065.
- **PROPOSTO** (status `PROPOSTO` no ficheiro): 3 ADRs —
  0061, 0062, 0067.

### Decisão

Adoptar **6 ACEITES estritas** como valor canónico.
Clarificar no snapshot a interpretação:

> "ACEITE" estrita refere status formal do ficheiro;
> "EM VIGOR" é categoria distinta; o agregado de 8 só
> faz sentido se "ACEITE" for usado em sentido lato
> (ACEITE + EM VIGOR).

### Justificação

P202 honra status canónico declarado nos ficheiros ADR.
Consolidados P190/P192 incorreram em erro propagado
(detectado em N8.1).

### Alteração concreta

Snapshot §4 reescreve a secção em três sub-listas:

- ACEITES estritas: 6.
- EM VIGOR: 2.
- PROPOSTAS pendentes: 3.

Total ciclo: 11 ADRs novas (slot 0063 vazio).

---

## §3 Detalhe — D2 (testes workspace)

### Estado anterior

P192 consolidado §10: "Tests workspace: 1.802 verdes
(inalterados)".
P192B spec linhas 60, 110, 252, 504, 519: "1.802 verdes".

### Estado real

`cargo test --workspace` em 2026-05-05 (executado durante
P201 e re-executado em P202 para confirmação):

```
Total tests passed: 1823
```

Distribuição: 1563 + 215 + 24 + 21 = 1823.

### Decisão

Adoptar **1823 verdes** como valor empírico.

### Justificação

Diferença +21 tests reflecte adições em séries terminais
(P195+, P200B). Snapshot anterior tinha 1-2 dias de atraso
ao consolidar.

### Alteração concreta

Snapshot §2, §11, §13 actualizadas com 1823. Baseline P156A
mantém 1145; Δ = +678.

---

## §4 Detalhe — D7 (aplicações diagnóstico-primeiro)

### Estado anterior

P192 consolidado §10 linha 265: "33ª aplicação
diagnóstico-primeiro consecutiva".

### Estado real

Reconstrução completa (auditoria delta P201 §1):

- Pré-ciclo (P156A): 7 aplicações (P131A, P132A, P140A,
  P148, P154A, P156A, P156B).
- Ciclo: ~28 aplicações adicionais (P156J, P156L, P157,
  P158, P159, P159B, P160, P167, P180, P181A, P182A,
  P183A, P184A, P185A, P186A, P187A, P188A, P189A, P190A,
  P191A, P192A, P193A, P194A, P195A, P196A, P197A, P198A,
  P199A, P200A).

Total: ~35 (depende se P156L e P159B contam como diag
formal ou refino-via-ADR-0065).

### Decisão

Adoptar **range "~35 (33-35 plausível)"**. Não tentar
reconciliação forçada.

### Justificação

Discrepância marginal interpretativa (ambiguidade entre
diag formal e refino-via-ADR-0065). Spec value 33 e
empírico 35 ambos plausíveis.

### Alteração concreta

Snapshot §11 actualizado com "~35 (33-35 plausível)".

---

## §5 Detalhe — D11 (Layouter fields)

### Estado anterior

ADR-0070 tabela validação empírica:
`Layouter counter field | 19/20 fields | eliminado`.

Implicação: pós-eliminação, Layouter teria 19 fields.

P192 consolidado §8 linha 216: "F3 completo (Layouter
restantes 19 fields)".

### Estado real

Inspecção de `01_core/src/rules/layout/mod.rs:69`:

Struct `Layouter<M, S>` tem **22 fields** activos:

```text
metrics, sizer, font_size_pt, style, chain, page_config,
pages, current_items, cursor_x, cursor_y, line_start_x,
current_line, introspector, figure_progress,
is_height_unconstrained, cell_available_h, cell_origin_x,
cell_origin_y, cell_origin_w, locator, current_location,
runtime.
```

`counter` field eliminado ✅ (P190I).

### Decisão

Adoptar **22 fields** como baseline empírico. Recalibrar
F3 ortogonais para 21 (não 18).

### Justificação

Snapshot anterior foi capturado num estado intermédio.
P185C adicionou `locator` + `current_location` (+2 fields
M3 location-aware); P190C/D adicionaram
`runtime: LayouterRuntimeState` (+1 field agregador). Total
+3 face ao 19 declarado.

### Alteração concreta

Snapshot §5 reescreve com lista 22 fields explícita.
§8 (M8 escopo) recalibra F3 parcial: 21 fields ortogonais
(não 18).

---

## §6 Detalhe — N8.1 (ADR-0067 status)

### Estado anterior

Múltiplas referências como ACEITE:

- P190 consolidado §1 linha 28: "5 ADRs ciclo M5/M6:
  ADR-0067, ADR-0068 (ACEITES); ADR-0069 ACEITE...".
- P192A relatório linha 212: "5 ADRs ACEITES no ciclo
  M5/M6: ADR-0067, 0068, 0069, 0070, 0071".
- P192 consolidado linhas 155, 257, 339: ADR-0067 listada
  como ACEITE.
- P192B spec linhas 57, 98: ADR-0067 entre ACEITES.

### Estado real

Ficheiro
`00_nucleo/adr/typst-adr-0067-attribute-grammar-scoping.md`:

- Linha 3: `**Status**: \`PROPOSTO\``.
- Linha 4: `**Validado**: pendente — não vinculativo até
  primeira materialização.`
- Linha 5: `**Data**: 2026-05-02`.
- Linha 240: "ADR-0067 transita de `PROPOSTO` para
  `IMPLEMENTADO` quando todas estas condições forem
  verdadeiras: 1. Pelo menos uma propriedade alvo
  (provavelmente `numbering_active`) materializada com
  o pattern. 2. Walk recebe o parâmetro de atributos
  herdados sem violar P163. 3. Tests E2E confirmam
  scoping léxico funciona — set rule num
  `Content::Styled` não vaza para irmãos. 4. Comparação
  de magnitude com Caminho 2 hipotético confirma que
  Caminho 3 é praticável."

**Nenhuma das 4 condições foi cumprida**.

### Decisão

**Manter PROPOSTO** no snapshot. P202 não promove ADRs
(per spec C8).

Registar correcção retroactiva no snapshot §4 e neste
registo. Promoção formal é trabalho de P203+ se for o
caminho (e exige passo administrativo distinto que
verifique satisfação das 4 condições do plano de
validação).

### Justificação

Spec C8 explicitamente declara: "P202 não promove ADRs.
Apenas regista o que está e regista a decisão."

A propagação do erro nos consolidados P190/P192 não
constitui uma promoção — promoção exige edição do
ficheiro ADR canónico e validação das condições.

### Alteração concreta

Snapshot §4:
- ADR-0067 listada em "PROPOSTAS pendentes" (não em
  ACEITES).
- Secção "Correcção retroactiva — ADR-0067 status"
  documenta a discrepância e cita o ficheiro canónico
  como autoridade.

---

## §7 Detalhe — N8.2 (ADR-0063 slot vazio)

### Estado anterior

P156K-meta linha 14: "ADR-0063 reservada outra crate".
P160A: confirmado que slot 0017 estava ocupado e usou-se
0066. Slot 0063 ficou conceptualmente reservado mas sem
ficheiro físico.

### Estado real

`ls 00_nucleo/adr/typst-adr-006[3]*.md` retorna nada.
**Não existe ficheiro ADR-0063**.

### Decisão

**Apenas documentar**. **Não criar ficheiro
ADR-0063-RESERVADO**.

### Justificação

Per spec C9: "Decisão recomendada: **apenas documentar**.
Criar ficheiro ADR para um slot vazio adiciona ruído sem
benefício estrutural."

Adicionalmente: per spec §8 não-objectivos: "P202 não cria
ADRs novas (ADR-0063 não é criado mesmo que C9 fosse pela
alternativa de formalização — recomendação contrária em
C9)."

### Alteração concreta

Snapshot §4 inclui sub-secção "ADR-0063: slot conceptual
vazio" explicando:
- Não existe ficheiro físico.
- Reservado conceptualmente para column flow.
- Convenção implícita aceite e documentada.
- P202 mantém slot vazio.

---

## §8 Detalhe — N8.3 (datação P157-P160 ausente)

### Estado anterior

Cabeçalhos de P157, P157A-C, P158, P158A-C, P159,
P159A-G, P160 não declaram data explícita.

### Estado real

P156L é 2026-04-26; P161 é 2026-04-30. Janela inferida:
**entre 2026-04-26 e 2026-04-30**.

### Decisão

**Aceitar ausência**. **Não modificar relatórios anteriores
retroactivamente**.

### Justificação

Per spec C10: "Decisão recomendada: **aceitar ausência**.
Modificar relatórios anteriores retroactivamente quebra
preservação histórica. Registar a janela inferida no
historiograma."

### Alteração concreta

Não há alteração ao snapshot. A janela inferida já está
documentada no historiograma `00_nucleo/historiograma-passos.md`
§1.2 ("(s/ data explícita)" + janela inferida).

---

## §9 Detalhe — N8.4 (datação anómala P190/P192 vs P195-P200)

### Estado anterior

Datas em ficheiros relatório:
- P190G/H/I: 2026-05-05.
- P191A-C: 2026-05-05.
- P192A-B: 2026-05-05.
- P193-P200: 2026-05-03/04.

Anomalia: P193+ depende semanticamente de M6 fechado em
P190I 2026-05-05, mas P193-P200 datam 2026-05-03/04.

### Estado real

Hipótese da auditoria delta P201 §8.4: a sequência §9 P189
foi preparada estructuralmente em paralelo com M5 universal
fechar (P200B 2026-05-04), e M6 fechou no dia seguinte
(P190I 2026-05-05). Datas reflectem **ordem de publicação**,
não **dependência semântica**.

### Decisão

**Aceitar a hipótese e documentar** a distinção "ordem de
publicação" vs "ordem de dependência semântica".

### Justificação

Per spec C11: "Decisão recomendada: **aceitar e documentar**.
P202 regista a distinção; auditor humano pode validar via
Git log se quiser."

Investigação mais profunda em Git log fica fora do escopo
de P202.

### Alteração concreta

Snapshot §3 lista marcos pela ordem de dependência semântica
(M5 universal P200B → M6 P190I → M7 P192B), não pela ordem
de datação dos relatórios. A auditoria delta §8.4 absorve
a documentação detalhada.

---

## §10 Detalhe — N8.5 (relatórios individuais ausentes)

### Estado anterior

- **P190B-H**: sem relatórios individuais; consolidado P190
  absorve.
- **P183B**: sem relatório individual; P183C absorve com
  documentação retroactiva como "primitiva inadequada".
- **P160B**: sem relatório; descartado por P161.
- **P181J**: relatório curto; consolidado P181 absorve.

### Estado real

Mesmo. Inspecção empírica confirma:
- `ls typst-passo-190b-relatorio.md` → não existe.
- `ls typst-passo-183b-relatorio.md` → não existe.
- `ls typst-passo-160b-relatorio.md` → não existe.
- `ls typst-passo-181j-relatorio.md` → existe (mas curto;
  consolidado absorve).

### Decisão

**Aceitar absorção pelos consolidados**. **Formalizar a
convenção** "Pattern absorção pelo consolidado".

**Não reconstruir relatórios em falta retroactivamente**.

### Justificação

Per spec C12: "Decisão recomendada: **aceitar absorção e
formalizar a convenção**. ADR-0036 não obriga a relatório
individual quando passo é parte de série coberta por
consolidado."

Reconstrução retroactiva quebraria a regra de preservação
histórica.

### Alteração concreta

Snapshot §10 "Convenções operacionais activas" inclui
explicitamente:
> "Pattern absorção pelo consolidado (relatórios
> individuais podem ser absorvidos)."

---

## §11 Itens explicitamente fora de escopo de P202

P202 **NÃO** executa as seguintes acções (per spec §8
não-objectivos):

- **Promoção formal de ADR-0067** — fica para P203+ se for
  o caminho. Exige verificação das 4 condições do plano de
  validação ADR-0067.
- **Criação de ADR-0063** — slot mantém-se conceptual sem
  ficheiro.
- **Decisão sobre o caminho de M8** — P202 reporta; não
  decide.
- **Modificação retroactiva de relatórios anteriores**
  (P157-P160 datação; P190B-H reportos individuais; P183B
  reporto individual; P160B reporto individual).
- **Investigação Git log** para validar ordem de dependência
  vs publicação (auditoria delta §8.4).
- **Promoção de ADRs PROPOSTAS** (0061 Layout Fase X;
  0062 hayagriva).
- **Criação de ADR-0073 ou superior**.
- **Modificação de código**.

---

## §12 Critério de progressão (verificação)

Per spec P202 §6, P202 está concluído quando:

- [x] Os 3 ficheiros existem:
  - `00_nucleo/snapshot-2026-05-05.md` (snapshot reescrito).
  - `00_nucleo/diagnosticos/typst-passo-202-reconciliacao.md`
    (este registo).
  - `00_nucleo/materialization/typst-passo-202-relatorio.md`
    (relatório do passo).
- [x] Backup do snapshot anterior existe:
  `00_nucleo/snapshot-2026-05-05.pre-P202.md` (reconstrução
  conceptual a partir das fontes pré-P202).

  **Nota interpretativa**: o snapshot anterior nunca existiu
  como ficheiro discreto. Os valores estavam dispersos
  por P192 consolidado §10, P192B spec, ADR-0070 e spec
  P201 §1. O backup é uma reconstrução conceptual; os
  ficheiros fonte permanecem intactos. Esta decisão é
  documentada em §1 do backup e em §3.1 do relatório
  P202.

- [x] C1-C12 todos endereçados (ver §1 tabela e §2-§10
  detalhes).
- [x] 4 divergências (D1, D2, D7, D11) reflectidas no
  snapshot reescrito.
- [x] 5 notas de auditor (N8.1-N8.5) com decisão registada
  e alteração concreta documentada (ou ausência justificada).
- [x] Snapshot reescrito é internamente consistente
  (§3↔§4↔§5↔§11 sem contradição) — verificação cruzada
  feita.

---

## §13 Lista de divergências e notas

### Divergências (4) — todas reconciliadas

| Cód | Tópico | Acção |
|-----|--------|-------|
| D1 | 7 ADRs ACEITES | Reescrever §4 com 6 estritas + 2 EM VIGOR + 3 PROPOSTAS |
| D2 | 1.802 tests | Substituir por 1823 |
| D7 | 33 aplicações | Substituir por "~35 (33-35 plausível)" |
| D11 | Layouter 19 fields | Substituir por 22 |

### Notas de auditor (5)

| Cód | Tópico | Decisão |
|-----|--------|---------|
| N8.1 | ADR-0067 status | Manter PROPOSTO; correcção retroactiva |
| N8.2 | ADR-0063 slot vazio | Apenas documentar; não criar ficheiro |
| N8.3 | Datação P157-P160 | Aceitar ausência |
| N8.4 | Datação anómala P190/P192 vs P195-P200 | Aceitar e documentar distinção |
| N8.5 | Relatórios individuais ausentes | Formalizar convenção de absorção |

---

## §14 Referências

- `00_nucleo/snapshot-2026-05-05.md` (snapshot reconciliado).
- `00_nucleo/snapshot-2026-05-05.pre-P202.md` (backup
  conceptual).
- `00_nucleo/diagnosticos/typst-passo-201-auditoria-delta.md`
  (auditoria que detectou as 4 divergências + 5 notas).
- `00_nucleo/historiograma-passos.md` (linha temporal
  completa P0-P200).
- `00_nucleo/materialization/typst-passo-201.md` (spec P201).
- `00_nucleo/materialization/typst-passo-201-relatorio.md`
  (relatório P201).
- `00_nucleo/materialization/typst-passo-202.md` (spec
  deste passo).
- `00_nucleo/materialization/typst-passo-202-relatorio.md`
  (relatório deste passo).
- ADRs canónicas: `00_nucleo/adr/typst-adr-006[1-2]*.md`,
  `00_nucleo/adr/typst-adr-006[4-9]*.md`,
  `00_nucleo/adr/typst-adr-007[0-2]*.md`.
- Consolidados pré-P202 com erro propagado:
  `typst-passo-190-relatorio-consolidado.md`,
  `typst-passo-192a-relatorio.md`,
  `typst-passo-192-relatorio-consolidado.md`,
  `typst-passo-192b.md`.
