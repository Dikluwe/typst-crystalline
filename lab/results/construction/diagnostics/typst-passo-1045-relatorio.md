# Relatório Passo 1045 — V18 (2 Casos) e V16 Remanescente (Pós-P1041)

**Data**: 2026-08-14  
**Passo**: 1045  
**Status**: Concluído com Sucesso  
**Objetivo**:
1. Validar e auditar os 2 casos de padrão de range numérico (`V18`) fora de módulos de lexing/numeração.
2. Levantar a contagem real, auditar e classificar o inventário remanescente de wildcards catch-all (`V16`) pós-Passo 1041.
**Ferramentas e Verificações**: `crystalline-lint --checks v16,v18 .`, análise estática de código-fonte, conferência contra a especificação PDF ISO 32000-1 §7.3.4.2 e execução da suíte completa de testes (`cargo test --workspace`).

---

## 1. Resumo Executivo

| Regra / Escopo | Ocorrências Aferidas | Classificação Semântica | Status Passo 1045 |
| :--- | :---: | :--- | :---: |
| **V18 (Range fora de lexing/numeração)** | **2 casos** | Parsing e escaping de bytes conforme ISO 32000-1 (PDF) | **Validado (0 bugs / 100% paridade)** |
| **V16 (Wildcards em Testes)** | **68 casos** | Asserções e extrações pontuais em `tests.rs` / integração | **Auditado (sem impacto em produção)** |
| **V16 (Classe A: Projeções/Predicados)** | **84 casos** | Funções `is_*`, `as_*`, `to_*`, `partial_cmp` e extratores | **Conforme (projeções neutras legítimas)** |
| **V16 (Classe B: Hubs com Erro a Jusante)** | **22 casos** | Despacho de métodos/campos com emissão de erro a jusante | **Conforme (hubs de despacho estático)** |
| **V16 (Classe C: DENY / Silenciamento)** | **0 casos** | Mascaramento de variantes com descarte arbitrário | **Zero casos residuais (100% limpo)** |

---

## 2. Fase A — V18 (2 Casos de Padrão de Range)

O linter apontou 2 ocorrências de `V18` fora dos diretórios `lexer/` e `stdlib/numbering.rs`:

### 2.1. Caso 1: `03_infra/src/export/builder.rs:2416`
- **Código**:
  ```rust
  fn escape_pdf_literal(s: &str) -> String {
      let mut out = String::with_capacity(s.len() + 2);
      out.push('(');
      for b in s.bytes() {
          match b {
              b'(' => out.push_str("\("),
              b')' => out.push_str("\)"),
              b'\' => out.push_str("\\"),
              0x00..=0x1f | 0x7f => out.push_str(&format!("\{:03o}", b)),
              _ => out.push(b as char),
          }
      }
      out.push(')');
      out
  }
  ```
