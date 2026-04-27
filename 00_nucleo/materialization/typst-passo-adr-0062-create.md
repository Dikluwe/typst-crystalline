# Passo ADR-0062-create — Criar ADR-0062 PROPOSTO (administrativo XS)

Passo administrativo XS para formalizar reserva pré-existente
de ADR-0062 (autorização de crate `hayagriva`) como ficheiro
ADR com status `PROPOSTO`. **Não materializa código**. **Não
promove a EM VIGOR ou IMPLEMENTADO** — apenas formaliza a
reserva como documento real para que decisões futuras (Bloco B
do diagnóstico P159B) possam referenciar ADR concreta.

**Independente de P158B** — pode ser executado antes, depois,
ou em paralelo. Sem dependência mútua.

**Identificador provisório**: `ADR-0062-create` ou `P159B.x`
ou `P-admin-1`. A decidir per convenção do projecto em sub-
passo .1 (provavelmente identificador específico por ser
administrativo, não materialização). Para clareza neste
enunciado, usa-se `ADR-0062-create` como referência.

---

## Estado actual antes de começar

- 63 ADRs após P159B (28 EM VIGOR; ADR-0060 IMPLEMENTADO).
- ADR-0062 existe como **reserva sem ficheiro** — apenas
  menção em README ADRs e em vários relatórios.
- Hash actual `entities/content.rs`: `ec58d849` (preservado
  em 10 passos consecutivos).
- 1412 tests; zero violations linter.

**Reserva ADR-0062 confirmada em diagnóstico P159 §1.2**:
> ADR-0062 reservada para hayagriva (autorização de crate
> específica).

**Não confirmado** (a confirmar em .1):
- Conteúdo concreto da reserva (apenas título; ou já tem
  esboço documentado algures?).
- Que precedentes de "autorização de crate" são citáveis
  (ADR-0024/0023/0057 mencionados em P159 §1.4).
- Convenção de naming exacta do ficheiro ADR
  (`typst-adr-0062-hayagriva-autorizacao.md`?
  `typst-adr-0062-hayagriva-bibliography-parsing.md`?).

**Política "sem novas reservas" preservada** — este passo NÃO
cria reservas adicionais. Apenas materializa documento ADR
para reserva pré-existente.

---

## Natureza do passo

**Tamanho**: XS.

**Justificação**: Trabalho documental puro. Criar 1 ficheiro
ADR com status PROPOSTO. Sem modificação de código. Sem
modificação de outras ADRs. Sem promoção a EM VIGOR ou
IMPLEMENTADO (essa decisão fica para passo futuro de
materialização hayagriva — P159G ou equivalente).

Granularidade preservada: 1 deliverable documental → mantém
peso XS análogo a passos administrativos pré-existentes.

**Risco muito baixo**: passo é puramente declarativo. Conteúdo
do ADR pode evoluir antes de promoção (PROPOSTO → EM VIGOR).

---

## Decisões já tomadas

- **Status inicial**: `PROPOSTO`. NÃO `EM VIGOR` nem
  `IMPLEMENTADO` — porque nenhum código de hayagriva foi
  ainda integrado. PROPOSTO é o status correcto para
  decisão tomada mas não em vigor.
- **Scope**: autorização de crate externa `hayagriva` em
  L1 (paridade vanilla typst).
- **Justificação técnica primária**: bibliography + cite
  exigem CSL parsing per paridade ADR-0033; CSL parsing
  cristalino do zero é trabalho desproporcionado vs reuso de
  hayagriva existente.
- **Sem código**: este passo NÃO modifica Cargo.toml nem
  adiciona dependências. Materialização real fica para passo
  futuro pós-promoção.
- **Sem nova reserva**: passo formaliza reserva pré-existente,
  não cria reserva nova.

## Decisões diferidas

- **Versão concreta de hayagriva** a usar quando integrar:
  diferida para passo de materialização (P159G ou equivalente).
- **Subset de hayagriva API** a usar: diferida.
- **Estratégia de quarentena vanilla** para hayagriva: diferida.
- **Ficheiros código afectados quando integrar**: documentados
  em DEBT-55 plano; não materializados aqui.
- **Promoção a EM VIGOR/IMPLEMENTADO**: passo separado futuro
  (P159G ou equivalente).

---

## Sub-passos

### .1 Inventário pré-redacção (obrigatório per ADR-0065)

Localizar e ler:
- `00_nucleo/adr/README.md` — confirmar convenção de naming
  exacta (`typst-adr-NNNN-titulo.md`? sufixo de status?).
