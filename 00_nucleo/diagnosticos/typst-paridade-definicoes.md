# Paridade — definições operacionais

**Status**: `PROPOSTO`
**Data**: 2026-04-24

> **Revisto no Passo 147 (2026-04-24)**: o documento original
> foi escrito durante a fase precoce do projecto. Decisões
> conceptuais (4 níveis P1–P4, modos `text_content` /
> `structural` / `geometric`, tolerâncias configuráveis,
> convenções de DTOs/nomenclatura) **permanecem válidas** e
> são input directo para o Passo 148 (materialização). As
> assinaturas Rust descritas (ex: `typst_core::rules::parse::parse`,
> `typst_core::rules::eval::eval`) reflectem APIs reais do
> projecto pós-146. Status canonizado para `**Status**:
> \`PROPOSTO\`` (P84.8g + P145).

**Companheiro de** `typst-paridade-plano-medicao.md`.
**Propósito**: definir com precisão o que conta como "passa" em cada nível, para que dois leitores cheguem à mesma conclusão sobre o mesmo input.

---

## P1 — Parse

### Inputs

Qualquer string ou ficheiro `.typ` no corpus.

### Função sob teste

```rust
typst_core::rules::parse::parse(input: &str) -> SyntaxNode
typst_core::rules::parse::parse_math(input: &str) -> SyntaxNode
typst_core::rules::parse::parse_code(input: &str) -> SyntaxNode
```

### Oráculo

```rust
typst_syntax::parse(input: &str) -> SyntaxNode
typst_syntax::parse_math(input: &str)
typst_syntax::parse_code(input: &str)
```

### Comparação

`compact_cristalino(crist) == compact_original(orig)` onde `CompactNode` é definido em `lab/parity/src/compact.rs`. Spans são removidos estruturalmente, não por filtro de texto.

### Critério de "passa"

Igualdade exacta. Não há tolerância em P1.

### Critério de "falha aceitável"

Nenhum. Divergência em P1 é defeito a corrigir antes de avançar para P2 do mesmo input.

---

## P2 — Eval

### Inputs

Ficheiro `.typ` que termina com `#let __resultado__ = <expr>` ou similar. O nome `__resultado__` é convenção do harness — qualquer ficheiro do corpus semantic/ deve declarar esta variável.

### Função sob teste

```rust
typst_core::rules::eval::eval(world, source) -> SourceResult<Module>
// e depois:
module.scope().get("__resultado__")
```

### Oráculo

```rust
typst::eval::eval(...)  // ou equivalente do original
// e depois:
module.scope().get("__resultado__")
```

### Comparação

`ValueDTO::from(crist) == ValueDTO::from(orig)`.

`ValueDTO` é o tipo neutro de comparação. Tem de ser definido em `lab/parity/src/value_dto.rs` com conversões `From<&typst_core::Value>` e `From<&typst::Value>`.

### Critério de "passa"

Igualdade de `ValueDTO`. Detalhes específicos:

| Tipo | Critério |
|------|----------|
| `Bool`, `Int`, `Str` | igualdade exacta |
| `Float` | bits idênticos (NaN inclusive — verificar comportamento Typst) |
| `Array` | mesmo comprimento, elementos iguais por posição |
| `Dict` | mesmas chaves, valores iguais; ordem **importa** se Typst preservar ordem de inserção (verificar) |
| `Func` | igualdade por nome ou hash; closures não comparam |
| `Content` | recurse para `ContentDTO` (definido em P3) |
| `Type` | nome do tipo igual |

### Critério de "falha aceitável"

- Função capturada (`Func`) só compara por nome — closures distintas com mesmo comportamento são consideradas iguais se o nome bater. É uma simplificação intencional.
- Erros: se ambos os lados retornam `Err`, comparam-se as mensagens? **Não** — comparam-se os spans (linha/coluna) onde o erro ocorreu. Mensagens são strings localizadas e podem divergir cosmeticamente.

---

## P3 — Layout

### Inputs

Ficheiro `.typ` qualquer.

### Função sob teste

```rust
typst_core::rules::layout::layout(content) -> PagedDocument
```

### Oráculo

```rust
typst::layout::layout(content) -> PagedDocument
// ou função equivalente do original
```

### Comparação

Em três modos, do mais permissivo para o mais estrito:

#### Modo `text_content`

Extrai o texto de cada `Frame` em ordem (concatenando `FrameItem::Text { text, ... }`). Compara strings.

