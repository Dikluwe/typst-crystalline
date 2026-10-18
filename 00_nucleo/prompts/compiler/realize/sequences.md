# L0 — compiler/realize/sequences
Hash do Código: acb987d6

Layer: L1
Exclusive consumer: `01_core/src/compiler/realize/sequences.rs`


Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/sequence-realization.toml sha256:825182e1dfe3591249a5efbc4c0ddbd937ebffde953c6595e0d1ed8176106435
- 00_nucleo/prompts/_nuclei/eval/pending-text.toml sha256:3f300957a00ac2f02354331585c19b935428e05d5b8ac2115a533764262250c8
- 00_nucleo/prompts/_nuclei/eval/textual-recipe-frontier.toml sha256:f225f3039d4663e9bfcb408d26cf1cd6e557f089013bc2d95090ce16c52080c6

## R2-S — realização nominal de Sequence, ratificada em 2026-10-17

### Medição anterior à decisão

As coletas before/expanded/controls.json.gz e candidate.log em
00_nucleo/diagnosticos/sequence-constructor-2026-10-17 fixam HEAD
b1b486f3d42431f8ea360bc5712637ee1e3d4f6c, working tree não commitado,
UTC/diffstat/fontes/binários e pin bd5ebec285505889e8ff1d4f68804ab147a699f3.
Oito REDs iniciais; cinco passaram no candidato, show e recursão ainda não.
markup_stream.rs:240–260 aplana antes de selecionar a sequência; realize/mod.rs:
170–210 visita filhos antes de receita; apply.rs:140–170 delega não-localizáveis
ao ponto-fixo. O vanilla aceita alias e rejeita nova Sequence com mesmos
filhos. São efeitos, escopo, valores e diagnósticos de linguagem.

O dono aprovou a revisão
00_nucleo/materialization/sequence-realization-2026-10-17/review.md, manifesto
46132cb53898f2b126a08bf3b0495ed4c05b531c380846bccbd3d0906477c327,
nesta sessão. O gate ADR-0127 §2.3 cobre a ordem de realização desta nova
fronteira antes dos filhos/Table/math; não cobre novo contrato público.

### Responsabilidade exclusiva

Implementar o descendente puro de captura e realização de Sequence. Transportar
fronteiras usando ShowRecipe existente com marca privada no snapshot de styles.
Manter snapshots de regras/styles/guards/file e origem; um argumento protegido
carrega os RuleIds consumidos nesse transporte. Filtrar guards globais de
Sequence de uma captura fresca, sem filtrar guards de outras famílias.

Percorrer recipes marcados em pre-order; instalar/restaurar o ambiente lexical
em todos os retornos. Escolher a regra aplicável não consumida com precedência
existente, construir it protegido e delegar um único passo ao owner apply.
Produtos novos são revisitados; aliases guardados podem aplicar regras anteriores
ainda não consumidas. Sem regra restante, realizar os filhos efetivamente emitidos
e remover só o transporte privado. Não executar receita oculta, reavaliar arquivo
ou fabricar tags/localização de Sequence. Conservar a marca até proteção não
ser mais necessária no produto corrente; não usar igualdade de corpo como guard.

Show-set de Sequence conserva estilos e não repete efeitos. A expansão usa o
contador privado existente occurrence_show_depth e o limite/hints vigentes;
full_error acrescenta somente classificação diagnóstica, sem encerrar por
morph_canon. Herança/Source e demais callbacks pertencem aos owners atuais.

Os helpers de captura/snapshot/realização têm visibilidade apenas crate/compiler;
nenhum tipo/campo/trait/pub fn externo novo. RAM local, sem globals/I/O/unsafe.

### Aceitação

S1–S20 ratificados, RED antes de código, GREEN bilateral e regressões; não
reduzir expectativas por limite mecânico. Todos os owners 1:1 e núcleos pinados
válidos antes do resselo. Build/workspace/doctests separados, Clippy/Rustdoc
estritos, fmt/linhagem/reversa e SARIF sem novo erro/warning. 12GiB/High8GiB/
swap0/jobs1 e comandos pesados sequenciais. Sem publicação/release.

### Flag e isolamento do transporte

R2s-first.log deixa Empty sem Y enquanto outras sequências passam; eval consulta
has_deferred_show_rules para decidir a realização. Toda captura marcada efetiva
deve assentar essa flag privada, inclusive corpo vazio. Ao instalar os styles
da receita, sombrear a marca de transporte com false no ambiente da avaliação:
recipes genéricos criados pelo callback não devem herdar classificação Sequence.
O argumento protegido recebe marca própria true. Isto não muda a superfície
pública nem o contrato de captura de outras famílias.

