# L0 — stdlib/foundations/query
Hash do Código: 33f15fe5

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/introspection/content-snapshot.toml sha256:5a0270231de70be1212dbd17298cce34b7161b527d74b4b589e3f4c69d35ce24
- 00_nucleo/prompts/_nuclei/selectors/native-bare-identity.toml sha256:a37779d03a4ba0f16ebcc5919477bfdcf783021511a28cf8c9203c67cd64a9e5

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/foundations/query.rs`

## Evidence and classification

`01_core/src/compiler/stdlib/foundations/query.rs:26`, `01_core/src/compiler/stdlib/foundations/query.rs:45`, `01_core/src/compiler/stdlib/foundations/query.rs:74` identifies the ownership boundary. Inspection is not new bilateral parity evidence or upstream intent. Historical evidence retains its original provenance in `lab/results/construction/diagnostics/sanitization-parser-stdlib-history.md`.

## Contract and ownership

Own metadata/query/locate/here/target native validation and projection. metadata embeds an opaque value in Content for introspection. Delegate selector parsing statically to `00_nucleo/prompts/core/compiler/stdlib/foundations/selector.md`, preserving established kind strings, label strings, Location/Selector/Label and element functions.

query preserves selected order/cardinality and each exact Location. Clone the complete stored entry into Value::LocatedContent(entry,loc), not only content or the caller's style chain; absent synthetic entry falls back to Value::Location. Present snapshots must not fall back field-by-field. Identical contents can remain language-equal with distinct locations; never search location by equality/hash/first occurrence. locate returns the first match Location or none, not all matches.

Preserve the entire Element selector including empty filter group/function identity/order. Before selection validate new Element leaves through the typed static native-recognition owner in `00_nucleo/prompts/core/compiler/eval/bindings/value_methods.md`; do not call constructors, compare names or infer invalid type from zero matches. text fails `text is not locatable`; Strong/Emph remain valid without requiring ElementKind. Compositions validate their new leaves while preserving legacy rules. This restriction does not apply to selector construction or show use.

After existing argument validation, record the effective query/locate request and observed result through EvalContext's private observation API. Empty query is an observation; equal locations with altered content/fields are not equality of results. locate records only its observed first occurrence/absence, not all later matches. Here records current_location or its absence; its existing contextual error remains. Revalidation selects/projects against the candidate without executing contextual bodies or constructors; use the observation comparator, not Debug/repr/location-only equality, and report incompleteness rather than success. Current-location identity is a retained-attempt precondition, not a candidate snapshot fact; a changed block requires fresh pipeline evaluation. Logging a read does not select a block for reexecution; Element-counter demand owns that selection. metadata/selector creation remain without reads.

target requires known context, rejects arguments and preserves `can only be used when context is known` with the existing hints. It returns the propagated target paged/html/bundle, with stable target/features as attempt inputs, not query results. The old paged-only statement contradicts current code at `01_core/src/compiler/stdlib/foundations/query.rs:159` and the separate-target architecture of ADR-0128; it is archived explicitly, not used to revert current public target observations. Do not change propagation, callbacks or pipeline phases.

## Acceptance

Preserve syntax, semantics, morphology, diagnostics/hints/spans, bindings, defaults and identities. No productive code or compatibility change is authorized by this sanitation. Preserve pinned shared claims and exclusive ownership under ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130.

## D08A — seletor bare de table.cell (PROPOSTO, gate ADR-0127)

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado;
UTC, diffstat, entradas, comandos, hashes das fontes e executáveis em
lab/results/construction/diagnostics/math-residuos-table-cell-2026-10-06/{before,table-additional-before,table-aliases-before}.json.gz.
Vanilla pinado 9f2b6e8715237cb086899a42873660fe744622e8.
selector(table.cell) devolve cell, where() devolve cell.where(:), a comparação
é false, query/counter rejeitam cell não locatable. Alias verdadeiro medido
por CodeBlock conserva esses resultados. A fixture anterior que definiu alias
apenas no documento não disponibilizava a variável à expressão eval e não
prova alias positivo; table-aliases-before corrige a fixture.

Fonte: lab/typst-original/crates/typst-library/src/foundations/selector.rs:309–321
distingue dict None de Some; :421–450 valida locatabilidade separadamente.
No Tekt, entities/selector.rs:34–37 só representa grupo Element presente;
stdlib/foundations/selector.rs:69–77,348–354 omite a nativa cell; os demais
limites atuais e hashes estão em d08-owner-audit.json.

Classificação: formato, identidade e rejeição de consulta são linguagem;
o novo carrier fechado é mecânica escolhida para preservar esses observáveis.
A distinção do vanilla é explícita na fonte; não se infere intenção apenas
da saída. Inferência: a nova variante basta à fatia de seleção/locatabilidade;
a hipótese será refutada por um caso público cuja ausência de grupo ou
identidade não caiba nela. Não se presume correção de realização de células.

### Contrato materializável somente após ratificação

query e locate consomem o novo seletor e validam locatabilidade antes de consultar ou registrar observação. A âncora da validação é value_span da primeira ocorrência posicional não detached, com fallback args.span. O erro cell is not locatable é preservado para função direta, alias, NativeElement, where e composições; não indexar células para resolver o erro. Preservar aridade, context read, identidade, metadata e target.

### Fronteira incompleta e aceitação

D08A cobre apenas carrier, construção, repr, igualdade, composição e
locatabilidade. D08 permanece incompleto: D08B completa show direto, projeção
de body/campos e preparação de células explícitas/implícitas, incluindo
posições, padding, spans, header/footer e escopo lexical. D08B precisa de
seus L0s medidos e de gate próprio caso mude contrato ou fase; esta ratificação
não o autoriza. Não esconder negativos D08B ou declarar paridade universal.

Após ratificação: promover este conjunto atomicamente; escrever testes
linguísticos e verificar RED válido antes do código; materializar com
despacho estático e Rust seguro; validar 1:1/V26 antes de ressellar somente
owners deste manifesto; build, suíte core e linter sem erros. Revalidar
selector cell bare/where/alias, igualdade true/false, query/counter diretos
e compostos, função homônima, partial/closure; preservar D05, D07 e query.
