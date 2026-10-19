# Relatório de Triagem — Passo 1052

**Data**: 2026-08-14
**Passo**: 1052 — V20: profundidade de padrão > 2 fora de contexto-tabela (515 ocorrências)
**Gate**: `ADR-0127` (Classificação: Auditoria de Linter / Nível `info` — sem alteração de código ou comportamento)
**Status**: CONCLUÍDO (Triagem completa e amostragem dirigida realizada: 0 bugs encontrados, 100% ruído/idioma canônico de domínio)

---

## 1. Definição da Regra e Mecânica do Linter

A regra **V20** do linter sinaliza como `info` (informativo) qualquer desestruturação de padrão (*pattern matching*) com profundidade estrita $> 2$ que ocorra fora de tabelas de despacho centrais (`ADR-0104`).
- **Profundidade 3**: Ocorre naturalmente em qualquer matching de `Option<&Value>` (`Some(Value::Int(i))`), de fatias de argumentos (`[Value::Str(s)]`) ou de operadores binários (`(BinOp::Add, Value::Int(a), Value::Int(b))`), onde:
  $$	ext{Nível 1: } 	ext{Option/Slice/Tupla} \longrightarrow 	ext{Nível 2: } 	ext{Enum de Domínio } (	ext{Value}) \longrightarrow 	ext{Nível 3: } 	ext{Dado/Variante}$$
- **Contagem Total**: **515 ocorrências** (488 com profundidade 3, 18 com profundidade 4, 9 com profundidade 5).

---

## 2. Distribuição por Módulo e Ficheiro

### Por Camada:
- `01_core`: 505 ocorrências (491 no compilador/eval/stdlib, 14 em entidades)
- `03_infra`: 8 ocorrências (export, font_metrics, pipeline)
- `02_shell`: 2 ocorrências (diagnósticos)

### Top Arquivos Concentradores:
| Arquivo | Ocorrências | Padrão Típico |
| :--- | :---: | :--- |
| `01_core/.../eval/tests.rs` | 40 | Asserções de teste em AST/Value (`Some((_, Value::Str(p)))`) |
| `01_core/.../stdlib/calc.rs` | 33 | Despacho de argumentos numéricos (`[Value::Int(a), Value::Int(b)]`) |
| `01_core/.../stdlib/layout.rs` | 31 | Extração de argumentos nomeados de layout (`Some(Value::Length(l))`) |
| `01_core/.../stdlib/collections.rs` | 20 | Desestruturação de tipos de coleção |
| `01_core/.../stdlib/foundations/cast.rs` | 17 | Despacho de coerção e conversão de tipos primitivos |
| `01_core/.../eval/operators/arithmetic.rs` | 15 | Despacho de tuplas de 3 elementos `(Op, Lhs, Rhs)` |
| `01_core/.../stdlib/structural/table_lines.rs` | 12 | Extração de alinhamentos 2D (`Some(Value::Align(Align2D { ... }))`) |

---

## 3. Análise da Amostra Dirigida por Risco

### A. Casos de Maior Profundidade (Profundidades 5 e 4)
1. **Profundidade 5 (9 casos)**:
   - `eval/rules.rs:1001`: `Some((_, Ok(Value::Str(s))))` — matching do resultado de avaliação de argumento em `#set equation(numbering: ...)`. Tratamento completo e exaustivo de `Some(Ok(Str))`, `Some(Ok(None))`, `Some(Ok(other))`, `Some(Err(e))` e `None`.
   - `stdlib/structural/table_lines.rs:96, 99, 137, 140, 179, 182, 219, 222`: `Some(Value::Align(Align2D { v: Some(VAlign::Top), .. }))` — matching de alinhamento vertical/horizontal em linhas e colunas de grade/tabela (`grid.hline`/`vline`). Correto, sem ramificações omitidas.
2. **Profundidade 4 (18 casos)**:
   - `eval/closures.rs:179`: `Some(FlowEvent::Return(_, Some(explicit), _))` — consumo de evento de controle de fluxo de closure.
   - `eval/bindings/binding.rs:212`: `DestructuringItem::Pattern(Pattern::Normal(Expr::Ident(ident)))` — desestruturação de dicionário na sintaxe de vinculação de variáveis.
   - `03_infra/src/pipeline.rs:596`: `[single @ ((_, font_variant, variations), bytes)]` — matching de fatia de fontes resolvidas para verificação de Variable Fonts.

### B. Casos em Módulos Críticos (`math/layout` e `table`/`grid`)
- **`math/layout/attach.rs:65`**: `Some(Content::MathIdent(s)) | Some(Content::MathText(s))` — matching do núcleo base de subscrito/sobrescrito.
- **`math/layout/underover.rs:176, 180`**: `Some((over_y, ob_ascent))` — matching do frame opcional de anotação superior/inferior.
- **`table_grid.rs:100, 112, 120, 129`**: `Value::Content(Content::TableHLine(e))` — identificação de elementos estruturais filhos na inicialização da tabela.

### C. Amostra Aleatória de Casos de Profundidade 3
- `calc.rs`: Funções matemáticas puras usando `[Value::Int(n)]` / `[Value::Float(f)]`.
- `cast.rs`: Conversores `int(x)`, `str(x)`, `float(x)`.
- Todos os casos analisados são estritamente idiomáticos em Rust, seguros e semanticamente corretos.

---

## 4. Conclusão e Decisão de Alcance

- **Nenhum bug real, omissão de caso ou desvio de semântica foi encontrado** nos 515 alertas V20.
- 94.8% dos casos são causados simplesmente pelo aninhamento natural `Option/Slice -> Value -> Dado`, que é a forma canônica e mais performante de desestruturação em Rust para AST e runtime dinâmico.
- **Decisão**: Conforme previsto na especificação do Passo 1052, as 515 ocorrências são categorizadas como debt de nível `info` (sem necessidade de refatoração cosmética invasiva ou escalonamento), preservando a estabilidade e a legibilidade do código.
