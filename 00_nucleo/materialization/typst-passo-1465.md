# Passo 1465 — materializar `format.html` e transportar `pretty` até o export HTML

**Estado:** L0 CAUSAL ESCRITO — AGUARDA CONFIRMAÇÃO HUMANA PÓS-COMMIT
(ADR-0127)
**Data da medição:** 2026-09-20T10:54:00-03:00
**Baseline cristalino:** `8e74134a721310cc4fb2a596a195398e16f1d5db`
**Tree do baseline:** `9e34e0233998ca60ad86145ab3ad8771e67a8726`, limpa
(`git status --short`, `git diff` e `git diff HEAD --stat` sem saída)
**Vanilla ratificado:** `586e1bd43fae6c9a973218163d3165c53ab8d16d`
**SHA-256 do binário vanilla medido:**
`eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`

## 1. Objetivo e fronteira

P1465 completa o quarto filho reservado da raiz pública `format`:

```text
format.html                         somente com feature html
├── pretty: bool = false
├── elem / frame                    mesmas funções que html.elem / html.frame
└── 112 constructors tipados       mesmas funções que html.<tag>
```

`format.html` é função/elemento settable, não módulo. A chamada manual falha
com `cannot be constructed manually`; seu namespace tem `pretty` e os 114
membros funcionais já produzidos pelo módulo global `html`. O passo reutiliza
esses valores, sem copiar o catálogo, criar wrappers ou reimplementar HTML.

O passo também materializa a opção documental:

```text
--pretty explícito > format.html.pretty do documento > false
```

Isso inclui validação de `#set format.html(pretty: ...)`, neutralidade da regra
vazia, rejeição em containers, extração pura e resolução antes do encoder. O
CLI e o encoder HTML já têm, respectivamente, o tri-state e o booleano final;
permanecem inalterados.

P1465 não cria novo target, tag HTML, encoder, serializer, carrier de DOM,
feature ou flag. Não fecha paridade HTML geral nem altera `format.pdf`,
`format.png`, `format.svg`, bundle, acessibilidade PDF ou as opções PDF
reservadas para P1466–P1468.

## 2. Medição anterior à decisão

### 2.1 Superfície e gate no vanilla ratificado

Com `--features html`, o binário ratificado mediu:

```text
(repr(type(format.html)), repr(format.html), format.html.pretty,
 format.html == std.format.html,
 format.html.elem == html.elem,
 format.html.frame == html.frame,
 format.html.div == html.div)
=> (function, html, false, true, true, true, true)

repr((format.html.elem, format.html.frame, format.html.div))
=> (elem, frame, div)

(format.html.with(pretty: true).elem == html.elem,
 format.html.with(pretty: true).frame == html.frame,
 format.html.with(pretty: true).div == html.div)
=> (true, true, true)

format.html()
=> erro: cannot be constructed manually
```

A enumeração da fonte cristalina vigente conta 112 entradas tipadas. Um probe
gerado a partir desses nomes comparou `elem`, `frame` e as 112 entradas entre
os dois caminhos e devolveu 114 valores `true`. Na fonte vanilla,
`typst-html/src/format.rs:130-158` define `pretty`, proíbe construção manual e
instala `HtmlElem`, `FrameElem` e `crate::typed::define(scope)` no mesmo scope
do elemento de formato.

Sem `html`, tanto no perfil default quanto em `a11y-extras`, `format.html`
termina com exit 1, span somente em `html`, e:

```text
cannot access field `html` because the `html` feature is not enabled
hint: try enabling the `html` feature
hint: see https://typst.app/help/compiler-features for more details
```

`std.format.html` produz a mesma mensagem e os mesmos hints. Selecionar target
HTML, ativar `a11y-extras` ou acessar outro formato não ativa `html`.

### 2.2 Set rule e ordem observável

Com a feature ativa, o vanilla aceita zero ou um named `pretty: bool`. Os
negativos medidos são:

| Entrada | Resultado público |
|---|---|
| `#set format.html(nope: true)` | `unexpected argument: nope` |
| `#set format.html(pretty: 1)` | `expected boolean, found integer` |
| `#set format.html(true)` | `unexpected argument` |
| named `pretty` duplicado | `duplicate argument: pretty` |

