# Passo P157C — `Content::TableHeader` + `Content::TableFooter` (Model Fase 2 sub-passo 3 — fecha table foundations)

Terceiro e último sub-passo substantivo de Model Fase 2 declarada
em ADR-0060 §"Decisão 1" sub-passo 3. Materializa par simétrico
`TableHeader`/`TableFooter` per scope decidido em diagnóstico
P157 §3. Após P157C, **table foundations declarado em ADR-0060
fica fechado** (3 sub-passos M cada, granularidade preservada).

**Décima segunda aplicação consecutiva de materialização**
desde início da série granular P156C.

**Primeira aplicação concreta de ADR-0064 Caso D em domínio
Model** (Caso D já validado em Layout P156D weak, P156G
breakable, P156J justify=true). Patamar empírico cross-domínio
do ADR meta P156K cresce a 4 dimensões: Casos A, B, C, D
agora todos validados em pelo menos 2 domínios.

Target pós-passo: cobertura Model agregada ~50% inalterada
(sub-entradas qualitativas, paridade P157B); cobertura
estrutural cresce com 2 variants novas. Precedente para
fechamento de "table foundations" como conjunto.

---

## Estado actual antes de começar

- 63 ADRs após P157B (28 EM VIGOR; ADR-0060 IMPLEMENTADO).
- Layout: 78% (inalterado).
- Cobertura Model agregada: ~50% (inalterada em P157B; ganho
  qualitativo via expansão estrutural).
- Hash actual `entities/content.rs`: `ec58d849` (preservado
  P156L → P157 → P157A → P157B).
- 1353 tests (lib+integ+diagnostic); zero violations linter.
- 54 variants Content; 44 stdlib funcs.
- Padrões consolidados pós-P157B: granularidade N=11;
  inventariar N=9; Smart→Option N=8; §análise risco N=9;
  reuso `Sides<T>` N=2; reuso `extract_length` N=7; reuso
  `extract_tracks` N=2; helper privado parametrizado
  `extract_usize_or_none_min` N=4 usos.

**Diagnóstico P157** declarou (§3 + §5):
- P157C: variants `Content::TableHeader` e `Content::TableFooter`
  como par simétrico.
- Granularidade N=12.
- Tests ~10-15.
- ADR-0064 Caso D para `repeat: bool` default true.

**Leituras prévias obrigatórias**:
- `00_nucleo/diagnosticos/diagnostico-model-fase-2-passo-157.md`
  — §3 e §5 (esboço P157C).
- `00_nucleo/materialization/typst-passo-157a-relatorio.md` —
  precedente Table.
- `00_nucleo/materialization/typst-passo-157b-relatorio.md` —
  precedente TableCell + decisão naming flat (`table_header` /
  `table_footer` esperados; confirmar em .1).
- `00_nucleo/adr/typst-adr-0064-smart-para-option-default.md`
  — Caso D (`bool` com default não-`false` usa tipo directo
  com documentação explícita do default vanilla).
- `00_nucleo/adr/typst-adr-0033-paridade-observavel.md` —
  fundamento para divergência naming.
- `00_nucleo/adr/typst-adr-0054-perfil-graded.md` —
  fundamento para `repeat: bool` armazenado sem algoritmo
  de repetição em page breaks.
- `00_nucleo/DEBT.md` — DEBT-56 (column flow multi-region;
  permanece aberto após P157C; relevante a `repeat`).
- `01_core/src/entities/content.rs` — variants `Content::Table`
  (P157A) e `Content::TableCell` (P157B) para padrão estrutural.
- `01_core/src/rules/stdlib/structural.rs` — `native_table` e
  `native_table_cell` (P157A/B) para padrão de stdlib func.
- `lab/typst-original/crates/typst-library/src/model/table.rs`
  — `TableHeader`/`TableFooter` vanilla (referência).

---

## Natureza do passo

**Tamanho**: M.

**Justificação**: 2 features (variants `TableHeader` + `TableFooter`)
**simétricas** — par tratado como uma unidade conceptual. Field
único `repeat: bool` em ambas (Caso D ADR-0064). Sem algoritmo
de repetição em page breaks (diferido per DEBT-56). Reusa
infraestrutura P157A/B.

