# L0 — infra/pipeline/tests
Hash do Código: 63207863

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/compilation-metric-cache.toml sha256:b4372cbdd36e0e25377d1258638a6859bcc88c9e584b91b377a2a12d0d66135f
- 00_nucleo/prompts/_nuclei/expression-source-identity.toml sha256:fb383f227b0bcd8b1e61899942fa0383a4cda66093da58bda2e87844e2895d2a
- 00_nucleo/prompts/_nuclei/fonts/shaped-face-identity.toml sha256:96fe00a0c6071bb88ffc63806b3e42c23b78c398fa26d46866e0f099ab0d4e36
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:08c39c6b9cf6fdc571564646d12d71f9b12f2a5d2c029ca90f9253c39c28bb95
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:79d5c48d0b9d0fb7d995207d18b1f9437209bf35c78a35cbfcedccea8b90abd2
- 00_nucleo/prompts/_nuclei/page-counter-snapshot.toml sha256:bdbe23b72d7083b7672d71e851dc3d35baa8ed7ccb66f009f61edcf770d0aeff
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:3ee5760ffa44b5fd889b9d6772972e3b8417be332e13726cd9eea24b32e686f8




**Layer:** L3
**Exclusive consumer:** `03_infra/src/pipeline/tests.rs`

## Current evidence

- `03_infra/src/pipeline/tests.rs:18` — `p1247_contexto_svg_filtra_pagina_e_posicao_homologa`.
- `03_infra/src/pipeline/tests.rs:41` — `MockWorld`.
- `03_infra/src/pipeline/tests.rs:47` — `library`.
- `03_infra/src/pipeline/tests.rs:50` — `book`.
- `03_infra/src/pipeline/tests.rs:53` — `main`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/pipeline/tests.rs`.

## Owned contract

- Own only the integrated pipeline suite and fixtures, using the existing public facades and descendant helpers. Tests do not authorize production behavior or a new pipeline phase.
- Preserve integrated format precedence, feature separation, contextual snapshots, callbacks, bundle, resources and diagnostics oracles. Use structure, text, DOM, dimensions and positions rather than complete serialized byte hashes.
- Characterize fractional boxes on finite lines: suffix position for 1fr, proportional positive shares, zero share, isolated negative share and safe zero-sum/nonfinite ratios. Preserve repeat ceiling 1000, link scope and strictly-more-copies-at-greater-width relation.
- Keep repeat controls with/without prefix at equal width, distinct sibling 1fr repeats/suffixes, a later block body and nested boxes. Observe paged output, not private repeat/fr indices; do not adapt expectations to the candidate.

Verify that resource collection and resolution for a paged document containing only shapes succeed with a World whose catalog and font accessors reject any access. Construct layout before applying the rejecting World: shape layout can legitimately require baseline and cap-height metrics. This checks export resource demand independently of installed host fonts or wall-clock timing.

Verify with the real Ubuntu Sans variable fixture that unknown-axis-only resource requests retain source bytes even with an unavailable configured Python. Keep a matching `wght` control: it must still fail with the existing dependency diagnostic when the tool is absent. Do not mutate process-global environment in tests; the isolated test command supplies the missing interpreter.

## Verification and boundary

Run `cargo test -p typst-infra --lib pipeline::tests -- --test-threads=1` when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

## Separadores textuais — materialização autorizada em 2026-09-28

### Medição anterior à decisão

`03_infra/src/pipeline/tests.rs` é um consumer do percurso FrameItem. As sondas RED fonte→PDF de
`00_nucleo/diagnosticos/fatia-funcional-tekt-2026-09-28/` e a inspeção
dos handlers desse consumer precedem esta alteração. A aprovação do contrato
público em `00_nucleo/prompts/entities/layout_types.md` foi recebida em
2026-09-28 e registrada na proveniência da materialização.

### Responsabilidade deste owner

Adaptar projeções e traversals exaustivos de fixtures ao carrier: posição é transportada; neutro para contagem de pontos, tinta e medições de comportamento existente.

Conservar as responsabilidades existentes; a aceitação segue texto e geometria
observáveis, incluindo os controles de adjacência e os invariantes pinados.

## Identidade textual — controles do contrato aprovado

Medição anterior: initial.json, before/resultados.json e red.log em
`00_nucleo/diagnosticos/identidade-hifen-discricionario-2026-09-28/` registram
texto/caixas e a perda de SHY. O dono confirmou o contrato
`00_nucleo/prompts/entities/layout_types.md`. A compilação de fixtures exaustivas
exige tratar a variante nova. Projeções visuais descem nos filhos; extractors
textuais usam text uma vez. Expectativas de identidade discricionária passam a
SHY, mantendo U+002D literal. Não relaxar caixas/seleção. A suíte de shaper
exercita fixture Noto obrigatória e ausência de fontes, conserva envelope e
prova que text substituto não altera glyphs/avanços nem contamina o cache.

## Regressão do PDF/UA no bundle — 2026-09-30

A medição causal em `00_nucleo/diagnosticos/auditoria-composicao-construcao-2026-09-29/probe.json` antecede a decisão: PDF isolado sob UA-1 sem título falha, bundle publica um PDF declarado UA-1 sem título. Adicionar teste integrado com a fixture World existente para exigir, no bundle, o diagnóstico exato de ausência de título e nenhum ficheiro retornado; manter controle de sucesso quando o título está presente e ausência de exigência fora de UA-1. O teste materializa somente a aceitação dos owners `infra/pipeline/pdf.md` e `infra/pipeline/bundle.md`, sem legitimar lógica produtiva por este prompt.

O controle GREEN usa `title:` nomeado em `#document("one.pdf", title: [Okay])[...]`, pois é o carrier próprio da entrada bundle; `#set document` é uma regra de eval do percurso individual, não a metadata de uma entrada. Conferir que o PDF emitido contém título e identificação UA-1. Um segundo documento sem título não herda o título do primeiro.


