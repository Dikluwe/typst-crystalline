# Relatório de Execução — Passo 1053 (Revisado com Medição Empírica)

**Data**: 2026-08-14
**Passo**: 1053 — Auditoria de Placeholders: Constantes Usadas para "Fechar Buraco"
**Gate**: `ADR-0127` (Classificação: Auditoria Metodológica / Desenho de Método, Medição e Catalogação)
**Status**: CONCLUÍDO (Varredura sistemática concluída, medição empírica diferencial realizada contra Vanilla Typst 0.15.1, proveniência canônica mapeada e emenda metodológica universal formalizada)

---

## 1. Fase 0 — Definição do Padrão e Matriz Diferencial

### 1.1 O que conta como "Placeholder de Fechar Buraco"

Um **placeholder de fechar buraco** é qualquer valor literal numérico ou de string hardcoded no código que **deveria derivar de uma fonte de verdade externa variável por contexto** (métrica da fonte ativa, tabela OpenType MATH, configuração do documento/estilo ou parâmetros explícitos do elemento), mas foi inserido de forma estática e arbitrária para fazer o código compilar ou simular visualmente uma aproximação rasa.

### 1.2 Matriz Comparativa de Caracterização

| Característica | Placeholder de Fechar Buraco | Constante Canônica de Especificação / Domínio |
| :--- | :--- | :--- |
| **Exemplos Reais** | `gap = style.size * 0.2` (P1042), `Ascent 800` (P1051), `StemV 80` (P1053), `text.len() * 0.5` (bidi P1053) | `par.leading = 0.65em`, `FRAC_PADDING = 0.1em`, `DELIM_SHORT_FALL = 0.1em` |
| **Origem do Valor** | Inventado ou aproximado arbitrariamente para contornar falta de ligação à fonte real. | Definido formalmente na especificação da linguagem Typst (`typst-library`). |
| **Variação por Contexto** | Varia por fonte ou parâmetro, mas é tratado incorretamente como universal fixo. | É o valor padrão (*default fallback*) quando o usuário não configura propriedade explícita. |
| **Sobrescrita por Parâmetro** | Ignora parâmetros do usuário ou métricas de glifos reais. | O código lê ativamente o campo do elemento (`elem.gap`, `elem.leading`) e só recai no default se `None`. |
| **Proveniência Documentada** | Nenhuma, ou comentários vagos como "parece bom", "aproximação". | Citação exata de `file:line` do vanilla (`matrix.rs:15`, `resolve.rs:748`, `par.rs:210`). |

---

## 2. Fase A — Estratégias de Busca Combinadas

A auditoria combinou 4 frentes complementares sobre todo o código de `01_core` e `03_infra`:

1. **Grep por Padrões Numéricos Suspeitos**:
   - Multiplicadores em variáveis de dimensão e avanço (`style.size * 0.[0-9]+`, `size.val() * 0.[0-9]+`);
   - Estimativas de largura por contagem de caracteres (`text.len() as f64 * ...`);
   - Dicionários PDF com valores inteiros/reais fixos (`/(Ascent|Descent|CapHeight|StemV|Flags)\s+[0-9]+`).
2. **Cruzamento com Prompts L0**:
   - Auditoria dos prompts em `00_nucleo/prompts/compiler/math/layout/` e `00_nucleo/prompts/compiler/layout/` para verificar se as fórmulas citam proveniência no vanilla typst.
3. **Varredura Léxica de Confissões em Comentários**:
   - Busca por termos de aproximação (`approx`, `aproximação`, `placeholder`, `hardcoded`, `fallback`, `TODO`).
4. **Análise Dirigida por Área de Risco Histórico**:
   - `03_infra/src/export/` (geração de streams PDF/SVG);
   - `03_infra/src/layout_bidi.rs` (reordenação e fusão de linhas bi-direcionais);
   - `01_core/src/compiler/layout/` e `01_core/src/compiler/math/layout/` (cálculo de geometria).

---

## 3. Fase B — Catálogo de Achados com Medição Empírica e Proveniência Canônica

### 3.1 Achado 5: Emissão de `/StemV 80` no Descritor de Fonte PDF (`builder.rs:1111, 1601`)

- **Diagnóstico Direto**:
  - O Crystalline **não tenta ler o peso ou métrica da fonte e emite `/StemV 80` estaticamente sempre** em `build_paged_single_font` e `build_paged_multi_font`.
