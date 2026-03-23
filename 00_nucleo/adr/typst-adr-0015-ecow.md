# ⚖️ ADR-0015: `ecow` removido do parser — `String`/`SyntaxText` interno

**Status**: `PROPOSTO`
**Data**: 2026-03-23

---

## Contexto

O parser (`parser.rs`) e o lexer (`lexer.rs`) usam `EcoString` e
`eco_format!` da crate `ecow` para construir internamente:
- Mensagens de erro sintáctico (texto de `SyntaxError::message`)
- Texto de tokens durante a construção de `SyntaxNode`

`ecow` já foi avaliado no ADR-0004 (contexto de `SyntaxNode`),
onde foi adoptada a Opção C: `SyntaxText(Arc<str>)` como tipo de
domínio público, com `EcoString` confinada à construção interna.

A questão agora é se `EcoString` precisa de existir *dentro* do
parser, ou se `String`/`format!` são suficientes para os usos
actuais.

---

## Análise de uso

Os usos de `EcoString`/`eco_format!` no parser e lexer são de dois
tipos:

**Tipo 1 — Mensagens de erro** (maioria dos usos):
```rust
// Original
node.convert_to_error(eco_format!("expected {}, found {}", expected, found));

// Após migração
node.convert_to_error(format!("expected {}, found {}", expected, found));
```
`SyntaxError::message` é `SyntaxText` — aceita `From<String>`.
A conversão `String → Arc<str>` acontece na construção do erro,
uma vez por erro sintáctico. Custo: uma alocação extra por erro.
No pipeline de parse de um documento típico, erros sintácticos são
raros — o custo é imperceptível.

**Tipo 2 — Texto de tokens** (usos pontuais):
```rust
// Original — clone O(1) de EcoString
let text: EcoString = lexer.slice().into();

// Após migração — Arc<str> via SyntaxText
let text: SyntaxText = lexer.slice().into();  // From<&str> já existe
```
`lexer.slice()` retorna `&str` — a conversão para `SyntaxText`
via `From<&str>` cria `Arc::from(slice)` directamente, sem passar
por `EcoString`. O clone O(1) do `EcoString` é substituído pelo
clone O(1) do `Arc<str>` — custo idêntico.

---

## Decisão

`ecow` é removido do parser e lexer. `EcoString` e `eco_format!`
são substituídos por `String`, `format!` e `SyntaxText` conforme
o contexto.

`ecow` não entra em `[l1_allowed_external]`.

Regras de substituição no Passo 4:

| Original | Substituição |
|----------|-------------|
| `EcoString` em assinatura interna | `SyntaxText` se exposto, `String` se temporário |
| `eco_format!(...)` | `format!(...)` |
| `EcoString::from(s)` onde `s: &str` | `SyntaxText::from(s)` |
| Clone de `EcoString` para passar para `SyntaxNode` | `SyntaxText::from(slice)` — `Arc::from(&str)` |

---

## Excepção a verificar no Passo 4

Se o parser usar `EcoString` para texto de tokens que são clonados
repetidamente *dentro* do pipeline de parse incremental (reparsing),
a substituição por `String` introduz alocações no hot path.

Critério de detecção: se `EcoString::clone()` aparecer dentro de
um loop que itera sobre os filhos de um nó durante reparsing, a
substituição deve ser `SyntaxText` (clone O(1) via `Arc`) — não
`String` (clone O(n)).

Para todos os outros usos (construção de erros, texto final de
tokens), `String`/`SyntaxText` são a substituição correcta.

---

## Estado de `ecow` após esta ADR

Com ADR-0015 executada, `ecow` deixa de existir em qualquer parte
de `01_core`. O `Cargo.toml` de `typst-core` não tem `ecow` como
dependência.

`ecow` pode continuar a existir em `03_infra` se necessário para
compatibilidade com APIs do ecossistema Typst durante a migração
— essa decisão pertence aos passos de L3.

---

## Prompts afectados

| Prompt | Natureza da mudança |
|--------|---------------------|
| `00_nucleo/prompts/rules/parse.md` | Documentar ausência de `ecow`; tabela de substituições; referenciar ADR-0015 |

---

## Consequências

**Positivas**: L1 sem `ecow` — `[l1_allowed_external]` não cresce
para além das seis entradas actuais; o parser é construído
exclusivamente com `std` e as crates Unicode já autorizadas.

**Negativas**: Uma alocação extra por erro sintáctico
(`format!` → `String` → `Arc<str>`). Imperceptível no contexto
de uso — erros sintácticos são raros em documentos válidos e o
parse acontece uma vez por ficheiro.

**Neutras**: `SyntaxText(Arc<str>)` já existe em L1 desde ADR-0004
(Opção C). Esta ADR fecha o círculo: `EcoString` nunca precisou
de entrar em L1 — `SyntaxText` é a fronteira correcta e suficiente.

---

## Alternativas consideradas

| Alternativa | Prós | Contras |
|-------------|------|---------|
| Autorizar `ecow` em `[l1_allowed_external]` | Zero alocações extra; zero refactorização | `ecow` em L1 após decisão explícita de Opção C em ADR-0004; precedente de crate de optimização em L1 |
| `ecow` como detalhe privado de `parse.rs` com `ecow` autorizado | Clone O(1) preservado | Autorização global — `ecow` fica visível em toda L1, não apenas no parser |

---

## Referências

- ADR-0004 — Opção C: `SyntaxText(Arc<str>)` como tipo de domínio; `EcoString` fora de L1
- `ecow`: https://github.com/typst/ecow
- Diagnóstico Passo 4 — `EcoString`, `eco_format!` em parser.rs e lexer.rs
