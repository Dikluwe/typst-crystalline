# P1217 — fechar `ordering-comparables` e reavaliar `eval-apply-binary`

**Estado:** EXECUTADO — `ordering-comparables` FECHADO  
**Predecessor causal:** P1216  
**Fila:** `ordering-comparables`, atualmente `MISSING`  
**Relação DSM:** `eval-apply-binary`, atualmente `parcial`

## 1. Objetivo

Fechar os quatro ramos públicos de ordenação que o P1216 mediu como presentes
no vanilla ratificado e ausentes no cristalino:

```text
1fr < 2fr
vanilla:    true
cristalino: cannot compare fraction and fraction

decimal("1.0") < 2
vanilla:    true
cristalino: cannot compare decimal and integer

10% < (20% + 0pt)
vanilla:    true
cristalino: cannot compare ratio and relative length

datetime(year: 2020, month: 1, day: 1)
  < datetime(year: 2021, month: 1, day: 1)
vanilla:    true
cristalino: cannot compare datetime and datetime
```

O passo fecha somente semântica de ordenação já existente na linguagem. Não
altera sintaxe, contrato público, representação de `Value`, default de produto
ou fase do pipeline.

Resultado terminal preferido:

```text
ORDERING COMPARABLES GREEN — QUEUE RESOLVED
```

`eval-apply-binary` só pode ser promovida depois de nova auditoria nominal dos
operadores; os quatro GREENs não autorizam fechamento automático da relação.

## 2. Baseline e entradas protegidas

Congelar antes do patch:

- vanilla ratificado `a51e02804`;
- HEAD, horário e `git diff HEAD --stat`;
- SHA-256 dos dois binários, lente e mapa DSM;
- `p1216-operator-oraculos.tsv`, `p1216-operator-resultados.tsv`,
  `p1216-operator-descoberta.tsv` e `p1216-operator-ataques.tsv`;
- linha `ordering-comparables` da fila P1213;
- relação `eval-apply-binary` do mapa;
- L0s vigentes `operators.md`, `ordering.md`, `error_formatting.md`,
  `repr.md`, `entities/value.md` e os owners dos quatro tipos.

Política de `Unknown`: constructor público não suportado, combinação de kinds
de datetime não construível ou valor opaco permanece `Unknown`. Não conta como
GREEN nem ausência de divergência.

## 3. Medição antes da decisão

Publicar primeiro:

```text
00_nucleo/diagnosticos/p1217-ordering-oraculos.tsv
```

Colunas mínimas:

```text
id | expression | vanilla_exit | vanilla_observable | class |
source_file_line | mutant_rejected
```

Confirmar na fonte vanilla, com `file:line`, os braços de `compare` para:

- `Fraction ↔ Fraction`;
- `Decimal ↔ Int` nas duas direções;
- `Ratio ↔ Relative` nas duas direções, guardado por parte absoluta zero;
- `Datetime ↔ Datetime`, incluindo `try_cmp_datetimes` e kinds.

Não inferir coerções a partir dos quatro exemplos. Medir cada fronteira antes
de escrever a tabela L0 definitiva.

## 4. Matriz obrigatória — `Fraction`

Executar `<`, `<=`, `>` e `>=` com:

- `1fr` e `2fr`;
- valores iguais;
- zero e negativo, se a sintaxe pública os aceitar;
- ordem inversa;
- `Fraction` contra `Int`, `Float`, `Length` e `Ratio`.

Critério provável, a confirmar: somente `Fraction ↔ Fraction` é comparável e
usa a ordem numérica de sua representação. Pares com outros tipos continuam
no erro `cannot compare {kind} and {kind}`.

## 5. Matriz obrigatória — `Decimal ↔ Int`

Medir nos dois sentidos e com os quatro operadores:

- `decimal("1.0") < 2`;
- `2 > decimal("1.0")`;
- igualdade de magnitude sob `<`, `<=`, `>` e `>=`;
- inteiros negativos e zero;
- limites de `i64` aceitos pelo parser;
- decimal fracionário contra inteiro;
- `Decimal ↔ Float`, para confirmar se continua incomparável;
- arrays cujo primeiro elemento decisivo seja `Decimal ↔ Int`.

A conversão deve ser decimal exata. É proibido passar por `f64` ou introduzir
tolerância não existente no vanilla.

## 6. Matriz obrigatória — `Ratio ↔ Relative`

Medir os dois sentidos e quatro operadores:

- `10% < (20% + 0pt)`;
- `(20% + 0pt) > 10%`;
- valores iguais;
- zero e negativos;
- relative puro produzido por caminhos sintáticos distintos;
- relative com parte absoluta não zero, por exemplo `20% + 1pt`;
- `Ratio ↔ Length` como controle incompatível;
- arrays com `Ratio ↔ Relative` como primeiro par decisivo.

O guard vanilla provável é:

```text
Ratio ↔ Relative é comparável somente quando Relative.abs == 0
```

Confirmar se a parte relativa é comparada diretamente e se o relative com
`0pt` é normalizado antes do dispatcher. Um guard rejeitado deve continuar
emitindo o diagnóstico correto; não pode ser forçado a uma ordem inventada.

## 7. Matriz obrigatória — `Datetime`

Separar os kinds públicos antes de decidir:

- data completa contra data completa;
- hora completa contra hora completa, incluindo hour/minute/second;
- datetime completa contra datetime completa, se o constructor aceitar;
- datas iguais e ordem inversa;
- limites válidos de calendário;
- date contra time;
- date contra datetime;
- time contra datetime;
- arrays cujo primeiro elemento decisivo seja datetime;
- datetime contra outro tipo.

Para kinds homogêneos, medir valores e os quatro operadores. Para kinds
incompatíveis, congelar a mensagem integral do vanilla. A fonte ratificada usa
tratamento específico de datetime; não substituir por `repr_value` nem pela
mensagem genérica sem medir.

Erros do próprio constructor, como `time is incomplete`, não são evidência do
dispatcher de ordenação. Toda sonda de comparação deve primeiro construir
dois valores válidos.

## 8. Gate L0

Atualizar primeiro `00_nucleo/prompts/compiler/eval/operators/ordering.md`
com a tabela medida. Atualizar `error_formatting.md` somente se a mensagem de
kinds incompatíveis de datetime exigir um helper compartilhado.

Fluxo contínuo ADR-0127 é aplicável enquanto a mudança permanecer fórmula
interna de paridade. Parar para confirmação se surgir necessidade de:

- mudar `Value`, `Datetime` ou outra entidade pública;
- adicionar método ou campo público;
- mudar parser/sintaxe;
- alterar comportamento default;
- mover responsabilidade entre eval e layout;
- quebrar compatibilidade.

## 9. Testes RED

Antes da implementação, escrever testes co-localizados em `ordering.rs` e
confirmar RED para:

- quatro operadores de `Fraction`;
- `Decimal ↔ Int` nos dois sentidos;
- conversão decimal exata, inclusive valor fracionário;
- `Ratio ↔ Relative` nos dois sentidos com `abs == 0`;
- rejeição de relative com parte absoluta não zero;
- cada kind homogêneo de datetime construível;
- mensagem exata entre kinds incompatíveis;
- propagação de cada nova combinação dentro de array;
- controles incompatíveis permanecem erro;
- toda a matriz P1216 continua verde.

Adicionar pelo menos quatro sondas pela linguagem pública. Testes apenas de
helpers internos não fecham o contrato.

## 10. Implementação permitida

Aplicar a menor mudança legitimada pela matriz:

- `Fraction ↔ Fraction`: comparação numérica conforme o tipo vigente;
- `Decimal ↔ Int`: converter `Int` para `Decimal` de forma exata;
- `Ratio ↔ Relative`: comparar componentes relativas somente sob o guard
  vanilla confirmado;
