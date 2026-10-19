# Relatório — Passo 914: deslocamentos adaptativos e kerning de duas alturas em `attach.rs` (Achado `attach.rs` de P911)

**Data:** 2026-07-25  
**Precede este passo:** `00_nucleo/diagnosticos/typst-passo-911-relatorio.md` (Achado em `attach.rs`)  
**Decisão de Escopo:** Opção Dividida aprovada pelo dono. Implementar em P914 os deslocamentos adaptativos, ajuste simultâneo de gap e kerning de duas alturas de correção. Adiar o conceito de `cramped` para o Passo 915.

---

## Resumo executivo

O Passo 914 corrigiu o desvio estrutural em `01_core/src/engine/math/layout/attach.rs` identificado no Passo 911, onde os deslocamentos verticais de subscritos e sobrescritos eram tratados como constantes fixas da fonte (`superscript_shift_up` / `subscript_shift_down`), e o kerning de aproximação utilizava apenas uma altura simples da base.

Com esta implementação, os deslocamentos passam a ser calculados adaptativamente via `compute_script_shifts`, respeitando o máximo dos limites de queda, topo/fundo e expansão simultânea de gap (`sub_superscript_gap_min`). O kerning passa a somar as correções da base e do script nas duas alturas de contacto por quadrante.

---

## Fase A — Decisão de Escopo e Atualização do L0

### 1. Registro da Decisão de Escopo
Perante as opções de design tático apresentadas no plano de P914:
* **Opção Selecionada (Dividida)**: Concentrar o Passo 914 nas correções geométricas adaptativas (extremos + gap simultâneo + kerning de 2 alturas) que não exigem alteração no estado global de estilo. A introdução do conceito `cramped` (e sua propagação em frações/denominadores) fica reservada como passo dedicado em P915.

### 2. Atualização e Sincronização do Prompt L0
O Prompt L0 [`00_nucleo/prompts/engine/math/layout/attach.md`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/00_nucleo/prompts/engine/math/layout/attach.md) foi atualizado para documentar a fórmula de `compute_script_shifts` e kerning de 2 alturas. O hash do arquivo foi sincronizado com `01_core/src/engine/math/layout/attach.rs` via `crystalline-lint --fix-hashes .`.

---

## Fase B — Implementação

### 1. Extensão de `MathConstants` (`01_core` e `03_infra`)
Em `01_core/src/entities/math_constants.rs`, a estrutura `MathConstants` e seu construtor `fallback()` ganharam os 6 novos campos de controle de script:
* `superscript_bottom_min`, `superscript_bottom_max_with_subscript`, `superscript_baseline_drop_max`
* `sub_superscript_gap_min`, `subscript_top_max`, `subscript_baseline_drop_min`

A leitura dessas constantes a partir das tabelas OpenType MATH da fonte foi integrada em `03_infra/src/font_metrics.rs`.

### 2. Algoritmo Adaptativo em `attach.rs`
Em `01_core/src/engine/math/layout/attach.rs`:
* **`compute_script_shifts`**: Avalia os 4 quadrantes de scripts (`tl`, `bl`, `sup`, `sub`), calculando os deslocamentos `shift_up` e `shift_down` como o máximo entre a constante base, o termo de queda pela base (para bases não-texto) e os limites das caixas dos scripts.
* **Ajuste Simultâneo**: Se `sup` e `sub` coexistem na mesma base e o gap resultante for menor que `sub_superscript_gap_min`, realiza o cálculo de ajuste proporcional (`sup_only` + `rest`), expandindo a separação vertical entre eles.
* **`compute_math_kern`**: Avalia a soma dos kerning da base e do script invertido em duas alturas de conexão (topo e fundo da caixa delimitadora do script), tomando o maior valor de recuo/avanço.

### 3. Testes Unitários
* Adicionado o teste unitário [`p914_layout_attach_shift_adaptativo_expande_gap_quando_necessario`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core/src/engine/math/layout/tests.rs#L2293) em `01_core/src/engine/math/layout/tests.rs`.

---

## Fase C — Validação e Regressão

| Verificação | Resultado |
|---|---|
| Suíte Completa de Testes (`cargo test`) | ✅ **735 passed** (0 falhas) |
| Linter de Arquitetura Cristalina (`crystalline-lint .`) | ✅ **0 violações** |

---

## Conclusão

Com a conclusão do Passo 914, os deslocamentos e kerning de scripts em `attach.rs` seguem fielmente as fórmulas adaptativas do Typst vanilla. O conceito de `cramped` fica pronto para ser integrado no Passo 915.
