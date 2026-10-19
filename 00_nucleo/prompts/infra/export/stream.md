# L0 — stream
Hash do Código: 83199c99

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/shaped-face-identity.toml sha256:96fe00a0c6071bb88ffc63806b3e42c23b78c398fa26d46866e0f099ab0d4e36
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:08c39c6b9cf6fdc571564646d12d71f9b12f2a5d2c029ca90f9253c39c28bb95
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:79d5c48d0b9d0fb7d995207d18b1f9437209bf35c78a35cbfcedccea8b90abd2
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:3ee5760ffa44b5fd889b9d6772972e3b8417be332e13726cd9eea24b32e686f8
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:4d947acae2d9bf8baff2639b7c9c1bdebb7b260e7ca4ac282d4f9d0e7a7feaee


**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/stream.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/stream.rs:54` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Façade contract

Own FontScenario, PageContext, borrowed resource/font maps, constructors and internal
compatibility reexports. Preserve Type1/CIDFont/Multifont static variants, existing
visibility and supplied graphic mode. Raw context constructors initialize tags to
Disabled and UA1/oracle to false; product defaults belong to the export façade,
not these constructors. Preserve independent tagging/oracle settings. Descendants
receive this context; font/image/gradient resources are prepared outside this owner.

Own the deterministic page-local MCID prepass in with_pdf_tags. Restart at zero per
page and traverse `page.items` in the current content order. This prepass does not
traverse the page background or foreground layers. Formula, Label and TH/TD own
their MCIDs outside Artifact. Artifact suppresses descendants; UA1 boundary carriers
consume nothing, semantic owners stop further UA1 leaf collection, and real Text,
TextShaped, Glyph and Image leaves receive UA1 MCIDs outside those owners. Group/Link
recurse. Disabled tagging does not populate marked-content assignments. Expose the
same assignments to drawing and builder structure construction, not parallel rules.

Keep the existing inline regression suite owned by this consumer. Page assembly,
recursive painting, text operators and shape primitives belong only to the children
below; this façade does not own their algorithms or PDF stream compression.

The façade reexports the private marked Glyph emission variants to the draw
owner for Large-operator occurrence identity. This is internal composition of
existing PDF stages: it does not choose text from GIDs, create a new mode or
alter the ordinary Glyph emission API.

## Delegated owners

- `00_nucleo/prompts/infra/export/stream/draw.md` owns `03_infra/src/export/stream/draw.rs`.
- `00_nucleo/prompts/infra/export/stream/page.md` owns `03_infra/src/export/stream/page.rs`.
- `00_nucleo/prompts/infra/export/stream/shape.md` owns `03_infra/src/export/stream/shape.rs`.
- `00_nucleo/prompts/infra/export/stream/text.md` owns `03_infra/src/export/stream/text.rs`.

## Separadores textuais — materialização autorizada em 2026-09-28

### Medição anterior à decisão

`03_infra/src/export/stream.rs` é um consumer do percurso FrameItem. As sondas RED fonte→PDF de
`lab/results/construction/diagnostics/fatia-funcional-tekt-2026-09-28/` e a inspeção
dos handlers desse consumer precedem esta alteração. A aprovação do contrato
público em `00_nucleo/prompts/core/entities/layout_types.md` foi recebida em
2026-09-28 e registrada na proveniência da materialização.

### Responsabilidade deste owner

Declarar o módulo descendente separator; manter despacho estático e recursos do stream existentes.

Conservar as responsabilidades existentes; a aceitação segue texto e geometria
observáveis, incluindo os controles de adjacência e os invariantes pinados.

Em UA1, TextSeparator recebe o mesmo tratamento de folha textual Span e MCID coordenado, preservando enclausuramento e semântica do parágrafo.

## Identidade textual — materialização aprovada

### Medição anterior à decisão

As sondas bilaterais e o estado exato em
`lab/results/construction/diagnostics/identidade-hifen-discricionario-2026-09-28/initial.json`
e `before/` medem U+002D versus SHY com geometria coincidente em oito controles.
A inspeção do percurso FrameItem deste consumer está arquivada em approved-before/.
O dono confirmou o contrato em `00_nucleo/prompts/core/entities/layout_types.md`.

### Responsabilidade exclusiva

Declarar módulo privado text_replacement. No mapa de tagging, descer nos filhos, sem novo MCID próprio do envelope.

O Núcleo text-replacement define apenas invariantes compartilhados; este prompt
possui somente o consumer declarado. Aceitação: texto/caixas, testes causais,
transporte recursivo e controle de hífen literal; não bytes PDF ou nova seleção.

## Migração do carrier bidi ratificado (2026-10-05)

Medição anterior à decisão: o inventário `lab/results/construction/diagnostics/bidi-contexto-2026-10-05/inventory.json` identifica as referências desta unidade e os owners 1:1. O contrato do manifesto 1ae7513e foi confirmado; nesta unidade ajustar constructors não certificados com None e patterns de leitura com `..`, preservando os observáveis de linguagem existentes. Transformações obedecem ao núcleo pinado de origem; sem ampliar fase/API.


## Face efetiva de TextShaped — materialização ratificada 2026-10-05

Medição anterior à decisão: `lab/results/construction/diagnostics/identidade-fonte-shaped-2026-10-05/{initial.json.gz,probe.txt,bilateral.json}` distingue GIDs da regular de seleção Oblique. O campo público `TextShaped.font: Option<Font>` do owner layout_types foi confirmado pelo humano (SHA f4a17206500baf5952f898135944d55f9fca905285eafc995315ead1121dd409). O snapshot e inventário desta rodada estão em `lab/results/construction/diagnostics/identidade-fonte-shaped-implementacao-2026-10-05/`.

Este owner migra apenas suas referências: constructors legados usam None; transformações preservam a fonte; comparação/junção shaped discrimina face e eixos. Emissão do shaper fornece clone da fonte efetiva também após cache hit; não alterar semântica/estilo para induzir seleção. Collectors/subset/CMap e PDF usam binding da mesma face original com coordenadas do estilo. PNG/SVG preparam bindings locais e instanciam uma vez por face/pedido antes de render, mantendo índice de recursos original distinto dos bytes instanciados. None segue resolução legada. API pública e FontKey existentes permanecem. Não criar fontes artificiais, unsafe, globals ou fase nova. Funções privadas podem receber fonte/bindings explicitamente.

A associação é por bytes da face original e pedido de eixos; nomes e slots não bastam. A tabela local dura a exportação e conserva ordem dos recursos fornecidos, anexando faces faltantes. Para multifont, distribuir itens pelos bindings uma vez antes de coletores por fonte; não reutilizar GIDs/CMap globais em faces diferentes. Código deste owner continua 1:1 com seu L0; referências a outros owners não compartilham propriedade. Aceitação RED→GREEN e controles bilaterais incluindo miss/hit/VF, sem promessa de ganho de tempo.

Claims compartilhadas desta migração pertencem ao núcleo shaped-face-identity pinado acima; este owner materializa somente as referências da sua unidade.


## Memoização local da associação — medição 2026-10-05

`first-benchmark.json` desta rodada mede +25.36ms na fixture grid320 e +14ms em rtl160/mixed160. `font_binding.rs:index` compara bytes completos por consulta; `runs.rs:verbose_run_compatible` compara Font por bytes em cada adjacência. Inferência causal, a confirmar por A/B.

Usar cache privado por tabela de recursos/exportação para positivos validados: conservar clone Arc da fonte para manter sua memória viva e index após igualdade de bytes/pedido na primeira ocorrência. Acerto exige identidade de slice (endereço e comprimento) e mesmo pedido completo. Não serializar endereço ou criar identidade pública por ponteiro. Fontes de alocações distintas consultam igualdade completa na primeira vez. Não cachear miss no binder que anexa recursos; índices positivos permanecem estáveis por append. Nenhum global ou reúso entre sessões. Comparação de merge usa identidade de slice como atalho para igualdade, mantendo comparação de bytes para fontes de alocações diferentes. Este cache não memoiza nem invalida conteúdo entre Worlds.

### Refinamento de escopo após compilação — 2026-10-13

Após remover o import sem uso do facade export, check-initial-failed.log aponta o reexport privado draw_item_local desta unidade (linha 43). A pesquisa da fonte encontra somente o alias e menções em comentários neste owner; draw preserva sua definição e chamadas próprias. Remover somente esse reexport pub(super), conservando os demais reexports exigidos por descendentes, PageContext, FontScenario e algoritmos. O snapshot anterior está em additional-before.json.gz, no mesmo diretório do diagnóstico. Correção interna sem API pública/default/fase novos. Evidência e estado: `lab/results/construction/diagnostics/infra-imports-docs-ci-2026-10-13/`. Revalidar todos os targets e preservar nomes/assertions da suíte.

## Import de chave após posicionamento cônico — 2026-10-14

Medição anterior à decisão: green.log em
lab/results/construction/diagnostics/conic-posicionamento-2026-10-14/ aponta import
sem uso de dedup_key_for em stream.rs:11. O helper shape agora consulta
explicitamente a chave por ocorrência no facade gradients; nenhuma outra
referência neste owner usa esse alias. facade-before.json.gz conserva o
estado, L0 e fonte anterior. Remover somente esse import privado, sem mudar
PageContext, reexports ou algoritmos. Manutenção interna ADR-0127;
compilar e revalidar a suíte e a linhagem sem supressão de diagnóstico.

## Referencial de pintura — transporte aprovado T1b — 2026-10-14

### Medição anterior à decisão

O manifesto `d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d`
foi aprovado pelo humano para T1–T3; recibo em
`lab/history/construction/records/00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`.
A evidência bilateral de Parent e controles Solid está no contrato aprovado;
ela não é substituída por inspeção estrutural. A rodada
`lab/results/construction/diagnostics/paint-root-2026-10-14/before/results.json.gz` conserva
fontes, estado sujo, HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, UTC,
diffstat e hashes dos binários; cobre contêineres, Place e Rotate com Solid
e Gradient Parent. O inventário before-source/syntax-nodes registra neste
consumer as construções/desestruturações antes da edição: [788].
layout_types.rs:531–539 não distingue raízes de grupos técnicos; helpers.rs:253–269
reconstrói o Group e precisará preservar o campo. Box agrega fundo/corpo;
Block mantém body plano exceto no clip. Preservação do referencial é requisito
de linguagem; formato dos envelopes é mecânica. A aprovação não fecha as
divergências gerais de geometria previamente medidas em Rotate/Box.

### Responsabilidade exclusiva

Migrar construções e padrões de Group para o campo paint_root aprovado. Grupos técnicos novos e fixtures legadas são false; reconstruções transportam o valor recebido, sem inferir raiz a partir de dimensão, clipping ou transformação. Preservar os demais campos e o comportamento de desenho/recursos. Exportadores ainda não usam este metadado para patterns; essa integração é T2 e não está certificada aqui.

### Aceitação e incompletude

L0 primeiro, testes RED→GREEN, resselo restrito; conservar geometria e texto
nos controles fonte→PDF/SVG e revalidar suíte/build/doc/check/fmt/linhagem.
A migração deste metadado não substitui Shape.fill/stroke por FramePaint nem
implementa patterns PDF/SVG/raster; T1c/T2/T3 continuam explicitamente abertos
e já autorizados. Não alegar paridade de tiling ou produção a partir do build.


## Transporte preparado em Shape — T1c/T2 aprovado — 2026-10-14

### Medição anterior à decisão

`lab/results/construction/diagnostics/tiling-nativo-2026-10-14/before.json.gz` registra
HEAD, working tree, diffstat/UTC e fontes/owners antes desta migração.
layout_types.rs:510–511 ainda transporta Paint/Stroke; shape.rs:151–191
materializa só fill em grupos; tiling.rs:85 limita/clona a grade. O preparador
prepare_cell já retorna TilingFrame uma vez por ocorrência. As sondas
`tiling-preparacao-2026-10-14/{before,after}-renders/results.json.gz`
comprovam que dense-open e stroke-open ainda divergem nos três targets.
O contrato foi aprovado no manifesto
`d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d`;
recibo em `lab/history/construction/records/00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`.
A preparação/contagem de objetos é mecânica de transporte; render, contornos,
texto, geometria e diagnósticos são obrigações observáveis de linguagem.

### Responsabilidade exclusiva desta unidade

Contexto privado recebe mapa de referências de tiling por ocorrência; construtores mantêm ausência por padrão para fixtures sem tiling. Emissão decorativa de célula desabilita temporariamente tags/UA1, preservando cenário de fontes e mapas de recursos, e restaura os campos antes da página.

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

O stream decorativo de célula usa draw_item_local no espaço Y-down; a matriz do padrão faz a inversão externa. Não clonar itens, não usar altura de página artificial.

## Higiene da migração nativa

Medição: verified-v3/followup-clippy-inventory.json e followup-architecture.log, com candidate-final e workspace-pre próprios, registram quatro f32::hypot proibidos, um argumento adicional, dois itens produtivos depois de módulos de teste e dois wildcards neutros novos.

Decisão interna em fluxo contínuo (ADR-0127): Posicionar build_cell_stream antes do módulo de testes, sem alterar assinatura ou comportamento privado. Não elevar limites nem adicionar allow. Validar suíte e renders antes/depois, além de formato, doc e linhagem.

## Composição de período e alpha sólido — 2026-10-14

### Medição anterior à decisão

`lab/results/construction/diagnostics/tiling-overlap-2026-10-14/before.json.gz` preserva fontes, owners, HEAD, UTC e working tree. `tiling-nativo-2026-10-14/final-renders` e `remaining-analysis.md` registram cell8×8/pitch4×4 alpha50%: PDF vermelho opaco; SVG só uma contribuição; PNG quatro contribuições (255,15,15). `pdf_tiling.rs:184` emite BBox=size, `svg/paints.rs:148` usa overflow visible; `stream/shape.rs:155` e `stream/text.rs:275` descartam alpha. São defeitos observáveis de composição; o vanilla também corta overlap e não serve de requisito para esse defeito. Hipótese refutável: compor explicitamente um período preserva controles sem overlap e quatro contribuições coincidem com fonte-over analítica.

### Responsabilidade desta unidade

Reexportar privadamente helpers de opacidade do owner shape para recursos/draw/text/page. Sem alterar PageContext ou APIs externas.

### Aceitação

Correção interna ADR-0127 dentro do T1–T3 já aprovado, sem nova API/fase/flag/default. L0 primeiro, RED→GREEN, composição analítica (incluindo ordem com cores distintas), controles simples/nested/rotated/text/image e extração/seleção de páginas. Build, suíte workspace, formato, Rustdoc e linhagem. Alpha variável de gradientes e qualidade PNG seguem incompletos; não certificar pacote de produção com esses itens abertos.


## Aridade das fronteiras — `clippy::too_many_arguments`

### Evidência anterior à decisão

A leitura estrutural abaixo usa HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado,
capturado em 2026-10-15T16:39:18.873477+00:00. Fontes completas, L0 anteriores, assinaturas,
hashes, UTC, status e `git diff HEAD --stat` estão em
`lab/results/construction/diagnostics/too-many-arguments-2026-10-15/before.json.gz`.
A classificação individual e as cláusulas deste L0 lidas antes da decisão estão em
`lab/results/construction/diagnostics/too-many-arguments-2026-10-15/classification.json`.
Os spans Clippy foram reconfirmados contra as fontes do inventário anterior.
É medição de estrutura Rust; não é nova prova de paridade ou de falha funcional.

- `C062`: `03_infra/src/export/stream.rs:224`, `cidfont`, 9 argumentos contados pelo Clippy (limiar 7).
- `C064`: `03_infra/src/export/stream.rs:255`, `multifont`, 11 argumentos contados pelo Clippy (limiar 7).

### Prescrição exclusiva por função

As correções abaixo são alvos internos de manutenção, ainda não materializados nesta auditoria.
Preservar as assinaturas públicas vigentes e as decisões de linguagem deste owner.
A necessidade de corrigir estrutura é julgamento de manutenção apoiado nos dados
explicitados; o aviso de aridade, sozinho, não demonstra um defeito funcional.

#### `C062` — `cidfont`: Correção interna prescrita

O construtor de PageContext desmonta quatro componentes de FontScenario::Cidfont apenas para remontar esse enum; os recursos de página e o modo são outro grupo. Reutilizar a fronteira FontScenario já existente na construção interna.

Compor PageContext com FontScenario já construído e os recursos de página/modo, em construtor comum interno. Migrar cidfont e seus chamadores; manter variantes e dados por empréstimo.

Aceitação: Sem clone de mapas/fontes; tags Disabled, UA1/oracle false antes dos setters, Type1/CIDFont/Multifont, MCIDs e recursos ficam iguais.

#### `C064` — `multifont`: Correção interna prescrita

O construtor Multifont repete a montagem de FontScenario com seis tabelas correlatas e os mesmos recursos/modo de PageContext. A composição deve receber o cenário já tipado, sem criar outro conjunto de arrays nem prometer validação de cardinalidade inexistente.

Usar o mesmo construtor interno que recebe FontScenario, mantendo as tabelas multifont existentes e LookupCache default. Não redesenhar o enum nem expor tipo público novo.

Aceitação: Preservar o alinhamento de índices fornecido pelo builder, glyph reverse/bitmap/eixos e seleção de face; nenhum novo fallback ou alegação de validação de arrays.

### Fronteira de materialização

Nesta auditoria, somente o L0 e seu selo de linhagem são atualizados; atributos de
supressão e mudanças de assinatura/corpo ficam pendentes da etapa de materialização.
As entradas com correção exigem testes de linguagem/integração do owner e controle
de chamadas antes/depois, sem relaxar fixtures ou tolerâncias. Manter as camadas,
a fase, os defaults, o despacho estático e a ausência de estado global mutável.
Correções de assinatura privada devem migrar também os chamadores e seus L0 antes
do código. Qualquer necessidade nova de contrato público ou compatibilidade externa
exige proposta concreta e confirmação ADR-0127; esta seção não a antecipa.


## Tipos explícitos e leitura local — type_complexity (2026-10-15)

### Medição anterior à decisão

03_infra/src/export/stream.rs:76 (C061); 03_infra/src/export/stream.rs:260 (C063).
O snapshot before.json.gz em lab/results/construction/diagnostics/type-complexity-2026-10-15
conserva HEAD, working tree não commitado, UTC, diffstat, fontes/owners e hashes.
O gate red-clippy.log falha com este lint promovido a erro antes da edição.
A inspeção distingue tuplas posicionais, repetição do alias FontKey e contratos
estáticos já explícitos. É manutenção de representação Rust; não medição de
benefício da IA, bug funcional ou paridade nova.

### Responsabilidade e aceitação

Usar export::FontKey no campo fonts de FontScenario::Multifont e no parâmetro equivalente de PageContext::multifont, preservando o mesmo lifetime e slice emprestado. O alias já existe e não altera nenhum tipo efetivo, variante, field ou contrato de chamada. Retirar somente os três imports de tipos que deixam de ter uso; não materializar nesta fatia a reorganização de aridade prescrita separadamente.

Correção interna em fluxo contínuo ADR-0127: nenhuma entidade, assinatura pública
resolvida, default ou fase é alterada. Uma abreviação de tipo só é permitida com
expansão comprovadamente idêntica; incompatibilidade ou tipo público novo exige
outro gate. As exceções são nominais apenas para type_complexity; não autorizar
supressão em módulo/impl/crate nem elevar limiar. Esta permissão pontual supera
proibições genéricas anteriores somente para os três itens justificados neste lote;
continuam proibidas supressões dos demais casos destinados à correção.
Usar os controles funcionais existentes sem mudar seus esperados. O gate selecionado
deve passar RED→GREEN, incluindo unfulfilled_lint_expectations como erro; revalidar
build, Clippy all-targets, suíte workspace, formato, Rustdoc e linhagem com resselo
restrito. Não criar testes que apenas espelhem aliases ou nomes de campos.


## Aridade interna — materialização de seis casos, 2026-10-16

### Medição anterior à decisão

As fontes, chamadores, cláusulas vigentes, spans e inventário de 28 avisos foram conferidos antes da decisão. `lab/results/construction/diagnostics/clippy-aridade-interna-2026-10-16/before.json.gz` guarda fontes/L0 completos, hashes, HEAD, working tree não commitado, UTC e git diff HEAD --stat. A auditoria anterior classifica estes casos como correção; não são exceções diretas. O RED deve observar as aridades atuais antes de alterações; controles funcionais existentes serão corridos antes/depois. É refatoração de representação privada, sem nova paridade ou ganho de performance medido.

### Responsabilidade exclusiva e aceitação

C062/C064: substituir os construtores internos cidfont/multifont por with_font_scenario, recebendo os quatro recursos emprestados, FontScenario existente e StreamMode (seis argumentos). type1 conserva sua assinatura e delega com FontScenario::Type1. O novo construtor conserva todos os campos/defaults de PageContext: tiling_names None, tags Disabled, semantic_mcids vazio, ua1/oracle false, modo recebido. Não alterar FontScenario/visibilidade/variants ou criar validação de cardinalidade/clone de tabelas. Atualizar a fixture CIDFont do teste de stream para construir o mesmo cenário, mantendo seus asserts e bytes esperados.

Fluxo contínuo ADR-0127: nenhuma API externamente pública, default do produto, fase, dependência ou semântica é alterada. Tipos/constructors novos ficam internos; contratos públicos existentes permanecem. Este adendo concretiza somente os seis casos nominais; substitui proibições genéricas de supressão apenas para as duas expectativas residuais condicionadas C047/C066 após remover _text. As outras cinco aridades a corrigir continuam visíveis. Não elevar limiar ou silenciar no módulo/impl/crate. L0 precede código e resselo restrito. Confirmar preservação das assinaturas externamente públicas, campos/valores transportados, controles PNG/SVG/PDF, precedência/diagnósticos/recursos/MCIDs e ausência de mutações globais. Usar os testes existentes sem relaxar fixtures/esperados; não escrever testes que espelhem os carriers privados. Revalidar suíte workspace completa, build, Clippy all-targets e unfulfilled_lint_expectations, Rustdoc, fmt e linhagem estrita/reversa, mantendo 12 GiB, swap zero e um job.


## Guards de domínio explícitos — V17, 2026-10-16

### Medição anterior à decisão

T325: 03_infra/src/export/stream.rs:219. Fontes e L0 vigentes lidos antes da prescrição; SHA-256, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, status e git diff HEAD --stat em `lab/results/construction/diagnostics/tekt-guards-2026-10-16/before.json.gz`. O inventário anterior, reconfirmado por hash, registra 28 guards em 21 owners; a regra CI pinada manda desdobrar ou mover a condição para o corpo. A medição é estrutura Rust, não prova de defeito funcional ou de melhor leitura por IA. Nenhuma intenção upstream é inferida.

### Responsabilidade exclusiva e aceitação

Nos cinco tipos de folha de collect, mover ua1 && !inside_artifact ao corpo. Inserção por ocorrência e incremento de next continuam condicionados; preservar tags Disabled, semânticos donos de MCID, Artifact, recursão, ordem e escopo de página.

Manutenção interna em fluxo contínuo ADR-0127: sem contrato público, default, compatibilidade ou fase novos. A responsabilidade permanece somente no consumer exclusivo deste L0. Não criar helper apenas para esconder a conjunção, não adicionar supressões nem mudar a configuração do linter. Executar controles funcionais existentes antes do código e preservar seus esperados; o RED real é o gate V17 com 28 avisos, GREEN exige zero V17. Conferir caminhos de sucesso/fallback/erro, curto-circuito e efeitos; resselo restrito por owner. Revalidar build, suíte workspace completa, Clippy all-targets com warnings e expectativas não cumpridas como erro, formato, Rustdoc com warnings como erro, linhagem V1/V5/V15/V26 estrita/reversa e SARIF sem novos avisos. Executar sob 12 GiB, swap zero e um job. Não alegar ganho de memória/desempenho ou paridade global sem medição separada.


## FrameItem — exaustividade de decisões existentes, V16, 2026-10-16

### Medição anterior à decisão

F056: 03_infra/src/export/stream.rs:229 (collect). Os spans AST, patterns/guards/corpos e complements foram conferidos antes do código. `lab/results/construction/diagnostics/tekt-frame-items-2026-10-16/before.json.gz` conserva HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, fontes/L0/configuração, hashes, status e git diff HEAD --stat. `classification.json` registra os 70 casos e seus complements em 26 owners, usando a árvore do parser pinado do CI. Cláusulas pertinentes do L0 foram lidas junto da fonte; não há proposta de nova arquitetura ou interpretação de intenção upstream. Padrões e exaustividade Rust são mecânica; semântica/morfologia, geometria, efeitos e diagnósticos do produto devem permanecer.

### Prescrição exclusiva desta unidade

- **F056 — `collect`**: Trocar somente o padrão '_' pelo grupo nominal `Line`, `Shape`. Usar o mesmo prefixo Rust em escopo (`FrameItem`). Conservar integralmente o corpo atual do fallback, os braços anteriores, guards, transformações, chamadas, returns e ordem.

Manter callbacks, efeitos, metadados, clipping, fontes, positions, tinta e largura conforme os owners vigentes. Esta rodada não amplia travessias, não transforma wrappers em folhas nem usa text/ActualText como substituto da geometria. Helpers de teste mantêm fixtures, esperados, tolerâncias, contagens e falha Unknown; não aproveitar o saneamento para aceitar ausência de observação. Grupos nominais compartilham o mesmo corpo anterior: notas V19 de alternativas continuam visíveis, sem confundir cobertura de um braço com todas as alternativas.

### Gate e aceitação

Manutenção interna ADR-0127 em fluxo contínuo: nenhum enum/campo novo, método de trait/assinatura pública, default, dependência, compatibilidade ou fase nova. Não materializar propostas históricas pendentes por este adendo. L0 primeiro e resselo restrito em ambas as direções. Não adicionar supressões, alterar limiar ou reescrever match como if-let para esconder V16.

Antes do código, executar o controle novo de FrameVisitor e a suíte workspace nos mesmos esperados, e confirmar RED dos 70 avisos da família por leitura AST. Após código, zero V16 dessa família nos locais abrangidos e nenhum aviso novo; manter V16 externos como pendências. Nos matches do core produtivo alcançados por cargo check -p typst-core, usar cópia descartável com uma variante sentinela de FrameItem para medir ausência de E0004 nos locais antes e presença depois. A sonda cobre somente os locais efetivamente compilados: não extrapolar aos helpers cfg(test) ou L3 que a dependência impede de alcançar. Para todo o lote, conferir padrões/guards/corpos por replay, enum e assinaturas inalterados, build e testes reais. Não substituir corpos por stubs.

Revalidar suíte workspace completa, cargo build, Clippy workspace/all-targets com -D warnings e -D unfulfilled_lint_expectations, formato, Rustdoc -D warnings com quatro índices HTML, linhagem V1/V5/V15/V26 estrita/reversa, SARIF e diff --check. Fontes/L0/configuração ficam congelados durante comandos. Execuções pesadas sequenciais com MemoryMax=12 GiB, MemorySwapMax=0 e CARGO_BUILD_JOBS=1. Não apresentar saneamento estrutural como nova paridade global, ganho de memória ou benefício medido para IA.
