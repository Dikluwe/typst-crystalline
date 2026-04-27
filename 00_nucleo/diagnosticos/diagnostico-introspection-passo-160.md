# Diagnóstico Introspection — Passo P160

Diagnóstico de módulo precedendo materialização Introspection
per **ADR-0034 + ADR-0065 critério #5** (scope determinado por
inventário). **Quarto diagnóstico de módulo focado** (P157
table foundations / P158 figure-kinds / P159 bibliography+cite
/ **P160 Introspection**) + **primeira mudança de módulo
cross-domínio Model → Introspection** desde início da série
granular P156C. **Vigésima quarta aplicação consecutiva** do
padrão diagnóstico-primeiro.

---

## §1. ADRs/DEBTs Introspection

### 1.1 ADR-0017 estado factual

**Reserva sem ficheiro** pré-existente ("Introspection runtime
adiada"). Confirmações:
- P156B inventário 148 §A.9 — "Passo 17 ADR-0017 adiou; runtime
  não está pronto" para `here`/`locate`/`query`/`metadata`/
  `position`/`state`.
- P159B §3 categoria A — bloqueador transversal para `cite`
  cross-document refs + `measure` + outras features runtime.
- P159A §"Sem validação cross-reference" `Cite.key ∈
  Bibliography.keys` — refere ADR-0017 explicitamente.

**Sem ficheiro `00_nucleo/adr/typst-adr-0017-*.md`** confirmado.
Reserva apenas em comentários/relatórios. Promoção a PROPOSTO
via passo administrativo XS análogo a ADR-0062-create é
candidato pendente.

### 1.2 ADRs aplicáveis

- **ADR-0017 (reserva sem ficheiro)**: bloqueia features
  runtime queries (`here`/`locate`/`query`/`state`/cross-doc
  refs/`measure`).
- **ADR-0033** (paridade observable): fundamento para
  comportamento single-pass cristalino divergente do vanilla
  multi-pass.
- **ADR-0054** (perfil graded): fundamento para subset minimal
  Introspection (counters por kind sem runtime queries).
- **ADR-0065** critério #5 (scope determinado por inventário):
  P160 é a aplicação cumulativa.

### 1.3 DEBTs relacionados

Procura em `00_nucleo/DEBT.md`:
- **DEBT-18** (resolvido): "TOC mostraria valores de contadores
  no início do documento". Resolvido por
  `introspect::materialize_time` (P66 DEBT-18 fechado).
- **DEBT cross-document refs**: NÃO existe DEBT formal — refere
  ADR-0017 directamente (referência indirecta).
- **DEBT measure**: idem; ADR-0017 directo.
- **DEBT-55** (Bibliography + Cite XL): pré-condição ADR-0062
  hayagriva (Bloco B P159 não-Bloco A); ortogonal a P160.

**Conclusão §1.3**: Introspection não tem DEBTs próprios formais
— ADR-0017 captura tudo. Promoção a PROPOSTO desbloquearia
matérialização runtime queries.

---

## §2. Inventário código actual Introspection

### 2.1 Ficheiros e dimensões

- `01_core/src/rules/introspect.rs`: **1108 linhas**.
- `01_core/src/entities/counter_state.rs`: **333 linhas**.
- Ambos com hash @prompt-hash preservado L0-baseline (P156L →
  P159G N=17 consecutivos para content.rs; counter_state.rs
  preservado P158B/P159C/F/G).

### 2.2 Funcionalidades materializadas

**`introspect.rs` (1108 linhas)**:
- `pub fn introspect(content: &Content) -> CounterState` (linha
  21) — entry point pré-passagem analítica.
- `fn materialize_time(content, state) -> Content` (linha ~36)
  — substitui CounterDisplay por valor estático no momento
  exacto da introspecção (DEBT-18 fechado).
- `fn walk(content, state)` (linha 247) — pré-passagem
  recursiva sobre Content tree:
  - DFS recursivo single-pass.
  - Avança counters (heading hierárquico; equation flat;
    figure por kind).
  - Popula `resolved_labels` (Heading + Figure + Equation com
    label).
  - Popula `headings_for_toc` (com body materialize_time).
  - Popula `figure_numbers` por kind + `figure_label_numbers`.
  - Popula `bib_entries` (P159C) + `bib_numbers` (P159F).
  - Sinaliza `has_outline` se encontrar `Content::Outline`.

**`counter_state.rs` (333 linhas)**:
- Struct `CounterState` com 14 fields públicos:
  - `numbering_active` (HashMap; numbering on/off por kind).
  - `resolved_labels` (HashMap Label → text).
  - `headings_for_toc` (Vec).
  - `auto_label_counter` (gerador de labels automáticas).
  - `label_pages` (Layout-side; populado em layout).
  - `known_page_numbers` (Layout-side; iteração anterior).
  - `has_outline` (bool — fixpoint trigger).
  - `is_readonly` (Layout-side).
  - `figure_numbers` (HashMap por kind — Vec números).
  - `figure_label_numbers` (HashMap label → número).
  - `local_figure_counters` (auxiliar interno).
  - `lang: Option<Lang>` (P158B; subpadrão #15 N=1).
  - `bib_entries: Vec<BibEntry>` (P159C; subpadrão #15 N=2).
  - `bib_numbers: HashMap<String, u32>` (P159F; subpadrão #15 N=3).
- Métodos públicos: `new`, `is_numbering_active`, `step_hierarchical`,
  `format_hierarchical`, `step_flat`, `update_flat`, `get_flat`,
  `display_value`.

### 2.3 Variants Content cobertos por walk

Walk arm match trata 30+ variants Content (todos os existentes
em `Content` enum). Fallback `_ => {}` ou `Content::Empty` para
leaves sem efeito introspection.

**Cobertura interna walk**: 100% dos variants Content existentes
(58). Cada novo variant Content em sub-passos futuros adiciona
arm walk se necessário (paridade P157A/B/C/P159A/C).

### 2.4 Comportamento single-pass

**Pipeline cristalino**: `eval` → `introspect` (single-pass)
→ `layout` → `export_pdf`.

**Paridade vanilla**: vanilla usa multi-pass com `comemo` cache
e fixpoint convergence. Cristalino simplifica para single-pass
(documentado em `counter_state.rs::DEBT-10` comentário). Refino
multi-pass diferido a passos futuros se features cross-document
forem materializadas.

**Comentário relevante** em `counter_state.rs:23-28`:
> "Cristalino diverge do Typst original aqui: o original resolve
> contadores em duas passagens com `comemo` (para suportar
> referências para a frente). Esta implementação usa uma única
> passagem — suficiente para numeração sequencial de headings
> e contadores planos. DEBT-10: Resolver contadores em duas
> passagens com estado global quando o motor de introspecção
> completo for implementado (Passos 60+)."

**DEBT-10 estado**: comentário antigo (Passos 60+). Trabalho
parcial cumprido via `materialize_time` P66 (DEBT-18 fechado).
Multi-pass real continua diferido per ADR-0017.

### 2.5 Subpadrão #15 "infraestrutura state lookup" N=3

Estado cumulativo:
- P158B `state.lang` (lang resolution para supplements
  localizados).
- P159C `state.bib_entries` (cross-reference Cite ↔
  Bibliography para forms).
- P159F `state.bib_numbers` (numbering numérico Cite Normal/None).

**Patamar atinge limiar formalização N=3-4**. Promoção a ADR
meta possível em passo administrativo XS futuro NÃO reservado.
Subpadrão demonstra viabilidade single-pass para features que
não exigem runtime queries genuínas.

---

## §3. Features Introspection vanilla

Baseado em `lab/typst-original/crates/typst-library/src/introspection/`:

### 3.1 Tabela features × cobertura cristalina

| Feature | Vanilla LOC | Cristalino estado | Dependência runtime (ADR-0017) | Dependência measure (Layout) | Dependência multi-pass |
|---------|------------:|-------------------|:------------------------------:|:----------------------------:|:----------------------:|
| `counter()` | 991 | **`implementado`** (subset minimal) | NÃO (P60-62 single-pass) | NÃO | NÃO |
| `state()` | 522 | **`ausente`** | SIM (mutable state runtime) | NÃO | SIM |
| `here()` | 49 | **`ausente`** | SIM (current location) | NÃO | NÃO |
| `locate()` | 41 | **`ausente`** | SIM (position-aware computations) | NÃO | NÃO |
| `query()` | 285 | **`ausente`** | SIM (runtime introspection) | NÃO | SIM |
| `metadata()` | 30 | **`ausente`** | SIM (arbitrary metadata) | NÃO | NÃO |
| `position()` | 167 | **`ausente`** | SIM (target → location) | NÃO | NÃO |
| `convergence()` | 287 | **`ausente`** | SIM (fixpoint logic) | NÃO | SIM |
| `introspector` (struct) | 695 | **`ausente`** | SIM (engine) | NÃO | SIM |
| `location` (type) | 385 | **`ausente`** | SIM (location identifier) | NÃO | NÃO |
| `locator` (type) | 395 | **`ausente`** | SIM (locator engine) | NÃO | NÃO |
| `tag` (type) | 91 | **`ausente`** | SIM (tag locator) | NÃO | NÃO |
| `measure(body)` | (em layout/) | **`parcial`** (helper privado `measure_content`; sem stdlib expose) | NÃO directamente | SIM (Layout integration) | NÃO |

**Total LOC vanilla introspection/**: ~3983 linhas.
**Cobertura cristalina**: 1/13 implementado (counter); 1/13
parcial (measure helper privado); 11/13 ausentes.

### 3.2 Cobertura observable (paridade tabela A.9)

Tabela cobertura A.9 lista 6 features visíveis ao utilizador:
- `counter(key)` ✓ implementado.
- `state(key, ...)` ✗ ausente.
- `here()` / `locate()` ✗ ausente.
- `query(...)` ✗ ausente.
- `metadata(value)` ✗ ausente.
- `position(target)` ✗ ausente.

**Cobertura agregada A.9**: 1/6 = ~17% (per inventário 148).

### 3.3 Categorização por dependência

**Categoria 1 — Sem dependência ADR-0017** (single-pass viável):
- `counter()` ✓ já implementado.
- (Nenhuma outra feature observable single-pass óbvia — todas
  as restantes requerem runtime queries ou `Location`).

**Categoria 2 — Dependência ADR-0017** (runtime queries):
- `state()`, `here()`, `locate()`, `query()`, `metadata()`,
  `position()`, `convergence`, `introspector`, `location`,
  `locator`, `tag`.

**Categoria 3 — Dependência cross-módulo** (Layout integration):
- `measure()` (parcial; depende `layout/measure.rs`).

**Categoria 4 — Refinos qualitativos sem features novas**
(single-pass viável):
- Subpadrão #15 N=3 (state lookup) demonstra viabilidade.
- Refinos potenciais: `state.figure_kinds` para query de tipos
  de figuras presentes; `state.heading_levels_present` para
  query de levels de headings; etc. — análogos a `bib_entries`
  e `bib_numbers`.
- **Sem features observable user-facing** — só infraestrutura
  internal.

---

## §4. Análise tecto Introspection

### 4.1 Tecto Introspection puramente single-pass (sem ADR-0017)

**Resposta**: **~17%** (inalterada). Counters por kind já
implementado. Outras features observable user-facing exigem
ADR-0017 promovida.

**Refinos qualitativos possíveis sem ADR-0017**:
- Mais fields aditivos em `CounterState` para state lookup
  cross-domínio (subpadrão #15 N=3 → 4+).
- Novas walk arms para variants Content novos (paridade P157A/B/
  C/P159A/C).
- **NÃO movem cobertura agregada Introspection** — são
  infraestrutura internal sem features observable user-facing.

### 4.2 Tecto Introspection pós-ADR-0017 promovida

**Resposta**: **~83-100%** dependendo do scope minimal vs
completo per ADR-0054 graded.

**Subset minimal pós-ADR-0017**:
- `here()` / `locate()` / `query()` / `metadata()` / `state()`
  / `position()` materializáveis com runtime queries simples.
- Subset minimal ~50% das features observable.
- Cobertura ~17% → ~50%.

**Subset completo pós-ADR-0017 + measure**:
- Inclui `measure()` materializado (depende Layout integration).
- Cobertura ~17% → ~83-100%.

### 4.3 Diferença tecto puro vs pós-resolver

**Diferença empírica**:
- **+0pp** com refinos qualitativos puros (sem ADR-0017).
- **+33-66pp** pós-ADR-0017 promovida (subset minimal a
  completo).
- **+8-16pp** adicionais pós-`measure` materializado (depende
  cross-módulo).

**Tecto puro Introspection é trivialmente saturado**: counter()
já cobre os ~17% atingíveis sem ADR-0017. Refinos qualitativos
adicionam infraestrutura internal mas **não movem cobertura
agregada**.

### 4.4 Conclusão tecto

**Decisão arquitectural-chave §4**: **tecto Introspection puro
está EFECTIVAMENTE SATURADO**. Counter() já cobre os ~17%
atingíveis sem ADR-0017. Outros refinos qualitativos são
infraestrutura internal (subpadrão #15) sem impacto observable.

**Implicação**: Bloco A está **VAZIO** (paridade cenário §"O
que pode sair errado" do enunciado P160).

**Caminho válido per spec P160**: recomendação §6 muda para
"promover ADR-0017 via passo administrativo XS antes de qualquer
materialização Introspection observable". Análogo ao precedente
ADR-0062-create para Bibliography/Cite Bloco B.

---

## §5. Sequência candidata sub-passos materializáveis

### 5.1 Bloco A — Features sem dependência ADR-0017

**VAZIO**. Tecto puro Introspection trivialmente saturado por
counter() já implementado.

**Refinos qualitativos infraestrutura possíveis** (sem features
observable):
- **R1**: `state.figure_kinds: HashSet<String>` — para query
  futura de tipos de figuras presentes no documento. Subpadrão
  #15 cresce N=3 → 4. Tamanho: XS.
- **R2**: `state.heading_levels_present: BTreeSet<u8>` — para
  query futura de levels de headings presentes. Subpadrão #15
  cresce N=3 → 4. Tamanho: XS.
- **R3**: `state.equations_count: u32` — para query futura
  total equations. Subpadrão #15 cresce N=3 → 4. Tamanho: XS.

**Estes refinos NÃO movem cobertura observable Introspection
A.9**. Justificam-se apenas como pré-condição para features
runtime queries futuras (Bloco B com ADR-0017 promovida).

### 5.2 Bloco B — Features com dependência ADR-0017

Lista para informação (NÃO materializável em P160 puro):

| Identificador candidato | Feature | Tamanho est. | Pré-condição |
|-------------------------|---------|:------------:|--------------|
| P160A (após ADR-0017-create) | `state(key, init)` runtime mutable state | M | ADR-0017 PROPOSTO |
| P160B | `metadata(value)` arbitrary attaching | S+ | ADR-0017 PROPOSTO |
| P160C | `here()` / `locate()` current location | M | ADR-0017 PROPOSTO + `Location` type |
| P160D | `query(target)` runtime introspection | M+ | ADR-0017 PROPOSTO + `Location` + `query` engine |
| P160E | `position(target)` location-aware | S+ | depende P160C |

### 5.3 Bloco C — Features com dependência cross-módulo

Lista para informação (NÃO materializável em Introspection puro):

| Identificador candidato | Feature | Bloqueador |
|-------------------------|---------|------------|
| P160F (cross-módulo) | `measure(body)` stdlib expose | Layout integration (`measure_content` actual é privado; expose stdlib requer integração mais ampla) |
| Cross-document refs | Cite cross-document key resolution | ADR-0017 + multi-document pipeline |
| (cross-módulo) | Layout-aware introspection | Layout 2-pass refactor |

---

## §6. Recomendação de execução

### 6.1 Recomendação primária

**Promover ADR-0017 via passo administrativo XS** análogo a
ADR-0062-create. Justificação:

- Bloco A vazio (tecto puro saturado por counter()).
- Refinos qualitativos R1/R2/R3 são infraestrutura sem features
  observable — adiamento até prioridade clara é razoável.
- Bloco B com 5 candidatos materializáveis pós-ADR-0017
  PROPOSTO → desbloqueia ~33-50pp Introspection.
- Paridade precedente: ADR-0062-create XS administrativo
  desbloqueou Bloco B P159 (hayagriva pendente).

**Identificador sugerido**: **`ADR-0017-create`** (paridade
naming `ADR-0062-create`).

### 6.2 Alternativas avaliadas

**Opção B — Materializar refinos qualitativos R1/R2/R3 primeiro**:
- Pro: subpadrão #15 cresce N=3 → 4-6; consolida pattern.
- Con: sem ganho observable; trabalho de 3 passos XS sem mover
  cobertura; risco de "yak-shaving" sem prioridade clara.
- **Rejeitada** — esperar até prioridade observable surgir.

**Opção C — Mudar de módulo (Layout Fase 3 columns/colbreak ou
outro)**:
- Pro: Layout Fase 3 tem features observable concretas
  (columns/colbreak ~5-10pp Layout); Cobertura arquitectural
  pode crescer.
- Con: deixa Introspection saturada sem promover ADR-0017;
  pode atrasar features cross-document (cite refs).
- **Considerada — Opção C válida para sessão alternativa**.

**Opção D — `measure()` cross-módulo materialização (Bloco C)**:
- Pro: feature parcial → implementado (Layout integration).
- Con: cross-módulo significativo; depende Layout helpers
  públicos; risco médio-alto.
- **Diferida** — esperar até ADR-0017 promovida + necessidade
  concreta.

### 6.3 Recomendação final

**Recomendação**: executar **`ADR-0017-create`** como próximo
passo (XS administrativo; paridade `ADR-0062-create`).

**Pós-`ADR-0017-create`**: redirigir para **P160A** (state(key,
init) runtime mutable state; M) como primeiro candidato Bloco B.

**Validação humana**: sujeita a aprovação. Alternativa preferida
se Layout Fase 3 for prioridade: Opção C (mudar de módulo).

### 6.4 Estimativa sub-passos Introspection alcançáveis

Pós-`ADR-0017-create`:
- 5 candidatos Bloco B (P160A-E) × M each = 5 sub-passos M.
- Cobertura Introspection ~17% → ~50% pós-Bloco B subset
  minimal.
- Pós-`measure()` cross-módulo: ~50% → ~67-83%.
- Pós-cross-document refs (DEBT futuro): ~83% → ~100%.

**Saturação Introspection esperada após ~5-7 sub-passos**
pós-ADR-0017-create.

---

## Resumo executivo

P160 inventaria módulo Introspection (módulo cristalino mais
fraco com 17% cobertura) per ADR-0034 + ADR-0065 critério #5.
**Quarto diagnóstico de módulo focado** + **primeira mudança
de módulo cross-domínio Model → Introspection**.

**Decisões arquitecturais P160**:
- **Bloco A vazio** — tecto Introspection puro trivialmente
  saturado por counter() já implementado.
- **ADR-0017 confirmada como reserva sem ficheiro** — promoção
  a PROPOSTO via XS administrativo é candidato pendente.
- **Subpadrão #15 N=3** atinge limiar formalização (state.lang
  + state.bib_entries + state.bib_numbers).
- **Recomendação primária**: `ADR-0017-create` (XS administrativo)
  paridade `ADR-0062-create`. Pós-promoção: P160A (state runtime).
- **Alternativa válida**: Opção C (Layout Fase 3 columns/
  colbreak) se prioridade observable Layout for maior.

**Decisões diferidas (NÃO reservadas)**:
- Refinos qualitativos R1/R2/R3 (infraestrutura sem observable).
- `measure()` cross-módulo (Bloco C; depende Layout integration).
- Cross-document cite refs (ADR-0017 + multi-document pipeline).

**ADRs aplicadas**:
- ADR-0034: diagnóstico cumprido.
- ADR-0017: estado factual confirmado (reserva sem ficheiro).
- ADR-0033: paridade observable single-pass.
- ADR-0054: graded subset minimal.
- ADR-0065 critério #5: décima segunda aplicação concreta com
  diversidade cross-domínio nova.

**Conclusão**: P160 documenta empiricamente o tecto Introspection
puro (saturado em ~17%). **Recomendação primária ADR-0017-create**
sujeita a validação humana. **Alternativa Opção C válida** se
prioridade Layout Fase 3 for maior.

**Risco**: baixo — passo diagnóstico puramente documental;
sem código alterado; sem ADR nova; sem novas reservas.
