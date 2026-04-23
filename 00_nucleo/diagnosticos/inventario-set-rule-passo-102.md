# Inventário `#set` — Passo 102.A

Data: 2026-04-23.

---

## Parte 1 — Estado actual de `eval_set_rule`

### Arquitectura

`#set` **já está activo** no cristalino desde **Passo 30**. O
`eval_set_rule` está em `01_core/src/rules/eval/rules.rs:179`; é
invocado pelo dispatcher `eval_expr` no braço `Expr::SetRule(s)`
(`eval/mod.rs:360`).

### Targets suportados

| Target | Acção | Passo origem |
|--------|-------|--------------|
| `text` | Empilha `StyleDelta { bold?, italic?, size? }` em `*styles` | 30 |
| `heading` | Emite `Content::SetHeadingNumbering { active }` | 57 |
| `page` | Emite `Content::SetPage { width, height, margin }` | 81 |
| `figure` | Muta `*figure_numbering` e emite `Content::SetFigureNumbering { pattern }` | 75 (Passo 98 parametrizou) |

Outros targets (`par`, etc.) retornam `Value::None` — ignorados
silenciosamente.

### Arquitectura de aplicação do estilo em `text`

Em vez de emitir um `Content::Styled(body, styles)` envolvendo o
conteúdo subsequente (abordagem vanilla), o cristalino **baked-in**
o `TextStyle` em cada `Content::Text` no momento da produção em
`eval`:

```rust
// eval/mod.rs ~228
SyntaxKind::Text => {
    let style = TextStyle::from(&*styles);
    let text_node = Content::Text(child.text().as_str().into(), style);
    ...
}
```

O `*styles: &mut StyleChain` é propagado pelo eval (desde Passo 94,
ADR-0036 segunda aplicação). `#set text(bold: true)` faz
`*styles = styles.push(delta)` — a cadeia cresce; subsequentes
`Content::Text` capturam a vista resolvida.

### Propriedades cobertas em `#set text`

| Propriedade Typst | StyleDelta | Converte de |
|-------------------|-----------|-------------|
| `bold`   | `Option<bool>` | `Value::Bool` |
| `italic` | `Option<bool>` | `Value::Bool` |
| `size`   | `Option<f64>`  | `Value::Length` → `.abs.to_pt()` |

Outras (fill, weight, font, lang, style como string) — **não
suportadas**. Silenciosamente ignoradas (`_ => { /* ... */ }`).

### Testes existentes

6 testes em `eval/tests.rs`:
- `eval_set_text_bold`
- `eval_set_text_size`
- `eval_set_target_desconhecido_ignora` (#set par)
- `eval_set_e_content_combinados`
- `estilo_capturado_no_momento_da_producao`
- Plus testes em `pipeline_com_set_text_bold` (integração).

---

## Parte 2 — Catálogo de propriedades disponíveis

### Enum `Style` (pós-Passo 99)

```rust
pub enum Style {
    Bold(bool),
    Italic(bool),
    Size(Pt),
    Fill(Color),         // forward-compat, não consumido pelo Layouter hoje
    HeadingLevel(u8),    // forward-compat
}
```

### Intersecção `#set text(...)` × enum `Style`

| Typst syntax | `Style` variant | Implementado hoje? |
|-------------|-----------------|-------------------:|
| `#set text(bold: true)` | `Bold(true)` | Sim (baked-in) |
| `#set text(italic: true)` | `Italic(true)` | Sim (baked-in) |
| `#set text(size: Npt)` | `Size(Pt(N))` | Sim (baked-in) |
| `#set text(fill: color)` | `Fill(Color)` | **Não** — StyleDelta não tem fill-capture; `Content::Text.style` ganhou `fill` no Passo 100 mas não é preenchido por `#set` |

### Decisão

**#set `text` já activo** para bold/italic/size via arquitectura
bake-in. A arquitectura alternativa da spec (wrap em
`Content::Styled`) seria uma refactorização paralela com
sobreposição funcional: as duas produzem o mesmo output visual para
texto inline.

**Não fazer a refactorização para wrapping neste passo**. A dívida
estrutural (bake-in vs. wrapping) é registada como trabalho futuro
porque:

1. Zero regressão é o critério. Trocar arquitectura com 783 testes
   vinculados tem risco de perturbação.
2. O wrapping só traz valor **novo** para conteúdos que não sejam
   `Content::Text` (shapes, images, grids). Hoje, o Layouter tem
   arms dedicados para esses — alterar precisa de análise que
   excede este passo.
3. A regressão do `#set` baked-in **afectaria tests existentes**;
   a criação de `Content::Styled` **complementar** cria duplicação.

Acção neste passo:

- **Confirmar** que `#set text` funciona end-to-end (Parse → eval →
  layout → FrameItem.style correcto) via testes novos.
- **Cobrir** a propriedade `fill` que foi adicionada no Passo 100
  mas ainda não é preenchida por `#set text(fill: ...)`. Estender
  `StyleDelta` ou adicionar conversão `Value::Color → Style::Fill`.

Propriedades adiadas (bloqueadas por tipos não materializados):

- `text.font` — requer Font real.
- `text.lang`, `text.region` — requer vocabulário de localização.
- `par.leading`, `par.spacing` — requer sistema de parágrafo.
- `text.weight` como string ("bold", "regular") — hoje só aceita
  `bool`; mapeamento string→bool fica para passo futuro que expanda
  o vocabulário.

---

## Parte 3 — AST de `SetRule`

```rust
pub struct SetRule<'a> { ... }

impl<'a> SetRule<'a> {
    pub fn target(&self) -> Expr<'a> { ... }  // nome da função
    pub fn args(&self) -> Args<'a> { ... }   // argumentos
    // Opcional: .condition() para `#set text(...) if cond`
}
```

`target()` retorna um `Expr`, normalmente `Expr::Ident("text")`.
O cristalino faz `set.target().to_untyped().text_str()` para obter
a string — já implementado em `eval_set_rule`.

---

## Recomendação

`#set` já está activo. Este passo é **validação + gap fill**:

1. Adicionar suporte para `#set text(fill: color)` se `Color` está
   materializado em L1 (confirmar em 102.C).
2. Adicionar testes end-to-end que verifiquem `FrameItem::Text.style.size`
   reflecte `#set text(size: 18pt)` (não apenas que `eval` não dá Err).
3. Registar em ADR-0040 a decisão de manter bake-in + razão.
4. Abrir DEBT-49 para refactorização futura "produzir
   `Content::Styled` em vez de bake-in" se/quando se justificar
   (provavelmente depois de `Introspection` materializar).
