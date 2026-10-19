from pathlib import Path
import hashlib,json
p=Path(__file__).resolve().parent
n=Path('00_nucleo/prompts/_nuclei/layout/bibliography-plan.toml');n.write_text('''tekt = 1
kind = "nucleus"
id = "bibliography-plan"
title = "Plano privado de entradas bibliográficas"

[[claims]]
id = "ordered-entries"
level = "must"
statement = "Preservar ordem e fronteiras das entradas, prefixo opcional e corpo Content do CSL. Prefixo em qualquer entrada seleciona colunas para a lista; sem prefixos, distinguir hanging e plain pela indicação do CSL. Não inferir prefixos analisando texto já concatenado."

[[claims]]
id = "private-transport"
level = "must"
statement = "O plano é privado ao layout, imutável e compartilhado por Arc entre tentativas e checkpoints; rollback conserva integralmente o plano. Não adicionar campos a BibRenderCache nem mudar assinaturas públicas. Não executar callbacks, nova introspecção ou outra passagem CSL para reconstruir fronteiras."

[[claims]]
id = "language-not-mechanics"
level = "must"
statement = "Cache público conserva sua composição Content anterior. O consumidor físico usa fronteiras tipadas e containers existentes sem acrescentar ocorrências semânticas de Grid/Block ou Location artificial. Estado temporariamente incompleto de hanging precisa permanecer explícito nos L0s e nos resultados, sem ser certificado por melhora das colunas."
''')
pin=hashlib.sha256(n.read_bytes()+b'\0TEKT-NUCLEUS-DEPS-V1\0').hexdigest()
evidence='''
### Medição anterior à decisão

00_nucleo/diagnosticos/bibliography-structure-2026-10-18/before.json.gz registra
12 pares, fontes/argv/UTC/HEAD/status/diff HEAD --stat, textos/caixas/binários;
before-sources.json.gz conserva as fontes e L0s anteriores. HEAD
b1b486f3d42431f8ea360bc5712637ee1e3d4f6c, árvore não commitada, Tekt SHA-256
c2b1b2e93cf34f3c597a0a0fbbd51c8f8ac96a3c6bc0a4d9bd72373599acf899;
oracle bd5ebec285505889e8ff1d4f68804ab147a699f3, árvore
cac836c07e4dec2a89219370428292393a3216b5, binário SHA-256
9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9.
IEEE12 põe A. em x25.124 contra29.924 e a segunda entrada em y89.936
contra112.232. IEEE18 usa x33.686 contra40.886. No CSL sem prefixo, a segunda
entrada começa em y184.424 contra191.024. As sondas com par.spacing0/2em
separam margens de espaçamento dentro das linhas. Há também diferença de
agrupamento de citações adjacentes (MLA Silva/Doe), fora da composição da lista;
manter o RED explícito, não chamar igualdade só da lista de paridade geral.

bib_csl.rs:342–355 concatena first_field, espaço literal, content e Linebreak,
perdendo a fronteira de cada entrada e o hanging_indent. convergence.rs:58–82
transporta só BibRenderCache público; bibliography.rs:107–151 emite um Block
único. O pin typst-layout/src/rules.rs:463–531 compõe duas colunas Auto com
gutter0.65em e row-gutter par.spacing, ou um Block por entrada sem prefixo;
o formato autoral do código Rust não é requisito. Observáveis são posição,
ordem, conteúdo/estilo e escopo. Hipótese: transportar fronteiras reais e
prefixos corrige a coluna e a separação sem reformatar; controles de fontes,
ordem, campos vazios, sem prefixo, zero/2em, show-grid e estilo podem refutá-la.

### Classe e escopo explícito

Correção interna/de paridade ADR-0127 em fluxo contínuo, dentro do layout,
sem API pública, tipo público novo, flag/default, compatibilidade ou fase nova.
L0 antes de testes/código. A2-Structure preserva hanging no discriminante do
plano, mas sua aplicação física, overrides por ocorrência e composição
simple/optimized/RTL continuam em A2-HangingTransport/A2-ParIndent. Não forçar
fallback greedy para contornar o suporte de avanço inicial ainda ausente da
preparação. Margens de título, CSL completos, atribuição A2-Multi e agrupamento
de citações adjacentes não ficam certificados por esta etapa.
'''
accept='''
### Aceitação

RED funcional de coluna com gutter relativo e margem entre entradas antes de
implementar. Controles de cache público (Content anterior e assinaturas), CSL
sem prefixo, preservação de modo hanging/plain, vazio, ordenação/citações e
rollback. PDF bilateral com caixas e texto da lista, conservando divergências
de recuo e agrupamento de citações no relatório. Build/workspace/doctests,
Clippy e Rustdoc estritos, fmt, V1/V5/V15/V26 e reversa; SARIF sem erro/warning
novo. Resselo só após ownership/grafo válido. MemoryMax12GiB/High8GiB/swap0,
jobs1/debug0/incremental0, comandos pesados sequenciais; sem commit/push/release.
'''
roles={
'compiler/layout/bib_csl.md':'''Declarar submódulo privado plan. Um builder privado prepara simultaneamente BibRenderCache e Option<Arc<plan::BibliographyPlan>>, usando a mesma execução CSL já existente. As duas fachadas públicas mantêm assinatura, diagnósticos, citações e Content legado, extraindo somente o cache do resultado privado. A conversão do RenderedBibliography para plano tipado pertence exclusivamente a 00_nucleo/prompts/compiler/layout/bib_csl/plan.md; delegar conversores existentes de ElemChild/ElemChildren por descendência. Não renderizar CSL novamente nem converter strings para deduzir fronteiras.''',
'compiler/layout/bibliography.md':'''Quando há plano privado, fechar a entrada como já prescrito e despachar o título existente. Lista com prefixos usa Grid técnico de duas colunas Auto, gutter relativo0.65em, sem inset/pintura/tags. Representar row-gutter par.spacing por tracks Fixed entre rows Auto e células vazias nos tracks de intervalo; não criar margem terminal. Envolver a grade no Block existente e chamar layout diretamente, sem eval/show de Grid. Sem prefixos, despachar cada body em seu próprio Block breakable, conservando divisão plain/hanging no plano (ambos usam Block nesta subetapa incompleta). Sem plano, conservar cache programático e fallback anteriores. Não modificar Content público nem usar props genéricas para transportar o plano. Não alterar estratégia de quebra para simular hanging. As normas novas substituem o Block único de A2-fronteiras somente quando o plano está disponível.''',
'compiler/layout.md':'''Adicionar exclusivamente um slot privado Option<Arc<bib_csl::plan::BibliographyPlan>>, inicializado None no Layouter. O cache público existente permanece no mesmo tipo e os consumidores externos não mudam. Estado do plano não é Content nem atributo de Style. Dispatcher estático continua; coordenação e render permanecem nos owners próprios.''',
'compiler/layout/convergence.md':'''Preparar o par cache/plano uma vez a partir da mesma primeira declaração efetiva, locale/resolved_style/ordem já decididos. Propagar diagnósticos como antes. Cada tentativa recebe o cache e um clone Arc do plano; descoberta/ordem/escopo A2-CSL não mudam. Não mudar fachada pública nem executar outro driver de bibliografia para criar o plano.''',
'compiler/layout/checkpoint.md':'''Capturar/restaurar o slot privado do plano por clone/move da mesma Option<Arc>, ao lado do cache já existente. Rollback e medição especulativa não perdem o discriminante nem reconstroem o plano; demais buffers mantêm a disciplina atual.'''}
for rel,role in roles.items():
 f=Path('00_nucleo/prompts')/rel;s=f.read_text();line=f'- {n} sha256:{pin}\n'
 if 'Núcleos Tekt:\n' in s:s=s.replace('Núcleos Tekt:\n','Núcleos Tekt:\n'+line,1)
 else:s=s.replace('\n\n','\n\nNúcleos Tekt:\n'+line+'\n',1)
 s+='\n## A2-Structure — entradas CSL e colunas (2026-10-18)\n'+evidence+'\n### Responsabilidade exclusiva\n\n'+role+'\n'+accept;f.write_text(s)
