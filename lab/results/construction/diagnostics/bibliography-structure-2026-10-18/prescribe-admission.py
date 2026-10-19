from pathlib import Path
import json,hashlib,re,shutil
p=Path(__file__).resolve().parent;r=Path.cwd();shutil.copy2('target/debug/typst',p/'bin/typst-block-before')
evidence='''
### Medição anterior à decisão — admissão e continuação

pagination-block.json.gz e block-freeze.json.gz em
00_nucleo/diagnosticos/bibliography-structure-2026-10-18 preservam estado completo,
UTC/HEAD/fontes/binários/argv e caixas do candidato Block/Place c1c222ae54ab3b67487471326012a7f6c4ed86c1e6bd61b1830ec6a987c2ccea.
Em IEEE/200×90pt ele recupera duas páginas, mas a continuação começa em x8
contra29.924 no oracle bd5ebec285505889e8ff1d4f68804ab147a699f3 (binário
9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9).
region_progress.rs:75–76 reinstala a margem física ao transitar, descartando o
inset ativo. O teste corrigido para70pt também mostrou prefixo na primeira
página e corpo admitido na segunda: paragraph_pagination.rs:87–105 pode adiar
o corpo para respeitar widow/orphan depois que o Place já foi emitido.

A exigência inicial de FIRSTWORD sempre na primeira página da fixture FixedMetrics
não é o contrato correto: group_size em paragraph_pagination.rs:69 pode manter
três linhas juntas. Substituir esse critério pelo prefixo acompanhar a página
real da primeira linha; preservar o RED PDF90/duas páginas e a distinção entre
coluna estável e estratégia de quebra. Não forçar greedy nem desabilitar
widow/orphan para satisfazer um assert. Posições geométricas, conteúdo e agrupamento
são observáveis da língua; slots/representação são mecânica privada.

Correção interna ADR-0127 dentro do layout, sem assinatura/campo/trait público,
flag/default novo, compatibilidade ou execução eval/show. A primeira tinta do
prefixo continua antes da primeira tinta do corpo, após a admissão física do
parágrafo. Não é nova ordem de realização de show nem transferência de fase.
A hipótese é refutável por prefixo órfão, recuo perdido, emissão dupla, efeitos
novos, estratégia alterada ou regressão dos controles de Block/columns/subframe.
'''
ns={
'layout/bibliography-prefix-admission.toml':('bibliography-prefix-admission','''O prefixo bibliográfico pendente é contexto privado de layout; emitir uma vez na página/coluna e baseline da primeira folha textual efetivamente admitida do corpo, após widow/orphan/overflow, antes da tinta do corpo. Preservar chain e TextStyle do prefixo, não medir ou renderizar o corpo novamente nem executar callbacks/eval/show. Captura/restore conserva o contexto completo; subframes de medida/Place o suspendem e o restauram. Não criar Location ou metadado de introspecção artificial.'''),
'layout/block-inline-region.toml':('block-inline-region','''Enquanto o corpo de Block está ativo, conservar sua origem horizontal relativa ao início da região externa e largura interna resolvida. Após transição ordinária de página/coluna, reinstalar esses limites na nova região. Ao sair, restaurar o contexto externo; mesma região conserva restauração existente e região nova usa sua geometria externa atual. Checkpoints capturam/restauram; subframes isolam/restauram. Campos são privados de Layouter, não BlockElem; não restaurar altura ou origem de uma página encerrada.''')}
pins={}
for rel,(ident,claim) in ns.items():
 f=Path('00_nucleo/prompts/_nuclei')/rel;f.write_text(f'tekt = 1\nkind = "nucleus"\nid = "{ident}"\ntitle = "{ident}"\n\n[[claims]]\nid = "causal-context"\nlevel = "must"\nstatement = "{claim}"\n');pins[rel]=hashlib.sha256(f.read_bytes()+b'\0TEKT-NUCLEUS-DEPS-V1\0').hexdigest()
