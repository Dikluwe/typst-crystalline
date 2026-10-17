# L0 — measurement
Hash do Código: 8c051bf8

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:7b8a61eaafc8a07df12218f8f0560c33fae5c36f621b822b7350ccefc9774711
- 00_nucleo/prompts/_nuclei/layout/intrinsic-literal-width.toml sha256:2f3643db87d4237fa5eb9ea2c2c264c60397812126f9b9a81593cbc58b166944
- 00_nucleo/prompts/_nuclei/layout/subframe-inline-extents.toml sha256:d9b9c00602c2c68bbe82eb37a84cc01a4e0575bf84f5df8950b86b7f4dab7dd3


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/measurement.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/measurement.rs:27`, `01_core/src/compiler/layout/measurement.rs:64`, `01_core/src/compiler/layout/measurement.rs:354`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Own constrained Content/Stack measurement and isolated intrinsic measurement with causal width, height and baseline. Constrained dispatch handles the current text, sequences, shapes/curves, spaces, containers, columns, alignments and lexical-style cases using injected metrics; unsupported cases retain their current fallback. Relative Block dimensions reuse `00_nucleo/prompts/compiler/layout/block.md` resolution against the outer available region. Do not perform an emission walk merely to allocate fractional siblings.

Preserve the public `measure_content_real` pair and internal baseline-bearing companion: build an isolated zero-margin Layouter/subframe with the supplied chain, metrics and sizer, lay out once, and derive dimensions from the produced items/cursor/inline extents. Baseline is causal composition, including boxes/equations and inset, not a plain-text-classification shortcut or unconditional height baseline. A private extent marker, where the source uses one, preserves that measured extent without a new public Content variant. Restore lexical measurement style. Do not mutate the emitting Layouter's locator, commit its pages, iterate document fixpoint or execute callbacks.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

## Largura intrínseca de texto em coluna Auto — 2026-10-03

### Medição anterior à decisão

O controle bilateral pinado em `00_nucleo/diagnosticos/grid-vanilla-phase-2026-10-03.md` mediu quatro colunas Auto com desvios acumulados de 0, 2,5, 5,0 e 7,5 pt nos inícios das células. Com colunas fixas, os 320 inícios coincidiram em x dentro de 0,001 pt. A sonda de linguagem mediu 2,5 pt para o espaço sob Libertinus Serif 10pt nos dois motores. Em `01_core/src/compiler/layout/measurement.rs:70-96`, a largura de `Content::Text` soma um espaço após cada palavra, inclusive a última; `01_core/src/compiler/layout/grid/tracks.rs:79` usa essa largura em Auto. A fonte vanilla pinada mede o frame da célula em `lab/typst-original/crates/typst-layout/src/grid/layouter.rs:967-1100`. A atribuição causal ainda exige ablação e controle RED→GREEN; a observação não prova equivalência para células complexas.

### Decisão e aceitação

Na medição restrita de texto, contar o espaço entre palavras consecutivas, sem adicionar um espaço artificial após a última palavra de cada linha. A seleção de quebra deve comparar a largura da próxima palavra mais o separador quando a linha já contém palavra. Preservar a restrição de largura, a altura por número de linhas e os outros braços do dispatcher. Exigir regressões para uma palavra, várias palavras e quebra; comparar a geometria Auto antes/depois com o vanilla pinado, mantendo coluna fixa como controle. Esta é correção interna de paridade, sem mudança de contrato público, comportamento padrão ou fase do pipeline (ADR-0127).

## Transparência dos transportes de realização — 2026-10-05

### Medição anterior à decisão

A comparação pareada registra fontes e binários em `00_nucleo/diagnosticos/dupla-avaliacao-auditoria-2026-10-05/bench-paired-measurement-red.json.gz`. O controle `show_grid320` mostrou largura e raster divergentes em `geometry-measurement-red.json.gz`: a emissão preserva os textos, mas o dispatcher restrito em `01_core/src/compiler/layout/measurement.rs:69` não trata os transportes de realização e cai no fallback sem extensão. O controle sem show não divergiu. Esta observação mede uma regressão interna do ensaio; não infere intenção do vanilla.

### Decisão e aceitação

A medição restrita deve projetar `Realized` no conteúdo visual e `Located`/`ShowRecipe` no corpo, reutilizando os braços vigentes. O original consumido não soma largura ou altura. Não aplicar receitas, executar callbacks ou alocar identidades durante a medição. Exigir RED→GREEN para largura/altura de transportes e repetir o controle geométrico pareado da tabela com show textual. Correção interna, sem contrato ou fase novos, em fluxo contínuo ADR-0127.

## Sequência em coluna Auto e frame literal — 2026-10-06

### Medição anterior à decisão

`00_nucleo/diagnosticos/block-tabela-fluxo-2026-10-06/before.json` conserva
estado/HEAD/stat/status/fontes L0 e código/binários. O lote predecessor em
espaco-terminal-2026-10-06/verification.json.gz mede seis tabelas: ABC/123 e
árabe ficam em duas linhas no Tekt, uma no vanilla pinado 9f2b6e871.
measurement.rs:104-113 toma max das larguras de filhos e soma alturas de
Sequence, mesmo quando Styled/Text/Space formam um fluxo inline. O vanilla
inline/grid mede o frame completo (grid/layouter.rs:967-1100). Block RTL com
LTR literal terminal desloca ABC em 3.49658pt por espaço: block.rs:315 usa
line_content_right que exclui todo TextSeparator. page_geometry.rs:143-154
já distingue extensão auto de espaço literal da extensão de tinta da linha.
tracks.rs:108-113 ancora toda coluna em margin.left, inclusive uma coluna
Auto RTL; no oracle a coluna única termina na margem direita. Observáveis
de geometria/texto, não obrigação de copiar algoritmo ou bytes. Hipótese
refutável por quebra/âncora ainda divergente ou regressão de espaço fraco,
coluna fixa, estilos/linhas/parágrafos após medição de frame/âncora corretas.

### Classificação

Correção interna de paridade ADR-0127, sem API pública/fase/flag/dependência
ou unsafe novo. Não inferir intenção histórica de P80. Núcleo de extensão
intrínseca distingue dimensões do container de tinta/alinhamento de linha.

### Responsabilidade exclusiva

Para Sequence estruturalmente inline (Empty/Text/Space/Newline/Sequence,
Styled, Strong, Emph e transportes Realized/Located transparentes certificados),
medir o fluxo inteiro em Layouter efêmero isolado com métricas injetadas,
chain/style atuais, largura finita positiva fornecida e alinhamento de medida
Left. Isso remove translação externa da medida; não muda direção linguística.
Não executar callback, recursar pela medição restrita no helper, tocar locator/
páginas do emissor ou reexpandir conteúdo. Extrair extensão intrínseca pinada
dos itens emitidos e altura produzida, limitar largura ao orçamento fornecido.
Não-inline/Linebreak/Parbreak/containers/desconhecidos conservam dispatcher
anterior. Text simples conserva seu caminho atual; não refazer esse caminho
só para colunas de palavras simples. Validar segmentação lexical, estilos e
wrap; uma Sequence inline não equivale a um Stack vertical.


## D08B3-C2 — responsabilidade ratificada

Medição anterior: 00_nucleo/diagnosticos/table-origens-d08b3c2-2026-10-07/baseline.json.gz registra HEAD/working tree/UTC/fontes/argv e binários no pin 9f2b6e8715237cb086899a42873660fe744622e8. A fonte desta unidade tem a operação transparente correspondente nos ramos existentes de ShowRecipe/Located/Realized; Source não pode vazar como elemento observável. Aprovação humana `confirmo`, 2026-10-07, contrato em materialization/table-origens-d08b3c2-2026-10-07/contract.md.

A delegação transparente já owned de ShowRecipe.body também aceita Source.body, sem executar origem separada, publicar carrier ou atribuir Location. Preservar classificação/preparo/serialização conforme a operação desta unidade.

Manter owner exclusivo e despacho estático; sem I/O, assinatura/trait/flag nova ou fase eval↔layout. Testes e coletas C2/C/macro, builds e lineage necessários antes de certificar.

## Resultado de extensões inline — 2026-10-13

Medição anterior à decisão: core-first.log em box-composicao-2026-10-13
registra falha de measure_preserva_baseline_causal_de_texto_box_vazio_e_composto.
measurement.rs:509-512 lia line_inline_ascent/descent vazados pelo subframe;
a restauração correta do pai removeu esse canal implícito. O esperado
17pt para caixa 20pt com shift 3pt permanece correto e não será relaxado.
A fonte/owner e o estado anterior estão no snapshot before.json.gz desse
lote. Correção privada ADR-0127, sem campo/assinatura pública ou fase novos.

Ler as extensões retornadas em last_sub_frame_inline_extents, não o estado restaurado da linha pai. Conservar a fórmula de width/height/baseline e os esperados do teste causal P1491.

O núcleo pinado fixa a distinção resultado/estado externo. Exigir o teste
P1491 verde com os mesmos valores e a suíte de checkpoints/layout.

### Medição restrita de Box acompanha sua dimensão total

A inspeção de measurement.rs:272-294 durante a correção mostra que a
largura explícita ainda só restringe o body, o retorno usa bw+inset e a
altura explícita é apenas mínimo do corpo. Isso contradiz as dimensões
comprovadas em box-dimensoes-2026-10-13 para caixas não fracionárias.
O teste direto de medição usa corpo vazio e corpo maior, com inset/outset,
antes da mudança; exige 20×14pt para tamanho explícito 20×14pt, inclusive
se max_width for menor. O snapshot inicial desse owner está em before.json.gz.

No braço Boxed não fracionário, resolver dimensões explícitas totais;
medir o corpo na largura interna após retirar inset. Retornar tamanho
explícito quando presente, ou natural+inset quando auto; outset não ocupa
espaço. Não emitir, repetir callbacks ou modificar a instância emissora.
O braço fracionário conserva seu comportamento até a etapa B de composição.
Exigir RED→GREEN e controles de grid/preflight, sem relaxar seus esperados.
