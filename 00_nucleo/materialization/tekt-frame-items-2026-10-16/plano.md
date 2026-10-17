# V16 — FrameItem

Lote de 70 avisos em 26 owners, da árvore validada em tekt-enums-2026-10-16/final-state.json. Não altera enum, campos ou APIs.

1. Auditar cada match, guard, padrão de campo e L0 vigente; classificar complemento nominal apenas das variantes integralmente cobertas antes do catchall. Guards e padrões restritos não certificam cobertura integral.
2. Prescrever cada caso no seu L0 antes do código. Em FrameVisitor, remover o fallback inalcançável e seu allow: todas as onze variantes atuais já têm ramo; assinatura, callbacks e ordem ficam.
3. Testar a travessia das onze variantes e fronteiras de fallback antes da implementação; gate da família FrameItem RED. Baseline funcional workspace preserva a força dos observadores e asserts existentes.
4. Substituir padrões sem alterar corpos/fórmulas/fixtures. Fazer replay restrito e resselo direto/reverso.
5. Prova negativa de evolução no core produtivo em cópia com variante sentinela: medir E0004 somente nos matches produtivos que cargo check do core efetivamente alcança; não extrapolar a sonda aos helpers cfg(test) e L3 barrados pela dependência. Build/testes reais e conferência nominal cobrem o escopo restante.
6. Validar todo o workspace, Clippy estrito all-targets, Rustdoc, formato, linhagem e SARIF; 12 GiB, swap zero e um job. Manter pendências externas e notas informativas transparentes. Emitir relatório e manifesto, sem release.

## Encerramento

Etapa concluída em 2026-10-16T20:32:32.281211+00:00. As 13 validações passaram; 70 avisos V16 retirados e 261 avisos externos mantidos. Relatório: `00_nucleo/diagnosticos/tekt-frame-items-2026-10-16/relatorio.md`. Manifesto SHA-256 `21aa0a55f7ba60b9177627e9fe74ef6b9e206579f54fe3022c337507cafed453` conferido integralmente.

O usuário pediu pausa após a emissão do relatório. Não iniciar o lote seguinte; a meta geral permanece incompleta.
