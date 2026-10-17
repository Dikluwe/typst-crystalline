# Caminhos residuais e expressões locais

Removidos 37 diagnósticos do Clippy em 30 consumers: bindings e funções sem
consumidor, visibilidade interna excedente, expressões redundantes e literais
com dígitos sem efeito. O comportamento contratado e as expectativas dos
testes permanecem. Nenhuma supressão de lint foi adicionada.

## Proveniência e medição

Branch `Tekt`, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, **working tree
não commitado**. `caminhos-residuais-ci-2026-10-13/before.json.gz` conserva
fontes, owners, pesquisa de usos, inventário selecionado, UTC, status e
`git diff HEAD --stat`. A baseline contém 105 diagnósticos distintos de
core lib/testes, medida no lote anterior de predicados/fixtures.

As decisões sobre remoção/visibilidade foram adicionadas aos 13 owners antes
do código. Nos outros 17 somente o Hash do Código mudou. O resselo foi
restrito aos 30 consumers. As receitas, diffs, logs, inventários e este
relatório estão reunidos em `evidence.json.gz`; `after.json.gz` contém
estado Git e hashes finais. O linter de validação permanece no commit
publicado `49c6bb0e584104a0c89270431d8402f5c6c61150`.

## Remoções com consumidores preservados

| Fonte anterior | Item removido ou ajustado | Comportamento preservado |
| --- | --- | --- |
| layout/grid/row_groups.rs:82 | cálculo de rowspan sem uso | primeiro y explícito em conflito com o header |
| layout/sub_frame.rs:265 | varredura de leading descartada | altura vigente, isolamento e restauração externa |
| layout/boxed.rs:263 | soma box_h descartada | h_pt, Group, inset, clip e contribuição à linha |
| layout/heading.rs:113-114 | aquisição/descent descartados | baseline, flush, espaçamento e estilo |
| math/layout/accent.rs:153 | dx duplicado | accent_x continua usando os attachments |
| math/layout/attach.rs:688 | sub_top descartado e comentário incorreto | fórmula de gap e shifts simultâneos |
| stdlib/numbering.rs:168 | índice de enumerate sem uso | ordem, consumo de números e chamadas do formatador |
| layout/mod.rs:875 | binding passa a _font_size | mesmo parâmetro posicional; estilo vem da chain |

No construtor de Layouter, a fonte já declarava que o estilo inicial deriva
da StyleChain, não do argumento legado de tamanho. A correção não passa a
aplicar esse argumento nem muda a assinatura de tipos. bidi_cursor e
pending_repeats passam a campos privados de layout; seus consumers são
layout e descendentes, incluindo checkpoint. Os tipos não foram expostos
para resolver o aviso de visibilidade.

Foram removidas seis funções sem chamadas: extract_cell_fields,
native_array_all_static, native_str_clusters_static, str_trim,
expect_one_array e o wrapper de três argumentos counter_at_location.
O último pertence a um módulo pub(crate), sem reexport externo. Os aliases
em value_methods apontam para counter_at_location_owned; o caminho recorded
permanece usado. Os wrappers collection_static de array.all e str.clusters,
array_all_with_pred em array_predicate e str_trim_dispatch continuam ativos.
`active-routes.json.gz` conserva as rotas auxiliares examinadas.

A primeira compilação expôs um import de array_all_with_pred que só servia
ao wrapper removido. O owner foi refinado antes da remoção desse import;
a função em seu módulo continua sendo chamada por array_all. Nenhuma função
de linguagem foi retirada do registro.

## Simplificações e testes

O parser BibTeX recorta a string diretamente no nome ASCII da entrada;
seus limites vêm da varredura ASCII depois de @. Não se introduziu unwrap
artificial a partir da sugestão do Clippy. A classificação de observação,
a proteção de viúvas/órfãs, o replay de StateUpdate::Func e a condição de
placement mantêm os mesmos ramos e a mesma ordem de curto-circuito.

A soma de frações em line_commit continua sendo avaliada somente quando
há largura distribuível finita. Option::take/replace conservam o estado
anterior e o novo estado. Outline mantém ausência de title distinta de
Some(Ok(None)), incluindo as mesmas mensagens. O teste de Display de
Unnumberable verifica diretamente o erro, sem embrulhá-lo em Err para
imediatamente fazer unwrap_err. Os nomes e as expectativas dos testes foram
preservados; a fixture de níveis usa array com os mesmos quatro valores.

Os cinco literais sinalizados excessive_precision foram encurtados sem
alterar seus valores. Uma compilação independente com o mesmo rustc provou
identidade dos pares: f32 `3e4c8769`, f32 `3ddd91bc` e f64
`3feaf767a741088c`. Fonte, binário, versão do compilador, comandos e saída
estão em `literals-receipt.json.gz`. Isto prova equivalência de representação
entre os literais, não nova paridade de cor com o vanilla. As comparações
to_bits existentes continuam com os mesmos valores esperados.

## Validação

A primeira rodada foi encerrada com código 143 enquanto a suíte core ainda
rodava. O log parcial foi preservado em `interrupted-validation.json.gz` e
não conta como aprovação ou falha de assertion. Após remover o import
restante e ressellar, foi executada uma nova rodada completa.

A rodada completa encerrou em `2026-10-14T00:12:31.079384+00:00`; rustdoc encerrou em
`2026-10-14T00:12:40.156198+00:00`. `pre-validation.json.gz` delimita as fontes finais,
inalteradas durante a validação. Comandos e saídas estão nos recibos.

| Verificação | Resultado |
| --- | --- |
| Clippy core lib/testes com -D warnings | 105 → 68 diagnósticos distintos; ainda falha |
| Categorias selecionadas | 37 removidos, sem nova categoria |
| Check workspace com todos os targets | passou |
| Suíte core | 6.705 passaram, zero falhas, cinco ignorados |
| Registro de testes | 6.710 nomes, idênticos antes/depois |
| Build, formatação e diff check | passaram |
| Rustdoc workspace sem dependências, -D warnings | passou |
| Linhagem estrita V1/V5/V15/V26 | passou |
| Linhagem reversa | 965 consumers, zero erros |
| Linter arquitetural completo | zero erros, 380 avisos, 1611 notas |

Este lote não certifica a matriz remota de CI/release nem fecha os itens
funcionais da auditoria. Restam diagnósticos no core e nas outras crates;
as limitações de paginação, contexto e paridade exigem as suas próprias
medições. Nenhuma release foi publicada e nenhum pin foi atualizado.
