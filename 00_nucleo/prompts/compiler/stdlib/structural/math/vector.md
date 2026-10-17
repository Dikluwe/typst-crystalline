# L0 — compiler/stdlib/structural/math/vector
Hash do Código: 2f7b6a39

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/structural/math/vector.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Measurement before decision

At the uncommitted working tree captured in `00_nucleo/diagnosticos/atomizacao-stdlib-math-vector-2026-09-30-before.json.gz`, `01_core/src/compiler/stdlib/structural/math.rs:724-893` owns the vector options, delimiter conversion, named casts, content constructor and native function. The local `p1292_c_vec_tests` at `:2401-2422` checks pure ratio conversion. The `math_vec` test filter passed 3 tests before movement. This shows a coherent behavior boundary, not a speedup or fresh bilateral vanilla result.

## Contract

Own the complete `math.vec(..children, delim: "(", align: center, gap: 0.2em)` constructor family. It accepts variadic children, including zero children, converting Content, LocatedContent, String and Symbol to the same semantic content as before and diagnosing other types with the existing span and wording. Named `delim`, horizontal `align` and relative-length `gap` retain current cast order, default values and explicit-presence bits. A present named marks its bit even when equal to the default. Unknown named remains an error. Preserve the full `Rel<Length>` in the carrier, including a pure percentage converted to a relative component with zero absolute component.

The measured error vocabulary is retained: integer `gap` gives `expected relative length, found integer`; integer `align` gives `expected alignment, found integer`; integer `delim` gives `expected array, none, symbol, or string, found integer`. Delimiter strings, symbols, array pairs and none preserve their current mapping. No truncation, layout, region-dependent resolution, callback or dynamic registry is added. The percentage base and `height:auto` belong to `compiler/math/layout/vec.md`; this owner constructs semantic `Content::MathVec` only.

The parent `math.rs` owns static binding registration and calls the native function in this module. The local pure-ratio test moves with the constructor. No public signature, pipeline phase, default or compatibility change is authorized.

## Acceptance

The local ratio test and existing math/vector tests remain green. `cargo build` and `crystalline-lint .` have no new blocking violations. Each Prompt L0 has one productive consumer. No performance gain is claimed without a separate measurement.

## Exaustividade nominal Value — V16, 2026-10-16

### Medição anterior à decisão

M3 em 01_core/src/compiler/stdlib/structural/math/vector.rs:47. O inventário desta árvore não commitada (HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`) e os L0 vigentes foram lidos antes da prescrição. before.json.gz em `00_nucleo/diagnosticos/tekt-v16-math-value-2026-10-16/` conserva UTC, status, git diff HEAD --stat e hashes integrais. Value possui 39 variantes; os braços fallback medidos são internos e hoje uniformes. São avisos estruturais de evolução Rust, não prova de falha funcional ou de melhor leitura por IA. Não inferir intenção upstream.

### Decisão do owner e aceitação

M3 vec_delim_char aceita None, Str e Symbol. Enumerar nominalmente as outras 36 variantes com o mesmo retorno None. Preservar casts, diagnósticos, aliases e fallback de math_vec_delim sobre array, incluindo primeiro inválido e segundo ausente/inválido. Não alterar fronteiras que retornam erro para tipos inválidos nem as flags de opções.

Substituir somente o wildcard selecionado pelo complemento nominal das variantes atuais; compartilhar o corpo existente como grupo de alternativas equivalentes. V19 informativo é legítimo aqui por consequência/efeito idênticos e deve permanecer visível. Não duplicar corpos, criar helper de fachada, usar binding catchall, isenção ou mudança na configuração para esconder V16. Payloads não utilizados podem manter padrões internos `_` dentro de uma variante nominal. Assinaturas públicas, carriers, fase, defaults e diagnósticos não mudam.

Antes da implementação, escrever controles co-localizados para os casos aceitos/rejeitados e a morfologia/presença indicadas. Os controles de preservação devem passar antes/depois; o RED é V16 no fonte atual e o teste de evolução em cópia do workspace real: adicionar uma variante sentinela em Value deve ser barrado em cada decisão selecionada (hoje o fallback a absorve). GREEN exige E0004 nesses mesmos pontos, zero V16 selecionado, somente novos V19 equivalentes e nenhum outro diagnóstico novo. Os controles não afirmam cobertura de execução das 39 variantes nem paridade vanilla global.

Resselo direto/reverso apenas deste owner e consumer; revalidar build, workspace completo, Clippy all-targets com warnings/expectativas não cumpridas como erro, fmt, Rustdoc estrito, V1/V5/V15/V26 e linhagem reversa. Executar comandos pesados sequencialmente sob 12 GiB, swap zero, Cargo um job. Guardar comandos, fontes/hashes e UTC; preservar código alheio à etapa.
