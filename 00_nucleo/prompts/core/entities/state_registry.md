# Prompt L0 — `entities/state_registry`
Hash do Código: 32ea31c3

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/realization-order.toml sha256:8de2c32fbeeee7a30088b4ff3808e15a211f23a182673ef5cdebe54bd3eadad6

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/state_registry.rs`
**Criado em**: 2026-04-30 (P171 sub-passo .E — sub-store de M9 state feature)
**ADRs relevantes**: ADR-0033 (paridade vanilla), ADR-0066 (Introspection runtime)

---

## Contexto

`StateRegistry` é o sub-store de runtime mutable state para `Introspector`. P171 (M9 sub-passo 3) — feature `state(key, init)` + `state_update(key, value)`.

Vanilla equivalente: `lab/typst-original/.../introspection/state.rs`. Cristalino isola num sub-store dedicado por simetria com `LabelRegistry`/`CounterRegistry`/`MetadataStore`.

---

## Estrutura

`HashMap<String, Vec<(Location, Value)>>` — para cada key, lista ordenada de pares (Location, value). Init é a primeira entrada (cronologicamente). Updates posteriores adicionam ao Vec.

Decisão (P171.A.3): escolhida `HashMap` por:
- Lookup por key é O(1).
- Vec interno permite append em O(1).
- `value_at(key, location)` é O(n) onde n = updates da key (aceitável; n é tipicamente pequeno).

Alternativas consideradas: `BTreeMap<Location, ...>` (ordenação automática mas lookup por key complicado), `Vec<(Location, key, Value)>` (sort needed antes de lookup).

---

## Restrições Estruturais

- Camada **L1**: struct puro, sem I/O.
- Read-only após construção (mutação só via `pub(crate) fn init` / `update` / `apply_update` durante construção em `from_tags`).
- `Clone` derivado.

---

## Interface pública

```rust
use crate::entities::location::Location;
use crate::entities::value::Value;

#[derive(Debug, Clone, Default)]
pub struct StateRegistry { /* HashMap<String, Vec<(Location, Value)>> */ }

impl StateRegistry {
    pub fn empty() -> Self;
    pub fn value_at(&self, key: &str, location: Location) -> Option<&Value>;
    pub fn final_value(&self, key: &str) -> Option<&Value>;
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;

    pub(crate) fn init(&mut self, key: String, init: Value, location: Location);
    pub(crate) fn update(&mut self, key: String, value: Value, location: Location);
}
```

**P173**: o método de conveniência `apply_update(key, StateUpdate, location)`
foi removido. Razão: `StateUpdate::Func` requer `Engine + EvalContext`
para eval, que não estão neste sub-store. O match sobre `StateUpdate`
vive agora em `from_tags::from_tags` onde Engine está disponível;
`StateRegistry` expõe apenas as primitivas `init` + `update`.

---

## Semântica

- `empty()`: registry vazio.
- `init(key, init, location)`: regista valor inicial. Apenas a **primeira chamada para cada key** é considerada — segundo init é ignorado (decisão do cristalino; ver bloco P1031).
- `update(key, value, location)`: regista update. **Se key não foi inicializada, update é ignorado** (defensive; ver bloco P1031).

> **Correcção P1031 — duas alegações sobre o vanilla, ambas sem base na fonte.**
>
> A redacção anterior atribuía ao vanilla dois comportamentos: *"segundo init é ignorado
> (paridade vanilla)"* e *"vanilla geraria erro"* para update sem init. Nenhum dos dois se
> encontra no vanilla ratificado (`e0e8ca4d`) — e a razão é estrutural: **no Typst, `state`
> não tem "registo de init" nem "update sem init".** `state(key, init)` é um construtor que
> carrega o valor inicial consigo (`crates/typst-library/src/introspection/state.rs:192-194`:
> *"The key that identifies the state."* / *"The initial value of the state."*), pelo que
> todo `update` parte de um estado já definido por construção. As duas situações que estas
> linhas descrevem não existem na linguagem.
>
> **O que a documentação diz, e que é o que importa** (doc comments `#[func]`,
> `state.rs:300-315`, publicado em `typst.app/docs/reference/introspection/state/`):
>
> *"State is a part of your document and runs like a thread embedded in the document
> content. The value of a state is the result of all state updates that happened in the
> document up until that point. That's why `state.update` returns an invisible sliver of
> content that you need to return and include in the document — a state update that is not
> "placed" in the document does not happen, and "when" it happens is determined by where you
> place it."*
>
> Ou seja: o observável é o valor num ponto do documento, determinado pela posição dos
> updates. É contra isso que a paridade se mede, não contra a mecânica do registry.
>
> **Reclassificação**: as duas regras acima passam de "paridade vanilla" a **decisões
> defensivas do cristalino** para estados que a linguagem não pode produzir. Não são achados
> de código; são achados de enquadramento, corrigidos aqui.
- `value_at(key, location)`: encontra último (key-value) pair com `loc <= location` na ordem do Vec; retorna value ou None.
- `final_value(key)`: retorna o último value registado (init se nenhum update, ou último update aplicado).

