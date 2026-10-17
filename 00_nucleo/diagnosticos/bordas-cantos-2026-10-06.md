# Cantos das bordas de tabelas e grids

Corrigida a falta de tinta nos cantos das seis tabelas herdadas. A emissão
estende cada segmento por meia espessura em cada extremo, no próprio eixo,
sem mudar fronteiras das células, tracks ou texto. **18/18 controles de tinta
passam bilateralmente dentro de 1px a 600dpi**, incluindo três espessuras,
uma/duas colunas e LTR/RTL. O corpus herdado mantém **168/168**, sem regressões.

## Proveniência

Working tree não commitado, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`.
Estado anterior UTC `2026-10-06T12:50:37.730596+00:00`; corpus final iniciado em
`2026-10-06T12:55:46.319905+00:00`; raster isolado iniciado em
`2026-10-06T12:58:19.402705+00:00`. [before.json](bordas-cantos-2026-10-06/before.json)
conserva os dois consumers/L0 anteriores, stat/status e SHA do binário anterior;
[execution.json.gz](bordas-cantos-2026-10-06/execution.json.gz) conserva fontes
finais, stat/status, UTC, hashes e logs de testes/build. seals.json registra
os dois owners exclusivos. Alterações herdadas de outras frentes foram preservadas.

Binário Tekt anterior SHA-256 `f6820eeea52476b22ef837be276f3b11d263fa5caae5b4e73cb1e2c07b3c9750`;
final `c1d8b39ed7ac52c5a04fc364a3114215c36ae904ae6166d5c392a5a11d37c34a`. Coletas usam executáveis congelados.
Oracle vanilla pinado `9f2b6e8715237cb086899a42873660fe744622e8`, binário
SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`,
conforme proveniência ratificada P1489. Fontes em font-provenance.json.
SOURCE_DATE_EPOCH=0, RAYON_NUM_THREADS=1, tags desativadas, pretty=false.
Fontes das fixtures, comandos, extração bruta, XML/bbox e SHA dos PDFs/raster
estão nas coletas e scripts deste diretório. Não houve medição de tempo.

## Medição antes da decisão

No oracle, `lab/typst-original/crates/typst-layout/src/grid/layouter.rs:568-578`
e `:779-791` deslocam o início por menos meia espessura e acrescentam uma
espessura ao comprimento. No Tekt, `01_core/src/compiler/layout/grid/borders.rs:74-114`
encerrava os segmentos na fronteira das células. As seis tabelas do diagnóstico
anterior apresentavam 42–49 pixels de tinta do vanilla sem correspondência
bilateral dentro de 1px, concentrados nos cantos. Isso mede falta de cobertura
dos extremos; não prova intenção histórica nem exige copiar o mecanismo de
ordenação de strokes do vanilla.

O teste causal `border_segments_cover_half_thickness_at_both_ends` falhou
antes do código produtivo; o controle de segmentos vazios já passava. O primeiro
log exploratório red.log tem erro de tipo no teste, não é prova RED. A prova
comportamental é red-causal.log. Depois da correção, o teste herdado de colspan
esperava dx=100pt para tracks de 60+40pt e stroke de 2pt. O caminho pintado do
oracle mede 102pt. Sua expectativa foi ajustada no L0 de testes e no teste,
preservando a exigência de cobertura das duas colunas e espessura de 2pt.

## Decisão e implementação

Correção de morfologia visível das bordas, em fluxo contínuo ADR-0127:

- Horizontal: origem x0 − espessura/2; comprimento x1 − x0 + espessura.
- Vertical: origem y0 − espessura/2; comprimento y1 − y0 + espessura.
- Shape registra a dimensão correspondente ao caminho estendido. A extensão
  acontece só na emissão final, depois da coalescência; não se acumula no
  estado de segmentos abertos. Segmentos vazios continuam sem emissão.

Owners 1:1 e hashes locais ressellados. Nenhum novo unsafe, campo/assinatura
pública, dependência, flag, default ou mudança de fase. O exportador e a
mecânica de ordenação das bordas não foram modificados.

## Validação

**6.542 testes unitários L1 passaram, 5 ignorados**; testes de integração e
doc-tests do pacote também passaram. Builds debug/release e diff-check passaram.
Lint: **0 erros, 531 avisos, 1.578 notas**, sem diagnósticos acrescentados ou
removidos comparando regra/nível/mensagem/arquivo. V15/V26 íntegros; avisos
herdados não foram corrigidos globalmente nesta frente.

Corpus herdado **168/168** em texto sem formatação do extrator e caixas
agrupadas, tolerância <0,05pt. Os **18/18 controles isolados** passam também
nesse critério em modo verboso e compacto. As fontes e métricas desse recorte
foram mantidas; a correção muda tinta das bordas, não posição dos caracteres.
[summary.json](bordas-cantos-2026-10-06/summary.json) distingue cada coleta.

Raster gray a 600dpi, limiar 180, correspondência bilateral ±1px:
**zero pixels sem correspondência nos dois sentidos em cada um dos 18 casos**.
Inspeção visual confirmou cantos preenchidos tanto no traço padrão quanto
no traço de 3pt com duas colunas. Coleta em
[isolated-raster.json](bordas-cantos-2026-10-06/isolated-raster.json), com scripts
reprodutíveis. O critério é tinta nessa resolução/tolerância; não igualdade de
bytes/imagens nem certificação geral de strokes heterogêneos, caps e prioridade.

## Controle exploratório separado

A primeira matriz adicional envolveu a tabela em `align(left)`. Seus 12 casos
já divergiam antes na origem do frame: por exemplo, yMin do texto em 16,642578pt
contra 24,999978pt, e a tabela RTL de uma coluna ignorava a âncora Left do wrapper.
Continuam divergentes, sem regressão introduzida por esta correção. Evidência
integral em verification.json.gz, compact-verification.json.gz e raster.json.
São outra unidade de layout contextual, não falta de tinta dos cantos.

Para medir os extremos independentemente desse wrapper, os controles isolados
usam tabela direta com região preenchida pelas tracks e conteúdo idêntico nas
células. As duas matrizes foram preservadas; a falha exploratória não foi apagada
nem contada como sucesso. O recorte solicitado de cantos está fechado; a origem
geométrica de tabelas dentro de Align fica identificada para outra frente.
