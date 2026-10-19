# L0 — compiler/eval/rules/textual
Hash do Código: 989fa241

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/textual-recipe-frontier.toml sha256:f225f3039d4663e9bfcb408d26cf1cd6e557f089013bc2d95090ce16c52080c6

- 00_nucleo/prompts/_nuclei/eval/core.toml sha256:e7642a709c937928333439b2a78cdb3a6dbd6b56d67fcc67728efd2a26796e58
- 00_nucleo/prompts/_nuclei/eval/pending-text.toml sha256:3f300957a00ac2f02354331585c19b935428e05d5b8ac2115a533764262250c8
- 00_nucleo/prompts/_nuclei/eval/textual-consumed-slots.toml sha256:faaf206c9f4c20eba8b18b463ebe7836cb9671fd8672501c068eef2751f88d3c

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/rules/textual.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Measurement before decision

At the measured working tree, `01_core/src/compiler/eval/rules.rs:3932-4435`
contains `TextualSnapshot`, text-node interception, grouping, rule slicing,
space normalization and their focal tests. `compiler/eval/mod.rs:1371,1408,
1416,1470,1550` calls the three entrypoints. The existing `p1494` test filter
passes 17/17 before movement; this is structural baseline, not a new bilateral
parity measurement. Source state and command are recorded in
`00_nucleo/diagnosticos/atomizacao-textual-2026-09-30.md`.

## Decision and ownership

Move that behavior to this descendant module without changing algorithms,
order, diagnostics, spans or language morphology. The parent `rules.rs` retains
set/show dispatch and one-node show application, and exposes this child to the
eval coordinator. No public signature, default, pipeline phase or compatibility
changes. The native dispatcher remains static and exhaustive; the child does
not import render or infra and does not create dynamic dispatch.

`TextualSnapshot` captures the causal rule/style/guard state. The two callable
entrypoints are `intercept_text_element` and `realize_textual_parts`; the
behavioral helpers and focused tests live in the two descendant owners. Calls
from `eval/mod.rs` refer to this coordinator; no copied implementation remains
in `rules.rs`. The shared one-node show application and selector semantics remain
owned respectively by `compiler/eval/rules.md` and
`compiler/eval/selector_matching.md`.

## Refinement measurement

At the working tree captured in
`00_nucleo/diagnosticos/atomizacao-textual-algoritmos-2026-09-30-before.json.gz`,
this consumer contains the coordinator, the space algorithm at lines 335-463
and the matching algorithm at lines 96-333. The `p1494` filter passed 17 tests
before this second movement. The distinct inputs and helpers support two
behavioral owners; no CPU reduction follows from that measurement.

## Behavioral delegation

The coordinator retains `TextualSnapshot`, `intercept_text_element` and
`realize_textual_parts` with their existing signatures and phase. It invokes
space normalization only when `normalize_spaces` is true, then returns early
when show is disabled, then invokes the matching owner. Snapshot grouping
retains pointer identity for rules and equality of collapsed style and guard
state. Element-selector interception still uses the one-node show application
owned by `compiler/eval/rules.md`; textual selectors remain deferred.

`compiler/eval/rules/textual/space.md` exclusively owns collapse of adjacent
spaces, newline origin, explicit linebreak consumption and the pinned Unicode
spacing predicate. `compiler/eval/rules/textual/matching.md` exclusively owns
realized group traversal, selector precedence, matched-slice transformation,
while its descendant `compiler/eval/rules/textual/matching/projection.md`
exclusively owns preservation of unmatched original content. These algorithms stay in the
eval layer as descendant modules. This coordinator must not duplicate either
algorithm or introduce dynamic dispatch.

## Acceptance

The coordinator tests snapshot group identity across shared rules, distinct
rule allocations, collapsed style changes and guard changes. The existing focal
tests remain green, including literal/regex slicing, Unicode space handling,
lexical snapshots and guard behavior. Build and
`crystalline-lint .` have no new violations. Language content, diagnostics,
ordering and public contract are unchanged. No performance gain is claimed
without separate measurement. Header ownership remains 1:1.


## Realização com avaliação única

Com defer_show_rules, capturar grupos por snapshot lexical e fronteira de estilo antes de executar receitas textuais. Capturar também grupos cujo texto só surge da realização de elementos; não decidir match pela forma pré-show. Em realização, reutilizar matching/splice vigente, sem executar programa comum novamente.


## Realização com avaliação única

Fachada interna realize_captured_text realiza elementos já resolvidos no grupo, conserva a origem causal do grupo e delega matching/splice ao owner existente. Sem perda de origens por substituição textual.


## Realização com avaliação única

Normalização de espaço vigente é reutilizada internamente após realização de receitas; não duplicar regra Unicode de inserção.


## Realização com avaliação única

Snapshot de grupo textual conserva watermark contextual, incluindo conteúdo lexical capturado não inserido na árvore visual inicial.

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


## R3 — slots consumidos e apresentação de None (2026-10-18)

### Medição anterior à decisão