```rust
fn extract_text(doc: &PagedDocument) -> Vec<String> {
    doc.pages.iter().map(|page| {
        page.items.iter().filter_map(|item| match item {
            FrameItem::Text { text, .. } => Some(text.clone()),
            _ => None,
        }).collect::<Vec<_>>().join("")
    }).collect()
}

assert_eq!(extract_text(&crist), extract_text(&orig));
```

Critério: igualdade exacta dos vectores de texto por página.

#### Modo `structural`

Mesmo número de páginas. Mesmo número de itens por página. Mesmo tipo de cada item (`Text` vs `Group` vs `Shape`). Posições não são comparadas.

#### Modo `geometric`

Tudo o de `structural` mais: cada `pos` tem `|crist.x - orig.x| <= absolute_pt` e `|crist.y - orig.y| <= absolute_pt`.

Tolerância sugerida inicial: `absolute_pt = 5.0`. Ajustar empiricamente.

### Critério de "passa"

Configurável por ficheiro. Os ficheiros do corpus declaram o modo esperado nos seus metadados:

```toml
# corpus/visual/heading.typ.toml
modo_p3 = "text_content"   # geometria não é importante ainda
```

### Critério de "falha aceitável"

- Modo `text_content`: nenhum.
- Modo `structural`: itens decorativos do original (linhas, separadores) que o cristalino ainda não emite são ignorados se o ficheiro o declarar nos metadados.
- Modo `geometric`: divergência dentro de tolerância configurada.

---

## P4 — Export

### Inputs

Ficheiro `.typ` qualquer no corpus visual/.

### Função sob teste

```rust
typst_infra::export::export_pdf(doc) -> Vec<u8>
```

### Oráculo

```rust
typst::export::pdf(doc) -> Vec<u8>  // ou equivalente
```

### Comparação — Opção A (visual)

1. Renderizar cristalino para PNG: `pdftoppm -png -r 150 crist.pdf out_crist`
2. Renderizar original para PNG: `pdftoppm -png -r 150 orig.pdf out_orig`
3. Comparar pixel a pixel cada página correspondente

```rust
pub struct VisualTolerance {
    pub max_pixel_diff: u8,    // 0..=255
    pub max_diff_ratio: f64,   // 0.0..=1.0
}

fn compare_png(a: &Path, b: &Path, t: VisualTolerance) -> bool {
    // Carregar ambas as imagens com `image` crate
    // Para cada pixel: |a.r - b.r| + |a.g - b.g| + |a.b - b.b| <= t.max_pixel_diff
    // Conta pixels divergentes
    // ratio = divergentes / total <= t.max_diff_ratio
}
```

Tolerância sugerida inicial: `max_pixel_diff = 8`, `max_diff_ratio = 0.05` (5% dos pixels podem divergir até 8 unidades).

### Comparação — Opção B (textual)

Extrai operações de texto do PDF (substrings que comecem em `BT` e terminem em `ET`) ignorando metadados, IDs de objectos, e timestamps. Compara as listas de operações.

Esta opção é menos precisa mas não requer ferramentas externas.

### Critério de "passa"

- Opção A: dentro da tolerância visual.
- Opção B: lista de operações de texto idêntica (após normalização de coordenadas com tolerância numérica).

### Critério de "falha aceitável"

- Diferenças de hinting de fontes: aceites em A se dentro da tolerância de pixel.
- Ordem de objectos no PDF: irrelevante em ambas as opções (os PDFs nunca são byte-comparados).

---

## Convenções de relatório

Cada execução do harness produz um ficheiro `lab/parity/reports/latest.md` com a matriz da secção 6 do plano. Antes de o sobrescrever, é copiado para `lab/parity/reports/history/<data>-<passo>.md`.

O formato é Markdown puro para ser legível em qualquer editor e versionável no Git.

---

## Convenções de nomenclatura

- Ficheiros do corpus: `kebab-case.typ`
- Metadados: ficheiro `.typ.toml` adjacente
- Variável de resultado em P2: `__resultado__` (com dois underscores em cada lado)
- DTOs: sufixo `DTO` (ex: `ValueDTO`, `ContentDTO`, `FrameDTO`)
- Modos de comparação: `snake_case` (ex: `text_content`, `structural`, `geometric`)

---

## O que este documento não cobre

- Como executar a suite em CI — depende da decisão sobre ferramentas externas em P4
- Como reagir a regressões — política de "passo X regrediu de 80% para 75%, parar?" é decisão humana, não automática
- Performance: paridade não é benchmark; medição de tempo é assunto separado
