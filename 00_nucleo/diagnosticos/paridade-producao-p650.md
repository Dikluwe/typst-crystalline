# Relatório de Paridade — P650

**Passo:** 650  
**Data:** 2026-07-09  
**Foco:** Segunda ronda de auditoria de falhas silenciosas, com padrões não cobertos por P633.  
**Dependências:** P633 (método e padrões base), P638 (extensão ao vanilla).

---

## 1. Parte 1 — Cobertura desde P633

| Métrica | Valor |
|---|---|
| Total de ficheiros `.rs` em produção (01_core, 02_shell, 03_infra, 04_wiring) | 342 |
| Ficheiros novos ou alterados desde `paridade-producao-p633.md` | 25 |

### Ficheiros novos/alterados desde P633

```
01_core/src/entities/ast/code.rs
01_core/src/entities/ast/expr.rs
01_core/src/entities/ast/markup.rs
01_core/src/entities/layout_types.rs
01_core/src/entities/syntax_node.rs
01_core/src/rules/eval/bibliography.rs
01_core/src/rules/eval/bindings.rs
01_core/src/rules/eval/closures.rs
01_core/src/rules/eval/mod.rs
01_core/src/rules/eval/rules.rs
01_core/src/rules/eval/tests.rs
01_core/src/rules/introspect/fixpoint.rs
01_core/src/rules/introspect/from_tags.rs
01_core/src/rules/introspect.rs
01_core/src/rules/layout/bib_csl.rs
01_core/src/rules/layout/grid.rs
01_core/src/rules/layout/mod.rs
01_core/src/rules/layout/tests.rs
01_core/src/rules/lexer/code.rs
01_core/src/rules/lexer/markup.rs
01_core/src/rules/lexer/mod.rs
01_core/src/rules/parse/mod.rs
01_core/src/rules/parse/parser.rs
01_core/src/rules/stdlib/counter.rs
03_infra/src/pipeline.rs
```

**Nota:** todos estes ficheiros foram incluídos na varredura da Parte 2; nenhum foi assumido como seguro só por ter sido escrito recentemente.

---

## 2. Parte 2 — Padrões novos

### 2.1 Padrão 7 — `.unwrap_or(valor)` genérico

- **Ocorrências em produção:** ~210 em `01_core/src`, mais dezenas em `03_infra/src`.
- **Classificação geral:** a grande maioria são defaults neutros de domínio (`0.0`, `false`, `Length::ZERO`, `Value::None` para argumentos opcionais, etc.) — **Inofensivo**.
- **Casos confirmados como problemáticos:**

| file:line | Código | Problema | Teste directo |
|---|---|---|---|
| `01_core/src/rules/eval/rules.rs:646` | `eval_expr(named.expr(), ...).unwrap_or(Value::None)` para `heading.numbering` | Erro na expressão `numbering` é descartado. | `#set heading(numbering: 1/0)` compila com sucesso (exit 0), sem erro nem aviso. |
| `01_core/src/rules/eval/rules.rs:952` | `eval_expr(named.expr(), ...).unwrap_or(Value::None)` para args named de `#set` em elementos de utilizador | Erro na expressão do argumento é descartado. | `#let myelem(body) = body` + `#set myelem(foo: 1/0)` — o erro de `1/0` não é propagado (aparece apenas o warning de target não suportado). |

- **Caso suspeito (a investigar):**

| file:line | Código | Risco |
|---|---|---|
| `01_core/src/rules/stdlib/collections.rs:184,187` | `value_cmp(...).unwrap_or(Ordering::Equal)` | Comparação entre tipos incompatíveis assume `Equal`, tornando `array.sorted()` silenciosamente instável. Teste: `(1, "a", 2).sorted()` compila sem erro. |

### 2.2 Padrão 8 — `saturating_*` e `clamp`

- **Ocorrências em produção:** ~30 (principalmente em `gradient.rs`, `color.rs`, `heading.rs`, `calc.rs`, `grid.rs`).
- **Classificação:** **Inofensivo**. Todos os casos verificados são limites de domínio intencionais (cores, níveis de heading, pesos de fonte, geometria de grid) ou a função pública `calc.clamp()` que o utilizador invoca explicitamente.

### 2.3 Padrão 9 — `#[allow(dead_code)]`

