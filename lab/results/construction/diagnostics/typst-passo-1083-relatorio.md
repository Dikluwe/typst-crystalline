# Relatório de Execução — Passo 1083: Reclassificação N16[α/β/γ] — Lote 2 Completo (`stdlib/`, `eval/`, `math/`, `parse/`)

**Data**: 2026-08-18  
**Passo**: 1083 — Reclassificação N16[α/β/γ] — Lote 2 Completo (`stdlib/`, `eval/`, `math/`, `parse/`)  
**Gate**: `ADR-0127` (Classificação / Manutenibilidade de Código: Anotações de Taxonomia N16 sem alteração de semântica de execução)  
**Status**: CONCLUÍDO COM ÊXITO (100% dos 28 casos do Lote 2 auditados e anotados individualmente; 92/92 casos globais do repositório 100% tageados com conformidade formal)

---

## 1. Contexto e Motivação

O Passo 1083 finaliza o ciclo de formalização da taxonomia `N16[α/β/γ]` (iniciado no P1070 e auditado no P1081), cobrindo o **Lote 2 Completo**:
- **`stdlib/`**: 19 ocorrências (métodos de coleção, coerções numéricas, formas, transformações, figuras e bibliografia).
- **`eval/`**: 6 ocorrências (matrizes matemáticas, delimitadores, regras `#set` e fontes).
- **`math/`**: 2 ocorrências (`math/layout/mod.rs` e `math/layout/attach.rs`).
- **`parse/`**: 1 ocorrência (`parse/math.rs`).

---

## 2. Auditoria e Classificação Detalhada dos 28 Casos

A análise caso a caso resultou em **27 casos `N16[β]`** e **1 caso `N16[γ]`** (`math/layout/mod.rs:279`):

