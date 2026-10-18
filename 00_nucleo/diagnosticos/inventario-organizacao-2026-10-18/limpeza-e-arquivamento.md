# Candidatos a limpeza e arquivamento — nada executado

Medição antes da classificação: HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, working tree e UTC
em [README.md](README.md), `git-before.json.gz`, `filesystem.json.gz` e
`analysis.json.gz`. As tabelas são propostas para revisão, não uma lista de
exclusão autorizada. Nenhum rm, git clean, cargo clean, movimento ou remoção
foi executado. Os tamanhos são blocos por entrada, sem deduplicação de extents;
as categorias se sobrepõem e não devem ser somadas como espaço recuperável.

## Ordem sugerida

1. Proteger os arquivos novos e os recibos ignorados; escolher a retenção dos
   executáveis de referência/antes/depois.
2. Confirmar e limpar somente C01, o cache incremental de 13,033 GiB, como
   primeiro lote. Verificar novamente os caminhos e processos antes de executar.
3. Tratar C02/C04/C07; C03 requer mais reconstrução. Fazer os testes apropriados
   quando houver rebuild, mantendo o limite de 12 GiB.
4. Preparar arquivos autocontidos dos grupos A, validar e só então decidir quais
   cópias locais remover. C05/C06 dependem dessa preservação.
5. Consolidar documentos e nomes ambíguos. Movimentos alteram caminhos usados
   por scripts/manifests; preferir um índice e pacotes recuperáveis a reescrever
   silenciosamente evidências históricas seladas.

Não há autorização implícita para apagar todos os ignorados ou todo target.

## Lista revisável

| ID | Caminho/grupo | Tamanho medido | Tratamento proposto | Condição / impacto |
|---|---|---:|---|---|
| C01 | `target/debug/incremental/` | 13.033 GiB | Limpeza de cache | Reconstruível. Confirmar ausência de Cargo/Rustc ativos no momento de executar; perde aceleração de builds antigos. Nenhum arquivo rastreado nesse grupo. |
| C02 | `target/doc/` e `target/doc-prefix-nesting-quarantine-20261014/` | 0.110 GiB + 0.081 GiB | Limpeza após preservar prova Rustdoc | HTML gerado é reconstruível; manter logs/receitas e JSON de superfície pública que ainda for evidência. A quarentena documenta tentativas anteriores. |
| C03 | `target/debug/deps/`, `build/`, `.fingerprint/` | 16.778 GiB + 0.554 GiB + 0.024 GiB | Limpeza de build em lote posterior | Cargo recompila. Preservar o CLI validado e registrar a receita antes; tratar artefatos de build coerentemente. Tamanho não inclui todo target/debug. |
| C04 | `lab/surface-inventory/target/` | 0.920 GiB | Limpeza de cache de ferramenta | Preservar fonte, Cargo.lock, receitas e relatórios gerados fora de target. Não apagar lab/surface-inventory inteiro. |
| C05 | `target/release/` | 1.456 GiB | Preservação de executáveis antes da limpeza | Contém CLI e probes históricos. Não assumir que release corresponde ao código atual; copiar binário/hash/receita necessários antes de limpar caches. |
| C06 | `lab/typst-original/target/` | 2.181 GiB | Retenção especial do oracle | Preservar exatamente target/release/typst e proveniência pin/árvore/receita/SHA. Pode-se estudar limpeza seletiva das dependências; não apagar o oracle ratificado. |
| C07 | `**/__pycache__/` | 0.002 GiB | Limpeza de cache de baixo retorno | 201 arquivos, aproximadamente 2,32 MiB, todos ignorados na coleta. Não é uma categoria de evidência. |
| A01 | `00_nucleo/diagnosticos/**/bin/` | 2.433 GiB | Arquivar por experimento, com hash | 49 arquivos em diretórios bin, incluindo 27 rastreados (25 ELF). Nem todos são caches descartáveis. Preservar variantes before/after e perfis; binarios-rastreados.json.gz lista os rastreados. |
| A02 | `00_nucleo/diagnosticos/**/outputs/` | 0.760 GiB | Arquivar com fontes e dependências | 10546 arquivos, ignorados pelo Git. PDFs/SVGs/PNGs, fontes .typ e dados .bib podem ser a única cópia bruta. Manter pacote autocontido e verificar receitas antes de remover. |
| A03 | Cópias Rustdoc dentro de diagnósticos | 0.062 GiB | Consolidar depois de preservar evidência | 1420 arquivos pelo caminho rustdoc/static.files, incluindo 24 rastreados. Não apagar licenças isoladamente por serem duplicatas. Categoria sobrepõe outros diretórios. |
| A04 | Experimentos históricos grandes listados abaixo | Ver tabela | Arquivamento por diretório | Manter relatório, fontes, inputs, freezer, binários selecionados, resultados e hashes. Indexar o pacote e a forma de recuperar os caminhos antigos. Antiguidade/pin antigo não provam inutilidade. |
| D01 | Dois pares de documentos históricos duplicados | 11,5 KiB de conteúdo redundante | Consolidação documental futura | Escolher canônico após conferir referências; manter ponte no índice. Nenhum dos pares é um owner L0; não escolher apenas pelo nome. |
| D02 | `./--help` | 9818 bytes lógicos | Realocar/classificar após revisão | Arquivo JSON de sonda, rastreado, com nome ambíguo na raiz. Preservar conteúdo/hash e descobrir referências antes de mover; não executar como opção de comando. |
| D03 | Propostas substituídas e passos concluídos em materialization | Baixo retorno de disco | Arquivo histórico indexado | A1-S1 foi retirada em favor de A1-S2. Pacotes aprovados preservam rastreabilidade e não devem ser confundidos com L0 vigente. RG1 segue pendente, não obsoleto por estar antigo. |
| D04 | `AGENTS.md` como symlink absoluto | 64 bytes de destino | Revisão de portabilidade, sem exclusão | O link aponta para /home/dikluwe/Documentos/Antigravity/typst-crystalline/CLAUDE.md. Revisar uma referência portátil em etapa própria; não apagar as instruções do repositório. |

