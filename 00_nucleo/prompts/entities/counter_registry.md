# Prompt L0 — `entities/counter_registry`
Hash do Código: 13cfe667

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/realization-order.toml sha256:8de2c32fbeeee7a30088b4ff3808e15a211f23a182673ef5cdebe54bd3eadad6

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/counter_registry.rs`
**Criado em**: 2026-04-30 (P165 sub-passo .C — sub-store de M3)
**ADRs relevantes**: ADR-0033 (paridade vanilla), ADR-0066 (Introspection runtime)

---

## Contexto

`CounterRegistry` é o sub-store de counters indexados por kind (string). Aplica `CounterUpdate`s em ordem de chegada via `from_tags` (P165 .E).

Vanilla agrega no `ElementIntrospector` indirectamente — counters são queriáveis via `Selector` mas não há um "registry" dedicado. Cristalino isola por simetria com `LabelRegistry` e composição visível.

`CounterKey` enum vanilla (`Page | Selector(Selector) | Str(Str)`) **não** é replicado em M3. Em vez disso, usar `String` directo como chave (nomes: "heading", "figure", etc.). `CounterKey` rico fica para M9 quando counters custom forem adicionados via `counter("name")` em Typst markup.

---

## Restrições Estruturais

- Camada **L1**: struct puro, sem I/O.
- Read-only após construção (mutação só via `pub(crate) fn apply` durante fase de construção).
- `Clone` derivado.
- Forma interna: `HashMap<String, Vec<usize>>` — vector de números por kind, semelhante a `CounterStateLegacy.figure_numbers`.

---

## Interface pública

```rust
use crate::entities::counter_update::CounterUpdate;
use crate::entities::location::Location;

#[derive(Debug, Clone, Default)]
pub struct CounterRegistry {
    /* HashMap<String, Vec<usize>> — estado actual por key */
    /* HashMap<String, Vec<(Location, Vec<usize>)>> — history (P177) */
}

impl CounterRegistry {
    pub fn empty() -> Self;
    pub fn value(&self, key: &str) -> Option<&[usize]>;
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;

    /// **P170 (M9 sub-passo 2)** — formato hierárquico "1.2.3".
    pub fn format(&self, key: &str) -> Option<String>;

    /// **P177 (M9 sub-passo 7)** — valor do counter na `Location`
    /// indicada. Retorna o estado **após** todas as updates com
    /// `loc <= location`. `None` se não há update prévia.
    pub fn value_at(&self, key: &str, location: Location) -> Option<&[usize]>;

    /// **P184C** — n-ésimo snapshot (0-indexed) na história de `key`.
    /// Acesso por posição em vez de por `Location`. Suporta
    /// consumers que iteram counters por kind em ordem de
    /// aparecimento sem conhecer a `Location` (Layouter C3 figure
    /// auto-number per kind, `mod.rs:435–439`).
    /// `None` se key inexistente ou idx fora de range da história.
    pub fn value_at_index(&self, key: &str, idx: usize) -> Option<&[usize]>;

    pub(crate) fn apply(&mut self, key: String, update: CounterUpdate);

    /// **P170 (M9 sub-passo 2)** — step hierárquico ao nível indicado.
    /// Paridade com `CounterStateLegacy::step_hierarchical`. Resolve
    /// lacuna #5 (`m1-lacunas-captura.md`).
    pub(crate) fn apply_hierarchical(&mut self, key: String, level: usize);

    /// **P177 (M9 sub-passo 7)** — wrapper sobre `apply` que regista
    /// também snapshot na `history` para suportar `value_at`.
    /// Used por `from_tags` em arms `Figure`/`Citation`.
    pub(crate) fn apply_at(&mut self, key: String, update: CounterUpdate, location: Location);