**Granularidade**: 2 features simétricas contam como 1 unidade
conceptual per precedente P156C (pad + hide simétricos),
P156D (h + v simétricos) e P156G (block) — preserva N=12 do
padrão.

**Risco baixo**:
- **Baixo** porque é par simétrico aditivo análogo a P156C/D
  (variants simétricos sem algoritmo dinâmico).
- **Caso D ADR-0064 já validado em Layout** (P156D, P156G,
  P156J) — primeira aplicação Model é mecânica.
- **Sem nova decisão arquitectural-chave** (decisões de naming
  e módulo já estabelecidas em P157A/B).

---

## Decisões já tomadas

- **Variants Content** (par simétrico):
  ```rust
  TableHeader {
      body:   Box<Content>,
      repeat: bool,  // default true (Caso D; paridade vanilla)
  }
  TableFooter {
      body:   Box<Content>,
      repeat: bool,  // default true (Caso D; paridade vanilla)
  }
  ```

- **`repeat: bool`**: ADR-0064 Caso D. Vanilla usa `bool` com
  default `true`; cristalino traduz para `bool` directo (não
  Option) com documentação explícita do default não-`false`.
  Paridade com P156J `justify=true`.

- **`body: Box<Content>`**: paridade P156H Boxed, P156J Repeat
  e P157B TableCell (single child via Box, não Arc).

- **Algoritmo de repetição em page breaks**: **diferido per
  ADR-0054 graded**. Field `repeat` armazenado; `layout_grid`
  ignora em P157C — DEBT-56 permanece aberto. Documentado no
  relatório §análise de risco como limitação consciente.

- **Stdlib funcs `native_table_header` e `native_table_footer`**
  em `stdlib/structural.rs` (módulo Model continuação per P157A).

- **Naming `table_header` e `table_footer`** (flat, não vanilla
  `table.header`/`table.footer`): **paridade decisão P157B**.
  Mesma justificação: FieldAccess actual não suporta
  `Value::Func.subname`. Confirmar em sub-passo .1 por
  consistência com P157B.

- **Helper de parse `extract_bool_with_default`**: **decisão
  deferida a sub-passo .1**. Pré-decisão sem inventário:
  helper privado novo se não existir; reuso de helper existente
  (e.g. `extract_weak` de P156D) se semântica for compatível.
  Verificar em .1.

- **Pattern de tratamento simétrico**: variants tratados como
  par em todos os 9 sítios pattern-match — paridade interna
  TableHeader↔TableFooter. Diferenciação só em layout
  semântico futuro (header antes do conteúdo, footer depois).

## Decisões diferidas

- **Algoritmo de repetição em page breaks** (header repete no
  topo de cada página de continuação; footer repete no fim):
  **DEBT-56**. Fora de scope P157C. Decisão sobre quando
  fechar DEBT-56 fica para refactor multi-region dedicado.

- **`repeat-rows: Smart<usize>`** (vanilla suporta repetir só
  N linhas em vez de header inteiro): **diferido** per
  ADR-0054 graded. Documentar em diagnóstico .1 como limitação
  consciente. Aplicação concreta de ADR-0064 Caso A se
  materializado em refactor futuro (paridade P157B `x`/`y`).

- **`level: usize`** (vanilla suporta hierarquia de headers
  aninhados): **diferido** per ADR-0054 graded. Documentar.

- **Helper público `extract_bool_with_default`**: promoção a
  `pub(super)` ou helper público diferida até reuso (política
  N=3-4 consistente com `extract_length`/`extract_tracks`/
  `extract_usize_or_none_min`).

---

## Sub-passos

### .1 Inventário pré-materialização (obrigatório per ADR-0065)

Diagnóstico em
`00_nucleo/diagnosticos/diagnostico-table-header-footer-passo-157c.md`
com 7 itens canónicos (ADR-0034) + 2 itens específicos para
par simétrico:

1. Assinatura vanilla `TableHeader` e `TableFooter` minimal
   — confirmar field crítico `repeat` (default `true`) +
   fields diferidos (`repeat-rows: Smart<usize>`, `level:
   usize`, `body` — diferidos per ADR-0054 graded; documentar).
2. Comportamento observável (header renderiza antes do conteúdo
   da table; footer renderiza depois; `repeat=true` faz header/
   footer reaparecer em cada page break — diferido).
