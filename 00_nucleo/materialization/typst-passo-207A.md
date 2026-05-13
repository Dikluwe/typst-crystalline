# Passo 207A — Diagnóstico-primeiro do gap Introspector cristalino vs vanilla

**Série**: 207 (sub-passo `A` = diagnóstico-primeiro
formal). 40ª aplicação consecutiva do padrão.
**Tipo**: diagnóstico-primeiro de profundidade alta
(zero código tocado) + auditoria empírica cross-modular.
**Magnitude planeada**: M (M auditoria + S diagnóstico)
com ressalva L se escopo amplo (Introspector +
sub-stores + consumers) revelar gap extenso.
**Pré-condição**: P206E concluído; trajectória M8 + F3
+ vanilla integration fechada; ADR-0073 ACEITE completo
retroactivo; ADR-0074 ACEITE final; ADR-0075 ACEITE
final; DEBT-53 CLOSED; DEBT-54 OBSOLETED; tests workspace
cristalino 1873 verdes; tests `lab/parity` 75 verdes; 0
violations; blueprint anotado §3.0/§3.0bis/§3.0ter;
vanilla typst v0.14.2 disponível em
`lab/typst-original/` (quarentena) + `/usr/local/bin/typst`
(CLI ambiental).
**Output**: 4 ficheiros (auditoria + diagnóstico +
relatório + ADR proposto se C9 for afirmativa).

---

## §1 Propósito

Fixar empíricamente o gap entre o Introspector
cristalino actual e o vanilla v0.14.2 (quarentena) antes
de qualquer decisão de "completar".

A clarificação inicial fixou:

- **Escopo amplo**: Introspector + sub-stores
  associados + consumers (stdlib `here()` / `locate()`,
  counter/state, outline, etc.).
- **Referência vanilla**: versão em quarentena
  `lab/typst-original` (v0.14.2) — material empírico
  fixado.

P207A produz:

1. Mapeamento empírico literal do gap em 3 dimensões
   (trait + sub-stores + consumers).
2. Classificação do gap por categoria (paridade
   literal / divergência arquitectónica / extensão
   necessária / decisão pendente).
3. Cláusulas de decisão (C1–Cn) sem condicionais.
4. ADR proposta **se C9 for afirmativa** (provável dado
   escopo amplo).
5. Plano de sub-passos `*B+` ou sub-séries dependendo
   da magnitude.
6. Nome do marco arquitectónico fixado em C12 (M9, F4,
   ou outra etiqueta).

P207A respeita o padrão: inventário empírico antes de
qualquer decisão. **Não decide o trabalho de completar
— audita o gap**.

---

## §2 Tensão consciente entre os inputs

A clarificação inicial fixou:

- Escopo amplo (3 dimensões).
- Referência vanilla literal (quarentena).

A tensão a registar:

- **Escopo amplo multiplica volume de auditoria**.
  Trait Introspector (20 métodos per P204B) + 9
  sub-stores conhecidos + consumers (stdlib, counter,
  state, outline, e potencialmente outros) implica
  varrer múltiplas árvores: `01_core/src/entities/`,
  `01_core/src/rules/`, `02_shell/`, `03_infra/`,
  `lab/typst-original/crates/typst-library/src/introspection/`.
- **Referência vanilla literal exige inventário
  paralelo do código vanilla** — não apenas leitura do
  trait `Introspector` vanilla, mas de impls
  (`PagedIntrospector`, `HtmlIntrospector` se existir)
  e dos consumers vanilla equivalentes.
- **Magnitude pode aproximar-se de L** dado o escopo.
  P206A C10 fixou M agregado para a série P206; P207
  pode ter escopo maior.

P207A resolve assim:

- C1–C3 mapeia empíricamente trait + sub-stores +
  consumers.
- C4 classifica gap por categoria.
- C5 estima magnitude do trabalho de "completar" com
  base em C4.
