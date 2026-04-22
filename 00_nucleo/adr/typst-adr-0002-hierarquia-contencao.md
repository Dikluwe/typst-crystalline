# ⚖️ ADR-0002: Hierarquia de Contenção como Mecanismo de Layout

**Status**: `IDEIA`
**Nota**: direcção registada para consideração futura; não
materializar em L1 sem reavaliação prévia.
**Data**: 2026-03-22
**Contexto**: migração Typst → Arquitetura Cristalina

---

## Ideia central

O layout tipográfico é tratado hoje (TeX, Typst, InDesign) como uma
sequência plana de elementos que passam por um pipeline. A hierarquia
do documento existe implicitamente mas não é usada como mecanismo de
controle de escopo de optimização.

A proposta inverte isso: **a hierarquia é o mecanismo primário**.
Cada nível da hierarquia declara explicitamente o escopo de
interacções não-locais que pode realizar. O algoritmo de optimização
nunca precisa de explorar além do escopo declarado.

---

## Hierarquia de níveis proposta

```
DocumentLevel      — counters, referências cruzadas, TOC
└── SectionLevel   — numeração, headers
    └── PageLevel  — widow/orphan control, page floats
        └── ColumnLevel — balanceamento de colunas
            └── ParagraphLevel — quebra de linha (Knuth-Plass)
                └── LineLevel  — kerning, ligatures
                    └── WordLevel — hifenização
                        └── GlyphLevel — posicionamento sub-pixel
```

Cada nível tem:
- O escopo exacto do que pode tocar (declarado, não inferido)
- A permissão explícita de interacções não-locais
- Um algoritmo de optimização adequado à sua granularidade

---

## Modelo de tipos (esboço)

```rust
pub struct LayoutLevel {
    permitted_interactions: InteractionScope,
    children: Vec<LayoutLevel>,
}

pub enum InteractionScope {
    /// Nunca afecta vizinhos — glifo, letra
    SelfContained,
    /// Palavras negociam espaçamento entre si
    SiblingNegotiation,
    /// Float sobe N níveis no máximo
    ParentFloat { max_levels_up: u8 },
    /// Widow/orphan control com escopo declarado
    WidowControl {
        min_lines_top: u8,
        min_lines_bottom: u8,
        renegotiation_scope: RenegotiationScope,
    },
    /// Referências cruzadas, counters — escopo de documento
    DocumentWide,
}

pub enum RenegotiationScope {
    PreviousParagraph,
    PreviousN(u8),
    Section,
}
```

---

## O ganho combinatório

Para um documento com 100 parágrafos e 2 floats possíveis por parágrafo:

| Abordagem | Espaço de busca |
|-----------|-----------------|
| Global (TeX/Typst hoje) | `2^200` |
| Contenção no parágrafo | `100 × 2^2 = 400` |
| Contenção na frase | ainda menor |

A redução não é heurística — é estrutural. Não se está a aproximar
o óptimo global; está-se a redefinir o que "óptimo" significa dentro
de um escopo declarado.

---

## Widow control como nível explícito

Hoje, widow control no TeX é `\widowpenalty` — um parâmetro global
que afecta o algoritmo de quebra de página inteiro. Com a hierarquia
explícita, widow control é uma permissão de interacção declarada no
`PageLevel`:

```rust
pub enum PageLevelPolicy {
    WidowControl {
        min_lines_top: u8,
        min_lines_bottom: u8,
        renegotiation_scope: RenegotiationScope,
    }
}
```

Isso permite widow control diferente em secções diferentes do
documento — impossível em TeX sem hackery.

---

## Propriedade emergente

O autor do documento controla a combinatória implicitamente pela
estrutura que declara:

- Documento sem `ColumnLevel` nunca executa balanceamento de colunas
- Documento sem floats em `ParagraphLevel` nunca executa float placement
- A complexidade computacional do layout é determinada pela hierarquia
  declarada — não pelo engine tentando detectar o que é necessário

---

## Como se encaixa na Arquitetura Cristalina

A hierarquia de níveis é a Arquitetura Cristalina aplicada ao espaço
do problema de layout, não apenas ao código:

- Cada nível da hierarquia é uma camada com contratos declarados
- Interacções entre níveis são explícitas na assinatura — nunca implícitas
- O escopo de optimização é determinado pela hierarquia, não pelo algoritmo

Potencialmente verificável pelo `crystalline-lint` se o Typst
cristalino adoptar a hierarquia formalmente — um nível que viola
o seu `InteractionScope` declarado seria uma violation arquitectural.

---

## O que esta arquitectura habilitaria

| Sistema | Qualidade tipográfica | Extensibilidade | Performance incremental |
|---------|-----------------------|-----------------|------------------------|
| TeX | Alta (algoritmos ótimos) | Baixa | Baixa (sem incrementalidade) |
| InDesign | Alta (heurísticas maduras) | Zero | Baixa |
| Typst actual | Média (algoritmos imaturos) | Alta | Alta (comemo) |
| Typst cristalino com hierarquia | Alta | Alta | Alta |

---

## Relação com comemo (ADR-0003)

A hierarquia de contenção e `comemo` são complementares, não
alternativos. Ver ADR-0003.

---

## Estado: IDEIA

Esta ADR não tem plano de implementação. Regista o raciocínio
para quando a migração estiver estável e o pipeline cristalino
funcional. A implementação começa pelo `ParagraphLevel` —
o nível onde a diferença face ao Typst actual é mais mensurável.

---

## Referências

- Knuth & Plass (1981) — Breaking Paragraphs into Lines
- TeX: `\widowpenalty`, `\clubpenalty` — parâmetros globais como contra-exemplo
- Typst actual: `typst-layout` em `lab/typst-original/crates/`
- ADR-0001 — estratégia de migração e pipeline cristalino
- ADR-0003 — relação com comemo