- **Mecanismo Canônico do Vanilla Typst**:
  - `krilla-0.8.2/src/text/cid.rs:353`: o Vanilla calcula dinamicamente a espessura da haste vertical em função do peso real da fonte:
    $$\text{stem\_v} = 10.0 + 0.244 \times (\text{face.weight()} - 50.0)$$
- **Medição Diferencial no PDF**:
  - **Fonte Regular (Libertinus Serif, Weight 400)**:
    - Vanilla: `/StemV 95.4` ($10.0 + 0.244 \times 350$)
    - Crystalline: `/StemV 80` ($\mathbf{\Delta = -15.4}$)
  - **Fonte Bold (Libertinus Serif Bold, Weight 700)**:
    - Vanilla: `/StemV 168.6` ($10.0 + 0.244 \times 650$)
    - Crystalline: `/StemV 80` ($\mathbf{\Delta = -88.6}$)
- **Classificação**: **`Placeholder Real / Metadado de PDF`** (não altera layout visual, mas constitui divergência estrutural no stream de FontDescriptor).

---

### 3.2 Achados 2, 3 e 4: Medições Geométricas Diferenciais (Vanilla 0.15.1 vs Crystalline)

| Achado / Construto | Código de Teste Mínimo | Bounding Box Vanilla | Bounding Box Crystalline | Deltas Medidos ($\Delta x, \Delta y, \Delta w, \Delta h$) | Proveniência Canônica Vanilla | Classificação de Gravidade |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Achado 2A: `raw` inline** (`raw.rs:111`) | ``Text with `raw code` in line`` | $x=70.87, y=68.27$<br>$w=119.26, h=12.54$ | $x=70.87, y=68.27$<br>$w=111.21, h=12.54$ | $\mathbf{\Delta w = -8.05\text{ pt}}$<br>$\Delta x=0.0, \Delta y=0.0, \Delta h=0.0$ | `crates/typst-library/src/text/raw.rs:360-390`<br>(Vanilla mantém $100\%$ do `size` e troca família para monospace; Crystalline contrai para $0.9\times$) | **Gravidade Média** (altera largura total da linha em $-8\text{ pt}$) |
| **Achado 2B: `raw` bloco** (`raw.rs:111` & `cursor_x += size`) | ```` ```\nlet x = 1\n``` ```` | $x=70.87, y=70.87$<br>$w=47.68, h=8.80$ | $x=81.87, y=68.53$<br>$w=25.07, h=11.29$ | $\mathbf{\Delta x = +11.00\text{ pt}}$<br>$\mathbf{\Delta w = -22.61\text{ pt}}$<br>$\mathbf{\Delta y = -2.34\text{ pt}}$<br>$\mathbf{\Delta h = +2.49\text{ pt}}$ | `crates/typst-library/src/text/raw.rs:400-430`<br>(Vanilla não aplica indentação fixa $1\text{em}$ sem `inset`; Crystalline injeta $+1\text{em}$ e reduz tamanho) | **Gravidade Alta** ($\Delta x = +11\text{ pt}$ e $\Delta w = -22.6\text{ pt}$) |
| **Achado 3: `divider`** (`divider.rs:44`) | `Before\n\n---\n\nAfter` | Linha 1: $y=68.27$<br>Traço: $y=88.71$ ($\Delta y_1=20.44$)<br>Linha 2: $y=109.15$ ($\Delta y_2=20.44$) | Linha 1: $y=68.27$<br>Traço: $y=82.66$ ($\Delta y_1=14.39$)<br>Linha 2: $y=97.05$ ($\Delta y_2=14.39$) | **Traço**: $\mathbf{\Delta y = -6.05\text{ pt}}$<br>**Linha 2**: $\mathbf{\Delta y = -12.10\text{ pt}}$ | `crates/typst-library/src/layout/container.rs:342`<br>(Vanilla usa `BlockElem::spacing = 1.2em` = $13.2\text{ pt}$ acima/abaixo; Crystalline usa `size * 0.6` = $6.6\text{ pt}$) | **Gravidade Média-Alta** (compressão vertical cumulativa de $-12.1\text{ pt}$ por divisor) |
| **Achado 4: `quote` bloco** (`quote.rs:37`) | `#quote(block: true)[This is a quote]` | $x=81.87, y=68.27$<br>$w=65.62, h=12.54$ | $x=87.37, y=68.27$<br>$w=73.87, h=12.54$ | $\mathbf{\Delta x = +5.50\text{ pt}}$ ($+0.5\text{em}$)<br>$\mathbf{\Delta w = +8.25\text{ pt}}$ | `crates/typst-library/src/model/quote.rs:75-95`<br>(Vanilla aplica `indent = 1.0em` e `quotes = false` em bloco; Crystalline injeta indent $1.5\text{em}$ e aspas inteligentes) | **Gravidade Média** (deslocamento horizontal de $+5.5\text{ pt}$) |
| **Achado 1: Bidi Merge** (`layout_bidi.rs:645`) | Reordenação RTL com pontuação mista | Avaliação interna em `can_merge_lines` | Usa `text.len() * 0.5` | Heurística interna | `crates/typst-layout/src/flow/mod.rs` | **Gravidade Média** (heurística de junção de linhas) |

