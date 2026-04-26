# Passo P156L — `pad` refino sides individualizadas (Layout Fase 3 sub-passo 2)

Continuação directa da série granular P156C-K. Mantém cadência
granular (1 feature/passo, N=8 validado em P156J). Refino do
variant `Pad` existente (P156C) para suportar sides
individualizadas (top/right/bottom/left) per spec vanilla,
em vez do uniform single `Length` actual.

Target pós-passo: **Layout 84%** (entrada `pad` passa de
parcial → implementado puro; +6pp).

**Nona aplicação consecutiva** de ADR-0061 §"Aplicações
cumulativas". **Primeira aplicação de ADR-0065** com critério
#3 (expansão de variant existente) — ADR-0065 §Justificação
empírica passa de N=5 para N=6 implícito, mas só formalizado
em ADR meta futura.

---

## Estado actual antes de começar

- Layout 78% (14/18) após P156J.
- 1315 tests (lib+integ+diagnostic); zero violations linter.
- 52 variants Content; 42 stdlib funcs.
- Hash actual `entities/content.rs`: `ec58d849` (P156J).
- 63 ADRs (após P156K). ADR-0064 (Smart→Option) e ADR-0065
  (Inventariar primeiro) `EM VIGOR`.
- ADR-0061 PROPOSTO; §"Aplicações cumulativas" pré-anotada
  até P156J.

**Leituras prévias obrigatórias**:
- `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`
  — entrada `pad` (parcial) na tabela A.
- `00_nucleo/materialization/typst-passo-156c-relatorio.md` —
  origem de `Content::Pad` com single `Length`.
- `00_nucleo/adr/typst-adr-0064-smart-para-option-default.md`
  — Caso C aplicável (Length default zero → Option<Length>).
- `00_nucleo/adr/typst-adr-0065-inventariar-primeiro.md` —
  critério #3 (expansão de variant existente) aplicável.
- `lab/typst-original/crates/typst-library/src/layout/pad.rs`
  (vanilla, quarentena) — código de referência para spec
  sides individualizadas.
- `01_core/src/entities/sides.rs` — tipo `Sides<T>` introduzido
  em P156C; reusar.

---

## Natureza do passo

**Tamanho**: M.

**Justificação**: 1 feature (refino de variant existente). Não
é variant novo — é **expansão de variant existente** com
breaking change interno na assinatura. Crítico inventário per
ADR-0065 critério #3.

Granularidade preservada: 1 feature → mantém N=9 do padrão.

**Risco médio** (não baixo como passos aditivos P156C-J):
modificação de variant existente em 9 sítios pattern-match.
Regression tests críticos. §análise de risco com peso real,
não cerimonial.

---

## Decisões já tomadas

- **Assinatura do variant**:
  ```rust
  Pad {
      body: Box<Content>,
      sides: Sides<Option<Length>>,  // top, right, bottom, left
  }
  ```
  Substitui o actual:
  ```rust
  Pad { body: Box<Content>, padding: Option<Length> }
  ```
- **Tipo `sides`**: `Sides<Option<Length>>` per ADR-0064 Caso C
  (cada lado tem default zero vanilla; `None` per lado ↔ zero).
  Não é `Option<Sides<Length>>` porque a granularidade é por
  lado, não por struct inteira.
- **Reuso de `Sides<T>`**: tipo já existe em
  `01_core/src/entities/sides.rs` (P156C). Sexto reuso de
  infraestrutura genérica — preserva padrão #5 (reuso de
  template containers, agora N=4 → N=5).
- **Compatibilidade com stdlib `pad`**: stdlib func `pad`
  passa a aceitar named args:
  - `top`, `right`, `bottom`, `left` (cada um Length opcional).
  - `x` (atalho para left + right).
  - `y` (atalho para top + bottom).
  - `rest` (atalho para todos os 4 não especificados).
  Default vanilla: cada lado a zero.
- **Helper stdlib**: `extract_length` reusado N=7 vezes (per
  ADR-0064 §Implicações subpadrão emergente). Quinto argumento
  numa única func.
- **Helper novo**: `extract_sides<T>` para parse de named args
  com fallback `x`/`y`/`rest`. Adicionado a `stdlib/layout.rs`
  como helper privado (pré-decisão: privado; promoção a helper
  público diferida per padrão `extract_length` que ainda é
  privado).

