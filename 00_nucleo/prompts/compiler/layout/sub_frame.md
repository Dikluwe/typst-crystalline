# Prompt L0 — sublayout isolado
Hash do Código: 35dbb30a

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:2ccbb1e5daf5f6806e58cd48272b1463fbc9107300c0bce6ea1c3ccf7b0b8748

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/layout/sub_frame.rs`

## Contrato e aceitação

Executar conteúdo numa SubLayoutRegion salvando/restaurando cursor, região,
linha, collectors e pending state em LIFO. Devolver items, altura, decorações e
fixups no mesmo referencial local. Variante inline isola somente items do body
e deixa o caller decidir emissão.

## P1391 — isolamento do arm de linha overlong

### Medição anterior à decisão

O arm one-shot P1391 pertence à linha exterior validada. Herdá-lo por um
sub-frame permitiria que um flush interno o consumisse ou que o estado interno
vazasse de volta ao caller.

### Decisão e aceitação

Ao entrar no sub-layout, salvar e limpar o arm; ao sair, restaurar exatamente
o valor exterior. O corpo local nunca consome o fechamento pendente do pai.
Esta regra é estado LIFO privado: não reconhece Content, não altera geometria
do sub-frame e não cria API, fase, alocação proporcional ou recursão.

## P1392 — preservação do mesmo isolamento

A validação P1392 reutiliza o arm one-shot sem alterar sua vida útil. O
sub-frame continua salvando, limpando e restaurando o estado exterior em LIFO;
nenhum flush interno pode consumir um arm P1391 ou P1392 do pai. Não há código
ou geometria adicional neste owner além da linhagem atualizada.


## P1493 — neutralidade métrica do carrier de posição HTML (RATIFICADO; gate ADR-0127 confirmado em 2026-09-24)

### Medição anterior à decisão

O carrier geométrico P1493 precisa permanecer em \`current_line\` para que a
variante inline e os callers de box o devolvam e transladem. Contudo,
\`layout_sub_frame\` usa qualquer item pendente como \`had_items\` e fabrica
altura tipográfica; \`layout_sub_frame_inline\` também aplica \`line_h\` quando
não encontra texto. Um carrier isolado alteraria altura e alinhamento da ilha,
violando a sonda 30pt/20pt e o requisito de não-vazamento.

### Contrato ratificado

Reconhecer somente o sentinel privado P1493 exposto pelo owner descendente
\`layout/html_frame.md\`. O item continua sendo drenado/devolvido na mesma
ordem e no mesmo referencial que qualquer \`FrameItem\`, mas não conta como
item visual para \`had_items\`, decoração, fallback de altura ou métricas de
linha. Quando coexistir com conteúdo real, o conteúdo real decide as métricas;
quando for o único item, altura e avanço permanecem os do ContextBlock no-op.

O subframe não extrai ids, não produz \`HtmlPosition\` e não remove carriers.
Essas responsabilidades ficam no owner \`html_frame\`. Fora da variante
detalhada, nenhum sentinel existe e o comportamento do subframe é idêntico.

### P1493 — revisão adversarial de wrappers

Somente o sentinel direto é excluído de `had_items`. Wrappers `Semantic`,
`Link` e `Group` preexistentes conservam a materialidade da fachada legada
mesmo quando contêm somente o carrier; a remoção posterior do sentinel não
pode mudar a decisão de fluxo já tomada.