roles={
'compiler/layout/bibliography.md':(['layout/bibliography-prefix-admission.toml'],'''Trocar a emissão antecipada de Place por PendingPrefix privado com Content, chain, TextStyle, indent e direção. Instalar ao redor do Block do body e restaurar o contexto externo ao terminar a entrada. Corpo vazio com prefixo usa Block textual comum, sem deixar estado pendente para a próxima entrada. O helper privado emit_pending_prefix toma o contexto antes de renderizar (evita recursão), ancora um Place técnico à baseline da folha real e ao line_start_x/largura corrente, suspende temporariamente chain/style/block_place_region do corpo e os restaura. A página/coluna é a admitida, sem capturar Location ou deduzir posição de query por tinta. Prefixo continua CSL puro. Preparação mede só prefixos e não reexecuta o body. Preservar custos e backends de quebra. A sintaxe anterior emitido antes do corpo significa antes da tinta, não antes da admissão.'''),
'compiler/layout/word_layout.md':(['layout/bibliography-prefix-admission.toml'],'''No produtor privado push_text, depois da admissão da baseline feita por word/chunk/prepared e antes de highlight e folha, pedir a emissão do prefixo pendente ao owner bibliography. Ausência é no-op; prefixo é tomado antes de emissão recursiva. Não mover a chamada para ensure_initial_baseline/paragraph_pagination, que também preparam geometria antes da decisão de página.'''),
'compiler/layout/text.md':(['layout/bibliography-prefix-admission.toml'],'''No caminho de Text vazio que emite uma folha diretamente, consumir o prefixo pendente após text_baseline::admit e antes da folha. Texto não vazio continua usando word_layout; nenhuma chamada adicional à preparação/avaliação.'''),
'compiler/layout/block.md':(['layout/block-inline-region.toml'],'''Definir contexto Copy privado InlineRegion com offset horizontal relativo ao início da página/coluna exterior e largura interna. Instalar ao entrar no body depois de resolver inset e dimensões; restaurar em LIFO depois do body. Expor apenas helper descendente que reinstala a geometria horizontal na região atual a partir do contexto, ou da região externa natural quando ausente. A mesma região conserva saved_line_start/saved_width. Após mudança de região, restaurar o contexto do pai e a geometria atual desse pai; não copiar origem/limites da região encerrada nem alterar altura. O contexto não transfere inset vertical, pintura ou dimensões de outra fase.'''),
'compiler/layout/region_progress.md':(['layout/block-inline-region.toml'],'''Após criar a próxima coluna na mesma página, ou concluir o início da primeira coluna/página física, reinstalar o contexto inline de Block se Some, pelo helper do owner Block. Sem contexto, conservar o fluxo anterior. Fazer isso depois de estabelecer a geometria natural da região e antes de emitir a próxima linha. Não alterar ordem de drains, reset, floats ou aceitação do suffix.'''),
'compiler/layout.md':(list(ns),'''Adicionar somente dois slots privados, pending_bib_prefix: Option<bibliography::PendingPrefix> e block_inline_region: Option<block::InlineRegion>, ambos None inicialmente. Manter dispatcher e interfaces públicas. Estes contextos são capacidades de layout local, não estado global, domínio público, Style custom ou vtable.'''),
'compiler/layout/checkpoint.md':(list(ns),'''Capturar/restaurar pending_bib_prefix por clone/move e block_inline_region por Copy junto dos demais contextos. Restore deve preservar consumo/ausência e a região horizontal exata, sem renderizar nem recalcular a partir de texto.'''),
'compiler/layout/sub_frame.md':(list(ns),'''Suspender ambos os contextos ao entrar no subframe e restaurar em LIFO ao sair, juntamente com block_place_region e geometria já existentes. Medidas/Place não podem consumir o prefixo do corpo exterior nem herdar os limites de uma página exterior; Blocks internos podem instalar contexto próprio. Não alterar retornos, offsets, callbacks, dimensões ou a pureza da medição.''')}
d=json.loads((p/'prescription.json').read_text())
for rel,(refs,role) in roles.items():
 f=Path('00_nucleo/prompts')/rel;s=f.read_text()
 for ref in refs:
  line=f'- 00_nucleo/prompts/_nuclei/{ref} sha256:{pins[ref]}\n';s=s.replace('Núcleos Tekt:\n','Núcleos Tekt:\n'+line,1)
 lines=re.findall(r'^- 00_nucleo/prompts/_nuclei/[^\n]+ sha256:[a-f0-9]{64}$',s,re.M);it=iter(sorted(lines));s=re.sub(r'^- 00_nucleo/prompts/_nuclei/[^\n]+ sha256:[a-f0-9]{64}$',lambda _:next(it),s,flags=re.M)
 s+='\n## A2 — prefixo admitido e região inline de Block (2026-10-18)\n'+evidence+'\n### Responsabilidade exclusiva\n\n'+role+'\n\nAceitação: RED→GREEN de prefixo/corpo na mesma página e inset nas continuações, inclusive saída para OUTSIDE, simple/optimized e controles existentes. Medir PDF90/150/250, RTL, prefix-only/formatado, cores e regressões; build, suites/doctests/Clippy/Rustdoc/fmt/linhagem/reversa/diferencial Tekt. MemoryMax12GiB/High8GiB/swap0/jobs1; sem publicação. Hanging genérico/overrides e múltiplas bibliografias continuam frentes abertas.\n';f.write_text(s)
 if rel not in d['owners']:d['owners'].append(rel)
d['admission_nuclei']=pins;(p/'prescription.json').write_text(json.dumps(d,indent=2)+'\n')
# Make the test criterion honor actual paragraph admission; keep the PDF refuter.
f=Path('01_core/src/compiler/layout/bibliography.rs');s=f.read_text();s=s.replace('fn a2_structure_long_entry_starts_in_remaining_region()', 'fn a2_structure_prefix_follows_admitted_body()');a=s.index('        assert!(\n            document.pages[0].plain_text().contains("FIRSTWORD")',s.index('fn a2_structure_prefix_follows_admitted_body'));b=s.index('        assert!(document.plain_text().contains("OUTSIDE"));',a)
s=s[:a]+'''        let first_page = |needle: &str| document.pages.iter().position(|page| crate::compiler::layout::tests::frame_items_recursive(&page.items).iter().any(|item| matches!(item, FrameItem::Text{text,..} if text.as_str()==needle))).unwrap();
        assert_eq!(first_page("[1]"), first_page("A."), "{:?}", document.pages.iter().map(|p|p.plain_text()).collect::<Vec<_>>());
'''+s[b:];f.write_text(s)
(p/'admission-prescribed.json').write_text(json.dumps({'new_owners':list(roles),'pins':pins},indent=2)+'\n');print('prescribed',len(roles),'owners')
