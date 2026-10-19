# L0 — compiler/stdlib/structural/math/limits
Hash do Código: b38b90e9

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/selectors/math-native-bridges.toml sha256:19eb838b3e56711fa151d416bd24394a0a6f5c8286bb9ab2d0dd9bc542b4aee7

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/structural/math/limits.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Measurement before decision

The uncommitted working tree captured in `lab/results/construction/diagnostics/atomizacao-stdlib-math-restante-2026-09-30-before.json.gz` places this behavior in `01_core/src/compiler/stdlib/structural/math.rs`. The full core baseline passed 6,401 tests with 2 ignored before movement. This is a structural regression baseline, not a new bilateral vanilla measurement or benchmark. Historical language measurements and pin qualifications remain in the parent L0.

## Language contract

Own `scripts(body)` and `math.limits(body, inline: true)` constructors for MathLimitsOverride. Scripts accepts only one Content body, no named, and constructs lateral attachments. Limits validates named positional body, inline Bool, unknown named, and one Content body in the existing order; preserve explicit inline presence even when true. Keep native signatures and exact diagnostics; no layout.

The parent `math.rs` retains only static registration and shared catalog orchestration. Existing `compiler::stdlib` bridges retain their names and visibility via direct reexports. No public signature, default, pipeline phase or compatibility change is authorized.

## Acceptance

Existing core regressions remain green. The full core suite, `cargo build` and `crystalline-lint .` pass; header ownership is 1:1. No speedup is claimed without separate measurement.


## Identidade matemática nativa — 2026-10-17

### Medição anterior à decisão

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não
commitado; pin `9f2b6e8715237cb086899a42873660fe744622e8`.
`lab/results/construction/diagnostics/native-math-catalogue-2026-10-17/` fixa o estado
exato em before.json.gz: UTC, diff HEAD --stat e hashes de todos os consumers,
L0 e configuração protegidos. before-measurement.json.gz é reutilização
verificada da matriz final da etapa anterior: fontes e binário Tekt têm
identidade de hash com o baseline atual (baseline-reuse.json), não uma nova
execução. A matriz cobre os oito perfis de features e 3956 casos de funções;
26 rotas math ainda são recusadas em where/kind pelo Tekt e aceitas pelo pin.
before-batch.json.gz mede 227 casos detalhados de campos, exclusões, alias
parcial e counter, com comandos/binários/saídas e seus hashes.

batch-schema.json conserva blocos e SHA-256 das fontes primárias do pin,
file:line e declarações de ABI do Tekt; typst-macros/src/elem.rs:410–421
exclui internal/external de field_id. As anotações elem destas 26 funções
contêm Mathy e não Locatable. Isto é semântica/forma pública da linguagem,
não igualdade mecânica de Func nem inferência de intenção por render.

### Responsabilidade exclusiva e decisão

Somente a visibilidade Rust interna de native_math_scripts e
native_math_limits passa de pub(super) para pub(in crate::compiler), para
permitir reexport direto pelo parent. Parâmetros, retorno, corpos, diagnósticos,
ordem de validação e presença explícita permanecem literalmente preservados.
Nenhuma das duas funções vira API externa; nenhuma assinatura pública é
adicionada/modificada. Este adendo autoriza essas duas novas pontes internas,
preservando as pontes existentes descritas no contrato anterior.

### Aceitação e limites

Invariantes compartilhadas: consumir o Núcleo Tekt pinado
`00_nucleo/prompts/_nuclei/selectors/math-native-bridges.toml`; as
responsabilidades exclusivas acima definem o segmento de cada owner.

Correção interna de tabela/composição em fluxo contínuo ADR-0127. Nenhum campo,
trait, assinatura Rust externa, compatibilidade, default ou fase muda. L0
antes dos testes/código; RED funcional por identidade ausente, GREEN dos 26
casos com filtros e exclusões; controles de With, homônimo e helpers que não
são elementos. Validar antes/depois toda a matriz, preservando observáveis
fora das identidades autorizadas e aliases efetivos (accent/cancel globais
cristalinos incluídos, sem crédito de paridade se não existem no pin).

O sucesso de where/kind não certifica getters, síntese, indexação, show,
layout ou counter. Os controles counter permanecem explícitos e a sua
admissão separada não é alterada neste lote. As quinze rotas restantes de
formatos/HTML/PDF têm investigação própria; a colisão de callback dos formatos
não é resolvida por nomes. FG1/RM1/SY1/RG1 preservam escopos e gates.

Verificar ownership/V26 antes do resselo restrito aos cinco pares; build,
suíte completa, fmt, Clippy/Rustdoc estritos, linhagem e diagnóstico global
sem nova violação, sob MemoryMax12GiB/High10GiB/swap0/jobs1 sequenciais.
