# Inventário `TextStyle` — Passo 100.A

Data: 2026-04-23.

---

## Parte 1 — Contagem empírica

Grep em `01_core/src/` + `03_infra/src/` por `TextStyle`:

| Categoria | Contagem |
|-----------|---------:|
| Leituras (`.bold`/`.italic`/`.size`/`.fill`) | **55** sítios totais |
| Construtores (`TextStyle { ... }` ou `TextStyle::regular/bold/italic`) | **31** |
| Total de ocorrências de `TextStyle` no código | ~70 |

Distribuição por campo lido:

| Campo | Ocorrências |
|-------|------------:|
| `.size` | **55** |
| `.bold` | 4 |
| `.italic` | 3 |
| `.fill` | 0 (ainda não consumido) |

`.size` domina massivamente — é lida em todos os `self.metrics.advance(text, self.style.size)`
e `self.metrics.vertical_metrics(self.style.size)` do Layouter e do
math/layout. Cada palavra e cada nó matemático consulta `.size`.

## Parte 2 — Pipeline do Layouter

- `Layouter.style: TextStyle` — campo de estado actual.
- Push/pop via save/restore em `Content::Strong`, `Content::Emph`,
  `Content::Heading`, `Content::Text` (captura node_style), corpo de
  closure.
- Emitido em `FrameItem::Text { style: self.style, .. }` (text/layout_word/
  math) — **ponto de resolução implícito**.

Todos os métodos que hoje recebem/leem `TextStyle` poderiam aceitar
`&StyleChain` e resolver no fim. Mas o número de reads de `.size`
(55) torna a resolução em cada read custosa sem cache.

## Parte 3 — Decisão SR / SO

Gate da spec:

> Se >30 sítios de leitura (L) directa de campos e o export resolve
> cedo, preferir **SR**.

Satisfeito: 55 > 30. **Decisão: SR (Struct Resolvido)**.

### Forma do struct resolvido

Manter o nome `TextStyle` (menor ripple em testes legacy) e **estender**
com os campos forward-compat que a ADR-0038 introduziu:

```rust
pub struct TextStyle {
    pub bold:          bool,
    pub italic:        bool,
    pub size:          Pt,
    pub fill:          Option<Color>,     // novo
    pub heading_level: Option<u8>,        // novo
}
```

Semântica: `TextStyle` passa a ser **o resultado de resolver uma
`StyleChain`**, não uma cadeia em si. O `From<&StyleChain> for
TextStyle` existente (Passo 22) é o **ponto único de resolução**.

### Plano de activação

1. Estender `TextStyle` com `fill` e `heading_level` — todos os
   construtores literais existentes ganham `..TextStyle::default()`.
2. Substituir `self.style: TextStyle` por `self.chain: StyleChain` +
   cache lazy ou resolver-ao-emitir.
3. `Content::Styled` activo: `push/pop` sobre `self.chain`.
4. `FrameItem::Text.style` continua `TextStyle` (nome + localização
   inalterados). A única mudança para consumers é que o struct ganha
   campos adicionais.
5. Tests legacy que constroem `TextStyle { bold, italic, size }`
   ganham `..TextStyle::default()` — automático via script regex.

Zero lifetime contagion: `StyleChain` usa `Arc` interno desde o Passo 22.
`Layouter` permanece sem parâmetro de lifetime.

### Catálogo dos campos de `TextStyle` (após Passo 100)

| Campo | Tipo | Defaultt | Origem `Style` |
|-------|------|----------|----------------|
| `bold` | `bool` | `false` | `Style::Bold(bool)` |
| `italic` | `bool` | `false` | `Style::Italic(bool)` |
| `size` | `Pt` | `Pt(11.0)` | `Style::Size(Pt)` |
| `fill` | `Option<Color>` | `None` | `Style::Fill(Color)` |
| `heading_level` | `Option<u8>` | `None` | `Style::HeadingLevel(u8)` |

---

## Recomendação

1. **SR** confirmado (55 > 30 reads).
2. Manter nome `TextStyle` — evita ripple em tests legacy.
3. Estender com `fill` + `heading_level` para alinhar com `Style` enum.
4. Layouter: substituir `self.style` por `self.chain` (SO interno).
5. `FrameItem::Text.style: TextStyle` — resolvido no emit point.
6. `From<&StyleChain>` continua a ser o único ponto de resolução.
7. `TextStyle { bold, italic, size }` construtors em tests legacy
   ganham `..TextStyle::default()` via transformação Regex.

Próximo: ADR-0039.
