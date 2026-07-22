# Prompt — typst-passo-843: `foundations` (define) — 7 achados (#40-#46) + achado incidental `array.join` (#60)

**Origem**: achados #40 a #46 e #60 de P831 (lote 5)
**Estado**: aguardando execução

---

## Regra da linha de trabalho (obrigatória)
Comando exacto + saída literal (vanilla vs cristalino) para cada achado. Contagem de testes de `typst-core` a bater com os testes novos declarados.

---

## #40 (F1) — `repr(duration)` diverge

Cristalino `duration(3s)`; vanilla `duration(seconds: 3)`. `eval/repr.rs:91` vs `foundations/duration.rs:137-160`. **Implementação**: corrigir o formato de `repr` para o padrão nomeado do vanilla (`duration(seconds: N, ...)`, com os campos que o vanilla usa quando há mais de uma unidade envolvida — testar duration com dias/horas/minutos combinados, não só segundos).

## #41 (F2) — `repr` de `content` diverge

Cristalino `["hi"space*"bold"*]`; vanilla `sequence([hi], [ ], strong(body: [bold]))`. `repr.rs:329,339` vs `foundations/content/mod.rs:611`. **Implementação**: reescrever o `repr` de `Content` para o formato estrutural do vanilla (nome do elemento + campos, aninhado). Testar com sequências mais complexas (aninhamento de 2+ níveis) para confirmar recursão correta.

## #42 (F3) — `repr(type(none))`/`repr(type(auto))` divergem

Cristalino `none`/`auto`; vanilla `type(none)`/`type(auto)`. `repr.rs:124` vs `foundations/ty.rs:159-163`. **Implementação**: corrigir para o formato `type(...)` também nesses dois casos especiais (provavelmente já funciona para outros tipos — conferir por que `none`/`auto` têm caminho especial).

## #43 (F4) — tipo `bytes` sem constructor

Cristalino `error: type bytes does not have a constructor`; vanilla `bytes(3)` (ou `bytes((1,2,3))`, confirmar assinatura exata). `eval/closures.rs:866-868` vs `foundations/bytes.rs:244-245`. **Nota**: isso já apareceu como achado transversal em relatórios anteriores (P810, P819) — este é o passo que finalmente fecha essa lacuna. **Implementação**: adicionar o constructor `bytes(...)`, aceitando os tipos de entrada que o vanilla aceita (inteiro = N zeros, array de inteiros = bytes literais, string = bytes UTF-8, confirmar a lista completa).

## #44 (F5) — tipo `datetime` sem constructor

Mesma causa estrutural do #43 (`closures.rs:866-868`). Vanilla: `datetime(year: 2024, month: 1, day: 1, ...)` (`foundations/datetime.rs:265`). **Implementação**: adicionar o constructor `datetime(...)` com os argumentos nomeados do vanilla.

## #45 (F6) — `panic`: assinatura e mensagem divergentes

Cristalino: 1 argumento string, mensagem nua (`error: this is wrong`); vanilla: variádico, prefixo `panicked with: `, valores não-string exibidos via `repr` (`error: panicked with: this is wrong`). `stdlib/panic.rs:28-37` vs `foundations/mod.rs:140-152`. **Implementação**: tornar `panic` variádico (múltiplos argumentos concatenados, como o vanilla), adicionar o prefixo `panicked with: `, e usar `repr` para valores não-string.

## #46 (F7) — `assert`: mensagens divergentes

Cristalino: `error: Asserção falhou` (português) / mensagem customizada com prefixo `message:`; vanilla: `error: assertion failed` / `assertion failed: {msg custom}` (sem prefixo `message:`, concatenado direto). `stdlib/assert.rs:68` vs `foundations/mod.rs:179-181`. **Implementação**: corrigir as duas mensagens para o formato e texto (inglês) do vanilla.

---

## #60 (incidental) — `array.join` inexistente

`#("a", "b").join("-")` — cristalino `error: type array has no method 'join'`; vanilla `a-b`. Cristalino: `stdlib/collections.rs:32-111` (19 métodos implementados, falta `join`). Vanilla: `foundations/array.rs:755`.

### Sonda
Confirmar a assinatura completa de `array.join` no vanilla — separador opcional (default vazio?), tratamento de `last:` (separador diferente antes do último elemento, se o vanilla tiver essa opção — confirmar), e comportamento com elementos não-string (erro, ou conversão implícita via `repr`/`display`?).

### Implementação
Adicionar `join` a `collections.rs`, replicando a assinatura e semântica medida.

---

## Validação (comum aos 8 achados deste prompt)
1. Recompilar após cada correção. Repetir os casos de sonda, batendo com o vanilla.
2. Suíte completa, comando + contagem antes/depois, consolidada no fim.

## Relatório

`00_nucleo/diagnosticos/typst-passo-843-relatorio.md`, uma seção por achado (#40-#46, #60), testes nomeados `p843_f1_...` a `p843_f7_...` e `p843_join_...`.
