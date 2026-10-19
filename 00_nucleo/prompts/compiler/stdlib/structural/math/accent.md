# L0 — compiler/stdlib/structural/math/accent
Hash do Código: d7ebae7b

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/structural/math/accent.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Measurement before decision

At the uncommitted working tree captured in `lab/results/construction/diagnostics/atomizacao-stdlib-math-accent-2026-09-30-before.json.gz`, `01_core/src/compiler/stdlib/structural/math.rs:393-550` contains the alias table for combining marks, normalization, base/accent/size/dotless casts, semantic constructor and native. Static registration occurs later in the parent. The `p1429` filter passed 9 tests before movement. This is a structural regression baseline, not timing or a new bilateral vanilla measurement.

## Language contract

Own `math.accent(base, accent, size: 100%, dotless: true)`. Base and accent are required positionals. Base accepts Content, LocatedContent, String and None with the existing semantic conversion; other types preserve the content-cast diagnostic. Accent accepts a one-codepoint String or Symbol, or MathText containing one symbol. Normalize direct combining marks and the existing fixed alias table to canonical combining characters; preserve the mapping for hat, arrows, overline and other entries without environmental lookup. Invalid or multi-codepoint inputs keep their distinct current diagnostics.

Named `size` accepts Relative, Ratio and Length, preserving the full `Rel<Length>` and marking explicit presence even at the default. Named `dotless` accepts boolean and marks its explicit bit. Unknown, duplicate, positional-as-named, missing and excessive arguments retain their current validation order, spans and exact diagnostic wording. Construct the complete semantic `Content::MathAccent` carrier; do not execute layout or add a second table in eval or entities.

The parent `math.rs` owns only static `math.accent` registration. The structural hub reexports `math_accent_content` for eval and `native_accent` for global/module registries directly from this child, preserving visibility and public identity. No public signature, default, pipeline phase or compatibility change is authorized.

## Acceptance

The P1429 regression filter and full core suite remain green. A local test checks alias normalization and direct combining-codepoint preservation. `cargo build` and `crystalline-lint .` have no new blocking violations. Header ownership is 1:1. No speedup is claimed without separate measurement.

## Complemento de casts de figure — Type e Symbol, 2026-10-17

Medição anterior à decisão: type-kind-probe.json e more-cast-probe.json em
lab/results/construction/diagnostics/tekt-v16-figure-casts-2026-10-17 registram o candidato
que passou 5/6 testes mas falhou em kind:int. foundations/value.rs:647-658 do
pin 9f2b6e8715237cb086899a42873660fe744622e8 mede Func aceitando Type.constructor
ou Symbol.func; content/element.rs:210-214 exige to_element; symbol.rs:133-137,
math/accent.rs:189-214 e math/lr.rs:144-198 classificam a conversão simbólica.
Há 37 tipos exercitados no CLI; Location sem construtor foi conferido na
fonte pinada. type-kind-classes.json guarda a classificação completa dos 38.
Símbolo aceito como corpo/caption é convertido pelo pin a SymbolElem; o nosso
Content não tem essa variante e o markup atual conserva o valor como Text.
Esse carrier/morfologia exige evolução pública SY1, não pode ser declarado
resolvido pela preservação do texto nesta etapa.

Decisão exclusiva deste owner:

Adicionar apenas is_known_math_accent(&str)->bool pub(crate), consultando a
única ACCENT_ALIASES existente: codepoint canônico ou alias exato. Ao contrário
de normalize_math_accent, não aceitar qualquer char pelo fallback direct.
O helper é classificação pura para cast Function; não chama função, não
muda normalize_math_accent, construção de Accent nem layout.

Aceitação: L0 primeiro, RED funcional de Type/Symbol/valor textual antes do código,
GREEN e revalidação integral desta etapa; 12 GiB/swap zero/jobs1. Fluxo contínuo
ADR-0127, sem contrato Rust público, fase ou default de produto novo. Resselo só
dos owners após V15/V26; preservar testes de matemática e classificadores originais.