    /// **P177 (M9 sub-passo 7)** — wrapper sobre `apply_hierarchical`
    /// que regista snapshot. Used por `from_tags` em arm `Heading`.
    pub(crate) fn apply_hierarchical_at(&mut self, key: String, level: usize, location: Location);
}
```

---

## Semântica

- `empty()`: cria registry vazio.
- `value(key)`: retorna o slice actual do counter. `None` se nunca foi tocado.
- `apply(key, update)` (pub(crate)): aplica update.
  - `CounterUpdate::Step`: incrementa o último elemento. Se vector vazio, inicializa com `[1]`. Semelhante a `CounterStateLegacy::step_flat`.
  - `CounterUpdate::Update(value)`: define para `[value]`. Reseta hierarquia.
- **P170**: `apply_hierarchical(key, level)` (pub(crate)): paridade com `CounterStateLegacy::step_hierarchical`. Comportamento:
  - `[]` + `1` → `[1]`.
  - `[1]` + `2` → `[1, 1]`.
  - `[1, 1]` + `1` → `[2]`.
  - Level clamped a mínimo 1.
- **P170**: `format(key)` retorna `Option<String>` com Vec<usize> joined com "." ("1.2.1"). Forma equivalente a `CounterStateLegacy::format_hierarchical`. Resolve lacuna #5 — CounterRegistry deixa de ser flat em M9.

Hierarquia em M9: counters podem ter múltiplos níveis para Headings (paridade com walk arm `Content::Heading` em `introspect.rs::279`). Counters flat (figure, equation) continuam a usar `apply(_, Step)` que mantém Vec de tamanho 1.

- **P177**: `value_at(key, location)` retorna `Option<&[usize]>` com o valor do counter **após** todas as updates de `key` com `loc <= location`. Implementação via `history` (Vec ordenado por Location, monotonicamente crescente via `Locator`). Retorna `None` se nenhuma update precede `location` ou key inexistente.
- **P177**: `apply_at(key, update, location)` e `apply_hierarchical_at(key, level, location)` são wrappers sobre `apply`/`apply_hierarchical` que adicionalmente fazem snapshot do estado actual em `history[key]` com a Location indicada. `from_tags` migra para usar `_at` versions; tests existentes em `apply`/`apply_hierarchical` mantêm-se sem location (não populam history).
- **P184C**: `value_at_index(key, idx)` retorna `Option<&[usize]>` com o snapshot na posição `idx` (0-indexed) da `history[key]`. Acesso por posição (não por Location) — suporta consumers que iteram counters por kind em ordem de aparecimento sem conhecer Locations específicas (P184D consumer C3 em `mod.rs:435–439`). `None` se key inexistente ou idx fora de range. Para counters flat (figure, equation), o snapshot na posição `idx` é `[idx + 1]` (cada `apply_at(Step)` regista o estado pós-update, que para flat é o número actual). Para counters hierárquicos (heading), o snapshot reflecte o estado completo da hierarquia naquela Location.

---

## Invariantes

- `apply` é a única mutação — caller externo (não-test) só lê.
- Counters começam vazios; primeiro `Step` inicializa em 1.
- Igualdade entre instâncias é por conteúdo do HashMap interno.
- Sem ordering determinístico em iteração — caller que precise de ordem deve usar a chave conhecida.

---

## Tests obrigatórios (sub-passo .C P165)

- `CounterRegistry::empty().value("heading")` retorna `None`.
- Após `apply("heading", Step)`, `value("heading")` retorna `Some([1])`.
- 3 `Step` consecutivos produzem `[3]`.
- `apply("heading", Update(5))` → `value("heading")` retorna `Some([5])`.
- Counters distintos isolados — apply em "heading" não afecta "figure".

---

## Consumers actuais

Nenhum no momento da criação.

## Consumers planeados

- `rules/introspect/from_tags.rs` (P165 .E) — populador.
- `entities/introspector.rs` `TagIntrospector` (P165 .D) — leitor via getter.

---

## Sobre paridade

Vanilla não tem `CounterRegistry` separado — counters são parte do estado fixpoint via `comemo` + `ElementIntrospector` queries. Cristalino isola para coerência com `LabelRegistry`. Forma simplificada (flat counter, 1 nível) é suficiente para M3 minimal viable.

Forma rica (hierarquia, função counters via `Func`) fica para M9+ paralelamente à introdução de `CounterKey` enum vanilla.

Ver `00_nucleo/diagnosticos/inventario-tipos-introspection-vanilla.md` (2026-04-30) §3 para classificação de `Counter`/`CounterKey`/`CounterUpdate` vanilla.

---

## Resultado Esperado

- `01_core/src/entities/counter_registry.rs` — struct + 5 métodos + tests.
- Re-export em `01_core/src/entities/mod.rs`.

---

## Proposta P1148 — updates completos (AGUARDA GATE ADR-0127)

Medição da fonte ratificada `a51e02804` mostra que o update transporta estado
hierárquico, nível de step ou callback. Se o gate do contrato público em
`entities/counter_update.md` for aprovado, `apply` passa a:

- substituir o estado completo por `Set(Vec<usize>)`;
- aplicar `Step(level)` com a mesma regra hierárquica já existente em
  `apply_hierarchical`;
- aplicar `Func` ao array corrente e converter o resultado novamente em
  estado, propagando erro de avaliação em vez de o ignorar.

O callback executa na construção do introspector, não no field access. A fase
já transporta `Engine + EvalContext` para callbacks de `StateUpdate`; P1148
reusa esse canal e não cria uma fase nova. Qualquer alteração de assinatura
pública adicional descoberta na materialização volta ao gate ADR-0127.

---

## Log de ações para chaves filtradas

### Medição anterior à decisão

HEAD `2f42d64253547734564513a1159ee6b584c1c4b4`:
`entities/counter_registry.rs:25-36` contém estados e snapshots, não ações;
`:69-84` distingue Step/Set/Func. A medição histórica demonstra por fonte
que inferir ações pelas diferenças de snapshots perde Set versus Step.
A medição histórica registra no vanilla
Step/Func/Step com chave filtrada resultando em 12, não apenas tamanho da query.
Estado, UTC e binários estão nos registros; não existe candidato implementado.

### Contrato

Adicionar armazenamento **privado** de eventos ordenados, somente dados L1:
Location, origem automática/manual e ação CounterUpdate verdadeira; evento
manual conserva CounterKey inteira. Evento automático remete à ocorrência
canônica por Location e preserva Step(level) efetivo/gate do seu owner.
Registrar apenas uma ação automática por ocorrência, não duplicar a escrita
auxiliar figure:kind. Não deduzir ação por número anterior/posterior.

Acesso/mutação ao log é interno pub(crate), sem novo campo público ou método
de trait. Clone/default/empty preservam isolamento por documento/iteração.
O log não importa compiler nem executa Func; não é segundo store de campos.
Estado/histórico borrowed e métodos públicos existentes conservam o contrato
de leitura materializada. O cálculo de uma chave arbitrária da linguagem
devolve vetor owned no owner de compiler, não uma referência a temporário.

Este log prepara dados para a resolução sob demanda de
`compiler/introspect/from_tags.md`, aprovada pelo dono. A aprovação de fase
não dispensa os demais gates do P1339 antes da implementação.

## P1485.3 — ordem de Page no bundle (ratificado; ADR-0127)

### Medição anterior à decisão

`value_at` filtra por Location e escolhe a última entrada por inserção
(`01_core/src/entities/counter_registry.rs:151-159`); `apply_at` acrescenta
ao fim (`:182-194`). Pós-pass Page Func pode, portanto, inserir evento
de Location anterior depois da fronteira seguinte. É risco inferido
da fonte, não prova de output Func; sonda Set e proveniência no
diagnóstico P1485. Valor Page é linguagem, ordem de armazenamento é
mecânica (ADR-0107).

### Limite do owner

Suportar por helper privado a materialização de snapshots Page em ordem
causal Location + fonte exigida por `compiler/introspect/from_tags.md`
P1485.3. Não mudar assinatura pública, significado de outras chaves nem
transformar reset em ocorrência localizada. Compiler avalia Func; entidade
guarda só estados, Locations e ações, sem Engine, I/O ou import reverso.
O contrato público de introspecção P1485.3 foi confirmado pelo dono
em 2026-09-24.


## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-04-30 | P165 sub-passo .C: sub-store de counters por kind para Introspector M3 | `counter_registry.rs`, `counter_registry.md` |
| 2026-04-29 | P177 sub-passo .B: history field + `value_at` + `apply_at` / `apply_hierarchical_at` | `counter_registry.rs`, `counter_registry.md` |
| 2026-05-03 | P184C sub-passo .C: helper `value_at_index(key, idx)` para acesso por posição na história (não por Location). Suporta `Introspector::figure_number_at_index` (P184C .D) consumido por C3 (P184D). | `counter_registry.rs`, `counter_registry.md` |




## Medição da ordem antes da decisão

No HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, CounterRegistry::value_at e StateRegistry::value_at comparam Location.as_u128; queries before/after e eventos de contador fazem o mesmo. A realização prepara identidades antes de transformar, portanto o número deixa de ordenar origens e produtos. A sonda CLI e a sentinela single_evaluation_counter_follows_causal_order conservam a evidência no diagnóstico de implementação. Aplicar o núcleo realization-order; manter fallback histórico somente sem ordem registrada.


## Realização com avaliação única

Guardar ordem causal privada por Location, preenchida durante o walk, independente do contador numérico do identificador. Estado é por registry, não global.


## Realização com avaliação única

observe_location registra primeira ocorrência; order_key consulta ordinal e conserva fallback numérico somente para Location não observada em testes/consumers legados.


## Realização com avaliação única

Lookup temporal usa ordem causal registrada, preservando a semântica do prefixo quando IDs de origem e produto não são monotônicos.

### Escopo interno por documento do bundle

O registry conserva privadamente as Locations reais emitidas por Tag::Start em cada body, em ordem de documentos. O walk registra a faixa depois de visitar o body, inclusive uma faixa vazia. A realização Page usa esse pertencimento explícito, nunca fronteiras inferidas de valores de Location ou checkpoints repetíveis. Os acessores são internos à crate e não ampliam o contrato público.


## Consulta temporal sem varredura excedente — 2026-10-13

Medição anterior à decisão: `01_core/src/entities/counter_registry.rs:176` usa
iter/filter/last sobre o Vec de histórico. O predicado consulta somente
`order_key`, sem mutação, I/O ou callbacks; compara a ordem causal registrada
com fallback numérico para locais não observados. HEAD
`06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado;
fontes, L0, núcleo pinado, horário UTC, status e diffstat em
`00_nucleo/diagnosticos/predicados-fixtures-ci-2026-10-13/before.json.gz`.