C01–C06 são recortes de caches/builds já incluídos nos 47,191 GiB do inventário;
A01–A04 também se sobrepõem. Não somar os 32,329 GiB de target a seus filhos.

## Diretórios grandes candidatos a arquivo histórico

| Diretório | Tamanho alocado | Critério |
|---|---:|---|
| `00_nucleo/diagnosticos/layout-risk-reproduction/` | 0.752 GiB | Inventariar e arquivar como evidência histórica; confirmar dependências ativas. |
| `00_nucleo/diagnosticos/font-lifecycle-experiment/` | 0.477 GiB | Inventariar e arquivar como evidência histórica; confirmar dependências ativas. |
| `00_nucleo/diagnosticos/infra-performance-followup/` | 0.354 GiB | Inventariar e arquivar como evidência histórica; confirmar dependências ativas. |
| `00_nucleo/diagnosticos/performance-baseline/` | 0.353 GiB | Inventariar e arquivar como evidência histórica; confirmar dependências ativas. |
| `00_nucleo/diagnosticos/place-font-acquisition-study/` | 0.315 GiB | Inventariar e arquivar como evidência histórica; confirmar dependências ativas. |
| `00_nucleo/diagnosticos/transformacoes-2026-10-14/` | 0.386 GiB | Inventariar e arquivar como evidência histórica; confirmar dependências ativas. |

O grupo transformacoes inclui investigação de Rustdoc e etapas aprovadas;
conservar os resultados RED/GREEN e o diagnóstico do problema, mesmo quando o
HTML gerado puder ser reconstruído. Experimentos de fontes e desempenho
registram pins anteriores; seu valor é histórico, não prova do estado atual.

Os lotes recentes A1, Sequence, bibliografia, mixed-style e R3-T1 são a base
ativa do backlog atual. Sua inclusão futura num arquivo exige manter acesso
imediato às fontes e aos recibos, não simples remoção por data.

## Duplicatas documentais exatas

A comparação de conteúdo encontrou cinco grupos no conjunto de Markdown
varrido: dois pares de documentos de processo e três grupos de licenças geradas
pelo Rustdoc. Isso não é uma análise de duplicatas de todos os binários/PDFs.
SHA-256 e caminhos completos estão em `duplicate-documents.json.gz`.

