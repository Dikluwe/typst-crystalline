# P1157 — auditoria e nucleação de `page.numbering` por função

**Data:** 2026-08-25
**Estado:** `GATE ADR-0127 APROVADO — autorizado pelo dono em 2026-08-25`
**Baseline de linguagem:** vanilla ratificado `a51e02804`
**Dependência:** P1141–P1156 fechados; commit `34e3ffb2e`

## 1. Objetivo

Auditar a semântica pública de `page(numbering:)` quando recebe uma função,
medir o comportamento no vanilla ratificado e nuclear a representação e o
transporte necessários no cristalino. O passo termina depois de atualizar os
L0s e ressela-los, antes de qualquer alteração em L1.

Esta frente é a próxima unidade da fila ratificada no handoff pós-P1140. As
frentes anteriores até `content.fields` estão fechadas. Não reabrir `path`,
`counter`, `content.location` nem as 40 falhas já eliminadas.

## 2. Por que existe gate obrigatório

Hoje a numeração de página é representada publicamente como
`Option<EcoString>` em `Content::SetPage`, `PageConfig`, `Page` e `PageStore`.
O vanilla representa `Numbering::Pattern | Numbering::Func` e avalia a função
em contexto, com aridades diferentes conforme o consumidor.

Uma solução provável altera campos públicos de entidades e atravessa a
fronteira eval → layout. Portanto incidem os pontos 1 e 3 da ADR-0127:

- mudança de contrato público Rust;
- possível mudança de fase/ordem do pipeline para avaliar callbacks.

Mesmo sendo correção de paridade, o passo deve parar porque a forma do contrato
e o owner da avaliação são decisões arquiteturais. Nenhum teste RED de produção
nem código L1 deve ser escrito antes da aprovação do dono sobre os L0s.

## 3. Proveniência de entrada

Antes de medir, registrar:

```text
date --iso-8601=seconds
git rev-parse HEAD
git branch --show-current
git status --short
git diff HEAD --stat
lab/typst-original/target/release/typst --version
/usr/local/bin/typst --version
```

Baseline esperado:

- HEAD `34e3ffb2e` na branch `Tekt`;
- working tree limpa, exceto este passo novo;
- os dois binários vanilla correspondem ao pin ratificado `a51e02804`.

Não usar a string de versão isoladamente como prova de proveniência. Se HEAD ou
working tree divergirem, rebaselinear e registrar o estado exato antes de usar
qualquer número para decidir.

## 4. L0s e owners que devem ser lidos

Auditar integralmente, antes de propor a forma:

```text
00_nucleo/prompts/compiler/stdlib/numbering.md
00_nucleo/prompts/compiler/stdlib/layout.md
00_nucleo/prompts/compiler/eval.md
00_nucleo/prompts/compiler/layout.md
00_nucleo/prompts/compiler/introspect.md
00_nucleo/prompts/entities/content.md
00_nucleo/prompts/entities/layout_types.md
00_nucleo/prompts/entities/page_store.md
00_nucleo/prompts/entities/elements/page_run.md
```

Se algum path não existir, não inventar uma spec paralela: localizar o L0 owner
vigente pelo header `@prompt` do código. Confirmar os hashes atuais antes de
editar.

Owners mínimos a inventariar:

```text
01_core/src/compiler/stdlib/numbering.rs
01_core/src/compiler/stdlib/layout.rs
01_core/src/compiler/eval/rules.rs
01_core/src/entities/content.rs
01_core/src/entities/elements/page_run.rs
01_core/src/entities/layout_types.rs
01_core/src/entities/page_store.rs
01_core/src/entities/introspector.rs
01_core/src/compiler/layout/set_page.rs
01_core/src/compiler/layout/cursor.rs
01_core/src/compiler/layout/mod.rs
01_core/src/compiler/layout/references.rs
01_core/src/compiler/eval/call_dispatch.rs
```

Não varrer nem ler `00_nucleo/context/` ou `00_nucleo/materialization/`.

