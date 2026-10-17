# L0 — infra/shaper/cache
Hash do Código: f8a8d92b

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/shaped-face-identity.toml sha256:96fe00a0c6071bb88ffc63806b3e42c23b78c398fa26d46866e0f099ab0d4e36
- 00_nucleo/prompts/_nuclei/fonts/shaper-primary-cache.toml sha256:24dff6d00f015a26b2fdc0b84c18aa7debac4a43d5b0380b861cf5ac43403584

**Layer:** L3
**Exclusive consumer:** `03_infra/src/shaper/cache.rs`

## Current evidence

- `03_infra/src/shaper/cache.rs:19` — `ShapeCache`.
- `03_infra/src/shaper/cache.rs:24` — `CachedRun`.
- `03_infra/src/shaper/cache.rs:30` — `new`.
- `03_infra/src/shaper/cache.rs:34` — `key`.
- `03_infra/src/shaper/cache.rs:61` — `FaceCache`.

These are local source anchors, not a new upstream parity measurement.
`00_nucleo/diagnosticos/reuse-key-followup/results.json` supplies discriminating
public shaping controls: adjacent wght coordinates in Ubuntu Sans and Cantarell
produce different fresh advances but alias under four-decimal axis formatting.
Mixed runs then reuse the first result in either order. This local correction
addresses that demonstrated alias; it makes no new vanilla parity claim.
`00_nucleo/diagnosticos/shaper-prompt-audit/results.json` additionally demonstrates tracking aliases caused by decimal rounding and by changing size with an absolute tracking length.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/shaper/cache.rs`.

The language-demand controls in
`00_nucleo/diagnosticos/infra-performance-followup/language-control.json` and
buffer setup at `03_infra/src/shaper/shaping.rs:85` / `:249` identify a missing
semantic input. Once buffers honor language, cached runs must distinguish it;
otherwise same-face/text runs can reuse another language's glyphs and advances.
This is an internal cache representation correction, not a public API change.

## Owned contract

- Own ShapeCache/CachedRun and FaceCache/CachedFace. Run cache is local to a document; face cache is per instance/compilation and may be shared by the metrics facade.
- Preserve exact textual run keys: text, slot, direction, axis variations identified by their exact f32 bit patterns (no decimal rounding), effective tracking increment as an exact i32 in font units, calculated by the shaping owner before lookup effective ssty level, and optional explicit normalized language code. Keep absent language distinct from every explicit code, including explicit English. Values retain glyphs and i32 width in font units.
- Cache faces by slot including absence. Owned Font bytes keep parsed face borrows valid while CachedFace exists; preserve face index zero and existing encapsulated unsafe. No global cache or new eviction is authorized. The tracking key must identify the actual increment applied to cached glyphs; raw decimal lengths must not be rounded into a key. This correction covers explicit language identity; sufficiency of untested script dimensions remains unproved.

## Verification and boundary

Verify that distinct axis f32 bit patterns cannot alias through decimal formatting,
and identical run inputs still reuse their key. For the discriminating Ubuntu Sans
and Cantarell fixtures, mixed AB and BA document results must match isolated fresh
shaping at the corresponding coordinates. Keep these regression tests in this
consumer's test module. Language-sensitive emission and width regressions belong to the test owner `00_nucleo/prompts/infra/shaper/tests.md`; shared AB/BA and repeated runs must match corresponding fresh language controls. This is an internal cache-identity correction under
ADR-0127, with no public API, default mode, compatibility or pipeline phase change.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

## Migração do carrier bidi

Medição anterior à decisão: compiler RED (`context-red.log`, receipt da implementação 2026-10-05) identifica constructor de teste sem campo ratificado. Acrescentar bidi None para fixture sem origem certificada; aceitação mantém as asserções de linguagem existentes.


## Fonte do resultado memoizado — 2026-10-05

Medição anterior à decisão: `00_nucleo/diagnosticos/identidade-fonte-shaped-implementacao-2026-10-05/second-benchmark.json` mostra que a regressão maior foi removida; `shaper/shaping.rs` ainda chama World::font novamente ao construir cada carrier, incluindo hits. A fonte já foi carregada para produzir CachedRun. Fonte pública de TextShaped foi ratificada pelo humano (f4a17206500baf5952f898135944d55f9fca905285eafc995315ead1121dd409).

CachedRun privado conserva clone da Font efetivamente carregada junto dos glyphs/width. O produtor usa essa mesma fonte no carrier de miss e hit, sem consultar World novamente na saída. Keys, escopo/invalidação por documento e FaceCache anteriores ficam preservados; não adicionar unsafe nem ampliar a API pública. Ratificação se aplica ao campo público do carrier; esta alteração é interna em fluxo contínuo. Testes de fallback/hit, FonteSome, controles e revalidação de CLI.


## Reuso da cadeia primária no shaper — 2026-10-10

Medição anterior à decisão: font_selection.rs:16-91 reconstrói a cadeia
primária/classe/MATH/comum a cada chamada; cache.rs:64-87 já retém faces por
slot, mas não a resolução das famílias. resolution-calls-before.json.gz em
00_nucleo/diagnosticos/shaper-primary-reuse-2026-10-10 registra 1.602 chamadas
de primary_candidates e 9.612 de resolve_candidates, entre 9.619 buscas
select_pattern por fixture local-set/no-fill-1600. HEAD, árvore não
commitada, UTC, binário 5e66c09d e argv estão no receipt. Contagens GDB não
são tempos. A seleção e ordem são semântica; repetir consultas é mecânica.
Inferência: reusar a última resolução completa poupa consultas/alocações;
gates de consultas, saída, tempo e RSS podem refutar benefício ou segurança.

Responsabilidade deste owner sob o núcleo shaper-primary-cache pinado acima:
FaceCache possui uma entrada opcional inicialmente vazia de resolução
primária. Um carrier privado guarda FontList, FontVariant, math e candidatos
slots/units-per-em. Manter map por slot, sua política de ausência e o dono
dos bytes de CachedFace inalterados. Sem globals, unsafe novo, eviction da
map de faces ou compartilhamento entre instâncias/Worlds distintos.


## Manutenção local e observação de testes — 2026-10-16

### Medição anterior à decisão

`03_infra/src/shaper/cache.rs` nas linhas 88 contém os avisos registrados em `00_nucleo/diagnosticos/clippy-locais-2026-10-16/before.json.gz`. O snapshot conserva fontes/L0, hashes, HEAD, working tree não commitado, UTC e git diff HEAD --stat. A inspeção dos consumidores e usos precedeu esta decisão; o gate Clippy selecionado deve falhar antes do código. Manutenção de representação/observação Rust, sem nova medição de intenção upstream ou paridade.

### Responsabilidade e aceitação

No FaceCache::get, usar HashMap::entry(slot_idx).or_insert_with para executar World::font e CachedFace::new somente em miss. Guardar também None como resultado negativo; devolver empréstimo da face retida. Não antecipar consulta, duplicá-la, eliminar bytes owned que sustentam o borrow, alterar keys ou ampliar lifetime/escopo/API.

Fluxo contínuo ADR-0127: sem contrato público, default de produto ou fase novos. L0 precede código e resselo restrito. Nenhuma supressão de lint nova. Conferir redução exata do inventário e ausência de avisos novos, gate Clippy RED→GREEN e unfulfilled_lint_expectations como erro; executar suíte workspace, build, formato, Rustdoc e linhagem sob 12 GiB, swap zero e um job. Não criar testes que apenas espelhem derive, Entry ou posicionamento de módulos. O caso de bbox exige observação real conforme especificado; os demais controles existentes mantêm fixtures e expectativas.