---

### 3.3 Constantes Canônicas Validadas como Legítimas (Com Proveniência)

- **`matrix.column_gap` / `row_gap`** (`0.5em` / `0.2em`): `lab/typst-original/crates/typst-library/src/math/matrix.rs:15-16`.
- **`par.leading`** (`0.65em`): `lab/typst-original/crates/typst-library/src/model/par.rs:210`.
- **`block.spacing`** (`1.2em`): `lab/typst-original/crates/typst-library/src/layout/container.rs:342`.
- **`FRAC_PADDING`** (`0.1em`): `lab/typst-original/crates/typst-library/src/math/frac.rs:9`.
- **`DELIM_SHORT_FALL`** (`0.1em`): `lab/typst-original/crates/typst-library/src/math/fragment/glyph.rs:265-300`.
- **`ACCENT_SHORT_FALL`** (`0.5em`): `lab/typst-original/crates/typst-library/src/math/accent.rs:18`.

---

## 4. Fase C — Priorização e Backlog de Resolução

Com base nas medições objetivas:

1. **Prioridade 1 (Layout de Bloco & Raw)**:
   - `raw.rs`: Eliminar a redução arbitrária de font size para 0.9x ($\Delta w = -8.05\text{ pt}$) e indentação de 1em em bloco ($\Delta x = +11\text{ pt}$).
   - `divider.rs`: Adotar o espaçamento padrão de bloco $1.2\text{em}$ do Vanilla ($\Delta y = -12.10\text{ pt}$).
   - `quote.rs`: Ajustar indentação para $1.0\text{em}$ e suprimir aspas em bloco ($\Delta x = +5.50\text{ pt}$).
2. **Prioridade 2 (Metadados PDF & Bidi)**:
   - `builder.rs`: Substituir `/StemV 80` fixo pela fórmula dinâmica $10.0 + 0.244 \times (\text{weight} - 50.0)$ em `font_descriptor_metrics`.
   - `layout_bidi.rs`: Substituir `text.len() * 0.5` por `metrics.text_width(...)` em `can_merge_lines`.

---

## 5. Fase D — Regra Metodológica de Proveniência Universal

> **Regra Universal de Proveniência**:
> Toda e qualquer constante numérica, dimensional ou de metadado presente no compilador e na infraestrutura deve obedecer rigorosamente a uma de duas categorias:
> 
> 1. **Constante Canônica de Especificação**: Deve conter no comentário ou prompt L0 a citação exata de proveniência (`file:line` do repositório de referência do Typst Vanilla) e agir unicamente como valor padrão (*fallback*), permitindo que parâmetros fornecidos pelo usuário no elemento ou set-rules tenham precedência absoluta.
> 2. **Dado Contextual Dinâmico**: Grandezas que variam por fonte (ex: *ascent*, *descent*, *cap-height*, *stem-v*, avanços de glifos, constantes OpenType MATH) ou por documento **nunca** podem ser aproximadas por constantes estáticas hardcoded, devendo ser lidas/calculadas dinamicamente da respectiva fonte de verdade (`FontMetrics`, `MathConstants`, `Face::weight()`, `World`).
>
> **Proibição Absoluta**: É vedada a introdução de constantes numéricas ad-hoc para "fechar buracos" em testes ou satisfazer tipos sem embasamento na especificação canônica.

---

## 6. Verificação do Workspace

- **Crystalline Linter**: `0 erros`
- **Cargo Test Workspace**: `5.930 aprovados, 0 falhas, 3 doc-tests ignorados (100% PASS)`
