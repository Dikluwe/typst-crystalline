# L0 — Filesystem watch capability and local preview server
Hash do Código: 10e1aa2a

**Layer:** L3
**Exclusive consumer:** `03_infra/src/watch.rs`

## Evidence and ownership

`03_infra/src/watch.rs:56`, `:63`, `:67`, `:134` and `:173` define immediate fingerprint snapshots, consuming finalization, waiting and an in-memory HTTP preview server. Parsing belongs to `00_nucleo/prompts/shell/cli.md`, dependency inventory to `00_nucleo/prompts/infra/world.md`, and cycle composition to `00_nucleo/prompts/wiring/watch.md`. `BundleFile` belongs to `00_nucleo/prompts/infra/pipeline/bundle.md`.

Filesystem transitions and published preview output are observables. The causal constraint is capture before finalization: otherwise recovery may become the baseline and not trigger recompilation. Inference: a capability containing an already captured snapshot prevents this inversion. Lazy capture, an externally callable raw finalizer or baseline recapture would refute that property. Scheduling delays or observation of a later deletion alone do not prove ordering.

## Snapshot and consuming finalization APIs

```rust
pub struct WatchSnapshot { /* private baseline fields */ }
pub struct ArmedWatch { /* only an already captured WatchSnapshot */ }
pub fn snapshot(paths: &[PathBuf]) -> WatchSnapshot;
pub fn arm(paths: &[PathBuf]) -> ArmedWatch;
impl ArmedWatch {
    pub fn publish(self, staging: &Path, destination: &Path) -> io::Result<WatchSnapshot>;
    pub fn abandon(self, staging: &Path) -> WatchSnapshot;
    pub fn publish_directory(self, staging: &Path, destination: &Path) -> io::Result<WatchSnapshot>;
    pub fn abandon_directory(self, staging: &Path) -> WatchSnapshot;
}
pub fn wait_for_change_since(snapshot: WatchSnapshot, interval: Duration);
pub fn wait_for_change(paths: &[PathBuf], interval: Duration);
```

Capture each supplied path immediately, before returning. Fingerprints observe modified time where available, length and content hash. Missing/unreadable paths have an absent fingerprint; later creation remains observable. Creation, removal, changes and different content with unchanged length must be detectable.

`ArmedWatch` stores only the complete captured snapshot: no raw paths, closures or representation allowing lazy capture at finalization. Baseline fields remain opaque outside L3, with no external mutation/getters.

File publication consumes the capability and renames the completed staging file to the destination in a single atomic rename. Success returns the same captured baseline by move. Rename failure attempts staging cleanup but returns the original rename error, never a cleanup substitute.

Abandonment consumes the capability, attempts staging cleanup, leaves the valid destination untouched and returns the same baseline. Raw rename/remove finalizers remain private, so L4 cannot bypass capture.

Directory publication backs up an existing destination to an exclusive sibling, renames completed staging into place, and cleans the backup after success. On replacement failure, attempt to restore the previous destination and clean staging, returning the original replacement error. Restoration/cleanup are best-effort, not an unconditional successful-rollback or single-rename atomicity guarantee for the directory sequence. Directory abandonment removes only staging and returns the captured baseline.

Only specific validated staging/backup/destination paths are in scope; never recursively target a generic filesystem root.

`wait_for_change_since` polls after sleeping for the supplied interval, comparing current fingerprints to the passed baseline without recapturing or replacing it. Public `snapshot` and the compatibility `wait_for_change` remain available; the latter is immediate capture followed by waiting, not a productive finalization bypass.

## Local server API and defaults

```rust
pub struct BundleHttpServer { /* private state */ }
impl BundleHttpServer {
    pub fn new(title: &str, port: Option<u16>, live: bool) -> io::Result<Self>;
    pub fn addr(&self) -> SocketAddr;
    pub fn publish(&self, files: &[BundleFile]);
    pub fn publish_html(&self, html: &str);
}
```

Bind only `127.0.0.1`. An explicit port is tried alone, including ephemeral port 0. Without one, try 3000 through 3005 in order; exhaustion reports `no available local port in 3000..=3005`. The current title parameter does not add content to served documents.

Composition creates this server only for watch HTML/bundle; `--no-serve` prevents binding and `--no-reload` keeps HTTP without injecting the script. These flags do not enable compiler features. Successful output publication permits replacing the HTTP snapshot; a failed compilation preserves the last successful snapshot.

Serve a caller-validated in-memory snapshot, never arbitrary filesystem paths. `publish_html` maps the document to index.html; `publish` replaces the bundle route map. Readers see a published snapshot; successful publications notify active reload subscribers.

## Routes and observable output

Ignore query strings, percent-decode UTF-8 paths and normalize dot segments without escaping the root. Reject malformed decoding, backslash, NUL and above-root traversal. Root/trailing directory routes try index.html; a directory missing its final slash redirects with status 301 and a slash-terminated Location. Found files return 200; invalid or missing paths return 404 with `Not Found`.