## Decisões diferidas

- **Promoção de `extract_length` a helper público**: continua
  diferida (ADR-0064 §Implicações). Não é scope de P156L.
- **Promoção de `extract_sides<T>` a helper público**: diferida
  até segundo reuso (per padrão N=2 mínimo para promoção).

---

## Sub-passos

### .1 Inventário pré-materialização (obrigatório per ADR-0065 #3)

Diagnóstico em `00_nucleo/diagnosticos/diagnostico-pad-refino-passo-156l.md`
com 7 itens canónicos (ADR-0034) + 2 itens específicos para
expansão de variant existente:

1. Assinatura vanilla `PadElem` — confirmar 4 sides + atalhos
   x/y/rest.
2. Comportamento observável (cada lado independente; defaults
   zero; soma com gap quando aplicável).
3. ADR-0064 Caso aplicável (C: Length default zero → Option).
4. Variants Content existentes a estender (apenas `Pad`).
5. Helpers stdlib reusáveis (`extract_length` N=7) +
   helper novo (`extract_sides<T>`).
6. Limitações aceites (relative percentage por lado diferida
   se vanilla suportar — verificar em diagnóstico).
7. Tests planeados (regression do P156C; novos para 4 sides
   independentes; atalhos x/y/rest; defaults).
8. **(Específico expansão)** Sítios pattern-match a actualizar
   (esperados 9 — paridade P156I/J).
9. **(Específico expansão)** Tests existentes de P156C que
   continuam válidos vs tests que precisam de actualização de
   API.

### .2 Refactor de variant `Content::Pad`

`01_core/src/entities/content.rs`:
- Substituir variant antigo por novo (sides: Sides<Option<Length>>).
- Actualizar construtor `Content::pad(...)` para nova
  assinatura.
- Cobrir todos os 9 sítios pattern-match com nova estrutura.
- Manter helper `body: Box<Content>` inalterado.

### .3 Refactor de stdlib `native_pad`

`01_core/src/rules/stdlib/layout.rs`:
- Aceitar named args: `top`, `right`, `bottom`, `left`, `x`,
  `y`, `rest`.
- Implementar `extract_sides<T>` helper privado (parse com
  fallback x/y/rest).
- Validações: cada lado rejeita negativos; conflito x/left
  rejeitado; conflito y/top rejeitado; named arg desconhecido
  rejeitado.
- Manter compatibilidade conceptual: `pad(body, length)`
  posicional **deixa de funcionar** — breaking change na API
  cristalina, documentado em §análise de risco.

### .4 Tests

- **Regression tests P156C** (críticos):
  - 6 unit tests existentes de `Content::Pad` actualizados para
    nova API.
  - 11 stdlib tests existentes actualizados para nova API.
  - 2 layout E2E tests existentes actualizados.
- **Tests novos**:
  - Unit: 4 sides independentes; atalhos x/y/rest; conflito
    x+left rejeitado; conflito y+top rejeitado.
  - Stdlib: cada named arg isoladamente; combinações; defaults;
    breaking change `pad(body, length)` rejeitado com erro
    claro.
  - Layout E2E: paridade visual com pad uniforme (regression);
    pad assimétrico (top maior que bottom).
- **Δ esperado**: +12 a +18 tests (range mais estreito que
  P156J porque há mais regression e menos features novas).

### .5 Propagação de hashes

`crystalline-lint --fix-hashes .` para propagar hash novo de
`entities/content.rs` aos prompts L0 que o referenciam.

---

## Verificação

Numerada para reporte de conclusão:

1. `cargo test` em workspace: **1315 + Δ** tests, zero falhas
   (Δ esperado +12 a +18).
2. `crystalline-lint`: zero violations.
3. Contagem variants Content: **52** (inalterada — refino, não
   adição).
4. Contagem stdlib funcs: **42** (inalterada — refino, não
   adição).
5. Cobertura Layout: **84%** (entrada `pad` passa de `parcial`
   para `implementado puro` em
   `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`).
6. Hash actualizado em prompts L0 (`crystalline-lint --check-hashes`
   passa).
7. Regression tests P156C: 100% adaptados e a passar (6 unit +
   11 stdlib + 2 E2E = 19 tests pré-existentes).

---

## Critério de conclusão

