# Prompt L0 — `compiler/stdlib/format` — raiz pública de formatos
Hash do Código: c8e79d4b

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/compiler-feature-gates.toml sha256:59d8938dc06d347ccc9db23ae1b740876b369227daacd266a219811a661b3cb9

**Estado:** P1461/P1462/P1463/P1464 MATERIALIZADOS; P1465 CAUSAL — AGUARDA
CONFIRMAÇÃO ADR-0127.

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/stdlib/format.rs`
**Consumer exclusivo:** `01_core/src/compiler/stdlib/format.rs`
**Vanilla ratificado:** `586e1bd43fae6c9a973218163d3165c53ab8d16d`
**ADRs:** ADR-0107, ADR-0108, ADR-0127, ADR-0129

## Medição anterior à decisão

No baseline P1461, HEAD
`7392f490397c56876fcf6134e86c44b528b396bb`,
`01_core/src/compiler/eval/mod.rs:7594-7773` constrói o scope global e registra
diretamente o módulo `html`, mas não registra `format`.
`01_core/src/compiler/stdlib/mod.rs:23-69` não declara owner `format`.
`01_core/src/entities/module.rs` já oferece `Module::new(name, scope)`, nome,
scope, clone por `Arc` e a representação genérica de módulos; nenhuma nova
entidade é necessária.

Na fonte ratificada,
`lab/typst-original/crates/typst-library/src/format.rs:89-102` constrói um
módulo chamado `format` e só acrescenta os formatos recebidos, enquanto
`lab/typst-original/crates/typst-library/src/lib.rs:368-381` registra a raiz
independentemente de target e feature. Probes bilaterais congelados no Passo
1461 mediram `repr(type(format)) == "module"`,
`repr(format) == "<module format>"` e a ausência cristalina atual como
`unknown variable format` nos quatro perfis.

É inferência que `Module` e `Scope` vigentes bastam para esta raiz vazia. A
necessidade de alterar `entities/module.rs`, `Value`, lookup genérico, feature
gates ou a representação pública de módulos refuta a inferência e exige novo
desenho antes de código.

## Classificação de linguagem e mecânica

A existência da raiz, seu tipo `module`, o nome público `format`, sua
representação, identidade preservada por alias/`std` e o diagnóstico de field
ausente são semântica e morfologia públicas. O tipo concreto de coleção do
scope, endereço de `Arc`, ordem de alocação, vetor dinâmico upstream e
estrutura Rust são mecânica e não constituem paridade.

## Contrato P1461

O único consumer deste prompt define a factory interna:

```rust
pub(crate) fn make_format_module() -> Module
```

A factory devolve exatamente `Module::new("format", Scope::new())`. O módulo
tem nome `format` e scope vazio; não contém wrappers, placeholders, aliases,
stubs, valores marcadores, feature gates nem lógica de exportação. O owner não
faz I/O, não consulta target ou features e não altera `Module`, `Scope` ou
`Value`.

A fachada `compiler/stdlib/mod.rs` apenas declara o submódulo e reexporta a
factory em `pub(crate)` sob seu próprio L0. O owner `compiler/eval.md` registra
o valor produzido no scope global em todas as construções reais. Este arquivo
não legitima essas duas ligações e nenhum outro consumer produtivo pode apontar
para este prompt (ADR-0129).

## Incompletude deliberada do constructor

O módulo P1461 é deliberadamente incompleto. Seu scope vazio é o estado
contratado desta unidade, não uma alegação de fecho de `format.*`.

- P1462 completou `format.svg` e seu namespace;
- P1463 completou `format.png` e `ppi`;
- P1464 completou `format.pdf`, seus defaults de namespace e cinco
  membros reutilizados;
- P1465 completa `format.html`, `pretty`, gate `html` e seus membros.

`format.svg`, `format.png` e `format.pdf` estão materializados. Até P1465 ter
L0 e autorização próprios, `format.html` e seus descendentes permanecem
ausentes. É proibido criar filhos, aliases globais ou passthroughs para
antecipar a contagem dos passos seguintes.

## Aceitação linguística limitada

Nos perfis `default`, `a11y-extras`, `html` e `html+a11y`:

- `repr(type(format))` é `"module"`;
- `repr(format)` e `repr(std.format)` são `"<module format>"`;
- alias lexical conserva a identidade nominal e a representação;
- field ausente é diagnosticado como ausência no módulo nomeado `format`;
- a raiz existe independentemente de target e feature;
- nenhum filho `format.*` nem alias global novo surge neste passo.

A aceitação compara valor, representação e diagnóstico públicos, não endereço
de `Arc`, layout do scope, bytes de binário ou ordem de hash. `Unknown` por
baseline divergente, binário não pinado, timeout, parser opaco ou falha de
proveniência bloqueia; nunca é convertido em sucesso.

## Gate ADR-0127 de P1461

O gate humano de P1461 foi satisfeito e a raiz pública vazia foi materializada
e validada. Esse fato não autoriza qualquer filho reservado.

## P1462 — `format.svg` (MATERIALIZADO)

### Medição anterior à decisão

No baseline P1462, HEAD `7442d33b6bf48f136826d28cf71bd62f8710a432`,
o consumer devolve `Module::new("format", Scope::new())`; por isso o
cristalino diagnostica `module format does not contain svg`. No vanilla
ratificado, `lab/typst-original/crates/typst-svg/src/format.rs:65-104` expõe o
elemento settable `svg` com uma única propriedade `pretty: bool = false`.
Probes bilaterais mediram:

```text
repr((type(format.svg), format.svg, format.svg.pretty))
vanilla: (function, svg, false)
cristalino: module format does not contain svg
```

Chamar `format.svg()` no vanilla falha com `cannot be constructed manually`;
`#set format.svg(pretty: true)` é aceito, enquanto named desconhecido falha
com `unexpected argument: other` e inteiro falha com
`expected boolean, found integer`. Essas mensagens e valores são observáveis
da linguagem; macros, traits, downcast, endereço e representação Rust são
mecânica livre.

