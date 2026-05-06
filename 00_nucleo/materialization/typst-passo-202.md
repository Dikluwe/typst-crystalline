# Passo 202 — Reconciliação do snapshot 2026-05-05 com realidade empírica

**Série**: 202 (passo **L0-puro / administrativo**;
reconciliação do snapshot pré-M8 com a realidade empírica
detectada pela auditoria delta de P201).
**Precondição**: P201 concluído; auditoria delta P156A→P200
existe em `00_nucleo/diagnosticos/typst-passo-201-auditoria-delta.md`;
historiograma actualizado em `00_nucleo/historiograma-passos.md`;
4 divergências detectadas em §1 C6 da auditoria delta;
5 notas de auditor em §8 da auditoria delta.

**Numeração**: 202. Sem sufixo `A`. Não é diagnóstico-primeiro
de feature. É passo administrativo análogo a P156A e P201.
**Não bloqueia** o início de M8 — pelo contrário, é
pré-condição para que M8 parta de baseline correcto.

**Natureza**: passo **L0-puro / administrativo**. **Zero
código**. **Zero testes**. **Zero ADRs criadas**. **Zero
DEBTs criados**. Outputs são documentos markdown.

**Particularidade**: análoga a P201 — executado pelo
**Claude Code** lendo directamente o snapshot, a auditoria
delta, os ADRs e o código fonte conforme necessário para
verificar cada item antes de reescrever.

---

## §1 Contexto

P201 detectou que o snapshot 2026-05-05 ("Snapshot de
transição — M8 nova sessão") usado como material de partida
para M8 contém 4 divergências face à realidade empírica:

- #1 — 7 ADRs ACEITES no ciclo (real: 6 estritas ou 8 incl.
  EM VIGOR).
- #2 — 1.802 tests (real: 1823).
- #7 — 33 aplicações diagnóstico-primeiro (real: ~35).
- #11 — Layouter 19 fields (real: 22).

Apenas a #11 é estruturalmente relevante. As outras três são
derivas de 1–2 dias.

P201 também levantou 5 notas de auditor para investigação
(§8.1–8.5):

- 8.1 — ADR-0067 referida como ACEITE no consolidado P190
  mas ficheiro tem `Status: PROPOSTO`.
- 8.2 — ADR-0063 slot vazio reservado conceptualmente para
  column flow; convenção implícita não formalizada.
- 8.3 — Datação inconsistente em P157–P160 (cabeçalhos sem
  data explícita).
- 8.4 — Datação anómala P190–P192 (2026-05-05) vs P195–P200
  (2026-05-03/04) — relatórios publicados em ordem inversa
  à dependência semântica.
- 8.5 — Relatórios individuais ausentes (P190B-H, P183B,
  P160B, P181J).

A decisão tomada (sessão actual) é **reescrever o snapshot
para reflectir a realidade empírica** e endereçar todas as
9 questões num único passo administrativo.

---

## §2 Objectivo literal

Três outputs separados, com propósitos distintos:

### Output 1 — snapshot 2026-05-05 reescrito

Substitui o snapshot anterior. Mesma estrutura (§1–§13 do
original) com valores empíricos confirmados. Backup do
snapshot anterior obrigatório antes de escrever.

### Output 2 — registo de reconciliação

Documento que regista, para cada uma das 9 questões (4
divergências + 5 notas), a decisão tomada, a evidência
empírica, e a alteração concreta no snapshot. Serve de
auditoria do próprio P202.

### Output 3 — relatório do passo

Mesmo formato do relatório de P201. Identifica os outputs,
tempo de execução, decisões tomadas, sugestões para o
próximo passo.

---

## §3 Material a ler

Pasta autorizada para leitura (mesma autorização de P201):

- Snapshot anterior: ficheiro de transição M8 nova sessão
  (caminho exacto a confirmar — provavelmente em
  `00_nucleo/context/` ou `00_nucleo/`).
- `00_nucleo/diagnosticos/typst-passo-201-auditoria-delta.md`.
- `00_nucleo/historiograma-passos.md`.
- `00_nucleo/materialization/typst-passo-201.md`.
- `00_nucleo/materialization/typst-passo-201-relatorio.md`.
- `00_nucleo/adr/typst-adr-0061-*.md` a
  `00_nucleo/adr/typst-adr-0072-*.md` (todos os do ciclo).
- `00_nucleo/materialization/` para validação cruzada de
  passos referenciados.
- Código fonte para validação empírica (mesma lista de C6
  da auditoria delta).

Pasta `00_nucleo/context/` continua **não autorizada** —
excepto para ler o snapshot anterior se for esse o seu
único local.