3. ADR-0064 caso aplicável **confirmado**: Caso D para `repeat`
   em ambos.
4. Variants Content existentes a estender (nenhuma; 2 variants
   novos).
5. Helpers stdlib reusáveis: verificar `extract_weak` (P156D)
   ou `extract_bool` se existir; se não, helper novo
   `extract_bool_with_default`.
6. Limitações aceites (algoritmo de repetição em page breaks
   diferido DEBT-56; repeat-rows e level diferidos).
7. Tests planeados (variant present + stdlib happy path +
   defaults + edge cases ADR-0064 Caso D — range 10-15 per
   esboço P157 §5).
8. **(Específico par simétrico)** Confirmar paridade interna
   absoluta entre TableHeader e TableFooter — todos os fields,
   todos os pattern-match, todos os tests devem ser simétricos
   excepto naming.
9. **(Específico naming)** Confirmar consistência com P157B:
   `table_header` e `table_footer` flat per padrão FieldAccess.

### .2 Adicionar variants `Content::TableHeader` e `Content::TableFooter`

`01_core/src/entities/content.rs`:
- Adicionar `TableHeader { body, repeat }` com tipos per
  §"Decisões já tomadas".
- Adicionar `TableFooter { body, repeat }` em paralelo
  imediato (par simétrico).
- Cobrir todos os 9 sítios pattern-match Content existentes
  (paridade P156I/J/L/P157A/B).
- Construtores `Content::table_header(body, repeat)` e
  `Content::table_footer(body, repeat)`.
- **Tratamento simétrico em todos os arms**: cada sítio
  pattern-match deve ter as 2 entradas adjacentes para tornar
  paridade visualmente óbvia.

### .3 Adicionar stdlib funcs `native_table_header` e `native_table_footer`

`01_core/src/rules/stdlib/structural.rs`:
- Func `table_header(body, repeat: true) -> content`.
- Func `table_footer(body, repeat: true) -> content`.
- Reusar ou criar helper `extract_bool_with_default` per
  decisão em .1.
- Validações: `repeat` não-bool rejeitado; named arg
  desconhecido rejeitado; body inválido rejeitado.
- **Implementação simétrica**: as duas funcs devem ter a mesma
  estrutura linha-a-linha excepto o nome do variant Content.

Registadas em `eval/mod.rs::make_stdlib` como `table_header` e
`table_footer`. Re-exportadas em `stdlib/mod.rs`.

### .4 Layout para `Content::TableHeader` e `Content::TableFooter`

`01_core/src/rules/layout/mod.rs`:
- 2 pattern arms novos:
  ```rust
  Content::TableHeader { body, repeat: _ } => {
      self.layout_content(body);
  }
  Content::TableFooter { body, repeat: _ } => {
      self.layout_content(body);
  }
  ```
- Render minimal: body uma vez no contexto actual. `repeat`
  armazenado mas ignorado per ADR-0054 graded.