- C6 decide se P207 é série única ou se exige sub-séries
  (P208/P209/...).
- C7 fixa nome do marco arquitectónico.
- C12 registra `P207A.div-N` se escopo amplo revelar
  inviabilidade ou exigir redução fundamentada.

Pré-fixação do escopo é guidance, não constrangimento
absoluto. Se a auditoria mostrar que escopo amplo é XL
sem benefício proporcional, C12 legitima recomendação
de redução com fundamento empírico.

---

## §3 Cláusulas de auditoria (A1–An)

Esta secção é executada **primeiro**. Output empírico
alimenta C1+ adiante. Cada item reporta CONFIRMADO /
DIVERGÊNCIA / NÃO APLICÁVEL com evidência.

### Bloco 1 — Trait Introspector cristalino vs vanilla

#### A1 — Trait cristalino actual

Listar literalmente:

- Caminho exacto (esperado:
  `01_core/src/entities/introspector.rs`).
- Lista dos 20 métodos (per P204B A1).
- Bounds (`#[comemo::track] + Send + Sync`).
- Impl(s) existentes (`TagIntrospector` + wrappers de
  teste).
- Assinaturas exactas de cada método.

#### A2 — Trait vanilla actual

Listar literalmente:

- Caminho exacto em `lab/typst-original/crates/typst-library/src/introspection/introspector.rs`
  (per P205D D3 + P206C C1.1).
- Lista de **todos** os métodos (heading-level).
- Assinaturas exactas.
- Impls vanilla (esperado: `PagedIntrospector`,
  possíveis outros).

#### A3 — Comparação trait-a-trait

Tabela 1: cada método cristalino → equivalente vanilla
(ou NÃO EQUIVALENTE).
Tabela 2: cada método vanilla → equivalente cristalino
(ou NÃO EQUIVALENTE).
Tabela 3: divergências de assinatura (mesmo nome, tipo
diferente).

Critério: tabelas literais; sem inflar.

### Bloco 2 — Sub-stores associados

#### A4 — Sub-stores cristalinos

Per CLAUDE.md + P204+P205 trajectória:

- `LabelRegistry` (cristalino, vivo desde Passo XXX).
- `MetadataStore`.
- `BibStore`.
- `StateRegistry`.
- `CounterRegistry`.
- `ResolvedLabelStore`.
- `SealedPositions` (P205B).
- 2-3 outros (confirmar empíricamente).

Para cada um: caminho, fields, methods públicos,
consumers actuais.

#### A5 — Sub-stores vanilla

Auditar `lab/typst-original/crates/typst-library/src/introspection/`:

- Listar sub-stores equivalentes (se existirem).
- Identificar tipos vanilla análogos.
- Notar arquitectura diferente onde aplicável (per
  `P205A.div-1` — vanilla tem assimetria
  Engine + Layouter especializados).

#### A6 — Comparação sub-stores

Tabela: cristalino → vanilla (ou inexistente). Tabela
inversa: vanilla → cristalino (ou inexistente). Notar
divergências arquitectónicas legítimas (ex: vanilla
multi-impl Introspector vs cristalino única).

### Bloco 3 — Consumers (stdlib + rules)

#### A7 — `here()` / `locate()` em stdlib cristalino

Per P204F SKIP `here-locate.typ`:

- Confirmar empíricamente que ambas estão ausentes.
- Localizar o ponto onde existiriam (esperado em
  `01_core/src/stdlib/` ou similar).
- Vanilla: caminho exacto + assinatura + comportamento.

#### A8 — Counter / State

- `counter(...)` em cristalino: estado actual.
- `counter(...)` em vanilla: implementação completa.
- `state(...)` em cristalino vs vanilla.
- Outros consumers de `Introspector::query`,
  `position_of`, `state`.

#### A9 — Outline

- `outline()` em cristalino: estado actual (per P200
  série + P206D D5).