| Pares de processo | SHA-256 idêntico |
|---|---|
| `00_nucleo/materialization/relatorio-p448.md`<br>`00_nucleo/materialization/typst-passo-448-relatorio.md` | `e49089974a09cc49da94e9f0cd5c25fb56cb6006fe9c2544c8e0e7ddfde0a40f` |
| `00_nucleo/materialization/typst-passo-1121.md`<br>`00_nucleo/diagnosticos/typst-passo-1121-relatorio.md` | `9350386f6ac4d6e3ac5b31263abfcb375d2d24316655f64340a0144f5554161a` |
| `00_nucleo/diagnosticos/embedded-fonts-memoria2-2026-10-08/protocol.md`<br>`00_nucleo/diagnosticos/embedded-fonts-memoria2-2026-10-08/attempt-font-container/protocol.md` | `57660e3ab12b6a3cb3143d01670880fca2ed37a35c5dde0b6c12add064ca14d9` |
| `00_nucleo/diagnosticos/catalogo-seletivo-2026-10-07/performance-final/protocol.md`<br>`00_nucleo/diagnosticos/catalogo-seletivo-2026-10-07/performance/protocol.md` | `9f958c6ff259ee05c6193c12a396f873a89a21ec1baa1d7046cccdaa1249d7df` |

São candidatos à consolidação de navegação, com benefício de disco muito pequeno.
Os outros grupos são licenças em saídas geradas; tratar a árvore de geração,
preservando a atribuição das fontes, em vez de excluir licenças individualmente.
Não foi feita prova de ausência de referências para cada uma dessas cópias.

## Preservar antes de qualquer limpeza

- Os **26 arquivos produtivos/L0/núcleos não rastreados**. Eles fazem parte da
  implementação validada; não são rascunhos dispensáveis. Lista exata em
  `analysis.json.gz`, com hashes em `productive-state.json.gz`.
- Código, L0s e núcleos vigentes, ADRs, Cargo.lock/toolchain, catálogo de fontes,
  dados de testes, fixtures e arquivos necessários ao build.
- Aprovações, propostas ratificadas e refutações que explicam propostas retiradas.
- Fontes integrais dos casos, imports .bib/.csl/.typ, observer/receita, pin/árvore,
  perfil, argv, UTC, commit/status/diffstat e hash de cada executável medido.
- Resultados brutos RED, GREEN e refutadores abertos. Uma captura malsucedida não
  equivale ao RED funcional; preservar essa distinção no arquivo.
- Oracle vanilla ratificado e CLI atual validado. Se um cache contiver a única
  cópia de uma rlib/probe necessária para reproduzir a medição, arquivá-la antes.

**Risco de Git ignorar evidência:** no diagnóstico R3-T1 há 209 arquivos JSON/log
na raiz do lote que o Git ignora, incluindo suite-summary, audit, manifest e
recibos de recursos. Os arquivos .json.gz são admitidos, mas continuam não
rastreados até serem adicionados. Um commit simples não preserva tudo. Não
alterei o .gitignore nem forcei adição; a evidência consultada pelo inventário
foi copiada em formato compacto para `documentos-consultados.json.gz`.

O pacote compacto deste inventário não é backup de todos os 9,824 GiB de
diagnósticos. Antes de retirar dados, definir onde o arquivo completo ficará,
conferir sua integridade e testar recuperação do corpus afetado.

## Fora da limpeza proposta

- `.git/`: histórico e recuperação. Apagar arquivos do checkout não remove os
  blobs antigos do histórico; não há proposta de reescrever histórico ou GC.
- `lab/typst-original/` como um todo: é a fonte de referência, não lixo.
- Diretórios de código, owners L0 ou núcleos sem consumer presumido por busca
  textual: ownership deve ser verificado pelo grafo, não pelo tamanho/data.
- O link/instruções AGENTS e as integrações `.agents`, `.aws`, `.codex`; as
  integrações não foram varridas. Nada fora deste repositório foi inventariado
  para exclusão, inclusive os linter builds em /tmp.
