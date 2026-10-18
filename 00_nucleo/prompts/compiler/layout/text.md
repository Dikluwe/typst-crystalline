# L0 — text
Hash do Código: c7439e33

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/bibliography-prefix-admission.toml sha256:64d61513bab0293f918958404cf4fe0387dea84cc53f5442faad2b467fa4e077
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:6dbf8faa56960845c60734f5e047685ec5b3a6f14c0ce3a48d333b8982d0baa5
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:79d5c48d0b9d0fb7d995207d18b1f9437209bf35c78a35cbfcedccea8b90abd2
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:3ee5760ffa44b5fd889b9d6772972e3b8417be332e13726cd9eea24b32e686f8
- 00_nucleo/prompts/_nuclei/paragraph-selection.toml sha256:e9038b453bbb1971c9a75a76eced99c35db742f57bebbb09b03c4922b3862800


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/text.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/text.rs:20`, `01_core/src/compiler/layout/text.rs:209`, `01_core/src/compiler/layout/text.rs:237`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Build effective TextStyle from the active chain and cached style using the source's field-specific precedence; preserve family/variants/weight/direction/language/variation/edge/leading/tracking/highlight and math/cramped/script provenance. Carry math_text_item independently of math, and semantic_strong/semantic_emph only from their existing morphological provenance, not arbitrary bold/italic appearance.

Preserve current sub/superscript sizing fallback 0.65 and baseline offsets, and the existing SmallCaps lowercase-to-uppercase run rendering at factor 0.8 with temporary size restoration. Split ordinary text by literal spaces, dispatch words/chunks through Cursor and advance/record inter-part space opportunities as currently implemented. Restore the outer style. This unit does not shape through I/O, choose installed fonts, re-evaluate show rules or replace SpaceFromNewline's adapter.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

## Separadores textuais — materialização autorizada em 2026-09-28

### Medição anterior à decisão

`01_core/src/compiler/layout/text.rs` é um consumer do percurso FrameItem. As sondas RED fonte→PDF de
`00_nucleo/diagnosticos/fatia-funcional-tekt-2026-09-28/` e a inspeção
dos handlers desse consumer precedem esta alteração. A aprovação do contrato
público em `00_nucleo/prompts/entities/layout_types.md` foi recebida em
2026-09-28 e registrada na proveniência da materialização.

### Responsabilidade deste owner

Cada separador literal delega ao produtor interno de Space com origem Literal. Conservar avanço, oportunidades e ausência de flush imediato do loop literal; não duplicar lógica de registro.

Conservar as responsabilidades existentes; a aceitação segue texto e geometria
observáveis, incluindo os controles de adjacência e os invariantes pinados.

A resolução do estilo efetivo delega, sem mudança de precedência, ao owner `00_nucleo/prompts/compiler/layout/text/style.md`; Text mantém apenas a aplicação temporária do estilo e o layout de palavras.

## Preparação e seletores — 2026-09-28

### Medição anterior à decisão

O lote bilateral `00_nucleo/diagnosticos/correcao-seletores-paragrafo-2026-09-28/before-pdf/resultados.json` mede 32 entradas, com estado exato em initial.json e before/, contra vanilla 9f2b6e8715237cb086899a42873660fe744622e8. Em 140pt/margem 8pt, simple escolhe delta epsilon zeta eta, optimized escolhe delta epsilon zeta e junta eta à linha seguinte; Tekt ignora a estratégia. Em 240pt justificado, optimized admite eta na primeira linha reduzindo os espaços. Fonte: eval/rules.rs:3070-3100 e stdlib/structural/par.rs:131-150 validam e descartam linebreaks; text.rs:41 e word_layout.rs:29 escolhem incrementalmente; vanilla inline/mod.rs:194-196 resolve auto pela justificação, linebreak.rs:raw_ratio/raw_cost aplica redução até 1/3 e custos cúbicos/quadráticos; model/par.rs:Limits::SPACING_DEFAULT fixa 2/3..1.5. Estilos adjacentes, literal leading/trailing e conteúdo opaco estão medidos separadamente. Observação não prova intenção histórica; inferência refutável: os scope-outs L0 explícitos de preparação/optimized legitimaram o adiamento, pois ainda vedam a implementação global apesar do transporte aceito.

### Decisão e escopo

Correção interna de paridade ADR-0127: sem novos contratos públicos, flags ou fase. Primeira preparação global certificada: parágrafo latino composto por Text, Space, SpaceFromNewline, Empty, Sequence, Styled tipográfico e Strong/Emph, com hifenização desligada, direção LTR, sem tracking/math/smallcaps, largura finita e constante, fora de colunas/células/floats. O conjunto completo deve ser certificado antes de emitir; uma unidade recusada preserva o caminho anterior. Hifenização, Unicode UAX14 completo, links/objetos/linebreaks, largura variável, limites customizados e seleção incremental restante continuam explicitamente incompletos; este lote não declara estado da arte nem otimização completa da língua.

Antes do loop incremental, se guard não suprime, tentar preparação do Text isolado por paragraph_plan; emitir pelo plano só se certificado. Resto e SmallCaps continuam iguais.

## Linhas causais vazias em página auto — 2026-09-29

### Medição anterior à decisão

`00_nucleo/diagnosticos/correcao-auto-proximas-2026-09-29/before/results.json` mede fontes→PDF com binários pinados e entradas sem newline artificial; `initial.json` guarda HEAD, working tree, hashes e snapshots. `#text("")` e a variante 18pt têm Tekt 3×3pt versus vanilla 3×8.568/12.852pt. `abc\n\n` termina em Tekt 19.872×31.536pt versus vanilla 19.872×8.568pt, enquanto `abc\n\nxyz` coincide a 19.872×31.536pt. `#text("  ")\n\nabc` fica numa linha Tekt (26.112×8.568pt), mas em duas linhas vanilla (19.872×31.536pt). Fonte: `text.rs:37-86` não emite item para Text vazio; `html_frame.rs:72` exclui todo TextSeparator de materialidade; `line_commit.rs:280-286`, `parbreak.rs:43-48` e `mod.rs:1965` usam essa materialidade para linha; `parbreak.rs:40-85` avança cursor após fechar linha e `page_geometry.rs:180-201` usa cursor se não há fundo confirmado. Vanilla `inline/linebreak.rs:695-699` oferece breakpoint obrigatório até para texto vazio. Isto mede o observável; não infere a intenção histórica nem exige a mecânica do vanilla.

