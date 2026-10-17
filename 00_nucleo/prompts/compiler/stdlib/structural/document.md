# L0 — stdlib/structural/document
Hash do Código: 12024132

**Vanilla ratificado**: `586e1bd43`; `document` é independente, `asset` pertence a `typst-bundle/src/format.rs`.

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/structural/document.rs`

## Evidence and classification

`01_core/src/compiler/stdlib/structural/document.rs:25`, `01_core/src/compiler/stdlib/structural/document.rs:100`, `01_core/src/compiler/stdlib/structural/document.rs:145` identifies the ownership boundary. Inspection is not new bilateral parity evidence or upstream intent. Historical evidence retains its original provenance in `00_nucleo/diagnosticos/sanitization-parser-stdlib-history.md`.

## Contract and ownership

Own pure direct native_document/native_asset constructors, bundle-path normalization and string-list extraction, not set rules or target compilation. Preserve the existing direct bundle signatures, not the retired metadata-only document()/asset(path,kind:) placeholders.

asset requires two positionals path and data and no kind. Path is a nonempty Str normalized lexically to the virtual bundle root: repeated slashes and "." disappear, ".." removes one component without escaping root. Reject backslash (`backslashes are not allowed in bundle paths`, hint `use forward slashes instead`), escape (`path {path} would escape the bundle root`) and no components (`path must have at least one component`). data accepts UTF-8 encoded Str or byte-exact Bytes. Emit Content::Asset with normalized path/data; no reading/writing or extension-based categorization. Preserve missing path/data, third positional, named path/data/kind and cast diagnostics measured in `00_nucleo/diagnosticos/typst-p1485-bundle-sondas.md`, without claiming a new probe.

document requires positional path:Str and body:Content. Optional format is Func; metadata title is Content, author/keywords Str|Array[Str] normalized as lists, date Datetime. Preserve metadata and format explicitness in the Content carrier. Missing direct path is not silently metadata; set document remains an eval-rule operation for individual targets. Outside Bundle target retain `constructing a document is only supported in the bundle target`, whole-call span and target/set-rule hints. Bundle target inference/explicit format compilation is L3 ownership, not a native I/O operation. Registry feature gating is separately owned and must not publish the retired placeholder. Historical public-contract proposal/gates are archived, not newly ratified.

## Acceptance

Preserve syntax, semantics, morphology, diagnostics/hints/spans, bindings, defaults and identities. No productive code or compatibility change is authorized by this sanitation. Preserve pinned shared claims and exclusive ownership under ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130.


## Projeções de Value — exaustividade nominal, 2026-10-16

### Medição anterior à decisão

01_core/src/compiler/stdlib/structural/document.rs:149 (diagnostic_type_name, emitted-v16). O enum atual em 01_core/src/entities/value.rs:59–195 tem 39 variantes materializadas; o catálogo e forma de payload estão no inventário desta etapa, não na tabela histórica de variantes futuras. before.json.gz em 00_nucleo/diagnosticos/tekt-v16-value-projections-2026-10-16 conserva HEAD, working tree não commitado, UTC, status/diff HEAD --stat, hashes e os textos originais dos owners/consumers. contexts.json foi extraído do AST atual conferido com o SARIF pinado; classification.json distingue nove V16 emitidos e oito projeções latentes. Forma Rust/exaustividade é mecânica; nomes, conversões, empréstimos e observáveis de linguagem mantêm o contrato vigente. Não há nova medição de paridade ou desempenho.

### Decisão interna

diagnostic_type_name conserva somente Int→integer e Bool→boolean; Str continua str pela delegação de value.type_name(), não string. Nomear as 37 variantes complementares no mesmo corpo delegado. Não alterar native_document/native_asset, bindings/targets, normalização de caminho, cast de data, mensagens/hints/spans ou assinatura/default de bundle. Teste C7: integer/boolean, Str→str, demais nomes none/auto/float/array/content/bytes; distinguir explicitamente a política deste owner da de format_options.

As variantes que compartilham resultado conservam um corpo comum. A lista nominal exige decisão se Value cresce; não duplicar corpos para remover V19. Notas V19 permanecem informativas e registram alternativas escritas, não testes executados, combinações alcançáveis ou déficit N×. Controles positivos/negativos e limites são escolhidos pelo comportamento, sem tratar a execução de um braço como prova de cada alternativa. Nenhum allow/expect ou relaxamento de severidade/exceção para ocultar V16.

### Gate e aceitação

Fluxo contínuo ADR-0127: nenhuma assinatura/trait/campo público, nova variante produtiva, default de produto, quebra de compatibilidade ou fase diferente. L0 antes de testes e implementação, resselo bilateral restrito e V1/V5/V15/V26 válidos. Escrever controles no módulo, confirmar comportamento antes e depois; RED estrutural é ausência de E0004 nos 17 pontos selecionados numa cópia do core com variante sentinela acrescentada somente ao Value copiado. GREEN exige E0004 em cada um dos 17 pontos, sem stubs, e remoção exata dos nove V16 selecionados com os demais diagnósticos preservados. Build, suíte workspace, Clippy estrito, formato, Rustdoc e linhagem sob 12 GiB, swap zero e um job Cargo. O gate global permanece aberto enquanto os V16 externos existirem. O falso positivo do parser sobre None explícito é investigação separada e não será escondido neste consumer.
