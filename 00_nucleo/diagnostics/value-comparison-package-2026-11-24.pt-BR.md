# Proposta de pacote: comparação e ordenação de valores

**24 de novembro de 2026 · aprovado pelo dono em 24/11/2026 (“Aprovo A–F”, no chat) sob a [ADR-0127](../adr/adr-0127-l0-gate-stop-vs-continuous-flow.md) (§2.1, §2.4 e pacotes de paridade)**

A aprovação cobre o escopo abaixo. Durante a execução, o dono instruiu: “Tem erros que não precisam ser copiados do vanilla se forem comprovados comportamento errado”. Três pontos mudaram por isso:

- **E:** iguais ficam estáveis com `<` e com `<=`. Quando `by(anterior, seguinte)` é falso, o callback é chamado com o par invertido e só `true` troca. A inversão do vanilla com `<` não é copiada, e quem usa `<` hoje não vê mudança.
- **A:** o inteiro aparece como float na mensagem só se a conversão for exata. O vanilla mostra `9007199254740992.0` para `9007199254740993`.
- **`calc.ln(float.inf)`**, do pacote anterior: recusado com `the result is not a real number`, no lugar de `result close to -inf`.

## Medição

HEAD `f2c737125edd822ed6a8a1e4552607b2eee5db19`, com a árvore não commitada do pacote de `calc` (`git diff HEAD --stat`: 35 arquivos, listados na [execução daquele pacote](calc-numeric-package-2026-11-24.pt-BR.md)). 5.787 expressões de 21:45:17 a 21:45:40 de Brasília; sondas complementares até 21:47. Cristalino `8a82db8bdc7de8723261ffab5cee5c8639cc2f250677b5e6ea4ccc4b46245d77`, vanilla `e36333ee02be2066646ad632996d92468af9a60b834c6e518ce00ccf3221bb3b`. Script e saídas em `lab/results/comparison-package-2026-11-24/` (`cmp.json` SHA-256 `2062fb1900fa3c2c29509df5f914bd03ebdb040028ead28fb5af749d122f7fd9`), local e não versionado.

Na grade: 5.670 iguais, 49 aceitos só pelo cristalino, 21 só pelo vanilla, 8 valores diferentes e 39 mensagens diferentes.

| Caso | Vanilla | Cristalino |
|---|---|---|
| `1 < float.nan` e os outros três operadores | erro `cannot compare 1.0 with float.nan` | `false` |
| `while`, `if`, `filter` com essa condição | erro | segue o ramo falso |
| `1 == decimal("1")` | `true` | `false` |
| `decimal("1") in (1,)`, dicionários, `contains` | `true` | `false` |
| `(1, 1.0).dedup().len()`, `(10%, 10% + 0pt)` | `1` | `2` |
| `sorted()` de booleanos, comprimentos, ângulos, razões, frações, arrays, datas, durações | ordena | erro |
| `(decimal("2"), 1).sorted()` | ordena | erro |
| `sorted()` de 60 inteiros e strings misturados | erro | **pânico, código 101** |
| `sorted(by: (l, r) => l <= r)` com chaves iguais | conserva a ordem (`abc`) | inverte (`cba`) |
| `float.nan * 1% != 50% + 0pt` | `true` | `false`, e `==` também é `false` |
| `(1,) < (float.nan,)` | mostra `1.0` | mostra `1` |

Com `by: <` e chaves iguais, o resultado é o oposto: `cba` no vanilla e `abc` aqui. A documentação do vanilla pede `true` para iguais e deixa a ordem sem especificação quando o callback devolve `false` nos dois sentidos.

## Causas

1. `eval/operators/ordering.rs` tem braços rápidos por tipo que usam `<` do Rust; NaN cai em `false`. O caminho geral, `value_cmp`, já erra.
2. `stdlib/collections/array_predicate.rs:365` mantém uma segunda comparação, só para int, float, decimal, versão e string.
3. A ordenação usa `sort_by` do Rust e devolve `Equal` quando a comparação falha. Essa ordem inconsistente pode causar pânico.
4. `values_eq` não compara int com decimal, embora a ordenação compare.
5. A inserção de `sorted(by:)` chama o callback com o par na ordem inversa da documentada, e troca quando recebe `true`.
6. `array_dedup` usa a igualdade do Rust, não a da linguagem.
7. O braço `!=` de razão e relativo repete a tolerância em vez de negar `==`.

