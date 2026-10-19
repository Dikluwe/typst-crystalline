# Passo 1140.3 — fechar os dois `WRONG_KIND` math sem colapsar contratos distintos

**Data:** 2026-08-23  
**Origem:** inventário P1140 após P1140.2  
**Vanilla ratificado:** `upstream/main a51e02804`  
**Natureza:** paridade pública de linguagem, atomizada em duas frentes  
**Gate:** ADR-0127 para qualquer alteração de contrato público ou comportamento default

## Estado de execução — 2026-08-23T22:40:27-03:00

### P1140.3-A concluída

- `math.sqrt` passou de símbolo a função nativa de um `Content`;
- `$sqrt(x)$` e a função pública compartilham a construção de `MathRoot`;
- o binding extra `sym.sqrt` foi removido;
- o `repr` de raiz sem índice foi corrigido para a morfologia vanilla
  `root(radicand: ...)`;
- testes focados: **3 passed, 0 failed**;
- inventário reconstruído: `WRONG_KIND` **2 → 1**, restando somente
  `math.equation`;
- `crystalline-lint --fix-hashes .`: **0 drift warnings**.

Proveniência desta revalidação: HEAD
`a8959bd184871d72f470ee4dd06d829e6ff0e483`, working tree não commitada; o
`git diff HEAD --stat` registrou **24 ficheiros**, **217 inserções** e **31
remoções** (o próprio passo ainda não rastreado não entra nessa contagem).

### P1140.3-B — L0 ressellado; gate aberto

Os probes vanilla confirmaram `body` obrigatório, `block: false` e os named
args públicos `numbering`, `number-align`, `supplement` e `alt`, além dos erros
de aridade/nome. O L0 foi atualizado para uma primeira fase explícita
`body`/`block`; os quatro campos restantes foram atribuídos ao P1140.4 e devem
ser rejeitados, não ignorados, até lá.

Esta substituição de `math.equation = none` por função/elemento chamável muda
contrato público e comportamento por defeito. Conforme ADR-0127, a execução
para aqui antes dos testes RED e do código de P1140.3-B, aguardando confirmação
humana do L0 ressellado.

### P1140.3-B concluída após confirmação humana

A confirmação foi recebida e a frente executada em RED → GREEN. A nativa
aceita `body` e `block`, compartilha `equation_content` com a sintaxe dedicada,
rejeita os quatro campos reservados ao P1140.4 e corrige a morfologia de
`repr(Content::Equation)`. O inventário release final contém **0
`WRONG_KIND`**. Relatório:
`00_nucleo/diagnosticos/typst-passo-1140.3-relatorio.md`.

## 1. Objetivo

Fechar os dois `WRONG_KIND` restantes no inventário P1140:

1. `math.sqrt`: símbolo no cristalino, função no vanilla;
2. `math.equation`: `none` no cristalino, função/elemento no vanilla.

Os casos partilham o módulo público `math`, mas não a causa nem o tamanho do
contrato. O passo é dividido em:

- **P1140.3-A — `math.sqrt`:** corrigir a colisão função × símbolo e materializar
  o construtor de raiz quadrada no namespace público;
- **P1140.3-B — `math.equation`:** reconciliar o elemento chamável com a sintaxe
  `$...$` e com o transporte de estilos já existente.

Uma frente não pode usar o fechamento da outra como autorização implícita.

## 2. Proveniência inicial

Medição em `2026-08-23T22:27:49-03:00`:

- HEAD: `0314efaea6a5cb4c377b9190c814e3b38a080d68`;
- working tree não commitada, após implementação de P1140.2;
- vanilla: `lab/typst-original/target/release/typst`, baseline ratificado
  `a51e02804`;
- cristalino: `target/release/typst`, rebuild de P1140.2;
- fonte vanilla:
  `lab/typst-original/crates/typst-library/src/math/{mod,root,equation}.rs`;
- fonte cristalina:
  `01_core/src/compiler/stdlib/structural/math.rs`,
  `01_core/src/compiler/eval/math.rs` e
  `01_core/src/compiler/stdlib/sym.rs`.

O estado não commitado deve ser novamente registrado no fechamento com
`git diff HEAD --stat` e hora exata. Nenhum número desta seção autoriza fechar
o passo sem essa revalidação.

