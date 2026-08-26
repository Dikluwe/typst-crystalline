# Prompt L0 — `layout/vanilla_defaults` — Constantes Canónicas de Layout
Hash do Código: f9f664e8

**Camada**: L1 · **Alvo**: `01_core/src/compiler/layout/vanilla_defaults.rs`
**Origem**: Criado no **Passo 1058** para consolidar constantes canónicas do domínio `layout` com proveniência e fundamentação tipográfica formal (precedente `entities/math_constants.md`).

---

## 1. Grandezas Canónicas do Domínio Layout

### `PAR_SPACING = 1.2em`
* **Proveniência**: `lab/typst-original/crates/typst-library/src/model/par.rs:224`
* **Definição**: Espaçamento vertical padrão entre parágrafos distintos.
* **Razão Teórica / Porquê Científico**: Na tipografia computacional do Typst, a quebra de parágrafo (`Content::Parbreak`) estabelece uma nova unidade discursiva. Ao contrário de uma simples quebra de linha interna, parágrafos adjacentes colapsam seus espaçamentos para o maior valor entre eles (1.20em). A aplicação deste valor elimina a divergência de $-6.050\text{ pt}$ em relação ao leading intra-linha (P1056/P1057).

### `PAR_LEADING = 0.65em`
* **Proveniência**: `lab/typst-original/crates/typst-library/src/model/par.rs:210`
* **Definição**: Distância vertical entre o `bottom-edge` da linha anterior e o `top-edge` da linha subsequente dentro do mesmo parágrafo.
* **Razão Teórica / Porquê Científico**: O `leading` preserva o ritmo vertical da leitura contínua intra-bloco sem criar segmentação excessiva. O avanço de baseline intra-parágrafo é $\text{top} + |\text{bottom}| + \text{leading}$.

### `BLOCK_SPACING = 1.2em`
* **Proveniência**: `lab/typst-original/crates/typst-library/src/layout/container.rs:342`
* **Definição**: Espaçamento vertical padrão (`above`/`below`) para elementos de bloco genéricos (`BlockElem`, `DividerElem`, `Equation` em bloco).
* **Razão Teórica / Porquê Científico**: Blocos isolados fora do fluxo de texto contínuo herdam a distância de $1.20\text{em}$ para preservar a consistência visual com os parágrafos vizinhos, mas operam através do modelo de contenção e margens de bloco.
