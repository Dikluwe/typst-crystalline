from pathlib import Path
import json,gzip,hashlib,re,datetime,shutil,subprocess,tempfile
root=Path.cwd();diag=root/'00_nucleo/diagnosticos/transformacoes-2026-10-14';out=root/'00_nucleo/materialization/transformacoes-tipadas-2026-10-14';out.mkdir(exist_ok=False)
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
with gzip.open(diag/'before.json.gz','rt') as f:base=json.load(f)
assert all(sha(p)==h for p,h in base['files'].items())
nucleus='''tekt = 1
kind = "nucleus"
id = "transform-semantics"
title = "Identidade, presença e resolução das transformações"

[[claims]]
id = "operation-and-presence"
level = "must"
statement = "Move, Rotate, Scale e Skew conservam identidade nativa e presença lexical de cada argumento. Omissão, zero, false, auto e alinhamento parcial não colapsam. Matrix é uma operação Rust genérica distinta, nunca inferida como função nativa a partir dos coeficientes."

[[claims]]
id = "late-resolution"
level = "must"
statement = "Avaliação valida e transporta valores tipados, sem assar defaults de estilo ou resolver comprimentos dependentes de fonte/região/corpo. Layout resolve os valores no uso, produz matriz finita e preserva raiz de pintura; exporters consomem Frame e não repetem layout."

[[claims]]
id = "public-observation"
level = "must"
statement = "Campos, func, repr, encoders, seletores, igualdade da linguagem e observação conservadora preservam operação, valores e presença. Clone, mapas, wrappers, caches e realização não apagam essa distinção. Campos herdados de set não se tornam lexicalmente explícitos."

[[claims]]
id = "frame-geometry"
level = "must"
statement = "Origin resolve cada eixo sobre o frame lógico do corpo, com defaults center/horizon e alinhamentos direcionais resolvidos no estilo vigente. Reflow falso conserva tamanho/baseline/avanço lógicos; reflow verdadeiro transforma os limites e ajusta avanço/baseline segundo a operação. Move altera tinta sem alterar o frame lógico."

[[claims]]
id = "no-surrogate-transport"
level = "must-not"
statement = "Não codificar argumentos autorais em coeficientes sentinela, Styled artificial, metadata, strings, registry global, Location falsa ou matriz redundante. Não inferir kind pela matriz nem aplicar pivô ou origem duas vezes."

[[claims]]
id = "target-boundary"
level = "must"
statement = "Transformações não promovem o documento HTML a layout paginado. O diagnóstico estrito vigente para conteúdo não suportado permanece; não copiar a eliminação do body observada no pin HTML. Ilhas html.frame continuam com seu contrato próprio."
'''
npath='00_nucleo/prompts/_nuclei/transform-semantics.toml';pin=hashlib.sha256(nucleus.encode()+b'\0TEKT-NUCLEUS-DEPS-V1\0').hexdigest()
def put(path,text):
 p=out/'candidates'/path;p.parent.mkdir(parents=True,exist_ok=True);p.write_text(text)
