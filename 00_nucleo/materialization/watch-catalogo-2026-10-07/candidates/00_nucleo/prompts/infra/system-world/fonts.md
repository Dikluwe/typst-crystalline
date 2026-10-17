# L0 — infra/system-world/fonts
Hash do Código: d9c08969

**Layer:** L3
**Exclusive consumer:** `03_infra/src/world/fonts.rs`

## Current evidence

- `03_infra/src/world/fonts.rs:23` — `with_fonts`.
- `03_infra/src/world/fonts.rs:34` — `with_system_fonts`.
- `03_infra/src/world/fonts.rs:46` — `with_embedded_fonts`.
- `03_infra/src/world/fonts.rs:65` — `with_fonts_and_system`.
- `03_infra/src/world/fonts.rs:121` — `preload_coverage_if_needed`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/world/fonts.rs`.

## Owned contract

- Own font builders, lazy exact coverage and conservative preloading. Invalid FontInfo entries are dropped while book/slot indices remain aligned; absent system fonts are nonfatal.
- Preserve builder order: embedded text before embedded math/code; combined discovery uses embedded text, system, embedded math/code, project sources.
- System and combined builders capture discovery policy and owned project paths without discovering fonts. The first book, font or candidate request initializes the complete paired catalog once, safely under concurrent access, and retains its Arc for the world lifetime. An explicitly installed validated font session may also retain its generation as specified below. Explicit slot and embedded-only builders retain their existing acquisition behavior. Replacing builder configuration replaces its catalog and clears coverage belonging to the previous indices.
- Deferred discovery still obtains exact system coverage through the fontdb owner while temporary face bytes are available. Candidate queries must not map all system FontSlots to reconstruct coverage; preserve the P1410 resource bound.
- Prefer nonempty book coverage without loading font bytes. Only missing coverage triggers lazy cmap parsing/cache by slot; invalid faces yield empty coverage. Preserve exact codepoint filtering, candidate order and locks.
- The existing explicit preload method remains conservative for Text/MathText/RawTrimmed through embedded coverage union and first-uncovered candidate demand; calling it may initialize the catalog. Normal CLI compilation leaves activation to actual font demand. Post-evaluation content follows the same demand path.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/watch-font-generation.toml sha256:391cd1604976262b1b6e019b802ed72e78782260d2788bb04999297559605509

## Medição anterior à decisão de retenção (candidato não ratificado)

00_nucleo/diagnosticos/watch-retencao-2026-10-07.md registra HEAD/árvore/binários/UTC,
edições reais, recuperação e contagem dinâmica de aquisição por ciclo.
00_nucleo/diagnosticos/watch-validacao-2026-10-07/measurement.json mede validação por
descoberta versus aquisição completa no mesmo estado; controls.json testa adição,
remoção, substituição, mtime restaurado, links e configuração. São custos isolados,
não ganho de CLI ou prova de intenção upstream. FontBook/slots/cobertura devem
preservar seleção, conteúdo, diagnóstico e render na linguagem; duração/Arc são
mecânica. Inferimos benefício possível, refutado por custo, stale resources,
regressão ou retenção ilimitada. Fixtures/proveniência/limites constam do relatório.

### Decisão proposta sob ADR-0127 §2.1/§2.2

Aguardar ratificação do pacote 00_nucleo/materialization/watch-catalogo-2026-10-07/manifest.json
antes de instalar este candidato ou escrever código produtivo. Cada owner continua 1:1.

## Builder público da sessão (única assinatura nova deste owner)

```rust
pub fn with_font_catalog_session(
    self,
    session: Arc<FontCatalogSession>,
    font_paths: &[PathBuf],
) -> Self;
```

Capture Arc e cópia dos caminhos sem I/O ou descoberta. A primeira demanda de
catálogo delega à sessão; substituição de builder limpa coverage_cache e substitui
o OnceLock/closure, tal como builders atuais. Não compartilhar coverage_cache:
a cobertura precomputada em FontBook já pertence à geração; a cobertura ausente
é reconstruída no mapa fresco deste World. Builders antigos continuam independentes.

Adaptar o armazenamento interno para Arc sem alterar o retorno privado de
font_catalog(), nem a associação índice/book. A composição permanece texto
embutido → sistema → math/code embutido → projeto. Extraia helpers pub(super)
para a aquisição combinada fresca e para montar catálogo a partir da descoberta
preparada pelo child generation, consumindo os slots de projeto já descobertos.
São acesso entre descendentes L3, não API pública. Não duplicar os algoritmos de
metadata, cobertura ou seleção. O owner fontdb materializa o book do Database;
o owner fonts existente materializa o book dos slots de projeto. Não mover fases.
Testes inline do builder verificam laziness, substituição e independência de
sources, sem instrumentação pública.
