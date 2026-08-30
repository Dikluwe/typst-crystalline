# P1216 — fechar diagnósticos de operadores e reavaliar `eval-apply-binary`

**Estado:** EXECUTADO — DIAGNÓSTICOS FECHADOS; LACUNAS SEMÂNTICAS MAPEADAS
**Predecessores causais:** P1212, P1213 e P1215
**Fila:** `operator-diagnostics`, atualmente `DIAGNOSTIC-GAP`
**Relação DSM:** `eval-apply-binary`, atualmente `parcial`

## 1. Objetivo

Fechar as duas divergências textuais nominais encontradas no piloto P1212:

```text
1 in "hello"
vanilla:    cannot apply 'in' to integer and string
cristalino: cannot apply In to integer and string

1pt < 1em
vanilla:    cannot compare 1pt with 1em
cristalino: cannot compare length and length
```

Depois da correção, ampliar a matriz o suficiente para decidir de novo se a
linha `operator-diagnostics` pode virar `RESOLVED`. A relação estrutural
`eval-apply-binary` só pode ser promovida se a responsabilidade funcional do
fragmento declarado estiver exaurida; corrigir dois exemplos não a fecha
automaticamente.

Resultado terminal preferido:

```text
OPERATOR DIAGNOSTICS GREEN — QUEUE RESOLVED
```

Se a auditoria revelar superfície adicional ainda divergente:

```text
OPERATOR DIAGNOSTICS PARTIAL — NEW GAPS MAPPED
```

## 2. Baseline e entradas protegidas

Congelar antes de qualquer patch:

- vanilla ratificado `a51e02804`;
- HEAD, `git diff HEAD --stat` e horário;
- SHA-256 de `/usr/local/bin/typst`, cristalino release, lente e mapa;
- `00_nucleo/diagnosticos/p1212-sondas.tsv`;
- `00_nucleo/diagnosticos/p1212-vereditos.tsv`;
- `00_nucleo/diagnosticos/p1213-fila-implementacao.tsv`;
- relação `eval-apply-binary` do mapa DSM;
- L0s vigentes de `compiler/eval/operators`, `error_formatting`, `ordering`,
  `equality`, `arithmetic` e `compiler/eval/repr`.

Política de `Unknown`: construção que um dos parsers não aceite, valor opaco
ou ramo não alcançado pela linguagem pública permanece `Unknown`; não conta
como GREEN nem ausência.

## 3. Medição antes da decisão

O código vigente já mostra duas causas diferentes:

1. `binary_mismatch` usa `format!("cannot apply {op:?} ...")`, expondo o nome
   Rust `In` em vez do spelling Typst `'in'`;
2. `ordering::value_cmp` reduz incomparabilidade a `None`, apagando a causa.
   O caller então usa nomes de tipo, mas o vanilla distingue:
   - tipos/variantes incompatíveis → `cannot compare {kind} and {kind}`;
   - dois valores da mesma família com ordem parcial indefinida →
     `cannot compare {repr(a)} with {repr(b)}`.

Essas observações orientam as sondas, mas não substituem a medição binária.
Publicar primeiro
`00_nucleo/diagnosticos/p1216-operator-oraculos.tsv` com expressão, exit,
mensagem integral, região, hints/trace e mutante refutado.

## 4. Matriz obrigatória — membership

Medir nos dois binários, no mínimo:

### Casos válidos

- `"e" in "hello"`, `"x" not in "hello"`;
- `2 in (1, 2, 3)`, `4 not in (1, 2, 3)`;
- `"a" in (a: 1)`, `"z" not in (a: 1)`;
- `1 in bytes((0, 1, 2))`, se a linguagem ratificada aceitar;
- membership sobre content/selector ou outros containers somente após fonte
  vanilla confirmar que pertencem ao fragmento.

### Casos inválidos

- `1 in "hello"`;
- `1 not in "hello"`;
- `"a" in 1` e `"a" not in 1`;
- `none in true`;
- pelo menos um par onde lhs/rhs tenham nomes longos diferentes de seus nomes
  curtos (`boolean`, `integer`, `dictionary`, `relative length`).

Congelar separadamente o spelling de `'in'` e `'not in'`, incluindo aspas,
espaços e ordem dos operandos. Não derivar o texto de `Debug`.

