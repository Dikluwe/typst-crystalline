# Prompt L0 — `stdlib/foundations/regex` — construtor `regex`
Hash do Código: 39b9bb03

**Camada**: L1  
**Ficheiro alvo**: `01_core/src/compiler/stdlib/foundations/regex_constructor.rs`  
**Origem**: P1140.1-A — atomização behavior-preserving de `str.rs`  
**ADRs**: ADR-0107, ADR-0109

## Contrato

`native_regex(pattern: str) -> regex` compila o padrão e devolve
`Value::Regex`. Rejeita named args, aridade diferente de um, valor não-string e
padrão inválido com as mesmas mensagens vigentes antes da atomização.

O módulo contém o construtor e seus testes; não contém métodos de instância de
string nem lógica de eval. `foundations/mod.rs` reexporta a função pelo mesmo
path público Rust anterior.

## Critérios

```text
regex("a+")         -> Value::Regex
regex("[")          -> Err
regex(1)            -> Err
regex("a", foo: 1) -> Err
```

Mover para este ficheiro não altera nenhum observável nem transforma ainda o
binding global de `function` em `type`; essa mudança pertence à fase semântica
P1140.1-B e ao gate ADR-0127.


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

O binding global `regex` passa a `Value::Type(Type::Regex)` e sua chamada
delega a `native_regex` desta unidade. Compilação, erros e resultado permanecem
os mesmos; somente `repr(type(regex))` muda de `"function"` para `"type"`.
