# Passo P160A — Criar ADR-0017 PROPOSTO (administrativo XS)

Sub-passo administrativo XS de P160 (diagnóstico Introspection)
para formalizar reserva pré-existente de ADR-0017 (Introspection
runtime adiada) como ficheiro ADR com status `PROPOSTO`. **Não
materializa código**. **Não promove a EM VIGOR ou IMPLEMENTADO**
— apenas formaliza a reserva como documento real para que
decisões futuras (P160B state runtime + restantes Bloco B) possam
referenciar ADR concreta.

**Paridade estrutural com `ADR-0062-create`** (passo administrativo
XS análogo pendente desde P158B). Subpadrão emergente "passo
administrativo XS criar ADR PROPOSTO a partir de reserva
pré-existente" — segundo do tipo se ADR-0062-create for executada
antes; primeiro do tipo se P160A for primeiro.

**P160A é primeiro sub-passo de P160** com identificador letrado.
Substantivos Bloco B seguem: P160B (state runtime), P160C
(metadata), P160D (here/locate), P160E (query), P160F (position)
— per recomendação P160 §6 com identificadores deslocados +1
letra (era P160A-E no relatório; agora P160B-F).

---

## Estado actual antes de começar

- 63 ADRs após P160 (28 EM VIGOR; ADR-0060 IMPLEMENTADO;
  **ADR-0017 reserva sem ficheiro confirmada em P160 §1**;
  ADR-0062 reserva sem ficheiro mantida — ADR-0062-create
  ainda pendente).
- Layout: 78%. Model agregado: ~50%. **Introspection: 17%
  saturada por tecto puro** (per P160 §4).
- Hash actual `entities/content.rs`: `ec58d849` (preservado em
  **18 passos consecutivos** P156L → P160 via L0-baseline).
- 1501 tests workspace; zero violations linter.
- Padrões consolidados pós-P160: granularidade N=21;
  inventariar N=24; §análise risco N=24; estabilidade hash
  L0 content.rs N=18.

**Reserva ADR-0017 confirmada em P160 §1**:
> ADR-0017 confirmada como reserva sem ficheiro pré-existente
> ("Introspection runtime adiada"). Sem ficheiro
> `00_nucleo/adr/typst-adr-0017-*.md`. Confirmações cumulativas:
> inventário 148 §A.9 + P159B §3 categoria A + P159A
> cross-reference adiada.

**Não confirmado** (a confirmar em .1):
- Conteúdo concreto da reserva (apenas título; ou já tem
  esboço documentado algures?).
- Que precedentes são citáveis (P160 §1 menciona reserva sem
  precedentes formais; mas pode haver convenções).
- Convenção de naming exacta do ficheiro ADR
  (`typst-adr-0017-introspection-runtime-adiada.md`?).

**Política "sem novas reservas" preservada** — este passo NÃO
cria reservas adicionais. Apenas materializa documento ADR
para reserva pré-existente (paridade ADR-0062-create).

---

## Natureza do passo

**Tamanho**: XS.

**Justificação**: Trabalho documental puro. Criar 1 ficheiro
ADR com status PROPOSTO. Sem modificação de código. Sem
modificação de outras ADRs. Sem promoção a EM VIGOR ou
IMPLEMENTADO (essa decisão fica para passo futuro de
materialização Introspection runtime — P160B ou equivalente).

Granularidade preservada: 1 deliverable documental → mantém
peso XS análogo a ADR-0062-create.

**Risco muito baixo**: passo é puramente declarativo. Conteúdo
do ADR pode evoluir antes de promoção (PROPOSTO → EM VIGOR).

---

## Decisões já tomadas

- **Status inicial**: `PROPOSTO`. NÃO `EM VIGOR` nem
  `IMPLEMENTADO` — porque nenhum código de Introspection
  runtime foi ainda integrado. PROPOSTO é o status correcto
  para decisão tomada mas não em vigor (paridade ADR-0062-create).

- **Scope**: Introspection runtime — counter() runtime queries,
  state() mutable runtime state, locate()/here() position-aware,
  query() runtime introspection, metadata() arbitrary attaching,
  position() location-aware. **Conjunto de features que exigem
  multi-pass ou runtime queries genuínas** vs single-pass
  current.

