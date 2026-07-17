# Prompt L0 — `rules/eval/cast` — Casts implícitos de Value
Hash do Código: 00000000

**Camada**: L1
**Ficheiro alvo**: `01_core/src/engine/eval/cast.rs`
**Criado em**: 2026-06-25 (P469 — Value::Relative)
**ADRs**: ADR-0107 (paridade linguagem)

---

## Propósito

Casts implícitos de `Value` para tipos concretos de domínio.
No eval puro, `Value::Relative` não pode ser resolvido para `Length`
porque falta o contexto de layout (largura/altura do container).
Consumers em Trilha 7 recebem `Rel<Length>` e resolvem com contexto.

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
