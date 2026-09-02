# Wiring — typst-wiring
Hash do Código: 32ad6bca

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/compiler-feature-gates.toml sha256:59d8938dc06d347ccc9db23ae1b740876b369227daacd266a219811a661b3cb9
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

## P1288 — transporte ortogonal de `a11y-extras` (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

`04_wiring/src/main.rs:238-250` já desestrutura `features` do compile intent e
`:373-382` o encaminha somente ao caminho HTML. `main.rs:162` reporta hoje
apenas `html: false, bundle: false`; o caminho PDF não recebe o set. Logo o
wiring ainda não consegue exercitar `a11y-extras` em eval/compile PDF.

### Decisão proposta

L4 encaminha o mesmo `Features` canônico de L2 aos entrypoints apropriados de
L3 para compile e eval, inclusive no target PDF. Não interpreta, completa nem
ativa o set. Formato PDF/HTML, `PdfTags` e `StreamMode` permanecem eixos
ortogonais.

`InfoData.features` passa a reportar também `a11y_extras`, com `false` no
snapshot sem configuração explícita. `bundle` permanece false/scope-out. O
report é estado efetivo, não capacidade compilada.

Uma incapacidade devolvida pelo pipeline com `a11y-extras` ativo permanece
erro/Unknown para o harness; L4 não a converte em sucesso nem em feature
desligada.

## P1293.reopen-C — mapping do modo de serialização HTML (PROPOSTO; STOP ADR-0127)

### Medição anterior à decisão

O recibo residual P1293/C SHA-256
`4545df3baa07d09c5c004a77002d18eeaedb47a22aa4883dc2bc3ba12323676a`
mede duas serializações HTML válidas: a cristalina conservadora vigente e a
forma vanilla contextual. Busca read-only não encontrou tipo ou mapping de
modo; `04_wiring/src/main.rs:384` contém o único braço
`OutputFormat::Html`, e `:390` chama a pipeline com features, sem escolha de
serialização. O target já é semântico e separado do layout paginado, conforme
ADR-0128.

### Decisão proposta

No braço `OutputFormat::Html` de `RunIntent::Compile`, L4 faz o único mapping:

```text
typst_shell::cli::HtmlSerialization::Crystalline
    -> typst_infra::export::html::HtmlSerializationMode::Crystalline
typst_shell::cli::HtmlSerialization::Vanilla
    -> typst_infra::export::html::HtmlSerializationMode::Vanilla
```

O valor mapeado é passado à nova entry point L3 de compilação HTML. L4 não
inspeciona conteúdo, não implementa escaping, não altera DOM e não transforma
modo em feature/target. Nos braços PDF/PNG/SVG, o dado cru é ignorado e todo o
dispatch vigente permanece byte-conceitualmente igual. A ausência já chega de
L2 como `Crystalline`; L4 não mantém segundo default oculto.

Esta ligação materializa flag/campo/enums públicos e, por isso, permanece
bloqueada pelo novo gate humano ADR-0127. Após confirmação, os dois mappings,
o default e a neutralidade fora de HTML são testes obrigatórios. Qualquer
necessidade de tipo próprio em L4, branch de exporter fora de HTML ou alteração
de ordem eval→export refuta esta decisão e exige nova autoria.
