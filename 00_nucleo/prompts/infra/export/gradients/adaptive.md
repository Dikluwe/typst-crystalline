# L0 — infra/export/gradients/adaptive
Hash do Código: 462b00f9

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/gradients/adaptive.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0126, ADR-0127, ADR-0128, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/gradients/adaptive.rs:42` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## PDF sample-count selection

Own perceptual_distance_in_space, adaptive_n_for_stops and the separate SVG adaptive
adapter. PDF distance is Euclidean Oklab in f32; the space parameter is reserved
and does not select a native metric. For fewer than two stops return 16.
Otherwise use max adjacent-pair distance: below 0.05 → 16, below 0.30 → 32,
otherwise 64. Keep the existing PDF applicability selected by the resource caller;
this helper does not choose a shading strategy.

## SVG adaptive stops

Own svg_adaptive_stops, effective f64 offsets and private precise sampling/error
helpers. Linear/Radial preserve original stop order, alpha and coincident offsets.
Subdivide only positive-length intervals when anti_alias is enabled, comparing
the exact midpoint to encoded-sRGB interpolation between endpoints after
premultiplication. Threshold is 0.001 and maximum subdivision factor per interval
64; terminate deterministically. The Conic path returns original colors/effective
offsets without applying this Linear/Radial bisection.

For Oklab/Oklch, clamp encoded-sRGB float channels before endpoint midpoint mixing,
and clamp exact/approximate channels before alpha premultiplication/error comparison.
Do not clamp native-space sampling prematurely or defer this boundary to u8.
LinearRgb and other paths retain their current error conversion; no global Color
gamut policy is introduced.

The precise sampler covers Oklab, LinearRgb, Oklch, Hsl and Hsv: keep t/offsets
and local parameter in f64 until materializing separate f32 weights 1-local/local.
Each component uses (w0*c0+w1*c1)/(w0+w1). Apply short hue routing at Oklch index 2
and Hsl/Hsv index 0, retaining native-space alpha; normalize the resulting hue.
Other spaces keep the existing domain sample path. Preserve duplicate-stop side
selection, endpoints and interpolation of unspecified offsets. Do not reshape
the domain Gradient::sample API.

## Verification and scope

Preserve the adjudicated local budget: maximum encoded-sRGB premultiplied error
<= max(0.001, measured vanilla error)+1e-6; p95 <= measured vanilla p95+1e-6;
alpha maximum/p95 <= the measured vanilla envelope+1e-6. Coincident points are
adjudicated by declared order; continuous comparison starts at epsilon to the right.
Use a pinned, independently measured oracle, not automatic acceptance after u8
saturation, larger budgets/caps, stop counts or byte equality. Historical upstream
evidence and fixture provenance live in the export sanitization diagnostic; those
measurements do not establish parity against a later pin without remeasurement.

Supported SVG paint pairs are owned by
`00_nucleo/prompts/infra/export/svg/paints.md`; generating samples does not promote
Luma/CMYK or an unvalidated pair. PDF count selection remains independent.

## Cálculos numéricos dos exportadores — 2026-10-13

### Medição anterior à decisão

`03_infra/src/export/gradients/adaptive.rs` nas linhas [390] usa métodos apontados pelo Clippy. `clippy.toml` exige alternativas sem dependência da matemática da plataforma; libm =0.2.11 já está pinado no core/Cargo.lock. Os fontes da biblioteca, consumer, owner, manifest/lock, spans, HEAD, working tree não commitado, UTC, status e diffstat estão em `00_nucleo/diagnosticos/gradientes-numericos-ci-2026-10-13/before.json.gz`.

`before-renders.json` registra 24 fontes renderizadas em PDF/SVG por Tekt e pelo vanilla pinado 9f2b6e8715237cb086899a42873660fe744622e8, com hashes dos binários e comandos. `before-comparison.json` mede o canvas inteiro sem máscaras ajustadas pelo erro. Existem divergências anteriores; o corpus não é prova de paridade integral. A primeira tentativa de fixture usava hsv fora do namespace color, foi rejeitada pelo parser antes de qualquer mudança produtiva e está preservada em initial-fixture-error.log; cases.json contém as entradas válidas.

### Decisão e aceitação

No helper de erro usado exclusivamente pelos testes, substituir powi(2) por uma variável local delta e delta*delta, sem alterar o somatório, sqrt, premultiplicação ou budgets. Os algoritmos produtivos de amostragem não mudam.

Dependência direta interna de L3 fixada em =0.2.11 no manifest, reutilizando a versão já no lock. Não atualizar versões de pacotes nem adicionar I/O. Mudança interna em fluxo contínuo ADR-0127, sem contrato público, default, fase ou estratégia novos. L0 precede código e resselo é limitado aos owners envolvidos.

Validar as fórmulas com a suíte existente, exportação dos mesmos 24 documentos antes/depois e leitura raster/estrutural de PDF/SVG. Comparar a preservação local separadamente das divergências contra o pin vanilla; nenhuma diferença nova de render é aceita sem análise. Não alargar tolerâncias/limites, mudar fixtures ou supressões para esconder uma falha. Compilar todos os targets, executar a suíte infra, verificar build, Clippy, Rustdoc, formato e linhagem. Contagens e identidade de arquivos não certificam a paridade da linguagem.


## Expressões equivalentes e CI estrito — 2026-10-14

### Medição anterior à decisão

`03_infra/src/export/gradients/adaptive.rs` nas linhas [159] contém as expressões apontadas pelo inventário
Clippy. `00_nucleo/diagnosticos/ci-simplificacoes-2026-10-14/before.json.gz`
conserva fonte/owner, spans, HEAD, working tree não commitado, UTC e diffstat;
red.log registra os lints selecionados como erros antes de alterar código.
Os tipos e usos foram conferidos no consumer e no toolchain 1.92.0 pinado.
Esta é manutenção da expressão Rust; não é medição nova de paridade upstream.

### Responsabilidade e aceitação

O next_t da subdivisão SVG já é f64. Passá-lo diretamente a Ratio, sem From identidade. Preservar exatamente precisão, operações, ordem, budgets, offsets coincidentes, alpha e seleção de espaço; não mudar a interpolação ou o limite de subdivisões.

Correção interna em fluxo contínuo ADR-0127, sem API pública, default, fase,
unsafe, dependência ou supressão de lint novos. L0 precede código e resselo
fica restrito ao owner. Usar os controles funcionais existentes; não alterar
fixtures, expectativas, tolerâncias ou registrar testes que apenas espelhem
esta simplificação. Os lints selecionados devem passar como erros, seguidos
de suíte completa, build, all-targets, rustdoc, fmt, linhagem e inventário total.