00_nucleo/diagnosticos/sequence-r3-2026-10-18/{before,markup-after,
textual-before,transport-before,transport-before-sources}.json.gz conserva
HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c, working tree não commitado,
UTC/diffstat/fontes, executáveis e pin bd5ebec285505889e8ff1d4f68804ab147a699f3.
Sob show literal Z que não casa, nested.children expõe um pacote Sequence
mais dois Empty no Tekt; no pin são os três filhos originais. empty-child
expõe pacote mais Empty no Tekt, mas []/A no pin. markup_stream.rs:201–249
reconstitui o vetor incluindo slots que textual.rs:103–129 zerou; matching.rs:
90–95 também zera os siblings consumidos. São detalhes de transporte vazando
para a morfologia pública. CodeBlock/None interpolado desaparece; [#none#[]A]
tem dois filhos no Tekt e três no pin, e X#none;B sob show Empty resulta XB
contra XEB. A tabela value_to_display_content omite None por contrato histórico.
No pin typst-library/src/foundations/value.rs:191–210 apresenta None como
Content::empty; typst-eval/src/markup.rs:39–56 trata SetRule/ShowRule como
fronteiras próprias e :78 apensa o valor das demais expressões. Não copiar
buffers Rust nem inferir intenção a partir da implementação.

### Decisão e limites

Correção interna de paridade ADR-0127, sem contrato Rust externo, novo campo,
modo/default ou fase. Máscara privada Vec<bool>, sem Location falsa ou carrier
público novo. A hipótese é refutada por perda de Empty/Text vazio genuíno,
deslocamento de fronteira/estilo, repetição de callback ou índices inconsistentes.
A apresentação de None muda apenas na inserção do markup, não no helper genérico
usado por Math/CodeSetTail. SetRule/ShowRule continuam suas fronteiras próprias.

RED funcional antes de código: children/fields sob receita literal não casada,
filhos []/Text vazio, valor None de literal/let/assert/condicional, aplicação do
show Empty a None e set vazio/estilo sem Empty artificial. GREEN e corpus
bilateral, controles R2-S/EOF/slots/estilos/CJK/loops, build/suítes completas,
Clippy/Rustdoc estritos, formato/linhagem/reversa/diferencial SARIF. 12GiB,
High8GiB, swap zero, jobs1 e jobs pesados sequenciais; sem publicação.

A preservação de fronteiras do valor de markup não fecha a morfologia pública
de Styled/Styles, repr de labels ou a formação do valor de módulo/include.
R3-Reflection e o lote A3/include completarão esses eixos com medição própria.
Precedência/guardas textuais e show-set literal continuam no lote R3-Textual;
nenhuma mudança dessa política é legitimada por este transporte.

Responsabilidade exclusiva: realize_textual_parts retorna SourceResult<Vec<bool>>
crate-interno com o mesmo comprimento de parts, inicialmente false. Na captura
adiada, marcar start+1..end ao consumir seus slots. Sem show, retornar a máscara
vazia de consumo. Na rota imediata delegar a máscara produzida por matching.
realize_captured_text usa a máscara para montar somente o visual dos slots
preservados; original continua completo. Não alterar coalescimento/guards,
ordem de regras ou normalização de espaço nesta revisão.


### Limite adicional medido — grupos textuais em múltiplas caudas

transport-after.json.gz no diagnóstico R3 confirma a remoção dos Empty
consumidos e a presença dos None reais. style-tail ainda expõe um grupo
sintético Sequence(A,Space) como um filho, antes do Styled; rule-tail também
conserva forma de cauda divergente. O problema é adicional à máscara de consumo,
não somente uma diferença de repr. O pacote R3-TextualGroups completará a
projeção linguística dos grupos sintéticos em múltiplos escopos e sua igualdade/
seleção, preservando a Sequence escrita pelo autor. R3-Reflection completará
Styles e Styled públicos. Não considerar esses controles fechados nem achatar
Sequence genericamente para escondê-los. Este adendo registra o recorte ainda
incompleto; não autoriza modificar os owners dessas projeções sem L0 próprio.

## R3-T1 — candidato, NÃO vigente antes do gate de ordem (2026-10-18)

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

### Decisão sujeita a aprovação

Aplicar o núcleo textual-recipe-frontier pinado abaixo, somente depois da
confirmação humana do manifesto da revisão. Ele supersede neste escopo as
cláusulas históricas de execução provisória de show em ContentBlock e da
visita bottom-up de receitas pendentes, sem alterar a ordem de avaliação
de expressões comuns. Não reavaliar o programa, antecipar State/Counter,
assentar render em morfologia nem criar contrato Rust público.

### Responsabilidade exclusiva desta unidade

Possuir frames/snapshots e transporte de conclusão textual, sem publicar tipos. Compartilhar baseline/regras por Arc; transportar a classificação herdada/local no ShowRecipe/styles privado, com grafo acíclico e sem Location fictícia. Metadado de baseline guarda snapshot de regras e não um conteúdo executável; não guardar o carrier dentro dele próprio. Coalescer apenas pacotes PENDENTES do mesmo ambiente; conclusão de produto preserva sua fronteira. Expor helpers apenas crate/compiler para o coordenador. Restaurar state e guards mesmo quando matcher/callback falha.

### Aceitação desta revisão

RED→GREEN das famílias de acceptance.json, corpus R2-S/R3 e controles de uma avaliação. Revalidar PDF e HTML separadamente; build/workspace/doctests, Clippy/Rustdoc estritos, fmt, V1/V5/V15/V26 e linhagem reversa; diferencial SARIF sem novo erro/warning. Teto 12 GiB/High8 GiB/swap0/jobs1 e comandos pesados sequenciais. Não alegar ganho de RAM antes de ensaio pareado nem zerar o gate com supressão. Sem commit/push/release.
