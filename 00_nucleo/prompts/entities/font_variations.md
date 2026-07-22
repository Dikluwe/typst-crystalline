# Prompt L0 — entities/font_variations
Hash do Código: 1f69c06e

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/font_variations.rs`
**Passo**: P836 (achado #21 de P831)
**ADRs relevantes**: ADR-0033 (paridade vanilla — mensagens de erro são
observáveis), ADR-0107 (paridade com a linguagem), ADR-0108 (medição
antes de decisão — valores abaixo medidos em `temp/p836/` com vanilla
0.15.0, `lab/typst-original/target/release/typst`).

## Contexto

`FontVariations` é o contraponto cristalino de
`lab/typst-original/crates/typst-library/src/text/font/variations.rs::FontVariations`:
a lista de coordenadas de eixo OpenType explícitas vindas de
`#text(variations: (wght: 250))` (constructor) e de
`#set text(variations: ...)` (set rule — no vanilla o campo é
`#[fold] #[ghost]`, `text/mod.rs:846-850`, logo settable e dobrável).

L1 não pode depender de `ttf_parser` — a tag é armazenada como
`[u8; 4]` (padding com espaço `0x20`, paridade
`Tag::from_bytes_lossy`, `tag.rs:22-27`).

## Interface pública

```rust
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FontVariations(pub Vec<([u8; 4], f32)>);

impl FontVariations {
    /// `true` se não houver eixos.
    pub fn is_empty(&self) -> bool;

    /// Valida um `Value` vindo do eval (espera `Value::Dict`) e constrói
    /// a lista normalizada. Erros replicam o vanilla verbatim
    /// (mensagem + hints), com o hint final
    /// `occurred in tag at index {i} (`"{key}"`)` por entrada
    /// (`variations.rs:234 tag_hint_helper`).
    pub fn from_value(value: &Value, span: Span) -> Result<Self, Vec<SourceDiagnostic>>;

    /// Constrói a partir de um dict já validado (caminho do resolver da
    /// style chain — nunca falha; entradas inesperadas são ignoradas).
    pub fn from_validated_dict(dict: &IndexMap<EcoString, Value, FxBuildHasher>) -> Self;

    /// Fold da style chain (paridade `Fold for FontVariations`,
    /// `variations.rs:210-214`): `self` (interno) vence por tag;
    /// tags de `outer` ausentes em `self` sobrevivem.
    pub fn fold(&self, outer: &FontVariations) -> FontVariations;
}
```

## Semântica de validação (paridade vanilla, medida em P836)

Ordem das verificações por entrada `(key, value)` (índice `i` = posição
de iteração do dict):

1. **Tag com caracteres não ASCII-imprimíveis** (fora de `0x20..=0x7E`):
   `tag may contain only printable ASCII characters`
   + hint `found invalid cluster `"{cluster}"``
   (`tag.rs:91-97`). O cristalino itera `chars()` em vez de grapheme
   clusters (unicode-segmentation não está na whitelist L1) — diverge
   apenas em sequências combinantes, registado como limitação.
2. **Comprimento da tag** (bytes; ASCII garantido por 1): fora de
   `1..=4` → `tag must be one to four characters in length`
   + hint `found {n} characters` (`tag.rs:99-104`).
3. **Espaços**: espaço no índice 0 ou após o início do padding →
   `spaces may only appear as padding following a tag`, sem hint
   específico (`tag.rs:106-112`).
4. **Valor**: `Value::Int` → `f32`; `Value::Float` → `f32`; outro →
   `expected float, found {type}` com nomes de tipo vanilla
   (`int`→`integer`, `str`→`string`, `bool`→`boolean`).
5. **Valor não-dict** em `from_value`: `expected dictionary, found {type}`
   (sem hint de tag — medido: `#text(variations: 5)`).

Todos os erros de entrada (1-4) levam o hint final
`occurred in tag at index {i} (`"{key}"`)` **depois** do hint
específico. **Não há validação de faixa** (`wght: 99999` compila no
vanilla — medido em `temp/p836/e5_range.typ`, exit 0); o clamp é
feito pelo shaper/instancer, como no vanilla.

## Normalização

`from_value`/`from_validated_dict` devolvem a lista **ordenada por tag**
com dedup "later wins" (paridade `normalized()`, `variations.rs:196-208`).
Dicts da linguagem não têm chaves duplicadas (o eval rejeita), logo o
dedup só é exercido pelo `fold`.

## Invariantes

- Pureza L1: sem I/O, sem `ttf_parser`, sem unicode-segmentation.
- `PartialEq` estrutural — usado na chave de dedup de fontes do export
  (`(FontList, FontVariant, FontVariations)`).
