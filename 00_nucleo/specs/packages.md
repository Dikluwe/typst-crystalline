# Spec: `packages.rs` (Resolução e Armazenamento de Pacotes)

## 1. Objetivo Central
O arquivo `packages.rs` atua como uma fábrica para configurar o `SystemPackages` (provido pelo `typst_kit`). Ele define onde o Typst vai procurar os pacotes importados: no sistema de arquivos local (data dir do OS ou path customizado), no cache local, e na nuvem (Universe) usando o downloader HTTP.

## 2. Atomização da Lógica Pura (Para L1)

- **`PackageStorageConfig`**: Pode ser definido um DTO para separar o acoplamento direto dos argumentos da CLI (`PackageArgs`) da criação do repositório físico.

> Nota: A lógica do `packages.rs` é inteiramente focada na composição de dependências (I/O). A pesquisa, extração e validação real dos pacotes ocorrem dentro da biblioteca `typst_kit::packages`. 

## 3. Efeitos Colaterais Identificados (Para L3)

### Efeito 1: Leitura de File System
- `FsPackages::new()` — Cria o provedor de pacotes num path customizado.
- `FsPackages::system_data()` — Localiza o data directory do SO (ex: `~/.local/share/typst/packages`).
- `FsPackages::system_cache()` — Localiza o cache directory do SO (ex: `~/.cache/typst/packages`).

### Efeito 2: Acesso à Rede (Universe)
- `UniversePackages::new()` — Inicializa o registro online acoplado ao `downloader()` HTTP.

## 4. Estruturas de Dados Chave
- `SystemPackages`: O agregado que contém (1) Local Provider, (2) Cache Provider, (3) Network Provider.
