---
# P526 — Sonda de igualdade de saída: HTML, SVG, PNG, IDE/LSP

> **Passo:** 526
> **Data:** 2026-07-01
> **Foco:** Diagnóstico empírico do estado actual das funcionalidades declaradas "Fora de escopo" no handoff (HTML export, SVG export, raster render PNG, IDE/LSP). Medir: (1) quais já têm infraestrutura parcial no cristalino (código morto, stubs, ou caminhos de compilação condicional); (2) quais são puramente ausentes; (3) complexidade de implementação de cada uma para atingir paridade com vanilla 0.15.0; (4) dependências entre elas; (5) ordem de execução recomendada. Zero código de produção neste passo — apenas medição, inventário e decisão.
> **Tipo:** Diagnóstico
> **Tamanho:** M (~50 min)
> **ADR-0108 EM VIGOR** — medir antes de decidir.
> **ADR-0114 EM VIGOR** — sonda A.0 antes de spec.
> **ADR-0107 EM VIGOR** — paridade é de linguagem, não de mecânica; mas igualdade de saída (HTML/SVG/PNG/IDE) é mecânica de produção que o usuário declarou como pré-requisito para inovação.
> **Dependências:** P525 (VF MVP fechado, paridade de produção completa)
---

## Contexto

O handoff declarou HTML export, SVG export, raster render (PNG) e IDE/LSP como **Fora de escopo** permanentes (PDF-only). O usuário estabeleceu nova condição: **inovação (Lookahead, etc.) só quando houver igualdade de saída com o vanilla 0.15.0**, incluindo estas funcionalidades.

Este passo é a sonda A.0 para determinar o estado actual de cada uma, a complexidade de implementação, e a ordem de execução. Não se trata de implementar neste passo, mas de medir e decidir.

---

## Grupo 1 — Inventário de código existente (stubs, caminhos condicionais, código morto)

### 1.1 HTML export

```bash
grep -rn "html\|HTML" 01_core/src/ 03_infra/src/ 04_wiring/src/ --include="*.rs" | grep -v "//\|# " | head -n 30
grep -rn "format.*html\|html.*format\|export.*html" 03_infra/src/ 04_wiring/src/ --include="*.rs" | head -n 20
```

Perguntas:
- Existe algum módulo `html`, `export/html`, ou `render/html`?
- A CLI (`typst-wiring` ou `typst` bin) aceita `--format html` ou similar?
- O `Document` (entidade de layout) tem algum método `to_html` ou `render_html`?
- O parser/lexer tem tokens ou nodes específicos de HTML?

### 1.2 SVG export

```bash
grep -rn "svg\|SVG" 01_core/src/ 03_infra/src/ 04_wiring/src/ --include="*.rs" | grep -v "//\|# " | head -n 30
grep -rn "format.*svg\|svg.*format\|export.*svg" 03_infra/src/ 04_wiring/src/ --include="*.rs" | head -n 20
```

Perguntas:
- Existe módulo `svg` ou `render/svg`?
- A CLI aceita `--format svg`?
- O `Document` tem `render_svg`?
- Existe infraestrutura de paths/bezier/curvas que poderia ser reutilizada para SVG (provavelmente sim, dado P513 Curve elements)?

### 1.3 Raster render (PNG)

```bash
grep -rn "png\|PNG\|raster\|render\|image.*export" 01_core/src/ 03_infra/src/ 04_wiring/src/ --include="*.rs" | grep -v "//\|# " | head -n 30
grep -rn "format.*png\|png.*format" 03_infra/src/ 04_wiring/src/ --include="*.rs" | head -n 20
```

Perguntas:
- Existe módulo `png`, `raster`, `render/png`?
- Dependências no `Cargo.toml` para rasterização (ex.: `resvg`, `tiny-skia`, `png` crate)?
- A CLI aceita `--format png` ou `--ppi`?
- O `Document` tem `render_png`?

### 1.4 IDE / LSP

```bash
grep -rn "lsp\|LSP\|language.server\|ide\|completion\|hover\|goto" 01_core/src/ 03_infra/src/ 04_wiring/src/ --include="*.rs" | grep -v "//\|# " | head -n 30
grep -rn "tower-lsp\|lsp-types\|jsonrpc" Cargo.toml 01_core/Cargo.toml 03_infra/Cargo.toml 04_wiring/Cargo.toml 2>/dev/null
```

