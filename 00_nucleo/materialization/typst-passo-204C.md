# Passo 204C — Layouter ganha `'a`; field `introspector` migra para `Tracked<'a, dyn Introspector + 'a>`

**Série**: 204 (sub-passo `C` = implementação Layouter
após P204B foundational).
**Tipo**: implementação cross-modular.
**Magnitude planeada**: M.
**Pré-condição**: P204B concluído; `#[comemo::track]`
aplicado ao trait `Introspector`; trait fica
`Send + Sync`; 3 Hash impls em Value/BibEntry/Content;
3 sentinel tests activos; tests 1827 verdes; 0 violations;
ADR-0073 PROPOSTO em vigor.
**Output**: 3 ficheiros (inventário + relatório +
alterações de código).

---

## §1 Propósito

Migrar `Layouter` de `&dyn Introspector` para
`Tracked<'a, dyn Introspector + 'a>` em paridade com
vanilla typst (per A8 da auditoria P204A; per C3 do
diagnóstico P204A — alternativa **b**).

Trabalho concreto:

- Layouter ganha `'a` lifetime parameter.
- Field `introspector` muda de tipo concreto para
  `Tracked<'a, dyn Introspector + 'a>`.
- Construção via `track_with` (decisão do humano —
  alternativa fixada na clarificação inicial).
- ~10 consumers Layouter migrados.
- Wrapper de retrocompatibilidade na API pública —
  decisão fixada no inventário inicial com base em
  call sites externos.

P204C respeita a convenção P203 §9.1: começa com
inventário empírico antes de qualquer alteração.

---

## §2 Material de partida verificado em P204B

Antes de qualquer alteração, confirmar empíricamente:

- Trait `Introspector` em
  `01_core/src/entities/introspector.rs:37-164` com
  `#[comemo::track]` e bounds `Send + Sync`.
- `TagIntrospector` é `Send + Sync` automático (per
  P204B C3+C4).
- `Layouter` struct em path a confirmar (provavelmente
  `01_core/src/rules/layout/layouter.rs` ou
  `mod.rs`); 22 fields (per snapshot 2026-05-05 §5);
  field actual `introspector: TagIntrospector` (a
  confirmar).
- 3 sentinel tests P204B activos.

Sem isto, recuar para P204B.

---

## §3 Cláusulas de execução (sem condicionais)

### C1 — Inventário empírico inicial

Antes de tocar em código, listar literalmente:

- Caminho real do struct `Layouter`.
- Fields actuais (esperado: 22).
- Field actual de introspector — nome exacto, tipo
  exacto, visibilidade.
- Métodos do `impl Layouter` que consomem
  `self.introspector.<método>(...)` — esperado: ~10.
- Tipos `M: FontMetrics, S: ImageSizer = NullImageSizer`
  já parametrizados — confirmar.
- API pública: `pub fn layout(...)` — assinatura
  completa.
- Call sites externos de `pub fn layout`:
  - Em `02_shell/`.
  - Em `03_infra/`.
  - Em `04_wiring/`.
  - Em tests integrados.
- Construtores actuais do Layouter — onde é instanciado.

Output: tabela de fields + tabela de consumers + tabela
de call sites externos.

Critério: cada item etiquetado CONFIRMADO ou
**AJUSTE NECESSÁRIO**. Se houver ajuste estrutural não
trivial, registar em `P204C.div-N`.

### C2 — Decisão sobre wrapper de retrocompatibilidade

Com base em C1 (call sites externos), fixar uma das
duas alternativas:

- **Wrapper sim** — `pub fn layout` mantém assinatura
  pública sem `'a` exposto; constrói `Tracked` internamente
  via `track_with`. Aceita `&dyn Introspector` ou
  `TagIntrospector` por valor; converte.
- **Wrapper não** — `'a` atravessa para a API pública;
  call sites externos adaptam.

Critério para escolha: número de call sites externos +
estabilidade da API. Se call sites externos forem 0–2 e
controlados, "não" é viável. Se forem 3+ ou em crates
externos, "sim" é preferível.

C2 fixa **uma** alternativa. Sem ramos.

### C3 — Layouter struct — adicionar `'a`

Edição literal:

```text
- pub struct Layouter<M: FontMetrics, S: ImageSizer = NullImageSizer> {
+ pub struct Layouter<'a, M: FontMetrics, S: ImageSizer = NullImageSizer> {
      // 22 fields, com `introspector` modificado.
  }
```

Field `introspector`:

```text
- pub(super) introspector: TagIntrospector,
+ pub(super) introspector: Tracked<'a, dyn Introspector + 'a>,
```

`use comemo::Tracked;` adicionado se necessário.

### C4 — `impl<...> Layouter<...>` — adicionar `'a`

Cada `impl` block do Layouter ganha `'a` correspondente:

```text
- impl<M: FontMetrics, S: ImageSizer> Layouter<M, S> {
+ impl<'a, M: FontMetrics, S: ImageSizer> Layouter<'a, M, S> {
      // métodos.
  }
```

Critério: todos os impl blocks em concordância. Provável
contagem: 3–5 impls (a confirmar em C1).

### C5 — Construção via `track_with`

Construtor actual do Layouter (esperado em
`pub fn layout` ou em factory dedicada) muda:

```text
- let intr = TagIntrospector::empty();
- let mut layouter = Layouter::new(intr, ...);
+ let intr = TagIntrospector::empty();
+ comemo::track_with!(intr, |tracked| {
+     let mut layouter = Layouter::new(tracked, ...);
+     // resto do pipeline.
+ });
```

Notação ilustrativa. API exacta de `track_with`
confirmada em P204A A6 (lib.rs do crate comemo).

C5 substitui qualquer construção que aceitasse
`TagIntrospector` por valor por construção que aceita
`Tracked`.

### C6 — Migração dos ~10 consumers

Para cada call site `self.introspector.<método>(...)`:

- Sintaxe permanece (Tracked deref-coerces para acesso a
  métodos do trait).
- Tipos retornados podem mudar de `&T` para `T` em alguns
  métodos (`Tracked` materializa por clone). Verificar
  cada caso.

Output: tabela de consumers × call site × ajuste.

Casos prováveis de ajuste:
- Métodos que retornam `&Value` (state_value,
  state_final_value) — agora retornam `Value` por valor.
- Métodos que retornam `&str` (resolved_label_for) —
  agora retornam `String`.
- Métodos que retornam `&BibEntry` — agora retornam
  `BibEntry`.
- Métodos que retornam `&[...]` (query_metadata,
  headings_for_toc) — agora retornam `Vec<...>`.

Cada call site adaptado individualmente. Sem
condicionais.

### C7 — Adaptação de tests

Tests que constroem `Layouter` directamente precisam de
adaptação:

- Construir `TagIntrospector` como antes.
- Aplicar `track_with!` (ou padrão equivalente) para
  obter `Tracked`.
- Passar `Tracked` ao construtor.

Critério: tests workspace verdes (1827 mantém-se ou
oscila por tests reescritos).

### C8 — Compilação

```
cargo build --workspace 2>&1 | tail -20
```

Critério: verde. Caso falhe, listar cada erro literal e
resolver. Hipóteses prováveis:

- Lifetime mismatch em construtor.
- `Tracked` não Sized em campo (resolver com bound
  explícito).
- Coerção `&dyn Introspector` → `Tracked<dyn Introspector>`
  precisa de chamada explícita a `.track()` em algum
  lugar.

### C9 — Tests workspace

```
cargo test --workspace 2>&1 | tail -30
```

Critério: 1827+ tests verdes. P204C pode adicionar 0 a
3 sentinelas dedicadas (C10).

Caso algum test falhe, identificar e resolver.

### C10 — Sentinelas dedicadas (opcional, recomendado)

3 sentinelas candidatas:

- **`p204c_layouter_ganha_lifetime`** — falha de
  compilação se `'a` for removido da declaração.
- **`p204c_introspector_field_e_tracked`** — falha de
  compilação se field voltar a `TagIntrospector`.
- **`p204c_pipeline_end_to_end_via_tracked`** —
  constrói `Layouter` com `Tracked`, executa layout
  pequeno, asserir output.

Decisão dentro de P204C. Recomendado pelo menos 1.

### C11 — Linter

```
crystalline-lint .
```

Critério: 0 violations.

Hipóteses prováveis de violação nova:

- Regra de visibility para `Tracked` (lifetime aparece
  em tipo público; pode triggerar regra L1).
- Regra que limita uso de macros em L1 — `track_with!`
  fica em L4 wiring (per ADR-0001).

C11 termina com 0 violations. Resolver caso a caso.

### C12 — Documentação ADR-0073

Não modificar ADR-0073 PROPOSTO em P204C. ADR transita
para ACEITE estrutural quando a maioria dos sub-passos
B+ está completa (provavelmente em P204G ou P204H).

P204C apenas confirma migração Layouter bem sucedida.

### C13 — Critério de fecho de P204C

P204C está concluído quando:

- C1 inventário completo.
- C2 wrapper-decisão fixada com justificação.
- C3+C4 Layouter ganha `'a` em struct e impls.
- C5 construção via `track_with` aplicada.
- C6 ~10 consumers migrados.
- C7 tests adaptados.
- C8 compilação verde.
- C9 tests workspace verdes.
- C10 sentinelas (mínimo 1, recomendado 3).
- C11 linter 0 violations.
- Inventário registado.
- Relatório escrito.

### C14 — Sem cláusulas condicionais