Preservar exatamente a última entrada elegível na ordem do Vec e o resultado
None quando não houver uma. A pesquisa pode partir do fim e terminar na
primeira entrada cujo order_key seja <= ao alvo (`rfind`). Não reordenar o
histórico, substituir ordem causal por IDs nem mudar updates ou init.
A consulta permanece O(n) no pior caso; não inferir ganho medido de tempo.
Esta é simplificação interna equivalente sob o núcleo realization-order,
sem mudança de assinatura pública, default ou fase. Os exemplos históricos
com comparação numérica direta não substituem a cláusula causal vigente.
Preservar os testes temporais, incluindo origens com IDs não monotônicos.


## Entradas diretas e ordem da síntese — 2026-10-17

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado; order-before.json.gz/before.json.gz preservam estado, UTC, hashes e diffstat. trace-runtime-result.json e trace-runtime.log mostram a entry direta no índice puro, runtime e L3 sob NativeElement; a linguagem bare converte outline.entry em Kind(OutlineEntry), conforme foundations/selector.rs:113. O walk terminal atual não preenche esse kind. measurement-order-before.json no pin 9f2b6e8715237cb086899a42873660fe744622e8 mede BARE [Heading] versus [Heading,Direct], FILTERED [Direct,Heading] versus [Heading,Direct], STATE 2 versus 1 e COUNTER ((2,),) versus ((1,),) na Location da entrada sintetizada. Esses são observáveis de linguagem; não se exige igualdade Rust/bytes. A síntese pós-walk aloca entradas depois dos updates, mas sua posição causal é dentro do outline anterior. O núcleo realization-order já proíbe tratar identidade numérica como ordem. A correção é refutada por ordem, prefixo temporal, duplicação ou dados de entry errados.

