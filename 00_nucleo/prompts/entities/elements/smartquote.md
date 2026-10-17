# Prompt L0 — `entities/elements/smartquote` — `SmartQuoteElem`
Hash do Código: 14fb409e

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/smartquote.rs`
**Origem**: modelo D (ADR-0105), **Lote 9 P324** (por largura). Trait e glossário (§A.0): ver
`entities/elements/_comum.md`. **Não-locatável**. Leaf — `map_*` terminais.

---

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/smartquote-semantics.toml sha256:dc549d79f9e996433894effc8e4efd1f8f3da3d9c30933110eeabbab582ae9a8

## Struct vigente (ratificação 2026-10-06)

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SmartQuoteElem {
    pub double: Option<bool>,
    pub enabled: Option<bool>,
    pub alternative: Option<bool>,
    pub quotes: Option<SmartQuoteQuotes>,
}
```

`Content::SmartQuote { double }` → `Content::SmartQuote(Arc<SmartQuoteElem>)`.
Construtor ergonómico: `Content::smartquote(double: bool)`. **Deriva `Hash`/`Eq`**
(todos os campos, incluindo presença).

## P1286 — configuração explícita (GATE ADR-0127)

### Medição anterior à decisão

O vanilla (`text/smartquote.rs:51-89,201-317,335-419`) distingue argumento
omitido, `auto` explícito e override single/double. A auditoria cristalina
mostrou que o carrier alternativo `Content::Styled` é transparente em
`repr_content`, mas não em `content.func()`; portanto não preserva toda a
morfologia pública da folha. O carrier local deve permanecer na própria
`Content::SmartQuote`.

### Contrato público histórico P1286 (supersedido pelo contrato ratificado)

Após confirmação, os tipos canônicos são:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SmartQuotePair {
    pub open: EcoString,
    pub close: EcoString,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SmartQuoteOverrides {
    pub single: Option<SmartQuotePair>,
    pub double: Option<SmartQuotePair>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SmartQuoteQuotes {
    Auto,
    Custom(SmartQuoteOverrides),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SmartQuoteElem {
    pub double: bool,
    pub alternative: Option<bool>,
    pub quotes: Option<SmartQuoteQuotes>,
}
```

`None` externo significa argumento omitido e herança da StyleChain;
`Some(false)` é override explícito de `alternative`; `Some(Auto)` é
`quotes:auto` explícito e apaga quotes herdadas. Em `Custom`, `None` por membro
significa usar o par localizado para esse membro. String/array simples
preenchem apenas `double`; o dicionário pode preencher `single` e `double`.

`Content::smartquote(double)` permanece com a assinatura pública vigente e
constrói ambos os campos novos como `None`. Igualdade/hash incluem os três
campos; `plain_text`, `is_empty` e `map_*` conservam a semântica vigente e
preservam a configuração. A forma de `content.fields()` para estes novos
argumentos não foi medida e permanece `Unknown`; não a inventar neste lote.

Adicionar campos e tipos públicos ativa **PARAGEM OBRIGATÓRIA** ADR-0127.

## `impl Element for SmartQuoteElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `if self.double { "\"".to_string() } else { "'".to_string() }` (`content.rs:1722`) |
| `is_empty` | default `false` (`content.rs:1662`) |
| `map_content`/`map_text` | **terminais** (leaf) |
| `get_field`/`element_kind`/`to_payload` | default |

## `eq`

No estado histórico, `#[derive(PartialEq)]` comparava `double` (paridade
`content.rs:1992`). Após confirmação/materialização P1286, compara também
`alternative` e `quotes`, conforme o contrato público proposto acima.

> **Arm `|`-combinado**: `SmartQuote` está no arm terminal combinado de
> `map_content`/`map_text` com outros leaves. Como é terminal (sem binding),
> fica **no mesmo arm** (`{ .. }` → `(_)`) — **sem split**.


## Aspa semântica e resolução tardia — ratificada em 2026-10-06

O contrato desta seção é vigente e supersede as cláusulas históricas conflitantes
das seções anteriores apenas para SmartQuote. A ratificação humana autoriza a
materialização em fluxo RED→GREEN, com ownership exclusivo e pins abaixo.

### Medição anterior à decisão

smartquote-2026-10-06/{before,fields-before,partial-and-html,proposal-controls}.json.gz
registra pin 9f2b6e8715237cb086899a42873660fe744622e8, SHA-256 dos binários,
HEAD/working tree, diff HEAD --stat, UTC, fontes, argv/cwd/ambiente e stdout/stderr.
PDF usa texto extraído, HTML usa DOM/texto e diagnósticos, não igualdade de bytes PDF.
A avaliação converte aspas de markup para Text; campos double/enabled são perdidos.
A variável contendo aspas mantém glyph inglês sob uso em alemão/francês ou
smartquote(enabled:false). Primes 6'/2" viram aspas no Tekt. Show da função
nativa e nome público foram corrigidos separadamente e passam antes deste gate;
markup ainda não é SmartQuote. Constructor vazio tem fields vazio no vanilla;
double:false é campo conhecido, mas double omitido é Unset. enabled:false
permanece SmartQuote no vanilla. set double:false e show filtrado têm positivos
novos. HTML literal funciona porque recebe Text; SmartQuote programática hoje
é rejeitada. Título literal renderiza aspas; script recebe string, rejeitando
Content antes da exportação nos dois binários.

Fontes: markup_stream.rs:79–128 decide glyph em eval; smartquote.rs de layout
alterna dois flags sem texto precedente; stdlib/text/smartquote.rs:112–117 apaga
elemento disabled; observation/elements.rs:242–250 compara os campos atuais.
Vanilla typst-eval/markup.rs:141–146 cria elemento com double explícito;
typst-layout/inline/collect.rs:196–211 resolve estilo/contexto na coleta;
typst-library/text/smartquote.rs:119–158 distingue prime/apóstrofo/nesting.

### Decisão ratificada e limites

Preservar identidade/campos de SmartQuote em eval e resolver glyph só na
composição de render de cada target. Paged usa sua unidade; HTML permanece
semântico e não chama layout paginado para aspas (ADR-0128). A lógica de
seleção contextual é helper puro L1 único, com estado por fluxo/compilação,
sem I/O, globais, vtable, registry de propriedades ou cópia do algoritmo de
memoização upstream. Stack/estado é mecânica escolhida do Tekt; linguagem,
campo conhecido/omitido, glyph e estilo são aceitação. Escaping HTML permanece
com seus owners. Sem novos modos/flags, unsafe, API de watch ou math phase.

Este gate muda double público bool→Option<bool>, adiciona enabled:Option<bool>,
expõe helper/contexto puro e move escolha de glyph eval→render. Confirmação humana recebida em 2026-10-06. O contrato ratificado abaixo
supersede cláusulas históricas conflitantes apenas para aspas; ADRs/núcleos
existentes e propostas alheias permanecem intactos.

Contrato público ratificado:
```rust
pub struct SmartQuoteElem {
    pub double: Option<bool>,
    pub enabled: Option<bool>,
    pub alternative: Option<bool>,
    pub quotes: Option<SmartQuoteQuotes>,
}
```
None é argumento omitido/herdável; Some(false) é explícito, não default.
Markup tem double Some(kind). Função sem double tem None. Eq/Hash/Clone
incluem todos os campos; map_* mantém leaf/configuração. plain_text continua
fallback ASCII de double.unwrap_or(true), sem resolver estilos/glyphs. Preservar
SmartQuotePair/Overrides/Quotes. Este schema substitui o bool obrigatório
histórico; altera compatibilidade de structs Rust, explicitamente neste gate.
