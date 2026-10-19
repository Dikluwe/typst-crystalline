from pathlib import Path
import gzip,json,datetime,collections,csv
p=Path(__file__).resolve().parent;root=p.parents[2]
def read(name):return json.loads(gzip.decompress((p/name).read_bytes()))
def gib(n):return f'{n/1024**3:.3f}'
fs=read('filesystem.json.gz');a=read('analysis.json.gz');state=read('git-before.json.gz');comparison=read('closure-comparison.json.gz')
r3=root/'00_nucleo/diagnosticos/r3-t1-materializacao-2026-10-18'
suite=json.loads((r3/'suite-summary-candidate10.json').read_text())
resources=[json.loads((p/f).read_text()) for f in ['coleta-resources.json','analise-resources.json','verificacao-resources.json','fechamento-resources.json']]
peak=max(int(r['memory_after']['memory.peak']) for r in resources)
assert all(r['exit']==0 for r in resources) and not comparison['changed']
space_rows='\n'.join(f'| `{key}/` | {gib(fs["directories"][key]["allocated_bytes"])} | {fs["directories"][key]["files"]} | {role} |' for key,role in [('target','Builds e documentação gerada'),('00_nucleo','Prompts, decisões, processo e evidências'),('lab','Referências, experimentos e seus builds'),('.git','Histórico e metadados Git'),('01_core','Domínio puro'),('02_shell','CLI e formatação'),('03_infra','I/O e exportação'),('04_wiring','Composição'),('tools','Ferramentas de validação e observação')])
link='[estado e pendências](estado-e-pendencias.md)'
(p/'README.md').write_text(f'''# Inventário consolidado do projeto — 18/10/2026

Inventário documental e de arquivos concluído. Não foram apagados, movidos ou
arquivados arquivos, nem alterados código produtivo, L0s, APIs, CI ou pins.
A organização foi feita por um índice central, situação das frentes e proposta
de limpeza. As correções continuam pausadas.

- {link}: entregas, trabalho aberto, prioridades e dependências.
- [Limpeza e arquivamento propostos](limpeza-e-arquivamento.md): candidatos,
  tamanhos, riscos e condições para eventual execução.
- [Índice dos documentos](indice-documentos.csv) e
  [espaço por diretório](espaco-diretorios.csv): inventário detalhado.
- [Evidências e método](#evidências-e-método): proveniência e alcance das conclusões.

## Medição antes da classificação

HEAD `{state['head']}`, branch `{state['branch']}`, working tree não commitado.
Coleta UTC `{fs['start']}`–`{fs['end']}`. Os recibos preservam também a hora UTC;
a data deste índice é 18/10/2026, America/Sao_Paulo.

| Item | Estado medido |
|---|---|
| Arquivos rastreados | {len(state['tracked'])} |
| Arquivos rastreados modificados antes desta organização | {len(state['modified'])}: 93 em L0/núcleos, 86 em L1 e 6 em L3 |
| Arquivos não rastreados fora deste novo inventário | {sum(a['untracked_before_inventory_by_root'].values())} |
| Novos arquivos produtivos/L0/núcleos não rastreados | 26: 7 Rust, 7 L0 e 12 núcleos; lista em `analysis.json.gz` |
| Rust nas quatro camadas, incluindo testes/build scripts | 991: L1 805, L2 9, L3 152, L4 25 |
| Namespace de prompts | 998 arquivos fora de `_nuclei`, mais 109 núcleos TOML |
| Consumers no último gate reverso | 990; cardinalidade não é deduzida da contagem de extensões |
| Relação com o último fechamento | {comparison['protected_files']} arquivos protegidos idênticos ao freeze revisão 10 |

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

A revisão 10 do encerramento R3-T1 registrou **{suite['passed']} aprovados,
{suite['failed']} falhas, {suite['ignored']} ignorados**, incluindo cinco doctests.
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

{fs['files']} entradas de arquivos/symlink medidas: **{gib(fs['directories']['.']['allocated_bytes'])} GiB**
em blocos alocados e {gib(fs['directories']['.']['logical_bytes'])} GiB lógicos.
A medição não segue o link AGENTS.md nem entra nas integrações `.agents`, `.aws`
e `.codex`. Metadados de diretórios não estão somados; hardlinks/extents
compartilhados não são deduplicados. Tamanho é estimativa por entrada, não espaço
recuperável garantido. Os relatórios produzidos após a coleta não integram o total.

| Diretório | GiB alocados | Arquivos | Conteúdo predominante |
|---|---:|---:|---|
{space_rows}

A tabela detalhada contém pais e filhos: não somar linhas sobrepostas.
`target/debug/incremental/` sozinho ocupa **{gib(a['categories']['target/debug/incremental']['allocated_bytes'])} GiB**;
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
**{gib(peak)} GiB**, eventos max/OOM/OOM-kill zero. Recibos `coleta-resources.json`,
`analise-resources.json`, `verificacao-resources.json` e `fechamento-resources.json`; não confundir com consumo do compilador.

`analysis.json.gz` relaciona os 26 novos arquivos vitais e os recibos ignorados.
`documentos-consultados.json.gz` preserva os documentos usados nas decisões;
`verificacao.json.gz` e `manifest.json.gz` fecham a integridade desta entrega.
O README da raiz recebeu apenas o link para este inventário.
''')
def size(key):
 if key in fs['directories']:return gib(fs['directories'][key]['allocated_bytes'])+' GiB'
 return gib(a['categories'][key]['allocated_bytes'])+' GiB'
