# Passo 4 — parse() e lexer

## Contexto

Ler antes de começar:
- `00_nucleo/adr/0001-estrategia-migracao.md`
- `lab/typst-original/crates/typst-syntax/src/parser.rs`
- `lab/typst-original/crates/typst-syntax/src/lexer.rs`

Estado do Passo 3: 69 testes, World + TrackedWorld, stubs opacos.

`parse()` é documentada como função pura `&str → SyntaxNode` sem
dependências externas. Este passo verifica e materializa isso.
`Source` real e AST ficam para o Passo 5 — parse() sozinha
é suficiente e verificável de forma isolada.

---

## Diagnósticos obrigatórios antes de qualquer código

```bash
# Dependências externas do parser e lexer
grep "^use\|^extern" lab/typst-original/crates/typst-syntax/src/parser.rs \
  | grep -v "crate::\|super::\|std::" | head -20

grep "^use\|^extern" lab/typst-original/crates/typst-syntax/src/lexer.rs \
  | grep -v "crate::\|super::\|std::" | head -20

# Estado global
grep -n "^static\|OnceLock\|LazyLock\|Mutex" \
  lab/typst-original/crates/typst-syntax/src/parser.rs \
  lab/typst-original/crates/typst-syntax/src/lexer.rs

# Assinatura de parse()
grep -n "^pub fn parse" lab/typst-original/crates/typst-syntax/src/parser.rs

# Tamanho dos ficheiros
wc -l lab/typst-original/crates/typst-syntax/src/parser.rs \
       lab/typst-original/crates/typst-syntax/src/lexer.rs
```

**Reportar o output antes de continuar.**

Se aparecer qualquer externo que não seja `SyntaxNode`, `SyntaxKind`,
`SyntaxSet`, `Span` (já em L1) — parar e reportar ao developer.
Não adicionar externos sem decisão explícita.

---

## Tarefa 1 — Prompt L0

**Criar**: `00_nucleo/prompts/rules/parse.md`

O prompt deve documentar:

- `parse(text: &str) -> SyntaxNode` — função pura, sem I/O, sem estado
- Se existirem variantes (`parse_math`, `parse_code`) — documentar cada uma
- O lexer como detalhe de implementação (não é API pública de L1)
- Critérios de verificação (ver abaixo)

---

## Tarefa 2 — Migrar para 01_core/rules/

**Destino**: `01_core/src/rules/parse.rs`
             `01_core/src/rules/lexer.rs` (se o lexer for ficheiro separado)

Criar o directório `01_core/src/rules/` e `mod.rs`:
```rust
//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/mod.md
//! @prompt-hash <hash>
//! @layer L1
//! @updated 2026-03-22

pub mod parse;
```

Header em cada ficheiro migrado:
```rust
//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/parse.md
//! @prompt-hash <hash>
//! @layer L1
//! @updated 2026-03-22
```

Adicionar `pub mod rules;` ao `01_core/src/lib.rs`.

---

## Tarefa 3 — Testes de paridade

Os testes de paridade são a parte mais importante deste passo.
`parse()` tem de produzir output **estruturalmente idêntico** ao
original para o mesmo input.

```rust
#[cfg(test)]
mod paridade {
    use super::*;

    // Comparar com SyntaxNode::spanless_eq — ignora spans
    // (spans mudam com numberize, que depende de FileId)

    #[test]
    fn texto_simples() {
        let node = parse("Hello, world!");
        assert_eq!(node.kind(), SyntaxKind::Markup);
        assert!(!node.erroneous());
        // Verificar filhos: Text("Hello, world!")
    }

    #[test]
    fn texto_vazio() {
        let node = parse("");
        assert_eq!(node.kind(), SyntaxKind::Markup);
        assert!(!node.erroneous());
    }

    #[test]
    fn parse_nunca_falha() {
        // parse() não pode panic — erros viram nós de erro
        let node = parse("#{{{broken");
        assert_eq!(node.kind(), SyntaxKind::Markup);
        assert!(node.erroneous());
        assert!(!node.errors().is_empty());
    }

    #[test]
    fn expressao_matematica() {
        let node = parse("$x^2 + 1$");
        assert_eq!(node.kind(), SyntaxKind::Markup);
        let eq = node.children().find(|n| n.kind() == SyntaxKind::Equation);
        assert!(eq.is_some());
    }

    #[test]
    fn codigo_typst() {
        let node = parse("#let x = 1");
        let binding = node.children()
            .find(|n| n.kind() == SyntaxKind::LetBinding);
        assert!(binding.is_some());
    }
}
```

---

## Actualizar Cargo.toml

Após os diagnósticos, adicionar apenas as dependências confirmadas.
Se `parse()` for realmente pura, `01_core/Cargo.toml` não muda.

---

## Verificação final

```bash
cargo test -p typst-core
cargo build
crystalline-lint .
crystalline-lint --fix-hashes .
crystalline-lint .
# ✓ No violations found
```

Critério de conclusão: `parse("Hello")` retorna `SyntaxNode` com
`kind() == SyntaxKind::Markup` e zero erros.

---

## Ao terminar, reportar

- Output dos diagnósticos (externos encontrados ou zero)
- Se parse() é realmente pura ou se havia surpresas
- Número de testes (paridade + regressão)
- Tamanho de parser.rs e lexer.rs em linhas

Essa informação vai para o Passo 5 (Source real + AST tipada).
