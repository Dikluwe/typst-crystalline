# L0 — compiler/eval/modules
Hash do Código: 2904def5

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/builtin-module-identity.toml sha256:2b8888460a1e6008abda0ff71603fe4dace79ff0a2a7aad598f8aabfab3e3776
- 00_nucleo/prompts/_nuclei/eval/core.toml sha256:e7642a709c937928333439b2a78cdb3a6dbd6b56d67fcc67728efd2a26796e58
- 00_nucleo/prompts/_nuclei/eval/source-validation.toml sha256:e29edf5bc71d0c7f8b45d0a2a0db43820151a7d80f7484197c56dc8b3830d774

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/modules.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/modules.rs:44` — `fn eval_imported_file`.
- `01_core/src/compiler/eval/modules.rs:117` — `fn eval_module_import`.
- `01_core/src/compiler/eval/modules.rs:251` — `fn eval_module_include`.

Prior source readings supporting these clauses (snapshot provenance is
retained in the assigned history diagnostic, not asserted as a fresh measurement):

- `lab/typst-original/crates/typst-eval/src/import.rs:82-103`.
- `lab/typst-original/crates/typst-eval/src/import.rs:82-105`.
- `typst-library/src/lib.rs:374`.

## Contract

Own imported-file evaluation, module import and include. Obtain sources and
package resolution only through World; preserve current FileId, Route cycle
detection, isolated Engine/scopes, exported bindings, sink and diagnostic traces.
Construct the imported global stdlib object with public name global under lexical
binding std. Ordinary imported modules retain their own stored names.

A bare Module expression uses the syntactic import binding name (identifier,
field or literal file-derived name), not Module::name(). Dynamic source without
as/items fails with `dynamic import requires an explicit name` and
``you can name the import with `as` ``. Dynamic as/items/wildcard imports remain
valid. Wildcards copy exported bindings; item aliases preserve missing-item
diagnostics at the original-name span. Include keeps string/path routes and
causal traces without changing caller bindings into imported exports.

After successful source validation, a bare identifier import with neither as
nor items emits exactly one `this import has no effect` at the source identifier,
without hints or trace. Preserve it even if a later expression fails. Field,
file, alias, items, wildcard, dynamic and failed imports do not acquire this
warning by generalization. Keep existing rename-warning limitations explicit
rather than broadening the warning contract. Existing Portuguese diagnostic
literals in the consumer are observables, not prose to translate.


## Gate HTML e identidade de módulos — 2026-10-17

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
00_nucleo/diagnosticos/html-feature-identity-2026-10-17/before.json.gz registra
UTC, status, diff HEAD --stat, fontes e L0 completos e hashes; todos os 2123
hashes da etapa anterior foram conferidos antes de reutilizar sua matriz.
before-probe.json.gz conserva argv, fixtures, outputs e SHA-256 dos binários.
O vanilla é o pin 9f2b6e8715237cb086899a42873660fe744622e8 e o binário
7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea.
fields.rs:39–52 só aplica o gate html a format, não a std; identity.rs:84–101
reconhece módulos por nomes/bindings. As sondas com import correto refutam
paridade: std.html direto/alias/closure/reexport aceita Html desligada;
global.typ com type/format/document e asset comum recebe gate indevido;
format.typ com funções svg/png/pdf/html recebe gates html/bundle indevidos.
A tentativa inicial de sonda com let m = import foi inválida (import retorna
none) e está arquivada em candidate-probe-syntax; não conta como evidência.
A fonte upstream pinada module.rs:138–154 lê o guard do binding e code.rs:
347–368 ancora no field. Isto comprova semântica/diagnóstico, não exige
replicar BindingGuard ou endereços upstream. A instrução vigente no L0 pai
field_access.md:61–62 proíbe identificar std comparando bindings.

### Decisão e responsabilidade exclusiva

Na construção isolada da stdlib de arquivo importado usar Module::stdlib(stdlib.clone()). Módulos exportados de arquivo continuam Module::new com nome do arquivo e sem autoridade de std/format. Preservar resolução World, warnings, content e semântica de import/include.

Consumir o núcleo canônico pinado builtin-module-identity.toml. O marcador é
mecânica privada para cumprir gates vigentes, não novo contrato público;
ADR-0127 permite fluxo contínuo de correção/paridade. A API pública de Module
continua opaca, sem novos campos/métodos exportados; comparar declarações e
assinaturas antes/depois. Nenhuma alteração pública/fase/default é autorizada.

### Aceitação e limites

RED funcional antes da implementação; GREEN dos gates, homônimos, aliases,
closures e imports nos oito perfis; mensagem/hints/span e precedência causal.
Não atribuir crédito MATCH/Preserved de feature ativa a casos desligados:
classificar separadamente rejeição de feature e aceitação com feature ativa.
Conservar divergência prévia de igualdade entre std de arquivos distintos;
esta etapa não altera Eq de Module nem declara paridade global.
Testar a matriz anterior de caminhos std.html desligados e controles ativados;
sem publicar, commit ou release. Build, suíte workspace, Clippy all-targets e
Rustdoc estritos, fmt, V1/V5/V15/V26, reverse e diferencial global sem novo
erro/aviso. Somente resselo dos seis owners após validar ownership/núcleos.
Comandos pesados sequenciais com MemoryMax12GiB/High10GiB/swap0/jobs1.


## Continuação causal do gate — chamadas e set rules

Medição anterior à decisão: candidate-1/validation-green.log registra quatro
controles aprovados e falha real do teste de argumentos: std.html(panic(...))
executa panic antes do gate. before-set-probe.json.gz registra UTC, hashes,
fontes e saídas bilaterais no mesmo HEAD/árvore não commitada: std.html,
format.html, std.asset e format.bundle chamados avaliam argumentos antes do
gate; set std.html/std.format.html/alias apaga o erro do target e ignora os
argumentos. Variável/field ausente em set também é aceito indevidamente;
condition false conserva atalhamento. Html ativo em set por alias ignora panic
no Tekt e o executa no pin. São ordem causal, diagnóstico e efeito da linguagem.
collection_methods.rs:36 avalia argumentos antes de autenticar módulo; set.rs:
44 usa ok() na resolução e :326 autentica HTML pela sintaxe literal.

Conservar a responsabilidade anterior; fields extrai os três gates para check_module_field_feature(&Value, &str, Span, Features) -> SourceResult<()>, interno a eval, e o chama antes do lookup. Os demais owners não duplicam os diagnósticos nem regras.

L0 precede esta continuação; correção interna/paridade em fluxo ADR-0127.
O núcleo builtin-module-identity continua pinado e sem alteração. O contrato
público e os campos externos permanecem iguais; nenhum fallback novo ou
supressão. Os controles e a matriz dos oito perfis permanecem; acrescentar
verificação bilateral dos efeitos e de set com/sem feature. A suite completa
regressa após o source freeze novo, sem reaproveitar gates de fonte alterada.


## Fontes importadas inválidas e contexto herdado — 2026-10-17

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
00_nucleo/diagnosticos/import-syntax-2026-10-17/before.json.gz registra UTC,
status, diff HEAD --stat, fontes/L0 e hashes; 2125 hashes do fechamento anterior
foram conferidos. before-probe.json.gz guarda as sondas bilaterais com fixtures,
argv, saídas e binários; before-context-probe mede oito perfis e bundle real.
Vanilla pinado 9f2b6e8715237cb086899a42873660fe744622e8, binário SHA-256
7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea.
modules.rs:108 entra em eval_markup sem validar root; include repete o acesso
na linha 324. mod.rs:649/778 valida nas entradas principais, distinguindo
avisos. Reserved/mixed/nested encerram com panic 101 em ast/mod.rs:94, enquanto
o pin retorna erro sintático na fonte real. Strings/expressão faltantes podem
ser aceitas; prefixo panic executa antes do erro sintático; avisos do módulo são
perdidos. Fonte upstream lib.rs:71–82/128–137 valida errors_and_warnings antes
de avaliar; import.rs:36–47 registra trace de arquivo. Não se infere intenção
upstream por essa mecânica: a aceitação é ausência de crash/efeitos indevidos e
diagnóstico causal com spans/hints/traces.

modules.rs:104–106 herda só full_error: Html/A11yExtras/Bundle ativos falham em
import com mensagem de feature desligada, e documento construído em módulo no
target Bundle falha por contexto Paged. before-context-probe inclui bundle que
exporta child.html no pin e falha no Tekt; é erro de propagação, não novo target.
A tentativa anterior com format:format.html fica arquivada em
context-format-function-attempt: esse argumento tem divergência de contrato
separada, não é prova de target; a fixture válida usa inferência .html/auto.

### Decisão e responsabilidade exclusiva

Import local e pacote: após resolver Source e o gate de ciclo vigente, validar root pelo helper antes de bare_name, composição de stdlib e eval_imported_file; encaminhar warnings ao engine.sink.warn_diagnostic. Não executar prefixo de arquivo inválido nem publicar bindings. Marcar eval_imported_file como fronteira privada que recebe fonte já validada. Ao avaliar arquivo válido, local_ctx herda full_error, features e target do caller, mantendo apply_show_rules false, scopes/estilos/show-rules/guards locais e budgets atuais. Acrescentar Tracepoint::Import(path) no import.span aos erros de validação/avaliação de arquivo, pela mesma ordem causal usada no include; não generalizar trace a imports de objeto/avisos de bare identifier, nem a erros locais de seleção de items. Include string/path: após fonte/ciclo, validar root antes de criar engine filho e antes de qualquer efeito; manter o map_err Include existente, aplicando-o também aos erros de validação. Um helper privado pode reunir validate+eval no bloco para compartilhar o mesmo map_err sem alterar APIs. Preservar World, resolução de pacote, ciclos, binding lexical e demais diagnósticos portugueses. Testes co-localizados via APIs vigentes cobrem erros/hints/origens/traces e ausência de efeitos, avisos, package/path/include aninhado, perfis e target Bundle.

Correção interna/paridade em fluxo ADR-0127, com L0 antes do código. Não muda
contrato Rust público, defaults, flags, tipos de SourceDiagnostic/Tracepoint,
World, Sink ou fase eval/layout. O helper/closure são estáticos e privados;
o núcleo pinado source-validation compartilha a política entre estes owners.

### Aceitação e limites

RED funcional antes da implementação, sem catch produtivo; testes podem usar
catch_unwind somente para transformar o crash antigo em falha legível. GREEN
exige todos os erros antes dos efeitos, avisos não fatais e dedup estável,
proveniência da fonte importada e traces aninhados. Package por World em memória,
include com RootedPath, todos os perfis de features e documento em Bundle são
controles obrigatórios. Preservar controles de main/expressão/arquivo válido e
corpus já congelado; não atribuir crédito de feature ativa a rejeições quando
o perfil a desliga. Probes incluem stderr inteiro e HTML sem normalizar bytes
para declarar paridade; diferenças preexistentes de markup/diagnóstico não
ficam silenciosamente equivalentes. Parser, morfologia de include, contrato
de document.format e importações de categorias ainda não suportadas permanecem
frentes próprias se não forem corrigidas e medidas neste lote.

Validar ownership/V26 antes de resselo restrito. Build, testes workspace,
Clippy all-targets -D warnings/-D unfulfilled_lint_expectations, Rustdoc estrito,
fmt, V1/V5/V15/V26 e reversa, SARIF diferencial sem erros/avisos novos. A extração
pode mover a nota V19 uniforme dos três tipos de erro para o owner novo; conservar
o match exaustivo e sua justificação. Comandos pesados sequenciais sob12GiB,
high10GiB/swap0/jobs1. Sem commit/push/merge/release e sem supressões de lint.
