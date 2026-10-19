# Prompt L0 — `entities/elements/outline_entry` — `OutlineEntryElem`
Hash do Código: 44be7b9b

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67

**Camada:** L1 · **Alvo:** `01_core/src/entities/elements/outline_entry.rs`

## P1402 — entrada pública de outline

### Medição antes da decisão

No vanila ratificado `586e1bd43`, `outline.entry(1, heading[H])` constrói
`entry(level: 1, element: heading(body: [H]))`. Uma ocorrência realizada por
`#outline()` expõe `level`, o heading completo em `element` e o fill default
`repeat(body: [.], gap: 0.15em)`. `body()` devolve `[H]`; um `strong` produz
`cannot outline strong`. Fonte: `typst-library/src/model/outline.rs:468-522,
668-674`.

### Obrigação

`OutlineEntryElem` é unidade nativa estática, nunca `Dynamic`. Guarda `level:
usize`, `element: Content` e `fill: Option<Content>`. O nível é maior que zero.
`get_field` expõe os três campos. `body()` aceita Heading nesta fatia e devolve
seu Content sem converter para texto; outras variantes falham explicitamente no
consumer de chamada. `map_content` e `map_text` recursam em `element` e `fill`.
`plain_text` segue o body do elemento outlinable. A identidade pública é
`outline.entry`, mas a entidade não conhece scopes nem avaliação.

Como elemento locatável, `element_kind()` devolve
`ElementKind::OutlineEntry`. Essa identidade alimenta query sem transformar o
tipo em função dinâmica.

Não implementar `indented`, `inner`, `page` ou `prefix` em P1402.

## P1403 — dados causais privados para `prefix()` e `page()`

Após a confirmação humana do contrato em
`compiler/stdlib/structural/outline.md` §P1403, a unidade guarda também, sem os
expor em `get_field`, o prefixo semântico já realizado do elemento fonte e sua
`Location` causal. Entries construídas diretamente inicializam ambos como
ausentes; entries sintetizadas pelo introspector recebem os dois. `prefix()`
devolve o Content preservado ou `none`; `page()` pertence ao consumer de
stdlib, que usa a Location. Walkers preservam esses metadados e só transformam
o Content do prefixo. P1403 não implementa `inner` nem `indented`.


## Alinhamento OutlineEntry e walk exaustivo — 2026-10-16

### Medição anterior à decisão

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, lista exata git diff HEAD --stat, fontes/hashes e binários em `lab/results/construction/diagnostics/tekt-v16-introspect-walk-2026-10-16/before.json.gz`. Medições CLI anteriores em measurement-before.json e measurement-fields-before.json: no pin vanilla 9f2b6e8715237cb086899a42873660fe744622e8, entry direta aparece uma vez, sintetizada uma vez e mista duas vezes; no Tekt, a direta some da query (0 contra 1), mista perde a direta (1 contra 2), e body() direto falha cannot outline heading. Fonte: entities/elements/outline_entry.rs:61 body só aceita Heading bruto; :99 fornece NativeElement; introspect/extract_payload.rs:148 descarta OutlineEntry; walk.rs:805 trata entry terminal; outline_entries.rs:38–61 sintetiza entries após walk. Generic map_content visita element/fill (traversal.rs:221); realized.rs:139–281 usa essa travessia para certificar origens, motivo para testar um heading referenciado Located sem confundi-lo com descendente renderizado. O upstream pinado declara OutlineEntry Locatable em outline.rs:472–481. Isto mede semântica/morfologia e erros observáveis da linguagem, não impõe igualdade Rust/bytes/PDF ou mecânica vanilla. A hipótese de correção é refutada por query/fields/body errados, headings fantasmas, duplicação ou regressão em síntese/ordem/labels.

### Responsabilidade exclusiva vigente

