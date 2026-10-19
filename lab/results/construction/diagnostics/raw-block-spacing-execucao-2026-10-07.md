# Raw em bloco — espaçamento e origem regional

A autorização `Pode prosseguir` foi aplicada à pendência de Raw bloco. Correção interna de paridade em fluxo contínuo ADR-0127: L0 antes de código, RED→GREEN e recoleta. Não foram criados campos, traits, assinaturas públicas ou fases. A realização transporta a fronteira e os overrides de estilo; fontes/coordenadas continuam no layout.

## Medição anterior e implementação

`raw-block-spacing-2026-10-07/before.json.gz`, `before-additional.json.gz` e `before-state.json.gz` registram fontes completas, comandos, cwd, UTC, estado e binários. Before/Raw/After tinham yMin68.27018/85.806243/95.064120, contra68.27016/91.304203/108.594100 no vanilla. Raw dentro de Cell voltava à margem da página x70.86614, contra origem da célula75.86614. Unknown-language unia abc/def e o produto ABC substituído por show perdia a fronteira de bloco.

A fonte pinada `lab/typst-original/crates/typst-layout/src/rules.rs:655–679` realiza Raw bloco como Block; `typst-library/src/layout/container.rs:324–360` define margens e colapso. A hipótese de usar sempre13.2pt foi refutada por Raw consecutivos: eles usam margens fracas do tamanho Raw8.8pt,10.56pt; um parágrafo exterior11pt contribui13.2pt. Overrides explícitos prevalecem sobre margens fracas, nos dois sentidos. Zero/negativo e Auto interrompendo shorthand têm medidas próprias em edges-red/fourth.

Raw agora possui helpers privados de entrada/saída física, mantém origem local, deixa below pendente e registra a borda final real. Styled usa os mesmos helpers no produto substituído por show, conservando aplicação única. Apply transporta block, tamanho Length e propriedades de margem, sem medir. O dispatcher set reconhece Native block por identidade, incluindo aliases, no subconjunto spacing/above/below com Length/Auto; outros campos/tipos mantêm fallback e não são certificados. Cursor colapsa margem fraca com o parágrafo seguinte; margens fortes conservam prioridade. Sub-frame terminal mede a borda real, sem margem/leading hipotéticos, e trata Strong/Emph transparentes. Quebras explícitas de Raw bloco sobrevivem também sem gramática conhecida.

A inicialização do ascent regional é limitada ao sub-frame legado de Raw terminal. A tentativa incondicional provocou aquisição de font book para imagens/formas sem texto; `suites-third.log` registra os dois negativos e `suites-fourth.log` sua resolução. Não ocultar esses resultados intermediários. `green-second.log` contém um erro de compilação de Pt-=, corrigido sem adicionar trait; não é prova RED. O RED válido inicial teve3 novos testes falhando; strong-red adicionou controle independente de altura, e as coletas de glifos refutaram hipóteses mesmo quando os testes com métricas fixas passavam.

## Resultado final

25 controles Raw (14 principais,5 adicionais,2 aliases,4 extremos) conservam texto e ordem e têm delta vertical máximo0.00005pt. Incluem blocos isolados, prefixo/sufixo, consecutivos, multiline, gramática desconhecida, tamanhos global/show-set/em, células, substituição, margens explícitas/zero/negativas/auto, par.spacing, vazio e controle inline.

Dos4 refutadores adicionais, sizes-up, plain-cell e strong-cell coincidem até0.00002pt vertical. Sizes-down permanece negativo: After94.63018 no Tekt contra105.43020 no vanilla, diferença10.80002pt. Esse negativo já existia antes do reparo; envolve tamanho do parágrafo anterior e não deve ser fechado como Raw corrigido. Não se introduziu tolerância posterior para converter esse caso em sucesso. Os resíduos horizontais de fontes já medidos na etapa tipográfica também não são apagados pela comparação vertical.

6606 testes core,1194 infra,90 shell e10 integração query passaram (5 ignorados core). Builds debug/release passaram. Linter final:0 erros,544 warnings,1628 notes; warnings não significam zero ocorrências. Presença92/92, selector23/23, D05 29/29, D07 29/29+14/14, query36/36 e16 controles tipográficos preservados. D06 continua38/41 com os mesmos negativos root-render/raw-eval/raw-mat-eval.

## Proveniência e limites

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. Snapshot `tested-fourth.json.gz` contém UTC, diffstat/status exatos, conteúdos e hashes. Tekt final SHA-256 `6ca44e46ae2c1b1f8b44207052ea2fd63e3b24b4136c3fc3d25b6d20e9b6a20f`. Oracle pin `9f2b6e8715237cb086899a42873660fe744622e8`, binário SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`, árvore/receita P1489. As coletas preservam binários congelados, argv/cwd, fontes, resultados, SHA dos PDFs, famílias/tamanhos/cores e bboxes. Ambiente NO_COLOR=1, SOURCE_DATE_EPOCH=0, RAYON_NUM_THREADS=1.

`final-results.json` associa cada número à sua coleção e ao estado/binário medido. Sete pares owner/consumer e um núcleo pinado:15 arquivos de implementação/L0 alterados em relação à base inicial, zero fora desse conjunto. implementation.patch compara com essa base e conserva todo o trabalho anterior. Sem commit, reset, reseal global ou atualização global de interfaces.

Whitespace/tabs/linhas internas vazias, syntax customizada, receitas RawLine, reconstrução fresca, rich block/set properties e defaults HTML não foram certificados. B3-B ainda decide Cell isolada e reconstrução fresca em show; igualdade genérica de Table e demais limites permanecem. O recorte de espaçamento/origem Raw medido está corrigido; não é fechamento universal do renderer nem da linguagem.
