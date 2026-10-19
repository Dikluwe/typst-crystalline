# L0 — html frame
Hash do Código: 051a54e9

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/link-frame-scopes.toml sha256:4e487a99ea1f1e82a67dfaced0958f98392fb26b1a7d292355230d6ffa7f2d0b
- 00_nucleo/prompts/_nuclei/layout/position-frame-carriers.toml sha256:ff71dfc6648d6deb4b2b8054e3d58a93777d17837129f0f1b1e7e138ffb6943c
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:08c39c6b9cf6fdc571564646d12d71f9b12f2a5d2c029ca90f9253c39c28bb95
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:79d5c48d0b9d0fb7d995207d18b1f9437209bf35c78a35cbfcedccea8b90abd2
- 00_nucleo/prompts/_nuclei/place-coordinate-origin.toml sha256:2baf472bb540640d571ae618aa218e5f2b166839a2070f8a4a907ec65d7ded20
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:4d947acae2d9bf8baff2639b7c9c1bdebb7b260e7ca4ac282d4f9d0e7a7feaee

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/html_frame.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/html_frame.rs:22`, `01_core/src/compiler/layout/html_frame.rs:25`, `01_core/src/compiler/layout/html_frame.rs:30`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/core/compiler/layout/mod.md`; the contract below defines this unit's operations, not ownership of that hub.

In ordinary paginated dispatch, delegate once to the body without a visual wrapper or default page margin. Intrinsic entrypoints create an isolated Layouter with supplied chain/metrics/sizer, unbounded region and transparent Page cropped to supported visual dimensions, logical_number one, without joining the global paginated sequence. Nesting remains transparent inside the same island.

The detailed variant captures ContextBlock ids using private zero-size carriers in the existing FrameItem tree, not a public variant or HtmlElem sentinel. Transport carriers through box/place/grid/link/group/transform geometry; after all composition, extract deterministic id/HtmlPosition pairs in the island's causal reference frame before private vertical visual cropping. Then normalize only the visual Page, remove all carriers, and derive final visual dimensions without them. Missing id/Location correspondence does not fabricate a position.

Own the shared marker identity/material predicates: only a direct private marker is non-material. Existing Semantic, Link and Group wrappers retain ordinary legacy material classification even if they contain only a carrier; do not flatten them. Extent helpers ignore a direct marker but transformations still reach its point. No SVG encoder, World, PageStore or logical PDF-label behavior is introduced. The hub owns public reexports, this unit owns intrinsic implementation.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

## Separadores textuais — materialização autorizada em 2026-09-28

### Medição anterior à decisão

`01_core/src/compiler/layout/html_frame.rs` é um consumer do percurso FrameItem. As sondas RED fonte→PDF de
`lab/results/construction/diagnostics/fatia-funcional-tekt-2026-09-28/` e a inspeção
dos handlers desse consumer precedem esta alteração. A aprovação do contrato
público em `00_nucleo/prompts/core/entities/layout_types.md` foi recebida em
2026-09-28 e registrada na proveniência da materialização.

### Responsabilidade deste owner

TextSeparator é neutro para items_have_material e visible_top; nunca o confundir com marcador de contexto removível.

Conservar as responsabilidades existentes; a aceitação segue texto e geometria
observáveis, incluindo os controles de adjacência e os invariantes pinados.

O refutador P1493 de Page visível compara a projeção sem folhas TextSeparator, preservando dimensões, matriz/pivô e posição do marker; verifica separadamente a retenção do carrier textual. Estrutura do Vec não é a geometria visível.

## Identidade textual — materialização aprovada

### Medição anterior à decisão

As sondas bilaterais e o estado exato em
`lab/results/construction/diagnostics/identidade-hifen-discricionario-2026-09-28/initial.json`
e `before/` medem U+002D versus SHY com geometria coincidente em oito controles.
A inspeção do percurso FrameItem deste consumer está arquivada em approved-before/.
O dono confirmou o contrato em `00_nucleo/prompts/core/entities/layout_types.md`.

### Responsabilidade exclusiva

Conservar TextReplacement na remoção de marcadores contextuais, descendo nos filhos; materialidade/posição consultam somente os filhos visuais.

O Núcleo text-replacement define apenas invariantes compartilhados; este prompt
possui somente o consumer declarado. Aceitação: texto/caixas, testes causais,
transporte recursivo e controle de hífen literal; não bytes PDF ou nova seleção.

## Linhas causais vazias em página auto — 2026-09-29

### Medição anterior à decisão