### Filhos emitidos e snapshots genéricos

Depois de consumir guards de uma raiz, visitar seus filhos efetivamente
emitidos, incluindo Sequence crua aninhada criada antes da regra. Não
recapturar a própria raiz já guardada. Ao atravessar ShowRecipe genérico,
instalar/restaurar seu ambiente para visitar filhos sem trocar seu tipo
ou executar a receita genérica nesta fase. Raiz interna de agrupamento
textual não é uma Sequence nova da linguagem; pular somente essa raiz
artificial, preservando seus filhos. O corpo alterado é devolvido ao owner
genérico. Semântica de outras famílias segue no seu owner.

## R2-S13 — uma avaliação por grupo textual pendente

Medição antes da decisão: r2s-scope.json.gz e text-before.json.gz em
00_nucleo/diagnosticos/sequence-constructor-2026-10-17 preservam estado
não commitado em b1b486f3d42431f8ea360bc5712637ee1e3d4f6c, UTC,
diffstat, binários e pin bd5ebec285505889e8ff1d4f68804ab147a699f3.
As sondas textual-no-repeat/baseline/interpolation/captured produzem
AAAA(B) no Tekt contra AA(B) no vanilla. textual.rs:104–122 encapsula
grupos que já contêm receitas pendentes do mesmo snapshot;
realize/mod.rs:182–210 realiza ambas bottom-up, reaplicando o literal.
text-before cross-piece mede AB→X sobre filhos separados nos dois;
outer-hides-text mede Y sem executar panic do filho em ambos.

Decisão interna, dentro de S13 ratificado: o owner textual coalesce
somente receitas textuais ainda pendentes do mesmo ambiente causal
(identidade Arc das regras, estilos colapsados, guards e arquivo).
Preservar Source, ordem, morfologia e fronteiras com ambiente distinto
ou recipe não textual com efeito de elemento (a exceção escalar redundante está prescrita abaixo). Nenhum callback é executado pelo coalescimento.
Capturar o grupo com filhos coalescidos e, no owner Sequence, coalescer
o corpo textual depois de realizar as sequências efetivamente emitidas,
antes de devolver a receita ao owner textual. Não reaplicar o mesmo
grupo por cada camada de transporte; não pular Realized indiscriminadamente
nem alterar matching, precedência, scopes ou contratos públicos.

Testes RED→GREEN: literal e regex em interpolação/captura, identidade
Sequence simples e em bloco, prefixo lexical, matching entre peças e
filhos textuais ocultos. Fronteiras de snapshot distintas permanecem
opacas. A divergência independente local-text-scope (C vs B), causada
pela precedência de literais distintos, continua explícita no inventário
e não legitima alterar a ordem do matcher neste owner.

## Divisão explícita — pendências após R2/R2-S

Medição e proveniência: r2s-scope.json.gz/text-before.json.gz no diagnóstico
00_nucleo/diagnosticos/sequence-constructor-2026-10-17. Este pacote é
incompleto no universo de morfologia e regras textuais. R3-Symbol deve
completar identidade/igualdade do cast de Symbol (text/true no Tekt contra
symbol/false no vanilla); exige proposta/gate próprio se criar variante
pública, que R2-S não autoriza. R3-Markup deve preservar a Sequence
interpolada como filho imediato no valor de ContentBlock sem regra de
Sequence, hoje aplanada. R3-Textual deve medir/corrigir a precedência dos
literais em escopos distintos (C vs B), no owner matching. R3-Diagnostics
deve completar rastros genéricos de show de filhos emitidos. Estes nomes
identificam os pacotes de conclusão; não são prompts L0 nem autorização
para código adicional. Não classificar tais diferenças como mecânica
aceita ou paridade já demonstrada.

## R2-S — slots internos não são Empty explícito

Medição anterior à decisão: empty-slots-before.json.gz em
00_nucleo/diagnosticos/sequence-constructor-2026-10-17 registra fontes,
binários, UTC, HEAD/working tree/diffstat e pin bd5ebec285505889e8ff1d4f68804ab147a699f3.
Body simples produz EBODY contra BODY; filhos none/[] produzem EEA contra
EA; com regra textual A→AA aparecem Es adicionais. markup_stream.rs:
205–225 monta cauda incluindo slots Empty consumidos/normalizados;
textual.rs:104–120 monta pacotes com os mesmos slots; sequences.rs:
114–125 os visita como ocorrências reais.

