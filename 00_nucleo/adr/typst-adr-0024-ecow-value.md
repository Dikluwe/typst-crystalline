# ⚖️ ADR-0024: `ecow` → `[l1_allowed_external]` para `Value::Str`

**Status**: `IMPLEMENTADO`
**Data**: 2026-03-27
**Contexto**: Passo 13 — subset de Value

---

## Contexto

ADR-0015 removeu `EcoString` do parser (lexer e parse.rs), onde strings
são construídas uma vez durante a construção da CST e raramente clonadas.
A substituição por `String`/`SyntaxText` foi correcta nesse contexto.

`Value::Str` em eval() é um contexto fundamentalmente diferente. Durante
a avaliação de um documento Typst, strings são:
- Passadas como argumentos em cada chamada de função
- Capturadas em closures
- Retornadas de funções e atribuídas a variáveis
- Concatenadas com o operador `+`
- Comparadas, interpoladas, e inspeccionadas

Cada uma destas operações com `String` implica um `malloc` e uma cópia
O(n). `EcoString` usa contagem de referências — clone é O(1) quando a
string não é modificada, O(n) apenas quando é mutada (copy-on-write).

Substituir `EcoString` por `String` em `Value::Str` seria um erro técnico
grave com impacto de performance visível em documentos com strings
frequentes. ADR-0015 foi correcta para o parser; não é extensível
mecanicamente a Value.

---

## Análise de pureza

| Propriedade | Estado |
|-------------|--------|
| Zero I/O | ✓ — estrutura de dados em memória |
| Zero estado global mutável | ✓ |
| Determinismo total | ✓ |
| Clone O(1) para strings não mutadas | ✓ — propriedade crítica para eval() |

---

## Decisão

`ecow` é adicionado a `[l1_allowed_external]`:

```toml
[l1_allowed_external]
rust = [
    "thiserror",
    "comemo",
    "unicode_ident",
    "unicode_math_class",
    "unicode_script",
    "unicode_segmentation",
    "rustc_hash",
    "time",
    "indexmap",
    "ecow",  # ADR-0024 — EcoString em Value::Str; clone O(1) no hot path de eval()
]
```

`Value::Str` usa `EcoString`:

```rust
use ecow::EcoString;

pub enum Value {
    Str(EcoString),
    // ...
}
```

`EcoString` não entra nas assinaturas públicas de L1 além de `Value::Str`.
`SyntaxText(Arc<str>)` mantém-se como tipo de domínio para texto da CST
(ADR-0004) — dois contextos diferentes, duas representações correctas.

---

## Relação com ADR-0015

ADR-0015 não é revogada. O parser continua sem `ecow`. A distinção é:

| Contexto | Tipo | Razão |
|----------|------|-------|
| CST — texto de tokens | `SyntaxText(Arc<str>)` | Construído uma vez, raramente clonado |
| eval() — valor de string | `EcoString` | Clonado em cada passagem de argumento |

São contextos com características de uso opostas. A decisão correcta
é diferente em cada um.

---

## O que esta ADR não decide

- Outros usos de `ecow` além de `Value::Str` — avaliar caso a caso
- `EcoVec` para colecções em `Value::Array` — ADR separada quando Array migrar

---

## Consequências

**Positivas**: clone O(1) de strings em eval() — performance correcta
para documentos com strings frequentes.

**Negativas**: `ecow` entra em `[l1_allowed_external]` após ter sido
excluída em ADR-0015. A aparente inconsistência é real e documentada —
contextos diferentes têm requisitos diferentes.

**Neutras**: `EcoString` implementa `From<&str>`, `From<String>`,
`Deref<Target=str>` — interface compatível com os testes existentes.

---

## Referências

- ADR-0015 — remoção de `ecow` do parser (não revogada)
- ADR-0004 — `SyntaxText(Arc<str>)` para texto da CST
- `ecow`: https://github.com/typst/ecow
