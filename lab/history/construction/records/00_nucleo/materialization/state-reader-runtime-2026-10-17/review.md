# A1-S2 — leitura de State por objeto e callbacks sob demanda

## Medição anterior à decisão

HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c, working tree não commitado;
fontes/UTC/diffstat em before.json.gz e validation-freeze.json.gz do diagnóstico
00_nucleo/diagnosticos/lote-a1-callbacks-reflexao-seletores-2026-10-17.
Oráculo bd5ebec285505889e8ff1d4f68804ab147a699f3, SHA-256
9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9.
state-counterproof.json.gz conserva fontes, comandos e resultados de três binários.

Contraprovas: dois leitores da mesma chave com init 1/10, após +1, recebem
2/11 no vanilla; o Tekt falha. Update criado mas não inserido não altera State.
Callback inserido sem leitura não executa. Callback que falha antes de um Set
posterior continua falhando quando se pede final. Reutilizar o mesmo conteúdo de
update duas vezes produz duas atualizações. Upstream introspection/state.rs:226–245
explica init por leitor; :477–520 começa a sequência em state.init e aplica
Set/Func em ordem na leitura. ValueMethods/state.rs:25 atualmente transforma Func
em Set; o pós-passe from_tags/state_updates.rs:29–62 só executa após init global.

A1-S1, manifesto c72a7d91557b42fbe80e82bdef3caf4ea7a6df095c4bbb56b0e852f896505635,
foi retirada. A aprovação recebida fica registrada, mas não será aplicada à
proposta refutada. Não adicionar init ao update nem usar o primeiro receiver
como inicialização global. O init pertence ao leitor.

A sonda display-reader-init NÃO é prova de paridade: neste pin o vanilla não tem
State.display. Esse método e as nativas state_update_with/state_final/state_at/
state_display são compatibilidade Tekt, com controles próprios.

## Decisão proposta — sujeita a novo gate ADR-0127 §2.1/§2.3/§2.4

Usar StateUpdate::Func existente para funções passadas ao método update.
O método apenas constrói conteúdo. Registrar ações presentes no documento,
inclusive sem inicialização global, em ordem causal de Location. Resolver
get/at/final do State a partir do init do próprio objeto, aplicando Set e Func
até o limite pedido. Não ordenar pelo inteiro bruto de Location.

Mover a aplicação automática de Func do pós-walk para a leitura com Engine.
Construção, walk e layout não executam callbacks. Uma leitura aplica cada ação
anterior ao limite uma vez nessa resolução; outra leitura pode recalcular.
Não prometer contagem global de chamadas nem reproduzir o cache upstream.
Erro anterior a Set não pode ser descartado por otimização. Sem leitura não há
execução. Callback recebe o valor corrente e sua captura léxica, sem herdar
context/here do leitor: preservar/restaurar contexto mesmo no erro.

Ações e snapshots são dados L1 puros, sem I/O nem cache global. Acrescentar
ledger privado a StateRegistry e acesso pub(crate) necessário ao resolvedor.
Não adicionar campo público nem variante. Init não é copiado em cada update.
O compartilhamento dos registros entre snapshots deve evitar cópia profunda
repetida de payloads; medir custo real e não alegar ganho sem número.

### Alterações públicas exatas

Em compiler/stdlib/state.rs, modificar somente estes helpers de leitura:

