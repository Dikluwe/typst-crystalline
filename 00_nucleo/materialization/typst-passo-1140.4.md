# Passo 1140.4 — completar o contrato público de `math.equation` por eixos observáveis

**Data:** 2026-08-23  
**Origem:** scope-out explícito de P1140.3-B  
**Vanilla ratificado:** `upstream/main a51e02804`  
**Natureza:** paridade pública de linguagem, dividida por consumidor  
**Gate:** ADR-0127 antes de cada campo que amplie contrato/default/pipeline

## 1. Objetivo

Completar os quatro named args públicos que P1140.3-B mediu e deixou
explicitamente incompletos em `math.equation`:

1. `numbering`;
2. `number-align`;
3. `supplement`;
4. `alt`.

O passo não trata os quatro nomes como uma única alteração de struct. Cada um
tem consumidores e observáveis diferentes e deve fechar em RED → GREEN
separado. A função mínima `math.equation(body, block:)` entregue por P1140.3-B
permanece a base comum.

## Estado de execução — P1140.4-A no gate

Medição repetida em `2026-08-23T22:49:03-03:00`, HEAD
`a8959bd184871d72f470ee4dd06d829e6ff0e483`, working tree não commitada. Os
probes confirmaram `numbering: str | function | none` e erro para inteiro. A
auditoria encontrou que layout/introspecção reduziam o contrato a `Str`; os L0
foram atualizados para transporte tipado e materialização pós-fixpoint por
Location, reutilizando callbacks de contador e mantendo o layouter puro.

P1140.4-A altera contrato público e o estágio pós-fixpoint. Conforme ADR-0127,
a execução para antes do RED/código e aguarda confirmação humana específica
deste L0. A confirmação de A não autoriza B–D.

### Retificação E2E de P1140.4-A

Após o RED e a ligação inicial, o probe de produção
`#set math.equation(numbering: n => "N" + str(n))` compilou, mas o PDF mostrou
`Equation 1`, não `N1`. A fonte confirma que `03_infra/src/pipeline.rs` chama
`introspect_with_introspector` diretamente e não `run_fixpoint`; o callback
pós-fixpoint implementado em L1 não é alcançado pelo binário.

A frente fica retificada em **A1 (`str | none`)** e **A2 (`function`)**. A2
muda a fase efetiva do pipeline de produção e exige novo gate ADR-0127. É
proibido fechar A aceitando função sem execução. O código permanece WIP: testes
de superfície estão GREEN, mas o probe E2E refuta o fechamento.

## 2. Proveniência inicial

Estado medido no fechamento de P1140.3:

- HEAD: `a8959bd184871d72f470ee4dd06d829e6ff0e483`;
- working tree não commitada;
- hora: `2026-08-23T22:45:32-03:00`;
- `git diff HEAD --stat`: 25 ficheiros, 331 inserções, 43 remoções; ficheiros
  novos ainda não rastreados não entram nessa contagem;
- inventário release: `WRONG_KIND == 0`; `math.equation` passou para
  `UNVERIFIED_METADATA`;
- vanilla: `lab/typst-original/target/release/typst`, baseline `a51e02804`.

Repetir HEAD, hora e `git diff HEAD --stat` antes de usar qualquer número para
fechar este passo.

## 3. Medição anterior à decisão

### 3.1 Contrato vanilla já confirmado

```text
repr(math.equation(numbering: "(1)", block: true, [x]))
→ equation(block: true, numbering: "(1)", body: [x])

repr(math.equation(number-align: bottom, [x]))
→ equation(number-align: bottom, body: [x])

repr(math.equation(supplement: [Eq.], [x]))
→ equation(supplement: [Eq.], body: [x])

repr(math.equation(alt: "x", [x]))
→ equation(alt: "x", body: [x])
```

Também foi confirmado:

- `body` é obrigatório;
- segundo positional é `unexpected argument`;
- named desconhecido é `unexpected argument: <nome>`;
- default de `block` é `false`;
- default de `number-align` é `end + horizon` e é omitido do `repr`;
- numbering só conta e renderiza número para equação de bloco.

