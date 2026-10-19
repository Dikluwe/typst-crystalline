# L0 — Image byte metadata adapter
Hash do Código: c591db42

**Layer:** L3
**Exclusive consumer:** `03_infra/src/image_sizer.rs`

## Current evidence

- `03_infra/src/image_sizer.rs` — `ImageSizeImageSizer`, dimensions/DPI/orientation adapters and TIFF metadata readers.
- `01_core/src/compiler/layout/image.rs` — the L1 sizing consumer of ImageSizer.
- `03_infra/src/export/images.rs` — a separate PDF payload facade, outside this prompt's ownership.

These are current local source anchors, not a new upstream parity measurement.

## Contract

Own metadata inspection of already supplied image bytes. Preserve the existing ImageSizer trait implementation and Clone/Copy adapter; do not read paths, load World resources, decode pixels, rotate/re-encode bytes or expose external image-library types to L1. `imagesize::blob_size` yields dimensions as `Option<(u32,u32)>`; invalid/unsupported bytes return `None`.

For PNG DPI, prefer EXIF then pHYs; for JPEG, prefer EXIF then JFIF; other formats return `None`. pHYs uses 0.0254 for pixels/metre and the existing 2.54 conversion for unit 2. JFIF supports unit 1 with positive horizontal density. JPEG segment lengths must include the two length bytes: values below two or beyond the remaining buffer return None without slicing an inverted range, in both DPI and orientation scans. TIFF XResolution supports Rational/SRational with nonzero positive denominator; preserve current parser bounds and conversion. Do not assert a uniform positivity filter: zero PNG/EXIF density can currently be `Some(0.0)`, unlike zero JFIF density. This discrepancy from the old upstream citation remains pending in `lab/results/construction/diagnostics/sanitization-infra-wiring-history.md`.

Parse TIFF II/MM byte order, marker 42 and IFD0 from the existing PNG/JPEG EXIF containers. Orientation is tag 0x0112, count one, using the accepted BYTE/SHORT/LONG inline forms. Return only values 1 through 8; malformed/absent/unsupported values return `None`. Preserve bounds checking and no new EXIF dependency.

The existing L1 image layout owner interprets absent DPI/default sizing and orientation transforms. `00_nucleo/prompts/infra/world/paths.md` owns World byte acquisition; `00_nucleo/prompts/infra/pipeline/paged.md` owns the pre-export validation call; the excluded `00_nucleo/prompts/infra/export/images.md` and its children own validation/decoding/PDF payloads. This adapter does not duplicate their contracts.

## Verification boundary

Existing byte fixtures control dimensions, density precedence and legal/invalid orientation. Optional `/tmp/p773-dpi300.png` and `.jpg` probes are environmental legacy inputs, not required persistent fixtures or proof when absent. No replacement is assumed.


## Tipos explícitos e leitura local — type_complexity (2026-10-15)

### Medição anterior à decisão

03_infra/src/image_sizer.rs:268 (C093); 03_infra/src/image_sizer.rs:340 (C094).
O snapshot before.json.gz em lab/results/construction/diagnostics/type-complexity-2026-10-15
conserva HEAD, working tree não commitado, UTC, diffstat, fontes/owners e hashes.
O gate red-clippy.log falha com este lint promovido a erro antes da edição.
A inspeção distingue tuplas posicionais, repetição do alias FontKey e contratos
estáticos já explícitos. É manutenção de representação Rust; não medição de
benefício da IA, bug funcional ou paridade nova.

### Responsabilidade e aceitação

Nomear os dois leitores estáticos de TIFF em TiffReaders privado com read_u16 e read_u32, ambos fn-pointers existentes. Um helper privado tiff_readers escolhe II/MM na mesma ordem e retorna None para outro header; retirar o boolean _le sem uso. Os parsers mantêm seus testes de comprimento/magic antes de IFD, chamando os mesmos leitores e helpers nas mesmas fatias. Sem novo parser, bounds, dependência, precisão ou política de DPI/orientação.

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
