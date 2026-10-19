# Passo 205B — Sealing infrastructure + `SealedPositions`

**Série**: 205 (sub-passo `B` = implementação após
diagnóstico P205A).
**Tipo**: implementação focada (sub-store sealed).
**Magnitude planeada**: S–M.
**Pré-condição**: P205A concluído; ADR-0074 PROPOSTO em
`00_nucleo/adr/typst-adr-0074-f3-layouter-substores-trackable.md`;
auditoria empírica em
`typst-passo-205A-auditoria-f3.md`; diagnóstico em
`typst-passo-205A-diagnostico.md`; tests 1852 verdes;
0 violations; 17 sentinelas activas.
**Output**: 3 ficheiros (inventário + relatório +
alterações de código).

---

## §1 Propósito

Materializar a infraestrutura de sealing para o
sub-store `positions` per ADR-0074 (decisões fixadas em
P205A C1–C4). Trabalho concreto:

- Tipo `SealedPositions` em L1.
- `#[comemo::track]` aplicado per Padrão A literal
  (paridade arquitectónica com M8).
- `Layouter::finish` (ou consumer equivalente) produz
  sealed sub-store após cada iteração fixpoint.
- 2–3 sentinelas dedicadas.

P205B **não migra consumers** de `position_of` — isso é
trabalho de P205C. P205B materializa apenas a
infraestrutura.

P205B respeita o padrão: começa com inventário empírico
antes de qualquer alteração.

---

## §2 Material de partida verificado em P205A

Antes de qualquer alteração, confirmar empíricamente:

- ADR-0074 PROPOSTO existe.
- `runtime.positions: HashMap<Location, Position>` em
  `LayouterRuntimeState` (per P204D + auditoria P205A
  A2).
- Tipo `Position { page: NonZeroUsize, point: Point }`
  em `01_core/src/entities/position.rs` (per P204D).
- Hash impl manual via `to_bits()` em Position (per
  P204D).
- Loop fixpoint cristalino (TOC + run_fixpoint, MAX=5)
  funcional.
- `Layouter::finish` ou método análogo onde sealing
  pode ser aplicado.

Sem isto, recuar para P205A.

---

## §3 Cláusulas de execução (sem condicionais)

### C1 — Inventário empírico inicial

Antes de tocar em código, listar literalmente:

1. **`Layouter::finish`** — confirmar:
   - Existe? Caminho exacto.
   - Assinatura actual (parâmetros, retorno).
   - Quem invoca? Lista de call sites.
2. **`PagedDocument`** — confirmar:
   - Definição exacta (caminho, fields).
   - Consumers actuais (`pub` ou interno).
   - Pode ganhar campo novo sem quebrar API pública?
3. **`runtime.positions`** — confirmar:
   - Estado pós-P204D (populated single-pass via
     `advance_locator_if_locatable`).
   - Onde é acedido por consumers.
4. **`Arc<HashMap<...>>` ou similar wrapping** —
   confirmar:
   - Convenção cristalina existe (Arc, Rc, Box)?
   - Padrão usado em outros sub-stores cristalinos.
5. **Sealing point empírico** — identificar:
   - Após `l.finish()` no fixpoint loop?
   - No fim de `pub fn layout`?
   - Outro?
6. **`Send + Sync` em SealedPositions** —
   compatibilidade com requisitos de `#[comemo::track]`
   (per lição de P204B).
7. **Hash satisfeito por SealedPositions** —
   `Position` já tem Hash manual via `to_bits()` (per
   P204D); confirmar que `HashMap<Location, Position>`
   ou equivalente satisfaz quando wrapped.

Output: 7 sub-secções com etiqueta CONFIRMADO ou
**AJUSTE NECESSÁRIO**.

Se C1.1 ou C1.2 revelar obstrução estrutural, registar
`P205B.div-N` e re-fixar C2.

### C2 — Decisão sobre forma de sealing

Com base em C1.1 + C1.2 + C1.5, fixar:

- **Caminho A — Tuple** —
  `Layouter::finish() -> (PagedDocument, SealedPositions)`.
  Caller adapta para extrair separadamente.
- **Caminho B — Field anexado** —
  `PagedDocument` ganha `pub positions: Arc<SealedPositions>`.
  API pública muda (campo novo).
- **Caminho C — Sub-tipo de PagedDocument** —
  `PagedDocument` ganha sub-struct interno
  `IntrospectionData` que contém `positions` e futuros
  sealed sub-stores.

Critério para escolha:
- Se C1.2 mostrar que `PagedDocument` é amplamente
  consumido em API pública — Caminho B inflaciona;
  Caminho A é menos invasivo.
- Se `PagedDocument` for já mutável internamente sem
  quebrar consumers — Caminho B é natural.
- Caminho C antecipa P205D (label_pages trackable);
  trade-off é over-engineering vs simetria futura.

