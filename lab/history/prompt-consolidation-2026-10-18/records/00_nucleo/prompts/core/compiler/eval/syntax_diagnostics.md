# L0 — compiler/eval/syntax_diagnostics
Hash do Código: 40dbd115

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/source-validation.toml sha256:e29edf5bc71d0c7f8b45d0a2a0db43820151a7d80f7484197c56dc8b3830d774

Layer: L1
Exclusive consumer: `01_core/src/compiler/eval/syntax_diagnostics.rs`


## Fontes importadas inválidas e contexto herdado — 2026-10-17

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
lab/results/construction/diagnostics/import-syntax-2026-10-17/before.json.gz registra UTC,
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

Criar helper pub(super) fn validate_syntax(root: &SyntaxNode, emit_warning: impl FnMut(SourceDiagnostic)) -> SourceResult<()>. Ele enumera root.errors na ordem, classifica explicitamente as cinco SyntaxErrorKind atuais: NoTextWithinStars→Strong e NoTextWithinUnderscores→Emph para warning; Other/InvalidHexNumber/InvalidUnicodeCodepoint para erro. Normalizar span de warning exatamente como o bloco principal vigente: LinkedNode::find do Error vazio, parent kind esperado e len 2, senão span original. Construir SourceDiagnostic::warning/error com mensagem original e todos os hints em ordem. Emitir warnings pelo callback, coletar todos os erros e retornar Err se houver, Ok caso contrário. Sem catch_unwind produtivo, AST tipada, I/O, contexto global ou reinterpretação de mensagens. Este é o único owner da tradução/classificação comum.

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