### Contrato do filho

A factory P1461 passa a inserir exatamente um membro chamado `svg`. O valor é
uma função/elemento settable nominalmente `svg`, com namespace público
`pretty == false`. Sua chamada direta é sempre rejeitada com
`cannot be constructed manually`; ela não produz `Content` construível.

O contrato desta unidade é somente a descoberta e identidade do filho. A
validação/aplicação da set rule pertence a
`00_nucleo/prompts/compiler/eval/rules.md`; a extração documental pertence a
`00_nucleo/prompts/compiler/layout/format_options.md`; exportação e
precedência pertencem aos owners L3/L4 próprios. Este consumer não faz I/O,
não consulta target, feature, CLI ou documento e não formata SVG.

P1462 não cria `format.png`, `format.pdf`, `format.html`, aliases globais,
campos adicionais nem feature gates. A incompletude restante continua
deliberada: P1463 completa PNG, P1464 PDF e P1465 HTML.

### Aceitação e gate

Nos quatro perfis públicos, `type(format.svg)` é `function`, `repr` é `svg`,
`format.svg.pretty` é `false`, alias e `std.format.svg` preservam a identidade,
e chamada direta produz o diagnóstico medido. Membro não contratado continua
ausente. A confirmação humana posterior ao commit causal P1462 foi recebida em
2026-09-19. O confirmação, candidato, ataques e veredito final foram concluídos no
mesmo dia; P1463 não reabre esse contrato.

## P1463 — `format.png` e namespace `ppi` (MATERIALIZADO)

### Medição anterior à decisão

No baseline `23cd47e4ade4d6deb7095cb7400478feffb1e9f9`, o módulo contém
somente `svg`; `format.png` falha com `module format does not contain png`.
No vanilla ratificado,
`lab/typst-original/crates/typst-render/src/format.rs:81-96` expõe o elemento
settable `png`, com namespace `ppi: float = 144.0`. O probe
`repr((type(format.png), format.png, format.png.ppi))` produz
`(function, png, 144.0)`, e `format.png()` falha com
`cannot be constructed manually`.