## 5. Matriz obrigatória — comparação

Executar cada família com `<`, `<=`, `>` e `>=` quando aplicável.

### Comparáveis

- int/int, int/float e float/int;
- string/string e boolean/boolean;
- angle/angle, ratio/ratio, duration/duration, version/version;
- `1pt` com `2pt`; `1em` com `2em`;
- relative lengths comparáveis segundo os guards vanilla;
- arrays lexicográficos e prefixos de comprimentos diferentes.

### Incomparáveis por tipos/kinds distintos

- `1 < "1"`;
- `ltr < 1`;
- `1pt < 1`;
- datetime de kinds incompatíveis, se construível publicamente;
- arrays cujo primeiro elemento decisivo tenha tipos incompatíveis.

### Incomparáveis dentro da mesma família

- `1pt < 1em` e inverso;
- comprimentos mistos como `1pt + 1em` contra componentes puras;
- relative lengths cuja parte absoluta/relativa impede ordem;
- arrays cujo primeiro par decisivo contém comprimentos incomparáveis;
- float NaN somente se houver construção pública estável e o vanilla o
  classificar nesse caminho.

Para cada erro registrar se o vanilla usa:

```text
cannot compare {repr(a)} with {repr(b)}
cannot compare {kind(a)} and {kind(b)}
```

Não generalizar a partir de `1pt < 1em`: arrays e datetime podem propagar ou
formatar a causa de maneira própria.

## 6. Descoberta adicional

Antes de fechar o inventário, varrer a enum pública de operadores e a fonte
vanilla `foundations/ops.rs` para localizar outros fallbacks que ainda dependam
de `Debug`, nome curto ou perda de `repr`.

Cobrir pelo menos:

- `not in` além de `in`;
- os quatro operadores de ordenação;
- fronteiras de igualdade/atribuição somente se produzem erro público;
- operadores unários, já registrados no L0 como divergência futura.

Se os unários apresentarem RED reproduzível, acrescentá-los ao mapa/fila como
cluster nominal separado, salvo se a mesma correção pura e o mesmo contrato
os fecharem sem ampliar o risco. Não escondê-los para declarar o cluster
binário concluído.

Produzir `00_nucleo/diagnosticos/p1216-operator-descoberta.tsv` com:

```text
operator | source_owner | public_probe | vanilla | crystalline | class | next_action
```

## 7. Gate L0

Auditar e atualizar primeiro somente os owners causais medidos.

Correção de formatação interna e paridade vanilla segue fluxo contínuo
ADR-0127: L0 primeiro, RED→GREEN, resselo e revalidação. Parar para confirmação
se a solução exigir:

- mudança de assinatura pública ou campo de entidade;
- novo comportamento/default fora do diagnóstico;
- mudança de fase eval/layout;
- alteração de representação pública de valores;
- quebra de compatibilidade.

É provável que `error_formatting.md` seja owner do spelling de operadores e
`ordering.md` seja owner da preservação da causa parcial. Confirmar pelo fluxo
real antes de decidir. `repr.md` deve ser consumido, não duplicado, se o
vanilla usar representação de valores.

## 8. Testes RED

Antes da implementação, escrever testes co-localizados que falhem por texto:

- inválido `in` usa exatamente `'in'`;
- inválido `not in` usa exatamente `'not in'`;
- `1pt < 1em` preserva `1pt` e `1em` com `with`;
- tipos distintos preservam nomes longos com `and`;
- ordem dos operandos é preservada;
- os quatro operadores de ordenação compartilham a mesma classificação;
- array propaga a primeira causa incomparável;
- valores comparáveis continuam produzindo booleanos corretos;
- regiões diagnósticas continuam resolvíveis em `eval` e `compile` após P1215.

Confirmar RED com comando, falha esperada e predecessor causal. Teste apenas
de helper de string não substitui a sonda pela linguagem pública.

## 9. Implementação permitida

Aplicar a menor mudança que o contrato congelado legitimar:

- mapear `BinOp` para spellings Typst explícitos, sem `Debug` user-facing;
- preservar no resultado da comparação a distinção entre ordenação e causa de
  incomparabilidade, em vez de colapsar tudo em `Option<Ordering>` se a matriz
  provar essa necessidade;