C1 produz dados empíricos. C2 fixa wrapper-decisão com
**uma** alternativa, não duas. C3–C12 executam o caminho
fixado por C2.

Caso C8/C9/C11 detectem problemas estruturais (não
triviais), registar em `P204C.div-N` e:
- Resolver dentro de P204C (preferido).
- Recuar para P204B/P204A se for obstrução de baseline.

---

## §4 Outputs concretos

### Ficheiro 1 — Inventário interno

Localização:
`00_nucleo/diagnosticos/typst-passo-204C-inventario.md`.

Conteúdo:
- §1 C1 — fields actuais + consumers + call sites
  externos.
- §2 C2 — wrapper-decisão fixada com justificação.
- §3 C5 — construção `track_with` aplicada.
- §4 C6 — tabela consumers × ajuste.
- §5 Decisões durante a leitura.

### Ficheiro 2 — Relatório

Localização:
`00_nucleo/materialization/typst-passo-204C-relatorio.md`.

Conteúdo:
- O que foi feito.
- Tempo de execução.
- Métricas (tests pre/post; LOC delta).
- Decisões durante a leitura.
- Sugestão para próximo sub-passo (P204D).

### Ficheiro 3 — Alterações em código

Não é ficheiro discreto. Conjunto de alterações em:

- `Layouter` struct (provavelmente
  `01_core/src/rules/layout/layouter.rs`).
- `impl Layouter` (mesmo ficheiro ou separado).
- ~10 consumers em `mod.rs`, `equation.rs`,
  `references.rs`, `outline.rs` (caminhos a confirmar
  em C1).
- Construtor pública e/ou wrapper.
- Tests adaptados.
- Sentinelas P204C.

---

## §5 Critério de progressão para P204D

P204C fechado quando C13 cumprido.

Em caso de divergência empírica relevante (ex: `Layouter`
não tem 22 fields, consumers usam padrão diferente do
esperado, `Tracked` não compatível com construtor
existente), registar em `P204C.div-N` e:

- Resolver dentro de P204C.
- Recuar para P204A re-fixar C3 com novos dados (se
  for estrutural).

P204D só começa quando P204C fechado.

---

## §6 Convenções mantidas

- Sem condicionais estruturais nas cláusulas de
  execução.
- 3 outputs (inventário + relatório + código).
- Inventário empírico antes de implementação (per
  convenção P203 §9.1).
- Localização canónica:
  `00_nucleo/diagnosticos/` para inventário;
  `00_nucleo/materialization/` para relatório.
- Distinção fecho estrutural vs final mantida.
- Sem inflação retórica.

---

## §7 Não-objectivos

P204C não:

- Aplica `#[comemo::track]` em outros traits (P204B
  cobriu o único trait alvo).
- Materializa Position (P204D).
- Adiciona `evict()` wrapper (P204E).
- Adiciona ficheiros ao corpus de paridade (P204F).
- Adiciona benchmarks (P204G).
- Transita ADR-0073 para ACEITE (P204H).
- Cria ADR nova.
- Toca em `TagIntrospector` impl (já validado em P204B).
- Toca em loops fixpoint (preservados per C7 P204A).
- Materializa sub-stores trackable separadamente
  (decidido contra em C4 P204A — granularidade per-method
  via trait track).

---

## §8 Erro a não repetir

Da série P203 + P204A/B — quatro detecções consecutivas
de premissas erradas em specs. Padrão correcto: cada
passo começa com inventário empírico.

P204C aplica isso em C1 com cuidado redobrado:

- Caminho exacto do `Layouter` confirmado (não assumido
  a partir do snapshot).
- Field exacto de introspector confirmado (nome, tipo,
  visibilidade).
- Lista de consumers confirmada empíricamente (~10 é
  estimativa, não decreto).
- Lista de call sites externos confirmada antes de C2.

A spec não pré-fixa a decisão sobre wrapper (C2). Fixa-se
com base em C1.

Hipótese específica a testar em C1: campo `introspector`
pode estar em sub-struct interno do Layouter (não
directamente no top-level struct), o que muda o local de
edição em C3.

---

## §9 Particularidade — execução

P204C é trabalho de código cross-modular:

- Modificação do `Layouter` struct (1 ficheiro
  principal).
- Modificação de impls (1–N ficheiros).
- Migração de ~10 consumers (3–5 ficheiros).
- Adaptação de tests (vários ficheiros).
- Possíveis ajustes em call sites externos (depende de
  C2).

Volume médio. Magnitude M.

Recomendado Claude Code dado:
- Volume de leitura para C1 (consumers + call sites
  externos).
- Necessidade de iteração rápida com cargo build (para
  resolver lifetime errors progressivamente).
- Migração mecânica de consumers que beneficia de
  edição estruturada.

Sessão actual viável se houver disponibilidade, mas o
padrão dos sub-passos M anteriores favorece Claude
Code.