### 3.2 Estado cristalino

- `EquationElem` contém somente `body` e `block`;
- `numbering` de set-rule vive apenas em
  `StyleChain::custom("equation.numbering")`, divergência mecânica já
  autorizada pela ADR-0107;
- `layout/equation.rs` lê esse custom para decidir/formatar a numeração;
- o braço dedicado de `#set math.equation(...)` em `eval/rules.rs` trata
  somente `numbering` e retorna cedo; `block`, `number-align`, `supplement` e
  `alt` são atualmente ignorados por esse caminho;
- `number-align` não chega ao posicionamento do número;
- `supplement` não chega ao payload/ref de equação;
- `alt` não chega à árvore acessível/exportadores;
- a nativa P1140.3-B rejeita os quatro campos, conforme o L0 faseado.

Essas diferenças são semântica/morfologia da linguagem, não obrigação de
copiar a estrutura Rust vanilla.

## 4. Auditoria L0 obrigatória

Antes de qualquer teste RED, ler integralmente e confirmar os hashes dos L0
dos módulos efetivamente afetados, no mínimo:

- `00_nucleo/prompts/compiler/stdlib/structural/math.md`;
- `00_nucleo/prompts/compiler/stdlib/structural.md`;
- `00_nucleo/prompts/entities/elements/equation.md`;
- `00_nucleo/prompts/compiler/eval.md`;
- L0 de `eval/rules.rs`/set-rules;
- L0 de `compiler/layout/equation.rs`;
- L0 de introspecção, referências e acessibilidade somente quando cada eixo
  realmente os tocar.

Se a auditoria decidir criar `compiler/stdlib/structural/math/equation.rs`,
redigir antes o L0 próprio
`00_nucleo/prompts/compiler/stdlib/structural/math/equation.md`. Não criar o
arquivo de código apenas para reduzir LOC: a fronteira precisa ser sustentada
pela unidade pública `math.equation` e por seus testes.

## 5. Atomização do trabalho

### P1140.4-A — `numbering` no construtor

Aceitar no construtor o mesmo domínio medido no vanilla: padrão, função ou
`none`. Preservar a decisão vigente de transportar o valor pela style chain;
não adicionar `numbering` ao `EquationElem` apenas para copiar a mecânica
vanilla.

O conteúdo produzido pelo construtor deve transportar o estilo junto da
equação sem alterar o estilo exterior. Inline com numbering continua sem
contagem/render de número; block com numbering participa do contador,
referências e layout. `repr` deve revelar o named arg na morfologia do elemento,
mesmo que o dado esteja mecanicamente em `Content::Styled`.

Medir antes de decidir:

```typst
repr(math.equation(numbering: "(1)", [x]))
repr(math.equation(numbering: none, [x]))
repr(math.equation(numbering: n => str(n), block: true, [x]))
math.equation(numbering: 1, [x])
#set math.equation(numbering: "(I)")
```

### P1140.4-B — `number-align`

Materializar o tipo de alinhamento aceito e o default `end + horizon`. Medir
combinações horizontais/verticais, direção RTL e o que ocorre quando a equação
não tem número. O consumidor dono é o posicionamento da numeração em
`compiler/layout/equation.rs`; não mover lógica de layout para a entidade.

Probes mínimos:

```typst
repr(math.equation(number-align: bottom, [x]))
repr(math.equation(number-align: left + top, [x]))
math.equation(number-align: 1, [x])
#set math.equation(number-align: bottom)
```

Comparar posições por observável de língua/render, com proveniência de páginas
ou coordenadas; não usar igualdade de frames Rust como critério.

### P1140.4-C — `supplement`

Medir e implementar `auto`, `none`, conteúdo e função conforme o cast vanilla.
O supplement é observável em referências a equações numeradas; não basta
armazená-lo ou fazê-lo aparecer no `repr`. Definir fonte única entre named arg,
set-rule, introspecção e `@label`.

Probes mínimos:

```typst
repr(math.equation(supplement: auto, [x]))
repr(math.equation(supplement: none, [x]))
repr(math.equation(supplement: [Eq.], [x]))
#set math.equation(numbering: "(1)", supplement: [Eq.])
$ x $ <eq-x>
@eq-x
```

Medir locale/default `auto` antes de decidir sua representação. Não inventar
string fixa global.

### P1140.4-D — `alt`

Aceitar `str` ou `none` conforme o vanilla e transportar a descrição até o
consumidor de acessibilidade. `alt` não substitui `plain_text`, não altera o
layout visual e não deve ser descartado pelo exportador tagueado/HTML quando
esses pipelines suportarem o observável.

Probes mínimos:

```typst
repr(math.equation(alt: "x squared", [x^2]))
repr(math.equation(alt: none, [x]))
math.equation(alt: [x], [x])
#set math.equation(alt: "description")
```

Se o pipeline acessível necessário ainda não existir, separar claramente:
armazenamento/morfologia agora e consumo num passo nomeado. Não declarar
paridade de acessibilidade com um campo morto.

### P1140.4-E — set/show, reflexão e fechamento

Eliminar o retorno antecipado que silencia campos diferentes de `numbering`
em `#set math.equation`. Todos os cinco named args devem ser ou aplicados ou
rejeitados por diagnóstico correto; nenhum pode ser ignorado.

Revalidar:

- `#set math.equation(...)` por campo e em combinações;
- `#show math.equation: ...`;
- `query(math.equation)` e selectors;
- `repr`, `type` e metadados de parâmetros do binding;
- sintaxe `$...$` inline/block;
- numbering, referências e labels;
- map/hash/eq da entidade para qualquer campo que nela seja armazenado.

## 6. Forma arquitetural

- O match exaustivo sobre `Content` permanece.
- Construção/validação da função fica na unidade stdlib de `math.equation`.
- Dados semânticos pertencem à entidade ou style chain conforme a decisão L0;
  lógica de layout permanece em `compiler/layout/equation.rs`.
- Não introduzir `dyn`, registry, `PropMap` ou import reverso
  `entities → compiler`.
- `numbering` continua na chain salvo nova decisão explícita que revogue a
  divergência mecânica vigente.
- Atualizar `map_content`, `map_text`, `Hash`, `PartialEq`, payload e `repr`
  somente para campos realmente armazenados na entidade.

## 7. Gates ADR-0127

Cada frente altera contrato público, default ou consumidor do pipeline. Para
A–D:

1. medir vanilla e o cristalino;
2. atualizar o L0 dono;
3. ressellar hashes;
4. **parar para confirmação humana**;
5. somente depois escrever RED e código.

Uma confirmação de A não autoriza B–D. Correções internas descobertas durante
uma frente podem seguir em fluxo contínuo apenas se couberem expressamente nas
classes dispensadas pela ADR-0127.

## 8. Sequência de execução

1. Repetir proveniência e probes completos.
2. Auditar a representação de alinhamento, numbering, supplement e alt já
   disponível em L1.
3. Decidir se a unidade stdlib merece arquivo próprio e escrever seu L0.
4. Executar P1140.4-A em L0 → gate → RED → GREEN.
5. Repetir o ciclo separadamente para B, C e D.
6. Executar E para integração set/show/reflexão.
7. Regenerar inventário e probes; `WRONG_KIND` deve permanecer zero e
   `math.equation` só deixa `UNVERIFIED_METADATA` com prova dos metadados.
8. Escrever relatório em `00_nucleo/diagnosticos/`.
9. Rodar testes focados, suíte L1, `cargo build`, `crystalline-lint .`,
   `cargo fmt --check` e `git diff --check`.

## 9. Critérios de aceitação

- [ ] L0 medido e ressellado antes de cada frente.
- [ ] Gates A–D confirmados separadamente.
- [ ] Os cinco named args públicos do vanilla são aceitos com tipos/defaults
      medidos.
