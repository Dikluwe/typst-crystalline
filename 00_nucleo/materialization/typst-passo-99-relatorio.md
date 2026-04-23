# Passo 99 — Relatório de encerramento (Fundação `Style`/`Styles`/`StyleChain` em L1)

**Data**: 2026-04-23
**Precondição**: Passo 98 encerrado; `EvalContext` com 4 campos Regra 4;
764 L1 + 174 L3 + 6 ignorados; zero violations.
**ADRs aplicáveis**: ADR-0016 (LazyHash fora de L1 — não revogada),
ADR-0026 (divergência por enum linear), ADR-0033 (paridade),
ADR-0036 (atomização), ADR-0037 (coesão).
**ADR criada**: ADR-0038 "Sistema de estilos em L1" —
**PROMOVIDA A EM VIGOR** em 99.E.

---

## Sumário

Fundação tipada do sistema de estilos materializada em L1:

- Enum `Style` com **5 variantes** (Bold, Italic, Size, Fill, HeadingLevel).
- Struct `Styles(Vec<Style>)` — colecção de deltas tipados.
- `StyleChain::push_styles(&Styles)` — projecção tipada.
- `Content::Styled(Box<Content>, Styles)` — variante nova.
- Integração testada com teste conceptual de resolução top-wins.

Zero regressão: **764 → 780 L1 tests** (+16 novos), 174 L3 + 6 ignorados
inalterados. `crystalline-lint .` → zero violations.

ADR-0038 criada e promovida a **EM VIGOR**. DEBT-1 actualizado para
PARCIALMENTE RESOLVIDO (Passo 99). DEBT-48 aberto (substituir
`TextStyle` plano por `StyleChain` — consequência directa da decisão
COEX).

---

## 99.A — Decisão SUB/COEX

Inventário em `00_nucleo/diagnosticos/inventario-style-passo-99.md`.

**Contagem**: 70 sítios de `TextStyle` em `01_core/src/` + `03_infra/src/`
(~55 consumo, ~15 construção, ~10 testes).

**Critério da spec**: ≥ 15 consumo → COEX.

**Decisão**: **COEX** (55 ≫ 15). `TextStyle` permanece como vista
achatada; `From<&StyleChain>` já existe como ponte desde Passo 22.
`Content::Styled` usa `Styles`.

---

## 99.B — ADR-0038

Criada em `00_nucleo/adr/typst-adr-0038-sistema-estilos-l1.md` com
status inicial `PROPOSTO`. Promovida a **EM VIGOR** em 99.E após
validação empírica (780 tests, zero violations).

Conteúdo:

- Mapa de camadas: `Style`/`Styles`/`StyleChain` em L1; `LazyHash<T>`
  em L3 quando pipeline incremental real for activado (ADR-0016
  preservada).
- Divergência do vanilla: enum linear em vez de proc macros
  `#[elem]` (ADR-0026 como precedente).
- Regra de resolução: **top-wins** (paridade com vanilla, ADR-0033).
- Decisão SUB vs COEX: COEX registada.
- Variantes adiadas: `text.font`, `text.lang`, `par.leading`, etc.
- O que o ADR **não** decide: activação de `#set`/`#show` no eval,
  quando `LazyHash` vai para L3, quando Font real entra em L1,
  substituição de `TextStyle` em `FrameItem::Text`.

---

## 99.C.1 — `Style` enum + `Styles`

Ficheiro novo: `01_core/src/entities/style.rs`. 5 variantes:

```rust
pub enum Style {
    Bold(bool),
    Italic(bool),
    Size(Pt),
    Fill(Color),         // forward-compat
    HeadingLevel(u8),    // forward-compat
}

pub struct Styles { inner: Vec<Style> }
```

Métodos: `Styles::new()`, `push()`, `iter()`, `is_empty()`, `len()`,
`from_iter()`.

Prompt L0 criado em `00_nucleo/prompts/entities/style.md` com hash
`37404a23`. Header `@prompt-hash` alinhado.

**Testes novos**: 5 (`styles_new_vazio`, `styles_push_e_iter`,
`styles_from_iter`, `styles_eq`, `style_variantes_cobrem_catalog_99a`).

---

## 99.C.2 — `StyleChain` usa `Styles`

`StyleDelta` estendido com `fill: Option<Color>` e `heading_level:
Option<u8>` (forward-compat).

`StyleChain::push_styles(&Styles)` adicionado — projecta cada variante
de `Style` no campo correspondente do `StyleDelta`. Match exaustivo
— adicionar uma nova variante a `Style` força actualização deste método.

Novos accessors: `fill()`, `heading_level()`.

API antiga (`bold()`, `italic()`, `size()`, `push()`) preservada —
backward-compat.

**Testes novos**: 6 (`push_styles_projecta_bold_italic_size`,
`push_styles_herda_propriedade_nao_definida`,
`push_styles_topo_ganha_sobre_base`, `fill_forward_compat`,
`heading_level_forward_compat`,
`chain_aninhada_fill_heading_level_top_wins`).

---

## 99.C.3 — `Content::Styled(Box<Content>, Styles)`

Variante nova adicionada ao enum `Content`. Coberta em todos os
`match` exaustivos:

- `plain_text()` — transparente (devolve `body.plain_text()`).
- `is_empty()` — default `false` (um bloco estilizado não é "vazio").
- `map_text()` — propaga transformação; preserva estilos.
- `map_content()` — propaga `transform`; preserva estilos.
- `PartialEq` — compara body + styles.
- `introspect::materialize_time` e `introspect::walk` — transparente.
- `layout::layout_content` — transparente (ignora styles; bridge COEX
  via `TextStyle` continua a funcionar). Activação completa fica para
  passo dedicado.

**Testes novos**: 3 (`styled_plain_text_transparente`,
`styled_partial_eq`, `styled_preserva_estilos_em_map_text`).

---

## 99.D — Teste de integração conceptual

Em `style_chain.rs` — **sem activar `#set` no eval**:

### Teste 1: `integracao_content_styled_resolve_via_style_chain`

```rust
let body   = Content::text("hello");
let styles = Styles::from_iter([Style::Bold(true), Style::Size(Pt(18.0))]);
let styled = Content::Styled(Box::new(body), styles);

let chain = match &styled {
    Content::Styled(_body, ss) =>
        StyleChain::default_chain().push_styles(ss),
    _ => panic!(),
};

assert!(chain.bold());
assert_eq!(chain.size(), 18.0);
```

Demonstra que a fundação é **usável** — o consumidor futuro (eval)
terá apenas de fazer os passos 1–3 simulados manualmente aqui.

### Teste 2: `integracao_styled_aninhado_top_wins`

Styled dentro de Styled — o delta mais próximo do texto (inner)
sobrepõe o outer. Paridade directa com o vanilla (ADR-0033).

```rust
// outer: Bold(true), Italic(false)
// inner: Italic(true)
// Resolução: bold=true (de outer), italic=true (de inner — top-wins)
```

Ambos passam.

---

## 99.E — Encerramento

### Verificação

```
$ cargo test --workspace | grep "test result"
test result: ok. 780 passed; 0 failed; 0 ignored ...
test result: ok. 174 passed; 0 failed; 6 ignored ...

$ crystalline-lint .
✓ No violations found

$ grep -rn "LazyHash" 01_core/src/
# Só aparece em 3 comentários (stubs); zero uso real → ADR-0016 preservada.
```

### DEBT / ADR

- **DEBT-1 (StyleChain)**: actualizado para **PARCIALMENTE RESOLVIDO
  (Passo 99)** com secção nova listando o que o Passo 99 pagou.
- **DEBT-48** aberto — "Substituir TextStyle plano por StyleChain no
  Layouter e export". Escopo claro; depende da fundação do Passo 99.
- **ADR-0038** promovida de `PROPOSTO` para **EM VIGOR** — validada
  empiricamente.

---

## Números finais

| Métrica | Antes | Depois |
|---------|------:|-------:|
| L1 tests | 764 | **780** (+16) |
| L3 tests | 174 | 174 |
| Ignorados | 6 | 6 |
| Violations | 0 | 0 |
| Campos de `EvalContext` | 4 | 4 (inalterado) |
| Variantes de `Content` | N | N+1 (`Styled`) |
| Variantes de `Style` | — | **5** |

**Crescimento dos testes**: +16 em L1 (5 em `style.rs`, 6 em
`style_chain.rs` para novas APIs, 2 integração conceptual, 3 em
`content.rs` para `Content::Styled`).

---

## Lições

1. **Fundação antes de consumo**: materializar `Style`/`Styles`/`StyleChain`
   + `Content::Styled` sem activar no eval provou ser gestão de risco
   correcta. O teste de integração conceptual valida a API sem obrigar
   a escrever o pipeline completo de `#set`/`#show`.

2. **COEX como estratégia conservadora**: quando a substituição teria
   blast radius de 70 sítios, a coexistência com ponte (`From<&StyleChain>
   for TextStyle` já existente) permite que o próprio ADR-0038 seja
   um passo pequeno e autocontido. O DEBT-48 tem escopo claro para o
   dia em que fizer sentido fechar.

3. **Regra 6 ADR-0037 reforçada**: o enum linear `Style` (em vez de
   proc macros `#[elem]`) repete o padrão do enum `Content` (ADR-0026).
   O vocabulário tipado em L1 sem dependência de macros custom é a
   assinatura arquitectural do cristalino.

4. **Prompt L0 + hash discipline**: criar o prompt L0 antes do código
   e alinhar o hash é workflow que escala. A dupla linha-de-base
   "código ↔ prompt" fica imutável depois do `crystalline-lint` verde.

---

## Estado pós-Passo 99

Próximos candidatos (registados no fluxo, não decididos aqui):

1. **Activar `#set`/`#show` no eval** a consumir `Content::Styled`
   (ADR-0038 já em vigor como base).
2. **DEBT-48**: substituir `TextStyle` por `StyleChain` em
   `FrameItem::Text` + export — quando for rentável.
3. **Materialização de `Engine<'a>`**: agregador dos 9 parâmetros do
   eval (evidência empírica do Passo 98). `StyleChain` já atomizado
   como parâmetro; natural de agrupar.
4. **`LazyHash` em L3**: quando pipeline incremental real for
   activado. ADR-0016 preservada até lá.
