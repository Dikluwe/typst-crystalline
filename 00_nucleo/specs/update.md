# Spec: `update.rs` (Auto-Atualização do Binário)

## 1. Objetivo Central
O arquivo `update.rs` implementa o comando `typst update`, responsável por atualizar o binário do Typst automaticamente. Ele baixa releases do GitHub, extrai o binário compilado de archives `.tar.xz` (Linux/macOS) ou `.zip` (Windows), e substitui o executável atual via `self_replace`. Suporta rollback e downgrade forçado.

## 2. Atomização da Lógica Pura (Para L1)

### Comparação de Versões
- **`update_needed(current: &Version, release_tag: &str) -> Result<bool>`**: Compara versão semver atual com a tag do release. Lógica pura de parsing e comparação.
- **`is_downgrade(target: &Version, current: &Version) -> bool`**: Verifica se a atualização é um downgrade.
- **`has_update_command(version: &Version) -> bool`**: Verifica se a versão alvo tem o comando `update` (>= 0.8.0).

### Construção de URLs
- **`build_release_url(org: &str, repo: &str, tag: Option<&Version>) -> String`**: Constrói a URL da API do GitHub (latest vs tag específica).

### Resolução de Backup Path
- **`default_backup_path() -> PathBuf`**: Determina o diretório de backup conforme a plataforma (XDG no Linux, Application Support no macOS, %APPDATA% no Windows).

## 3. Efeitos Colaterais Identificados (Para L3)

### Efeito 1: Download HTTP
- `Downloader::download()` — Faz requisições HTTP à API do GitHub para obter informações de release e baixar assets.

### Efeito 2: Extração de Archives
- `extract_binary_from_zip()` — Descompacta ZIP.
- `extract_binary_from_tar_xz()` — Descompacta tar.xz.

### Efeito 3: Manipulação do Filesystem
- `fs::copy()`, `fs::create_dir_all()`, `fs::remove_file()` — Backup e limpeza.
- `env::current_exe()` — Localização do executável atual.
- `NamedTempFile::new()` — Arquivo temporário.

### Efeito 4: Self-Replace
- `self_replace::self_replace()` — Substituição atômica do binário em execução.

## 4. Estruturas de Dados

```rust
struct Asset { name: String, browser_download_url: String }
struct Release { tag_name: String, assets: Vec<Asset> }
enum UpdateAction { Update, Revert, AlreadyUpToDate }
```
