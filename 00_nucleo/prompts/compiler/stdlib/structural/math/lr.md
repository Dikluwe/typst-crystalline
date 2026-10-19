# L0 — compiler/stdlib/structural/math/lr
Hash do Código: 55c1b2a0

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/structural/math/lr.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Measurement before decision

The uncommitted working tree captured in `lab/results/construction/diagnostics/atomizacao-stdlib-math-restante-2026-09-30-before.json.gz` places this behavior in `01_core/src/compiler/stdlib/structural/math.rs`. The full core baseline passed 6,401 tests with 2 ignored before movement. This is a structural regression baseline, not a new bilateral vanilla measurement or benchmark. Historical language measurements and pin qualifications remain in the parent L0.

## Language contract

Own `math.lr(..body, size: 100%)` and its bare bridge. Accept zero or more positional values via the shared content cast, interleave canonical comma text, preserve MathSequence morphology, and retain explicit `size` presence even at 100%. Shared size validation is owned by casts. Keep diagnostics, order and static native identity.

The parent `math.rs` retains only static registration and shared catalog orchestration. Existing `compiler::stdlib` bridges retain their names and visibility via direct reexports. No public signature, default, pipeline phase or compatibility change is authorized.

## Acceptance

Existing core regressions remain green. The full core suite, `cargo build` and `crystalline-lint .` pass; header ownership is 1:1. No speedup is claimed without separate measurement.

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

Adicionar apenas is_callable_lr_symbol(&str)->bool pub(crate). String de um
codepoint presente no conjunto completo de inícios de DELIMS do pin, mais
ceil/floor, é callable; vazio, sequência de codepoints e fechamento não listado
não são. O inventário lr-symbol-starts.json é a medição. Isso classifica o cast,
não cria chamadas por símbolo nem modifica math.lr/size/body/algoritmo.

Aceitação: L0 primeiro, RED funcional de Type/Symbol/valor textual antes do código,
GREEN e revalidação integral desta etapa; 12 GiB/swap zero/jobs1. Fluxo contínuo
ADR-0127, sem contrato Rust público, fase ou default de produto novo. Resselo só
dos owners após V15/V26; preservar testes de matemática e classificadores originais.