Uma expressão de argumento é avaliada antes da validação ordinária do schema:
com feature ativa, `nope: panic("arg")` propaga o panic. Com feature ausente,
o gate de `format.html` acontece antes de avaliar qualquer argumento, inclusive
named desconhecido ou positional com `panic`. Essa ordem é observável e não
pode ser trocada por validação textual antecipada.

`#set format.html()` não escreve `false`: depois de `pretty: true`, a regra
vazia preserva a decisão anterior. Repetições não vazias seguem a cascata
lexical, com a última decisão root efetiva.

Dentro de `block`, a compilação HTML falha com:

```text
format set rules are not allowed inside of containers
```

O mesmo collector já aplica essa fronteira a SVG/PNG; P1465 acrescenta a chave
HTML sem criar uma segunda definição de root/container.

### 2.3 Efeito e precedência no export

Sobre `#html.div[Hello]`, a saída vanilla mediu:

| Documento | CLI | Resultado |
|---|---|---|
| sem regra | ausente | HTML compacto |
| `pretty: true` | ausente | HTML indentado |
| `pretty: true` | `--pretty=false` | HTML compacto |
| `pretty: false` | `--pretty=true` | HTML indentado |

Compacto e pretty preservam o mesmo DOM e escaping; diferem somente no
whitespace estrutural já contratado pelo encoder HTML. A decisão efetiva
também controla o SVG inline de `html.frame`, porque a pipeline vigente entrega
o mesmo booleano final ao encoder externo e ao resolvedor local do frame.

### 2.4 Estado cristalino causal

No baseline:

- `compiler/stdlib/format.rs:36-98` recebe somente a projeção PDF e constrói
  `svg`, `png` e `pdf`; `html` está ausente;
- `compiler/stdlib/html.rs:34-42,298-412` constrói uma vez o módulo global com
  `elem`, `frame` e 112 tags tipadas;
- `compiler/eval/mod.rs:7772-7775` constrói `pdf` uma vez, mas constrói `html`
  separadamente depois de `format`;
- `compiler/eval/bindings/field_access.rs:500-563` possui somente o intercept
  gated de `a11y-extras`; o gate bare de `html` está no lookup de identificador
  e não cobre o field `format.html`;
- `compiler/eval/rules.rs:1412-1505` reconhece somente `format.svg` e
  `format.png`;
- `compiler/layout/format_options.rs:14-125` coleta somente as chaves SVG/PNG;
- `DocumentFormatOptions` contém somente `svg` e `png`;
- `pipeline.rs:233-300` recebe `pretty: bool` no HTML e não consulta opção do
  documento;
- `wiring/main.rs:651-652,754-770` colapsa `None` em `false` antes do braço
  HTML, embora preserve o `Option<bool>` original para SVG.

Hashes do código lido:

| Consumer/dependência | SHA-256 |
|---|---|
| `01_core/src/compiler/stdlib/format.rs` | `d8d435775db40c81ac1d5e87411bfa22055585f111c206c7d1e0dd2552177c1a` |
| `01_core/src/compiler/stdlib/html.rs` | `6a1085e58997912e7c4fba055307b3f86ed63a20b963f6796c739310aee69632` |
| `01_core/src/compiler/eval/mod.rs` | `cccd451060ba34f9d9ebee33d999fcbd066fa666447bc6dab7e2e13feb6c6210` |
| `01_core/src/compiler/eval/bindings/field_access.rs` | `79ed0a3dc496c7c21fb16ed59a3cfe17a04142990b553370d13f80764b06669c` |
| `01_core/src/compiler/eval/rules.rs` | `c490e7f10c47b0652c9d54f97799eb7b6b67b3866a7bf7338ff433d62066fc03` |
| `01_core/src/compiler/layout/format_options.rs` | `4b767d084d02f511c7b8f5157cbfbbfc2a722a3fb028008a188534ce36d90df1` |
| `01_core/src/entities/document_format_options.rs` | `98ccc96be7b6d532badd840b6caf68bbc9b5074cedd6b42221e07e25e4a9009a` |
| `03_infra/src/pipeline.rs` | `82955944eb48d9156b9da41b8958298f543ad245efc105f1fbd182c3fd756062` |
| `04_wiring/src/main.rs` | `f9468404c258f9166c3624f1d524fac99d8259ed9afe2a79069d2dc3f494ad8d` |