## Realização com avaliação única

Expansão contextual após show conserva produtor preparado e consulta índice unificado: output contém Original | Generated. Esse teste cobre execução produtiva de pipeline, sem adapter de projeção dos testes L1.


## Realização com avaliação única

Receita que transporta conteúdo contextual capturado e cria outro produtor conserva ambos: contador de ids da realização não reutiliza ids já capturados na avaliação comum.


## Reuso sem transformação — 2026-10-05

Adicionar sentinelas dos helpers internos para conservação de um root Sequence sem ContextBlock pela preparação contextual e pela fase de realização de outlines. Observar também conteúdo, ausência de warnings, ordem e headings consultáveis. A comparação Arc::ptr_eq é sonda do trabalho mecânico evitado, não definição de identidade da linguagem. Revalidar os testes vigentes de callbacks/erro/outline/contexto, que não podem ser suprimidos pelo atalho.


## Sentinelas do candidato inicial — 2026-10-05

Evidência e decisão são do owner `00_nucleo/prompts/infra/pipeline/paged.md`. Antes da implementação, contar por thread apenas em cfg(test) entradas no wrapper de layout e exigir uma chamada para tabela simples que ocupa múltiplas páginas, PageStore/binding com todos os números lógicos e texto preservado. Exigir duas chamadas no caminho ordinário com numbering explícito; conservar teste de contexto que lê total de páginas e controles existentes de outline/callbacks/erro. O contador é sonda mecânica do trabalho evitado, nunca critério de paridade da linguagem. Incluir predicado de rejeição para contexto escondido em SetPage, tipos não listados, show residual e origens Realized não qualificadas. Fonte/estado RED e suítes GREEN acompanham o diagnóstico.


## Regressão page display — 2026-10-05

