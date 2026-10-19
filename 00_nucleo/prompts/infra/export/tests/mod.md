# L0 — infra/export/tests/mod
Hash do Código: 99722711

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/shaped-face-identity.toml sha256:96fe00a0c6071bb88ffc63806b3e42c23b78c398fa26d46866e0f099ab0d4e36
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:3ee5760ffa44b5fd889b9d6772972e3b8417be332e13726cd9eea24b32e686f8
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:4d947acae2d9bf8baff2639b7c9c1bdebb7b260e7ca4ac282d4f9d0e7a7feaee

**Layer:** L3 (test-only)
**Exclusive consumer:** `03_infra/src/export/tests/mod.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0126, ADR-0127, ADR-0128, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/tests/mod.rs:36` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## PDF test aggregation

Own the existing cross-feature PDF cfg(test) suite, shared fixtures/helpers and
declarations of specialized test modules. Preserve meaningful test names/assertions and restricted
access to the public/internal exporter API. Keep genuine joint gradient/image/font/
layout coverage rather than duplicating each child suite in this aggregator.

Specialized test owners are the corresponding files under
`00_nucleo/prompts/infra/export/tests/`: archival core, attachment AF, ICC/CIDSet,
empty A4f, UA identification/structure, spot resources and page labels.
They own coverage, not an alternative productive export contract.
Production behavior is owned by the builder and stream descendants.

## Verification and exclusions

Run the existing export::tests suite and specialized modules without using a
historical fixed test total as an acceptance criterion. Check cross-feature
resource/geometry/text closure. Do not claim global language parity, conformance
or performance from test counts or full-file byte equality. Independent validators
and snapshot harnesses remain separate evidence.

## Migração do carrier bidi ratificado (2026-10-05)

Medição anterior à decisão: o inventário `lab/results/construction/diagnostics/bidi-contexto-2026-10-05/inventory.json` identifica as referências desta unidade e os owners 1:1. O contrato do manifesto 1ae7513e foi confirmado; nesta unidade ajustar constructors não certificados com None e patterns de leitura com `..`, preservando os observáveis de linguagem existentes. Transformações obedecem ao núcleo pinado de origem; sem ampliar fase/API.


## Face efetiva de TextShaped — materialização ratificada 2026-10-05

Medição anterior à decisão: `lab/results/construction/diagnostics/identidade-fonte-shaped-2026-10-05/{initial.json.gz,probe.txt,bilateral.json}` distingue GIDs da regular de seleção Oblique. O campo público `TextShaped.font: Option<Font>` do owner layout_types foi confirmado pelo humano (SHA f4a17206500baf5952f898135944d55f9fca905285eafc995315ead1121dd409). O snapshot e inventário desta rodada estão em `lab/results/construction/diagnostics/identidade-fonte-shaped-implementacao-2026-10-05/`.

Este owner migra apenas suas referências: constructors legados usam None; transformações preservam a fonte; comparação/junção shaped discrimina face e eixos. Emissão do shaper fornece clone da fonte efetiva também após cache hit; não alterar semântica/estilo para induzir seleção. Collectors/subset/CMap e PDF usam binding da mesma face original com coordenadas do estilo. PNG/SVG preparam bindings locais e instanciam uma vez por face/pedido antes de render, mantendo índice de recursos original distinto dos bytes instanciados. None segue resolução legada. API pública e FontKey existentes permanecem. Não criar fontes artificiais, unsafe, globals ou fase nova. Funções privadas podem receber fonte/bindings explicitamente.

A associação é por bytes da face original e pedido de eixos; nomes e slots não bastam. A tabela local dura a exportação e conserva ordem dos recursos fornecidos, anexando faces faltantes. Para multifont, distribuir itens pelos bindings uma vez antes de coletores por fonte; não reutilizar GIDs/CMap globais em faces diferentes. Código deste owner continua 1:1 com seu L0; referências a outros owners não compartilham propriedade. Aceitação RED→GREEN e controles bilaterais incluindo miss/hit/VF, sem promessa de ganho de tempo.

