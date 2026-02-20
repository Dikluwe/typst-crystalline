# Spec: `eval.rs` (Avaliação de Expressões)

## 1. Objetivo Central
O arquivo `eval.rs` implementa o comando `typst query`/`typst eval` (embora o nome seja eval, trata de `evaluate_expression`). Ele compila o documento principal (se necessário) e avalia uma expressão código Typst num escopo isolado, imprimindo o resultado em um formato serializado (JSON, YAML).

## 2. Atomização da Lógica Pura (Para L1)

### Avaliação de Expressão
- **`evaluate_expression(...) -> SourceResult<Value>`**: Utiliza `typst_eval::eval_string` para computar a expressão em "SyntaxMode::Code". Recebe contexto, constrói a string e retorna o Value. 

### Serialização de Saída
- **`serialize(data: &impl Serialize, format: SerializationFormat, pretty: bool) -> StrResult<String>`**: Serializa o `Value` (ou qualquer DTO) em formato JSON (pretty ou raw) ou YAML usando `serde_json` e `serde_yaml`. 100% pura.

## 3. Efeitos Colaterais Identificados (Para L3)

### Efeito 1: Escrita de Resultado (Stdout)
- `println!("{serialized}")` — Imprime a string resultante na tela.

### Efeito 2: Configuração de Erro Global
- `set_failed()` (do crate principal) — Seta flag atômica global de erro caso a compilação ou evolução resulte em falhas.

### Efeito 3: Impressão de Diagnósticos
- `print_diagnostics()` — Re-aproveita o módulo compile para lidar via stderr.

## 4. Estruturas de Dados Chave
- Usa `SerializationFormat` (Json, Yaml) e `Value`.
