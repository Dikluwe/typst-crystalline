# Eixos variáveis no PDF — correção da herança de preparação

Rodada iniciada em 2026-10-05 e concluída em 2026-10-06. Correção interna de paridade em L3, sem unsafe novo, assinatura pública, flag ou fase de pipeline nova. Cinco consumers com seus cinco owners L0; sem resselo global.

## Medição antes da decisão

Controle causal: `#set text(font: "Ubuntu Sans", size: 20pt)` seguido de `#text(weight: 700, variations: (wght: 250))[Hello] #text(weight: 400, variations: (wght: 900))[Hello]`. O vanilla pinado mede 46,20/51,14pt; antes desta rodada o Tekt media 47,16pt em ambos. A sonda isolada e a sonda com catálogo completo conservam peso e eixos desde a avaliação; o shaping mede 2310/2557 unidades por em=1000, correspondente ao vanilla. Portanto, a hipótese de perda na avaliação/seleção/shaping foi refutada para este caso.

`03_infra/src/export/builder/fonts.rs` fazia subset da fonte original e só depois chamava o instancer. `03_infra/src/export/subset.rs` usava subset sem coordenadas; a saída já não tinha fvar/gvar/HVAR. `03_infra/src/font_variant.rs:209-213` retorna Some dos mesmos bytes quando fvar falta: não houve instanciação, mas o builder tratava esse retorno como sucesso. /W embutia H=697 em vez de H=689/734; nominal/TJ usava avanços variáveis, compensando contra uma fonte default. Extração das tabelas e controles independentes estão nos scripts e snapshots da pasta desta rodada.

Histórico: P529/P530 validaram subset-first com **oxifont-subset**, que conservava fonte variável pequena. P874 trocou a biblioteca para subsetter. A premissa de P530 sobreviveu, embora a API atual congele a instância default. Isso explica a incompatibilidade presente; não prova o primeiro commit histórico que manifestou o defeito nem intenção de preservar tal comportamento. A referência de língua continua vanilla `9f2b6e8715237cb086899a42873660fe744622e8`; `text.rs:79-94` entrega fonte e coordenadas conjuntamente ao PDF. Não copiamos a mecânica do vanilla.

## Correção

A preparação multifont fornece fonte original e coordenadas derivadas+explícitas ao adapter privado de `subsetter::subset_with_variations`, já disponível em 0.2.6 e sem dependência nova. O remapper continua transportando GID original→CID; /W e nominal/TJ passam a referir a mesma instância. Fallback a full embedding instancia os **bytes originais**, nunca o subset congelado. CFF2 passa a TrueType nesta rota; descriptor/FontFile já são escolhidos pelos bytes finais. O pedido default vazio também entra na API variável privada, corrigindo o recurso de espaço CFF2 antes embutido inteiro e rejeitado pelo Poppler. Wrappers públicos antigos e rota CIDFont única conservam seus escopos.

A suíte revelou uma suposição mecânica no teste P772u: exigia GID 223 em /W quando CFF2 ainda não era subsetado. Agora lê o CID realmente emitido no TJ e preserva integralmente `/W(CID) − delta TJ = x_advance`. Não foi removida a verificação de deslocamento.

A inspeção visual encontrou uma segunda causa, invisível nas caixas: `style.weight=700` ainda acionava `2 Tr`/0,800pt de stroke sobre a instância real wght250. Os pontos glyf e hmtx estavam corretos. A emissão verbose individual e coalescida usa 0 Tr quando **TextShaped tem fonte certificada com eixo wght**, sem alterar weight/bold semântico. O helper verifica a fonte somente se o estilo pediria stroke. Text/Type1, None e fontes estáticas conservam a política anterior; a revisão geral do faux-bold estático fica aberta. Compact shaped já não adicionava esse stroke.

## Verificação