Exercitar a fachada paginada com contexto de counter(page).display: padrão/callback, duas páginas, início lógico explícito. Comparar o texto emitido por página contra números lógicos esperados (1/2 e 7/8), não contra bytes PDF; conservar counters não página. O teste é RED antes de corrigir o owner de display; não alterar algoritmos do pipeline neste owner de testes.


## Snapshot realizado de Page — ratificado em 2026-10-05

Medição antes da decisão: HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree e binários exatos em `00_nucleo/diagnosticos/contexto-page-total-2026-10-05/measurement.json.gz`. single imprime B1/0 F0 contra B1/1 F1; duas páginas total/final 0 contra 2; update inicial 7 conserva total 7 contra 8; update intermediário altera página anterior (7/7 e 8/7 contra 1/7 e 7/7); step final imprime final 1 contra 2. Bundle imprime 1/7 no primeiro PDF contra 1/1, e final 7 no segundo contra 8. HTML misto devolve 1 contra 7. `03_infra/src/pipeline/paged.rs:360–365,840–913` usa o último update como início de todas as páginas; `context_stabilization.rs:969–1010` faz o mesmo por body. `compiler/stdlib/counter.rs:490–513` e `counter/display.rs:96–110` leem final global causal. Vanilla pinado 9f2b6e8715237cb086899a42873660fe744622e8, `lab/typst-original/crates/typst-library/src/introspection/counter.rs:246–254,730–773,934–963`, combina escopo documental, updates e avanços físicos. A classificação é paridade de linguagem; não exige copiar a mecânica de memoização upstream.

Decisão proposta: aplicar o Núcleo page-counter-snapshot. Esta unidade substitui o scope-out histórico de final apenas após ratificação. Contrato público e realização page-aware de updates exigem gate ADR-0127; não materializar ou ressellar antes da confirmação. Preservar as assinaturas existentes, ciclos e budget; não adivinhar página/total antes do snapshot conhecido.

Exigir RED→GREEN de current/final/both: uma e várias páginas; update inicial/intermediário/tardio; callbacks Func page-aware; páginas finais sem contexto; counters não Page; standalone HTML e bundles PDF/HTML/PDF com totais e updates distintos. Conferir texto e seleção contextual da língua antes de considerar estabilização. Casos não reproduzidos não qualificam paridade. Suítes existentes, build e lint com proveniência completa.


## Materialização do conjunto ratificado — 2026-10-05

Sentinelas page_snapshot current/final, callbacks e HTML são RED na CLI/código anteriores; exigir GREEN antes da suíte e preservar a evidência do estado exato.

Aceitação adicional: estados Page vazios e de vários níveis têm get/final realizados projetados em um nível, incluindo delta físico zero. Oráculo e fonte anteriores à decisão no owner counter_updates.

Ratificação: o humano confirmou o conjunto do manifesto `00_nucleo/diagnosticos/contexto-page-total-2026-10-05/l0-manifest.json` (SHA-256 1af5de11b1bbbe41cc59b907fdfafc21d98b6aab63fc1db0bfb1a028329ef66e). O gate ADR-0127 deste conjunto está satisfeito. Clarificações internas subsequentes seguem fluxo contínuo, L0 antes do código e RED→GREEN.

A suíte caracteriza a retenção de snapshots nas fixtures de tabela pequena e grande: current/final da compilação retornada têm a mesma geração e finais por ocorrência correspondem ao total realizado. Relatar número de entradas, capacidades e payload reservado por Vec, sem equiparar isso a RSS ou ganho de tempo.

Erro/recovery: um callback Page com retorno inválido falha; uma compilação válida subsequente no mesmo World continua produzindo o snapshot completo correto. Não impor igualdade de mensagem com o upstream sem medição específica.

## Sentinela da sessão contextual — 2026-10-05