- ADR-0024/0023/0057 (precedentes "autorização de crate"
  citados em P159 §1.4) — confirmar estrutura canónica que
  ADR-0062 deve seguir (Status, Contexto, Decisão, Consequências,
  Alternativas, Referências).
- ADR-0060 §"Decisão" sobre Bibliography — citar literal
  como contexto.
- DEBT-55 — citar como dependência de promoção futura.

Output: secção §1 de notas internas (não documentadas em
.md à parte; este é XS administrativo).

### .2 Redigir ADR-0062

Ficheiro novo:
`00_nucleo/adr/typst-adr-0062-hayagriva-bibliography-parsing.md`
(naming a confirmar em .1).

Conteúdo:
- **Status**: `PROPOSTO`.
- **Data**: data actual.
- **Contexto**:
  - Bibliography + Cite exigem CSL parsing per paridade
    ADR-0033 (citar literal).
  - Vanilla integra `hayagriva` profundamente em
    `typst-library/src/model/bibliography.rs` (1226 linhas).
  - DEBT-55 documenta plano XL com hayagriva.
  - P159A materializou subset minimal cristalino sem
    hayagriva (Vec<BibEntry> literal); refinos restantes
    exigem CSL.
- **Decisão**: autorizar uso de crate `hayagriva` em L1 para
  parsing de entries CSL e geração de citation strings.
  Status PROPOSTO até passo de materialização real (P159G ou
  equivalente).
- **Consequências**:
  - Positivas: paridade vanilla; reuso de CSL compliance
    existente.
  - Negativas: dependência externa em L1 (precedente
    documentado em ADR-0024/0023/0057); aumento de tempo
    de compilação; potencial conflito de versão.
- **Alternativas consideradas**:
  - Implementar CSL parsing cristalino do zero — rejeitada
    por desproporcionalidade.
  - Manter subset minimal sem hayagriva — adoptada como P159A
    (subset minimal); insuficiente para paridade ADR-0060.
  - Usar outra crate (e.g. `biblatex`) — pouco mainstream
    em ecosistema Rust; hayagriva é a default vanilla.
- **Referências**:
  - ADR-0024/0023/0057 (precedentes autorização crate
    externa — confirmar IDs em .1).
  - ADR-0060 §"Decisão" sobre Bibliography.
  - ADR-0033 (paridade observable).
  - DEBT-55 (plano XL).
  - P159A relatório (subset minimal cristalino).
  - P159B relatório (Bloco B identificado).

### .3 Actualizar README ADRs

`00_nucleo/adr/README.md`:
- Substituir entrada "ADR-0062 reservada hayagriva" por
  entrada concreta com link para ficheiro novo.
- Status PROPOSTO documentado.
- Contagem ADRs total: 63 → 64 (paridade incremento P156K
  ADR-0064/0065).

### .4 Actualizar contagem em ADR-0061 §"Aplicações cumulativas"

ADR-0061 §"Aplicações cumulativas":
- Linha cronológica nova para `ADR-0062-create` com slope
  "—" e tests Δ "0".
- Padrões metodológicos: inventariar-primeiro N=15 → 16
  (ADR-0065 critério #5 inventário trivial + critério #1
  naming convention).
- §análise de risco N=15 → 16 (passo administrativo XS).

### .5 Verificação documental

- Linter markdown se aplicável.
- Verificar links cruzados (ADR-0024/0023/0057, ADR-0060,
  ADR-0033, DEBT-55, P159A/B).

---

## Verificação

Numerada para reporte de conclusão:

1. Ficheiro ADR-0062 criado em
   `00_nucleo/adr/typst-adr-0062-*.md` com naming confirmado
   em .1.
2. Status `PROPOSTO` documentado.
3. Estrutura canónica seguida (Status / Data / Contexto /
   Decisão / Consequências / Alternativas / Referências).
4. ADRs precedentes (0024/0023/0057) citados como precedente
   "autorização crate externa".
5. ADR-0060 + ADR-0033 + DEBT-55 + P159A/B referenciadas.
6. README ADRs actualizado: entrada "reservada hayagriva"
   substituída por entrada concreta com link; contagem
   total **64** (era 63).
7. ADR-0061 §"Aplicações cumulativas" actualizada com linha
   `ADR-0062-create`.
8. **Sem código alterado** — `entities/content.rs` mantém
   `ec58d849` (11º passo consecutivo se P158B for posterior;
   senão 11º passo absoluto).
9. **Sem novas reservas** criadas (paridade política P158).
10. ADR-0062 NÃO promovida a EM VIGOR/IMPLEMENTADO neste
    passo — promoção fica para passo futuro de materialização
    hayagriva real.
