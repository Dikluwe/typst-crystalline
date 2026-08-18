# Relatório de Auditoria — Passo 1081: Auditoria de Estado dos Lotes 1 e 2 da Reclassificação N16[α/β/γ]

**Data**: 2026-08-18  
**Passo**: 1081 — Auditoria de Estado (Lotes 1 e 2 da Reclassificação N16)  
**Gate**: Nenhum (Auditoria puramente factual e de levantamento de inventário — sem alteração de código)  
**Status**: CONCLUÍDO (Inventário exato de 100% dos arquivos do repositório concluído)

---

## 1. Contexto e Motivação

No Passo 1070, a taxonomia `N16[α/β/γ]` foi desenhada e dividida em três lotes para reclassificação dos comentários `// neutro:` no Crystalline:
- **Lote 1**: `01_core/src/entities/` (estimado em ~26 casos).
- **Lote 2**: `01_core/src/compiler/stdlib/` e `eval/` (estimado em ~30 casos).
- **Lote 3**: `introspect/`, `layout/`, `export/` e infraestrutura (36 casos — 100% concluído no Passo 1080).

O Passo 1081 realizou a varredura exaustiva do repositório para determinar o estado real de implementação das anotações nos Lotes 1 e 2.

---

## 2. Reconciliação dos Números e Estado Atual

A contagem factual via script no repositório revelou a seguinte distribuição exata:

| Domínio / Lote | Escopo de Diretórios | Total de `// neutro:` | Com Tag `N16[...]` | Sem Tag (Formato Antigo) | Estado (§3 do L0) |
| :--- | :--- | :---: | :---: | :---: | :---: |
| **Lote 1** | `01_core/src/entities/` | **28** | 0 | 28 | **0% tageado** (Pendente) |
| **Lote 2** | `01_core/src/compiler/{stdlib, eval, math, parse}/` | **28** | 0 | 28 | **0% tageado** (Pendente) |
| **Lote 3** | `01_core/src/compiler/{introspect, layout}/` + `03_infra/` | **36** | 36 | 0 | **100% tageado** (Fechado no P1080) |
| **Total Global** | Todo o repositório Crystalline | **92** | **36** (39.1%) | **56** (60.9%) | — |

---

## 3. Inventário Detalhado dos Casos Pendentes

### 3.1 Lote 1 — `01_core/src/entities/` (28 Casos Pendentes)
Predominantemente composto por predicados booleanos (`is_*`), projeções de variantes e conversões de tipo:
1. `entities/func.rs:363`: desigualdade de representações de função (`_other => false`)
2. `entities/syntax_node.rs:199`: predicado `is_block` (`_ => false`)
3. `entities/value.rs:395`: verdade lógica padrão de valores (`_other => true`)
4. `entities/value.rs:403`: projeção `as_bool` (`_other => None`)
5. `entities/value.rs:420`: projeção `cast_float` (`_ => None`)
6. `entities/value.rs:474`: projeção `cast_decimal` (`_other => None`)
7. `entities/value.rs:495`: projeção `cast_duration` (`_other => None`)
8. `entities/value.rs:506`: projeção `cast_version` (`_other => None`)
9. `entities/value.rs:517`: projeção `cast_regex` (`_other => None`)
10. `entities/font_variations.rs:98`: filtro de tags inválidas em dict (`_ => continue`)
11. `entities/math_style.rs:127`: estilo de símbolo math (`_other => None`)
12. `entities/math_style.rs:161`: estilo de símbolo math (`_other => None`)
13. `entities/math_style.rs:202`: estilo de símbolo math (`_other => None`)
14. `entities/math_style.rs:213`: estilo de símbolo math (`_other => None`)
15. `entities/math_style.rs:247`: estilo de símbolo math (`_other => None`)
16. `entities/content.rs:2775`: predicado `is_empty` (`_ => false`)
17. `entities/content.rs:3100`: `PartialEq` de variantes sem arm dedicado (`_ => false`)
18. `entities/content.rs:3145`: `get_field` dinâmico (`_ => None`)
19. `entities/content.rs:3403`: `morph_canon` de nós sem canonicalização (`_ => None`)
20. `entities/content.rs:3958`: callback de `map_content` (`_ => Ok(None)`)
21. `entities/operators.rs:27`: precedência de operadores (`_ => return None`)
22. `entities/operators.rs:95`: aridade de operadores (`_ => return None`)
23. `entities/state_update.rs:58`: igualdade de operações de estado (`_other => false`)
24. `entities/color.rs:137`: desigualdade entre espaços de cor distintos (`_other => false`)
25. `entities/color.rs:669`: correção angular de matiz (`_other => None`)
26. `entities/ast/expr.rs:603`: operador unário `UnOp::from_kind` (`_ => return Option::None`)
27. `entities/ast/expr.rs:693`: operador binário `BinOp::from_kind` (`_ => return Option::None`)
28. `entities/ast/expr.rs:910`: extração de bindings em `Pattern` (`_ => vec![]`)

