# Spec: `info.rs` (Comando de Informações do Ambiente)

## 1. Objetivo Central
O arquivo `info.rs` implementa o comando `typst info`, responsável por reunir e exibir informações sobre o ambiente de execução do Typst: versão, plataforma, features de compilação e runtime, configuração de fontes, pacotes e variáveis de ambiente relevantes. A saída pode ser formatada como JSON/YAML (máquina) ou como texto colorido (humano).

## 2. Atomização da Lógica Pura (Para L1)

### Parsing de Features
- **`parse_features(feature_list: &str) -> Result<Features>`**: Transforma uma string CSV de nomes de features (`"html,a11y-extras"`) em uma struct tipada `Features { html: bool, a11y_extras: bool }`. Lógica puramente funcional sem efeitos colaterais (exceto a chamada a `crate::print_error` para features desconhecidas, que deve ser separada).

### Construção de Structs de Dados Puros
- **`Platform::new()`**: Retorna constantes compiladas (`OS`, `ARCH`).
- **`Settings::compile_features()`**: Itera sobre features de compilação como descrição humana.
- **`Features::features()`**: Itera sobre features de runtime como descrição humana.
- **`Fonts::custom_paths()` / `Fonts::included()`**: Formatação pura de dados já resolvidos.
- **`Packages::paths()`**: Formatação pura de caminhos de pacotes.
- **`Environment::vars()`**: Mapeia variáveis para pares chave-valor tipados.

### Formatação de Valores
- **`KeyValDesc::format()`**: Formatação tabular com padding.
- **`Value::format()`**: Renderização de valores tipados (`Unset`, `Bool`, `Path`, `String`).

## 3. Efeitos Colaterais Identificados (Para L3)

### Efeito 1: Leitura de Variáveis de Ambiente
A função `get_vars()` faz 15+ chamadas a `std::env::var()` para coletar todas as variáveis relevantes. Este é o principal efeito colateral do módulo.

### Efeito 2: Escrita Colorida no Terminal
- Funções `write_key()`, `write_value_simple()`, `write_value_special()` usam `codespan_reporting::term::termcolor` para escrever no terminal com cores ANSI (Cyan, Green, Blue).
- Dependem de `terminal::out()` (lock do stdout).

### Efeito 3: Resolução de Paths de Sistema
- `FsPackages::system_data()` e `FsPackages::system_cache()` acessam o filesystem para descobrir diretórios de dados e cache.

### Efeito 4: Serialização com Saída Stdout
- `println!("{serialized}")` para output JSON/YAML.

## 4. Glossário / Estruturas de Dados

```rust
struct Info { version, build, features, fonts, packages, env }
struct Build { commit, platform, settings }
struct Platform { os, arch }
struct Settings { self_update, http_server }
struct Features { html, a11y_extras }
struct Fonts { paths, system, embedded }
struct Packages { package_path, package_cache_path }
struct Environment { typst_cert, typst_features, ... }
enum Value<'a> { Unset, Bool(bool), Path(&'a Path), String(&'a str) }
struct KeyValDesc<'a> { key, val: Value, desc }
```
