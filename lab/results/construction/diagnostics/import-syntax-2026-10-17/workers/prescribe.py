from pathlib import Path
import json,gzip,hashlib,subprocess,datetime
out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'));scope=json.load(open(out/'scope.json'));new='01_core/src/compiler/eval/syntax_diagnostics.rs';owner='00_nucleo/prompts/compiler/eval/syntax_diagnostics.md';scope['sources'].append(new);scope['owners'][new]=owner;scope['new_sources']=[new];b['files'][new]='';b['files'][owner]=''
with gzip.open(out/'before.json.gz','wt') as f:json.dump(b,f)
pin='9f2b6e8715237cb086899a42873660fe744622e8';up={}
for p in ['crates/typst-eval/src/lib.rs','crates/typst-eval/src/import.rs']:
 s=subprocess.check_output(['git','show',pin+':'+p],text=True);up[p]=dict(text=s,sha256=hashlib.sha256(s.encode()).hexdigest())
with gzip.open(out/'pinned-source.json.gz','wt') as f:json.dump(dict(pin=pin,files=up),f)
nucleus='00_nucleo/prompts/_nuclei/eval/source-validation.toml';s='''tekt = 1
kind = "nucleus"
id = "eval-source-validation"
title = "Validação de fontes antes do acesso à AST tipada"

[[claims]]
id = "validate-before-eval"
level = "must"
statement = "Fontes de documento, expressão, import e include são validadas sintaticamente antes de visitar nós tipados ou executar o programa. Qualquer erro fatal impede avaliação, efeitos de prefixo, bindings e estilos parciais."

[[claims]]
id = "causal-diagnostics"
level = "must"
statement = "Erros preservam ordem, mensagem, span da fonte real e todos os hints do parser. Não capturar panic como substituto de validação, reancorar tudo no importador nem engolir erro em None."

[[claims]]
id = "warnings-remain-warnings"
level = "must"
statement = "NoTextWithinStars e NoTextWithinUnderscores são avisos não fatais. Preservar seus spans normalizados sobre o markup vazio e hints, mesmo junto de erro; enviar ao Sink existente que deduplica por origem e mensagem. Não generalizar outras falhas para warning."

[[claims]]
id = "static-ownership"
level = "must"
statement = "O coletor puro único classifica SyntaxErrorKind exaustivamente e é restrito a eval. Callers fornecem o canal de warnings, mantêm roteamento/isolamento/contexto e aplicam traces de import/include nas fronteiras existentes. Não introduzir I/O, globais ou API pública."
''';Path(nucleus).write_text(s);effective=hashlib.sha256(s.encode()+b'\0TEKT-NUCLEUS-DEPS-V1\0').hexdigest();scope['nuclei']=[nucleus];(out/'scope.json').write_text(json.dumps(scope,indent=2)+'\n')
responsibilities={
new:'''Criar helper pub(super) fn validate_syntax(root: &SyntaxNode, emit_warning: impl FnMut(SourceDiagnostic)) -> SourceResult<()>. Ele enumera root.errors na ordem, classifica explicitamente as cinco SyntaxErrorKind atuais: NoTextWithinStars→Strong e NoTextWithinUnderscores→Emph para warning; Other/InvalidHexNumber/InvalidUnicodeCodepoint para erro. Normalizar span de warning exatamente como o bloco principal vigente: LinkedNode::find do Error vazio, parent kind esperado e len 2, senão span original. Construir SourceDiagnostic::warning/error com mensagem original e todos os hints em ordem. Emitir warnings pelo callback, coletar todos os erros e retornar Err se houver, Ok caso contrário. Sem catch_unwind produtivo, AST tipada, I/O, contexto global ou reinterpretação de mensagens. Este é o único owner da tradução/classificação comum.''',
'01_core/src/compiler/eval/mod.rs':'''Declarar somente mod syntax_diagnostics e delegar os dois caminhos de validação (documento e expressão) ao helper. Documento usa Sink tracked warn_diagnostic; expressão usa engine.sink.warn_diagnostic dentro do bloco atual. Preservar ordem de validação relativa à execução, warnings de arquivo vazio, parser/Source usados, scopes/features/target, loop único de eval, registro de inputs e todas as assinaturas/campos/traits públicos. Retirar somente imports LinkedNode/SyntaxErrorKind que deixaram de ser usados no hub; manter SyntaxNode/SyntaxKind necessários aos filhos. Não duplicar normalização nem validar expressões com parser de markup.''',
'01_core/src/compiler/eval/modules.rs':'''Import local e pacote: após resolver Source e o gate de ciclo vigente, validar root pelo helper antes de bare_name, composição de stdlib e eval_imported_file; encaminhar warnings ao engine.sink.warn_diagnostic. Não executar prefixo de arquivo inválido nem publicar bindings. Marcar eval_imported_file como fronteira privada que recebe fonte já validada. Ao avaliar arquivo válido, local_ctx herda full_error, features e target do caller, mantendo apply_show_rules false, scopes/estilos/show-rules/guards locais e budgets atuais. Acrescentar Tracepoint::Import(path) no import.span aos erros de validação/avaliação de arquivo, pela mesma ordem causal usada no include; não generalizar trace a imports de objeto/avisos de bare identifier, nem a erros locais de seleção de items. Include string/path: após fonte/ciclo, validar root antes de criar engine filho e antes de qualquer efeito; manter o map_err Include existente, aplicando-o também aos erros de validação. Um helper privado pode reunir validate+eval no bloco para compartilhar o mesmo map_err sem alterar APIs. Preservar World, resolução de pacote, ciclos, binding lexical e demais diagnósticos portugueses. Testes co-localizados via APIs vigentes cobrem erros/hints/origens/traces e ausência de efeitos, avisos, package/path/include aninhado, perfis e target Bundle.'''}
for src,owner in scope['owners'].items():
 p=Path(owner);s=p.read_text() if p.exists() else '# L0 — compiler/eval/syntax_diagnostics\nHash do Código: 00000000\n\nLayer: L1\nExclusive consumer: `01_core/src/compiler/eval/syntax_diagnostics.rs`\n'
 line=f'- {nucleus} sha256:{effective}'
 if 'Núcleos Tekt:\n' in s:
  start=s.index('Núcleos Tekt:\n')+len('Núcleos Tekt:\n');end=s.index('\n\n',start);s=s[:start]+'\n'.join(sorted(s[start:end].splitlines()+[line]))+s[end:]
 else:s=s.replace('\n\n','\n\nNúcleos Tekt:\n'+line+'\n\n',1)
 s+='''\n\n## Fontes importadas inválidas e contexto herdado — 2026-10-17

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

'''+responsibilities[src]+'''

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
''';p.write_text(s)
(out/'l0-record.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),effective=effective,owners={p:hashlib.sha256(Path(p).read_bytes()).hexdigest() for p in scope['owners'].values()}),indent=2)+'\n');print('L0 owners',len(scope['sources']),effective)
