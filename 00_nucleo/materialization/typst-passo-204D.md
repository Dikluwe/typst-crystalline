# Passo 204D — Position concrete (tipo + runtime + `position_of`)

**Série**: 204 (sub-passo `D` = implementação Position
após P204C Layouter migration).
**Tipo**: implementação focada.
**Magnitude planeada**: S–M.
**Pré-condição**: P204C concluído; Layouter ganha `'a`;
field `introspector` é `Tracked<'a, dyn Introspector + 'a>`;
tests 1829 verdes; 0 violations; ADR-0073 PROPOSTO em
vigor; ADR-0066 ainda ACEITE com nota "intermediário até
M8" (transição superseding em P204H).
**Output**: 3 ficheiros (inventário + relatório +
alterações de código).

---

## §1 Propósito

Materializar Position concrete no cristalino — o concern
que ADR-0066 adiou para M8 e que P203 confirmou como
parte natural deste sub-marco.

Trabalho concreto:

- Tipo `Position` em L1 (réplica vanilla
  `PagedPosition { page: NonZeroUsize, point: Point }`,
  per P203A diagnóstico §3 + decisão fixada na
  clarificação inicial deste passo).
- `runtime.positions: HashMap<Location, Position>` em
  `LayouterRuntimeState` (per P203A diagnóstico §4 C3 —
  Layouter-runtime, não TagIntrospector).
- Layouter popula durante layout (single-pass; per
  P203A C4 — Layouter feedback).
- API do trait `Introspector::position_of` —
  decisão de migração de stub `Option<()>` para
  `Option<Position>` fixada no inventário inicial com base
  em verificação empírica.
- Tests E2E (2–3) cobrindo cenários canónicos.

P204D respeita a convenção: começa com inventário
empírico antes de qualquer alteração.

---

## §2 Material de partida verificado em P204C

Antes de qualquer alteração, confirmar empíricamente:

- Tipo `Point` existe em L1 (provavelmente
  `01_core/src/entities/point.rs` — confirmar caminho
  exacto).
- `LayouterRuntimeState` existe com 3 fields actuais
  (`label_pages`, `known_page_numbers`, `is_readonly`,
  per snapshot 2026-05-05 §5; P190C/D criou).
- Field `runtime: LayouterRuntimeState` no Layouter (a
  confirmar).
- Trait `Introspector::position_of` retorna
  `Option<()>` (stub estrutural) — confirmar.
- Vanilla typst `PagedPosition` em
  `lab/typst-original/crates/typst-library/src/introspection/`
  com forma `{ page: NonZeroUsize, point: Point }` (per
  P203A A2).

Sem isto, recuar para P204C.

---

## §3 Cláusulas de execução (sem condicionais)

### C1 — Inventário empírico inicial

Antes de tocar em código:

1. **Tipo Point** — confirmar:
   - Caminho real (`01_core/src/entities/...`).
   - Forma (`pub struct Point { x: f64, y: f64 }` ou
     similar).
   - Bounds derivados (`Hash`? `Eq`? `Clone`? `Copy`?).
2. **Vanilla PagedPosition** — confirmar:
   - Caminho real.
   - Forma exacta dos fields.
   - Como é construído (qual valor `page` recebe; qual
     valor `point`).
3. **LayouterRuntimeState** — confirmar:
   - Caminho real.
   - Fields actuais.
   - Como é populado durante layout.
