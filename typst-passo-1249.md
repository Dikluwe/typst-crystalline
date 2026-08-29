# P1249 — auditar glifos matemáticos e fallback textual SVG

**Estado:** FECHADO — MATERIALIZADO; CERTIFICADO FINAL PASS  
**Predecessor:** P1248  
**Saída:** texto/glifo SVG classificado sem confundir outline e ortografia.

Investigar o scope-out de glifos matemáticos diretos e o fallback quando fontes
não estão resolvidas. Medir conteúdo, posição, advance, transform, fill e forma
visível contra vanilla. IDs de glyph, ordem de defs e escolha mecânica entre
`<text>`/`<path>` não são gate quando a morfologia observável coincide.

Se exigir nova informação atravessando layout→export, parar no gate de mudança
de pipeline ADR-0127. Caso contrário, atualizar L0 e implementar somente o
subconjunto fechado, com fontes e hashes fixos.

## Resultado anterior retificado

`FrameItem::Glyph` já declara `style` e, portanto, `fill`; também transporta
`base_char`. A pipeline já usa esses dados para resolver e carregar a fonte
exata. O gap real é de transporte L3: o exporter recebe a lista de fontes, mas
não a associação entre cada glifo direto e o `FontKey` resolvido.

O fallback textual sem fontes é outro caso: permanece scope-out explícito e
válido dos wrappers page-only, não `CONTRACT-GAP` do carrier.

## Proposta executada

Foi escrita primeiro a proposta L0 para contexto L3 imutável de identidade de
fonte, compartilhada por SVG e pipeline através de Núcleo Tekt. A alternativa
não amplia `FrameItem::Glyph`, não procura `glyph_id` incidental em fontes e
não refaz shaping. Fonte ausente ou associação ambígua permanece `Unknown`.

O dono aprovou a arquitetura em 2026-08-28 e pediu melhoria de engenharia. A
associação foi refinada para mapear `GlyphFontRequest` normalizado ao `FontKey`
completo, sem índice posicional ou identidade de travessia. O runner reproduz
oito fronteiras e cinco opções; nenhum código produtivo, oracle ou mutante foi
executado. O próximo gate é o preseal segregado.

## Fechamento anterior — superado pela materialização abaixo

Neste ponto intermediário, P1249 estava fechado apenas como saneamento
arquitetural e ainda não declarava glifos matemáticos diretos preservados no
SVG de produção. A materialização final abaixo substitui esse estado. Texto sem
fontes continua scope-out e não integra essa alegação.

O saneamento P1249 não bloqueia a auditoria do P1250.

## Materialização segregada final — 2026-08-28

O contexto L3 imutável associa `GlyphFontRequest` normalizado ao `FontKey`
completo resolvido pela pipeline, recursivamente em grupos, links e envelopes
semânticos. O exporter extrai o `glyph_id` somente da face associada, aplica
`size / units_per_em`, posição e inversão Y uma vez, preserva fill/alpha e não
reaplica `x_advance`. Ausência, conflito, fonte mapeada ausente, face inválida,
escala inválida ou outline indisponível permanecem `Unknown`; o wrapper sem
fontes não fabrica texto ou outline.

O preseal reparado preservou 12 observáveis, 21 oráculos e rejeitou 15/15
mutações, score `1.0`. A verificação final executou cinco testes P1249 e 41
testes da suíte SVG, além de build, lint completo, V5/V15/V26 e diff-check.
As testemunhas distinguem duas faces reais com `glyph_id` colidente, variantes,
assembly de três peças, transform, não interferência do advance, outline exato
na mesma face e fatores exatos de escala.

Veredito: `PASS_SEGREGATED_FRAGMENT_UNATTESTED_ISOLATION`, limitado ao fragmento
P1249. A segregação é processual em filesystem compartilhado; não se alega
isolamento ambiental forte nem equivalência geral de glifos SVG.

Certificado: `00_nucleo/diagnosticos/p1249-final-certificate.tsv`.