## 5. Medição vanilla obrigatória

Criar sondas temporárias fora do repositório, por exemplo em `/tmp`, e executar
cada uma nos dois binários vanilla ratificados. Capturar stdout, stderr, exit
status e, quando houver documento, texto extraído ou outra observação no nível
da língua. Não usar igualdade de bytes ou estrutura Rust como critério.

### 5.1 Aceitação e tipo

Medir ao menos:

1. `#set page(numbering: n => [p#n])` em documento de uma página;
2. callback que devolve string, content, inteiro e `none`;
3. `numbering:` inválido: int, array e função com aridade incompatível;
4. `#page(numbering: callback)[body]` para confirmar escopo lexical do page-run;
5. alternância entre pattern, callback e `none` em páginas consecutivas.

### 5.2 Aridade observável

Instrumentar callbacks que tornem a aridade visível sem depender de logs:

- número visível de página: confirmar que a função recebe
  `(current, total)`;
- referência `#ref(<alvo>, form: "page")`: confirmar que recebe somente
  `(current)`;
- `loc.page-numbering()`: medir o valor/repr retornado para pattern, função e
  página sem numbering;
- `counter(page).display()` sob o mesmo `#set page(numbering:)`;
- total de páginas maior que um, inclusive com quebra e page-run lexical.

Medir callbacks variádicos e closures que capturam variável lexical. Separar
claramente “o callback foi preservado” de “o resultado renderizado coincidiu”.

### 5.3 Número lógico versus página física

Medir interação com `counter(page).update`, `counter(page).step`, pagebreak e
total final. O vanilla documenta números lógicos, que podem divergir do índice
físico. Registrar os argumentos efetivamente observáveis pelo callback e o
resultado de referências.

### 5.4 Header/footer explícito

Confirmar que um header explícito, ou footer explícito conforme
`number-align`, suprime a numeração automática sem perder o numbering usado por
referências e introspecção. Testar alinhamento top e bottom.

## 6. Medição da fonte ratificada

Ler e citar `file:line` pelo menos em:

```text
lab/typst-original/crates/typst-library/src/model/numbering.rs
lab/typst-original/crates/typst-library/src/layout/page.rs
lab/typst-original/crates/typst-layout/src/pages/run.rs
lab/typst-original/crates/typst-layout/src/introspect.rs
lab/typst-original/crates/typst-library/src/introspection/location.rs
lab/typst-original/crates/typst-library/src/model/reference.rs
```

A fonte já indica, como hipótese a confirmar pelas sondas:

- `Numbering` é `Pattern | Func`;
- página visível usa dois números para função;
- links/referências usam um número;
- o introspector conserva o objeto `Numbering`, não apenas texto formatado;
- função é sempre tratada como dependente do total no número visível.

Não transformar essa hipótese em decisão antes das sondas. Marcar qualquer
inferência e declarar qual resultado a refutaria.

## 7. Auditoria do cristalino

Construir uma matriz `observável → representação → producer → consumers`.
Incluir obrigatoriamente:

| Observável | Estado cristalino a confirmar |
|---|---|
| `page(numbering:)` | aceita apenas `Str | none` |
| `Content::SetPage.numbering` | `Option<EcoString>` |
| `PageRunElem.numbering` | `Option<EcoString>` |
| `PageConfig.numbering` | `Option<EcoString>` |
| `Page.numbering` | `Option<EcoString>` |
| `PageStore.numberings` | `Vec<Option<EcoString>>` |
| número visível | `format_counter`, com fixup para total |
| referência de página | formata string armazenada |
| `loc.page-numbering()` | expõe pattern textual ou `none` |

Verificar também:

- se a função global `numbering()` já fornece um owner reutilizável para
  aplicar `Str | Func` sem duplicação;
- como `Func` carrega closure, scopes, engine/context e spans;
- se callbacks podem ser avaliados durante layout sem introduzir estado global,
  I/O em L1 ou import reverso;
