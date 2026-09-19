# Prompt L0 — `compiler/stdlib/format` — raiz pública de formatos
Hash do Código: ac99324c

**Estado:** P1461/P1462 MATERIALIZADOS E CERTIFICADOS; CONTRATO P1463
PROPOSTO — AGUARDA CONFIRMAÇÃO ADR-0127.

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

- P1462 completou `format.svg` e suas opções;
- P1463 propõe completar `format.png` e `ppi`;
- P1464 completa `format.pdf` e seus cinco membros;
- P1465 completa `format.html`, `pretty`, gate `html` e seus membros.

`format.svg` está materializado. Até os passos restantes terem L0 e autorização
próprios, `format.png`, `format.pdf`, `format.html` e seus descendentes
permanecem ausentes. É proibido criar filhos, aliases globais ou passthroughs
para antecipar a contagem dos passos seguintes.

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
e certificada. Esse fato não autoriza qualquer filho reservado.

## P1462 — `format.svg` (MATERIALIZADO E CERTIFICADO)

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
2026-09-19. O selo, candidato, ataques e veredito final foram concluídos no
mesmo dia; P1463 não reabre esse contrato.

## P1463 — `format.png` e namespace `ppi` (PROPOSTO; gate ADR-0127)

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
dos siblings futuros nos quatro perfis. A ampliação pública exige confirmação
humana posterior ao commit causal P1463. Antes dela este texto não autoriza
teste RED, consumer nem alteração de header de linhagem.
