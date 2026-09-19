# Wiring — typst-wiring
Hash do Código: 715a11d4

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/compiler-feature-gates.toml sha256:59d8938dc06d347ccc9db23ae1b740876b369227daacd266a219811a661b3cb9
- 00_nucleo/prompts/_nuclei/export/format-option-precedence.toml sha256:68c5c10f124c2c098879318df16c59f6c1477e9172cfa334b603f45ec698ae11
- 00_nucleo/prompts/_nuclei/network/custom-ca-cert.toml sha256:0b28776068ad6b8e85a028a26cfc359679770b03f76d250bfd3ad68ff72643e3
- 00_nucleo/prompts/_nuclei/wiring/cli-observables.toml sha256:0e59744a924f0f6acbe7c4d20db9efc7caed4783c3b8d6b2c2c5a9cace340ece


## Módulo
`04_wiring/src/main.rs`

## Propósito

**Composição pura** do compilador cristalino. L4 consome `RunIntent`
de L2 (`typst_shell::cli::parse()`) e orquestra o pipeline L3.

Passos relevantes:
- **Passo 113** (ADR-0046): CLI mínima.
- **Passo 115** (ADR-0047): `clap` argparsing.
- **Passo 116** (ADR-0048): cores ANSI.
- **Passo 117** (ADR-0049): CLI movida para L2; L4 é composição pura.
- **Passo 119** (ADR-0050): formatter completamente em L2; drain
  inline em L4 (helper local `drain_to_stderr`).
- **Passo 120** (ADR-0051): `-o/--output` + default derivado em L2.
- **Passo 121** (ADR-0051): `--root` em L2; L4 consome `intent.root`
  directamente (sem `input.parent()` local).
- **Passo 122** (ADR-0051): `--font-path` (repetível) em L2; L4
  invoca `typst_infra::fonts::discover_fonts` + `.with_fonts(...)`.
- **Passo 517**: L4 activa descoberta de fontes do sistema por
  defeito via `SystemWorld::with_fonts_and_system(&font_paths)`.

## Contrato

### Uso

```bash
typst <INPUT> [OUTPUT] [-o FILE] [--root DIR] [--font-path DIR]... \
            [--color=auto|always|never] [--document-id UUID]
typst --help
typst --version
```

### Pipeline

1. `typst_shell::cli::parse()` → `RunIntent { input, output, root,
   font_paths, colored, document_id, inputs }`.
2. `main_path = input.file_name()` — falha → exit 2.
3. `SystemWorld::new(&root, &main_path)` → `SystemWorld` (L3).
   Falha de `new` → exit 2.
4. `world.with_fonts_and_system(&font_paths).with_inputs(inputs)` → `World`
   (L3). Combina fontes explicitamente passadas em `--font-path` com fontes
   do sistema carregadas via `fontdb`; `with_inputs` (P694) entrega os pares
   `--input` ao `World` para o módulo `sys`.
5. `world.source(world.main())` → `Source`.
6. **P927/P938** — `world.preload_coverage_if_needed(&source)`: percorre o source
   bruto e, se encontrar carateres não cobertos pelas fontes embutidas, dispara
   o scan lazy de coverage exacta das fontes do sistema antes do layout. Texto
   dinâmico (`context`, interpolações, `read()`) continua no caminho lazy.
7. `compile_to_pdf_bytes*_with_document_id(&world, &source, document_id)` (L3):
   - `eval` → `Module` + warnings.
   - `introspect` → `CounterState`.
   - `layout` → `PagedDocument`.
   - `export_pdf` → `Vec<u8>` (com `DocumentID` fixo quando fornecido).
8. `drain_to_stderr(world, &warnings, &input, colored)` — propaga
   `colored` do RunIntent. Para cada diagnóstico, reúne todos os `FileId`
   referidos pelo span principal e pelos tracepoints, carrega cada `Source`
   via `world.source(id)`, associa o nome de exibição via `world.path_of(id)`,
   deduplica por `FileId` e entrega `Vec<DiagnosticSource>` a L2. O nome é
   relativo ao current working directory quando esse prefixo puder ser
   removido; caso contrário usa o path fornecido por `SystemWorld`.
9. Em sucesso: `fs::write(output, pdf_bytes)`. Exit 0.
10. Em erro de eval: drena errors com mesmo `colored`. Exit 1.

### Exit codes

- **0** — sucesso (PDF escrito).
- **1** — erro de compilação (eval gerou errors).
- **2** — argumentos inválidos (via clap em L2) ou erro de I/O.

