# Spec: `world.rs` (Implementação do World do Typst)

## 1. Objetivo Central
O arquivo `world.rs` implementa o `SystemWorld`, que é a ponte entre o compilador do Typst e o sistema operacional. Ele fornece acesso a arquivos (fontes, imagens, código-fonte), variáveis de ambiente, relógio do sistema e biblioteca padrão configurada.

## 2. Atomização da Lógica Pura (Para L1)

### Inicialização da Biblioteca
- **`build_library(inputs: Dict, features: Vec<Feature>) -> Library`**: Configura a biblioteca padrão com as entradas e features experimentais (HTML, A11y).

### Gestão de Tempo de Compilação
- **`calculate_today(now: DateTime<Utc>, offset: Option<Duration>, fixed: bool) -> Datetime`**: Lógica de cálculo da data "hoje" considerando offsets de fuso horário e se o tempo deve ser fixo (reproducible builds) ou dinâmico.

### Resolução de Nomes para Diagnósticos
- **`resolve_diagnostic_name(id: FileId, root: &Path, workdir: &Path) -> String`**: Lógica de manipulação de caminhos para exibir nomes de arquivos amigáveis em erros, lidando com caminhos relativos ao projeto ou pacotes.

### Validação de Caminhos de Entrada
- **`VirtualPath::virtualize(root, path)`**: (Já existente no Typst, mas orquestrado aqui). Lógica de garantir que arquivos estão dentro do root.

## 3. Efeitos Colaterais Identificados (Para L3)

### Efeito 1: Sistema de Arquivos (FS)
- **`working_directory()`**: `std::env::current_dir()`.
- **`canonicalize_path(path: &Path)`**: Resolução de caminhos absolutos e links simbólicos.
- **`load_file(path: &Path)`**: Leitura física de arquivos do disco.
- **`discover_fonts(paths: &[PathBuf])`**: Varredura de diretórios em busca de fontes.

### Efeito 2: Entrada Padrão (Stdin)
- **`read_from_stdin()`**: Leitura de bytes do `io::stdin()`.

### Efeito 3: Relógio do Sistema
- **`get_system_time()`**: `Utc::now()` e conversão para fuso horário local.

### Efeito 4: Processamento Paralelo (Rayon)
- **`initialize_thread_pool(jobs: usize)`**: Configuração global do pool de threads do Rayon.

## 4. Estruturas de Dados Chave

```rust
struct SystemWorld {
    workdir: PathBuf,
    library: Library,
    fonts: FontStore,
    files: FileStore,
    now: Now,
}

enum Now {
    Fixed(DateTime<Utc>),
    System(OnceLock<DateTime<Utc>>),
}

enum WorldCreationError { ... }
```
