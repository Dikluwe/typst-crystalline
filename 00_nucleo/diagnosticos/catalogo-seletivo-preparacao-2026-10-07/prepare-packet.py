from pathlib import Path
import json,hashlib,gzip,subprocess,shutil,datetime
R=Path.cwd();M=R/'00_nucleo/materialization/catalogo-seletivo-2026-10-07';D=R/'00_nucleo/diagnosticos/catalogo-seletivo-preparacao-2026-10-07';M.mkdir(exist_ok=True);C=M/'candidates/00_nucleo/prompts/infra';(C/'fontdb').mkdir(parents=True,exist_ok=True)
sha=lambda b:hashlib.sha256(b).hexdigest();audit=json.loads((D/'audit.json').read_text());old=json.load(gzip.open(R/'00_nucleo/diagnosticos/catalogo-seletivo-2026-10-05/measurement.json.gz'))
def function(s,name):
 a=s.index('pub fn '+name+'(');b=s.index('{',a);depth=1;i=b+1
 while depth:depth+=(s[i]=='{')-(s[i]=='}');i+=1
 return s[a:i]
prior=subprocess.check_output(['git','show',old['head']+':03_infra/src/fonts.rs'],text=True)
assert not any('03_infra/src/fonts.rs' in line for line in old['status'].splitlines())
audit['metadata_extractor_body_unchanged_from_experiment']=function(prior,'font_info_from_bytes')==function((R/'03_infra/src/fonts.rs').read_text(),'font_info_from_bytes');assert audit['metadata_extractor_body_unchanged_from_experiment'];audit['metadata_body_evidence']='Historical receipt: fonts.rs clean at recorded HEAD; body extracted from that tree and compared with current body.'
(D/'audit.json').write_text(json.dumps(audit,indent=2,ensure_ascii=False)+'\n');shutil.copyfile('/tmp/audit-selective.py',D/'audit-inputs.py')
contract='''# Contrato ratificável — prioridade 3: aquisição seletiva do catálogo

Pacote de execução; não é L0. Candidatos L0 completos em candidates/. Nenhum código produtivo ou L0 canônico alterado nesta preparação.

## Medição que precede a decisão

Histórico medido em `00_nucleo/diagnosticos/catalogo-seletivo-2026-10-05.md`: aquisição/metadados 38,10→29,00ms e aquisição+coverage antiga 94,33→88,04ms. O primeiro protótipo, que copiava post inteiro, foi refutado. Esses valores são de sonda de05/10, não ganho atual/CLI, nem devem ser somados ao ganho da prioridade2.

Auditoria atual `00_nucleo/diagnosticos/catalogo-seletivo-preparacao-2026-10-07/audit.json`, com before.json.gz, HEAD/working tree exata/UTC/font hashes: 1.140 faces, máximo20 tabelas, buffer projetado máximo259.927 bytes; solicitação máxima259.959 bytes exclui até16 bytes extra de header/índice TTC. Todos os1.086 assets históricos conservam bytes; corpo font_info_from_bytes continua idêntico à árvore histórica limpa comprovada pelo recibo. Coverage mudou deliberadamente na prioridade2 e permanece vigente. Release anterior congelável SHAe44043d9471a6d2eec25da7562dd5b0a071d4e14ab0f307011ca978e46e49109; oracle vanilla pin9f2b6e8715237cb086899a42873660fe744622e8/bin7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea.

Fontes primárias: `03_infra/src/fontdb.rs:42–113` exige with_face_data e alinhamento book/slots; `03_infra/src/fonts.rs:339–405` extrai name/OS2/post e delega coverage. fontdb0.23.0 lib.rs:721–727,900–920 remapeia Source::File e usa bytes retidos de SharedFile. ttf-parser0.25.1 lib.rs:1433–1482 usa OS2 para estilo/peso/largura e post para monospace; tables/post.rs:334–379 valida32 bytes/version e, na v2,34+2×indices. Tamanho integral declarado deve ser válido antes de truncar post. O tail dos nomes post não é usado pelos metadados. Consulta de nomes/exceções e callback de coverage são dos owners existentes, não serão duplicados.

## Classificação, decisão e refutação

Seleção de fonte, metadados e coverage são observáveis da língua. Forma do buffer, read/mmap, identidade de DTOs e bytes omitidos são mecanismos; igualdade de DTOs é controle interno adicional, não definição universal de paridade (ADR0107/0108). Não inferir intenção upstream do mecanismo observado. Checagem extra: SharedFile usa Arc retido, não o caminho; reabrir seu path pode consultar outra fonte.

Inferência: ler menos payload do Source::File pode melhorar catálogo/CLI após coverage por intervalos. Refuta-se por seleção/coverage/book/slots/erros/render alterados, ou por ausência de benefício repetível/por regressão material em controles equivalentes. Não prometer ganho prévio. Se refutada, conservar/repor o caminho produtivo anterior e registrar todos os resultados.

Proposta: tentativa seletiva por defeito somente para Source::File regular em Unix; Source::SharedFile continua no caminho retido with_face_data. Binary continua excluído. Qualquer rejeição da tentativa, inclusive metadata/coverage inválida, retorna ao callback antigo; fallback não é eager-read de FontSlot. Sem nova API pública, entidade, trait, flag, fase, ordem, política de cache ou alteração de aquisição final de fontes.

Gate: ADR0127 §2.2 (“troca de caminho por defeito do compilador”) e §5 (“em caso de dúvida sobre a classe ... parar”). Classifico esta substituição da cláusula obrigatória with_face_data por uma tentativa padrão de I/O seletivo como mudança de caminho padrão; a confirmação da prioridade2 ratificou apenas coverage e excluiu expressamente prioridade3. Confirmar este pacote antes de instalar L0/escrever consumer novo. A ratificação autoriza os detalhes privados, RED→GREEN, resselo, builds/lint/corpus e medições deste contrato sem novas paragens genéricas.

## Ownership e escopo

1. `00_nucleo/prompts/infra/fontdb.md` ↔ `03_infra/src/fontdb.rs`: escolha File/SharedFile, fallback, composição dos loaders e slot/book.
2. Novo `00_nucleo/prompts/infra/fontdb/selective.md` ↔ novo `03_infra/src/fontdb/selective.rs`: aquisição limitada, projeção privada, estabilidade e extração de FontInfo. Child privado de fontdb; função privada de fachada `acquire(path:&Path,index:u32)->Option<FontInfo>` pode ser pub(super), sem escapar de L3.

`fonts.rs`, coverage.rs, FontSlot, World, L1 e seus L0 permanecem iguais. Parent reutiliza seu callback antigo para fallback. Child reutiliza font_info_from_bytes e extract_coverage vigentes sobre buffer privado normalizado para índice0. Nenhum consumer aponta ao passo, diagnóstico ou núcleo; nenhum prompt compartilhado1:N.

## Limites e aquisição

- Abrir File e obter metadata do mesmo handle; só arquivo regular. Fast path requer identidade Unix (dev/ino, len, mtime/mtime_nsec e ctime/ctime_nsec); se não disponível, declinar. Outros sistemas usam o caminho antigo.
- Confirmar assinatura sfnt 0x00010000/OTTO/true ou TTCv1/v2 elegível. Validar índice original, limites de array TTC/diretório e offsets absolutos com checked arithmetic/get/read_exact. Índice inválido, formato desconhecido, contagem excessiva ou layout não coberto declinam, não fabricam face.
- Máximo4.096 registros sfnt; diretório cabe nesse limite. Máximo8MiB de buffer projetado incluindo header/diretório/tabelas/padding. Máximo16MiB de bytes solicitados por tentativa, contando rereads/prefixos. Conferir budgets e limites antes de seek/alocação/leitura; usar try_reserve para retornar rejeição em falha de reserva. Limites são privados, não truncam metadata; atingidos, retorna ao caminho original.
- Preservar ordem e registros integrais do diretório, inclusive duplicatas; copiar head/hhea/maxp/name/OS2/post/cmap. Tabelas omitidas têm offset/length zerados; não ordenar/deduplicar entradas. Validar extensão integral de toda tabela selecionada contra tamanho original antes de aproveitar prefixo. Bounds inválidos declinam ao fallback, sem transformar opcional inválida em tabela válida.
- post: conservar min(length,32) para versões sem array v2; versão2 com campo de contagem completo conserva min(length,34+2×count). Se campo/array é truncado, conservar invalidade ou declinar; não trocar versão, pitch ou criar índices. O parser tipado continua determinando validade. Campos de nomes post não servem ao catálogo e não são fonte para glyph names/export.
- Reescrever offsets para buffer normalizado sfnt; nenhum byte/referência desse buffer vai a FontSlot/shaping/export. Extrair metadata e coverage com os owners atuais; retorno possui apenas FontInfo. Se qualquer parse/metadata falhar, declinar.
- Antes de aceitar resultado, comparar metadata/identidade final do handle e do path com o início; path trocado/truncamento/mtime/ctime diferentes declinam. Sem garantia inventada de snapshot atômico sob escritor concorrente: casos de input instável não qualificam números/paridade; fallback segue comportamento legado.
- Não cachear buffers, handles, FontInfo globalmente ou porpath; não deduplicar faces nem iniciar leitura seletiva sobre bytes embutidos/SharedFile. O resultado não muda falhas cacheadas/mmap final nem cria fallback eager de arquivo final.

## Sequência e aceitação autorizadas após ratificação

L0 canônico primeiro; testes no owner antes da implementação e RED runtime para projeção válida/caps/leitura reduzida, não erro de compilação ou threshold de velocidade. GREEN cobre fixtures TTF/OTF/variáveis, nomes/exceções/pitch/coverage, TTC índices e offsets absolutos, post versões/tamanhos/arrays/tail volumoso, duplicatas/ordem, tabelas opcionais/obrigatórias, assinatura/header/truncamento/count/overflow, limite em/fora do budget, falhas Read/Seek/reserva quando injetável, detecção de identidade alterada, File fallback e SharedFile com bytes retidos distintos do arquivo. Compare conjuntos membership/iter e metadados selecionáveis; DTO equality/limites são controles de mecanismo.

Comparar metadata/coverage de cada face estável instalada/embutida/fixture e ordem/book/slots à implementação anterior; inputs acima dos caps, rejeitados e SharedFile devem demonstrar fallback antigo. Filtrar FontInfo None exatamente como antes. Não tratar ausência ambiental como aprovação de fallback multiescrita.

Resselo somente desses dois pares; cargo build, release, workspace core/infra/shell, query CLI e crystalline-lint sem erros, owners sem V5/V6/V15/V26. Registrar warnings históricos e preservar todos os demais bytes produtivos/L0 recebidos, sem commit/reset/globalfix.

Medição combinada obrigatória: bytes solicitados (não I/O físico), catálogo, CLI completa e RSS contra release anterior congelado e vanilla pinado. Três rodadas/12 trios, permutações/warmups, CPU4/Rayon1/jobs1, gate30s/monitor/hashestáveis; mesmos oito fixtures/bytes/fonts da prioridade2, qualificação de texto/fonte/página/posição e raster de image/shapes/multiscript perante anterior. Repetibilidade <3% para afirmações precisas; negativas/dispersão preservadas. Proveniência porHEAD/working tree exata/diffstat/fullsources/UTC/argv/assets/bin SHA. Benchmark catálogo anterior usa coverage por intervalos já vigente; não volta ao callback por pontos nem soma ganhos históricos. Relatório/planodebitos/passoregistram conclusão ou hipótese refutada.
'''
(M/'contract.md').write_text(contract)
base=(R/'00_nucleo/prompts/infra/fontdb.md').read_text();base=base.replace('Use `Database::with_face_data` to extract metadata and exact Unicode coverage from the temporary bytes at the supplied face index.', 'For Source::File, first try the private selective child specified by `00_nucleo/prompts/infra/fontdb/selective.md`. If it declines, use `Database::with_face_data` and the original metadata/coverage callback at the supplied face index. Source::SharedFile always uses `Database::with_face_data` over retained bytes; do not reopen its path as a substitute.');base=base.replace('## Contract','''## Measurement before the acquisition decision

Historical `00_nucleo/diagnosticos/catalogo-seletivo-2026-10-05.md` records38.10→29.00ms metadata acquisition and94.33→88.04ms with old coverage; these are not current CLI gains. Audit `00_nucleo/diagnosticos/catalogo-seletivo-preparacao-2026-10-07/audit.json` records HEAD/dirty/UTC/assets,1,140 faces/max20 records/max259,927 projected bytes, unchanged historical assets and unchanged metadata extractor body; priority2 coverage differs and stays current. `03_infra/src/fontdb.rs:42–113` previously mandates with_face_data. fontdb0.23.0 lib.rs:721–727,900–920 distinguishes File reopening from SharedFile retained bytes. Metadata/source order and coverage observable in language must stay equivalent; I/O/buffer mechanics may differ. Infer a possible benefit; wrong metadata/coverage/selection/order/render or lack of repeatable benefit refutes it. Never infer upstream intent or add historical percentage improvements.

## Decision after ratification

Changing the default acquisition path needs the concrete catalogo-seletivo-2026-10-07 package ratified under ADR0127§2.2/§5 before code. No new public API or product flag. Parent owns path selection and pairing, child owns bounded File acquisition; rollback/refusal retains original path. Do not duplicate parsing or convert FontSlot mmap failure into eager reads.

## Contract''');base+='''

## Selective acquisition integration

Declare a private child selective and delegate only Source::File metadata acquisition to its private facade. Successful Some(FontInfo) includes exact coverage; None means declined attempt, so run the original callback rather than omit the face. Source::SharedFile must preserve retained bytes even if its path has changed or disappeared. Source::Binary remains excluded. Do not restructure discovery into sorting/dedup/cache phases, alter public loaders or slots, reparse the projected bytes at a collection index, or use them as final font source. Existing FontSlot and coverage owners remain unchanged.

The child is exclusively `03_infra/src/fontdb/selective.rs` ↔ `00_nucleo/prompts/infra/fontdb/selective.md`. No shared materializable prompt. Keep original metadata callback for fallback. Unsupported/nonUnix/unstable/overbudget/invalid selective input must conservatively use it; existing invalid-face metadata filter and slot/book insertion stay identical.

Acceptance: inline parent tests demonstrate File fallback, SharedFile retained data, empty/nonfont directories, complete metadata/coverage and exact slot/book indices/order. Child tests RED→GREEN first; per-face comparisons and equivalent-language/PDF/CLI corpus against frozen previous and pinned vanilla before any speed conclusion. Final build/tests/scoped lineage and three-round combined catalog/CLI/RSS measurement per ratified package; no public instrumentation or unrelated owners changed. If refuted, preserve/restore prior productive path and record negative evidence.
''';(C/'fontdb.md').write_text(base)
child='''# L0 — Bounded selective acquisition of font catalog metadata
Hash do Código: pendente

**Layer:** L3
**Exclusive consumer:** `03_infra/src/fontdb/selective.rs`
**State:** candidate awaiting catalogo-seletivo-2026-10-07 ratification; do not install canonical L0/code yet.

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
''';(C/'fontdb/selective.md').write_text(child)
pairs=[('03_infra/src/fontdb.rs','00_nucleo/prompts/infra/fontdb.md'),('03_infra/src/fontdb/selective.rs','00_nucleo/prompts/infra/fontdb/selective.md')]
manifest={'package':'catalogo-seletivo-2026-10-07','state':'awaiting_concrete_ratification','head':audit['head'],'base_release_sha256':audit['release_sha256'],'gate':'ADR-0127 §2.2/§5: proposed default File acquisition path; private API only','contract':{'path':str((M/'contract.md').relative_to(R)),'sha256':sha((M/'contract.md').read_bytes())},'owners':[],'unchanged_owners':['00_nucleo/prompts/infra/fonts.md','00_nucleo/prompts/infra/fonts/coverage.md','00_nucleo/prompts/entities/font_book.md'],'audit':{'path':str((D/'audit.json').relative_to(R)),'sha256':sha((D/'audit.json').read_bytes())}}
for consumer,owner in pairs:
 candidate=M/'candidates'/owner;manifest['owners'].append({'consumer':consumer,'owner':owner,'candidate':str(candidate.relative_to(R)),'candidate_sha256':sha(candidate.read_bytes()),'base_consumer_sha256':sha((R/consumer).read_bytes()) if (R/consumer).exists() else None,'base_l0_sha256':sha((R/owner).read_bytes()) if (R/owner).exists() else None})
