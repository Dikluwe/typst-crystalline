# L0 — infra/pipeline
Hash do Código: 7d2c0c47

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/compiler-feature-gates.toml sha256:59d8938dc06d347ccc9db23ae1b740876b369227daacd266a219811a661b3cb9
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67
- 00_nucleo/prompts/_nuclei/expression-source-identity.toml sha256:fb383f227b0bcd8b1e61899942fa0383a4cda66093da58bda2e87844e2895d2a

**Layer:** L3
**Exclusive consumer:** `03_infra/src/pipeline.rs`

## Coleta contextual em Table sem reconstrução — 2026-10-09

### Medição anterior à decisão

`lab/results/construction/diagnostics/table-local-scaling-followup-2026-10-09/stacks.json.gz` conserva HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, estado não commitado, hashes, comandos e UTC. Em 1.600 células com `set` local, 4.304 amostras de alocação passam pela coleta contextual contra 952 no bloco sem `set`; `TableElem::map_content` aparece em 9.835 contra 3.430 amostras. A tentativa de pular apenas a segunda ordenação vazia (`red-gate.json.gz`/`green-gate.json.gz`) reduziu 1.394.597 para 1.365.741 solicitações, 2,07%, aquém do gate de 5%, e foi revertida. Fonte: `pipeline.rs:403-460` usa `map_content` como visitante; `entities/content/traversal.rs:25-45` e `entities/elements/table{,_cell}.rs` reconstroem Table/Cell/Sequence até quando o callback só observa. Isto é custo mecânico, não propriedade da linguagem. Inferência: uma descida emprestada das três variantes evitará parte dessas alocações; o teste de alocações poderá refutá-la.

### Decisão interna e limite

Na coleta de `ContextBlock`, percorrer `Sequence`, `Table` e `TableCell` diretamente por referências, conservando a **mesma projeção visual e ordem** que `map_content`: Sequence em ordem; Table resolvida percorre cells resolvidas e depois caption, ignorando children/header/footer originais; Table crua percorre children, header, footer e caption; TableCell standalone percorre apenas visual standalone, senão visual resolvido, senão body cru. Usar a mesma recursão de coleta para cada filho, de modo que `Styled`, ShowRecipe, Located, Realized e Source conservem seus escopos e projeções. Para todas as outras variantes, manter o fallback exaustivo via L1 `map_content`; não criar uma tabela parcial sem fallback em L3. Não mudar substituição, ordenação, tratamento de IDs repetidos, fase, assinaturas públicas ou semântica de `map_content`.

Correção interna ADR-0127 em fluxo contínuo: L0 antes do código, gate RED→GREEN de pelo menos 5% menos solicitações no caso de 1.600 células, testes de contexto em Table/Cell crua e preparada, estilos/carriers/ordem, paridade de saída e revalidação dos casos contextuais existentes. Se a projeção direta divergir do `map_content` ou o ganho não ultrapassar o gate, reverter esta especialização.

Pinned Tekt dependencies (canonical linter metadata):

```text
```

## Current evidence

