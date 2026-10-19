# Histórico verificável da materialização

Este diagnóstico é processo, não L0. O plano está em materialization/hanging-indent-2026-10-18/passo.md.

- `before-sources.json.gz` guarda os bytes iniciais; o binário anterior é cópia imutável em bin/typst-before. As quatro matrizes têm fonte, SHA-256, argv, hora UTC, HEAD, working tree/diffstat e hashes dos binários. Os PDFs/bbox iniciais foram arquivados antes da sobrescrita.
- Na primeira matriz extra duas fontes de escopo usaram markup literal em vez da expressão `#[...]`. A versão errada e os artefatos estão preservados como extra-before-initial*. A matriz válida extra-before.json.gz foi recolhida antes da prescrição/decisão; extra-before2.log registra a repetição.
- V15/V26, resselo, V1/V5/V15/V26 e reversa passaram antes da implementação. `red-source-freeze.json.gz` identifica os hashes exatos do RED. `tests-before-code.json.gz` conserva os testes. A declaração do módulo e os testes eram a única adição executável então; corpos produtivos existentes não haviam mudado.
- `red.log`: 14 testes, 11 falhas funcionais e três controles aprovados. Nenhuma função faltante ou erro de compilação usado como RED.
- `green.log`: primeiro código, 12 aprovados e duas falhas. RTL selecionava menor largura mas alinhava à borda antiga; teste de checkpoint tentava recuperar prefixo movido entre buffers pelo flush da tentativa.
- `debug.log`: diagnóstico temporário por assert, já removido. Confirma Spec.dir LTR mesmo com text.dir RTL no canal custom. Mostra os frames do checkpoint artificial. A instrumentação não é critério de aceitação.
- L0 paragraph_indent atualizado antes da correção da direção. Prescrito usar text.dir do tronco lexical. O teste de checkpoint passou a capturar prefixo drenado/confirmado, como no protocolo produtivo de sufixo; não foi ampliado o contrato preexistente de rollback de buffers. L0 style_scope atualizado antes de permitir planejamento próprio dentro da nova fronteira real, restaurando a supressão herdada ao sair.
- `green2.log`: 13 aprovados, RTL corrigido; o novo assert de GGGG esperava20 mas media26 devido ao espaço literal inicial criado pela fixture. Removido esse espaço nas tentativas de sufixo independentes; preservadas as verificações do prefixo, ausência do sufixo rejeitado e recuo da continuação. Não é outra correção produtiva.
- Acrescentado teste discriminante simple/optimized no caminho fonte→layout. Ele verifica escolhas efetivamente diferentes com hanging, não apenas igualdade de coordenadas entre estratégias.

Não se alteraram APIs públicas, fase/ordem de realização ou pin do linter. A referência é o upstream pinado bd5ebec285505889e8ff1d4f68804ab147a699f3, não main móvel nem uma string de versão. As sete fontes consultadas foram comparadas aos objetos Git (`oracle-sources.json`).

## Limite da prova de checkpoint

A tentativa inicial com current_line não vazio demonstrou perda/retensão incorreta de itens após um flush entre capture/restore (DDDD desaparece e FFFF da tentativa permanece; debug.log). A mudança da fixture exercita uma fronteira já drenada e prova apenas a restauração de State nesse caso. Isso **não** prova que o helper bruto seja seguro para qualquer fronteira. O seu contrato por comprimentos e a movimentação entre buffers são anteriores ao lote (comparáveis em before-sources.json.gz), portanto a hipótese de defeito preexistente é uma inferência forte; deve ser testada em um caminho produtivo de Flush/float antes de decidir reparo. Não alegamos cobertura universal de rollback nem descartamos a observação. A auditoria de elegibilidade de checkpoint com prefixo em current_line permanece pendente própria.

## Limite da comparação Plain

A primeira auditoria causal tentou exigir páginas inteiras idênticas para custom optimized e falhou em bare3em. A citação anterior é um parágrafo normal e agora recebe corretamente o hanging genérico, alterando sua altura e transladando a bibliografia. A prova posterior compara apenas o corpo a partir de CUSTOM-BIB, normalizado pela origem desse corpo, e exige igualdade de todas as palavras e coordenadas relativas; a diferença anterior de escolha da primeira linha Plain permanece aberta. causal-audit-initial.py guarda o critério inicial. Isso não altera o critério de paridade completa de0.001pt, nem permite fechar o caso Plain.

## Refinamento final: decorações

A suíte inicial9026 e todos os seus gates passaram antes da revisão complementar. Seus logs, freeze, resultados e metadados foram preservados em initial-validation/; o binário é bin/typst-text-validated (SHA-256 acdb65256c1a635dc6e37b444075abc3724f836a23d740925a24857b549b86f2). Não são confundidos com a validação final posterior.

A medição SVG12 confirmou inícios incorretos de decorações em continuções recuadas (x8 vs18 com h10). A primeira coleta só selecionava path e omitiu as primitivas line do Tekt; o probe inicial/log/gzip foram mantidos e a inspeção corrigida registra também transforms de ancestrais, sem recompilar ou alterar a fonte. Y/espessura/cauda já divergem em h0 e continuam frente própria.

Quatro L0s foram atualizados primeiro (incluindo decorations como novo consumer do núcleo existente). V15/V26 e linhagem passaram antes do código; deco-red.log teve15 controles aprovados e dois REDs funcionais. O refinamento usa preview somente de leitura, borda esquerda efetiva no collector de flush/subframe e no segmento final; deco-green.log aprovou17 testes.

Um preview negativo ainda podia fabricar um traço para body vazio. L0 primeiro, novo teste: deco-empty-red.log confirmou um traço de -10 a0 sem body (17 aprovados, um RED). A guarda exige admissão material quando preview difere do cursor e não houve segmento flushado; não inicia admissão para desenhar. deco-empty-green.log aprovou18, incluindo os três tipos de decoração, corpo vazio/metadata/Par vazio, sinais, entrada após quebra e subframe. Depois só foram corrigidos comentários dos helpers, antes do resselo/freeze final.

A validação final repete o corpus com o novo binário e a suíte ampla. Nenhum resultado do primeiro cohort é somado aos testes finais. O escopo complementar de origem é LTR; offsets/espessura/cauda anteriores e a geometria completa de RTL permanecem separados, sem alegação universal de paridade das decorações.

## Relógio do PDF na validação final

A rodada ampla final falhou em um único teste Infra de bytes determinísticos. A análise de workspace.log extraiu os PDFs de13392 bytes: quatro bytes diferentes, todos nas datas Info/XMP, ao atravessar10:09:55→10:09:56Z. Substituir somente essas datas iguala todo o PDF. metadata.rs:22–28 e seu L0 já oferecem CRYSTALLINE_PDF_FIXED_EPOCH/SOURCE_DATE_EPOCH; faltou configurar o primeiro nesta execução de snapshots. finish-final.py repete o alvo inteiro com o override existente, sem mudar código/asserts. A suíte efetiva substitui o alvo, reaproveitando só os outros7680 aprovados no freeze final idêntico. Logs/PDFs e a interrupção original são preservados. Não é prova de regressão geométrica nem alteração do relógio padrão do produto.

O script de validação foi ajustado para oferecer o epoch de snapshot por setdefault em execuções futuras, respeitando override externo. validate-unfixed-clock.py preserva a receita anterior à correção de configuração; finish-final.py registra a revalidação efetiva. Nenhuma fonte produtiva foi alterada por essa configuração.