put(npath,nucleus)
evidence='''## Medição anterior à decisão

`00_nucleo/diagnosticos/transformacoes-2026-10-14/{before,results}.json.gz` e
`controls/results.json`, `binding-controls/results.json`, `context-controls/results.json`
preservam HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree, UTC,
diffstat, fontes, comandos e hashes de binários. Pin 9f2b6e8715237cb086899a42873660fe744622e8.
TransformElem em entities/elements/transform.rs:19 armazena apenas matrix/body;
stdlib/transforms.rs:19/51/91/191 descarta argumentos e reduz valores à matriz;
layout/transform.rs calcula pivô central e usa has_orphaned como desvio de layout.
No pin, typst-library/src/layout/transform.rs conserva os quatro elementos e
typst-layout/src/transforms.rs resolve estilos, região, corpo, origin e reflow.

Sondas: fields vazio versus body/argumentos; func falso versus função nativa;
rotate(0deg)[X] == scale(100%)[X] true versus false; defaults explícitos colapsam.
Origin/reflow são ignorados ou rejeitados; scale(x:150%) escala também y no
Tekt, enquanto y permanece100% no pin; move perde percentual/em. Campos ausentes
devem continuar desconhecidos lexicalmente. set rotate(23deg) é válido, mas
angle nomeado não é; getters contextuais refletem estilo no pin. Os primeiros
controles contextuais rotularam o wrapper e são inválidos para medir value;
context-controls corrige a posição da label. HTML pinado avisa e elimina o body;
Tekt rejeita a operação, de acordo com seu contrato HTML estrito.

Semântica, sintaxe, morfologia, posições/avanços e diagnósticos são linguagem;
enum versus structs upstream e número de passos de medição são mecânica.
A hipótese é refutável por campos/presença que ainda colapsem, estilo assado,
origem incorreta, erro perdido, reflow falso alterando avanço ou body descartado.

'''
owner='00_nucleo/prompts/entities/elements/transform.md';codehash=re.search(r'^Hash do Código: (.+)$',Path(owner).read_text(),re.M)[1]
entity=f'''# L0 — entities/elements/transform
Hash do Código: {codehash}

Núcleos Tekt:
- {npath} sha256:{pin}

Camada L1. Consumer exclusivo: `01_core/src/entities/elements/transform.rs`.
ADRs 0107, 0108, 0109, 0127, 0128, 0129 e 0130.

'''+evidence+'''## Contrato tipado autoritativo

Esta seção substitui o carrier matrix/body anterior. Os tipos públicos pertencem
a este único owner; os nomes e campos abaixo fazem parte do contrato Rust:

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct TransformElem {
    pub operation: TransformOperation,
    pub body: Content,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TransformScale {
    Auto,
    Ratio(Ratio),
    Length(Length),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TransformOperation {
    Matrix(TransformMatrix),
    Move {
        dx: Option<Rel<Length>>,
        dy: Option<Rel<Length>>,
    },
    Rotate {
        angle: Option<Angle>,
        origin: Option<Align2D>,
        reflow: Option<bool>,
    },
    Scale {
        x: Option<TransformScale>,
        y: Option<TransformScale>,
        origin: Option<Align2D>,
        reflow: Option<bool>,
    },
    Skew {
        ax: Option<Angle>,
        ay: Option<Angle>,
        origin: Option<Align2D>,
        reflow: Option<bool>,
    },
}
```

Importar Content, Rel e os tipos geométricos vigentes; não criar outra família
de unidades/alinhamentos. Option representa presença, não default resolvido.
Some(TransformScale::Auto) distingue auto de omissão. Body é obrigatório na
construção linguística e sempre aparece em fields, mesmo vazio.
Não manter matrix redundante ao lado de operation.

## Projeção e preservação

Nome e função nativa derivam da variante, nunca de coeficientes. Ordem lexical:
Move dx,dy,body; Rotate angle,origin,reflow,body; Scale x,y,origin,reflow,body;
Skew ax,ay,origin,reflow,body. Omitir apenas campos None na projeção lexical.
get_field devolve valores explicitamente conhecidos; o classificador de métodos
separa declarado/omitido de não declarado. Nenhum default de layout aparece como
campo explícito. Identidade e igualdade de linguagem distinguem variantes e
presença; não comparar somente a matriz resultante. Observação conservadora não
reutiliza resultados de operações/presenças diferentes como se fossem iguais.

plain_text delega ao body; is_empty mantém o default false do Element. Clone,
Hash manual compatível com igualdade, map_content/map_text e reconstruções
preservam operation integralmente. O despacho de layout continua em
`00_nucleo/prompts/compiler/layout/transform.md`, sem import reverso ou lógica
de render na entidade. Campos não promovem a família a locatable por padrão.

## Compatibilidade Rust e Matrix

O campo público matrix é removido; literais externos e leituras desse campo
precisam migrar para operation. Para o uso matricial genérico, escrever
operation: TransformOperation::Matrix(matrix). A assinatura pública existente
Content::transform(matrix, body) permanece e constrói Matrix. Matrix conserva
a semântica genérica anterior, sem se anunciar como move/rotate/scale/skew,
sem herdar propriedades nativas dessas funções e sem novo construtor público.
Native constructors passam a criar as variantes tipadas diretamente.

## Aceitação

RED→GREEN de identidade, omissão/zero/false/auto, eixos parciais, units e mapas.
Integração deve cobrir construtores, set/show, getters contextuais, serialização,
layout e controles por target. Build verde sozinho não certifica a migração.
Sem I/O, estado global, vtable, PropMap de entidade ou inferência por render.
'''
put(owner,entity)
stdlib='''## Contrato completo de construção e estilo

O carrier tipado de `00_nucleo/prompts/entities/elements/transform.md` substitui
a redução prematura para matriz. Preservar assinaturas públicas native_*.
Validar pela sequência de ocorrências de Args e spans; não procurar apenas
valores numericamente convenientes e descartar os demais. Strings e none
convertem para Content conforme casts vigentes; números não são body. Body é
posicional obrigatório. Argumentos desconhecidos, sobras, tipos errados e
body nomeado produzem os diagnósticos medidos, sem fallback identidade/zero.

- move: dx/dy nomeados aceitam Rel<Length> (ratio, length, mistura). Defaults
  não são inseridos lexicalmente. Int/float/string não viram pontos.
- rotate: angle posicional opcional de tipo Angle, encontrado como no parse
  tipado pinado, inclusive depois de body string. Angle nomeado é rejeitado.
  Origin aceita Align2D preservando eixos ausentes; reflow aceita bool.
- scale: no máximo um fator posicional Auto/Ratio/Length, que assenta ambos os
  eixos; x/y nomeados sobrepõem somente o eixo correspondente. Sem fator, y
  omitido não copia x. Ratio/Length/Auto são distintos; int/float e relative
  length misto não são fatores válidos. Dois auto são construíveis e observáveis;
  o erro de resolução ocorre em layout, não ao inspecionar fields.
- skew: ax/ay apenas nomeados e Angle; origin/reflow como rotate. Não rejeitar
  ângulos finitos por um limiar artificial de proximidade a90 graus. Validação
  de geometria não finita cabe à resolução, sem converter erro em identidade.

O default efetivo é move zero/zero, rotate zero, scale100%/100%, skew zero/zero,
origin center+horizon e reflow false. São defaults de estilo/layout, separados
da presença lexical. set usa os mesmos casts/bindings, sem body: angle de rotate
continua posicional, fator de scale assenta x/y e sobreposições preservam eixos.
Origin faz fold por eixo: um eixo explícito sobrepõe o herdado, os demais herdam.
Set não inventa campos em fields do constructor. Aliases/with devem respeitar
identidade nativa e avaliação única dos argumentos; getters contextuais usam o
mesmo esquema. Atualizar os owners de set/field access antes de implementar.

Não resolver em/fontes/percentuais/escala absoluta em eval. Não usar Styled
artificial para esconder os argumentos autorais; o canal normal de estilos
permanece com seus owners. Parsing, campos e diagnósticos são observáveis.

## Aceitação

Reproduzir todo o corpus de queries e bindings; estender para spans, ordem,
duplicatas, aliases/with, set e leitura contextual. Fields, func, repr, equality,
show/where e encoders devem concordar sobre presença e identidade. Sem promessa
de paridade de Rust/mecânica de macros ou de número de passagens upstream.
'''
def appended(owner,body):
 text=Path(owner).read_text()
 # Insert the pin into the canonical metadata, sorted with existing pins.
 lines=text.splitlines();pins=[l for l in lines if re.match(r'^- 00_nucleo/prompts/_nuclei/.* sha256:',l)]
 newpin=f'- {npath} sha256:{pin}'
 if pins:
  first=lines.index(pins[0]);lines=[l for l in lines if l not in pins];lines[first:first]=sorted(pins+[newpin])
 else:lines[2:2]=['','Núcleos Tekt:',newpin,'']
 text='\n'.join(lines)+'\n'
 text+='\n## Transformações tipadas — contrato substitutivo\n\n'+evidence+'Esta seção substitui cláusulas anteriores conflitantes somente para a família\nMove/Rotate/Scale/Skew; demais responsabilidades permanecem.\n\n'+body
 put(owner,text)