Preserve common extension MIME values: html/htm `text/html; charset=utf-8`, css `text/css`, js `text/javascript`, svg `image/svg+xml`, pdf `application/pdf`, png `image/png`, jpg/jpeg `image/jpeg`, json `application/json`, txt `text/plain; charset=utf-8`; unknown types use `application/octet-stream`. Assets are byte-exact unless designated HTML documents receive live injection.

`/__events` is SSE, initially sending `: connected\n\n`; publish notification is `event: reload\ndata: now\n\n`. The endpoint remains available when live injection is disabled. With live mode, inject this literal script before the first case-sensitive `</body>`, or at the end when absent:

```html
<script>new EventSource("/__events").addEventListener("reload",()=>location.reload())</script>
```

Inject only into served HTML documents; never modify written disk output.

## Acceptance

Assert capture-before-removal/publication with an observed staging path, unchanged returned baseline, absent-path creation, unchanged-size content changes, original rename errors and best-effort cleanup. Waiting must not recapture.

Capability and snapshot integration owners are `00_nucleo/prompts/infra/tests/armed_watch_contract.md` and `00_nucleo/prompts/infra/tests/watch_snapshot_contract.md`. External CLI tests have their own owner at `00_nucleo/prompts/wiring/tests/cli.md`.

Check loopback binding, explicit/default ports, 200/301/404, asset bytes/MIME, directory output safety, HTML-only served injection, SSE reload and preservation after failure. Use an explicit ephemeral port and independent readiness signals. Timeouts only bound failure; sleeps, repetition and scheduler luck are not readiness proof. No external server or mechanically identical vanilla threading/headers/locking is required.


## Manutenção local e observação de testes — 2026-10-16

### Medição anterior à decisão

`03_infra/src/watch.rs` nas linhas 170 contém os avisos registrados em `lab/results/construction/diagnostics/clippy-locais-2026-10-16/before.json.gz`. O snapshot conserva fontes/L0, hashes, HEAD, working tree não commitado, UTC e git diff HEAD --stat. A inspeção dos consumidores e usos precedeu esta decisão; o gate Clippy selecionado deve falhar antes do código. Manutenção de representação/observação Rust, sem nova medição de intenção upstream ou paridade.

### Responsabilidade e aceitação

Retirar somente o campo privado redundante BundleHttpServer.live e sua inicialização. O parâmetro público new(..., live) e o live capturado pelo worker e passado a serve_connection permanecem idênticos. bool não possui Drop/recursos causais; aquisição/HTTP/SSE/injeção, API e defaults não mudam.

Fluxo contínuo ADR-0127: sem contrato público, default de produto ou fase novos. L0 precede código e resselo restrito. Nenhuma supressão de lint nova. Conferir redução exata do inventário e ausência de avisos novos, gate Clippy RED→GREEN e unfulfilled_lint_expectations como erro; executar suíte workspace, build, formato, Rustdoc e linhagem sob 12 GiB, swap zero e um job. Não criar testes que apenas espelhem derive, Entry ou posicionamento de módulos. O caso de bbox exige observação real conforme especificado; os demais controles existentes mantêm fixtures e expectativas.


## Decisões por cardinalidade e hexadecimal — V18, 2026-10-16

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado. before.json.gz em lab/results/construction/diagnostics/tekt-ranges-2026-10-16 conserva UTC, fontes/L0, hashes, Cargo manifests/lock/toolchain/config, status e git diff HEAD --stat. A validação anterior, conferida por hashes antes desta etapa, registrou seis V18: quatro em call_spans.rs:52/161/206/226 e dois em watch.rs:287/288. Os L0 atuais e os ramos foram lidos antes da decisão. É estrutura de decisões Rust; as mensagens/spans/rotas são observáveis a preservar, sem nova medição de paridade global.

A sonda isolada extrai exatamente decode_url_path da fonte e substitui somente sua closure digit no candidato. probe-details.json registra argv/rustc 1.92/SHA-256/UTC/retornos; probe-result.json registra 12 GiB, swap zero, um job e eventos. Compara todos os 256 bytes e 65.536 pares: os 22 bytes hexadecimais ASCII e 484 pares aceitos são iguais. Compara também as duas funções completas nas strings de pares e em 13 controles com saídas exigidas de UTF-8, query, segmentos ponto, truncamento, inválidos, NUL e backslash. Os números são desta sonda, não benchmark de HTTP/RSS. Nenhum dado deriva de main móvel do vanilla.

### Responsabilidade exclusiva

Substituir somente a closure digit manual por char::from(value).to_digit(16).map(|digit| digit as u8). Preservar hi/lo e to_ascii_lowercase, checagens get(index+1/2)?, combinação hi*16+lo, avanço, from_utf8, split de query e toda a normalização/rejeição existente. A conversão std é pura e devolve os mesmos dígitos medidos. Não extrair API, ampliar caractere aceito, ler filesystem, decodificar duas vezes, alterar MIME/status/header, injeção/SSE/loopback, captura de watch ou publicação. A mudança reduz a tabela manual em seu lugar atual; nenhum motivo para mover função entre camadas só para evitar o linter.

