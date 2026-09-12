# Prompt L0 — `compiler/eval/bindings/value_methods` — métodos de instância com args em AST
Hash do Código: 0e9227bb

## P1339 — adapter ligado de version.at sem parser duplicado

### Medição anterior à decisão

No HEAD `2f42d64253547734564513a1159ee6b584c1c4b4`, consumer intacto,
`value_methods.rs:623-662` valida índice ligado com mensagem genérica de
aridade e não rejeita named; `entities/version.rs:103-120` já possui a
regra de índice. As sondas P1339 full-final e boundaries medem as duas
formas e suas origens; `diagnosticos/p1339-remaining-l0-design.md` fixa
a classificação e a delegação proposta.

### Decisão

eval_version_method_value torna-se adapter AST→Args: avalia uma vez,
conserva ocorrências/spans e delega receiver e Args a
stdlib::dispatch_version_method. O owner primitives-constructors/version
faz a inserção causal de self, a validação comum e os diagnósticos; a fórmula
permanece Version::at. Não conservar o parser anterior, sua tolerância a
named ou mensagem genérica de aridade. O span integral da chamada é
transportado pelo caller interno, sem inventar origem de argumento.

Demais métodos de state/counter/color/selector não migram para este adapter.
Campos de Version, constructor, componentes e PARITY_VERSION ficam intactos.
Sucedem somente a localização anterior do parser ligado e suas divergências
diagnósticas, sem API externa nova nem segunda causa semântica.

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/eval/bindings/value_methods.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/eval/bindings.md`
**ADRs**: ADR-0107 (paridade língua), ADR-0044 (`Engine<'_>`)

---

## Contexto

Este nó reúne os métodos de instância sobre tipos de valor cujos **argumentos
ainda estão em AST** no momento do despacho — `state`, `counter`, `color`,
`version` e os combinadores de `selector`. Todos partilham a mesma abertura
(`call_dispatch::eval_args`) e por isso recebem `Scopes`/`EvalContext`/`Engine`.

**No vanilla não existe um ficheiro correspondente**: estas são nativas
declaradas junto do tipo em `typst-library::{introspection, visualize,
foundations}`. A agregação é do cristalino. O que justifica o nó não é o
vanilla mas a co-mudança: P506 (state+counter), P640 (counter.display), P742
(color+state), P796 (version), P417/P423/P504 (selector) movem estes símbolos
em conjunto e quase nunca em conjunto com os outros nós.

## Restrições Estruturais

- L1 puro; nenhuma leitura de ficheiro ou relógio.
- O contexto é usado de três formas distintas, **medidas por `file:line`** e a
  respeitar em qualquer alteração (lição do Passo 1012 — a presença do
  parâmetro não é prova de uso):
  1. **Pass-through**: `eval_version_method_value`, `eval_element_where`,
     `eval_selector_or_and`, `eval_selector_within` usam `ctx`/`engine`
     **apenas** para `eval_args`; depois disso, zero usos.
  2. **`Engine` real**: `eval_color_method` lê `engine.world` e
     `engine.current_file` para as nativas de cor.
  3. **Estado/introspecção**: `eval_state_method` e `eval_counter_method_value`
     propagam `ctx` para `state_*`/`counter_*` e para o introspector.

## Instrução

### `eval_state_method(state, method, args, …)`

`update`, `get`, `display`, `at`, `final` — delegando em
`stdlib::state::{state_update, state_get, state_display, state_at_location,
state_final}`. `at` passa por `state_at_dispatch`, que resolve o argumento
(label ou localização) antes de consultar o introspector.

### `eval_counter_method_value(counter, method, args, …)`

`step`, `update`, `get`, `display`, `at`, `final` — delegando em
`stdlib::counter::*`. O caminho de `display` passa por
`parse_counter_display_args` (padrão de numeração opcional),
`extract_display_at_label` e `render_counter_at_label` (P640 — unificação de
`counter.display` e das suas mensagens de erro).

### `eval_color_method(color, method, args, …)`

`lighten`, `darken`, `saturate`, `desaturate`, `negate`, `rotate`, `mix`,
`opacify`, `transparentize`, `components`, `space`, `to-hex` — delegando em
`stdlib::color`. Único método deste nó que precisa de `World`/`current_file`.

### `eval_version_method_value(version, method, args, …)`

`at` — componente por índice (P411/P796).

### Combinadores de selector

`eval_element_where` (`elem.where(field: v)`), `eval_selector_or_and`
(`.or()`/`.and()`), `eval_selector_within` (`.within()`), suportados por
`value_to_query_selector` (conversão `Value → Selector`).

