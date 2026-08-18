# Relatório de Execução — Passo 1080: Reclassificação N16[α/β/γ] — Lote 3 (`introspect/`, `layout/`, `export/`)

**Data**: 2026-08-18  
**Passo**: 1080 — Reclassificação N16[α/β/γ] — Lote 3 (`introspect/`, `layout/`, `export/`)  
**Gate**: `ADR-0127` (Classificação / Manutenibilidade de Código: Anotações de Taxonomia N16 sem alteração de semântica de execução)  
**Status**: CONCLUÍDO COM ÊXITO (100% dos 36 casos do Lote 3 auditados e anotados individualmente)

---

## 1. Contexto e Motivação

No Passo 1070, foi estabelecida a taxonomia de saturação de wildcards `N16[α/β/γ]` (ADR-0017 / ADR-0127) para classificar todos os braços catch-all (`_ =>`) do compilador Crystalline em três categorias fundamentadas:
- **`N16[α]` (Impossibilidade Estrutural / Fechamento / Despacho Modular)**: Casos em que o domínio é exaurido ou os nós restantes são processados exclusivamente por despachantes especializados à parte.
- **`N16[β]` (Comportamento Uniforme por Contrato)**: Projeções puras de campos, predicados booleanos (`is_*`) ou extrações onde tipos sem a propriedade retornam uniformemente `None` ou valor neutro.
- **`N16[γ]` (Fallback Deliberado Aberto / Vigilância Ativa)**: Fallbacks de layout ou AST aberta onde a adição de novas variantes exige revisão explícita para evitar supressão silenciosa de comportamento.

Enquanto os Lotes 1 (`entities/`) e 2 (`stdlib/`/`eval/`) são predominantemente constituídos de projeções mecânicas (`N16[β]`), o **Lote 3 (`introspect/`, `layout/`, `export/`)** foi identificado no P1070 como o único conjunto exigindo **auditoria manual caso a caso a 100%**.

---

## 2. Inventário e Auditoria Completa do Lote 3 (36/36 Casos)

O levantamento exato identificou **36 ocorrências** de comentários `// neutro:` no Lote 3. Todos os 36 foram analisados no seu contexto semântico e anotados com justificativa individual:

| # | Arquivo e Linha | Trecho do Braço | Classe | Justificativa Semântica Individual |
| :--- | :--- | :--- | :---: | :--- |
| 1 | `01_core/src/compiler/introspect/extract_payload.rs:94` | `_ => None` | **`N16[γ]`** | Fallback aberto de AST: variantes não-locatables em M1 retornam `None`; novos elementos locatables futuros exigirão arm explícito. |
| 2 | `01_core/src/compiler/introspect/labelled.rs:73` | `_ => (None, None)` | **`N16[γ]`** | Fallback de rotulagem: `Content` sem suporte de labelling retorna `(None, None)`; novos nós rotuláveis futuros exigirão arm. |
| 3 | `01_core/src/compiler/introspect.rs:124` | `_ => {}` | **`N16[β]`** | Varredura estática: variantes terminais/folha (sem filhos) não contribuem chaves a indexar. |
| 4 | `01_core/src/compiler/layout/columns.rs:95` | `_ => None` | **`N16[β]`** | Projeção de propriedade: `Content` sem direção textual explícita retorna `None`. |
| 5 | `01_core/src/compiler/layout/columns.rs:123` | `_ => true` | **`N16[β]`** | Predicado de ocupação: `Content` estrutural não-vazio presume visibilidade em colunas. |
| 6 | `01_core/src/compiler/layout/image.rs:133` | `_ => None` | **`N16[β]`** | Projeção de tipo: `Value` não-dimensionável retorna `None` na extração de pontos para imagem. |
| 7 | `01_core/src/compiler/layout/text.rs:101` | `_ => return None` | **`N16[β]`** | Projeção de texto: `Content` não-textual retorna `None` na extração de string pura. |
| 8 | `01_core/src/compiler/layout/text.rs:143` | `_ => None` | **`N16[β]`** | Projeção de estilo: `Content` não-textual retorna `None` na extração de `TextStyle`. |
| 9 | `01_core/src/compiler/layout/equation.rs:425` | `_ => None` | **`N16[β]`** | Projeção uniforme: valor de numbering não-string extraído como `None` (paridade vanilla). |
| 10 | `01_core/src/compiler/layout/references.rs:278` | `_ => None` | **`N16[β]`** | Resolução de referências: `Content` sem suporte de ref retorna `None`. |
| 11 | `01_core/src/compiler/layout/cursor.rs:323` | `_ => None` | **`N16[β]`** | Métrica de layout: `FrameItem` não-textual não contribui altura de fonte para cálculo de linha. |
| 12 | `01_core/src/compiler/layout/cursor.rs:355` | `_ => None` | **`N16[β]`** | Métrica de layout: `FrameItem` não-textual não contribui `leading` entre linhas. |
| 13 | `01_core/src/compiler/layout/cursor.rs:714` | `_ => 0.0` | **`N16[α]`** | Esgotamento de enum horizontal: `HAlign` só tem Left, Right, Center, Start, End. None/Left/Start produzem `x_offset = 0.0`. |
| 14 | `01_core/src/compiler/layout/grid.rs:242` | `_ => (None, None)` | **`N16[β]`** | Projeção de célula: `Content` em grade que não seja `GridCell` não possui override de posição `(y, rowspan)`. |
| 15 | `01_core/src/compiler/layout/grid.rs:679` | `_ => (None, ...)` | **`N16[β]`** | Projeção de célula: `Content` não-cell retorna tupla de `None`s para propriedades de célula. |
| 16 | `01_core/src/compiler/layout/grid.rs:1255` | `_ => None` | **`N16[β]`** | Projeção de célula: `Content` não-cell retorna `None` para override de stroke de borda. |
| 17 | `01_core/src/compiler/layout/helpers.rs:278` | `None => fallback` | **`N16[γ]`** | Fallback deliberado: resolução de `Value::Auto` ou ausente para a dimensão de contexto. |
| 18 | `01_core/src/compiler/layout/helpers.rs:322` | `_ => (0.0, 0.0)` | **`N16[γ]`** | Fallback aberto de layout: `Content` sem medição explícita retorna dimensões zeradas. |
| 19 | `01_core/src/compiler/layout/mod.rs:2006` | `_ => (0.0, 0.0)` | **`N16[γ]`** | Fallback de dimensão estática: nós sem regra de medição direta retornam `(0.0, 0.0)`. |
| 20 | `01_core/src/compiler/layout/mod.rs:2439` | `_ => None` | **`N16[γ]`** | Despacho dinâmico aberto: `Content` sem braço explícito de layout retorna `None`. |
| 21 | `01_core/src/compiler/layout/sequence.rs:122` | `_ => None` | **`N16[β]`** | Projeção de container: `Content` atômico sem sub-sequência retorna `None`. |
| 22 | `01_core/src/compiler/layout/sub_frame.rs:179` | `_ => None` | **`N16[β]`** | Projeção de frame: `FrameItem` não-textual retorna `None` na extração de estilo. |
| 23 | `01_core/src/compiler/layout/sub_frame.rs:208` | `_ => None` | **`N16[β]`** | Projeção de frame: `FrameItem` não-textual retorna `None` na extração de métricas. |
| 24 | `03_infra/src/export/stream.rs:1427` | `_ => {}` | **`N16[α]`** | Despacho modular fechado: despachante exclusivo de vetores; texto e imagens possuem despachantes próprios. |
| 25 | `03_infra/src/query_helpers.rs:232` | `_other => 0` | **`N16[β]`** | Predicado de consulta: nós que não coincidem com o `ElementKind` consultado retornam 0. |
| 26 | `03_infra/src/query_helpers.rs:246` | `_other => 0` | **`N16[β]`** | Predicado de contagem: elementos que não pertencem a listas retornam 0. |
| 27 | `03_infra/src/query_helpers.rs:260` | `_other => 0` | **`N16[β]`** | Predicado de contagem: elementos que não pertencem a enumerações retornam 0. |
| 28 | `03_infra/src/shaper.rs:503` | `_other => None` | **`N16[β]`** | Mapeamento tipográfico: `MathSize` não-Script/ScriptScript não gera nível `ssty`. |
| 29 | `03_infra/src/shaper.rs:1835` | `_other => None` | **`N16[β]`** | Extração de teste: `FrameItem` não-textual filtrado em coleta de glifos. |
| 30 | `03_infra/src/shaper.rs:1880` | `_other => None` | **`N16[β]`** | Extração de teste: `FrameItem` não-textual filtrado em renderização de teste. |
| 31 | `03_infra/src/shaper.rs:2120` | `_other => None` | **`N16[β]`** | Extração de teste: `FrameItem` não-textual filtrado em cálculo de largura. |
| 32 | `03_infra/src/shaper.rs:2361` | `_other => false` | **`N16[β]`** | Predicado de direção: `FrameItem` não-textual não possui direção RTL. |
| 33 | `03_infra/src/font_metrics.rs:318` | `None => default` | **`N16[γ]`** | Fallback de métrica: `TextEdge::None` resolve para default dependente do contexto. |
| 34 | `03_infra/src/font_metrics.rs:399` | `_ => None` | **`N16[β]`** | Projeção de tamanho: `MathSize` fora de sub/sobrescrito retorna `None` para `ssty`. |
| 35 | `03_infra/src/layout_bidi.rs:677` | `_ => None` | **`N16[β]`** | Projeção de altura: `FrameItem` não-textual retorna `None` em medição tipográfica bidi. |
| 36 | `03_infra/src/pipeline.rs:258` | `_ => {}` | **`N16[β]`** | Visitação de AST: nós que não são `ContextBlock` ou `Styled` são ignorados na coleta de blocos. |

---

## 3. Distribuição Estatística Final do Lote 3

- **`N16[β]` (Comportamento Uniforme por Contrato)**: **27 casos = 75.0%**
- **`N16[γ]` (Fallback Deliberado Aberto / Vigilância Ativa)**: **7 casos = 19.4%**
- **`N16[α]` (Impossibilidade Estrutural / Despacho Modular Fechado)**: **2 casos = 5.6%**
- **Total Auditado e Anotado**: **36 casos = 100%**.

---

## 4. Validação e Qualidade

- `crystalline-lint .`: **0 erros, 0 avisos de drift de hash**.
- `cargo test --workspace`: **5.950 testes aprovados (100% PASS)**.