- RED real das larguras PDF em `red3.log`; GREEN dos dois modos em `green.log`.
- RED real do stroke em `stroke-red3.log`; regressão individual/coalescida + None no teste final.
- **1.172 testes L3 passaram**, nenhum falhou; build debug e release passaram. L1 não foi alterado nesta rodada.
- **58/64** controles passaram texto normalizado e caixas agrupadas (tolerância 0,05pt), contra 52/64 antes; seis melhoraram e nenhum positivo regrediu. Todas as fontes/strings/comandos/bboxes/stderr estão em verification.json.gz. Controles incluem explícito contra derivado, wght fracionário/fora de faixa, wdth herdado com override de wght, fonte estática e wrap.
- Ubuntu Sans/TrueType e Cantarell/CFF2 conferidos visualmente a 600dpi: mesmas caixas de tinta, **zero pixels escuros sem correspondência dentro de 1px em ambas as direções**, limiar 180. Não é alegação de igualdade de bytes nem pixel a pixel. Raster intermediário registra a refutação causada pelo stroke sintético.
- Poppler não emite aviso no CFF2 final; o baseline emite três avisos. Os avisos no log bilateral final pertencem ao baseline.
- Lint: **0 erros**, **531 warnings e 1.576 notes preexistentes**, multiset de diagnósticos idêntico ao final da frente anterior; V15/V26 sem resultados. Isto não significa zero violations globais. `git diff --check` passou.

## Tempo da CLI inteira

Coleta final 2026-10-06T03:03:12.321470+00:00: dois aquecimentos e 15 rodadas pareadas com ordem aleatória; SOURCE_DATE_EPOCH=0, RAYON_NUM_THREADS=1, sem build/lint/render concorrentes. Inclui fontes, compilação e PDF; não atribui o delta a L1. Medianas em ms:

| Controle | Antes | Depois | Vanilla | Delta pareado | IQR do delta |
|---|---:|---:|---:|---:|---:|
| grid320 | 196.43 | 196.37 | 148.17 | +0.10 | -2.78 a +2.01 |
| variable-axes | 185.03 | 185.08 | 128.19 | +0.49 | -2.82 a +2.47 |
| variable-cff2 | 508.36 | 179.97 | 126.79 | -328.57 | -337.41 a -325.62 |

O ganho forte nesta fixture CFF2 foi reproduzido nas três coletas (arquivos first-/pre-stroke-/benchmark.json). Ele combina a preparação nativa e a correção do recurso default; não foi feito A/B isolando cada contribuição. A tabela e a fixture TrueType não demonstram ganho consistente. O guard de fontTools existente e o fallback permanecem; não foi removida a dependência runtime do produto inteiro.

## Proveniência e limites

HEAD **06848cdcb5450f2c555626bf116bc7acd82bd82b**, **working tree não commitado**. Estado anterior em before.json; estado produtivo final, textos integrais dos cinco owners/consumers, UTC, git diff HEAD --stat/status e logs em execution.json.gz. verification.json.gz e benchmark.json guardam o estado exato de cada coleta; seals.json contém SHA de código/L0. summary.json registra todos os SHA das fixtures de fontes. Executáveis/receitas de sondas em probes.json; as bibliotecas core/infra debug foram reconstruídas depois e não se atribui a elas um hash pós-build como se fosse o anterior.

Binário anterior: `7a089e7dc9f0189cd6fa999002ac5652ef05531bfb16d8e3cd3439ef08d9343c`. Final: `58f40a987ce5e50916b4112874acee9cec29832d6196975b443100b8b5de68e1`. Vanilla pinado: `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`; não se usa /usr/local/bin/typst nem só a versão pública.

Os seis controles ainda divergentes são Block/Align (três), extração de controles bidi autorais (dois) e espaço literal terminal (um). Esta rodada corrige a perda de coordenadas no subset multifont e o stroke redundante para fontes variáveis certificadas; não certifica todos os eixos/fontes, optical sizing, fallback de instanciação com falha, rota CIDFont única variável, faux-bold estático ou paridade geral de PNG/SVG. Coverage::from_ranges continua proposta separada, sem materialização.

Próxima pendência delimitada: direção/alinhamento de Block e precedência de Align, medindo o transporte do contexto até subframes antes de decidir outro contrato.