Correção interna dentro da captura ratificada: ao montar o corpo da cauda
Sequence no owner markup e o corpo do pacote pendente no owner textual,
remover somente slots Content::Empty nus do vetor. As ocorrências Empty
explícitas sob regra Sequence já possuem captura/Source própria antes
de entrar nesse vetor. Conservar esses carriers, Text vazio e arrays
de filhos do construtor; não filtrar Empty arbitrariamente no visitor
de domínio. Corpus body/[]/none/construtor vazio/filho none/filho []/prefixo/
textual deve coincidir com o vanilla PDF, inclusive documento sem corpo.
L0 primeiro, RED em módulo antes do código; revalidar PDF e HTML separadamente.

## R2-S17 — vazio de scaffolding HTML não é ocorrência do autor

Medição antes da classificação: target-controls.json.gz e
footnotes-controls.json.gz em 00_nucleo/diagnosticos/sequence-constructor-2026-10-17
conservam estado não commitado/UTC/diffstat/fontes/argv/binários no pin
bd5ebec285505889e8ff1d4f68804ab147a699f3. HTML do vanilla bare-show
contém <p>Y</p>Y; PDF contém Y. Um filtro children:() faz o vanilla
acrescentar FOOTER após BODY apenas no HTML automático; html.body
explícito retira esse acréscimo no vanilla. O Tekt dessa coleta tem
o defeito independente EBODY de slots, tratado acima, sem aceitá-lo.

Fonte: lab/typst-original/crates/typst-html/src/document.rs:293–302
sempre realiza FootnoteContainer e anexa seu fragmento ao body automático.
rules.rs:382–387 pretende não criar contêiner sem notas, mas retorna
Content::empty; typst-realize/src/lib.rs:259–270 aplica show antes de
expandir Sequence. Inferência apoiada pelo controle com html.body: o
texto extra é vazamento do vazio interno de notas para o show do autor,
não conteúdo de uma nota ou ocorrência escrita pelo autor. Seria
refutada se o body explícito conservasse esse acréscimo ou se a origem
fosse conteúdo do autor. Não emular esse artefato de render no Tekt.
Preservar show de [] e de filhos Empty explicitamente construídos;
a intenção não é inferida só do comportamento, mas do comentário e
do caminho de fonte com controle diferencial. A diferença por target
continua registrada individualmente, sem declarar igualdade HTML geral.

### Medição de replay nativo — incorporado a R2-S13

A coleta emission-before.json.gz no diagnóstico de R2 confirma AA nos
dois compiladores quando Sequence emite [A] ou text("A") sob show A→AA.
O controle sem Sequence, #text("A"), ainda produz AAAA contra AA: há
replay genérico não textual separado dos pacotes textuais coalescidos.
O reparo desse replay escalar foi incorporado a R2-S13 abaixo. Receitas
genéricas com callbacks aplicáveis conservam sua barreira; não as remover
para esconder diferenças de realização.

## R2-S — raiz de pacote textual e EOF causal

Medição anterior à decisão: validation-attempt-2.json.gz, trace-recipes,
trace-markup2 e trace-textual2 em 00_nucleo/diagnosticos/sequence-constructor-2026-10-17
mostram 11 testes verdes e dois REDs. eof-before.json.gz mede as fontes
exatas sem acrescentar newline: Styled com EOF produz Y nos dois; sem
EOF produz AB nos dois. A expectativa unitária Y usava a fonte sem EOF,
diferente da sonda anterior. Preservar ambos os controles externos; o
unitário Y passa a incluir o EOF medido, não a relaxar uma regra de show.

Alias textual sem EOF produz AAAAAAAAB no Tekt e AAB no vanilla.
sequences.rs:112–116 pula a raiz de todo pacote textual, mas a construção
Content::sequence normaliza um único item para o próprio item. Quando
esse item é ShowRecipe marcado, pular a raiz deixa a receita original
pendente e realiza outra captura nos seus filhos. O owner genérico volta
a executá-la e repete regras textuais. trace-textual2 registra essa
receita marcada ainda opaca no coalescimento após o percurso.

Correção interna: skip_root pula somente uma raiz crua Empty/Sequence
artificial de pacote, nunca ShowRecipe/Styled ou outra raiz normalizada.
Uma receita marcada na raiz efetiva deve decidir antes de seus filhos,
como qualquer outra ocorrência. Manter comparação causal estrita de
estilos/regras/guards/arquivo no coalescimento e todos os controles de
slots. Os logs cfg(test) temporários foram arquivados e serão removidos
do código antes da validação final. Sem campo/tipo/API/fase nova.

