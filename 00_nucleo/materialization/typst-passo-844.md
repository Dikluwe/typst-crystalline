# Prompt — typst-passo-844: `introspection` — 8 achados (#47-#54)

**Origem**: achados #47 a #54 de P831 (lote 5)
**Estado**: aguardando execução

---

## Regra da linha de trabalho (obrigatória)
Comando exacto + saída literal (vanilla vs cristalino) para cada achado. Contagem de testes de `typst-core` a bater com os testes novos declarados.

---

## #47 (A1) — `query()` devolve `location` em vez de `content`

`type(query(<meta>).first())` — cristalino `location`; vanilla `content`. Colateral: `query(<meta>).first().value` — cristalino `error: cannot access fields on type location`; vanilla emite o valor real. Cristalino: `stdlib/foundations.rs:1202-1213`. Vanilla: `introspection/query.rs:160`. **Implementação**: corrigir `query()` para devolver o `content` do elemento encontrado (com seus campos acessíveis), não só a localização.

## #48 (A2) — `query()`/`locate()` não aceitam seletor de função de elemento

`locate(heading)` — cristalino erro (`parse_selector_arg` sem braço para `Value::Func`, `foundations.rs:1225-1278`); vanilla funciona (seleciona por tipo de elemento). **Implementação**: adicionar o braço `Value::Func` ao parser de seletor, tratando uma função de elemento (como `heading`, `strong`) como seletor por tipo.

## #49 (A3) — `state.at`/`state.final`/`counter.final` não ligados no dispatch

As funções nativas **já existem** (`native_state_at` em `foundations.rs:1154`, `native_state_final` em `:1108`, `native_counter_final` em `:1051`) mas não estão conectadas no dispatch de métodos (`eval/bindings.rs:962-996,1187-1241`). **Este é o achado mais barato do lote** — a implementação já existe, só falta a ligação. **Implementação**: adicionar as três entradas no dispatch.

## #50 (A4) — `counter.at()` não aceita `location`

`counter(heading).at(here())` — cristalino `error: counter.at() requer label ou string`; vanilla `3`. `bindings.rs:1229-1235` vs `counter.rs:452`. **Implementação**: aceitar `Location` como argumento de `counter.at()`, além de label/string já suportados.

## #51 (A5) — `#context ((3,))` mostra join em vez de repr

Cristalino `3` (junta os elementos com `.`); vanilla `(3,)` (repr do array de 1 elemento — note a conexão com o achado #4/P801, já corrigido para `repr()` direto, mas aqui é o display dentro de `#context`, caminho diferente). `stdlib/state.rs:148-159`. **Sonda**: confirmar se fora de `#context` o array já mostra `(3,)` corretamente (P801 já corrigiu isso) — o achado é específico do caminho `value_to_content` usado por `#context`. **Implementação**: usar a mesma rotina de `repr` (já corrigida em P801) dentro de `value_to_content` para arrays, em vez de um join próprio.

## #52 (A6) — `counter.display()` ignora numbering do `#set heading(numbering:)`

Sem argumento, `counter.display()` — cristalino `1`; vanilla `1.` (usa o padrão de numeração ativo do `#set`). `stdlib/counter.rs:182-186` vs `counter.rs:379`. **Implementação**: quando `display()` não recebe padrão explícito, usar o numbering ativo do contexto (mesmo mecanismo já usado para numeração de heading em si).

## #53 (A7) — `counter.display(pattern)` é stub

Padrões testados (`"I"`, `"A"`, `"i"`, `"①"`, `"1.1"`) com contador=2 — cristalino `I|A|i|①|2.1` (não aplica o padrão real); vanilla `II|B|ii|②|2` (numeração romana/alfabética/circular real). Comentário no próprio código cristalino já admite "Pattern minimal" (`counter.rs:234-246`). **Implementação**: portar a lógica real de formatação de padrão de numeração do vanilla (`counter.rs:634`) — provavelmente compartilhável com a lógica de `#numbering()` já existente no projeto (P793, handoff antigo) se ela cobrir os mesmos estilos.

## #54 (A8) — `#context` entre headings desalinha a numeração

`= Um` / `#context 1` / `= Dois` / `= Três` — cristalino numera `1.|1.|2.` (Dois repete o número de Um); vanilla `1.|2.|3.`. Acontece com qualquer `#context` entre headings. Pontos a investigar (já localizados por P831): `layout/heading.rs:58-84`, `entities/counter_registry.rs:127-134`, `pipeline.rs:107-160`. **Sonda**: reproduzir o caso mínimo e confirmar se o `#context` está incrementando ou resetando o contador de heading incorretamente ao ser avaliado no meio do documento.

---

## Validação (comum aos 8)
1. Recompilar após cada correção. Repetir os casos de sonda, batendo com o vanilla.
2. Suíte completa, comando + contagem antes/depois, consolidada no fim.

## Relatório

`00_nucleo/diagnosticos/typst-passo-844-relatorio.md`, uma seção por achado (#47-#54), testes nomeados `p844_a1_...` a `p844_a8_...`.
