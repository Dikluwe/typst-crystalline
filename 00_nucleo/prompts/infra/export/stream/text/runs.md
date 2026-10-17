# L0 — stream/text/runs
Hash do Código: bc9faca5

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/shaped-face-identity.toml sha256:96fe00a0c6071bb88ffc63806b3e42c23b78c398fa26d46866e0f099ab0d4e36
- 00_nucleo/prompts/_nuclei/layout/pdf-shaped-advances.toml sha256:d6da63fc3b0282aaeb2573e92e850f807a2691c04d3578c54e61c1f4e01a51f3
- 00_nucleo/prompts/_nuclei/layout/text-tracking.toml sha256:3b0c535436048536ecd3133fcf8299e88c128a62c222661f36b7bcc6d98a598c

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/stream/text/runs.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

Before extraction, `03_infra/src/export/stream/text.rs:621,655,696,767`
contained envelope compatibility, run-boundary selection, run emission and
rounded inter-item TJ cursor compensation. The source snapshot and tests are
preserved in `00_nucleo/diagnosticos/atomizacao-separadores-2026-09-28/`.
The bilateral separator probes in
`00_nucleo/diagnosticos/fatia-funcional-tekt-2026-09-28/` reproduce a text
extraction divergence against vanilla pin
`9f2b6e8715237cb086899a42873660fe744622e8`. They do not establish total PDF
byte equality, general reader behavior or the earliest historical failing build.

## Ownership and contract

Own compatibility, boundaries and emission of coalesced shaped prose runs.
Receive FrameItems and the prepared FontScenario/PageContext from the ancestor.
Keep the existing static interface within export::stream; allocate no font
resources and perform no shaping, I/O, instancing or language evaluation.

Coalesce only consecutive shaped prose with equal baseline and complete
emission envelope: font identity and variations, size, fill, weight,
bold/italic, tracking, direction, math flags/size and units_per_em. Empty
glyphs, incompatible envelopes, nontext, group/link scopes, math and bitmap
leaves terminate or prevent runs. UA1 call-site policy belongs to the page
owner; this unit does not independently decide tagging policy.

Emit balanced verbose envelopes through the text owner's prefix/render-mode/
tracking helpers. Select the prepared single-font or multifont mappings by
complete style. Preserve the supplied nominal/actual advances, offsets and
old-to-new GIDs. Every inter-item TJ boundary compensates from the effectively
rounded cursor to the next absolute x position; include per-glyph rounding
in that cursor. Do not replace absolute positioning with nominal advances.

## Acceptance and limit

This extraction preserves current text, geometry, scope boundaries, diagnostics
and rendering operators. Existing P979 and text-operator regressions verify
compatible grouping, envelope boundaries and rounded cursor positioning.
The missing separator provenance is unresolved by this structural extraction;
this L0 does not authorize inventing spaces from positive geometric gaps.

## Tracking shaped — soma única e estado PDF

### Medição anterior à decisão

`00_nucleo/diagnosticos/shy-autoral-2026-09-28/tracking-controls.json`
mede plain-verbose com desvio máximo 2.652pt contra vanilla pinado,
enquanto plain-compact tem 0.348pt. Estado, binários e comandos estão no
recibo daquela frente. O novo estado e a matriz estão em
`00_nucleo/diagnosticos/tracking-pdf-2026-09-28/initial.json` e `before/`.
`03_infra/src/shaper/shaping.rs:319-326` soma tracking aos avanços;
`03_infra/src/export/stream/text.rs:639,668` e
`03_infra/src/export/stream/text/runs.rs:107` emitem Tc novamente.
O cursor de runs considera os avanços e ajustes TJ, sem essa soma Tc.
O vanilla pinado aplica tracking nos glyphs em
`lab/typst-original/crates/typst-layout/src/inline/shaping.rs:1269-1292`.
É comportamento de espaçamento medido, não requisito de operador ou bytes.
A hipótese é refutável por avanços sem tracking ou ausência do Tc adicional.

### Decisão de paridade interna

