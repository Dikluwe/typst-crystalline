# Sonda A.0 — P512 — Table/Grid HLine/VLine e Curve Elements

**Data:** 2026-06-30  
**Passo:** 512  
**Tipo:** Sonda A.0 (diagnóstico imutável, ADR-0114)  
**Foco:** Verificar empiricamente o estado dos elementos de linha em tabelas/grids e elementos de curva.

---

## 1. Metodologia

Foram executadas 4 sondas independentes:

1. **Parser/sintaxe** — compilar snippets mínimos de cada elemento.
2. **Variants em `Content`** — procurar `TableHLine`, `TableVLine`, `GridHLine`, `GridVLine`, `CurveMove`, `CurveLine`, `CurveCubic`, `CurveQuad`, `CurveClose` em `01_core/src/entities/content.rs`.
3. **Construtores nativos** — procurar `native_table_hline`, `native_table_vline`, `native_grid_hline`, `native_grid_vline`, `native_curve_*` em `01_core/src/engine/stdlib/`.
4. **Layout handlers** — procurar os mesmos variants em `01_core/src/engine/layout/`.

---

## 2. Resultados

### 2.1 Sonda de Parser/Sintaxe

| Elemento | Snippet | Resultado | Observação |
|---|---|---|---|
| `table.hline` | `#table(columns: 2, table.hline(), [A], [B])` | **FAIL** | `função não tem campo 'hline'` |
| `table.vline` | `#table(columns: 2, table.vline(), [A], [B])` | **FAIL** | `função não tem campo 'vline'` |
| `grid.hline` | `#grid(columns: 2, grid.hline(), [A], [B])` | **FAIL** | `esta função não tem campos` |
| `grid.vline` | `#grid(columns: 2, grid.vline(), [A], [B])` | **FAIL** | `esta função não tem campos` |
| `curve.move` | `#curve.move((0,0))` | **FAIL** | `esta função não tem campos` |
| `curve.line` | `#curve.line((100,0))` | **FAIL** | `esta função não tem campos` |
| `curve.cubic` | `#curve.cubic((0,0), (50,50), (100,0))` | **FAIL** | `esta função não tem campos` |
| `curve.quad` | `#curve.quad((0,0), (50,50))` | **FAIL** | `esta função não tem campos` |
| `curve.close` | `#curve.close()` | **FAIL** | `esta função não tem campos` |

`table` e `grid` existem como funções nativas, mas não expõem campos `hline`/`vline`. `curve` não existe como módulo/função acessível.

### 2.2 Sonda de Variants em `Content`

```bash
rg -n "TableHLine\|TableVLine\|GridHLine\|GridVLine\|CurveMove\|CurveLine\|CurveCubic\|CurveQuad\|CurveClose" \
   01_core/src/entities/content.rs --type rs
```

**Resultado:** nenhuma ocorrência.

### 2.3 Sonda de Construtores Nativos

```bash
rg -n "native_table_hline\|native_table_vline\|native_grid_hline\|native_grid_vline\|native_curve" \
   01_core/src/engine/stdlib/ --type rs
```

**Resultado:** nenhuma ocorrência.

### 2.4 Sonda de Layout Handlers

```bash
rg -n "TableHLine\|TableVLine\|GridHLine\|GridVLine\|CurveMove\|CurveLine\|CurveCubic\|CurveQuad\|CurveClose" \
   01_core/src/engine/layout/ --type rs
```

**Resultado:** nenhuma ocorrência.

---

## 3. Tabela de Classificação (ADR-0107)

### 3.1 Table/Grid Lines

| Elemento | Sintaxe (língua) | Semântica (língua) | Morfologia (língua) | Mecânica (diverge) |
|---|---|---|---|---|
| `table.hline` | Parser FAIL | Linha horizontal entre células | Args: start, end, stroke, position | Estrutura interna do `TableElem` |
| `table.vline` | Parser FAIL | Linha vertical entre células | Args: start, end, stroke, position | Estrutura interna do `TableElem` |
| `grid.hline` | Parser FAIL | Linha horizontal entre células | Args: start, end, stroke, position | Estrutura interna do `GridElem` |
| `grid.vline` | Parser FAIL | Linha vertical entre células | Args: start, end, stroke, position | Estrutura interna do `GridElem` |

### 3.2 Curve Elements

| Elemento | Sintaxe (língua) | Semântica (língua) | Morfologia (língua) | Mecânica (diverge) |
|---|---|---|---|---|
| `curve.move` | Parser FAIL | Mover caneta para ponto | Arg: posição (array de 2) | Estrutura interna do `CurveElem` |
| `curve.line` | Parser FAIL | Linha reta para ponto | Arg: posição (array de 2) | Estrutura interna do `CurveElem` |
| `curve.cubic` | Parser FAIL | Curva cúbica de Bézier | Args: 3 pontos | Estrutura interna do `CurveElem` |
| `curve.quad` | Parser FAIL | Curva quadrática de Bézier | Args: 2 pontos | Estrutura interna do `CurveElem` |
| `curve.close` | Parser FAIL | Fechar path | Sem args | Estrutura interna do `CurveElem` |

**Conclusão:** Todos os elementos são **língua** (sintaxe, semântica, morfologia). A mecânica (estrutura interna do `Content`) diverge de propósito (ADR-0107, ADR-0026). A paridade exige que a sintaxe produza o resultado semântico correto, não que a estrutura interna seja idêntica ao vanilla.

---

## 4. Decisão Derivada

- **Gate duro ADR-0114 satisfeito:** 9/9 elementos estão completamente ausentes (parser falha, sem variants, construtores ou layout handlers).
- **Decisão:** P512 = **Spec de Materialização**. Deve ser redigida antes de qualquer código.
- **Atenção:** `table.hline`/`table.vline` requerem alteração ao módulo `table` (adicção de campos/exports), o que pode ter impacto no layout de tabelas existente. Recomenda-se implementar primeiro `grid.hline`/`grid.vline` como prova de conceito (API idêntica, menos complexidade semântica).

---

## 5. Critério de Fecho da Sonda

- [x] 9 comandos de sonda executados (parser).
- [x] 4 comandos de sonda executados (variants, construtores, layout).
- [x] Tabela de resultados preenchida (seção 2).
- [x] Decisão derivada da sonda documentada (seção 4).
- [x] Classificação língua vs mecânica explícita para cada elemento (seção 3).
- [x] `00_nucleo/diagnosticos/sonda-p512-table-grid-curve.md` produzido.
- [x] Gate duro satisfeito (≥5 ausentes) → `00_nucleo/materialization/typst-passo-512-spec.md` redigida.