## Critérios de Verificação

```
#let s = state("k", 0); s.update(1); #s.get()      → 1
#counter("c").step(); #counter("c").get()          → (1,)
#counter(heading).display("1.1")                   → numeração conforme padrão
#counter("c").at(<lbl>)                            → valor no local da label
rgb("#ff0000").lighten(50%)                        → cor mais clara
rgb("#ff0000").to-hex()                            → "#ff0000"
version(1, 2, 3).at(1)                             → 2
heading.where(level: 1)                            → Selector de heading nível 1
heading.where(level: 1).or(strong)                 → Selector Or
figure.where(kind: image).within(heading)          → Selector Within
```

## Resultado Esperado

- `eval_state_method`, `eval_counter_method_value`, `eval_color_method`,
  `eval_version_method_value`, `eval_element_where`, `eval_selector_or_and`,
  `eval_selector_within` visíveis em `eval`; os auxiliares privados ao nó.

## P1149 — argumentos contextuais completos de counter

`parse_counter_display_args` preserva `at:` como `Label`, `Location`,
`Selector` ou `Auto`, aceita `both: bool` e transporta o numbering posicional
como string, função ou `Auto` ao owner. Não formata por concatenação local.

`eval_counter_static_method_value` cobre somente `counter.at` e
`counter.display`, extraindo o receiver e preservando literal label antes da
avaliação genérica. Depois delega a `stdlib/counter`; resolução de location,
numbering, total final e callback não vivem neste nó.

## P1284 — limite fechado do glue AST

Este nó continua owner apenas da orchestration que realmente precisa dos
argumentos em AST: `state`, `counter`, `color`, `version`, `where` e os
combinadores de selector. Coleções, `arguments`, direção, alinhamento,
duração, comprimento e localização não movem sua semântica para cá.

Para selector, P1284 autoriza somente:

- `and(self, ..others)` → `Selector::And`, preservando receiver primeiro e a
  ordem dos demais selectors;
- `or(self, ..others)` → `Selector::Or`, com a mesma disciplina;
- `within(self, ancestor)` → `Selector::Within` já existente.

As formas não ligadas recebem `self` primeiro e chamam estas mesmas rotas.
Conversão `Value → Selector`, aridade, tipo, metadata e erros são únicos; não
há implementação paralela no wrapper.

`before(self,end,inclusive:true)` e `after(self,start,inclusive:true)` ficam
fora do match e são `BLOCKED_ADR0127_PUBLIC_CONTRACT`: o enum público atual
não contém as variantes, e os consumers exaustivos exigiriam revisão. Não
simular com `Within`, `And`, `Or`, `Where` ou composição aproximada.

No ramo de cor, `color.map` não passa por este nó: é constante de tipo com
kind `module`, descoberta por `field_access` e construída por `stdlib/color`.
`color.spot/tint` permanece bloqueado. Nenhuma alteração de contrato Rust
público, default ou fase é autorizada.

## P1307-R4 — transporte do carrier Args aprovado

### Medição anterior à decisão

Sobre HEAD `b303f1f15b610e09872b567027e0d806387fde8c` e working tree composto
P1306 + L0 R3 aprovado, baseline R4 SHA-256
`52df1c661c20d9eb612bbd5c89ae3cae8c44735aeab27c11e4a1da0d4d145e57`,
`01_core/src/compiler/eval/bindings/value_methods.rs:202` insere Color em
Args avaliado; `:256-284,524-548` constrói Args de display a partir de AST
com spans disponíveis, mas guarda somente views. O texto L0 anterior foi
congelado em `00_nucleo/diagnosticos/p1307-r4-transport-amendment.json`.
Esses writers não podem manter Some incoerente após migração de eval_args.

### Decisão de adaptação interna

Este owner consome a API de `entities/args.md` aprovada pelo dono no gate
P1307-R3. Na chamada ligada de cor, antepor uma ocorrência positional com
Value::Color do receiver e spans individuais detached (o helper não recebe
origem lexical do receiver), reconstruindo por from_occurrences e preservando
span agregado e origens dos demais argumentos. None continua síntese explícita
por from_parts. Não inferir origem por igualdade de cor nem invalidar Some.