## R2-S13 — texto emitido e receitas escalares redundantes

Medição antes da decisão: validation-attempt-3.json.gz e emission-after.json.gz
em 00_nucleo/diagnosticos/sequence-constructor-2026-10-17 preservam estado
working tree, UTC, fontes/argv/binários/diffstat no pin bd5ebec285505889e8ff1d4f68804ab147a699f3.
Sequence emitindo [A] ou text("A") sob A→AA produz AAAA contra AA.
O controle text("A") sem Sequence já produz AAAA no release de pilha
SHA 2ab1495d555ff4d61e67eb14bef67597f7f668a0e3bce3a8129623011443441c
(native-before.json.gz). textual.rs:142–161 compara marca/guard de
Sequence como se distinguissem escopo textual e conserva receitas
genéricas escalares que aplicam o mesmo conjunto literal antes do pai.
trace-textual2/instrumentation-sources registram esses campos privados.

Responsabilidade do owner Sequence: fornecer helper crate-interno que
compara o transporte textual excluindo somente a chave privada
eval.sequence.occurrence do delta colapsado e guards cujo RuleId pertence
a regra Sequence no snapshot. Guards desconhecidos e de outras famílias
permanecem. Nenhuma filtragem de estilos reais nem assinatura externa.

Responsabilidade do owner textual: conservar agrupamento lexical estrito
TextualSnapshot::same_group. No coalescimento, pacotes textuais com
mesmo Arc de regras, arquivo e transporte textual equivalente são uma
única realização. Para receita genérica não textual, só eliminar o
transporte redundante se o corpo bruto for Text (permitido Source),
as regras ordenadas têm exatamente os mesmos RuleIds do pai, o arquivo
e o transporte textual coincidem, e nenhuma regra de elemento casa
com esse Text. Nesse caso não há callback de elemento a executar e
o pai já possui a aplicação literal/regex correspondente; devolver o
corpo com sua Source. Não eliminar receita Styled/estrutural/contextual
ou receita com regra de elemento aplicável. Não retirar regras do Engine
nem executar callbacks no coalescimento. IDs são a identidade já vigente
no mesmo contexto; não comparar só nomes, Debug ou texto do produto.

RED→GREEN de emissão [A]/text("A") e controle escalar nativo, com os
esperados do pin; controles de guard de outra família e estilo real
continuam opacos. Revalidar os 13 controles anteriores sem relaxamento.
Esta é correção interna de aplicação única já ratificada em S13, sem
novo tipo/campo público, fase ou mudança da política dos demais elementos.

## R2-S — origem somente quando a captura é necessária

Medição antes da decisão: validation-attempt-4.json.gz, complete-workspace.log
e conditional-origin-before-sources.json.gz em 00_nucleo/diagnosticos/sequence-constructor-2026-10-17
conservam HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c, estado não
commitado/UTC/diffstat/fontes/binários e pin bd5ebec285505889e8ff1d4f68804ab147a699f3.
A suíte de core passou 7228 testes, falhou 24 e ignorou 5. Os dois
controles de numbering local deixam de encontrar elementos internos;
os demais falham na geometria do prefixo/cauda após 1fr. A referência R1
validada antes desta etapa tinha zero falhas nesses testes.

markup_stream.rs passa content.with_source_span(child.span()) ao helper
mesmo quando não há regra Sequence ou quando o corpo não é Sequence.
O helper devolve esse novo carrier mesmo sem capturar. Dentro de um
ContentBlock, append_markup_sequence preserva Source, alterando as
fronteiras legadas de estilos e coluna que não precisavam de captura.
A adição de origem deve pertencer à operação efetiva de captura.

Correção interna da rota aprovada: a fachada crate-interna capture_at
recebe corpo cru e Span real; só acrescenta Source quando apply_show_rules
está ativo, existe snapshot pertinente e o corpo é Empty/Sequence.
capture conserva a assinatura interna atual e delega com Span detached;
capture_scope recebe Span explícito e só o usa dentro do ramo que cria
a receita marcada. Markup e CodeBlock passam o corpo sem embrulho prévio
e a origem real separadamente. Nenhuma regra pertinente ou corpo não
capturável devolve exatamente o conteúdo recebido; isso preserva os
escopos por defeito e evita alocar carriers inúteis. Não modificar os
algoritmos de layout nem os esperados dos 24 testes.