rows=[
('C01','`target/debug/incremental/`',size('target/debug/incremental'),'Limpeza de cache','Reconstruível. Confirmar ausência de Cargo/Rustc ativos no momento de executar; perde aceleração de builds antigos. Nenhum arquivo rastreado nesse grupo.'),
('C02','`target/doc/` e `target/doc-prefix-nesting-quarantine-20261014/`',size('target/doc')+' + '+size('target/doc-prefix-nesting-quarantine-20261014'),'Limpeza após preservar prova Rustdoc','HTML gerado é reconstruível; manter logs/receitas e JSON de superfície pública que ainda for evidência. A quarentena documenta tentativas anteriores.'),
('C03','`target/debug/deps/`, `build/`, `.fingerprint/`',size('target/debug/deps')+' + '+size('target/debug/build')+' + '+size('target/debug/.fingerprint'),'Limpeza de build em lote posterior','Cargo recompila. Preservar o CLI validado e registrar a receita antes; tratar artefatos de build coerentemente. Tamanho não inclui todo target/debug.'),
('C04','`lab/surface-inventory/target/`',size('lab/surface-inventory/target'),'Limpeza de cache de ferramenta','Preservar fonte, Cargo.lock, receitas e relatórios gerados fora de target. Não apagar lab/surface-inventory inteiro.'),
('C05','`target/release/`',size('target/release'),'Preservação de executáveis antes da limpeza','Contém CLI e probes históricos. Não assumir que release corresponde ao código atual; copiar binário/hash/receita necessários antes de limpar caches.'),
('C06','`lab/typst-original/target/`',size('lab/typst-original/target'),'Retenção especial do oracle','Preservar exatamente target/release/typst e proveniência pin/árvore/receita/SHA. Pode-se estudar limpeza seletiva das dependências; não apagar o oracle ratificado.'),
('C07','`**/__pycache__/`',size('cache-python'),'Limpeza de cache de baixo retorno','201 arquivos, aproximadamente 2,32 MiB, todos ignorados na coleta. Não é uma categoria de evidência.'),
('A01','`00_nucleo/diagnosticos/**/bin/`',size('diagnostics-bin'),'Arquivar por experimento, com hash','49 arquivos em diretórios bin, incluindo 27 rastreados (25 ELF). Nem todos são caches descartáveis. Preservar variantes before/after e perfis; binarios-rastreados.json.gz lista os rastreados.'),
('A02','`00_nucleo/diagnosticos/**/outputs/`',size('diagnostics-outputs'),'Arquivar com fontes e dependências','10546 arquivos, ignorados pelo Git. PDFs/SVGs/PNGs, fontes .typ e dados .bib podem ser a única cópia bruta. Manter pacote autocontido e verificar receitas antes de remover.'),
('A03','Cópias Rustdoc dentro de diagnósticos',size('diagnostics-rustdoc'),'Consolidar depois de preservar evidência','1420 arquivos pelo caminho rustdoc/static.files, incluindo 24 rastreados. Não apagar licenças isoladamente por serem duplicatas. Categoria sobrepõe outros diretórios.'),
('A04','Experimentos históricos grandes listados abaixo','Ver tabela','Arquivamento por diretório','Manter relatório, fontes, inputs, freezer, binários selecionados, resultados e hashes. Indexar o pacote e a forma de recuperar os caminhos antigos. Antiguidade/pin antigo não provam inutilidade.'),
('D01','Dois pares de documentos históricos duplicados','11,5 KiB de conteúdo redundante','Consolidação documental futura','Escolher canônico após conferir referências; manter ponte no índice. Nenhum dos pares é um owner L0; não escolher apenas pelo nome.'),
('D02','`./--help`','9818 bytes lógicos','Realocar/classificar após revisão','Arquivo JSON de sonda, rastreado, com nome ambíguo na raiz. Preservar conteúdo/hash e descobrir referências antes de mover; não executar como opção de comando.'),
('D03','Propostas substituídas e passos concluídos em materialization','Baixo retorno de disco','Arquivo histórico indexado','A1-S1 foi retirada em favor de A1-S2. Pacotes aprovados preservam rastreabilidade e não devem ser confundidos com L0 vigente. RG1 segue pendente, não obsoleto por estar antigo.'),
('D04','`AGENTS.md` como symlink absoluto','64 bytes de destino','Revisão de portabilidade, sem exclusão','O link aponta para /home/dikluwe/Documentos/Antigravity/typst-crystalline/CLAUDE.md. Revisar uma referência portátil em etapa própria; não apagar as instruções do repositório.'),
]
clean_rows='\n'.join('| '+' | '.join(row)+' |' for row in rows)
historical=['layout-risk-reproduction','font-lifecycle-experiment','infra-performance-followup','performance-baseline','place-font-acquisition-study','transformacoes-2026-10-14']
history_rows='\n'.join(f'| `00_nucleo/diagnosticos/{folder}/` | {size("00_nucleo/diagnosticos/"+folder)} | Inventariar e arquivar como evidência histórica; confirmar dependências ativas. |' for folder in historical)
dup=read('duplicate-documents.json.gz')
realdups=[d for d in dup if not any('/rustdoc/' in f or '/static.files/' in f for f in d['paths'])]
dup_rows='\n'.join('| '+'<br>'.join('`'+f+'`' for f in d['paths'])+' | `'+d['sha256']+'` |' for d in realdups)
(p/'limpeza-e-arquivamento.md').write_text(f'''# Candidatos a limpeza e arquivamento — nada executado

Medição antes da classificação: HEAD `{state['head']}`, working tree e UTC
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
{clean_rows}

C01–C06 são recortes de caches/builds já incluídos nos 47,191 GiB do inventário;
A01–A04 também se sobrepõem. Não somar os 32,329 GiB de target a seus filhos.

## Diretórios grandes candidatos a arquivo histórico

| Diretório | Tamanho alocado | Critério |
|---|---:|---|
{history_rows}

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
{dup_rows}

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
''')
print(json.dumps(dict(index=str(p/'README.md'),backlog=str(p/'estado-e-pendencias.md'),cleanup=str(p/'limpeza-e-arquivamento.md'),peak_gib=gib(peak))))
