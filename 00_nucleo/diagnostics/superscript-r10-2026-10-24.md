# R10 — limites gerais de `super`

Os **seis limites remanescentes de R9 estão corrigidos no corpus medido**. Os **69 controles passam em PDF e SVG, em debug e release**, preservando todas as palavras, páginas físicas e a tolerância de **0,001 pt**. No executável recebido de R9, o mesmo corpus passa em **15/69** e falha em **54/69**. Os 15 controles originais passam integralmente; os 54 adicionais cobrem 11/17 pt, três fontes, quebras iniciais/consecutivas/terminais, fallback explícito, ordem fria/aquecida, tamanho explícito e tracking assinado.

A referência permanece o commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, árvore `cac836c07e4dec2a89219370428292393a3216b5`, executável SHA-256 `9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`. Versão textual e bytes de saída não foram usados como critério de linguagem. A receita ratificada continua no [resync fixado](../../lab/results/construction/diagnostics/vanilla-resync-2026-10-17/relatorio.md).

## Proveniência e escopo

HEAD `cf21f47a3b63f0e9fc5308925e1d0b67925e7227`, **working tree não commitado**. O estado recebido corresponde exatamente aos inputs encerrados em R9. O encerramento foi auditado em `2026-10-24T21:22:18.730056+00:00`. [baseline.json](../../lab/results/construction/diagnostics/superscript-r10-2026-10-24/baseline.json), [received-source.json.gz](../../lab/results/construction/diagnostics/superscript-r10-2026-10-24/received-source.json.gz), [received-69-audit.json](../../lab/results/construction/diagnostics/superscript-r10-2026-10-24/received-69-audit.json) e [completion.json](../../lab/results/construction/diagnostics/superscript-r10-2026-10-24/completion.json) preservam código, lista exata de alterações recebidas, UTC, fontes e binários. A matriz adicional recebida foi executada posteriormente com o binário recebido: seu recibo distingue inputs de origem desse executável e estado do orquestrador; a medição não foi atribuída ao código novo.

Os seis casos originais fechados são: `DejaVu Sans-literal-newline`, `Nimbus Sans-literal-newline`, `Nimbus Sans-hard-break`, `Nimbus Sans-warm-mixed`, `Liberation Serif-literal-newline`, `Liberation Serif-warm-mixed`. [baseline-audit.json](../../lab/results/construction/diagnostics/superscript-r10-2026-10-24/baseline-audit.json) e [remaining-final-debug.json](../../lab/results/construction/diagnostics/superscript-r10-2026-10-24/remaining-final-debug.json) mantêm o antes/depois sem apagar falhas intermediárias.

## Causas e correções

- Quebras literais dentro de `super` agora participam do fluxo original: reiniciam a origem da linha e posicionam os irmãos depois da última linha. A quebra terminal conserva o próximo espaço original, sem fabricar um strut vazio; o estado é isolado em subframes e preservado no rollback.
- A escolha nativa/sintética respeita o texto causal completo, inclusive a quebra explícita. O fallback recebe sua própria face, tamanho e baseline, mantendo o tamanho externo para o envelope tipográfico e o tamanho reduzido para pintura. Um fallback amplo não toma o prefixo que a fonte declarada cobre.
- Tracking assinado é escalado conforme o tamanho de pintura; o intervalo anterior é preservado entre faixas geradas da mesma origem. Espaços textuais respeitam a fonte declarada sem prioridade incidental de tabela MATH. O SVG aplica o mesmo resíduo fracionário já usado pelo PDF.
- Glifos PNG sem outline conservam pintura e identidade textual: o SVG usa o raster real, e o PDF usa Type3 extraível com ToUnicode e CharProcs que reutilizam os recursos de imagem. O fechamento de ActualText precede o Q pendente, preservando a mesma caixa do glifo em TextReplacement.

A adição pública **`FontMetrics::text_superscript_runs`** foi explicitamente aprovada no L0 SHA-256 `65d01600110f9bffea1eaff0ca349f737ab426e384fb891e88d8f6504cf911b9`. [approved-metrics.md](../../lab/results/construction/diagnostics/superscript-r10-2026-10-24/approved-metrics.md) e [approval.json](../../lab/results/construction/diagnostics/superscript-r10-2026-10-24/approval.json) preservam o contrato aprovado. A integração posterior é interna, sem novos campos de entidades, modo ou mudança de fase. São **17 consumidores e 17 L0s** correspondentes, selados em [canonical-final.json](../../lab/results/construction/diagnostics/superscript-r10-2026-10-24/canonical-final.json). A nova fonte de teste bitmap é autoral, com 1.016 bytes e proveniência em [bitmap-fixture.json](../../lab/results/construction/diagnostics/superscript-r10-2026-10-24/bitmap-fixture.json); os arquivos de fonte recebidos permanecem intactos.