Claims compartilhadas desta migração pertencem ao núcleo shaped-face-identity pinado acima; este owner materializa somente as referências da sua unidade.

## Invariante TJ após subset variável — 2026-10-05

Medição anterior à decisão: `03_infra/src/export/tests/mod.rs:9622-9626` exige GID original 223 no /W. O log infra-tests.log desta frente mostra CID 1 com /W=1020 após subset CFF2→TrueType. O teste foi escrito quando CFF2 não era subsetado. O observável a preservar é /W(CID realmente emitido) − delta TJ = x_advance, não o número do GID original (ADR-0107). Corrigir somente o lookup de largura: extrair o CID hexadecimal do único glifo no array TJ e buscar essa entrada em /W; preservar a fórmula, valor de avanço e nome do teste. Não enfraquecer o teste ignorando delta ou largura. Implementação produtiva permanece com seu owner.

## Imports e documentação de CI — 2026-10-13

### Medição anterior à decisão

`03_infra/src/export/tests/mod.rs` nas linhas 649, 695, 2077, 3186, 3237, 3289, 3807, 5091, 6548, 6549, 6550, 6551, 6621, 6622, 6756, 7123, 7195, 7313, 7424, 7494, 7495, 7637, 7638, 7639, 7843, 7870, 8021, 8022, 8023, 8024, 8025 contém os avisos selecionados. Fontes/owners, spans, saída original do compilador, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, status e diffstat estão em `lab/results/construction/diagnostics/infra-imports-docs-ci-2026-10-13/before.json.gz`. A pesquisa distingue referências produtivas, imports necessários somente à suíte descendente e menções em comentários; não constitui nova medição de paridade.

### Decisão e aceitação

Remover somente os imports privados comprovadamente sem uso neste consumer; manter os reexports públicos e os imports exigidos por chamadas ou contratos. A lista exata de spans anterior à edição está no diagnóstico citado.

Corrigir a separação/continuação de parágrafos e listas dos comentários Rustdoc. Preservar o conteúdo técnico e todas as asserções/casos de teste.

Manutenção interna sem mudança de assinatura pública, default, fase, fórmulas ou critérios de aceitação (fluxo contínuo ADR-0127). Não introduzir supressões de lint. Revalidar compilação de todos os targets, nomes e execução da suíte de infraestrutura, build, Clippy do workspace, Rustdoc e linhagem com resselo restrito. Não criar testes que apenas espelhem remoção de imports ou formatação de comentários.

## Testes observáveis, sem contagem histórica como prova

### Medição anterior à decisão

`03_infra/src/export/tests/mod.rs:6348,6522,7030,7395,7776,8069,8490`
contém casos cujo único comando é `assert!(true, ...)`: eles registram
reaplicações históricas de padrões, sem observar comportamento.
Em `:8023`, o teste P279 de texto em grupo verifica apenas `cm` e seus
comentários ainda aceitam texto descartado. O teste P281 em `:8125` já
verifica tanto `(hello) Tj` quanto `cm`; os casos CIDFont, multifont,
glyph, linha e grupo aninhado seguem na mesma suíte.
Estado anterior, trechos completos e posições estão em
`lab/results/construction/diagnostics/testes-observaveis-ci-2026-10-13/before.json.gz`
(HEAD, working tree não commitado, status, diffstat e UTC).

### Decisão e aceitação

Remover os sete casos de asserção constante e o sentinel P279 redundante
que aceita o antigo stub. Esta exceção explícita à preservação de nomes
não permite retirar testes que observem saída, nem reduzir asserções dos
casos restantes. Preservar as verificações de bbox em Block/Boxed/Group,
deduplicação de patterns e emissão de Shape/Image/Text/Glyph/Line em grupos.
O histórico dos casos retirados pertence ao diagnóstico, não à suíte executável.

