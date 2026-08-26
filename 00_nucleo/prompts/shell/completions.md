# Prompt L0 — comando `typst completions`
Hash do Código: be42ff66

**Camadas:** L2/L4  
**Ficheiros alvo:** `02_shell/src/cli.rs`, `02_shell/src/completions.rs`,
`04_wiring/src/main.rs`  
**Estado:** especificado; aguarda confirmação ADR-0127

## Medição anterior à decisão

O vanilla recebe um shell enumerado e gera o script a partir da própria árvore
clap (`args.rs:267-273`; `completions.rs:8-12`). Medição direta do Bash
ratificado mostra que o comando oculto `query` permanece nas tabelas internas
e em `opts` (linhas 39, 88 e 521 do stdout); portanto `hide` no help não implica
remoção da completion. O cristalino já usa `CommandFactory`, mas não possuía o
comando.

**Classificação:** protocolo público de shell. A fonte da verdade deve ser a
árvore clap vigente; manter templates manuais seria refutado assim que uma flag
nova não aparecesse na completion.

## Contrato

```text
typst completions <bash|elvish|fish|powershell|zsh>
```

1. Shell inválido falha no parsing com exit 2.
2. Script é escrito integralmente em stdout; stderr vazio no sucesso.
3. Geração usa `Args::command()` sem filtro manual. `query` oculto pode aparecer
   como no vanilla; não manter uma segunda árvore divergente só para completions.
4. Nenhum ficheiro é criado e nenhuma configuração do utilizador é alterada.
5. Erro de escrita em stdout retorna exit não zero.

Adicionar `clap_complete` somente a L2. L4 faz apenas o dispatch do
`RunIntent` de saída imediata; L3 não participa.

## Testes e aceitação

Um snapshot estrutural por shell (nome do binário, `compile`, `fonts`,
`completions` e `watch` quando existir), stdout não vazio e determinismo entre duas
execuções do mesmo build. Não exigir bytes idênticos entre versões distintas
de `clap_complete` sem medir mudança pública.

## Gate

Novo comando público: confirmação obrigatória ADR-0127.
