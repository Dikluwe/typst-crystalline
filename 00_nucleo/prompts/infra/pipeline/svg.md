# L0 — infra/pipeline/svg
Hash do Código: f1a5150c

**Layer:** L3
**Exclusive consumer:** `03_infra/src/pipeline/svg.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/export/format-option-precedence.toml sha256:68c5c10f124c2c098879318df16c59f6c1477e9172cfa334b603f45ec698ae11
- 00_nucleo/prompts/_nuclei/export/svg-destination-context.toml sha256:13cad5ab1322bad4c569eec2aaf544452530cb972c3421130d0b9cb127170ef1
- 00_nucleo/prompts/_nuclei/export/svg-glyph-font-context.toml sha256:4d185c303f0262799e475ff76459118a64fdbd406dfd704d95a836b4b254985c
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:08c39c6b9cf6fdc571564646d12d71f9b12f2a5d2c029ca90f9253c39c28bb95
```

## Current evidence

- `03_infra/src/pipeline/svg.rs:12` — `compile_to_svg_string`.
- `03_infra/src/pipeline/svg.rs:21` — `compile_to_svg_string_full_error`.
- `03_infra/src/pipeline/svg.rs:33` — `compile_to_svg_string_with_timings_full_error`.
- `03_infra/src/pipeline/svg.rs:47` — `compile_to_svg_string_with_timings_full_error_and_features`.
- `03_infra/src/pipeline/svg.rs:59` — `compile_to_svg_string_with_timings_full_error_and_features_and_pretty`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/pipeline/svg.rs`.

## Owned contract

- Own SVG facades, resolved pretty, selected-page destination context and glyph-font associations. Compile through the common paged path; export the first page with existing empty-page fallback.
- Resolve pretty as explicit caller then document.svg.pretty then its default. Decision-free wrappers pass None and boolean compatibility wrappers pass Some. The exporter receives only final SvgOptions; do not teach it CLI/document precedence.
- Derive destinations only from labels on the selected page with corresponding extracted positions. Do not invent missing positions or move maps into Page; cross-page unresolved links remain Unknown without dangling fragments. Exporter owns ID spelling.
- Associate each normalized GlyphFontRequest with the complete FontKey resolved through FallbackFontMetrics. Distinguish base character, requested family, variant, variations and relevant math state independently of slot/address/traversal occurrence. Conflicting results invalidate an association; missing entries remain Unknown, never default slot zero.
- Collect associations across background/body/foreground and Group/Link/Semantic. Without fonts do not fabricate associations, reread sources or repeat layout. Inline `html.frame` SVG uses the HTML owner's resolved option, not format.svg.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

## Identidade textual — materialização aprovada

### Medição anterior à decisão

As sondas bilaterais e o estado exato em
`lab/results/construction/diagnostics/identidade-hifen-discricionario-2026-09-28/initial.json`
e `before/` medem U+002D versus SHY com geometria coincidente em oito controles.
A inspeção do percurso FrameItem deste consumer está arquivada em approved-before/.
O dono confirmou o contrato em `00_nucleo/prompts/core/entities/layout_types.md`.

### Responsabilidade exclusiva

Tratar TextReplacement como envelope visual transparente nos percursos deste owner; descender nos filhos preservando text, ordem, coordenadas e o cálculo existente, sem novo algoritmo ou avanço.

O Núcleo text-replacement define apenas invariantes compartilhados; este prompt
possui somente o consumer declarado. Aceitação: texto/caixas, testes causais,
transporte recursivo e controle de hífen literal; não bytes PDF ou nova seleção.


## Preparação de bindings de exportação — 2026-10-05

Medição: `lab/results/construction/diagnostics/identidade-fonte-shaped-2026-10-05/probe.txt`. Consumir helper privado resolve_export_fonts: fontes originais vinculadas, com gate de instanciação existente. Instanciação fica no consumidor exportador, na mesma fase de render/export, preservando diagnósticos/API/seleção de páginas. O nome antigo prometia bytes já instanciados e não descreve este contrato.


## Manutenção local e observação de testes — 2026-10-16

### Medição anterior à decisão

`03_infra/src/pipeline/svg.rs` nas linhas 160 contém os avisos registrados em `lab/results/construction/diagnostics/clippy-locais-2026-10-16/before.json.gz`. O snapshot conserva fontes/L0, hashes, HEAD, working tree não commitado, UTC e git diff HEAD --stat. A inspeção dos consumidores e usos precedeu esta decisão; o gate Clippy selecionado deve falhar antes do código. Manutenção de representação/observação Rust, sem nova medição de intenção upstream ou paridade.

### Responsabilidade e aceitação

Mover somente o módulo #[cfg(test)] p1462_tests para o fim do consumer. Manter nomes, corpos, atributos, import super, APIs e ordem funcional de todos os helpers produtivos, sem extração/dispatch novo.

Fluxo contínuo ADR-0127: sem contrato público, default de produto ou fase novos. L0 precede código e resselo restrito. Nenhuma supressão de lint nova. Conferir redução exata do inventário e ausência de avisos novos, gate Clippy RED→GREEN e unfulfilled_lint_expectations como erro; executar suíte workspace, build, formato, Rustdoc e linhagem sob 12 GiB, swap zero e um job. Não criar testes que apenas espelhem derive, Entry ou posicionamento de módulos. O caso de bbox exige observação real conforme especificado; os demais controles existentes mantêm fixtures e expectativas.


## FrameItem — exaustividade de decisões existentes, V16, 2026-10-16

### Medição anterior à decisão

F066: 03_infra/src/pipeline/svg.rs:197 (collect). Os spans AST, patterns/guards/corpos e complements foram conferidos antes do código. `lab/results/construction/diagnostics/tekt-frame-items-2026-10-16/before.json.gz` conserva HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, fontes/L0/configuração, hashes, status e git diff HEAD --stat. `classification.json` registra os 70 casos e seus complements em 26 owners, usando a árvore do parser pinado do CI. Cláusulas pertinentes do L0 foram lidas junto da fonte; não há proposta de nova arquitetura ou interpretação de intenção upstream. Padrões e exaustividade Rust são mecânica; semântica/morfologia, geometria, efeitos e diagnósticos do produto devem permanecer.

### Prescrição exclusiva desta unidade

- **F066 — `collect`**: Trocar somente o padrão '_' pelo grupo nominal `TextSeparator`, `Text`, `TextShaped`, `Line`, `Image`, `Shape`. Usar o mesmo prefixo Rust em escopo (`FrameItem`). Conservar integralmente o corpo atual do fallback, os braços anteriores, guards, transformações, chamadas, returns e ordem.

Manter callbacks, efeitos, metadados, clipping, fontes, positions, tinta e largura conforme os owners vigentes. Esta rodada não amplia travessias, não transforma wrappers em folhas nem usa text/ActualText como substituto da geometria. Helpers de teste mantêm fixtures, esperados, tolerâncias, contagens e falha Unknown; não aproveitar o saneamento para aceitar ausência de observação. Grupos nominais compartilham o mesmo corpo anterior: notas V19 de alternativas continuam visíveis, sem confundir cobertura de um braço com todas as alternativas.

### Gate e aceitação

Manutenção interna ADR-0127 em fluxo contínuo: nenhum enum/campo novo, método de trait/assinatura pública, default, dependência, compatibilidade ou fase nova. Não materializar propostas históricas pendentes por este adendo. L0 primeiro e resselo restrito em ambas as direções. Não adicionar supressões, alterar limiar ou reescrever match como if-let para esconder V16.

Antes do código, executar o controle novo de FrameVisitor e a suíte workspace nos mesmos esperados, e confirmar RED dos 70 avisos da família por leitura AST. Após código, zero V16 dessa família nos locais abrangidos e nenhum aviso novo; manter V16 externos como pendências. Nos matches do core produtivo alcançados por cargo check -p typst-core, usar cópia descartável com uma variante sentinela de FrameItem para medir ausência de E0004 nos locais antes e presença depois. A sonda cobre somente os locais efetivamente compilados: não extrapolar aos helpers cfg(test) ou L3 que a dependência impede de alcançar. Para todo o lote, conferir padrões/guards/corpos por replay, enum e assinaturas inalterados, build e testes reais. Não substituir corpos por stubs.

Revalidar suíte workspace completa, cargo build, Clippy workspace/all-targets com -D warnings e -D unfulfilled_lint_expectations, formato, Rustdoc -D warnings com quatro índices HTML, linhagem V1/V5/V15/V26 estrita/reversa, SARIF e diff --check. Fontes/L0/configuração ficam congelados durante comandos. Execuções pesadas sequenciais com MemoryMax=12 GiB, MemorySwapMax=0 e CARGO_BUILD_JOBS=1. Não apresentar saneamento estrutural como nova paridade global, ganho de memória ou benefício medido para IA.
