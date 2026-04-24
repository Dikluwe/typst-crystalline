# Prompt L0 — StyleChain
Hash do Código: bda71ca6

## Módulo
`01_core/src/entities/style_chain.rs`

## Propósito
`StyleChain` é uma lista ligada imutável de deltas de estilo.
Permite que `#set text(bold: true)` afecte apenas o conteúdo subsequente,
com herança: um nó filho herda todas as propriedades não definidas do pai.

## Motivação (DEBT-1)
`TextStyle { bold, italic, size }` era uma struct plana que não suportava
`#set` rules. `StyleChain` substitui essa representação.

## Representação

```rust
pub struct StyleDelta {
    pub bold:   Option<bool>,
    pub italic: Option<bool>,
    pub size:   Option<f64>,       // pontos tipográficos
    pub fill:   Option<Color>,     // Passo 99 (ADR-0038)
    pub heading_level: Option<u8>, // Passo 99 (ADR-0038) forward-compat
    pub weight:   Option<u16>,     // Passo 126 — inerte
    pub tracking: Option<Length>,  // Passo 127 — inerte; preserva abs+em
    pub leading:  Option<Length>,  // Passo 128 — capturado em text; migra p/ par
    pub lang:     Option<Lang>,    // Passo 131B (ADR-0052) — tipo semântico validado
    pub font:     Option<FontList>,// Passo 132B (ADR-0053) — tipo agregador; covers deferido
}

pub struct StyleChain(Option<Arc<StyleNode>>);
```

`StyleDelta::empty()` — nenhuma propriedade definida.
`StyleChain::default_chain()` — bold=false, italic=false, size=11.0pt.
`StyleChain::push(delta)` — nova cadeia herdando desta, com delta por cima. O(1).

## Resolução de propriedades

Percorre a cadeia do topo para a raiz, retorna o primeiro valor encontrado.
Se nenhum nó define a propriedade, usa o valor por defeito do accessor.

## Bridge para layout/export

`impl From<&StyleChain> for TextStyle` — converte para `TextStyle` plano,
compatível com o layout e export actuais durante a migração.

## Camada
L1 — pura. Sem I/O de sistema. Usa apenas `Arc` (RAM).

## Critérios de Verificação

- `StyleChain::default_chain()` retorna bold=false, italic=false, size=11.0
- `push(StyleDelta { bold: Some(true), .. })` propaga bold para filhos
- Herança: filho com `bold: None` herda bold do pai
- Clone de `StyleChain` é O(1) (só clona o Arc do topo)
- `From<&StyleChain> for TextStyle` converte correctamente