Os dois builders AST de counter.display devem construir ocorrências de cada
argumento aceito na ordem de avaliação, com span do argumento completo e da
expressão-valor, e finalizar por from_occurrences com o agregado vigente.
O literal label em at continua preservado pelo tratamento sintático existente;
não avaliá-lo novamente. Manter critérios, aridade, casts, rejeição de spread,
mensagens e ordem de falhas atuais; não promover estes métodos à validação
nova de encoders ou ampliar suporte de counter/selector/color.

Testes preservam os caminhos ligados/estáticos de display, at/both, label,
cor e erros preexistentes, além da coerência Some/views. É inferência que
esses dados já disponíveis bastem; nova informação de domínio ou mudança
pública além do carrier aprovado exige reabertura. Fluxo contínuo ADR-0127
para esta migração interna, com L0 primeiro e verificação independente.

## P1339 — semântica comum de `function.where` e chamada ligada

### Medição anterior à decisão

HEAD `2f42d64253547734564513a1159ee6b584c1c4b4`, consumer sem diff:
`value_methods.rs:676-750` só reconhece heading/figure, rejeita strong/emph/raw,
descarta spreads e exige ao menos um campo. No vanilla ratificado,
`foundations/func.rs:412-450` documenta o filtro por função de elemento,
valida nomes, não converte valores pelo tipo do parâmetro construtor e
rejeita função não-elemento. `With(elemento)` também é rejeitado.
`p1339-full-final-vanilla-runs.json` e `p1339-full-boundaries-vanilla-runs.json`
medem tipo, repr, prioridade de falhas, spans e formas estática/ligada sobre
o baseline e estado integral registrados em cada recibo.

A sonda focal `p1339-where-l0-vanilla.json`, UTC
`2026-09-10T00:04:21.279738+00:00`–`00:04:21.402736+00:00`, confirma que
o grupo preserva ordem e que spread sobrescreve o valor sem mover a chave.
Isso é linguagem; a estrutura SmallVec do vanilla não prescreve a nossa.

### Decisão proposta — owner semântico, sem novo módulo produtivo

Este owner passa a possuir um helper puro interno de construção de `where`
sobre receiver Func e Args já avaliados. A chamada estática e a ligada
devem convergir nesse helper; não duplicar validação ou normalização. As
assinaturas auxiliares podem ser `pub(crate)`/visibilidade entre módulos,
nunca uma nova API pública de entidade/trait.

Somente para `where`, esta decisão sucede o limite P1284 que restringia o
owner à orquestração com AST. As demais famílias mantêm esse limite; não se
transfere semântica de coleções, encoders ou outros métodos para este nó.

1. Reconhecer função nativa real de elemento por identidade de implementação
   (`native_fn_addr`/equivalente tipado e `fn_addr_eq`), não só por nome,
   namespace, igualdade Func ou resultado de executar o receiver. Aliases
   preservam identidade. Não desembrulhar With, promover closure/plugin ou
   confundir o Element de utilizador com um builtin vanilla.
2. O conjunto de reconhecimento é estático e declara os campos linguísticos
   válidos por elemento. Deve cobrir os elementos expostos pertinentes, não
   somente os exemplos heading/figure/strong/emph/raw/text/table da sonda.
   Não criar registro genérico, vtable nem usar o fallback não chamável de
   `content_elem_func` como identidade de elemento.
3. Consumir Args mantendo ocorrências e spans; avaliar uma vez, na ordem
   observável do caminho. Duplicação sintática, efeitos de spreads, cast do
   receiver e positional extra conservam a prioridade vanilla medida. A
   forma estática avalia argumentos antes do cast; a ligada resolve o
   receiver antes dos argumentos. A convergência semântica não apaga essa
   diferença de avaliação. Nunca reavaliar receiver em fallback.
4. Validar nomes de campos, sem cast dos valores: por exemplo
   `heading.where(level: "bad")` é filtro válido, não erro de constructor.
   Chaves repetidas via spread têm último valor e primeira posição;
   literais named duplicados continuam sujeitos ao erro sintático.
5. Produzir `Selector::Element { function, fields }`, inclusive grupo vazio.
   Não usar a cadeia histórica de QuerySelector::Where para representar um
   grupo público; suas variantes existentes continuam válidas para outros
   consumers. Ordem de avaliação nunca se reconstrói do grupo normalizado.

Mensagens e origens obrigatórias seguem os casos medidos:

```text
missing argument: self
expected function, found <tipo>
`where()` can only be called on element functions
element `<nome>` does not have field `<campo>`
unexpected argument
```

O positional inesperado aponta à ocorrência original. Não uniformizar erros
da forma ligada inválida com os casts da
forma estática. O nome público da função descoberta é `where`; extrair
`f.where` como valor não se torna válido só porque `function.where` existe.

