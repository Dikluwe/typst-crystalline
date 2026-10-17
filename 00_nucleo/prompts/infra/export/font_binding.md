# L0 — infra/export/font_binding
Hash do Código: 2dd39803

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/shaped-face-identity.toml sha256:96fe00a0c6071bb88ffc63806b3e42c23b78c398fa26d46866e0f099ab0d4e36
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:4d947acae2d9bf8baff2639b7c9c1bdebb7b260e7ca4ac282d4f9d0e7a7feaee

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/font_binding.rs`

## Medição anterior à decisão

`00_nucleo/diagnosticos/identidade-fonte-shaped-2026-10-05/probe.txt`
demonstra GIDs da regular associados à Oblique por resolução posterior do
estilo. `shaper/shaping.rs:254-255,358-362` perde a face e
`export/stream/text.rs:22-43` usa apenas a chave de estilo. O contrato
TextShaped.font foi confirmado (f4a17206500baf5952f898135944d55f9fca905285eafc995315ead1121dd409).
Isto é integridade mecânica necessária ao desenho correto, não obrigação de
copiar bytes PDF ou algoritmos vanilla. A prova refutaria a hipótese se a
mesma fonte já fosse usada em ambas as etapas.

## Contrato privado

Possuir binding por face original e pedido completo (variante derivada +
eixos explícitos), independente de World/slot/família. Consumir FontKey público
existente, sem mudar seu alias ou assinaturas públicas. Conservar os recursos
legados fornecidos em ordem; anexar face real faltante na primeira ocorrência,
percorrendo background/body/foreground e containers por visitor. Nome de família
continua real; não fabricar nomes/variants para simular identidade.

Para Some, índice deve corresponder aos mesmos bytes de origem e pedido;
não usar índice zero ou seleção de estilo quando falta binding. None conserva
seleção anterior. Binding PDF mantém bytes originais enquanto instancia/subset
ficam associados pelo mesmo índice. Preparação SVG/PNG possui arrays paralelos
originais e preparados, instancia uma vez por recurso, consulta o original e
retorna bytes preparados; falha segue fallback de instanciação existente.
Nunca buscar novamente fonte via World. Escopo de cache é uma exportação.

Distribuir folhas textuais/Glyph por recurso numa travessia; as views privadas
para coleta de glyphs, clusters, chars e CMap não possuem geometria de produto
nem modificam documento original. Não colocar GIDs de uma face na outra por
simples teste gid < number_of_glyphs. Não reparar identidade ToUnicode autoral
por ocorrência nesta frente.

Aceitação: testes RED→GREEN de fallback, duplicatas de metadados, None,
fontes variáveis/eixos e reutilização; tinta/texto/geometria bilateral e
PDF/SVG/PNG. Sem unsafe, estado global ou mudança de fase/API pública.

Claims compartilhadas desta migração pertencem ao núcleo shaped-face-identity pinado acima; este owner materializa somente as referências da sua unidade.

Glyph direto matemático é scope-out de identidade de fonte nesta rodada: as views conservam sua participação legada nas faces e o emissor usa o mapa reverso vigente.


## Memoização local da associação — medição 2026-10-05

`first-benchmark.json` desta rodada mede +25.36ms na fixture grid320 e +14ms em rtl160/mixed160. `font_binding.rs:index` compara bytes completos por consulta; `runs.rs:verbose_run_compatible` compara Font por bytes em cada adjacência. Inferência causal, a confirmar por A/B.

Usar cache privado por tabela de recursos/exportação para positivos validados: conservar clone Arc da fonte para manter sua memória viva e index após igualdade de bytes/pedido na primeira ocorrência. Acerto exige identidade de slice (endereço e comprimento) e mesmo pedido completo. Não serializar endereço ou criar identidade pública por ponteiro. Fontes de alocações distintas consultam igualdade completa na primeira vez. Não cachear miss no binder que anexa recursos; índices positivos permanecem estáveis por append. Nenhum global ou reúso entre sessões. Comparação de merge usa identidade de slice como atalho para igualdade, mantendo comparação de bytes para fontes de alocações diferentes. Este cache não memoiza nem invalida conteúdo entre Worlds.

## Cópia sob demanda do prefixo — medição 2026-10-08

`03_infra/src/export/font_binding.rs:117-127` no estado congelado em
`00_nucleo/diagnosticos/font-binding-memoria1-2026-10-08/before.json.gz`
executa `supplied.to_vec()` antes da travessia, copiando os Vec de bytes de
todos os recursos; se nenhum recurso for anexado, descarta a cópia e retorna
o slice original. A duplicação temporária é mecânica interna, não linguagem.
Inferência: eliminar essa cópia pode reduzir o pico; a hipótese de redução
de RSS será refutada se a bateria pareada não mostrar redução estável.

O vínculo do documento começa emprestando o prefixo fornecido. Somente na
primeira face realmente anexável ausente materializa uma cópia do prefixo;
depois anexa em ordem de primeira ocorrência na mesma tabela. A travessia
continua única, com os positivos locais válidos por append; miss não é
memoizado. Fonte sem família explícita nem metadados válidos não dispara
cópia. Documento vazio, fonte None e tabela já completa conservam empréstimo
sem copiar buffers de fontes. Preservar face/pedido, duplicatas, ordem de
background/body/foreground/containers e entrada original sem mutação.

Aceitação: RED→GREEN com testemunha privada do ponto de cópia (zero para
prefixo completo, uma para várias faces faltantes), equivalência de recursos
e exportações existentes; comparação pareada de tempo/RSS. Nenhuma mudança
de assinatura pública, fase ou comportamento padrão. Preparação SVG/PNG
continua com seus arrays paralelos; não ampliar o escopo desta correção.


## Transporte preparado em Shape — T1c/T2 aprovado — 2026-10-14

### Medição anterior à decisão

`00_nucleo/diagnosticos/tiling-nativo-2026-10-14/before.json.gz` registra
HEAD, working tree, diffstat/UTC e fontes/owners antes desta migração.
layout_types.rs:510–511 ainda transporta Paint/Stroke; shape.rs:151–191
materializa só fill em grupos; tiling.rs:85 limita/clona a grade. O preparador
prepare_cell já retorna TilingFrame uma vez por ocorrência. As sondas
`tiling-preparacao-2026-10-14/{before,after}-renders/results.json.gz`
comprovam que dense-open e stroke-open ainda divergem nos três targets.
O contrato foi aprovado no manifesto
`d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d`;
recibo em `00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`.
A preparação/contagem de objetos é mecânica de transporte; render, contornos,
texto, geometria e diagnósticos são obrigações observáveis de linguagem.

### Responsabilidade exclusiva desta unidade

Percorrer recursos visuais também dentro de fill/stroke TilingFrame para binding e partição de fontes. Usar walker privado de recursos; manter o visitor semântico L1 sem descida em paint.

### Aceitação e continuidade

Aplicar o núcleo pinado. A ratificação supersede cláusulas antigas que
aguardavam aprovação ou exigiam Paint/Stroke declarativo dentro de Shape,
somente no âmbito do contrato aprovado. Preservar os demais comportamentos
vigentes desta unidade. L0 primeiro e testes RED→GREEN; migração explícita de
fixtures e consumidores, sem conversão total From<Paint>, sem PartialEq nos
carriers e sem fallback sólido para obter build verde. Não elevar o limite
antigo nem transferir layout para L3. Validar transporte fill/stroke, recursos,
clipping, Parent afim, alvos nativos, controles Solid/Gradient, semântica de
página e toda a suíte/build/doc/fmt/linhagem. T1c, T2 e T3 só estão concluídos
quando integrados e verificados; estado intermediário quebrado ou parcial
não é certificação de produção. T4 e publicação continuam fora deste pacote.
