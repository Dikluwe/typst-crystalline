# V16 — lote de projeções de Value concluído

**Nove V16 corrigidos: 262 → 253 erros emitidos.** Oito projeções latentes
adicionais foram fechadas no mesmo owner; 17 decisões em quatro consumers e seus
quatro L0. O objetivo global de corrigir todas as pendências continua aberto.

## Correção e preservação

A entidade Value mantém suas 39 variantes e assinaturas. is_none, truthy e os
doze casts passam a nomear as variantes atuais que chegam ao resultado genérico.
As três projeções de diagnóstico em callee_error, format_options e document
preservam seus contratos próprios: document continua usando str; as outras
rotas conservam string onde já aplicável. Content/LocatedContent continuam
identificando o elemento no erro de callee, sem perder a projeção consultada.

Não foram alteradas fórmulas, unidades, limites, sinais, empréstimos, identidade
de Arc Version, payloads, chamadas ou mensagens. A conversão Float de Duration
continua em segundos e a de Int em nanossegundos. Esta etapa não corrige nem
certifica como paridade as bordas numéricas não medidas, incluindo NaN nessa
conversão. O enum produtivo, Type, igualdade/hash, representação dos dados e pipeline permanecem.
Não houve contrato público, flag, default ou quebra de compatibilidade novos.

Os grupos equivalentes compartilham um corpo. As **17
notas V19 novas** estão mantidas e prescritas nos L0, sem copiar corpos nem
confundir alternativas escritas com cobertura de testes. Sem allow/expect,
exceção ou relaxamento da política V16. A comparação remove apenas os nove
V16 previstos, sem diagnóstico novo fora de V19.

## L0, testes e prova estrutural

L0 atualizado antes do código/testes, medição antes de decisão e resselo
bilateral restrito. before.json.gz, classification.json e l0-before-code.json
registram ownership e fontes; materialization.json conserva cada complemento
nominal e replay de edição. stage.diff abrange apenas os oito arquivos previstos.

Os sete testes novos passaram antes da alteração dos padrões e na validação
final. Exercitam verdade/vazio, casts estritos, empréstimos, coerções, sinal,
overflow, inválidos de Decimal/Version/Regex e nomes de diagnóstico. São controles
do contrato vigente, não prova nova de paridade universal nem cobertura de cada
payload das 39 variantes. Não houve RED funcional; a manutenção preserva resultados.

O RED→GREEN é estrutural: em cópias reais do workspace, uma variante sentinela
adicionada apenas ao Value copiado passou de **0 para 17 E0004 nos pontos alvo**.
O total da cópia foi 7 → 24, sem outro tipo
de erro, usando cargo check do core real e sem stubs. Recibos/fontes em
probe-*-result.json e probe-*-input.json.gz. Nenhuma variante foi adicionada à
fonte produtiva. code-guard.json verifica escopo e preservação dos corpos,
com exceção da expansão prescrita de matches! em is_none para match exaustivo.

## Defeito do parser identificado e ainda pendente

O inventário atual encontrou nove emissões V16 cujo padrão é None explícito.
A fixture mínima em none-probe-result.json compila sem wildcard, mas o linter
pinado denuncia None. Acrescentar uma variante ao enum interno já produz E0004
nessa fixture antes de qualquer reparo: é um falso positivo do parser, que não
distingue esse constructor de binding. Os nove locais do projeto estão em
none-parser-findings.json, com os contextos AST de origem. Seus consumers não
foram reescritos para esconder a emissão. A correção do linter é uma próxima
ação concreta para o objetivo global, não foi declarada concluída nesta etapa.

## Validação final

**8633 testes aprovados, 0 falhas,
7 ignorados** na suíte completa. Build workspace, formato,
Clippy workspace/all-targets com -D warnings e -D unfulfilled_lint_expectations,
V1/V5/V15/V26 estritos, linhagem reversa, Rustdoc -D warnings e git diff --check
passaram. Os quatro índices HTML foram conferidos em rustdoc-indices.json.

O SARIF completo continua em exit 1 pelos 253 V16 emitidos, incluindo os nove
casos None acima. V19 tem 888 notas e V20 tem
871 notas. A comparação por regra/nível/arquivo/mensagem
ignora deslocamentos de linhas e conserva todos os diagnósticos externos;
architecture-delta.json e remaining-v16.json guardam os dados. T027 e T158/T160
continuam rastreados sem emissão, com suas fontes intactas.

## RAM e proveniência

Execuções pesadas sequenciais em cgroup: MemoryMax=12 GiB, MemorySwapMax=0,
CARGO_BUILD_JOBS=1. Maior pico: 12693929984 bytes (11.822 GiB), zero OOM
e OOM-kill. O pico permaneceu abaixo do teto configurado. Os recibos
*-resources.json preservam também os contadores memory.events.

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. Snapshot inicial UTC
`2026-10-17T00:59:26.667526+00:00`; validação final UTC `2026-10-17T01:10:00.332795+00:00`–`2026-10-17T01:19:19.146085+00:00`.
Status completo, git diff HEAD --stat, fontes e hashes antes/depois em
before.json.gz e final-state.json. Fontes/L0/configuração ficaram congelados
durante a validação. Linter pin `e53de6fd2bff9f3a87a3e8a076f5e470ce4bb7a0`, SHA-256 do binário
`9bc3a8521bc7db1dc0afffa04fc5d24012cf7ffbd5376bb8e9729242af2c40aa`. Suíte com vanilla pin
9f2b6e8715237cb086899a42873660fe744622e8 e binário SHA-256 ratificado
7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea;
ambiente/receita em workspace/workspace-pre.json.gz.
Nenhum commit, push ou release nesta etapa.

[Correções](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-v16-value-projections-2026-10-16/lista-correcoes.md) · [Pendências](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-v16-value-projections-2026-10-16/lista-pendencias.md) · [Parser](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-v16-value-projections-2026-10-16/parser-pendencias.md).
