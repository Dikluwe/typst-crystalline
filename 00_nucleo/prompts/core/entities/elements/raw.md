# Prompt L0 — `entities/elements/raw` — `RawElem`
Hash do Código: 91c80bf8

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/raw-field-presence.toml sha256:f29e26ff07053f940cfe4ab5358fc9a6ac8d4d689528e169b5f8567492f52f46

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/raw.rs`
**Origem**: modelo D (ADR-0105), **Lote 7 P322** (por largura). Trait e glossário (§A.0): ver
`entities/elements/_comum.md`. **Não-locatável** (confirmado P322). Comportamento
idêntico ao braço atual.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct RawElem {
    pub text:  EcoString,
    pub lang:  Option<EcoString>,
    pub block: bool,
    pub set_fields: u8,
}
```

`Content::Raw { text, lang, block }` → `Content::Raw(Arc<RawElem>)`.
Construtor ergonómico preservado: `Content::raw(text, lang, block)`.
(P502 — `native_raw` passa a aceitar `lang:` e `block:` named; syntax highlighting real continua scope-out per ADR-0054.)
**Deriva `Hash`** (`EcoString`/`Option<EcoString>`/`bool`).

## `impl Element`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `self.text.to_string()` (`content.rs:1683`) |
| `is_empty` | default `false` |
| `map_content`/`map_text` | **terminais** (folha) |
| `get_field`/`element_kind`/`to_payload` | default |

## `eq`

`#[derive(PartialEq)]` compara `text + lang + block + set_fields` (paridade `content.rs:1857`).

## P1413 — `RawLineElem` (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

No baseline `e7b2ddd44`, o cristalino não possui `raw.line`. No vanila
ratificado, cada raw inline/bloco sintetiza uma instância por linha, inclusive
linhas vazias, e o constructor público aceita quatro argumentos requeridos.

### Contrato proposto

O mesmo owner declara `RawLineElem { number: i64, count: i64, text: EcoString,
body: Content }`. `plain_text` devolve `text`; `map_content` e `map_text`
atravessam `body`; `get_field` expõe exatamente `number`, `count`, `text` e
`body`. A identidade dinâmica estável é `raw.line`, reutilizando
`Content::Dynamic` sem ampliar o enum fechado `Content`. Igualdade e hash são
estruturais. O constructor rejeita cardinalidade ou tipos incompatíveis.

Esta ampliação de contrato público aguarda confirmação humana explícita antes
de L1.


## Presença lexical de Raw — ratificada em 2026-10-14

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
lab/results/construction/diagnostics/content-serialization-2026-10-14/
{before,raw-controls}/results.json.gz conserva fonte/estado/UTC, pin vanilla
9f2b6e8715237cb086899a42873660fe744622e8, argv e SHA-256 dos binários.
RawElem em entities/elements/raw.rs:21–25 tem text/lang/block, sem presença.
raw("x") e raw("x",block:false) são iguais no Tekt e diferentes no pin;
lang:none também se perde. fields/has/at e JSON/YAML confirmam a distinção.
Fonte pinada typst-library/src/text/raw.rs:289/324/361 declara os campos;
markup e display assentam block e, quando fornecido, lang. Isso é linguagem,
não a obrigação de usar o storage, Packed ou igualdade Rust do upstream.

### Contrato público ratificado

Aprovação registrada em lab/history/construction/records/00_nucleo/materialization/raw-presenca-2026-10-14/approval-receipt.json. Adicionar somente o campo público set_fields:u8 a RawElem,
mantendo text:EcoString, lang:Option<EcoString> e block:bool. Declarar no
mesmo owner pub const RAW_SET_BLOCK:u8=1 e RAW_SET_LANG:u8=2. Bits restantes
são reservados e os construtores do repositório os deixam em zero.

text é sempre conhecido. Bit block define se block é conhecido, inclusive
false. Bit lang define se lang é conhecido; Some vira Str e None conhecido
vira Value::None. A ausência do bit significa Unset, não default exposto.
Implementar get_field do trait Element já existente para text e campos
conhecidos; desconhecidos/omitidos devolvem None. Não adicionar método ao
trait, mudar tipo dos campos existentes ou introduzir fase/mode/flag.

Clone/Hash/PartialEq conservam o campo. Map de folha copia a presença.
ObservationEq deve considerar a presença, pois getters e encoders observam
a diferença. O teste de igualdade da linguagem é o par medido de valores,
não uma obrigação de igualdade interna com Rust ou com o vanilla.

### Adaptações necessárias depois da aprovação

Content::raw(text,lang,block) mantém assinatura e comportamento visual:
marca block como conhecido (parâmetro obrigatório) e lang conhecido quando
Some. O native_raw ajusta a máscara conforme argumentos lexicais: block é
conhecido para Bool fornecido; ausência e a conversão legada block:none
continuam ausência. lang é conhecido sempre que o argumento existe, inclusive
none. A compatibilidade atual de raw() e os casts/diagnósticos legados não
são alterados por este gate; permanecem pendências separadas, sem atribuir
paridade a essas diferenças. Não há busca no AST por nomes nem dupla avaliação.

Content::get_field delega ao elemento. content_methods distingue Set/Unset/
Undeclared para text/block/lang e enumera somente conhecidos na ordem acima.
CLI e encoding usam a máscara/projeção lexical, mantendo snapshots Some
exatos. RawLine, layout, fontes, temas e enriquecimento de campos realizados
não são autorizados por esta mudança. Não promover Raw a locatable.

### Aceitação e gate

Teste RED→GREEN para omitted/false/true, lang omitted/none/string, fields/
has/at, aliases/Source, mapas, igualdade, serialização CLI/stdlib JSON/YAML e
snapshots Some/None. Validar markup inline/bloco e apresentação Bool/Auto,
sem perder o produto/render existente nem executar show duas vezes. Revalidar
suíte completa, build, linhagem 1:1/núcleo/pins e inventário de avisos.

A adição de campo público quebra construção externa por struct literal:
esse é o motivo concreto de ADR-0127 §2.1/§2.4. Nada desta seção autoriza L1
antes de confirmação humana do manifesto. Após aprovação, instalar o L0 e o
núcleo nos caminhos canônicos, atualizar os owners dependentes com pins,
escrever testes RED e só então implementar. Os rascunhos em materialization
não são L0 vigente nem alvo de @prompt.

A seção de presença ratificada supersede o getter default e a descrição inicial sem máscara somente para RawElem. A proposta histórica RawLine não é alterada nem ratificada por este recibo.
