# Spec: `timings.rs` (Timer Global)

## 1. Objetivo Central
O módulo `timings.rs` suporta a geração de traces de performance JSON (`typst_timing`) para análise. Ele captura os tempos de execução por span durante o ciclo de compilação ou watch, converte IDs de Span em nomes de arquivo/linhas baseados no `SystemWorld`, e escreve os logs de forma seqüencial usando marcações `{n}` no nome do arquivo.

## 2. Atomização da Lógica Pura (Para L1)

### Formatação Inteligente de Caminho
- **`format_recording_path(path: &Path, index: usize) -> StrResult<PathBuf>`**: Substitui a macro `{n}` no caminho pelo iterador do watch. Caso não seja um nome que permita multiplas compilações (`{n}`) e tentarmos salvar mais de uma vez (watch repetido), ele gera erro de domínio puro.

### Resolução de Span
- **`resolve_span(world: &dyn World, span: Span) -> Option<(String, u32)>`**: Consulta o `world` global sem nenhum acoplamento adicional de sistema para mapear Byte Offset para um par `(FileId, LineNumber)`. Essa tradução é 100% isenta de novos I/Os.

## 3. Efeitos Colaterais Identificados (Para L3)

### Efeito 1: Mutação de Estado Global de Timing
- `typst_timing::enable()` e `clear()`. Crate baseia-se em macros globais injetados indiretamente.

### Efeito 2: File System IO
- `File::create(path)`
- `BufWriter::with_capacity(1 << 20, file)`

### Efeito 3: Delegação de Closure (Application Service)
- O Timer encadeia as rotinas principais de compilação usando a closure `f: impl FnOnce(&mut SystemWorld) -> T`.

## 4. Estruturas de Dados Chave
- `Timer`: Wrapper de estado retendo o path output e o ponteiro indexador para watch.