## 3. Medição anterior à decisão

### 3.1 Inventário

Após P1140.2, o catálogo contém:

| Classe | Total |
|---|---:|
| `MATCH` | 799 |
| `WRONG_KIND` | 2 |

Os únicos `WRONG_KIND` são:

| Path | Vanilla | Cristalino |
|---|---|---|
| `math.equation` | `function` | `none` |
| `math.sqrt` | `function` | `symbol` |

### 3.2 Probes observáveis

| Expressão | Vanilla | Cristalino antes de P1140.3 |
|---|---|---|
| `repr(type(math.equation))` | `"function"` | `"type(none)"` |
| `repr(type(math.sqrt))` | `"function"` | `"symbol"` |
| `repr(type(math.sqrt([x])))` | `"content"` | erro: símbolo não chamável |
| `repr(math.sqrt([x]))` | `"root(radicand: [x])"` | erro: símbolo não chamável |
| `repr(type(math.equation([x])))` | `"content"` | erro: `none` não chamável |
| `repr(math.equation([x]))` | `"equation(body: [x])"` | erro: `none` não chamável |
| `repr(type(sym.sqrt))` | erro: campo ausente | `"symbol"` |
| `repr(sym.sqrt)` | erro: campo ausente | `"symbol(\"√\")"` |
| `repr(type(math.root))` | `"function"` | erro: campo ausente |

A última linha é descoberta adjacente, não autorização automática para
expandir P1140.3. `math.root` deve ser classificado no início de P1140.3-A:
ou entra por acoplamento inseparável com `MathRootElem`, ou fica registrado
como `MISSING_MEMBER` para passo próprio. A decisão vem depois da medição.

### 3.3 Fonte vanilla — `sqrt`

`math/mod.rs` registra `sqrt` com `define_func::<sqrt>()`. `math/root.rs`
define `sqrt(radicand: Content) -> Content`, produzindo `RootElem` sem índice.
O símbolo `√` não é publicado como `sym.sqrt`; portanto o path extra
cristalino não é uma alternativa equivalente à função.

### 3.4 Fonte cristalina — `sqrt`

O avaliador de modo math já intercepta o nome sintático `sqrt` e produz
`Content::math_root(None, radicand)`. Porém `make_math_module()` importa os
símbolos de `sym` depois de montar suas funções e, como nenhuma função `sqrt`
foi registrada, o `sqrt` da tabela cristalina de símbolos ocupa o path.
Em código, field access encontra esse símbolo e a chamada falha.

Logo a correção não deve duplicar o algoritmo de raiz: deve criar uma única
nativa dona do construtor e fazer o modo math delegar ou preservar exatamente
a mesma morfologia.

### 3.5 Fonte vanilla — `equation`

`math/mod.rs` registra `EquationElem` como elemento. O struct vanilla expõe:

- `block`, default `false`;
- `numbering` opcional;
- `number-align`, default `end + horizon`;
- `supplement`;
- `alt`;
- `body` obrigatório;
- campos internos de tamanho/variante/cramped/bold.

A função pública e a sintaxe `$...$` produzem a mesma família morfológica,
mas isso não obriga igualdade da estrutura Rust (ADR-0107).

### 3.6 Fonte cristalina — `equation`

O cristalino já possui `EquationElem { body, block }`, sintaxe `$...$`, layout,
introspecção e transporte de `equation.numbering` pela `StyleChain`. Contudo
`make_math_module()` registra `equation` como `Value::None`, apesar de o L0
vigente chamá-lo de “alias”. Testes antigos consolidam esse placeholder.

O L0 `compiler/stdlib/structural/math.md` também descreve o módulo como
`Value::Dict`, enquanto o código e a paridade vigente usam `Value::Module`.
Essas frases estão desatualizadas e devem ser corrigidas antes do código.

## 4. Classificação

As duas diferenças são semântica e morfologia públicas da linguagem
(ADR-0107), pois alteram kind, chamabilidade e conteúdo produzido.

- `math.sqrt` é correção localizada de binding/tabela e construtor já
  representável por `MathRootElem`;
- `math.equation` envolve contrato de elemento, defaults, named args e
  integração set/show. Tratar seu `Value::None` como mera entrada de tabela
  esconderia uma implementação parcial.

