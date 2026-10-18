from pathlib import Path
import json,gzip,hashlib,datetime,subprocess
p=Path(__file__).resolve().parent
common='''
## A1-S2 — leitura por objeto aprovada em 2026-10-17

### Medição anterior à decisão

HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c, working tree não commitado;
UTC/status/diffstat/fontes em before.json.gz e final-freeze.json.gz de
00_nucleo/diagnosticos/lote-a1-callbacks-reflexao-seletores-2026-10-17.
state-counterproof.json.gz registra RED bilateral: init 1/10 da mesma chave
após +1 exige 2/11; callback não lido não executa; atualização reutilizada
executa por ocorrência. Pin bd5ebec285505889e8ff1d4f68804ab147a699f3,
upstream introspection/state.rs:226–245, 419–455 e 477–520. O init do leitor
inicia a sequência; a ordem vem do documento, não do inteiro de Location.
O método atual trata Func como Set e o pós-passe eager perde essa semântica.
Estes são valores/erros de linguagem, não igualdade Rust ou algoritmo exigido.
A1-S1 foi retirada; gate A1-S2 confirmado pelo dono nesta sessão, revisão
00_nucleo/materialization/state-reader-runtime-2026-10-17/review.md,
SHA-256 6d6a176b8e27cc53c38fe6650c03c217448059f4e7287fd8848fe54e80247fa4.

### Responsabilidade exclusiva — supersede a fase histórica de State

'''
contracts={
 'entities/state_registry':'''Preservar definições públicas e assinaturas de StateRegistry. Adicionar somente
registro privado de ações StateUpdate por key/Location, independente de existir
init global. Capturar Set e Func, deduplicar somente a mesma ocorrência de mesma
Location, ordenar leituras por order_key causal e respeitar limite inclusivo.
Nunca executar callbacks neste owner. A visão pública materializada continua
legada; o leitor moderno não usa seu primeiro init como inicialização global.

Representar privadamente valores materializados com Arc<Value>. Operações
registradas usam enum interno pub(crate) RecordedStateUpdate: Set(Arc<Value>)
e Func(Func), sem alterar StateUpdate público. Compartilhar o Arc do Set entre
histórico legado e ação, sem clone profundo adicional. A lista de ações por key
é Arc<Vec<...>> e mutação usa Arc::make_mut; clone de snapshots compartilha dados,
mas extensões posteriores não alteram a cópia anterior. Helpers crate-private
registram ações, extraem ações ordenadas com limite e obtêm init legado elegível.
Registro Set preserva auto-init/último valor legado; Func não materializa valor.
init/update explícitos históricos continuam com suas regras e não inventam ação.
Testar ordem não numérica, snapshots independentes e partilha física do payload;
isto verifica a prescrição de RAM, sem confundi-la com paridade de linguagem.
''',
 'compiler/introspect/populate':'''Delegar cada payload StateUpdate ao registro privado de ações, em vez de
executar ou duplicar sua lógica Set/Func. Registrar inclusive Func sem init.
State legado ainda registra init; os índices, tags, labels e contadores mantêm
ordem e identidade. Walk não aplica callback e não avalia init de leitor.
''',
 'compiler/introspect/from_tags/state_updates':'''O helper público apply_state_funcs conserva assinatura e aplicação explícita
legada para callers Rust e fixtures existentes. Não é mais chamado pelo caminho
automático runtime/fixpoint. Não ampliar esse helper nem usá-lo para inicializar
leitor moderno; leitura funcional sob demanda pertence ao resolvedor de State.
''',
 'compiler/introspect/entrypoints':'''Retirar somente a chamada automática apply_state_funcs dos dois entrypoints
runtime. Manter registro puro das ações no walk. Coordenar versão interna
fallible dos displays de State para propagar erros de update exigidos pela
leitura; conservar callback/coerção defensiva legada de display. Não mudar as
assinaturas públicas destes entrypoints nem os demais pós-processadores.
''',
 'compiler/introspect/fixpoint':'''Retirar aplicação automática eager de StateUpdate::Func; os leitores com Engine
resolvem as ações registradas. Coordenar display interno fallible e preservar
cap=5, hash/critério de convergência, precedência dos demais passes e APIs.
Testes que pretendem observar valor funcional usam leitura runtime com Engine,
não assumem que o trait legado sem Engine materialize Func automaticamente.
''',
 'compiler/stdlib/state':'''Modernos get/at/final resolvem a sequência de ações a partir do init do receiver.
Capturar o resultado completo (inclusive erro) na observação causal existente.
Display de State lê o mesmo valor antes de sua callback; essa extensão Tekt não
é método do vanilla atual. Replay delega a leitura com Engine, conserva contexto
capturado e não reaplica a callback externa de display nem o corpo contextual.

Alterar apenas as cinco assinaturas ratificadas: state_get/state_final recebem
(state: &State, ctx: &mut EvalContext, engine: &mut Engine<'_>, span: Span);
state_at_location recebe state, Location, ctx mutável, Engine mutável, Span.
Todas retornam SourceResult<Value>. native_state_final/native_state_at passam
a (ctx,args,world,current_file,scopes,engine) no ABI NativeFuncWithEngine já
existente. Sem novos campos públicos, variantes públicas ou mudança de traits.

Declarar descendente privado state/resolve.rs para o fold. Adapter crate-private
state_method_update projeta Func em StateUpdate::Func e delega outros valores
a state_update; este helper público antigo permanece construtor Set com mesma
assinatura. Leituras legadas por key começam em init legado elegível ou None,
compartilham ações modernas/legadas, e mantêm ausência de chave/label/validação.
Adapters sem ações preservam fixtures/valores materializados da API legada.
O helper privado de leitura legada com registry explícito também serve displays.

Testes R1–R12 antes do código, incluindo callback não lido, init por objeto,
Func/Set/Func, erro, captura e contexto restaurado. Preservar coerções de display
que não pertencem a este reparo. As cláusulas antigas que fixavam leituras sem
Engine, update Func fora do escopo ou replay sempre materializado são sucedidas.
''',
 'compiler/eval/bindings/value_methods/state':'''Projetar update funcional pelo adapter state_method_update sem executá-lo.
Get/at/final passam Engine e contexto mutável para os helpers aprovados.
Preservar validação antes de contexto, resolução/observação de labels, spans e
captura. Rejeitar argumentos excedentes/nomeados onde a linguagem exige aridade
exata; não permitir que o caminho AST aceite o que o método capturado rejeita.
''',
 'compiler/eval/call_dispatch/value_methods':'''Migrar somente o despacho de State: update usa o adapter interno Func/Set,
get/final/at encaminham o Engine já disponível. A rota capturada como valor
mantém a mesma semântica da chamada direta, sem reconhecimento por nome.
Preservar todos os outros receivers, validação e diagnósticos existentes.
''',
 'compiler/stdlib/registry/contextual':'''Registrar somente native_state_final/native_state_at via Func::native_with_engine,
com mesmo nome público e Value::Func. As demais nativas conservam ABI/bindings.
Não executar reads nesta composição. O ABI NativeFuncWithEngine já existe;
esta cláusula ratificada supersede a exigência anterior de ABI intacto destas duas.
''',
 'compiler/introspect/from_tags/displays':'''O display legado de State demanda resolução das ações com Engine no seu ponto.
A callback externa de display/coerções defensivas mantêm comportamento anterior.
Criar helper interno fallible para propagar erro do update demandado; o helper
público apply_state_displays conserva retorno unit e comportamento defensivo
para callers explícitos. Runtime/fixpoint usam o helper interno, sem executar
callbacks State não demandados. Counter display e seus contratos permanecem.
''',
 'compiler/introspect/from_tags':'''Reexportar somente internamente o display checked de State; preservar exports
públicos e o helper apply_state_funcs explícito legado. Acrescentar testes reais
com Engine: dois init por leitor, prefixo/final Func/Set/Func; callback sem leitura
e erro na leitura; contexto/here não herdado e restaurado no sucesso/erro.
Testes do helper explícito legado continuam independentes do runtime automático.
''',
 'compiler/eval':'''Migrar o teste de replay contextual para a assinatura aprovada de state_get,
usando seu Engine existente. A mudança observável no candidato deve vir de Set
registrado como ação, não de trocar o init global: init moderno vem do objeto.
Preservar o propósito: replay usa Location/contexto capturados, não ambiente
posterior; não duplicar observações. Nenhum campo/API/entrypoint deste hub muda.
''',
 'compiler/stdlib/tests/table_grid':'''Migrar somente as chamadas aos dois helpers nativos State para o ABI com Engine,
usando with_engine/Scopes das fixtures existentes. Preservar entradas, nomes,
valores/erros esperados e a intenção dos controles de ausência de chave/label,
init e updates materializados; não inserir exceções produtivas para passar teste.
''',
 'compiler/eval/tests':'''Acrescentar controles de linguagem para update funcional direto e método guardado
em variável, com conteúdo inserido/reutilizado; a mera criação não executa.
O init pertence a cada leitor. Não chamar State.display prova vanilla: é extensão
Tekt. Estados com Set conservam paridade e controles anteriores.
'''
}
old={}
for owner,contract in contracts.items():
 file=Path('00_nucleo/prompts')/(owner+'.md');old[str(file)]=hashlib.sha256(file.read_bytes()).hexdigest()
 file.write_text(file.read_text()+'\n'+common+contract+'''
### Aceitação e fronteiras

Gate A1-S2 ratificado para as cinco assinaturas e a fase de leitura. L0 primeiro,
RED funcional antes do código; GREEN contra o pin indicado para linguagem e
controles próprios das extensões Tekt. Não ampliar outros contratos públicos.
Build/workspace/doctests, Clippy/Rustdoc estritos, fmt, linhagem V1/V5/V15/V26 e
reversa; manter V16 sem aumento. MemoryMax12GiB/High8GiB/swap0/jobs1, tarefas
pesadas sequenciais. Medir consumo sem inferir ganho global só da representação.
''')
file=Path('00_nucleo/prompts/compiler/stdlib/state/resolve.md');file.parent.mkdir(exist_ok=True)
file.write_text('''# L0 — compiler/stdlib/state/resolve
Hash do Código: 00000000

Camada L1. Consumer produtivo único: 01_core/src/compiler/stdlib/state/resolve.rs.

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/state/language-semantics.toml sha256:27acb21a5e0b2e0cb3b65de61bba5266158f3e9828392fe92a1a3b160e9d61a4
- 00_nucleo/prompts/_nuclei/realization-order.toml sha256:8de2c32fbeeee7a30088b4ff3808e15a211f23a182673ef5cdebe54bd3eadad6

'''+common+'''Resolver privadamente uma sequência registrada, com init pertencente ao leitor.
Aplicar Set por valor e Func com argumento único corrente, captura léxica e
Scopes local. Não herdar in_context/current_location do leitor no callback;
restaurar ambos mesmo em erro. Não executar Func na construção/walk/layout.
Resultados/erros são SourceResult<Value>; não usar fallback sólido, None ou
Empty para esconder erro do update. A rota moderna observa o resultado completo.

Guardar somente o valor selecionado e o acumulador necessário, não toda a
sequência de Values se não houver necessidade semântica. Ações compartilhadas
não copiam init nem payload Set profundamente por snapshot. Sem cache global,
I/O, dispatcher dinâmico, mudança de propriedade dos campos ou nova API pública.
Não prometer número global de chamadas: cada leitura legítima pode recalcular;
replay não cria corpos/callbacks de display ou leituras novas por instrumentação.

R1–R12 e controles de ordem/prefixo/erros devem refutar a implementação antes de
aceitá-la. RAM12GiB/High8GiB/swap0/jobs1; validar linguagem, não bytes/algoritmo.
''')
record={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'approval':'Aprovar A1-S2','review_sha256':'6d6a176b8e27cc53c38fe6650c03c217448059f4e7287fd8848fe54e80247fa4','before_hashes':old,'after_hashes':{f:hashlib.sha256(Path(f).read_bytes()).hexdigest() for f in old},'new_owner':str(file)}
(p/'state-l0-before-code.json.gz').write_bytes(gzip.compress(json.dumps(record,indent=2).encode()))