- **Análise Semântica e Paridade**:
  - Trata-se da rotina canônica de escape de strings literais PDF (delimitadas por parênteses `(...)`).
  - Segundo a especificação **ISO 32000-1 §7.3.4.2 (Literal Strings)**:
    1. Caracteres especiais `(`, `)`, `\` devem ser escapados com barra invertida.
    2. Todos os caracteres de controle ASCII (`0x00..=0x1f`) e o caractere DEL (`0x7f`) não são imprimíveis e devem ser representados como sequências octais de 3 dígitos (`\ddd`).
  - **Conformidade Vanilla**: O crate `pdf-writer` utilizado pelo Vanilla Typst aplica exatamente o mesmo intervalo de controle (`0x00..=0x1f | 0x7f`). O range é exato, sem off-by-one ou problemas de limite.

### 2.2. Caso 2: `03_infra/src/export/oracle.rs:66`
- **Código**:
  ```rust
  match bytes[i] {
      b'<' => { /* extrai hex part */ }
      b'0'..=b'9' | b'-' => {
          let start_n = i;
          if bytes[i] == b'-' { i += 1; }
          while i < bytes.len() && bytes[i].is_ascii_digit() { i += 1; }
          let num = std::str::from_utf8(&bytes[start_n..i]).ok()?;
          let value: i64 = num.parse().ok()?;
          if value != 0 { return None; }
      }
      b']' => { /* fecha array TJ */ }
      _ => { i += 1; }
  }
  ```
- **Análise Semântica e Paridade**:
  - Trata-se do micro-parser de streams PDF que extrai argumentos de operadores de posicionamento `TJ` (`[<hex> -kerning <hex>] TJ`).
  - O range `b'0'..=b'9' | b'-'` reconhece o início de literais numéricos com ou sem sinal negativo para verificar se há deslocamentos de kerning não-nulos.
  - Trata-se de operação pura de lexing/parsing de bytes dentro da ferramenta de auditoria visual de PDF (`oracle.rs`).

---

## 3. Fase 0 & B — Inventário Real e Classificação do V16 Remanescente

### 3.1. Contagem Real Aferida
- **Total de linhas reportadas**: 176
  - **2 avisos de exceções obsoletas** em `crystalline.toml` causadas por deslocamento de 1 linha pós-Passo 1042/1043 (`shaper.rs:501/2359` $ightarrow$ `502/2360`).
  - **174 ocorrências ativas de wildcard**.

### 3.2. Decomposição Estrutural das 174 Ocorrências Ativas
1. **Arquivos de Teste (68 ocorrências)**:
   - `01_core/src/compiler/eval/tests.rs`: 40 casos (asserções pontuais em cenários de teste, ex.: `let Content::Equation(e) = content else { panic!() }`).
   - `01_core/src/compiler/layout/tests.rs`: 19 casos.
   - `03_infra/src/integration_tests.rs`: 6 casos.
   - `01_core/src/compiler/math/layout/tests.rs`: 3 casos.
2. **Entidades e Hubs Centrais de AST (26 ocorrências em produção)**:
   - `entities/value.rs` (6 casos): métodos de conversão e extração de tipo (`to_str()`, `to_int()`, `type_name()`).
   - `entities/content.rs` (5 casos): predicados estruturais (`is_leaf()`, `has_label()`).
   - `entities/math_style.rs` (4 casos): projeção de variantes de estilo matemático.
   - `entities/ast/expr.rs` (3 casos): extratores sintáticos de expressões.
   - `entities/color.rs` (2 casos): conversão de espaços de cor.
   - `entities/operators.rs` (2 casos): mapeamento de precedência e aridade de operadores.
   - `entities/func.rs`, `entities/state_update.rs`, `entities/syntax_node.rs`, `entities/font_variations.rs` (1 caso cada).
3. **Compilador e Módulos Stdlib (70 ocorrências em produção)**:
   - `eval/math.rs` (5 casos): resolução de delimitadores e símbolos math (`parse_delim_char`).
   - `stdlib/layout.rs` (5 casos): helpers de alinhamento e dimensões.
   - `stdlib/shapes.rs` (4 casos): argumentos opcionais de polígonos/linhas.
   - `eval/mod.rs`, `layout/cursor.rs`, `layout/grid.rs`, `eval/bindings/field_access.rs`, etc.
4. **Infraestrutura e Exportação (10 ocorrências em produção)**:
   - `query_helpers.rs` (3 casos): contadores de nós auxiliares.
   - `font_metrics.rs` (2 casos): fallback de métricas de glifos desconhecidos.
   - `shaper.rs` (2 casos): classificação de itens não-textuais.
   - `export/stream.rs`, `layout_bidi.rs`, `pipeline.rs` (1 caso cada).

### 3.3. Classificação Semântica (Taxonomia P1041)
- **Classe A (Projeções Neutras e Predicados)**: **84 casos** (todos legítimos, retornando `None` ou `false` para variantes sem a propriedade consultada).
- **Classe B (Hubs de Despacho com Erro Tipado a Jusante)**: **22 casos** (despachos de métodos e campos onde a ausência de casamento desvia para `binary_mismatch` ou diagnóstico tipado).
- **Classe C (DENY / Mascaramento de Erro)**: **0 casos residuais**. Todos os 8 casos reais de DENY identificados na auditoria inicial foram definitivamente eliminados no Passo 1041.

---

## 4. Fase C — Validação

- `crystalline-lint --checks v16,v18 .`: todas as ocorrências mapeadas e classificadas.
- `cargo test --workspace`: **5.924 testes unitários e de integração aprovados (100%)**, com 0 falhas e 0 regressões.