Ver medição anterior à decisão nos owners paged/context_stabilization. Compilar uma tabela gerada por contexto selecionado com counter(heading.where(level:1)).final(); conferir texto e páginas e comprovar que suas tentativas de layout não criam caches de métricas independentes. Sonda thread-local cfg(test) conta somente o ramo de criação no wrapper de layout; zerar antes da compilação. RED no caminho atual, GREEN depois de fornecer o handle da sessão. Os demais helpers e seus caches não entram nessa contagem.

## Sentinela de reconciliação selecionada — 2026-10-05

Ver medição antes da decisão nos owners paged/context_stabilization. Compilar tabela gerada por contexto com leitura filtrada, conferir conteúdo e números lógicos, e exigir zero métricas frescas no braço final de reconciliação. Sonda cfg(test) local ao wrapper paginado, thread-local por teste; não instrumentar produção ou alterar o owner das equações de reconciliação. RED no descarte atual do handle, GREEN com transporte interno. Conferir também que prepare sem seleção não transporta resultado settled.

## Sentinela de retenção até bidi selecionado — 2026-10-05

Medição antes da decisão no owner paged, seção Métricas retidas no bidi contextual selecionado: 299 cálculos no bidi RTL selecionado. Compilar contexto selecionado com texto RTL e sufixo latino/número, verificar conteúdo e números lógicos e exigir zero métricas frescas no braço bidi do coordenador. Instrumentação thread-local cfg(test), sem instrumentar produção. RED com métricas frescas, GREEN com empréstimo do handle aceito. As sentinelas anteriores conservam layout e reconciliação.

## Migração do carrier bidi ratificado (2026-10-05)

Medição anterior à decisão: o inventário `00_nucleo/diagnosticos/bidi-contexto-2026-10-05/inventory.json` identifica as referências desta unidade e os owners 1:1. O contrato do manifesto 1ae7513e foi confirmado; nesta unidade ajustar constructors não certificados com None e patterns de leitura com `..`, preservando os observáveis de linguagem existentes. Transformações obedecem ao núcleo pinado de origem; sem ampliar fase/API.

## Envelopes de identidade no pipeline completo

Medição anterior à decisão: `00_nucleo/diagnosticos/bidi-contexto-implementacao-2026-10-05/verification.json.gz` preserva o primeiro candidato; RTL mixed/LTR numeric ainda coincidem com baseline errado. `pipeline-context.log` registra RED pelo pipeline completo: todas as folhas sem span; `realize/mod.rs:38–44` insere Located em ocorrências. Avaliação direta não exercitava esse envelope. Correção interna: Located é transparente somente para certificação da sua body textual integral, sem ignorar corpos opacos nem alterar registro de Location. Direção lexical e coleta devem atravessar o mesmo envelope. O teste do pipeline exige spans em todas as folhas textuais e ordem RTL com primeiro trecho latino; preservar artefatos do primeiro candidato.


## Face efetiva de TextShaped — materialização ratificada 2026-10-05

Medição anterior à decisão: `00_nucleo/diagnosticos/identidade-fonte-shaped-2026-10-05/{initial.json.gz,probe.txt,bilateral.json}` distingue GIDs da regular de seleção Oblique. O campo público `TextShaped.font: Option<Font>` do owner layout_types foi confirmado pelo humano (SHA f4a17206500baf5952f898135944d55f9fca905285eafc995315ead1121dd409). O snapshot e inventário desta rodada estão em `00_nucleo/diagnosticos/identidade-fonte-shaped-implementacao-2026-10-05/`.

Este owner migra apenas suas referências: constructors legados usam None; transformações preservam a fonte; comparação/junção shaped discrimina face e eixos. Emissão do shaper fornece clone da fonte efetiva também após cache hit; não alterar semântica/estilo para induzir seleção. Collectors/subset/CMap e PDF usam binding da mesma face original com coordenadas do estilo. PNG/SVG preparam bindings locais e instanciam uma vez por face/pedido antes de render, mantendo índice de recursos original distinto dos bytes instanciados. None segue resolução legada. API pública e FontKey existentes permanecem. Não criar fontes artificiais, unsafe, globals ou fase nova. Funções privadas podem receber fonte/bindings explicitamente.

