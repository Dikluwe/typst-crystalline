# L0 — wiring/compile
Hash do Código: 561e2ab5

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/compiler-feature-gates.toml sha256:59d8938dc06d347ccc9db23ae1b740876b369227daacd266a219811a661b3cb9
- 00_nucleo/prompts/_nuclei/export/format-option-precedence.toml sha256:68c5c10f124c2c098879318df16c59f6c1477e9172cfa334b603f45ec698ae11
- 00_nucleo/prompts/_nuclei/network/custom-ca-cert.toml sha256:0b28776068ad6b8e85a028a26cfc359679770b03f76d250bfd3ad68ff72643e3
- 00_nucleo/prompts/_nuclei/watch-font-generation.toml sha256:391cd1604976262b1b6e019b802ed72e78782260d2788bb04999297559605509
- 00_nucleo/prompts/_nuclei/watch-source-generation.toml sha256:7ec137b6996e0ea442e3cdda52ce723fd32cb67338ab363b7ff2d733458ef8fa
- 00_nucleo/prompts/_nuclei/wiring/cli-observables.toml sha256:0e59744a924f0f6acbe7c4d20db9efc7caed4783c3b8d6b2c2c5a9cace340ece

**Layer:** L4
**Exclusive consumer:** `04_wiring/src/compile.rs`

```text
```

## Current evidence