(M/'manifest.json').write_text(json.dumps(manifest,indent=2,ensure_ascii=False)+'\n');mh=sha((M/'manifest.json').read_bytes());(M/'manifest.sha256').write_text(mh+'  manifest.json\n')
(R/'00_nucleo/materialization/catalogo-seletivo-2026-10-07.md').write_text(f'''# Prioridade 3 — pacote completo preparado

Estado: aguardando ratificação concreta da tentativa padrão de aquisição seletiva para Source::File. Nenhum consumer/L0 canônico/hash/binário alterado nesta preparação; protótipo histórico continua diagnóstico.

Contrato completo: `00_nucleo/materialization/catalogo-seletivo-2026-10-07/contract.md`. Dois candidatos L0 completos, owners exclusivos1:1, base fingerprints e manifesto. Manifesto SHA-256 `{mh}`. Sem nova API pública; helper L3 privado. SharedFile preserva bytes retidos; Source::File regular/Unix tenta leitura limitada, demais casos usam with_face_data original. Buffer parcial nunca serve a FontSlot/shaping/export. Caps e estabilidade explícitos, RED→GREEN/integração/PDF/tempo/RSS exigidos.

Auditoria vigente: `00_nucleo/diagnosticos/catalogo-seletivo-preparacao-2026-10-07/audit.json` e before.json.gz. 1.140 faces elegíveis nos caps,20 registros e259.927 bytes máximos;1.086 assets históricos conservados, metadata body idêntico. Coverage atual já usa intervalos; ganho conjunto ainda não medido. Release e44043d9471a6d2eec25da7562dd5b0a071d4e14ab0f307011ca978e46e49109 preservado.

Gate: `00_nucleo/adr/typst-adr-0127-gate-l0-paragem-vs-fluxo.md` §2.2: “troca de caminho por defeito do compilador”; §5: “em caso de dúvida sobre a classe ... parar”. Classificação registrada: novo caminho padrão de aquisição substitui cláusula atual de Database::with_face_data. A ratificação autoriza materialização do pacote completo, seus testes/fallback/selos e medições sem nova paragem por detalhes privados. A confirmação anterior autorizou exclusivamente prioridade2 e não este caminho novo.
''')
print('manifest SHA256',mh)
