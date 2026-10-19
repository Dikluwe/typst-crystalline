# Inventário consolidado do projeto — 18/10/2026

Inventário documental e de arquivos concluído. Não foram apagados, movidos ou
arquivados arquivos, nem alterados código produtivo, L0s, APIs, CI ou pins.
A organização foi feita por um índice central, situação das frentes e proposta
de limpeza. As correções continuam pausadas.

- [estado e pendências](estado-e-pendencias.md): entregas, trabalho aberto, prioridades e dependências.
- [Limpeza e arquivamento propostos](limpeza-e-arquivamento.md): candidatos,
  tamanhos, riscos e condições para eventual execução.
- [Índice dos documentos](indice-documentos.csv) e
  [espaço por diretório](espaco-diretorios.csv): inventário detalhado.
- [Evidências e método](#evidências-e-método): proveniência e alcance das conclusões.

## Medição antes da classificação

HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, branch `Tekt`, working tree não commitado.
Coleta UTC `2026-10-18T14:46:10.374851+00:00`–`2026-10-18T14:46:31.645938+00:00`. Os recibos preservam também a hora UTC;
a data deste índice é 18/10/2026, America/Sao_Paulo.

| Item | Estado medido |
|---|---|
| Arquivos rastreados | 42781 |
| Arquivos rastreados modificados antes desta organização | 185: 93 em L0/núcleos, 86 em L1 e 6 em L3 |
| Arquivos não rastreados fora deste novo inventário | 836 |
| Novos arquivos produtivos/L0/núcleos não rastreados | 26: 7 Rust, 7 L0 e 12 núcleos; lista em `analysis.json.gz` |
| Rust nas quatro camadas, incluindo testes/build scripts | 991: L1 805, L2 9, L3 152, L4 25 |
| Namespace de prompts | 998 arquivos fora de `_nuclei`, mais 109 núcleos TOML |
| Consumers no último gate reverso | 990; cardinalidade não é deduzida da contagem de extensões |
| Relação com o último fechamento | 2105 arquivos protegidos idênticos ao freeze revisão 10 |

O namespace L0 contém templates e documentos auxiliares; contá-los como owners
materializáveis seria incorreto. `productive-state.json.gz` traz os hashes por
arquivo. `git-before.json.gz` guarda a lista exata de modificados/não rastreados
e `git diff HEAD --stat`; essas contagens não são um número de bugs.

## O que temos

| Área | Papel e entrada principal |
|---|---|
| L0 — `00_nucleo/prompts/` | Especificações proprietárias; núcleos em `_nuclei/`; não são candidatos à limpeza por antiguidade. |
| L1 — `01_core/` | Entidades, lexer/parser, avaliação, realização, introspecção, matemática e layout puro. |
| L2 — `02_shell/` | Interface CLI, argumentos e apresentação de diagnósticos. |
| L3 — `03_infra/` | Fontes, filesystem, exports PDF/SVG/PNG/HTML e pipeline. |
| L4 — `04_wiring/` | Binário `typst`, composição e testes de integração. |
| `benches/` | Quinto membro do workspace; benchmarks. Não pertence à implementação vanilla. |
| `tools/ci/`, `.github/workflows/` | Gates de CI, construção do oracle e empacotamento com proveniência. |
| `lab/typst-original/` | Fonte upstream em quarentena, fora do workspace cristalino; oracle separado. |
| `lab/` e ferramentas de geometria/paridade | Experimentos e instrumentos; retirar somente após inventariar dependências. |
| `00_nucleo/adr/` | Decisões vigentes e históricas; preservar para interpretar contratos. |
| `00_nucleo/diagnosticos/` | Evidências e relatórios; o índice distingue estado vigente de histórico. |
| `00_nucleo/materialization/` | Passos e propostas; não substituem os prompts L0 produtivos. |

## Estado das validações

A revisão 10 do encerramento R3-T1 registrou **9111 aprovados,
0 falhas, 7 ignorados**, incluindo cinco doctests.
Build, Clippy estrito, Rustdoc, fmt e linhagem passaram. O inventário não repetiu
a suíte; comprovou que o freeze de fontes protegido permaneceu idêntico.

O Tekt global continua bloqueado por **105 V16**; V19/V20 permanecem notas
informativas (1144/879 no fechamento). Há divergências de linguagem e lacunas de
produto. Suíte verde não equivale a paridade universal ou autorização de release.

Referências executáveis:

- Vanilla: pin `bd5ebec285505889e8ff1d4f68804ab147a699f3`,
  SHA-256 `9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.
- Linter do CI: commit `12797073972e375d9ab078cf6750c8b30b71e2b9`.
- Evidência mais recente: [fechamento R3-T1](../r3-t1-materializacao-2026-10-18/relatorio.md).
- Regras operacionais: [CI/release](../../../tools/ci/README.md).

CI/release estão implementados, mas V16 bloqueia o gate agregado. Não se fez
nova execução remota, build multi-OS, Docker ou release durante este inventário.
A receita de testes mantém doctests separados dos testes que invocam Rustdoc
JSON; isso não é uma correção geral comprovada de todos os E0463 do Cargo.

## Espaço no checkout

218521 entradas de arquivos/symlink medidas: **47.191 GiB**
em blocos alocados e 46.718 GiB lógicos.
A medição não segue o link AGENTS.md nem entra nas integrações `.agents`, `.aws`
e `.codex`. Metadados de diretórios não estão somados; hardlinks/extents
compartilhados não são deduplicados. Tamanho é estimativa por entrada, não espaço
recuperável garantido. Os relatórios produzidos após a coleta não integram o total.

| Diretório | GiB alocados | Arquivos | Conteúdo predominante |
|---|---:|---:|---|
| `target/` | 32.329 | 36424 | Builds e documentação gerada |
| `00_nucleo/` | 9.866 | 162347 | Prompts, decisões, processo e evidências |
| `lab/` | 3.172 | 10428 | Referências, experimentos e seus builds |
| `.git/` | 1.778 | 7281 | Histórico e metadados Git |
| `01_core/` | 0.013 | 808 | Domínio puro |
| `02_shell/` | 0.000 | 10 | CLI e formatação |
| `03_infra/` | 0.006 | 191 | I/O e exportação |
| `04_wiring/` | 0.000 | 26 | Composição |
| `tools/` | 0.025 | 931 | Ferramentas de validação e observação |

A tabela detalhada contém pais e filhos: não somar linhas sobrepostas.
`target/debug/incremental/` sozinho ocupa **13.033 GiB**;
é a primeira oportunidade de limpeza de cache após confirmação específica.

## Organização documental realizada

Este índice passa a ser o ponto de entrada da situação. Relatórios antigos
permanecem íntegros; seus números pertencem aos freezes originais.

- “Aguardando R3-T1” em relatórios de bibliografia é estado histórico: a aprovação
  foi recebida e a materialização avançou; Context ainda impede fechamento amplo.
- Pilha 64×2048 foi corrigida no lote state-stack; não repetir como falha atual
  só porque aparece no relatório A1 anterior.
- Construtor/seleção de Sequence foi tratado em R2; não reabrir como pendência R1.
- Recuo francês e métricas mistas foram implementados depois das primeiras
  análises A2; persistem resíduos explicitados no backlog.
- “Publicação bloqueada por Clippy” no relatório de 13/10 é histórico: o gate
  Clippy atual passou; os bloqueios atuais incluem V16 e paridade pendente.
- Não somar resultados de testes ou corpora de etapas diferentes como cobertura.

## Evidências e método

Varredura de metadados e leitura de documentos; nenhum build novo, benchmark ou
comparação de saída do produto. `files.tsv.gz` lista caminhos, tipo, Git,
tamanhos e mtime. Os hashes de docs estão em `documents.json.gz`; duplicatas
textuais exatas em `duplicate-documents.json.gz`, sem presumir que sejam lixo.

Coleta, análise, verificação e fechamento executados em cgroup MemoryMax=12 GiB, MemoryHigh=8 GiB,
swap zero e sem tarefas pesadas paralelas. Pico desses quatro jobs:
**0.509 GiB**, eventos max/OOM/OOM-kill zero. Recibos `coleta-resources.json`,
`analise-resources.json`, `verificacao-resources.json` e `fechamento-resources.json`; não confundir com consumo do compilador.

`analysis.json.gz` relaciona os 26 novos arquivos vitais e os recibos ignorados.
`documentos-consultados.json.gz` preserva os documentos usados nas decisões;
`verificacao.json.gz` e `manifest.json.gz` fecham a integridade desta entrega.
O README da raiz recebeu apenas o link para este inventário.
