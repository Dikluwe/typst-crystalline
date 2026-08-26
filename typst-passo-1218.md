# P1218 — fechar operadores unários e auditar exaustão de `eval-apply-binary`

**Estado:** EXECUTADO — UNÁRIOS GREEN; AUDITORIA BINÁRIA ABRIU `decimal-int-arithmetic`  
**Predecessores causais:** P1216 e P1217  
**Cluster:** `unary-operators`, atualmente `Unknown` nominal no mapa  
**Relação DSM:** `eval-apply-binary`, atualmente `parcial`

## 1. Objetivo

Medir e fechar a superfície pública dos três operadores unários Typst:

```text
+value
-value
not value
```

O código vigente já indica dois candidatos concretos:

1. `Pos` aceita apenas `Int`, `Float` e `Length`, enquanto o vanilla também
   aceita `Decimal`, `Angle`, `Ratio`, `Relative` e `Fraction`;
2. a fronteira usa nomes Rust (`Pos`, `Neg`, `Not`) e nomes curtos de tipo,
   enquanto o vanilla usa spellings como `unary '+'`, `'-'` e `'not'` e a
   representação pública do operando.

Depois, auditar nominalmente toda a enum `BinOp` e a tabela vanilla. A relação
`eval-apply-binary` só pode ser promovida se nenhum ramo binário funcional
continuar sem medição. Divergências adicionais tornam-se linhas próprias na
fila; não são escondidas para fechar o mapa.

Resultado preferido:

```text
UNARY OPERATORS GREEN — BINARY AUDIT EXHAUSTIVE
```

## 2. Baseline e entradas protegidas

Congelar antes do patch:

- vanilla ratificado `a51e02804`;
- HEAD, horário e `git diff HEAD --stat`;
- SHA-256 dos dois binários, lente e mapa;
- artefatos P1216 e P1217;
- relação `eval-apply-binary` e fila P1213;
- L0s `operators.md`, `arithmetic.md`, `error_formatting.md`, `eval.md` e
  owners de `UnOp`/`BinOp`;
- enums públicas `UnOp` e `BinOp` e fonte vanilla `foundations/ops.rs`.

Política de `Unknown`: expressão rejeitada pelo parser antes do dispatcher,
valor opaco ou ramo não construível publicamente permanece `Unknown` e não
conta como GREEN.

## 3. Oráculos antes da implementação

Produzir:

```text
00_nucleo/diagnosticos/p1218-unary-oraculos.tsv
```

Colunas:

```text
id | expression | vanilla_exit | vanilla_observable | class |
source_file_line | mutant_rejected
```

Medir texto integral, exit, região, hints e trace. Não derivar mensagens de
`Debug`, `type_name()` ou suposições sobre a enum.

## 4. Matriz `Pos`

Executar `+` sobre:

- `Int`, `Float`, `Decimal`;
- `Length`, `Angle`, `Ratio`, `Relative`, `Fraction`;
- `Bool`, `Str`, `Bytes`, `Content`, `Array`, `Dict`, `Datetime`;
- pelo menos um tipo dinâmico/estrutural publicamente construível;
- limites numéricos e zero;
- cada valor válido dentro de array ou chamada, para confirmar precedência.

Congelar a distinção vanilla entre:

```text
cannot apply unary '+' to {value}
cannot apply '+' to {value}
```

se ambas forem publicamente alcançáveis. Não uniformizar mensagens distintas.

## 5. Matriz `Neg`

Executar `-` sobre:

- `Int`, incluindo overflow e `int.min`;
- `Float`, `Decimal`, `Length`, `Relative`, `Angle`, `Ratio`, `Fraction` e
  `Duration`;
- `Datetime`, cujo vanilla usa `cannot apply unary '-' ...`;
- `Bool`, `Str`, `Bytes`, `Array`, `Dict` e Content;
- valores zero e negativos construíveis sem ambiguidade sintática.

Preservar os hints especiais do parser para o mínimo inteiro. Não confundir
erro de literal com a semântica de `eval_unary_op`.

## 6. Matriz `Not`

Executar `not` sobre:

- `true` e `false`;
- todos os tipos primitivos não booleanos;
- Array, Dict, Content, Datetime e funções construíveis;
- expressão aninhada e combinação com `and`/`or`, verificando precedência e
  short-circuit.

O erro esperado deve usar exatamente `'not'` e a representação/nome que o
vanilla medir; não o variant Rust `Not`.

## 7. Auditoria binária exaustiva

Construir uma tabela regenerável:

```text
00_nucleo/diagnosticos/p1218-binop-auditoria.tsv
```

Uma linha por variante pública de `BinOp`:

```text
variant | spelling | vanilla_owner | crystalline_owner | public_probe |
value_parity | diagnostic_parity | evidence | remaining_gap
```

Cobrir `Add`, `Sub`, `Mul`, `Div`, `And`, `Or`, `Eq`, `Neq`, `Lt`, `Leq`,
`Gt`, `Geq`, `In`, `NotIn`, `Assign`, `AddAssign`, `SubAssign`, `MulAssign` e
`DivAssign`.

