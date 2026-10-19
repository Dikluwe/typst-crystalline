from pathlib import Path
import gzip,json,hashlib,datetime,collections
p=Path(__file__).resolve().parent;root=p.parents[2]
def load(name):return json.loads(gzip.decompress((p/name).read_bytes()))
def sha(path):
 h=hashlib.sha256()
 with path.open('rb') as inp:
  for b in iter(lambda:inp.read(1024*1024),b''):h.update(b)
 return h.hexdigest()
def gib(n):return f'{n/1024**3:.3f}'
r=load('result.json.gz');plan=load('plan.json.gz')
resources={f:json.loads((p/f).read_text()) for f in ['limpeza-resources.json','verificacao-e-retencao-resources.json']}
assert resources['verificacao-e-retencao-resources.json']['exit']==0
peak=max(int(x['memory_after']['memory.peak']) for x in resources.values())
groups=collections.defaultdict(lambda:dict(roots=0,files=0,allocated=0))
for x in plan['roots']:
 g=groups[x['group']];g['roots']+=1;g['files']+=x['files'];g['allocated']+=x['allocated_bytes']
names={'C01':'Incremental debug','C02':'HTML/índices Rustdoc e quarentena','C03':'Deps/build/fingerprints debug','C04':'Builds da ferramenta surface-inventory','C05':'Deps/build/fingerprints/incremental release','C06':'Deps/build/fingerprints/incremental do oracle','C07':'Caches Python'}
rows='\n'.join(f'| {key} — {names[key]} | {v["roots"]} | {v["files"]} | {gib(v["allocated"])} |' for key,v in sorted(groups.items()))
(p/'relatorio.md').write_text(f'''# Limpeza de caches executada — 18/10/2026

Limpeza autorizada após o inventário. Foram retirados **{len(r['roots_removed'])}
diretórios de cache**, preservando fontes, L0s, arquivos novos, dados de sondas,
aprovações, histórico Git e executáveis de referência nos caminhos originais.
Os grupos de arquivamento histórico A01–A04 e documentos D01–D04 não foram
removidos ou movidos. Nenhum código produtivo ou API mudou; sem commit/release.

## Medição e resultado

HEAD `{r['head']}`, branch Tekt, working tree não commitado.
Coleta anterior em `before.json.gz`: status e diffstat exatos, arquivos Git,
hashes e espaço livre. A primeira execução e a continuação têm UTC/comandos e
cgroup nos recibos preservados; conclusão UTC `{r['end']}`.

| Grupo do inventário | Diretórios | Arquivos/entradas | GiB alocados nos caminhos removidos |
|---|---:|---:|---:|
{rows}

Total medido nos caminhos de cache: **{gib(r['removed_paths_allocated_bytes'])} GiB**.
Esse número inclui artefatos preservados; não é espaço recuperado líquido.
Antes de remover os caches foram retidas 706 referências a artefatos, deduplicadas
em **{r['retention']['objects_verified']} objetos distintos**. As cópias intermediárias
ocupavam 15,836 GiB por entrada; após compactação e verificação integral foram
substituídas por `retencao/artefatos.tar.zst`, de
**{gib(r['retention']['compressed_bytes'])} GiB**.

Espaço livre observado no filesystem: **{gib(r['disk_free_before_bytes'])} →
{gib(r['disk_free_after_bytes'])} GiB**, aumento de
**{gib(r['disk_free_after_bytes']-r['disk_free_before_bytes'])} GiB** durante a janela.
Esse filesystem é compartilhado; reflinks/extents, metadados e atividade externa
podem alterar o valor. Tamanhos por caminho não deduplicam extents e não devem
ser apresentados como garantia de recuperação física.

## O que foi preservado

- {r['productive_files_unchanged']} arquivos de fonte/configuração/L0 conferidos por SHA-256.
- Todos os caminhos rastreados e não rastreados do estado anterior; sem git clean.
- Os 26 novos arquivos produtivos/L0/núcleos, também copiados integralmente em
  `arquivos-novos-protegidos.json.gz` para recuperação local.
- CLI Tekt debug, release/probes na raiz dos perfis e vanilla pinado: hashes
  mantidos nos mesmos caminhos. `result.json.gz` registra os anchors.
- Bibliotecas/rmeta do próprio Typst e executáveis de testes/probes encontrados
  nos caches removidos, mais seis JSONs Rustdoc: retidos no arquivo compactado.
- Binários before/after dos diagnósticos, relatórios históricos, fontes .typ,
  dados importados e outputs de medições; nenhum grupo A foi retirado.

O oracle permanece no pin `bd5ebec285505889e8ff1d4f68804ab147a699f3`, SHA-256
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.

## Verificações

Não havia processo Cargo/Rustc/Rustdoc/Typst ativo antes das remoções. Foram
adquiridos locks dos perfis existentes e verificado que os diretórios candidatos
não continham arquivos rastreados ou novos visíveis ao Git.

- Fontes/configuração/L0: hashes intactos; conjunto de modificados rastreados
  idêntico ao anterior. As alterações locais de implementação foram preservadas.
- Executáveis e bibliotecas mantidos na raiz dos perfis: SHA-256 intacto.
- Archive: todos os objetos descompactados em streaming e comparados com SHA-256
  e tamanho; só depois foram removidas as cópias intermediárias desta limpeza.
- Smoke PDF: CLI Tekt e vanilla compilaram a mesma fonte e ambos emitiram
  `CACHE OK`. Hashes de binários/saídas e argv estão no resultado.
- `git diff --check`: passou. Não houve rebuild ou nova suíte Rust; futuros
  builds reconstruirão as dependências removidas.

A primeira execução removeu os caches, mas seu verificador final parou por tentar
reabrir seis `.pyc` que o inventário anterior incluíra na lista de produção por
estarem sob tools/ci. Eram caches Python ignorados autorizados, não fontes.
A continuação classifica apenas esses seis caminhos planejados como exclusões
esperadas e confere os demais arquivos integralmente; o log e exit 1 original
foram preservados. Nenhuma falta de fonte foi ignorada para obter sucesso.

## RAM e integridade

Dois jobs sequenciais, MemoryMax=12 GiB, MemoryHigh=8 GiB, swap zero;
compressão zstd com uma thread. Pico: **{gib(peak)} GiB**, sem OOM/OOM-kill.
Receitas e eventos completos estão em `resources.json.gz`.

`plan.json.gz` e `removed-files.tsv.gz` registram os caminhos exatos; `retencao.json.gz`
mapeia cada artefato original ao membro do archive; `result.json.gz` registra o
estado posterior. O manifesto sela os recibos e o SHA do archive.

## Recuperação e próximos passos

Listar artefatos: `python3 00_nucleo/diagnosticos/limpeza-caches-2026-10-18/restore.py --list`.
Restaurar um caminho exato do manifesto: usar `--original CAMINHO` no mesmo script.
O helper confere o SHA do archive e do objeto, recusa caminhos externos/symlinks
e não sobrescreve arquivos diferentes. A recuperação de artefatos grandes deve
usar o runner com o teto de 12 GiB, como as outras operações desta etapa.

A retenção é local e ignorada pelo Git: preservá-la antes de futuras limpezas
mais amplas. Os anexos compactos e os fontes novos também precisam ser versionados
ou copiados para outro armazenamento antes de descarte do checkout.

Os trabalhos pendentes continuam no [estado consolidado](../inventario-organizacao-2026-10-18/estado-e-pendencias.md).
A limpeza não corrige os V16 nem amplia a paridade. Arquivar experimentos e
consolidar documentos exige preservar seus inputs/manifests e referências; não
foi executado neste lote de caches.

Retificação do texto do inventário anterior: os dados brutos de duplicatas
registram quatro pares de documentos e um grupo de seis licenças Rustdoc,
não dois pares e três grupos de licenças. As quatro linhas da tabela original
já mostravam esses pares. Nenhuma dessas cópias foi apagada; o diagnóstico
anterior e seu manifesto permanecem históricos e intactos.
''')
(p/'resources.json.gz').write_bytes(gzip.compress(json.dumps(resources,ensure_ascii=False,indent=2).encode()))
print(json.dumps(dict(report=str(p/'relatorio.md'),free_gain_gib=gib(r['disk_free_after_bytes']-r['disk_free_before_bytes']),retention_gib=gib(r['retention']['compressed_bytes']),ram_gib=gib(peak)),ensure_ascii=False))
