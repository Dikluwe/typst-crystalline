# Relatório — identidade dos formatos e catálogo HTML/PDF

Etapa concluída: as quinze rotas restantes agora têm identidade de elemento reconhecida em `.where()` e `figure(kind: ...)`. Os cinco formatos usam um identificador interno fechado; HTML elem/frame e PDF attach/artifact usam seus callbacks reais. Não há nova API Rust externa. O objetivo global permanece ativo.

## Proveniência

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. Baseline `2026-10-17T15:24:16.761245+00:00`, fechamento `2026-10-17T16:18:54.111291+00:00`. before.json.gz/final-state.json.gz conservam status, diff HEAD --stat, fontes afetadas e hashes de 2123 arquivos protegidos. A matriz anterior foi reutilizada apenas após igualdade exata de hashes de fontes/configuração/binário com a etapa anterior, em baseline-reuse.json. A posterior foi executada sobre a versão final.

Vanilla pinado `9f2b6e8715237cb086899a42873660fe744622e8`, binário SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`. before/after-measurement.json.gz e os lotes detalhados registram UTC, argv, binários e saídas. pinned-schema.json.gz conserva nove declarações do pin, blobs/hash/linhas e exclusões internal/external.

## Implementação e contratos

Onze pares consumer/owner atualizados; um Núcleo Tekt `selectors/format-native-identity.toml`, consumido/pinado pelos onze L0. A especificação antecedeu testes/código. `FormatElementKind` e `FuncRepr::Format(kind, NativeFunc)` são restritos ao crate. Func, NativeFunc e NativeFuncWithEngine conservam campos e assinaturas públicas; public-contract-before/after.json compara as interfaces dos onze consumers e os structs públicos de Func.

A testemunha distingue svg/png/pdf/html/bundle mesmo compartilhando callback. Nome, namespace, Eq Rust ou ponteiro comum não autenticam formato. Clones preservam identidade; uma Native comum reconstruída pelo mesmo ponteiro continua sem a capacidade. O constructor privado recebe o NativeFunc original; não há wrapper de chamada, novo binding, novo default, nova fase ou duplicação de factory. As chamadas continuam pela ABI estática existente, com o mesmo callback.

Nome, namespace, native_fn_addr, span, Debug, clone/With e Eq/Hash públicos preservam os observáveis anteriores. Eq/Hash continuam compatíveis com Native comum de mesmo nome; essa igualdade não é usada como prova de elemento. A relação observacional inclui o novo identificador e distingue Format de Native comum, sem executar. A factory global pdf/html e as projeções mantêm namespaces e membros clonados. Os quatro constructors de conteúdo permanecem Native; somente html.frame amplia visibilidade interna para reexport de metadados.

integrity-result.json prova os nove corpos de callbacks intactos e recupera as três factories anteriores removendo apenas a nova construção do carrier. Não houve commit, push, merge ou release, nem supressão de lint. Os manifestos TS1 e RG1 estão preservados.

| Identidade canônica | Campos selecionáveis | Locatable |
|---|---|---|
| `bundle` | nenhum | false |
| `html` | `pretty` | false |
| `elem` | `tag`, `attrs`, `body` | false |
| `frame` | `body` | false |
| `pdf` | `pages`, `standard`, `tagged`, `pretty` | false |
| `artifact` | `kind`, `body` | false |
| `attach` | `path`, `data`, `relationship`, `mime-type`, `description` | true |
| `png` | `ppi` | false |
| `svg` | `pretty` | false |

HtmlElem exclui css/parent/role internos; label continua filtro comum. With não é elemento bare. As rotas globais/projetadas conservam o comportamento anterior de gates e aliases. Há uma lacuna preexistente em std.html e descendentes sem a feature html, registrada em html-off-availability-audit.json; não se certifica paridade desse gate.

## Verificação

- RED funcional: dezesseis falhas (quinze admissões e distinção observacional) e dois controles passaram. red-input.json.gz fixa fontes, owners, hashes, status/diffstat no momento anterior à execução. O gerador inicialmente encontrou um import stdlib também no módulo de teste; a busca foi restrita à função produtiva, sem resetar trabalho. Tentativa registrada em implementation-first-attempt.*.
- GREEN focado: 21 testes passaram, incluindo três guardas diretas V19 adicionadas após o RED inicial. A matriz anterior foi ampliada para as sete representações e executada também isoladamente. Suíte completa: 8900 passaram, zero falhas, 7 ignorados; fontes congeladas.
- Matriz completa: 3956 casos, oito perfis. 208 observáveis mudaram somente nas quinze rotas autorizadas; 184 valores de linguagem foram corrigidos para coincidir com o pin. Os outros 20096 observáveis permaneceram idênticos, inclusive todas as saídas do oráculo. Nenhuma omissão resta nesta matriz de admissão onde a função está disponível nos dois lados. As 24 mudanças em perfis de disponibilidade só Tekt são separadas em availability-residuals.json e não recebem crédito de paridade. A matriz parte dos bindings Tekt e não certifica que todo binding upstream exista.
- Lote detalhado: 144 casos; 79 valores de linguagem coincidem com o pin; 21 erros têm mensagem correta e âncora residual; 15 controles de counter, 15 parciais e 14 rejeições de construção foram preservados. Counter/bare selector/query/show continuam frentes separadas.
- Compatibilidade: 456 casos bilaterais de nomes, repr, aliases/igualdade, partial-name e namespace em oito perfis, todos preservados exatamente nos dois binários. O aviso antigo do Tekt para a fixture vazia continua explicitamente tratado pelo verificador; não se declara stderr igual ao pin nem se descartam avisos arbitrários.
- Tamanhos medidos no RED e GREEN: `{'func': 16, 'repr': 120, 'native': 32, 'engine': 32}` bytes, sem alteração em Func, FuncRepr, NativeFunc e NativeFuncWithEngine. O layout Rust é controle de custo/API, não critério de paridade da língua.
- Build, fmt, Clippy workspace/all-targets estrito, Rustdoc -D warnings com quatro índices, V1/V5/V15/V26, reverse e diff-check passaram. Ownership/V26 precederam o resselo restrito aos onze pares.
- Erros/avisos anteriores foram preservados por mapeamento de linhas. Quatro notas V19 existentes foram estendidas somente pela alternativa Format, com a contagem correspondente; nenhuma alternativa anterior foi removida. Existem 8 novas notas V19 nas uniões explícitas Native/Format: mesmo payload/ABI para a operação, com Native e Format exercitados nos testes de projeção/observação, nos 144 casos e nos 456 controles. Os padrões exatos, fontes/hashes e vínculos estão em architecture-delta.json. Nenhum novo erro/aviso; o gate global ainda é bloqueado pelos 117 V16 anteriores.
- candidate-1 conserva a primeira versão validada até o comparador. Antes da versão final, os L0 de três owners prescreveram guardas diretas: sete representações em identidade/hash/projeções; cinco kinds com mesmo nome/callback que panica se chamado; matriz de observação que varia kind, callback, nome e namespace separadamente. Nenhum comportamento produtivo foi alterado nessa ampliação de testes, e todos os gates e sondas foram repetidos.
- A primeira tentativa de iniciar os gates não criou processo porque a revisão automática expirou. Uma repetição reduzida foi aprovada e executou os gates; execution-review-timeout.json registra a ocorrência. Não houve conclusão de que o comando fosse inseguro.

## RAM e fronteiras

Comandos pesados sequenciais em MemoryMax12GiB, MemoryHigh10GiB, swap0 e Cargo jobs1. Pico contabilizado pelo cgroup: 10741780480 bytes (10.004 GiB); eventos `{'low': 0, 'high': 140, 'max': 0, 'oom': 0, 'oom_kill': 0, 'oom_group_kill': 0}`. Sem atingir MemoryMax/OOM/oom_kill. Inclui cache, não somente RSS.

[Pendências](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/native-format-catalogue-2026-10-17/follow-ups.md). A admissão de metadados não certifica execução de seletores, síntese, getters completos, layout, morfologia, índice ou contadores. FG1/RM1/SY1/RG1 e os V16 permanecem abertos.

[Retificação sobre FuncRepr](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/native-format-catalogue-2026-10-17/retificacao-func-repr.md): a nota anterior que o chamava público no L0 era incorreta; a declaração já era pub(crate). Os arquivos anteriores ficam preservados como histórico. Evidências e hashes finais estão em final-state.json.gz e SHA256SUMS.
