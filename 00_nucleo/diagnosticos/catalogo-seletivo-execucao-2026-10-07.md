# Prioridade 3 — aquisição seletiva do catálogo: execução e medição

Registro final UTC: 2026-10-08T00:28:09.104266+00:00. Working tree não commitado no HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`. Inventário exato/diffstat/fontes recebidas em `catalogo-seletivo-2026-10-07/before-code.json.gz`; fontes finais testadas em `tested-final.json.gz`, comandos/UTC em `validation.json`, hashes em `seals.json`. `final-verification.json` confirma preservação dos demais consumers/L0 e identidade do release final com o medido.

O dono confirmou o pacote com “confirmo”; manifesto SHA-256 `045ef18d7a16143e640669cf5923db1a213eb677b6c278d4ee4a0bf542973c2e`. Instalados os dois L0 antes do código, sem alterar candidatos/contrato/manifesto ratificados. Mudanças produtivas restritas aos pares fontdb.rs/fontdb.md e novo fontdb/selective.rs/fontdb/selective.md. Sem alteração de API pública, L1, flags, fase, Slot, World, parser de coverage ou algoritmo de metadados. Sem commit/reset/fix global.

## Evidência, implementação e decisão

Antes da decisão, experimento de05/10 e auditoria preparatória comprovaram os campos de metadados usados e a redução do post sem trocar validade/pitch/índices. Assets históricos e corpo de font_info_from_bytes permanecem iguais; os tempos históricos não são somados ao ganho da prioridade2. fontdb0.23.0 lib.rs:721–727,900–920 distingue File de SharedFile retido; ttf-parser0.25.1 post.rs:334–379 valida versão/header/array, sem usar o tail de glyph names para metadados. Aquisição/buffer/igualdade DTO são mecanismos; família/variant/coverage/seleção/render são observáveis da língua. Não inferimos intenção upstream.

Source::File regular em Unix tenta o child privado: lê diretório sfnt/TTC e head/hhea/maxp/name/OS2/post/cmap. Tabelas sem uso no catálogo ficam omitidas no buffer temporário. Post conserva o prefixo validado32 bytes ou34+2×count na v2, verificando antes o range integral original. Preserva ordem/duplicatas, índice original e offsets absolutos TTC; rebaseia para sfnt-at-zero. Usa os extratores vigentes, com coverage por intervalos. O retorno contém apenas FontInfo própria; bytes/Face/handle não seguem para FontSlot/shaping/export.

Limites:4.096 registros sfnt,8MiB buffer e16MiB de bytes solicitados incluindo prefixos/rereads, verificados antes de leitura/alocação/seek; checked arithmetic/slices e try_reserve. Captura dev/ino/len/mtime/ctime com nanosegundos e verifica handle/path após extração. Não promete snapshot atômico com escritor concorrente. Fonte fora dos limites, formato/layout/índice não coberto, erro de I/O/reserva/metadata/parse ou identidade alterada declina ao callback with_face_data existente. Nenhuma nova política eager de FontSlot.

SharedFile conserva exclusivamente os bytes retidos pelo Database, incluindo caminho alterado/ausente; Binary continua excluído. Outras plataformas seguem pelo caminho anterior. Ordem, duplicatas, filtros de metadata None e inserção emparelhada slot/book conservados.

## Testes e controles

RED runtime em red.log: projeção válida e redução de leituras falharam com stub antes da implementação; cinco controles de rejeição/fallback ficaram verdes. GREEN final inclui11 testes do child +dois novos do parent,18 na frente fontdb ao somar os cinco existentes. Cobrem fixtures/variáveis/OTF/nomes/exceções/TTC/post/offsets/bounds/caps/Read/Seek/identidade/duplicatas/fallback e SharedFile. O teste de SharedFile usa outra fonte válida no path, demonstra que seu metadata difere e confirma retorno dos bytes retidos também após remover o arquivo.

7.954 testes aprovados:6.637 core,1.217 infra,90 shell e10 CLI query; cinco core históricos ignorados. Builds debug/release e lint sem erros;535 warnings históricos e1640 notes. Status/argv/UTC em validation.json; novas linhagens válidas. Logs intermediários preservam um erro de nome de método no teste, corrigido para a API infos existente; não foi usado como evidência RED.

Comparadas 1140 faces: 1112 sistema, 17 embutidas, 11 fixtures. Todos os metadados/coverage equivalentes, inclusive None da fixture sem nome. Fast path aceitou 1139 faces, exatamente todas as 1139 selecionáveis. Book, slots, ordem e cada path/índice também iguais. Leitura solicitada por lote de sistema: 51,122,918 bytes; isto não mede I/O físico, nem equivale a bytes efetivamente tocados no mmap legado. Caps não excluíram as faces selecionáveis deste ambiente; não é promessa para qualquer fonte futura.

Todos os oito fixtures ×três rodadas preservaram stdout/stderr, textos, páginas/dimensões, famílias/tamanhos/cores e caixas perante anterior. Image/shapes/multiscript também conservaram raster a144dpi perante anterior. Negativas vanilla ficam explícitas: context/text-dense/multiscript não qualificam integralmente geometria/fonts; raster image/multiscript difere de vanilla; shapes coincide. Igualdade mecânica DTO/PNG é controle adicional, não definição universal de paridade.

## Proveniência e recoleta

Release final SHA-256 `96f22a1d63d51bfcc258ca2849563b037e66a67b7e34febaf4779da2dd146a4b`; anterior prioridade2 `e44043d9471a6d2eec25da7562dd5b0a071d4e14ab0f307011ca978e46e49109` congelado. Vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`, binário `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`, proveniência P1489. Não usar só string de versão/raiz como oracle.