11. `crystalline-lint`: zero violations.

---

## Critério de conclusão

- Verificações 1-11 passam.
- Relatório separado em
  `00_nucleo/materialization/adr-0062-create-relatorio.md`
  (ou identificador final a decidir em .1) produzido com:
  - Resumo do executado.
  - Confirmação de cada verificação.
  - **§análise de risco** (padrão N=15 → 16; passo
    administrativo XS — primeiro do tipo "criar ADR a partir
    de reserva pré-existente").
  - Confirmação: contagem ADRs 63 → 64; ADR-0062 PROPOSTO
    formalizado.
  - **Implicação**: Bloco B do diagnóstico P159B agora pode
    iniciar com referência concreta a ADR-0062 (em vez de
    referência a reserva sem ficheiro).

---

## O que pode sair errado

**Cenários gerais**:
- Inventário .1 revela que ADR-0024/0023/0057 NÃO existem
  ou têm scope diferente do esperado → procurar precedentes
  reais; documentar.
- ADR-0024/0023/0057 terem precedente que NÃO é "autorização
  crate" mas sim outro tipo de decisão técnica → procurar
  ADRs específicas de crate authorization; pode revelar que
  não existe precedente formal e este passo estabelece o
  precedente.
- Convenção de naming ter mudado desde reservas pré-existentes
  → ajustar naming do ficheiro novo per convenção actual.

**Cenários específicos**:
- Conteúdo concreto da reserva ADR-0062 já estar documentado
  em algum lugar (e.g. comentário em Cargo.toml, nota em
  diagnóstico antigo) → reusar conteúdo; documentar fonte.
- Decisão de Status ser ambígua (PROPOSTO vs IDEIA vs
  ADIADO) → manter PROPOSTO per pré-decisão; alterar só se
  inventário .1 revelar que IDEIA/ADIADO é mais preciso.
- Outras ADRs ainda em estado "reserva sem ficheiro" no
  README → documentar mas NÃO criar neste passo (uma reserva
  por passo XS para preservar atomicidade).

---

## Notas operacionais

- **Passo administrativo XS**. Custo muito baixo; benefício
  claro (Bloco B desbloqueado para futuro com referência
  concreta).
- **Independente de P158B**. Pode ser executado antes, depois,
  ou em paralelo. Sem dependência mútua.
- **Status PROPOSTO é deliberado**. NÃO promover a EM VIGOR
  ou IMPLEMENTADO neste passo — essa promoção é decisão
  arquitectural maior que fica para passo de materialização
  hayagriva real (P159G ou equivalente).
- **Contagem ADRs**: 63 → 64. Paridade incremento P156K
  (ADR-0064/0065 +2).
- **Política "sem novas reservas" preservada** — este passo
  formaliza reserva pré-existente, não cria nova. Não viola
  política.
- **Subpadrão emergente**: "passo administrativo XS criar ADR
  PROPOSTO a partir de reserva pré-existente" — primeiro do
  tipo nesta sessão. Candidato a precedente para outras
  reservas se aplicável (e.g. ADR sobre column flow se
  reservada para DEBT-56 futuro).

---

## Pós-passo

Após conclusão de ADR-0062-create:

**Layout fica em 78% inalterado**. **Model fica em ~50%
inalterado**. **ADRs total cresce 63 → 64**.

**ADR-0062 PROPOSTO** disponível para referência em passos
futuros de Bloco B (P159G hayagriva integration; P159H/I/J
CSL styles).

**Próxima decisão** (sem candidata pré-acordada):
- Voltar a Bloco A (P158B se ainda não executado; P159C/D/F).
- Iniciar Bloco B com primeiro passo hayagriva (P159G —
  Cargo.toml + crystalline.toml; XS administrativo).
- Mudar de módulo.
- Outras direcções pendentes.

ADR-0060 mantém-se `IMPLEMENTADO`. ADR-0061 mantém-se
`PROPOSTO`. **ADR-0062 PROPOSTO** (era reserva).

Padrão granularidade 1-2 features/passo (N=15) NÃO é desafiado
por este passo (administrativo XS; sem materialização).

**Reservas pendentes** (não criadas neste passo):
- ADR-0017 Introspection runtime adiada — pré-existente; NÃO
  formalizada como ADR concreta neste passo. Candidata a
  passo administrativo XS análogo se prioritário.

**Pausa natural após ADR-0062-create — Bloco B desbloqueado;
política "sem novas reservas" preservada (formaliza
pré-existente); contagem ADRs cresce. Decisão humana sobre
próxima direcção tem máxima informação.**