### Contrato interno segregado

Text vazio explícito conserva uma unidade de texto de avanço zero com estilo efetivo e posição de baseline; ela estabelece altura tipográfica sem criar tinta, largura ou fronteira de tracking. Texto vazio em linha já material preserva a posição de palavras e a extração textual.

Aceitação: testes RED→GREEN e matriz bilateral, regressões anteriores, build/lint sem erros; resselo somente dos owners deste manifesto.

## Migração do carrier bidi ratificado (2026-10-05)

Medição anterior à decisão: o inventário `00_nucleo/diagnosticos/bidi-contexto-2026-10-05/inventory.json` identifica as referências desta unidade e os owners 1:1. O contrato do manifesto 1ae7513e foi confirmado; nesta unidade ajustar constructors não certificados com None e patterns de leitura com `..`, preservando os observáveis de linguagem existentes. Transformações obedecem ao núcleo pinado de origem; sem ampliar fase/API.

## Transporte lógico textual — contrato ratificado em 2026-10-05

Medição anterior à decisão: `00_nucleo/diagnosticos/bidi-contexto-2026-10-05/probe.json.gz` refuta resolver 456 789 isoladamente; a medição de fonte e os locais afetados estão no inventário e initial.json.gz daquele diagnóstico. A API imutável e os três campos opcionais foram ratificados pelo humano (manifesto 1ae7513e). Correção interna de paridade sem mudar fase.

