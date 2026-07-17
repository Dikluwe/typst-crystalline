---
# P635 — `#break`/`#continue`/`#return` não funcionam mesmo dentro do contexto certo

> **Passo:** 635
> **Data:** 2026-07-09
> **Foco:** P634 encontrou, de passagem, que o cristalino nunca implementou o mecanismo que faz `#break`, `#continue`, e `#return` afectarem de facto o fluxo de execução. Mesmo usados correctamente — `#break` dentro de um `#for`, `#return` dentro de uma função — avaliam para `Value::None` e o ciclo/função continua normalmente, como se a instrução não existisse. Isto foi encontrado por acidente e catalogado como "trabalho futuro" atrás de três outros grupos de correcções menores. Este passo eleva a prioridade e implementa o mecanismo a sério.
> **Tipo:** Sonda + Implementação.
> **Tamanho:** L. Toca o avaliador em vários pontos — ciclos, funções, blocos de código.
> **ADR-0108 EM VIGOR.** **ADR-0114 EM VIGOR** — isto é uma peça central da linguagem; sonda obrigatória antes de qualquer código, para não repetir o padrão desta sequência inteira de "corrigir depressa, descobrir mais tarde que não chegou".

---

## Contexto

O vanilla usa um mecanismo chamado `FlowEvent`, propagado através da máquina de avaliação: quando `#break` é avaliado dentro de um ciclo, não devolve só um valor — sinaliza ao ciclo que deve parar de iterar. O mesmo para `#continue` (salta para a próxima iteração) e `#return` (sai da função com o valor dado). O cristalino nunca teve isto — `#break`, `#continue`, e `#return` sempre foram tratados como expressões que avaliam para algo, sem nunca comunicar de volta ao ciclo/função que os envolve.

---

## Sonda

### Confirmar o mecanismo do vanilla

```bash
grep -n "FlowEvent\|enum Flow" lab/typst-original/crates/typst-eval/src/flow.rs
grep -rn "FlowEvent" lab/typst-original/crates/typst-eval/src/*.rs | head -30
```

Mapear como o `FlowEvent` é propagado: através de quê (um campo no `Engine`? um valor especial devolvido por `eval_expr`? uma excepção/erro usado como mecanismo de controlo?).

### Confirmar o estado exacto do cristalino hoje

```bash
grep -n "fn eval_for\|fn eval_while\|fn eval_closure" 01_core/src/engine/eval/control_flow.rs 01_core/src/engine/eval/closures.rs
```

Confirmar como os ciclos (`eval_for`, `eval_while`) e as funções (`eval_closure` ou equivalente) processam o corpo hoje — presumivelmente chamam `eval_expr` para cada iteração/chamada e nunca verificam se o resultado sinaliza paragem.

### Testar directamente o alcance do problema

```bash
cat > /tmp/p635-break.typ <<'EOF'
#let resultado = ()
#for i in range(10) {
  if i == 3 { break }
  resultado.push(i)
}
#resultado
EOF
./target/release/typst /tmp/p635-break.typ /tmp/p635.pdf
pdftotext /tmp/p635.pdf -
```

Esperado no vanilla: `resultado` fica `(0, 1, 2)`. Confirmar o que o cristalino produz hoje — provavelmente `(0, 1, 2, 4, 5, 6, 7, 8, 9)` (falta o 3, mas o ciclo não pára).

```bash
cat > /tmp/p635-return.typ <<'EOF'
#let f(x) = {
  if x < 0 { return "negativo" }
  "positivo"
}
#f(-5)
#f(5)
EOF
./target/release/typst /tmp/p635-return.typ /tmp/p635-return.pdf
pdftotext /tmp/p635-return.pdf -
```

### Critério de fecho da sonda

- [ ] Mecanismo do vanilla confirmado, com `file:line`.
- [ ] Estado exacto do cristalino confirmado — onde `eval_for`/`eval_while`/funções processam o corpo sem verificar sinalização de paragem.
- [ ] Alcance do problema confirmado com testes directos, números concretos (não "provavelmente").
- [ ] Confirmado se este problema afecta algum documento já usado no corpus desta conversa inteira (procurar por `break`/`continue`/`return` no corpus).

```bash
grep -rl "break\|continue\|return" lab/parity/corpus/*/*.typ 2>/dev/null
```

Se algum documento do corpus já usar estas palavras-chave, confirmar se os testes desse documento alguma vez verificaram o resultado com atenção suficiente para teriam apanhado isto, ou se passaram por acaso.

---

## Implementação

Depende da sonda, mas a direcção mais provável: introduzir um mecanismo de sinalização (`enum FlowEvent { Break, Continue, Return(Value) }`) que `eval_expr` possa devolver junto com o valor normal, e que `eval_for`/`eval_while`/o avaliador de funções verifiquem depois de cada avaliação do corpo, parando ou saltando conforme apropriado.

### Critério de fecho da implementação

- [ ] `#break` dentro de `#for`/`#while` pára o ciclo de facto.
- [ ] `#continue` salta para a iteração seguinte de facto.
- [ ] `#return` sai da função com o valor dado, sem continuar a avaliar o resto do corpo.
- [ ] `#break`/`#continue`/`#return` fora de contexto continuam a produzir o erro já implementado em P634.
- [ ] Ciclos aninhados testados — `#break` só sai do ciclo mais interno, não de todos.

---

## Validação

Repetir os testes da sonda, confirmando o resultado correcto:

```bash
./target/release/typst /tmp/p635-break.typ /tmp/p635-depois.pdf
pdftotext /tmp/p635-depois.pdf -
```

Esperado: `(0, 1, 2)`.

```bash
cat > /tmp/p635-aninhado.typ <<'EOF'
#let resultado = ()
#for i in range(3) {
  for j in range(3) {
    if j == 1 { break }
    resultado.push((i, j))
  }
}
#resultado
EOF
./target/release/typst /tmp/p635-aninhado.typ /tmp/p635-aninhado.pdf
pdftotext /tmp/p635-aninhado.pdf -
```

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Sonda completa, mecanismo do vanilla e estado do cristalino confirmados.
- [ ] `FlowEvent` (ou equivalente) implementado.
- [ ] `#break`, `#continue`, `#return` funcionam de facto, testados com números concretos.
- [ ] Ciclos aninhados testados.
- [ ] Casos fora de contexto (já corrigidos em P634) sem regressão.
- [ ] Corpus verificado — qualquer documento que já usasse estas palavras-chave, re-testado.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p635.md`, com hash do commit.

---

## Nota sobre prioridade

Isto passa à frente dos itens 8–16 e 18–23 de P633 (regras `#set`, `counter.display`) já planeados a seguir. Aqueles são casos de validação em falta — um valor errado, aceite sem aviso. Isto é uma funcionalidade central da linguagem, ausente por completo, apesar de existir sintaxe para a invocar. A gravidade não é a mesma.