## RED→GREEN e validação

Foram adicionados **12 testes de módulo**, com **dez testemunhos RED funcionais por asserção** e **dois controles positivos/defensivos** do contrato/default e da recusa de partição inválida. O novo contrato também possui forwarding explícito pelo provider dinâmico emprestado. O gate legado P941 também foi corrigido após seu RED na primeira validação: agora exige dois objetos raster compartilhados e todas as três ocorrências 🎉🚀🎉, em vez de contar referências textuais /Im. O estado e a falha originais permanecem em validation-complete e p941-gate-measurement.json. A segunda tentativa passou nos testes e foi interrompida pelo Clippy porque o novo teste SVG construía uma regex dentro do laço; validation-complete2 conserva a falha, e clippy-correction.json registra a construção única do parser, sem alteração dos controles ou supressão do lint. A validação final está em validation-complete3. Os registros preservam as tentativas refutadas e os erros de harness: [module-red-green.json](../../lab/results/construction/diagnostics/superscript-r10-2026-10-24/module-red-green.json). O refutador de TextReplacement demonstrou caixa degenerada antes da correção; o controle final exige caixa completa idêntica em Compact/Verbose e single/multifont.

| Gate | Resultado |
|---|---|
| Workspace | 12 gates aprovados: build, testes, Clippy com warnings negados, fmt e quatro índices Rustdoc |
| Testes | 9921 Rust aprovados, 7 ignorados preexistentes; 42 Python aprovados |
| R10 | 69 PDF + 69 SVG por build; maior delta PDF 0.000021000 pt e de origem de glifos SVG 0.000000001000 pt |
| Bitmap | PNG decodificado idêntico; geometria de imagem ≤0,001 pt; zero pixels diferentes nas duas fontes a 72/600 dpi em ambos os builds |
| Preservação R8/R9 | 28 PDF + 22 SVG de R8 e 75 PDF + 75 SVG de R9 por build |
| A–H/R1–R7 | 21 replays e 5 suplementos, incluindo 198 símbolos públicos |
| Estrutura | Lint completo sem erros/warnings; V1/V5/V15/V26 e linhagem reversa dos 1003 consumidores sem erros |
| Recursos | Gates de R6 mantidos: RSS geométrico 0.951613× R5; executável normalizado 42787368 bytes (0.855199× R5); sete workloads e três stresses aprovados |
| Preservação | 2128 inputs recebidos fora de R10 intactos; 36370 arquivos históricos verificados |

Recibos: [closing-audit.json](../../lab/results/construction/diagnostics/superscript-r10-2026-10-24/closing-audit.json), [pipeline.json](../../lab/results/construction/diagnostics/superscript-r10-2026-10-24/pipeline.json), [validation-complete3/validation-commands.json](../../lab/results/construction/diagnostics/superscript-r10-2026-10-24/validation-complete3/validation-commands.json), [revalidation/audit.json](../../lab/results/construction/diagnostics/superscript-r10-2026-10-24/revalidation/audit.json) e [preservation-audit-close.json](../../lab/results/construction/diagnostics/superscript-r10-2026-10-24/preservation-audit-close.json). As medições de recursos usam a receita de R6, um job, teto configurado de 12 GiB e zero swap, sem builds concorrentes. Os números relativos a R5 preservam esse denominador; não são apresentados como ganho de R10 sobre R9.

## Limites remanescentes

O conjunto de 16 refutadores herdados de R9 agora passa integralmente. Isso fecha os seis casos gerais de `super` e preserva o fechamento anterior da nota longa; não certifica toda combinação possível do elemento.

A nova via Type3 exige um conjunto integralmente suportado de até 256 glifos PNG por fonte. Conjuntos maiores, formatos raster não suportados e outros caminhos recusados mantêm o fallback anterior e não recebem declaração de paridade por este lote. Os limites anteriores de R2, R3, R4 e pintura R5 continuam explícitos na auditoria de replays. R10 não declara paridade universal.