### Diagnósticos

Formato humano vanilla-espelhado (adendo P1139 à ADR-0045); política de cores
da ADR-0048 via `colored` do `RunIntent`. Tudo em stderr; stdout nunca usado.

## Separação de camadas (ADR-0049 + ADR-0050)

- **L2** (`02_shell`): `clap`, `Args`, `ColorWhen`, `resolve_colored_with`,
  `RunIntent`, `parse()`, `DiagnosticSource` e `format_diagnostic` humano.
- **L3** (`03_infra`): pipeline, `SystemWorld`, export. Sem formatação
  user-facing (removida no Passo 119).
- **L4** (`04_wiring`): `main()` **thin**. Helper local
  `drain_to_stderr` que materializa todas as fontes referidas, aplica
  `format_diagnostic` + `eprint!` e não entrega `World` a L2.
  Zero deps directas em `clap`; cria tipos? Não — só `PathBuf`
  locais e a função helper.

### Guardas

- **V12 do linter**: L4 não cria tipos. Satisfeito — nenhum struct,
  enum ou trait definido em `main.rs`.
- **`clap` não importado em L4**: `use clap::Parser` **não** aparece.
  Se aparecer em passo futuro, é sinal de que lógica escapou para
  cá e deve migrar para L2.

## P1137-B-001 — dispatch do comando `eval` (gate ADR-0127)

### Medição anterior à decisão

Em 2026-08-23, `04_wiring/src/main.rs:50-177` desestrutura uma única struct
`RunIntent`, constrói `SystemWorld`, lê um source e despacha apenas exportação.
Não existe caminho de stdout para valores. O vanilla ratificado despacha
`Command::Eval` em `typst-cli/src/main.rs:75` e imprime o valor sem criar
artefacto de documento.

### Decisão

`main` faz match exaustivo em `RunIntent`:

- `Compile(intent)` preserva integralmente o caminho atual;
- `Eval(intent)` cria um `SystemWorld` mínimo com root corrente, fontes do
  sistema e source sintético, chama `eval_expression_with_sink`, drena warnings
  e errors pelo formatter L2 e escreve o valor em stdout conforme `EvalFormat`;
- JSON é serialização estrutural de valores JSON-representáveis; `pretty`
  altera apenas whitespace;
- raw escreve bytes exatos para string/bytes, sem newline;
- sucesso retorna 0, erro de avaliação/serialização retorna 1, falha de I/O ou
  argumentos retorna 2.

L4 continua composição: a conversão estrutural `Value`→JSON deve viver em L2
como formatação pública reutilizável, e L4 apenas seleciona/chama o formatter.
Não se usa `repr_value` como substituto de JSON. `query` e avaliação contextual
ficam explicitamente fora desta entrega.

## P1137-I-001 — dispatch do comando `query` (gate ADR-0127)

**Medição anterior à decisão (2026-08-23):** o match de `RunIntent` possui
somente Compile/Eval; `query_helpers` já integra eval+introspect, mas não é
consumido por L4.

**Decisão:** `RunIntent::Query` cria `SystemWorld` a partir do input, carrega a
source, chama `query_elements`, drena warnings/erros e delega a serialização L2.
Stdout recebe JSON; sucesso retorna 0, erro semântico/selector/serialização
retorna 1 e I/O/argumentos retorna 2. L4 não reimplementa selector nem campos de
heading. O warning de deprecação é emitido antes do resultado, em stderr.

## Escopo futuro

Fora dos passos 113–122:

- Subcomandos (entram em L2).
- Flags funcionais (`--format`, `--ignore-system-fonts`, env vars,
  etc.) — entram em `Args` de L2, reflectem em `RunIntent`.
- JSON / SARIF — formatters em L3 ou L2.
- Outros exports (PNG, SVG, HTML).
- Virtualização de imports (resolução real contra `root`) — hoje
  `SystemWorld` ignora `root` para imports e usa `directory_of(
  current_file)`.

## P1362 — corpos de status temporizado no ciclo `watch`

### Medição anterior à decisão

O recibo `diagnosticos/p1362-watch-status-preflight.md` registra que o vanila
possui quatro corpos de status em stderr e que o ciclo cristalino não os emite.
`run_compile_observed` já detém a coleção tipada de warnings, mas devolve apenas
exit e dependências. O formatter puro P1361 já existe em L2.

### Decisão de composição

Somente `run_watch` emite `WatchStatus::Compiling.message()` antes de chamar a
compilação. Ele captura `Instant::now()` nesse ponto e, após o retorno, emite:

