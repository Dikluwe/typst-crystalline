# Spec: `main.rs` (Entry Point do CLI)

## 1. Objetivo Central
O arquivo `main.rs` é a raiz de orquestração do binário `typst`. Ele:
1. Parseia os argumentos CLI via `clap::Parser` em um `LazyLock<CliArguments>`.
2. Despacha cada subcomando para o módulo correspondente via `dispatch()`.
3. Gerencia o exit code do processo via `thread_local!` cell.
4. Imprime erros e hints formatados com cores no terminal.
5. Fornece a função utilitária `serialize()` para JSON/YAML.
6. Inclui um stub de `update` quando a feature `self-update` está desabilitada.

## 2. Atomização da Lógica Pura (Para L1)

### Serialização (`serialize`)
- Já clivada anteriormente em `eval_logic.rs` como `serialize_value`. O `main.rs` legado exporta esta função para uso transversal. **Não precisa de duplicação.**

### Formatação de Erro e Hint
- **`format_error_prefix() -> &'static str`**: Retorna a string `"error"`.
- **`format_hint_prefix() -> &'static str`**: Retorna a string `"hint"`.
- Estas são triviais, mas a lógica de *montagem* das mensagens formatadas pode ser isolada.

### Stub de Update sem Feature
- Gera um `bail!` com mensagem fixa. Lógica de domínio pura sem I/O.

## 3. Efeitos Colaterais Identificados (Para L3)

### Efeito 1: Parsing de Argumentos e Greeting
- `CliArguments::try_parse()` — Parsing do `clap`, com interceptação do erro para exibir `greet`.
- `LazyLock` — Estado estático mutável.

### Efeito 2: Terminal Output (Erros e Hints)
- `terminal::out()` — Handle global para escrita colorida.
- `set_color()`, `reset()` — Formatação com `term::Styles`.

### Efeito 3: Exit Code e SIGPIPE
- `sigpipe::reset()` — Reset de sinal POSIX.
- `EXIT` thread_local — Controle do status do processo.

### Efeito 4: Dispatch de Subcomandos
- `dispatch()` — Orquestra a invocação de cada handler.

## 4. Estruturas de Dados Chave
- `CliArguments`, `Command` — enums de subcomando do CLI.
- `EXIT: Cell<ExitCode>` — Thread-local com o exit code final.