appended('00_nucleo/prompts/compiler/stdlib/transforms.md',stdlib)
layout='''## Resolução e geometria

Consumir TransformOperation explicitamente. Matrix mantém o comportamento
genérico anterior por um braço próprio; não usar essa compatibilidade como
fallback das quatro variantes nativas. Conservar assinatura privada, topologia
estática e representação Group com paint_root false para envelope técnico.

As variantes nativas medem frame lógico do body com tamanho/baseline reais,
incluindo espaços, boxes e conteúdo vazio. Não usar apenas bbox da tinta ou
arestas da fonte como substituto universal da medida. Ignorar marcadores privados
de HTML no cálculo sem removê-los do transporte. Preservar caminhos dos fixups,
separadores, Source, TextReplacement, recursos e collectors.

Move resolve Rel<Length> com a região do uso e em com o estilo de fonte atual;
aplica translação visual e mantém tamanho/baseline/avanço. Rotate/Skew resolvem
angles/defaults, e Scale resolve Ratio diretamente ou Length contra a dimensão
lógica medida do body; um eixo Auto acompanha a escala do outro. Dois Auto
produzem `x and y cannot both be auto` na resolução, inclusive via set. Preservar
escala zero e sinais negativos sem substituí-los por identidade. Medir casos
degenerados antes de escolher diagnósticos; não inventar limites por conveniência.

Origin resolve alinhamento por eixo sobre o frame lógico (topo do frame, não
baseline usada como origem arbitrária): T(pivot)*M*T(-pivot). Start/end usam
direção efetiva. Um alinhamento parcial faz fold com herdado/default. A posição
do parent é aplicada uma vez. Não inferir reflow pela presença de alinhamentos
órfãos, tipo do body ou coeficientes da matriz.

Reflow false mantém a caixa lógica e baseline do body; somente a tinta muda,
inclusive em contexto inline, block, place e nested. Reflow true transforma os
quatro cantos do frame, desloca pela mínima coordenada e ajusta caixa/avanço.
Scale sem flip vertical preserva baseline transformada; skew conserva a linha
transformada da baseline no meio do frame; rotate e flip vertical seguem os
observáveis medidos do pin sem fingir uma baseline de fonte para qualquer body.
Transformações não perdem labels, texto, clipping, recursos ou ordem de leitura.

Resolução de units passa da extração prematura em eval para layout; avaliação
do body, efeitos e diagnósticos não são duplicados por medições. Sub-layout e
tentativas restauram cursores, coletores e contexto; nenhuma mutação global.
O mecanismo de medição pode diferir do pin desde que preserve os observáveis.
Exporters não avaliam argumentos nem precisam conhecer TransformOperation.

## Aceitação

Geometria causal com controles default/origin parcial/extremos, reflow false/true,
Scale x somente, fator+override, absolute/auto, move percent/em, skew e nested.
Incluir texto/matemática inline e baseline, following content, quebra de página,
RTL, vazio/zero/flip, clips, gradientes Parent e recursos. Validar fonte→PDF/SVG/PNG
e campos antes/depois da realização; não alinhar imagens para esconder posição.
HTML fora de ilhas permanece no diagnóstico estrito do owner HTML, sem drops.
'''
appended('00_nucleo/prompts/compiler/layout/transform.md',layout)
appended('00_nucleo/prompts/entities/content/construction/layout.md','''## Construção Rust genérica

Content::transform(matrix: TransformMatrix, body: Content) -> Content mantém a
assinatura e passa a construir TransformElem { operation:
TransformOperation::Matrix(matrix), body }. Nenhum argumento nativo é inferido
da matriz. Demais constructors desta unidade permanecem. NativeMove/Rotate/
Scale/Skew são construídos pelo owner stdlib, com presença tipada independente.
''')
inventory=json.loads((diag/'consumers.json').read_text())
extra=['01_core/src/entities/content/fields.rs','01_core/src/compiler/eval/bindings/field_access/content_methods.rs','01_core/src/compiler/eval/bindings/value_methods/element_where.rs','01_core/src/compiler/eval/rules/set.rs','01_core/src/compiler/eval/rules/show.rs','01_core/src/compiler/stdlib/loading/encoding.rs','02_shell/src/cli.rs','03_infra/src/export/html/inline.rs']
seen={x['path'] for x in inventory}
for source in extra:
 if source not in seen:
  text=Path(source).read_text();m=re.search(r'^//! @prompt (.+)$',text,re.M);assert m,source
  inventory.append(dict(path=source,owner=m[1],lines=[],role='semantic boundary / validation; inspect before editing'))