Normalizar somente self.element por source_body() no match de body(), conservando o Content semântico do heading.body por clone e o erro cannot outline <elem> para outros tipos. Não remover Source do campo armazenado, não unwrap Strong/Emph, não inventar prefix/location, não executar callbacks, não percorrer campos como conteúdo visual nem ampliar os tipos outlinable. Preservar assinaturas, campos, construtores, to_payload, map_content/map_text e get_field. Adicionar o pin canônico do núcleo content-source-origin. Isso restaura transparência de origem já vigente e paridade medida no pin atual; não é novo modo/default.

Esta revisão supersede somente as ressalvas anteriores de pendência OutlineEntry neste owner; preserva os demais contratos vigentes.

### Aceitação

L0 primeiro, testes co-localizados antes da implementação e RED funcional real; manter controles existentes e a medição CLI anterior intacta. Repetir fixtures direct/synthesized/mixed com body() e com acesso ao campo, comparar resultados de linguagem, não bytes PDF. Sonda Content sentinela separada em cópia do workspace real: os seis pontos W1–W6 devem acrescentar E0004 após a correção e não antes, sem outros erros; preservar logs/inputs. Resselo só dos cinco pares afetados e grafo V15/V26 válido. Gates: build workspace, fmt, Clippy all-targets -D warnings e unfulfilled_lint_expectations, Rustdoc -D warnings, suíte workspace completa, V1/V5/V15/V26 e reversa, diff-check e SARIF diferencial. Esperado: seis V16 removidos (239→233), seis V19 informativos novos; V19 existente do complemento do extractor perde uma alternativa e os dois grupos de needs_fresh_index migram OutlineEntry de false para true. Nenhum V20 novo ou outro diagnóstico alterado após mapear deslocamentos. V19 é grupo de consequência uniforme prescrito aqui, não cobertura de execução de cada alternativa. Sem supressões ou configurações globais. 12 GiB, swap zero, um job Cargo, comandos pesados sequenciais; estado/UTC/hashes registrados. Correção interna/paridade ADR-0127, sem API/campo/trait/default/fase ou compatibilidade novos. Não publicar, commitar ou fazer push. O objetivo geral permanece aberto com os demais V16.


### Complemento de transparência — Located em element

O novo caminho produtivo atribui Located às ocorrências; a entidade aceita Content de heading e a Location é carrier de identidade, não novo elemento da linguagem. Acrescentar teste anterior à mudança com Located(Source(Heading)) e Source(Located(Heading)), conservando o body semântico Strong e o campo element intacto. Se refutar o match atual, body deve percorrer somente Source/Located por referências até a variante semântica, conservando a decisão Heading ou o mesmo erro de outros tipos. Não unwrap Realized/Strong/Styled nem fabricar Location/prefix. O RED desse complemento é registrado separadamente do RED inicial; nenhuma mudança de interface ou fase.


## I2 — leitura emprestada do prefixo (2026-10-17)

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado;
lab/results/construction/diagnostics/tekt-v16-bibliography-static-2026-10-17/before.json.gz
conserva fontes/hashes/status/diffstat/UTC. outline_entry.rs:73–75 retorna
Option<Content> por clone. O campo prefix privado não pode ser emprestado pelo
visitante de outro módulo sem um accessor interno. Isso mede contrato/custo
de navegação, não ganho de linguagem ou benchmark de heap.

### Responsabilidade exclusiva e aceitação

Adicionar apenas pub(crate) prefix_ref(&self)->Option<&Content>, devolvendo
self.prefix.as_ref(). Preservar prefix() público, campos, construção, Location,
body(), mapeamento e todos os comportamentos. O visitante estático usará a
referência e não clonará o Content para navegar. Teste local deve confirmar
None na entry direta e referência ao mesmo Content armazenado na realizada,
sem usar isso como prova de paridade. Sem método de trait, API pública,
default ou fase novos; fluxo interno ADR-0127. Validar com o lote I2 e resselo
restrito aos três owners; 12 GiB, swap zero, jobs1, sem publicação.
