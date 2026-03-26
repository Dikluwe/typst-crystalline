# Passo 6 — SourceResult, SourceDiagnostic e stubs (v3)

## Contexto e decisão de âmbito

O diagnóstico confirmou que `eval()` depende de `typst-library`
através de 7 imports em `lib.rs`. A conclusão precipitada seria
"eval() vai para L3 porque tem muitas dependências". Essa conclusão
está errada.

**O erro de classificação**: `eval()` é o motor de avaliação de uma
AST — é lógica de domínio central de um compilador, não I/O.
`typst-library` está mal estratificada no original: mistura tipos
de domínio puro (`Module`, `Value`, `Content`, `SourceDiagnostic`)
com implementações que têm dependências externas. A migração
cristalina é o processo de separar isso.

**Âmbito deste passo**: migrar o que é domínio puro identificado
no diagnóstico. Adiar `eval()` e os tipos complexos (`Module`,
`Value`, `Content`) para quando a análise de `typst-library` estiver
feita.

O que migra agora:
1. `SourceDiagnostic`, `Tracepoint`, `SourceResult` → L1
2. `Module(())` stub opaco → L1
3. `Value(())` stub opaco → L1
4. ADR-0016 documenta o adiamento de `eval()` e a estratégia para `typst-library`

Pré-condição: `cargo test -p typst-core` — 150 testes, zero violations.

---

## Diagnóstico complementar — Tracepoint

Antes de qualquer código, verificar o que é `Tracepoint`:

```bash
# Onde Tracepoint é definido
grep -rn "^pub enum Tracepoint\|^pub struct Tracepoint" \
  lab/typst-original/crates/ | head -5

# O que contém
grep -n "Tracepoint\|enum Tracepoint" \
  lab/typst-original/crates/typst-library/src/engine.rs 2>/dev/null | head -20
# ou onde quer que esteja definido

# Dependências externas do ficheiro onde Tracepoint está
grep "^use\|^extern" \
  lab/typst-original/crates/typst-library/src/engine.rs 2>/dev/null \
  | grep -v "crate::\|super::\|std::" | head -10
```

**Critério de decisão**:
- `Tracepoint` é `enum` com variantes que contêm apenas `Span`,
  `String`, ou primitivos → L1 directo
- `Tracepoint` contém tipos de `typst-library` → stub opaco em L1,
  migração completa adiada

---

## Tarefa 1 — ADR-0016: adiamento de eval() e estratégia typst-library

**Criar**: `00_nucleo/adr/0016-adiamento-eval-typst-library.md`

Conteúdo essencial:

```
Decisão: eval() não migra neste passo.

Razão: typst-library mistura tipos de domínio puro (Module, Value,
Content, SourceDiagnostic) com implementações que têm dependências
externas. Migrar eval() para L3 "tal como está" colocaria o motor
central do compilador em Infraestrutura — inversão arquitectural.

O trabalho em falta antes de eval() poder migrar:
1. Análise completa de typst-library/src/foundations/
2. Separação dos tipos de domínio puro (L1) das implementações (L3)
3. Module, Value, Content reais em L1 sem dependências externas
4. Só então eval() pode ser implementado em 01_core/rules/

Stubs criados neste passo para desbloquear o pipeline:
- Module(()) em 01_core/entities/module.rs
- Value(()) em 01_core/entities/value.rs
```

---

## Tarefa 2 — SourceDiagnostic e SourceResult em L1

**Criar**: `00_nucleo/prompts/entities/source-result.md`
**Criar**: `01_core/src/entities/source_result.rs`

### Nota sobre ecow e performance

`SourceDiagnostic` no original usa `EcoString` para mensagens e
`EcoVec` para colecções. ADR-0015 substituiu `EcoString` por
`String` no parser onde o custo é insignificante. Para
`SourceDiagnostic`, o custo também é insignificante — erros são
raros e não estão no hot path. A substituição por `String`/`Vec`
é correcta aqui.

Quando `Value` e `Content` chegarem, a decisão sobre `ecow` será
revista — esses tipos estão no hot path de avaliação.

### Substituições a aplicar

| Original | Substituição L1 | Razão |
|----------|----------------|-------|
| `EcoString` | `String` | ADR-0015 — erros não são hot path |
| `EcoVec<SourceDiagnostic>` | `Vec<SourceDiagnostic>` | ADR-0015 |
| `EcoVec<Spanned<EcoString>>` | `Vec<Spanned<String>>` (hints) | ADR-0015 |
| `EcoVec<Spanned<Tracepoint>>` | `Vec<Spanned<Tracepoint>>` | ADR-0015 |

### Interface alvo