- reutilizar `eval::repr::repr_value` para mensagens value-sensitive;
- manter `vanilla_type_name` como ponto único de nomes longos;
- propagar erro do primeiro elemento decisivo em arrays, se medido assim.

Não:

- comparar strings de erro para escolher fluxo;
- usar `type_name()` curto onde o vanilla usa kind longo;
- implementar `Display` global de `BinOp` sem auditar todos os consumidores;
- tornar `repr_value` dependente de ordering;
- alterar a verdade/semântica dos operadores para corrigir texto;
- usar panic, registry dinâmico ou estado global;
- corrigir lacunas de SVG, quotes, layout ou PDF neste passo.

## 10. Ataques obrigatórios

As sondas devem rejeitar, no mínimo:

1. manter `In`/`NotIn` via `Debug`;
2. omitir as aspas do spelling;
3. trocar `'not in'` por `'in'`;
4. inverter lhs/rhs;
5. usar sempre nomes de tipo;
6. usar sempre repr de valor;
7. usar `and` onde vanilla usa `with`;
8. usar `with` onde vanilla usa `and`;
9. perder a causa dentro de array lexicográfico;
10. considerar `1pt` e `1em` ordenáveis por coerção inventada;
11. regressar comparações válidas;
12. produzir mensagem correta com span detached/incorreto.

Calcular o score do conjunto válido. Sem motor externo ou isolamento real,
declarar a limitação; `Unknown` explícito não entra no denominador.

## 11. Verificação A/B

Produzir `00_nucleo/diagnosticos/p1216-operator-resultados.tsv` e repetir duas
vezes:

- toda a matriz P1212 de 21 sondas;
- a matriz ampliada deste passo;
- controles de valores, short-circuit e spans;
- `typst eval` e ficheiros `.typ` compilados para os erros principais.

Critério:

```text
valor/exit + mensagem integral + região + hints/trace aplicáveis
```

O primeiro ensaio de P1212 foi inválido por assimetria de flags; os comandos
deste passo devem ser byte-simétricos entre os binários.

## 12. Mapa e fila

Se todos os casos do cluster binário passarem, mudar:

```text
operator-diagnostics: DIAGNOSTIC-GAP → RESOLVED
```

Adicionar oráculos, resultados, ataques e laudo como evidências do mapa.

Para `eval-apply-binary`:

- promover a `declarada-fechada` somente se a auditoria confirmar que os cinco
  destinos mapeados cobrem toda a responsabilidade declarada e a matriz cobre
  os ramos L0 relevantes;
- caso contrário, manter `parcial` e substituir a nota genérica por uma lista
  nominal dos ramos ainda não medidos;
- qualquer nova divergência vira linha própria na fila, não nota perdida.

## 13. Gates finais

Executar, no mínimo:

```text
cargo test -p typst-core <testes focais de operadores>
cargo test --workspace
cargo build --workspace --quiet
cargo build --release --bin typst
crystalline-lint --fix-hashes .
crystalline-lint .
cargo fmt --all -- --check
git diff --check
lente --comparar ... --mapa-correspondencia ...
```

Executar matriz e lente duas vezes e registrar hashes. Falha temporal do teste
de watch deve ser preservada e repetida isoladamente, seguindo os recibos
P1214/P1215.

## 14. Separação de autoridades

Usar o protocolo completo de materialização segregada:

- A congela contrato/oráculos sem ler o patch;
- B cria mutantes de spelling, classificação e propagação;
- C implementa contra entradas seladas;
- D executa A/B sem promover mapa;
- E verifica, classifica novos achados e decide promoção.

Se uma única sessão tiver acesso a tudo, declarar
`EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO`.

## 15. Continuação se o cluster não puder ser fechado

Se a fonte ou as sondas refutarem o escopo acima, não improvisar uma correção
larga. Encerrar P1216 como descoberta reproduzível, atualizar a fila e escrever
o passo seguinte para o primeiro gap nominal confirmado.

Se o cluster fechar, o próximo item da fila P1213 é `svg-morphology`. Esse
passo deverá localizar a primeira subárvore semântica divergente antes de
alterar o exporter.
