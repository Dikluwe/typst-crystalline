# P1322 — retificação R2: warning HTML é contradição L0

Este sucessor **preserva** `p1322-classification-query-hint-addendum.md` (SHA-256 `cca16ba1b8ad9f4776fdfd49ca5236641c056fb17f249681c76da5e198604381`) e os nove inputs C congelados. Não houve execução de produto nem alteração de L0. Corrige a prioridade HTML e o efeito na seleção, após objeção substancial independente de D e conferência do coordenador. As observações dos seis IDs/51 células, os deltas de query/help/path/version e seus limites continuam os descritos no anexo anterior; somente as conclusões incompatíveis abaixo são supersedidas.

## Evidência que refuta a classificação anterior

`00_nucleo/prompts/wiring.md:171–173` contrata especificamente: `OutputFormat::Html` chama a compilação HTML, escreve UTF-8 e **emite o warning experimental medido no vanilla**. `:83–85` define diagnóstico humano vanilla-espelhado. Esta é uma promessa específica sobre esse warning na rota canônica, não apenas uma aspiração genérica de paridade do projeto.

O objeto de warning medido no vanilla ratificado é construído em `lab/typst-original/crates/typst/src/lib.rs:246–255`: uma headline e três hints. O recibo P1322 R2, SHA-256 `92be1cf09282fc96354a68deadaed4210fb78aa75b09bb1ef0f488efd6a6a5b3`, preserva os três em stderr tanto em P1137-X-002 quanto em P1138-X-004, perfis html/combinado, três ordens. `04_wiring/src/main.rs:388–392` emite somente a headline. Não se encontrou cláusula normativa que reduza o warning prometido à headline nem que autorize omitir os hints.

A frase do anexo anterior exigindo “cláusula literal suficiente” para prioridade 2 foi incorreta: exigia que o L0 transcrevesse as três strings, apesar de já remeter especificamente ao warning medido. A regra P1322 não impõe essa exigência adicional. Sob ADR-0107/0108, hints pertencem ao observável diagnóstico; o MATCH de artefato não reduz a obrigação do canal stderr. **Classificação corrigida: contradição L0 em rota canônica, prioridade 2.** Não é regressão histórica nova; a projeção anterior simplesmente não julgava esse canal.

## Coorte e ownership completo do fragmento

Coorte: `html-experimental-warning-hints`. Path causal único: `CLI.compile.html.warning`. Dois IDs são testemunhas das grafias legada/compile explícita do mesmo caminho; dois perfis habilitados e repetições não multiplicam paths.

Owner produtivo: `04_wiring/src/main.rs`; L0 proprietário único: `00_nucleo/prompts/wiring.md`. A hipótese de owner único limita-se a **completar a emissão fixa já existente** com os três hints constantes e sua terminação de parágrafo, mantendo condição Feature::Html, ordem da emissão, canais, exit codes, target e compilação. Não requer dados novos: nenhuma interpolação, inspeção de DOM, lógica de negócio, novo tipo, estrutura pública ou API de formatter. O L0 já atribui a emissão desse warning a L4. Completar o literal existente não exige mover a formatação semântica de query para L4 nem criar um algoritmo de diagnóstico ali.

A separação L2/L4 em wiring.md:86–98 continua obrigatória. Este recorte não propõe formatter dinâmico ou alternativo: se a futura materialização precisar construir/transformar diagnósticos genericamente, modificar `02_shell/src/diagnostic.rs`, criar API pública ou alterar o fio da pipeline, a hipótese de owner único está refutada e deve ser reaberta antes de código. O formatter L2 existente não é owner adicional apenas por ser capaz de imprimir hints; a correção do fragmento constante não depende de alterá-lo.

Risco demonstrado do recorte: rank 1, uma emissão fixa guardada por uma feature já existente. Preservações futuras obrigatórias: três hints e ordem/terminação, headline única, sem warning quando desligada, mesmas saídas HTML projetadas, nenhuma ativação por target, nenhuma alteração nos modos de serialização, em PDF/SVG/PNG ou no warning dinâmico de query. Testes RED→GREEN e revisão independente ainda serão necessários; esta auditoria não os executou nem autoriza materialização.

Gate: correção interna de paridade ADR-0127, L0-first + resselo + RED→GREEN + revalidação. Uma API pública, default, fase ou quebra de compatibilidade necessária refutaria esse gate contínuo e exigiria parada. A obrigação L0 futura deve tornar explícito o envelope de três hints já abrangido pela promessa atual, sem introduzir outro padrão HTML.

## Seleção R2

Ranking corrigido: `[2,1,-1,1,"html-experimental-warning-hints"]`. Supera `[3,1,-6,1,"namespace-function-missing-field"]` **pela prioridade**, antes do desempate de paths. Portanto a recomendação única P1323 passa a ser **html-experimental-warning-hints**. A seleção Some anterior continua preservada como decisão supersedida, não selecionada em paralelo.

`p1322-classification-selection-r2.json` é o sucessor da seleção congelada, sem mudar matrizes, catálogo ou contagens principais. Acrescenta as duas dívidas de hints separadas e a intenção de metadados do help não resolvida. Query continua prioridade diagnóstica 3 com owner set completo/risco Unknown para seu formatter geral; help não é declarado integralmente fechado. O path externo tem política L0 documentada conforme anexo, e o hash de build é identidade mecânica, sem criação de coorte de reparo.

Este é o parecer corrigido de C, não sua aprovação. D julga a coerência normativa, owner completo, risco, preservações e reordenação. A objeção D efetivamente mudou a decisão; não será diluída como ajuste editorial.
