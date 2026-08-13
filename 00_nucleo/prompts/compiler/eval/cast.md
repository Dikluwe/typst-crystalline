# Prompt L0 — `rules/eval/cast` — Casts implícitos de Value
Hash do Código: 00000000

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/eval/cast.rs`
**Criado em**: 2026-06-25 (P469 — Value::Relative)
**ADRs**: ADR-0107 (paridade linguagem)

---

## Propósito

Casts implícitos de `Value` para tipos concretos de domínio.
No eval puro, `Value::Relative` não pode ser resolvido para `Length`
porque falta o contexto de layout (largura/altura do container).
Consumers em Trilha 7 recebem `Rel<Length>` e resolvem com contexto.

> **Fonte de paridade (P1031)** — a documentação oficial define `relative` como
> *"A length in relation to some known length"* e explicita que a componente `ratio`
> só ganha valor quando existe esse comprimento de referência (ex.: *"the rectangle's
> width is set to `{25%}`, so it takes up one fourth of the page's inner width"*).
> Doc comment `#[ty]` do vanilla ratificado (`e0e8ca4d`) em
> `lab/typst-original/crates/typst-library/src/layout/rel.rs:11-25` — texto publicado em
> `typst.app/docs/reference/layout/relative/`.
>
> Isto sustenta a afirmação do propósito: sem o comprimento de referência (que só o
> layout conhece), a resolução `Relative → Length` não é definível. Guardas:
> `01_core/src/compiler/eval/cast.rs:47` (`cast_length_relative_needs_context`) e `:42`/`:53`.
>
> **Natureza da citação**: contextual. A documentação não diz a frase "falta o contexto de
> layout"; diz que a relação é com *um comprimento conhecido*. A conclusão de que o eval
> puro não o conhece é do cristalino (separação eval/layout), não da linguagem.

---

## API

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum CastError {
    TypeMismatch,
    NeedsContext(&'static str),
}

pub fn cast_length(value: Value) -> Result<Length, CastError>;
```

- `cast_length(Value::Length(l))` → `Ok(l)`.
- `cast_length(Value::Relative(_))` → `Err(NeedsContext(...))`.
- `cast_length(outro)` → `Err(TypeMismatch)`.

---

## Critérios de verificação

- `cast_length(Value::Length(Length::cm(2.0)))` → `Ok(Length::cm(2.0))`.
- `cast_length(Value::Relative(...))` → `Err(CastError::NeedsContext(_))`.
- `cast_length(Value::Int(42))` → `Err(CastError::TypeMismatch)`.

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-06-25 | P469: cast_length + CastError para Value::Relative | `cast.rs`, `cast.md` |