Fonte upstream usada para decidir:

| Fonte | SHA-256 |
|---|---|
| `lab/typst-original/crates/typst-library/src/format.rs` | `37ce18defae73642aa9e6f74cb8a78dffba575b004d9a8ddfbf52cb272898a1f` |
| `lab/typst-original/crates/typst-html/src/format.rs` | `366ea4aa68932ecf95c0d81863b3ca2905fe3ae48164b2029099d95905e8ce7b` |

Não havia binário cristalino atual no baseline; por isso nenhuma saída antiga
de `target/` foi tratada como prova.

## 3. Classificação ADR-0107/0108

São linguagem pública: existência/tipo/nome/default de `format.html`, gate e
diagnóstico, 114 membros e sua igualdade com `html.*`, chamada proibida,
validação/ordem de argumentos, efeito root, rejeição nested, cascata e
precedência CLI/documento/default. Compacto versus pretty é morfologia pública
quanto ao whitespace estrutural, não igualdade de bytes.

São mecânica livre: clone de `Value`, endereço de `Arc`, ordem interna do
`Scope`, helper usado para reconhecer o módulo, número de travessias e nome da
API interna, desde que os observáveis permaneçam.

É inferência que o módulo `html` vigente é a fonte única suficiente para os 114
membros e que o collector SVG/PNG pode incorporar uma terceira chave. Refutam
a inferência: membro global sem projeção, identidade linguística perdida,
`Content::Styled` removido antes da coleta, necessidade de nova fase ou opção
que não possa ser representada pelo carrier atual. Um refutador exige novo L0
antes de ampliar código.

## 4. Decisão arquitetural causal

1. `make_stdlib_with_features` constrói `make_html_module()` exatamente uma
   vez, entrega por referência esse valor e o PDF efetivo a
   `make_format_module`, e depois registra o mesmo módulo global `html`.
2. A factory `format` acrescenta uma função nativa `html`, não construtível,
   com `pretty: false` e clones de todos os bindings do scope HTML recebido.
   Ausência, duplicação ou tipo não funcional em membro recebido é erro de
   composição; não existe catálogo paralelo.
3. O field access reconhece somente o módulo `format` efetivo e intercepta o
   field `html` quando a feature está desligada, antes do lookup, com mensagem,
   hints e span medidos. Outros fields e módulos homônimos não recebem o gate.
4. `eval_set_rule` reconhece estruturalmente `format.html`. Primeiro aplica o
   gate; com a feature ativa avalia argumentos e então valida exatamente o
   named opcional `pretty: bool`. Quando presente grava
   `format.html.pretty`; quando ausente não grava chave.
5. `DocumentFormatOptions` acrescenta
   `HtmlFormatOptions { pub pretty: bool }`, default `false`; o campo
   documental é booleano resolvido, nunca `Option<bool>`.
6. O collector vigente reconhece `format.html.pretty` nas travessias root e
   exaustiva, preserva last-write-wins e a mesma rejeição de containers, sem
   misturar SVG, PNG ou HTML.
7. A pipeline acrescenta uma entry point HTML com
   `pretty_override: Option<bool>`. As fachadas sem pretty delegam `None`; a
   fachada de compatibilidade `pretty: bool` delega `Some(pretty)`. Depois do
   eval e antes da preparação HTML, coleta as opções e resolve:

   ```text
   pretty_override.unwrap_or(options.html.pretty)
   ```

   O booleano final único alimenta o encoder HTML e o SVG de `html.frame`.
8. L4 preserva o `Option<bool>` original somente até o braço HTML e chama a
   nova entry point. PDF mantém o comportamento vigente até P1466; SVG já usa
   seu próprio override; PNG continua alheio a pretty.

O Núcleo `compiler-feature-gates` permanece sem alteração, com pin efetivo
`59d8938dc06d347ccc9db23ae1b740876b369227daacd266a219811a661b3cb9`.
O Núcleo `format-option-precedence` permanece sem alteração, com pin efetivo
`68c5c10f124c2c098879318df16c59f6c1477e9172cfa334b603f45ec698ae11`.
Código nunca aponta diretamente para Núcleo.

## 5. Ownership 1:1 antes do código

