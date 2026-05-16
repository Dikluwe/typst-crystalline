# Prompt L0 — StyleChain
Hash do Código: ee1bb41e

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

---

## Anotação cumulativa P266 — Cobertura empírica StyleChain confirmada (Fase A)

**Data**: 2026-05-15.

P266 audit Fase A confirmou estado real cumulativo `StyleDelta`
+ resolvers + consumers. Anotações "inerte" originais em
linhas 25-27 são **factualmente desactualizadas** — os campos
têm consumers reais materializados em passos subsequentes:

### Promoções implementado → implementado⁺ confirmadas

| Campo | Consumer real | Passo | Status |
|-------|---------------|-------|--------|
| `tracking: Option<Length>` | PDF `Tc` operator + Cursor advance | P137 | implementado⁺ |
| `leading: Option<Length>` | `line_height = default + leading` em cursor | P128 | implementado⁺ |
| `lang: Option<Lang>` | Hyphenation hypher + smart-quotes localize | P144 + P155 | implementado⁺ |
| `weight: Option<u16>` | Faux-bold `faux_bold_stroke_pt` + PDF `2 Tr` | P139 | implementado⁺ |

### Estado actual 10 fields StyleDelta (confirmado P266.A)

- 10 fields (não 12 originalmente esperados).
- 10 resolvers em `impl StyleChain` (bold/italic/size/fill/
  heading_level/weight/tracking/leading/lang/font).
- `impl From<&StyleChain> for TextStyle` materializado (linha
  272+).

### Cobertura Text agregada empírica P266

- 12/12 entradas StyleChain (A.1-A.12) fechadas (implementado
  ou implementado⁺).
- Cobertura StyleChain subset = **100%** estrutural.
- StyleChain subsistema A: 4/12 promoções implementado⁺
  (A.8 tracking + A.9 leading + A.12 lang + via E.1 faux-bold
  consumer).

### Cross-references

- ADR-0038 — Content::Styled (consumer StyleChain via
  `push_styles`).
- ADR-0039 — TextStyle SR (bridge preservado).
- ADR-0052 — Lang tipo (consumer StyleDelta.lang).
- ADR-0053 — FontList (consumer StyleDelta.font; covers
  inabitado).
- ADR-0054 — Perfil graded (cobertura subsystem).
- ADR-0080 — L0 minimal para refactors aditivos (esta anotação
  preserva representação base).
- ADR-0084 + 0085 — Auditoria condicional + diagnóstico
  imutável (primeiro consumo directo P266).
- `00_nucleo/diagnosticos/diagnostico-text-fase-a-passo-266.md`
  — diagnóstico imutável Fase A (cobertura agregada ~86%).