---

### 3.2 Lote 2 — `01_core/src/compiler/{stdlib, eval, math, parse}/` (28 Casos Pendentes)
Composto por coerções numéricas, extração de argumentos da stdlib e despacho de regras:
1. `stdlib/collections.rs:115`: projeção de indexação em coleção (`_ => None`)
2. `stdlib/collections.rs:1176`: projeção de iterabilidade (`_ => return None`)
3. `stdlib/layout.rs:83`: extração de pontos em layout (`_ => 0.0`)
4. `stdlib/layout.rs:189`: projeção de body em layout (`_ => None`)
5. `stdlib/layout.rs:203`: projeção de alinhamento (`_ => None`)
6. `stdlib/layout.rs:217`: projeção de colunas em grid/layout (`None => vec![]`)
7. `stdlib/layout.rs:452`: fallback de largura (`_ => None`)
8. `stdlib/shapes.rs:54`: coerção de cor em formas (`_ => None`)
9. `stdlib/shapes.rs:67`: coerção de fill em formas (`_ => None`)
10. `stdlib/shapes.rs:275`: extração de dimensão de polígono (`_ => 0.0`)
11. `stdlib/shapes.rs:339`: extração de dimensão de linha (`_ => 0.0`)
12. `stdlib/transforms.rs:29`: extração de deslocamento (`_ => 0.0`)
13. `stdlib/transforms.rs:199`: extração de ângulo (`_ => None`)
14. `stdlib/calc.rs:176`: predicado `is_nan` (`_ => false`)
15. `stdlib/calc.rs:181`: predicado `is_infinite` (`_ => false`)
16. `stdlib/figure_image.rs:39`: dimensão de imagem (`_ => None`)
17. `stdlib/figure_image.rs:94`: extração de caption (`_ => None`)
18. `stdlib/structural/bibliography.rs:326`: locale de bibliografia (`_ => None`)
19. `stdlib/foundations/color.rs:161`: coerção de cor (`_ => None`)
20. `eval/math.rs:979`: argumentos posicionais em `vec()` (`_ => {}`)
21. `eval/math.rs:1034`: argumentos posicionais em `lr()` (`_ => {}`)
22. `eval/math.rs:1310`: projeção de delimitador em matriz (`_ => None`)
23. `eval/math.rs:1318`: projeção de delimitador caractere (`_ => None`)
24. `eval/rules.rs:1014`: argumento numbering omitido em set-rule (`None => {}`)
25. `eval/rules.rs:1693`: filtro de fontes não-nominais (`_ => None`)
26. `math/layout/mod.rs:279`: tratamento especial de layout math (`_ => false`)
27. `math/layout/attach.rs:157`: predicado large-ops em math (`_ => false`)
28. `parse/math.rs:207`: parse de identificadores em math (`_ => return None`)

---

## 4. Conclusão e Próximos Passos

1. **Estado Factual**:
   - O **Lote 3** (o único que exigia auditoria semântica complexa devido a riscos de AST aberta no layout e exportação) está **100% concluído e verificado**.
   - Os **Lotes 1 e 2** encontram-se **0% tageados** (total de 56 ocorrências remanescentes no formato antigo).
2. **Recomendação de Sequenciamento**:
   - Executar a anotação assistida do **Lote 1 (Passo 1082)**: 28 casos em `entities/` (quase 100% mecânicos `N16[β]`).
   - Executar a anotação assistida do **Lote 2 (Passo 1083)**: 28 casos em `stdlib/`, `eval/`, `math/` e `parse/`.
   - Ao final, 100% dos 92 comentários `// neutro:` do repositório estarão em conformidade formal com `N16[α/β/γ]`.