| Prompt L0 | Consumer exclusivo |
|---|---|
| `compiler/stdlib/format.md` | `01_core/src/compiler/stdlib/format.rs` |
| `compiler/eval.md` | `01_core/src/compiler/eval/mod.rs` |
| `compiler/eval/bindings/field_access.md` | `01_core/src/compiler/eval/bindings/field_access.rs` |
| `compiler/eval/rules.md` | `01_core/src/compiler/eval/rules.rs` |
| `compiler/layout/format_options.md` | `01_core/src/compiler/layout/format_options.rs` |
| `entities/document_format_options.md` | `01_core/src/entities/document_format_options.rs` |
| `infra/pipeline.md` | `03_infra/src/pipeline.rs` |
| `wiring.md` | `04_wiring/src/main.rs` |
| `wiring/tests/p1465-format-html.md` | `04_wiring/tests/p1465_format_html.rs` |

`compiler/stdlib/html.md`, `shell/cli.md`, `infra/export/html.md` e
`infra/export/mod.md` permanecem inalterados: já fornecem os 114 membros, o
tri-state CLI e a serialização compacta/pretty. Alterá-los sem refutador seria
duplicar responsabilidade.

## 6. Gate ADR-0127

P1465 adiciona função e campos públicos, set rule, carrier público, API L3 e
muda o default efetivo do export HTML quando `--pretty` está ausente. Também
amplia a extração de opções entre eval e export. A paragem é obrigatória:

1. escrever este passo e os nove L0s da seção 5;
2. validar V15/V26 e commitar somente passo + L0;
3. **parar para confirmação humana posterior ao commit causal**;
4. somente depois criar teste RED, alterar código e ressellar linhagem.

A confirmação de P1464 não autoriza P1465.

## 7. Testes após confirmação

O teste funcional independente deve cobrir os perfis `default`,
`a11y-extras`, `html` e `html+a11y`:

- gate exato de `format.html`/`std.format.html`, inclusive antes de `panic` em
  argumentos de set rule;
- tipo, nome, `pretty == false`, construção proibida, alias e `.with`;
- igualdade dos 114 membros com `html.*`, incluindo `elem`, `frame`, primeira,
  intermediária e última tag como sentinelas explícitas;
- regra vazia, true/false, repetição nas duas ordens, unknown/type/positional/
  duplicate e avaliação de argumentos;
- rejeição em ao menos dois containers e aceitação root;
- default compacto, regra documental pretty, dois overrides CLI e igualdade
  de DOM entre compacto/pretty;
- mesmo booleano efetivo no SVG inline de `html.frame`;
- neutralidade de `format.svg`, `format.png`, `format.pdf`, gates PDF, targets
  não HTML e modo de serialização HTML;
- nenhuma alegação de bundle, tags novas ou paridade HTML completa.

Os gates finais são:

```text
cargo test -p typst-core <testes focais dos owners P1465>
cargo test -p typst-infra <testes focais HTML/format_options>
cargo test -p typst-wiring --test p1465_format_html
cargo test --workspace
cargo clippy --workspace --all-targets --all-features
cargo build --workspace
crystalline-lint .
verificação de referências quebradas
git diff --check
```

Falhas históricas só podem ser preservadas quando reproduzidas no commit
causal com o mesmo nome e observável. Timeout, OOM, falta de espaço, binário
sem proveniência ou execução opaca são `Unknown` e bloqueiam conclusão.

## 8. RAM e temporários

Toda compilação, cache, target, fixture, log e temporário descartável usa uma
raiz exclusiva em `/dev/shm`, sem fallback para o HD. Antes de cada lote,
verificar capacidade e `MemAvailable`; executar serialmente os testes de
subprocesso; aplicar limite por processo e watchdog compatíveis com o teto de
32 GiB acordado. Se `/dev/shm` estiver indisponível, sem espaço, ou o consumo
ameaçar o teto, abortar o lote e reportar `Unknown` em vez de repetir a
alocação de aproximadamente 40,5 GB observada no passo anterior.

Nenhum manifesto administrativo, selo, recibo, certificado, catálogo de
mutantes ou cadeia de hashes deve ser criado. Artefatos versionados do passo:
este documento, L0s, código e testes funcionais.