`lab/results/construction/diagnostics/correcao-auto-proximas-2026-09-29/before/results.json` mede fontes→PDF com binários pinados e entradas sem newline artificial; `initial.json` guarda HEAD, working tree, hashes e snapshots. `#text("")` e a variante 18pt têm Tekt 3×3pt versus vanilla 3×8.568/12.852pt. `abc\n\n` termina em Tekt 19.872×31.536pt versus vanilla 19.872×8.568pt, enquanto `abc\n\nxyz` coincide a 19.872×31.536pt. `#text("  ")\n\nabc` fica numa linha Tekt (26.112×8.568pt), mas em duas linhas vanilla (19.872×31.536pt). Fonte: `text.rs:37-86` não emite item para Text vazio; `html_frame.rs:72` exclui todo TextSeparator de materialidade; `line_commit.rs:280-286`, `parbreak.rs:43-48` e `mod.rs:1965` usam essa materialidade para linha; `parbreak.rs:40-85` avança cursor após fechar linha e `page_geometry.rs:180-201` usa cursor se não há fundo confirmado. Vanilla `inline/linebreak.rs:695-699` oferece breakpoint obrigatório até para texto vazio. Isto mede o observável; não infere a intenção histórica nem exige a mecânica do vanilla.

### Contrato interno segregado

Manter `items_have_material` como predicado de tinta/material visual, com TextSeparator neutro. Acrescentar predicado privado distinto para existência de linha causal: material visual ou TextSeparator Literal; Markup/Newline inicial e marcadores contextuais não criam linha.

Aceitação: testes RED→GREEN e matriz bilateral, regressões anteriores, build/lint sem erros; resselo somente dos owners deste manifesto.

## Iteração e parâmetros internos — 2026-10-13

### Medição anterior à decisão

`01_core/src/compiler/layout/html_frame.rs` nas linhas 167 contém os avisos selecionados do Clippy. O snapshot anterior à decisão, spans conferidos contra a fonte, owners, referências, binário, UTC, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, status e diffstat estão em `lab/results/construction/diagnostics/iteracao-parametros-ci-2026-10-13/before.json.gz`. A inspeção mede mecânica interna e usos; não é nova prova bilateral de paridade ou ganho de desempenho.

### Decisão e aceitação

Remover somente o parâmetro privado metrics e a generalização M de strip_context_marker: nenhuma operação do helper usa métricas, apenas as repassava a si próprio. A remoção recursiva conserva wrappers TextReplacement/Semantic/Link/Group, campos, ordem e filhos ordinários. filter_map chama diretamente o mesmo helper privado. Coleta de posições permanece antes da remoção, e aquisição/layout de métricas permanece nos entrypoints existentes. Não transformar wrappers vazios em Empty, nem remover TextSeparator.

Refatoração interna equivalente em fluxo contínuo ADR-0127, sem contrato público, default ou fase novos. Conservar as expectativas dos testes existentes; não criar testes que apenas espelhem estes loops. Validar suíte core (incluindo conflitos, spans, fixups, marcadores e seleção diferencial), todos os targets, build, documentação, Clippy e linhagem com resselo restrito. Nenhuma supressão adicional de lint é autorizada por este lote.

## Posições no referencial composto — 2026-10-14

### Medição anterior à decisão

box-contexto-2026-10-14/probe.json.gz, fontes/binários/HEAD/working tree/UTC
preservados: Box, inset2pt, prefixo e aninhamento publicam (0,0), mesmo
quando a tinta se desloca. A diferença da baseline do context direto é
uma questão separada: não copiar o vanilla apenas por diferir. mod.rs:
1131-1148/1180-1200 escreve pontos locais em runtime.positions; Box agrupa
o corpo em outro referencial; finish.rs:295-302 sela o mapa sem recompor
esses pontos. Realized escreve com or_insert; labels têm mapa paralelo.
HTML já transporta markers privados pelo frame em html_frame.rs:30-199.
O registro single-pass e sealing por sub-store são preservados, ADR-0073/
0074 §§posição/modelo híbrido; não se adota introspecção global vanilla.
A inspeção do núcleo coordinates e seus owners mostrou usos absolutos
em grid e explicitamente locais em Place/Box. A qualificação do núcleo
conserva ambos e exige transformação até a página, sem dupla origem.
Snapshot anterior: posicoes-compostas-2026-10-14/before.json.gz.
Geometria/identidade são observáveis; o carrier e seu índice são mecânica.

### Contrato deste owner

Estender o predicado compartilhado de carrier direto para reconhecer também o carrier paginado estrito do novo owner. O protocolo HTML id/HtmlPosition e sua extração anterior ao crop permanecem. Remover ambos os carriers antes da saída visual intrínseca, inclusive sem captura HTML detalhada. Grupos e wrappers existentes mantêm sua classificação; não tornar todo wrapper de carrier implicitamente vazio.