4. **Trait Introspector** — `position_of`:
   - Linha exacta (per P204B inventário §1, era método
     #5).
   - Assinatura exacta (`fn position_of(&self, loc:
     Location) -> Option<()>`).
   - Consumers em produção (per P204A A3 — esperado
     zero).
   - Tests existentes que invocam `position_of`.
5. **Layouter — current_location e current_page**:
   - Como o Layouter conhece página corrente?
   - Como o Layouter conhece location corrente
     (`current_location`)?
   - Onde é o ponto de emissão natural de Position?
6. **Vanilla pipeline** — verificar A7 da auditoria:
   - Vanilla calcula Position post-layout (fase 3
     separada).
   - Cristalino diverge intencionalmente (single-pass
     durante layout).
   - Confirmar que essa divergência mantém paridade
     observable (Position resultante é o mesmo mapping).

Output: 6 sub-secções com etiqueta CONFIRMADO ou
**AJUSTE NECESSÁRIO**. Cada item com evidência empírica.

Se algum item revelar obstrução estrutural, registar em
`P204D.div-N` e re-fixar cláusula afectada.

### C2 — Decisão sobre API do trait

Com base em C1.4 + C1.5, fixar:

- **Migrar stub** — `position_of` muda de `Option<()>`
  para `Option<Position>`. Trait API ganha o tipo concreto.
- **Manter stub provisoriamente** — Position é populado
  em runtime mas trait API só migra em sub-passo
  posterior (P204E ou P204H).

Critério para escolha: existência de consumers actuais e
custo de migração. Per P204A A3, esperado zero consumers
em produção. Caso confirmado, "migrar stub" é trivial.

C2 fixa **uma** alternativa. Sem ramos.

### C3 — Forma do tipo Position

Decisão fixada na clarificação inicial: **réplica vanilla
`PagedPosition`**.

Forma literal (notação ilustrativa):

```text
pub struct Position {
    pub page: NonZeroUsize,
    pub point: Point,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]  // ou impls manuais
```

Bounds necessários:
- `Hash` — para satisfazer comemo (per P204B padrão).
- `Eq` ou `PartialEq` — para asserções.
- `Clone` ou `Copy` — para retornar de queries.
- `Debug` — para diagnósticos.

Se `Point` não derivar `Hash` ou `Copy` (a confirmar em
C1.1), aplicar mesmo padrão de P204B (Hash via Debug
formatting; Copy se Point for `{ f64, f64 }`).

Localização: `01_core/src/entities/position.rs`.

### C4 — Sub-store `runtime.positions`

Decisão fixada per P203A C3: **`LayouterRuntimeState`**,
não `TagIntrospector`.

Justificação re-iterada: walk-time não pode calcular
Position (per P203A A5); população single-pass durante
layout (per P203A C4); padrão "Layouter-runtime" já
estabelecido para state populado durante layout (P190C/D).

Edição literal:

```text
pub struct LayouterRuntimeState {
    pub label_pages: HashMap<Label, NonZeroUsize>,
    pub known_page_numbers: ...,
    pub is_readonly: bool,
+   pub positions: HashMap<Location, Position>,
}
```

`runtime` ganha 4º field.

### C5 — Layouter feedback single-pass

Decisão fixada per P203A C4 — Layouter feedback
single-pass (não fixpoint, não walk-time).

Mecanismo:

- Durante layout, sempre que Layouter processa um
  `Content` locatable (com `current_location: Some(loc)`),
  emite uma entry:
  ```text
  runtime.positions.insert(
      loc,
      Position {
          page: NonZeroUsize::new(current_page).unwrap(),
          point: Point { x: cursor_x, y: cursor_y },
      },
  );
  ```
- Idempotência: `insert` substitui. Se Layouter re-layouta
  (TOC fixpoint), segunda passagem sobrescreve com valores
  correctos.

Localização do `insert`: ponto canónico onde Layouter
processa locatable. A identificar em C1.5.

C5 não tem ramos. Sub-passo dependente de C1.5 — se
C1.5 revelar que não há ponto único canónico (vários
sites de processamento de locatables), agrupar em helper
function `record_position(layouter, loc)` chamado de cada
site.

### C6 — Trait `position_of` — implementação

Se C2 = "migrar stub":

```text
fn position_of(&self, loc: Location) -> Option<Position> {
    // implementação real: consulta a runtime.positions
    // — mas isto requer acesso ao runtime que não está
    // no TagIntrospector.
    todo!("delegado em consumer")
}
```

Aqui surge tensão estrutural. `TagIntrospector` (a impl
actual) não tem acesso a `runtime.positions` (que vive
no `Layouter`, não na trait impl).

Três alternativas:

- **C6a** — `TagIntrospector::position_of` retorna `None`
  sempre (consumer obtém Position directamente do
  Layouter via `layouter.runtime.positions.get(&loc)`).
- **C6b** — Trait separa-se: `Introspector` (queries
  estruturais) vs `LayoutIntrospector` (queries
  runtime). Position vive no segundo.
- **C6c** — `TagIntrospector` ganha campo opcional
  `positions: Option<&HashMap<Location, Position>>`
  populado pelo Layouter antes de tracking. Inverte a
  direcção.

C6 fixa-se com base em A8 da auditoria P204A (vanilla
pipeline) e C1.5/C1.6. **Uma** alternativa fixada, sem
ramos.

Se C2 = "manter stub provisoriamente": C6 trivial —
`position_of` continua a retornar `Option<()>`; runtime
populado mas trait API não migra.

### C7 — Tests E2E

Adicionar 2–3 tests:

- **Test 1** — Position básico: documento com 1 label
  numa página específica; confirmar que
  `runtime.positions` contém entry com `page` correcto.
- **Test 2** — Position multi-página: documento com 3+
  páginas e label em cada; confirmar mapping.
- **Test 3 (opcional)** — Position via trait API (se C2
  = migrar stub): consultar via
  `introspector.position_of(loc)` e confirmar `Some(Position)`.

Critério: tests verdes; 1829 → 1831 ou 1832.

### C8 — Compilação

```
cargo build --workspace 2>&1 | tail -20
```

Critério: verde. Hipóteses prováveis de erro:

- `Point` sem `Hash` — resolver com derive ou manual.
- `NonZeroUsize` sem import — adicionar
  `std::num::NonZeroUsize`.
- Lifetime mismatch se C6 alternativa exigir borrow.

### C9 — Tests workspace

```
cargo test --workspace 2>&1 | tail -30
```

Critério: 1829+ tests verdes (mais 2–3 de C7).

### C10 — Linter

```
crystalline-lint .
```

Critério: 0 violations.

### C11 — Sentinelas dedicadas

Adicionar 1–2 sentinelas:

- `p204d_position_struct_existe` — falha de compilação
  se tipo `Position` for removido.
- `p204d_runtime_positions_field_existe` — falha de
  compilação se field for removido.

Decisão dentro de P204D. Recomendado pelo menos 1.

### C12 — Documentação ADR

ADR-0073 mantém-se em PROPOSTO. ADR-0066 não é
modificado em P204D — a transição "superseded" é
trabalho de P204H.

Adicionar nota lateral em ADR-0073 (não no estado, na
secção "Plano de materialização") confirmando que P204D
foi concluído. Edição cirúrgica: 1 linha.

### C13 — Critério de fecho de P204D

P204D concluído quando:

- C1 inventário completo.
- C2 API-decisão fixada com justificação.
- C3 tipo `Position` criado.
- C4 sub-store `runtime.positions` adicionado.
- C5 população single-pass aplicada.
- C6 trait API resolvida (uma das alternativas).
- C7 tests E2E (mínimo 2).
- C8 compilação verde.
- C9 tests workspace verdes.
- C10 linter 0 violations.
- C11 sentinelas (mínimo 1).
- Inventário registado.
- Relatório escrito.

### C14 — Sem cláusulas condicionais

C1 produz dados. C2 fixa **uma** alternativa de API.
C3–C5 executam decisões fixas. C6 fixa **uma**
alternativa de implementação.

Casos que dependem de auditoria são marcados como
"output de C1", não como `if` na spec.

---

## §4 Outputs concretos

### Ficheiro 1 — Inventário interno

Localização:
`00_nucleo/diagnosticos/typst-passo-204D-inventario.md`.

Conteúdo:
- §1 C1 — inventário empírico (6 sub-secções).
- §2 C2 — API-decisão fixada com justificação.
- §3 C6 — alternativa fixada com justificação.
- §4 C5 — ponto de emissão de Position no Layouter.
- §5 Decisões durante a leitura.

### Ficheiro 2 — Relatório

Localização:
`00_nucleo/materialization/typst-passo-204D-relatorio.md`.

Conteúdo:
- O que foi feito.
- Tempo de execução.
- Métricas (tests pre/post; LOC delta).
- Decisões durante a leitura.
- Sugestão para próximo sub-passo (P204E).

### Ficheiro 3 — Alterações em código

Não é ficheiro discreto. Conjunto de alterações em:

- `01_core/src/entities/position.rs` (novo).
- `01_core/src/entities/mod.rs` ou `lib.rs` (export).
- `LayouterRuntimeState` (campo).
- `01_core/src/rules/layout/...` (chamadas a
  `runtime.positions.insert`).
- Trait `Introspector::position_of` (assinatura, se C2 =
  migrar).
- Tests E2E.
- Sentinelas.

---

## §5 Critério de progressão para P204E

P204D fechado quando C13 cumprido.

Em caso de divergência empírica relevante (ex: `Point`
não existe em L1, `LayouterRuntimeState` divergir do
esperado, trait API não-migrável trivialmente), registar
em `P204D.div-N` e:

- Resolver dentro de P204D (preferido).
- Recuar para P204A re-fixar C8 com novos dados (se
  obstrução for estrutural).

P204E só começa quando P204D fechado.

---

## §6 Convenções mantidas

- Sem condicionais estruturais nas cláusulas de
  execução.
- 3 outputs (inventário + relatório + código).
- Inventário empírico antes de implementação.
- Localização canónica:
  `00_nucleo/diagnosticos/` para inventário;
  `00_nucleo/materialization/` para relatório.
- Distinção fecho estrutural vs final mantida.
- Sem inflação retórica.

---

## §7 Não-objectivos

P204D não:

- Adiciona `evict()` wrapper (P204E).
- Adiciona ficheiros ao corpus de paridade (P204F).
- Adiciona benchmarks (P204G).
- Transita ADR-0073 para ACEITE (P204H).
- Transita ADR-0066 para superseded (P204H).
- Cria ADR nova.
- Toca em consumers fora dos 3 ou 4 sites necessários
  para popular Position.
- Modifica loops fixpoint (preservados).
- Materializa sub-stores trackable separadamente.
- Materializa Position no walk-time (rejeitado em P203A
  C4).
- Materializa Position via fixpoint (rejeitado em P203A
  C4).

---

## §8 Erro a não repetir

P203A revelou que premissa "Position resolve lacunas
#1/#1b" estava errada. P204A revelou que padrão B3 era
indirecção desnecessária (Padrão A literal disponível).
P204B revelou que tipos retornados precisavam de Hash
(não verificado em A10). P204C teve sem divergências.

P204D corre risco específico: a tensão estrutural em C6
(`TagIntrospector` não tem acesso a `runtime.positions`).
**Não pré-fixei a alternativa**. C6 lista 3 hipóteses; a
escolha emerge de C1.5 + C1.6 + análise da auditoria
P204A A8 (vanilla pipeline).

Se C6 escolher C6b (separação de traits), trabalho
inflaciona — possivelmente recua para P204A para
re-fixar C2/C3. Caso contrário, P204D fica em S–M.

Hipótese mais provável: C6a (TagIntrospector retorna
None; consumer obtém Position via Layouter directamente).
Cristalino single-pass + Position vive no Layouter, não
no Introspector → consumers que precisam de Position
usam `layouter.runtime.positions.get(&loc)` em vez de
`introspector.position_of(loc)`. Trait API permanece
estrutural; runtime API expõe Position.

Mas isto é hipótese, não decisão da spec. C6 fixa-se em
P204D.

---

## §9 Particularidade — execução

P204D é trabalho de código focado:

- 1 ficheiro novo (`position.rs`).
- Modificação de `LayouterRuntimeState` (1 ficheiro).
- 1–4 sites de população em Layouter (a identificar em
  C1.5).
- Trait modification (se C2 = migrar).
- 2–3 tests E2E.
- 1–2 sentinelas.

Volume baixo a médio. Magnitude S–M.

Recomendado Claude Code dado:
- Decisão estrutural em C6 que beneficia de iteração
  rápida com cargo build.
- Inventário em C1.5 (ponto de emissão) requer
  exploração do Layouter.

Sessão actual viável se C6 não revelar obstrução
estrutural. Caso revele, Claude Code é mais apropriado
para iterar.
