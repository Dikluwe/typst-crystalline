# Prompt — typst-passo-860: fechar `measure()` de vez — largura E altura, sem deixar nada pendente

**Origem**: P858 corrigiu a largura de `measure()` (métrica real de fonte via `FontMetrics` injetado). A altura ficou sem correção — vem de um mecanismo diferente (`layout_sub_frame`, avanço de linha) que P858 não tocou. O DEBT-69 foi marcado como fechado prematuramente; este passo é para fechar de verdade, os dois números juntos.
**Estado**: aguardando execução. **Este é o passo final do achado #34 — não sai daqui até bater o critério de parada abaixo.**

---

## Critério de parada (definir isto logo no início, não no fim)

`measure()` está fechado quando os seis casos abaixo — os mesmos usados em P842/P849/P858 — baterem com o vanilla **em largura E altura ao mesmo tempo**, não só um dos dois:

1. `measure([hello])`
2. `measure([x])`
3. `measure([abcd])`
4. `measure([a b])`
5. `measure([])`
6. `measure([hello])` com `#set text(size: 20pt)`

"Bater" significa: mesma ordem de grandeza e mesma fonte de comparação (o mesmo binário vanilla, a mesma fonte de fallback nos dois lados — não comparar a largura do cristalino com fallback do sistema contra a altura do vanilla com Helvetica embutida, como aconteceu em P858, o que tornou a comparação de altura inconclusiva desde o início). Se depois de implementado ainda houver diferença residual pequena (arredondamento, sub-pixel), isso é aceitável e deve ser registrado como tal — mas uma diferença de ordem de grandeza como a que existe hoje (7-8pt de erro na altura) não é.

---

## Por que a altura ainda está errada (não é mistério, é mecanismo não tocado)

`measure_content_real` (`01_core/src/engine/layout/mod.rs:1790`, já ajustado por P858 para largura) devolve a altura vinda do avanço de linha calculado por `layout_sub_frame` — isto é uma aproximação de espaçamento de linha, não a caixa real do texto (bounding box de tinta). O vanilla mede a altura real dos glifos.

**Este projeto já resolveu exatamente este tipo de problema antes**: o passo 813 (equação em bloco) precisou da mesma coisa — trocar aproximação de linha por bounds reais de tinta — e criou `FontMetrics::text_ink_bounds(text, size, style) -> (Pt, Pt)` (ascent, descent reais dos glifos), com implementação concreta em `03_infra/src/font_metrics.rs` (`FontBookMetrics` e `FallbackFontMetrics`). Esse método já existe, já está testado, já está em produção desde P813. Este passo não é pesquisa nova — é usar essa peça já pronta no lugar que ainda falta.

---

## Passo 1 — Confirmar que `text_ink_bounds` serve para este caso

1. Ler a assinatura e o comportamento de `FontMetrics::text_ink_bounds` (definido em `01_core/src/engine/layout/metrics.rs`, implementado em `03_infra/src/font_metrics.rs`) e confirmar que ele devolve exatamente o que `measure()` precisa para altura: ascent + descent reais do texto medido, não uma aproximação de linha.
2. Se houver alguma diferença de forma entre o que `text_ink_bounds` devolve e o que `measure()` precisa devolver (o vanilla devolve um par largura/altura único, não ascent/descent separados — confirmar a conversão exata: altura = ascent + descent? há algum ajuste de sinal ou baseline envolvido, como em P813?), resolver isso antes de prosseguir.

## Passo 2 — Corrigir a fonte de comparação primeiro (antes de medir qualquer coisa)

Antes de qualquer medição "antes/depois", garantir que os dois binários (vanilla e cristalino) estão usando a **mesma fonte** para o texto de teste — o mesmo problema que tornou a medição de altura de P858 inconclusiva (comparação entre fallback do sistema e Helvetica embutida) não pode se repetir aqui. Usar `--font-path` explícito nos dois binários apontando para a mesma fonte, ou confirmar que o documento de teste força a mesma família nos dois (`#set text(font: "...")`) antes de medir.

## Passo 3 — Implementação

Em `measure_content_real` (ou onde a altura for hoje calculada a partir do avanço de linha), trocar essa fonte de dado por `text_ink_bounds` sobre o mesmo conteúdo já usado para a largura — mesmo padrão de P858 (a mesma métrica `engine.font_metrics` já injetada serve para isto, não precisa de mecanismo novo).

## Passo 4 — Medição, com a fonte controlada

1. Recompilar. Rodar os seis casos do critério de parada, com a mesma fonte nos dois binários (Passo 2), reportando largura E altura lado a lado numa única tabela — não em relatórios separados que dificultam ver os dois juntos.
2. Se algum caso não bater, não parar e chamar de "residual pré-existente" sem justificar — investigar até ter uma explicação concreta (arredondamento de subpixel é aceitável; qualquer coisa maior que ~1pt de diferença não é, sem explicação clara do porquê).
3. Confirmar que documentos sem `#context`/`measure()` continuam sem nenhuma mudança de comportamento (mesma checagem já feita em P858, repetir para garantir que a mudança de altura não regrediu isso).

## Passo 5 — Fechar DEBT-69 de verdade

Só depois do critério de parada bater nos seis casos: atualizar `00_nucleo/diagnosticos/debt/DEBT.md`, DEBT-69, confirmando que largura e altura estão fechadas juntas, com a tabela final anexada. Se por algum motivo um dos seis casos não puder ser fechado (motivo técnico real, não estimativa), a dívida continua aberta com escopo reduzido e explícito — não se marca como fechada com uma ressalva escondida no meio do texto.

## Relatório

`00_nucleo/diagnosticos/typst-passo-860-relatorio.md` com: a tabela dos seis casos, largura e altura lado a lado, vanilla vs cristalino, com a mesma fonte nos dois lados em todas as linhas. Sem esse formato de tabela única e comparável, o relatório não está completo.