- [ ] Nenhum named arg de constructor ou set-rule é ignorado silenciosamente.
- [ ] `numbering` mantém gate block-level e fonte única.
- [ ] `number-align` altera a posição do número nos casos observáveis.
- [ ] `supplement` participa de referências e respeita `auto`/locale.
- [ ] `alt` chega ao consumidor acessível ou fica em fase nomeada sem alegação
      de paridade completa.
- [ ] `repr` omite defaults e expõe valores customizados como o vanilla.
- [ ] `$...$`, show/selectors/query, labels e referências não regridem.
- [ ] `WRONG_KIND == 0` permanece.
- [ ] Metadados só são marcados verificados a partir de probes reproduzíveis.
- [ ] Relatório final fica em `00_nucleo/diagnosticos/`.
- [ ] Testes/travas finais passam; falhas preexistentes são registradas.

## 10. Fora de escopo

- `math.root` e demais `MISSING_MEMBER`;
- mudanças no algoritmo geral de layout matemático sem relação com a posição
  da numeração;
- copiar tipos/traits internos do vanilla por igualdade mecânica;
- corrigir as duas falhas preexistentes P862;
- declarar acessibilidade completa se o consumidor final não for exercitado;
- iniciar nova família do inventário P1140 antes de fechar ou fasear
  explicitamente A–E.

## 11. Resultado esperado

`math.equation` deixa de ser apenas uma função de kind correto e passa a ter o
contrato público completo, com cada campo ligado ao consumidor que lhe dá
significado. O passo preserva a atomização: constructor, estilo, layout,
referência e acessibilidade cooperam por contratos explícitos, sem concentrar
toda a lógica no `EquationElem` nem no hub `structural/math.rs`.

## 12. Estado de execução

- **P1140.4-A — implementado e integrado em 2026-08-23.** `numbering`
  aceita `str | function | none`; callback recebe o número convergido e o
  resultado materializado alimenta equação e referência.
- A medição ponta a ponta descobriu um bypass: a pipeline paginada de produção
  fazia apenas introspecção estrutural, enquanto os pós-processadores que
  requerem `Engine` estavam ligados ao orquestrador de fixpoint usado por
  testes. A integração comum foi adicionada antes do layout e coberta por teste
  da API pública de compilação.
- **P1140.4-B — no gate L0; P1140.4-C–E — não iniciados.** Permanecem
  sujeitos aos gates separados da seção 7.
- A auditoria transversal do bypass está em
  `00_nucleo/diagnosticos/typst-p1140.4-bypass-pos-processadores.md`.

### Gate P1140.4-B — `number-align`

Medição repetida em `2026-08-24T08:47:19-03:00`, HEAD
`ffd527c85dd7d547413d33cbc2d27a80e32a3f8c`, working tree não commitada;
o `git diff HEAD --stat` registava 33 ficheiros, 539 inserções e 76 remoções
antes das alterações documentais deste gate.

O vanilla pinado confirmou:

- cast `alignment`, com horizontal restrito a `start | left | right | end` e
  vertical `top | horizon | bottom`;
- default efetivo `end + horizon`, completado por eixo no consumidor;
- `repr` preserva o valor explícito sem assar os componentes omitidos;
- em RTL, `start` ancora à direita e `end` à esquerda;
- em multiline, `top` usa a primeira baseline, `bottom` a última e `horizon`
  centra os frames; em uma linha, os três coincidem na baseline;
- sem numbering ativo, o alinhamento não tem efeito visual.

Os L0 donos foram atualizados para transporte em style chain, cast/set-rule,
geometria de linhas produzida pelo mesmo run matemático e consumo pelo layout.
Como a frente amplia contrato público e muda posicionamento por defeito, a
execução parou aqui no gate ADR-0127; naquele estado ainda não havia teste RED
nem código de B.

### Fechamento P1140.4-B

Gate confirmado pelo dono em 2026-08-24. Implementado em RED → GREEN:

- construtor e set-rule aceitam/validam `number-align` e o transportam na
  style chain sem alterar `EquationElem`;