### Aceitação e limites

Fluxo contínuo ADR-0127: correção interna de representação de decisão, sem assinatura/campo/trait público, comportamento por defeito, dependência, fase ou compatibilidade externos novos. L0 primeiro, testes funcionais antes da implementação e gate V18 RED real. Os novos testes devem passar no baseline: demonstram preservação, não fabricar uma falha de produto onde só havia aviso estrutural. O RED é o linter --checks v18 --fail-on warning; GREEN exige zero V18 e exatamente seis avisos Tekt a menos, sem avisos novos por regra.

T1: âncoras float para zero/um/dois/mais argumentos, bound/spread, named e falta de posição capturada; named prevalece, unbound vazio ancora call mesmo com spread, bound vazio/spread usa args.span. T2: Attach/Mono/Script preservam prioridade relativa de aridade, tipo, nome desconhecido e cramped, com e sem spread; Binom e HTML não mudam. T3: decoder reconhece todos os octetos percent-encoded ASCII válidos e mantém rejeições (NUL, backslash e UTF-8 inválido), case de hex, query, segmentos ponto, raiz e barra final. T4: controles existentes de diagnóstico real P1293 e rotas HTTP/CLI mantêm spans, mensagens, 200/301/404 e saída. T5: build, Clippy workspace/all-targets -D warnings com unfulfilled_lint_expectations como erro, fmt, suíte workspace, Rustdoc -D warnings com quatro índices, diff --check e linhagens estrita/reversa; 12 GiB, swap zero e um job. Fontes/L0 congelados durante comandos; resselo restrito aos dois pares.

Não renomear módulos para escapar da regra, adicionar expectativas/supressões, rebaixar o linter ou alterar esperados para mascarar divergência. Não alegar melhoria de performance ou leitura por IA não medida. Nenhum release.


## Transporte de publicação em L3 — V12, 2026-10-16

### Medição anterior à decisão

O snapshot before.json.gz em lab/results/construction/diagnostics/tekt-v12-2026-10-16 conserva HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, árvore não commitada, UTC, fontes, L0, hashes, status e git diff HEAD --stat. O inventário anterior, conferido por hashes, emite 19 V12: um enum produtivo em 04_wiring/src/main.rs:22 e 18 declarações auxiliares de testes. O enum privado Published tem exatamente Html(String) e Bundle(Vec<BundleFile>); compile.rs:90/99/363/539/541 o produz e watch.rs:103/104 despacha os dois braços. O proprietário atual é wiring.md, que autoriza o carrier privado em L4: esta proposta atualiza explicitamente essa decisão, em vez de contrariá-la sem registro. É propriedade/representação Rust, sem nova alegação de paridade ou intenção upstream.

### Verificação comum

T1: preservar os controles reais de watch HTML e bundle HTTP/reload, sucesso/erro, no-serve/no-reload e último snapshot válido; testes p1485_watch_bundle_http_reload_e_preserva_snapshot_apos_erro e p1485_watch_flags_no_reload_no_serve_e_porta_ocupada em 04_wiring/tests/cli.rs, mais controles HTTP de 03_infra/src/watch.rs. T2: preservar compile de HTML/bundle, árvore completa, assets byte-exatos, stdout, warnings, exit e ausência de publicação em erro nos testes CLI existentes, sem editar esperados. T3: V12 RED real antes e zero produtivo depois; conferir por diff que só houve transferência do enum e ajuste do import, sem inferir modalidade por extensão. Verificar build, Clippy workspace/all-targets -D warnings e -D unfulfilled_lint_expectations, suíte workspace, formato, Rustdoc -D warnings e quatro índices, linhagem V1/V5/V15/V26 e reversa, SARIF e diff --check. Não escrever testes que apenas espelhem a sintaxe da mudança de owner. Execuções pesadas sequenciais sob MemoryMax=12 GiB, MemorySwapMax=0 e CARGO_BUILD_JOBS=1. Não publicar release.

### Responsabilidade exclusiva

Este owner passa a possuir o carrier de publicação, junto da capacidade de preview HTTP já existente. Acrescentar em 03_infra/src/watch.rs, sem novo módulo/re-export:

```rust
pub enum Published {
    Html(String),
    Bundle(Vec<BundleFile>),
}
```

BundleFile continua sendo o tipo existente de 00_nucleo/prompts/infra/pipeline/bundle.md. Não acrescentar derives, métodos, campos/variantes, I/O na construção, cópia dos payloads, flags ou defaults. Manter ownership por move e drop dos mesmos String/Vec. Todas as APIs existentes de watch/server permanecem. O tipo é novo contrato público Rust: código externo poderá depender dele e removê-lo depois será incompatível; requer aprovação ADR-0127 §2.1/§2.4 antes do código.