A associação é por bytes da face original e pedido de eixos; nomes e slots não bastam. A tabela local dura a exportação e conserva ordem dos recursos fornecidos, anexando faces faltantes. Para multifont, distribuir itens pelos bindings uma vez antes de coletores por fonte; não reutilizar GIDs/CMap globais em faces diferentes. Código deste owner continua 1:1 com seu L0; referências a outros owners não compartilham propriedade. Aceitação RED→GREEN e controles bilaterais incluindo miss/hit/VF, sem promessa de ganho de tempo.

Claims compartilhadas desta migração pertencem ao núcleo shaped-face-identity pinado acima; este owner materializa somente as referências da sua unidade.

## Identidade de documento e expressão — gate confirmado 2026-10-06

### Evidência anterior à decisão

`00_nucleo/diagnosticos/diagnostic-paths-2026-10-06/identity-next-audit.json`
conserva comandos, fontes, UTC, HEAD, árvore alterada e hashes dos binários do
Tekt e do vanilla pinado 9f2b6e8715237cb086899a42873660fe744622e8. Os casos
separam erro/warning do documento de erro da expressão: o documento perde sua
fonte, enquanto a expressão recebe indevidamente o nome do documento.
`01_core/src/compiler/eval/mod.rs:609-615` usa world.main() para Source e base;
`04_wiring/src/eval.rs:66-95` repete esse id ao fornecer o primary ao formatter.
`03_infra/src/world/cache.rs:58-78` registra arquivos e recicla o contador;
`lab/typst-original/crates/typst-cli/src/eval.rs:159-179` e
`lab/typst-original/crates/typst-cli/src/world.rs:176-190` distinguem expressão,
stdin e ausência de entrada por identidade. A igualdade dos inteiros e o
interner global upstream são mecânica, não requisito (ADR-0107). A origem
mostrada, fonte, posição e resolução relativa são observáveis de linguagem.

### Decisão e escopo do gate

Testar erro/warning do documento e erro da expressão com Sources de ids
distintos, introspector válido e falha de parser. Controlar que o documento
é avaliado uma vez com efeito observável/instrumentação de World de teste,
sem impor uma única passagem de layout ou fixpoint. Conservar origens de
imports/callbacks e base de read relativa. Fachadas legadas ficam como controles.

Gate ADR-0127 confirmado pelo dono em 2026-10-06 (resposta: confirmo), conforme
00_nucleo/materialization/origem-fontes-expressao-aprovacao-2026-10-06.json.
Consumer exclusivo preservado; Hash do Código acompanha a materialização.


## Query CLI compilada — gate ADR-0127 confirmado

### Medição anterior à decisão

query-residuos-2026-10-06/before.json.gz registra HEAD/working tree, UTC,
pin/binary hashes, fontes, comandos e stdout/stderr/status. query-stdin-callback
retorna sucesso onde vanilla reporta erro e trace. query-context-success perde
metadata produzido; nested-error também desaparece. query-selector-quotes
apresenta rejeição P206C antes do erro do documento. where-level é rejeitado,
mas vanilla retorna [2]. Document-error-invalid-selector demonstra precedência
da compilação; valid-document-invalid/text-selector demonstra retrieval error
sem depreciação. eval-context-reference e eval-where-reference já coincidem
usando a compilação paginada atual. Upstream typst-cli/src/query.rs:34–66
compila antes de retrieve; :86–105 avalia selector em Scope default e depois
converte para LocatableSelector. Fontes locais: query_helpers.rs:488–522 consulta
somente Module; pipeline.rs:182–214 já usa compilação com introspector final.

### Decisão confirmada

