# L0 — infra/font_metrics/math
Hash do Código: cdc47a90

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/ssty-math-size.toml sha256:661b7b74b7601c1b4fb11dda93fea95ee8ebcd62f9e3605507e5ce1373334808

**Layer:** L3
**Exclusive consumer:** `03_infra/src/font_metrics/math.rs`

## Current evidence

- `03_infra/src/font_metrics/math.rs:17` — `extract_variants`.
- `03_infra/src/font_metrics/math.rs:52` — `extract_assembly`.
- `03_infra/src/font_metrics/math.rs:101` — `extract_variants_horizontal`.
- `03_infra/src/font_metrics/math.rs:159` — `extract_assembly_horizontal`.
- `03_infra/src/font_metrics/math.rs:207` — `build_math_glyph_reverse_map`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/font_metrics/math.rs`.

## Owned contract

- Own MATH constants/variants/assemblies/kern extraction, reverse cmap and canonical ssty eligibility/substitution helpers. Translate parser data into existing neutral L1 types only.
- Keep every existing MATH field in design units and percentages divided by 100. Missing tables fall back; a present zero value is not missing.
- Variant stretch advance and hmtx horizontal advance remain distinct. Attachment uses the table value or (hmtx+italic-correction)/2. Assemblies preserve connectors/extenders, horizontal advances and minimum overlap.
- Kern tables preserve four quadrants with n heights and n+1 values, including the final unbounded None. Reverse cmap retains first base for a shared glyph, including large operators and existing base/underscore fallback.
- ssty eligibility is one character or an ASCII numeric sequence with at least one digit and at most one decimal point. Only math Script/ScriptScript outside TextItem requests level 1/2; missing GSUB leaves the base unchanged. No family-name special case or duplicated shaper predicate.

## Verification and boundary

Run `cargo test -p typst-infra --lib font_metrics:: -- --test-threads=1` when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

## Evitar reverse cmap descartada sem MATH variants — 2026-10-10

### Medição anterior à decisão

`00_nucleo/diagnosticos/cpu-rebaseline-2026-10-10/{profiles-normalized,font-selection-probe,font-resources}.json.gz`
registra fonte, HEAD/árvore/UTC/binários e seleção: o corpus de 800 pesos usa
três faces sem MATH, mas gera 800 recursos distintos. O perfil inclui 6070
amostras com reverse-map; presença inclusiva não é tempo exclusivo.
`math.rs:239-254` constrói cmap_reverse antes de consultar variants/assemblies;
os quatro extractors em :17-202 retornam vazio quando MATH ou variants falta.
O resultado intermediário é descartado e o mapa retornado é vazio.
`00_nucleo/diagnosticos/math-reverse-map-skip-2026-10-10/{before,allocation-equal-red}.json.gz`
registra estado corrente depois da correção Cell: HEAD
06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado,
release 71e968c20d5ba32c652b2b66c4fd444c672158ba9bb27b0272f59cfaa5530c78.
Baseline variants-1600 solicita 1117479 alocações glibc. Evidência local para
remover trabalho privado; não inferir equivalência do corpus com vanilla,
pois a geometria/identidade dos recursos ainda difere.

### Decisão interna e aceitação

Retornar HashMap vazio antes de percorrer cmap quando a face parseada não
possui MATH variants. Sem MATH ou com MATH sem variants, todos os extractors
continuam vazios. Com variants presente, conservar integralmente o algoritmo,
ordem de todas as subtables, first-wins, Unicode real/fallback e assemblies
verticais/horizontais/operadores grandes. Não reduzir pelo nome/família,
número de glifos, peso ou igualdade de bytes de recursos. Não alterar
identidade de FontKey, binding, axes, subset, fallback ou seleção de fontes.
É saída antecipada privada, ADR-0127 contínuo, sem cache global ou API nova.

Testes primeiro: faces embutidas sem MATH retornam vazio; face com MATH e
variants ausente retorna vazio; MATH com variants mantém assembly, variantes
e ToUnicode real existente. Esses controles semânticos já passam no baseline.
RED de recurso pré-código: variants-1600 deve solicitar no máximo 1116479
alocações (ao menos 1000 abaixo do baseline); controles families/no-fill não
aumentam. GREEN após código, e qualificação antigo/candidato de texto,
páginas, caixas, raster e warnings incluindo corpus matemático e variantes.
Medir tempo/RSS pareado nativo; retirar se saída mudar ou ganho não se
confirmar. Executar testes font_metrics, build e lint com resselo restrito.


## V16 — projeção nominal de MathSize para ssty (2026-10-17)

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
00_nucleo/diagnosticos/tekt-v16-ssty-2026-10-17/before.json.gz conserva UTC,
status, git diff HEAD --stat, fontes/L0 e 2128 hashes conferidos do fechamento
anterior. O SARIF em 00_nucleo/diagnosticos/import-syntax-2026-10-17/validation-architecture.log
registra 117 V16, incluindo font_metrics/math.rs:337 e shaper/shaping.rs:28.
MathSize em 01_core/src/entities/layout_types.rs:147 possui Display, Text,
Script e ScriptScript. math.rs:330–338 e shaping.rs:21–30 retornam Some(1/2)
para Script/ScriptScript; o wildcard devolve None para os dois tamanhos atuais
restantes e ocultaria uma variante futura. Os L0 vigentes já exigem esses
níveis e gates de math/TextItem/elegibilidade; não há decisão linguística nova.
before-probe-result.json conserva fontes/hashes, rustc/argv/UTC e 224 controles
com resultados atuais. A sonda extrai funções e declaração real de MathSize,
modelando somente os três campos de TextStyle lidos; no mutante descartável,
a variante TektSstyFuture é aceita em ambos os matches (zero E0004). O RED é
essa falta de proteção estrutural; controles funcionais atuais passam.

É mecânica de manutenção Rust: enum crescente precisa de decisão explícita;
não é medição nova de paridade nem de velocidade de leitura por IA. Os níveis
ssty continuam o contrato de linguagem/render anterior. A inferência de que o
reparo conserva os quatro casos atuais é refutada por qualquer controle,
replay textual ou regressão divergente. Não substituir uma decisão semântica
por enumeração cosmética: Display/Text são ausência intencional de pedido,
justificada no núcleo pinado; Script/ScriptScript mantêm níveis distintos.

### Responsabilidade exclusiva

Em ssty_level_of, conservar assinatura e guard inicial e substituir somente o wildcard por MathSize::Display | MathSize::Text => None. Não alterar elegibilidade, substituição GSUB, reverse cmap ou demais funções.

A política compartilhada está em 00_nucleo/prompts/_nuclei/fonts/ssty-math-size.toml,
pinada neste owner. Fluxo contínuo ADR-0127: nenhuma mudança de contrato público,
comportamento padrão ou fase; campos e enum público permanecem byte a byte.

### Aceitação

Primeiro registrar o RED estrutural e os controles antes do código. Depois,
a mesma sonda com variante futura precisa produzir exatamente dois E0004 nos
matches auditados; controles 224/224 idênticos. O modelo parcial de TextStyle
não prova shaping integral: conferir também os testes reais existentes de
font_metrics e integração ssty/TextItem, a suíte workspace e os gates finais.
Não adicionar testes que apenas reescrevam o match; o guard de crescimento é
a nova propriedade verificada. Replay do código permite somente o reparo
nominal prescrito e o hash do header. SARIF: retirar precisamente os dois V16,
adicionar duas notas V19 dos grupos Display/Text com motivo e cobertura dos
dois tamanhos; todos os outros diagnósticos devem permanecer por fonte, regra,
severidade e mensagem. Sem V20 novo, supressão ou alteração de configuração.
Grafo V15/V26 antes de resselo restrito aos dois pares; build, fmt, Clippy
all-targets -D warnings/-D unfulfilled_lint_expectations, Rustdoc estrito,
V1/V5/V15/V26, reverse lineage, suíte completa, diff --check e integridade.
Comandos pesados sequenciais sob MemoryMax12GiB/High10GiB/swap0/jobs1; sem
commit/push/merge/release. O objetivo global continua além deste lote.
