from pathlib import Path
import json
out=Path('00_nucleo/diagnosticos/tekt-v16-bibliography-static-2026-10-17')
common='''

## I2 — cobertura estática bibliográfica (2026-10-17)

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
00_nucleo/diagnosticos/tekt-v16-bibliography-static-2026-10-17/before.json.gz
registra UTC, status, diff HEAD --stat, os textos anteriores e 2130 hashes
conferidos. bibliography.rs:21–79 enumera só parte dos containers e omite
Heading, Repeat, Columns e TableCell crua; :87–266 repete o catálogo parcial.
traversal.rs:337–1105 já possui enumeração estática pré-ordem completa, com
substituição terminal; :89–107 distingue Source, Located e Realized. O owner
TableCell seleciona standalone.visual, resolved.visual ou body, nesta ordem.
introspect/walk.rs:1206–1268 confirma os corpos de grid/header/footer/cell,
attribution de Quote, title de Bibliography e supplement de Cite; :1396
estabelece Document/Asset como recursos separados. São perdas internas de
conteúdo, não razão para copiar igualdade, bytes ou mecânica upstream.

A hipótese é que os conteúdos estáticos atualmente ignorados perdem chaves e
conversão Ref→Cite. Testes RED devem confirmar antes do código. A hipótese é
refutada se o caminho já conservar a chave/citação; não alterar esperados para
forçar essa classificação. Carriers internos usam a prova metamórfica da I1;
sem inferir paridade universal ou corrigir os limites do walker de math/HTML.
'''
contracts={
'00_nucleo/prompts/compiler/introspect/bibliography.md':'''### Responsabilidade exclusiva

Completar I2 por reutilização das travessias do domínio: coleta por visita
emprestada de conteúdo estático, conversão por map_content_pre já existente.
Não manter segunda tabela de reconstrução dos elementos no compiler.
A coleta insere entries de toda Bibliography alcançada e também visita title.
Document/Asset são fronteiras de outro recurso; não vazar chaves nem converter
suas referências usando bibliografia do documento hospedeiro. Dynamic e
ContextBlock são opacos na coleta; não executar callbacks, recipes ou closures.
Não descer em Value, estilos, snapshots de regras ou Realized.original.

Conversão substitui somente Ref de chave conhecida pelo mesmo Cite existente,
com supplement preservado; desconhecidos ficam intactos. Dynamic mantém sua
delegação legada dyn_map_content com convert_refs. Outline conserva título e
corpo já realizado, sem refazer realização nem perder capturas. O resto usa
map_content_pre, incluindo células raw/standalone/resolved, headers/footers,
math e demais containers. Source segue sua normalização causal já ratificada;
Location, Realized.original, planos/props/stage e snapshots são preservados.
Fast path sem chaves devolve a entrada. A classificação aberta de callback
(None significa sem substituição) é contrato de transformação, não default
semântico para elementos desconhecidos; a enumeração exaustiva vive no domínio.

### Aceitação

RED→GREEN de coleta/conversão em containers omitidos, campos opcionais e
seqüências aninhadas; isolamento de Document, origem Realized e raw oculto de
células preparadas. I1 continua íntegra. Conversão desconhecida não cria Cite;
coleta não chama Dynamic; conversão mantém callbacks legados e limites de Ref.
Não certificar que o walker de introspecção já publica eventos de todos os
containers atravessados; medir a transformação e a ordem publicada nos casos
que o walker suporta. Não alterar o walker, API, fases ou pipeline.
''',
'00_nucleo/prompts/entities/content/traversal.md':'''### Responsabilidade exclusiva

Adicionar somente helper interno pub(crate) visit_content_static, por
empréstimo, com closure FnMut(&Content)->bool: false poda o subgrafo; true
visita filhos estáticos. Sem alocar clones/coleções para navegar. Source,
ShowRecipe, Located e Realized são transparentes ao callback: body/visual
somente, nunca origem de Realized nem snapshots capturados. A enumeração de
Content é exaustiva, sem wildcard terminal; variantes novas exigem decisão.

Usar os campos Content navegáveis do mapeamento público: corpos simples,
seqüências, campos opcionais de Figure/Quote/captions/footnotes/bibliografia,
math (inclusive slots presentes e matrizes), HTML body, PageRun body e campos
estáticos de marginais/supplement/background/foreground; raw Table visita
children/header/footer/caption, preparada visita cells/caption; TableCell
visita exclusivamente visual() já definido. Outline visita title e realized_body;
OutlineEntry visita element/fill/prefix; Document body/title são navegáveis
apenas quando o callback não podar. Ref, SetPage, Value, callbacks, estados,
recursos e Dynamic/ContextBlock são terminais nesta vista estática. Não avaliar
Func nem chamar dyn_map_content. Não modificar map_content/map_text/pre-order
nem métodos públicos dos elementos. Nenhum tipo/trait/campo público novo.

### Aceitação

Testes locais de ordem/poda, transparência de origem, seleção exclusiva da
vista preparada e ausência de efeitos Dynamic. O helper é leitura estrutural,
não emite ocorrência/Location nem aplica regras. Variantes fictícias devem
provocar E0004 na enumeração de filhos. Dados/leaves não são tratados como
containers via introspecção Debug ou strings de nomes.
'''}
tail='''
### Gate e validação

Correção interna ADR-0127: L0 antes de testes/código e resselo restrito aos
dois owners após grafo V15/V26 válido. Sem campo/assinatura pública, flag,
default ou mudança de fase; sem dependência ou publicação. Não silenciar V16,
V19 ou V20. Justificar no L0 o grupo nominal de terminais da visita (folhas
ou dados/callbacks opacos, sem Content estático ativo). Build, RED→GREEN,
I1, workspace completo, fmt, Clippy all-targets -D warnings e
-D unfulfilled_lint_expectations, Rustdoc estrito/quatro índices, linhagem
V1/V5/V15/V26/reversa, diff --check e freeze das fontes. Pesados seqüenciais,
MemoryMax12GiB/High10GiB/swap0/jobs1. Relatório com estado exato, argv,
hashes e UTC. I2 substitui a dívida de cobertura dos containers estáticos da
bibliografia da I1; demais frentes e objetivo global permanecem abertos.
'''
for p,c in contracts.items():
 f=Path(p);f.write_text(f.read_text()+common+c+tail)
(out/'workers/prescribe.py').write_text(Path('/tmp/bib-i2-prescribe.py').read_text())
print('L0 prescribed before tests/code')
