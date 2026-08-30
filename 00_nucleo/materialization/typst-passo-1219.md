# P1219 — fechar aritmética `Decimal ↔ Int` e readjudicar `eval-apply-binary`

**Estado:** EXECUTADO — DECIMAL↔INT GREEN; `eval-apply-binary` READJUDICADO  
**Predecessor causal:** P1218  
**Cluster:** `decimal-int-arithmetic`, atualmente `MISSING`  
**Relação DSM:** `eval-apply-binary`, atualmente `parcial`

## 1. Objetivo

Medir e fechar as oito operações direcionais que o vanilla aceita e o
cristalino ainda rejeita:

```text
Decimal + Int    Int + Decimal
Decimal - Int    Int - Decimal
Decimal * Int    Int * Decimal
Decimal / Int    Int / Decimal
```

O resultado deve permanecer `Decimal`, convertendo `Int` exatamente para
Decimal antes da operação. Depois do fechamento, repetir a auditoria nominal
de P1218 e readjudicar `eval-apply-binary`. A promoção só é permitida se não
restar outro ramo público funcional ou diagnóstico no fragmento declarado.

Resultado preferido:

```text
DECIMAL ↔ INT ARITHMETIC GREEN — APPLY-BINARY READJUDICATED
```

## 2. Baseline e entradas protegidas

Congelar antes de qualquer alteração:

- vanilla ratificado `a51e02804` e SHA-256 do binário;
- HEAD, horário e `git diff HEAD --stat`/`git status --short` completos;
- SHA-256 do binário cristalino, lente e mapa DSM;
- L0 `compiler/eval/operators/arithmetic.md` e respetivo consumer;
- `entities/decimal.md`, `Decimal::from_i64` e a versão de `rust_decimal`;
- fonte vanilla `typst-library/src/foundations/ops.rs`, em especial os braços
  Decimal↔Int de `add`, `sub`, `mul` e `div`;
- artefatos P1216–P1218, fila P1213 e correspondência
  `eval-apply-binary`.

Não usar números de outra working tree para fechar o passo. Toda contagem
decisória deve registrar commit ou working tree não commitado e lista exata
dos ficheiros alterados.

## 3. Medição antes da decisão

Produzir antes da implementação:

```text
00_nucleo/diagnosticos/p1219-decimal-int-oraculos.tsv
```

Colunas:

```text
id | expression | direction | operator | vanilla_exit |
vanilla_observable | source_file_line | mutant_rejected
```

Medir diretamente no vanilla, sem inferir de `rust_decimal`. Para cada uma
das oito direções, congelar valor/repr, tipo público, exit, mensagem, região,
hints e trace quando existirem.

## 4. Matriz mínima obrigatória

Para cada direção e operador, cobrir:

- inteiros positivos, negativos e zero;
- Decimal inteiro, fracionário, negativo e zero;
- ordem não comutativa em `-` e `/`;
- `i64::MIN` e `i64::MAX` quando construíveis pela linguagem;
- Decimal no limite de precisão/range aceito pelo constructor;
- trailing zeros e escala (`decimal("1.00")`);
- resultado exato que denunciaria coerção via `f64`, por exemplo inteiro
  acima de `2^53`;
- expressão aninhada e uso dentro de array/chamada para precedência;
- divisão por `Int(0)`, `Decimal(0)` e zeros com escala;
- overflow/underflow de `+`, `-`, `*` e `/`.

Usar `repr(...)` onde a serialização JSON do CLI não representa diretamente
o tipo. Não transformar limitação do serializer em gap do operador.

## 5. Semântica a confirmar

A fonte vanilla vigente indica os seguintes candidatos, que só se tornam
contrato após as sondas:

```text
Decimal op Int => Decimal op Decimal::from(Int)
Int op Decimal => Decimal::from(Int) op Decimal
```

Confirmar também:

- conversão `Int → Decimal` exata, nunca por `f64`;
- preservação da direção em subtração e divisão;
- divisão por zero interceptada antes do cálculo;
- mensagem vanilla de overflow/underflow, sem panic nem texto nativo da
  biblioteca escapar;
- escala/repr observável do resultado, sem exigir igualdade mecânica da
  representação Rust quando a linguagem não a observa.

## 6. Gate L0

Atualizar primeiro
`00_nucleo/prompts/compiler/eval/operators/arithmetic.md`:

- remover o scope-out explícito de coerção cruzada Decimal;
- adicionar as oito combinações à tabela normativa;
- atualizar critérios de verificação, divisão por zero e overflow conforme
  os oráculos medidos;
- manter explícitas as combinações realmente fora de escopo.

É correção interna de paridade e segue fluxo contínuo ADR-0127: L0 primeiro,
resselo e RED→GREEN sem paragem. Parar se a medição exigir contrato público,
novo tipo, sintaxe, comportamento default, mudança de fase ou quebra de
compatibilidade.

## 7. Testes RED

Antes da implementação, adicionar testes no consumer público e confirmar RED
para:

1. as oito direções básicas;
2. subtração e divisão com operandos trocados produzindo valores distintos;
3. inteiro acima de `2^53` convertido exatamente;
4. sinais mistos e zero;
5. divisor `Int(0)` e `Decimal(0)` com mensagem verbatim;
6. limites e overflow medidos;
7. resultado público sempre Decimal;
8. expressão real pelo parser/eval, não apenas chamada direta do helper;
9. P1216, P1217 e P1218 continuam GREEN.