A lista registrada deve perder exatamente esses oito nomes, sem adições;
o restante da fonte, exceto cabeçalho de linhagem e linhas vazias, permanece
idêntico. Executar a suíte de infraestrutura e conferir compilação, formato,
linhagem e inventário de CI. Não atribuir paridade a contagens de testes ou
à simples presença de operadores. Não introduzir supressões de lint.
Manutenção interna de testes, sem mudança produtiva ou de contrato público
(fluxo contínuo ADR-0127).

## Cálculos numéricos dos exportadores — 2026-10-13

### Medição anterior à decisão

`03_infra/src/export/tests/mod.rs` nas linhas [3911] usa métodos apontados pelo Clippy. `clippy.toml` exige alternativas sem dependência da matemática da plataforma; libm =0.2.11 já está pinado no core/Cargo.lock. Os fontes da biblioteca, consumer, owner, manifest/lock, spans, HEAD, working tree não commitado, UTC, status e diffstat estão em `lab/results/construction/diagnostics/gradientes-numericos-ci-2026-10-13/before.json.gz`.

`before-renders.json` registra 24 fontes renderizadas em PDF/SVG por Tekt e pelo vanilla pinado 9f2b6e8715237cb086899a42873660fe744622e8, com hashes dos binários e comandos. `before-comparison.json` mede o canvas inteiro sem máscaras ajustadas pelo erro. Existem divergências anteriores; o corpus não é prova de paridade integral. A primeira tentativa de fixture usava hsv fora do namespace color, foi rejeitada pelo parser antes de qualquer mudança produtiva e está preservada em initial-fixture-error.log; cases.json contém as entradas válidas.

### Decisão e aceitação

No teste de controle de arco de 90 graus, usar a identidade tan(pi/8)=sqrt(2)-1 com a constante f32 SQRT_2 para a referência independente do offset. Conservar inputs, nomes e tolerâncias. Não calcular o esperado chamando a mesma tangente da implementação.

Dependência direta interna de L3 fixada em =0.2.11 no manifest, reutilizando a versão já no lock. Não atualizar versões de pacotes nem adicionar I/O. Mudança interna em fluxo contínuo ADR-0127, sem contrato público, default, fase ou estratégia novos. L0 precede código e resselo é limitado aos owners envolvidos.

Validar as fórmulas com a suíte existente, exportação dos mesmos 24 documentos antes/depois e leitura raster/estrutural de PDF/SVG. Comparar a preservação local separadamente das divergências contra o pin vanilla; nenhuma diferença nova de render é aceita sem análise. Não alargar tolerâncias/limites, mudar fixtures ou supressões para esconder uma falha. Compilar todos os targets, executar a suíte infra, verificar build, Clippy, Rustdoc, formato e linhagem. Contagens e identidade de arquivos não certificam a paridade da linguagem.

## Navegação por destinos nomeados — 2026-10-14

Medição anterior: labels-fronteira-2026-10-14/reader-probes.json.gz confirma
que os destinos não são resolvidos pelo MuPDF, embora os testes P460/P463
procurem /Dests, /sec1 e /D /sec1 nos bytes. O snapshot destes testes está
em destinos-pdf-2026-10-14/before.json.gz. O owner produtivo
00_nucleo/prompts/infra/export/builder/navigation.md agora exige árvore
de nomes e referências por strings; a antiga representação misturava dois
formatos PDF incompatíveis.

Preservar os nomes dos testes, labels cobertos, conversão y-up e exigência
de ação GoTo. Atualizar a observação para a árvore /Names e a mesma string
UTF-16BE no destino e na ação, sem exigir o token inválido /sec1. Não relaxar
para mera presença de /Dests: verificar o par chave/array na árvore e o
operando concreto da ação. A resolução real pertence à verificação com
leitor independente; bytes isolados não certificam navegação.