| # | Arquivo e Linha | Escopo | Trecho do Braço | Classe | Justificativa Semântica Individual |
| :- | :--- | :---: | :--- | :---: | :--- |
| 1 | `01_core/src/compiler/eval/math.rs:979` | `eval` | `_ => {}` | **`N16[β]`** | Varredura de argumentos de `vec()`: argumentos nomeados que não afetam a matriz são ignorados uniformemente no loop. |
| 2 | `01_core/src/compiler/eval/math.rs:1034` | `eval` | `_ => {}` | **`N16[β]`** | Varredura de argumentos de `lr()`: argumentos não posicionais/não-delim são ignorados uniformemente. |
| 3 | `01_core/src/compiler/eval/math.rs:1310` | `eval` | `_ => None` | **`N16[β]`** | Projeção de delimitador: tipos que não são `Str`, `None` ou `Array` retornam uniformemente `None` em `parse_delim_val`. |
| 4 | `01_core/src/compiler/eval/math.rs:1318` | `eval` | `_ => None` | **`N16[β]`** | Projeção de caractere delimitador: tipos não textuais retornam uniformemente `None` em `parse_delim_char`. |
| 5 | `01_core/src/compiler/eval/rules.rs:1014` | `eval` | `None => {}` | **`N16[β]`** | Argumento opcional omitido: quando `numbering` não é fornecido na `#set` rule, os estilos permanecem inalterados por contrato. |
| 6 | `01_core/src/compiler/eval/rules.rs:1693` | `eval` | `_ => None` | **`N16[β]`** | Filtro de famílias tipográficas: itens de array que não sejam nomes válidos de fonte retornam `None`. |
| 7 | `01_core/src/compiler/math/layout/mod.rs:279` | `math` | `_ => false` | **`N16[γ]`** | Fallback aberto sob evolução de AST: `needs_external_layout` não possui proteção dinâmica; variantes novas de container não listadas cairiam silenciosamente em `plain_text()` em vez de `layout_external`. |
| 8 | `01_core/src/compiler/math/layout/attach.rs:157` | `math` | `_ => false` | **`N16[β]`** | Predicado estrutural: nós que não sejam `MathOp` ou identificadores de operadores matemáticos não são large-ops. |
| 9 | `01_core/src/compiler/parse/math.rs:207` | `parse` | `_ => return None` | **`N16[β]`** | Projeção de token: tokens de sintaxe que não são operadores de expressão matemática retornam `None`. |
| 10 | `01_core/src/compiler/stdlib/collections.rs:115` | `stdlib` | `_ => None` | **`N16[β]`** | Projeção de métodos nativos: métodos não pertencentes à coleção retornam `None` no dispatch dinâmico de métodos. |
| 11 | `01_core/src/compiler/stdlib/collections.rs:1176` | `stdlib` | `_ => return None` | **`N16[β]`** | Projeção de tipos chamáveis: tipos sem construtor chamável associado retornam `None` em `type_as_callable`. |
| 12 | `01_core/src/compiler/stdlib/layout.rs:83` | `stdlib` | `_ => 0.0` | **`N16[β]`** | Coerção numérica pura: valores não dimensionais retornam `0.0` na extração de pontos para layout. |
| 13 | `01_core/src/compiler/stdlib/layout.rs:189` | `stdlib` | `_ => None` | **`N16[β]`** | Projeção de tipo: argumentos que não casam com `Value::Align` nem `Value::Str` retornam `None` na extração de alinhamento. |
| 14 | `01_core/src/compiler/stdlib/layout.rs:203` | `stdlib` | `_ => None` | **`N16[β]`** | Projeção de tipo: valores não dimensionais/não-fração retornam `None` no parse de dimensionamento de track (`parse_track_sizing`). |
| 15 | `01_core/src/compiler/stdlib/layout.rs:217` | `stdlib` | `None => vec![]` | **`N16[β]`** | Parâmetro opcional ausente: quando `columns` ou `rows` não são passados em `grid()`, retorna lista vazia para derivação automática. |
| 16 | `01_core/src/compiler/stdlib/layout.rs:452` | `stdlib` | `_ => None` | **`N16[β]`** | Projeção dimensional: valores que não possuem comprimento mensurável retornam `None` em `extract_length`. |
| 17 | `01_core/src/compiler/stdlib/shapes.rs:54` | `stdlib` | `_ => None` | **`N16[β]`** | Projeção de cor: valores que não representam cores válidas ou strings nomeadas retornam `None` no parse de cor. |
| 18 | `01_core/src/compiler/stdlib/shapes.rs:67` | `stdlib` | `_ => None` | **`N16[β]`** | Projeção de preenchimento: valores que não representam tinta (`Paint`) retornam `None` em `parse_paint`. |
| 19 | `01_core/src/compiler/stdlib/shapes.rs:275` | `stdlib` | `_ => 0.0` | **`N16[β]`** | Coerção de dimensão: valores não numéricos retornam `0.0` na extração de raio/dimensões de polígono. |
| 20 | `01_core/src/compiler/stdlib/shapes.rs:339` | `stdlib` | `_ => 0.0` | **`N16[β]`** | Coerção de dimensão: valores não numéricos retornam `0.0` na extração de coordenadas de linha. |
| 21 | `01_core/src/compiler/stdlib/transforms.rs:29` | `stdlib` | `_ => 0.0` | **`N16[β]`** | Coerção de deslocamento: valores não numéricos retornam `0.0` na extração de `dx`/`dy` de transformação. |
| 22 | `01_core/src/compiler/stdlib/transforms.rs:199` | `stdlib` | `_ => None` | **`N16[β]`** | Projeção angular: valores não angulares retornam `None` na extração de radianos em rotações. |
| 23 | `01_core/src/compiler/stdlib/calc.rs:176` | `stdlib` | `_ => false` | **`N16[β]`** | Predicado numérico: tipos não numéricos retornam `false` no predicado `base_zero` de exponenciação. |
| 24 | `01_core/src/compiler/stdlib/calc.rs:181` | `stdlib` | `_ => false` | **`N16[β]`** | Predicado numérico: tipos não numéricos retornam `false` no predicado `exp_zero` de exponenciação. |
| 25 | `01_core/src/compiler/stdlib/figure_image.rs:39` | `stdlib` | `_ => None` | **`N16[β]`** | Projeção de tipo de figura: corpos que não representam imagens, tabelas ou código retornam `None` na inferência de tipo. |
| 26 | `01_core/src/compiler/stdlib/figure_image.rs:94` | `stdlib` | `_ => None` | **`N16[β]`** | Projeção de kind: valores inválidos de `kind` retornam `None` para acionar a inferência a partir do corpo. |
| 27 | `01_core/src/compiler/stdlib/structural/bibliography.rs:326` | `stdlib` | `_ => None` | **`N16[β]`** | Projeção de locale: valores que não sejam strings retornam `None` no campo de locale da bibliografia. |
| 28 | `01_core/src/compiler/stdlib/foundations/color.rs:161` | `stdlib` | `_ => None` | **`N16[β]`** | Projeção de componente cromático: tipos não numéricos retornam `None` na extração de canais de cor. |

