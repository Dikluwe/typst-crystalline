# Passo 100 — Relatório de encerramento (DEBT-48, forma de estilo no FrameItem)

**Data**: 2026-04-23
**Precondição**: Passo 99 encerrado; fundação `Style`/`Styles`/`StyleChain`/
`Content::Styled` em L1; 780 L1 + 174 L3; zero violations.
**ADR criada**: ADR-0039 "Forma de estilo no `FrameItem::Text`" —
**PROMOVIDA A EM VIGOR** em 100.E.

---

## Sumário

DEBT-48 encerrado. `Content::Styled` activado no Layouter via
push/pop sobre `StyleChain`. `TextStyle` redefinido como "vista
achatada de uma StyleChain" (SR — Struct Resolvido) e estendido com
`fill` + `heading_level` para paridade com o enum `Style` do
ADR-0038.

Zero regressão funcional: **780 → 783 L1** (+3 integração), 174 L3 +
6 ignorados inalterados. `crystalline-lint .` → zero violations.

---

## 100.A — Inventário + decisão SR/SO

Inventário em `00_nucleo/diagnosticos/inventario-textstyle-passo-100.md`.

**Contagem**:

| Campo lido | Ocorrências |
|-----------|------------:|
| `.size` | 55 |
| `.bold` | 4 |
| `.italic` | 3 |
| `.fill` | 0 |

55 ≫ 30 → **Decisão SR**. Resolver por read em 55 sítios adicionaria
custo sem ganho. `TextStyle` preservado como "struct resolvido" no
`FrameItem::Text`.

---

## 100.B — ADR-0039

Criada em `00_nucleo/adr/typst-adr-0039-frameitem-style.md` com
status `PROPOSTO`. **Promovida a EM VIGOR em 100.E**.

Conteúdo:

- Decisão SR registada com evidência empírica.
- `TextStyle` redefinido semanticamente (mesmo nome, significado
  muda: passa a ser o *resultado* de resolver uma `StyleChain`).
- Plano de activação: Layouter ganha `chain: StyleChain`;
  `Content::Styled` faz push/pop; `TextStyle::from(&chain)` é o
  ponto único de resolução.
- Sem lifetime contagion (StyleChain usa Arc).
- Relação com ADR-0016/0026/0033/0036/0037/0038 documentada.

---

## 100.C — Refactor

### C.1 — `TextStyle` estendido

Em `entities/layout_types.rs`:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct TextStyle {
    pub bold:          bool,
    pub italic:        bool,
    pub size:          Pt,
    pub fill:          Option<Color>,      // Passo 100 — forward-compat
    pub heading_level: Option<u8>,         // Passo 100 — forward-compat
}
```

`Pt` ganhou `Default` (retorna `Pt(0.0)`). Constructors
`TextStyle::regular/bold/italic` actualizados para usar
`..Self::default()`.

### C.2 — `From<&StyleChain>` estendido

```rust
impl From<&StyleChain> for TextStyle {
    fn from(chain: &StyleChain) -> Self {
        TextStyle {
            bold:          chain.bold(),
            italic:        chain.italic(),
            size:          Pt(chain.size()),
            fill:          chain.fill(),
            heading_level: chain.heading_level(),
        }
    }
}
```

Ponto único de resolução. Chamado pelo Layouter em cada push/pop
da cadeia.

### C.3 — Layouter com `chain: StyleChain`

Adicionado campo novo:

```rust
pub(super) style: TextStyle,   // cache da vista resolvida
pub(super) chain: StyleChain,  // source-of-truth
```

Inicializado em `Layouter::new` com `StyleChain::default_chain()`.

### C.4 — `Content::Styled` activo no Layouter

Substituiu o comportamento transparente do Passo 99:

```rust
Content::Styled(body, styles) => {
    let prev_chain = self.chain.clone();  // O(1) Arc::clone
    let prev_style = self.style;
    self.chain = self.chain.push_styles(styles);
    self.style = TextStyle::from(&self.chain);
    self.layout_content(body);
    self.chain = prev_chain;
    self.style = prev_style;
}
```

### C.5 — `Content::Text` arm: merge correcto

Problema descoberto no primeiro run dos testes: `Content::Text(s,
node_style)` ignorava `self.style` (cache da chain). Fix:

```rust
let effective = TextStyle {
    bold:   node_style.bold   || self.style.bold,
    italic: node_style.italic || self.style.italic,
    size:   if self.style.size > self.font_size_pt {
        self.style.size   // chain/heading sobrepõe base
    } else {
        node_style.size   // #set text(size:) capturado em eval
    },
    fill:          self.style.fill.or(node_style.fill),
    heading_level: self.style.heading_level.or(node_style.heading_level),
};
```

Regra: propriedades **activas** (bold/italic=true, size > base, fill
Some, heading_level Some) vindas da cadeia sobrepõem o `node_style`
do eval. Propriedades **passivas** herdam o node_style. Preserva a
semântica histórica (heading > base) + adiciona a nova (Styled
envolve Text).

### C.6 — Construtores legacy

Transformação Regex mecânica: `TextStyle { bold: .., italic: .., size: .. }`
→ `TextStyle { ..., ..TextStyle::default() }`. 3 construtores em
`layout/mod.rs` + 1 em `style_chain.rs` actualizados automaticamente.

---

## 100.D — Testes de integração

3 testes novos em `layout/tests.rs` (submódulo
`tests_styled_integration`):

### Teste 1 — aplicação básica

```rust
let styled = Content::Styled(
    Box::new(Content::text("hello")),
    Styles::from_iter([Style::Bold(true), Style::Size(Pt(18.0))]),
);
let doc = layout(&styled, CounterState::new());
// Verifica: FrameItem::Text.style.bold == true, .size == Pt(18.0).
```

### Teste 2 — aninhamento top-wins

```rust
let inner = Content::Styled([Italic(true)], Content::text("hi"));
let outer = Content::Styled([Bold(true), Italic(false)], inner);
// Verifica: bold=true (outer — inner não toca); italic=true (inner
// — mais próximo do texto, sobrepõe Italic(false) do outer).
```

### Teste 3 — save/restore

```rust
let seq = Sequence[ Styled([Bold(true)], "STYLED"), Space, "plain" ];
// Verifica: "STYLED" tem bold=true; "plain" tem bold=false (após
// save/restore, o estilo do chamador não é contaminado).
```

Todos passam.

---

## 100.E — Encerramento

### Verificação

```
$ cargo test --workspace | grep "test result"
test result: ok. 783 passed; 0 failed; 0 ignored ...
test result: ok. 174 passed; 0 failed; 6 ignored ...