Perguntas:
- Existe bin `typst-lsp` ou módulo `lsp`?
- Dependências LSP no Cargo.toml?
- Existe infraestrutura de análise semântica (AST com spans, source mapping, symbol table) que poderia alimentar um LSP?
- O parser/lexer preserva spans/offsets para diagnostics?

### 1.5 Critério de fecho

- [ ] Para cada funcionalidade (HTML, SVG, PNG, IDE): listar ficheiros/módulos existentes (se houver).
- [ ] Para cada funcionalidade: classificar como `AUSENTE` (zero código), `STUB` (código morto ou não funcional), ou `PARCIAL` (alguma infraestrutura reutilizável).
- [ ] Inventário de dependências Cargo.toml relevantes.

---

## Grupo 2 — Comportamento actual da CLI

### 2.1 Testar flags de formato

```bash
./target/release/typst --help
./target/release/typst compile --help
```

Verificar se aparecem:
- `--format` (html, svg, png, pdf)
- `--ppi` (para PNG)
- `--pages` (para PNG/SVG)

### 2.2 Tentar compilar para HTML/SVG/PNG

```bash
# HTML
cat > /tmp/test.typ << 'EOF'
Hello world.
EOF
./target/release/typst compile /tmp/test.typ /tmp/test.html 2>&1 || echo "EXIT: $?"
./target/release/typst compile --format html /tmp/test.typ /tmp/test.html 2>&1 || echo "EXIT: $?"

# SVG
./target/release/typst compile --format svg /tmp/test.typ /tmp/test.svg 2>&1 || echo "EXIT: $?"

# PNG
./target/release/typst compile --format png /tmp/test.typ /tmp/test.png 2>&1 || echo "EXIT: $?"
```

### 2.3 Critério de fecho

- [ ] Saída de `--help` registada.
- [ ] Resultado de cada tentativa (erro, panic, output inesperado) registado.
- [ ] Classificação: `AUSENTE` (flag não existe), `ERRO` (flag existe mas falha), `STUB` (produz output vazio/inválido).

---

## Grupo 3 — Análise de complexidade por funcionalidade

### 3.1 HTML export

O Typst vanilla gera HTML a partir do `Document` (árvore de layout). Requer:

| Componente | Descrição | Esforço estimado | Reutilizável do cristalino? |
|------------|-----------|------------------|----------------------------|
| A. Layout tree → HTML DOM | Mapear `Frame`, `Text`, `Image`, `Shape`, `Table`, `Grid` para elementos HTML/CSS | M–L | O `Document` e `Frame` existem; mapeamento é novo |
| B. CSS generation | Estilos inline ou `<style>`: fontes, cores, posicionamento, margens, etc. | M | Parcial — `TextStyle`, `ParElem` têm propriedades CSS-análogas |
| C. Font embedding (web fonts) | Referenciar fontes do sistema como `@font-face` ou usar fontes do sistema | S–M | fontdb já descobre fontes (P515) |
| D. Math → MathML | Fórmulas matemáticas em MathML ou imagens | L–XL | Math elements existem (P510–P511); MathML é markup diferente |
| E. Interactive elements | Links, outlines, bookmarks | S | PDF já suporta (P517); HTML é `<a>`, `<nav>` |
| F. Images | Referenciar ficheiros externos ou embutir base64 | S | `image` element existe |

**Estimativa total HTML:** L–XL (depende da fidelidade; MVP sem MathML = M–L).

### 3.2 SVG export

| Componente | Descrição | Esforço estimado | Reutilizável? |
|------------|-----------|------------------|---------------|
| A. Layout tree → SVG | Mapear `Frame` para `<svg>`, `<text>`, `<path>`, `<image>`, `<rect>`, etc. | M | Paths/bezier existem (P513); coordenadas de layout existem |
| B. Text rendering | `<text>` com fontes referenciadas ou convertidas para paths | S–M | Shaping existe (P515); fontes podem ser referenciadas ou subsetadas |
| C. Font embedding | `@font-face` em SVG ou converter glyphs para `<path>` | M | Subsetting existe (P516); converter glyphs para paths requer parse de outlines |
| D. Images | `<image>` com href externo ou base64 embutido | S | `image` element existe |
| E. Gradients/patterns | Preenchimentos complexos | S–M | `Gradient`, `Pattern` existem em entities? |

**Estimativa total SVG:** M–L (MVP com fontes referenciadas = M; com fontes embutidas = L).