- `repr` preserva alinhamento explícito e combina-o com `numbering`;
- o layout resolve margens físicas e `start`/`end` por `text.dir`;
- a medição matemática devolve baselines da primeira/última linha no mesmo
  run, permitindo `top`/`horizon`/`bottom` sem relayout;
- o fixup de página `width: auto` transporta também a margem escolhida.

Probes CLI contra o artefato final confirmaram margem esquerda/direita, RTL e
ordem vertical `top < horizon < bottom`; testes L1 cobrem cast/repr/set-rule,
margens, RTL e multiline. **P1140.4-C–E permanecem não iniciados e sujeitos a
seus próprios gates.**

Validação final repetida em `2026-08-24`, HEAD
`ffd527c85dd7d547413d33cbc2d27a80e32a3f8c`, working tree não commitada. No
momento da medição, `git diff HEAD --stat` registava 40 ficheiros, 1011
inserções e 97 remoções (inclui P1140.4-A e B). A suíte L1 terminou com 5135
testes aprovados e somente as duas falhas P862 preexistentes declaradas fora de
escopo; os 6 testes focados de P1140.4-B e o teste legado atualizado passaram.

### Gate P1140.4-C — `supplement`

Medição repetida em `2026-08-24T09:02:44-03:00`, HEAD
`ffd527c85dd7d547413d33cbc2d27a80e32a3f8c`, working tree não commitada; antes
das alterações L0 deste gate, `git diff HEAD --stat` registava 40 ficheiros,
1018 inserções e 97 remoções.

O vanilla pinado confirmou:

- domínio `content | function | none | auto`, com string castável para
  conteúdo e erro exato para inteiro direto;
- default `auto`, localizado pela língua da equação (`en/pt/de/fr/es/it`
  medidos), não pela língua posterior da referência;
- `none` e conteúdo vazio removem o prefixo; conteúdo não vazio junta ao
  número com NBSP;
- função recebe a própria equação e seu retorno passa pelo cast para conteúdo;
- suplemento explícito em `@ref[...]` sobrepõe o suplemento do elemento;
- `repr` omite apenas o campo ausente e preserva todas as formas explícitas.

O cristalino rejeita hoje `supplement` no constructor/set-rule e usa o default
de Equation pela língua corrente da referência. Os L0 donos foram atualizados
para transporte lexical sem ampliar `EquationElem`, materialização no
pós-processador com Engine, sub-store por Location e consumo read-only pela
referência. A frente muda contrato público e comportamento de referências;
portanto para aqui no gate ADR-0127, sem teste RED nem código de C.

### Fechamento P1140.4-C

Gate confirmado pelo dono em 2026-08-24. Implementado em RED → GREEN:

- constructor e set-rule aceitam/canonicalizam `content | function | none |
  auto`, incluindo string → conteúdo, e `repr` preserva explicitude;
- o walk captura especificação e língua na equação; o pós-processador com
  `Engine` materializa o suplemento uma vez por Location;
- callback recebe a equação e pode observar `block`/`body`; retorno passa pelo
  cast de display;
- o `Introspector` entrega apenas `Content` ao layout; referência explícita
  sobrepõe o alvo e conteúdo não vazio usa NBSP;
- helper atomizado cobre `en/pt/de/fr/es/it` medidos, com fallback inglês;
- a pipeline de produção cobre callback, locale do alvo e override da ref.

Validação em `2026-08-24T09:25:48-03:00`, HEAD
`ffd527c85dd7d547413d33cbc2d27a80e32a3f8c`, working tree não commitada;
`git diff HEAD --stat` registava 51 ficheiros, 1522 inserções e 108 remoções
(acumulado de P1140.4-A–C). L1: 5138 aprovados e somente as duas falhas P862
preexistentes. L3: 824 aprovados e uma falha ambiental preexistente no teste
de CA local (`Operation not permitted` no sandbox); o teste E2E C passou.
**As antigas frentes P1140.4-D–E foram atomizadas no P1140.5 e permanecem não
iniciadas, sujeitas ao gate próprio daquele passo.**