f=Path('00_nucleo/prompts/compiler/layout/bib_csl/plan.md');f.parent.mkdir(exist_ok=True);f.write_text('# L0 — plano de bibliografia CSL\nHash do Código: 00000000\n\nNúcleos Tekt:\n- '+str(n)+' sha256:'+pin+'\n\nLayer: L1. Exclusive consumer: `01_core/src/compiler/layout/bib_csl/plan.rs`.\nArchitecture: ADR-0107/0108/0109/0127/0129/0130.\n'+evidence+'''
### Responsabilidade exclusiva

Definir tipos privados ao layout para entradas {prefix: Option<Content>, body:
Content} e plano discriminado Columns/Hanging/Plain, cada um com entradas
ordenadas. Columns prevalece se qualquer prefixo existir, mesmo vazio; caso
contrário usar o flag hanging_indent do CSL. Converter os nós com os helpers do
pai, conservando wrappers/links e sem formatar outra vez. Expor ao pai a
conversão e a projeção legacy_content: para cada entrada, prefixo se Some,
espaço literal se Some, body, Linebreak, exatamente como o renderer público
anterior, inclusive ausência de prefixo e lista vazia. O plano é artefato do
layout, não campo público ou entidade de linguagem. Não contém I/O, relógio,
ambiente, callbacks nem Location sintetizada. Pin compartilhado rege transporte.
'''+accept)
(p/'prescription.json').write_text(json.dumps({'owners':list(roles)+['compiler/layout/bib_csl/plan.md'],'nucleus':str(n),'effective_pin':pin},indent=2)+'\n');print(pin)