TextShaped CIDFont/Multifont carrega os avanços autoritativos já ajustados:
tracking deve vir desses avanços somente. Tanto emissão single quanto
coalescida estabelece explicitamente Tc=0 dentro do bloco BT antes de TJ,
inclusive tracking ausente/zero, para eliminar estado herdado. O zero é
o estado normativo de espaçamento adicional, não uma exceção de fonte, GID,
fixture ou largura. Não subtrair tracking dos avanços nem inferir causa
pelo texto. Text e fallback Type1, que emitem Tj sem avanços shaped,
continuam usando seu tracking existente. Bitmap continua desenho de imagem.
Sem contrato público, flag, modo padrão ou fase nova: fluxo ADR-0127.
Arredondamento dos avanços, medição de layout e política de tracking em
ligaduras são frentes independentes; esta correção não os certifica.

### Responsabilidade exclusiva

Na emissão coalescida, usar push_shaped_spacing do owner ancestral em vez
de push_tc_if_tracking. O modelo de cursor continua sendo somente nominal,
avanço shaped e TJ arredondado. Preservar compatibilidade, boundaries e
compensação absoluta. Testes causais pela interface emit_verbose_text_run
ficam no consumer ancestral text, sem ownership compartilhado do código.

## Correção de tracking — 2026-09-29

### Medição anterior à decisão

`00_nucleo/diagnosticos/tracking-residual-2026-09-29/initial.json`, `results.json`, `summary.json` e `causal-checks.json` registram 36 entradas/108 PDFs contra vanilla pinado, fontes, UTC, HEAD e working tree completa. A palavra isolada acumula +0.052pt com tracking 0.2pt, mas fronteiras perdem um intervalo na adjacência, dois com um espaço e quatro com três espaços literais. ±1.2pt elimina a quantização sem eliminar as fronteiras. O código shaper/shaping.rs:225-232,319-326 arredonda em i32; text.rs:49-83 divide palavras; space.rs:15-33 aplica somente avanço nominal. O estado anterior à materialização está em `00_nucleo/diagnosticos/correcao-tracking-2026-09-29/initial.json`.

### Decisão interna e responsabilidade

Correção interna de paridade em fluxo contínuo ADR-0127, sem contrato público, flag ou fase nova. As cláusulas desta rodada supersedem somente a preservação dos limites de tracking descritos abaixo; demais políticas de render permanecem.

No cursor coalescido, usar a mesma correção fracionária que a folha single. Fronteira TJ de run com tracking não zero usa cinco casas, compensando o cursor efetivamente serializado; outros runs preservam arredondamento inteiro. Conservar posições absolutas, nominal/GID/offset, compatibilidade e envelopes.

Aceitação: testes RED→GREEN de fronteiras/seleção e interpretação independente Tc/TJ, controles bilaterais de tracking ±/zero, repetidos, adjacência e wrap; build e lint sem erros, resselo restrito.


## Face efetiva de TextShaped — materialização ratificada 2026-10-05

Medição anterior à decisão: `00_nucleo/diagnosticos/identidade-fonte-shaped-2026-10-05/{initial.json.gz,probe.txt,bilateral.json}` distingue GIDs da regular de seleção Oblique. O campo público `TextShaped.font: Option<Font>` do owner layout_types foi confirmado pelo humano (SHA f4a17206500baf5952f898135944d55f9fca905285eafc995315ead1121dd409). O snapshot e inventário desta rodada estão em `00_nucleo/diagnosticos/identidade-fonte-shaped-implementacao-2026-10-05/`.

Este owner migra apenas suas referências: constructors legados usam None; transformações preservam a fonte; comparação/junção shaped discrimina face e eixos. Emissão do shaper fornece clone da fonte efetiva também após cache hit; não alterar semântica/estilo para induzir seleção. Collectors/subset/CMap e PDF usam binding da mesma face original com coordenadas do estilo. PNG/SVG preparam bindings locais e instanciam uma vez por face/pedido antes de render, mantendo índice de recursos original distinto dos bytes instanciados. None segue resolução legada. API pública e FontKey existentes permanecem. Não criar fontes artificiais, unsafe, globals ou fase nova. Funções privadas podem receber fonte/bindings explicitamente.

A associação é por bytes da face original e pedido de eixos; nomes e slots não bastam. A tabela local dura a exportação e conserva ordem dos recursos fornecidos, anexando faces faltantes. Para multifont, distribuir itens pelos bindings uma vez antes de coletores por fonte; não reutilizar GIDs/CMap globais em faces diferentes. Código deste owner continua 1:1 com seu L0; referências a outros owners não compartilham propriedade. Aceitação RED→GREEN e controles bilaterais incluindo miss/hit/VF, sem promessa de ganho de tempo.