## Geometria e amostragem cônica — 2026-10-14

### Medição anterior à decisão

`conic.rs:142-143,241-242` fixa centro/raio; `:271-272` associa paradas CMYK
por índice circular, ignorando offsets e a amostra final. `resources.rs:508-552`
fixa coordenadas de 8 bits e Decode unitário. As sondas antes/depois de
posicionamento e refuters no diagnóstico conic-posicionamento-2026-10-14 provam
cantos brancos, centro ignorado e offsets CMYK sem efeito, apesar de sample()
responder corretamente. before.json.gz em conic-geometria-amostragem-2026-10-14
conserva fontes/L0, HEAD, working tree, diffstat, UTC e os binários. O corpus
bilateral mede centro e cores nos eixos/diagonal, quadrado/retângulo e rotação.
A fonte pinada typst-pdf/src/paint.rs:274-299 e os pixels concordam: o início
é à esquerda e o sentido é horário; o código Tekt anterior não corresponde.
O vanilla observado nesses controles não é um fallback uniforme.
PDF Reference 1.3, tabela 4.31 (página PDF 250), permite coordenadas 16 bits e
Decode independente por eixo; é contrato de formato, não igualdade upstream.

### Responsabilidade e aceitação

Preservar cobertura de Type 6, componentes, flag, determinismo, extensões e
recursos. Substituir assertions que congelam contagem de patches/coords de 8 bits
por verificações de stride válido (RGB 61, CMYK 65), flag, componentes e amostras
nos endpoints. Testes de helpers históricos continuam explícitos como históricos,
sem alegar política produtiva. Decode CMYK exige seis pares, intervalos geométricos
ordenados cobrindo a referência e quatro pares [0,1], não uma string unitária.
Renomear testes cujo nome promete contagem obsoleta. Testes funcionais RED no
owner conic verificam centro, cobertura, origem/sentido/aspecto e offsets/limites
laterais; render independente deve confirmar as propriedades na imagem.

Correção interna de fórmula/paridade ADR-0127, sem contrato público externo,
modo/flag padrão ou mudança de fase. L0 precede testes RED e código. Não mudar
expectativas funcionais para absorver erro; assertions mecânicas obsoletas são
substituídas pela verificação independente descrita. Comparar corpus completo,
validar qpdf, raster, todas as regressões e linhagem/reverso. A referência
binária cônica só pode ser regenerada após demonstrar a mudança intencional
pelo harness e pelo protocolo vigente do corpus.

## Coordenadas parent sem dependência de relógio — 2026-10-14

A suíte first-validation/workspace.log de conic-geometria-amostragem-2026-10-14
falhou em p273_6_shape_inside_block_carries_parent_bbox_observable_diff.
parent-failure-comparison.json compara os PDFs extraídos do próprio panic:
somente Info e XMP mudaram, com datas em segundos consecutivos; o Shading é igual.
A igualdade integral de bytes testa o relógio, e assert_ne pode passar apenas
pela data, sem comprovar mudança de coordenadas.

Nos quatro testes parent/self de Block/Boxed P273.6/P273.7, comparar as quatro
coordenadas do único Shading linear. O helper deve exigir um ShadingType 2,
um único Coords e exatamente quatro números; não aceitar ausência silenciosa.
Verificar o valor de page já medido no PDF e as relações originais: parent
menor altera coordenadas; page explícita equivale ao fallback; Self ignora parent. Preservar documentos, espaços, paints e modo de exportação.
Não alterar relógio global, metadata de produção ou snapshots. Esse diagnóstico
não redefine critérios de outros testes PDF; a evidência é específica desses
quatro controles de coordenadas. Reexecutar o grupo e a suíte completa.

