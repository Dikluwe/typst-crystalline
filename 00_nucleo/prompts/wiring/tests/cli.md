# L0 — wiring/tests/cli
Hash do Código: c299a97e

**Layer:** L4
**Exclusive consumer:** `04_wiring/tests/cli.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/expression-source-identity.toml sha256:fb383f227b0bcd8b1e61899942fa0383a4cda66093da58bda2e87844e2895d2a
- 00_nucleo/prompts/_nuclei/smartquote-semantics.toml sha256:dc549d79f9e996433894effc8e4efd1f8f3da3d9c30933110eeabbab582ae9a8
- 00_nucleo/prompts/_nuclei/wiring/cli-observables.toml sha256:0e59744a924f0f6acbe7c4d20db9efc7caed4783c3b8d6b2c2c5a9cace340ece
```

## Current evidence

- `04_wiring/tests/cli.rs:38` — `temp_typ`.
- `04_wiring/tests/cli.rs:46` — `temp_pdf`.
- `04_wiring/tests/cli.rs:54` — `cleanup`.
- `04_wiring/tests/cli.rs:60` — `WatchedChild`.
- `04_wiring/tests/cli.rs:63` — `assert_alive`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 04_wiring/tests/cli.rs`.

## Owned contract

- Own the external CARGO_BIN_EXE_typst integration suite only. Observe exit status, stdout/stderr and artifacts, never import private main modules or implement parsing/compilation/formatting.
- Cover commands/aliases/help/rejections, root/font/input/certificate/document id, eval/query/fonts/info/completions, formatted cross-file/plugin spans, transactional init, formats/features/casts and semantic HTML morphology.
- Success with no warning preserves empty stderr; file output leaves stdout empty. Errors precede experimental format warnings, followed by domain warnings; warning-only compilation retains success; semantic errors and missing physical query input exit 1; other argument/I/O errors 2. Check format signatures and content/DOM rather than complete byte parity.
- Use isolated temporary directories and bounded processes. Watch guards kill and wait on every path; each relevant mutation occurs once. Exercise initial publish, asset change, irrelevant-file filtering, temporary error with last artifact retained and recovery while the child remains alive. Sleeps/retries do not prove internal causal ordering; ArmedWatch integration owns arm/publish/wait.
- Nushell completions run twice deterministically with nonempty script/module/exported command structure and empty stderr in isolated HOME/XDG, producing no files. Preserve Bash/invalid-shell controls and public w alias; accept only the existing singular/plural help annotation spellings.
- Preserve canonical repr fields/order and semantic HTML feature/target separation, void/nested/escaped/whitespace cases. Default PNG must finish within memory/time bounds and produce A4 IHDR 1191 by 1684 at 144 PPI, never pass by OOM or absent output.
- Bundle cases cover binary assets, nested mixed-format documents, normalized duplicates, invalid paths, loose content and feature absence. Controlled watch/HTTP tests cover no_serve/no_reload/port, 200/301/404, MIME, raw asset bytes, response-only HTML reload script, SSE and last-snapshot preservation. Leave no daemon or child process.

## Verification and boundary

Run `cargo test -p typst --test cli -- --test-threads=1` with the harness prerequisites above.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

## Canary de warnings vigente — 2026-10-06

outer-sealed.log registra cli_sucesso_com_warning e disciplina_warnings_antes_de_errors falhando porque hyphenate já é consumido desde a correção anterior de paridade. A sonda warning-control.typ em typst-unsealed emite no text within stars para ** e conserva exit 0 com conteúdo. Migrar somente essas fixtures de warning para **, mantendo asserções de saída, arquivo, stderr e ordem warning/error; não reintroduzir warning de propriedade válida nem enfraquecer o teste. A ausência de permissão de loopback em dois testes watch é limitação do sandbox, não fixture descartada.

## Caminhos de diagnóstico — 2026-10-06

Medição: diagnostic-paths-2026-10-06/before.json compara unknown em arquivo fora da cwd: a fonte correta e seu span coincidem, mas o caminho Tekt é absoluto e o vanilla é relativo com .. . Acrescentar testes black-box com cwd/arquivos isolados: input principal absoluto e relativo, import com erro e trace, query e eval contextual transiente, filho da cwd e nomes com espaço/Unicode. Validar status, fonte referida e posição/hints pertinentes sem projetar caminhos nem enfraquecer canaries anteriores. Não chamar o helper candidato para obter a expectativa. A igualdade completa bilateral pertence à coleta reproduzível.

A saída medida do pin para broken confirma que o diagnóstico primário usa ┌─ lib.typ:1:21, mas o trace usa while calling broken at main.typ:2:1 em linha própria. green.log conserva a falha da expectativa inicial que exigia ┌─ também no trace; corrigir essa expectativa para o formato real, sem eliminar a verificação do segundo arquivo, linha ou chamada.

measurement.json.gz amplia a coleta e refuta atribuir a expressão de eval ao nome do documento: o oracle usa <input-expression>. Este é problema de origem, além do cálculo relativo de um arquivo físico. Retirar a expectativa inicial incorreta de nome real para a expressão do teste de caminhos físicos; conservar o caso adversarial integral como DIFF na coleta, para a frente de identidade de Source. O teste físico cobre compile/query e trace, sem certificar nomes transientes. A ordem de warnings de query/compile e o suporte a compile stdin também permanecem diferenças medidas, fora deste reparo do cálculo de caminhos.

## Identidade de documento e expressão — gate confirmado 2026-10-06

### Evidência anterior à decisão

`00_nucleo/diagnosticos/diagnostic-paths-2026-10-06/identity-next-audit.json`
conserva comandos, fontes, UTC, HEAD, árvore alterada e hashes dos binários do
Tekt e do vanilla pinado 9f2b6e8715237cb086899a42873660fe744622e8. Os casos
separam erro/warning do documento de erro da expressão: o documento perde sua
fonte, enquanto a expressão recebe indevidamente o nome do documento.
`01_core/src/compiler/eval/mod.rs:609-615` usa world.main() para Source e base;
`04_wiring/src/eval.rs:66-95` repete esse id ao fornecer o primary ao formatter.
`03_infra/src/world/cache.rs:58-78` registra arquivos e recicla o contador;
`lab/typst-original/crates/typst-cli/src/eval.rs:159-179` e
`lab/typst-original/crates/typst-cli/src/world.rs:176-190` distinguem expressão,
stdin e ausência de entrada por identidade. A igualdade dos inteiros e o
interner global upstream são mecânica, não requisito (ADR-0107). A origem
mostrada, fonte, posição e resolução relativa são observáveis de linguagem.

### Decisão e escopo do gate

Acrescentar regressões black-box de eval --in: erro do documento com trecho
e posição reais; warning do documento e erro da expressão sem troca de fontes;
expressão inválida com <input-expression>; documento stdin com <stdin>; import
com erro/trace e callback contextual originado no documento; read relativo com
cwd diferente da pasta do input e fonte Unicode. Não projetar nomes ou spans.
O caret curto do warning empty-stars é débito independente e não deve ser
reclassificado como resolvido por esta correção de identidade.

Gate ADR-0127 confirmado pelo dono em 2026-10-06 (resposta: confirmo), conforme
00_nucleo/materialization/origem-fontes-expressao-aprovacao-2026-10-06.json.
Consumer exclusivo preservado; Hash do Código acompanha a materialização.

## Caret de empty markup — 2026-10-06

Após a coleta before.json em empty-markup-warning-2026-10-06, exigir caret de
dois caracteres para ** e __ em eval --in com expressão válida e com erro
posterior. Conservar caminho do documento, <input-expression>, hints e códigos
de saída. A correção é do alcance publicado por eval; os dois negativos da
coleta de identidade devem passar sem projetar stderr. Isto não certifica
ordem de warnings em compile/query.

## Compilação de stdin com saída explícita — 2026-10-06

### Medição anterior à decisão

`00_nucleo/diagnosticos/compile-stdin-2026-10-06/before.json.gz` conserva
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

## Output obrigatório para stdin — 2026-10-06

### Medição antes da decisão

`00_nucleo/diagnosticos/plano-debitos-2026-10-06/before.json.gz` conserva
HEAD/working tree/UTC, fontes, comandos e hashes dos binários. Compile/c -
sem output aceita e escreve -.pdf no Tekt; o pin exige OUTPUT e sai 2.
`lab/typst-original/crates/typst-cli/src/args.rs:351-355` declara
required_if_eq(input, -), comprovando intenção. `02_shell/src/cli.rs:922-935`
deriva output mesmo nesse caso. O stdin implícito é extensão já estabelecida
do Tekt: vanilla rejeita o subcomando -, e essa forma não é sonda de paridade.

### Contrato e aceitação

Antes de resolver formato/output ou construir RunIntent, input exatamente -
sem output positional nem -o/--output sai 2 com corpo de diagnóstico
`the following required arguments were not provided:\n  <OUTPUT>`.
Usar a árvore clap existente para apresentar usage/help, sem fabricar opções
vanilla ausentes no Tekt. Nenhum stdin, World ou arquivo é adquirido em L2.
Isso inclui c, compile e a forma implícita já suportada; vale nos formatos
pdf/svg/html/bundle. Output positional ou flag continua suficiente; a flag
vence positional na mesma ordem atual. Entrada física continua derivando
output quando ausente. Não alterar assinaturas, flags ou helpers públicos.

Testar black-box rejeição sem -.pdf/árvore bundle e stdout vazio; aceitação
positional/flag/stdout e prioridade flag em ambas ordens; preservar o default
para entrada física. Correção de paridade ADR-0127 em fluxo contínuo.
Usage difere legitimamente pela superfície de flags já existente; conservar
stderr bruto na coleta, sem declarar identidade integral desse envelope.

## Seletor nativo de math.equation — 2026-10-06

### Evidência antes da decisão

`00_nucleo/diagnosticos/plano-debitos-2026-10-06/math-before.json.gz`
conserva pin/HEAD/árvore/UTC/fontes/comandos/SHA. Equação explícita é locatable,
mas query(math.equation), alias e selector(math.equation) são rejeitados.
`01_core/src/compiler/stdlib/foundations/selector.rs:269-283` reconhece apenas
heading/figure/table/metadata/outline.entry por identidade estática e omite
o constructor já existente de equation. `structural/math.rs:97-103` registra
o mesmo constructor nativo no módulo público; o índice já trata Equation.
O pin devolve a equação nesses controles. ElementKind::Equation já existe;
não é necessário ampliar enums ou reconstruir a biblioteca para reconhecer.

### Decisão e aceitação

O hub estrutural expõe somente um bridge interno pub(super) para
math::equation::native_math_equation, com acesso limitado ao subtree stdlib;
não ampliar visibilidade de módulos, API exportada de compiler::stdlib ou
assinaturas. O owner selector acrescenta a função à tabela estática de
ElementKind::Equation usando fn_addr_eq, mantendo alias pela função real.
Não usar nome do Func ou string math.equation como identidade. Não executar
constructors, World ou make_math_module em produção para descobrir binding.

Testar parse_selector_arg/native_selector, alias nativo com nome diferente e
func diferente com nome math.equation. CLI deve observar número de equações e
ordem/fields dos elementos já existentes. Não envolver matemática bare em
Equation, alterar fase, promover todas as funções ou alegar correção da
morfologia de matriz. .where é responsabilidade de element_where e fica
fora deste lote; table.cell também permanece independente. Correção interna
de mapeamento/paridade em fluxo contínuo ADR-0127.

### Envelope de usage do erro — refinamento medido

final.json.gz registra Usage: compile [OPTIONS] [INPUT] [OUTPUT] no primeiro
reparo. Essa apresentação perde o nome real do executável e indica opcionais
que o erro exige. Construir o erro a partir de clone do subcomando existente,
com bin_name typst compile e input/output required apenas nessa árvore de
apresentação do erro. Não alterar a árvore usada para parse/help/completions
ou aceitar/rejeitar novas formas. Exigir Usage: typst compile e <INPUT> <OUTPUT>.
As opções suportadas continuam as do Tekt; não fabricar creation-timestamp do
oracle nem declarar stderr integralmente igual enquanto o usage divergir.

## Regressores da ordem/hint de warnings — gate confirmado ADR-0127

before.json.gz/plano-debitos-2026-10-06 conserva stderr bruto e refuta a ordem
warnings → errors e hint genérico. O gate foi confirmado; atualizar a sentinela
histórica disciplina_warnings_antes_de_errors para a ordem medida, preservando
as verificações de ambos diagnósticos, conteúdo, status e arquivo.
Acrescentar casos query clean/warning/error/warning+error, hint com field/one,
identifier/campo especial, alias, selector com quotes/Unicode, erro de
cardinalidade e stdin com erro/trace/import. Medir stdout/stderr/status contra
pin sem projetar partes divergentes. Testes puros L2 cobrem a função pública,
escapes e ausência de I/O. Não exigir aceitação de cell na query: o oracle
rejeita cell não locatable; show/selector são eixos separados.

Gate confirmado pelo humano em 2026-10-06 (resposta confirmo), com candidatos
e hashes preservados em 00_nucleo/materialization/warnings-cli-aprovacao-2026-10-06.json.

Controle HTML: errors precedem aviso experimental, que precede aviso de strong vazio;
em sucesso manter experimental antes do aviso de strong vazio (before.json.gz).

Paths de diagnósticos permanecem relativos ao cwd; o hint de query usa input.display()
como medido no vanilla, incluindo caminho absoluto fornecido. O teste de path deve
examinar o diagnóstico, sem proibir o caminho deliberadamente presente no hint.

## Entrada query inexistente

before.json.gz em query-residuos-2026-10-06 mede relativa, ./ e absoluta.
Testar stdout vazio, exit 1 e stderr integral de input file not found, sem
aviso de depreciação. Guardar cwd/argumento original na expectativa; remover
árvore temporária. Não modificar a expectativa de erro de compile.


## Query CLI compilada — gate ADR-0127 confirmado

### Medição anterior à decisão

query-residuos-2026-10-06/before.json.gz registra HEAD/working tree, UTC,
pin/binary hashes, fontes, comandos e stdout/stderr/status. query-stdin-callback
retorna sucesso onde vanilla reporta erro e trace. query-context-success perde
metadata produzido; nested-error também desaparece. query-selector-quotes
apresenta rejeição P206C antes do erro do documento. where-level é rejeitado,
mas vanilla retorna [2]. Document-error-invalid-selector demonstra precedência
da compilação; valid-document-invalid/text-selector demonstra retrieval error
sem depreciação. eval-context-reference e eval-where-reference já coincidem
usando a compilação paginada atual. Upstream typst-cli/src/query.rs:34–66
compila antes de retrieve; :86–105 avalia selector em Scope default e depois
converte para LocatableSelector. Fontes locais: query_helpers.rs:488–522 consulta
somente Module; pipeline.rs:182–214 já usa compilação com introspector final.

### Decisão confirmada

Correção da linguagem, não cópia da mecânica upstream. CLI query passa a
compilar documento paginado completo antes da avaliação de selector, sem
exportar arquivo. A execução comum do programa permanece única, conforme os
owners vigentes de realização/contexto. Reutilizar o pipeline paged completo:
nenhuma expansão avulsa, índice pré-contexto ou ciclo de estabilização paralelo.
Distinguir falha de compilação e de retrieval por enum explícito, nunca por
mensagens, spans destacados ou busca de substrings. A nova API e o caminho de query compilada foram confirmados pelo humano
em 2026-10-06 (confirmo); recibo query-compilada-aprovacao-2026-10-06.json.
Os candidatos/manifesto permanecem como recibo histórico; este é o L0 vigente.

Testes RED antes de materializar: os três resíduos originais, callback válido
com metadata e aninhado com erro, .where(level), .where(body com apóstrofo/Unicode),
documento inválido com selector inválido, documento válido com selector inválido
ou texto, label e math.equation. Compilação inválida precede selector e recebe
warnings/depreciação; retrieval inválido não recebe nenhum dos dois. Verificar
stdout/status/stderr e morfologia de Content, sem igualdade Rust ou bytes PDF.
Preservar testes anteriores de stdout/cardinality/field/path/import/argv.
Não ajustar critérios para aceitar perda de campos/Source ou programa não executado.

## Limite medido de aspas no corpo — scope-out independente

refined-debug.json.gz em query-compilada-2026-10-06 compara query-where-body e
**eval-where-body-reference**, que não usa o novo helper. Ambos devolvem o mesmo
corpo completo no Tekt; vanilla preserva smartquote(double:false), enquanto o
Tekt já realiza a aspa em Text("’") em markup_stream.rs:125. A filtragem encontra
um elemento correto; não há perda de campo causada pela nova consulta. O RED
que exigia smartquote em query mediu um débito adicional da construção/serialização
de Content, não transporte de fields desta mudança. Registrar este negativo
separadamente, sem declarar igualdade morfológica de aspas ou corrigir glyph
por nome no shell. Alterar a fase/representação de aspas exige o L0 proprietário
e medição própria; fora deste pacote. A aceitação de selector with body nesta
frente exige cardinalidade correta, corpo sequence completo, textos adjacentes
e identidade de resultado com a consulta equivalente em eval --in. Os critérios
de Source, campos transportados e todos os três controles originais permanecem.


## Aspa semântica e resolução tardia — ratificada em 2026-10-06

O contrato desta seção é vigente e supersede as cláusulas históricas conflitantes
das seções anteriores apenas para SmartQuote. A ratificação humana autoriza a
materialização em fluxo RED→GREEN, com ownership exclusivo e pins abaixo.

### Medição anterior à decisão

smartquote-2026-10-06/{before,fields-before,partial-and-html,proposal-controls}.json.gz
registra pin 9f2b6e8715237cb086899a42873660fe744622e8, SHA-256 dos binários,
HEAD/working tree, diff HEAD --stat, UTC, fontes, argv/cwd/ambiente e stdout/stderr.
PDF usa texto extraído, HTML usa DOM/texto e diagnósticos, não igualdade de bytes PDF.
A avaliação converte aspas de markup para Text; campos double/enabled são perdidos.
A variável contendo aspas mantém glyph inglês sob uso em alemão/francês ou
smartquote(enabled:false). Primes 6'/2" viram aspas no Tekt. Show da função
nativa e nome público foram corrigidos separadamente e passam antes deste gate;
markup ainda não é SmartQuote. Constructor vazio tem fields vazio no vanilla;
double:false é campo conhecido, mas double omitido é Unset. enabled:false
permanece SmartQuote no vanilla. set double:false e show filtrado têm positivos
novos. HTML literal funciona porque recebe Text; SmartQuote programática hoje
é rejeitada. Título literal renderiza aspas; script recebe string, rejeitando
Content antes da exportação nos dois binários.

Fontes: markup_stream.rs:79–128 decide glyph em eval; smartquote.rs de layout
alterna dois flags sem texto precedente; stdlib/text/smartquote.rs:112–117 apaga
elemento disabled; observation/elements.rs:242–250 compara os campos atuais.
Vanilla typst-eval/markup.rs:141–146 cria elemento com double explícito;
typst-layout/inline/collect.rs:196–211 resolve estilo/contexto na coleta;
typst-library/text/smartquote.rs:119–158 distingue prime/apóstrofo/nesting.

### Decisão ratificada e limites

Preservar identidade/campos de SmartQuote em eval e resolver glyph só na
composição de render de cada target. Paged usa sua unidade; HTML permanece
semântico e não chama layout paginado para aspas (ADR-0128). A lógica de
seleção contextual é helper puro L1 único, com estado por fluxo/compilação,
sem I/O, globais, vtable, registry de propriedades ou cópia do algoritmo de
memoização upstream. Stack/estado é mecânica escolhida do Tekt; linguagem,
campo conhecido/omitido, glyph e estilo são aceitação. Escaping HTML permanece
com seus owners. Sem novos modos/flags, unsafe, API de watch ou math phase.

Este gate muda double público bool→Option<bool>, adiciona enabled:Option<bool>,
expõe helper/contexto puro e move escolha de glyph eval→render. Confirmação humana recebida em 2026-10-06. O contrato ratificado abaixo
supersede cláusulas históricas conflitantes apenas para aspas; ADRs/núcleos
existentes e propostas alheias permanecem intactos.

Testes CLI bilaterais de query/eval body com smartquote double verdadeiro/falso,
fields vazio/presente e getter Unknown versus false, reuse Lang/disabled,
show smartquote/alias e .where(double), PDF/HTML/default/title, primes/wrap.
Original matriz query 23/23 permanece íntegra. Controles invalid script/style
ou recursão show text são rejeições, não testemunhos positivos de quote.
Não usar PDF bytes como paridade ou mudar Fonte/SOURCE_DATE_EPOCH por conveniência.


## Fronteira do harness — V12, 2026-10-16

### Medição anterior à decisão

O snapshot before.json.gz de 00_nucleo/diagnosticos/tekt-v12-2026-10-16 conserva HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, árvore não commitada, UTC, fontes/L0/configuração/lock, hashes, status e git diff HEAD --stat. Os 2077 hashes coincidem com o checkpoint final da etapa FrameItem: a suíte anterior passou com 8622 testes, zero falhas e sete ignorados nesse mesmo estado. gate-baseline-result.json confirma RED dos 19 V12 antes de qualquer mudança de L0 ou código. items.json contém os itens AST completos e classification.json registra a separação por unidade. Cláusulas pertinentes dos owners foram lidas junto dos helpers; não há nova alegação de paridade ou intenção upstream.

O linter identifica enum e impl inerente de topo em L4, inclusive suporte de teste. A separação aqui é de responsabilidade: cenários, fixtures de linguagem e asserções de produto ficam em L4; helpers independentes de subprocesso/filesystem/observação passam a uma unidade de suporte L3. Não mover a suíte completa, encaixar declarações em módulo L4 para explorar a leitura rasa do parser, excluir testes do lint, converter impl em funções só para escapar da regra ou alterar severidade.

### Responsabilidade exclusiva

Transferir os tipos WatchedChild, seus impls e Drop existentes para `03_infra/tests/support/wiring/cli.rs`, owner `00_nucleo/prompts/infra/test-support/wiring/cli.md`. Conservar em `04_wiring/tests/cli.rs` todos os cenários e observações finais. Importar somente WatchedChild por módulo harness com #[path] estático para L3. Não usar esse módulo na biblioteca de produção. Os corpos anteriores desta transferência estão medidos em: 04_wiring/tests/cli.rs:60 struct_item WatchedChild; 04_wiring/tests/cli.rs:62 impl_item WatchedChild; 04_wiring/tests/cli.rs:76 impl_item WatchedChild.

### Aceitação e limites

Esta separação de harness é interna ADR-0127: nenhum código de suporte integra a biblioteca L3 ou a CLI de produção e pub(super) só permite acesso no crate de teste. Nenhuma API pública de produto, dependência, flag, default ou fase muda. O contrato público Published é proposta separada, já ratificada, e não autoriza outros contratos. Não adicionar derives, supressões, hooks, nova política de cleanup, tolerância ou fallback de verificação.

Preservar todos os corpos dos itens movidos, salvo a visibilidade mínima ao módulo de teste pai. Campos de Checks permanecem privados. Imports std devem ser próprios de L3, sem use super::* ou dependência de lógica L4. BIN, invocação dos subprocessos, opções, cenários, esperados e #[test] permanecem em L4; o helper não lê CARGO_BIN_EXE_typst. Não consolidar diferenças legítimas entre harnesses (prefixos, mensagens, atomicidade de criação e cleanup).

Conferir replay restrito das transferências, funções de teste idênticas, atributos/expectativas sem mudanças, grafo 1:1 válido, ausência do suporte na biblioteca produtiva, V12 GREEN zero e SARIF completo sem avisos novos. Revalidar suíte workspace sob TMPDIR explícito em /dev/shm e oráculo vanilla pinado 9f2b6e8715237cb086899a42873660fe744622e8 com binário SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea; cargo build, Clippy workspace/all-targets -D warnings e -D unfulfilled_lint_expectations, formato, Rustdoc -D warnings com quatro índices, V1/V5/V15/V26 estrito e linhagem reversa, diff --check. L0 primeiro; resselo restrito aos owners e código autorizado. Jobs pesados sequenciais: MemoryMax=12 GiB, MemorySwapMax=0, CARGO_BUILD_JOBS=1. Fontes/L0/configuração congelados durante comandos. Não escrever testes de espelho da sintaxe de import nem alegar ganho de RAM/IA não medido. Sem release.