- `Datetime ↔ Datetime`: reutilizar a ordem parcial/kind do tipo existente ou
  introduzir helper privado no owner de ordering;
- propagar `CompareFailure` por arrays sem reformatar a causa;
- reutilizar `vanilla_type_name` e `repr_value` conforme o contrato P1216.

Não:

- converter decimal para float;
- ordenar types diferentes por discriminante;
- comparar datetime por `repr` textual;
- comparar datas por string formatada;
- remover guards de relative;
- tornar todos os values comparáveis;
- alterar igualdade `==`;
- tocar SVG, quotes, geometry, PDF ou YAML.

## 11. Ataques obrigatórios

As sondas devem rejeitar, no mínimo:

1. manter `Fraction` incomparável;
2. permitir `Fraction ↔ Int` sem fonte vanilla;
3. converter `Decimal` por `f64`;
4. implementar somente `Decimal < Int`, perdendo a direção inversa;
5. tratar igualdade incorretamente em `<=`/`>=`;
6. aceitar `Ratio ↔ Relative` com parte absoluta não zero;
7. inverter ratio e relative;
8. ordenar datetime por texto;
9. misturar date e time como se fossem o mesmo kind;
10. usar mensagem genérica onde vanilla nomeia kinds;
11. perder a causa dentro de array;
12. regressar os diagnósticos P1216;
13. regressar comparações antigas de int/float/string/length;
14. produzir GREEN por converter `Unknown` implicitamente.

Exigir `mutation_score = 1.0` para mutantes válidos. Sem motor externo ou
isolamento real, declarar a limitação.

## 12. Verificação A/B

Produzir e repetir duas vezes:

```text
00_nucleo/diagnosticos/p1217-ordering-resultados.tsv
```

Executar:

- as 24 sondas finais do P1216;
- toda a matriz ampliada deste passo;
- pelo menos quatro sondas em ficheiros `.typ` para spans;
- controles de short-circuit, equality e mensagens de membership.

Comparar:

```text
exit + valor/mensagem integral + região + hints/trace aplicáveis
```

Os comandos devem ser simétricos entre os binários. Registrar hashes das duas
rodadas e preservar qualquer diferença fora do escopo como achado nominal.

## 13. Mapa e fila

Se toda a matriz declarada passar:

```text
ordering-comparables: MISSING → RESOLVED
```

Adicionar oráculos, resultados, ataques e laudo à relação
`eval-apply-binary`.

Depois auditar a enum pública de operadores e os braços vanilla:

- se não houver ramo binário funcional sem medição, promover a relação ao
  nível sustentado pelos artefatos;
- se unários ou outro braço permanecerem `Unknown`, manter `parcial` e listar
  nominalmente o que falta;
- nova divergência vira linha própria na fila.

Não promover `eval-apply-binary` por contagem de quatro fixes.

## 14. Gates finais

Executar, no mínimo:

```text
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

Executar a matriz e a lente duas vezes. Registrar HEAD, working tree, horário
e SHA-256 dos binários, mapa, lente e artefatos decisórios.

## 15. Separação de autoridades

Usar protocolo completo de materialização segregada:

- A congela contrato e oráculos sem ler o patch;
- B cria mutantes contra os quatro ramos;
- C implementa somente contra entradas seladas;
- D executa A/B sem promover mapa;
- E verifica recibos e decide fila/mapa.

Se uma única sessão tiver acesso a tudo, declarar:

```text
EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO
```

## 16. Continuação

Se os quatro ramos fecharem e nenhum binário novo aparecer, o próximo item da
fila é `svg-morphology`, salvo se a auditoria final de operadores produzir um
gap nominal anterior.

Se algum ramo exigir mudança pública protegida pelo ADR-0127, encerrar P1217
como descoberta reproduzível e escrever um sucessor com o gate explícito; não
misturar aprovação arquitetural com os fixes internos restantes.
