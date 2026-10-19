from pathlib import Path
import json,gzip,hashlib,datetime
p=Path(__file__).resolve().parent
common='''
## A1-H2 — suplemento antes das consultas, aprovado em 2026-10-17

### Medição anterior à decisão

HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c, working tree não commitado;
final-freeze.json.gz e target-controls.json.gz no diagnóstico
00_nucleo/diagnosticos/lote-a1-callbacks-reflexao-seletores-2026-10-17 registram
UTC/diffstat/fontes/argv. Pin bd5ebec285505889e8ff1d4f68804ab147a699f3.
heading-raw-constructor-func passa no baseline/vanilla e falha no candidato:
Styled de transporte altera func. Query também vê wrapper causal temporário;
HTML ignora callback panic. Fonte walk.rs:930–945; pipeline/paged.rs:419–427 e
pipeline/html.rs:101–106 preparam a consulta pelo walk puro. São valores e erros
de linguagem, não um requisito de copiar algoritmo/estrutura do vanilla.
Gate A1-H2 confirmado pelo dono nesta sessão: revisão
00_nucleo/materialization/heading-supplement-runtime-2026-10-17/review.md,
SHA-256 66739b13596013f4343bdc3ffacbdb9081151e3d9863e5d962e8ff13fd68775c.

### Responsabilidade exclusiva

'''
contracts={
 'entities/value':'''Preservar todas as assinaturas públicas de IntrospectedContent e Value. O slot
privado fields continua Option<Arc<...>>, agora apontando para cabeçalho privado
SnapshotFields com mapa único e heading_styles: Option<Arc<Styles>>. Derivar
Clone/Debug/PartialEq do cabeçalho privado, preservando comparação estrutural do
mapa e dos styles pendentes (mecânica Rust, não igualdade de linguagem).
new move o mapa e começa sem styles pendentes; fields devolve o mesmo &IndexMap.
Adicionar apenas helpers crate-private para anexar/consultar styles pendentes,
via cópia local quando Arc compartilhado. Styles auxiliares nunca viram campos
públicos. Clones existentes permanecem imutáveis. Não aumentar slots, sizeof ou
alinhamento de Value/IntrospectedContent; medir antes/depois no mesmo ABI.
''',
 'compiler/introspect/walk':'''Guardar Content semântico intacto na entrada canônica; não criar Styled para
transportar causalidade de Heading. Quando houver heading.supplement custom,
anexar os styles causais ao cabeçalho privado do snapshot existente. Preservar
mapa completo/ordem/label por último e Location; não criar segundo store nem
executar callback no walk. O entrypoint puro projeta Heading como Heading.
''',
 'compiler/introspect/from_tags/headings':'''Finalizar Heading pelo índice causal existente de Heading, somente se o snapshot
possui styles pendentes. Guardar temporariamente apenas Locations pendentes;
não alocar para todos os defaults. O consumo da pendência impede repetir a
callback se o índice repetir Location ou a fachada for chamada novamente.
Base é entry.content() Heading, não wrapper nem corpo/hash/label como identidade.
Usar styles capturados para callback, sem inventar here/context, restaurando
Engine.styles no erro. Publicar nova entrada com supplement Content e sem styles
pendentes; clones anteriores não mudam. Preservar resolved_labels, numbering e
demais campos/label. Manter responsabilidade exclusiva do callback neste filho;
entrypoints só delegam, sem duplicar lógica. Chamadas runtime usam a mesma função.
''',
 'compiler/introspect/entrypoints':'''Adicionar somente a fachada pública aprovada:
realize_heading_supplements(intr: &mut TagIntrospector, engine: &mut Engine<'_>,
ctx: &mut EvalContext) -> SourceResult<()>.
Delegar ao filho proprietário. Finaliza apenas Heading pendente, sem walk,
State/Counter/equações, layout ou exportação. Demais entrypoints preservam
assinaturas; atualizar chamada interna do filho para índice, sem exigir tags.
''',
 'compiler/introspect':'''Reexportar a nova fachada realize_heading_supplements de entrypoints, com o
contrato público exato A1-H2. Sem lógica de realização neste hub nem novo trait.
''',
 'compiler/introspect/fixpoint':'''Atualizar somente chamada interna do finalizador Heading para usar o índice e
pendência do snapshot; preservar limite, convergência e a retirada eager de
State aprovada separadamente em A1-S2. Sem novas APIs deste owner.
''',
 'compiler/introspect/from_tags':'''Manter o reexport interno do finalizador e testes causais. Acrescentar teste
com snapshot puro e duas chamadas da fachada: somente a primeira aplica a
callback. Verificar Content semântico, fields e imutabilidade de clones.
Preservar testes State e explícitos legados; não transferir a lógica do filho.
''',
 'compiler/stdlib/structural/heading':'''Construir Heading base uma vez e transportar numbering/supplement explicitamente
fornecidos em um único Styles, com marcador interno heading.constructor=true.
Não confundir ausência com numbering:none ou supplement:none/auto: transportar
presença usando o Styles existente. Não executar callback. Numeration usa os
mesmos heading.numbering/heading.numbering.pattern existentes. O marcador é
interno e só é colocado pelo construtor; wrappers Styled externos não o herdam.
Validar supplement antes de retornar. Não mudar HeadingElem ou qualquer campo
público. A igualdade semântica do suplemento continua dívida separada A3.
''',
 'entities/content/fields':'''Adicionar projeção crate-private constructor_heading que reconhece somente
Styled marcado heading.constructor=true cujo filho direto (desconsiderando
Source) é Heading. Retorna Heading e seus Styles. Styled comum fica opaco.
Getters dessa forma projetam supplement explicitamente armazenado, numbering
string/none conforme gate/pattern e os campos já existentes do Heading base.
Não remover wrappers em geral nem importar compiler a partir de entities.
''',
 'compiler/eval/bindings/field_access/content_methods':'''Usar a projeção nominal constructor_heading para func()==native_heading e para
presença/at/has/fields, incluindo supplement e numbering explícitos. Ordem raw:
level,depth,numbering,supplement,outlined,bookmarked,body; somente campos Set são
emitidos, body por último. Fora das propriedades transportadas delegar ao Heading
base. Não aceitar Styled comum de texto ou wrapper externo como Heading.
Snapshots continuam autoritativos; não reinferir seus campos pelo estilo atual.
''',
 'compiler/eval/bindings/field_access/fields':'''Acesso estrito da forma marcada passa por content_field (Set/Unset/Undeclared),
preservando distinção de campo omitido. Styled não marcado continua opaco;
nenhuma transparência geral para obter text/Heading ou mudar morfologia.
''',
 'infra/pipeline':'''Declarar somente mod runtime; ele compõe Engine/ctx causais para os filhos.
Não incluir lógica semântica de Heading neste hub. Preservar exports e APIs.
''',
 'infra/pipeline/paged':'''Compor o setup de introspecção pelo helper privado runtime, preservando a
fachada privada existente e seu default Paged/features vazio. Adicionar variante
privada com target/features explícitos para a sessão contextual. No snapshot
inicial (inclusive vindo de realização), chamar somente a fachada L1 Heading
antes de prepare(context), propagando erros e warnings. Não antecipar outros
callbacks. Preservar posição do pós-passe completo runtime posterior, métricas,
layout, versão, opções, paginação e APIs públicas deste owner.
''',
 'infra/pipeline/html':'''No snapshot inicial, inclusive vindo de realização, chamar somente a fachada
L1 Heading via runtime privado com EvalTarget::Html e features efetivas antes
de prepare_html. Propagar erro e warnings. Não executar layout paginado global
ou callbacks nos encoders. Preservar ilhas html.frame, positions, gates e pretty.
''',
 'infra/pipeline/context_stabilization':'''No ramo de runtime candidato não bundle, fornecer self.target/self.features ao
helper privado de introspecção, em vez de defaults Paged/sem features. Manter
bundle existente, seleção, 5 tentativas, ordem de produtores, snapshots e cache
de métricas. Nenhuma lógica de fold ou callback de Heading passa para L3.
''',
 'compiler/eval/tests':'''Adicionar controles raw: func, supplement presente, numbering:none explícito,
campos em ordem e wrapper Styled comum opaco, incluindo texto. Comparar valores
da linguagem, preservando negativos de ausência e sem espelhar o helper.
'''
}
old={}
for owner,contract in contracts.items():
 file=Path('00_nucleo/prompts')/(owner+'.md');old[str(file)]=hashlib.sha256(file.read_bytes()).hexdigest();file.write_text(file.read_text()+'\n'+common+contract+'''
### Aceitação e limites

H1–H8 da revisão ratificada, RED bilateral antes do código. Nenhum contrato
público além da fachada Heading está autorizado por H2; A1-S2 cobre separadamente
cinco helpers State. Sem novos campos públicos. Styled ordinário e demais
callbacks conservam suas fronteiras. Workspace/doctests/build/Clippy/Rustdoc,
fmt, V1/V5/V15/V26/reversa e V16 sem aumento; 12GiB/High8GiB/swap0/jobs1,
comandos pesados sequenciais. Ganho de RAM exige medição, não intenção.
''')
f=Path('00_nucleo/prompts/infra/pipeline/runtime.md');f.write_text('''# L0 — infra/pipeline/runtime
Hash do Código: 00000000

Camada L3. Consumer produtivo único: 03_infra/src/pipeline/runtime.rs.

'''+common+'''Compor privadamente EvalContext/Engine/Sink com World, Source.id, Route, métricas,
StyleChain inicial e stacks existentes. Contexto recebe target/features explícitos.
Helper genérico with_introspection_runtime executa somente a operação L1 fornecida,
retornando resultado e diagnostics do Sink; sem decisão semântica de elementos.
Helper de Heading chama exclusivamente a fachada pública ratificada L1.
Não antecipar State/Counter/equações nem fazer layout, I/O adicional, cache global
ou selecionar modo. Lifetime local por operação; Engine/World permanecem externos
ao L1 puro. Assinaturas deste módulo são restritas a pipeline e descendentes.

H1–H8, prova bilateral PDF/HTML e gates completos sob 12GiB/High8GiB/swap0/jobs1.
''')
(p/'heading-l0-before-code.json.gz').write_bytes(gzip.compress(json.dumps({'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'approval':'Aprovar A1-H2','review_sha256':'66739b13596013f4343bdc3ffacbdb9081151e3d9863e5d962e8ff13fd68775c','before':old,'after':{f:hashlib.sha256(Path(f).read_bytes()).hexdigest() for f in old},'new_owner':str(f)},indent=2).encode()))
