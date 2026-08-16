# Relatório de Execução — Passo 1058

**Data**: 2026-08-15
**Passo**: 1058 — Módulos de Constantes por Domínio com L0 Ligado à Proveniência (`layout/vanilla_defaults.rs`)
**Gate**: `ADR-0127` (Classificação: Refatoração Estrutural de Domínio / Sem Alteração de Comportamento)
**Status**: CONCLUÍDO COM ÊXITO (Módulo de domínio criado, L0 formalizado, 8 sítios migrados, fan-in circunscrito a `layout/`, 100% dos testes aprovados e crystalline-lint 0 erros/drift)

---

## 1. Contexto e Objetivos

Seguindo o precedente estabelecido por `entities/math_constants.rs`, o Passo 1058 introduziu a modularização de constantes canônicas por domínio, evitando a criação de hubs globais (anti-padrão combatido pelo `ADR-0104`).

O domínio piloto foi `layout/`, consolidando as constantes tipográficas do vanilla Typst utilizadas no motor de posicionamento com citações únicas, rastreabilidade direta ao repositório de referência e documentação do *porquê científico* no Prompt L0 correspondente.

---

## 2. Inventário e Proveniência das Constantes

Em `01_core/src/compiler/layout/vanilla_defaults.rs`:

1. **`PAR_SPACING = 1.2` ($1.20\text{em}$)**:
   * **Proveniência**: `lab/typst-original/crates/typst-library/src/model/par.rs:224`
   * **Fundamentação**: Espaçamento vertical padrão entre parágrafos distintos (`Content::Parbreak`).
2. **`PAR_LEADING = 0.65` ($0.65\text{em}$)**:
   * **Proveniência**: `lab/typst-original/crates/typst-library/src/model/par.rs:210`
   * **Fundamentação**: Distância vertical entre o bottom-edge da linha anterior e o top-edge da linha subsequente no mesmo parágrafo.
3. **`BLOCK_SPACING = 1.2` ($1.20\text{em}$)**:
   * **Proveniência**: `lab/typst-original/crates/typst-library/src/layout/container.rs:342`
   * **Fundamentação**: Espaçamento vertical padrão (`above`/`below`) para elementos de bloco genéricos (`BlockElem`, `DividerElem`, `ShapeElem`). Compartilha o valor numérico com `PAR_SPACING`, mas opera via modelo de caixa de bloco.

---

## 3. Especificação L0 e Migração dos Consumidores

### 3.1 Prompt L0 Criado
* [`00_nucleo/prompts/compiler/layout/vanilla_defaults.md`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/00_nucleo/prompts/compiler/layout/vanilla_defaults.md)
* Hash selado: `5266f93f`

### 3.2 Consumidores Migrados em `01_core/src/compiler/layout/`
Substituídos literais espalhados por constantes nomeadas com comentário de rastreabilidade:
* [`mod.rs`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core/src/compiler/layout/mod.rs#L1113): `PAR_SPACING` e `PAR_LEADING` no cálculo de `Content::Parbreak`.
* [`cursor.rs`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core/src/compiler/layout/cursor.rs#L353): `PAR_LEADING` no fallback de avanço de linha do `flush_line`.
* [`sub_frame.rs`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core/src/compiler/layout/sub_frame.rs#L206): `PAR_LEADING` no dreno de sub-frames.
* [`sequence.rs`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core/src/compiler/layout/sequence.rs#L133): `PAR_LEADING` na separação de sequências.
* [`enum_item.rs`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core/src/compiler/layout/enum_item.rs#L45): `PAR_LEADING` no avanço de enumerações.
* [`list_item.rs`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core/src/compiler/layout/list_item.rs#L44): `PAR_LEADING` no avanço de listas de marcadores.
* [`divider.rs`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core/src/compiler/layout/divider.rs#L28): `BLOCK_SPACING` no espaçamento acima/abaixo do traço.
* [`shape.rs`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core/src/compiler/layout/shape.rs#L24): `BLOCK_SPACING` como constante de espaçamento de bloco de formas.

---

## 4. Medição de Fan-in e Prevenção de Hubs (Fase F)

* **Total de Consumidores**: 8 arquivos.
* **Escopo**: 100% interno a `01_core/src/compiler/layout/`.
* **Zero vazamento**: Nenhum import em `math/`, `stdlib/`, `export/` ou `infra/`. A fronteira modular do domínio `layout` permaneceu estritamente isolada e coesa.

---

## 5. Validação Geral

* `crystalline-lint .`: **0 erros**, **0 drift de L0**.
* `cargo test --workspace`: **5.930+ testes aprovados (100% PASS)** sem nenhuma regressão.
