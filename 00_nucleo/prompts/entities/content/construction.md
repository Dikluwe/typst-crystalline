# L0 — entities/content/construction
Hash do Código: 791c4f48

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/raw-field-presence.toml sha256:f29e26ff07053f940cfe4ab5358fc9a6ac8d4d689528e169b5f8567492f52f46

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/content/construction.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Own the residual public `Content` constructors other than sequence normalization. The math, layout, table and reference families are owned respectively by `00_nucleo/prompts/entities/content/construction/math.md`, `layout.md`, `tables.md` and `references.md`. They must preserve every signature, default, omission
state, and payload. Construction remains pure L1 and uses only closed native
variants plus the existing explicit `Dynamic` extension boundary. No registry,
property map, I/O, or rendering is allowed.

## Acceptance

All existing constructor, representation, morphology, and parity tests pass;
callers compile unchanged; the module includes local tests.

## Presença lexical Raw — ratificada em 2026-10-14

### Medição anterior à decisão

content-serialization-2026-10-14/{before,raw-controls}/results.json.gz registra HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree, UTC, fontes/binários e pin 9f2b6e8715237cb086899a42873660fe744622e8. RawElem raw.rs:21–25 perde omissão; native_raw markup.rs:78–136 colapsa ausência/false/none. Campos, igualdade e encoders distinguem esses valores no pin. Trata-se de semântica/morfologia, não mecânica Rust.

### Responsabilidade e aceitação

Content::raw conserva assinatura; marca block sempre e lang apenas Some. Testar getters e maps sem tocar render.

Manifesto aprovado b799576490d06bb6fde27c32c78572b63e1279b599d4684629fe4484bc4e0a45, recibo em lab/history/construction/records/00_nucleo/materialization/raw-presenca-2026-10-14/approval-receipt.json. Esta seção completa apenas a presença anteriormente incompleta de Raw e supersede as cláusulas de ausência de máscara. Preservar contratos alheios; sem fase/default/render novo. Validar testes RED→GREEN, suíte, build e linhagem.
