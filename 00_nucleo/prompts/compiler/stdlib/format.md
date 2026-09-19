# Prompt L0 — `compiler/stdlib/format` — raiz pública de formatos

**Estado:** CONTRATO P1461 AGUARDA CONFIRMAÇÃO ADR-0127 — sem consumer e sem
`Hash do Código` até a materialização posterior ao gate humano.

**Camada:** L1
**Alvo planejado:** `01_core/src/compiler/stdlib/format.rs`
**Consumer futuro exclusivo:** `01_core/src/compiler/stdlib/format.rs`
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

- P1462 completa `format.svg` e suas opções;
- P1463 completa `format.png` e `ppi`;
- P1464 completa `format.pdf` e seus cinco membros;
- P1465 completa `format.html`, `pretty`, gate `html` e seus membros.

Até esses passos terem L0 e autorização próprios, `format.svg`, `format.png`,
`format.pdf`, `format.html` e todos os seus descendentes permanecem ausentes.
É proibido criar filhos, aliases globais ou passthroughs para antecipar a
contagem dos passos seguintes.

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

## Gate ADR-0127

Adicionar a raiz pública `format` amplia o contrato da linguagem. Este L0 não
autoriza teste RED, consumer, header de linhagem ou código produtivo antes da
confirmação humana explícita posterior ao commit causal P1461. Após a
confirmação, contrato, oráculos, mutantes e preseal segregados devem ser
congelados antes da implementação candidata.
