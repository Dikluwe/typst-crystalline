# Prompt L0 — `compiler/stdlib/structural/outline` — `outline`, `lof`, `lot`
Hash do Código: 8f2eec97

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/structural/outline.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/stdlib/structural.md` — dono de
`structural/mod.rs` e da história acumulada. Este L0 especifica **a superfície do nó**.
**Convenções partilhadas**: `00_nucleo/prompts/compiler/stdlib/_comum.md`
**Vanilla**: `typst-library/src/model/outline.rs` (`OutlineElem`). `lof`/`lot` **não têm
homólogo vanilla** — são extensão cristalina, aliases de `outline(target:)`.

**Fronteira medida — o único cluster real de seccionamento**: `2ca61c873` (2026-06-27) é
commit de corpo real das três. Não é inserção pura: o corpo de `native_outline` foi
**alterado** (`Ok(Value::Content(Content::outline_with(title, depth, indent)))` removido,
substituído pelo parser de `target:` + `OutlineElem::with_target(...)`), e `lof`/`lot`
nasceram no mesmo commit **como aliases desse `target:`**.

```
$ git show 2ca61c873 -U0 -- <path> | grep '^@@'
@@ -267 +269,61 @@ pub fn native_outline(      ← -267 = linha removida: alteração de corpo
```

Um commit só, portanto **evidência fina** — mas com **mecanismo**: as três partilham a
entidade (`OutlineElem` + `OutlineTarget`), e os aliases não podiam existir sem a mudança
em `outline`. Mecanismo + mudança de corpo real distingue este caso do padrão "duas
funções nascidas no mesmo lote sem se tocarem", que não é sinal de fronteira.

> Antes de 2026-08-13 este nó chamava-se `sectioning` e incluía `heading`, `title` e
> `divider`. Esses três estavam lá sustentados por clusters que não existiam (artefacto de
> atribuição de fronteira) e saíram para nós próprios; o núcleo medido é este.

---

## Contexto

`outline` produz um sumário navegável do documento. `lof` e `lot` são a mesma coisa com
`OutlineTarget` diferente — é isso, e só isso, que as mantém no mesmo nó: mudar o conjunto
de targets muda as três ao mesmo tempo.

O título por omissão (`"Índice"`) e a resolução das entradas são do layout; este nó só
constrói o elemento.

## Instrução

### `outline(title: ?, depth: ?, indent: ?, target: ?)`

Título por **named `title`** ou por **1.º posicional** — os dois ao mesmo tempo são erro
(`outline(): não pode usar título posicional e named 'title' simultaneamente`).

| Nomeado | Tipo aceite | Default | Nota |
|---|---|---|---|
| `title` | `Content \| Str \| none` | `None` | o layout renderiza "Índice" quando ausente |
| `depth` | `Int` ≥ 1 | `3` | `0` → `outline(depth:): depth deve ser >= 1` |
| `indent` | `Bool \| Length \| Func \| auto \| none` | `OutlineIndent::Auto` | `bool`, medida fixa, ou função de indentação |
| `target` | `"headings"\|"heading"` · `"figures"\|"figure"` · `"tables"\|"table"` | `Headings` | valor fora da lista → erro que enumera os três aceites |

Emite `Content::Outline(OutlineElem::with_target(title, depth, indent, target))`.

### `lof(title: ?)` / `lot(title: ?)`

Só `title` (`Content | Str | none`); **zero** posicionais. Emitem `Content::lof(title)` /
`Content::lot(title)` — equivalentes a `outline(target: "figures")` e
`outline(target: "tables")`. Tipo errado → `{fn}(title:): espera content ou string,
recebeu {tipo}`.

## Restrições Estruturais

- L1 puro. `_ctx`/`_world`/`_current_file` são pass-through nas três — nunca usados.
- `lof`/`lot` **não duplicam** a lógica de `outline`: delegam nos construtores de
  `Content` com o target fixo. Se um passo futuro lhes acrescentar `depth`/`indent`, faz-se
  por delegação em `outline`, não por cópia do parser.
- O default de `title` é `None`, não a string "Índice": quem materializa o texto é o
  layout. Assar o título aqui seria mudança de fase (ADR-0127).

## Critérios de Verificação

```
#outline()                          → Outline{title:None, depth:3, indent:Auto, target:Headings}
#outline[Sumário]                   → título posicional
#outline(title: "S")                → título named
#outline[S](title: "T")             → Err (posicional + named)
#outline(depth: 2)                  → depth 2
#outline(depth: 0)                  → Err "depth deve ser >= 1"
#outline(depth: "x")                → Err "espera int, recebeu string"
#outline(indent: false)             → OutlineIndent::Bool(false)
#outline(indent: 1em)               → OutlineIndent::Length
#outline(target: "figures")         → target Figures
#outline(target: "xpto")            → Err enumerando headings/figures/tables
#lof() / #lot()                     → Outline com target Figures / Tables
#lof(title: "Figuras")              → título named
#lof(title: 1)                      → Err "lof(title:): espera content ou string, recebeu integer"
```

## P1284 — gate `outline.entry`; este consumer permanece inalterado

### Medição antes da decisão

O vanilla expõe `outline.entry` como função-elemento com scope próprio e cinco
subfields (`body`, `inner`, `page`, `prefix`, `indented`). O cristalino possui
somente `OutlineElem`/`Content::Outline`; não possui `OutlineEntry` nem variante
`Content` equivalente. Além disso, os defaults P1284 medidos para `outline`
não coincidem com os defaults históricos acima.

### Decisão

`outline.entry` e seus cinco filhos são
`BLOCKED_ADR0127_PUBLIC_CONTRACT`. Este L0 não autoriza editar
`structural/outline.rs`, criar dict/module/stub, nem reutilizar `OutlineElem`
como entry. Filhos permanecem `Unknown(blocked_by_ancestor)`.

Alterar `outline` para `target:heading`, `depth:none` ou qualquer default
medido no residual também exige decisão humana por comportamento default e
compatibilidade. Até esse gate, P1284 preserva integralmente as assinaturas e
defaults vigentes deste owner.

## P1402 — proposta pública `outline.entry` e `body()` (gate ADR-0127 pendente)

### Medição antes da decisão

No baseline `be15d0ab73af089860d03db390508edf21a4bdac`, working tree limpa,
medido em `2026-09-14T22:02:33-03:00`, a sonda limitada descrita no Passo 1402
removeu a antiga incerteza de construção de P1400/P1284. Com uma fixture que
contém `#outline()` e um heading, o vanila ratificado `586e1bd43` expõe:

- `repr(outline.entry)` → `entry`;
- `repr(outline.entry.body)` → `body`;
- `query(outline.entry).first()` → uma ocorrência `entry` com `level`,
  `element` e `fill` realizados;
- `query(outline.entry).first().body()` → o Content do heading;
- `outline.entry(1, heading[H])` → construção pública válida.

Sem outline a consulta retorna zero; dois outlines sobre o mesmo heading geram
duas ocorrências. `outline.entry(1, strong[H]).body()` falha com `cannot outline
strong`. A fonte ratificada declara o scope em `model/outline.rs:246-250`, os
campos em `:468-522` e o método em `:668-674`. No consumer cristalino não existe
scope, construtor ou entry. Esta medição refuta `Unknown(blocked_by_ancestor)` e
classifica a mudança como contrato público novo.

### Contrato público proposto

O scope público de `outline` passa a conter a função-elemento `entry`. Ela é
selecionável por `query(outline.entry)`, tem nome/repr público `entry` e aceita:

```typ
outline.entry(level, element, fill: repeat(body: [.], gap: 0.15em))
```

- `level` é inteiro posicional obrigatório, 1-based e maior que zero;
- `element` é Content posicional obrigatório;
- `fill` é Content opcional ou `none`; quando omitido, usa o preenchimento
  linguístico equivalente a `repeat(body: [.], gap: 0.15em)`;
- os campos públicos observáveis da ocorrência são `level`, `element` e `fill`;
- a função pública `outline.entry.body` é chamável como método `entry.body()` e
  devolve o corpo semântico do elemento outlinable;
- elemento que não implementa a capacidade linguística outlinable produz erro
  explícito `cannot outline {nome}`, sem coerção para texto ou vazio.

Cada realização de `outline()` com target headings produz uma ocorrência por
elemento incluído, depois dos filtros `depth` e `outlined`, na ordem documental.
Sem outline não há ocorrência; dois outlines podem produzir duas entries para o
mesmo elemento fonte. O `element` preserva sua localização e campos realizados.

Esta cláusula não autoriza `indented`, `inner`, `page` ou `prefix`, nem altera os
defaults ou assinaturas existentes de `outline`, `lof` e `lot`. Também não
autoriza mover a geração para outra fase do pipeline: a materialização deve usar
o ponto de realização já vigente do outline e ser nucleada nos owners internos
afetados antes de código.

### Gate

Esta proposta substitui o bloqueio de P1284 somente após confirmação humana
explícita, conforme ADR-0127. Até a confirmação, o consumer permanece inalterado
e este texto é especificação proposta, não autorização para L1–L4.

## P1403 — proposta pública `prefix()` e `page()`

### Medição antes da decisão

No baseline cristalino limpo
`b7804e4daa9d3dd823dba5d1b828e60c845cc515`, medido em
`2026-09-14T22:50:19-03:00`, o vanila ratificado expõe as identidades
`outline.entry.prefix` e `outline.entry.page`. Sobre a entry consultada do
documento P1402 numerado, elas devolvem respectivamente `[1.]` e `[1]`. Um
heading sem numbering devolve `none` em `prefix`; page numbering `"I"` produz
`[I]`. Fonte: `model/outline.rs:629-667,676-693`. O cristalino não contém as
duas identidades. A rejeição atual de `#show outline.entry` acontece antes da
chamada e permanece lacuna separada.

### Contrato público proposto

O scope de `outline.entry` ganha as funções contextuais estáticas `prefix` e
`page`, ambas sem argumentos:

- `prefix()` resolve a capacidade outlinable do `element`; sem numbering
  retorna `none`; com numbering e Location preservada, formata o contador do
  elemento nessa Location e devolve Content, mantendo o prefixo específico do
  tipo;
- `page()` exige a Location do `element`, consulta a página e seu numbering no
  introspector contextual, usa padrão decimal `"1"` quando não há numbering e
  devolve Content formatado;
- Location ausente produz erro explícito e dica de customização por show rule;
- nenhum caminho usa página corrente, contador flat, texto pré-assado ou
  coerção de Content para string.

Esta materialização continua incompleta de forma declarada: P1404 completa
`inner`, P1405 completa `indented` e P1406 faz `#show outline.entry` alcançar
as entries geradas. P1403 não autoriza essas três parcelas nem muda a fase de
realização do outline.

### Gate

As duas funções ampliam o contrato público. Confirmação humana explícita
recebida em 2026-09-14; o contrato acima está autorizado para L1–L4 conforme
ADR-0127.

## P1404 — proposta pública `inner()`

### Medição antes da decisão

No baseline cristalino limpo
`efd82c1eb08bf40c4849df53bdca8c763cecdaa4`, medido em
`2026-09-14T23:18:11-03:00`, o vanila ratificado expõe
`outline.entry.inner` e, para a entry numerada P1402, devolve Content com a
morfologia `body, space, box(width: 1fr, fill), space, U+2060, page`. Com
`fill:none`, o trecho central é `h(1fr)` sem espaços laterais. Body forte e
fill customizado permanecem Content. Fonte:
`model/outline.rs:629-746`. O cristalino rejeita `inner` como método ausente.

### Contrato público proposto

O scope de `outline.entry` ganha a função contextual estática `inner`, sem
argumentos e chamável como `entry.inner()`:

- resolve `body()` e `page()` pelos helpers já proprietários; erros de elemento
  não outlinable e Location ausente são preservados;
- com fill, devolve sequência de body, espaço, caixa inline `1fr` contendo o
  fill, espaço, word joiner U+2060 e página;
- sem fill, devolve body, espaço horizontal `1fr`, U+2060 e página;
- em direção RTL efetiva, isola somente o body com U+202B/U+202C;
- body, fill e página permanecem Content, nunca string pré-assada;
- o suporte de largura fracionária necessário à caixa é carrier interno desta
  construção e não autoriza aceitar `box(width: 1fr)` na função pública geral.

P1404 permanece incompleto por desenho: P1405 completa `indented` e P1406 faz
`#show outline.entry` alcançar as entries geradas. Não há alegação sobre essas
parcelas nem sobre paridade geral.

### Gate

`inner` amplia o contrato público. Confirmação humana explícita recebida em
2026-09-14; o contrato acima está autorizado para L1–L4 conforme ADR-0127.

## P1405 — proposta pública `indented()`

### Medição antes da decisão

No baseline cristalino limpo
`a159b5f18ad42eab12393b4dd101c0d6f9c1b542`, medido a partir de
`2026-09-14T23:46:34-03:00`, o vanila ratificado expõe
`outline.entry.indented(prefix, inner, gap: 0.5em)`. A medição limitada de dois
níveis confirmou: auto alinha máximos de prefixo entre entries e usa fallback
`1.2em`; indent fixo multiplica `level − 1`; callback recebe `level − 1`;
prefixo ausente elimina o hanging; RTL usa o lado lógico direito. O body com
prefixo preserva marker semântico PDF e, somente em auto, informação interna
para a introspecção dos máximos. Fonte:
`model/outline.rs:524-628,798-833`. O cristalino não possui a identidade nem o
método; `#show outline.entry` continua bloqueado separadamente até P1406.

### Contrato público proposto

O scope de `outline.entry` ganha a função contextual estática `indented`,
chamável como:

```typ
entry.indented(prefix, inner, gap: 0.5em)
```

- `prefix` é Content ou `none`; `inner` é Content; `gap` é Length;
- contexto desconhecido falha antes da execução; contexto sem parent outline
  falha com `must be called within the context of an outline`;
- a largura do prefixo é medida com métricas/estilos efetivos e somada a gap;
- indent fixo usa `(level − 1) × indent`; função é chamada uma vez com
  `level − 1`; por compatibilidade cristalina, `true` equivale a auto e
  `false` a zero fixo;
- auto soma máximos por nível anterior, com fallback `1.2em`, e alinha o
  hanging atual ao máximo do nível;
- com prefixo, o body contém informação de alinhamento apenas em auto,
  `h(-hanging)`, prefixo marcado como label PDF,
  `h(hanging-prefix_width)` e inner; sem prefixo contém somente inner;
- o Block resultante aplica `base + hanging` ao lado lógico inicial: esquerda
  em LTR, direita em RTL;
- prefixo e inner permanecem Content e medidas observadas não são assadas.

P1406 continua sendo o único passo autorizado a aceitar `outline.entry` como
selector de show rule, transportar o parent e integrar o show default. P1405
não altera o layout legado do outline nem a fase do pipeline.

### Gate

`indented` amplia o contrato público e introduz novo comportamento contextual.
Confirmação humana explícita recebida em 2026-09-15; o contrato acima está
autorizado para L1–L4 conforme ADR-0127.

### Transporte interno autorizado

O parent não é dado da entry: uma entry direta ou recuperada por `query`
continua sem parent e deve falhar. P1406 fornecerá `OutlineIndent` e uma chave
opaca do outline por propriedades privadas na cadeia de estilos durante a
realização do show rule. P1405 pode consumir essas propriedades, mas não as
fabricar. O cálculo auto lê somente `prefix-info` introspectado com a mesma
chave e mede o prefixo atual com a `FontMetrics` injetada.

O marker `prefix-info` é um `Content::Styled` vazio com propriedade custom
privada, portanto não cria texto nem superfície pública. O prefixo marcado para
PDF usa igualmente um carrier `Styled` privado; o layout converte este carrier
em `SemanticKind::Label`, preservando o Content visual dentro do envelope.

## P1406 — seletor de show e snapshot lexical (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

No baseline `fe3209458abaa9177f41bbf4795755916c277ae8`, a declaração direta
`#show outline.entry` é rejeitada e a forma `outline.entry.where(level: 2)` é
aceita sem efeito. No vanila ratificado `586e1bd43`, ambas alcançam as entries
geradas. Uma regra colocada entre dois outlines transforma somente o segundo.
A fonte medida em `model/outline.rs:272-318,397-407` gera entries após
introspecção e injeta o parent durante a realização.

### Contrato proposto

`native_outline` captura, em carrier privado, o snapshot lexical necessário
para realizar as show rules vigentes naquela ocorrência. `outline.entry` passa
a ser aceito como seletor nativo direto, equivalente à base de
`outline.entry.where(...)`. O carrier nunca é campo público Typst e não torna
regra posterior retroativa. A realização fornece temporariamente
`outline.parent.indent` e `outline.parent.key`; não armazena parent na entry.

O default para headings compõe, como Content, o link à Location causal ao redor
de `entry.indented(entry.prefix(), entry.inner())`. A closure nunca é executada
no layouter. Figuras, tabelas, equações e wording exato do alt PDF permanecem
fora deste passo.

Confirmação humana explícita recebida em 2026-09-15; esta ampliação pública e a
mudança de fase estão autorizadas para código produtivo e resselo.

## P1412 — presença do argumento `indent`

### Medição anterior à decisão

No baseline `2ff1a23d4`, `native_outline` conhece pelo mapa de argumentos se
`indent` foi omitido, mas descartava essa informação ao construir
`OutlineElem`. Sem ela, `auto` default e `auto` explícito são indistinguíveis e
uma set rule não pode respeitar precedência.

### Decisão

`native_outline` passa ao carrier privado do elemento a presença exata do
argumento named `indent`. O parsing, os tipos aceitos e a assinatura pública
Typst permanecem inalterados. Aliases `lof`/`lot` mantêm seus defaults próprios
e não participam da regra `#set outline` deste passo.


## Aridade das fronteiras — `clippy::too_many_arguments`

### Evidência anterior à decisão

A leitura estrutural abaixo usa HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado,
capturado em 2026-10-15T16:39:18.873477+00:00. Fontes completas, L0 anteriores, assinaturas,
hashes, UTC, status e `git diff HEAD --stat` estão em
`00_nucleo/diagnosticos/too-many-arguments-2026-10-15/before.json.gz`.
A classificação individual e as cláusulas deste L0 lidas antes da decisão estão em
`00_nucleo/diagnosticos/too-many-arguments-2026-10-15/classification.json`.
Os spans Clippy foram reconfirmados contra as fontes do inventário anterior.
É medição de estrutura Rust; não é nova prova de paridade ou de falha funcional.

- `C021`: `01_core/src/compiler/stdlib/structural/outline.rs:340`, `compose_outline_entry_indented`, 9 argumentos contados pelo Clippy (limiar 7).

### Prescrição exclusiva por função

As correções abaixo são alvos internos de manutenção, ainda não materializados nesta auditoria.
Preservar as assinaturas públicas vigentes e as decisões de linguagem deste owner.
A necessidade de corrigir estrutura é julgamento de manutenção apoiado nos dados
explicitados; o aviso de aridade, sozinho, não demonstra um defeito funcional.

#### `C021` — `compose_outline_entry_indented`: Correção interna prescrita

O chamador já calcula um plano de indentação (base/hanging/auto/chave/nível/direção) antes de compor o conteúdo. Sete valores desse plano são transportados como posicionais separados; nomear o plano privado preserva presença de prefixo e torna a operação verificável.

Plano privado de indentação com prefix_width_pt, base, hanging_pt, auto, parent_key, level e rtl; compositor recebe prefix, inner e o plano. Preservar exatamente as combinações Option e fallback atuais.

Aceitação: Sem prefixo, auto/fixo/callback, múltiplos níveis, parent_key e RTL mantêm conteúdo, padding e markers; não reexecutar callback nem recalcular métricas.

### Fronteira de materialização

Nesta auditoria, somente o L0 e seu selo de linhagem são atualizados; atributos de
supressão e mudanças de assinatura/corpo ficam pendentes da etapa de materialização.
As entradas com correção exigem testes de linguagem/integração do owner e controle
de chamadas antes/depois, sem relaxar fixtures ou tolerâncias. Manter as camadas,
a fase, os defaults, o despacho estático e a ausência de estado global mutável.
Correções de assinatura privada devem migrar também os chamadores e seus L0 antes
do código. Qualquer necessidade nova de contrato público ou compatibilidade externa
exige proposta concreta e confirmação ADR-0127; esta seção não a antecipa.


## Aridade final — materialização interna, 2026-10-16

### Medição anterior à decisão

Fontes e L0 vigentes conferidos antes do código; snapshot 00_nucleo/diagnosticos/clippy-aridade-final-2026-10-16/before.json.gz conserva HEAD, working tree não commitado, UTC, status, diffstat e hashes globais. callers.txt registra busca no workspace, incluindo fixtures; as chamadas homônimas do math::MathLayouter são outro motor e permanecem intactas. Inventário anterior tem 22 avisos, cinco destas aridades. Trata-se de estrutura privada Rust, sem nova afirmação de paridade ou intenção upstream.

### Responsabilidade exclusiva e aceitação

Concretizar C021 com OutlineIndentPlan privado: prefix_width_pt Option<f64>, base Length, hanging_pt Option<f64>, auto bool, parent_key i64, level usize e rtl bool. Compositor recebe prefix, inner e plano, desempacota uma vez e mantém o corpo. Migrar chamada produtiva e duas fixtures pelos mesmos valores nomeados, sem alterar asserts, None/Some, callback, métricas ou markers.

Este adendo materializa somente o escopo nominal e supersede seu estado pendente da auditoria. Fluxo contínuo ADR-0127: sem contrato externo, default, fase ou compatibilidade novos. L0 primeiro e resselo restrito. RED de too_many_arguments antes do código, GREEN sem novas supressões ou alteração de limiar; expectativas anteriores devem continuar satisfeitas. Conferir corpos e transporte de valores, preservar fixtures/esperados, sem testes que espelhem carriers. Executar controles existentes de Grid/Table/outline e SVG/tiling antes/depois, suíte workspace completa, build, Clippy all-targets, Rustdoc, formato, linhagem estrita/reversa e Tekt sem novos avisos, sob 12 GiB, swap zero e um job. Resultados não certificam paridade global ou conformidade PDF/UA.


## Tamanho dos enums — decisão medida 2026-10-16

### Medição anterior à decisão

00_nucleo/diagnosticos/clippy-enums-2026-10-16/before.json.gz conserva fontes/L0, hashes globais, HEAD, working tree não commitado, UTC e diffstat; callers.txt registra usos no workspace. probe-details.json e measurements.json guardam rustc 1.92/x86_64, comandos/binário/rlib e tamanhos/alocações de candidatos isolados antes de código. O enum privado é espelhado fielmente da fonte; os tipos públicos são medidos diretamente do rlib. O observador unsafe existe somente no executável diagnóstico, nunca na camada produtiva. É medição de representação Rust, não igualdade de linguagem, intenção upstream ou prova de ganho global.

### Responsabilidade exclusiva e aceitação

Migrar somente a captura ao final de native_outline_entry_indented para ContextReadRequest::outline_entry_indented, mantendo os quatro argumentos na mesma ordem/expressões: Content::OutlineEntry(Arc::new(entry)), args.items[1] clonado ou None, args.items[2] clonado ou None e gap. Não calcular medidas, prefixos, callback ou resultado novamente; span/Ok(result)/observe_context_read continuam. Constructor privado do request é definido por 00_nucleo/prompts/compiler/eval/context_reads.md, com mesma posse dos valores e sem nova API externa. Todo o parsing, cálculo de base/hanging/RTL, observação, erros e output anterior fica igual.

Fluxo contínuo ADR-0127: esta etapa altera somente representação/constructor privados ao crate e metadados nominais; os contratos externos dos outcomes e entidades permanecem. L0 antes do código, resselo restrito. Gate de tamanho privado RED→GREEN e Clippy large_enum_variant deve confirmar três spans selecionados antes e resolver um com código e dois com expectativas satisfeitas. Não elevar limiar ou silenciar módulo/impl/crate; não estender expectativas a Tag/Transformation ou ao request privado. Preservar fixtures/esperados e testar os controles existentes de outline (incluindo oito casos P1405), contexto/replay, callbacks e pipeline. Validar workspace completo/build/Clippy/expectativas/Rustdoc/fmt/linhagem, sem avisos Tekt novos, sob 12 GiB, swap zero e um job. Tag/Transformation continuam medidos, sem alteração pública autorizada neste pacote.
