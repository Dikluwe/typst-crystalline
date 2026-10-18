# Limpeza de caches executada — 18/10/2026

Limpeza autorizada após o inventário. Foram retirados **63
diretórios de cache**, preservando fontes, L0s, arquivos novos, dados de sondas,
aprovações, histórico Git e executáveis de referência nos caminhos originais.
Os grupos de arquivamento histórico A01–A04 e documentos D01–D04 não foram
removidos ou movidos. Nenhum código produtivo ou API mudou; sem commit/release.
O README da raiz recebeu o link do relatório; um apontador de limpeza executada
foi adicionado ao diretório do inventário, sem reescrever seus relatórios selados.

## Medição e resultado

HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, branch Tekt, working tree não commitado.
Coleta anterior em `before.json.gz`: status e diffstat exatos, arquivos Git,
hashes e espaço livre. A primeira execução e a continuação têm UTC/comandos e
cgroup nos recibos preservados; conclusão UTC `2026-10-18T15:24:23.253243+00:00`.

| Grupo do inventário | Diretórios | Arquivos/entradas | GiB alocados nos caminhos removidos |
|---|---:|---:|---:|
| C01 — Incremental debug | 1 | 17684 | 13.033 |
| C02 — HTML/índices Rustdoc e quarentena | 2 | 5339 | 0.191 |
| C03 — Deps/build/fingerprints debug | 3 | 10107 | 17.355 |
| C04 — Builds da ferramenta surface-inventory | 4 | 2125 | 0.896 |
| C05 — Deps/build/fingerprints/incremental release | 4 | 2413 | 1.303 |
| C06 — Deps/build/fingerprints/incremental do oracle | 4 | 4604 | 2.048 |
| C07 — Caches Python | 45 | 201 | 0.002 |

Total medido nos caminhos de cache: **34.829 GiB**.
Esse número inclui artefatos preservados; não é espaço recuperado líquido.
Antes de remover os caches foram retidas 706 referências a artefatos, deduplicadas
em **469 objetos distintos**. As cópias intermediárias
ocupavam 15,836 GiB por entrada; após compactação e verificação integral foram
substituídas por `retencao/artefatos.tar.zst`, de
**3.654 GiB**.

Espaço livre observado no filesystem: **92.477 →
120.484 GiB**, aumento de
**28.007 GiB** durante a janela.
Esse filesystem é compartilhado; reflinks/extents, metadados e atividade externa
podem alterar o valor. Tamanhos por caminho não deduplicam extents e não devem
ser apresentados como garantia de recuperação física.

## O que foi preservado

- 2112 arquivos de fonte/configuração/L0 conferidos por SHA-256.
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
compressão zstd com uma thread. Pico: **5.273 GiB**, sem OOM/OOM-kill.
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