Inferência: P1140.3-A pode seguir em fluxo contínuo se a auditoria confirmar
que só corrige tabela interna/paridade já especificada. P1140.3-B provavelmente
altera contrato público/default e deve parar no gate ADR-0127. A inferência é
refutada se o L0 vigente já especificar integralmente uma nativa pública com a
mesma assinatura e defaults.

## 5. Auditoria L0 obrigatória

Antes de qualquer teste RED, ler integralmente e confirmar hashes de:

- `00_nucleo/prompts/compiler/stdlib/structural/math.md`;
- `00_nucleo/prompts/compiler/stdlib/structural.md`;
- `00_nucleo/prompts/entities/elements/math_root.md`;
- `00_nucleo/prompts/entities/elements/equation.md`;
- L0 do dispatcher de chamadas/elementos que for efetivamente tocado;
- L0 de `sym` se a tabela `sym.sqrt` for corrigida.

Atualizações mínimas esperadas:

1. corrigir `Value::Dict` → `Value::Module` no L0 de `structural/math`;
2. substituir a promessa de `equation` como alias/`None` pelo contrato medido;
3. especificar a função `math.sqrt(Content) -> Content::MathRoot`;
4. declarar explicitamente quais campos públicos de `math.equation` entram
   nesta execução e quais ficam incompletos, nomeando o passo futuro que os
   completa, conforme a regra de divisão de constructor;
5. separar `sym.sqrt` extra da função `math.sqrt`.

Se P1140.3-B exigir campo novo em `EquationElem`, assinatura pública nova,
default novo ou mudança de pipeline, ressellar os L0 e **parar para confirmação
humana** antes do RED.

## 6. P1140.3-A — `math.sqrt`

### 6.1 Forma de implementação

1. Criar uma free function nativa dona de `sqrt(radicand)` na unidade math da
   stdlib; ela aceita exatamente um `Content` e devolve
   `Value::Content(Content::math_root(None, radicand))`.
2. Registrar `math.sqrt` como `Value::Func` antes da importação dos símbolos.
3. Garantir que a importação `sym → math` não sobrescreve funções.
4. Remover ou renomear a entrada cristalina extra `sym.sqrt` somente depois de
   medir a tabela vanilla correspondente; não manter o path extra como
   “compatibilidade” sem decisão explícita.
5. Reusar a mesma construção no modo math. Não manter dois parsers de aridade
   com mensagens e aceitação divergentes.

Se `math.rs` ficar menos legível, atomizar em
`compiler/stdlib/structural/math/root.rs`, chamado por free function a partir
do hub. Não mover lógica para `MathRootElem`, pois isso acoplaria dado a
avaliação e contrariaria a forma B da ADR-0109.

### 6.2 Testes RED

```typst
repr(type(math.sqrt)) == "function"
type(math.sqrt([x])) == content
repr(math.sqrt([x])) == "root(radicand: [x])"
$ sqrt(x) $                    // continua MathRoot sem índice
math.sqrt()                    // erro
math.sqrt([x], [y])            // erro unexpected argument
math.sqrt(1)                   // erro de cast para content
```

Não regressão:

- `calc.sqrt` continua numérico e independente;
- `$sqrt(x)$` mantém layout com radical e overline;
- `math.sqrt` não volta a ser sombreado pela tabela de símbolos;
- símbolos homônimos válidos permanecem acessíveis por seu path vanilla real.

## 7. P1140.3-B — `math.equation`

### 7.1 Medir antes de escolher o subconjunto

Executar no vanilla, no mínimo:

```typst
repr(type(math.equation))
repr(math.equation([x]))
repr(math.equation(block: true, [x]))
repr(math.equation(numbering: "(1)", block: true, [x]))
repr(math.equation(number-align: bottom, [x]))
repr(math.equation(supplement: [Eq.], [x]))
repr(math.equation(alt: "x", [x]))
math.equation()
math.equation([x], [y])
math.equation(foo: 1, [x])
```

Para cada observável, registrar saída/erro, HEAD, diff stat e hora. Medir
também `#set math.equation(...)` e `$...$` para distinguir contrato do
construtor, styles settable e sintaxe dedicada.

### 7.2 Decisão condicionada à medição

