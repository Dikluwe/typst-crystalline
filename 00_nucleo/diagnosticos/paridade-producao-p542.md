# Relatório de Paridade de Produção — P542

**Data:** 2026-07-03  
**Passo:** 542  
**Prompt L0:** `00_nucleo/prompts/rules/layout.md` (hash `9c9b7122`)  
**Dependências:** P534 (fallback multi-script), P538e (fallback quando a fonte default não existe), P484 (bidi runs)

## Objectivo

Investigar e corrigir a quebra de linha errada quando há troca de fonte a
meio de uma linha (fallback multi-script). O cristalino ocupava muito mais
linhas do que o vanilla para o mesmo texto misto.

## Sonda

### Teste directo

```typst
Hello 你好 مرحبا world, this is a longer line with mixed scripts to see where it wraps.
```

Resultado cristalino (`pdftotext`):

```text
Hello 你好 مرحبا world,
see where it wraps.

this

is

a longer

line

with mixed scripts

to
```

Resultado vanilla (`pdftotext`):

```text
Hello 你好 مرحبا world, this is a longer line with mixed scripts to see where it wraps.
```

O vanilla mantém tudo numa única linha; o cristalino fragmenta em 11 linhas.

### Causa confirmada

1. **Onde o Layouter decide a quebra de linha:**
   `01_core/src/rules/layout/cursor.rs:95` — `layout_word()` chama
   `self.word_width(word)`, que usa `self.metrics.advance(word, self.style.size)`.
   A métrica usada é a fonte declarada no `TextStyle` (ou `FixedMetrics`
   monoespaçado), não a fonte real que o shaper vai escolher para cada
   caractere.

2. **Onde o fallback de fonte acontece:**
   `03_infra/src/shaper.rs:92` — `try_shape()` segmenta o texto por script
   Unicode e resolve fallback carácter-a-carácter sobre o `FontBook` do
   `World`. Isto corre **depois** do layout, na fase `shape_document`
   (`03_infra/src/pipeline.rs:366`).

3. **A dimensão real do problema:**
   A pipeline de produção (`compile_to_pdf_bytes_impl`) chama
   `layout_with_introspector()` (`01_core/src/rules/layout/mod.rs:1645`),
   que instancia o `Layouter` com `FixedMetrics`
   (`01_core/src/rules/layout/mod.rs:1703`). Ou seja, o layout de produção
   **não usa métricas reais de fonte** para medir texto — usa largura
   monoespaçada fixa. O shaper só ajusta o visual depois, sem alterar as
   quebras já decididas.

A causa suspeitada no enunciado do passo está confirmada, mas o problema é
mais profundo: não é apenas que o Layouter não sabe qual fonte vai ser
usada — é que o Layouter de produção nem sequer está equipado para usar
métricas reais de fonte.

## Porque não foi feito um fix parcial

A correcção completa exige integrar a resolução de fonte real (com fallback
multi-script) **antes** da decisão de quebra de linha. As opções avaliadas:

| Opção | Descrição | Avaliação |
|-------|-----------|-----------|
| A | Mover `shape_document` para antes do layout | Reestruturação massiva do pipeline; o layout consumiria `TextShaped` em vez de `Text`. |
| B | Estender `FontMetrics`/`layout_with_introspector` para receber métricas reais com fallback | Mudança média/grande em L1/L3: novo `FallbackFontMetrics` em L3, alteração da assinatura pública de layout, adaptação de todos os callers/testes. |
| C | Heurística no `FixedMetrics` para alargar caracteres não-latinos | Hack; não garante paridade e introduz regressões silenciosas. |

Nenhuma das opções pequenas resolve o problema todo. O passo 542 instrui
explicitamente: *"se a sonda confirmar que é maior do que M, registar isso em
vez de forçar um fix parcial pela terceira vez"*. A dimensão avaliada é
**L a XL**, devido ao impacto na interface `layout_with_introspector`, na
pipeline de produção e nos testes existentes, mais a necessidade de
validação de performance.

## Recomendação

Criar um passo dedicado (ex.: P5xx ou P6xx) para:

1. Introduzir `FallbackFontMetrics` em L3 que, dado um `World` e um
   `TextStyle`, resolva a fonte real por caractere/script e meça a largura
   com a face correcta.
2. Alterar `layout_with_introspector` (ou adicionar
   `layout_with_introspector_and_metrics`) para aceitar um `FontMetrics`
   dinâmico.
3. Actualizar `compile_to_pdf_bytes_impl` para construir e passar o
   `FallbackFontMetrics` ao layout.
4. Validar paridade de quebra de linha com texto misto e regressão de
   performance no caso comum (texto puro latim).

## Ficheiros inspeccionados

- `01_core/src/rules/layout/cursor.rs:94-129` — decisão de quebra de linha.
- `01_core/src/rules/layout/mod.rs:1645-1776` — `layout_with_introspector`
  usa `FixedMetrics`.
- `01_core/src/rules/layout/metrics.rs:21-73` — trait `FontMetrics`.
- `03_infra/src/shaper.rs:92-205` — `try_shape` resolve fallback depois do
  layout.
- `03_infra/src/pipeline.rs:281-387` — ordem das fases: layout → shape.
- `03_infra/src/font_metrics.rs:103-117` — `FontBookMetrics::advance` mede
  uma única face.

## Validação

```bash
./target/release/typst /tmp/wrap-test.typ /tmp/wrap.pdf
/usr/local/bin/typst compile /tmp/wrap-test.typ /tmp/wrap-vanilla.pdf
pdftotext /tmp/wrap.pdf -
pdftotext /tmp/wrap-vanilla.pdf -
```

Confirmada a divergência: 11 linhas no cristalino vs 1 linha no vanilla.

## Conclusão

P542 está concluído como **sonda**. A causa foi confirmada e a dimensão do
trabalho foi avaliada como grande demais para um fix parcial. Registou-se a
recomendação de passo dedicado para integrar métricas de fonte real com
fallback no layout, antes da decisão de quebra de linha.