$ crystalline-lint .
✓ No violations found

$ grep -rn "LazyHash" 01_core/src/
# 3 comentários (stubs); zero uso real → ADR-0016 preservada.
```

### DEBT / ADR

- **DEBT-48**: **ENCERRADO (Passo 100) ✓** — movido para Secção 2
  de DEBT.md com secção "Encerramento (Passo 100)" detalhando
  mudanças aplicadas.
- **DEBT-1**: mantido como **PARCIALMENTE RESOLVIDO**, mas com
  header `(estrutura paga em Passo 100)`. A dívida estrutural está
  paga; faltam apenas tarefas independentes (activação de `#set` no
  eval, remoção de wrappers Strong/Emph do layout quando eval os
  substituir, propriedades adicionais bloqueadas por tipos não
  materializados).
- **ADR-0039**: promovida de `PROPOSTO` para **EM VIGOR**.

---

## Números finais

| Métrica | Antes | Depois |
|---------|------:|-------:|
| L1 tests | 780 | **783** (+3) |
| L3 tests | 174 | 174 |
| Ignorados | 6 | 6 |
| Violations | 0 | 0 |
| Campos de `TextStyle` | 3 | **5** (+fill, +heading_level) |
| Campos de `EvalContext` | 4 | 4 (inalterado) |
| Variantes de `Content` | N+1 | N+1 (inalterado desde Passo 99) |
| ADRs activos | 38 | **39** (+0039) |

---

## Lições

1. **Content::Text arm merge** — regra explicitada: "propriedades
   activas (true/positivas) da chain sobrepõem node_style". Sem esta
   regra, o Layouter ignoraria o estilo de `Content::Styled` quando
   o body contivesse `Content::Text` (o caso mais comum). O teste
   de integração falhou primeiro e apanhou o bug logo.

2. **Cache + source-of-truth**: manter `self.style` como cache de
   `self.chain` (sincronizado em cada push/pop) evita 55 reads
   repetidos de `TextStyle::from(&chain)` no hot path.
   Sincronização é O(1) amortizado — só muda quando `Content::Styled`
   ou `Content::Strong/Emph/Heading` empurra/desempilha.

3. **Mudança mínima, nome preservado**: manter o nome `TextStyle`
   (em vez de renomear para `Resolved`) eliminou ripple em ~10 tests
   legacy. Semanticamente o struct passou a ser "vista resolvida",
   o documento da struct é o que comunica a mudança.

4. **`TextStyle::default()` + `Default` em `Pt`**: a adição de
   `#[derive(Default)]` em `TextStyle` e `Pt` tornou o spread
   `..TextStyle::default()` o mecanismo natural para migrar
   construtores legacy. A alternativa (usar literal completo com
   `fill: None, heading_level: None`) seria 2 fields extra por site.

5. **Activação incremental sem tocar o eval**: a validação de 100.D
   foi feita construindo `Content::Styled` manualmente e chamando
   `layout(...)`. O eval pode continuar sem produzir `Content::Styled`
   até passo dedicado; o pipeline Layouter→export já honra o
   contrato.

---

## Estado pós-Passo 100

### Fundação completa e funcional

- `Style` enum + `Styles` collection (Passo 99, ADR-0038).
- `StyleChain` em L1 com `push_styles(&Styles)`, `fill()`,
  `heading_level()` (Passo 99).
- `Content::Styled(Box<Content>, Styles)` variante (Passo 99).
- Layouter processa `Content::Styled` via push/pop na `chain`
  (Passo 100, ADR-0039).
- `TextStyle` estendido com `fill` + `heading_level` (Passo 100).
- `FrameItem::Text.style: TextStyle` — contrato estável para export.
- `From<&StyleChain> for TextStyle` — ponto único de resolução.

### Trabalho futuro

1. **Activar `#set`/`#show` no `eval_markup`**: produzir
   `Content::Styled` a partir de `#set text(bold: true)` e do
   corpo. O pipeline Layouter→export já aceita.
2. **Remover wrappers `Content::Strong/Emph` do layout**:
   substituir por `Content::Styled([Style::Bold(true)], ...)` no
   eval; o layout ficaria apenas com o arm `Styled` (sem arms
   dedicados para Strong/Emph). Optimização de coesão.
3. **Propriedades adicionais** (`text.font`, `text.lang`,
   `par.leading`): bloqueadas por tipos não materializados (ADR-0038).
4. **Materialização de `Engine<'a>`**: agregador dos 9 parâmetros
   das funções `eval_*`. `StyleChain` já está preparado para entrar
   naturalmente.
