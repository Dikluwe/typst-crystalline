# Inventário — Sistema de estilos L1 (Passo 99.A)

Data: 2026-04-23.

---

## Parte 1 — Catálogo de variantes do enum `Style` (L1)

### Propriedades actualmente em uso

| Propriedade | Tipo | Origem no cristalino | Origem vanilla |
|-------------|------|----------------------|----------------|
| `Bold` | `bool` | `TextStyle.bold`, `StyleDelta.bold` | `TextElem::bold` |
| `Italic` | `bool` | `TextStyle.italic`, `StyleDelta.italic` | `TextElem::italic` |
| `Size` | `Pt` (wrapper sobre `f64`) | `TextStyle.size`, `StyleDelta.size` | `TextElem::size` |

### Propriedades adicionadas como superconjunto (forward-compat)

Adiadas do vanilla mas prontas para activação em passo futuro quando
os tipos existirem ou a semântica for implementada:

| Propriedade | Tipo | Notas |
|-------------|------|-------|
| `Fill` | `Color` | Cor de texto — `Color` já existe em `layout_types.rs`. |
| `HeadingLevel` | `u8` | Já representado no AST via `Content::Heading{level, ..}` — a variante aqui existe para que `#set heading(level: N)` seja exprimível via StyleChain no futuro. |

### Variantes adiadas (bloqueadas por tipos não materializados)

| Propriedade vanilla | Razão do adiamento |
|--------------------|---------------------|
| `text.font` | Requer `Font` real — stub em L1 |
| `text.lang`, `text.region` | Requer vocabulário de localização ainda não materializado |
| `par.leading`, `par.spacing` | Sistema de parágrafo com `Length` typed não entrou em L1 |
| `heading.numbering` com `Content` complexo | Tem-se `Content::SetHeadingNumbering` directo; sem necessidade imediata |
| Todos os derivados de `#[elem]` proc macro | Divergência ADR-0026 — enum linear em L1 |

Total de variantes no enum `Style` para Passo 99: **5** (Bold, Italic,
Size, Fill, HeadingLevel). Abaixo do limite de 30 da nota operacional.

---

## Parte 2 — Decisão sobre `TextStyle`

### Contagem empírica

```
$ grep -rn "TextStyle" 01_core/src/ 03_infra/src/ | wc -l
70
```

Distribuição (top files):

| Ficheiro | Ocorrências |
|----------|------------:|
| `01_core/src/rules/math/layout/mod.rs` | 9 |
| `01_core/src/entities/layout_types.rs` | 9 |
| `01_core/src/rules/layout/mod.rs` | 8 |
| `01_core/src/rules/math/layout/tests.rs` | 5 |
| `01_core/src/entities/style_chain.rs` | 5 |
| `01_core/src/rules/layout/tests.rs` | 4 |
| `01_core/src/entities/content.rs` | 4 |
| `01_core/src/rules/math/layout/root.rs` | 3 |
| `01_core/src/rules/math/layout/frac.rs` | 3 |
| `01_core/src/rules/math/layout/attach.rs` | 3 |

Sítios de **construção** (`TextStyle { bold: ..., italic: ..., size: ... }`
ou `TextStyle::regular/bold/italic`): ~15.

Sítios de **consumo** (`.bold`, `.italic`, `.size` de um `TextStyle`): ~55.

Testes que dependem da estrutura exacta: ~10.

### Critério da spec

> Se os sítios de consumo de `TextStyle` passam dos 15, preferir COEX.

55 ≫ 15. **Decisão: COEX (coexistência com ponte).**

### Estratégia COEX

- `TextStyle` permanece como "vista achatada para o Layouter actual"
  em `entities/layout_types.rs`.
- `From<&StyleChain> for TextStyle` (já existe desde Passo 22) faz a
  ponte.
- `Content::Styled` usa `Styles` (colecção de `Style`), não `TextStyle`.
- `StyleChain` evolui para suportar `Styles` via novos métodos; a API
  antiga (`bold()`, `italic()`, `size()`) permanece como accessors que
  fazem `lookup(Style::...)`.
- DEBT novo a abrir em 99.E: "substituir TextStyle por StyleChain no
  Layouter e em `FrameItem::Text`".

Minimiza o blast radius: o Passo 99 foca a fundação tipada
(`Style`/`Styles`/`StyleChain`/`Content::Styled`) sem forçar reescrita
de ~55 sítios de consumo ao mesmo tempo.

---

## Regra de resolução (topo vs raiz)

Lendo o código actual (`StyleChain::resolve_bool`):

```rust
let mut node = self.0.as_deref();
while let Some(n) = node {
    if let Some(v) = f(&n.delta) {
        return Some(v);   // primeiro delta que define ganha
    }
    node = n.parent.as_deref();
}
```

A resolução percorre do **topo para a raiz** — o nó mais recente (mais
próximo do texto quando inserido via `push`) ganha. Confere com o
vanilla (`StyleChain::get` também percorre `chain.link` descendente).

**Decisão**: manter a semântica actual — o delta mais próximo do texto
ganha. Já é paridade com o vanilla.

---

## Recomendação

1. **COEX** para TextStyle — pelo menos 55 sítios tornam SUB impraticável
   num único passo.
2. Enum `Style` com **5 variantes** (Bold, Italic, Size, Fill, HeadingLevel).
3. `Styles(Vec<Style>)` — `EcoVec` não está autorizado em L1 hoje; usar
   `Vec` e deixar optimização para passo futuro se necessária.
4. `StyleChain` evolui para usar `Arc<Styles>` como delta em vez de
   `StyleDelta` — mantém a API antiga via accessors.
5. `Content::Styled(Box<Content>, Styles)` — nova variante.
6. Regra de resolução: top-wins (já em vigor).

Próximo: criar ADR-0038.
