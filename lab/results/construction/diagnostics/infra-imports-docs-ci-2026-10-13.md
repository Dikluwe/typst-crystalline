# Infraestrutura: imports, documentação e validação completa

Removidos 66 diagnósticos do inventário do workspace: 35 de imports sem
uso, 29 de continuação de listas Rustdoc, um atributo duplicado e uma
linha vazia após documentação. O total passou de 406 para 340; na
infraestrutura, de 342 para 276. Não houve supressão adicional no código
ou mudança do gate de CI.

## Proveniência e medição anterior à decisão

Branch Tekt, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, **working tree
não commitado**. `infra-imports-docs-ci-2026-10-13/before.json.gz` contém
fontes, L0, spans atuais conferidos, diagnósticos originais do compilador,
UTC, status e diffstat. A baseline vem do inventário completo
`iteracao-parametros-ci-2026-10-13`. O registro inicial de testes de infra
foi capturado antes de alterar código. `additional-before.json.gz` registra
a inclusão posterior de stream.rs no escopo, antes de sua alteração.

## Mudanças e fronteira de comportamento

Os imports privados sem consumidor foram retirados dos adapters, agregadores
e fixtures. Os helpers de gradientes e PNG usados pelos testes do export,
e os helpers de pipeline usados pela suíte descendente, continuam
acessíveis sob cfg(test). Reexports públicos e implementações permanecem.
Os builders mantêm seus acessos qualificados aos helpers de bitmap.

As listas numeradas dos comentários de testes receberam a indentação
Markdown correta. O parágrafo P840 de fonts foi separado da lista anterior;
a frase incompleta sobre is_italic foi encerrada sem inventar a justificativa
que faltava. Em fallback, o sinal de mais textual virou conjunção para não
iniciar uma lista. Foi retirada a linha vazia entre a documentação de
collect_fonts_in_items e a função. Nenhuma fórmula, fixture ou asserção
mudou.

O cfg(test) interno duplicado de p1431_line_cap_tests foi removido. A
declaração do módulo no pai continua guardada por cfg(test), mantendo a
mesma suíte e observações de raster/PDF para Butt/Round/Square.

O lote afetou 18 consumers e seus owners. Os L0 foram atualizados antes
das alterações e o resselo foi restrito a esses consumers. A comparação
suplementar `body-preservation.json` confirma igualdade textual das linhas
restantes após excluir comentários de linha, imports, linhas vazias e
atributos cfg(test). É uma verificação mecânica delimitada, não prova de
paridade da linguagem; a suíte completa é a evidência de regressão.

## Refinamento descoberto pela compilação

A primeira checagem de todos os targets falhou porque PageContext, embora
sem uso na biblioteca normal, era necessário aos testes internos de
stream/text. O import foi restaurado sob cfg(test); nenhum teste foi
removido para contornar o erro. O log inicial permanece em
`check-initial-failed.log` e o snapshot em `initial-pre-validation.json.gz`.

Retirar imports do facade também expôs reexports privados redundantes:
os seis aliases de gradientes requeridos apenas pela suíte passaram a
cfg(test), enquanto suas definições e chamadas produtivas internas
permanecem. O alias draw_item_local de stream.rs foi removido; draw
conserva sua função e recursão. Esses refinamentos tiveram L0 anterior ao
código, snapshot próprio quando necessário e novo resselo. A rodada final
compilou todos os targets e preservou os 1.292 nomes de testes de infra.

## Validação final

Checagens estáticas concluídas em `2026-10-14T00:48:47.772439+00:00`. A suíte completa do
workspace terminou em `2026-10-14T00:55:10.385581+00:00`, exit zero, com **8.361 testes
aprovados, zero falhas e 7 ignorados**, distribuídos em
32 resumos de execução. Testes ignorados não recebem
crédito de execução. Os logs detalham cada teste e seu status.

A execução seguiu o ambiente Linux previsto pela CI: testes seriais,
TMPDIR isolado em /dev/shm, Cargo offline/locked, linter publicado pinado
no PATH e TYPST_VANILLA apontando para o binário comprovado do pin
`9f2b6e8715237cb086899a42873660fe744622e8`, SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
O diretório temporário pertenceu somente a esta execução. O acesso local
necessário a tmpfs/sockets dos testes foi autorizado pelo mecanismo de
aprovação automática.

`workspace-pre.json.gz` registra hashes de todas as fontes Rust das camadas
e benchmarks, estado Git, ambiente e comando. Os hashes permaneceram iguais
até o fim. `workspace-result.json` contém os resumos e resultado; logs,
receitas, diffs e inventários estão em `evidence.json.gz`, verificado após
descompressão. `pre-validation.json.gz` e `after.json.gz` delimitam o estado
validado dos owners deste lote.

| Verificação | Resultado |
| --- | --- |
| Check workspace/all-targets | passou |
| Suíte completa workspace | 8.361 passaram; zero falhas; 7 ignorados |
| Registro de testes infra | 1.292 antes/depois; nenhum removido ou adicionado |
| Build, formatação e diff check | passaram |
| Rustdoc workspace com -D warnings | passou |
| Linhagem estrita V1/V5/V15/V26 | passou |
| Linhagem reversa | 965 consumers, zero erros |
| Linter arquitetural completo | zero erros, 380 avisos, 1611 notas |

O inventário de Clippy usa workspace/all-targets com --cap-lints warn
apenas para coletar todos os diagnósticos, inclusive dos crates posteriores
ao core. Seu exit zero não significa aprovação do gate real, que continua
com -D warnings e está pendente. A distribuição atual é:

| Diretório | Diagnósticos distintos |
| --- | --- |
| 01_core | 37 |
| 02_shell | 14 |
| 03_infra | 276 |
| 04_wiring | 13 |

Esse resultado não certifica a matriz remota Windows/macOS/i686/cross/
Docker nem fecha as pendências funcionais da auditoria. A quantidade de
testes aprovados não é certificação de paridade ou de conformidade PDF/UA.
A CI/release completa continua aberta; nenhuma release foi publicada.
