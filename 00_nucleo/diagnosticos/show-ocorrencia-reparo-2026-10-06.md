# Reparo de show por ocorrência — 2026-10-06

O gate de compatibilidade de `00_nucleo/prompts/compiler/eval/rules/apply.md`
foi confirmado pelo dono. A proteção produtiva agora distingue o alias `it`
de um elemento recém-construído, mesmo quando os corpos são iguais. As quatro
gerações ilimitadas da auditoria original passam de aceitação indevida a
`maximum show rule depth exceeded`, com os dois hints do vanilla.

## Proveniência e autorização

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitada.
O L0 aprovado tinha SHA-256
`aa9279ffba25c39ce8054acbec126fe6176acbca675fcd94013846ce4498b6b6`;
seus bytes exatos e a confirmação estão em
`show-recursao-auditoria-2026-10-06/apply-l0-approved.md.txt` e
`l0-proposal.json`. Refinamentos privados foram especificados antes do código,
com núcleo compartilhado `00_nucleo/prompts/_nuclei/show-occurrence.toml`.
Não houve nova assinatura pública, entidade, flag, fase ou unsafe.

A coleta final foi encerrada em 2026-10-06T14:35:58.444118+00:00. Cada coleção
registra seu próprio UTC, comandos, fontes, HEAD e git diff HEAD --stat/status.
`show-recursao-auditoria-2026-10-06/repair/execution.json.gz` conserva os oito
consumers alterados, seus L0s, núcleo e hashes dos logs. Não há commit deste lote.

Binário Tekt congelado em `/tmp/tekt-show-occurrence/typst-final`, SHA-256
`859abc64883582a3686a3ab5d3d537cb02bbe0e63af33ea1c52d6f3ea039d8c2`.
Oracle `lab/typst-original/target/release/typst`, pin
`9f2b6e8715237cb086899a42873660fe744622e8`, SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
O baseline anterior do Tekt tem SHA-256
`d448be2e5831441836dba19333c0d74989a64c16c6cb72bcdd17075fe7ac80b8`.

## Implementação e responsabilidade

`eval/rules/apply.rs` protege pares Location/RuleId em estado privado da realização:
entrada em andamento protege o alias; produto concluído pode ser reutilizado.
Construções novas conservam elegibilidade. O limite acompanha transformações
ativas, sem encerrar por igualdade morfológica. O histórico de full_error serve
somente à classificação adicional do diagnóstico. O helper isolado de terminação
legado não foi alterado.

Visuais Realized concluídos delimitam suas ocorrências já preparadas. A travessia
não substitui o produto inteiro dentro de cada alias emitido. Isso corrige a
amplificação encontrada no primeiro controle com block. O registro conserva
originais para introspecção, produtos, ordem causal e identidades; não reavalia o
programa nem usa igualdade de corpos como identidade.

`realize/mod.rs` reserva todos os IDs de regras capturadas antes de executar
receitas. `eval/mod.rs` inicializa os estados privados. `markup_stream.rs` realiza
localizáveis interpolados no ambiente lexical da inserção; texto e parágrafos
mantêm seus coordenadores próprios. `call_dispatch/invoke.rs` e `markup.rs`
conservam a origem real de erros de chamadas/heading quando ela está disponível.
Não fabricam origem para um Content constante que não a transporta.

`eval/tests.rs` acrescenta controles de gerações infinitas, aliases em containers,
composição, regras locais, efeitos e ranges causais. Expectativas produtivas
P348/P350c da política revogada foram atualizadas conforme o gate; a cadeia finita
a→b→c→it e os testes do helper legado permanecem. `wiring/tests/cli.rs` corrige dois
canaries antigos: hyphenate já implementado não deve emitir warning. As fixtures
usam agora o aviso de linguagem `no text within stars`, mantendo o conteúdo, exit,
arquivo, mensagem específica e ordem warning/error.

## RED, controles e revalidação

Os registros abaixo ficam em `show-recursao-auditoria-2026-10-06/repair/`.

- `red.log`: a geração nova era aceita; o controle finito já passava.
- `controls-first.json`: alias em block emitia quatro vezes, e uma regra local
  não atuava sobre it. Os controles finais emitem duas vezes e aplicam Local.
- `span-red.log`: a recursão de markup não conservava o range = Z. O teste final
  valida os ranges reais de markup e heading[Z].
- `core-delivered.log`: **6.551 testes L1 passaram**, cinco ignorados;
  mais oito testes de integração e três doctests passaram.
- `outer-libs-delivered.log`: **1.183 testes L3** e **87 L2** passaram.
  Integrações L3, bibliotecas L4 e demais controles estão em outer-sealed.log;
  as falhas antigas de canary e de loopback dessa execução são preservadas.
- `cli-delivered.log`: **105/105 testes CLI**, serializados, passaram, incluindo
  watch/HTTP. A escalada aprovada permitiu apenas a execução fora do bloqueio de
  loopback que causava PermissionDenied no sandbox.
- `build-delivered.log` e `release-delivered.log`: builds debug/release concluídos.
- `lint-delivered.json`: **zero erros, 531 warnings e 1.577 notes**. Não equivale
  a zero violations globais. Os owners deste lote estão selados e sem V5/V6/V15/V26.

`show-eight.json.gz` revalida os oito casos originais: quatro rejeições ilimitadas
e quatro aceitações finitas coincidem. `controls.json.gz` amplia a observação a
35 casos: 32 coincidem no critério explicitado de aceitação/texto, query ou DOM;
três diferenças anteriores foram reproduzidas também no baseline congelado em
`legacy-differences-baseline.json`. Esse critério não certifica toda a geometria
nem a igualdade completa dos diagnósticos dos 35 casos.

`layout-verification.json.gz`: **228/228 fixtures** conservam texto comparado
sem espaços/controlos de direção de formatação e caixas agrupadas com diferença
inferior a 0,05 pt contra o pin. Extrações e caixas cruas permanecem no arquivo;
não é uma nova certificação raster de todo o corpus.

`macro.json.gz`: **31 MATCH e cinco DIFF em 36 casos**, mantendo comparação crua
nos diagnósticos. As cinco diferenças agora são caminhos absoluto/relativo.
`macro-with-path-projection.json.gz` demonstra igualdade de mensagens, ranges,
carets e hints após substituir exclusivamente o caminho conhecido da fonte.
O script aplica o caminho mais longo primeiro, para não deixar um prefixo relativo
quando o caminho absoluto é sufixo dele. A projeção não altera o status cru nem
é contabilizada como 36 MATCH exatos. Não representa percentual global de paridade.

## Pendências delimitadas

1. Um heading constante construído antes da captura agora é corretamente rejeitado
   se sua substituição gera recursão ilimitada, mas falta a origem da construção
   no diagnóstico. O oracle conserva essa origem. Requer auditar transporte de
   proveniência, sem reconstruir span pelo corpo ou texto do erro.
2. Caminhos dos diagnósticos continuam diferentes na comparação crua da matriz.
3. Duas fixtures de math.mat fora de markup matemático omitem delimitadores no Tekt;
   ambas já divergem no baseline anterior. Seus efeitos/asserts executam corretamente.
4. `show table.cell` continua sem seletor suportado no Tekt, também no baseline.

A correção demonstrada é de comportamento e reutilização por ocorrência. Este lote
não mede ganho de tempo; o custo das travessias e do cache permanece uma pergunta
para um ensaio pareado de documentos linguisticamente equivalentes.