Correção da linguagem, não cópia da mecânica upstream. CLI query passa a
compilar documento paginado completo antes da avaliação de selector, sem
exportar arquivo. A execução comum do programa permanece única, conforme os
owners vigentes de realização/contexto. Reutilizar o pipeline paged completo:
nenhuma expansão avulsa, índice pré-contexto ou ciclo de estabilização paralelo.
Distinguir falha de compilação e de retrieval por enum explícito, nunca por
mensagens, spans destacados ou busca de substrings. A nova API e o caminho de query compilada foram confirmados pelo humano
em 2026-10-06 (confirmo); recibo query-compilada-aprovacao-2026-10-06.json.
Os candidatos/manifesto permanecem como recibo histórico; este é o L0 vigente.

Testar fachada compilada: callback com variável desconhecida falha com span/trace
original; metadata gerada em context aparece no introspector final; warnings
são conservados uma vez. Confirmar que não há artefato exportado. Usar fontes
e Worlds isolados existentes; não testar somente chamada de helper/match privado.

## Prova de reuso da Cell local sem numbering — 2026-10-09

Medição: `00_nucleo/diagnosticos/table-local-scaling-followup-2026-10-09/layout-call-counts-before.json.gz` registra release, HEAD/árvore/UTC/fixtures/comandos e duas passagens completas para `local-set-1600` contra uma para `no-fill-1600`. `03_infra/src/pipeline/paged.rs:13-53,434-468` liga a diferença à prova estrutural que recusa a receita local e força segundo layout no ciclo de numeração. É custo de mecânica; preservar texto, caixas, páginas e tinta é observável de linguagem.

Testar via fachada paginada que Table local multipágina com `set table.cell(fill: red)` e sem numbering reutiliza o primeiro candidato completo: uma chamada contada no wrapper `cfg(test)`, todas as células e páginas presentes. Um show local capturado mantém duas chamadas; numbering existente continua duas. Não inferir independência de qualquer ShowRecipe ou função arbitrária. Este consumer de testes apenas exerce o contrato do owner `infra/pipeline/paged.md`; não define o algoritmo nem amplia API/fase/default.

## Imports e documentação de CI — 2026-10-13

### Medição anterior à decisão

`03_infra/src/pipeline/tests.rs` nas linhas 1492 contém os avisos selecionados. Fontes/owners, spans, saída original do compilador, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, status e diffstat estão em `00_nucleo/diagnosticos/infra-imports-docs-ci-2026-10-13/before.json.gz`. A pesquisa distingue referências produtivas, imports necessários somente à suíte descendente e menções em comentários; não constitui nova medição de paridade.

### Decisão e aceitação

Remover somente os imports privados comprovadamente sem uso neste consumer; manter os reexports públicos e os imports exigidos por chamadas ou contratos. A lista exata de spans anterior à edição está no diagnóstico citado.

Manutenção interna sem mudança de assinatura pública, default, fase, fórmulas ou critérios de aceitação (fluxo contínuo ADR-0127). Não introduzir supressões de lint. Revalidar compilação de todos os targets, nomes e execução da suíte de infraestrutura, build, Clippy do workspace, Rustdoc e linhagem com resselo restrito. Não criar testes que apenas espelhem remoção de imports ou formatação de comentários.


## Resultado paginado nomeado — type_complexity (2026-10-15)

Medição anterior à decisão: pipeline/tests.rs:3030 desestrutura a tupla privada
do helper paginado; o compile de todos os targets identifica este chamador.
Fonte/owner anteriores foram conferidos contra os hashes do snapshot inicial e
conservados em before.json.gz do diagnóstico type-complexity-2026-10-15.

Na fixture page_snapshot_retention_keeps_one_complete_generation, desestruturar
o novo PagedCompilation por nomes document/introspector, mantendo os bindings
doc/intr. Preservar integralmente programa, casos 4/320, assertions, contagens e
sonda de retenção. Não alterar tolerâncias ou expectativas, nem criar teste de
espelho dos campos. O owner paged continua dono do resultado e da compilação.
Mudança privada de fixture sob ADR-0127, sem API/default/fase; revalidar suíte
workspace, gate de lint, build e linhagem com resselo restrito.


