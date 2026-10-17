# S1–S16 — plano de aceitação após aprovação

1. S1: API externa positiva com Size(Length) e campo Option<Length>; migração absoluta de todos os adapters inventariados. Nenhuma assinatura/trait nova.
2. S2: constructor named/positional18pt,2em,2em+3pt; seis permutações Color/Length/body e alias.
3. S3: nested18pt→2em=36pt e2em+3pt=39pt, incluindo escopos irmãos e restauração.
4. S4: set posicional/nomeado, múltiplos sets relativos, capture/show-heading e margem/leading derivados do tamanho correto.
5. S5: nomeado não consome posicional do mesmo tipo; sobras e body inválido têm erro/hint/span pinados.
6. S6: body None/Str/Content, ausência, body nomeado e tamanho/fill sem body. Validar sequência de ocorrências e precedência, sem perder spreads/dupes.
7. S7: collapse com/sem base typed; prefixo local_delta_since conserva declaração mesmo igual ao default; round trip preserva função afim.
8. S8: Typed e custom text.size no mesmo nó mantêm precedência explícita; cadeia pai não duplica em.
9. S9: Raw defaults/show-set, body/inline/raw.block.size; pontes math e scripts mantêm tamanho Pt resolvido, sem fator duplo.
10. S10: heading, parágrafo global/preparado, Box/inset e contexto com tamanhos absolutos antigos preservam geometria.
11. S11: zero/negativos medidos; medir NaN/infinito antes de formular requisitos. Não impor positividade não existente no pin.
12. S12: preservar corpo, ordem, efeitos/avaliação única e presença de Styled. Igualdade de dois Styleds com valores diferentes continua achado separado: não alterar PartialEq Rust para forçar um resultado linguístico.
13. S13: mensagens/hints e spans causais (incluindo named/span sintético e parser dupes); cores sólidas verificadas em saída visual, não só transporte.
14. S14: probe de tamanho dos tipos materializados + carga sob12GiB/swap0/Cargo jobs1; medir impacto, sem alegar otimização prévia.
15. S15: RED funcional antes de implementação; suíte completa/build/Clippy -D warnings/-D unfulfilled_lint_expectations; Rustdoc -D warnings/quatro índices, fmt/diff, V1/V5/V15/V26 estrito/reverse, SARIF diferencial.
16. S16: revalidar Par/Box e os controles de RG1 em nova baseline; preservar a proposta pública RG1 e reconciliar somente adapters/L0 adicionais. Sem merge/release/push.

Os testes ainda não foram escritos ou executados contra a API nova. Este plano não é evidência GREEN. Paint Gradient/Tiling, suporte de seletor show text e igualdade Styled são frentes separadas; não declarar NativeText universalmente equivalente quando só TS1 passar.
