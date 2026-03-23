# ⚖️ ADR-0014: `unscanny` → `01_core/rules/lexer/scanner.rs` (inline)

**Status**: `PROPOSTO`
**Data**: 2026-03-23

---

## Contexto

O lexer (`lexer.rs`) usa `unscanny::Scanner` como motor central de
travessia de string — não é uma chamada pontual, é a abstracção
fundamental sobre a qual o lexer inteiro opera.

`Scanner` gere um `&str` com um índice de byte e expõe:
- Cursor management: posição actual, avanço seguro em UTF-8
- Peeking & eating: `peek()`, `eat()`, `eat_while()`, `eat_until()`
- Selection: `select()` — retorna o slice do que foi consumido

Tamanho: ~350–400 linhas de código efectivo (excluindo testes e docs).

Duas opções foram avaliadas:

**Opção A — Inline**: copiar `Scanner` para
`01_core/src/rules/lexer/scanner.rs` como módulo interno de L1.

**Opção B — Autorizar**: adicionar `unscanny` a `[l1_allowed_external]`.

---

## Análise comparativa

| Critério | Opção A (Inline) | Opção B (Autorizar) |
|----------|-----------------|---------------------|
| Independência de API | ✓ mudanças upstream não quebram L1 | ✗ actualizações podem quebrar lexer |
| Customização | ✓ métodos específicos do Typst (ex: `eat_comment`) adicionáveis directamente | ✗ requer wrapper |
| Bugs de borda UTF-8 | somos donos — correcção imediata e rastreável | upstream corrige, nós recebemos passivamente |
| Tamanho do inline | 350–400 linhas — justifica módulo próprio, gerível como dono | — |
| Dependências transitivas | zero | zero (unscanny é zero-dep) |
| Pureza de L1 | ✓ zero crates externas para o motor do lexer | ✗ dependência de crate de utilidade |

A diferença face às crates Unicode (ADRs 0010–0013): `unscanny` é
um algoritmo de utilidade (~400 linhas), não uma tabela de dados
derivada de standard internacional (~milhares de entradas). Inlinear
algoritmos de utilidade é preferível a inlinear tabelas de dados —
o risco de manutenção é diferente em natureza e escala.

---

## Decisão

**Opção A — Inline.**

`Scanner` é migrado para `01_core/src/rules/lexer/scanner.rs`.
`unscanny` não entra em `[l1_allowed_external]`.

A interface pública de `Scanner` é mantida idêntica à do `unscanny`
original para que a migração do `lexer.rs` no Passo 4 seja um
search-replace de imports sem alteração de lógica:

```rust
// Antes (lab/typst-original):
use unscanny::Scanner;

// Depois (01_core):
use crate::rules::lexer::scanner::Scanner;
```

O módulo interno pode receber métodos específicos do léxico Typst
directamente (ex: `eat_raw_delim`, `eat_escape`) sem precisar de
wrapper — vantagem que a Opção B não permite.

---

## Estrutura do módulo

```
01_core/src/rules/
  mod.rs
  lexer/
    mod.rs
    scanner.rs   ← Scanner inlinado de unscanny (Apache-2.0)
  parse.rs       ← parse(), parse_code(), parse_math()
```

Header de `scanner.rs`:

```rust
//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/scanner.md
//! @prompt-hash <hash>
//! @layer L1
//! @updated 2026-03-23
//!
//! String scanner para o lexer do Typst.
//! Inlinado de `unscanny` (Apache-2.0) — ADR-0014.
//! Origem: https://github.com/typst/unscanny
```

Um prompt L0 é criado para este módulo:
`00_nucleo/prompts/rules/scanner.md`

---

## Nota de atribuição

`Scanner` é derivado de `unscanny` (licença Apache-2.0, autoria
Typst GmbH). O comentário de origem é mantido no ficheiro.
Como o fork já é do repositório Typst, a atribuição está implícita
no histórico git — o comentário serve de marcador explícito para
leitores de `scanner.rs` que não conheçam a origem.

---

## Prompts afectados

| Prompt | Natureza da mudança |
|--------|---------------------|
| `00_nucleo/prompts/rules/scanner.md` | Criar — novo módulo |
| `00_nucleo/prompts/rules/parse.md` | Actualizar import: `unscanny::Scanner` → `crate::rules::lexer::scanner::Scanner` |

---

## Consequências

**Positivas**: L1 sem dependência de crate externa para o motor do
lexer; `Scanner` customizável directamente para o léxico Typst;
mudanças de API upstream não afectam o lexer.

**Negativas**: Somos donos de ~400 linhas de gestão de cursor UTF-8.
Se um bug de borda existir no `unscanny` original e for corrigido
upstream, a correcção não chega automaticamente — mas será visível
nos testes de paridade do Passo 9.

**Neutras**: A interface pública de `Scanner` é idêntica — a
migração do lexer.rs é search-replace de imports.

---

## Alternativas consideradas

| Alternativa | Prós | Contras |
|-------------|------|---------|
| Autorizar `unscanny` em `[l1_allowed_external]` | Zero manutenção de Scanner | API pode mudar; sem customização directa; precedente de crates de utilidade em L1 |

---

## Referências

- `unscanny`: https://github.com/typst/unscanny
- Diagnóstico Passo 4 — `Scanner` é o motor central de lexer.rs
- ADR-0010–0013 — contraste: tabelas de dados vs algoritmos de utilidade
