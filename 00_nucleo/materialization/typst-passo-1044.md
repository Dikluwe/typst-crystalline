# Passo 1044 — V19: corrigir a contagem AST vanilla-vs-cristalino (or-patterns + derives)

**Tipo**: Duas correcções de método na mesma medição, porque as duas afectam o mesmo
número final (a equivalência de 99,5% usada para "validar" `ADR-0109`, nunca fechada com
confiança):
1. **Or-patterns** (`A | B | C => foo()`) contam como 1 braço de `match`, mas representam
   N casos reais — subestimação não-uniforme entre os dois lados (265 ocorrências, V19).
2. **Derives triviais** (`#[derive(Debug, Clone, PartialEq, Hash, Serialize)]`) geram
   `match`/`if` por variante de enum em código pós-expansão — boilerplate nunca fonte de
   bug real, que pode estar a inflacionar a contagem do vanilla sem equivalente
   proporcional no cristalino (pergunta levantada e nunca respondida, de duas
   conversas atrás).
**Objectivo**: um número de equivalência fiável, não os 99,5% brutos já descartados como
insuficientes.
**Pré-condição**: `git status` limpo. HEAD ≥ Passo 1043.

---

## Fase A — V19: expandir or-patterns para contagem real

```
crystalline-lint --checks v19 . > /tmp/v19-lista.txt
```

Para cada uma das 265 ocorrências: contar quantas alternativas o or-pattern condensa
(`A | B | C` = 3). Produzir dois números por ficheiro/módulo: **contagem de braços**
(o que a AST bruta mede) e **contagem de casos reais** (braços, com or-patterns expandidos
ao número de alternativas). A diferença entre os dois é o "factor de subestimação N×" que
o V19 já sinalizava.

**Aplicar a mesma expansão aos dois lados** — vanilla pós-expansão de macros e cristalino
— antes de comparar. Se um lado usa mais or-patterns do que o outro (hipótese já
levantada), a contagem bruta favorece esse lado artificialmente; a contagem expandida
corrige isso.

## Fase B — Separar derives triviais de macros de domínio

Para o vanilla pós-expansão: identificar quais `match`/`if` vêm de `#[derive(...)]`
(Debug/Clone/PartialEq/Hash/Serialize/etc., aplicados a enums grandes, geram
`match self { A => ..., B => ..., }` por variante, sem lógica de domínio) vs quais vêm das
macros de domínio (`#[elem]`/`#[func]`/`#[ty]`, que geram despacho real).

```
cargo expand -p typst-library 2>/dev/null | grep -B5 'match self' | grep -c 'derive'
```
(ajustar comando conforme necessário — o objectivo é uma contagem separada, não um número
único a coincidir por sorte).

Para o cristalino: confirmar se há equivalente de derive-boilerplate a descontar do lado
dele também, antes de comparar só a fatia "lógica de domínio" dos dois lados.

## Fase C — Recalcular a equivalência

Com os dois ajustes aplicados (or-patterns expandidos + derives triviais descontados),
recalcular a proporção `01_core` (cristalino) vs `typst-library` (vanilla) que motivou a
alegação de 99,5%. Reportar o número novo, e se a conclusão da comparação original
("o cristalino trouxe à luz exactamente o que as macros geravam") continua sustentada ou
precisa de ser revista.

**Não presumir que o número novo vai confirmar ou contradizer o anterior** — reportar o
que sair, mesmo que seja inconclusivo ou que exija mais uma correcção de método ainda não
identificada.

## Fase D — Fechar ou reabrir a questão de `ADR-0109`

Com o número recalculado: se a equivalência se mantiver próxima (ordem de grandeza
semelhante à original), registar isto como confirmação com método corrigido — a alegação
original sobrevive, agora com base sólida. Se divergir de forma relevante, registar como
achado a reportar ao dono, sem decidir sozinho o que isso implica para `ADR-0109` (decisão
de ADR fica fora de âmbito desta frente, per acordo já estabelecido).

---

## O que este passo NÃO faz

- Não altera nenhum código de produção — é medição e reconciliação de método.
- Não decide o conteúdo do `ADR-0109` — só fornece o número correcto para essa decisão,
  que fica para depois (ADRs fora de âmbito até a paridade fechar, per decisão anterior).

## Resultado esperado

Número de equivalência AST recalculado, com or-patterns e derives tratados
correctamente nos dois lados — fecha (ou reabre com base sólida) uma questão que ficou
pendente há várias dezenas de passos.
