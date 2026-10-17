# Execução da composição de Box

Este passo não é L0; o owner é `00_nucleo/prompts/compiler/layout/boxed.md`.

A. Compor integralmente caixas não fracionárias com dimensão lógica,
inset, outset, baseline, recorte e corpo local. Validar testes e PDFs.

B. Migrar a composição das caixas width_fraction preservando o vínculo
entre a alocação no fechamento da linha e a região local do corpo/repeats.
Clip, fundo e dimensões precisam usar a alocação final; nenhum caminho
histórico deve ser declarado produção pronta apenas por preservar testes.

C. Revalidar composição aninhada com Block, decoração, alinhamentos,
parent bbox e corpus de CI; continuar os demais itens da auditoria geral.

Estado medido em box-composicao-2026-10-13.md: A permanece aberta para
a baseline da caixa pequena inicial sem texto; B e C também estão abertas.
Não interpretar os controles verdes de P+Box+Q como conclusão de A.

Seguimento em 2026-10-14: a baseline da caixa pequena inicial e o controle
de texto menor foram corrigidos e revalidados em
[box-baseline-inicial-2026-10-14.md](../diagnosticos/box-baseline-inicial-2026-10-14.md).
A etapa B agora tem as sondas de box-fracionaria-2026-10-14.md; demais
limites permanecem abertos. Esta nota não altera as medições históricas.