### Responsabilidade exclusiva

Adicionar somente pub(crate) fn observe_locations_after(&mut self, anchor: Location, locations: &[Location]). Sem API externa/campo/método de trait novo. Se anchor não foi observado, não alterar o registry; lista vazia ou já observada é no-op. Deduplicar novas Locations preservando sua ordem (HashSet puro local, não busca quadrática). Deslocar os ordinais já existentes estritamente posteriores à âncora pelo tamanho do bloco e registrar o bloco depois da âncora. Ordem densa, aliases/repetição idempotentes, ordem relativa antiga intacta; observe_location posterior continua anexando sem colidir. Não reordenar/mutar valores, ações, históricos ou document_locations, não executar callbacks nem importar compiler. O owner de introspect/outline_entries decide a âncora e o bloco; esta entidade só mantém dados de ordem, sob o núcleo realization-order pinado.

### Aceitação

Testes funcionais antes da mudança: índice bare da entry própria e aliases; ordem de três entries (dois outlines com entry direta entre eles), where e ausência de heading fantasma; estado e contador lidos na Location sintetizada devem refletir o prefixo do outline. Depois, testes de contrato dos dois helpers internos antes da sua implementação, com âncora desconhecida, duplicatas/repetição, IDs não monotônicos e append denso. Repetir nove sondas CLI sem relaxar resultados (seis originais e order/state/counter). Suíte completa, build, fmt, Clippy estrito, Rustdoc estrito, V1/V5/V15/V26/reversa/diff e SARIF: seis V16 removidos, sete V19 previstos e três mensagens existentes atualizadas, nenhum V20 novo. 12 GiB, swap zero, um job e comandos pesados sequenciais. Esta é correção interna/paridade ADR-0127, com helpers crate-private e nove pares L0/consumer; nenhuma assinatura externa, default deliberado ou fase nova. Resselo restrito, sem commit/push/release. As demais pendências de V16 continuam abertas.
