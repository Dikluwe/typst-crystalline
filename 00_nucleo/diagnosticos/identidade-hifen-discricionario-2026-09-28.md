# Identidade do hífen discricionário até o PDF — correção materializada

As quebras automáticas já escolhidas pelo Tekt conservam agora a identidade textual
U+00AD até o PDF. O desenho continua usando o hífen visual; hífens literais mantêm
U+002D. Esta frente não altera o algoritmo que escolhe as quebras.

## Proveniência e autorização

Working tree não commitada sobre `17f13b1df79b87ea065ee62e78164b18e477da3d`.
`initial.json` registra o estado anterior e `final-receipt.json` registra o estado
verificado, com UTC, status completo, `git diff HEAD --stat`, hashes dos binários
 e dos logs. Os caminhos destes registros são relativos ao diretório
`identidade-hifen-discricionario-2026-09-28/`, junto deste relatório.
`current-source.json` contém os hashes completos das fontes produtivas/L0;
sua identidade foi reconferida depois de testes, builds e sondas.

O vanilla é o pin `9f2b6e8715237cb086899a42873660fe744622e8`, binário SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
O Tekt anterior tem SHA-256
`1ca83fd69c72643e53d54dcbdad9488e0738cb2d37d71057a17e61ffa3b03717`;
o Tekt final tem SHA-256
`144c32b06abce5458ac652a71dee83add05518c8934d923b4b308664245818a3`.
A fonte Noto Sans Regular usada está identificada em `initial.json`.

O usuário confirmou o contrato público com “confirmo”. `authorization.json`
registra a proposta autorizada, SHA-256
`b744f80e801eceddefadc76e79cf343ba19174d65ad84f53a2c4b9ccaebc1870`.
A autorização cobre `FrameItem::TextReplacement { text: EcoString, items:
Vec<FrameItem> }`. Os L0 foram atualizados antes da materialização, com resselo
restrito a cada owner. `scope.json`, `scoped-seals.json`, `changed-scope.json`
e `review.diff.txt` tornam o escopo revisável: quarenta pares owner/consumer,
mais um núcleo compartilhado; oitenta e um arquivos produtivos/L0 alterados
em relação ao estado inicial desta frente. A árvore já possuía outras alterações.

## Medição antes da decisão

Na medição inicial, oito controles en/pt × simple/optimized × justify false/true
coincidiam visualmente com o vanilla dentro de 0.015pt, mas trinta ocorrências
extraídas pelo bbox eram U+002D no Tekt e U+00AD no vanilla. A extração comum
removia parte dos hífens de fim de linha: ausência de '-' no `.txt` não provava SHY.
PDFs, XML, comandos e fontes estão em `before/`; `findings.json` registra a análise.

`01_core/src/compiler/layout/paragraph_hyphen.rs:208` materializava o fragmento
visual sem transportar a causa; `01_core/src/compiler/layout/break_selection.rs:51`
concatenava '-' ao prefixo incremental. Os L0 anteriores especificavam esse
comportamento e deixavam a identidade discricionária como limitação. A lacuna
estava também na especificação. Isso não prova intenção histórica ou “preguiça”.
O vanilla em `lab/typst-original/crates/typst-layout/src/inline/line.rs` e
`inline/shaping.rs` distingue glifo visual e conteúdo extraível da quebra soft.
Os caminhos exatos e linhas medidos constam da proposta preservada em
`before-source/` e `approved-before/`.

A decisão aprovada foi transportar identidade por ocorrência em um envelope.
Um mapeamento global do glifo '-' para U+00AD perderia a distinção de hífens
literais que usam a mesma fonte/glifo. Nenhuma heurística de posição, sufixo ou
aparência decide essa identidade.

## Materialização e atomização

Os seletores global e incremental fornecem dados privados de substituição.
O layout conserva os filhos visuais no novo envelope; o caminho incremental
preserva o prefixo inteiro para conservar seu kerning. Métricas, deslocamentos,
recursos, shaping e percursos visuais descem nos filhos. A projeção textual lê
uma vez o texto substituto. O envelope não acrescenta avanço nem geometria.