A implementação deve produzir `Content::Equation` pela unidade dona, sem
duplicar a sintaxe `$...$`. Há duas saídas admissíveis após a auditoria:

- **completa:** suportar todos os campos públicos representáveis e transportar
  os restantes na entidade/style chain com L0 e gate próprios;
- **faseada:** implementar somente o subconjunto explicitamente declarado no
  L0 como incompleto e nomear o passo que completa cada campo ausente.

Não é admissível apenas trocar `Value::None` por uma função que ignora named
args ou aceita defaults inventados. Também não é admissível mover numbering da
chain para o struct só para copiar a mecânica vanilla: o observável é a
linguagem, e a divergência estrutural atual é autorizada pela ADR-0107.

### 7.3 Integrações obrigatórias

- `#set math.equation(numbering: ...)` continua usando a fonte única vigente;
- show/set por `math.equation` continua reconhecendo o elemento;
- `query(math.equation)`/selectors não podem depender do antigo `Value::None`;
- sintaxe inline e block `$...$` mantém a morfologia;
- numbering continua restrito a equation block quando esse é o contrato
  medido;
- referências, supplement e alt só entram se houver representação e consumo
  legitimados por L0; caso contrário ficam scope-out explícito, não ignorados.

### 7.4 Testes RED mínimos

```typst
repr(type(math.equation)) == "function"
type(math.equation([x])) == content
repr(math.equation([x])) == "equation(body: [x])"
math.equation(block: true, [x])
math.equation()                 // erro: body obrigatório
math.equation([x], [y])         // erro unexpected argument
math.equation(foo: 1, [x])      // erro unexpected argument: foo
```

Acrescentar testes para cada named arg autorizado pelo L0 atualizado e para
cada scope-out explícito.

## 8. Sequência de execução

1. Repetir probes e registrar proveniência exata.
2. Auditar e corrigir os L0 donos, medindo antes de decidir.
3. Ressellar hashes.
4. Se houver classe ADR-0127, parar no gate e obter confirmação humana.
5. Executar P1140.3-A em RED → GREEN.
6. Recontar inventário: `math.sqrt` deve deixar `WRONG_KIND` sem regressão de
   paths adjacentes.
7. Executar a auditoria ampliada de P1140.3-B.
8. Atualizar L0 de equation e parar novamente se o contrato medido ampliar o
   gate já confirmado.
9. Executar P1140.3-B em RED → GREEN.
10. Recontar inventário: `WRONG_KIND == 0`.
11. Regenerar probes e atualizar o relatório em `00_nucleo/diagnosticos/`.
12. Executar testes focados, suíte L1, `cargo build`,
    `crystalline-lint .`, `cargo fmt --check` e `git diff --check`.

## 9. Critérios de aceitação

- [ ] L0 medido, atualizado e ressellado antes do código.
- [ ] Gates ADR-0127 cumpridos separadamente para A e B quando aplicáveis.
- [ ] `math.sqrt` é função pública e produz `MathRoot`.
- [ ] `calc.sqrt` não regride.
- [ ] A colisão com símbolo é corrigida sem criar alias não-vanilla.
- [ ] `math.equation` é função/elemento público chamável.
- [ ] Sintaxe `$...$`, set/show, selectors e introspecção não regridem.
- [ ] Nenhum named arg é silenciosamente ignorado.
- [ ] Constructor faseado, se escolhido, declara incompletude e passo de
      fechamento no L0.
- [ ] Inventário termina com `WRONG_KIND == 0`.
- [ ] Relatório fica em `00_nucleo/diagnosticos/`, não em `prompts/` nem na
      raiz.
- [ ] Testes e travas finais passam; falhas preexistentes são registradas com
      proveniência e não mascaradas.

## 10. Fora de escopo

- implementar todos os `MISSING_MEMBER` de math;
- corrigir `math.root` sem a classificação exigida em §3.2;
- alterar algoritmo de layout de radical/equation;
- mudar representação Rust apenas para copiar o vanilla;
- P1141/SVG;
- metadados reflexivos gerais de `ParamInfo`.

## 11. Próxima decisão após o fechamento

Com `WRONG_KIND == 0`, retornar ao catálogo P1140 e escolher a próxima frente
por família — globais ausentes, membros funcionais ou tabelas de símbolos —
sem misturar contagem de paths com prioridade arquitetural.
