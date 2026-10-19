# V16 — representação e identidade

Escopo: sete owners de entidades (Color, adjustments, conversion, mixing, MathStyleKind, Func e StateUpdate), 11 avisos e fallbacks adicionais desses enums. Corrigir por padrões nominais, mantendo resultados e contratos. Snapshot anterior e classificação por match ficam em diagnosticos/tekt-enums-2026-10-16.

1. Medir fonte/owner e registrar inventário/hash/HEAD/working tree/UTC.
2. Prescrever no L0 cada padrão e seus fallbacks, qualificar descrições históricas conflitantes; manutenção interna ADR-0127 sem mudança de API/default/fase.
3. Escrever controles de comportamento e executar antes do código; gate V16 RED por escopo. Sonda de evolução em cópia descartável acrescenta variantes apenas para verificar E0004 nos matches selecionados; não mudar enum produtivo.
4. Substituir somente padrões; manter corpos, ordem de braços/guards/calls e definições dos enums. Formatar e ressellar pares selecionados, validar ambas as direções.
5. Executar sonda GREEN, suíte/build/Clippy estrito all-targets/Rustdoc/fmt/linhagem/SARIF, sob 12 GiB, swap zero e um job.
6. Preservar T027/T158/T160/T311/T313 do lote anterior e inventário restante; emitir relatório/manifesto, sem release.

## Fechamento

Etapa concluída em 2026-10-16T19:53:47.869347+00:00. 21 matches nominalizados; 11 avisos e uma nota V16 removidos; nove fallbacks sem emissão também protegidos. Definições de enums e APIs preservadas. Sonda de evolução: 0/21 → 21/21 matches exigem E0004 na cópia com sentinelas. Workspace 8621 aprovados, zero falhas, 7 ignorados; Clippy zero, build/fmt/Rustdoc/linhagem direta/reversa aprovados. Pico final 12 GiB, teto aplicado, swap zero e sem OOM. Relatório e manifesto em diagnosticos/tekt-enums-2026-10-16; índice AST seguinte em diagnosticos/tekt-wildcards-2026-10-16. Meta geral aberta: 312 V16, 19 V12 e locais preservados sem emissão. Sem commit, push ou release.