files={}
for x in inventory:
 for p in [x['path'],x['owner']]:
  assert p is not None;files[p]=dict(sha256=sha(p),text=Path(p).read_text())
with gzip.open(out/'base-files.json.gz','wt') as f:json.dump(dict(utc=now(),head=base['head'],files=files,inventory=inventory),f)
# Validate only graph/ownership in a disposable complete source mirror. No code or canonical prompt is changed.
with tempfile.TemporaryDirectory(prefix='tekt-transform-proposal-') as d:
 stage=Path(d)
 for top in ['01_core','02_shell','03_infra','04_wiring','benches','00_nucleo/prompts']:
  for p in (root/top).rglob('*'):
   if p.is_file() and p.suffix in ['.rs','.md','.toml']:
    q=stage/p.relative_to(root);q.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(p,q)
 for n in ['Cargo.toml','crystalline.toml']:shutil.copyfile(root/n,stage/n)
 for p in (out/'candidates').rglob('*'):
  if p.is_file():q=stage/p.relative_to(out/'candidates');q.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(p,q)
 cmd=['/tmp/tekt-linter-ci-target/debug/crystalline-lint',str(stage),'--checks','v15,v26','--fail-on','warning'];r=subprocess.run(cmd,capture_output=True,text=True)
 (out/'graph-validation.log').write_text(r.stdout+r.stderr);assert r.returncode==0,(r.returncode,r.stdout[-1500:],r.stderr[-1500:])