### Contrato do filho

A factory acrescenta exatamente um segundo membro chamado `png`. O valor é
uma função/elemento nominalmente `png`, não construtível, com namespace
público contendo somente `ppi == Value::Float(144.0)`. Alias lexical e
`std.format.png` preservam identidade, nome e namespace. O filho existe nos
quatro perfis e não depende de target, `html` ou `a11y-extras`.

Este owner legitima somente descoberta, identidade, namespace/default e
construção proibida. Validação da set rule pertence a
`00_nucleo/prompts/compiler/eval/rules.md`; extração a
`00_nucleo/prompts/compiler/layout/format_options.md`; resolução e transporte
aos owners L2–L4. Não fazer I/O, rasterizar, consultar documento ou converter
unidades aqui.

P1463 não cria `format.pdf`, `format.html`, aliases globais, campos além de
`ppi` nem feature gates. P1464 e P1465 continuam os donos reservados.

### Aceitação e gate

Exigir tipo, nome, default float, alias/`std`, construção proibida e ausência
dos siblings futuros nos quatro perfis. A confirmação humana posterior ao
commit causal P1463 foi recebida em 2026-09-19; código, testes e validação foram
materializados no mesmo dia. P1464 não reabre o transporte produtivo de `ppi`.

## P1464 — `format.pdf`, defaults e membros reutilizados (MATERIALIZADO)

### Medição anterior à decisão

No baseline `a1286f958f15a539dca17a3696ef83ef9e5b1b78`, este consumer cria
somente `svg` e `png`. O módulo global produzido por
`compiler/stdlib/pdf.rs::make_pdf_module` já contém `attach` e `artifact` em
todos os perfis e contém `table-summary`, `header-cell` e `data-cell` somente
com `a11y-extras`. O vanilla ratificado
`586e1bd43fae6c9a973218163d3165c53ab8d16d` mede:

```text
repr((type(format.pdf), format.pdf, format.pdf.pages,
      format.pdf.standard, format.pdf.tagged, format.pdf.pretty))
=> (function, pdf, none, ("1.7",), auto, false)

format.pdf.attach == pdf.attach       => true
format.pdf.artifact == pdf.artifact   => true
format.pdf()                           => cannot be constructed manually
```

Com `a11y-extras`, os três membros condicionais são funções com nomes curtos e
iguais aos respectivos valores globais; sem a feature, não existem. Alias,
`std.format.pdf` e a aplicação parcial `.with(pretty: true)` preservam o
namespace funcional. A ordem de inserção, o endereço de `Arc` e a forma da
factory são mecânica; tipo, nome, defaults, igualdade linguística, presença e
diagnóstico são observáveis públicos.

### Contrato do filho

A factory de `format` recebe a projeção efetiva já construída do módulo global
`pdf`; este owner não chama `make_pdf_module`, não duplica suas nativas e não
reimplementa seus gates. Ela acrescenta um terceiro membro `pdf`, como
`Value::Func` nativa nominalmente `pdf`, não construtível e com namespace:

```text
pages    = none
standard = ("1.7",)
tagged   = auto
pretty   = false
attach/artifact = clones dos bindings do módulo pdf efetivo
trio a11y        = clones somente quando presentes no módulo pdf efetivo
```

Os clones preservam a identidade linguística de função: cada membro compara
igual ao seu correspondente `pdf.*`, conserva o nome curto e continua
disponível através de `.with`. A ausência de algum membro obrigatório no valor
PDF recebido é erro de composição, nunca motivo para criar stub, função por
nome ou fallback silencioso. A função `format.pdf` não é `Value::Module` e a
chamada direta reutiliza exatamente `cannot be constructed manually`.

O pin `compiler-feature-gates` exige que o trio condicionado esteja todo
ausente sem `a11y-extras` e todo presente com a feature; target, `html` e o
formato de saída não alteram o conjunto. `format.svg`, `format.png`, o módulo
global `pdf` e os seus bindings permanecem semanticamente idênticos.
`format.html` continua ausente até P1465.