- **Ocorrências em produção:** 7.
- **Classificação:** **Inofensivo**. Todas são APIs de reparse/replace incremental planeada (`parser.rs`, `markup.rs`, `code.rs`, `syntax_node.rs`) ou helper `available_height()` em `layout/mod.rs` que tem utilidade futura documentada.

### 2.4 Padrão 10 — `Vec::retain` / `filter`

- **Ocorrências em produção:** ~30 em `01_core/src/rules` (sem contar testes).
- **Classificação geral:** **Inofensivo**. A maioria são filtros de tags/introspecção, contagem de tipos, ou funções de biblioteca pública (`array.filter`, `dict.filter`) onde o utilizador controla o predicado.
- **Caso observado:** `01_core/src/rules/eval/math.rs:257` (`cols.retain(|c| !c.is_empty())` em `cases(...)`) remove colunas vazias de delimitadores de alinhamento — comportamento esperado em math cases.

### 2.5 Padrão 11 — `eprintln!` / `log::warn!` / `log::error!`

- **Ocorrências em produção:** 4 em `01_core/src`, 4 em `03_infra/src` (excluindo testes).
- **Classificação:** **Confirmado** — mensagens que só chegam ao terminal do desenvolvedor, não ao diagnóstico do utilizador.

| file:line | Código | Contexto |
|---|---|---|
| `01_core/src/entities/content.rs:2311` | `eprintln!("P627 segments count: {}", segments.len())` | Particionamento de `columns` com pagebreaks. Aparece sempre que se usa `#set page(columns: n)`. |
| `03_infra/src/export/images.rs:279` | `eprintln!("PNG inválido — imagem omitida: {}", e)` | Imagem inválida é omitida do PDF sem erro de compilação. |
| `03_infra/src/export/images.rs:285` | `eprintln!("Formato de imagem desconhecido — imagem omitido")` | Formato de imagem desconhecido é omitido sem erro. |
| `03_infra/src/export/builder.rs:658` | `eprintln!("Aviso: falha ao instanciar fonte variável ...")` | Falha de instanciação de fonte variável reportada só no terminal. |
| `03_infra/src/font_variant.rs:193` | `eprintln!("fontTools instancer falhou: ...")` | Chamado pelo builder antes do aviso acima; também só no terminal. |

### 2.6 Padrão 12 — `panic!` / `unreachable!()`

- **Ocorrências em produção:** centenas, mas a esmagadora maioria dentro de `#[cfg(test)]`.
- **Classificação:** **Inofensivo**. Os `unreachable!()` em produção (ex: `lexer/code.rs:192`) cobrem casos realmente inalcançáveis por construção. Os `panic!()` em produção são downcasts internos em funções nativas onde o tipo é garantido pelo caller.
- **Nota:** não foi encontrado nenhum `panic!` ou `unreachable!()` em produção que pareça alcançável por input do utilizador nesta ronda.

---

## 3. Lista priorizada de follow-ups

| Prioridade | Item | file:line | Tipo |
|---|---|---|---|
| 1 | `eprintln!` de debug deixado em `content.rs` | `01_core/src/entities/content.rs:2311` | Confirmação de debug esquecido |
| 2 | Imagem inválida/desenhecida omitida com `eprintln!` | `03_infra/src/export/images.rs:279,285` | Falha silenciosa |
| 3 | Falha de instanciação de fonte variável só em `eprintln!` | `03_infra/src/export/builder.rs:658` + `03_infra/src/font_variant.rs:193` | Diagnóstico não chega ao utilizador |
| 4 | Erros em `#set heading(numbering: ...)` descartados | `01_core/src/rules/eval/rules.rs:646` | Falha silenciosa |
| 5 | Erros em args named de `#set` para user elements descartados | `01_core/src/rules/eval/rules.rs:952` | Falha silenciosa |
| 6 | `array.sorted()` com tipos incompatíveis assume `Equal` | `01_core/src/rules/stdlib/collections.rs:184,187` | Comportamento incorreto silencioso |

---

## 4. Decisão

Nenhuma correcção foi feita neste passo. Cada item da lista priorizada deve virar o seu próprio passo de correcção, com sonda-causa-correcção, tal como a sequência P633-P649 fez.

---

## 5. Validação

- `cargo test --workspace`: todos os crates passaram (sem alterações de código, apenas auditoria).
- `crystalline-lint .`: `✓ No violations found`.
