# L0 — infra/shaper/reconciliation
Hash do Código: 1a264f74

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:08c39c6b9cf6fdc571564646d12d71f9b12f2a5d2c029ca90f9253c39c28bb95
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:79d5c48d0b9d0fb7d995207d18b1f9437209bf35c78a35cbfcedccea8b90abd2
- 00_nucleo/prompts/_nuclei/layout/text-tracking.toml sha256:3b0c535436048536ecd3133fcf8299e88c128a62c222661f36b7bcc6d98a598c
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:3ee5760ffa44b5fd889b9d6772972e3b8417be332e13726cd9eea24b32e686f8


**Layer:** L3
**Exclusive consumer:** `03_infra/src/shaper/reconciliation.rs`

## Current evidence

- `03_infra/src/shaper/reconciliation.rs:28` — `fix_line_positions`.
- `03_infra/src/shaper/reconciliation.rs:98` — `reconcile_rtl_positions`.
- `03_infra/src/shaper/reconciliation.rs:149` — `fix_line_positions_page`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/shaper/reconciliation.rs`.

## Owned contract

- Own fix_line_positions and helpers. Adjust only top-level page.items under the existing nonrecursive policy, grouping baselines within 0.5pt and ordering by physical x; preserve y.
- Skip the entire line if it contains math TextShaped or Glyph.
- LTR anchors its first item and accumulates real-minus-estimated advances while preserving layout gaps. Nontext widths remain stable under the existing supported cases.
- RTL captures `gap[i]=x[i+1]-(x[i]+w_est[i])` and `right_edge=x[last]+w_est[last]`; reconstruct `x_real[last]=right_edge-w_real[last]`, then `x_real[i]=x_real[i+1]-gap[i]-w_real[i]`. Preserve normal/justified gaps and the right edge; do not replace them with fixture constants or recompute layout decisions.
- Permit an internal helper to borrow `FallbackFontMetrics` already warmed by the same World and compilation. Keep the existing public `fix_line_positions(world, doc)` signature and its fresh-metrics behavior for all other callers. Width equations, item order and positions remain identical; only the cache lifetime changes.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

## Separadores textuais — materialização autorizada em 2026-09-28

### Medição anterior à decisão

`03_infra/src/shaper/reconciliation.rs` é um consumer do percurso FrameItem. As sondas RED fonte→PDF de
`00_nucleo/diagnosticos/fatia-funcional-tekt-2026-09-28/` e a inspeção
dos handlers desse consumer precedem esta alteração. A aprovação do contrato
público em `00_nucleo/prompts/entities/layout_types.md` foi recebida em
2026-09-28 e registrada na proveniência da materialização.

### Responsabilidade deste owner

Deslocar carriers com as diferenças de shaping; tratar advance como largura já aplicada (estimated == real), nunca reaplicá-lo.

Conservar as responsabilidades existentes; a aceitação segue texto e geometria
observáveis, incluindo os controles de adjacência e os invariantes pinados.

## Identidade textual — materialização aprovada

### Medição anterior à decisão

As sondas bilaterais e o estado exato em
`00_nucleo/diagnosticos/identidade-hifen-discricionario-2026-09-28/initial.json`
e `before/` medem U+002D versus SHY com geometria coincidente em oito controles.
A inspeção do percurso FrameItem deste consumer está arquivada em approved-before/.
O dono confirmou o contrato em `00_nucleo/prompts/entities/layout_types.md`.

### Responsabilidade exclusiva

Resolver x/y e larguras visual estimada/real pelos filhos e deslocá-los pelo mesmo delta mantendo text. No cálculo de shifts LTR, aplicar a diferença das larguras visuais do envelope, tal como folha textual; não estimar pela string substituta.

O Núcleo text-replacement define apenas invariantes compartilhados; este prompt
possui somente o consumer declarado. Aceitação: texto/caixas, testes causais,
transporte recursivo e controle de hífen literal; não bytes PDF ou nova seleção.

## Correção de tracking — 2026-09-29

### Medição anterior à decisão

`00_nucleo/diagnosticos/tracking-residual-2026-09-29/initial.json`, `results.json`, `summary.json` e `causal-checks.json` registram 36 entradas/108 PDFs contra vanilla pinado, fontes, UTC, HEAD e working tree completa. A palavra isolada acumula +0.052pt com tracking 0.2pt, mas fronteiras perdem um intervalo na adjacência, dois com um espaço e quatro com três espaços literais. ±1.2pt elimina a quantização sem eliminar as fronteiras. O código shaper/shaping.rs:225-232,319-326 arredonda em i32; text.rs:49-83 divide palavras; space.rs:15-33 aplica somente avanço nominal. O estado anterior à materialização está em `00_nucleo/diagnosticos/correcao-tracking-2026-09-29/initial.json`.

### Decisão interna e responsabilidade

Correção interna de paridade em fluxo contínuo ADR-0127, sem contrato público, flag ou fase nova. As cláusulas desta rodada supersedem somente a preservação dos limites de tracking descritos abaixo; demais políticas de render permanecem.

Usar tracking::width para largura real das folhas TextShaped, também nos envelopes TextReplacement e no cálculo RTL já existente; remover fórmula duplicada LTR em favor de estimated_and_real_width. Carriers continuam estimated == real: intervalos já vieram do L1. Não acrescentar fronteiras novamente nesta passagem.

Aceitação: testes RED→GREEN de fronteiras/seleção e interpretação independente Tc/TJ, controles bilaterais de tracking ±/zero, repetidos, adjacência e wrap; build e lint sem erros, resselo restrito.

## Contexto integral bidi ratificado — 2026-10-05

Medição anterior à decisão: o lote bilateral e a refutação Unicode estão em `00_nucleo/diagnosticos/bidi-contexto-2026-10-05/{initial,probe}.json.gz`, com fontes/estado/binários. `layout_bidi.rs:73–82,227–240` redetecta base e inventa separadores; `shaper/bidi.rs:34` redetecta o fragmento; isto perde a entrada integral ratificada pelo humano. Correção interna de paridade, sem nova fase/assinatura pública externa.

Para spans certificados, usar registry proprietário local de strings auxiliares e mapas por input Arc, depois views BidiInfo emprestadas normais; não usar unsafe/self-reference. Embeddings virtuais LRE/RLE/PDF só quando dir do segmento difere da base; espaços reais e Linebreak não justificado LF (justify=true fora do certificado). Não retirar controles autorais. Resolver uma vez por input em cada registry; o pipeline paginado compartilha registry/views entre reorder e shape. Wrappers públicos existentes podem construir registry local.

Reorder consulta intervalo real da linha, intersects exatas por segment/range e níveis com regras de fim de linha; não midpoint nem espaços inventados. Text misto divide por fronteiras originais, preserva estilo e subspan. Itens não textuais e fontes desconhecidas não ganham certificado. Reorder conserva âncora física aceita por L1, largura/avanço reais; não aplicar fallback de suffix/coalescing ao contexto certificado.

Shape consulta os runs do input completo para span original e não auto-detecta direção de fragmento; manter offsets de subruns por travessia sequencial, emitir subspan exato para cada TextShaped. Chave de cache existente já discrimina direção; não pôr contexto em TextStyle. Reconciliation usa base do input quando existe em vez de qualquer run RTL. Fallback None anterior permanece. Testes RED→GREEN da refutação de 456 789, overrides/neutral, extração sem controles auxiliares e medição bilateral.


## FrameItem — exaustividade de decisões existentes, V16, 2026-10-16

### Medição anterior à decisão

F069: 03_infra/src/shaper/reconciliation.rs:210 (fix_line_positions_page); F070: 03_infra/src/shaper/reconciliation.rs:224 (fix_line_positions_page). Os spans AST, patterns/guards/corpos e complements foram conferidos antes do código. `00_nucleo/diagnosticos/tekt-frame-items-2026-10-16/before.json.gz` conserva HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, fontes/L0/configuração, hashes, status e git diff HEAD --stat. `classification.json` registra os 70 casos e seus complements em 26 owners, usando a árvore do parser pinado do CI. Cláusulas pertinentes do L0 foram lidas junto da fonte; não há proposta de nova arquitetura ou interpretação de intenção upstream. Padrões e exaustividade Rust são mecânica; semântica/morfologia, geometria, efeitos e diagnósticos do produto devem permanecer.

### Prescrição exclusiva desta unidade

- **F069 — `fix_line_positions_page`**: Trocar somente o padrão '_' pelo grupo nominal `TextReplacement`, `TextSeparator`, `Semantic`, `Text`, `Line`, `Image`, `Shape`, `Group`, `Link`. Usar o mesmo prefixo Rust em escopo (`FrameItem`). Conservar integralmente o corpo atual do fallback, os braços anteriores, guards, transformações, chamadas, returns e ordem.

- **F070 — `fix_line_positions_page`**: Trocar somente o padrão '_other' pelo grupo nominal `TextReplacement`, `TextSeparator`, `Semantic`, `Line`, `Glyph`, `Image`, `Shape`, `Group`, `Link`. Usar o mesmo prefixo Rust em escopo (`FrameItem`). Conservar integralmente o corpo atual do fallback, os braços anteriores, guards, transformações, chamadas, returns e ordem.

Manter callbacks, efeitos, metadados, clipping, fontes, positions, tinta e largura conforme os owners vigentes. Esta rodada não amplia travessias, não transforma wrappers em folhas nem usa text/ActualText como substituto da geometria. Helpers de teste mantêm fixtures, esperados, tolerâncias, contagens e falha Unknown; não aproveitar o saneamento para aceitar ausência de observação. Grupos nominais compartilham o mesmo corpo anterior: notas V19 de alternativas continuam visíveis, sem confundir cobertura de um braço com todas as alternativas.

### Gate e aceitação

Manutenção interna ADR-0127 em fluxo contínuo: nenhum enum/campo novo, método de trait/assinatura pública, default, dependência, compatibilidade ou fase nova. Não materializar propostas históricas pendentes por este adendo. L0 primeiro e resselo restrito em ambas as direções. Não adicionar supressões, alterar limiar ou reescrever match como if-let para esconder V16.

Antes do código, executar o controle novo de FrameVisitor e a suíte workspace nos mesmos esperados, e confirmar RED dos 70 avisos da família por leitura AST. Após código, zero V16 dessa família nos locais abrangidos e nenhum aviso novo; manter V16 externos como pendências. Nos matches do core produtivo alcançados por cargo check -p typst-core, usar cópia descartável com uma variante sentinela de FrameItem para medir ausência de E0004 nos locais antes e presença depois. A sonda cobre somente os locais efetivamente compilados: não extrapolar aos helpers cfg(test) ou L3 que a dependência impede de alcançar. Para todo o lote, conferir padrões/guards/corpos por replay, enum e assinaturas inalterados, build e testes reais. Não substituir corpos por stubs.

Revalidar suíte workspace completa, cargo build, Clippy workspace/all-targets com -D warnings e -D unfulfilled_lint_expectations, formato, Rustdoc -D warnings com quatro índices HTML, linhagem V1/V5/V15/V26 estrita/reversa, SARIF e diff --check. Fontes/L0/configuração ficam congelados durante comandos. Execuções pesadas sequenciais com MemoryMax=12 GiB, MemorySwapMax=0 e CARGO_BUILD_JOBS=1. Não apresentar saneamento estrutural como nova paridade global, ganho de memória ou benefício medido para IA.
