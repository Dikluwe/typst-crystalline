from pathlib import Path
import json,gzip,hashlib,datetime
p=Path(__file__).resolve().parent;root=p.parents[2]
def load(name):return json.loads(gzip.decompress((p/name).read_bytes()))
h=load('commits.json.gz');a=load('audit.json.gz');plan=load('commit-plan.json.gz')
resources={f.name:json.loads(f.read_text()) for f in p.glob('*-resources.json')}
peak=max(int(v['memory_after']['memory.peak']) for v in resources.values())
rows='\n'.join(f'| {i} | `{r["commit"][:12]}` | {r["title"]} | {len(r["files"])} |' for i,r in enumerate(h['commits'],1))
whites=[w for c in h['commits'] for w in c.get('archive_whitespace',[])]
(p/'relatorio.md').write_text(f'''# Commits por etapa — 18/10/2026

As alterações locais foram organizadas em **16 commits assinados**: 15 etapas
listadas abaixo e o commit que contém este relatório/índice final. A série parte
de `{h['base']}`, branch `Tekt`; não houve amend, rebase, reset do working tree,
push ou release. O conteúdo final de código/L0 permanece igual ao estado local
que o humano pediu para commitar.

## Evidência antes da organização

O último freeze funcional R3-T1 registrou 9111 testes aprovados, zero falhas e
sete ignorados, incluindo cinco doctests. Esta organização não repetiu build ou
suíte: os caches tinham acabado de ser limpos e não houve mudança funcional.
O inventário de origem e os relatórios de cada etapa preservam HEAD/estado,
fontes, UTC, receitas e resultados; os números antigos continuam vinculados aos
seus próprios snapshots, não foram reatribuídos aos novos hashes de commit.

Para separar arquivos compartilhados por várias etapas, reconstruí os estados
históricos a partir de snapshots completos e patches reversos. Os primeiros
checkpoints foram confrontados também com os hashes dos freezes selados.
Os 12 estados de código tiveram V1/V5/V15/V26 e selos reversos reexecutados em
cópia temporária, sem alterar os fontes produtivos. Depois dos commits, a árvore
produtiva de cada um foi comparada com o checkpoint exato e a assinatura SSH foi
verificada usando a chave pública configurada pelo usuário. Nenhuma configuração
Git global/de repositório foi modificada.

## Série de commits

| Ordem | Commit | Etapa | Arquivos no commit |
|---|---|---|---:|
{rows}
| 16 | Este commit | Registra organização dos commits, recuperação de evidências e índice final | Metadados/documentação |

A identidade do último commit é obtida pelo próprio histórico Git, evitando
colocar um hash autorreferente no documento. `commits.json.gz` mapeia os 15
predecessores; `audit.json.gz` guarda a conferência independente das árvores e
assinaturas. Os 26 novos arquivos de Rust/L0/núcleos foram incluídos nas etapas
proprietárias, com as propostas e aprovações pertinentes.

## Evidências necessárias versionadas

Além dos relatórios, scripts, snapshots e manifests já admitidos pelo Git,
foram criados **{len(a['packages'])} pacotes** `evidencias-commit.json.gz`, com
**{a['evidence_files']} arquivos** antes ignorados: logs, recibos JSON, patches,
fontes .typ, dados .bib/.csl e observadores/saídas textuais pertinentes.
Total compactado adicional: **{a['evidence_compressed_bytes']/1024**2:.2f} MiB**.
Cada pacote contém bytes em base64, tamanho, modo e SHA-256, com índice separado.
Os arquivos originais continuaram locais e não foram modificados.

Todos os pacotes foram decodificados e seus arquivos confrontados por tamanho,
hash e bytes com os originais. A política de captura está em pack-evidence.py e
os índices em evidence-packages.json.gz. As evidências e scripts anteriores
não foram reescritos para adaptar sua proveniência ao histórico novo.

PDFs/PNGs, fontes binárias e executáveis grandes não foram adicionados em massa.
Permanecem nos caminhos locais registrados pelos manifests, incluindo o oracle
e o arquivo de retenção de caches de 3,65 GiB. Os dados compactos, receitas,
fontes das sondas e hashes necessários à interpretação/reprodução foram
versionados. Isso não equivale a backup externo de todos os outputs brutos.

## Recuperar evidências após um clone

Verificar sem escrever:

```sh
python3 00_nucleo/diagnosticos/commits-por-etapa-2026-10-18/restore-evidence.py 00_nucleo/diagnosticos/r3-t1-materializacao-2026-10-18/evidencias-commit.json.gz
```

Adicionar `--restore` ao comando para materializar os caminhos originais.
O helper valida os hashes, confina os caminhos ao repositório e recusa
sobrescrever conteúdo diferente. Repetir para os pacotes dos lotes a reproduzir
antes de executar seus scripts históricos. As fixtures e metadados compactos
são recuperáveis; binários/saídas grandes dependem do armazenamento local ou de
rebuild conforme a receita e o pin originais.

## Verificação final e limites

- 12 checkpoints com linhagem estrita e selos reversos válidos.
- 15 árvores commitadas e assinaturas verificadas antes do commit de entrega;
  o último commit é verificado pelo comando de fechamento.
- {a['source_files_unchanged']} arquivos Rust/L0/núcleos conferidos contra o estado final
  anterior à organização, sem diferenças de conteúdo.
- Nenhum arquivo visível ao Git ficou fora de uma etapa; o último commit inclui
  o link no README, estes scripts e os recibos compactos da organização.
- Três avisos de whitespace em sondas históricas seladas foram preservados
  exatamente: linha em branco final nas duas cópias de custom-csl.py e espaço final em
  causal-audit.py. Seus hashes coincidem com os manifests originais. A checagem
  de whitespace produtivo permaneceu estrita; não se modificaram políticas de
  Clippy/Tekt ou asserções para acomodar esses arquivos de evidência.

As tentativas de preparar a cópia dos checkpoints e de aplicar a checagem a
arquivos históricos estão preservadas em execucao.json.gz, incluindo os exits
não zero. O setup precisou do manifesto de benches e de ajuste no gravador de
recibos; depois os 12 checkpoints passaram. São falhas do organizador, não RED
funcional do produto. A retomada de commits continuou dos commits já assinados,
sem reescrever o histórico.

Operações pesadas sequenciais em cgroup: teto **12 GiB**, High 8 GiB, swap zero.
Pico da organização até a auditoria: **{peak/1024**3:.3f} GiB**; recursos por job
preservados em execucao.json.gz. O commit final não recompila o produto.

A prontidão de produção e a paridade geral continuam abertas: 105 V16 herdados,
Context, coerções de retorno e demais frentes do inventário permanecem pendentes.
Esta série materializa o que já estava feito; não declara essas dívidas resolvidas.
''')
# Preserve raw orchestration receipts, including failed attempts, without forcing
# generated logs/JSON individually or including transient checkpoint copies.
raw={}
for q in sorted(p.iterdir()):
 if q.is_file() and (q.suffix in ['.json','.log']):raw[q.name]=dict(sha256=hashlib.sha256(q.read_bytes()).hexdigest(),text=q.read_text())
(p/'execucao.json.gz').write_bytes(gzip.compress(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),resources=resources,files=raw),ensure_ascii=False,indent=2).encode()))
h.pop('delivery_commit_pending',None);h['delivery_commit']='O commit que contém este arquivo; título: Registra commits por etapa e evidências da entrega'
(p/'commits.json.gz').write_bytes(gzip.compress(json.dumps(h,ensure_ascii=False,indent=2).encode()))
print(json.dumps(dict(commits=16,packages=len(a['packages']),evidence_files=a['evidence_files'],ram_gib=peak/1024**3,archival_whitespace=len(whites)),ensure_ascii=False))
