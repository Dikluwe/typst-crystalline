# L0 — infra/shaper/font_selection
Hash do Código: 16aac30f

**Layer:** L3
**Exclusive consumer:** `03_infra/src/shaper/font_selection.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/bidi-visual-identity.toml sha256:3c6209895df993843d15259c7ee95195e7f0dc732ee56cf3cc553257e30298b0
- 00_nucleo/prompts/_nuclei/fonts/fallback-selection.toml sha256:faf6c20021b467fdb2a864625f4f6dd4386b5ef28c137b946e5cb4e4ce073d52
- 00_nucleo/prompts/_nuclei/fonts/shaper-primary-cache.toml sha256:24dff6d00f015a26b2fdc0b84c18aa7debac4a43d5b0380b861cf5ac43403584
```

## Current evidence

- `03_infra/src/shaper/font_selection.rs:16` — `primary_candidates`.
- `03_infra/src/shaper/font_selection.rs:116` — `CandidateSet`.
- `03_infra/src/shaper/font_selection.rs:326` — `intern_fallback_slot`.
- `03_infra/src/shaper/font_selection.rs:363` — `split_run_by_font`.
- `03_infra/src/shaper/font_selection.rs:439` — `is_p1404_format_control`.

These are local source anchors, not a new upstream parity measurement.
`lab/results/construction/diagnostics/shaper-prompt-audit/results.json` demonstrates sparse scoring against the wrong slots and a same-face Latin/Greek run merged across its script boundary.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/shaper/font_selection.rs`.

## Owned contract

- Own FontList resolution, primary/class/MATH fallback, lazy coverage, similarity preference, dense local interning and script/font segmentation. Measurement and emission use the same primary chain.
- Try declared families with the actual TextStyle variant; accept only readable faces. If none resolve, consume the shared class fallback catalog before global fallback. Catalog spelling/order belongs to `00_nucleo/prompts/infra/fallback_fonts.md`.
- In the existing resolved-declared-list branch, append the math chain when style.math or a primary face has MATH; do not invent a second append branch. Family names do not define math content semantics.
- Global candidates come from World::candidates_for_char, are lazily loaded and confirmed by glyph lookup. Preserve longest-covering-run preference, with FontBook similarity using the first resolved primary to break existing preference ties. Never load uncovered catalog entries or intervening slots.
- The local fallback vector is a sparse interner: one dense local position per distinct global slot; result indices use primary.len()+local position. Resolve through that candidate's associated slot, never treat the local index as a global FontBook index or allocate proportional to the greatest slot. Translate candidates to their associated global slots before FontBook scoring, then translate the selected global slot back to its local candidate position without loading intervening faces.
- U+2060 and the explicit bidi set from the pinned bidi-visual-identity nucleus stay in authored content and bypass coverage lookup, joining the current sub-run or first primary when isolated. Shaping/export owners govern placeholder removal and visual Unicode identity; other invisible characters are not generalized.
- Common/Inherited scripts inherit the active script, initially the first non-generic script in the bidi run when one exists. Update the active script when transitioning; a script or font change splits the sub-run even if both scripts use the same face. Preserve original cluster offsets and existing missing-glyph fallback, without a new color-font/render policy.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

## Fallback comum após famílias declaradas — 2026-09-29

Medição: `lab/typst-original/crates/typst-library/src/text/mod.rs:1083-1101` inclui a cadeia comum depois da família declarada; `lab/typst-original/crates/typst-layout/src/inline/shaping.rs:904-948` tenta as famílias antes do fallback global. Em `correcao-rtl-restante-2026-09-29/after-leading`, os seis probes hebraicos mantêm 7,68 pt de divergência; `pdffonts` identifica Libertinus Serif no vanilla e Noto Sans Hebrew no cristalino. O primeiro cobre o texto hebraico e o segundo aparece pelo fallback global. A hipótese de causa é a omissão da cadeia comum no ramo em que a primária resolve; refutação: fonte ou posição ainda diferente com a cadeia presente. Isto é paridade tipográfica observável, não exigência de bytes iguais.

Após as famílias declaradas legíveis, oferecer a cadeia comum na ordem do catálogo, sem duplicar slots. A primária declarada mantém prioridade. A cadeia matemática continua a preceder a comum em contexto matemático; a recuperação por classe quando nenhuma declarada resolve conserva sua política. A busca global por cobertura e similaridade vem depois das cadeias explícitas. Medição e emissão compartilham essa ordem.


## Controles bidi e cobertura (2026-10-06)

Medição anterior à decisão: first-raster.json em
`lab/results/construction/diagnostics/controles-bidi-pdf-2026-10-06/` registra notdef sobre
A em control-ltr-lri; second-verification.json.gz registra troca de face e
2.7662pt de divergência em control-ltr-pdi. `font_selection.rs:452-454`
reconhece somente três controles; `:396-416` consulta cobertura para os
restantes. Hipótese refutável se o mesmo problema persistir após manter
controles no sub-run primário sem consulta de cobertura. Correção interna
ADR-0127: ampliar apenas o conjunto bidi explícito do núcleo, preservando
U+2060 e política dos demais caracteres. Sem API/fase/unsafe novo.


## Reuso da cadeia primária no shaper — 2026-10-10

Medição anterior à decisão: font_selection.rs:16-91 reconstrói a cadeia
primária/classe/MATH/comum a cada chamada; cache.rs:64-87 já retém faces por
slot, mas não a resolução das famílias. resolution-calls-before.json.gz em
lab/results/construction/diagnostics/shaper-primary-reuse-2026-10-10 registra 1.602 chamadas
de primary_candidates e 9.612 de resolve_candidates, entre 9.619 buscas
select_pattern por fixture local-set/no-fill-1600. HEAD, árvore não
commitada, UTC, binário 5e66c09d e argv estão no receipt. Contagens GDB não
são tempos. A seleção e ordem são semântica; repetir consultas é mecânica.
Inferência: reusar a última resolução completa poupa consultas/alocações;
gates de consultas, saída, tempo e RSS podem refutar benefício ou segurança.

Responsabilidade deste owner sob o núcleo shaper-primary-cache pinado acima:
primary_candidates consulta a entrada com igualdade completa antes
de reconstruir a cadeia. No miss executa o algoritmo vigente integralmente;
publica resultado somente não vazio e sem qualquer entrada None na map de
faces. O filtro conservador não altera o slot-negative cache anterior. A
última entrada válida pode sobreviver a um pedido não publicável. Retornar
Vec independente dos candidatos; deixar cobertura global/script/instanciamento
e chaves de run inalterados. Testar RED de consultas repetidas e refutadores
de ordem/regex/variantes/math, instância nova e falha de face, com controles
frescos. Mudança interna em fluxo contínuo ADR-0127; API/default/fase iguais.


## Expressões equivalentes e CI estrito — 2026-10-14

### Medição anterior à decisão

`03_infra/src/shaper/font_selection.rs` nas linhas [66, 131, 302, 350] contém as expressões apontadas pelo inventário
Clippy. `lab/results/construction/diagnostics/ci-simplificacoes-2026-10-14/before.json.gz`
conserva fonte/owner, spans, HEAD, working tree não commitado, UTC e diffstat;
red.log registra os lints selecionados como erros antes de alterar código.
Os tipos e usos foram conferidos no consumer e no toolchain 1.92.0 pinado.
Esta é manutenção da expressão Rust; não é medição nova de paridade upstream.

### Responsabilidade e aceitação

Nos predicados de MATH e cobertura de slot opcional, usar is_some_and com ausência false e a mesma consulta apenas quando há Some. Retirar casts u16→u16 de units_per_em, preservando max(1). Não modificar ordem/identidade de candidatos, fallback, carregamento lazy, subdivisão por script, política de cache ou heurística de cobertura.

Correção interna em fluxo contínuo ADR-0127, sem API pública, default, fase,
unsafe, dependência ou supressão de lint novos. L0 precede código e resselo
fica restrito ao owner. Usar os controles funcionais existentes; não alterar
fixtures, expectativas, tolerâncias ou registrar testes que apenas espelhem
esta simplificação. Os lints selecionados devem passar como erros, seguidos
de suíte completa, build, all-targets, rustdoc, fmt, linhagem e inventário total.


## Simplificações locais de Clippy — 2026-10-15

### Medição anterior à decisão

`03_infra/src/shaper/font_selection.rs:129` empresta novamente variant, já recebido por referência; `:660` aloca um Vec de casos fixos utilizado somente como sequência no teste.
`lab/results/construction/diagnostics/avisos-clippy-tekt-2026-10-15/before.json.gz`
conserva fontes, owners, HEAD, working tree não commitado, UTC e diffstat;
selected-red.json identifica os diagnósticos do compilador antes da edição.
São expressões Rust equivalentes, sem nova medição de paridade upstream.

### Responsabilidade e aceitação

Repassar variant diretamente a select_pattern. No controle changed_families_variants_and_math_match_fresh_in_both_orders, armazenar os mesmos casos em array de tamanho fixo, preservando todas as ordens, iterações e expectativas. Não alterar resolução, cobertura, cache ou segmentação.

Manutenção interna em fluxo contínuo ADR-0127: conservar contratos públicos,
defaults e fases. Não adicionar supressões nem testes que espelhem sintaxe.
O gate dos avisos selecionados deve passar a GREEN; validar a suíte existente,
build, Clippy all-targets, formato, documentação e linhagem com resselo restrito.