- se o fixpoint de layout precisa transportar o callback até conhecer o total;
- se referência e número visível exigem chamadas distintas por aridade;
- quais derives (`PartialEq`, `Debug`, hash) deixam de ser válidos ou mudam de
  mecânica ao transportar `Func`.

## 8. Decisão L0 a redigir

Depois das medições, atualizar os L0s owners antes de qualquer código. A spec
deve decidir explicitamente:

1. o tipo canónico de numbering compartilhado (`Pattern | Func`) e seu owner;
2. quais entidades públicas trocam `Option<EcoString>` por esse tipo;
3. como representar os três estados: omitido/não alterar, `none`, e valor;
4. onde o callback é invocado e por que essa fase preserva contexto e total;
5. aridade de número visível versus referência;
6. transporte por `PageRun`, `SetPage`, `PageConfig`, snapshot `Page` e
   `PageStore`;
7. comportamento de `loc.page-numbering()` e referências;
8. interação com header/footer explícito e `number-align`;
9. erros e spans de callback;
10. compatibilidade e migração de testes/constructors públicos Rust.

Preferir reutilizar a abstração já especificada em `stdlib/numbering.md`. Não
duplicar um enum só para page sem demonstrar que o owner compartilhado é
insuficiente. Também não guardar apenas o resultado formatado: isso perderia a
aridade diferente de referências e a identidade funcional observável.

Se a medição revelar que a mudança pode ser inteiramente interna sem alterar
campo público nem fase, documentar a prova. Na dúvida, manter o gate.

## 9. Resselo e gate

Após editar os L0s:

```text
crystalline-lint --fix-hashes .
crystalline-lint .
git diff --check
git diff -- 00_nucleo/prompts/
git status --short
```

Registrar:

- hashes L0 novos;
- lista exata de L0s alterados;
- matriz das sondas nos dois vanilla;
- proposta de contrato e pipeline;
- inventário previsto de código/testes para o passo seguinte;
- HEAD, hora e `git diff HEAD --stat` da medição.

Então **PARAR** e pedir aprovação explícita do dono. Não adicionar ou alterar
campos, enums, signatures, traits, `Content`, `PageRunElem`, `PageConfig`,
`Page`, `PageStore`, eval, layout ou testes RED de produção neste passo.

## 10. Critérios de aceitação

- proveniência reproduzível do estado cristalino e dos dois vanilla;
- sondas cobrem tipo, retorno, erro, aridade 1/2, closure, total, referência,
  introspecção, escopo lexical, número lógico e header/footer;
- toda decisão vem depois da medição que a sustenta;
- língua e mecânica são classificadas conforme ADR-0107;
- inferências estão marcadas com condição de refutação;
- L0s owners foram atualizados e ressela-dos sem V5;
- nenhum código L1–L4 foi alterado;
- gate ADR-0127 apresentado ao dono com diff e hashes exatos.

## 11. Handoff

Após aprovação do gate, escrever P1158 para testes RED e materialização
incremental do contrato ratificado. P1158 deve começar pelo menor caminho
vertical que preserve callback do constructor à página visível, depois fechar
referências/introspecção e revalidar o workspace integral. Se o dono rejeitar
a forma proposta, corrigir primeiro os L0s e ressela-los; não adaptar o código
em paralelo.

## 12. Execução P1157

### 12.1 Proveniência

- Hora inicial: `2026-08-25T07:56:37-03:00`.
- HEAD: `34e3ffb2e06b105835939d0b4fd62e31c250211f`.
- Branch: `Tekt`.
- Entrada: somente `typst-passo-1157.md` não rastreado.
- Vanilla: os dois executáveis reportaram o mesmo build ratificado; a prova de
  alvo permanece o pin de fonte `a51e02804`, não a string de versão.

### 12.2 Resultados das sondas

Os dois binários produziram resultados idênticos:

| Sonda | Resultado observável |
|---|---|
| callback binário, 2 páginas | `V1/2`, `V2/2` |
| callback variádico + footer explícito + ref | `R1` |
| `loc.page-numbering()` | repr `(..) => ..`; margem `L2` |
| counter page atualizado para 7 | `N7/9`, `N8/9`, `N9/9`; ref `N7` |
| dois page-runs lexicais | callback `X1/2`; pattern `II` |
| retorno inteiro de `numbering(callback)` | `42` |
| retorno none | conteúdo vazio, exit 0 |
| `numbering: 42` | erro `expected string, function, or none` |
| callback unário visível | erro `unexpected argument` |
| callback ternário visível | erro `missing argument: c` |
| footer explícito | margem `EXPLICIT`; ref ainda chama callback → `AUTO1` |

Conclusão de língua: o objeto de numbering deve sobreviver intacto; número
visível usa dois números lógicos, referência usa um, e marginal explícito
suprime emissão sem apagar numbering introspectável/referenciável.

### 12.3 L0s atualizados

Criado:

```text
00_nucleo/prompts/entities/numbering.md
```

Atualizados:

```text
00_nucleo/prompts/compiler/stdlib/numbering.md
00_nucleo/prompts/compiler/stdlib/layout.md
00_nucleo/prompts/compiler/eval.md
00_nucleo/prompts/compiler/eval/call_dispatch.md
00_nucleo/prompts/compiler/layout.md
00_nucleo/prompts/compiler/layout_references.md
00_nucleo/prompts/compiler/introspect.md
00_nucleo/prompts/entities/content.md
00_nucleo/prompts/entities/elements/page_run.md
00_nucleo/prompts/entities/layout_types.md
00_nucleo/prompts/entities/page_store.md
00_nucleo/prompts/entities/introspector.md
```

Forma proposta:

- `Numbering::{Pattern(EcoString), Func(Func)}` como entidade L1 pura;
- deltas `Option<Option<Numbering>>` em SetPage/PageRun;
- snapshots `Option<Numbering>` em PageConfig/Page/PageStore;
- fixpoint realiza Content visível com `[current, total]` e Content de ref com
  `[current]`, preservando também o Numbering cru;
- layout e referência consomem Content realizado, sem executar callbacks;
- `loc.page-numbering()` devolve Str/Func/None a partir do objeto cru.

### 12.4 Resselo

`crystalline-lint --fix-hashes .` atualizou somente headers de linhagem em 33
ficheiros L1; o diff confirma uma substituição `@prompt-hash` por ficheiro,
sem lógica, teste, assinatura ou contrato materializado. Hashes principais:

```text
compiler/eval.md                  c1151e70
compiler/eval/call_dispatch.md    89d9e00a
compiler/introspect.md            3eddbd40
compiler/layout.md                12178489
entities/content.md               0fef45d0
entities/elements/page_run.md     ca20382f
entities/introspector.md          3e018674
entities/layout_types.md          3f259b60
entities/page_store.md            da26f2f0
```

Os L0s `stdlib/layout`, `stdlib/numbering` e `layout_references` não possuem
consumer com header hash gerido pelo linter. O L0 novo `entities/numbering`
mantém `Hash do Código` pendente porque o ficheiro alvo só poderá nascer em
P1158 após este gate.

Validação documental:

```text
git diff --check: exit 0
crystalline-lint .: exit 0; zero violations; zero V5
```

Warnings V16–V20 históricos continuam fora do escopo.

### 12.5 Gate

P1157 para aqui. Nenhum campo, enum, trait, assinatura, teste RED ou lógica de
produção foi materializado. Autorizar P1158 significa aprovar explicitamente:

1. o novo contrato público `Numbering`;
2. a troca dos campos públicos acima;
3. a realização de callbacks no fixpoint com vistas de aridade 2 e 1;
4. a alteração do contrato público de `Introspector::page_numbering`.

**Decisão do dono:** aprovado por “Continue” em 2026-08-25. P1158 autorizado.