```rust
// 01_core/src/entities/source_result.rs

/// Severidade de um diagnóstico do compilador.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Severity {
    Error,
    Warning,
}

/// Ponto de rastreio para stack traces de erros.
/// Definição baseada no diagnóstico — confirmar variantes reais.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Tracepoint {
    Call(Option<String>),   // chamada de função com nome opcional
    Show(String),           // aplicação de show rule
    Import,                 // import de módulo
}

/// Diagnóstico emitido pelo compilador Typst.
#[derive(Debug, Clone)]
pub struct SourceDiagnostic {
    pub severity: Severity,
    pub span:     Span,
    pub message:  String,
    pub hints:    Vec<String>,
    pub trace:    Vec<Spanned<Tracepoint>>,
}

impl SourceDiagnostic {
    pub fn error(span: Span, message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Error,
            span,
            message: message.into(),
            hints: vec![],
            trace: vec![],
        }
    }

    pub fn warning(span: Span, message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Warning,
            span,
            message: message.into(),
            hints: vec![],
            trace: vec![],
        }
    }

    pub fn with_hint(mut self, hint: impl Into<String>) -> Self {
        self.hints.push(hint.into());
        self
    }
}

/// Resultado de uma operação do compilador Typst.
pub type SourceResult<T> = Result<T, Vec<SourceDiagnostic>>;
```

**Adicionar a `entities/mod.rs`**:
```rust
pub mod source_result;
```

Critérios de verificação:
```
Dado SourceDiagnostic::error(span, "msg")
Quando severity for lido
Então Severity::Error

Dado SourceDiagnostic::error(span, "msg").with_hint("h")
Quando hints for lido
Então vec!["h"]

Dado SourceResult::Err(vec![diag])
Quando is_err() for chamado
Então true

Dado SourceResult::Ok(42u32)
Quando unwrap() for chamado
Então 42
```

---

## Tarefa 3 — Stubs opacos para Module e Value

**Criar**: `01_core/src/entities/module.rs`
**Criar**: `01_core/src/entities/value.rs`

```rust
// 01_core/src/entities/module.rs
//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/module.md
//! @prompt-hash <hash>
//! @layer L1
//! @updated 2026-03-23

/// Resultado da avaliação de um ficheiro Typst.
///
/// Stub opaco — interior definido quando typst-library/foundations/
/// for analisada e Module real for migrado.
/// Ver ADR-0016.
pub struct Module(());

// 01_core/src/entities/value.rs
//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/value.md
//! @prompt-hash <hash>
//! @layer L1
//! @updated 2026-03-23

/// Valor em tempo de avaliação do Typst.
///
/// Stub opaco — interior definido quando typst-library/foundations/
/// for analisada e Value real for migrado.
/// Ver ADR-0016.
pub struct Value(());
```

Criar prompts mínimos para ambos:
- `00_nucleo/prompts/entities/module.md` — stub, referenciar ADR-0016
- `00_nucleo/prompts/entities/value.md` — stub, referenciar ADR-0016

**Adicionar a `entities/mod.rs`**:
```rust
pub mod module;
pub mod value;
pub mod source_result;
```

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

Testes para `SourceDiagnostic`:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::span::Span;

    #[test]
    fn error_construtor() {
        let d = SourceDiagnostic::error(Span::detached(), "msg");
        assert_eq!(d.severity, Severity::Error);
        assert_eq!(d.message, "msg");
        assert!(d.hints.is_empty());
        assert!(d.trace.is_empty());
    }

    #[test]
    fn warning_construtor() {
        let d = SourceDiagnostic::warning(Span::detached(), "warn");
        assert_eq!(d.severity, Severity::Warning);
    }

    #[test]
    fn with_hint() {
        let d = SourceDiagnostic::error(Span::detached(), "e")
            .with_hint("try this");
        assert_eq!(d.hints, vec!["try this"]);
    }

    #[test]
    fn source_result_ok() {
        let r: SourceResult<u32> = Ok(42);
        assert_eq!(r.unwrap(), 42);
    }

    #[test]
    fn source_result_err() {
        let r: SourceResult<u32> = Err(vec![
            SourceDiagnostic::error(Span::detached(), "e")
        ]);
        assert!(r.is_err());
        assert_eq!(r.unwrap_err().len(), 1);
    }
}
```

---

## Ao terminar, reportar

- Variantes reais de `Tracepoint` encontradas no diagnóstico
- Se `Tracepoint` migrou directamente ou ficou como stub
- Número total de testes
- Zero violations confirmado

Esta informação vai para ADR-0016 e para o trabalho de análise
de `typst-library/src/foundations/` que precede o `eval()` real.
