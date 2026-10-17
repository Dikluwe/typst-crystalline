# L0 — Bounded selective acquisition of font catalog metadata
Hash do Código: d87c5534

**Layer:** L3
**Exclusive consumer:** `03_infra/src/fontdb/selective.rs`
**State:** ratified canonical owner.

## Measurement before decision

`00_nucleo/diagnosticos/catalogo-seletivo-2026-10-05.md` records the rejected full-post variant and measured prefix experiment (metadata38.10→29.00ms, old coverage94.33→88.04ms), with primary parser anchors and full provenance. No current CLI gain follows from those numbers. `00_nucleo/diagnosticos/catalogo-seletivo-preparacao-2026-10-07/audit.json` records1,140 faces/max20 sfnt records/max259,927 projected bytes and259,959 requested bytes excluding ≤16 extra TTC header/index bytes, all historic1,086 assets unchanged and metadata extractor body unchanged. HEAD/dirty bytes/UTC and asset SHA accompany every count. Existing interval coverage remains current, so combination must be freshly measured.

ttf-parser0.25.1 lib.rs:1102–1111,1230–1233 requires head/hhea/maxp;1433–1482 style/weight/width read OS2, monospace post. post.rs:334–379 validates32-byte header/version and34+2×count for v2, not glyph-name tail. `03_infra/src/fonts.rs:339–405` consults names/OS2/post and delegates coverage to its current owner. fontdb0.23.0 lib.rs:721–727,900–920 exposes fresh File reads but retained SharedFile bytes; helper must never substitute a path read for SharedFile's memory.

## Classification and decision

Observable selectable font metadata, exact coverage, family/fallback and rendering preserve language behavior (ADR0107/0108). Buffer/table order, bytes read, mapping and DTO equality are mechanism controls. No claim of upstream intent. Hypothesis of lower acquisition cost is refuted by semantic/corpus divergences or absence of repeatable combined benefit. This helper is not a final-font reader, parser replacement, discovery owner or cache. Gate ADR0127§2.2/§5 is the default acquisition path; materialize only after concrete package ratification.

## Private contract

Own a private `acquire(path:&Path,index:u32)->Option<FontInfo>` facade accessible to its parent only. None always declines optimization, never instructs dropping a font; parent falls back under `00_nucleo/prompts/infra/fontdb.md`. Accept only regular Source::File inputs provided by parent; Unix identity required; unsupported platforms decline. No new public types/APIs, unsafe, mutable globals, feature flags or I/O in L1.

Open one File and capture its metadata. Read sfnt/TTC with Read+Seek/read_exact and checked arithmetic. Supported sfnt signatures are0x00010000/OTTO/true and TTC v1/v2; original face index/array/directory/table offsets must be valid. Collection offsets are absolute to original bytes. Unsupported signatures/layouts/indices decline; do not fabricate validity or broaden accepted input. A private generic reader may enable deterministic fault/cap tests without exposing it publicly.

Budget before every operation: at most4,096 sfnt records,8MiB complete projected buffer and16MiB cumulative requested bytes (including headers, directory, prefixes/rereads). Validate offset+full declared length against actual original length. Bound all allocations and use try_reserve; any overflow, budget/error/reserve failure declines. No attacker-controlled full-file allocation or eager fallback introduced. These caps cause fallback, never truncation of needed metadata; audit demonstrates all measured faces within them but does not promise host/font universality.

Preserve complete sfnt directory order and duplicate records. Copy head,hhea,maxp,name,OS/2,cmap completely. Omitted records keep their positions with offset/length zero. For post, first validate full declared bounds; retain min(length,32) outside v2; v2 with count available retains min(length,34+2×count). Truncated version/header/count/index arrays remain invalid or cause decline; no fabricated version/pitch/indices. Typed ttf-parser determines validity. Rebase copied offsets checked into sfnt-at-zero buffer; if required parsing/metadata fails, decline. No glyph-name payload, outline/shape table or variable instantiation is derived from this buffer.

Use current `font_info_from_bytes(buffer,0)` and current `extract_coverage` on a typed Face of projected buffer; do not duplicate name decoding, exceptions, style inference, interval collection or normalization. Return owned neutral FontInfo only. Projected bytes/Face/handles must not escape to FontSlot, World, shaper/export, or caches. No metadata-only mode/flag: system catalog still requires exact eager coverage.

Before returning Some, verify handle and current path identify the initial regular file: dev/ino,len,mtime/mtime_nsec,ctime/ctime_nsec. Unsupported identity/metadata failure/path replacement/changed length or timestamps decline. This is best-effort change detection, not atomic snapshot guarantee under concurrent writers; unstable sources cannot certify measurements. Do not mutate files/metadata or retain handles across acquisitions. No SharedFile/Binary/embedded path substitution; no sorting/dedup; no altered final mmap/loading failure policy.

## Tests and acceptance