- `Success(elapsed)` quando exit é zero e `has_warnings` é falso;
- `PartialSuccess(elapsed)` quando exit é zero e `has_warnings` é verdadeiro;
- `Error` quando exit é não zero.

`run_compile_observed` passa a devolver também `has_warnings`, calculado por
`!warnings.is_empty()` antes de consumir/apresentar a coleção. Early returns
usam false. O wrapper `run_compile` ignora esse terceiro valor e permanece sem
status. Não analisar stderr, não alterar o conteúdo dos diagnósticos e não
introduzir tipo em L4.

Os corpos são escritos em stderr com uma quebra de linha. Timestamp, cor,
fullscreen, limpeza/reordenação de terminal e posição exata perante
diagnósticos ficam fora de P1362. Nenhuma mudança em polling, armamento,
publicação, abandono, dependências ou eviction.

## P1363 — prefixo local `HH:MM:SS`

### Medição e decisão

`diagnosticos/p1363-watch-timestamp-preflight.md` mede o envelope vanila
`[{hora local em %H:%M:%S}] {corpo}`. L4 declara a dependência `time` já pinada
no workspace. P1363 introduziu dois helpers privados:

```rust
fn watch_line_at(status: &WatchStatus, time: time::Time) -> String;
fn watch_line_now(status: &WatchStatus) -> String;
```

`watch_line_at` apenas passa `hour`, `minute`, `second` a
`typst_shell::duration::format_watch_line`. Em P1363, `watch_line_now` tentava
`OffsetDateTime::now_local()` e, se o offset local não estivesse disponível,
usava `OffsetDateTime::now_utc()`; então delegava o `Time`. Os dois pontos de emissão de
P1362 passaram a chamá-lo e continuaram exclusivos de `run_watch`. P1364
substitui esse helper por `watch_block_now` e remove `watch_line_now` depois de
seus call sites chegarem a zero; `watch_line_at` permanece como unidade pura.

Teste unitário usa somente `Time::from_hms(3, 4, 5)` e nunca chama o helper
`now`, o loop ou um processo. Não mover relógio para L2/L1. Cor, fullscreen,
cursor, data/offset/fração, posição perante diagnósticos e mecânica do watcher
permanecem fora.

## P1364 — terminação do bloco não-fullscreen

### Medição e decisão

`diagnosticos/p1364-watch-spacing-preflight.md` mede em
`lab/typst-original/crates/typst-cli/src/watch.rs:97-109` que a mensagem já
termina com uma quebra e que estados diferentes de `Compiling` recebem uma
segunda quebra. O cristalino P1363 usa uma única `eprintln!` para todas as
variantes.

L4 mantém `watch_line_at` como linha sem terminador e acrescenta os helpers
privados `watch_block_at(status, time) -> String` e
`watch_block_now(status) -> String`. O primeiro acrescenta exatamente `\n` a
`Compiling` e `\n\n` a `Success`, `PartialSuccess` e `Error`; o segundo apenas
obtém o mesmo instante local-com-fallback definido em P1363 e delega. Os dois
pontos de `run_watch` escrevem o bloco pronto sem acrescentar outro terminador.
Como `watch_block_now` assume integralmente a captura do instante e os únicos
call sites produtivos, o helper privado intermediário `watch_line_now` é
removido; manter código morto não integra o comportamento requerido.

Teste unitário usa somente `Time::from_hms(3, 4, 5)` e duração literal. Cor,
fullscreen, limpeza/cursor, ordem perante diagnósticos, buffering, flush e
toda mecânica do watcher permanecem fora. Nenhuma API pública muda.

## P1365 — fronteira inicial da duração do `watch`

### Medição e decisão

`diagnosticos/p1365-watch-timer-boundary-preflight.md` mede que o vanila
ratificado captura o marcador monotônico antes de formar e emitir o status
`Compiling`, enquanto o cristalino P1364 o captura depois. Como a duração
resultante integra a mensagem pública de sucesso, a fronteira é observável;
`Instant`, closures e ordem interna específica são mecânica livre.

L4 acrescenta `prepare_watch_cycle_with(capture, format_compiling)`, helper
privado que chama uma vez `capture` e, somente depois, uma vez
`format_compiling`, devolvendo o par sem alteração. O wrapper privado
`prepare_watch_cycle()` fornece `Instant::now` e
`watch_block_now(&WatchStatus::Compiling)`. A cada iteração, `run_watch` chama
o wrapper, emite o bloco pronto e usa exclusivamente o marcador devolvido para
o `elapsed()` do status terminal.