Registrar o comando e resumo do RED. Um teste que já passe não prova o gap e
deve ser reclassificado como controle.

## 8. Implementação permitida

Aplicar a menor mudança legitimada no owner aritmético:

- converter `i64` com `Decimal::from_i64`;
- executar na mesma direção do AST;
- retornar `Value::Decimal`;
- usar operações checked se o vanilla medido as usa;
- traduzir falha para a mensagem pública medida;
- reutilizar o gate existente de divisão por zero.

Preferir um helper privado apenas se reduzir duplicação sem esconder direção,
operador ou tratamento de erro.

Não:

- converter por `as f64`/`Decimal::from_f64`;
- adicionar coerção Decimal↔Float;
- alterar igualdade ou ordering já fechados em P1217;
- alterar parser, precedência ou short-circuit;
- generalizar para uma torre numérica nova;
- expor erro/panic de `rust_decimal` ao utilizador;
- tocar em gaps posteriores da fila;
- promover DSM por contagem mecânica.

## 9. Ataques obrigatórios

Produzir:

```text
00_nucleo/diagnosticos/p1219-decimal-int-ataques.tsv
```

Rejeitar, no mínimo:

1. implementar somente `Decimal op Int`;
2. implementar somente `Int op Decimal`;
3. esquecer um dos quatro operadores;
4. inverter operandos de subtração;
5. inverter operandos de divisão;
6. converter o Int via `f64`;
7. devolver Float;
8. truncar Decimal para Int;
9. perder trailing zeros onde forem observáveis;
10. deixar divisão por `Int(0)` chegar à biblioteca;
11. deixar divisão por `Decimal(0.00)` escapar do gate;
12. panic em overflow;
13. emitir mensagem mecânica de `rust_decimal`;
14. regressar Decimal↔Int ordering/equality;
15. alterar Decimal↔Float;
16. promover o mapa sem reauditar 19/19 `BinOp`.

Exigir `mutation_score = 1.0` para todos os mutantes válidos.

## 10. A/B final

Produzir duas rodadas independentes em:

```text
00_nucleo/diagnosticos/p1219-decimal-int-resultados.tsv
```

Executar:

- matriz completa deste passo;
- oito casos focais mínimos;
- 58 casos P1217;
- 16 casos unários P1218;
- controles de equality, ordering, assignment, membership e short-circuit;
- pelo menos oito ficheiros `.typ`: quatro positivos, um por operador, e
  quatro erros/limites, comparando mensagem e região.

Comparar `exit + valor/repr + mensagem + região + hints/trace`. Diferença de
apresentação do path deve ser classificada separadamente, nunca ocultada nem
confundida com span.

## 11. Reauditoria binária

Regenerar ou atualizar:

```text
00_nucleo/diagnosticos/p1219-binop-readjudicacao.tsv
```

Uma linha por cada uma das 19 variantes de `BinOp`, preservando as colunas de
P1218:

```text
variant | spelling | vanilla_owner | crystalline_owner | public_probe |
value_parity | diagnostic_parity | evidence | remaining_gap
```

Não copiar estados de P1218 sem nova execução. Assign e assigns compostos
continuam classificados pelo seu owner real, sem forçar equivalência
mecânica com o dispatcher puro.

## 12. Mapa e fila

Se a matriz fechar:

```text
decimal-int-arithmetic: MISSING → RESOLVED
```

Adicionar oráculos, resultados, ataques, readjudicação e laudo ao mapa DSM.

Promover `eval-apply-binary` de `parcial` para `declarada-fechada` somente se:

- 19/19 variantes tiverem owner e probe atual;
- nenhum `remaining_gap` funcional/diagnóstico permanecer no fragmento;
- as duas rodadas A/B forem estáveis;
- a alegação registrar explicitamente o limite: semântica pública dos
  operadores, não igualdade mecânica da implementação Rust.

Se surgir qualquer novo RED, abrir cluster nominal na fila antes de
`svg-morphology`, manter `parcial` e escrever na nota exatamente o que falta.

## 13. Laudo

Produzir:

```text
00_nucleo/diagnosticos/typst-p1219-decimal-int-arithmetic.md
```

O laudo deve conter medição antes da decisão, proveniência, RED→GREEN,
matrizes, mutantes, divergências classificadas, veredito DSM e próximo gap.

## 14. Gates finais

```text
cargo test -p typst-core p1219
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

Repetir matriz e lente duas vezes, registrando SHA-256 das entradas e saídas
decisórias, HEAD, working tree e horário.

## 15. Separação de autoridades

Usar protocolo Tekt completo:

- A congela obrigação, fonte e oráculos;
- B produz mutantes sem ver a implementação final;
- C implementa contra entradas seladas;
- D executa A/B sem editar mapa ou veredito;
- E verifica evidência e decide a promoção.

Numa única sessão, declarar:

```text
EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO
```

## 16. Continuação

Se `decimal-int-arithmetic` fechar e `eval-apply-binary` puder ser promovido,
o próximo cluster da fila é `svg-morphology`. Se a readjudicação revelar novo
buraco, o sucessor deve tratar o primeiro gap nominal antes de avançar para
SVG.
