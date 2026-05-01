# Passo P165 — `Introspector` trait + sub-stores + `from_tags` (M3)

Passo único de M3 do refactor Introspection. Materializa o
primeiro consumer real do `Vec<Tag>` emitido em P162: trait
`Introspector` com implementação concreta que constrói
snapshot consultável a partir de tags.

**Paraleliza com `CounterStateLegacy`**: M3 não substitui o
tipo legacy. Walk continua a popular ambos. M4-M5 fazem
migração de consumers; M6 elimina `CounterStateLegacy`.

**Pré-condição**: M2 concluído (P164). `is_locatable`
disponível como utilitária; `extract_payload` produz payload
type-safe; tags emitidas em paralelo no walk.

**Restrições**:
- Não modificar walk em `rules/introspect.rs` quanto à
  emissão de tags (já estabelecida em P162).
- Walk pode ser estendido para popular `Introspector` em
  paralelo, mas API pública `introspect() ->
  CounterStateLegacy` preservada.
- Não eliminar `CounterStateLegacy` (M6).
- Não migrar consumers (M4-M5).
- Output observable não muda; snapshot tests passam inalterados.

---

## Sub-passos

### .A Inventário e decisões pendentes

Reverificar (não confiar em P164):

1. `extract_payload` em
   `01_core/src/rules/introspect/extract_payload.rs` retorna
   `Option<ElementPayload>` com 3 variants (Heading, Figure,
   Citation). Confirmar.
2. `is_locatable` em
   `01_core/src/rules/introspect/locatable.rs` cobre os 56
   variants de Content. Confirmar.
3. `Tag` enum:
   - `Tag::Start(Location, ElementInfo)` — info contém
     `payload: ElementPayload` e `label: Option<Label>`.
   - `Tag::End(Location, u128)` — `u128` é content hash.
4. Walk emite `Vec<Tag>` em paralelo a `CounterStateLegacy`.
   Tags actualmente descartadas em `introspect()`.
5. **Convenção de nomes para implementação concreta de
   trait**:
   - Procurar no cristalino exemplos de pattern "trait +
     impl struct". Exemplos típicos: `World` trait com
     `WorldImpl`? `Engine` trait? Outras convenções?
   - Se há padrão consistente (ex. sufixo `Impl`, prefixo
     `Default`, ou nome descritivo do target), adoptar.
   - Se não há padrão, propor nome próprio com justificação.
   - Cláusula gate trivial aplicável: decidir localmente,
     documentar.
6. **`comemo::Track` e cristalino**:
   - `grep -rn "comemo::track\|#\[track\]" 01_core/src/`.
   - Confirmar que cristalino usa `comemo` (P162 contexto
     mencionou).
   - Identificar exemplos de trait com `#[comemo::track]`
     já em uso. Adoptar mesmo padrão para `Introspector`.
   - Se `comemo` não está em uso ou tem forma diferente:
     gate substancial, parar e reabrir.
7. **Sub-stores**: confirmar que `LabelRegistry`,
   `CounterRegistry` ainda não existem como tipos no
   cristalino. Procurar:
   - `grep -rn "struct LabelRegistry\|struct CounterRegistry" 01_core/src/`.
   - Se algum existe (improvável, mas possível): adaptar
     em vez de duplicar.
8. **`MetadataStore`**: confirmar que NÃO se cria em P165.
   Razão: `extract_payload` em M1 não tem variant `Metadata`.
   `MetadataStore` será adicionado em M9 quando feature
   `metadata()` for adicionada.

Output: notas internas + decisões registadas para usar
em sub-passos seguintes:
- Nome da implementação concreta de `Introspector`.
- Padrão `comemo::Track` (forma exacta).
- Confirmação de ausência de sub-stores existentes.

**Critério de saída e gate de decisão**:
- Se `comemo` não está em uso no cristalino: **parar**
  reabrir. Decisão `Introspector` como trait depende de
  `comemo` para fazer sentido. Pode ter que regredir para
  struct concreta (alternativa em §8.2 do desenho).