---

## §4 Cláusulas a executar

### C1 — Localizar e fazer backup do snapshot anterior

1. Localizar o ficheiro do snapshot 2026-05-05 ("Snapshot
   de transição — M8 nova sessão").
2. Fazer backup em
   `00_nucleo/snapshot-2026-05-05.pre-P202.md` (mesma
   convenção que P201 usou para o historiograma).
3. Confirmar que o backup é byte-idêntico ao original
   antes de qualquer escrita.

### C2 — Reescrever §3 (Estado de marcos arquitectónicos)

Verificar cada marco do snapshot face à auditoria delta §5:

- M1, M2, M3, M3 location-aware, M4, M4-residual, M5
  incremental, M5 universal, M6, M7, M9, F1, F3 parcial.
- Sub-passo final de cada marco (P163, P164, P165, P185E,
  P166, P188B, P189B, P200B, P190I, P192B, P182F, P190I).

Se a auditoria delta listar marco que o snapshot omite (ex:
M3 location-aware separado de M3, M4-residual separado de
M4), incorporar.

### C3 — Reescrever §4 (ADRs ACEITES no ciclo)

Substituir "7 ADRs ACEITES" pelo valor empírico:

- 6 ACEITES estritas: 0066, 0068, 0069, 0070, 0071, 0072.
- 2 EM VIGOR: 0064, 0065.
- 3 PROPOSTAS pendentes: 0061, 0062, 0067.
- Total: 11 ADRs novas no ciclo (0061–0072 menos 0063).

Documentar a interpretação: "ACEITE" estrita refere status
formal do ficheiro; "EM VIGOR" é categoria distinta; o
agregado de 8 só faz sentido se "ACEITE" for usado em
sentido lato.

Resolver explicitamente o caso ADR-0067 (ver C8 abaixo).

### C4 — Reescrever §5 (Estado estrutural pós-P192B)

Validar empíricamente cada item e corrigir:

- Trait `Introspector` 20 métodos: **CONFIRMADO** pela
  auditoria. Manter listagem.
- `TagIntrospector` 9 sub-stores: **CONFIRMADO**. Substituir
  "outros 2 — confirmar empíricamente em M8 diagnóstico"
  pela listagem real (auditoria §1 #10): `labels`,
  `counters`, `kind_index`, `figure_label_numbers`,
  `metadata`, `state`, `bib_store`, `resolved_labels`,
  `headings_for_toc`.
- `ElementPayload` 13 variants: validar empíricamente.
- `ElementKind` 10 variants: validar empíricamente.
- `Content` enum 13 variants: validar empíricamente.
- `LayouterRuntimeState` 3 fields: validar.
- **`Layouter` 22 fields, sem `counter`** (não 19): listar
  os 22 (auditoria §1 #11).
- Walk fn 7 parâmetros: **CONFIRMADO**. Manter.
- Helpers privados família ADR-0069: validar 3 + 1
  eliminado.
- 2 loops fixpoint: **CONFIRMADO**. Manter.
- Tests workspace: **1823 verdes** (não 1.802).
- Linter: **0 violations**. Manter.

### C5 — Reescrever §11 (Métricas cumulativas)

Substituir pelos valores da auditoria delta §6:

- 1823 tests verdes (era 1.802; baseline P156A 1145; Δ=+678).
- 70 ADRs total (61 → 70, -1 slot 0063).
- ~35 aplicações diagnóstico-primeiro (era 33; range plausível).
- LOC produção líquido: -990 série P190; -969 from_tags
  eliminado.
- Walk fn 5→7 parâmetros (P162→P191B/P190G/P190I).
- TagIntrospector 4→9 sub-stores.
- Introspector trait ~10→20 métodos.
- Layouter ~19→22 fields (com `counter` eliminado em P190I).

### C6 — Reescrever §7 (Lacunas residuais)

Substituir tabela do snapshot pela tabela real da auditoria
delta §2:

- #1 (Position) — residual; último P165.
- #1b (Position-related) — residual; depende de #1.
- #2 (Counter at locations) — parcial; último P185B / P187B+.
- #3 (headings_for_toc) — fechada P200B.
- #4 (numbering_active StyleChain-like) — fechada P182F.
- #5 (CounterRegistry hierárquico) — fechada P170.
- #6 (Bibliography full-stack) — fechada P181I.
- #7 (Outline locatable) — fechada P178.

Conclusão: apenas #1 e #1b residuais. #2 parcial. #3–#7
fechadas no ciclo.

### C7 — Reescrever §8 (M8 escopo) e §13 (resumo nova sessão)

Recalibrar com baseline correcto:

- Trait `Introspector` 20 métodos para `#[comemo::track]`.
- Layouter 22 fields como baseline (não 19); F3 parcial
  ortogonal cobre os outros 21 fields ortogonais (não 18).
- Tests baseline 1823 (não 1.802).
- Magnitude esperada permanece **L cross-modular**.

§13 (Resumo para abrir nova sessão) reescreve a frase de
abertura para Claude com os valores correctos.

### C8 — Resolver nota 8.1 (ADR-0067 status)

Investigar:

1. Ler `00_nucleo/adr/typst-adr-0067-attribute-grammar-scoping.md`
   completo.
2. Pesquisar `00_nucleo/materialization/` por referências a
   ADR-0067 (`grep -rn "0067" 00_nucleo/materialization/`).
3. Identificar em que passo a ADR-0067 foi proposta
   formalmente.
4. Identificar se algum passo a aceitou (transição PROPOSTO
   → ACEITE) ou se o consolidado P190 incorre em erro de
   status.

Decidir entre:

- **Manter PROPOSTO** — o consolidado P190 incorre em erro;
  registar correcção retroactiva no registo de reconciliação
  (Output 2).
- **Promover a ACEITE** — exige passo administrativo distinto;
  P202 não promove ADRs unilateralmente. Se for este o
  caminho, registar como recomendação para P203 e manter
  PROPOSTO em P202.

P202 não promove ADRs. Apenas regista o que está e regista
a decisão.

### C9 — Resolver nota 8.2 (ADR-0063 slot vazio)

Decidir entre:

- **Formalizar a convenção** — criar ficheiro
  `00_nucleo/adr/typst-adr-0063-RESERVADO.md` com nota:
  "slot reservado conceptualmente para column flow;
  descoberto em P160A como número 0017 ocupado". Estado:
  RESERVADO (categoria nova).
- **Apenas documentar no registo de reconciliação** sem
  criar ficheiro.

Decisão recomendada: **apenas documentar**. Criar ficheiro
ADR para um slot vazio adiciona ruído sem benefício
estrutural. Documentar em Output 2 como "convenção
implícita aceite".

### C10 — Resolver nota 8.3 (datação P157–P160)

Investigar relatórios P157, P157A-C, P158, P158A-C, P159,
P159A-G, P160 quanto a:

- Datas inferíveis por contexto (entre P156L 2026-04-26 e
  P161 2026-04-30).
- Referências cruzadas em commits Git.
- Ordenação semântica.

Decidir entre:

- **Enriquecer cabeçalhos com data inferida** — adiciona
  metadata útil ao histórico.
- **Aceitar ausência** — passos terão data marcada como
  "entre 2026-04-26 e 2026-04-30 (inferida)" no
  historiograma; relatórios originais não modificados.

Decisão recomendada: **aceitar ausência**. Modificar
relatórios anteriores retroactivamente quebra preservação
histórica. Registar a janela inferida no historiograma.

### C11 — Resolver nota 8.4 (datação anómala P190 vs P195)

Reformular a anomalia:

- P190G/H/I, P191A-C, P192A-B: 2026-05-05.
- P193–P200 (sequência §9): 2026-05-03/04.
- Mas P193+ depende semanticamente de M6 fechado em P190I.

Hipótese da auditoria: a sequência §9 foi preparada em
paralelo com M5 universal a fechar (P200B 2026-05-04), e
M6 fechou no dia seguinte (P190I 2026-05-05). Datas reflectem
ordem de publicação, não dependência semântica.

Decidir:

- **Aceitar a hipótese e documentar** — regista a distinção
  "ordem de publicação" vs "ordem de dependência semântica"
  no historiograma e no registo de reconciliação.
- **Não aceitar** — exige investigação mais profunda em
  Git log, fora do escopo deste passo.

Decisão recomendada: **aceitar e documentar**. P202 regista
a distinção; auditor humano pode validar via Git log se
quiser.

### C12 — Resolver nota 8.5 (relatórios individuais ausentes)

Lista da auditoria delta §8.5:

- P190B-H — sem relatórios; consolidado P190 absorve.
- P183B — sem relatório; P183C absorve.
- P160B — sem relatório; descartado por P161.
- P181J — relatório curto; consolidado P181 absorve.

Decidir entre:

- **Reconstruir relatórios em falta retroactivamente** —
  preserva atomicidade per ADR-0036, mas rompe a regra de
  preservação histórica de relatórios anteriores.
- **Aceitar absorção pelos consolidados** — regista a
  excepção formal no registo de reconciliação como
  "convenção de absorção pelo consolidado".

Decisão recomendada: **aceitar absorção e formalizar a
convenção**. ADR-0036 não obriga a relatório individual
quando passo é parte de série coberta por consolidado.
Documentar em Output 2 como "convenção operacional formal".

---

## §5 Outputs esperados

### Ficheiro 1 — snapshot reescrito

Localização: substituir ficheiro original (caminho a
identificar em C1).

Backup obrigatório em
`00_nucleo/snapshot-2026-05-05.pre-P202.md` antes da
escrita.

Estrutura: idêntica ao original (§1–§13). Conteúdo
actualizado para reflectir empírico:

- §3 marcos com sub-passos finais reais.
- §4 ADRs com 6 ACEITES, 2 EM VIGOR, 3 PROPOSTAS.
- §5 estado estrutural com 22 fields Layouter, 9 sub-stores
  listadas explicitamente, 1823 tests.
- §7 lacunas com #1/#1b residuais, #2 parcial, #3–#7
  fechadas.
- §8 escopo M8 recalibrado.
- §11 métricas com 1823 tests, 70 ADRs, ~35 diag-1º.
- §13 resumo nova sessão recalibrado.

### Ficheiro 2 — registo de reconciliação

Localização:
`00_nucleo/diagnosticos/typst-passo-202-reconciliacao.md`.

Estrutura:

1. Cabeçalho com escopo (4 divergências + 5 notas).
2. Tabela "questão → decisão → alteração concreta no
   snapshot → evidência".
3. Para cada item, secção breve com:
   - Estado anterior (citado do snapshot).
   - Estado real (citado da auditoria delta + verificação
     empírica adicional se necessário).
   - Decisão (qual das alternativas em C8–C12 foi escolhida).
   - Justificação (uma linha).
4. Lista de itens explicitamente fora de escopo de P202
   (ex: promoção de ADR-0067 fica para P203 se for o caso).

### Ficheiro 3 — relatório do passo

Localização:
`00_nucleo/materialization/typst-passo-202-relatorio.md`.

Mesmo formato do relatório de P201:

- O que foi feito.
- Tempo de execução.
- Decisões tomadas durante a leitura.
- Sugestões para o próximo passo (não-vinculativas).

---

## §6 Critério de progressão

P202 está concluído quando:

- Os 3 ficheiros existem.
- Backup do snapshot anterior existe e é byte-idêntico ao
  original.
- C1–C12 todos endereçados no registo de reconciliação.
- 4 divergências (§1 da auditoria delta) reflectidas no
  snapshot reescrito.
- 5 notas de auditor (§8) com decisão registada.
- Snapshot reescrito é internamente consistente
  (§3↔§4↔§5↔§11 sem contradição).

Após P202 concluído, o **próximo passo** (P203) pode ser:

- P203A — diagnóstico-primeiro de M8 (caminho default).
- P203 administrativo — promoção formal de ADR-0067 se C8
  recomendar.
- Outro caminho informado pelo que P202 revelar durante a
  reescrita.

P202 não decide o próximo passo. Reporta.

---

## §7 Convenções mantidas

- Sem código Rust.
- Sem condicionais em sub-passos (este passo não tem
  sub-passos).
- 3 outputs padrão.
- Distinção fecho estrutural vs arquitectural mantida.
- Preservação histórica: backup obrigatório do snapshot
  anterior antes de qualquer escrita.
- Sem inflação: sem "patamar", sem "limiar", sem
  "consolidação", sem "deriva", sem "subpadrão", sem
  "cumulativo", sem "cross-domínio", sem "paridade
  observable" como bandeira retórica.

---

## §8 Não-objectivos

P202 não:

- Decide o caminho de M8.
- Propõe ADR-0073.
- Toca em código.
- Promove ADRs (ADR-0067 mantém o status que tem; promoção
  é trabalho de outro passo se necessário).
- Cria ADRs novas (ADR-0063 não é criado mesmo que C9 fosse
  pela alternativa de formalização — recomendação contrária
  em C9).
- Modifica relatórios de passos anteriores retroactivamente
  (C10, C12).
- Pré-define sub-passos de M8.

---

## §9 Erro a não repetir

A spec de P202 segue o mesmo princípio de P201: passo
administrativo único, sem sub-passos pré-definidos. C1–C12
são cláusulas de execução do mesmo passo, não sub-passos
encadeados.

P203 (próximo passo de feature ou administrativo) só é
fixado depois de P202 produzir os 3 outputs.
