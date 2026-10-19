# L0 — compiler/stdlib/structural/math/matrix
Hash do Código: b86af868

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/structural/math/matrix.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Measurement before decision

At the uncommitted working tree captured in `lab/results/construction/diagnostics/atomizacao-stdlib-math-matrix-2026-09-30-before.json.gz`, `01_core/src/compiler/stdlib/structural/math.rs:725-1042` contains `MathMatrixOptions`, relative-length and augment casts, named option application, row interpretation, rectangular padding and the `math.mat` native. Static binding occurs later in the parent. The `p1431` test filter passed 29 tests before movement. This is a structural and regression baseline, not a performance result or a new bilateral vanilla measurement.

## Language contract

Own `math.mat` with variadic positional rows and named `delim`, `align`, `augment`, `gap`, `row-gap` and `column-gap`. Unknown named and duplicate occurrence are errors; `rows:` is not accepted. Preserve validation order and the source spans and wording of each diagnostic. `gap` fills both axes first; specific row/column gaps override it. The defaults and explicit-presence bits are retained, including when a supplied value equals its default. Alignment must be horizontal. Relative length accepts Relative, Ratio and Length without losing its relative component. Delimiter conversion is called from the exclusive `compiler/stdlib/structural/math/cases.md` owner; this prompt does not duplicate that mapping.

Scalar positionals form one row. In array mode all positionals must be arrays; in delimited-row mode they must be supported parenthesized MathDelimited rows. Mixed incompatible shapes keep their existing errors. Convert each cell with the existing Content/LocatedContent/None/Symbol/value-to-content mapping. Pad shorter rows with Empty to the maximum width. Preserve row order, empty rows and comma splitting in delimited rows. Construct the complete semantic `Content::MathMatrix`; do not execute layout or separate `&`.

`augment` accepts none, an integer vertical offset or a dictionary with `hline`, `vline` and `stroke`. Offsets accept an integer or array of integers and are checked against dimensions under the same current source-span gate. The dictionary consumes these three recognized fields and tolerates other internal keys, matching the measured upstream asymmetry; this does not permit invalid types for known fields or unknown top-level named arguments. Stroke extraction uses the existing stdlib helper. Preserve the existing `MathMatrixExplicit` bits and `MathMatrixAugment` carrier.

The parent `math.rs` owns only static registration of the native binding. The structural hub reexports `MathMatrixOptions`, `math_matrix_augment` and `apply_math_matrix_option` directly from this child with unchanged visibility; eval and layout consumers keep their established bridge. No dynamic dispatch, public signature change, default, pipeline phase or compatibility break is authorized.

## Acceptance

A local regression checks rectangular padding and `gap` shorthand with a
specific `row-gap` override. The P1431 regression filter and full core suite
remain green. `cargo build` and `crystalline-lint .` have no new blocking violations. Header ownership is 1:1. No performance gain is claimed without a separate measurement.

## D07 — measurement and correction of shorthand stroke presence

`lab/results/construction/diagnostics/matrix-morfologia-2026-10-06/final.json.gz` records
`augment-json` and `augment-fields`: a supplied `2pt` is serialized as
`2pt + rgb("#000000")` instead of the pinned vanilla's `2pt`.
`lab/typst-original/crates/typst-library/src/math/matrix.rs:434` uses
`Stroke::from_value`; `01_core/src/compiler/stdlib/layout/casting.rs:179–197`
leaves `specified` empty for Length/Color shortcuts. The existing carrier
`entities/geometry.rs:37–44` already represents presence.

Decision: the matrix augment conversion must mark only thickness for a
Length shortcut and only paint for a Color shortcut after the existing
validated extraction. Preserve supplied Stroke presence, geometry, defaults,
diagnostics and all other extract_stroke consumers. This is an internal
parity correction, without a public contract change. Acceptance: a local
RED→GREEN test proves thickness present and paint absent for `2pt`; the
bilateral augment JSON and fields controls coincide.

## Resíduos D07 — âncoras individuais de casts

`lab/results/construction/diagnostics/math-residuos-table-cell-2026-10-06/before.json.gz` mede augment-array e augment-array-keys
com mensagem igual mas span agregado no Tekt. delim-int/align-int/gap-str e
Code #true/#false/#left/#center corroboram o mesmo caminho.
`compiler/stdlib/structural/math/matrix.rs:263–271` passa args.span aos casts;
`entities/args.rs:17–21` já transporta value_span. O vanilla ancora a conversão
do argumento ao valor fornecido. Diagnóstico é observável de linguagem.

Decisão: ao aplicar opções nomeadas, usar o value_span da ocorrência
correspondente não detached; se não houver origem individual, conservar
args.span. Não mudar validação de aridade, ordem gap/específicos, duplicação,
assinaturas ou transporte de Args. Aceitação: RED→GREEN de um argumento
augment-array com span individual e igualdade bilateral dos diagnósticos.