C2 fixa **uma** alternativa.

### C3 — Definição literal de `SealedPositions`

Com base em C2 + lições de P204B (Hash) e P204D (Hash
manual):

```text
pub struct SealedPositions(Arc<HashMap<Location, Position>>);

#[comemo::track]
impl SealedPositions {
    fn position_of(&self, location: Location) -> Option<Position>;
}
```

Notação ilustrativa. Decisões concretas a fixar dentro
de P205B:

- `Arc` ou `Box` ou ownership directo. Decisão depende
  de C1.4.
- `#[comemo::track] impl SealedPositions` directo, ou
  via trait dedicada `pub trait PositionStore`.

Per C3 do diagnóstico P205A: Padrão A literal favorito.
Se struct concreta basta (uma única impl), `#[comemo::track]
impl` directo é suficiente. Caso haja necessidade de
múltiplas impls (futuro `PagedPositions` /
`HtmlPositions`), trait dedicada justifica-se.

C3 fixa forma concreta.

### C4 — Localização

Decisão:

- `01_core/src/entities/sealed_positions.rs` (módulo
  dedicado).
- `01_core/src/entities/position.rs` (estende ficheiro
  existente).

Caminho preferido: módulo dedicado. Per padrão dos sub-stores
cristalinos (`bib_store.rs`, `state_registry.rs`).

C4 fixa caminho.

### C5 — Sealing literal em `Layouter::finish`

Edição literal (notação ilustrativa, depende de C2):

```text
- pub fn finish(self) -> PagedDocument { ... }
+ pub fn finish(self) -> (PagedDocument, SealedPositions) { ... }
```

Ou (Caminho B):

```text
- pub fn finish(self) -> PagedDocument {
+     PagedDocument { pages, positions: Arc::new(...) }
+ }
```

Sealing extrai `self.runtime.positions` para `Arc` (ou
forma fixada em C2/C3).

Loop fixpoint adapta-se: extrai sealed sub-store
opcionalmente, passa para iteração seguinte se
necessário.

### C6 — L0 prompt

Per CLAUDE.md Protocolo de Nucleação (lição de P204D e
P204G), módulo novo em L1 exige L0 prompt.

Se C4 = módulo dedicado: criar
`00_nucleo/prompts/entities/sealed-positions.md`.

Se C4 = estende `position.rs`: actualizar L0 prompt
existente `entities/position.md` com secção sobre
sealing.

`--fix-hashes` aplicado no fim para sincronizar.

### C7 — Sentinelas

2–3 sentinelas:

- `p205b_sealed_positions_struct_existe` — falha de
  compilação se tipo for removido.
- `p205b_sealed_positions_e_track` — verifica
  `comemo::Track` impl gerado pelo macro.
- `p205b_layouter_finish_produz_sealed` (opcional) —
  invoca `Layouter::finish` e confirma que sealed
  sub-store é populated.

Decisão dentro de P205B (mínimo 2; recomendado 3 se
construir Layouter em fixture for trivial — per P204C
boilerplate é mecânico).

### C8 — Compilação

```
cargo build --workspace 2>&1 | tail -10
```

Critério: verde. Hipóteses prováveis de erro:

- `SealedPositions` precisa `Send + Sync` derivados —
  `Arc<HashMap<...>>` é trivial se K e V são.
- `Position` Hash manual via `to_bits()` (per P204D)
  satisfeito; verificação no compile.
- Lifetime mismatch se Caminho A criar referências
  cruzadas.

### C9 — Tests workspace

```
cargo test --workspace 2>&1 | tail -10
```

Critério: 1852+ tests verdes (com 2–3 novas sentinelas
de C7).

### C10 — Linter

```
crystalline-lint .
```

Critério: 0 violations. Se L0 prompt criado em C6,
correr `crystalline-lint --fix-hashes .` para
sincronização automática (lição de P204D/E/G).

### C11 — Documentação ADR-0074

ADR-0074 mantém PROPOSTO. Anotação cirúrgica em §P205B
do plano de materialização: `✅ MATERIALIZADO 2026-05-07`
+ sumário (1–2 linhas).

P205C transita ADR para ACEITE estrutural ou aguarda
P205E.

### C12 — Critério de fecho de P205B

P205B concluído quando:

- C1 inventário completo (7 sub-secções).
- C2 forma de sealing fixada com justificação.
- C3 definição literal de `SealedPositions`.
- C4 localização fixada.
- C5 sealing aplicado em `Layouter::finish`.
- C6 L0 prompt criado/actualizado.
- C7 sentinelas (mínimo 2).
- C8 compilação verde.
- C9 tests workspace verdes.
- C10 linter 0 violations.
- C11 ADR-0074 anotada.
- Inventário registado.
- Relatório escrito.

### C13 — Sem cláusulas condicionais

