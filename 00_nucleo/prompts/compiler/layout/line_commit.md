# Prompt L0 — fechamento de linha no layout paginado
Hash do Código: 6cf28557

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:2ccbb1e5daf5f6806e58cd48272b1463fbc9107300c0bce6ea1c3ccf7b0b8748

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/layout/line_commit.rs`
**Vanilla ratificado:** `9f2b6e8715237cb086899a42873660fe744622e8`
**ADRs:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129

## P1503 — medição antes da decisão

No HEAD limpo `0de5edabed197d23b647cfc68ebefda2f5c2a8d5`, antes de
código P1503, `01_core/src/compiler/layout/cursor.rs:406-1038` contém
`expand_fr_spacings`, `justify_current_line`, `flush_line`, o arm one-shot de
wrap e `flush_line_with_outer_singleton`. O helper de justificação ocupa
`:67-105`. O mesmo arquivo mantém transições em `:1039-1420`, floats/replay
em `:1421-1764` e segmentação em `:1878-1985`; o estado/checkpoint fica em
`compiler/layout/mod.rs:326-1417`. O diagnóstico
`00_nucleo/diagnosticos/typst-p1503-responsabilidades-cursor.md` registra
funções, dependências, hashes e baseline. Dois documentos com texto extraído
equivalente mostram diferenças de tempo do comando completo, mas a medição
interna de `repeat` atribui somente 0,705052 ms a todo layout; portanto não
se infere ganho de performance desta extração.

## Decisão e ownership

Esta unidade é dona exclusivamente do **commit da linha corrente**: expansão
horizontal `fr`/repeat, justificação, ajuste de baseline/leading, emissão de
decoração e numeração da linha, drenagem dos items, avanço do cursor e pedido
de transição quando a tinta não cabe. Ela recebe o `Layouter` existente por
referência e usa apenas estado já possuído pelo owner
`compiler/layout.md`. Não cria novo estado, cópia de checkpoint, estrutura de
documento, API pública, fase de pipeline ou dispatcher dinâmico.

`compiler/layout/cursor.md` conserva a formação de palavra/chunk, wrap,
baseline inicial, transições de página/coluna, coordenação de floats/replay,
footnotes/cell tails e segmentação CJK. O pedido `new_page` é uma chamada ao
owner Cursor; esta unidade não absorve nem duplica essa política.
`compiler/layout/sequence.md`, `compiler/layout/repeat.md` e
`compiler/layout/columns.md` continuam donos de suas decisões de preflight,
materialização de repeat e consolidação de colunas. O dispatcher de
`compiler/layout.md` permanece estático e exaustivo.

As fórmulas de fechamento documentadas historicamente em
`compiler/layout/cursor.md` (P1382, amendments P1389–P1408, P1414–P1419 e
P1493) passam a ser obrigação desta unidade. Essas secções permanecem no
antigo prompt como proveniência histórica, não como segundo owner produtivo.

## Contrato observável

- `flush_line` consome o arm one-shot do prefixo de wrap; fechamento explícito
  certificado usa a mesma fórmula. Sem itens materiais, não acumula altura
  nem cria página. Carrier HTML isolado não conta como tinta, mas é
  transportado para extração.
- Expansão horizontal distribui o espaço finito pelos pesos `fr` conforme a
  fórmula já validada; soma/razão não finita dá parcela zero e limpa buffers
  sem loop. `repeat` recebe somente a parcela associada à sua fronteira e
  insere cópias antes do suffix, com índices ajustados uma vez. Largura
  automática/infinita não expande. Justificação desloca items/links com seus
  bounds e respeita RTL, sem inventar oportunidade depois do fim material.
- A linha usa o máximo real de ascent/descent de texto e extensões inline;
  tamanho misto toma próximo ascent e leading do estilo exterior. O
  singleton validado de wrap conserva as exceções P1389/P1391/P1392.
  Itens e collectors recebem a mesma translação vertical antes do drain.
- Quando reserva de float torna a tinta incompatível com a região efetiva,
  sinaliza rejeição ao checkpoint do owner `compiler/layout.md` ou pede a
  transição ordinária ao Cursor. Não migra `current_line` isoladamente,
  não antecipa conteúdo futuro e não duplica efeitos pós-marker.
  Uma linha terminal fracionária cuja tinta cabe não fabrica página vazia
  apenas pelo avanço hipotético da baseline seguinte.
- Numeração de linha só nasce após estabilizar baseline e página causal;
  flush vazio/subframe não incrementa contador. Patterns segmentados
  registram candidatas diferidas no contexto já definido, callbacks seguem
  seu caminho existente; margem/clearance/fixups e suppression overlay
  preservam os resultados P1414–P1419. Este contrato **não** aprova uma
  ampliação pública ainda pendente nos L0s históricos.
- O commit drena `current_line` para `current_items`, limpa oportunidades de
  justificação, registra avanço/descida causal e restaura `cursor_x` ao início
  da linha. Página, colunas, floats e footnotes continuam nos seus owners.

## Aceitação e limites

Caracterização anterior e posterior deve cobrir linha vazia/material,
wrap, estilos mistos, singleton, `fr`/repeat, RTL, linha numerada, colunas,
float/flush transacional, altura auto, carrier HTML e erro. Saída é
comparada na língua: texto, morfologia, página, posição, estilo e diagnóstico
onde observáveis; bytes PDF e `FrameItem` não são gates. A extração é
content-preserving: uma diferença funcional exige passo/medição separados.

O critério arquitetural é o fechamento legível nesta unidade com chamadas
explícitas aos owners vizinhos, sem alargamento de visibilidade, clone de
estado nem lógica de paginação duplicada. Medir tempo/CPU/RAM com fonte e
binário pinados antes/depois; não declarar ganho por tamanho do arquivo.
Alteração de assinatura pública, default, fase ou compatibilidade exige
paragem ADR-0127; a transferência interna de método segue L0 primeiro,
RED arquitetural→GREEN e revalidação contínua.
