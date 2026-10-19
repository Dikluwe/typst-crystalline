# Correção dos custos de parágrafo — 2026-09-28

`text.costs.hyphenation` e `text.costs.runt` passam a influenciar a seleção global certificada de linhas. A configuração compõe por campo entre escopos: configurar `runt` preserva a hifenização exterior; um dicionário vazio não apaga os valores herdados. Valores negativos permanecem aceitos, conforme a linguagem medida no vanilla.

## Medição que fundamentou a decisão

O estado anterior descartava `costs` em `set text`, enquanto o constructor aceitava o argumento sem transportá-lo. Os seletores consultavam constantes fixas. Os L0 anteriores mantinham essa implementação adiada; a evidência não permite atribuir intenção ou motivação ao desenvolvimento.

A fonte pinada em `lab/typst-original/crates/typst-library/src/text/mod.rs:1507-1564` valida Dictionary, Ratio por campo e composição parcial. `lab/typst-original/crates/typst-layout/src/inline/mod.rs:240` lê o custo compartilhado do parágrafo. `inline/linebreak.rs:29-30,961-962` registra os defaults e sua escala. Os inputs, fontes, comandos, stderr, PDFs e observações anteriores estão em [before-pdf](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/correcao-custos-paragrafo-2026-09-28/before-pdf/proveniencia.json), vinculados ao estado [initial.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/correcao-custos-paragrafo-2026-09-28/initial.json). As observações finais estão em [after-pdf](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/correcao-custos-paragrafo-2026-09-28/after-pdf/proveniencia.json).

Foram comparadas 66 entradas antes e depois. Nas entradas válidas desse conjunto, as 57 geometrias finais coincidem com o vanilla; três divergências geométricas anteriores foram corrigidas. Nove entradas inválidas anteriormente aceitas agora produzem o mesmo bloco de erro e posições causais do vanilla. O comparador usa palavras e caixas físicas, com tolerância de 0,0005 pt; a quantidade de elementos `<line>` do extrator não é critério de linhas físicas.

A primeira matriz de `runt` coincidia com o vanilla, mas não provocava uma mudança de escolha. A asserção de sensibilidade permaneceu falhando e foi conservada em [acceptance-before-runt-witness.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/correcao-custos-paragrafo-2026-09-28/acceptance-before-runt-witness.json). Uma busca independente no oráculo encontrou uma entrada com largura de 70 pt e custos 0%/1000000%. Os dois controles adicionais coincidem entre compiladores e mudam as posições das palavras mantendo o texto. Foram adicionados após a implementação: não existe medição anterior do Tekt para esses dois inputs e eles não entram na contagem de melhorias com baseline. A busca e os controles são reproduzíveis em [runt-search.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/correcao-custos-paragrafo-2026-09-28/runt-search.json) e [runt-witness](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/correcao-custos-paragrafo-2026-09-28/runt-witness/proveniencia.json).

## Correção e atomização

O owner privado [text_costs.rs](/repos/Antigravity/typst-crystalline/01_core/src/compiler/text_costs.rs) concentra cast, canais por componente e resolução dos custos; seu [L0](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/compiler/text_costs.md) é proprietário exclusivo. O [núcleo paragraph-costs](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/_nuclei/paragraph-costs.toml) compartilha os invariantes com os demais prompts, mediante pins. Não legitima código diretamente.

A tabela `FIELDS` é a única definição dos nomes/canais. Eval e constructor reutilizam o parser; a preparação reutiliza `is_channel`. Os defaults normativos 135/100 ficam nomeados nesse owner e são multiplicados pelos ratios. A correção remove a imposição desses valores fixos ao usuário; não substitui constantes legítimas por configuração inventada. A penalidade consecutiva de hifenização recebe a mesma escala. O seletor simples conserva seu comportamento, e as fórmulas de justificação não foram alteradas.

Os campos conhecidos são validados na ordem `hyphenation`, `runt`, `widow`, `orphan`, antes das chaves desconhecidas. Os diagnósticos preservam tipos, prioridade, chaves e span do dicionário medidos no oráculo. A mutação da cadeia acontece somente depois da validação completa.