## Escopo proposto

| Parte | Mudança | Efeito |
|---|---|---|
| A | os quatro operadores usam a comparação única | NaN vira erro, com int mostrado como float |
| B | `sorted()` e `sorted(key:)` usam a mesma comparação, sem pânico | aceita mais tipos; pânico vira erro |
| C | int e decimal iguais em `==`, `!=`, `in`, arrays, dicionários e `assert.eq` | valor muda |
| D | `dedup` usa a igualdade da linguagem | remove mais repetidos |
| E | `sorted(by:)` chama `by(anterior, seguinte)` e troca com `false` | `<=` e `>=` ficam estáveis; com `<` e `>` os iguais se invertem, como no vanilla |
| F | `!=` é a negação de `==` | corrige NaN |

`min` e `max` de `calc` deixam o tratamento local de NaN e usam a mesma comparação.

**Compatibilidade.** A faz documentos que hoje compilam errarem. C, D, E e F mudam valores de documentos que hoje compilam. B só aceita mais ou troca pânico por erro. Não há campo, flag, fase, dependência ou interface pública nova.

**Donos.** `eval/operators/{ordering,equality}`, `stdlib/collections/array_predicate`, `stdlib/collections` (chamadas de `dedup`), `calc/extrema` e os testes. A comparação única fica em `ordering`; a igualdade, em `equality`.

## Divergências mantidas e fora

- Ordem de razões e ângulos NaN: o cristalino erra, o vanilla compara por uma ordem total interna. Não copio.
- `float.nan * 1%` igual a si mesmo no vanilla: mantenho `false`.
- `clamp` e `floor` de NaN ficam como no pacote anterior.
- Tolerância de 1e-9 entre razão e relativo, `float.nan * 1fr`, idioma das mensagens, e a posição exata dos erros de `sorted`, hoje nos argumentos.
- O par citado no erro de `sorted` depende do algoritmo. O texto segue a forma do vanilla, o par pode diferir.

## Aceitação

1. L0 primeiro, selos e linhagem limpos.
2. RED por parte, no CLI, com valor, mensagem e posição. Controles: curto-circuito (`false and 1 < float.nan`), estabilidade, `key` avaliada uma vez por elemento, erros do callback, listas grandes misturadas e ordem inconsistente em `by`.
3. Hash e igualdade Rust de `Value` não mudam.
4. Grade repetida; toda diferença restante nas listas acima.
5. Suíte completa, lint e linhagem reversa.

## Decisão pedida

Aprovar A–F como um pacote. A alternativa é aprovar agora A, B e F, que corrigem erro silencioso e pânico, e deixar C, D e E.

## Execução

Árvore não commitada sobre `f2c737125`, junto com o pacote de `calc`: `git diff HEAD --stat` com 47 arquivos, mais os dois diagnósticos. Binário `8ceeac539036038b94a3b80352966bc8691d71189d48d729e57b9fe09c5ecc61`.

- **RED:** três testes novos de CLI, e o caso `ln(float.inf)`, falharam antes do código e passam agora.
- **Suíte:** 10.932 passaram, zero falhas, sete ignorados, com `TMPDIR` em `/dev/shm` e `TYPST_VANILLA`. Lint sem erros nem avisos na cópia dos arquivos do repositório. Linhagem reversa: 1.030, zero erros.
- **Grade**, 21:58:58 a 21:59:21, `cmp-after.json` SHA-256 `4c26e1ccaa1a3f791948a9e6bf90d0ac03dea4e25e7a9f77be97af90a3a3f651`: 5.757 iguais, de 5.787. As 30 restantes são divergências declaradas: 22 de razão NaN e 8 mensagens em que o vanilla mostra outro inteiro. Nenhuma expressão é aceita só pelo cristalino.
- **Grade de `calc`**, repetida: 2.055 iguais. A única mudança é o texto de `ln(float.inf)`.

**Ajustes.** `array.contains` e `array.split` também usavam a igualdade do Rust; `(1, 2, 3).split(2.0)` não dividia. `split` não estava listado e foi corrigido pela mesma causa. `sorted` usa uma ordenação por fusão própria, que devolve o primeiro erro. Com `by`, o callback é chamado de novo com o par invertido quando devolve falso, e o número de chamadas difere do vanilla e do anterior.

**Achado fora.** `datetime(…).year()` não existe no cristalino.