- `outline()` em vanilla.
- Comparação literal incluindo as 3 divergências
  arquitectónicas P206C (outline-toc count diff).

#### A10 — Bibliography

- `bibliography()` / `cite()` em cristalino: estado
  parcial (per P206C divergência `cite-bibliography.typ`
  + P181 series).
- Vanilla full impl.

#### A11 — Outros consumers

- Page-relevantes (`page(loc)`, `pages(loc)`,
  `page_numbering`, `page_supplement`) — per P205D D3
  inexistentes em cristalino.
- `label_count` — per P205D D3 inexistente em
  cristalino.
- Outros métodos vanilla expostos ao stdlib mas não
  ainda em cristalino.

### Bloco 4 — Selector enum + parsing

#### A12 — `Selector` cristalino

Per P206C C1.4:

- Enum com **apenas** `Kind(ElementKind)` (P175
  minimal).
- Sem `Label`, `Where`, `And`, `Or`.
- Parsing standalone (text → Selector) **inexistente**.

#### A13 — `Selector` vanilla

- Enum completo (esperado: Label / Kind /
  WhereSelector / And / Or / Before / After / etc.).
- Parsing standalone via syntax `<>` ou similar.

#### A14 — Gap concreto

Comparação literal. Implicações para queries complexas.

### Bloco 5 — Classificação do gap

#### A15 — Categorias

Cada item da auditoria A1–A14 cai em uma de 4
categorias:

- **PARIDADE LITERAL** — cristalino tem equivalente
  vanilla; sem trabalho.
- **DIVERGÊNCIA ARQUITECTÓNICA LEGÍTIMA** — cristalino
  e vanilla divergem intencionalmente; sem trabalho a
  fazer.
- **EXTENSÃO NECESSÁRIA** — vanilla tem; cristalino
  não tem; trabalho a fazer para "completar".
- **DECISÃO PENDENTE** — não claro se é divergência
  legítima ou extensão necessária; humano decide.

Output: tabela com todos os itens classificados +
contagem por categoria.

#### A16 — Magnitude estimada de "completar"

Para cada item EXTENSÃO NECESSÁRIA:

- Custo estimado (S/M/L/XL).
- Dependências (sub-stores prévios necessários).
- Bloqueios (ex: stdlib precisa expandir-se primeiro).

Output: lista ordenada por dependência + magnitude
agregada.

---

## §4 Cláusulas de decisão (C1–Cn)

Estas cláusulas são fixadas **depois** da auditoria,
com base no output empírico. Cada uma é fixada sem
condicionais.

### C1 — Trait gaps prioritários

Lista literal dos métodos trait EXTENSÃO NECESSÁRIA
ordenados por prioridade (com base em A11 + A16).

Hipótese específica per P205D D3: métodos
page-relevantes (`page(loc)`, `pages(loc)`,
`page_numbering`, `page_supplement`) + `label_count`
são EXTENSÃO NECESSÁRIA prováveis.

### C2 — Sub-stores gaps prioritários

Lista literal dos sub-stores EXTENSÃO NECESSÁRIA.

Hipótese específica per P205D: `SealedLabelPages` foi
deferido em P205D. P207A pode reabrir a decisão se
consumers reais aparecerem.

### C3 — Consumers prioritários

Lista literal dos consumers EXTENSÃO NECESSÁRIA com
dependências.

Hipótese específica: `here()` / `locate()` desbloqueia
consumer real para `position_of` (per P206C D9
expectativa) + Selector::Label extension.

### C4 — Selector enum extensions

Decisão sobre `Selector::Label`, `Where`, `And`, `Or`:

- Materializar todos.
- Materializar minimal subset (Label).
- Adiar (manter P175 minimal).

C4 fixa **uma**.

### C5 — Stdlib expansion

Decisão sobre `here()` / `locate()`:

- Materializar como parte de P207.
- Materializar em série dedicada P208.
- Adiar (gap conhecido; consumer real ainda inexistente).