- Verificações 1-7 passam.
- Relatório separado em
  `00_nucleo/materialization/typst-passo-156l-relatorio.md`
  produzido com:
  - Resumo do executado.
  - Confirmação de cada verificação.
  - **§análise de risco** (padrão N=5 → N=6; com peso real
    desta vez por ser refactor, não aditivo).
  - Slope cumulativo actualizado (mesa P156C-L).
  - ADR-0061 §"Aplicações cumulativas" anotada com P156L.
  - **Confirmação**: ADR-0065 critério #3 (expansão de variant
    existente) aplicado pela primeira vez. ADR-0064 Caso C
    aplicado (segunda vez).

---

## O que pode sair errado

**Cenários gerais**:
- Inventário .1 revela que vanilla suporta `Rel<Length>` (relative
  percentage) por lado e não apenas `Length` absoluto → expandir
  decisão para `Sides<Option<Rel<Length>>>` antes de avançar
  para .2. Documentar como ADR-0064 Caso A (não C) se for
  o caso.
- Regression tests P156C falharem por divergência semântica
  (e.g. `pad(body, length)` posicional ser interpretado como
  `rest=length` vs erro) → decisão arquitectural deve ser
  registada explicitamente em .1, não inferida em .2.

**Cenários específicos**:
- Helper `extract_sides<T>` ter complexidade superior ao
  esperado (e.g. type bounds difíceis de exprimir
  genericamente) → fallback para implementação não-genérica
  específica de Length; promoção a genérico diferida.
- Conflito entre `x` e `left` (ou `y` e `top`) num único call
  ter semântica vanilla "último vence" em vez de "rejeitar" →
  ajustar decisão para alinhar paridade ADR-0033. Verificar
  em .1.
- Stdlib breaking change (`pad(body, length)` deixar de
  funcionar) afectar tests de outras stdlib funcs (e.g.
  composições com `pad` em testes de `block`/`box`/`stack`)
  → grep antes de .3 para identificar callers internos.

---

## Notas operacionais

- **Primeiro passo M com refactor real** desde início da série
  granular (P156C). P156F (skew) foi expansão limitada de
  TransformMatrix — refactor em escopo restrito. P156L é
  refactor de variant Content + stdlib pública + 19 tests
  existentes a adaptar. **Risco médio, não baixo.**
- §análise de risco no relatório terá peso real (não
  cerimonial). Pode ser oportunidade de revisitar ADR-0065
  §"Critério não-trivial #3" com nota empírica reforçada.
- Reuso de `Sides<T>` (P156C) é sexto. Padrão #5 (reuso de
  template containers) passa de N=4 a N=5 — patamar moderado
  → forte. Candidato a ADR meta futura se mantiver crescimento.
- Reuso de `extract_length` chega a N=7. Subpadrão emergente
  documentado em ADR-0064 §Implicações continua a reforçar-se.
- ADR-0064 Caso C (segunda aplicação concreta) confirma
  estabilidade do padrão. Caso D (`bool` default não-`false`)
  não se aplica em P156L.

---

## Pós-passo

Após conclusão de P156L, **Layout fica em 84%** com 3 entradas
pendentes (era 4):
- `columns` / `colbreak` (Fase 3 condicional — DEBT-56 column
  flow L+ aberto em P156B; ADR-0063 reservada).
- `place` (refino column scope — depende de columns).
- `measure` (depende de ADR-0017 Introspection runtime).

**Próxima decisão (per pré-acordo)**: abordagem para Introspection.
Identificador a definir pós-P156L (P157 mantém-se reservado para
Model Fase 2 table foundations; identificador para Introspection
pode ser P156M ou outro). Três sub-opções a decidir:

1. Sub-passo granular S/M sobre uma feature específica de
   Introspection (per padrão validado N=9 com P156L).
2. Diagnóstico amplo do estado de Introspection (passo S+
   documental, semelhante a P156B para Layout).
3. Atacar `measure` directamente (entrada de Layout que depende
   de Introspection runtime; encadeia caminhos cruzados).

ADR-0061 mantém-se PROPOSTO. Promoção a IMPLEMENTADO continua
diferida — `measure` continua bloqueada por dependência cruzada
com Introspection, mesmo com Layout a 84%.

§"Aplicações cumulativas" será re-anotada com P156L. Padrão
granularidade 1-2 features/passo passa a N=9 (não formalizado
em ADR — continua candidato).