### Incompletude deliberada e aceitação

P1464 legitima somente descoberta, identidade, defaults visíveis, construção
proibida, reutilização dos cinco membros e gate de presença. Não legitima
`#set format.pdf(...)`, transporte ao exportador, seleção de páginas, tagging,
serialização pretty, PDF/A ou PDF/UA. Os passos que completam esses eixos são:

- P1465: `format.html` e membros;
- P1466: `tagged` e `pretty` de ponta a ponta;
- P1467: `pages` e `--pages`;
- P1468: `standard`, `--pdf-standard` e conformidade real.

Nos perfis `default`, `a11y-extras`, `html` e `html+a11y`, exigir tipo/nome,
os quatro defaults exatos, alias/`std`, rejeição da chamada direta, igualdade
dos membros, nomes curtos, gate integral e namespace preservado por `.with`.
Não dar crédito a opção documental ainda não transportada nem converter
`Unknown` em sucesso.

A confirmação humana explícita posterior ao commit causal P1464 foi recebida
em 2026-09-20. Implementação e teste independente partiram desse baseline; o
teste passou de RED (`1/4`) para GREEN (`4/4`) sem ampliar o recorte dos passos
P1465–P1468.

## P1465 — `format.html`, `pretty` e membros reutilizados (PROPOSTO)

### Medição anterior à decisão

No baseline `8e74134a721310cc4fb2a596a195398e16f1d5db`, este consumer recebe
somente o módulo PDF efetivo e expõe `svg`, `png` e `pdf`. O módulo global
produzido por `compiler/stdlib/html.rs::make_html_module` já contém `elem`,
`frame` e 112 constructors tipados, todos como funções produtivas.

No vanilla ratificado `586e1bd43fae6c9a973218163d3165c53ab8d16d`, com a
feature `html`:

```text
type(format.html)                   => function
repr(format.html)                   => html
format.html.pretty                  => false
format.html()                       => cannot be constructed manually
format.html.elem == html.elem       => true
format.html.frame == html.frame     => true
format.html.<cada tag> == html.<tag> => true (112/112)
```

Alias, `std.format.html` e `.with(pretty: true)` preservam nome e namespace.
Sem a feature, o binding não existe na superfície e o acesso recebe o
diagnóstico de feature `html` desativada. Tipo, nome, default, igualdade,
presença e diagnóstico são linguagem pública; clones, endereços, ordem de
inserção e forma da factory são mecânica.

### Contrato do filho

A factory passa a receber também a projeção efetiva já construída do módulo
global `html`. Ela valida que o valor é o módulo `html`, cria uma função nativa
nominalmente `html`, não construtível, e forma seu namespace com:

```text
pretty = false
elem/frame/112 tags = clones dos bindings do módulo html efetivo
```

Este owner não chama `make_html_module`, não enumera nomes de tags, não cria
wrappers e não reimplementa funções HTML. Todo binding projetado deve ser
função; duplicação, ausência ou variante incompatível é erro de composição,
nunca stub ou omissão silenciosa. Os clones preservam igualdade linguística,
nome curto e descoberta através de `.with`.

O valor `format.html` permanece presente internamente na definição de
`format`, mas é gated na superfície pelo owner de field access. O target, o
exporter e `a11y-extras` não ativam `html`. `format.svg`, `format.png`,
`format.pdf` e o módulo global `html` permanecem semanticamente idênticos.

### Opção documental e fronteira

Este owner legitima a descoberta de `pretty: false`, não sua extração ou
precedência. A set rule pertence a `compiler/eval/rules.md`; carrier e coleta
pertencem aos owners documentais; resolução/export pertence a pipeline/wiring.
O contrato vertical P1465 exige `--pretty > documento > false`, mas não move
essas responsabilidades para esta factory.

Aceitação cobre os quatro perfis, os 114 pares, alias/`std`/`.with`, gate,
default e construção proibida. Esta ampliação pública aguarda confirmação
humana explícita posterior ao commit causal P1465; antes dela não autoriza
código, teste RED nem alteração de header.