Esta unidade participa do transporte por Cursor privado, input Arc e spans exatos. Preparar antes de emitir o parágrafo textual inteiro, base lexical já certificada; conservar offsets por ordem de visita, não matching de strings. Sequence/Par criam escopo, nested Sequence herda; checkpoint conserva cursor e span corrente; subframe isola e restaura. Text fornece subspans para palavras e espaços literais; Space consome seu segmento mesmo quando fraco; Linebreak consome fronteira. Word emite span somente se texto exato não foi transformado.

Escopo inicial: incremental simple, fora de células/colunas/subframes, sem tracking/hifenização/justificação/smallcaps, Text sem newline/tab/segmentação linguística. Região recusada tem None. Preparação global anterior permanece para parágrafos que não precisam do contexto bidi; para o subconjunto certificado com base RTL ou caracteres RTL/override oposto, encaminhar pelo incremental sem mudar os breaks permitidos. Não interferir em optimized explicitamente escolhido. O owner privado bidi_origin prepara/valida; esta unidade só coordena o seu papel. Testes de origem por ocorrências, UTF8, restauração e parbreak; validação bilateral de wraps.

## A2 — prefixo admitido e região inline de Block (2026-10-18)

### Medição anterior à decisão — admissão e continuação

pagination-block.json.gz e block-freeze.json.gz em
00_nucleo/diagnosticos/bibliography-structure-2026-10-18 preservam estado completo,
UTC/HEAD/fontes/binários/argv e caixas do candidato Block/Place c1c222ae54ab3b67487471326012a7f6c4ed86c1e6bd61b1830ec6a987c2ccea.
Em IEEE/200×90pt ele recupera duas páginas, mas a continuação começa em x8
contra29.924 no oracle bd5ebec285505889e8ff1d4f68804ab147a699f3 (binário
9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9).
region_progress.rs:75–76 reinstala a margem física ao transitar, descartando o
inset ativo. O teste corrigido para70pt também mostrou prefixo na primeira
página e corpo admitido na segunda: paragraph_pagination.rs:87–105 pode adiar
o corpo para respeitar widow/orphan depois que o Place já foi emitido.

A exigência inicial de FIRSTWORD sempre na primeira página da fixture FixedMetrics
não é o contrato correto: group_size em paragraph_pagination.rs:69 pode manter
três linhas juntas. Substituir esse critério pelo prefixo acompanhar a página
real da primeira linha; preservar o RED PDF90/duas páginas e a distinção entre
coluna estável e estratégia de quebra. Não forçar greedy nem desabilitar
widow/orphan para satisfazer um assert. Posições geométricas, conteúdo e agrupamento
são observáveis da língua; slots/representação são mecânica privada.

Correção interna ADR-0127 dentro do layout, sem assinatura/campo/trait público,
flag/default novo, compatibilidade ou execução eval/show. A primeira tinta do
prefixo continua antes da primeira tinta do corpo, após a admissão física do
parágrafo. Não é nova ordem de realização de show nem transferência de fase.
A hipótese é refutável por prefixo órfão, recuo perdido, emissão dupla, efeitos
novos, estratégia alterada ou regressão dos controles de Block/columns/subframe.

### Responsabilidade exclusiva

No caminho de Text vazio que emite uma folha diretamente, consumir o prefixo pendente após text_baseline::admit e antes da folha. Texto não vazio continua usando word_layout; nenhuma chamada adicional à preparação/avaliação.

Aceitação: RED→GREEN de prefixo/corpo na mesma página e inset nas continuações, inclusive saída para OUTSIDE, simple/optimized e controles existentes. Medir PDF90/150/250, RTL, prefix-only/formatado, cores e regressões; build, suites/doctests/Clippy/Rustdoc/fmt/linhagem/reversa/diferencial Tekt. MemoryMax12GiB/High8GiB/swap0/jobs1; sem publicação. Hanging genérico/overrides e múltiplas bibliografias continuam frentes abertas.