### 3.3 Raster render (PNG)

| Componente | Descrição | Esforço estimado | Reutilizável? |
|------------|-----------|------------------|---------------|
| A. Layout tree → bitmap | Rasterizar o `Document` para uma superfície de pixels | M | Necessita de rasterizer (resvg, tiny-skia, ou cairo) |
| B. Text rasterization | Renderizar shaped text com fontes | M | HarfBuzz + fontdb + rasterizer; ou usar resvg que já faz isso |
| C. Path rasterization | Preencher e traçar paths (bezier, curvas) | S–M | resvg/tiny-skia fazem isso |
| D. Image compositing | Compor imagens, transparência, blending | S | rasterizer faz isso |
| E. Multi-page / paged output | PNG por página ou uma só imagem longa | S | CLI flag `--pages` |
| F. DPI / PPI | Escala de 72dpi (PDF) para 96dpi+ (PNG) | XS | Factor de escala |

**Abordagem recomendada:** usar `resvg` + `tiny-skia` (crates Rust) que já rasterizam SVG. Fluxo: `Document` → SVG (reutilizar Grupo 3.2) → resvg → PNG. Isso reduz o esforço se SVG for implementado primeiro.

**Estimativa total PNG (via SVG):** S–M (se SVG existe). Se SVG não existe: M–L (rasterizer directo).

### 3.4 IDE / LSP

| Componente | Descrição | Esforço estimado | Reutilizável? |
|------------|-----------|------------------|---------------|
| A. LSP server (tower-lsp) | Boilerplate de servidor LSP (initialize, shutdown, etc.) | S | `tower-lsp` crate existe; boilerplate padrão |
| B. Source mapping | Mapear posições no source .typ para AST nodes e vice-versa | M | Parser/lexer preserva spans? Verificar |
| C. Semantic analysis | Resolver símbolos, imports, scopes, tipos | L–XL | Eval engine já faz isso; precisa de API de introspecção |
| D. Diagnostics | Compilar e reportar erros com posições | S–M | Eval engine já produz diagnostics; formatar para LSP |
| E. Completions | Auto-complete de funções, variáveis, campos | L | Requer symbol table + scope analysis |
| F. Hover / Go to Definition | Resolver referências e mostrar documentação | L | Requer semantic analysis + source mapping |
| G. Formatting | Formatador de código Typst | M–L | Requer pretty-printer do AST |

**Estimativa total IDE/LSP:** XL (LSP completo com completions, hover, goto-definition). MVP (diagnostics + básico) = L.

### 3.5 Critério de fecho

- [ ] Tabela de componentes preenchida para cada funcionalidade.
- [ ] Estimativa de esforço total por funcionalidade (MVP e completo).
- [ ] Ordem de dependências: SVG → PNG (se via SVG); HTML independente; LSP independente mas mais complexo.

---

## Grupo 4 — Dependências e ordem de execução

### 4.1 Dependências entre funcionalidades

```
HTML ── independente
SVG ───┬── independente
       └── PNG (via SVG)
LSP ─── independente (mas requer semantic analysis do eval)
```

### 4.2 Ordem recomendada

Baseado em complexidade e reutilização:

| Ordem | Funcionalidade | Tamanho | Razão |
|-------|---------------|---------|-------|
| 1 | **SVG export** | M–L | Reutiliza infraestrutura de layout, paths, shaping. Serve como base para PNG. |
| 2 | **PNG export** | S–M (se SVG feito) | Via resvg/tiny-skia a partir de SVG. Quase "grátis" se SVG existe. |
| 3 | **HTML export** | M–L (MVP) | Independente, mas requer mapeamento DOM/CSS. MVP sem MathML é M. |
| 4 | **IDE / LSP** | XL | Mais complexo, menos prioritário se o foco é "igualdade de saída" (compilação). |

### 4.3 Alternativa: HTML primeiro

Se o objetivo é "igualdade de saída" para publicação web, HTML pode ser mais prioritário que SVG/PNG. Mas SVG/PNG compartilham mais infraestrutura com o PDF existente (layout, coordenadas, paths).

### 4.4 Critério de fecho

- [ ] Ordem de execução proposta e justificada.
- [ ] Dependências mapeadas.

---

## Grupo 5 — Paridade com vanilla 0.15.0

### 5.1 O que o vanilla suporta

Verificar no `lab/typst-original`:

```bash
./lab/typst-original/target/release/typst --help
./lab/typst-original/target/release/typst compile --help
```

Espera-se:
- `typst compile --format html`
- `typst compile --format svg`
- `typst compile --format png --ppi 144`
- `typst compile --pages 1-3`
- `typst lsp` (ou binário separado `typst-lsp`)

### 5.2 Testar vanilla

```bash
./lab/typst-original/target/release/typst compile --format html /tmp/test.typ /tmp/test-vanilla.html
./lab/typst-original/target/release/typst compile --format svg /tmp/test.typ /tmp/test-vanilla.svg
./lab/typst-original/target/release/typst compile --format png --ppi 144 /tmp/test.typ /tmp/test-vanilla.png
```

Inspecionar outputs para entender a fidelidade esperada.

### 5.3 Critério de fecho

- [ ] Capacidades do vanilla confirmadas (flags, formatos, opções).
- [ ] Outputs de vanilla inspecionados (tamanho, estrutura, fidelidade).
- [ ] Lista de requisitos mínimos para "igualdade de saída" definida.

---

## Tabela final de classificação

| Funcionalidade | Código existente | CLI actual | Esforço MVP | Esforço completo | Dependências | Prioridade proposta |
|----------------|------------------|------------|-------------|------------------|--------------|---------------------|
| HTML export | _a preencher_ | _a preencher_ | M–L | L–XL | — | 3 |
| SVG export | _a preencher_ | _a preencher_ | M | L | — | 1 |
| PNG export | _a preencher_ | _a preencher_ | S–M (pós-SVG) | M | SVG | 2 |
| IDE/LSP | _a preencher_ | _a preencher_ | L | XL | Semantic analysis | 4 |

---

## Decisão de prosseguimento

Baseado nos resultados da sonda:

- **Se código existente for mínimo (AUSENTE para todas):**
  - P527: Especificação de SVG export (Trilha 8). Tamanho M–L.
  - P528+: Implementação de SVG export (3–5 passos).
  - P53x: PNG export (Trilha 9, S–M se SVG feito).
  - P54x: HTML export (Trilha 10, M–L).
  - P55x+: IDE/LSP (Trilha 11, XL, pode ser scope-out se o utilizador aceitar).

- **Se alguma funcionalidade tiver infraestrutura parcial (STUB/PARCIAL):**
  - Priorizar a que tiver mais código reutilizável.
  - Documentar o que existe e o que falta.

- **Se o utilizador quiser apenas "igualdade de saída" para compilação (não IDE):**
  - Scope-out IDE/LSP como último (ou permanente).
  - Focar em SVG → PNG → HTML.

---

## Relatório de execução

O relatório deve ser produzido em:
`00_nucleo/diagnosticos/sonda-igualdade-saida-p526.md`

Conteúdo mínimo:
- Resultados do Grupo 1 (inventário de código).
- Resultados do Grupo 2 (comportamento CLI).
- Resultados do Grupo 3 (complexidade por funcionalidade).
- Resultados do Grupo 4 (dependências e ordem).
- Resultados do Grupo 5 (paridade com vanilla).
- Tabela final de classificação.
- Recomendação de prosseguimento e ordem de trilhas.

---

## Comandos de reprodução

```bash
# Grupo 1 — inventário de código
grep -rn "html\|HTML" 01_core/src/ 03_infra/src/ 04_wiring/src/ --include="*.rs" | grep -v "//\|# " | head -n 30
grep -rn "svg\|SVG" 01_core/src/ 03_infra/src/ 04_wiring/src/ --include="*.rs" | grep -v "//\|# " | head -n 30
grep -rn "png\|PNG\|raster" 01_core/src/ 03_infra/src/ 04_wiring/src/ --include="*.rs" | grep -v "//\|# " | head -n 30
grep -rn "lsp\|LSP\|language.server\|ide" 01_core/src/ 03_infra/src/ 04_wiring/src/ --include="*.rs" | grep -v "//\|# " | head -n 30
grep -rn "tower-lsp\|lsp-types\|jsonrpc" Cargo.toml 01_core/Cargo.toml 03_infra/Cargo.toml 04_wiring/Cargo.toml 2>/dev/null

# Grupo 2 — CLI
./target/release/typst --help
./target/release/typst compile --help

# Grupo 5 — vanilla
./lab/typst-original/target/release/typst --help
./lab/typst-original/target/release/typst compile --help
```
