# Passo 1000 — Reconhecimento: estrutura de `eval` (cristalino vs vanilla) e do `main` vanilla

**Tipo**: Reconhecimento/mapeamento — **sem decisão, sem correcção, sem código alterado**.
**Executor sugerido**: modelo mais simples e rápido (não precisa do raciocínio pesado usado
nos passos de diagnóstico/fix) — esta é uma tarefa de leitura e listagem, não de causa-raiz.
**Motivo**: antes de fatiar `engine/eval.md` (candidato já identificado, ADR-0104 + Passo
999 Critério A/B), precisamos de saber como o **vanilla organiza a mesma responsabilidade**
— se já existe uma estrutura modular no código de referência, fatiar o cristalino para a
espelhar é mais barato e mais fácil de manter do que inventar uma divisão nova do zero.
**Output**: um documento de mapeamento, não um diagnóstico ADR-0034 (não há causa-raiz a
procurar aqui, só estrutura a descrever).

---

## Parte 1 — Estrutura do `eval` no cristalino

```
find 01_core/src/engine/eval -type f -name '*.rs' | sort
wc -l 01_core/src/engine/eval/*.rs
```

Para cada ficheiro: uma linha a dizer o que cobre (ex.: `math.rs` — avaliação de expressões
matemáticas). Não é preciso ler o corpo de cada função, só a lista de `pub fn`/`fn` de topo
de nível por ficheiro:

```
grep -n '^pub fn \|^fn ' 01_core/src/engine/eval/*.rs
```

## Parte 2 — Estrutura do `eval`/avaliador no vanilla (`lab/typst-original`)

Mesmo levantamento, no código do vanilla — confirmar primeiro qual é o directório
equivalente (pode não se chamar `eval`):

```
find lab/typst-original/crates -type d -iname '*eval*'
```

Se não existir directório com esse nome, procurar onde a avaliação de `Expr`/AST acontece
(procurar pela função central de despacho, tipicamente algo como `fn eval(&mut self, expr:
&Expr)` ou equivalente):

```
grep -rn 'fn eval' lab/typst-original/crates/*/src/ | grep -v test
```

Listar os ficheiros envolvidos e a divisão que o vanilla usa (por tipo de expressão? por
módulo de linguagem — code/markup/math? outra coisa?).

## Parte 3 — Estrutura do `main` vanilla

```
find lab/typst-original -maxdepth 3 -iname 'main.rs'
```

Para o(s) `main.rs` encontrado(s): listar a estrutura de alto nível — como o binário arranca,
que comando/subcomandos existem (compilar, watch, fontes, etc.), que crates/módulos são
chamados a partir daí. Não é preciso perceber a lógica interna de cada subcomando, só o mapa:
`main() → dispatch de subcomando → módulo X`.

## Parte 4 — Comparação directa (só listar, não julgar)

Uma tabela simples: responsabilidade × onde vive no cristalino × onde vive no vanilla.
Exemplo de forma (preencher com o que for encontrado, não presumir):

| Responsabilidade | Cristalino (`01_core/src/engine/eval/`) | Vanilla |
|---|---|---|
| Avaliação de expressões math | `math.rs` | ? |
| Avaliação de markup | ? | ? |
| Bindings/closures | ? | ? |
| ... | ... | ... |

---

## O que este passo NÃO faz

- Não decide como fatiar `eval.md`.
- Não avalia se a estrutura do vanilla é "melhor" — só descreve.
- Não escreve nenhum prompt novo.
- Não altera nenhum ficheiro de código.

## Resultado esperado

Documento de mapeamento (`00_nucleo/diagnosticos/mapeamento-eval-vanilla-cristalino-passo-1000.md`
ou equivalente), com as 4 partes acima preenchidas. Este documento alimenta o próximo passo
(fatiamento de `eval.md`), que aí sim é decisão e precisa do modelo mais forte.
