# Spec: `init.rs` (Scaffolding de Projeto)

## 1. Objetivo Central
O arquivo `init.rs` implementa o comando `typst init`. Ele inicializa um novo projeto na máquina do usuário copiando o conteúdo de um template (que na verdade é um pacote acessível do Universe ou cache).

## 2. Atomização da Lógica Pura (Para L1)

### Determinação do Diretório
- **`resolve_project_dir(arg_dir: Option<&str>, package_name: &str) -> PathBuf`**: Define o diretório de destino: se o usuário passou `--dir`, usa ele; senão, usa o próprio nome do pacote. Lógica pura.

### Erros de Domínio
- **Tipos de Erro**: O pacote não é um template; diretório do projeto já existe; diretório do template não existe no pacote; manifesto malformado.

## 3. Efeitos Colaterais Identificados (Para L3)

### Efeito 1: Sistema de Pacotes
- Acesso ao sistema de pacotes via `packages::system(&command.package)` para buscar o template.
- Obtenção da última versão (`packages.latest_version(&spec)`).
- Download / Descompactação do pacote (`packages.obtain(&spec)`).

### Efeito 2: Sistema de Arquivos (Leitura do Manifesto)
- `std::fs::read_to_string` para ler `typst.toml`.
- Parsing TOML em `PackageManifest`.

### Efeito 3: Sistema de Arquivos (Cópia de Template)
- Validação se o diretório do projeto alvo já existe.
- `fs_extra::dir::copy` para copiar os arquivos do template do pacote para o diretório alvo.

### Efeito 4: Terminal (Output formatado)
- Configuração de terminal em cores via `termcolor`.
- Impressão do sumário da inicialização do projeto (`print_summary`).

## 4. Estruturas de Dados Chave
- `PackageSpec`, `VersionlessPackageSpec`, `PackageManifest`, `TemplateInfo`.