---

## 3. Análise Detalhada dos Casos Críticos e Reclassificação de `math/layout/mod.rs:279`

1. **`math/layout/mod.rs:279` (`_ => false` em `needs_external_layout`) — Reclassificado como `N16[γ]`**:
   - *Análise de fluxo*: `needs_external_layout` decide se um nó `Content` embutido em math deve ser despachado para `layout_external(other, style)` (`true`) ou se entra na rota de texto corrida `other.plain_text()` (`false`, linhas 733-740).
   - *Risco de evolução de AST*: Ao contrário de `get_field` (onde nós dinâmicos delegam via `dyn_get_field`), `needs_external_layout` lista manualmente apenas 5 nós (`Equation`, `Boxed`, `Align`, `Pad`, `Block`). Se uma nova variante de container/forma (ex.: `Content::Canvas`, `Content::Table`, `Content::Rotate`) for adicionada à AST, ela cairá no `_ => false`, sendo incorretamente achatada por `plain_text()` em vez de passar por `layout_external`.
   - *Conclusão*: Trata-se de um fallback aberto direto sobre a evolução de `Content`, análogo a `layout/mod.rs:2006` (`_ => (0.0, 0.0)` no P1080). Reclassificado para **`N16[γ]`** (vigilância contínua sob evolução da AST).

2. **`stdlib/layout.rs:217` (`None => vec![]`)**:
   - *Análise*: Trata o argumento opcional `columns` ou `rows` em `grid()`. Quando o argumento não é fornecido na chamada (`None`), a grade não possui tracks pré-alocadas (`vec![]`), delegando a derivação para as células de conteúdo. Trata-se de comportamento uniforme por contrato de argumento opcional, e não de fallback aberto. Confirmado **`N16[β]`**.

3. **`eval/rules.rs:1014` (`None => {}`)**:
   - *Análise*: Trata a regra `#set math.equation()`. Quando o argumento `numbering` não é especificado (`None`), o style chain não é mutado, preservando a numeração corrente. Trata-se de no-op uniforme por contrato de set-rule parcial. Confirmado **`N16[β]`**.

4. **`math/layout/attach.rs:157` (`_ => false` em large-ops)**:
   - *Análise*: Identifica uniformemente se o nó base é um operador matemático com limites. Predicado booleano sobre AST. Confirmado **`N16[β]`**.

---

## 4. Reconciliação Global Final do Repositório (92/92 Casos)

Com a conclusão do Passo 1083, o levantamento exaustivo do repositório Crystalline atinge **100% de conformidade com a taxonomia N16**:

| Lote | Domínio do Código | Total de Casos | Distribuição das Classes | Passo de Execução | Estado Final |
| :--- | :--- | :---: | :---: | :---: | :---: |
| **Lote 3** | `introspect/`, `layout/`, `export/` e infra | **36** | 2 α, 27 β, 7 γ | Passo 1080 | **100% Concluído** |
| **Lote 1** | `01_core/src/entities/` | **28** | 28 β | Passo 1082 | **100% Concluído** |
| **Lote 2** | `stdlib/`, `eval/`, `math/`, `parse/` | **28** | 27 β, 1 γ | Passo 1083 | **100% Concluído** |
| **Total Global** | **Todo o repositório Crystalline** | **92** | **2 α (2.2%), 82 β (89.1%), 8 γ (8.7%)** | — | **100% Concluído (0 Pendências)** |

---

## 5. Validação e Qualidade Final

1. **Cobertura Global**: 92/92 casos (100%) tageados com a taxonomia formal `N16[α/β/γ]`. Zero comentários `// neutro:` em formato legado.
2. **`crystalline-lint --checks v16 .`**: 100% das tags validadas com sucesso.
3. **`cargo test --workspace`**: **5.951 testes aprovados (100% PASS)** em todos os 6 targets do workspace (0 falhas).
4. **`crystalline-lint .`**: **0 erros**.