A primeira materialização criou uma barreira de preparação para os novos canais. A medição refutou esse fallback: `restore` e `inline` divergiam. Um segundo teste RED mostrou que um custo inline neutro para glyphs fragmentava a palavra. A preparação agora reconhece exclusivamente os quatro canais canônicos sem criar espaço ou fronteira; restaura o escopo exterior e conserva o custo compartilhado. Um wrapper que cobre todo o fluxo continua sendo fatorado pela realização já existente antes de formar `Par`. O ajuste não promove estilos parciais ao exterior nem muda a fase do pipeline.

A rodada segue o fluxo contínuo de correções internas/de paridade da ADR-0127: L0 antes de código, testes RED→GREEN, resselo restrito aos owners e revalidação. Não introduz assinatura pública, flag, fase ou contrato novo. Há 11 pares prompt/consumer e um núcleo novo, totalizando 23 arquivos produtivos/L0 alterados nesta rodada; a revisão não detectou mudança produtiva/L0 fora desse escopo. A árvore já continha alterações extensas anteriores, preservadas e registradas no recibo.

## Validação e proveniência

Estado: HEAD `17f13b1df79b87ea065ee62e78164b18e477da3d` com working tree não commitada. O [recibo final](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/correcao-custos-paragrafo-2026-09-28/final-receipt.json) registra UTC `2026-09-28T22:31:39.670078+00:00`, lista completa de arquivos alterados, `git diff HEAD --stat`, hashes de todas as fontes produtivas/L0 e dos artefatos. As fontes permanecem idênticas entre revisão, sondas finais e controles adicionais.

| Evidência | Resultado |
|---|---|
| Testes core | 6.357 passaram; zero falhas; dois ignorados |
| Testes infra | 1.127 passaram; zero falhas |
| Build dev e release | passaram |
| Crystalline lint | zero erros; 504 warnings; 1.515 notes |
| Entradas bilaterais | 295: 68 novas + 227 regressões |
| Asserções | 1.144 passaram |
| Novas entradas válidas | 59 geometrias coincidentes |
| Melhorias com medição anterior | três geometrias; nove validações inválidas |

Os números correspondem aos logs e hashes do recibo, não a uma árvore limpa ou a uma string de versão. As regressões anteriores conservam seus controles de fallback e limitações; passar esses controles não prova paridade dos contextos ainda abertos.

Oráculo: pin `9f2b6e8715237cb086899a42873660fe744622e8`, árvore `6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4`, receita P1489, binário `lab/typst-original/target/release/typst`, SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.

Tekt medido: `target/release/typst`, SHA-256 `5fd64d361e3c4c387cbf9149660e9e7f7c624f8aeabaf46bbba8d5a2c02d2d98`. O binário anterior era `df6b38883edc780ea1ae04c200f2bbf7089f856a8a0965d9930e5ff5b3b31595`. Não houve commit ou publicação nesta rodada.

## Limites preservados

`widow/orphan` são validados e transportados, mas sua influência física na paginação permanece incompleta. `set text` emite aviso factual quando esses campos são fornecidos. O constructor não dispõe de Sink para emitir esse aviso; este relatório e o L0 não afirmam que a paginação foi implementada.

A cópia textual de hífen discricionário no PDF ainda difere (`-` versus soft hyphen). A normalização do comparador serve somente à geometria visual, não fecha essa diferença textual. Unicode completo, regiões de largura variável, paginação e contextos mistos fora do subconjunto certificado continuam abertos. Cenários de custos não finitos não foram certificados. Não há medição de desempenho nem alegação de estado da arte nesta rodada.

A próxima frente ainda exige medição própria; paginação com `widow/orphan` é uma candidata identificada, sem implementação ou decisão arquitetural antecipada.

Uma tentativa de preparação combinada foi rejeitada pela revisão automática antes da execução, por interpretar o destino L0 como escrita Markdown em Rust. O escopo foi conferido e as escritas de L0 e testes foram realizadas separadamente. A alternativa foi permitida e não resta bloqueio; o registro está em [approval-review-resolution.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/correcao-custos-paragrafo-2026-09-28/approval-review-resolution.json).