RED já observado na suíte integral; exigir GREEN de f2s, integração de
set/prefixo/1fr e controles Sequence antes da nova validação completa.
Manter origem de [] e chamadas, guards nominais, não localização e
coalescimento causal. Nenhuma assinatura Rust externa, campo público,
modo padrão ou fase adicional é criada.

## R3-T1 — realização causal ratificada (2026-10-18)

### Medição anterior à decisão

00_nucleo/diagnosticos/textual-realization-2026-10-18/{before,scope,refuters}.json.gz
preserva HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c, working tree não
commitado, UTC, status/diffstat, fontes/argv/hashes dos binários e pin vanilla
bd5ebec285505889e8ff1d4f68804ab147a699f3. before-sources.json.gz contém
fontes/L0 vigentes completos. Não há código candidato implementado nesta medição.
matching.rs:118–150 aplica regex reversa e depois literais em declaração;
:194–207 volta a expor produtos a passes subsequentes. textual.rs:102–123
captura snapshot completo. realize/mod.rs:177–217 visita receitas bottom-up.
apply.rs:620–648 e matching.rs:218–225 chamam funções sem capturar os filhos.
expression_dispatch.rs:125–129 já distingue insertion_rules do estado local.

Exemplos medidos: literais idênticos C/B; empate literal/regex R/L; overlap
X/YB; primeiro efeito LATER/FIRST; função encadeada inversa BBBB/BB;
produto separado AB/X; produto+vizinho X/AB; Strong ocultando filho dá
panic HIDDEN/Q; global capturado seguido de global nova dá L/R, mas local
capturado dá L/L. Entrada CJK local 水\n果 dá 水果/X. A fonte pinada
foundations/selector.rs:110–125 unifica literal escapado com regex;
typst-realize/src/lib.rs:1328–1405 escolhe o início e carrega revogação
no produto. Esses dados medem linguagem/efeitos; não ratificam mecânica Rust,
igualdade de buffers ou bytes PDF. É inferência causal que a separação de
escopos e a visita anterior aos filhos expliquem o conjunto; os controles de
acceptance.json, igualdade/fields/estilo/query e erros podem refutá-la.

### Decisão ratificada

Aplicar o núcleo textual-recipe-frontier pinado neste L0. A confirmação humana
do manifesto 47e714dfbb813eae9f2be20b8f57de5c197cda13db1871c8bb3da49a08430302
ratificou esta ordem. Ele supersede neste escopo as
cláusulas históricas de execução provisória de show em ContentBlock e da
visita bottom-up de receitas pendentes, sem alterar a ordem de avaliação
de expressões comuns. Não reavaliar o programa, antecipar State/Counter,
assentar render em morfologia nem criar contrato Rust público.

### Responsabilidade exclusiva desta unidade

Conservar captura e passo nominal de raiz de Sequence já ratificados. O coordenador privado chama este owner na ocorrência efetivamente emitida, antes dos seus filhos, sem varrer primeiro todas as receitas genéricas do documento. Não executar receitas textuais nesta unidade nem achatar a Sequence antes de selecioná-la. Guards, alias/fresh, Source, estilos e S1–S20 permanecem; o núcleo anterior sequence-realization continua obrigatório.

### Aceitação desta revisão

RED→GREEN das famílias de acceptance.json, corpus R2-S/R3 e controles de uma avaliação. Revalidar PDF e HTML separadamente; build/workspace/doctests, Clippy/Rustdoc estritos, fmt, V1/V5/V15/V26 e linhagem reversa; diferencial SARIF sem novo erro/warning. Teto 12 GiB/High8 GiB/swap0/jobs1 e comandos pesados sequenciais. Não alegar ganho de RAM antes de ensaio pareado nem zerar o gate com supressão. Sem commit/push/release.

### R3-T1 — entrada única do coordenador

A antiga fachada crate-interna sequences::realize deixa de ser necessária após
a substituição do prepass global. Removê-la; o coordenador recipes é a entrada
única e chama realize_root para a decisão nominal de Sequence. Preservar a
seleção, os guards e a distinção entre alias e reconstrução fresca.

### R3-T1 — reconhecimento do packet antes de coalescer

O predicado marked é visível apenas dentro de crate::compiler para permitir
ao owner textual preservar recipes Sequence antes de coalescer. Ele continua
lendo somente o marcador privado existente; nenhuma API pública é criada.
