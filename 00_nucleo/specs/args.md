# Spec: `args.rs` (CLI Parser e Entrypoint)

## 1. Objetivo Central
O arquivo `args.rs` original define a estrutura de dados (via `clap`) da interface de linha de comando (CLI) do Typst. Seu objetivo principal é capturar, tipar, validar sintaticamente e mapear entradas do usuário (flags, opções, subcomandos) para estruturas internas (`CliArguments`, `Command`, `CompileArgs`, etc.). Ele também fornece *parsers* simples e classes wrappers (como `Input` e `Output`) para manipulação primária do que o usuário digitou, servindo como a porta de entrada da aplicação.

## 2. Atomização da Lógica Pura (Para o futuro L1)
As funções e métodos a seguir **não dependem de I/O** e devem ser transferidas para módulos de lógica pura (Tekt L1):

- **`parse_page_number(value: &str) -> Result<NonZeroUsize, &str>`**: Regra de negócio simples para formatar e validar que strings numéricas de páginas não podem ser zero.
- **`Pages::from_str(value: &str) -> Result<Self, &str>`**: Analisador (`parser`) lógico de *ranges* textuais (ex: `1-3`, `4-`, `-5`) para transformar os índices lógicos em representações de intervalo inclusivo (`RangeInclusive<Option<NonZeroUsize>>`).
- **`parse_sys_input_pair(raw: &str) -> Result<(String, String), String>`**: Utilitário puro de string splitting (no `=` sign) para separar chaves e valores.
- **`parse_source_date_epoch(raw: &str) -> Result<DateTime<Utc>, String>`**: Parsing de uma *string* representando o timestamp numérico para um objeto de data explícito, sem depender do *clock* real.
- **`OutputFormat::is_paged(&self) -> bool`**: Retorna logicamente se a variação formatada (PDF, PNG, SVG) diz respeito à saída empaginada ou não (HTML).
- **`input_value_parser` e `output_value_parser`**: Tradutores semânticos puros atrelados ao Clap que validam se a string lida é vazia e convertem magicamente o token `"-"` em suas variantes enumeradas `Stdin`/`Stdout`, sem ainda acionar IO.

## 3. Efeitos Colaterais Identificados (Para os futuros Contratos L3)
A implementação atual de `args.rs` se acopla a efeitos colaterais (side-effects) que prejudicam a testabilidade unitária pura e violam o `Invariante de Nucleação`. São eles:

- **Efeito 1: Leitura Indireta de Variáveis de Ambiente (`std::env`)**: Diversos campos nas estruturas `clap` usam a macro `env = "..."` (ex: `TYPST_CERT`, `TYPST_ROOT`, `TYPST_FONT_PATHS`, `TYPST_PACKAGE_PATH`). Isso acopla o domínio lógico de *Argumentos* com o ambiente físico do Sistema Operacional.
- **Efeito 2: Acesso de Escrita ao FileSystem (`std::fs::File::create`, `std::fs::write`)**: O *wrapper* `Output::write` ou `Output::open` não retorna apenas a intenção de escrita; ele interage e modifica ativamente o disco rígido, disparando chamadas L3 disfarçadas de tipos L0/L1. 
- **Efeito 3: Interação com Standard Output (`std::io::stdout()`, `std::io::Write`)**: O *wrapper* `Output::open` retorna o tipo `OpenOutput` que trava (`lock`) a saída direta do terminal caso o output configurado seja `Stdout`.

*(Ver `00_nucleo/contracts/args_io.rs` para as abstrações/interfaces propostas).*

## 4. Glossário / Assinaturas (Estruturas de Dados)

### Raízes de Comando
```rust
pub struct CliArguments {
    pub command: Command,
    pub color: ColorChoice,
    pub cert: Option<PathBuf>,
}

pub enum Command {
    Compile(CompileCommand),
    Watch(WatchCommand),
    Init(InitCommand),
    Query(QueryCommand),
    Eval(EvalCommand),
    Fonts(FontsCommand),
    Update(UpdateCommand),
    Completions(CompletionsCommand),
    Info(InfoCommand),
}
```

### Acumuladores de Contexto
```rust
pub struct CompileArgs { pub input: Input, pub output: Option<Output>, pub format: Option<OutputFormat>, /* ... */ }
pub struct WorldArgs { pub root: Option<PathBuf>, pub font: FontArgs, pub package: PackageArgs, /* ... */ }
pub struct ProcessArgs { pub jobs: Option<usize>, pub features: Vec<Feature>, pub diagnostic_format: DiagnosticFormat }
pub struct PackageArgs { pub package_path: Option<PathBuf>, pub package_cache_path: Option<PathBuf> }
pub struct FontArgs { pub font_paths: Vec<PathBuf>, pub ignore_system_fonts: bool, pub ignore_embedded_fonts: bool }
```

### Wrappers Periféricos
```rust
pub enum Input { Stdin, Path(PathBuf) }
pub enum Output { Stdout, Path(PathBuf) }
pub enum OpenOutput<'a> { Stdout(std::io::StdoutLock<'a>), File(std::fs::File) }
```

### Semânticas Enum
```rust
pub enum OutputFormat { Pdf, Png, Svg, Html }
pub enum Target { Paged, Html }
pub enum SerializationFormat { Json, Yaml }
pub enum DiagnosticFormat { Human, Short }
pub enum DepsFormat { Json, Zero, Make }
pub enum Feature { Html, A11yExtras }
pub enum PdfStandard { V_1_4, V_1_5, /* ... */ A_1b, /* ... */ UA_1 }
```