Correção interna ADR-0127: sem API/campo público, flag/default ou mudança
de fase. L0 primeiro; RED→GREEN de coordenadas, ausência de tinta/altura
fictícia, identidade, ordem, clones visuais e rollback; revalidar os
controles HTML, geometria, contexto/queries e suíte completa.

## Captura de links através de fronteiras — 2026-10-14

### Medição anterior à decisão

lab/results/construction/diagnostics/links-paginacao-2026-10-14.md e os recibos
posicoes-compostas-2026-10-14/automatic-boundary-probes registram fonte,
HEAD/árvore suja/UTC, binários e pin vanilla. Página 160×40pt/margem 10pt:
texto comum atravessa a quebra; Link URL, Ref e Ref Page abortam com índice
antigo após esvaziamento dos buffers. references.rs:270 aparece na pilha;
link.rs:22-32 usa o mesmo mecanismo. outline.rs:28-50 também guarda índices
antes do corpo. Limitar o índice não prova preservação do prefixo, ordem,
fragmentos, destino ou geometria. Snapshot anterior deste reparo:
lab/results/construction/diagnostics/links-captura-2026-10-14/before.json.gz.
Os observáveis são conteúdo, destino, regiões e bounds; limites privados
são mecânica. Correção interna ADR-0127, sem API, default ou fase nova.

### Contrato deste owner

Reconhecer limites privados de link como marcadores diretos neutros, junto dos marcadores existentes. Resolver links antes de remover metadados/cortar e publicar a ilha intrínseca. Preservar protocolo de contextos e targets.

Aceitação: RED→GREEN de quebras antes/dentro do corpo, preservação de
prefixo/sufixo, regiões, destinos/bounds, corpo rico/aninhamento/colunas,
conteúdo diferido, contexto/rollback e ausência de limites privados na saída.
Revalidar os controles sem quebra e a suíte, build e linhagem.

## Origem geométrica da ilha e do carrier — 2026-10-14

### Medição antes da decisão

place-origem-local-2026-10-14/html-failures.log reproduz quatro falhas P1493 existentes: aqui() publica y11.2 em vez de20 dentro de Place(Box HTML), diferença de ascender8.8 em FixedMetrics. O entrypoint html_frame.rs:219 compõe em baseline legada; context_block.rs:16–22 subtrai ascender ativo mesmo quando Place/Box usam origem geométrica0. Os testes de pipeline preservam explicitamente x30/y20, nenhuma página e fixpoint do mesmo target. O estado anterior está em html-before.json.gz; workspace-interrupted conserva a primeira ocorrência e o estado integral. Captura e desenho devem usar a mesma geometria, como exige o núcleo coordinates; não é motivo para converter HTML em pipeline paginado nem mudar ADR-0128.

### Responsabilidade exclusiva

Compor a ilha intrínseca pelo helper de origem geométrica. Texto instala a própria baseline; a captura continua antes do crop visual, e carriers são removidos depois da projeção. Não alterar target, feature, fixpoint, PageStore, assinaturas ou política de posições.

Correção interna ADR-0127 para preservar o contrato HTML já vigente. As expectativas P1493 permanecem iguais. Validar todos os testes p1493, contexto/HTML e suíte completa, além dos controles PDF/SVG e gates de linhagem; sem mudança pública ou de fase.

## Referencial de pintura — transporte aprovado T1b — 2026-10-14

### Medição anterior à decisão

O manifesto `d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d`
foi aprovado pelo humano para T1–T3; recibo em
`lab/history/construction/records/00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`.
A evidência bilateral de Parent e controles Solid está no contrato aprovado;
ela não é substituída por inspeção estrutural. A rodada
`lab/results/construction/diagnostics/paint-root-2026-10-14/before/results.json.gz` conserva
fontes, estado sujo, HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, UTC,
diffstat e hashes dos binários; cobre contêineres, Place e Rotate com Solid
e Gradient Parent. O inventário before-source/syntax-nodes registra neste
consumer as construções/desestruturações antes da edição: [184, 193].
layout_types.rs:531–539 não distingue raízes de grupos técnicos; helpers.rs:253–269
reconstrói o Group e precisará preservar o campo. Box agrega fundo/corpo;
Block mantém body plano exceto no clip. Preservação do referencial é requisito
de linguagem; formato dos envelopes é mecânica. A aprovação não fecha as
divergências gerais de geometria previamente medidas em Rotate/Box.