## Expressões locais verificadas — Clippy 2026-10-16

### Medição anterior à decisão

03_infra/src/pipeline/tests.rs:388 (clippy::redundant_guards). O snapshot `00_nucleo/diagnosticos/clippy-expressions-2026-10-16/before.json.gz` conserva fontes/L0, HEAD, working tree não commitado, status, diffstat e UTC; initial-diagnostics.json conserva sugestões e spans. Os usos e tipos foram inspecionados antes da decisão. São representações Rust internas; nenhuma nova paridade upstream ou ganho de performance é alegado.

### Responsabilidade exclusiva e aceitação

No contador de pontos de destino, incorporar LinkTarget::Destination(_) ao pattern FrameItem::Link em vez do guard matches. Preservar recursão pelos demais wrappers, contagem, fixtures e asserções.

Fluxo contínuo ADR-0127: sem API pública, default de produto, fase, dependência ou supressão novos. L0 precede código e resselo restrito. Preservar inputs e resultados dos controles existentes, sem relaxar tolerâncias. Gate dos lints selecionados RED→GREEN; executar suíte workspace, build, Clippy all-targets, formato, Rustdoc e linhagem. Não criar testes que apenas espelhem a sintaxe Rust.


## FrameItem — exaustividade de decisões existentes, V16, 2026-10-16

### Medição anterior à decisão

F067: 03_infra/src/pipeline/tests.rs:750 (frame_items_text); F068: 03_infra/src/pipeline/tests.rs:764 (contains_bold_text). Os spans AST, patterns/guards/corpos e complements foram conferidos antes do código. `00_nucleo/diagnosticos/tekt-frame-items-2026-10-16/before.json.gz` conserva HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, fontes/L0/configuração, hashes, status e git diff HEAD --stat. `classification.json` registra os 70 casos e seus complements em 26 owners, usando a árvore do parser pinado do CI. Cláusulas pertinentes do L0 foram lidas junto da fonte; não há proposta de nova arquitetura ou interpretação de intenção upstream. Padrões e exaustividade Rust são mecânica; semântica/morfologia, geometria, efeitos e diagnósticos do produto devem permanecer.

### Prescrição exclusiva desta unidade

- **F067 — `frame_items_text`**: Trocar somente o padrão '_' pelo grupo nominal `TextSeparator`, `Line`, `Glyph`, `Image`, `Shape`. Usar o mesmo prefixo Rust em escopo (`FrameItem`). Conservar integralmente o corpo atual do fallback, os braços anteriores, guards, transformações, chamadas, returns e ordem.

- **F068 — `contains_bold_text`**: Trocar somente o padrão '_' pelo grupo nominal `TextSeparator`, `Line`, `Glyph`, `Image`, `Shape`. Usar o mesmo prefixo Rust em escopo (`FrameItem`). Conservar integralmente o corpo atual do fallback, os braços anteriores, guards, transformações, chamadas, returns e ordem.

Manter callbacks, efeitos, metadados, clipping, fontes, positions, tinta e largura conforme os owners vigentes. Esta rodada não amplia travessias, não transforma wrappers em folhas nem usa text/ActualText como substituto da geometria. Helpers de teste mantêm fixtures, esperados, tolerâncias, contagens e falha Unknown; não aproveitar o saneamento para aceitar ausência de observação. Grupos nominais compartilham o mesmo corpo anterior: notas V19 de alternativas continuam visíveis, sem confundir cobertura de um braço com todas as alternativas.

### Gate e aceitação