C5 fixa **uma**.

### C6 — Estrutura da trajectória

Decisão fixada com base em A15 + A16:

- **Caminho 1 — Série única P207A–E** — se magnitude
  agregada ≤ M.
- **Caminho 2 — Sub-séries dedicadas** — se magnitude
  agregada > M; ex: P207 trait extensions; P208 stdlib;
  P209 selector enum.
- **Caminho 3 — Marco arquitectónico inteiro** — se
  magnitude agregada > L; criar marco novo (M9 ou
  similar) com múltiplas séries.

C6 fixa **uma**.

### C7 — Nome do marco arquitectónico

Fixar etiqueta com base em padrão do projecto:

- M9 (continuação numérica de M8).
- F4 (continuação de F3 = refactor).
- Outra (per convenção).

Critério: examinar blueprint + ADRs anteriores para
fixar padrão.

### C8 — Magnitude agregada e orçamento

Output: estimativa total de "completar Introspector"
com base em A15 + A16 + C4 + C5 + C6.

Range possível: M (escopo reduzido) a XL (escopo
completo).

### C9 — ADR proposta?

Decisão: criar ADR nova ou estender ADR existente:

- Sim — "completar Introspector" é decisão arquitectural
  com alternativas reais. Cada marco até agora (M7,
  M8, F3, vanilla integration) ganhou ADR dedicada.
- Não — auditoria documenta sem precisar de decisão
  arquitectural separada.

Hipótese provável: criar ADR (paralelo a 0072 M7,
0073 M8, 0074 F3, 0075 vanilla integration).

C9 fixa **uma**.

### C10 — Decisões pendentes

Lista literal de itens DECISÃO PENDENTE (A15) que
exigem clarificação humana antes de P207B prosseguir.

Output: questões formuladas para humano.

### C11 — Sub-passos `*B+` (ou plano de série)

Plano sem ramos. Quantidade e nomenclatura dependem
de C6.

### C12 — Possível `P207A.div-N` sobre escopo

Se A15 + A16 + C8 mostrarem que escopo amplo é
inviável (XL+ sem benefício proporcional), registar
`P207A.div-N`:

- Recomendar redução de escopo (ex: trait apenas, sem
  consumers).
- Documentar fundamento empírico.
- Solicitar decisão ao humano antes de prosseguir
  P207B.

Pré-fixação não absorve obrigação de inflar quando
empírico mostra inviabilidade.

### C13 — Sem cláusulas condicionais

C1–C12 fixadas com valores concretos.

---

## §5 Outputs concretos

Quatro ficheiros (3 sempre + 1 condicional em C9):

### Ficheiro 1 — Auditoria empírica

Localização:
`00_nucleo/diagnosticos/typst-passo-207A-auditoria-introspector.md`.

Conteúdo: A1–A16 com tabelas literais.

### Ficheiro 2 — Diagnóstico

Localização:
`00_nucleo/diagnosticos/typst-passo-207A-diagnostico.md`.

Conteúdo: C1–C13 fixadas; questões para humano (C10);
plano `*B+` ou sub-séries.

### Ficheiro 3 — Relatório

Localização:
`00_nucleo/materialization/typst-passo-207A-relatorio.md`.

### Ficheiro 4 — ADR PROPOSTO (condicional em C9)

Localização (se C9 = afirmativa):
`00_nucleo/adr/typst-adr-0076-introspector-completion.md`
(ou nome fixado em C7).

Estado: PROPOSTO. Estrutura per `template-adr.md`.

---

## §6 Critério de progressão para `*B`

P207A só transita para `*B` quando:

- A1–A16 todos com etiqueta CONFIRMADO ou DIVERGÊNCIA.
- C1–C13 instanciadas com valores concretos.
- ADR PROPOSTO escrito (se C9 afirmativa).
- Magnitude calibrada (C8).
- Plano `*B+` sem condicionais.
- Decisões pendentes (C10) **respondidas pelo humano**
  antes de P207B começar.

