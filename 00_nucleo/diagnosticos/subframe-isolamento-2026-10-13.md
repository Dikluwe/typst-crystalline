# Isolamento do overflow de subframes

Corrigidos predicados de overflow ordinário que podiam avançar a página
externa durante layout isolado. O corpo excedente agora permanece no
frame local para o chamador dimensionar, posicionar e recortar.

## Proveniência e decisão

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado.
Snapshot inicial `2026-10-14T01:45:11.321093+00:00` em before.json.gz: fontes, owners,
status e git diff HEAD --stat. red-state.json.gz preserva o estado dos
testes antes da correção; pre-validation.json.gz e workspace-pre.json.gz
preservam os estados verificados, hashes e ambiente. Nenhuma fonte mudou
durante a validação estática ou a suíte completa.

A investigação dos containers revelou que shape.rs, cursor.rs,
line_commit.rs e region_progress.rs ainda admitiam avanço externo mesmo
com is_sub_frame ativo. sub_frame.rs isolava buffers e cursores, mas não
podia recuperar páginas já comitadas pelos descendentes. O contrato de
isolamento já justificava o reparo; a precisão das responsabilidades foi
registrada primeiro nos cinco owners L0 e no núcleo subframe-overflow.
Os anchors da medição anterior à decisão estão nesses L0 e nos snapshots.

## Reparos e regressões

Cinco condições de avanço/rejeição passaram a distinguir o subframe do
fluxo paginado. next_region e pedidos explícitos de pagebreak não foram
alterados. O código produtivo de sub_frame.rs permaneceu intacto; recebeu
os testes e seu resselo de linhagem.

Os testes de forma maior que a região e texto multilinha falharam antes
do reparo; o controle de paginação externa passou. Depois, os três
passaram. Os controles exercitam origem geométrica e de baseline,
retenção de todos os itens e restauração do prefixo, cursores e dimensões
do chamador. Os esperados e a pequena altura de teste foram preservados.

## Validação

Validação estática em `2026-10-14T01:50:11.060416+00:00`: build, check de todos
os targets, formato, Rustdoc com -D warnings, linhagem estrita/reversa e
diff check passaram. Arquitetura: 0 erros,
380 avisos e 1615 notas. O inventário Clippy
permanece em 228 diagnósticos; sua execução com cap-lints warn não fecha
o gate de CI com -D warnings.

Suíte completa do workspace concluída em `2026-10-14T01:56:13.903280+00:00`:
**8361 passaram, 0 falhas e 7 ignorados**.
Comando cargo test --offline --locked --workspace --no-fail-fast --
--test-threads=1; ambiente, tmpfs, oráculo pinado, hashes e resultados
individuais estão em workspace-pre.json.gz e workspace-result.json.

## Limites e trabalho restante

Repetir as sondas fonte→PDF de containers mostrou que block-shape-moved
passou de duas páginas para uma, ainda sem pintura. Os demais controles
continuam com os problemas registrados. A correção do isolamento não
fecha a composição de block/box: coordenadas locais, coleta de itens,
dimensões e recorte ainda precisam de reparos específicos. A medição
posterior e o hash do binário estão em containers-clip-2026-10-13/
after-renders.json, preservados no arquivo de evidências daquele lote.

Também permanecem abertos conic, outros itens da auditoria funcional,
os avisos de Clippy e a matriz remota de CI/release. Sem commit ou
publicação. Este lote confirma defeitos no código, mas não estabelece
uma proporção global entre defeitos de código e de prompts.

## Evidências

Os snapshots comprimidos preservam os estados. evidence.json.gz reúne
logs, resultados, diffs e receitas com hashes verificados após leitura.
