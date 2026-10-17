# L0 — table
Hash do Código: ff1ef264

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/position-frame-carriers.toml sha256:ff71dfc6648d6deb4b2b8054e3d58a93777d17837129f0f1b1e7e138ffb6943c
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:16ae092d990d3d26f03232afcabf63f3b5c08ffb5b6297989368f0346d14f2ab
- 00_nucleo/prompts/_nuclei/table/row-groups.toml sha256:472e9e168c439ce5e1d17557fd8dbb55047e2f3c2f259eada2ff8b763ae27ade

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/table.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/table.rs:27`, `01_core/src/compiler/layout/table.rs:127`, `01_core/src/compiler/layout/table.rs:155`, `01_core/src/compiler/layout/table.rs:174`, `01_core/src/compiler/layout/table.rs:229`, `01_core/src/compiler/layout/table.rs:243`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Render an optional caption above the table, adding a prefix only when table.numbering is a String. Read the causal Table counter from the injected introspector with current fallback one, register its page for LoT, format or use decimal fallback, and retain the existing English `Table number: ` extension prefix. Caption remains outside the table carrier. This direct table-numbering extension is not a Figure and does not acquire figure selectors/show behavior by documentation.

Project visual children/header/footer and delegate all tracks, lines, align/inset, stroke/fill and visual row-group/pagination behavior to `00_nucleo/prompts/compiler/layout/grid.md`. Build the current parallel, visually empty Table semantic carriers after that layout, preserving summary and explicit cell kind/level/scope/spans in their current representation. Header Auto cells receive the current level-one Column classification; explicit Data remains Data. Uniform automatic headers produce grouped head/body carriers, mixed explicit Data retains direct rows. Place header logical rows only on the first affected page; retain the source's current sequential logical-row projection and even partition by affected pages.

The source does not construct logical footer rows, derive arbitrary header levels from TableHeader or align logical partition to measured visual breaks. Full Headers applicability relations and PDF/UA/AT/reflow proof remain unresolved, not promises of this sanitation. The older enclosing-frame proposal must not hide the actual parallel-empty-carrier behavior. Exporters own MCIDs and relation serialization; this owner creates no new layout pass.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

## D08B3-A — tabela resolvida na inserção; proposta pendente ADR-0127

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
00_nucleo/diagnosticos/table-cell-realizacao-d08b3-2026-10-07/
{before,additional,order,aliases}.json.gz registra pin 9f2b6e8715237cb086899a42873660fe744622e8,
UTC, diffstat/status, fontes/binários, argumentos e resultados. Oracle binário
7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea; Tekt
7c0957f45fe40290aa809e3358c9a87979aa1e28058511ad0ee978feea6713a2.
Vanilla lab/typst-original/crates/typst-library/src/layout/grid/resolve.rs:204–278 resolve campos, :1500–1530 rejeita
conflitos na ordem, :1665–1703 cria vazios, :1945–1988 deriva breakable.
Tekt grid_placement.rs:91–112 prioriza explícitas e :262–270 só conhece GridCell;
realize/mod.rs:160–200 visita receitas bottom-up; table.rs:127–152 despeja
wrappers sem atributos. order mede zero efeitos de células quando a tabela é
substituída e zero efeitos de heading quando a célula o oculta; aliases mede
LOCAL a, OUTER LOCAL a e NEW a. É linguagem, sem copiar mecânica/bytes/igualdade
Rust. Hipótese refutada por campos desconhecidos, efeito oculto ou ordem errada
após a mudança; cache/paginação não são inferidos como equivalentes.

### Responsabilidade exclusiva proposta

Table com plano completo consome cells/coords/roles do plano e delega a engine grid comum; não despejar wrappers, recalcular ocupação, executar receitas ou indexar corpos crus ocultados. Sem plano, conservar fallback histórico para fixtures/callers não preparados. Semantic carriers usam coords/spans/kind do plano, preservando modelo paralelo vazio e limites PDF/UA existentes; não duplicar header em body. Caption continua no owner atual.

### Gate, aceitação e divisão explícita

Proposta não vigente até confirmação humana do manifesto: há cinco campos públicos,
dois carriers públicos e uma fachada pública nova, além da fase lógica antes de Cell
show. Após ratificação, RED→GREEN, suites/build/lint/preservação restrita e coletas
D08A/B2, D05/D06/D07/query obrigatórios. Aceitação em linguagem/efeitos/geométricas
reais, nunca igualdade de bytes ou normalização de whitespace para fechar render.
Este primeiro pacote D08B3-A é incompleto no universo Cell: D08B3-B completa defaults
de Cell isolada (raw-cell-no-placement/outside-fields) e decisão separada de geração
fresca show-cell-reconstruct. Defaults/fields genéricos de Table (table-root-equality),
styles funções/paint/strokes/insets ricos, linhas/gutters ricos, spans multipágina e
conformidade PDF/UA continuam fora da certificação. Não mudar casts legados extras
do D08B2, sintaxe, flags ou política não-localizável global neste pacote.

### Espaçamento de tabela aninhada após fluxo inline — medição 2026-10-07

geometry-fifth.json (binário 59da863f, snapshot tested-fifth, pin/HEAD/estado/UTC)
mede CELL exterior yMin68.27018 e interior82.65818, contra68.27016/88.70814 no
vanilla: delta adicional6.05pt em texto11pt. É geometria observável, não bytes.
Fonte vanilla layout/container.rs:342 e owner vanilla_defaults.rs:14–23 fixam
BLOCK_SPACING1.2em contra PAR_LEADING0.65em; block.rs:106–134 usa baseline prévia
+ margem acima e inicialização geométrica. Hipótese: Table nested preparada após
inline estava consumindo leading de linha onde exige gap de bloco. Refutadores:
recoleta nos glifos e controles sem prefixo/cache/linhas/fontes.
No render de Table preparada dentro de célula, sem caption e com linha inline
precedente, fechar essa linha e usar baseline/descent prévios + margem acima
BLOCK_SPACING (collapse com margem pendente), mantendo initial_baseline_pending.
Não assar coordenadas em eval, chamar Engine/Func ou mexer em Table crua/primeira
linha sem prefixo. Correção de fórmula de paridade, sem contrato público/fase.

### Refutação do primeiro reparo de espaçamento

geometry-eighth.json, snapshot tested-eighth.json.gz e binário
4495f38a78cd27484991c83272b331b73bf089783acb001b9d58fa34cbc386d6
reproduzem o RED: gap interior ainda 6.05pt menor no Tekt.
sub_frame.rs:137–142 reinicia block_chain_active=false; line_commit.rs:460
atualiza a baseline da linha fechada sem ativar essa flag. Portanto a condição
block_chain_active não legitima suprimir a margem após uma linha inline
existente. No caso estreito especificado acima, usar a baseline recém-fechada
independentemente dessa flag. As condições de célula, plano, caption ausente e
linha não vazia continuam obrigatórias. Recoletar glifos antes de fechar.

### Simetria entre medição e emissão da tabela aninhada

geometry-ninth.json e after-ninth.json.gz refutam o segundo reparo: a célula
perde CELL a. tracks.rs:202 mede o body em sub-frame sem instalar regions.cell;
cell_body.rs:129 instala regions.cell somente para a emissão. A condição
regions.cell.is_some produz altura medida menor que a altura emitida. Usar a
fronteira física is_sub_frame para aplicar o mesmo espaçamento nos dois
percursos de Table preparada com prefixo inline, mantendo os demais gates.
A fórmula não depende de altura final, pois esse cálculo pertence à medição.
Aceitação exige sobrevivência dos três termos e seus glifos, não somente
deltas entre pares disponíveis.

## Posições no referencial composto — 2026-10-14

### Medição anterior à decisão

box-contexto-2026-10-14/probe.json.gz, fontes/binários/HEAD/working tree/UTC
preservados: Box, inset2pt, prefixo e aninhamento publicam (0,0), mesmo
quando a tinta se desloca. A diferença da baseline do context direto é
uma questão separada: não copiar o vanilla apenas por diferir. mod.rs:
1131-1148/1180-1200 escreve pontos locais em runtime.positions; Box agrupa
o corpo em outro referencial; finish.rs:295-302 sela o mapa sem recompor
esses pontos. Realized escreve com or_insert; labels têm mapa paralelo.
HTML já transporta markers privados pelo frame em html_frame.rs:30-199.
O registro single-pass e sealing por sub-store são preservados, ADR-0073/
0074 §§posição/modelo híbrido; não se adota introspecção global vanilla.
A inspeção do núcleo coordinates e seus owners mostrou usos absolutos
em grid e explicitamente locais em Place/Box. A qualificação do núcleo
conserva ambos e exige transformação até a página, sem dupla origem.
Snapshot anterior: posicoes-compostas-2026-10-14/before.json.gz.
Geometria/identidade são observáveis; o carrier e seu índice são mecânica.

### Contrato deste owner

No fechamento preliminar de linha em subframe para tabela resolvida sem caption, carriers posicionais diretos sozinhos não constituem conteúdo a fechar. Conservar todos os outros itens, incluindo separadores, e o restante da composição sem mudança.

Correção interna ADR-0127: sem API/campo público, flag/default ou mudança
de fase. L0 primeiro; RED→GREEN de coordenadas, ausência de tinta/altura
fictícia, identidade, ordem, clones visuais e rollback; revalidar os
controles HTML, geometria, contexto/queries e suíte completa.


## Aridade final — materialização interna, 2026-10-16

### Medição anterior à decisão

Fontes e L0 vigentes conferidos antes do código; snapshot 00_nucleo/diagnosticos/clippy-aridade-final-2026-10-16/before.json.gz conserva HEAD, working tree não commitado, UTC, status, diffstat e hashes globais. callers.txt registra busca no workspace, incluindo fixtures; as chamadas homônimas do math::MathLayouter são outro motor e permanecem intactas. Inventário anterior tem 22 avisos, cinco destas aridades. Trata-se de estrutura privada Rust, sem nova afirmação de paridade ou intenção upstream.

### Responsabilidade exclusiva e aceitação

Concretizar a chamada ao motor Grid com super::grid::GridLayoutInput: columns/rows de e, cells de visual_children, regras de e, gutter None, align/inset de e, visual_header/footer.as_ref e stroke/fill.as_ref. Preservar resolved/fallback, preparação dos visuais e todos os carriers semânticos posteriores.

Este adendo materializa somente o escopo nominal e supersede seu estado pendente da auditoria. Fluxo contínuo ADR-0127: sem contrato externo, default, fase ou compatibilidade novos. L0 primeiro e resselo restrito. RED de too_many_arguments antes do código, GREEN sem novas supressões ou alteração de limiar; expectativas anteriores devem continuar satisfeitas. Conferir corpos e transporte de valores, preservar fixtures/esperados, sem testes que espelhem carriers. Executar controles existentes de Grid/Table/outline e SVG/tiling antes/depois, suíte workspace completa, build, Clippy all-targets, Rustdoc, formato, linhagem estrita/reversa e Tekt sem novos avisos, sob 12 GiB, swap zero e um job. Resultados não certificam paridade global ou conformidade PDF/UA.


## Guards de domínio explícitos — V17, 2026-10-16

### Medição anterior à decisão

T166: 01_core/src/compiler/layout/table.rs:157. Fontes e L0 vigentes lidos antes da prescrição; SHA-256, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, status e git diff HEAD --stat em `00_nucleo/diagnosticos/tekt-guards-2026-10-16/before.json.gz`. O inventário anterior, reconfirmado por hash, registra 28 guards em 21 owners; a regra CI pinada manda desdobrar ou mover a condição para o corpo. A medição é estrutura Rust, não prova de defeito funcional ou de melhor leitura por IA. Nenhuma intenção upstream é inferida.

### Responsabilidade exclusiva e aceitação

Unir braços TableCell em visual_content. Resolved continua prioritário e clonado; somente wrapper sem nenhum dos nove atributos opcionais é removido recursivamente. Demais cells preservam o conteúdo completo, sem perder presença/semântica ou realizar body.

Manutenção interna em fluxo contínuo ADR-0127: sem contrato público, default, compatibilidade ou fase novos. A responsabilidade permanece somente no consumer exclusivo deste L0. Não criar helper apenas para esconder a conjunção, não adicionar supressões nem mudar a configuração do linter. Executar controles funcionais existentes antes do código e preservar seus esperados; o RED real é o gate V17 com 28 avisos, GREEN exige zero V17. Conferir caminhos de sucesso/fallback/erro, curto-circuito e efeitos; resselo restrito por owner. Revalidar build, suíte workspace completa, Clippy all-targets com warnings e expectativas não cumpridas como erro, formato, Rustdoc com warnings como erro, linhagem V1/V5/V15/V26 estrita/reversa e SARIF sem novos avisos. Executar sob 12 GiB, swap zero e um job. Não alegar ganho de memória/desempenho ou paridade global sem medição separada.


## RG1 — grupos Table ordenados e síntese estrutural — candidato 2026-10-17

### Medição anterior à decisão

Medição anterior: 00_nucleo/diagnosticos/table-row-groups-2026-10-17/before.json.gz, measurement.json.gz, supplement.json.gz, lines.json.gz, styles-phase.json.gz e svg.json.gz conservam HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, UTC/status/diffstat/fontes/binários/argv/cwd. Pin vanilla 9f2b6e8715237cb086899a42873660fe744622e8, binário 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea. table_grid.rs:126–150 funde grupos; row_groups.rs:22–35/70/355–366 conta entradas, table_cells.rs:1204 aplica receita antes do planner e :1259–1282 deriva prefixos. model/table.rs:307–316 sintetiza; typst-realize/src/lib.rs:367–385/558–568 sintetiza antes de receita; layout/grid/resolve.rs:1380–1457/1597–1650 preserva intervalos e cursores; typst-html/src/rules.rs:591–665 projeta papéis. TableHeader level/presença somem no Tekt, coordenadas de linha não são lidas em table_lines.rs:173–260, e supressão None em layout/grid/rules.rs:40–46 deixa traços. svg-analysis.json confere todos os strokes retos e mudanças pareadas (mesmo binário): top vermelho vanilla 82→41, Tekt 82→82; azul após None vanilla 41→0, Tekt 40→40. Estes são efeitos/células/roles/erros/geometria da língua; não obrigação de copiar vtables, estruturas Rust, bytes SVG/PDF ou inferir intenção upstream. O primeiro extrator SVG não reconhecia line/h/v; foi descartado como prova, conservado e substituído por extração completa registrada.

### Responsabilidade exclusiva

Consumir exclusivamente cells/coords/spans/row_groups/linhas de plano completo para geometria e carriers semânticos. Sem plano, callers/fixtures de layout cru usam adaptador puro do mesmo planner com propriedades lexicais, sem Engine, Func, preparação de receitas ou ocupação duplicada. Reusar motor geométrico Grid com entrada privada de posições resolvidas; não reposicionar como GridExplicitFirst. Semantic carriers seguem coords/roles efetivos, preservam Cell Header/Data explícito, caption/summary e não duplicam headers em body nem descartam footer. Source lexical e plano não são duas emissões. Preservar espaçamento e referenciais compostos vigentes. A partição multipágina/Headers aplicabilidade e PDF/UA permanecem etapa própria: não certificar com o conserto lógico.

### Gate, aceitação e incompletude

Contrato candidato RG1: não vigente antes da confirmação humana do manifesto em 00_nucleo/materialization/table-row-groups-2026-10-17/review.md. A aprovação precisa abranger contratos públicos, compatibilidade e fase (ADR-0127 §2.1/§2.3/§2.4). Após aprovação, esta seção supersede apenas cláusulas históricas conflitantes de grupos Table/linhas e da fronteira estrutural/receitas. Outras propostas não são ratificadas. Não implementar campo/API novo antes do gate. Instalar L0/kernel/pins primeiro, testes funcionais RED antes do código, sem tratar erro de compilação/setup como RED; GREEN mais R1–R20 e corpus bilateral, build, workspace completo, Clippy all-targets -D warnings/-D unfulfilled_lint_expectations, Rustdoc -D warnings/quatro índices, formato, V1/V5/V15/V26 e reversa, diferencial SARIF/replay/diff-check. V16 externos continuam abertos; não alterar severity, criar allow/expect ou esconder um complemento em if-let só para zerar lint. V19 tem justificativa local e corpo compartilhado quando a política é a mesma. Núcleos são compartilhamento normativo; cada L0 conserva exatamente um consumer. Sob MemoryMax=12 GiB, swap zero, Cargo um job, comandos pesados sequenciais; sondas/mutações usam target Cargo isolado e nenhuma variante mutante vai para target principal. Sem commit/push/merge/release ou pin novo.