O teste usa closures sentinela e valores literais, sem relógio, sleep, I/O ou
processo. Cor, fullscreen, flush, ordem do status terminal perante
diagnósticos e toda mecânica do watcher permanecem fora. Nenhuma API pública
muda.

## P1366 — status terminal antes dos diagnósticos

### Medição e decisão

`diagnosticos/p1366-watch-diagnostic-order-preflight.md` mede que o vanila
emite `Success`, `PartialSuccess` ou `Error` antes de apresentar warnings e
errors. No cristalino P1365, `run_compile_observed` apresenta os diagnósticos
antes de retornar, e `run_watch` só então emite o status terminal.

L4 introduz o helper privado `report_in_order(status, diagnostics)`, que chama
as duas closures exatamente uma vez e nessa ordem. `run_compile_observed`
recebe uma callback privada `FnMut(ExitCode, bool)` e usa o helper em todas as
saídas. O caminho `watch` fornece a callback que deriva o `WatchStatus` com o
marcador P1365 e emite o bloco P1364; o status pós-retorno é removido. O caminho
`compile` fornece callback vazia e continua sem status de watch.

Resultado, artefato e timings de sucesso são resolvidos antes do anúncio de
sucesso. Falhas preliminares ou de escrita anunciam `Error` antes da mensagem
operacional. No segundo estágio, warnings continuam antes de errors. O teste
do helper usa somente closures em memória. Cor, fullscreen, flush, medições
opt-in e toda mecânica do watcher permanecem fora. Nenhuma API pública muda.

## P1367 — escrita e flush de cada bloco de status

### Medição e decisão

`diagnosticos/p1367-watch-flush-preflight.md` mede que o vanila chama
`flush()` depois de formar cada status, enquanto o cristalino P1366 usa
`eprint!` sem fronteira explícita observável. O flush é relevante porque o
status `Compiling` deve chegar ao utilizador antes de uma compilação longa.

L4 acrescenta `write_watch_block(out, block) -> io::Result<()>`, helper privado
que executa `write_all` uma vez com os bytes integrais e depois `flush` uma
vez. Falha de escrita impede flush; falha de flush é propagada. O wrapper
privado `emit_watch_block` bloqueia stderr e exige sucesso, preservando a
semântica de falha das macros de impressão. Os dois call sites exclusivos de
status P1366 passam a usar o wrapper; diagnósticos não passam por ele.

O teste usa somente writers falsos em memória e cobre sucesso, falha de escrita
e falha de flush. Cor, fullscreen, TTY, buffering posterior ao flush e toda
mecânica do watcher permanecem fora. Nenhuma API pública muda.


## P956 — tradução da flag `--compact` para `StreamMode`

ADR-0126 (emendada P956): `main.rs` traduz o `RunIntent.compact: bool` (L2,
cru) para `StreamMode` (L3) — `compact == true` → `StreamMode::Compact`,
senão `StreamMode::Verbose` — e passa-o como último argumento nas duas
chamadas PDF (`compile_to_pdf_bytes_with_timings_full_error_and_document_id`
e `compile_to_pdf_bytes_full_error_and_document_id`; ver `infra/pipeline.md`
§P956). Os caminhos PNG/SVG não recebem modo (a flag não lhes diz respeito).

## P1137-X-002 — dispatch HTML (ADR-0128)

`OutputFormat::Html` chama `compile_to_html_string`, escreve UTF-8 e emite o
warning experimental medido no vanilla. Não traduz HTML para páginas/SVG/PDF.

### P1323 — envelope completo do warning experimental HTML

#### Medição anterior à decisão

`04_wiring/src/main.rs:388–392` emite apenas a headline, enquanto
`lab/typst-original/crates/typst/src/lib.rs:249–256` constrói headline e três
hints. A medição fresca em `00_nucleo/diagnosticos/p1323-baseline.json`, SHA-256
`8abba2b5f0e3280b4632e4d3bec1db6c2c7a8ee55ab3e9bc9d0d44f197eaa897`,
registra os quatro perfis, saídas completas e fonte, HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093` com working tree não commitado
e diff integral. O alvo é upstream `a51e02804`, binário SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
HTML habilitado produz o warning com os três hints no vanilla e somente a
headline no cristalino. O formato HTML não ativa a feature quando ausente.

#### Obrigação, fronteira e aceitação

A promessa anterior abrange o warning completo, não apenas a headline.
Na emissão fixa existente do braço OutputFormat::Html, quando Feature::Html
está ativa, stderr deve receber uma única vez este envelope textual:

```text
warning: html export is under active development and incomplete
 = hint: its behaviour may change at any time
 = hint: do not rely on this feature for production use cases
 = hint: see https://github.com/typst/typst/issues/5512 for more information

