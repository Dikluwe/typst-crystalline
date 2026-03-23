# ⚖️ ADR-0010: `unicode_ident` → `[l1_allowed_external]`

**Status**: `PROPOSTO`
**Data**: 2026-03-23

---

## Contexto

O lexer (`lexer.rs`) usa `unicode_ident::is_xid_start` e
`is_xid_continue` para determinar se um caractere pode iniciar
ou continuar um identificador Typst.

A especificação de identificadores do Typst segue o standard Unicode
UAX #31 (Unicode Identifier and Pattern Syntax), que define
`XID_Start` e `XID_Continue` como as classes de caracteres válidos
para identificadores em linguagens de programação.

Implementar estas tabelas em L1 manualmente seria reproduzir ~800KB
de dados Unicode derivados de um standard em evolução — trabalho
sem ganho arquitectural e com risco de divergência.

`unicode_ident` é mantido pela equipa do compilador Rust e é a
implementação de referência de UAX #31 no ecossistema Rust.

---

## Análise de pureza

| Propriedade | Estado |
|-------------|--------|
| Zero I/O | ✓ — tabelas compiladas em tempo de compilação |
| Zero estado global mutável | ✓ — funções puras sobre dados estáticos |
| Determinismo total | ✓ — mesma entrada, mesma saída em qualquer ambiente |
| Dependências transitivas | ✓ — zero dependências externas |

V13 (MutableStateInCore) não dispara. V14 (ExternalTypeInContract)
não dispara — nenhum tipo de `unicode_ident` aparece em assinaturas
públicas de L1; a crate expõe apenas funções `fn(char) -> bool`.

---

## Decisão

`unicode_ident` é adicionado a `[l1_allowed_external]` em
`crystalline.toml`:

```toml
[l1_allowed_external]
rust = ["thiserror", "comemo", "unicode_ident"]
```

E a `[workspace.dependencies]` no `Cargo.toml` raiz:

```toml
unicode-ident = "1"
```

E a `[dependencies]` de `01_core/Cargo.toml`:

```toml
unicode-ident = { workspace = true }
```

---

## Uso no lexer

```rust
use unicode_ident::{is_xid_continue, is_xid_start};

// Determina se 'c' pode iniciar um identificador Typst
fn is_id_start(c: char) -> bool {
    is_xid_start(c) || c == '_'
}

// Determina se 'c' pode continuar um identificador Typst
fn is_id_continue(c: char) -> bool {
    is_xid_continue(c) || c == '_' || c == '-'
}
```

---

## Prompts afectados

| Prompt | Natureza da mudança |
|--------|---------------------|
| `00_nucleo/prompts/rules/parse.md` | Documentar `unicode_ident` como externo autorizado; referenciar ADR-0010 |

---

## Consequências

**Positivas**: O lexer cumpre UAX #31 sem reproduzir tabelas Unicode
manualmente; a especificação de identificadores é correcta para
qualquer script humano (Latim, Cirílico, CJK, etc.).

**Negativas**: Dependência de crate externa em L1 — mitigada pelo
facto de ser zero-dependency, mantida pelo compilador Rust e
semanticamente parte da especificação da linguagem.

**Neutras**: `unicode_ident` é um dos pacotes mais descarregados
do ecossistema Rust — risco de abandono negligenciável.

---

## Alternativas consideradas

| Alternativa | Prós | Contras |
|-------------|------|---------|
| Inlining das tabelas em L1 | Zero dependências | ~800KB de dados derivados de standard em evolução; risco de divergência |
| Mover validação de identificadores para L3 | L1 sem crate | O domínio deixa de saber o que é um identificador válido — semanticamente incorrecto |

---

## Referências

- Unicode UAX #31: https://www.unicode.org/reports/tr31/
- `unicode-ident`: https://github.com/dtolnay/unicode-ident
- Diagnóstico Passo 4 — `is_xid_start`, `is_xid_continue` em lexer.rs
