# Tracking no PDF — correção da soma dupla

TextShaped com fontes embutidas aplica agora tracking uma única vez, pelos
avanços recebidos do shaper. A correção abrange emissão individual, emissão
fundida, compacto/verboso e restauração gráfica diferida. Text e fallback Type1
conservam seus caminhos anteriores; bitmap permanece desenho de imagem.

## Proveniência

Esta frente começou em 28/09 e terminou em 29/09; os nomes dos artefatos conservam
a data inicial. Working tree não commitada sobre
`17f13b1df79b87ea065ee62e78164b18e477da3d`.
Os registros abaixo ficam em `tracking-pdf-2026-09-28/`, junto deste relatório.
`initial.json` e `final-receipt.json` registram UTC, status completo,
`git diff HEAD --stat` e binários. `current-source.json` contém os hashes completos
produtivos/L0, reconferidos após a validação. As fontes produtivas PDF não mudaram
entre as sondas e o fechamento; `pre-snapshot-source.json` identifica o estado
anterior à manutenção posterior do corpus de teste.

- Vanilla pinado: `9f2b6e8715237cb086899a42873660fe744622e8`.
- SHA-256 vanilla: `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
- SHA-256 Tekt anterior: `96b383826b294dc147ace9eb9c012f5825ae97abcab85b0c94d82bba5f32ea8d`.
- SHA-256 Tekt final: `b9067a8a70e74f73876b65f583c0a422950e2c863b62ff9b6f5a25868c6d40e2`.

Três pares owner/consumer e um núcleo foram atualizados. A mudança produtiva
funcional está nos dois consumers de emissão textual; o terceiro é o harness de
snapshots, atualizado apenas em linhagem/documentação. `scope.json`,
`scoped-seals.json`, `nucleus.json`, `changed-scope.json` e `review.diff.txt`
registram o escopo. `extra-scope.json` identifica MANIFEST e referência CIDFont.
A árvore já tinha outras alterações; elas não são atribuídas a esta frente.

## Medição anterior à decisão

A frente anterior mediu desvio máximo de 2.652pt no controle sem SHY
`representation end` com tracking 0.2pt, modo verboso, contra vanilla.
O compacto já tinha somente 0.348pt. A matriz inicial desta frente repetiu o
resultado e testou trinta entradas, cada uma em Tekt compacto, Tekt verboso e
vanilla, com fonte Noto Sans versionada. Vinte e duas entradas apresentavam
posições/grupamentos de palavras divergentes entre os modos antes da correção.
Estado, comandos, inputs, XML bbox, extração UTF-8 e PDFs estão em `before/`.

`03_infra/src/shaper/shaping.rs:319-326` soma tracking aos avanços.
No snapshot anterior, `export/stream/text.rs:639,668` somava Tc novamente;
`export/stream/text/runs.rs:107` repetia isso na emissão fundida. Seu modelo de
cursor compensava somente os avanços/TJ e não a soma Tc. O L0 especificava esse
helper também para runs shaped: a lacuna estava na especificação. Isso não
permite inferir intenção histórica. O vanilla pinado aplica tracking aos glyphs
em `lab/typst-original/crates/typst-layout/src/inline/shaping.rs:1269-1292`.

Os testes independentes interpretaram os deslocamentos Tc/TJ e falharam tanto
no caso individual quanto no fundido, usando tracking positivo e estado herdado.
O controle de Text/Type1 passou antes da implementação. `red.log` conserva as
falhas; `green.log` conserva a passagem após a correção.

## Decisão, atomização e linhagem

A fonte única de espaçamento dos runs shaped são seus avanços autoritativos.
O helper privado `push_shaped_spacing`, no owner textual existente, estabelece
Tc=0 dentro de BT antes de TJ. É o estado normativo de espaçamento adicional,
sem exceção de fonte, GID, fixture, texto ou largura. Single e coalescido usam
o mesmo helper. O despacho continua estático; não há registry ou callbacks novos.

`00_nucleo/prompts/_nuclei/layout/pdf-shaped-advances.toml` compartilha os
invariantes de avanços, estado explícito e fronteiras absolutas entre os dois
prompts proprietários, com pins completos. Cada consumer continua com owner
exclusivo. O L0 foi atualizado antes do código; resselo foi restrito ao escopo.

É correção interna de paridade em fluxo contínuo ADR-0127, sem campo/assinatura
pública, fase, flag ou modo padrão novo. Não se subtrai tracking dos avanços,
não se altera shaping, seleção de quebras, fonte ou offsets. A compensação TJ
entre itens fundidos conserva o cursor arredondado existente.

## Validação final

- Três testes novos cobrem CIDFont/Multifont, single/coalescido, ambos os modos,
  tracking positivo/negativo/zero/ausente e Tc herdado. Conferem posições por
  um interpretador mínimo independente e fronteiras absolutas do trecho seguinte.
  Verificam também que Text/Type1 continua com seu tracking.
- A suíte infra passou: 1135 testes unitários e 34 de integração, sem falhas.
  Builds dev e release offline concluíram. Os logs e hashes estão no recibo final.
- Lint final: zero erros, 503 warnings, 1554 notes. Nenhum V5/V6/V15/V26 nos owners
  desta frente; não há alegação de eliminar avisos preexistentes fora do escopo.
- Os trinta controles novos têm pixels idênticos entre compacto e verboso,
  rasterizados por MuPDF a 144dpi. Todos os compactos também preservam pixels
  anteriores. As posições dos glyphs das palavras coincidem entre modos dentro
  de 0.015pt; o maior desvio medido é aproximadamente 0.000017pt.
- Quatro regressões com tracking tinham alteração esperada em relação ao verboso
  anterior. Cada uma foi compilada em compacto nos binários anterior e novo:
  pixels preservados, e verboso corrigido com pixels idênticos ao compacto.
  Junto aos trinta controles, são sessenta e oito comparações de raster idênticas.
- A reprodução das 401 entradas anteriores conserva 397 sem mudanças observadas.
  As quatro mudanças esperadas são `after-pdf/tracking` e
  `regression-policy/tracking--0.333`, `tracking-0.333`, `tracking-3`, validadas
  pelos controles acima. O recibo bruto mantém essas quatro diferenças;
  não foram apagadas ou chamadas de regressões sem análise. As observações do
  vanilla pinado foram reutilizadas com inputs idênticos; não são novas
  execuções bilaterais dos 401 casos.

A matriz está em `after/`; `visual/` contém rasters e traces. O recibo final
mantém a distinção entre glyphs de palavras, espaços, extração e imagem.
A sequência Unicode preserva SHY e U+002D. Não se alega igualdade de bytes PDF.

## Manutenção do snapshot CIDFont

Somente o caso 09 do corpus P307b falhou após a implementação: 6759 bytes
anteriores contra 6768 novos. Foi regenerado pelo harness conforme o L0, com
Compact, tags Disabled e epoch fixa, duas vezes. Os resultados foram
byte-idênticos; qpdf aprovou ambos; texto extraído e raster permaneceram iguais.
Retirar os operadores `0 Tc` da stream decodificada nova restitui exatamente a
stream antiga. `snapshot-receipt.json` registra hashes e provas. MANIFEST foi
atualizado; nenhum teste binário foi desativado. A execução final, sem UPDATE,
aprovou todo o corpus. Isso é controle cristalino, não paridade binária vanilla.

## Limites abertos e próxima frente

No controle `plain-0.2`, o desvio máximo contra vanilla caiu de 2.652pt para
0.348pt. O residual coincide com o compacto anterior; medição/arredondamento de
tracking ainda não estão fechados. Oito dos trinta casos coincidem também em
palavras/caixas com vanilla dentro de 0.015pt; esta frente não certifica os demais
nessa dimensão, nem tracking em ligaduras ou todos os scripts.

A análise inicial de trace incluindo todos os glyphs encontrou até 0.048pt de
diferença nos espaços vazios U+0020. A fonte anterior já emitia posições de Text
no compacto com uma casa decimal, enquanto o verboso usa cinco. O recibo bruto
`raw-trace-receipt.json` conserva esse dado. A tolerância de glyphs das palavras
não foi alargada para encobri-lo. Em `ligature-1.2`, o Poppler agrupa palavras de
forma diferente entre os modos, apesar da imagem idêntica e das mesmas posições
dos glyphs das palavras. Espaços e extração precisam de controle próprio de
precisão; não se afirma igualdade bruta de texto desse caso.

O próximo trabalho indicado pelas medições é alinhar medição/emissão e
arredondamento de tracking, além da precisão dos carriers de espaço. Seleção
explícita de SHY, compostos, RTL integral e os limites registrados na frente
anterior continuam abertos.