C1 produz dados. C2 fixa **uma** alternativa de sealing.
C3 fixa forma concreta. C4 fixa localização. C5 executa
decisões fixas.

Decisões internas (Arc vs Box em C3; mínimo de
sentinelas em C7) resolvem dentro do passo sem ramos
estruturais.

---

## §4 Outputs concretos

### Ficheiro 1 — Inventário interno

Localização:
`00_nucleo/diagnosticos/typst-passo-205B-inventario.md`.

Conteúdo:
- §1 C1 — inventário (7 sub-secções).
- §2 C2 — caminho de sealing fixado.
- §3 C3+C4 — forma e localização concretas.
- §4 C5 — alterações literais em finish.
- §5 Decisões durante a leitura.

### Ficheiro 2 — Relatório

Localização:
`00_nucleo/materialization/typst-passo-205B-relatorio.md`.

Conteúdo:
- O que foi feito.
- Tempo de execução.
- Métricas (tests pre/post; LOC delta).
- Decisões.
- Sugestão para próximo sub-passo (P205C).

### Ficheiro 3 — Alterações em código

Não é ficheiro discreto. Conjunto de:

- 1 ficheiro novo (`sealed_positions.rs` ou similar).
- Possível L0 prompt novo.
- `01_core/src/entities/mod.rs` (export).
- `Layouter::finish` modificado (adaptação consumers se
  C2 = A; field novo se C2 = B).
- Loop fixpoint adaptado (extrai sealed sub-store).
- Sentinelas.
- Anotação cirúrgica em ADR-0074.

---

## §5 Critério de progressão para P205C

P205B fechado quando C12 cumprido.

Em caso de divergência empírica relevante (ex:
`Layouter::finish` não existe como esperado, `PagedDocument`
não pode ganhar campo trivialmente, `Arc<HashMap>` não
satisfaz `Hash`/`Send`/`Sync`), registar em
`P205B.div-N` e:

- Resolver dentro de P205B (preferido).
- Recuar para P205A re-fixar C2/C3 se obstrução for
  estrutural.

P205C só começa quando P205B fechado.

---

## §6 Convenções mantidas

- Sem condicionais estruturais.
- 3 outputs.
- Inventário empírico antes de implementação.
- Localização canónica:
  `00_nucleo/diagnosticos/` para inventário;
  `00_nucleo/materialization/` para relatório.
- Distinção fecho estrutural vs final mantida.
- Sem inflação retórica.

---

## §7 Não-objectivos

P205B não:

- Migra consumers de `position_of` (P205C).
- Materializa `position_of` impl real em
  `TagIntrospector` (P205C).
- Implementa `label_pages` trackable (P205D).
- Transita ADR-0074 para ACEITE (P205E).
- Cria ADR nova além de ADR-0074 já PROPOSTO.
- Toca em loop fixpoint além de adaptar `finish`
  retorno.
- Modifica `runtime.positions` populated em P204D —
  apenas extrai-o no fim.
- Endereça `label_pages` ou `known_page_numbers` (esses
  são P205D).

---

## §8 Erro a não repetir

Da série P204 — 6 sub-passos sem inflação. P205B segue
mesmo padrão.

Risco específico: a tentação de **adoptar trait dedicada
`PositionStore` por simetria com Padrão A no
`Introspector`**. Per C3, struct concreta basta se há
apenas uma impl. Não criar trait sem consumer com
necessidade real (over-engineering).

Outro risco: **sealing global pós-fixpoint** em vez de
**sealing por iteração** (per C4 do diagnóstico
P205A). Diagnóstico fixou per-iteração; P205B não
inventa global por simplicidade.

Hipótese específica a testar em C1.1: `Layouter::finish`
pode não existir como método nomeado — pode ser
inline em `pub fn layout`. C1 confirma antes de C5
modificar.

Hipótese mais provável após auditoria empírica: forma de
sealing depende crucialmente de quantos consumers
externos `PagedDocument` tem. Caminho A (tuple) mais
seguro se há muitos; Caminho B (field) mais natural se
há poucos.

---

## §9 Particularidade — execução

P205B é trabalho de código focado:

- 1 ficheiro novo (~30–80 LOC).
- Possível L0 prompt (~30–50 LOC).
- Modificação de `Layouter::finish` ou equivalente (5–15
  LOC).
- Adaptação do loop fixpoint (5–10 LOC).
- 2–3 sentinelas (~30–50 LOC).
- Verificação compilação + tests + linter.

Volume médio. Magnitude S–M.

Recomendado Claude Code dado:

- Investigação em C1 sobre consumers de `PagedDocument`
  (decisão C2 depende disso).
- Iteração rápida com cargo build em caso de Hash/Send/Sync
  surpresa em `SealedPositions`.

Sessão actual viável se C1 não revelar obstrução.
