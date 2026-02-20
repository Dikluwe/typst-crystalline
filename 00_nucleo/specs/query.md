# Spec: `query.rs` (Consultas ao Documento)

## 1. Objetivo Central
O arquivo `query.rs` implementa o comando `typst query` (atualmente depreciado em favor de `typst eval`). Ele compila o documento principal, avalia uma string de seletor dentro do documento, filtra/relaciona os elementos encontrados e formata a saída em JSON ou YAML. Adicionalmente, emite um aviso estruturado sobre a depreciação do comando sugerindo a substituição por `eval`.

## 2. Atomização da Lógica Pura (Para L1)

### Avaliação do Seletor (`retrieve`)
- Avalia a string de seleção utilizando `typst_eval::eval_string`.
- Verifica os elementos batendo o seletor no `Introspector` gerado pela compilação.
- Não possui I/O, opera no contexto da engine do Typst.

### Formatação de Mapeamento (`format`)
- Aplica filtros nos resultados encontrados (ex: extração de `.field`).
- Valida restrições do comando (ex: `command.one` exige conter exatamente 1 item).
- Converte os dados no formato final usando serialização.

### Aviso de Depreciação (`deprecation_warning`)
- Gera o texto rico mostrando como substituir a chamada do CLI atual pela nova chamada de `typst eval`.
- Retorna um `SourceDiagnostic::warning`. Lógica pura de manipulação de string e metadados.

## 3. Efeitos Colaterais Identificados (Para L3)

### Efeito 1: Escrita do Resultado
- `println!("{serialized}")` — Imprime a string do resultado da query formatada.

### Efeito 2: Aviso de Falha e Diagnósticos
- `set_failed()` — Controla a flag de erro global.
- `print_diagnostics` — Utiliza infraestrutura herdada do `compile` para exibir warnings ou erros formatados no `stderr`.

## 4. Estruturas de Dados Chave
- Usa `Target` (tipo do output final: pdf/html, afeta compilação inicial).
- `QueryCommand`, manipulando flags de retorno (`one`, `field`, `format`).