O primeiro ensaio do novo teste acrescentou valores Y-up presumidos para Block
(772) e Boxed (692), sem medição prévia; parent-tests-green.log registra sua
refutação (70 e 150). Essa suposição do fixture não deve redefinir o contrato
vigente de apply_parent_transform, que aplica origem+span dos valores recebidos.
Conservar o escopo original dos quatro testes: relações de coordenadas, agora
sem dependência de metadata. A interpretação física do Y dos parents lineares
fica explicitamente aberta para medição causal em outro diagnóstico; não se
legitima 70/150 como posicionamento físico correto por fixá-los como snapshots.

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
consumer as construções/desestruturações antes da edição: [178, 652, 698, 711, 741, 790, 805, 1075, 1344, 6514, 6625, 6719, 6727, 6799, 6929, 7362, 7433, 7506, 7577, 7585, 7707, 7777, 7832, 7840, 7928, 7979, 8043, 8108, 8192, 8234, 8279, 8287].
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

Migrar fixtures de FrameItem::Shape para FramePaint/FrameStroke, preservando valores e asserts semânticos existentes. Tiling deve construir célula preparada explícita; fixtures de avaliação continuam declarativas. Não adicionar conversão produtiva total nem enfraquecer asserções.

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


## Inicialização explícita dos fixtures — field_reassign_with_default

### Medição anterior à decisão

`03_infra/src/export/tests/mod.rs` nas linhas 8205, 8550 cria defaults e em seguida sobrescreve campos locais de testes. A fonte, os spans e os L0 anteriores estão em `lab/results/construction/diagnostics/field-reassign-2026-10-15/before.json.gz`, com HEAD, working tree, UTC e diffstat; initial-diagnostics.json conserva os inicializadores apontados. Esta manutenção trata representação Rust, sem nova medição de paridade ou mudança no default do produto.

### Responsabilidade e aceitação

Inicializar font de style_b e fill do estilo local nos literais TextStyle. Conservar font_b.clone(), o vermelho RGB e Styles do teste de underline; este lote não remove o estilo conceitual sem uso nem redefine a cobertura desse teste. Não alterar documentos ou asserts de recursos, grupos e underline.

Usar `..Tipo::default()` para os campos restantes e retirar `mut` apenas onde deixam de existir mutações. Sem novos tipos, builders, APIs ou supressões de lint. L0 precede código e resselo restrito (fluxo contínuo ADR-0127). O gate selecionado deve falhar antes e passar depois; preservar nomes, inputs, expectativas e tolerâncias dos testes. Revalidar Clippy all-targets, suíte workspace, build, formato, Rustdoc e linhagem. Não criar testes que apenas espelhem a sintaxe do inicializador.


## Manutenção local e observação de testes — 2026-10-16

### Medição anterior à decisão

`03_infra/src/export/tests/mod.rs` nas linhas 6083 contém os avisos registrados em `lab/results/construction/diagnostics/clippy-locais-2026-10-16/before.json.gz`. O snapshot conserva fontes/L0, hashes, HEAD, working tree não commitado, UTC e git diff HEAD --stat. A inspeção dos consumidores e usos precedeu esta decisão; o gate Clippy selecionado deve falhar antes do código. Manutenção de representação/observação Rust, sem nova medição de intenção upstream ou paridade.

### Responsabilidade e aceitação

Substituir apenas o corpo de p273_6_layouter_parent_bbox_consumed_by_block_arm: o antigo Some(Rect) seguido de is_some/unwrap não exercita layout. O mesmo teste deve construir Shape com gradiente Parent dentro de Content::block 200x100 e executar layout real, observar bbox emitido 200x100, exportar PDF e conferir extensão horizontal de 200pt e diferença do mesmo Shape fora do bloco. Não inventar bbox manual como entrada da prova. Conservar o nome e todos os controles adjacentes/expectativas. Esta cláusula supersede preservação literal somente desse smoke test vazio de observação produtiva; não permite retirada de testes de bbox.

