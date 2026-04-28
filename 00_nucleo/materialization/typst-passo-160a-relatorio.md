# Relatório Passo P160A — Criar ADR-0066 PROPOSTO Introspection runtime

Materialização do **sub-passo administrativo XS de P160** para
formalizar reserva conceptual pré-existente "Introspection
runtime adiada" como ficheiro ADR concreto com status `PROPOSTO`.
**Décima primeira aplicação consecutiva de materialização** desde
P156C (passo administrativo); **segunda aplicação do subpadrão
emergente** "passo administrativo XS criar ADR PROPOSTO a partir
de reserva pré-existente" (ADR-0062-create + P160A; atinge
meio-caminho limiar formalização N=3-4).

---

## Resumo do executado

1. **Inventário §1** (notas internas):
   - **Descoberta crítica**: ADR-0017 número já está IMPLEMENTADO
     (`typst-adr-0017-adiamento-eval-typst-library.md`) desde
     2026-03-26 para tópico distinto ("adiamento de eval() e
     estratégia typst-library"). A reserva conceptual referida
     cumulativamente como "ADR-0017 Introspection runtime adiada"
     em P156B/P159A/P159B/P160 sempre foi conceptual sem ficheiro.
   - **Slot 0063 reservado** conceptualmente para column flow
     (sem ficheiro).
   - **0064/0065 já usados**; **0066 próximo disponível**.
   - **Resolução**: usar ADR-0066 em vez de reocupar 0017
     (reocupação seria divergência observable do ADR existente
     IMPLEMENTADO).

2. **Ficheiro ADR-0066 criado** em
   `00_nucleo/adr/typst-adr-0066-introspection-runtime-adiada.md`
   com estrutura canónica:
   - **Nota sobre numeração** (secção dedicada) explicando
     divergência histórica entre reserva conceptual "ADR-0017"
     e ADR concreta ADR-0066.
   - Status / Data / Contexto (cobertura cristalina actual +
     subpadrão #15 N=3 single-pass viável).
   - Decisão (subset minimal pós-promoção: state + metadata +
     here/locate + query + position).
   - Análise pureza paridade ADR-0029.
   - Consequências (positivas: ~17% → ~50% subset minimal;
     negativas: complexidade pipeline 2-pass).
   - Alternativas consideradas (Alt A manter single-pass;
     Alt B sem ADR; Alt C pipeline vanilla integral).
   - Plano promoção futuro (P160B state → P160C metadata → ...).
   - Precedentes citáveis (ADR-0062-create N=1; ADR-0033;
     ADR-0054; ADR-0029; ADR-0017 existente paridade estrutural
     histórica).
   - Referências e Próximos passos.

3. **README ADRs actualizado**:
   - Linha tabela ADR-0066 adicionada com nota explicativa
     sobre divergência de numeração 0017 vs 0066.
   - Total ADRs: 64 → **65**.
   - Distribuição PROPOSTO: 12 → **13** (+0066).
   - Entrada cronológica P160A adicionada antes de P160 com
     descrição completa.

4. **ADR-0061 §"Aplicações cumulativas" actualizada**:
   - Linha P160A adicionada na tabela slope cumulativo.
   - Padrões metodológicos: granularidade N=21 (inalterada);
     inventariar primeiro N=24 → **25**; §análise de risco
     N=24 → **25**.
   - Critérios formalmente validados ADR-0065: critério #1
     (naming) cresce N=2 → **3** com ADR-0062-create + P160A;
     critério #5 décima terceira aplicação concreta.

---

## Confirmação das verificações (1-11)

1. **Ficheiro ADR criado** ✓ —
   `00_nucleo/adr/typst-adr-0066-introspection-runtime-adiada.md`
   (naming confirmado em .1 com decisão de usar 0066 em vez
   de 0017).

2. **Status `PROPOSTO`** ✓ documentado.

3. **Estrutura canónica seguida** ✓ — Status / Data / Nota
   numeração / Contexto / Decisão / Análise pureza / Consequências /
   Alternativas / Plano promoção / Precedentes / Referências /
   Próximos passos.

4. **ADRs precedentes citados** ✓ — 0017 (existente; paridade
   histórica), 0029 (pureza física), 0033 (paridade observable),
   0034 (estrutura diagnóstico), 0054 (graded), 0060 (Model
   roadmap), 0061 (Layout roadmap), 0062 (paridade administrativa),
   0065 (inventariar primeiro).

5. **P160 + P159B referenciados** ✓ como base factual em
   §"Contexto" e §"Precedentes citáveis".

6. **README ADRs actualizado** ✓:
   - Linha tabela ADR-0066 com explicação de divergência de
     numeração.
   - Contagem total **65** (era 64; +1 por ADR-0066).
   - Distribuição PROPOSTO **13** (era 12; +0066).
   - Entrada cronológica P160A adicionada antes de P160.

7. **ADR-0061 §"Aplicações cumulativas" actualizada** ✓ — linha
   P160A adicionada; padrões N=24→25 inventariar/risco; critério
   #1 naming N=2→3.

8. **Sem código alterado** ✓ — `entities/content.rs` mantém
   `ec58d849` (**19º passo consecutivo** via L0-baseline).

9. **Sem novas reservas criadas** ✓ — passo formaliza reserva
   conceptual pré-existente (não cria nova). Slot 0063 column
   flow mantém-se reservado mas não reforçado.

10. **ADR-0066 NÃO promovida a EM VIGOR/IMPLEMENTADO** ✓ —
    promoção fica para passo futuro (P160B subset minimal
    state runtime).

11. **`crystalline-lint`**: ✓ No violations found.

---

## §Análise de risco (N=25)

**Vigésima quinta aplicação consecutiva** do padrão "§análise
de risco no relatório" (P156F/G/H/I/J/K/L + P157/A/B/C +
P158/A/B/C + P159/A/B/C/D/F/E/G + ADR-0062-create + P160 +
**P160A**).

**Risco realizado**: **muito baixo** (alinhado com previsão da
spec §"Natureza do passo" — passo administrativo XS).

**Eixos avaliados**:

| Eixo | Risco | Justificação |
|------|-------|--------------|
| Estrutura | nulo | Passo administrativo; sem código alterado. |
| Backwards compat | nulo | Sem alteração de tipos, helpers, ou variants. |
| Hash content.rs | nulo | Preservado `ec58d849` 19º consecutivo. |
| Tests | nulo | 1501 workspace inalterado; sem novos tests. |
| Conflito de numeração 0017 | resolvido | Descoberta em §1; resolução clara (usar 0066); nota explícita no ficheiro ADR. |
| Subpadrão N=1→2 | nulo | Patamar saudável; meio-caminho limiar formalização. |
| ADR-0066 PROPOSTO sem promoção | aceite | Decisão deliberada; promoção fica para P160B. |

**Cenários da spec §"O que pode sair errado"**:
- Convenção de naming ter mudado — **não realizado**: paridade
  ADR-0062-create directa (`typst-adr-NNNN-titulo-descritivo.md`).
- Conteúdo concreto da reserva já documentado — **parcialmente
  realizado**: P160 §1-§6 forneceram conteúdo factual completo;
  reusado integralmente em §"Contexto" do ADR-0066.
- DEBT-10 ainda activo — **não realizado**: comentário antigo
  parcialmente cumprido via `materialize_time` P66; não bloqueador.
- Decisão Status ambígua — **não realizado**: PROPOSTO mantido
  per pré-decisão; matriz de alternativas claramente regista
  trajectória de promoção.
- ADR-0062-create já executada → ajustar contagem — **realizado
  e mitigado**: ADR-0062-create já tinha sido executada (PROPOSTO
  desde 2026-04-27); contagem 63 → 64 → 65 cumulativa correcta.
- **Conflito de numeração 0017** — **REALIZADO** (cenário não
  previsto na spec mas tratado): descoberta em §1; resolução
  clara via uso de 0066 + nota dedicada no ADR.

**Padrão consolidado**: **25 aplicações consecutivas** sem
materialização que exceda risco previsto.

---

## ADR-0061 §"Aplicações cumulativas" anotada

Linha P160A adicionada à tabela:

| Passo | Feature(s) | Slope | Cobertura cumulativa | Tests Δ |
|-------|-----------|------:|---------------------:|--------:|
| **P160A** | **(administrativo XS — criar ADR-0066 PROPOSTO Introspection runtime; resolve confusão de numeração 0017 vs 0066)** | — | — (sem código; ADRs total 64 → 65) | **0** |

Padrões atualizados:
- Granularidade 1-2 features/passo: **N=21 inalterada** (passo
  administrativo XS).
- Inventariar primeiro: **N=24 → 25** (ADR-0065 critério #5
  décima terceira aplicação concreta + critério #1 naming
  segunda aplicação isolada concreta em passo administrativo
  XS após ADR-0062-create — patamar #1 cresce N=2 → 3).
- §análise de risco no relatório: **N=24 → 25**.
- ADR-0064: NÃO directamente aplicável em P160A.
- Subpadrões #14/#15/#16/P155 cross-feature/refactor field
  para Option: N inalterados (passo administrativo).
- **Subpadrão "passo administrativo XS criar ADR PROPOSTO a
  partir de reserva pré-existente": N=1 → 2** (ADR-0062-create
  + P160A; atinge meio-caminho limiar formalização N=3-4).

---

## Confirmações finais

- **Subpadrão "passo administrativo XS criar ADR PROPOSTO"
  cresce N=1 → 2**: ✓ confirmado. ADR-0062-create + P160A.
  Atinge meio-caminho limiar formalização N=3-4. Próxima
  aplicação (se houver outra reserva conceptual) consolidaria
  patamar.
- **Contagem ADRs incrementada**: ✓ 64 → 65.
- **ADR-0066 PROPOSTO formalizado**: ✓ ficheiro com estrutura
  canónica + nota dedicada sobre divergência de numeração.
- **Estabilidade hash content.rs N=18 → 19**: ✓ confirmado
  via L0-baseline interpretation (passo administrativo sem
  código alterado).

**Implicação confirmada**: P160B (state runtime) e restantes
Bloco B do diagnóstico P160 (P160C metadata, P160D here/locate,
P160E query, P160F position) agora podem iniciar com **referência
concreta a ADR-0066 PROPOSTO** (em vez de referência a reserva
sem ficheiro).

**Decisão crítica registada**: divergência de numeração entre
reserva conceptual histórica ("ADR-0017 Introspection runtime")
e ADR concreta (ADR-0066) — documentada explicitamente no
ficheiro ADR-0066 §"Nota sobre numeração". Refactor cumulativo
de relatórios antigos NÃO necessário per política — comentários
históricos preservam-se com nota interpretativa.

---

## Estado pós-P160A

- **Layout**: 78% inalterada.
- **Model agregado**: ~50% inalterada.
- **Cobertura ampla impl+impl⁺+parcial**: 77% (inalterada).
- **Cobertura arquitectural**: **82%** inalterada (passo
  administrativo; sem código alterado).
- **Cobertura Introspection**: 17% confirmada como saturada
  por tecto puro single-pass (per P160 §4).
- **Hash `entities/content.rs`**: `ec58d849` (**19º passo
  consecutivo** preservado via L0-baseline).
- **65 ADRs** (era 64; +1 ADR-0066). Distribuição:
  - **PROPOSTO: 13** (era 12; +0066).
  - EM VIGOR: 28 (inalterada).
  - IMPLEMENTADO: 19 (inalterada).
  - IDEIA: 2 (inalterada).
  - Outros: 3.
- **Tests**: 1501 workspace inalterado.
- **Variants Content**: 58 (inalterada).
- **Stdlib funcs**: 48 (inalterada).
- **Padrões consolidados**:
  - Granularidade N=21 (inalterada — passo administrativo).
  - **Inventariar primeiro N=24 → 25** (ADR-0065 critério #5
    décima terceira aplicação concreta + critério #1 naming
    segunda aplicação isolada concreta XS).
  - Smart→Option Caso A patamar N=7 (inalterado).
  - **§análise risco N=24 → 25**.
  - Estabilidade hash L0 content.rs **N=19**.
  - Tipo entity em ficheiro próprio N=5 (inalterado).
  - Infraestrutura state lookup N=3 (inalterado — confirmado
    em ADR-0066 §"Contexto" como infraestrutura única
    materializável sem ADR-0066 promovida).
  - Subpadrão #16 (refino tipo entity sem alteração Content):
    N=3 (inalterado — limiar formalização atingido).
  - P155 cross-feature N=1 (inalterado).
  - Refactor de field para Option N=1 (inalterado).
  - Helper `optional_str` cumulativo N=12 (inalterado).
  - **Subpadrão "passo administrativo XS criar ADR PROPOSTO":
    N=1 → 2** (atinge meio-caminho limiar formalização).

**ADR-0060 mantém-se `IMPLEMENTADO`**. **ADR-0061 mantém-se
`PROPOSTO`**. **ADR-0017 (existente) mantém-se `IMPLEMENTADO`**
(eval deferral; tópico distinto). **ADR-0062 mantém-se
`PROPOSTO`**. **ADR-0066 PROPOSTO** (novo).

**Reservas pendentes** (não criadas neste passo):
- Slot 0063 column flow — pré-existente; preserva-se sem
  ficheiro per política "sem novas reservas".

**Próxima decisão (sem candidata pré-acordada — política "sem
novas reservas")**:

- **P160B** (próximo natural): materializar `state(key, init)`
  runtime mutable state. M; +10-15 tests; +6-8pp Introspection.
  Promoção ADR-0066 PROPOSTO → IMPLEMENTADO após este passo
  materializa.
- **P160C-F** (sequência Bloco B): metadata, here/locate, query,
  position. Cada um M; cumulativo +30-40pp Introspection.
- **Bloco C cross-módulo** (após Bloco B saturado): `measure()`
  stdlib expose; cross-document refs.
- **Mudança de módulo** (Layout Fase 3 columns/colbreak ou
  outro) se prioridade observable Layout for maior.
- **Conjunto administrativo XS** — promoções acumuladas
  (`optional_str` N=12 helper público; ADR meta subpadrão
  #15/#16; L0 content.md update).

**Pausa natural após P160A — Bloco B Introspection desbloqueado
com referência concreta a ADR-0066 PROPOSTO; subpadrão "passo
administrativo XS criar ADR PROPOSTO" atinge N=2 (meio-caminho
limiar formalização); descoberta de conflito de numeração 0017
resolvida com decisão clara registada (usar 0066); padrão
diagnóstico-primeiro N=25 consolidado. Decisão humana sobre
próxima direcção tem máxima informação acumulada.**
