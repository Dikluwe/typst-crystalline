# Proposta de pacote: métodos pelo tipo e aritmética de datas

**25 de novembro de 2026 · aprovado pelo dono em 25/11/2026 (“Aprovo A–D”, no chat) sob a [ADR-0127](../adr/adr-0127-l0-gate-stop-vs-continuous-flow.md) (§2.1 e pacotes de paridade)**

A [revisão dos L1](l1-review-2026-11-24.pt-BR.md) põe `datetime` e cor como receptor depois das famílias de `set`. Das famílias, a maior que sobra é a das decorações e do texto. Ela fica para depois porque `layout/decorations.rs` tem uma alteração de outra autoria ainda sem commit.

## Os problemas principais

1. **Um método que é função do tipo não pode ser chamado no valor.** No vanilla, `valor.f(…)` chama a função `f` do escopo do tipo com o valor como primeiro argumento. O cristalino só aceita a forma estática: `datetime.year(d)` funciona e `d.year()` falha. O mesmo vale para `red.rgb()`, `red.hsl()` e as outras cinco conversões de cor.
2. **Datas não somam nem subtraem.** `d + duration(days: 3)`, `d - duration(…)` e `d1 - d2` falham com `cannot add datetime and duration`.

## Medição

HEAD `39936c553`, árvore não commitada do pacote de grade e da alteração de outra autoria em `layout/decorations` (`git diff HEAD --stat`: 93 arquivos). Cristalino `90d9821c316e8764621ed789c1b66306972a2f15e9bf061f070a55859877d6c8` (debug), vanilla `e36333ee02be2066646ad632996d92468af9a60b834c6e518ce00ccf3221bb3b`. Sondas de tipos e de `set` refeitas de 22:46:24 a 22:47:40 de Brasília. Conjunto de 50 expressões e casos de borda logo depois, na mesma árvore. Saídas em `lab/results/l1-review-2026-11-24/` e no scratchpad da sessão, locais e não versionados.

Das 50 expressões, 28 diferem:

| Grupo | Casos | Exemplos |
|---|---:|---|
| método pelo escopo do tipo | 11 | `d.year()`, `d.display()`, `d.weekday()`, `d.ordinal()`, `red.rgb()`, `red.cmyk()`, `red.luma()`, `(1,2,3).range()`, `"a".from-unicode()` |
| aritmética de datas | 10 | `d + duration`, `duration + d`, `d - duration`, `d1 - d2`, hora + duração com volta do relógio |
| outros | 7 | `(1pt).mm()` dá `0.35277777777777775` contra `…778`; `(1pt).to-absolute()` e `(1em).to-absolute()` exigem contexto; `selector.and` e `selector.or` mostram `(heading \| figure)` contra `selector.or(figure, heading)`; a saída JSON de `counter.step()`; o texto do erro de modificador de símbolo |

`(1,2,3).range()` e `"a".from-unicode()` também falham no vanilla, mas pela ponte: a função recebe o valor e recusa o tipo (`expected integer, found array`). O cristalino diz que o método não existe.

## Causa

- [call.rs:236](../../01_core/src/compiler/eval/call_dispatch/call.rs) termina a cadeia de métodos em `field_callee_error`, sem consultar o escopo do tipo. O vanilla consulta primeiro (`typst-eval/src/call.rs:247`, `target.ty().scope().get(field)`).
- Os operadores `+` e `-` não têm os pares `datetime`/`duration`. O vanilla os define em `foundations/ops.rs:155-157` e `210-212`.
- `mm` converte por outro caminho numérico, e `to-absolute` pede contexto mesmo quando o comprimento não depende de estilo.

## Escopo proposto

| Parte | Mudança | Efeito |
|---|---|---|
| A | antes do erro de método inexistente, procurar a função no escopo do tipo do valor e chamá-la com o valor como primeiro argumento | 11 casos; vale para todo tipo com função de escopo |
| B | `datetime + duration`, `duration + datetime`, `datetime - duration` e `datetime - datetime` para data, hora e data com hora. Uma data sem hora que recebe horas vira data com hora. Tipos diferentes dão `cannot subtract time from date` | 10 casos |
| C | `mm`, `cm` e `inches` pela mesma conversão do vanilla; `to-absolute` sem contexto quando o comprimento não tem `em` | 3 casos |
| D | `repr` de `selector.and` e `selector.or` na forma da linguagem | 2 casos |

## Compatibilidade e contrato