**P173**: `apply_update` removido. Match sobre `StateUpdate` (Set vs Func) vive em `from_tags` onde `Engine` está disponível para eval de Func.

---

## Algoritmo `value_at`

```rust
fn value_at(&self, key: &str, location: Location) -> Option<&Value> {
    let history = self.inner.get(key)?;
    history
        .iter()
        .filter(|(loc, _)| loc.as_u128() <= location.as_u128())
        .last()
        .map(|(_, v)| v)
}
```

Comparação por `as_u128()` directa — Locations são monotonicamente crescentes pelo `Locator` (P161), portanto a ordem de inserção é a ordem cronológica.

---

## Tests obrigatórios (sub-passo .E P171)

- `empty()` retorna `None` em qualquer query.
- `init` only → `value_at` em qualquer location ≥ init_loc retorna init.
- `update` após `init` aplica no ponto correcto (antes do update: init; depois: novo).
- Múltiplos updates em sequência: cada location resolve para o valor correcto.
- Keys distintas isoladas.
- `update` sem `init` é ignorado.
- Segundo `init` para mesma key é ignorado.

---

## Consumers actuais

Nenhum no momento da criação. Consumido em P171 .F por `rules/introspect/from_tags.rs` arms `ElementPayload::State` e `ElementPayload::StateUpdate`.

## Consumers planeados

- `entities/introspector.rs::TagIntrospector.state` field — exposição via `state_value` / `state_final_value`.
- M5 retorno: `Layouter` pode consumir `state` para resolver flags como `numbering_active("heading")`.
- M9+ features que dependem de state runtime.

---

## Sobre paridade

Vanilla resolve o estado por **iterações de layout** repetidas até convergir. Cristalino simplifica para single-pass linear (sem fixpoint M7). Suficiente para casos comuns onde `state.update` aparece no mesmo documento que `state.value_at`. Refino futuro (M9+) pode adicionar fixpoint quando consumers reais exigirem.