### Responsabilidade exclusiva

Migrar construções e padrões de Group para o campo paint_root aprovado. Grupos técnicos novos e fixtures legadas são false; reconstruções transportam o valor recebido, sem inferir raiz a partir de dimensão, clipping ou transformação. Preservar os demais campos e o comportamento de desenho/recursos. Exportadores ainda não usam este metadado para patterns; essa integração é T2 e não está certificada aqui. Nos ramos que desestruturam e recompõem Group, capturar paint_root e reinseri-lo sem mudança. Não redefinir true como false durante projeção, remoção de marcadores ou rebasing.

### Aceitação e incompletude

L0 primeiro, testes RED→GREEN, resselo restrito; conservar geometria e texto
nos controles fonte→PDF/SVG e revalidar suíte/build/doc/check/fmt/linhagem.
A migração deste metadado não substitui Shape.fill/stroke por FramePaint nem
implementa patterns PDF/SVG/raster; T1c/T2/T3 continuam explicitamente abertos
e já autorizados. Não alegar paridade de tiling ou produção a partir do build.


## FrameItem — exaustividade de decisões existentes, V16, 2026-10-16

### Medição anterior à decisão

F007: 01_core/src/compiler/layout/html_frame.rs:163 (collect_context_positions). Os spans AST, patterns/guards/corpos e complements foram conferidos antes do código. `lab/results/construction/diagnostics/tekt-frame-items-2026-10-16/before.json.gz` conserva HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, fontes/L0/configuração, hashes, status e git diff HEAD --stat. `classification.json` registra os 70 casos e seus complements em 26 owners, usando a árvore do parser pinado do CI. Cláusulas pertinentes do L0 foram lidas junto da fonte; não há proposta de nova arquitetura ou interpretação de intenção upstream. Padrões e exaustividade Rust são mecânica; semântica/morfologia, geometria, efeitos e diagnósticos do produto devem permanecer.

### Prescrição exclusiva desta unidade

- **F007 — `collect_context_positions`**: Trocar somente o padrão '_' pelo grupo nominal `TextSeparator`, `Text`, `TextShaped`, `Line`, `Glyph`, `Image`, `Shape`. Usar o mesmo prefixo Rust em escopo (`FrameItem`). Conservar integralmente o corpo atual do fallback, os braços anteriores, guards, transformações, chamadas, returns e ordem.

Manter callbacks, efeitos, metadados, clipping, fontes, positions, tinta e largura conforme os owners vigentes. Esta rodada não amplia travessias, não transforma wrappers em folhas nem usa text/ActualText como substituto da geometria. Helpers de teste mantêm fixtures, esperados, tolerâncias, contagens e falha Unknown; não aproveitar o saneamento para aceitar ausência de observação. Grupos nominais compartilham o mesmo corpo anterior: notas V19 de alternativas continuam visíveis, sem confundir cobertura de um braço com todas as alternativas.

### Gate e aceitação

Manutenção interna ADR-0127 em fluxo contínuo: nenhum enum/campo novo, método de trait/assinatura pública, default, dependência, compatibilidade ou fase nova. Não materializar propostas históricas pendentes por este adendo. L0 primeiro e resselo restrito em ambas as direções. Não adicionar supressões, alterar limiar ou reescrever match como if-let para esconder V16.

Antes do código, executar o controle novo de FrameVisitor e a suíte workspace nos mesmos esperados, e confirmar RED dos 70 avisos da família por leitura AST. Após código, zero V16 dessa família nos locais abrangidos e nenhum aviso novo; manter V16 externos como pendências. Nos matches do core produtivo alcançados por cargo check -p typst-core, usar cópia descartável com uma variante sentinela de FrameItem para medir ausência de E0004 nos locais antes e presença depois. A sonda cobre somente os locais efetivamente compilados: não extrapolar aos helpers cfg(test) ou L3 que a dependência impede de alcançar. Para todo o lote, conferir padrões/guards/corpos por replay, enum e assinaturas inalterados, build e testes reais. Não substituir corpos por stubs.

Revalidar suíte workspace completa, cargo build, Clippy workspace/all-targets com -D warnings e -D unfulfilled_lint_expectations, formato, Rustdoc -D warnings com quatro índices HTML, linhagem V1/V5/V15/V26 estrita/reversa, SARIF e diff --check. Fontes/L0/configuração ficam congelados durante comandos. Execuções pesadas sequenciais com MemoryMax=12 GiB, MemorySwapMax=0 e CARGO_BUILD_JOBS=1. Não apresentar saneamento estrutural como nova paridade global, ganho de memória ou benefício medido para IA.