Inline tests precede implementation and demonstrate runtime RED for valid projection/membership/limits/omitted payload; no compile-only RED or performance threshold. Reuse existing licensed fixtures and independent expected name/style/pitch/coverage controls, plus original metadata callback as an additional reference. Include TTF/OTF/variable, original TTC indices/absolute offsets, post valid/invalid versions/prefix/indices/tails, optional/required bounds, duplicate order, corrupt headers/counts/overflow, exact budget edges and early failure before excess allocation/read. Reader injection tests Read/Seek faults deterministically; identity predicate tests detect modified/replaced files. Parent integration tests prove fallback and retained SharedFile behavior, complete ordering and paired slot/book indices.

Final acceptance is language selection/fallback/metadatas and equivalent text/font/page/geometry/PDF/errors, not a demand for Rust or PDF byte equality with vanilla. Per-face equality/book/slots and raster against previous are additional controls. Measure combined acquisition with current interval coverage, full CLI and process RSS against frozen previous and vanilla pin9f2b6e8715237cb086899a42873660fe744622e8. Record source/HEAD/dirty/UTC/argv/asset/bin hashes and all samples, caps/fallback diagnostics outside productive APIs. Three rounds, quiet/monitor/stable hashes, repeatability<3% for precise performance claims. No gain promise from historical experiment; no loss of unsupported input through fast-path rejection. Scope-only seals; build/tests/lint without errors and owners without V5/V6/V15/V26; preserve received tree and negative results, no commit.

Ratificação: dono confirmou com `confirmo` o pacote catalogo-seletivo-2026-10-07, manifesto045ef18d7a16143e640669cf5923db1a213eb677b6c278d4ee4a0bf542973c2e, em2026-10-07.

## Leitura dos payloads diretamente no buffer projetado — 2026-10-10

### Medição anterior à decisão

`00_nucleo/diagnosticos/style-cpu-profile-2026-10-10/cpu-style-current.json.gz`
registra binário90ab169f7d583f948613f6b1f6694d9b144f7402f44406b91fdd4687981fb335,
HEAD/dirty/UTC/argv/scripts e48processos de quatro controles1600. Classificação
por presença de fresh_catalog/fontdb.selective/extração de cobertura
aparece em54–73% das amostras. São stacks inclusivos de1ms, não CPU exclusivo
do Reader. `selective.rs:78–93` aloca/zera um Vec por get, enquanto
`selective.rs:178–187` lê payload para ele, reserva espaço em out e copia
novamente para out. Esta segunda cópia é demonstrada pela fonte, não
quantificada como latência no perfil.

`00_nucleo/diagnosticos/selective-font-buffer-reuse-2026-10-10/before.json.gz`
guarda source/L0, inventário produtivo, fontes, HEAD, working tree e
lista/diffstat. Inferência refutável: ler o payload diretamente na cauda
do buffer já projetado pode poupar alocações/cópias sem mudar aquisição.
Buffers/cópias são mecânica; seleção, metadata, cobertura, ordenação e
fallback continuam observáveis da linguagem. Não inferir intenção upstream.

### Decisão interna, fluxo contínuo ADR-0127

Dentro de project, manter get para cabeçalhos, diretório e prefixo post.
Para cada payload final, reservar/expandir out e ler do mesmo offset
original diretamente no slice novo. Um helper privado do Reader pode
fazer append limitado; na falha não publicar a cauda parcial, restaurando
o comprimento anterior. Cabeçalhos/diretório/prefixos, ordem e número de
leituras/seek dos payloads, requested e limites antes de operação ficam
como antes. Sem pré-leitura, coalescência, cache ou mudanças de source.

Aplicar full-declared-bounds, MAX_BUFFER no resultado e operação, MAX_READ
no acumulado, checked arithmetic e try_reserve antes da leitura. Falha
de budget/reserve/seek/read declina e mantém fallback do parent, nunca
publica metadata parcial. Retain offset/length dos records com mesmo
rebasing, duplicatas/omissões e política post. Identidade do arquivo,
SharedFile/Binary/embedded, parser e extração de cobertura não mudam.
Não há assinatura pública, flag, default de aquisição ou fase nova.

### Aceitação e refutadores

Testes no módulo, antes da implementação: caminho de payload direto
deve ler dentro da alocação do resultado (gate mecânico RED→GREEN),
metadata/cobertura/font selection permanecem; falhas de payload após
cabeçalhos, leituras parciais, limites/overflow, duplicatas/post/TTC e
fallback têm revalidação. Identidade de buffer é controle mecânico da
otimização e não requisito de paridade. Gate externo de alocações, corpus
de faces, linguagem/render/query, suites/build/lint e rodadas pareadas de
tempo/RSS certificam a mudança. Ausência de benefício repetível ou
regressão persistente de memória/tempo refuta; restaurar source/L0 em
caso de rejeição. Sem extrapolar a porcentagem de CPU do catálogo em
ganho prometido. Resselo exclusivo deste owner.