- **Justificação técnica primária**: Introspection vanilla é
  fundamentalmente runtime/multi-pass; cristalino actual é
  single-pass per ADR-0033 minimal. Promover Introspection
  runtime a PROPOSTO formaliza a decisão de extender pipeline
  cristalino para suportar runtime queries.

- **Sem código**: este passo NÃO modifica `introspect.rs` nem
  adiciona pipeline runtime. Materialização real fica para
  passo futuro pós-promoção (P160B state primeiro candidato
  Bloco B substantivo).

- **Sem nova reserva**: passo formaliza reserva pré-existente,
  não cria reserva nova.

- **Identificador P160A**: sub-passo administrativo de P160
  per decisão humana pós-P160. Substantivos Bloco B deslocados
  +1 letra (era P160A-E no relatório P160; agora P160B-F).

## Decisões diferidas

- **Pipeline runtime vs multi-pass refactor**: a decidir em
  passo de materialização real (P160B ou equivalente).
- **Subset de features Introspection runtime** a implementar
  primeiro: diferida para passo de materialização (P160B
  state subset minimal recomendado por P160 §6).
- **Cross-document refs** (cite cross-document family 159):
  diferida; depende multi-document pipeline além de runtime.
- **Promoção a EM VIGOR/IMPLEMENTADO**: passo separado futuro
  (P160B materialização ou equivalente).

---

## Sub-passos

### .1 Inventário pré-redacção (obrigatório per ADR-0065)

Localizar e ler:
- `00_nucleo/adr/README.md` — confirmar convenção de naming
  exacta (paridade ADR-0062-create esperado se já executada).
- ADR-0033 paridade observable — citar como contexto para
  divergência single-pass vs vanilla multi-pass.
- ADR-0054 graded — citar como fundamento para subset minimal
  Introspection.
- ADR-0017 reserva sem ficheiro — confirmar estado actual em
  README e relatórios (P160 §1 mencionou).
- P160 §3-§6 — citar como ponto de partida para subset
  minimal pós-promoção.
- DEBT-10 antigo — confirmar se foi fechado (P160 §1 mencionou
  "comentário antigo já parcial cumprido via materialize_time").

Output: notas internas (não documentadas em .md à parte; este
é XS administrativo).

### .2 Redigir ADR-0017

Ficheiro novo:
`00_nucleo/adr/typst-adr-0017-introspection-runtime-adiada.md`
(naming a confirmar em .1; paridade convenção ADR-0062-create
se já executada).

Conteúdo:
- **Status**: `PROPOSTO`.
- **Data**: data actual.
- **Contexto**:
  - Cristalino actual é single-pass per ADR-0033 minimal.
  - Vanilla Introspection é fundamentalmente runtime/multi-pass
    (counter() runtime queries; state() mutable; query()
    runtime introspection; etc.).
  - P160 confirmou que counter() já cobre o atingível sem
    promoção (cobertura ~17% saturada por tecto puro).
  - 11 features Introspection vanilla bloqueadas por esta
    reserva (per P160 §3 categorização).
- **Decisão**: promover Introspection runtime a PROPOSTO.
  Subset minimal pós-promoção: state runtime + metadata +
  here()/locate() + query() (per P160 §6 recomendação
  pós-promoção). Status PROPOSTO até passo de materialização
  real (P160B ou equivalente).
- **Consequências**:
  - Positivas: paridade observable mais ampla com vanilla
    (Introspection 17% → ~50% pós-Bloco B subset minimal);
    desbloqueia features família 159 cross-document;
    desbloqueia counters refinados.
  - Negativas: complexidade de pipeline aumenta (2-pass
    runtime queries); divergência arquitectural vs single-pass
    actual; superfície de testes cresce significativamente.
- **Alternativas consideradas**:
  - Manter single-pass com features observable limitadas —
    rejeitada porque tecto saturado em ~17% per P160 §4.
  - Implementar Introspection runtime cristalino do zero
    sem ADR formal — rejeitada porque magnitude da decisão
    arquitectural exige formalização.
  - Adoptar pipeline vanilla integralmente — rejeitada por
    desproporcionalidade vs subset minimal.
