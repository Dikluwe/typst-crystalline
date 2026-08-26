# Prompt L0 — comando `typst watch`
Hash do Código: 257f724f

**Camadas:** L2/L3/L4  
**Ficheiros alvo:** `02_shell/src/cli.rs`, `03_infra/src/watch.rs`,
`03_infra/src/world.rs`, `04_wiring/src/main.rs`  
**Estado:** especificado; aguarda confirmação ADR-0127

## Medição anterior à decisão

- O vanilla declara `watch`, alias visível `w`, reutilizando os argumentos de
  compilação (`lab/typst-original/crates/typst-cli/src/args.rs:86-88,122-132`).
- Faz compilação inicial, observa as dependências do World, reseta caches e
  recompila após mudanças (`watch.rs:59-82`). Rejeita stdout como output
  (`watch.rs:22-24`).
- O cristalino tem apenas `compile/eval/query` em
  `02_shell/src/cli.rs:185-195`. `SystemWorld` não expõe hoje uma lista pública
  completa de dependências observadas. `04_wiring/src/eviction.rs` já reserva
  `comemo::evict(10)` para watch futuro.

**Classificação:** comportamento público de produto. A inferência de que um
watch apenas do ficheiro principal seria suficiente é falsa; seria refutada
imediatamente por alteração num `#import`.

## Contrato público

```text
typst watch INPUT [OUTPUT] [opções de compile]
typst w INPUT [OUTPUT] [opções de compile]
```

1. Reutiliza `CompileArgs` sem duplicar defaults.
2. Exige output em ficheiro; output stdout falha antes do laço.
3. Compila uma vez imediatamente.
4. Observa input e dependências realmente lidas na compilação mais recente.
5. Alteração relevante invalida as leituras do World, recompila e atualiza o
   conjunto observado.
6. Erro de compilação não encerra o processo; erro fatal do watcher encerra
   com código não zero.
7. Ctrl-C termina sem corromper o último artefacto válido.
8. HTTP server do vanilla fica fora do escopo até L0 próprio.

## Divisão obrigatória da entrega

Este L0 está **deliberadamente incompleto** até
`P1137-C001-WATCH-2 — dependências transitivas`:

- `WATCH-1`: comando/alias, compilação inicial, mudança do input principal,
  output atómico e ciclo RED→GREEN.
- `WATCH-2`: instrumentação de `SystemWorld` para ficheiros/imports/assets,
  substituição atómica do conjunto observado e revalidação transitiva.

`WATCH-1` não pode ser anunciado como paridade completa; a matriz o classifica
como `PARTIAL` até `WATCH-2`.

## Topologia

- L2: `WatchIntent` e parsing puro.
- L3: watcher de filesystem e inventário de dependências; todo I/O vive aqui.
- L4: composição do intent com a mesma pipeline de compile.
- L1 não importa watcher, relógio ou filesystem.

## Testes e aceitação

- Help e alias; rejeição de stdout.
- Compilação inicial cria artefacto.
- Mudança do input recompila uma vez.
- Em `WATCH-2`, mudança de import e asset recompila; ficheiro irrelevante não.
- Erro transitório preserva o último artefacto e a sessão recupera.

Aceitação é comportamento observável, não igualdade do mecanismo de eventos.

## Gate

Novo comando e comportamento persistente: confirmação obrigatória ADR-0127.