O helper não realiza conteúdo, não consulta introspector e não imprime repr.
Encaminhamento por field_access/call_dispatch, apresentação, query e counter
exigem suas atualizações L0 proprietárias na continuação de P1339 antes do
código. Esta seção não autoriza alterar outros métodos deste owner nem
antecipa os lotes seguintes.

Aceitação posterior ao gate: criação e identidade de seletor, filtros vazios
e múltiplos, aliases, spreads, rejeição de With/closure/nativa não-elemento,
ordem de panic e spans; aplicação de filtros positivos e negativos por show.
Inferência de que Args contém as origens suficientes deve ser refutada por
uma chamada com perda de span/ocorrência; não inventar campos públicos para
reparar isso incidentalmente. A cláusula P1284 que não autorizava mudança
pública continua valendo fora da extensão explicitamente submetida aqui.

### Integração de locatability e despacho de counter P1339

Medição: query/counter exigem LocatableSelector no vanilla
(`introspection/query.rs:160-175`, `introspection/counter.rs:338-357`), mas
where/show aceitam text. O reconhecimento nativo deste owner deve fornecer
também um predicado interno estático de locatability para os consumers de
Selector::Element; suas entradas devem ser justificadas pelas declarações
vanilla e pela classificação de ocorrência cristalina. Strong/Emph aprovados
são aceitos, text não. Não promover função por nome nem executar constructor;
não fazer o próprio where rejeitar um elemento válido só por não locatável.

Medição de despacho: `value_methods.rs:392,397,478` já tem ctx/scopes/engine
quando chama os helpers públicos de counter, mas get/final antigos não
recebem Engine. Na resolução de fase aprovada de `compiler/stdlib/counter.md`,
as chamadas ligadas cuja chave contenha Element delegam aos helpers internos
owned com esses recursos já disponíveis. Preservar consumo/validação de Args
e suas origens; os demais counters seguem os helpers anteriores. Não alterar
assinaturas públicas para transportar Engine nem executar Func na simples
descoberta do método. A resolução contextual pertence ao owner de counter;
este consumer apenas transporta os recursos já disponíveis.

### P1339 — resolução de label é uma entrada contextual

Medição no HEAD `2f42d64253547734564513a1159ee6b584c1c4b4`, consumer
intacto: `value_methods.rs:127-153` resolve label antes de state_at_location;
uma ausência devolve Err antes de entrar no owner state. Rastrear apenas
state_value perderia essa dependência. Proveniência no recibo de integração
de observações P1339.

O helper interno de resolução registra label original, Location encontrada
ou erro e span no registro privado de EvalContext. Faz isso para literal e
valor avaliado, sem avaliar novamente o argumento. Replay consulta o label
original contra candidate, conservando mensagem/ordem preexistentes; não
reavalia AST nem substitui o pedido por Location previamente resolvida.
O owner state registra depois sua leitura de valor, caso tenha sido alcançada.
Ausência que vira presença invalida a tentativa, inclusive quando esta já
estava selecionada por leitura Element anterior. A resolução não marca
seleção filtrada por si só. Não ampliar selectors aceitos por state.at.

No despacho de counter, transportar a demanda aos helpers owned antes da
resolução introspectiva que possa falhar; não executar uma consulta invisível
no glue nem corrigir fallbacks legados. Parsing e avaliação de argumentos
continuam na ordem vigente, com suas leituras causais registradas normalmente.

## P1342 — origem da ocorrência ligada `counter.update`

### Medição anterior à decisão

`diagnosticos/p1342-topology-audit-r1.md` mede que
`eval_counter_method_value` possui `args.span()` antes de `eval_args`, mas o
descarta ao chamar `counter_update`. O span do callback não identifica a
ocorrência de update.

### Decisão vigente

Sob `cfg(p1339_observation)`, somente o ramo ligado `update` passa ao owner de
counter o span agregado capturado antes da avaliação e o `EvalContext` real.
O owner anexa o carrier somente se a ação efetiva for `CounterUpdate::Func`.
Avaliação, prioridade de erros, Args e delegação continuam uma vez e na ordem
vigente. A forma estática faz captura equivalente no owner, que já recebe
`Args.span` e `EvalContext`.

Não inferir origem do Func, valor ou output nem procurar por nome. Sem o cfg,
a chamada e assinatura normal permanecem atuais. O recorte cobre o callback
focal P1342, não step/set, todos os métodos de counter ou a matriz P1340.