manifest=dict(status='pending-human-approval',utc=now(),head=base['head'],scope='Typed Move/Rotate/Scale/Skew through constructors, language observation, styles and paged layout; Matrix Rust compatibility; no publication.',public_changes=['TransformElem.matrix removed; TransformElem.operation: TransformOperation added','New public TransformOperation and TransformScale enums as specified in entity candidate','Content::transform signature unchanged; produces Matrix variant'],phase_change='Relative/font/body-dependent values retained during eval and resolved at layout use.',candidates={str(p.relative_to(out)):sha(p) for p in sorted((out/'candidates').rglob('*')) if p.is_file()},nucleus_pin=pin,base_files={p:v['sha256'] for p,v in files.items()},inventory=inventory,evidence={str(p.relative_to(root)):sha(p) for p in [diag/'before.json.gz',diag/'results.json.gz',diag/'controls/results.json',diag/'binding-controls/results.json',diag/'context-controls/results.json']},graph_validation=dict(checks='V15,V26',exit=0,log_sha256=sha(out/'graph-validation.log')))
(out/'manifest.json').write_text(json.dumps(manifest,ensure_ascii=False,indent=2)+'\n');digest=sha(out/'manifest.json');(out/'manifest.sha256').write_text(digest+'  manifest.json\n')
review=f'''# Proposta: transformações tipadas e resolução no layout

**Aguardando confirmação. Nenhum código ou L0 canônico desta proposta foi alterado.**

Substituir `TransformElem.matrix` por `operation: TransformOperation` e manter
`body: Content`. A enum pública distingue Matrix, Move, Rotate, Scale e Skew;
os argumentos das quatro funções são tipados e opcionais para preservar presença.
`TransformScale` distingue Auto, Ratio e Length. A assinatura de
`Content::transform(matrix, body)` permanece e cria Matrix para uso Rust genérico.
Literais Rust externos e leituras de `.matrix` precisarão migrar.

O [contrato exato](candidates/00_nucleo/prompts/entities/elements/transform.md)
contém todos os tipos, campos e regras. Os candidatos de
[construção nativa](candidates/00_nucleo/prompts/compiler/stdlib/transforms.md),
[layout](candidates/00_nucleo/prompts/compiler/layout/transform.md) e
[construção Rust](candidates/00_nucleo/prompts/entities/content/construction/layout.md)
definem as responsabilidades, com núcleo compartilhado pinado. Cada prompt
mantém um único consumer. A validação V15/V26 passou na cópia de revisão.

## Mudanças observáveis incluídas

- Preservar `origin`, `reflow`, units, auto e argumentos omitidos; corrigir fields,
  func, repr, igualdade, observação, encoders, set/show/where e getters contextuais.
- Resolver em/percentual/escala por tamanho no layout, com região, fonte e body
  efetivos. Aplicar pivô sobre frame lógico e reflow explícito, preservando fluxo,
  baseline, recursos e ordem de leitura.
- Corrigir a validação de chamadas que hoje são aceitas ou ignoradas indevidamente.
  `scale(2)` passa a erro: usar `scale(200%)`. `scale(x:150%)` deixa y em100%.
  `rotate(angle:23deg)` é erro; o ângulo é posicional. `skew(12deg)` é erro;
  usar `skew(ax:12deg)`. `move(dx:2)` requer unidade, como `2pt`.
- Conservar a rejeição estrita de transformações não suportadas em HTML fora de
  ilhas; o pin avisa e elimina TEXT nos controles. Não reproduzir essa perda de
  conteúdo nem transformar todo HTML em documento paginado.

## Aceitação autorizada pela proposta

X1: casts, ordem/consumo/spans e diagnóstico de argumentos, incluindo negativos.
X2: campos explícitos/omitidos, identidade nativa, igualdade/observação, mapas e
encoders, inclusive zero/false/auto e identidades matriciais coincidentes.
X3: estilos/aliases/with, fold por eixo, show/where, getters contextuais e ausência
de defaults assados nos campos; sem efeitos duplicados na medição.
X4: origin/default/RTL, Move relativo/em, Scale Ratio/Length/Auto, reflow, limites,
baseline, texto/matemática, following content, nested, clips, Parent e recursos.
X5: integração PDF/SVG/PNG, controles HTML e Matrix Rust, suíte completa,
build/doc/fmt, inventário de avisos e linhagem direta/reversa.

Depois de confirmar, instalar os candidatos, atualizar os L0 dependentes da lista
de impacto antes de seus consumers, testar RED→GREEN e ressellar apenas owners
alterados. A lista de impacto é conservadora; presença na lista não obriga editar
arquivo que só atravessa body. Nenhum contrato público adicional fica autorizado
por inferência: se surgir outro, exige revisão própria. Não inclui release.

Manifesto SHA-256: `{digest}`. [Manifesto](manifest.json),
[fontes/owners base](base-files.json.gz), [checagem do grafo](graph-validation.log).

A [ADR-0127 §2](/repos/Antigravity/typst-crystalline/00_nucleo/adr/typst-adr-0127-gate-l0-paragem-vs-fluxo.md)
exige a confirmação: “adicionar, remover ou alterar campo público” e “Mudança
arquitectural de fase/ordem de pipeline”. Aplica-se à remoção de matrix/adição de
operation e à resolução tardia de valores antes reduzidos em eval. As aprovações
anteriores de Raw e tiling não cobrem esse contrato novo.
'''
(out/'review.md').write_text(review)
assert all(sha(p)==h for p,h in base['files'].items())
print(json.dumps(dict(review=str(out/'review.md'),manifest_sha256=digest,inventory=len(inventory),canonical_sources_unchanged=True),indent=2))