- state_get(state: &State, ctx: &mut EvalContext, engine: &mut Engine<'_>, span: Span)
  -> SourceResult<Value>.
- state_final com a mesma lista acima.
- state_at_location(state: &State, location: Location, ctx: &mut EvalContext,
  engine: &mut Engine<'_>, span: Span) -> SourceResult<Value>.
- native_state_final e native_state_at passam ao ABI NativeFuncWithEngine já
  existente: (ctx: &mut EvalContext, args: &Args, world: &dyn World,
  current_file: FileId, scopes: &mut Scopes<'_>, engine: &mut Engine<'_>)
  -> SourceResult<Value>. Registro via Func::native_with_engine existente.

Os callers Rust desses cinco helpers precisam migrar. State/StateUpdateElem/
ElementPayload/Introspector e StateUpdate mantêm suas definições públicas.
state_update(key,value) mantém assinatura e continua construtor Set; o método
moderno distingue Func antes de chamar esse helper.

As leituras legadas por chave usam o mesmo resolvedor com init legado emitido
quando houver, ou None, e compartilham a sequência de ações com os objetos
modernos. Preservar validação/ausência de chave e label das nativas legadas.
state.display e state_display continuam extensões Tekt e usam o valor resolvido;
callback de display não substitui callback de update. Preservar assinatura dos
construtores e do helper público apply_state_funcs; este último pode conservar
aplicação explícita legada para callers Rust, mas sai do caminho automático de
runtime para impedir execução antecipada. Não manter dois cálculos automáticos.

## L0s proprietários a atualizar antes de código

Aplicar o contrato deste pacote nas responsabilidades correspondentes, sem
compartilhar owners nem tratar este review como L0:

- 00_nucleo/prompts/entities/state_registry.md: ledger privado, ordem e partilha.
- 00_nucleo/prompts/compiler/introspect/populate.md: registrar Set/Func sem executar.
- 00_nucleo/prompts/compiler/introspect/from_tags/state_updates.md: compatibilidade
  do helper explícito; execução automática retirada.
- 00_nucleo/prompts/compiler/introspect/entrypoints.md e fixpoint.md: retirar chamada
  eager automática; preservar outros callbacks e convergência.
- 00_nucleo/prompts/compiler/stdlib/state.md: cinco assinaturas, leitura por objeto,
  limites/contexto/observação/replay e extensões legadas.
- Novo owner 00_nucleo/prompts/compiler/stdlib/state/resolve.md, consumer
  01_core/src/compiler/stdlib/state/resolve.rs: fold interno com Engine, erros e
  restauração causal. Nenhuma API pública desse descendente.
- 00_nucleo/prompts/compiler/eval/bindings/value_methods/state.md: Func/Set, chamadas
  Engine-aware e validação antes de contexto.
- 00_nucleo/prompts/compiler/stdlib/registry/contextual.md: ABI Engine das duas nativas.
- 00_nucleo/prompts/compiler/introspect/from_tags/displays.md: display legado resolve
  as ações com Engine sem executar callbacks não lidos.
- Prompts dos testes/callers identificados por busca: apenas migração prescrita,
  sem ampliar outros contratos. Uma assinatura adicional exige novo gate.

## Aceitação R1–R12

R1 dois leitores de mesma key, init diferente, get/at/final; R2 init None e função
que o recebe; R3 Func/Set/Func em ordem causal, inclusive Location não numérica;
R4 erro anterior a Set e captura léxica; R5 callback não lido e conteúdo criado
não inserido; R6 mesma atualização inserida duas vezes e duas keys independentes;
R7 limites at antes/depois e final posterior; R8 contexto não herdado/restaurado
após sucesso/erro; R9 nativas legadas e mistura moderna/legada em ambas direções;
R10 display Tekt com valor correto, sem reivindicar método vanilla inexistente;
R11 observação/replay/convergência e snapshots independentes, sem execução extra
por instrumentação; R12 migração Rust dos cinco helpers, suíte completa e RAM.

L0 primeiro; RED antes do código; GREEN bilateral no pin indicado, controles
legados próprios. Build, workspace/doctests, Clippy/Rustdoc estritos, fmt,
linhagem V1/V5/V15/V26 e reversa; não aumentar V16. MemoryMax12GiB, High8GiB,
swap0, jobs1 e tarefas pesadas sequenciais. Medir cópias/retensão e pico sem
promessa genérica de desempenho. Não inclui commit, publicação ou release.
