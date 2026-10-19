# L0 — compiler/layout/realized
Hash do Código: f25b6927

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67
- 00_nucleo/prompts/_nuclei/layout/position-frame-carriers.toml sha256:ff71dfc6648d6deb4b2b8054e3d58a93777d17837129f0f1b1e7e138ffb6943c

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/realized.rs`
**Governança:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidência antes da decisão

O backtrace do binário da memoização, SHA-256 `70bb22e90f7b0fa1a660de1c8f0df0a89f8ebb6ddd82a77fd4794f753ebb07f5`, apontou duas chamadas de introspecção inicial vindas de layout_content_once ao emitir um heading transformado. A fonte em `01_core/src/compiler/layout/mod.rs:1163–1179` reconstruía o índice da origem de cada Realized para extrair suas Locations. No caminho aprovado de realização, origens frequentemente já são Located. A sonda e o estado RED estão em `lab/results/construction/diagnostics/reuso-sem-transformacao-2026-10-05/`; o backtrace bruto será preservado no recibo. Isso legitima a investigação de trabalho redundante, não um ganho ainda medido.

## Ownership e contrato interno

Possuir a lógica de registro das âncoras das origens consumidas por Realized, como free function estática descendente de layout. O hub apenas delega ao helper e emite visual. Não criar trait, vtable, novos campos públicos ou API externa. Estado continua no Layouter e identidade continua nos owners existentes de Location/Locator/Content.

Colher Locations explícitas pela travessia existente, incluindo origens aninhadas de Realized. A coleta só qualifica quando cada locatable foi preparado: delegar classificação a extract_payload, nunca duplicá-la. Outline/Bibliography/Ref/Dynamic/Document/Asset conservam fallback, inclusive como body imediato de Located, pela síntese, conversão ou exclusão da introspecção. Located com body sem payload também conserva fallback. Erro da travessia invalida a prova e exige o caminho anterior. Não usar igualdade/hash visual ou endereço cru como chave.

Sem prova de completude, reconstruir o índice da origem com introspect_with_introspector e extrair as keys de elements como antes. Com prova, usar as Locations colhidas sem reconstruir stores. Registrar no ponto de entrada do produto, com a mesma página e cursor, observe do Locator e positions.entry.or_insert. Conservar anchors previamente registradas, descendentes, aliases e IDs; não emitir a origem nem gerar um novo ID. Nenhum cache global, I/O, relógio ou unsafe.

## Aceitação

RED→GREEN prova âncora de origem preparada sem walk de introspecção. Cobrir descendentes preparados e fallback para origem sem Located. Revalidar labels, contadores, contexto, outlines, bibliografia, targets e geometria nos controles pertinentes. Contagem de chamadas e tempos vêm acompanhados de HEAD, working tree, binários, fontes e timestamps. Ausência de ganho deve ser reportada.

Extração/otimização interna na mesma fase e assinatura de fachada: fluxo contínuo ADR-0127. O L0 precede o código; interface externa e linguagem permanecem.

## D05 — fronteira do produto textual de equação de bloco

### Medição antes da decisão

Equation-where-2026-10-06/delivered.json.gz, pin 9f2b6e871, binários/HEAD/diff/UTC
preservados: show-block emite 𝑎 DISPLAY na mesma linha no Tekt e DISPLAY em linha
própria no vanilla. O filtro e produto coincidem; a fronteira de bloco perde-se.
layout/mod.rs:1172–1175 emite visual de Realized sem considerar a origem;
paragraph.rs:114–130 já realiza o segmento textual em fronteira própria.

### Decisão e aceitação

A unidade agora orquestra registro da origem e emissão do visual. Se a origem
é Equation com block:true e o visual é textual certificado (Text, Empty, espaços,
Linebreak, Sequence, Styled, Strong/Emph e wrappers Located/ShowRecipe),
com texto não-vazio, manter fronteiras
de parágrafo antes/depois do produto usando a unidade paragraph existente.
Registrar âncora após fechar a linha anterior; não criar Content/Location nova,
não emitir a origem nem repetir recipe. Outros tipos de origem e visuais fora
desse recorte conservam caminho anterior. Não reinterpretar Equation intacta
como texto. Sem novo campo/API/fase; correção de render na camada vigente.

Fluxo contínuo ADR-0127, L0 primeiro e RED→GREEN; manter fallback fora do
recorte, build/lint e nova medição bilateral. Não é novo modo/default deliberado.

Visual Realized aninhado, Empty isolado e conteúdo só de espaços não qualificam
este recorte; conservam fallback, evitando certificar remoção/dupla fronteira.


## D08B3-C2 — responsabilidade ratificada

Medição anterior: lab/results/construction/diagnostics/table-origens-d08b3c2-2026-10-07/baseline.json.gz registra HEAD/working tree/UTC/fontes/argv e binários no pin 9f2b6e8715237cb086899a42873660fe744622e8. A fonte desta unidade tem a operação transparente correspondente nos ramos existentes de ShowRecipe/Located/Realized; Source não pode vazar como elemento observável. Aprovação humana `confirmo`, 2026-10-07, contrato em lab/history/construction/records/00_nucleo/materialization/table-origens-d08b3c2-2026-10-07/contract.md.

regressions/equation show-block refuta Source como produto textual desconhecido: classificar origem Equation/block e produto por body transparente, preservando fronteira de parágrafo e âncora do Realized. Não alterar alvo/fase de equação.

Manter owner exclusivo e despacho estático; sem I/O, assinatura/trait/flag nova ou fase eval↔layout. Testes e coletas C2/C/macro, builds e lineage necessários antes de certificar.



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

Preservar o algoritmo de coleta e observe. Para cada origem, usar o registro de posição com política somente-se-ausente: não criar captura nem sobrescrever uma origem já registrada. O helper de posição transporta o ponto capturado sem reexecutar a origem.

Correção interna ADR-0127: sem API/campo público, flag/default ou mudança
de fase. L0 primeiro; RED→GREEN de coordenadas, ausência de tinta/altura
fictícia, identidade, ordem, clones visuais e rollback; revalidar os
controles HTML, geometria, contexto/queries e suíte completa.


## Guards de domínio explícitos — V17, 2026-10-16

### Medição anterior à decisão

T159: 01_core/src/compiler/layout/realized.rs:41. Fontes e L0 vigentes lidos antes da prescrição; SHA-256, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, status e git diff HEAD --stat em `lab/results/construction/diagnostics/tekt-guards-2026-10-16/before.json.gz`. O inventário anterior, reconfirmado por hash, registra 28 guards em 21 owners; a regra CI pinada manda desdobrar ou mover a condição para o corpo. A medição é estrutura Rust, não prova de defeito funcional ou de melhor leitura por IA. Nenhuma intenção upstream é inferida.

### Responsabilidade exclusiva e aceitação

Unir o catchall da visita de prepared_locations e decidir no corpo needs_fresh_index || extract_payload.is_some. Preservar curto-circuito, complete=false e ausência de coleta indevida; Located/Realized e fallback de introspecção conservam ordem e chamadas.

Manutenção interna em fluxo contínuo ADR-0127: sem contrato público, default, compatibilidade ou fase novos. A responsabilidade permanece somente no consumer exclusivo deste L0. Não criar helper apenas para esconder a conjunção, não adicionar supressões nem mudar a configuração do linter. Executar controles funcionais existentes antes do código e preservar seus esperados; o RED real é o gate V17 com 28 avisos, GREEN exige zero V17. Conferir caminhos de sucesso/fallback/erro, curto-circuito e efeitos; resselo restrito por owner. Revalidar build, suíte workspace completa, Clippy all-targets com warnings e expectativas não cumpridas como erro, formato, Rustdoc com warnings como erro, linhagem V1/V5/V15/V26 estrita/reversa e SARIF sem novos avisos. Executar sob 12 GiB, swap zero e um job. Não alegar ganho de memória/desempenho ou paridade global sem medição separada.


## Exaustividade de origens e Source — 2026-10-16

### Medição antes da decisão

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, status, git diff HEAD --stat e fontes/hashes integrais em `lab/results/construction/diagnostics/tekt-v16-introspect-origins-2026-10-16/before.json.gz`. Value tem 39 variantes e Content 120. eval/mod.rs:956 conserva fallback None latente T027; layout/realized.rs:14–45 usa predicado implícito e binding other para T158/T160; :77–130 usa cadeias if-let com false implícito. extract_payload.rs:18–98 despacha sem normalizar Source; locatable.rs:21 classifica Source pelo body. O núcleo content/source-origin.toml exige transparência e ocorrência única. Inferência a refutar por teste RED antes da implementação: Source(Heading) retorna None pela extração atual, e Located(Source(Heading)) perde a prova de preparação. map_content em entities/content/traversal.rs:79 já trata Source transparentemente. Não se infere intenção upstream nem nova paridade vanilla destes fatos; não mudar a pendência OutlineEntry sem investigação própria.

### Decisão exclusiva e aceitação

needs_fresh_index torna explícitos os seis casos atuais Outline/Bibliography/Ref/Dynamic/Document/Asset (true) e o complemento nominal (false). prepared_locations preserva os braços Located/Realized, ordem e efeito; o complemento nominal usa o parâmetro node já disponível no corpo, sem captura adicional. Conservar a delegação a extract_payload e não duplicar sua tabela. block_equation_origin e textual_product passam de if-let/matches! com false implícito a matches nominais completos: manter exatamente as classes e recursões atuais, incluindo Source normalizado por source_body, Realized apenas na origem de equação e ausência de unwrap de Realized no certificado textual. Sequence usa all; Empty/espaços são textuais mas a emissão ainda exige plain_text não vazio no layout inalterado. Não criar Source callback novo: map_content já o trata transparentemente. Com o extractor corrigido, Located cujo body é Source(Heading) deve qualificar e colher a mesma Location; Source exterior a Located e original de Realized não duplicam visitas/Locations. Tipos especiais e conteúdo não preparado mantêm fallback. Não mudar registro, ponto/página, ordem, IDs, pipeline ou carriers.

O grupo nominal com consequência idêntica pode permanecer como V19 informativo, prescrito por responsabilidade acima, sem duplicar corpos nem supressões globais. Correções estruturais adicionam barreira local contra variantes novas; não afirmam defeito funcional em todos os fallbacks nem cobertura de execução de todas as alternativas. Preservar firmas públicas e comportamento fora da transparência Source já contratada.

Escrever testes co-localizados dos casos Source produtores/não produtores, Located/Source, coleta sem duplicação, fallbacks e predicados; confirmar RED da transparência antes do código. Controles de preservação e single_evaluation_ existentes passam antes/depois. Em cópias do workspace real, variantes sentinelas separadas em Value e Content devem produzir E0004 nos seis pontos selecionados depois da correção; guardar ausência dessas barreiras antes, demais erros e fontes. Resselo somente dos pares editados. Validar build, suíte workspace completa, Clippy all-targets com warnings/expectativas como erro, fmt, Rustdoc estrito, V1/V5/V15/V26 e reversa; SARIF remove somente V16 selecionado e mantém demais diagnósticos, admitindo apenas V19 equivalentes prescritos. Recursos: 12 GiB, swap zero, Cargo um job e comandos pesados sequenciais. Não declarar fechado o objetivo geral ou OutlineEntry por fechar este lote.

Complemento medido antes dos testes: needs_fresh_index deve normalizar Source pelo source_body existente antes do match. A extração Source corrigida não pode permitir que Located(Source(Outline/Bibliography/Ref/Dynamic/Document/Asset)) passe pelo caminho rápido. Hoje a ausência de payload Source mascara essa falta de normalização. O teste de classe especial com Source deve ser RED antes; preservar fallback para as seis classes após transparência, sem criar callback Source (map_content já o omite).

### Refinamento medido de sintaxe plana

preview-architecture.log e flattening-before.json.gz preservam o estado inicial desta materialização, HEAD/árvore/UTC, e o único V20 novo em realized.rs:155: profundidade 3 da captura other @ (grupo nominal). Substituir somente essa captura pela enumeração nominal plana e reutilizar node no corpo, alpha-renomeando needs_fresh_index(other)/extract_payload(other) para node. Não criar helper, descartar variantes ou alterar curto-circuito; a barreira de evolução permanece. Isso supersede a opção de captura anteriormente descrita; V19 continua informativo, V20 não deve aumentar. O L0 deste refinamento precede o ajuste do código.


## Alinhamento OutlineEntry e walk exaustivo — 2026-10-16

### Medição anterior à decisão

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, lista exata git diff HEAD --stat, fontes/hashes e binários em `lab/results/construction/diagnostics/tekt-v16-introspect-walk-2026-10-16/before.json.gz`. Medições CLI anteriores em measurement-before.json e measurement-fields-before.json: no pin vanilla 9f2b6e8715237cb086899a42873660fe744622e8, entry direta aparece uma vez, sintetizada uma vez e mista duas vezes; no Tekt, a direta some da query (0 contra 1), mista perde a direta (1 contra 2), e body() direto falha cannot outline heading. Fonte: entities/elements/outline_entry.rs:61 body só aceita Heading bruto; :99 fornece NativeElement; introspect/extract_payload.rs:148 descarta OutlineEntry; walk.rs:805 trata entry terminal; outline_entries.rs:38–61 sintetiza entries após walk. Generic map_content visita element/fill (traversal.rs:221); realized.rs:139–281 usa essa travessia para certificar origens, motivo para testar um heading referenciado Located sem confundi-lo com descendente renderizado. O upstream pinado declara OutlineEntry Locatable em outline.rs:472–481. Isto mede semântica/morfologia e erros observáveis da linguagem, não impõe igualdade Rust/bytes/PDF ou mecânica vanilla. A hipótese de correção é refutada por query/fields/body errados, headings fantasmas, duplicação ou regressão em síntese/ordem/labels.

### Responsabilidade exclusiva vigente

Adicionar OutlineEntry às classes que exigem índice fresco, retirando-a do complemento false. O walk é terminal para a entry, enquanto map_content visita o campo element semântico; o certificado genérico não pode tratar uma Location referenciada do heading como origem renderizada da entry, nem certificar entry nua como preparada. A nova extração não deve habilitar acidentalmente esse atalho. Preservar as seis classes especiais anteriores e Source normalizado; restantes decisões/coleta/registro/predicados inalterados. Testar entry nua com element Located, entry Located, Source e Realized; a prova deve recusar atalho e fallback conserva o índice causal, sem novos IDs de produto.

Esta revisão supersede somente as ressalvas anteriores de pendência OutlineEntry neste owner; preserva os demais contratos vigentes.

### Aceitação

L0 primeiro, testes co-localizados antes da implementação e RED funcional real; manter controles existentes e a medição CLI anterior intacta. Repetir fixtures direct/synthesized/mixed com body() e com acesso ao campo, comparar resultados de linguagem, não bytes PDF. Sonda Content sentinela separada em cópia do workspace real: os seis pontos W1–W6 devem acrescentar E0004 após a correção e não antes, sem outros erros; preservar logs/inputs. Resselo só dos cinco pares afetados e grafo V15/V26 válido. Gates: build workspace, fmt, Clippy all-targets -D warnings e unfulfilled_lint_expectations, Rustdoc -D warnings, suíte workspace completa, V1/V5/V15/V26 e reversa, diff-check e SARIF diferencial. Esperado: seis V16 removidos (239→233), seis V19 informativos novos; V19 existente do complemento do extractor perde uma alternativa e os dois grupos de needs_fresh_index migram OutlineEntry de false para true. Nenhum V20 novo ou outro diagnóstico alterado após mapear deslocamentos. V19 é grupo de consequência uniforme prescrito aqui, não cobertura de execução de cada alternativa. Sem supressões ou configurações globais. 12 GiB, swap zero, um job Cargo, comandos pesados sequenciais; estado/UTC/hashes registrados. Correção interna/paridade ADR-0127, sem API/campo/trait/default/fase ou compatibilidade novos. Não publicar, commitar ou fazer push. O objetivo geral permanece aberto com os demais V16.