- `04_wiring/src/compile.rs:40` — `report_in_order`.
- `04_wiring/src/compile.rs:45` — `write_bundle_output`.
- `04_wiring/src/compile.rs:81` — `run_compile`.
- `04_wiring/src/compile.rs:86` — `run_compile_observed`.
- `04_wiring/src/compile.rs:546` — `p1323_html_experimental_warning_matches_ratified_envelope`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 04_wiring/src/compile.rs`.

## Owned contract

L4 composes the existing L1 domain, L2 presentation and L3 infrastructure. Font/file/package acquisition, compilation and exporting remain delegated to L3. Existing stdout/stderr and output-tree publication calls are wiring orchestration (ADR-0049/0050); they do not authorize moving domain algorithms or resource acquisition into L4.

- Own run_compile/run_compile_observed, report_in_order, write_bundle_output and experimental-warning constants. Preserve observed return values (exit, dependencies, input kind, optional Published) and the no-op-observer wrapper.
- Keep pre-World validation order: bundle stdout/feature; explicit PDF pages+tagged; CLI-only accessible-standard/tagging with conditional pages hint; conflicting PdfStandards. Preserve early exits and callback order, without implicit feature activation.
- Construct SystemWorld with current root/main/font/input/CA/plugin configuration. Map compact only to StreamMode; preserve tagging legacy false/explicit Some/absent None and pretty/pages/standard/PPI typed overrides until L3 resolves them. PDF oracle selection is PDF-only.
- Leave font acquisition to L3 resource demand; normal compilation does not invoke explicit source coverage preloading. System/project fonts remain available through the unchanged builders and World contract.
- Keep exhaustive target dispatch Bundle/Pdf/Png/Svg/Html, every timing/non-timing path and all BundleCompileOptions fields. Map HTML serialization to the existing L3 enum only; never perform layout/export or reconstruct format defaults in L4.
- On successful output write bytes or complete BundleFile tree. Preserve normal-component checks, parent creation, canonical/symlink/escape checks and existing publication policy; do not add incidental hardening or atomicity changes.
- Compilation errors exit 1; write/timing failures exit 2; success exits 0 and alone produces Published. report_in_order invokes status then diagnostics once: errors, conditional experimental warnings, domain warnings, then operational error. Preserve opt-in CRYSTALLINE_MEASUREMENTS=1 through the existing L3 APIs.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

## `--pdf-standard` no target bundle — proposta pendente de gate ADR-0127 (2026-09-30)

### Medição anterior à decisão

A sonda `lab/results/construction/diagnostics/pdf-standard-bundle-2026-09-30/results.json`
mede PDF isolado sem título rejeitado sob `--pdf-standard=ua-1` e PDF de
bundle sem título publicado sob a mesma flag. `04_wiring/src/compile.rs:155`
limita a construção de `standard_override` ao target PDF direto; em
`04_wiring/src/compile.rs:325`, o campo homólogo de `BundleCompileOptions`
recebe consequentemente `None`. `03_infra/src/pipeline/bundle.rs` já resolve
esse campo por entrada. A hipótese de falta de fio é refutada se o bundle
receber Some(PdfStandards) para essa flag; o código medido envia None.

### Contrato proposto, ainda não vigente

Se autorizado, L4 valida a lista explícita de standards por
`PdfStandards::new` nos targets PDF e Bundle, mantendo ausência como `None`.
Passa esse `Option<PdfStandards>` a `BundleCompileOptions` sem reconstruir
opções por entrada. A pipeline L3 aplica a precedência caller > documento >
default e suas validações PDF somente às entradas PDF; erros impedem a
publicação da coleção. O gate CLI-only de standard acessível versus tags
permanece no PDF direto; o bundle usa o gate documental por entrada. O
resto da ordem pré-World e dos targets não PDF permanece igual. Esta
proposta altera o efeito observável da flag no bundle e **não autoriza L4**
antes da confirmação ADR-0127. Depois da confirmação, atualizar o contrato
vigente, ressellar este owner e verificar RED→GREEN.

## Compilação de stdin com saída explícita — 2026-10-06

### Medição anterior à decisão

`lab/results/construction/diagnostics/compile-stdin-2026-10-06/before.json.gz` conserva
HEAD/árvore/UTC, fontes, comandos e SHA-256 dos binários. Com arquivo físico
`-` contendo #unknown_physical e stdin independente, compile - abre esse
arquivo no Tekt; o pin 9f2b6e8715237cb086899a42873660fe744622e8 usa stdin.
`04_wiring/src/compile.rs:223-270` compõe SystemWorld::new para toda entrada;
`03_infra/src/world.rs:102-106` já fornece for_stdin sem leitura física.
`lab/typst-original/crates/typst-cli/src/args.rs:164` documenta explicitamente
`-` como stdin; world.rs:222-242 valida raiz e seleciona a identidade stdin,
world.rs:282-290 adquire bytes. A opção documentada comprova intenção;
a estrutura de FileId e a aquisição lazy upstream são mecânica (ADR-0107).
Origem, diagnóstico, recursos e resultado são observáveis. A inferência de
que a seleção de World causa a diferença é refutada se trocar somente essa
composição não preservar a entrada/resultado nos controles.

### Contrato e aceitação

Para input exatamente `-`, validar/canonicalizar a raiz já resolvida por L2,
adquirir bytes de stdin uma vez, aceitar BrokenPipe parcial como o pin e
validar UTF-8. Raiz ausente sai 1 com root directory not found (searched at ...);
UTF-8 inválido sai 1 com file is not valid UTF-8 e o hint de arquivo sem extensão.
Outras falhas de aquisição saem 2 com diagnóstico operacional. Status callback
é chamado uma vez antes do diagnóstico nos exits deste ramo.
Compor SystemWorld::for_stdin com texto adquirido e os mesmos builders de
fontes/inputs/CA/plugins. Não abrir `-` nem criar arquivo para o documento.
Imports/read usam a raiz fornecida; diagnósticos e traces mantêm a identidade
stdin com caminho relativo à cwd, sem renomear origens de imports.
Passar a Source existente aos mesmos braços de formato/timings, preservando
publicação, avisos e fases. Não alterar assinaturas públicas ou flags.

Regressões black-box devem distinguir stdin de um arquivo `-`, validar PDF
em arquivo/stdout, SVG/PNG/HTML, inputs/import/read com raiz explícita, erro
e trace causal, warning de ** e UTF-8 inválido. Comparar conteúdo de linguagem
e diagnósticos, não bytes inteiros de PDF/SVG nem números internos de ids.
O parser ainda deriva output quando omitido; a validação de output obrigatório
para stdin não pertence a este recorte e permanece pendência explícita.
Correção interna de paridade em fluxo contínuo ADR-0127, sem contrato público,
default novo ou mudança de fase. Não alterar a ordem warning/error existente.

### Saída stdout — evidência adicional antes do reparo

`green.log` mantém o RED residual: compilação aceita stdin, mas a saída `-`
não chega a stdout. `04_wiring/src/compile.rs:487-492` usa std::fs::write
para todos os outputs não-bundle; before.json.gz conserva PDF em stdout no
vanilla. Encaminhar output exatamente `-` a stdout.lock().write_all no mesmo
ponto de publicação, com os mesmos bytes exportados e tratamento de falha.
Vale também para entrada física; bundle continua rejeitado antes do World e
watch continua rejeitando stdout. Exigir que o arquivo físico `-` não seja
sobrescrito, com sucesso/status/stderr e assinatura de PDF em stdout.

### Refutação de legenda incondicional — coleta after.json.gz

O controle root_error refuta a expectativa inicial <stdin> sob qualquer raiz:
com --root project e cwd pai, o pin apresenta project/<stdin>:1:1. Usar o
formatter existente e o caminho sintético root/<stdin> do World, sem primary
que substitua o caminho por <stdin>. Na raiz cwd o nome é <stdin>; em raiz
filha é project/<stdin>. O conteúdo e o id causal permanecem os mesmos.
Corrigir a expectativa de teste pela sonda upstream, preservando os dois
nomes no controle bilateral, inclusive o trace sob raiz explícita.

## Ordem de diagnósticos de compile — gate confirmado ADR-0127

before.json.gz/plano-debitos-2026-10-06 mede warning-error contra o pin:
compile.rs:779 upstream usa errors.iter().chain(warnings). O contrato local e
ADR-0045 adendo §8 mandam o contrário, a substituição foi confirmada pelo dono no gate de 2026-10-06.
Status observado continua antes de qualquer diagnóstico. Em falha, drenar
errors primeiro; depois warnings experimentais de formato, warnings do domínio,
e por último erros operacionais. Em sucesso, preservar warnings experimentais
antes dos warnings do domínio. Não alterar mensagem/span/hint/trace, cor, exit ou publicação.

Gate confirmado pelo humano em 2026-10-06 (resposta confirmo), com candidatos
e hashes preservados em lab/history/construction/records/00_nucleo/materialization/warnings-cli-aprovacao-2026-10-06.json.

Refinamento medido antes da materialização: warnings-cli-2026-10-06/before.json.gz,
controles compile-html-warning-error/only, refuta colocar experimental após domínio.
A ordem interna dos avisos upstream permanece preservada; somente errors passam à frente.


## Medição anterior à decisão de retenção ratificada

lab/results/construction/diagnostics/watch-retencao-2026-10-07.md registra HEAD/árvore/binários/UTC,
edições reais, recuperação e contagem dinâmica de aquisição por ciclo.
lab/results/construction/diagnostics/watch-validacao-2026-10-07/measurement.json mede validação por
descoberta versus aquisição completa no mesmo estado; controls.json testa adição,
remoção, substituição, mtime restaurado, links e configuração. São custos isolados,
não ganho de CLI ou prova de intenção upstream. FontBook/slots/cobertura devem
preservar seleção, conteúdo, diagnóstico e render na linguagem; duração/Arc são
mecânica. Inferimos benefício possível, refutado por custo, stale resources,
regressão ou retenção ilimitada. Fixtures/proveniência/limites constam do relatório.

### Decisão ratificada sob ADR-0127 §2.1/§2.2

Dono ratificou com Confirmo o pacote lab/history/construction/records/00_nucleo/materialization/watch-catalogo-2026-10-07/manifest.json. Receipt: lab/results/construction/diagnostics/watch-catalogo-execucao-2026-10-07/ratification.json. Cada owner continua 1:1.

## Composição com sessão opcional, somente helper privado

Preserve run_compile e run_compile_observed como wrappers compatíveis, sem sessão,
para compile comum. Um helper privado/pub(super) do owner aceita opção de Arc da
FontCatalogSession; sem Arc mantém with_fonts_and_system, com Arc aplica o novo
with_font_catalog_session com os mesmos font_paths. Construa SystemWorld novo em
toda chamada, mantendo validações/inputs/CA/plugins/source antes do pipeline e
retorno atual de deps/exit/input-kind/Published. Não reordenar fases ou diagnósticos,
alterar flags/timers, tornar a sessão global ou reutilizar um World anterior.
Somente watch passa a sessão. O handler mantém stdout, stdin, todos os targets e
precedência/export/PDF-publicação atuais. Propostas históricas não ratificadas
deste prompt continuam fora do escopo.

## Reuso sintático de watch — lote A

### Medição anterior à decisão

lab/results/construction/diagnostics/incremental-prioridade6-2026-10-08/before.json.gz,
measurement.json.gz, control-measurement.json.gz e count.json registram árvore,
HEAD, UTC, comandos e releases. Source::new reconstrói CST; World novo constrói
Source main/import a cada edição. O import mínimo já absorve recursos no eval_ms;
não atribuir toda a métrica a parsing. O contrato causal de Source/id/caminho vem
dos owners existentes; o reuso é mecânica e precisa preservar observáveis de língua.
Hipótese de benefício refutada por custo, diagnóstico, efeito, stale source,
geometria ou identificação diferente. Não inferir intenção upstream.

### Contrato ratificado

Transportar Arc opcional SourceSession por helper observado privado, junto da
sessão de fontes existente. APIs públicas compile/observed conservam assinatura
e delegam sem sessão sintática. Para physical input com sessão, usar constructor
new_with_source_session antes da leitura eager; sem sessão/transiente, caminho
existente. Não executar validação ou lógica de cache em L4. Preservar erros,
inputs/font_paths/plugins/cert/targets/staging/deps/timers e ordem de diagnóstico.

Aceitação: RED→GREEN, contagem de produção separada de tempos, três rodadas CLI
pareadas contra release P5, startup/RSS e controles de invalidação/identidade/
efeitos/Unicode/diagnóstico/assets/fontes/geométricas. Resselo apenas deste owner,
build/suítes pertinentes e lint zero erros; não fechar incrementalidade completa.


## Expressões equivalentes e CI estrito — 2026-10-14

### Medição anterior à decisão

`04_wiring/src/compile.rs` nas linhas [523, 547] contém as expressões apontadas pelo inventário
Clippy. `lab/results/construction/diagnostics/ci-simplificacoes-2026-10-14/before.json.gz`
conserva fonte/owner, spans, HEAD, working tree não commitado, UTC e diffstat;
red.log registra os lints selecionados como erros antes de alterar código.
Os tipos e usos foram conferidos no consumer e no toolchain 1.92.0 pinado.
Esta é manutenção da expressão Rust; não é medição nova de paridade upstream.

### Responsabilidade e aceitação

Nos dois caminhos de gravação de timings, path já é &PathBuf; passá-lo diretamente a std::fs::write. Preservar chamadas, path, JSON, tratamento de erro e sua ordem de publicação/status. Não mudar aquisição ou pipeline.

Correção interna em fluxo contínuo ADR-0127, sem API pública, default, fase,
unsafe, dependência ou supressão de lint novos. L0 precede código e resselo
fica restrito ao owner. Usar os controles funcionais existentes; não alterar
fixtures, expectativas, tolerâncias ou registrar testes que apenas espelhem
esta simplificação. Os lints selecionados devem passar como erros, seguidos
de suíte completa, build, all-targets, rustdoc, fmt, linhagem e inventário total.


## Tipos explícitos e leitura local — type_complexity (2026-10-15)

### Medição anterior à decisão

04_wiring/src/compile.rs:174 (C128).
O snapshot before.json.gz em lab/results/construction/diagnostics/type-complexity-2026-10-15
conserva HEAD, working tree não commitado, UTC, diffstat, fontes/owners e hashes.
O gate red-clippy.log falha com este lint promovido a erro antes da edição.
A inspeção distingue tuplas posicionais, repetição do alias FontKey e contratos
estáticos já explícitos. É manutenção de representação Rust; não medição de
benefício da IA, bug funcional ou paridade nova.

### Responsabilidade e aceitação

Manter o const local ACCESSIBLE como tabela de quatro pares PdfStandard/factory de label existentes, sem novo tipo L4. Autorizar #[expect(clippy::type_complexity, reason = "ACCESSIBLE: composição estática de standards e labels existentes")], somente no const. Preservar entradas, ordem, avaliação lazy da label, mensagem/hint e validação CLI antes de World.

Correção interna em fluxo contínuo ADR-0127: nenhuma entidade, assinatura pública
resolvida, default ou fase é alterada. Uma abreviação de tipo só é permitida com
expansão comprovadamente idêntica; incompatibilidade ou tipo público novo exige
outro gate. As exceções são nominais apenas para type_complexity; não autorizar
supressão em módulo/impl/crate nem elevar limiar. Esta permissão pontual supera
proibições genéricas anteriores somente para os três itens justificados neste lote;
continuam proibidas supressões dos demais casos destinados à correção.
Usar os controles funcionais existentes sem mudar seus esperados. O gate selecionado
deve passar RED→GREEN, incluindo unfulfilled_lint_expectations como erro; revalidar
build, Clippy all-targets, suíte workspace, formato, Rustdoc e linhagem com resselo
restrito. Não criar testes que apenas espelhem aliases ou nomes de campos.


## Transporte de publicação em L3 — V12, 2026-10-16

### Medição anterior à decisão

O snapshot before.json.gz em lab/results/construction/diagnostics/tekt-v12-2026-10-16 conserva HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, árvore não commitada, UTC, fontes, L0, hashes, status e git diff HEAD --stat. O inventário anterior, conferido por hashes, emite 19 V12: um enum produtivo em 04_wiring/src/main.rs:22 e 18 declarações auxiliares de testes. O enum privado Published tem exatamente Html(String) e Bundle(Vec<BundleFile>); compile.rs:90/99/363/539/541 o produz e watch.rs:103/104 despacha os dois braços. O proprietário atual é wiring.md, que autoriza o carrier privado em L4: esta proposta atualiza explicitamente essa decisão, em vez de contrariá-la sem registro. É propriedade/representação Rust, sem nova alegação de paridade ou intenção upstream.

### Verificação comum

T1: preservar os controles reais de watch HTML e bundle HTTP/reload, sucesso/erro, no-serve/no-reload e último snapshot válido; testes p1485_watch_bundle_http_reload_e_preserva_snapshot_apos_erro e p1485_watch_flags_no_reload_no_serve_e_porta_ocupada em 04_wiring/tests/cli.rs, mais controles HTTP de 03_infra/src/watch.rs. T2: preservar compile de HTML/bundle, árvore completa, assets byte-exatos, stdout, warnings, exit e ausência de publicação em erro nos testes CLI existentes, sem editar esperados. T3: V12 RED real antes e zero produtivo depois; conferir por diff que só houve transferência do enum e ajuste do import, sem inferir modalidade por extensão. Verificar build, Clippy workspace/all-targets -D warnings e -D unfulfilled_lint_expectations, suíte workspace, formato, Rustdoc -D warnings e quatro índices, linhagem V1/V5/V15/V26 e reversa, SARIF e diff --check. Não escrever testes que apenas espelhem a sintaxe da mudança de owner. Execuções pesadas sequenciais sob MemoryMax=12 GiB, MemorySwapMax=0 e CARGO_BUILD_JOBS=1. Não publicar release.

### Responsabilidade exclusiva

A assinatura interna continua com Option<Published>, agora resolvido pelo import privado do pai para typst_infra::watch::Published. Conservar produtores Html/Bundle, todas as condições de sucesso/erro, payloads por move, ordem e ausência None. Nenhuma nova conversão, método ou lógica de publicação.