Separar atribuições do dispatcher puro: confirmar ownership e evidência sem
forçar correspondência mecânica inexistente. Auditar também divergências já
registradas no L0 — por exemplo coerções binárias de Decimal — e abrir linha
de fila se ainda forem públicas e RED. Uma nota histórica de scope-out não
substitui nova medição.

## 8. Gate L0

Atualizar primeiro `arithmetic.md` para a matriz positiva/negativa medida e
`error_formatting.md` para a fronteira unária. Atualizar o hub ou `eval.md`
somente se a auditoria provar ownership incompleto.

Fluxo contínuo ADR-0127 enquanto forem correções internas de paridade. Parar
se exigir contrato público, campo de entidade, sintaxe, comportamento default,
mudança de fase ou incompatibilidade.

## 9. Testes RED

Antes da implementação, confirmar RED para:

- `Pos` nos cinco tipos válidos atualmente ausentes;
- mensagens exatas de `Pos`, `Neg` e `Not` inválidos;
- distinção `unary '+'`/`'+'` e `unary '-'`/`'-'`, se medida;
- nomes longos ou repr do operando conforme vanilla;
- ordem e span da expressão completa;
- overflow de `Neg Int` preservado;
- todos os casos válidos existentes continuam corretos;
- P1216 e P1217 continuam verdes.

Adicionar sondas pela linguagem pública; testes de helper não bastam.

## 10. Implementação permitida

Aplicar a menor mudança legitimada:

- completar `Pos` com identidades vanilla medidas;
- mapear `UnOp` para spelling explícito, nunca `Debug` user-facing;
- reutilizar `repr_value` e `vanilla_type_name` conforme cada formato medido;
- criar helper privado de formatação unária se evitar duplicação sem alterar
  contratos públicos;
- preservar checked negation e mensagens especiais.

Não:

- aceitar novos tipos sem fonte/probe;
- implementar truthiness para `not`;
- converter valores em números;
- alterar precedência ou parser;
- tocar nos novos gaps binários encontrados pela auditoria, salvo se o L0 e o
  contrato do próprio P1218 os cobrirem explicitamente sem ampliar risco;
- promover o mapa por contagem ou narrativa.

## 11. Ataques obrigatórios

Rejeitar, no mínimo:

1. manter `Pos` incompleto;
2. expor `Pos`, `Neg` ou `Not` via Debug;
3. omitir aspas do operador;
4. omitir `unary` onde vanilla exige;
5. adicionar `unary` onde vanilla não usa;
6. usar sempre nome de tipo em vez de repr;
7. usar sempre repr onde vanilla distingue tipo dinâmico;
8. aceitar `not 1` por truthiness;
9. perder checked overflow de inteiro;
10. regressar neg de Decimal/Length/Duration;
11. regressar precedência com `and`/`or`;
12. deslocar ou destacar span incorreto;
13. converter `Unknown` em sucesso;
14. declarar auditoria binária exaustiva omitindo uma variante da enum.

Exigir `mutation_score = 1.0` para mutantes válidos.

## 12. A/B e spans

Produzir duas rodadas de:

```text
00_nucleo/diagnosticos/p1218-unary-resultados.tsv
```

Executar:

- matriz unária completa;
- 58 sondas finais P1217;
- controles de atribuição, short-circuit e membership;
- pelo menos seis ficheiros `.typ`: três positivos e três erros, cobrindo os
  três operadores e spans.

Comparar `exit + valor/mensagem + região + hints/trace` e registrar hashes.

## 13. Mapa e fila

Se a matriz unária fechar, criar/promover:

```text
unary-operators: UNKNOWN → RESOLVED
```

Adicionar oráculos, auditoria binária, resultados, ataques e laudo ao mapa.

Para `eval-apply-binary`:

- promover somente se a tabela tiver todas as variantes, owners e evidências,
  sem gap funcional ou diagnóstico aberto no fragmento declarado;
- caso contrário manter `parcial`, mas substituir `unários Unknown` pelos gaps
  nominais recém-medidos;
- qualquer RED vira linha própria na fila antes de `svg-morphology` se for
  superfície central do eval.

## 14. Gates finais

```text
cargo test -p typst-core p1218
cargo test -p typst-core p1217
cargo test -p typst-core p1216
cargo test --workspace
cargo build --workspace --quiet
cargo build --release --bin typst
crystalline-lint --fix-hashes .
crystalline-lint .
cargo fmt --all -- --check
git diff --check
lente --comparar --antes lab/typst-original --depois . \
  --mapa-correspondencia \
  00_nucleo/diagnosticos/dsm/typst-correspondencias-v1.toml
```

Repetir matriz e lente duas vezes, registrando HEAD, working tree, horário e
SHA-256 de entradas e saídas decisórias.

## 15. Separação de autoridades

Usar protocolo completo:

- A congela contrato/oráculos;
- B produz mutantes;
- C implementa contra entradas seladas;
- D executa A/B sem editar mapa;
- E verifica e decide promoção.

Numa única sessão, declarar `EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO`.

## 16. Continuação

Se unários e auditoria binária fecharem, o próximo cluster é
`svg-morphology`. Se a auditoria revelar coerções ou braços ainda ausentes,
escrever o sucessor para o primeiro gap nominal e manter a relação parcial.