## Guards de domínio explícitos — V17, 2026-10-16

### Medição anterior à decisão

T259: 01_core/src/compiler/stdlib/structural/math/matrix.rs:305; T260: 01_core/src/compiler/stdlib/structural/math/matrix.rs:308. Fontes e L0 vigentes lidos antes da prescrição; SHA-256, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, status e git diff HEAD --stat em `lab/results/construction/diagnostics/tekt-guards-2026-10-16/before.json.gz`. O inventário anterior, reconfirmado por hash, registra 28 guards em 21 owners; a regra CI pinada manda desdobrar ou mover a condição para o corpo. A medição é estrutura Rust, não prova de defeito funcional ou de melhor leitura por IA. Nenhuma intenção upstream é inferida.

### Responsabilidade exclusiva e aceitação

Selecionar linhas com match da tupla (array_mode, delimited_mode, value): (true,false,Array) converte células, (false,true,_) interpreta linha delimitada; todos os outros produzem o mesmo erro de array no mesmo span. Conservar detecção prévia, ordem, padding e modo escalar quando ambos falsos; mistura com ambos true permanece erro.

Manutenção interna em fluxo contínuo ADR-0127: sem contrato público, default, compatibilidade ou fase novos. A responsabilidade permanece somente no consumer exclusivo deste L0. Não criar helper apenas para esconder a conjunção, não adicionar supressões nem mudar a configuração do linter. Executar controles funcionais existentes antes do código e preservar seus esperados; o RED real é o gate V17 com 28 avisos, GREEN exige zero V17. Conferir caminhos de sucesso/fallback/erro, curto-circuito e efeitos; resselo restrito por owner. Revalidar build, suíte workspace completa, Clippy all-targets com warnings e expectativas não cumpridas como erro, formato, Rustdoc com warnings como erro, linhagem V1/V5/V15/V26 estrita/reversa e SARIF sem novos avisos. Executar sob 12 GiB, swap zero e um job. Não alegar ganho de memória/desempenho ou paridade global sem medição separada.

## Exaustividade nominal Value — V16, 2026-10-16

### Medição anterior à decisão

M4 em 01_core/src/compiler/stdlib/structural/math/matrix.rs:114. O inventário desta árvore não commitada (HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`) e os L0 vigentes foram lidos antes da prescrição. before.json.gz em `lab/results/construction/diagnostics/tekt-v16-math-value-2026-10-16/` conserva UTC, status, git diff HEAD --stat e hashes integrais. Value possui 39 variantes; os braços fallback medidos são internos e hoje uniformes. São avisos estruturais de evolução Rust, não prova de falha funcional ou de melhor leitura por IA. Não inferir intenção upstream.

### Decisão do owner e aceitação

M4 está no match interno de math_matrix_augment, depois de extract_stroke: Length marca somente specified.thickness, Color marca somente specified.paint. O grupo nominal das outras 37 variantes preserva o corpo vazio, sem modificar os bits extraídos. No fluxo atual o grupo produtivo recebe Stroke; os outros tipos são rejeitados anteriormente pelo extractor ou tratados pelo match externo. Não promover essa impossibilidade atual a wildcard aberto. Preservar o payload Stroke fornecido, cap/join/dash/overhang e seus bits; as fronteiras externas de erro, spans e validação permanecem iguais.

Substituir somente o wildcard selecionado pelo complemento nominal das variantes atuais; compartilhar o corpo existente como grupo de alternativas equivalentes. V19 informativo é legítimo aqui por consequência/efeito idênticos e deve permanecer visível. Não duplicar corpos, criar helper de fachada, usar binding catchall, isenção ou mudança na configuração para esconder V16. Payloads não utilizados podem manter padrões internos `_` dentro de uma variante nominal. Assinaturas públicas, carriers, fase, defaults e diagnósticos não mudam.

Antes da implementação, escrever controles co-localizados para os casos aceitos/rejeitados e a morfologia/presença indicadas. Os controles de preservação devem passar antes/depois; o RED é V16 no fonte atual e o teste de evolução em cópia do workspace real: adicionar uma variante sentinela em Value deve ser barrado em cada decisão selecionada (hoje o fallback a absorve). GREEN exige E0004 nesses mesmos pontos, zero V16 selecionado, somente novos V19 equivalentes e nenhum outro diagnóstico novo. Os controles não afirmam cobertura de execução das 39 variantes nem paridade vanilla global.

Resselo direto/reverso apenas deste owner e consumer; revalidar build, workspace completo, Clippy all-targets com warnings/expectativas não cumpridas como erro, fmt, Rustdoc estrito, V1/V5/V15/V26 e linhagem reversa. Executar comandos pesados sequencialmente sob 12 GiB, swap zero, Cargo um job. Guardar comandos, fontes/hashes e UTC; preservar código alheio à etapa.