- Se nome da implementação concreta não tem padrão claro:
  cláusula gate trivial — propor nome, documentar.
- Se sub-store já existe: adaptar local sem parar.
- Senão, prosseguir para .B.

### .B Criar L0+L1 de `LabelRegistry`

Sub-store de `Label → Location`. Construído a partir de
`Tag::Start` quando `info.label.is_some()`.

1. L0 em `00_nucleo/prompts/entities/label_registry.md`:
   - Cabeçalho com campo "Hash do Código" em branco.
   - Camada L1, ficheiro alvo
     `01_core/src/entities/label_registry.rs`.
   - ADRs: ADR-0033, ADR-0066.
   - Origem vanilla: nenhuma directa. Vanilla agrega no
     `ElementIntrospector` field `labels: MultiMap<Label, usize>`.
     Cristalino isola em tipo próprio (decisão registada em
     desenho como divergência consciente — "melhor que
     vanilla").
   - Restrições estruturais:
     - `pub struct LabelRegistry { /* ... */ }`.
     - Construção via builder pattern ou método `from_tags`.
     - Read-only após construção.
     - `Clone`.
   - API:
     - `pub fn lookup(&self, label: &Label) -> Option<Location>`.
     - Decisão sobre múltiplos labels iguais: vanilla usa
       MultiMap; cristalino pode usar MultiMap ou rejeitar
       duplicados. Decidir em .B conforme tests existentes.
   - Critérios de verificação:
     - `LabelRegistry::empty().lookup(&label)` retorna `None`.
     - Após adicionar `(label, location)`, `lookup(&label)`
       retorna `Some(location)`.
     - Com 5 labels distintos, todos resolvem correctamente.

2. L1 em `01_core/src/entities/label_registry.rs`:
   - Cabeçalho `@prompt`.
   - Implementação concreta com `HashMap<Label, Location>` ou
     `MultiMap` se duplicados forem permitidos.
   - Construtor `pub fn empty() -> Self`.
   - Builder: `pub(crate) fn add(&mut self, label: Label, location: Location)`.
   - `pub fn lookup(&self, label: &Label) -> Option<Location>`.
   - Tests co-localizados.

3. Update `01_core/src/entities/mod.rs`: re-export
   `LabelRegistry`.

**Critério de saída**:
- `cargo check` passa.
- `cargo test` — tests novos passam.
- L0 e L1 existem com cabeçalhos correctos.
- Linter passa.

### .C Criar L0+L1 de `CounterRegistry`

Sub-store de counters indexados por kind. Aplica
`CounterUpdate`s em ordem de Location.

1. L0 em `00_nucleo/prompts/entities/counter_registry.md`:
   - Cabeçalho com campo "Hash do Código" em branco.
   - Camada L1, ficheiro alvo
     `01_core/src/entities/counter_registry.rs`.
   - ADRs: ADR-0033, ADR-0066.
   - Origem vanilla: nenhuma directa. Vanilla agrega no
     `ElementIntrospector` indirectamente via queries por
     selector. Cristalino isola.
   - Restrições estruturais:
     - `pub struct CounterRegistry { /* ... */ }`.
     - Read-only após construção.
     - `Clone`.
   - API:
     - `pub fn empty() -> Self`.
     - `pub(crate) fn apply(&mut self, key: CounterKey, update: CounterUpdate)`.
     - `pub fn value(&self, key: &CounterKey) -> Option<&[usize]>`
       — valor hierárquico actual do counter.
     - `pub fn value_at(&self, key: &CounterKey, location: Location) -> Option<&[usize]>`
       — valor no ponto da location especificada (forward
       reference; precisa de iteração registada).
   - **Decisão**: `CounterKey` ainda não existe como tipo. Em
     M3 minimal, usar `String` (nome do counter: "heading",
     "figure", "cite"). `CounterKey` enum vanilla
     (`Page | Selector | Str`) fica para M9 quando counters
     custom forem adicionados.
   - Critérios de verificação:
     - `CounterRegistry::empty().value("heading")` retorna
       `None`.
     - Após `apply("heading", Step)`, `value("heading")`
       retorna `Some([1])`.
     - Hierarquia: 3 `Step` consecutivos em níveis 1, 2, 1
       produzem sequência correcta.

2. L1 em `01_core/src/entities/counter_registry.rs`:
   - Cabeçalho `@prompt`.
   - Implementação. `HashMap<String, Vec<usize>>` ou
     equivalente determinístico.
   - Tests co-localizados.

3. Update `entities/mod.rs`: re-export.

**Critério de saída**: igual a .B.

### .D Criar L0+L1 de `Introspector` (trait + struct)

1. L0 em `00_nucleo/prompts/entities/introspector.md`:
   - Cabeçalho com campo "Hash do Código" em branco.
   - Camada L1, ficheiro alvo
     `01_core/src/entities/introspector.rs`.
   - ADRs: ADR-0033, ADR-0066.
   - Origem vanilla:
     `lab/typst-original/.../introspection/introspector.rs`.
   - Restrições estruturais:
     - **Trait `Introspector`** com `#[comemo::track]` (ou
       padrão equivalente confirmado em .A número 6).
     - Métodos:
       - `query_by_kind(&self, kind: ElementKind) -> Vec<Location>`.
       - `query_by_label(&self, label: &Label) -> Option<Location>`.
       - `query_first(&self, kind: ElementKind) -> Option<Location>`.
       - `query_unique(&self, kind: ElementKind) -> Option<Location>`
         — retorna location apenas se houver exactamente um
         elemento desse kind.
       - `position_of(&self, location: Location) -> Option<Position>`.
     - **Struct concreta** (nome decidido em .A):
       - Implementa `Introspector` trait.
       - Composta por sub-stores: `LabelRegistry`,
         `CounterRegistry`, índice por kind, mapa
         Location→Position.
       - Read-only após construção.
       - Construção em `.E` via `from_tags`.
   - Critérios de verificação:
     - Trait expõe 5 métodos nomeados acima.
     - Struct concreta implementa trait.
     - Composição visível (sub-stores como fields públicos
       ou via getters).

2. L1 em `01_core/src/entities/introspector.rs`:
   - Cabeçalho `@prompt`.
   - Definição do trait + struct concreta com nome decidido
     em .A.
   - Implementação delega para sub-stores.
   - Tests co-localizados:
     - Construir struct concreta vazia, métodos retornam
       `None`/`Vec::new()`.
     - Construir struct concreta com 1 LabelRegistry + 1
       CounterRegistry + 1 índice por kind populados,
       métodos retornam valores correctos.

3. Update `entities/mod.rs`: re-export `Introspector` (trait)
   + struct concreta.

**Critério de saída**: igual a .B.

### .E Criar L0+L1 de `from_tags`

Construtor da struct concreta a partir de `&[Tag]`. Match
exaustivo sobre `ElementPayload`.

1. L0 em `00_nucleo/prompts/rules/introspect/from_tags.md`:
   - Cabeçalho com campo "Hash do Código" em branco.
   - Camada L1, ficheiro alvo
     `01_core/src/rules/introspect/from_tags.rs`.
   - ADRs: ADR-0033, ADR-0066.
   - Origem vanilla: similar a `ElementIntrospectorBuilder`
     em `lab/typst-original/.../introspection/introspector.rs`.
   - Restrições estruturais:
     - Função pura, sem efeitos secundários.
     - `pub fn from_tags(tags: &[Tag]) -> <StructConcreta>`.
     - Match **exaustivo** sobre `ElementPayload` (compilador
       força revisão quando variant novo for adicionado).
     - Bracketing válido: assume tags já bracketed (`Tag::Start`
       sempre seguido de `Tag::End` correspondente);
       comportamento se mal-formado é debug-assert (panic em
       debug, indefinido em release).
   - Critérios de verificação:
     - `from_tags(&[])` produz struct vazia.
     - `from_tags(&[Start(loc, info), End(loc, hash)])` produz
       struct com índice por kind populado.
     - Heading com label produz LabelRegistry com entry
       correcto.
     - Heading com counter_update produz CounterRegistry
       com entry correcto.

2. L1 em `01_core/src/rules/introspect/from_tags.rs`:
   - Cabeçalho `@prompt`.
   - Implementação:

   ```rust
   pub fn from_tags(tags: &[Tag]) -> <StructConcreta> {
       let mut labels = LabelRegistry::empty();
       let mut counters = CounterRegistry::empty();
       let mut kind_index: HashMap<ElementKind, Vec<Location>> = HashMap::new();
       let mut positions: HashMap<Location, Position> = HashMap::new();

       for tag in tags {
           match tag {
               Tag::Start(loc, info) => {
                   if let Some(label) = &info.label {
                       labels.add(label.clone(), *loc);
                   }
                   match &info.payload {
                       ElementPayload::Heading { counter_update, .. } => {
                           kind_index.entry(ElementKind::Heading).or_default().push(*loc);
                           counters.apply("heading".to_string(), counter_update.clone());
                       }
                       ElementPayload::Figure { counter_update, .. } => {
                           kind_index.entry(ElementKind::Figure).or_default().push(*loc);
                           counters.apply("figure".to_string(), counter_update.clone());
                       }
                       ElementPayload::Citation { .. } => {
                           kind_index.entry(ElementKind::Citation).or_default().push(*loc);
                       }
                   }
               }
               Tag::End(_, _) => {
                   // hash não usado em from_tags; é input para
                   // detecção de mudança em fixpoint (M7+)
               }
           }
       }

       <StructConcreta> { labels, counters, kind_index, positions }
   }
   ```

   - **Position** ainda não tem mecanismo concreto de
     população em M3. Mapa fica vazio. Adiar para passo
     futuro (M5 ou M9 quando layout integrar).
   - Tests co-localizados.

3. Update `rules/introspect.rs`: declarar `pub mod from_tags;`
   em paralelo a `pub mod extract_payload;` e
   `pub mod locatable;`.

**Critério de saída**: igual a .B.

### .F Walk popula `Introspector` em paralelo

Walk continua a popular `CounterStateLegacy` exactamente como
antes. Adicionalmente, popula `Introspector` via `from_tags`.

1. Em `01_core/src/rules/introspect.rs`, modificar
   `pub fn introspect()`:

   ```rust
   pub fn introspect(content: &Content) -> CounterStateLegacy {
       let mut state = CounterStateLegacy::new();
       let mut locator = Locator::new();
       let mut tags: Vec<Tag> = Vec::new();
       let label_from_parent: Option<&Label> = None;
       walk(content, &mut state, &mut locator, &mut tags, label_from_parent);

       // M3: construir Introspector em paralelo
       let _introspector = from_tags(&tags);
       // _introspector é descartado em M3; M4-M5 começarão a
       // expô-lo como output adicional ou via novo entry point.
       // CounterStateLegacy continua a ser único output público.

       drop(tags);
       state
   }
   ```

   `_introspector` underscore-prefixed para indicar
   construído mas não usado (mata warnings).

2. Update L0 `00_nucleo/prompts/rules/introspect.md`:
   - Documentar que `introspect()` constrói `Introspector`
     em paralelo via `from_tags`. Resultado é descartado em
     M3 (M4-M5 começarão a expô-lo).

**Critério de saída**:
- `cargo check` passa.
- `cargo test` — todos os tests existentes passam.
- API pública preservada (assinatura `introspect()`
  inalterada — continua a retornar `CounterStateLegacy`).
- Linter passa.

### .G Tests E2E paralelo a `CounterStateLegacy`

Tests novos verificam que `Introspector` construído em
paralelo carrega informação consistente com
`CounterStateLegacy`.

1. Helper de teste em `#[cfg(test)]`:

   ```rust
   #[cfg(test)]
   pub(crate) fn introspect_with_introspector(
       content: &Content,
   ) -> (CounterStateLegacy, <StructConcreta>) {
       let mut state = CounterStateLegacy::new();
       let mut locator = Locator::new();
       let mut tags: Vec<Tag> = Vec::new();
       walk(content, &mut state, &mut locator, &mut tags, None);
       let introspector = from_tags(&tags);
       (state, introspector)
   }
   ```

2. Tests E2E (em `rules/introspect.rs` ou módulo dedicado):

   - **Test consistência heading**: walk sobre Content com
     headings em níveis [1, 2, 2, 3] → Introspector tem
     4 headings indexados; CounterStateLegacy tem mesmo
     número via `format_hierarchical`.
   - **Test consistência figure**: walk sobre Content com
     N figures → Introspector indexa N; CounterStateLegacy
     em `figure_numbers` tem mesmo número (modulo
     divergência kind documentada em
     `m1-lacunas-captura.md`).
   - **Test consistência citation**: walk com 3 citations
     → Introspector indexa 3 com keys correctas.
   - **Test query_by_label**: walk com Heading labelled →
     `introspector.query_by_label(&label)` retorna
     `Some(location)`; mesma location aparece no índice
     por kind.
   - **Test query_first vs query_unique**: walk com 1
     Figure → `query_first(Figure)` retorna `Some(loc)`;
     `query_unique(Figure)` retorna `Some(loc)`. Walk com
     2 Figures → `query_first` retorna `Some(loc1)`;
     `query_unique` retorna `None`.

**Critério de saída**:
- 5 tests novos passam.
- `cargo test` — todos os tests passam.
- Linter passa.

### .H Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace` — todos os tests passam.
   Δ vs P164 baseline (1564). Tests novos: ~15-20
   (LabelRegistry + CounterRegistry + Introspector +
   from_tags + E2E).
3. `crystalline-lint`: zero violations.
4. 4 ficheiros L1 novos:
   - `entities/label_registry.rs`.
   - `entities/counter_registry.rs`.
   - `entities/introspector.rs`.
   - `rules/introspect/from_tags.rs`.
5. 4 L0 novos correspondentes.
6. L0 `introspect.md` actualizado para documentar
   construção em paralelo do `Introspector`.
7. Walk emite tags como antes (P162 lógica preservada).
8. `pub fn introspect()` retorna `CounterStateLegacy`
   (assinatura preservada — `Introspector` construído mas
   descartado).
9. Snapshot tests de paridade ADR-0033 passam inalterados.
10. Linter passa em verificação final.

### .I Encerramento

Escrever
`00_nucleo/materialization/typst-passo-165-relatorio.md` com:

- Resumo: trait `Introspector` + struct concreta
  (nome decidido em .A); 2 sub-stores; `from_tags`;
  walk popula em paralelo a `CounterStateLegacy`.
- Confirmação de cada verificação .H.
- Hashes finais de 4 L0 novos + 1 L0 modificado
  (preenchidos pelo linter).
- Decisões registadas em .A:
  - Nome da implementação concreta.
  - Padrão `comemo::Track` adoptado.
  - Confirmação ausência de sub-stores existentes.
  - `MetadataStore` adiado para M9 (registar como
    pendência).
- Δ tests vs baseline P164.
- Estado pós-passo: M3 concluído.
  `Introspector` construído em paralelo a
  `CounterStateLegacy`. M4 desbloqueado — começar migração
  de consumers para `Introspector` (primeiro consumer real,
  provavelmente layout ou `materialize_time`).

---

## Critério de conclusão

Todas em conjunto:

1. .A produziu inventário sem disparar gate substancial.
2. `LabelRegistry` criado em `entities/label_registry.rs`.
3. `CounterRegistry` criado em `entities/counter_registry.rs`.
4. `Introspector` (trait + struct concreta) criado em
   `entities/introspector.rs`.
5. `from_tags` criado em `rules/introspect/from_tags.rs`
   com match exaustivo sobre `ElementPayload`.
6. Walk popula `Introspector` em paralelo a
   `CounterStateLegacy`. Resultado descartado.
7. Tests E2E confirmam consistência entre `Introspector` e
   `CounterStateLegacy`.
8. Verificações .H 1-10 passam.
9. Relatório .I escrito.
10. Output observable não muda.

---

## O que pode sair errado

- **`comemo` ausente ou com forma diferente (.A número 6
  gate substancial)**: `Introspector` como trait com
  `#[comemo::track]` é a base do desenho fixpoint (M7+).
  Se cristalino não usa `comemo`, parar e reabrir decisão
  §8.2 (trait vs struct concreta). Pode regredir para
  struct concreta — implicação é que memoização entre
  iterações em M7 será mais difícil.
- **Nome da implementação concreta não tem padrão claro
  no cristalino**: cláusula gate trivial. Propor nome
  baseado em descrição (ex. `BasicIntrospector`,
  `TagIntrospector`, ou simplesmente sem trait separado).
  Documentar.
- **Match exaustivo em `from_tags` força cobertura de
  variants futuros**: ao adicionar variant novo a
  `ElementPayload` (M9+), `from_tags` deixa de compilar.
  É a propriedade desejada — força revisão. Mas pode
  surpreender. Documentar no L0.
- **Tests E2E detectam divergência entre Introspector e
  CounterStateLegacy**: provavelmente uma das 3 divergências
  já documentadas em `m1-lacunas-captura.md`. Se for nova
  divergência, registar e adiar para passo correctivo.
- **`Position` não tem mecanismo de população em M3**:
  mapa Location→Position fica vazio. `position_of` retorna
  sempre `None`. Documentar como pendência para M5/M9.
- **Walk popular Introspector em paralelo aumenta tempo de
  execução**: tests podem ficar mais lentos. Mensurável,
  registar Δ no relatório. Optimização (não construir
  introspector se não for usado) pode vir em M4-M5.
- **`HashMap<ElementKind, ...>` itera em ordem
  não-determinística**: se algum método de query expõe
  ordem (ex. `query_by_kind` retorna `Vec<Location>`),
  pode haver não-determinismo entre runs. Usar `BTreeMap`
  ou ordenar antes de retornar.
- **Linter detecta divergência L0↔L1**: mais ficheiros
  novos = mais oportunidade de divergência. Ajustar
  conforme erro reportado.

---

## Notas operacionais

- **Tamanho**: M-L. Comparável a P162. 4 L0+L1 novos +
  modificação de walk + tests é trabalho substancial.
- **Pré-condição M4-M5**: `Introspector` populado em
  paralelo é base para migração de consumers. M4 começa
  a expor `Introspector` como output adicional;
  M5 migra primeiro consumer (layout ou
  `materialize_time`).
- **`CounterStateLegacy` preservado**: M3 não substitui.
  Walk continua a popular ambos. Eliminação só em M6.
- **Cláusula gate trivial** (formalizada em P163):
  aplicável. Decisões locais sobre nomes, ausência de
  sub-stores, ou variantes ligeiras de padrão podem ser
  resolvidas sem parar, com documentação no relatório.
- **`MetadataStore` adiado**: P165 não cria. Razão:
  `extract_payload` em M1 não tem variant `Metadata`.
  Quando feature `metadata()` for adicionada (M9),
  `MetadataStore` é criado em paralelo.
- **Match exaustivo em `from_tags`**: decisão dura, igual
  a `is_locatable` (P164). Compilador força revisão
  quando `ElementPayload` for estendido.
- **Position vazio em M3**: documentar pendência. Mapa
  Location→Position só é populado quando layout integrar.
  Em M3, `position_of` retorna sempre `None`. Aceitável
  porque consumers reais virão em M4-M5.
