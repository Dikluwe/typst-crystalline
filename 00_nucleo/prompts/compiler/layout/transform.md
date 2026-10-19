# L0 — transform
Hash do Código: 80a0fc1e

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:6dbf8faa56960845c60734f5e047685ec5b3a6f14c0ce3a48d333b8982d0baa5
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:08c39c6b9cf6fdc571564646d12d71f9b12f2a5d2c029ca90f9253c39c28bb95
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:79d5c48d0b9d0fb7d995207d18b1f9437209bf35c78a35cbfcedccea8b90abd2
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:4d947acae2d9bf8baff2639b7c9c1bdebb7b260e7ca4ac282d4f9d0e7a7feaee
- 00_nucleo/prompts/_nuclei/transform-semantics.toml sha256:cbd8b56af347a35e867b2c4695581721d5983ec24ee7bb022e9f9987500d40f3

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/transform.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/transform.rs:26`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Apply the existing rotate/scale/skew/move matrix to a measured subframe around the resolved pivot and preserve Group/local-child representation, inline extent, reflow behavior and nested fixup paths. Bounds and pivot calculation ignore only direct private HTML markers; keep those markers among the transformed children. Centered reconstruction retains matrix.tx/ty in addition to pivot terms so move translations reach causal positions. Do not derive transformed bounds from marker coordinates, discard the point before Group composition or apply the parent origin twice. `00_nucleo/prompts/compiler/layout/html_frame.md` owns extraction/removal, not this unit.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

## Separadores textuais — materialização autorizada em 2026-09-28

### Medição anterior à decisão

`01_core/src/compiler/layout/transform.rs` é um consumer do percurso FrameItem. As sondas RED fonte→PDF de
`lab/results/construction/diagnostics/fatia-funcional-tekt-2026-09-28/` e a inspeção
dos handlers desse consumer precedem esta alteração. A aprovação do contrato
público em `00_nucleo/prompts/entities/layout_types.md` foi recebida em
2026-09-28 e registrada na proveniência da materialização.

### Responsabilidade deste owner

Transportar posição, avanço já consumido, estilo e origem do TextSeparator sem criar tinta, baseline ou novo avanço; manter ordem dos filhos.

Conservar as responsabilidades existentes; a aceitação segue texto e geometria
observáveis, incluindo os controles de adjacência e os invariantes pinados.

A segunda medição da AABB inline, após rebase dos sub_items, também exclui TextSeparator antes de acumular max_x. Transportar carrier não pode ampliar orig_w_exact nem deslocar o pivô.

## Identidade textual — materialização aprovada

### Medição anterior à decisão

As sondas bilaterais e o estado exato em
`lab/results/construction/diagnostics/identidade-hifen-discricionario-2026-09-28/initial.json`
e `before/` medem U+002D versus SHY com geometria coincidente em oito controles.
A inspeção do percurso FrameItem deste consumer está arquivada em approved-before/.
O dono confirmou o contrato em `00_nucleo/prompts/entities/layout_types.md`.

### Responsabilidade exclusiva

Tratar TextReplacement como envelope visual transparente nos percursos deste owner; descender nos filhos preservando text, ordem, coordenadas e o cálculo existente, sem novo algoritmo ou avanço.

O Núcleo text-replacement define apenas invariantes compartilhados; este prompt
possui somente o consumer declarado. Aceitação: texto/caixas, testes causais,
transporte recursivo e controle de hífen literal; não bytes PDF ou nova seleção.

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
consumer as construções/desestruturações antes da edição: [149, 303].
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

## Transformações tipadas — contrato substitutivo

## Medição anterior à decisão

`lab/results/construction/diagnostics/transformacoes-2026-10-14/{before,results}.json.gz` e
`controls/results.json`, `binding-controls/results.json`, `context-controls/results.json`
preservam HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree, UTC,
diffstat, fontes, comandos e hashes de binários. Pin 9f2b6e8715237cb086899a42873660fe744622e8.
TransformElem em entities/elements/transform.rs:19 armazena apenas matrix/body;
stdlib/transforms.rs:19/51/91/191 descarta argumentos e reduz valores à matriz;
layout/transform.rs calcula pivô central e usa has_orphaned como desvio de layout.
No pin, typst-library/src/layout/transform.rs conserva os quatro elementos e
typst-layout/src/transforms.rs resolve estilos, região, corpo, origin e reflow.

Sondas: fields vazio versus body/argumentos; func falso versus função nativa;
rotate(0deg)[X] == scale(100%)[X] true versus false; defaults explícitos colapsam.
Origin/reflow são ignorados ou rejeitados; scale(x:150%) escala também y no
Tekt, enquanto y permanece100% no pin; move perde percentual/em. Campos ausentes
devem continuar desconhecidos lexicalmente. set rotate(23deg) é válido, mas
angle nomeado não é; getters contextuais refletem estilo no pin. Os primeiros
controles contextuais rotularam o wrapper e são inválidos para medir value;
context-controls corrige a posição da label. HTML pinado avisa e elimina o body;
Tekt rejeita a operação, de acordo com seu contrato HTML estrito.

Semântica, sintaxe, morfologia, posições/avanços e diagnósticos são linguagem;
enum versus structs upstream e número de passos de medição são mecânica.
A hipótese é refutável por campos/presença que ainda colapsem, estilo assado,
origem incorreta, erro perdido, reflow falso alterando avanço ou body descartado.

Esta seção substitui cláusulas anteriores conflitantes somente para a família
Move/Rotate/Scale/Skew; demais responsabilidades permanecem.

## Resolução e geometria

Consumir TransformOperation explicitamente. Matrix mantém o comportamento
genérico anterior por um braço próprio; não usar essa compatibilidade como
fallback das quatro variantes nativas. Conservar assinatura privada, topologia
estática e representação Group com paint_root false para envelope técnico.

As variantes nativas medem frame lógico do body com tamanho/baseline reais,
incluindo espaços, boxes e conteúdo vazio. Não usar apenas bbox da tinta ou
arestas da fonte como substituto universal da medida. Ignorar marcadores privados
de HTML no cálculo sem removê-los do transporte. Preservar caminhos dos fixups,
separadores, Source, TextReplacement, recursos e collectors.

Move resolve Rel<Length> com a região do uso e em com o estilo de fonte atual;
aplica translação visual e mantém tamanho/baseline/avanço. Rotate/Skew resolvem
angles/defaults, e Scale resolve Ratio diretamente ou Length contra a dimensão
lógica medida do body; um eixo Auto acompanha a escala do outro. Dois Auto
produzem `x and y cannot both be auto` na resolução, inclusive via set. Preservar
escala zero e sinais negativos sem substituí-los por identidade. Medir casos
degenerados antes de escolher diagnósticos; não inventar limites por conveniência.

Origin resolve alinhamento por eixo sobre o frame lógico (topo do frame, não
baseline usada como origem arbitrária): T(pivot)*M*T(-pivot). Start/end usam
direção efetiva. Um alinhamento parcial faz fold com herdado/default. A posição
do parent é aplicada uma vez. Não inferir reflow pela presença de alinhamentos
órfãos, tipo do body ou coeficientes da matriz.

Reflow false mantém a caixa lógica e baseline do body; somente a tinta muda,
inclusive em contexto inline, block, place e nested. Reflow true transforma os
quatro cantos do frame, desloca pela mínima coordenada e ajusta caixa/avanço.
Scale sem flip vertical preserva baseline transformada; skew conserva a linha
transformada da baseline no meio do frame; rotate e flip vertical seguem os
observáveis medidos do pin sem fingir uma baseline de fonte para qualquer body.
Transformações não perdem labels, texto, clipping, recursos ou ordem de leitura.

Resolução de units passa da extração prematura em eval para layout; avaliação
do body, efeitos e diagnósticos não são duplicados por medições. Sub-layout e
tentativas restauram cursores, coletores e contexto; nenhuma mutação global.
O mecanismo de medição pode diferir do pin desde que preserve os observáveis.
Exporters não avaliam argumentos nem precisam conhecer TransformOperation.

## Aceitação

Geometria causal com controles default/origin parcial/extremos, reflow false/true,
Scale x somente, fator+override, absolute/auto, move percent/em, skew e nested.
Incluir texto/matemática inline e baseline, following content, quebra de página,
RTL, vazio/zero/flip, clips, gradientes Parent e recursos. Validar fonte→PDF/SVG/PNG
e campos antes/depois da realização; não alinhar imagens para esconder posição.
HTML fora de ilhas permanece no diagnóstico estrito do owner HTML, sem drops.

## Região relativa e fluxo das transformações — medição causal

`transformacoes-2026-10-14/after-first/results.json.gz` registra o candidato e
seus hashes: Move percentual fica fora da página porque a altura é o sentinel;
textos A/TEST/B ficam numa linha no Tekt e em blocos no pin. As imagens foram
inspecionadas, sem realinhamento. Fonte pinada `typst-layout/src/rules.rs:725–739`
usa BlockElem::single_layouter para as quatro funções, confirmando a semântica
de bloco; não é uma convenção mecânica de enum. Transform Rust genérico não
é uma função da linguagem. Correção dentro de X4 aprovado, sem novo contrato
público. A hipótese é refutada por deslocamento em região explícita/aninhada,
espaçamento, perda de body ou vazamento após rollback.

Move usa tamanho real do contexto de porcentagens, não a sentinela de paginação. As quatro variantes nativas são unidades de bloco no fluxo (rules.rs:725–739 upstream), inclusive no corpo de Place; dentro de Box a caixa externa estabelece a ocorrência inline. Terminar linha anterior, colapsar above/below de block vigentes, registrar fim/largura lógicos e conservar baseline interna do frame. Matrix Rust mantém fluxo legado. Reflow false mantém tamanho do body, não implica que a função nativa seja inline. Valores de origem e bbox são sobre o frame medido, independentes da margem externa. Refutar por irmãos, texto e subframes.

## Região de medição de reflow — correção causal

### Medição anterior à decisão

`lab/results/construction/diagnostics/transformacoes-2026-10-14/refuters-first/results.json.gz`
preserva fontes, HEAD, diffstat, UTC e hashes dos dois binários. As imagens
`reflow-wrap` e `reflow-wrap-rotate` foram inspecionadas: scale 200% mantém
ONE/TWO na mesma linha no candidato; no pin cada palavra fica numa linha.
Rotate 23deg quebra antes de THREE no candidato; o pin conserva ONE/TWO/THREE.
`lab/typst-original/crates/typst-layout/src/transforms.rs:40–46,80–88,205–218`
mede o corpo numa região ajustada e compõe usando o tamanho lógico medido.
O consumer cristalino em `01_core/src/compiler/layout/transform.rs:378–389`
ainda usa a largura original para todas as variantes. Isso altera quebras e
avanços, observáveis de linguagem, não mera mecânica de medição. Os snapshots
anteriores à correção estão em `implementation/reflow-before.json.gz`.

### Responsabilidade

Para reflow true, medir sob a região transformada: escala divide cada dimensão
regional pelo fator (módulo), rotação usa bbox da região pela rotação inversa,
skew usa bbox da região pelo shear. Preservar dimensões infinitas e escala zero
sem normalizar o fator para identidade. Resolver Length contra o tamanho lógico
original antes de ajustar a região; Auto acompanha o fator do outro eixo.
Compor o corpo na região do tamanho lógico obtido pela medição. Reflow false e
Move conservam a região original. Não deduzir a largura por bbox da tinta.
Medições especulativas usam checkpoint/restauração do estado mutável existente;
nenhuma label, contador, nota, coletor ou diagnóstico pode ser duplicado. A
composição efetiva produz os efeitos uma vez. Conservar erros de medição que
impeçam determinar a região. Não alterar APIs públicas nem fase do pipeline.

### Refutadores

Quebra de palavras sob scale 200%, escala negativa, x absoluto com y auto,
rotação com reflow, uso via set/alias, reflow falso e controles geométricos já
existentes; verificar posições e conteúdo, não apenas contagem de imagens.
Baseline de caixas, RTL e outras divergências do corpus permanecem abertas.

A segunda sonda (`refuters-second`, reflow-wrap-rotate) e a inspeção de
block.rs:250–261 mostram que a altura auto usa infinito como marcador de Block,
mesmo numa região externa finita. Ao calcular a região de transformação, esse
marcador não apaga a altura externa real preservada em relative_region_size;
na raiz usar a região da página. Altura explícita finita do Block prevalece.
Conservar infinito quando a própria região externa é ilimitada.

## Exaustividade da classificação privada

A validação em `transformacoes-2026-10-14/png-afim/validation/followup-architecture.log`
identifica defaults V16 novos em first_baseline e terminal_native. Manter os
resultados medidos, enumerando explicitamente as variantes sem baseline e as
unidades que não são transformações nativas terminais. A adição futura de variante
deve exigir decisão do compilador, não cair em default neutro silencioso.
Isso não promove wrappers adicionais a transparentes nem altera Matrix legado.

## Largura lógica sob overflow e espaço terminal — correção medida

### Evidência anterior à decisão

`lab/results/construction/diagnostics/transformacoes-2026-10-14/logical-before-complete/results.json.gz`
registra HEAD, árvore não commitada, diffstat, UTC, fontes e binários.
Em flip-long-word, flip-wide-shape e flip-inline-width a reflexão desloca o
conteúdo excedente porque transform.rs:651–654 toma cursor e tinta como
largura do frame. Em space-weak o cursor inclui espaço terminal de markup;
o controle space-literal conserva legitimamente o avanço literal. A fonte
pinada `lab/typst-original/crates/typst-layout/src/flow/distribute.rs:650–651`
limita o tamanho lógico à região; não recorta os filhos. Geometria de origem
e escala é observável da linguagem, não exige copiar a estrutura do Frame.

### Contrato interno

Usar a testemunha causal de largura do subframe, limitada pela largura da
região. Cursor terminal e bbox dos descendentes não ampliam o frame lógico.
O conteúdo excedente continua emitido; limitar o tamanho não instala clip.
A origem, reflexão e escala absoluta usam essa largura. Conservar avanços
textuais literais ao final do parágrafo, excluir espaços fracos terminais
e preservar espaçamentos explícitos. Place/overlays não ampliam o fluxo.
Sem contrato público ou fase nova (ADR-0127, fluxo contínuo). Refutar com
texto longo, shape e Box explícitos maiores que a região, espaços literais
versus markup, h isolado, transformação aninhada e corpus de reflow.

## Restrição horizontal interna de Block

A sonda shape-dim-before-complete e a regressão
implementation/shape-dim-green-final.log mostram Block80/inset5 com
Rotate0/Rect50% ainda usando37.5pt em vez35pt. layout_native:362 usa
available_width da raiz (75pt), mas block_place_region já preserva a largura
interna70pt. Usar essa região interna como largura disponível para medir e
compor o body nativo, inclusive a medição de escala absoluta; fora de Block
conservar available_width. Não mudar Matrix legado, assinatura, fase ou
semântica de reflow. A fórmula evita aplicar percentual sobre a margem/inset
externo; RED→GREEN e refutadores existentes permanecem obrigatórios.


## FrameItem — exaustividade de decisões existentes, V16, 2026-10-16

### Medição anterior à decisão

F036: 01_core/src/compiler/layout/transform.rs:183 (layout_matrix); F037: 01_core/src/compiler/layout/transform.rs:197 (layout_matrix); F038: 01_core/src/compiler/layout/transform.rs:273 (layout_matrix). Os spans AST, patterns/guards/corpos e complements foram conferidos antes do código. `lab/results/construction/diagnostics/tekt-frame-items-2026-10-16/before.json.gz` conserva HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, fontes/L0/configuração, hashes, status e git diff HEAD --stat. `classification.json` registra os 70 casos e seus complements em 26 owners, usando a árvore do parser pinado do CI. Cláusulas pertinentes do L0 foram lidas junto da fonte; não há proposta de nova arquitetura ou interpretação de intenção upstream. Padrões e exaustividade Rust são mecânica; semântica/morfologia, geometria, efeitos e diagnósticos do produto devem permanecer.

### Prescrição exclusiva desta unidade

- **F036 — `layout_matrix`**: Trocar somente o padrão '_' pelo grupo nominal `TextReplacement`, `Semantic`, `Link`. Usar o mesmo prefixo Rust em escopo (`FrameItem`). Conservar integralmente o corpo atual do fallback, os braços anteriores, guards, transformações, chamadas, returns e ordem.

- **F037 — `layout_matrix`**: Trocar somente o padrão '_' pelo grupo nominal `TextReplacement`, `Semantic`, `Link`. Usar o mesmo prefixo Rust em escopo (`FrameItem`). Conservar integralmente o corpo atual do fallback, os braços anteriores, guards, transformações, chamadas, returns e ordem.

- **F038 — `layout_matrix`**: Trocar somente o padrão '_' pelo grupo nominal `TextReplacement`, `TextSeparator`, `Semantic`, `Line`, `Link`. Usar o mesmo prefixo Rust em escopo (`FrameItem`). Conservar integralmente o corpo atual do fallback, os braços anteriores, guards, transformações, chamadas, returns e ordem.

Manter callbacks, efeitos, metadados, clipping, fontes, positions, tinta e largura conforme os owners vigentes. Esta rodada não amplia travessias, não transforma wrappers em folhas nem usa text/ActualText como substituto da geometria. Helpers de teste mantêm fixtures, esperados, tolerâncias, contagens e falha Unknown; não aproveitar o saneamento para aceitar ausência de observação. Grupos nominais compartilham o mesmo corpo anterior: notas V19 de alternativas continuam visíveis, sem confundir cobertura de um braço com todas as alternativas.

### Gate e aceitação

Manutenção interna ADR-0127 em fluxo contínuo: nenhum enum/campo novo, método de trait/assinatura pública, default, dependência, compatibilidade ou fase nova. Não materializar propostas históricas pendentes por este adendo. L0 primeiro e resselo restrito em ambas as direções. Não adicionar supressões, alterar limiar ou reescrever match como if-let para esconder V16.

Antes do código, executar o controle novo de FrameVisitor e a suíte workspace nos mesmos esperados, e confirmar RED dos 70 avisos da família por leitura AST. Após código, zero V16 dessa família nos locais abrangidos e nenhum aviso novo; manter V16 externos como pendências. Nos matches do core produtivo alcançados por cargo check -p typst-core, usar cópia descartável com uma variante sentinela de FrameItem para medir ausência de E0004 nos locais antes e presença depois. A sonda cobre somente os locais efetivamente compilados: não extrapolar aos helpers cfg(test) ou L3 que a dependência impede de alcançar. Para todo o lote, conferir padrões/guards/corpos por replay, enum e assinaturas inalterados, build e testes reais. Não substituir corpos por stubs.

Revalidar suíte workspace completa, cargo build, Clippy workspace/all-targets com -D warnings e -D unfulfilled_lint_expectations, formato, Rustdoc -D warnings com quatro índices HTML, linhagem V1/V5/V15/V26 estrita/reversa, SARIF e diff --check. Fontes/L0/configuração ficam congelados durante comandos. Execuções pesadas sequenciais com MemoryMax=12 GiB, MemorySwapMax=0 e CARGO_BUILD_JOBS=1. Não apresentar saneamento estrutural como nova paridade global, ganho de memória ou benefício medido para IA.