```

São duas quebras de linha após a última linha de hint. Preservar headline,
ordem dos hints, canal e posição da emissão antes da compilação já existentes.
Uma constante privada `HTML_EXPERIMENTAL_WARNING: &str` pode nomear o literal
com a terminação completa; a emissão não acrescenta nem remove newline.
Isso não cria formatter genérico, interpolação, tipo ou API pública.
Testes no próprio módulo conferem conteúdo integral e testes de processo
conferem a ligação efetiva entre o literal, a condição e stderr.

Sem Feature::Html, não emitir este warning; preservar o erro vigente do gate.
Não confundir o controle de ausência com sucesso HTML sem feature. Preservar
artefatos, modos de serialização, STDOUT/exit, demais diagnósticos, formatos
PDF/PNG/SVG, eval/query/help, cores/infraestrutura de formatação e pipeline.
Paridade exata aqui é do envelope textual sem cores; não alegar paridade ANSI
global nem alterar o mecanismo de cores para fechar este fragmento.

Headline/hints/origem/canal são língua diagnóstica sob ADR-0107/0108; a forma
Rust do literal é mecânica. A intenção é a promessa específica deste L0.
É inferência que a emissão fixa basta em um owner: formatter/API adicional,
dados irrecuperáveis, outro consumer causal ou mudança de fase refutam essa
suficiência e obrigam a reabrir o escopo antes de código. Correção interna
ADR-0127 em fluxo contínuo, L0-first + resselo + RED→GREEN + revalidação.
Aceitação inclui testes positivos, ausência com feature desligada, repetição
e inversão, preservação dos artefatos e controles de outros formatos. Unknown
obrigatório não é sucesso. Não inferir paridade HTML global deste warning.

### P1165 — fio de features e default (RASCUNHO; ADR-0127)

**Medição:** `main.rs:161` anuncia `features.html = true` fixo e
`main.rs:371-379` despacha HTML sem gate. No vanilla ratificado, `info` anuncia
false por default e compile HTML sem feature falha.

Após aprovação, L4 transporta a coleção do intent até a pipeline, recusa HTML
quando `Feature::Html` está ausente com o diagnóstico medido e calcula
`InfoData.features.html` do estado efetivo, nunca como constante de
capacidade. L4 não inventa ativação por formato nem lógica de linguagem.

## P1137-C-001 — dispatch `compile` canónico (AGUARDA CONFIRMAÇÃO ADR-0127)

Após aprovação do contrato em `shell/cli.md`, L4 continua a receber o mesmo
`CompileIntent`: somente a origem sintática muda de argumentos posicionais
globais para o subcomando `compile`. A tradução de compatibilidade da invocação
legada pertence a L2; L4 não distingue as duas grafias.

Nenhum braço de dispatch será criado para `watch`, `init`, `fonts`,
`completions` ou `info` até a respectiva capacidade existir. O wiring não deve
conter stubs de sucesso nem anunciar comandos que terminam invariavelmente em
“não implementado”.
## P1137-CERT — injeção da CA

L4 passa `cert_path` de `CompileIntent`, `EvalIntent` e `QueryIntent` para
`SystemWorld::with_custom_ca`. Em `info`, combina presença via flag com a
presença de `TYPST_CERT`, sem expor o path. L4 não lê PEM nem configura TLS.

## P1137-INIT — composição

L4 encaminha `InitIntent` para `typst_infra::project_init::initialize`, imprime
destino e entrypoint em sucesso e converte falhas em exit 1. Toda resolução,
validação e escrita permanece em L3.
## P1137-WATCH — composição incremental

Medição anterior à decisão: `run_compile` construía um `SystemWorld` efémero,
escrevia diretamente no destino e descartava o inventário de ficheiros lidos;
`eviction::crystalline_evict` já existia reservado para watch.

Decisão: `RunIntent::Watch` compila imediatamente e repete somente quando L3
detectar mudança no main ou numa dependência registada pelo último world. Cada
iteração usa um world novo, compila para staging no mesmo diretório e só
substitui o destino após sucesso. Erro de compilação preserva o último
artefacto válido e mantém o ciclo; erro ao publicar é fatal (exit 2). Entre
iterações L4 chama `crystalline_evict(10)`. Output `-` é rejeitado antes do
ciclo.

## P1140.6 — tradução de `--no-pdf-tags`

### Medição antes da decisão

L4 já traduz `compact` para `StreamMode`, mas não existe tradução ortogonal
para tagging.

### Decisão

Nos caminhos PDF de `compile` e `watch`, L4 traduz
`intent.no_pdf_tags == false` para `PdfTags::Enabled` e `true` para
`PdfTags::Disabled`, passando o enum depois de `StreamMode` às entry points
L3. Não combinar as duas decisões nem propagar tagging aos exports PNG, SVG ou
HTML. L4 continua sem criar tipo próprio e sem validar PDF/UA.

## P1359 — precedência de `--pdf-tagged`

### Medição antes da decisão

P1140.6 já materializou a tradução thin do legado `no_pdf_tags` para
`PdfTags`, e todas as entry points PDF já recebem esse enum. O novo protocolo
tri-state medido no vanilla ratificado é resolvido antes da chamada ao
exportador e não exige nova fase nem mudança em L3.

### Decisão

L4 destrutura `pdf_tagged: Option<bool>` de `CompileIntent` e resolve:

```text
Some(true)  -> PdfTags::Enabled
Some(false) -> PdfTags::Disabled
None + no_pdf_tags -> PdfTags::Disabled
None sem legado    -> PdfTags::Enabled
```

O valor moderno explícito vence o legado. O enum continua sendo passado
somente aos caminhos PDF; PNG, SVG e HTML permanecem neutros. `StreamMode` e
`compact` continuam eixo independente. L4 não emite política de standards,
não implementa seleção de páginas e não altera exportadores.

O gate ADR-0127 foi confirmado pelo dono em 2026-09-12 junto com a autorização
explícita para implementar P1359.

## P1360 — composição ortogonal de `pretty`

L4 destrutura `pretty: Option<bool>` de `CompileIntent` e resolve ausência
como `false`, coerente com os defaults de formato medidos. O booleano segue
somente ao encoder do target: PDF recebe `PdfSerializationMode`, SVG recebe
`SvgOptions.pretty`, HTML recebe a variante pretty da pipeline e PNG ignora o
valor.

`pretty` não altera `StreamMode`, `PdfTags`, feature HTML,
`HtmlSerializationMode`, formato selecionado, layout ou código de saída. Em
particular, `--compact --pretty=true` mantém operadores compactos com
serialização PDF legível, e `--pdf-tagged=false --pretty=true` mantém tags
desligadas. L4 não formata bytes nem inspeciona DOM/PDF/SVG.

## P1198 — fronteira com as suítes de integração

Este prompt possui exclusivamente `04_wiring/src/main.rs`. Os observáveis de
processo compartilhados com a suíte CLI estão no Núcleo Tekt pinado acima.
`wiring/tests/cli.md` possui a suíte do binário;
`wiring/tests/crystalline_lint.md` possui a suíte independente de V14. Testes
não legitimam código produtivo e o owner produtivo não absorve o harness.

## P1215 — fonte transitória do comando `eval`

`run_eval` materializa em memória uma `Source` code com `world.main()` e a
expressão recebida, byte-idêntica à usada pelo entrypoint L1, e fornece-a ao
formatter apenas durante a drenagem de diagnósticos. L4 não decide regiões nem
reescreve spans/mensagens; somente torna resolvível a fonte transitória que não
existe no filesystem. Os demais comandos mantêm a resolução pelo `World`.

## P1327-R2 — erros de avaliação antes dos warnings no comando eval

### Medição anterior à decisão

`00_nucleo/diagnosticos/p1327-cli-candidate.json`, SHA-256
`3940e04d86b519b248f57e3711f815726a9648b005df532146b9496c8df3a317`,
registra HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree
não commitado com diff/stat, UTC, comando e binário C1
`5f00502b5055fedf8ca1dc2c879112ac1300e12621e11fdb2c731364450a62f9`.
O corpus integral `p1327-ab-cli-candidate.json` em diagnosticos, SHA-256
`6ffa5fc5ce6925eb0c85e40749adff5d1688ce006fe2400cdad3a4edeb47fafe`,
tem 24 diferenças: somente dois casos com warning de import e erro posterior,
nos quatro perfis e três ordens. Exit, stdout e cada bloco diagnóstico são
iguais; C1 imprime warning antes do erro, enquanto o vanilla ratificado
`a51e02804` imprime erro antes do warning. Os oráculos não foram alterados.

`04_wiring/src/main.rs:500-507` drena warnings antes de consultar result;
`:510-520` só depois imprime os errors. No vanilla,
`lab/typst-original/crates/typst-cli/src/eval.rs:64-81` coleta errors, e
`typst-cli/src/compile.rs:727` os apresenta antes dos warnings. A revisão
`00_nucleo/diagnosticos/p1327-review-cli-failure.md` refuta a suficiência
do owner modules para o transcript completo. O estado anterior a esta
sucessão está em `p1327-r2-baseline.json` em diagnosticos, SHA-256
`84056881f6481305cd530532849bf470837bbc966c4f7fd138b0094c46954b3e`.

### Decisão e aceitação

Somente em `run_eval`, quando a avaliação devolve Err, drenar todos os
diagnósticos errors na ordem recebida e depois todos os warnings na ordem
recebida, uma vez cada, mantendo exit 1 e stdout vazio. Não selecionar por
mensagem, tipo de import ou testemunha. Preservar severidade, hints, trace,
spans, Source transitória, formatter, cor e resolução de fontes.

Quando a avaliação devolve Ok, preservar warnings antes da serialização e
do stdout. Erros posteriores de serialização/I/O mantêm a ordem e códigos
vigentes. Não tocar compile, query, watch, exporter, avaliação L1, parser,
sink/deduplicação ou drain genérico; não transformar warnings em erros.

Esta sucessão é composição privada de paridade diagnóstica ADR-0127 em
fluxo contínuo, sem API, flag, default, fase ou compatibilidade novos.
O RED CLI C1 integral é obrigatório e permanece registrado; exigir GREEN
dos mesmos transcripts, controle sem warning, warning sem erro, erro sem
warning, valores/serializadores existentes, perfis e repetição/inversão.
Warnings preexistentes combinados com erro seguem a mesma ordem de emissão;
probes independentes adicionais verificam essa generalidade, sem ajustar
expected ao candidato. Unknown obrigatório e alteração fora de run_eval
refutam o escopo. Nenhuma aceitação implica paridade geral de diagnósticos.

## P1285 — transporte de formato e fonte transitória de `query`

### Medição antes da decisão

`run_query` chama `serialize_query` sem transportar o formato já decidido em
L2, portanto todo sucesso é forçado a JSON. Além disso, embora o argumento
posicional anunciado aceite `-`, L4 entrega esse token a `SystemWorld::new`
como nome de ficheiro e falha antes da avaliação. No vanilla ratificado
`a51e02804`, as sondas de query por stdin preservam o mesmo resultado semântico
que uma fonte física e `--format yaml` seleciona YAML.

### Decisão

L4 permanece composição fina: encaminha `QueryIntent.format` ao serializer L2
sem reinterpretá-lo. Quando `input == "-"`, lê stdin como UTF-8, cria um
`SystemWorld::for_eval` e uma `Source` markup transitória com `world.main()`;
quando é outro path, conserva o fluxo físico existente. Os dois ramos entregam
a mesma combinação `World + Source` a `query_elements`, mantêm warning e
diagnósticos em stderr e output estruturado em stdout. L4 não serializa YAML,
não enumera campos, não decide selectors e não altera cardinalidade.

Esta correção materializa comportamento já anunciado pelo argumento `-` e o
formato público confirmado em L2; não cria flag, default ou assinatura pública
nova.

## Transporte ortogonal de `a11y-extras`

L4 encaminha o mesmo `Features` canônico de L2 aos entrypoints de compile e
eval, inclusive PDF. Não interpreta nem ativa o set. Formato, target,
`PdfTags` e `StreamMode` permanecem eixos ortogonais.

`InfoData.features` reporta `a11y_extras` como estado efetivo. Incapacidade
de pipeline permanece erro; L4 não a converte em sucesso ou feature desligada.

## Modo de serialização HTML

No braço HTML, L4 faz somente o mapping:

```text
shell::HtmlSerialization::Crystalline -> infra::HtmlSerializationMode::Crystalline
shell::HtmlSerialization::Vanilla     -> infra::HtmlSerializationMode::Vanilla
```

O valor segue para a entrypoint L3. L4 não inspeciona conteúdo, implementa
escaping ou altera DOM. PDF, PNG e SVG ignoram esse dado. O default já chega de
L2 como `Crystalline`.

## Composição de `watch`

Depois de `run_compile_observed` produzir exit code e dependências, L4:

1. normaliza o conjunto transitivo, usando apenas o input como fallback quando
   vazio;
2. chama `typst_infra::watch::arm` e recebe `ArmedWatch` já com snapshot;
3. no sucesso, consome a capacidade com `publish`; falha de rename é fatal;
4. no erro de compilação, consome com `abandon` e preserva o último destino;
5. executa `crystalline_evict(10)`;
6. passa o mesmo snapshot a `wait_for_change_since`.

```text
compile -> normalize -> arm
  success: publish
  error:   abandon
