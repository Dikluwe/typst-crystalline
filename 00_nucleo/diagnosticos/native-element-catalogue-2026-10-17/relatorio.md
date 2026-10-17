# Relatório — catálogo de elementos nativos, lote 1

Etapa concluída: quinze identidades nativas existentes agora têm catálogo de campos e capacidade de locatabilidade reconhecidos por callback tipado. A correção alcança `.where()` e a admissão de `figure(kind: ...)`, incluindo aliases. TS1/equality e os demais consumers foram preservados. O objetivo global continua ativo.

## Proveniência

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. Coleta anterior: `2026-10-17T14:03:29.616579+00:00`; fechamento: `2026-10-17T14:36:35.102750+00:00`. `before.json.gz` e `final-state.json.gz` registram estado completo, diff HEAD --stat e hashes de 2121 arquivos protegidos. Apenas o consumer `element_where.rs` e seu Prompt L0 proprietário mudaram neste lote. `scoped.patch` preserva a diferença sobre a árvore anterior, sem confundir alterações preexistentes do projeto com este trabalho.

Oráculo vanilla pinado: `9f2b6e8715237cb086899a42873660fe744622e8`, executável SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`. Binários Tekt antes/depois, UTC, comandos completos e todas as saídas estão em `before-measurement.json.gz`/`after-measurement.json.gz`; fontes primárias e blocos literais com hashes/linhas estão em `batch-schema.json`.

## Medição e correção

A auditoria percorreu o registry efetivo em oito perfis de features e mediu 3956 casos de funções. Encontrou 57 caminhos com omissão de identidade em where e kind, correspondendo a 420 recusas por operação ao somar os perfis. São caminhos/aliases, não 57 implementações distintas. A enumeração não certifica todos os fields associados a Type, nem funções de usuário ou suporte de layout.

O lote corrige quinze callbacks, reduzindo as rotas omitidas de 57 para 41: dezesseis rotas do pin mais o alias global Tekt `op` acompanham suas funções efetivas. O reconhecimento usa fn_addr_eq com ABI própria; math.op permanece NativeWithEngine, com seis argumentos. Nome de Func, PartialEq e execução do constructor não são usados como prova de identidade. Page e funções homônimas permaneceram como controles. Não houve mudança de API pública Rust, fase, default de produto, commit, push ou release.

| Binding principal | Campos selecionáveis | Locatable |
|---|---|---|
| `std.title` | `body` | true |
| `std.document` | `path`, `format`, `title`, `author`, `description`, `keywords`, `date`, `body` | true |
| `std.divider` | nenhum | false |
| `std.list.item` | `body` | false |
| `std.enum.item` | `number`, `body` | false |
| `std.terms.item` | `term`, `description` | false |
| `std.footnote.entry` | `note`, `separator`, `clearance`, `gap`, `indent` | true |
| `std.place.flush` | nenhum | false |
| `std.math.op` | `text`, `limits` | false |
| `std.curve.move` | `start`, `relative` | false |
| `std.curve.line` | `end`, `relative` | false |
| `std.curve.quad` | `control`, `end`, `relative` | false |
| `std.curve.cubic` | `control-start`, `control-end`, `end`, `relative` | false |
| `std.curve.close` | `mode` | false |
| `std.asset` | `path`, `data` | true |

O L0 foi atualizado antes dos testes/código, por fluxo contínuo ADR-0127 de tabela/paridade. Não foram criados novos bindings. Campos desconhecidos seguem rejeitados com nome público; label continua permitido. Closures/With e callbacks homônimos não ganham identidade.

## Verificação

- RED funcional: quinze testes falharam por metadados ausentes e um controle passou. Uma primeira tentativa teve erro de construção de Label na fixture; foi corrigida e arquivada como `red-harness-*`, sem ser contada como RED funcional.
- GREEN focado: dezesseis testes passaram. Suíte workspace completa: 8852 passaram, 0 falharam, 7 ignorados, com fontes congeladas.
- Matriz antes/depois: 256 observáveis Tekt mudaram exclusivamente no conjunto autorizado; 240 reparos têm valores de linguagem iguais ao pin. Os demais 20048 observáveis, incluindo todas as saídas do oráculo, permaneceram idênticos.
- Lote detalhado: 110 casos bilaterais. 80 resultados de linguagem coincidem com o pin; quinze controles de counter permanecem divergentes e quinze erros de campo desconhecido têm mensagem correta com âncora ainda divergente. Não são declarados fechados.
- O arquivo de entrada da sonda é vazio. O Tekt já emitia o aviso exato `ficheiro vazio: sem conteúdo` em 4720 controles bem-sucedidos anteriores. Os novos sucessos mantêm esse aviso, ausente no pin. O verificador exige esses bytes exatos e registra a diferença; não declara igualdade integral de stderr nem descarta avisos arbitrários.
- Build, fmt, Clippy workspace/all-targets com warnings/unfulfilled_lint_expectations negados, Rustdoc com warnings negados e quatro índices HTML, V1/V5/V15/V26 e linhagem reversa passaram. V15/V26 foram validados antes do resselo restrito ao par proprietário.
- O gate global continua bloqueado pelos 117 V16 anteriores; V19=1076 notes, V20=873 notes. Todos os diagnósticos foram preservados após mapear deslocamentos de linhas; não houve novo erro/aviso. Esta etapa de paridade não é uma redução de V16 nem uma aprovação do gate global.

## RAM

Todos os comandos pesados foram sequenciais em cgroup com MemoryMax=12 GiB, MemoryHigh=10 GiB, MemorySwapMax=0 e Cargo jobs1. Maior memory.peak: 10326949888 bytes (9.618 GiB). Eventos agregados: `{'low': 0, 'high': 0, 'max': 0, 'oom': 0, 'oom_kill': 0, 'oom_group_kill': 0}`. Sem atingir MemoryMax/OOM/oom_kill. O pico inclui memória de processos e cache contabilizados pelo cgroup; não é RSS isolado.

## O que continua aberto

Há 41 rotas restantes na matriz: 26 de matemática e 15 de formatos/HTML/PDF, listadas em [follow-ups.md](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/native-element-catalogue-2026-10-17/follow-ups.md). Formatos raiz compartilham callback e precisam de identidade própria antes de corrigir metadados. Counter e bare selector ainda têm admissão separada; capacidades declaradas aqui não comprovam índice, síntese ou render. As entradas históricas da tabela também não receberam certificação universal neste lote.

FG1, RM1, SY1 e a proposta RG1 conservam suas fronteiras. Os manifestos originais TS1 aprovado e RG1 pendente permanecem íntegros. Nenhuma nova aprovação foi inferida. Evidências, escopo e testes estão preservados no diretório deste relatório e em `SHA256SUMS`; o estado exato de fechamento está em `final-state.json.gz`.