Manutenção interna ADR-0127 em fluxo contínuo: nenhum enum/campo novo, método de trait/assinatura pública, default, dependência, compatibilidade ou fase nova. Não materializar propostas históricas pendentes por este adendo. L0 primeiro e resselo restrito em ambas as direções. Não adicionar supressões, alterar limiar ou reescrever match como if-let para esconder V16.

Antes do código, executar o controle novo de FrameVisitor e a suíte workspace nos mesmos esperados, e confirmar RED dos 70 avisos da família por leitura AST. Após código, zero V16 dessa família nos locais abrangidos e nenhum aviso novo; manter V16 externos como pendências. Nos matches do core produtivo alcançados por cargo check -p typst-core, usar cópia descartável com uma variante sentinela de FrameItem para medir ausência de E0004 nos locais antes e presença depois. A sonda cobre somente os locais efetivamente compilados: não extrapolar aos helpers cfg(test) ou L3 que a dependência impede de alcançar. Para todo o lote, conferir padrões/guards/corpos por replay, enum e assinaturas inalterados, build e testes reais. Não substituir corpos por stubs.

Revalidar suíte workspace completa, cargo build, Clippy workspace/all-targets com -D warnings e -D unfulfilled_lint_expectations, formato, Rustdoc -D warnings com quatro índices HTML, linhagem V1/V5/V15/V26 estrita/reversa, SARIF e diff --check. Fontes/L0/configuração ficam congelados durante comandos. Execuções pesadas sequenciais com MemoryMax=12 GiB, MemorySwapMax=0 e CARGO_BUILD_JOBS=1. Não apresentar saneamento estrutural como nova paridade global, ganho de memória ou benefício medido para IA.


## TS1 — tamanho completo de texto (ratificado pelo humano, 2026-10-17)

### Medição anterior à decisão

`00_nucleo/diagnosticos/text-argumentos-2026-10-17/before.json.gz`, `before-measurement.json.gz`,
`morph-before-measurement.json.gz` e `audit.json` conservam HEAD
06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, UTC, status/diffstat,
fontes/L0/binários/argv e o pin9f2b6e8715237cb086899a42873660fe744622e8.
Em named-em, A mede0pt contra24pt; em affine,3pt contra27pt; nested sobre18pt
mede0pt contra36pt. As seis permutações Size/Color/body são aceitas pelo pin
mas rejeitadas pelo Tekt. Set posicional18pt é ignorado (12 contra18pt).
Fonte pinada text/mod.rs:290,303,924–930,1136–1142 e args.rs:178–247
estabelece busca tipada, preferência nomeada, body restante e composição.
style_chain.rs:513–517 e constructor.rs:123 descartam em; collapse em
style_chain.rs:394–395,419–422 aplica top-wins onde tamanho precisa fold.
É tamanho/escopo observável da linguagem; a representação Rust é escolha
interna. Não inferir intenção histórica. Refutar por restauração, Raw,
heading, math, capture/show, dupes/spans, zero/negativos ou métricas divergentes.
A sonda de igualdade refutou a hipótese de comparar os valores declarados:
no pin, text(size:2em)[A] == text(size:24pt)[A] é true. StyledElem::eq no pin
content/mod.rs:779–782 compara child. Esse defeito de igualdade requer sua
própria correção; o tipo completo não é prova de fechamento desse eixo.

### Ratificação

O humano aprovou TS1 e os dois contratos públicos da proposta de manifesto
771f5fd3d36545eb4b4fb7f0c0f2c91061470d1f010fc9a887b001a636d8feba.
Implementação e testes S1–S16 sob 12 GiB/swap zero; sem release/merge/push.
Resselo somente após implementação e grafo válido. RG1 continua independente.

### Responsabilidade exclusiva adicional

Somente fixtures/adapters de testes: substituir Style::Size(Pt) por Size(Length absoluto), e expectativas de StyleDelta.size por Option<Length>. Conservar os observáveis linguísticos e os contratos funcionais existentes; não trocar geometria correta por snapshots da representação nova.