Claims compartilhadas desta migração pertencem ao núcleo shaped-face-identity pinado acima; este owner materializa somente as referências da sua unidade.


## Memoização local da associação — medição 2026-10-05

`first-benchmark.json` desta rodada mede +25.36ms na fixture grid320 e +14ms em rtl160/mixed160. `font_binding.rs:index` compara bytes completos por consulta; `runs.rs:verbose_run_compatible` compara Font por bytes em cada adjacência. Inferência causal, a confirmar por A/B.

Usar cache privado por tabela de recursos/exportação para positivos validados: conservar clone Arc da fonte para manter sua memória viva e index após igualdade de bytes/pedido na primeira ocorrência. Acerto exige identidade de slice (endereço e comprimento) e mesmo pedido completo. Não serializar endereço ou criar identidade pública por ponteiro. Fontes de alocações distintas consultam igualdade completa na primeira vez. Não cachear miss no binder que anexa recursos; índices positivos permanecem estáveis por append. Nenhum global ou reúso entre sessões. Comparação de merge usa identidade de slice como atalho para igualdade, mantendo comparação de bytes para fontes de alocações diferentes. Este cache não memoiza nem invalida conteúdo entre Worlds.

## Eixo real e espessamento sintético — correção interna 2026-10-05

Medição anterior à decisão: controle variable-axes-after.pdf tem fonte wght250 com hmtx e pontos glyf corretos, mas stream contém `2 Tr`/`0.800 w` pelo style.weight=700. Raster 600dpi revela tinta adicional que bbox de palavras não revela. A causa é `stream/text.rs:274`/calls shaped e `stream/text/runs.rs:105`, que reaplicam faux-bold apesar de wght efetivamente realizado. Fonte/eixos originais e semântica permanecem distintos.

Decisão de paridade visual (ADR-0107): somente para TextShaped com fonte certificada Some cuja fvar declara eixo wght, emissão verbose simples e coalescida usa preenchimento 0 Tr; não sintetizar peso por cima da instância variável já preparada. Helper privado shared no owner text verifica o eixo real somente quando o estilo pediria stroke. Manter Text/Type1, None e fontes estáticas no comportamento vigente, sem reescrever weight/bold semântico e sem nova API, fase, unsafe ou I/O adicional. Compact shaped já não aplica esse stroke. Faux-bold de fontes estáticas e outros formatos continua fora desta correção. Testar single+coalesced com weight700/wght250 em RED→GREEN e conservar None; raster e caixas bilaterais completos.


## Auxiliares de teste e bindings produtivos — 2026-10-14

### Medição anterior à decisão

`03_infra/src/export/stream/text/runs.rs` nas linhas [72] apresenta os avisos selecionados.
`00_nucleo/diagnosticos/ci-helpers-testes-2026-10-14/before.json.gz`
conserva fonte/owner, HEAD, working tree não commitado, UTC e diffstat.
references-before.json.gz registra chamadas, imports, módulos e referências;
o estado da suíte/inventário anterior foi verificado por hashes. A lista de
testes foi capturada antes da edição. Não se infere inutilidade funcional
apenas de dead_code: as chamadas privadas e sua compilação foram auditadas.

### Responsabilidade e aceitação

No teste inicial style.math de verbose_run_end, retirar somente o binding font sem uso. Conservar o binding e a consulta de face no braço multifont e nos outros lugares que realmente escolhem ou comparam fontes. Limites de run, bitmap, matemática, baseline e identidade efetiva continuam iguais.

Manutenção interna em fluxo contínuo ADR-0127: nenhuma API pública, campo,
default de produto, fase, dependência ou supressão de lint muda. O gate de
diagnósticos selecionados deve falhar antes e passar depois; ele não é um
novo teste funcional nem prova de paridade. Preservar a lista e as expectativas
dos testes existentes, executar a suíte completa, compilar lib/all-targets,
verificar build/rustdoc/fmt/linhagem e registrar o inventário total. Não criar
testes que apenas espelhem cfg ou remoção de bindings. Resselo restrito ao owner.
