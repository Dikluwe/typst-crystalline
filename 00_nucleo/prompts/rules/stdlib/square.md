# Prompt L0 — `stdlib/square` — helper geométrico sobre `Rect`
Hash do Código: c6d45acb

**Camada**: L1
**Ficheiro alvo**: `01_core/src/rules/stdlib/shapes.rs`
**Origem**: Passo 390 (`typst-passo-390.md`) — dívida genuína acidental (balde D), XS, derivável de `Rect`.
**ADRs**: ADR-0033 (paridade vanilla), ADR-0107 (paridade linguagem), ADR-0017 (não aplica — sem variant novo).

---

## 1. Contexto

O vanilla expõe `square(width, height: auto)` como um retângulo com lados iguais. No cristalino, `Rect` (`ShapeKind::Rect`) já existe via `native_rect`. `square` é um helper sintático que constrói `Rect` com `width == height` quando `height` não é fornecido.

## 2. Arquitetura

- **Sem tipo novo**: reutiliza `ShapeKind::Rect` e `Content::Shape` existentes.
- **Sem layout/render novo**: a saída é idêntica à de `rect(width: w, height: w)` por construção.
- **Convenção de assinatura e helpers**: ver `stdlib/_comum.md`.

## 3. Função nativa

Assinatura `fn native_square(ctx: &mut EvalContext<'_>, args: &Args) -> SourceResult<Value>` (ver `_comum.md`).

- `width`: primeiro argumento posicional ou nomeado `width`. Obrigatório.
- `height`: nomeado `height`; se omitido, assume o valor de `width`.
- `fill` / `stroke`: opcionais, mesmo parsing e fallback de `native_rect`.
- Argumentos nomeados desconhecidos → erro (padrão das nativas do cluster shapes).
- Devolve `Value::Content(Content::shape(ShapeKind::Rect, width, height, fill, final_stroke))`.

## 4. Paridade vanilla

- `square(1cm)` ≡ `rect(width: 1cm, height: 1cm)` (morfologicamente — mesma forma `Rect`).
- `square(1cm, height: 2cm)` comporta-se como `rect(width: 1cm, height: 2cm)` (fallback aceite).

## 5. Testes

- `square(w)` produz `ShapeKind::Rect` com `width == height == w`.
- `square(w, height: h)` com `h != w` produz `Rect` genérico.
- `square()` sem width → erro.
- Argumento nomeado inválido → erro.
- `square(w)` sem cores → stroke preta 1pt (paridade com `rect`).

## 6. Scope-out

- Não criar `ShapeKind::Square`.
- Não adicionar variant `Value` ou `Content`.
- Não tocar em layout/render/export.
