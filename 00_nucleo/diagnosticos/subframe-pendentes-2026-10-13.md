# Pendentes de linha nos subframes isolados

Corrigidos vazamento e consumo cruzado de frações/repetições entre região
isolada e linha externa. O subframe também passou a finalizar as frações
e repetições da última linha antes de devolver seus itens.

## Proveniência e medição anterior à decisão

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. Snapshot inicial
em `2026-10-14T02:02:37.498012+00:00`: fontes, owners, status e git diff
HEAD --stat em before.json.gz. A rodada anterior fez progresso verificável:
concluiu a suíte completa e preservou evidências do isolamento de overflow.
Este lote examinou o requisito seguinte para compor Box: isolar também os
pendentes cujos índices pertencem à linha trocada.

sub_frame.rs trocava current_line sem trocar pending_fr,
pending_fr_natural_widths, pending_repeats e justify_opportunities. O
fechamento de uma linha local podia consumir índices do pai. O drain final
manual devolvia frações/repeats locais sem expansão, deixando pendentes
referindo-se à linha externa restaurada. Os anchors anteriores à decisão
estão no L0 e nas fontes completas preservadas.

O L0 foi atualizado antes dos testes/implementação. Na versão final dos
quatro testes, a execução contra o código anterior falhou integralmente;
contra o reparo todos passaram, em `2026-10-14T02:07:39.052558+00:00`.
Os estados e hashes exatos estão em red-green-final.json.gz. red-state e
first-candidate preservam as tentativas anteriores, incluindo suas falhas.

## Correção e aceitação

O caminho isolado salva/limpa/restaura os quatro buffers junto com sua
linha. A cauda local expande frações/repeats antes de alinhamento e drain.
O caminho inline não foi alterado. Os testes cobrem ambas as origens,
fração local terminando na borda direita, repeat local com oito peças nas
posições calculadas, flush interno sem consumir a fração/repetição do pai,
conteúdo sem duplicação e linha externa finalizada igual ao controle que
não executa o subframe.

Uma expectativa adicional do primeiro candidato expôs uma diferença de
preenchimento mesmo sem subframe: o teste esperava 14 peças e recebeu 13.
Não se trocou o esperado por 13: o teste de isolamento passou a usar o
controle sem filho, preservando seu assert independente de borda direita.
A primeira hipótese era uma soma ausente em line_commit.rs:189-195.
Entretanto, repeat-controls.json a qualifica: no vanilla Repeat isolado
ocupa a região e move o texto seguinte para outra linha; no Tekt ele é
inline, como ainda especifica seu L0. A diferença não legitima uma simples
soma sem decidir primeiro o contrato de fluxo. A questão permanece aberta,
sem alegação de paridade geral de Repeat ou de correção daquela fórmula.

## Sondas fonte→PDF

Página 120×100pt, margem 10pt, fonte 11pt; fontes/comandos completos em
before-renders.json e after-renders.json. Em place-fr, B antes terminava
junto de A, em x=24.113pt; depois termina em x=110.001pt, na borda direita
esperada de 110pt, com diferença de 0.001pt na extração PDF. O vanilla
pinado termina em 110pt. Repeat local passa de um ponto para 41, como no
controle pinado; não se afirma identidade de bytes ou ausência de erro
de serialização.

Na sonda parent-repeat-child-flush, o texto A do filho era repetido 38
vezes usando o repeat do pai. Depois aparece uma vez, assim como B; a
repetição permanece no pai. As sondas com Place e prefixo ainda divergem
do vanilla no fluxo/baseline externa e no preenchimento do repeat. Elas
provam a remoção do consumo cruzado, não fecham todos esses comportamentos.

Binário final `e2b75b298c46d0a7962f44cfb21e4ffc2c26f1ee7dec9ba35b6eb73c4268d905`; referência vanilla
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`, pin
9f2b6e8715237cb086899a42873660fe744622e8. Os snapshots vinculam cada
medição ao estado fonte e aos hashes.

## Gates

Em `2026-10-14T02:09:40.369500+00:00`, build, check workspace/all-targets, formato,
Rustdoc -D warnings, linhagem estrita/reversa e diff check passaram.
Arquitetura: 0 erros, 380 avisos e
1615 notas. Inventário Clippy: 228 diagnósticos, sem delta;
cap-lints warn não equivale ao gate CI -D warnings, ainda aberto.

Suíte completa do workspace concluída em `2026-10-14T02:15:53.822482+00:00`:
**8365 passaram, 0 falhas e 7 ignorados**.
Cargo offline/locked, todos os pacotes, execução serial, tmpfs, linter
publicado pinado e oráculo comprovado. workspace-pre.json.gz e
workspace-result.json registram ambiente/comando/hashes e confirmam que
as fontes permaneceram inalteradas durante a suíte.

Após a suíte, a hipótese sobre Repeat foi qualificada no L0 conforme as
sondas adicionais. Em `2026-10-14T02:17:21.099395+00:00`, resselo, linhagem estrita,
linhagem reversa e diff check passaram novamente. A comparação normalizada
confirma que o código/testes só mudaram na linha @prompt-hash; nenhum
comportamento executável mudou. post-suite-qualification.json.gz conserva
fontes/L0 finais, comandos e essa prova, sem substituir os snapshots testados.

## Trabalho restante e evidências

A geometria dos produtores de block/box ainda está aberta. As sondas
[box-dimensoes-2026-10-13.md](box-dimensoes-2026-10-13.md) acrescentam
provas de inset/outset, perda de baseline e clipping horizontal ignorado,
com controle positivo da caixa vazia. Também permanecem conic, demais
itens da auditoria, Clippy e matriz remota de CI/release. Não houve commit,
push, publicação, mudança de contrato público ou fase.

Snapshots .json.gz preservam os estados. evidence.json.gz contém logs,
resultados, diffs, receitas, fontes Typst e PDFs/XML com SHA-256 conferido
após descompressão. Não usar estes resultados para declarar produção
concluída nem para inferir uma proporção global código versus prompts.