A e B só aceitam o que hoje é erro. C muda a última casa decimal de `mm` e torna válido um `to-absolute` que hoje erra. D muda o texto de `repr`. Não há tipo, variante, flag, fase nem dependência nova.

## Comportamentos do vanilla a decidir pela prova

| Caso | Vanilla | Tratamento proposto |
|---|---|---|
| `datetime(year: 9999, month: 12, day: 31) + duration(days: 1)` | pânico do processo | **erro comprovado:** o compilador cai sem diagnóstico. Proponho o erro `the result is out of range` na posição da expressão |
| `d + duration(weeks: 99999999)` | pânico | idem |
| `datetime(hour: 23, …) + duration(hours: 2)` | dá `hour: 1`, o relógio volta | seguir: é a regra de hora sem data |
| `(1em).to-absolute()` sem contexto | `11pt`, o tamanho padrão | **não comprovado:** seguir o vanilla |

## Fora

- Decorações e texto (`underline`, `overline`, `strike`, `highlight`, `sub`, `super`, `smallcaps`, `strong.delta`, as propriedades de `text`). É o próximo pacote, quando a alteração de outra autoria em `layout/decorations.rs` for commitada.
- `pad`, `place`, `stack`, `repeat`, `outline`, `raw` e `quote` com `set`, e os conteúdos de célula que ainda divergem.
- Saída JSON de `typst eval` para conteúdo, inclusive `counter.step()`, e o texto do erro de modificador de símbolo.

## Aceitação

1. L0s primeiro, com selos e linhagem limpos.
2. RED no CLI por parte, com valor e erro com coluna.
3. As 50 expressões: as 26 do escopo coincidem; a saída JSON de `counter.step()` e o texto do erro de símbolo ficam fora.
4. Toda função de escopo de tipo do vanilla chamada no valor, pela sonda de tipos: o mesmo resultado ou o mesmo erro.
5. Métodos nativos existentes não mudam: a ponte só age quando o método não existe.
6. Suíte completa, lint e linhagem reversa.

## Decisão pedida

Aprovar A–D. A alternativa é aprovar só A e B, que cobrem 21 dos 26 casos.

## Execução

**25 de novembro de 2026.** HEAD `6b69a9052`, árvore não commitada deste pacote e da alteração de outra autoria em `layout/decorations`. Cristalino `b891d6cf011e61f11eea0799a657ec74aed733ec624a3e9684e1003c18923ea8` (debug), vanilla o mesmo da medição. Conjunto de 50 expressões de 23:08:42 a 23:08:44 de Brasília: 46 coincidem (eram 22). Suíte completa 10.971 aprovados, 0 falhas, 7 ignorados; lint sem violações; linhagem reversa 1.034 consumidores, 0 erros.

**Feito.** A, B e D como propostos. Em A, as mensagens das funções estáticas `range`, `str.from-unicode` e dos stops de gradiente passaram a ser as do vanilla, para que a ponte dê o mesmo erro. Em B, a soma que sai do calendário dá `the result is out of range`, onde o vanilla entra em pânico. Em D, `x.or(y)` guarda o receptor depois dos outros, como o vanilla, e as formas estáticas `selector.or(...)` e `selector.and(...)` aceitam qualquer valor selecionável como receptor.

**Premissa refutada em C.** A proposta supunha que o vanilla aceita `to-absolute` sem contexto quando o comprimento não tem `em`. A medição mostrou que, num documento, o vanilla também exige contexto, até para `1pt`. A diferença está só no comando `typst eval`, que no vanilla conhece os estilos (`text.size` dá `11pt`) e não conhece a posição (`here()` erra). Isso é comportamento da CLI, contrato protegido que a aprovação não cobriu, e não foi alterado. C ficou com a conversão de unidades: `mm`, `cm`, `inches` e os literais passam pela escala interna do vanilla.

**Limites que ficam.**

- `typst eval` sem contexto de estilos: `to-absolute`, `text.size`, `text.lang`. Pede decisão sobre a CLI.
- `(1cm + 1mm).mm()` dá `10.999999999999998`: o vanilla soma unidades internas inteiras e o cristalino soma pontos em ponto flutuante.
- `heading.or(figure)` é aceito aqui e recusado no vanilla; vem do interceptador de seletores anterior a este pacote.
- `selector(<a>)` e `selector("x")` erram aqui.
- A saída JSON de `counter.step()` e o texto do erro de modificador de símbolo.
