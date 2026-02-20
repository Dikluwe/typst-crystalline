# Spec: `compile.rs` (Orquestrador de Compilação e Exportação)

## 1. Objetivo Central
O arquivo `compile.rs` original é o motor principal que invoca a biblioteca do `typst` para converter um código-fonte na sua representação final e exportá-la para o sistema de arquivos.
Ele é encarregado de:
- Preprocessar os argumentos brutos (`CompileCommand`) em uma configuração estável (`CompileConfig`);
- Orquestrar o ciclo de vida do `SystemWorld` junto do `Compiler`;
- Rotear a saída gerada (PagedDocument ou HtmlDocument) para os exportadores específicos (`pdf`, `png`, `svg`, `html`);
- Exibir diagnósticos ricos no terminal e interagir com features do sistema (abrir o PDF, rodar servidor HTTP no modo watch).

## 2. Atomização da Lógica Pura (Para o futuro L1)
O arquivo atual mistura orquestração de I/O com regras de negócio. As seguintes funções ou fluxos devem ser isolados em módulos de domínio puro da L1:

- **Estratégia de Inferência de Formato**: A lógica em `CompileConfig::new_impl` que analisa a `PathBuf` ou os `Command Args` para determinar a variável pura `OutputFormat` (Pdf, Png, Svg, Html).
- **Validação de Standards e Tags PDF**: O bloco que verifica se `no_pdf_tags` fere os padrões restritivos de PDF/A ou PDF/UA, gerando uma lista de mensagens de `Warning` puras.
- **Processamento de Templates Numéricos (`output_template::*`)**: Funções `has_indexable_template` e `format` responsáveis unicamente por substituir strings lógicas (`"{p}", "{0p}", "{n}", "{t}"`) pelo número do índice renderizado, usando `width` em base 10.
- **Conversão de Structs Temporais (`convert_datetime`)**: Conversor matemático sem efeito colateral de `chrono::DateTime<Tz>` para `typst::foundations::Datetime`.
- **Match de Tipos e Coerção (`impl From<PdfStandard> for typst_pdf::PdfStandard`)**: Tratação puramente semântica entre tipos de frameworks externos.
- **Validação Restritiva de Stdout/Deps (`CompileConfig::new_impl`)**: Regras lógicas puras que rejeitam a combinação de Watch Mode com output via Stdout, ou Deps via Stdout se o principal já for Stdout.

## 3. Efeitos Colaterais Identificados (Para os futuros Contratos L3)
A camada que engolobiliza os processamentos se interliga profundamente ao ambiente da máquina. Os Seguintes I/Os violam a arquitetura pura e devem ter interfaces (contratos) definidos no Núcleo:

- **Efeito 1: Relógio do Sistema (Clock I/O)**: O uso de `chrono::Local::now()` para obter o timestamp local no `export_pdf`, e o uso de `std::time::Instant::now()` e `start.elapsed()` para cronometrar a operação.
- **Efeito 2: Acionador de Processos (Spawning I/O)**: O módulo `open_output` e `open_path` utilizam a crate externa `open::that_detached` para interagir com a interface Desktop do sistema operacional e subir aplicações fora do controle do Typst.
- **Efeito 3: Servidor HTTP Acoplado**: Subir um socket ou um `HttpServer` via porta de rede local (para HTML watcher).
- **Efeito 4: Escrita Direta Otimizada**: Usar caminhos diretos (IO de filesystem) na escrita dos buffers (`config.output.write(...)` e `write_deps()`).
- **Efeito 5: Diagnósticos com Código ANSI no Terminal**: Uso da biblioteca `codespan_reporting` para jogar bytes coloridos e erros lidos sob os locks do `stdout/stderr` (através de `terminal::out()`).
- **Efeito 6: Estado Global/Cache de Exportação (`ExportCache`)**: Controle de concorrência global em `RwLock<Vec<u128>>` e cálculo de `hash128` das imagens renderizadas, invalidando renderizações futuras (um puro side effect que quebra o determinismo idempotente).

*(Ver `00_nucleo/contracts/compile_io.rs` para as abstrações/interfaces propostas).*

## 4. Glossário / Assinaturas (Estruturas de Dados)

### Configuração
```rust
pub struct CompileConfig {
    pub warnings: Vec<HintedString>,
    pub output_format: OutputFormat,
    pub pdf_standards: PdfStandards,
    // ... [Dependências legadas misturadas]
}
```

### Modelos Enums
```rust
enum ImageExportFormat { Png, Svg }
```

### Caches em Memória
```rust
pub struct ExportCache {
    pub cache: RwLock<Vec<u128>>,
}
```