> **Fonte de paridade (P1031)** — a afirmação anterior (*"Vanilla `state.rs` armazena state
> via fixpoint comemo"*) nomeava o mecanismo errado: `comemo` é a camada de memoização, não
> o resolvedor de estado. O mecanismo real é a **iteração de layout**, e está **documentado
> na linguagem** — vanilla ratificado (`e0e8ca4d`),
> `crates/typst-library/src/introspection/state.rs:339-349`, doc comment do parâmetro de
> `state.update`:
>
> *"When updating the state based on its previous value, you should prefer the function form
> instead of retrieving the previous value from the context. This allows the compiler to
> resolve the final state efficiently, **minimizing the number of layout iterations
> required**. In the following example, `{fill.update(f => not f)}` will paint odd items in
> the bullet list as expected. However, if it's replaced with
> `{context fill.update(not fill.get())}`, then **layout will not converge within 5
> attempts**, as each update will take one additional iteration to propagate."*
>
> E `state.rs:184-188`: *"In general, you should try not to generate state updates from
> within context expressions. […] Sometimes, it cannot be helped, but in those cases it is
> up to you to ensure that the result converges."*
>
> **Consequência para o scope-out**: a convergência iterativa **é superfície de linguagem
> observável** (o utilizador vê o erro de não-convergência, e a página de referência de
> iterações do compilador é pública). O single-pass do cristalino não é, portanto, apenas
> uma simplificação mecânica invisível — é uma restrição de comportamento. Continua
> legítimo como scope-out declarado, mas fica registado com essa natureza e não como
> "mecânica interna". Frase corrigida acima.

---

## Resultado Esperado

- `01_core/src/entities/state_registry.rs` — struct + 8 métodos + 7 tests.
- Re-export em `01_core/src/entities/mod.rs`.

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-04-30 | P171: sub-store de runtime mutable state para Introspector M9 | `state_registry.rs`, `state_registry.md` |
| 2026-04-29 | P173: removido `apply_update` — match Set/Func vive em `from_tags` onde `Engine` está disponível | `state_registry.rs`, `state_registry.md` |




## Medição da ordem antes da decisão

No HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, CounterRegistry::value_at e StateRegistry::value_at comparam Location.as_u128; queries before/after e eventos de contador fazem o mesmo. A realização prepara identidades antes de transformar, portanto o número deixa de ordenar origens e produtos. A sonda CLI e a sentinela single_evaluation_counter_follows_causal_order conservam a evidência no diagnóstico de implementação. Aplicar o núcleo realization-order; manter fallback histórico somente sem ordem registrada.


## Realização com avaliação única

Guardar ordem causal privada por Location, preenchida durante o walk, independente do contador numérico do identificador. Estado é por registry, não global.


## Realização com avaliação única

observe_location registra primeira ocorrência; order_key consulta ordinal e conserva fallback numérico somente para Location não observada em testes/consumers legados.


## Realização com avaliação única

Lookup temporal usa ordem causal registrada, preservando a semântica do prefixo quando IDs de origem e produto não são monotônicos.


## Consulta temporal sem varredura excedente — 2026-10-13

Medição anterior à decisão: `01_core/src/entities/state_registry.rs:80` usa
iter/filter/last sobre o Vec de histórico. O predicado consulta somente
`order_key`, sem mutação, I/O ou callbacks; compara a ordem causal registrada
com fallback numérico para locais não observados. HEAD
`06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado;
fontes, L0, núcleo pinado, horário UTC, status e diffstat em
`lab/results/construction/diagnostics/predicados-fixtures-ci-2026-10-13/before.json.gz`.

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

Adicionar somente pub(crate) fn observe_locations_after(&mut self, anchor: Location, locations: &[Location]), com semântica de dados: âncora não observada/lista vazia/já observada são no-op; deduplicar novas Locations em ordem por HashSet local puro, deslocar ordinais posteriores e registrar o bloco logo depois da âncora. Preservar densidade e ordem relativa dos antigos, idempotência, clones e append posterior. O compiler fornece a decisão de âncora, nunca inferir por número de Location. Não alterar init/update, histórico, lookup, valores, fases, I/O ou interface externa. Aplicar o núcleo realization-order já pinado, sem importar compiler.

### Aceitação

Testes funcionais antes da mudança: índice bare da entry própria e aliases; ordem de três entries (dois outlines com entry direta entre eles), where e ausência de heading fantasma; estado e contador lidos na Location sintetizada devem refletir o prefixo do outline. Depois, testes de contrato dos dois helpers internos antes da sua implementação, com âncora desconhecida, duplicatas/repetição, IDs não monotônicos e append denso. Repetir nove sondas CLI sem relaxar resultados (seis originais e order/state/counter). Suíte completa, build, fmt, Clippy estrito, Rustdoc estrito, V1/V5/V15/V26/reversa/diff e SARIF: seis V16 removidos, sete V19 previstos e três mensagens existentes atualizadas, nenhum V20 novo. 12 GiB, swap zero, um job e comandos pesados sequenciais. Esta é correção interna/paridade ADR-0127, com helpers crate-private e nove pares L0/consumer; nenhuma assinatura externa, default deliberado ou fase nova. Resselo restrito, sem commit/push/release. As demais pendências de V16 continuam abertas.


## A1-S2 — leitura por objeto aprovada em 2026-10-17

### Medição anterior à decisão

HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c, working tree não commitado;
UTC/status/diffstat/fontes em before.json.gz e final-freeze.json.gz de
lab/results/construction/diagnostics/lote-a1-callbacks-reflexao-seletores-2026-10-17.
state-counterproof.json.gz registra RED bilateral: init 1/10 da mesma chave
após +1 exige 2/11; callback não lido não executa; atualização reutilizada
executa por ocorrência. Pin bd5ebec285505889e8ff1d4f68804ab147a699f3,
upstream introspection/state.rs:226–245, 419–455 e 477–520. O init do leitor
inicia a sequência; a ordem vem do documento, não do inteiro de Location.
O método atual trata Func como Set e o pós-passe eager perde essa semântica.
Estes são valores/erros de linguagem, não igualdade Rust ou algoritmo exigido.
A1-S1 foi retirada; gate A1-S2 confirmado pelo dono nesta sessão, revisão
lab/history/construction/records/00_nucleo/materialization/state-reader-runtime-2026-10-17/review.md,
SHA-256 6d6a176b8e27cc53c38fe6650c03c217448059f4e7287fd8848fe54e80247fa4.

### Responsabilidade exclusiva — supersede a fase histórica de State

Preservar definições públicas e assinaturas de StateRegistry. Adicionar somente
registro privado de ações StateUpdate por key/Location, independente de existir
init global. Capturar Set e Func, deduplicar somente a mesma ocorrência de mesma
Location, ordenar leituras por order_key causal e respeitar limite inclusivo.
Nunca executar callbacks neste owner. A visão pública materializada continua
legada; o leitor moderno não usa seu primeiro init como inicialização global.

Representar privadamente valores materializados com Arc<Value>. Operações
registradas usam enum interno pub(crate) RecordedStateUpdate: Set(Arc<Value>)
e Func(Func), sem alterar StateUpdate público. Compartilhar o Arc do Set entre
histórico legado e ação, sem clone profundo adicional. A lista de ações por key
é Arc<Vec<...>> e mutação usa Arc::make_mut; clone de snapshots compartilha dados,
mas extensões posteriores não alteram a cópia anterior. Helpers crate-private
registram ações, extraem ações ordenadas com limite e obtêm init legado elegível.
Registro Set preserva auto-init/último valor legado; Func não materializa valor.
init/update explícitos históricos continuam com suas regras e não inventam ação.
Testar ordem não numérica, snapshots independentes e partilha física do payload;
isto verifica a prescrição de RAM, sem confundi-la com paridade de linguagem.

### Aceitação e fronteiras

Gate A1-S2 ratificado para as cinco assinaturas e a fase de leitura. L0 primeiro,
RED funcional antes do código; GREEN contra o pin indicado para linguagem e
controles próprios das extensões Tekt. Não ampliar outros contratos públicos.
Build/workspace/doctests, Clippy/Rustdoc estritos, fmt, linhagem V1/V5/V15/V26 e
reversa; manter V16 sem aumento. MemoryMax12GiB/High8GiB/swap0/jobs1, tarefas
pesadas sequenciais. Medir consumo sem inferir ganho global só da representação.

O enum de ações compartilhadas é privado ao crate, não nova variante do enum
público StateUpdate. O helper extrai ordinal causal e operação compartilhada;
o resolvedor pode computar a sequência inteira da key para propagar erros
posteriores, selecionando o valor do prefixo pelo ordinal. Não filtrar ações
futuras antes de testar erros: state-future-error em target-controls.json.gz
refuta essa otimização. Uma consulta de outra key não executa estas ações.
