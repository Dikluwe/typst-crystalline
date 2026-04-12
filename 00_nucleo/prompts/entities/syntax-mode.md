# Prompt L0 — `entities/syntax_mode`

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/syntax_mode.rs`
**Criado em**: 2026-03-23 (Passo 4 — junto com o parser)
**Atualizado em**: 2026-04-12 (restauro — prompt dedicado criado; antes coberto por `rules/parse.md`)
**ADRs relevantes**: nenhum ADR dedicado; parte integrante do pipeline parse/lex (Passo 4)

---

## Contexto e Objetivo

O Lexer e o Parser do Cristalino operam com regras léxicas distintas consoante
o contexto sintáctico. O mesmo caractere `*` pode ser um operador matemático
(no modo Math), um marcador de itálico (no modo Markup) ou um operador de
multiplicação (no modo Code).

`SyntaxMode` representa a **máquina de estados base** da análise léxica —
o modo activo que determina como o `Lexer` tokeniza cada sequência de
caracteres e como o `Parser` interpreta os tokens.

A troca de modo ocorre explicitamente durante o parsing:
- `#`/`{`/`}` → alterna para/de Code
- `$...$` → entra/sai de Math
- Markup é o modo de topo (entrada do compilador)

Origem: `lab/typst-original/crates/typst-syntax/src/set.rs` (usado em Lexer/Parser)

---

## Restrições Estruturais

- Camada **L1**: zero I/O, zero estado global.
- `SyntaxMode` é `Copy + Clone + Debug + PartialEq + Eq + Hash` — sem dados associados.
- As variantes são **marcadores de estado puros** (unit variants): sem campos.
- Sem dependências externas.

---

## Instrução

```rust
/// O modo sintáctico de uma porção de código Typst.
///
/// Determina como o lexer tokeniza e o parser interpreta o texto.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum SyntaxMode {
    /// Texto e markup — o nível de topo.
    /// Exemplos: parágrafos, headings, ênfase (`*bold*`, `_italic_`).
    Markup,
    /// Átomos matemáticos, operadores, etc. — dentro de equações.
    /// Entra com `$`, sai com `$`.
    Math,
    /// Palavras-chave, literais e operadores — após `#` ou dentro de `{...}`.
    Code,
}
```

---

## Critérios de Verificação

```
// Copy — copiar não move
let a = SyntaxMode::Markup;
let b = a;
a == b                       → true

// Variantes distintas
SyntaxMode::Markup != SyntaxMode::Math
SyntaxMode::Math   != SyntaxMode::Code
SyntaxMode::Markup != SyntaxMode::Code

// Debug
format!("{:?}", SyntaxMode::Markup)  = "Markup"
format!("{:?}", SyntaxMode::Math)    = "Math"
format!("{:?}", SyntaxMode::Code)    = "Code"
```

---

## Resultado Esperado

- `01_core/src/entities/syntax_mode.rs` com `SyntaxMode` e testes co-localizados
- Cabeçalho de linhagem apontando para este ficheiro
  (`@prompt 00_nucleo/prompts/entities/syntax-mode.md`)

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-03-23 | Criação — Passo 4: enum de modo para Lexer/Parser | `syntax_mode.rs` |
| 2026-04-12 | Restauro — prompt dedicado criado (antes coberto por `rules/parse.md`) | `syntax-mode.md` |