A emissão de identidade foi atomizada em
`03_infra/src/export/stream/text_replacement.rs`, com owner exclusivo
`00_nucleo/prompts/infra/export/stream/text_replacement.md`. Essa unidade emite
`ActualText` UTF-16BE com BOM e os delimitadores BDC/EMC; seleção de fontes,
shaping, transformações e tagging continuam com seus owners. O núcleo
`00_nucleo/prompts/_nuclei/layout/text-replacement.toml` compartilha invariantes,
pinados pelos prompts consumidores. O despacho permanece estático/exaustivo.

## Falha encontrada durante a validação e correção

A primeira materialização passou nos testes de transporte, mas o PDF verboso
extraía o substituto fora da posição do texto. As provas refutadas estão
preservadas em `first-round/`: não servem como evidência de conclusão.
Uma sonda controlada de dezoito streams em `envelope-probe/` isolou o limite:
fechar ActualText depois de restaurar a transformação gráfica deslocava o bbox
no Poppler. Fechar antes da última restauração conserva a posição.

O teste `envelope-red.log` reproduziu a ordem incorreta. A emissão de texto agora
permite adiar a restauração final privada; o dispatcher fecha ActualText e só
então restaura o estado gráfico. Não há análise de strings de operadores nem
mudança do modo de produção. O teste cobre Type1 top/local, compacto/verboso,
com/sem tags e negrito sintético. Os L0 dos emissores registram esse limite.

## Evidências finais

- Os dois testes causais de layout falharam antes da implementação (`red.log`) e
  passaram depois. Os testes finais registram 6365 testes core e 1130 infra
  aprovados, além das suítes de integração; duas exclusões core já existentes.
- Builds dev e release offline concluíram. `lint-final.sarif` tem zero erros,
  503 warnings e 1555 notes; não há V5/V6/V15/V26 nos owners desta frente.
  Não se afirma que todos os avisos preexistentes do repositório foram eliminados.
- Nos oito controles automáticos, todas as palavras bbox e a extração comum
  coincidem textualmente com o vanilla, incluindo as trinta ocorrências U+00AD.
  A geometria coincide dentro do critério declarado de 0.015pt.
- Foram executadas doze entradas bilaterais principais e dezesseis adicionais.
  Duas adicionais são entradas inválidas em ambos os compiladores e não contam
  como demonstração de renderização. As demais cobrem coexistência com hífen
  literal, estilos, link, box, move, fallback incremental/fontes, tags, compacto
  e um documento válido aceito por `--pdf-standard ua-1`.
- Vinte e seis PDFs Tekt válidos foram rasterizados antes/depois a 144dpi com
  MuPDF. Todos têm pixels idênticos. O texto visual e a geometria bbox anteriores
  também foram preservados, inclusive onde já divergiam do vanilla.
- A reprodução de 401 entradas anteriores realizou 1930 verificações sem falhas.
  Reexecutou Tekt e reutilizou as observações certificadas do vanilla pinado,
  conferindo entradas idênticas. Não são 401 novas execuções bilaterais.
  SHY→'-' foi normalizado apenas nessa comparação visual, não na prova textual.

Os resultados finais estão em `after/`, `after-extra/`, `raster/`,
`regressions/` e `final-receipt.json`. Os runners e snapshots têm extensão `.txt`
para não serem confundidos com consumers produtivos pelo linter.

## Limites que continuam abertos

O SHY autoral ainda é extraído duas vezes nos controles de largura 100/300pt,
contra uma ocorrência no vanilla, mesmo sem quebra. A seleção de quebras em
compostos também possui diferenças anteriores. O controle box conserva sua
seleção anterior, que não coincide integralmente com o vanilla. Essas falhas
não foram fechadas pela identidade de hifenização automática.

A aceitação do documento UA-1 pelo compilador não é certificação completa de
acessibilidade. Os testes cobrem os caminhos descritos, sem alegar cobertura
integral de bidi/RTL, todas as combinações de fallback ou todo o Unicode.

## Seguimento posterior

A duplicação do SHY autoral em runs com fonte resolvida foi tratada na frente
[SHY autoral](shy-autoral-2026-09-28.md), com estado e provas próprios.
As observações anteriores deste relatório continuam referentes ao seu snapshot.
Seleção explícita estreita e compostos permanecem abertos.