- `03_infra/src/pipeline.rs:174` — `eval_expression_with_document_features`.
- `03_infra/src/pipeline.rs:244` — `eval_to_module_with_sink_target_features`.
- `03_infra/src/pipeline.rs:355` — `collect_context_blocks_into`.
- `03_infra/src/pipeline.rs:409` — `substitute_context_blocks`.
- `03_infra/src/pipeline.rs:438` — `Timings`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/pipeline.rs`.

## Owned contract

- Own the evaluation adapters, comemo setup (`Routines`, `Traced`, `Sink`, `Route`), causal context walkers, public expansion facades and `Timings` serialization. Return raw diagnostics; never format them or write output.
- Isolated expression evaluation must not manufacture markup content. Document-context evaluation compiles through the paged owner, evaluates against its final introspector, and appends expression warnings after compilation warnings without exporting.
- Keep compatibility defaults (`full_error=false`, empty features, Paged evaluation) and explicit target/features unchanged. Reject assets outside Bundle with `assets are only supported in the bundle target` and hint `try enabling the bundle target`.
- Discover context blocks through the exhaustive L1 content traversal, accumulating the effective ancestor `StyleChain`. Substitute through the same topology; delegate attempt coordination to its child rather than implement a second stabilization algorithm.
- Preserve public import paths through explicit reexports. Private helper access stays within this module family. Target compilation, resources and stabilization belong to the child owners listed below; this prompt does not own their bodies.
- Timing fields describe existing production phases, including shaping and subsetting. Do not add boundary callbacks, an observation ledger, extra replay or an instrumented default.

## Child owners

- `00_nucleo/prompts/infra/pipeline/bundle.md` owns only `03_infra/src/pipeline/bundle.rs`.
- `00_nucleo/prompts/infra/pipeline/context_stabilization.md` owns only `03_infra/src/pipeline/context_stabilization.rs`.
- `00_nucleo/prompts/infra/pipeline/html.md` owns only `03_infra/src/pipeline/html.rs`.
- `00_nucleo/prompts/infra/pipeline/paged.md` owns only `03_infra/src/pipeline/paged.rs`.
- `00_nucleo/prompts/infra/pipeline/pdf.md` owns only `03_infra/src/pipeline/pdf.rs`.
- `00_nucleo/prompts/infra/pipeline/raster.md` owns only `03_infra/src/pipeline/raster.rs`.
- `00_nucleo/prompts/infra/pipeline/resources.md` owns only `03_infra/src/pipeline/resources.rs`.
- `00_nucleo/prompts/infra/pipeline/svg.md` owns only `03_infra/src/pipeline/svg.rs`.
- `00_nucleo/prompts/infra/pipeline/tests.md` owns only `03_infra/src/pipeline/tests.rs`.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.


## Realização com avaliação única

Timings.eval_ms cobre avaliação única e realização de receitas da fachada compatível; não descrever a métrica como duas execuções do programa.


## Realização com avaliação única

Coleta contextual atravessa raiz de carriers explicitamente e somente o visual de Realized; o contexto de Located não desaparece porque seu corpo raiz não recebe callback duplicado.


## Realização com avaliação única

Visitante bottom-up reentra nos carriers com a cadeia causal correta, coletando apenas contextos que participam do visual atual.


## Realização com avaliação única

Substituição contextual conserva o Located do marcador original antes do resultado resolvido, preservando here/location e registro de Position. Não trocar a raiz locatable por Sequence dentro do mesmo Located.

## Remoção de wrapper privado sem consumidor — 2026-10-05

Depois de a sessão usar o helper com métricas explícitas, `rg -n "layout_with_realized_math_callbacks\(" 03_infra -g "*.rs"` encontra apenas a definição em paged.rs:592. O import privado em pipeline.rs:90 fica sem uso. Remover essa fachada privada sem consumer e seu import; preservar os helpers com métricas explícitas e com seed usado por bundles. Não alterar reexports públicos, assinaturas públicas ou defaults dos callers remanescentes. Correção interna e reversível no mesmo domínio, fluxo contínuo ADR-0127.


## Preparação de bindings de exportação — 2026-10-05

Medição: `lab/results/construction/diagnostics/identidade-fonte-shaped-2026-10-05/probe.txt`. Consumir helper privado resolve_export_fonts: fontes originais vinculadas, com gate de instanciação existente. Instanciação fica no consumidor exportador, na mesma fase de render/export, preservando diagnósticos/API/seleção de páginas. O nome antigo prometia bytes já instanciados e não descreve este contrato.

## Identidade de documento e expressão — gate confirmado 2026-10-06

### Evidência anterior à decisão

`lab/results/construction/diagnostics/diagnostic-paths-2026-10-06/identity-next-audit.json`
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

Adicionar fachadas públicas sem serialização/export:

```rust
pub fn eval_source_with_sink_features(
    world: &dyn World, expression: &Source, features: Features,
) -> (SourceResult<Value>, Vec<SourceDiagnostic>);
pub fn eval_source_with_document_features(
    world: &dyn World, document: &Source, expression: &Source, features: Features,
) -> (SourceResult<Value>, Vec<SourceDiagnostic>);
```

A primeira delega ao novo entrypoint L1 com introspector vazio. A segunda usa
exatamente o compilador paginado existente, recebe seu introspector final e
delega ao entrypoint Source-based. Em falha do documento, retorna suas fontes
cruas e não avalia expressão; em sucesso acrescenta warnings da expressão aos
do documento, sem perder spans/traces. Não reexecutar programa para localizar
erros, reconstruir índice ou transportar bool inferindo a fase do diagnóstico.
Preservar as fachadas string e suas assinaturas/compatibilidade existentes.
Nenhuma função privada do paged torna-se pública por conveniência de wiring.

Gate ADR-0127 confirmado pelo dono em 2026-10-06 (resposta: confirmo), conforme
lab/history/construction/records/00_nucleo/materialization/origem-fontes-expressao-aprovacao-2026-10-06.json.
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

Este owner adiciona a fachada pública:
```rust
pub fn compile_query_context(
    world: &dyn World,
    source: &Source,
) -> (SourceResult<typst_core::entities::introspector::TagIntrospector>, Vec<SourceDiagnostic>);
```
Com Timings local, chamar compile_to_paged_document_full_error_and_features_with_introspector
existente (full_error=false, Features::default()), projetando somente o
introspector final do par e descartando PagedDocument sem exportar. Preservar
Source causal e warnings crus de compilação. Sem segunda avaliação, novos
flags, observador, caches persistentes ou mudança do pipeline para outros callers.
O algoritmo de paginação/contexto continua com os child owners vigentes.


## D08B3-C2 — responsabilidade ratificada

Medição anterior: lab/results/construction/diagnostics/table-origens-d08b3c2-2026-10-07/baseline.json.gz registra HEAD/working tree/UTC/fontes/argv e binários no pin 9f2b6e8715237cb086899a42873660fe744622e8. A fonte desta unidade tem a operação transparente correspondente nos ramos existentes de ShowRecipe/Located/Realized; Source não pode vazar como elemento observável. Aprovação humana `confirmo`, 2026-10-07, contrato em lab/history/construction/records/00_nucleo/materialization/table-origens-d08b3c2-2026-10-07/contract.md.

A delegação transparente já owned de ShowRecipe.body também aceita Source.body, sem executar origem separada, publicar carrier ou atribuir Location. Preservar classificação/preparo/serialização conforme a operação desta unidade.

Manter owner exclusivo e despacho estático; sem I/O, assinatura/trait/flag nova ou fase eval↔layout. Testes e coletas C2/C/macro, builds e lineage necessários antes de certificar.

## Imports e documentação de CI — 2026-10-13

### Medição anterior à decisão

`03_infra/src/pipeline.rs` nas linhas 39, 85, 101, 127, 148 contém os avisos selecionados. Fontes/owners, spans, saída original do compilador, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, status e diffstat estão em `lab/results/construction/diagnostics/infra-imports-docs-ci-2026-10-13/before.json.gz`. A pesquisa distingue referências produtivas, imports necessários somente à suíte descendente e menções em comentários; não constitui nova medição de paridade.

### Decisão e aceitação

Remover somente os imports privados comprovadamente sem uso neste consumer; manter os reexports públicos e os imports exigidos por chamadas ou contratos. A lista exata de spans anterior à edição está no diagnóstico citado.

Os helpers compile_to_paged_document_full_error, collect_bundle_entries, require_ua1_image_alt, resolve_pdf_pages_and_tags, first_font_from_doc e resolve_font ficam importados somente sob cfg(test), conservando o acesso da suíte descendente. Remover apenas o import não usado de instantiate_variable_font, sem eliminar a implementação ou alterar o caminho de instanciamento.

Manutenção interna sem mudança de assinatura pública, default, fase, fórmulas ou critérios de aceitação (fluxo contínuo ADR-0127). Não introduzir supressões de lint. Revalidar compilação de todos os targets, nomes e execução da suíte de infraestrutura, build, Clippy do workspace, Rustdoc e linhagem com resselo restrito. Não criar testes que apenas espelhem remoção de imports ou formatação de comentários.


## Tipos explícitos e leitura local — type_complexity (2026-10-15)

### Medição anterior à decisão

03_infra/src/pipeline.rs:163,212.
O snapshot before.json.gz em lab/results/construction/diagnostics/type-complexity-2026-10-15
conserva HEAD, working tree não commitado, UTC, diffstat, fontes/owners e hashes.
O gate red-clippy.log falha com este lint promovido a erro antes da edição.
A inspeção distingue tuplas posicionais, repetição do alias FontKey e contratos
estáticos já explícitos. É manutenção de representação Rust; não medição de
benefício da IA, bug funcional ou paridade nova.

### Responsabilidade e aceitação

Adaptar somente os consumidores do resultado privado PagedCompilation: compile_query_context projeta introspector por nome; eval_source_with_document_features extrai o mesmo introspector por movimento. Preservar assinaturas públicas, compilação única, spans/source identity, append de warnings e expressão contra o snapshot final. Não executar nova compilação ou exportação.

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


## A1-H2 — suplemento antes das consultas, aprovado em 2026-10-17

### Medição anterior à decisão

HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c, working tree não commitado;
final-freeze.json.gz e target-controls.json.gz no diagnóstico
lab/results/construction/diagnostics/lote-a1-callbacks-reflexao-seletores-2026-10-17 registram
UTC/diffstat/fontes/argv. Pin bd5ebec285505889e8ff1d4f68804ab147a699f3.
heading-raw-constructor-func passa no baseline/vanilla e falha no candidato:
Styled de transporte altera func. Query também vê wrapper causal temporário;
HTML ignora callback panic. Fonte walk.rs:930–945; pipeline/paged.rs:419–427 e
pipeline/html.rs:101–106 preparam a consulta pelo walk puro. São valores e erros
de linguagem, não um requisito de copiar algoritmo/estrutura do vanilla.
Gate A1-H2 confirmado pelo dono nesta sessão: revisão
lab/history/construction/records/00_nucleo/materialization/heading-supplement-runtime-2026-10-17/review.md,
SHA-256 66739b13596013f4343bdc3ffacbdb9081151e3d9863e5d962e8ff13fd68775c.

### Responsabilidade exclusiva

Declarar somente mod runtime; ele compõe Engine/ctx causais para os filhos.
Não incluir lógica semântica de Heading neste hub. Preservar exports e APIs.

### Aceitação e limites

H1–H8 da revisão ratificada, RED bilateral antes do código. Nenhum contrato
público além da fachada Heading está autorizado por H2; A1-S2 cobre separadamente
cinco helpers State. Sem novos campos públicos. Styled ordinário e demais
callbacks conservam suas fronteiras. Workspace/doctests/build/Clippy/Rustdoc,
fmt, V1/V5/V15/V26/reversa e V16 sem aumento; 12GiB/High8GiB/swap0/jobs1,
comandos pesados sequenciais. Ganho de RAM exige medição, não intenção.