- **Referências**:
  - ADR-0033 (paridade observable; fundamento single-pass
    actual).
  - ADR-0054 (graded; fundamento subset minimal).
  - ADR-0034 (estrutura diagnóstico canónica).
  - ADR-0060 (Model roadmap; menção Introspection futura).
  - ADR-0061 (Layout roadmap; depende measure que depende
    Introspection cross-módulo).
  - ADR-0062 (paridade administrativa hayagriva; XS análogo
    se já executado ADR-0062-create).
  - P160 relatório (Introspection diagnóstico; tecto
    saturado).
  - P159B §3 categoria A (features bloqueadas por ADR-0017).

### .3 Actualizar README ADRs

`00_nucleo/adr/README.md`:
- Substituir entrada "ADR-0017 reservada Introspection runtime"
  por entrada concreta com link para ficheiro novo.
- Status PROPOSTO documentado.
- Contagem ADRs total: 63 → 64 (ou 65 se ADR-0062-create
  executado entre P160 e este passo; verificar em .1).

### .4 Actualizar contagem em ADR-0061 §"Aplicações cumulativas"

ADR-0061 §"Aplicações cumulativas":
- Linha cronológica nova para `P160A` (com nota "criar
  ADR-0017 PROPOSTO; XS administrativo") com slope "—" e
  tests Δ "0".
- Padrões metodológicos: inventariar-primeiro N=24 → 25
  (ADR-0065 critério #5 inventário trivial + critério #1
  naming convention).
- §análise de risco N=24 → 25 (passo administrativo XS).

### .5 Verificação documental

- Linter markdown se aplicável.
- Verificar links cruzados (ADR-0033, ADR-0054, ADR-0034,
  ADR-0060, ADR-0061, ADR-0062, P160, P159B).

---

## Verificação

Numerada para reporte de conclusão:

1. Ficheiro ADR-0017 criado em
   `00_nucleo/adr/typst-adr-0017-*.md` com naming confirmado
   em .1.
2. Status `PROPOSTO` documentado.
3. Estrutura canónica seguida (Status / Data / Contexto /
   Decisão / Consequências / Alternativas / Referências).
4. ADRs precedentes (0033/0054/0034/0060/0061) citados como
   contexto.
5. P160 + P159B referenciados como base factual.
6. README ADRs actualizado: entrada "reservada Introspection
   runtime" substituída por entrada concreta com link;
   contagem total **64** (era 63) ou **65** se
   ADR-0062-create já executada (verificar em .1).
7. ADR-0061 §"Aplicações cumulativas" actualizada com linha
   `P160A`.
8. **Sem código alterado** — `entities/content.rs` mantém
   `ec58d849` (19º passo consecutivo).
9. **Sem novas reservas** criadas (paridade política
   P158/P159).
10. ADR-0017 NÃO promovida a EM VIGOR/IMPLEMENTADO neste
    passo — promoção fica para passo futuro de materialização
    Introspection runtime real (P160B subset minimal).
11. `crystalline-lint`: zero violations.

---

## Critério de conclusão

- Verificações 1-11 passam.
- Relatório separado em
  `00_nucleo/materialization/typst-passo-160a-relatorio.md`
  produzido com:
  - Resumo do executado.
  - Confirmação de cada verificação.
  - **§análise de risco** (padrão N=24 → 25; segundo passo
    administrativo XS criar ADR PROPOSTO a partir de reserva
    pré-existente; subpadrão emergente N=2 se ADR-0062-create
    já executada, ou N=1 se for primeiro do tipo).
  - Confirmação: contagem ADRs incrementada; ADR-0017
    PROPOSTO formalizado.
  - **Implicação**: P160B (state runtime) e restantes Bloco B
    do diagnóstico P160 agora podem iniciar com referência
    concreta a ADR-0017 (em vez de referência a reserva sem
    ficheiro).

---

## O que pode sair errado

**Cenários gerais**:
- Inventário .1 revela que ADR-0033/0054/0034/0060/0061 NÃO
  têm contexto directamente útil — procurar precedentes mais
  específicos em ADRs de pipeline ou observability; documentar.
- Convenção de naming ter mudado desde reservas pré-existentes
  → ajustar naming do ficheiro novo per convenção actual.

**Cenários específicos**:
- Conteúdo concreto da reserva ADR-0017 já estar documentado
  em algum lugar (e.g. comentário em introspect.rs, nota em
  diagnóstico antigo) → reusar conteúdo; documentar fonte
  (P160 §1 não detectou; mas .1 pode procurar mais a fundo).
- DEBT-10 ainda activo (P160 §1 disse "fechado" mas pode ser
  parcialmente activo) → documentar estado factual em .1.
- Decisão de Status ser ambígua (PROPOSTO vs IDEIA vs ADIADO)
  → manter PROPOSTO per pré-decisão; alterar só se inventário
  .1 revelar que IDEIA/ADIADO é mais preciso.
- ADR-0062-create já executada entre P160 e este passo → ajustar
  contagem ADRs total (63 → 64 → 65 cumulativo) e referenciar
  ADR-0062 PROPOSTO concretamente em alternativas/referências.
  Confirmar em .1.

---

## Notas operacionais

- **Passo administrativo XS**. Custo muito baixo; benefício
  claro (Bloco B Introspection desbloqueado para futuro com
  referência concreta).
- **Identificador P160A**: sub-passo administrativo de P160.
  Substantivos Bloco B deslocados +1 letra: P160B (state),
  P160C (metadata), P160D (here/locate), P160E (query), P160F
  (position).
- **Independente de P160B**. Pode ser executado antes,
  desbloqueando Bloco B. Sem dependência mútua além de
  ADR-0017 PROPOSTO ser pré-condição declarativa para
  materialização.
- **Status PROPOSTO é deliberado**. NÃO promover a EM VIGOR
  ou IMPLEMENTADO neste passo — essa promoção é decisão
  arquitectural maior que fica para passo de materialização
  Introspection runtime real (P160B subset minimal).
- **Contagem ADRs**: 63 → 64 (ou 65 se ADR-0062-create já
  executada).
- **Política "sem novas reservas" preservada** — este passo
  formaliza reserva pré-existente, não cria nova. Não viola
  política. Paridade ADR-0062-create.
- **Subpadrão emergente** "passo administrativo XS criar ADR
  PROPOSTO a partir de reserva pré-existente":
  - Se ADR-0062-create já executada antes de P160A: subpadrão
    N=1 → 2 (ADR-0062-create + P160A). Atinge meio-caminho
    limiar formalização N=3-4.
  - Se P160A executada primeiro: subpadrão N=1 primeiro do
    tipo.
  - Confirmar em .1 estado factual.

---

## Pós-passo

Após conclusão de P160A:

**Layout fica em 78% inalterado**. **Model fica em ~50%
inalterado**. **Introspection fica em 17% (saturada por tecto
puro)**. **ADRs total cresce 63 → 64 (ou 65)**.

**ADR-0017 PROPOSTO** disponível para referência em passos
futuros de Bloco B (P160B state; P160C metadata; P160D
here/locate; P160E query; P160F position).

**Próxima decisão** (sem candidata pré-acordada — política
"sem novas reservas"):
- Iniciar Bloco B com **P160B** (state runtime; M; +6-8pp
  Introspection) — primeiro candidato per recomendação P160 §6
  pós-promoção.
- ADR-0062-create se ainda pendente — outro XS administrativo
  paridade.
- Conjunto administrativo XS — promoções acumuladas
  (`optional_str` N=12; ADR meta subpadrão #15/#16; L0 content.md
  update).
- Mudar de módulo (Layout Fase 3 columns/colbreak ou outro).

ADR-0060 mantém-se `IMPLEMENTADO`. ADR-0061 mantém-se
`PROPOSTO`. **ADR-0017 PROPOSTO** (era reserva). ADR-0062
mantém-se conforme estado actual (PROPOSTO se ADR-0062-create
executada; reserva sem ficheiro caso contrário).

Padrão granularidade 1-2 features/passo (N=21) NÃO é desafiado
por este passo (administrativo XS; sem materialização).

**Reservas pendentes** (não criadas neste passo):
- ADR-0062 hayagriva — pré-existente; pendente até ADR-0062-create
  ser executada.

**Pausa natural após P160A — Bloco B Introspection desbloqueado;
política "sem novas reservas" preservada (formaliza pré-existente);
contagem ADRs cresce; subpadrão emergente "passo administrativo
XS criar ADR PROPOSTO" pode crescer N=1→2 se ADR-0062-create
já executada. Decisão humana sobre próxima direcção tem máxima
informação.**