Fluxo contínuo ADR-0127: sem contrato público, default de produto ou fase novos. L0 precede código e resselo restrito. Nenhuma supressão de lint nova. Conferir redução exata do inventário e ausência de avisos novos, gate Clippy RED→GREEN e unfulfilled_lint_expectations como erro; executar suíte workspace, build, formato, Rustdoc e linhagem sob 12 GiB, swap zero e um job. Não criar testes que apenas espelhem derive, Entry ou posicionamento de módulos. O caso de bbox exige observação real conforme especificado; os demais controles existentes mantêm fixtures e expectativas.


### Exaustividade do observador bbox — correção após o primeiro gate

Medição antes da decisão: o primeiro gate completo desta etapa, em `lab/results/construction/diagnostics/clippy-locais-2026-10-16/first-validation/validation-architecture.log`, identificou V16 em export/tests/mod.rs:6092 no novo emitted_bbox, `_ => None`. Clippy já havia medido 79 avisos; o gate barrou fechamento por aumentar V16 de 327 para 328. O enum FrameItem vigente em entities/layout_types.rs:373–547 foi conferido antes da correção.

O helper deve enumerar todas as variantes explicitamente: Shape observa seu bbox; Group/Link/Semantic/TextReplacement percorrem os filhos; TextSeparator/Text/TextShaped/Line/Glyph/Image devolvem None porque não possuem parent_bbox_at_emit nem filhos. Não usar wildcard para variantes futuras: o compilador deve exigir uma decisão quando o enum crescer. Preservar a fixture Block/Shape, os asserts de bbox/PDF e todas as outras correções desta etapa. L0 primeiro, resselo apenas deste owner/consumer, revalidar o gate Clippy e toda a suíte/linhagem sem elevar baseline ou suprimir V16.


## Migração das fixtures de PageContext — 2026-10-16

### Medição anterior à decisão

A primeira compilação all-targets em `lab/results/construction/diagnostics/clippy-aridade-interna-2026-10-16/first-validation/validation-clippy.log` encontrou cinco E0599 neste consumer (linhas 8166, 8229, 8309, 9753 e 9947), apesar do build produtivo aprovado. O arquivo e este L0 conferem com os hashes globais do snapshot inicial; conteúdos antes da edição e extensão de escopo estão preservados no diagnóstico. Os chamadores e cada expressão foram lidos antes desta decisão.

### Responsabilidade e aceitação

Migrar as cinco chamadas de fixtures aos antigos PageContext::cidfont/multifont para with_font_scenario e FontScenario existente nomeado. Preservar, na mesma ordem, todas as expressões dos quatro recursos, mapas, fontes, bitmap, modo e Box::leak já existentes nas fixtures; não limpar ou mudar lifetime neste passo. CIDFont mantém bitmap None; Multifont mantém bindings Some(LookupCache::default()) uma vez por contexto. Adicionar somente import privado FontScenario. Manter nomes, fixtures, asserts, strings e números esperados integralmente. Não restaurar wrappers antigos nem silenciar erro de compilação. As chamadas são internas ao módulo de testes, sem API de produto nova.

Fluxo contínuo ADR-0127: L0 antes do código e resselo restrito deste owner/consumer. Conferir cada argumento antigo contra o campo novo e equivalência do restante do arquivo após mascarar exclusivamente as chamadas migradas/import/hash. Reexecutar compilação all-targets, suíte workspace completa e gates da etapa com 12 GiB, swap zero e um job. Não alterar esperados para obter GREEN nem incluir testes que espelhem o carrier.

## Owned test module declarations

This consumer declares the following separately owned test modules: `pdfa_core_metadata_tests`, `pdfa_associated_files_tests`, `pdfa_icc_version_and_cidset_tests`, `pdfa4f_empty_embedded_files_tests`, `pdfua_identification_tests`, `pdfua_structure_tests`, `pdf_spot_colors_tests`, `pdf_page_labels_tests`. Their names describe their test responsibilities; each test module retains its own L0 owner. This module delegates the test bodies and does not absorb their ownership.