- **`layout_grid` NÃO modificado** (paridade verificações
  P157A #8 e P157B #10).

### .5 Tests

**Tests simétricos**: cada test para TableHeader tem par
imediato para TableFooter, com mesma estrutura excepto o
naming.

- **Unit tests** em `entities/content.rs` (~6 = 3 pares):
  - Constructor default (`repeat=true`).
  - Constructor com `repeat=false`.
  - `is_empty` proxy via body.
  - `plain_text` recurse no body.
  - `PartialEq` cobertura (3 vias: body, repeat).
  - `map_text` recurse + preserva `repeat`.
  - **Cada um repetido para TableFooter** — total 12 unit
    tests aproximadamente.

- **Stdlib tests** em `stdlib/mod.rs` (~6 = 3 pares):
  - Defaults (`repeat=true`).
  - `repeat=false` explícito.
  - Sem body rejeitado.
  - `repeat=int` rejeitado.
  - Named arg desconhecido rejeitado.
  - **Cada um repetido para `native_table_footer`**.

- **Layout E2E tests** em `layout/tests.rs` (2 = 1 par):
  - `layout_table_header_renderiza_body_no_contexto_actual`
    — header renderiza body uma vez.
  - `layout_table_footer_renderiza_body_no_contexto_actual` —
    footer renderiza body uma vez.

- **Test integrativo** (1):
  - `layout_table_com_header_cell_footer_renderiza_tudo` —
    Table contendo TableHeader + TableCell + TableFooter
    renderiza os três conteúdos em ordem.

**Δ esperado**: +18 a +23 tests (range mais largo que P157B
porque par simétrico duplica naturalmente; documentar como
"par simétrico tem Δ ~2× single").

### .6 Propagação de hashes

`crystalline-lint --fix-hashes .` para propagar hash novo de
`entities/content.rs` aos prompts L0 que o referenciam.

Esperado "Nothing to fix" (refactor aditivo, paridade P157A/B).

---

## Verificação

Numerada para reporte de conclusão:

1. `cargo test` em workspace: **1353 + Δ** tests, zero falhas
   (Δ esperado +18 a +23).
2. `crystalline-lint`: zero violations.
3. Contagem variants Content: **56** (54 → 56; +2 par).
4. Contagem stdlib funcs: **46** (44 → 46; +2 par).
5. Cobertura Model: agregada inalterada (~50%) — sub-entradas
   `table.header` e `table.footer` adicionadas como
   `implementado parcial` (algoritmo repetição diferido per
   ADR-0054 graded). Cobertura estrutural cresce.
6. Hash actualizado em prompts L0 (`crystalline-lint --check-hashes`
   passa) ou hash preservado (passo aditivo per P157A/B).
7. **DEBT-56 permanece aberto** (algoritmo repetição diferido);
   confirmar no relatório §"Estado pós-passo".
8. Paridade interna TableHeader↔TableFooter: confirmar que
   tests, pattern-match e estrutura são simétricos excepto
   naming.
9. ADR-0064 Caso D **primeira aplicação concreta em Model**;
   patamar Caso D N=3 (Layout) → N=4 (cross-domínio).
10. `layout_grid` original NÃO modificado (paridade P157A #8 e
    P157B #10).
11. **"Table foundations" declarado em ADR-0060 fica fechado**
    com P157A + P157B + P157C; confirmar no relatório
    §"Estado pós-passo".

---

## Critério de conclusão

- Verificações 1-11 passam.
- Relatório separado em
  `00_nucleo/materialization/typst-passo-157c-relatorio.md`
  produzido com:
  - Resumo do executado.
  - Confirmação de cada verificação.
  - **§análise de risco** (padrão N=9 → 10; décima aplicação
    consecutiva — primeiro par simétrico em Model;
    documentar peso real do risco vs cerimonial).
  - Slope cumulativo Model (mesa P155-P157C).
  - **Mesa fechamento "table foundations"**: P157A + P157B +
    P157C como conjunto materializado per ADR-0060 §"Decisão 1"
    sub-passo 3.
  - ADR-0061 §"Aplicações cumulativas" anotada com P157C.
  - **Confirmação**: ADR-0064 Caso D primeira aplicação Model;
    Casos A/B/C/D agora todos validados em Layout E em Model
    (auto-validação cross-domínio cross-caso completa).
    ADR-0065 critério aplicável (#2 escolha de tipo se
    inventário .1 detectar decisão significativa em helper).
  - **DEBT-56**: confirmar permanência aberto e justificar
    decisão graded.

---

## O que pode sair errado

**Cenários gerais**:
- Inventário .1 revela que vanilla `TableHeader` tem field
  obrigatório não previsto (e.g. `level: usize` sem default
  per spec hierárquica) → ajustar variant para incluir field
  ou registar como ADR-0054 graded explícita; documentar.
- Inventário .1 revela que `TableFooter` em vanilla não tem
  paridade com `TableHeader` (e.g. footer não suporta
  `repeat-rows`) → ajustar paridade interna cristalina ao que
  vanilla suporta; tratar como pseudo-simétrico em vez de
  totalmente simétrico.

**Cenários específicos**:
- Helper `extract_bool_with_default` ter complexidade superior
  ao esperado (e.g. interactuar com auto/none) → fallback para
  parse trivial inline; sem helper. Documentar.
- Pattern-match exhaustive falhar em variant existente fora
  de `content.rs` (paridade P157A/B) → grep por `Content::`
  em todo o crate.
- Tests E2E "header renderiza antes do conteúdo da table"
  divergir do comportamento P157C (que ignora ordem semântica
  e renderiza body literal) → tests E2E focados em
  **renderização do body**, não em ordem semântica que requer
  algoritmo de repetição.
- Decisão de paridade interna ser quebrada por divergência
  vanilla TableHeader vs TableFooter → registar limitação
  empírica; tornar tests assimetricamente quando justificado.

---

## Notas operacionais

- **Décima segunda aplicação de materialização**. Patamar
  empírico forte. Sem reformulação esperada (mantém precedente
  N=11 sem reformulação).
- **§análise de risco no relatório** com peso real (décima
  aplicação consecutiva). Primeira aplicação Model com par
  simétrico — documentar como precedente para futuros pares.
- **DEBT-56 permanece aberto**. P157C contribui para
  fechamento futuro armazenando o field necessário ao algoritmo
  (`repeat: bool`), mas não fecha por si. DEBT-56 cobre
  multi-region (column flow + repeat de header/footer); decisão
  sobre fechamento fica para refactor dedicado (escopo L+).
- **`extract_bool_with_default`** se for novo helper:
  candidato a `pub(super)` se outro passo Model precisar
  (e.g. P158 figure-kinds com `caption-position: bool`).
- **ADR-0064 Caso D patamar Model**: primeira aplicação
  concreta. **Auto-validação ADR-0064 cross-domínio cross-caso
  completa**: Casos A, B, C, D todos com aplicações concretas
  em pelo menos 2 domínios após P157C. Patamar empírico ADR
  meta atinge maturidade.
- **Par simétrico**: precedente claro para pares futuros
  (e.g. `figure.caption` + `figure.numbering`?). Documentar
  como subpadrão emergente de "tratamento simétrico" se P158/
  P159 também usarem.
- **"Table foundations" fechado**: ADR-0060 §"Decisão 1"
  sub-passo 3 fica integralmente materializado com P157A +
  P157B + P157C. Marca conceptual importante — primeira fase
  Model (Fase 2) fechada com 3 sub-passos M cada, granularidade
  preservada N=10/11/12.

---

## Pós-passo

Após conclusão de P157C:

**Layout fica em 78% inalterado**. **Model agregado fica em
~50%** (inalterado; ganho qualitativo). **DEBT-34e e DEBT-56
permanecem abertos**. **"Table foundations" declarado em
ADR-0060 fica fechado**.

**Próxima decisão** (sem candidata pré-acordada — reabre lista
das 6 candidatas remanescentes pós-P157A/B):

1. **P158** — figure-kinds (reservado per documento de estado
   pós-P156I; continua Model Fase 3? ou Fase 2 expandida?
   verificar ADR-0060). Granularidade N=13 candidata.
2. **P159** — bibliography + cite (reservado; Model). DEBT-55
   relacionado; ADR-0062 reservada hayagriva.
3. Continuar Fase 3 Layout (columns/colbreak — DEBT-56;
   quebra granularidade; inclui repeat de header/footer per
   §6 deste enunciado).
4. Footnote area (sub-fase prioritária ADR-0061 Decisão 5).
5. Promover ADR-0061 a IMPLEMENTADO (3 caminhos documentados;
   caminho 1 a 50% pós-P156J).
6. Promover `extract_length` a helper público (N=7 patamar
   forte).
7. Atacar Introspection (17% cobertura).
8. Fechar DEBT-34e e DEBT-56 (refactor placement Grid +
   multi-region; escopo L+; pode ser único refactor ou par).

ADR-0060 mantém-se `IMPLEMENTADO` (Fase 1 fechada em P155;
Fase 2 fechada em P157C). **Possível promoção de ADR-0060 a
revisão R1** com confirmação de Fase 2 fechada — decisão
documental se prioritária.

ADR-0061 mantém-se `PROPOSTO` (Layout não tocado).

Padrão granularidade 1-2 features/passo (N=12 com P157C se
fechar sem reformulação) **NÃO** é formalizado em ADR. Continua
candidato. **Patamar empírico cross-domínio cross-caso completo**
(Casos A/B/C/D ADR-0064 todos em Layout e Model) atinge
saturação — possível ADR meta de "ADR-0064 caso completion"
candidato administrativo XS futuro.

**Pausa natural após P157C — table foundations fechado;
ADR-0064 atinge saturação cross-domínio cross-caso; padrões
P156-P157 consolidados. Decisão humana sobre próxima direcção
tem máxima informação acumulada.**