-> evict -> wait(snapshot)
```

L4 move a capacidade, mas não calcula fingerprints, não faz polling e não
executa I/O de watch diretamente. Helpers crus de commit/discard não são sua
interface. Publicação atômica, cleanup de staging e renovação das dependências
permanecem obrigatórios.

## P1401 — composição de `eval --in` (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

No baseline P1401, `04_wiring/src/main.rs::run_eval` fixa raiz `.` e constrói
`SystemWorld::for_eval`, portanto nunca oferece documento contextual; o vanila
ratificado aceita o mesmo ensaio com `--in`.

### Decisão proposta

L4 preserva esse ramo quando `EvalIntent.input` é `None`. Quando há input,
compõe o `SystemWorld` físico correspondente — ou uma fonte única recebida de
stdin para `-` — e chama a operação contextual de L3. A expressão continua a
ser serializada e diagnosticada pelo fluxo vigente. L4 não implementa query,
não inspeciona seletores e não duplica fases do compilador. Falha de leitura ou
compilação retorna exit 1 pelos diagnósticos normais; erro de parsing CLI segue
exit 2 em L2. O fio público aguarda confirmação ADR-0127.

## P1462 — preservar ausência de `pretty` no braço SVG (MATERIALIZADO E CERTIFICADO)

### Medição anterior à decisão

P1360 colapsa `CompileIntent.pretty: Option<bool>` em `false` antes do dispatch.
Isso é correto enquanto nenhum formato possui opção documental, mas impede o
SVG de distinguir ausência de `--pretty` de `--pretty=false`. No vanilla
ratificado, CLI explícito vence `format.svg.pretty`; sem CLI, a regra do
documento vence; sem ambos, o default é `false`.

### Decisão proposta

Preservar o `Option<bool>` original exclusivamente até a chamada do braço SVG
e passá-lo à nova entry point L3 de override parcial. `Some(true)` e
`Some(false)` são overrides explícitos; `None` delega ao documento. Este fio
aplica o Núcleo Tekt `format-option-precedence` pinado acima.

PDF e HTML continuam a receber o booleano já resolvido por P1360 até P1464 e
P1465 introduzirem, com L0/gate próprios, opções documentais homólogas. PNG
continua a ignorar `pretty`. `--compact`, `PdfTags`, features, formato, output,
exit codes e diagnósticos não mudam. L4 não lê `PagedDocument`, não interpreta
set rules e não formata SVG.

A mudança de default observável no caminho SVG e a chamada de API nova foram
confirmadas pelo humano em 2026-09-19, depois do commit causal P1462, e a
materialização foi certificada no mesmo dia.

## P1463 — transportar ausência/valor de `ppi` somente ao braço PNG (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

No baseline `23cd47e4ade4d6deb7095cb7400478feffb1e9f9`,
`run_compile_observed` destrutura `CompileIntent` sem `ppi` e os dois caminhos
PNG chamam a entry point que usa `RenderOptions::default()`. Não há valor a
preservar entre L2 e L3.

### Decisão proposta

Desestruturar `ppi: Option<f64>` sem aplicar default ou converter unidade. Nos
dois ramos PNG — com e sem `timings_json` — chamar a entry point L3 de override
parcial e passar o mesmo `ppi`. `None` delega ao documento; `Some(value)` é
override explícito e vence o documento segundo o Núcleo
`format-option-precedence` pinado acima.

PDF, SVG e HTML aceitam a presença sintática da flag em L2, mas não recebem o
valor nem mudam comportamento. `watch` reutiliza o mesmo `CompileIntent` e a
mesma composição por iteração. L4 não divide por 72, não lê
`PagedDocument.format_options`, não constrói `RenderOptions`, não valida
finitude e não inspeciona bytes PNG.

Preservar integralmente `pretty`, `compact`, `PdfTags`, feature HTML,
serialização HTML, `document_id`, timings, publicação atômica de watch e
códigos de saída. A nova chamada pública e o transporte do campo aguardam
confirmação humana posterior ao commit causal P1463 antes de consumer/header/
teste RED.