Em caso de divergência empírica relevante, registar em
`P207A.div-N` e:

- Resolver dentro de P207A se trivial.
- Recuar para humano se afecta escopo (C12).

---

## §7 Convenções mantidas

- Sem código nas specs.
- Sem condicionais.
- Cada passo começa com inventário empírico.
- 4 outputs (3 sempre + 1 condicional).
- Localização canónica.
- Sem inflação retórica.

---

## §8 Não-objectivos

P207A não:

- Toca em código.
- Materializa qualquer método trait.
- Estende `Selector` enum.
- Implementa stdlib `here()` / `locate()`.
- Modifica sub-stores existentes.
- Cria sub-stores novos.
- Decide trabalho de "completar" — apenas audita o
  gap.
- Materializa CLI subcomando deferred (P206C.div-1).
- Endereça outras pendências fora do escopo
  Introspector.
- Modifica ADRs já ACEITES (0073/0074/0075) excepto
  por cross-reference se necessário.
- Reescreve consolidados anteriores.

---

## §9 Erro a não repetir

Da série P204A + P206A — pattern empírico de
diagnóstico-primeiro: inventário antes de decisão;
classificação rigorosa em categorias.

Risco específico de P207A: **assumir que "vanilla é a
verdade canónica" e classificar tudo como EXTENSÃO
NECESSÁRIA**. Per `P205A.div-1` + P206C — cristalino e
vanilla têm divergências arquitectónicas legítimas.
Algumas omissões cristalinas são deliberadas (single-pass
vs assimetria vanilla; `FixedMetrics` vs
`FontBookMetrics`; etc.). C4 (categoria DIVERGÊNCIA
ARQUITECTÓNICA LEGÍTIMA) protege contra inflação.

Outro risco: **inflar escopo para "completar tudo"**.
Pré-fixação foi "escopo amplo" — mas se A16 mostrar
custo XL+ sem benefício proporcional (ex: 80% de
extensões sem consumers reais), C12 legitima
recomendação de redução.

Outro risco: **ignorar que alguns "gaps" são DEBT-X
pre-existing** que já têm planos próprios (ex:
bibliography stdlib é DEBT-X separado per P181;
outline-toc divergência é design intencional per
P200/P206D D5). C4 deve distinguir gaps Introspector
de gaps de outros sistemas.

Hipótese mais provável: A15 produz mix de categorias:
~30% PARIDADE LITERAL + ~25% DIVERGÊNCIA ARQUITECTÓNICA
+ ~35% EXTENSÃO NECESSÁRIA + ~10% DECISÃO PENDENTE.
Magnitude agregada provável: L (com sub-séries) ou XL
(se completar tudo).

Hipótese específica: C6 = Caminho 2 ou 3 — trabalho é
provavelmente maior que série única.

Mas são hipóteses, não decisões. C1–C13 fixam-se com
base em A1–A16.

---

## §10 Particularidade — execução

P207A é diagnóstico de profundidade alta com escopo
amplo:

- Trait Introspector cristalino (20 métodos).
- Trait Introspector vanilla (heading-level: 20+
  métodos esperados).
- Sub-stores cristalinos (~9 catalogados).
- Sub-stores vanilla (a confirmar).
- Consumers: stdlib + counter/state + outline +
  bibliography + page-relevantes + outros.
- Selector enum (ambos lados).

Volume **alto**. Magnitude M (auditoria) + S
(diagnóstico), com ressalva L se gap revelar-se
extenso.

Recomendado Claude Code dado:

- Volume de leitura para A1–A14 (múltiplas árvores
  cristalino + vanilla quarentena).
- Necessidade de grep cross-modular.
- Decisão arquitectural em C6 + C7 que beneficia de
  comparação detalhada.

Sessão actual viável apenas com tempo significativo.
Caso contrário, Claude Code é fortemente preferido.
