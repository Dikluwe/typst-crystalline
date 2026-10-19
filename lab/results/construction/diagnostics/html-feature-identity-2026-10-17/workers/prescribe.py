from pathlib import Path
import gzip,json,hashlib,re,datetime,subprocess
out=Path(__file__).resolve().parents[1];scope=json.load(open(out/'scope.json'));b=json.load(gzip.open(out/'before.json.gz'));assert all(Path(p).read_text()==t for p,t in b['files'].items())
pin='9f2b6e8715237cb086899a42873660fe744622e8';up={}
for p in ['crates/typst-library/src/foundations/module.rs','crates/typst-eval/src/code.rs']:
 t=subprocess.check_output(['git','show',pin+':'+p],text=True);up[p]=dict(text=t,sha256=hashlib.sha256(t.encode()).hexdigest())
with gzip.open(out/'pinned-source.json.gz','wt') as f:json.dump(dict(pin=pin,files=up),f)
nucleus='00_nucleo/prompts/_nuclei/builtin-module-identity.toml';text='''tekt = 1
kind = "nucleus"
id = "builtin-module-identity"
title = "Identidade interna dos módulos std e format"

[[claims]]
id = "trusted-origin"
level = "must"
statement = "A composição nativa atribui identidade interna Std ou Format; Module::new produz módulo comum para qualquer nome e bindings. Nenhum nome local, scope semelhante ou conteúdo de usuário autentica módulo nativo."

[[claims]]
id = "alias-lifetime"
level = "must"
statement = "A identidade acompanha o Module clonado em aliases, closures, imports e reexports e independe de lookup posterior do nome std; igualdade Arc, debug, nomes e API pública existentes são preservados."

[[claims]]
id = "feature-boundary"
level = "must"
statement = "No acesso AST, std.html e format.html exigem Html, std.asset e format.bundle exigem Bundle, inclusive aliases. Módulos e dicionários comuns homônimos não recebem estes gates. O diagnóstico precede lookup e argumentos, usa span do field e mantém mensagem e hints existentes. Target não ativa feature."

[[claims]]
id = "private-mechanics"
level = "must-not"
statement = "A marca interna altera assinaturas/campos públicos, igualdade de Module, mutabilidade, wrappers de função, fases, defaults ou políticas de outros namespaces."
''';Path(nucleus).write_text(text);effective=hashlib.sha256(text.encode()+b'\0TEKT-NUCLEUS-DEPS-V1\0').hexdigest();scope['nuclei']=[nucleus];(out/'scope.json').write_text(json.dumps(scope,indent=2)+'\n')
responsibilities={
'01_core/src/entities/module.rs':'''Adicionar somente enum privado ModuleKind { Ordinary, Std, Format } e campo privado kind em ModuleInner. Module::new continua público e delega à construção privada com Ordinary, mesmo com nome global/format e bindings idênticos. Adicionar factories pub(crate) Module::stdlib(scope) e Module::format(scope) com nomes global/format fixos e predicados pub(crate) is_std/is_format. Não mudar debug nem Eq/Clone, setters, conteúdo, cache, metadata ou bibliografia. Medir sizeof de Module/ModuleInner antes/depois sem prometer ganho.''',
'01_core/src/compiler/eval/mod.rs':'''Nas duas construções produtivas de std (eval de expressão/source e eval documental), usar Module::stdlib(stdlib.clone()). Preservar scopes, nomes globais, fases, features, entradas e defaults. Testes devem passar por cada entrypoint; não marcar módulos finais de documento como std.''',
'01_core/src/compiler/eval/modules.rs':'''Na construção isolada da stdlib de arquivo importado usar Module::stdlib(stdlib.clone()). Módulos exportados de arquivo continuam Module::new com nome do arquivo e sem autoridade de std/format. Preservar resolução World, warnings, content e semântica de import/include.''',
'01_core/src/compiler/stdlib/format.rs':'''A factory make_format_module devolve Module::format(scope) em vez de Module::new("format", scope). Isto supersede somente a mecânica histórica de construção genérica P1461. Não altera scope, members, defaults, clones, funções privadas marcadas Format ou assinaturas.''',
'01_core/src/compiler/eval/bindings/field_access/identity.rs':'''is_effective_std_module e is_effective_format_module delegam aos predicados internos do Module; remover autenticação por nome/type/document/svg/png/pdf/html. A política de símbolo obsoleto, PDF/a11y e identidades de funções permanece intocada. Testar com scopes nativos copiados por Module::new e por clones reais, não só nomes vazios.''',
'01_core/src/compiler/eval/bindings/field_access/fields.rs':'''O gate de html inclui módulo Std ou Format efetivo. Gates asset e bundle continuam nos respectivos tipos efetivos. Preservar prioridade, avaliação única do receiver, curto-circuito anterior a argumentos/set, span field, mensagens e dois hints. A API pura eval_value_field_access não recebe política de feature. Escrever testes funcionais antes do código usando API atual e Module::new/stdlib reais, com máscaras de oito perfis, aliases e shadowing de std dentro de closure, efeitos e spans. Não adicionar supressões.'''}
for src,owner in scope['owners'].items():
 p=Path(owner);s=p.read_text();line=f'- {nucleus} sha256:{effective}'
 if 'Núcleos Tekt:\n' in s:
  s=s.replace('Núcleos Tekt:\n','Núcleos Tekt:\n'+line+'\n',1)
 else:s=s.replace('\n\n','\n\nNúcleos Tekt:\n'+line+'\n\n',1)
 s+='''\n\n## Gate HTML e identidade de módulos — 2026-10-17

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

'''+responsibilities[src]+'''

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
'''
 p.write_text(s)
(out/'l0-record.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),nucleus=nucleus,effective=effective,owners={p:hashlib.sha256(Path(p).read_bytes()).hexdigest() for p in scope['owners'].values()}),indent=2)+'\n');print(effective)
