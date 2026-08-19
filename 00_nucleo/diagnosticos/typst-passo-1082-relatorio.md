# Relatório de Execução — Passo 1082: Reclassificação N16[α/β/γ] — Lote 1 (`entities/`)

**Data**: 2026-08-18  
**Passo**: 1082 — Reclassificação N16[α/β/γ] — Lote 1 (`entities/`)  
**Gate**: `ADR-0127` (Classificação / Manutenibilidade de Código: Anotações de Taxonomia N16 sem alteração de semântica de execução)  
**Status**: CONCLUÍDO COM ÊXITO (100% dos 28 casos do Lote 1 auditados, classificados individualmente e anotados com tag formal `N16[β]`)

---

## 1. Contexto e Motivação

O Passo 1070 estabeleceu a taxonomia formal de wildcards `N16[α/β/γ]` per `ADR-0017` / `ADR-0127`:
- **`N16[α]`**: Impossibilidade Estrutural / Fechamento / Despacho Modular.
- **`N16[β]`**: Comportamento Uniforme Genuíno por Contrato (projeções de tipo, predicados estruturais, desigualdade entre variantes heterogêneas).
- **`N16[γ]`**: Fallback Deliberado Aberto / Vigilância Ativa.

No inventário consolidado pelo Passo 1081, o **Lote 1 (`01_core/src/entities/`)** foi identificado com exatamente **28 ocorrências** de comentários `// neutro:`.

---

## 2. Auditoria e Classificação Detalhada dos 28 Casos

Dos 28 casos inventariados em `01_core/src/entities/`, **27 residem em código de produção** e **1 reside em módulo de teste** (`content.rs:3958`, em `#[cfg(test)]`). Todos os 28 foram analisados individualmente:

| # | Arquivo e Linha | Escopo | Trecho do Braço | Classe | Justificativa Semântica Individual |
| :- | :--- | :---: | :--- | :---: | :--- |
| 1 | `01_core/src/entities/ast/expr.rs:603` | Produção | `_ => return Option::None` | **`N16[β]`** | Projeção de token: `SyntaxKind` não-unário retorna uniformemente `None` em `UnOp::from_kind`. |
| 2 | `01_core/src/entities/ast/expr.rs:693` | Produção | `_ => return Option::None` | **`N16[β]`** | Projeção de token: `SyntaxKind` não-binário retorna uniformemente `None` em `BinOp::from_kind`. |
| 3 | `01_core/src/entities/ast/expr.rs:910` | Produção | `_ => vec![]` | **`N16[β]`** | Extração estrutural de bindings: nós de `Pattern` sem introdução de variáveis retornam uniformemente `vec![]`. |
| 4 | `01_core/src/entities/func.rs:363` | Produção | `_other => false` | **`N16[β]`** | Desigualdade de representações: instâncias de `Func` com variantes internas distintas de `FuncRepr` são estritamente desiguais. |
| 5 | `01_core/src/entities/syntax_node.rs:199` | Produção | `_ => false` | **`N16[β]`** | Desigualdade de nós de sintaxe: tipos heterogêneos (`Leaf` vs `Inner`, etc.) avaliam como desiguais em `spanless_eq`. |
| 6 | `01_core/src/entities/value.rs:395` | Produção | `_other => true` | **`N16[β]`** | Predicado de truthiness: tipos de valor sem conceito semântico de vazio avaliam uniformemente como truthy por especificação. |
| 7 | `01_core/src/entities/value.rs:403` | Produção | `_other => None` | **`N16[β]`** | Projeção de tipo pura (`cast_bool`): todas as variantes não-Bool retornam uniformemente `None`. |
| 8 | `01_core/src/entities/value.rs:420` | Produção | `_ => None` | **`N16[β]`** | Coerção numérica pura (`cast_float`): todas as variantes não numéricas (`!Float` e `!Int`) retornam `None`. |
| 9 | `01_core/src/entities/value.rs:474` | Produção | `_other => None` | **`N16[β]`** | Projeção de tipo (`cast_decimal`): todas as variantes heterogêneas incompatíveis com `Decimal` retornam `None`. |
| 10 | `01_core/src/entities/value.rs:495` | Produção | `_other => None` | **`N16[β]`** | Projeção de tipo (`cast_duration`): todas as variantes incompatíveis com `Duration` retornam `None`. |
| 11 | `01_core/src/entities/value.rs:506` | Produção | `_other => None` | **`N16[β]`** | Projeção de tipo (`cast_version`): todas as variantes incompatíveis com `Version` retornam `None`. |
| 12 | `01_core/src/entities/value.rs:517` | Produção | `_other => None` | **`N16[β]`** | Projeção de tipo (`cast_regex`): todas as variantes incompatíveis com `Regex` retornam `None`. |
| 13 | `01_core/src/entities/font_variations.rs:98` | Produção | `_ => continue` | **`N16[β]`** | Filtro de eixos de variação: entradas de dicionário com tipos não numéricos (`!Int`, `!Float`) são ignoradas uniformemente sem afetar as demais. |
| 14 | `01_core/src/entities/math_style.rs:127` | Produção | `_other => None` | **`N16[β]`** | Projeção de glifo: variantes de `MathStyleKind` sem Variation Selector (VS1/VS2) retornam uniformemente `None`. |
| 15 | `01_core/src/entities/math_style.rs:161` | Produção | `_other => None` | **`N16[β]`** | Mapeamento esparso: caracteres fora do conjunto de formas especiais de símbolos gregos não possuem mapeamento itálico no bloco Plain. |
| 16 | `01_core/src/entities/math_style.rs:202` | Produção | `_other => None` | **`N16[β]`** | Projeção de base Unicode: estilos matemáticos sem base alfabética direta contígua retornam uniformemente `None`. |
| 17 | `01_core/src/entities/math_style.rs:213` | Produção | `_other => None` | **`N16[β]`** | Projeção de base Unicode: estilos matemáticos que não afetam dígitos retornam uniformemente `None`. |
| 18 | `01_core/src/entities/math_style.rs:247` | Produção | `_other => None` | **`N16[β]`** | Tabela esparsa de exceções BMP: combinações glifo/estilo sem exceção de codepoint retornam `None`. |
| 19 | `01_core/src/entities/content.rs:2775` | Produção | `_ => false` | **`N16[β]`** | Predicado estrutural (`is_empty`): nós de `Content` estruturais não explicitamente nulos avaliam como não-vazios. |
| 20 | `01_core/src/entities/content.rs:3100` | Produção | `_ => false` | **`N16[β]`** | Desigualdade de variantes em `PartialEq`: variantes distintas ou sem comparador dedicado avaliam uniformemente como desiguais (`false`). |
| 21 | `01_core/src/entities/content.rs:3145` | Produção | `_ => None` | **`N16[β]`** | Projeção dinâmica de campos (`get_field`): nós sem campos expostos para regras de show retornam uniformemente `None`. |
| 22 | `01_core/src/entities/content.rs:3403` | Produção | `_ => None` | **`N16[β]`** | Canonicalização morfológica (`morph_canon`): nós sem regra especial de reescrita retornam `None` (preservação do nó original). |
| 23 | `01_core/src/entities/content.rs:3958` | Teste | `_ => Ok(None)` | **`N16[β]`** | Preservação em callback de teste: nós não-alvo em teste de `map_content` são preservados sem transformação (`Ok(None)`). |
| 24 | `01_core/src/entities/operators.rs:27` | Produção | `_ => return None` | **`N16[β]`** | Projeção de parser: tokens de sintaxe sem semântica de operador unário retornam `None`. |
| 25 | `01_core/src/entities/operators.rs:95` | Produção | `_ => return None` | **`N16[β]`** | Projeção de parser: tokens de sintaxe sem semântica de operador binário retornam `None`. |
| 26 | `01_core/src/entities/state_update.rs:58` | Produção | `_other => false` | **`N16[β]`** | Desigualdade em `PartialEq`: tipos distintos de atualização de estado (`Set` vs `Func`) são estritamente desiguais. |
| 27 | `01_core/src/entities/color.rs:137` | Produção | `_other => false` | **`N16[β]`** | Desigualdade em `PartialEq`: cores em espaços cromáticos distintos (ex.: `Rgb` vs `Cmyk`) são estritamente desiguais. |
| 28 | `01_core/src/entities/color.rs:669` | Produção | `_other => None` | **`N16[β]`** | Projeção cromática: espaços sem componente angular/hue retornam uniformemente `None` no cálculo de interpolação circular. |

---

## 3. Análise Comparativa e Rejeição Fundamentada de α e γ

Para garantir que a classificação não foi homogeneizada acriticamente, analisamos detalhadamente os pontos onde classes alternativas (α ou γ) poderiam ser cogitadas:

1. **`01_core/src/entities/ast/expr.rs:603` e `operators.rs:27` (`UnOp::from_kind`) — Hipótese α**:
   - *Por que cogitar α?* Poderia argumentar-se que a gramática Typst fecha estruturalmente os operadores unários em `+`, `-`, `not`, sendo outros tokens impossíveis de operarem como unários.
   - *Razão da rejeição de α em favor de β*: `SyntaxKind` é um enum léxico global aberto a toda a linguagem. A função `from_kind` atua como um discriminador/projeção que seleciona um subconjunto de tokens e retorna `None` para os demais. Sob a taxonomia `ADR-0017`, conversões de um superconjunto léxico para `Option<Subtipo>` são categorizadas como projeções de tipo uniformes (`N16[β]`).

2. **`01_core/src/entities/color.rs:669` (`mix_in_space` -> `hue_idx`) — Hipótese α**:
   - *Por que cogitar α?* OKLCH, HSL e HSV são os únicos espaços cilíndricos com coordenada angular de matiz no modelo de cores.
   - *Razão da rejeição de α em favor de β*: Não há garantia estrutural estática de fechamento de `ColorSpace` (novos modelos como HWB poderiam ser adicionados). A função não despacha um subsistema fechado (como em `export/stream.rs`), mas projeta uniformemente a ausência de índice de matiz em espaços cartesianos (`None`). Logo, é `N16[β]`.

3. **`01_core/src/entities/content.rs:3145` (`get_field`) — Hipótese γ**:
   - *Por que cogitar γ?* `Content` é uma AST aberta; novos elementos podem conter campos consultáveis por regras de `#show`. Poder-se-ia temer que `_ => None` silenciasse a adição de campos em novos nós.
   - *Razão da rejeição de γ em favor de β*: Na arquitetura Crystalline, elementos dinâmicos delegam campos via `dyn_get_field` (`Content::Dynamic(e) => e.dyn_get_field(f)`). O wildcard é o comportamento uniforme legítimo de runtime para nós estáticos que não possuem o campo solicitado. Não se trata de fallback heurístico de renderização (que seria γ), e sim de consulta de reflexão (`None` para campo inexistente). Logo, `N16[β]`.

4. **`01_core/src/entities/font_variations.rs:98` (`_ => continue`) — Hipótese γ**:
   - *Por que cogitar γ?* Trata-se de um controle de fluxo `continue` dentro de loop de dicionário.
   - *Razão da rejeição de γ em favor de β*: Os únicos valores que representam coordenadas numéricas de variação de fonte são `Value::Int` e `Value::Float`. Todas as demais variantes de `Value` (como `Str`, `Array`, `Dict`, etc.) são tipos espúrios que não compõem eixos de fonte, sendo filtradas uniformemente pelo predicado. Logo, `N16[β]`.

5. **`01_core/src/entities/content.rs:3958` (`map_content` em teste) — Enquadramento de Escopo**:
   - *Análise*: Esta ocorrência reside no teste unitário `test_map_content_bottom_up` em `#[cfg(test)]`. Foi incluída no inventário inicial de 92 casos porque continha o comentário literal legado `// neutro:`. Foi mantida com anotação formal `N16[β]` para manter a coerência textual e do linter a 100%, registrando-se que representa o único caso de teste dentro do Lote 1.

---

## 4. Reconciliação Factual da Suíte de Testes (5.951 Testes)

A verificação completa via `cargo test --workspace` confirmou a aprovação de **5.951 testes (100% PASS)** em todos os crates do workspace:

- **`typst-core` (unittests)**: 5.073 passados
- **`typst-infra` (integration & unittests)**: 796 passados
- **`typst-shell` (unittests)**: 41 passados
- **`typst` (main binary tests)**: 2 passados
- **`cli` (integration tests)**: 37 passados
- **`crystalline-lint` (tests)**: 2 passados
- **Total Workspace**: **5.951 testes passados, 0 falhas, 3 ignorados (doc-tests intencionais)**.

*(Nota: o número preliminar de 878 citado anteriormente decorreu de truncamento do buffer no crate `typst-core`, já devidamente corrigido e reconciliado).*

---

## 5. Verificação e Conformidade Final

1. **Lote 1 Tageado**: 28/28 casos (100%) com tag formal `N16[β]`.
2. **`crystalline-lint --checks v16 .`**: 100% das tags validadas com sucesso.
3. **`cargo test --workspace`**: **5.951 testes aprovados (100% PASS)**.
4. **`crystalline-lint .`**: **0 erros**.