O teste SharedFile foi fortalecido após a primeira coleta: o prefixo runtime permaneceu idêntico, mas o release recompilado ganhou hash diferente. Por isso as medições foram repetidas com o binário final. Initial performance/ e probe-results/, scripts/recibos e binaries congelados permanecem; resultados finais são performance-final/ e probe-results-final/, controles controls-final.json/visual-final/. repeat-reason.json registra o motivo; não selecionamos subconjunto de amostras nem apagamos as negativas iniciais.

Probe usa módulo produtivo diretamente; catálogo atual é a biblioteca produtiva, controle anterior usa with_face_data com os mesmos metadata/coverage por intervalos vigentes. Instrumented.rs tem prefixo produtivo verificado e apenas facade diagnóstica fora dos timers para contagem; nenhum hook público de produção. Discovery é excluída da aquisição e incluída no catálogo. Três rodadas/36 pares alternados por modo. Bytes/assets/fontconfig/source/HEAD/dirty/argv/UTC e executable SHA nos identities/estados.

CLI:CPU4/Rayon1/vanillajobs1,30s quiet gate e monitor de concorrência/hashes; três rodadas/12 trios,6 permutações e2 warmups por engine/bloco. 864 amostras +144 warmups,24 blocos válidos. Wall inclui GNU time/taskset iguais; RSS após exec. Sem cold-disk/watch/incremental. Repetibilidade requer diferença média-mediana e faixa de medianas<3%; valores que falham são indicativos.

## Resultados finais

Aquisição + metadados + coverage, sem descoberta: mediana 55.39→51.25ms; redução pareada 7.87%. Faixa de medianas entre rodadas anterior/atual 1.54%/1.15%; razão pareada 0.96%. Dispersão impede tratar os números absolutos como promessa precisa.
Catálogo completo: mediana 94.64→91.15ms; redução pareada 3.75%. Faixa de medianas entre rodadas anterior/atual 1.22%/1.21%; razão pareada 0.50%. Passa o critério de faixa das medianas e diferença média-mediana abaixo de3% em ambos os controles.

| Fixture | Anterior ms | Atual ms | Vanilla ms | Delta pareado vs anterior | RSS anterior→atual MiB | Repetibilidade anterior/atual <3% | Geometria equivalente a vanilla |
|---|---:|---:|---:|---:|---:|---|---|
| context | 127.48 | 124.19 | 141.89 | -2.76% | 44.21→44.13 | não | não |
| image | 9.14 | 9.53 | 11.30 | +4.02% | 18.36→18.40 | não | sim |
| multiscript | 260.85 | 258.96 | 184.80 | -1.51% | 143.30→143.29 | não | não |
| pipeline-single-font | 118.99 | 115.90 | 135.25 | -4.52% | 42.78→42.78 | não | sim |
| shapes | 12.85 | 12.14 | 11.70 | -1.65% | 19.92→19.96 | não | sim |
| table-alias | 144.35 | 141.58 | 150.13 | -2.83% | 45.30→45.48 | não | sim |
| table-local | 191.37 | 183.90 | 177.23 | +2.75% | 45.42→45.54 | não | sim |
| text-dense | 158.36 | 156.85 | 151.28 | -2.31% | 44.28→44.29 | não | não |

Gates repetíveis com ganho: nenhum no gate estrito; conservar como indicação.

Aquisição teve um outlier que fez a diferença média-mediana atual chegar a4,345% na segunda rodada; embora medianas variem menos de3%, seu ganho de7,87% deve ser indicativo. Não houve exclusão de amostras. Ganhos incrementais desta coleta são perante prioridade2, não perante R1 nem somáveis aos percentuais anteriores. RSS ficou praticamente igual, sem economia relevante de RAM demonstrada. JPEG e shapes sem texto não executam o catálogo desta frente; conservar seus deltas/ICs e dispersão, sem atribuição causal não perfilada ou promessa de aceleração universal. CLI/fontes/catálogos mantêm diferenças existentes perante vanilla; esta otimização não fecha render/Styled/Table universalmente.

Decisão: manter a tentativa seletiva padrão ratificada pelo ganho repetível do catálogo integral com book/slots/metadata/coverage e outputs anteriores equivalentes. Catálogo passa faixa das medianas e diferença média-mediana <3% em ambas implementações; IC95 da razão [0.9607737151294041, 0.9679808166412397]. A CLI final não passou o gate estrito em nenhum fixture: deltas são indicativos e não autorizam afirmar ganho geral de compilação. JPEG teve +4,02% pareado nesta coleta (cerca de0,39ms nas medianas, IC acima de1), sem repetibilidade; Table local +2,75% com IC incluindo1, inconclusivo. Essas negativas permanecem registradas, sem atribuição causal certificada. O teste fortalecido alterou binário; não promover o ganho CLI repetível observado no release inicial como certificado do release final. Prioridade3 materializada no escopo de aquisição seletiva, sem nova API e sem commit.

Provas finais: final-verification.json, validation.json, seals.json, controls-final.json, performance-final/{identity,index,summary}.json, probe-results-final/{identity,validate,acquire,catalog,execution}.json, snapshots/estados/assets e logs.
