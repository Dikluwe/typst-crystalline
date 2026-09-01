# Prompt L0 — `stdlib/foundations/selector` — construtor e parsing
Hash do Código: 38a8528f

**Camada**: L1  
**Ficheiro alvo**: `01_core/src/compiler/stdlib/foundations/selector.rs`  
**Origem**: P1140.1-A — atomização behavior-preserving de `query.rs`  
**ADRs**: ADR-0107, ADR-0109

## Responsabilidade

Esta unidade possui:

- `native_selector` — converte kind string, label string ou função nativa de
  elemento em `Value::Selector`;
- `element_kind_of_native_func` — mapeamento estático de funções de elemento;
- `parse_selector_arg` — parser partilhado por `query` e `locate`.

`query.rs` continua dono de query/locate/here/target/metadata e chama
`parse_selector_arg` por free function. Não há callback, registro dinâmico,
vtable ou nova visibilidade além da mínima entre módulos irmãos.

## Semântica preservada

- `"<sec>"` → `Selector::Label`;
- kind conhecido → `Selector::Kind`;
- `Location`, `Selector` e `Label` preservam identidade semântica;
- funções `heading`, `figure`, `table`, `metadata` mapeiam para seus kinds;
- função não-elemento erra `only element functions can be used as selectors`;
- aridade, tipos e mensagens permanecem byte-idênticos.

Os testes de constructor e do parser mudam junto com esta unidade. A
atomização não converte ainda o binding `selector` de `function` em `type`;
isso pertence a P1140.1-B e ao gate ADR-0127.


## P1140.1-B — medição anterior à decisão (2026-08-23)

No vanilla ratificado `a51e02804`, `repr(type(PATH))` devolve `"type"`; no
cristalino anterior a esta mudança devolve `"function"`. O catálogo P1140 e
os probes públicos em `00_nucleo/diagnosticos/superficie-linguagem-p1140*`
medem a divergência para `decimal`, `duration`, `regex`, `selector`, `stroke`,
`tiling` e `version`. Os construtores atuais foram novamente executados após
a atomização P1140.1-A: catálogo byte-idêntico e 22 probes byte-idênticos ao
baseline estrutural. Esta é divergência de semântica pública da linguagem,
não de mecânica Rust (ADR-0107).

## P1140.1-B — binding de tipo

O binding global `selector` passa a `Value::Type(Type::Selector)` e sua chamada
delega a `native_selector`. O parser compartilhado de query/locate não muda.
Somente o kind público do binding é corrigido.

## P1285 — inputs públicos já materializados

### Medição antes da decisão

No vanilla ratificado, `selector(heading.where(level: 1))` preserva o selector
recebido; `selector("")` falha com `text selector is empty`;
`selector(regex(""))` falha com `regex selector is empty`; e uma regex não
vazia que casa texto vazio, como `a*`, falha com
`regex matches empty text`. A primeira candidata P1285 ainda devolvia
`argumento inválido (selector|regex)`, impedindo inclusive o fallback público
de serialização de `Value::Selector`.

### Decisão

`native_selector` aceita `Value::Selector` por identidade e `Value::Regex`
como `Selector::Regex` depois das duas validações de vazio. String vazia usa o
diagnóstico textual medido. Kind/label/função existentes preservam sua
semântica. Não se adiciona variant, assinatura ou tipo público; é correção de
paridade do constructor já exposto.

Aceitação: selector composto preserva `repr`; os três vazios falham com as
mensagens medidas; regex válida não vazia produz `Value::Selector`.
