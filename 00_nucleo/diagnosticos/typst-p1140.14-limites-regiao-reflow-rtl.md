# Diagnóstico P1140.14 — Limites de região e reflow RTL

**Estado:** concluído  
**Data:** 2026-08-24

## Resultado

A hipótese inicial foi confirmada e refinada. `line_content_right` não é limite
de região, mas a falha aparece antes mesmo de regiões aninhadas: L3 também não
possui os separadores e oportunidades que participaram do wrapping em L1.

A correção removeu integralmente a fusão de baselines da passagem bidi. L3
agora respeita as quebras decididas pelo layout e limita-se a reordenar items
dentro de cada linha visual.

## Proveniência

Medição final em **2026-08-24T12:50:53-03:00**.

- commit: `45b547073d7686cdd5d3e3030c82de3e22ec395f`;
- estado: working tree não commitado;
- diff antes deste relatório: **2 ficheiros alterados, 56 inserções e 270
  remoções**;
- ficheiros então alterados: `infra/layout_bidi.md` e `layout_bidi.rs`;
- vanilla: `/usr/local/bin/typst`;
- cristalino: `./target/debug/typst`, reconstruído após a correção;
- observável: caixas e baselines de `pdftotext -bbox`.

## RED

Página de 160 pt, margens simétricas de 10 pt, texto hebraico de 10 pt:

- vanilla: **8 palavras**, sete na baseline **7.64 pt** e uma na baseline
  **20.72 pt**;
- cristalino antes: **1 palavra extraída**, todas as letras fundidas na
  baseline **7.64 pt**.

O reflow somava larguras dos `FrameItem::Text`, concluía que o conteúdo cabia e
fundia as duas linhas. Entretanto os espaços existiam apenas como avanços de
`cursor_x`; não estavam na soma nem podiam ser reconstruídos por L3.

## Decisão

Removidos:

- `reflow_rtl_paragraphs`;
- `same_paragraph`;
- `line_advance`;
- `is_predominantly_rtl`;
- `try_fuse_paragraph`;
- todo o estado `fused_lines`.

A alternativa de inferir um espaço entre todos os items foi rejeitada: items
adjacentes também podem representar fragmentos sem espaço, smallcaps,
segmentação CJK, shaping ou interpolação. Transportar todo o line breaking para
L3 duplicaria a fase e contrariaria a soberania do layout.

Os marcadores públicos de `linebreak` e `parbreak` foram preservados para não
introduzir quebra contratual independente. Eles continuam transparentes.

## GREEN diferencial

Após a remoção do reflow:

- controle simétrico: **8 palavras** em ambos, mesmas duas baselines e mesmas
  caixas;
- margens assimétricas: **10 palavras** em ambos;
- colunas: **15 palavras** em ambos;
- bloco de largura explícita: **9 palavras** em ambos.

Os números vêm da mesma medição e estado descritos em “Proveniência”.

## Divergências independentes encontradas

As três sondas estruturais preservam agora morfologia e número de linhas, mas
ainda divergem em posição absoluta:

- margem assimétrica: o cristalino ancora RTL no lado calculado pela margem
  escalar, não no `right` específico;
- colunas: o vanilla ocupa primeiro a coluna direita; o cristalino, a esquerda;
- bloco com largura explícita: o vanilla ancora o bloco/linha à direita e o
  cristalino à esquerda.

Essas divergências já existem na geometria produzida por L1 e não são causadas
pela passagem bidi. Devem formar o próximo passo, sem recolocar reflow em L3.

## Testes e validação

- novos testes `p1140_14_wrapping_preserva_baselines_do_layout` e
  `p1140_14_nao_refaz_quebra_rtl_pos_layout`: GREEN;
- `cargo test -p typst-infra`: **832 passed, 0 failed**;
- `cargo build`: aprovado;
- `crystalline-lint .`: zero violações;
- `git diff --check`: aprovado;
- busca estrutural: nenhuma função removida permanece.

Warnings Rust e avisos informativos V16–V20 preexistentes não são violações.

## Próxima frente

P1140.15 deve medir e corrigir a origem/âncora RTL em regiões L1: margens
assimétricas, ordem de colunas e alinhamento de blocos/subframes. Cada eixo deve
ser atomizado se tiver causas diferentes.
