# P1339 — revisão do desenho de query/introspector e counter

Data de inspeção: 2026-09-10, encerramento de leituras aproximadamente
00:22 UTC. HEAD produtivo: `2f42d64253547734564513a1159ee6b584c1c4b4`.
Investigação sem edição produtiva ou redação de L0; único artefato escrito por
esta autoridade é este diagnóstico. Regime: executado sem atestação de
isolamento. O agente recebeu o contexto e a proposta da autoridade principal;
leu os L0s e fontes antecedentes. Isto é revisão arquitetural, não contrato
selado, teste A/B independente ou PASS de implementação.

Skill lida: `tekt-materializacao-segregada/SKILL.md`, incluindo
`references/papeis-e-capacidades.md` e `references/artefatos-e-gates.md`.
Nenhuma ADR de segregação foi encontrada por busca textual limitada a
`00_nucleo/adr/`. Não foram lidos nem listados materialization/context.

## Proveniência e limites das leituras

À abertura, `git diff HEAD --stat` mostrava somente:

```text
 .../compiler/eval/bindings/value_methods.md        | 88 ++++++++++++++++++++++
 .../prompts/compiler/eval/operators/equality.md    | 43 +++++++++++
 .../prompts/compiler/eval/selector_matching.md     | 58 ++++++++++++++
 00_nucleo/prompts/entities/selector.md             | 80 ++++++++++++++++++++
 00_nucleo/prompts/entities/show.md                 | 46 +++++++++++
 5 files changed, 315 insertions(+)
```

A autoridade principal continuou a editar documentação durante esta revisão.
No final das leituras, o mesmo comando mostrava:

```text
 .../compiler/eval/bindings/value_methods.md        | 89 ++++++++++++++++++++++
 .../prompts/compiler/eval/operators/equality.md    | 44 +++++++++++
 00_nucleo/prompts/compiler/eval/repr.md             | 38 +++++++++
 00_nucleo/prompts/compiler/eval/rules.md            | 34 +++++++++
 .../prompts/compiler/eval/selector_matching.md     | 59 ++++++++++++++
 00_nucleo/prompts/entities/selector.md              | 82 ++++++++++++++++++++
 00_nucleo/prompts/entities/show.md                  | 48 ++++++++++++
 7 files changed, 394 insertions(+)
```

Todas as referências produtivas abaixo são ao antecedente sem diff produtivo.
As medições vanilla citadas são do recibo
`00_nucleo/diagnosticos/p1339-where-integration-probe-runs.json`, que contém
comandos, fixtures, horários, HEAD e estado da árvore; não foram reexecutadas
por esta autoridade. A referência upstream é o ratificado `a51e02804`.
Números de linhas dos L0s podem deslocar-se pelas edições concorrentes; as
cláusulas citadas também são identificadas pelo nome.

## Medições anteriores à decisão

1. `01_core/src/entities/introspector.rs:137` exige
   `query(&Selector) -> Vec<Location>`; o bloco de implementação começa em
   `:559` e o match exaustivo em `:634`. And/Or/Within recursam pelo mesmo
   método (`:651`, `:664`, `:689`). Um remendo apenas em native_query deixaria
   a API Rust e esses consumidores sem semântica uniforme.
2. O store já guarda `IntrospectedContent` em `introspector.rs:372`.
   `entities/value.rs:26-45` contém árvore e snapshot opcional acessíveis
   separadamente. O Núcleo `introspection/content-snapshot.toml` exige que
   Some seja autoridade completa: campo ausente não admite fallback individual.
3. A identidade real está acessível por `Func::native_fn_addr`
   (`entities/func.rs:315-338`), e o reconhecimento dos endereços dos builtins
   pertence a compiler. `compiler/eval/operators/equality.rs:101` possui o
   comparador de linguagem. Os L0s P1339 de selector_matching e equality
   permitem reutilização interna estática, mantendo entidades sem novo import
   de compiler. O fallback de nome de `content_elem_func` não prova identidade.
4. `compiler/stdlib/foundations/query.rs:52-63` já obtém Locations e preserva
   a entrada completa ao construir LocatedContent. `:76-77` mostra que locate
   também chama o método de trait. Não há falta de carrier de campos para
   Heading já indexado.
5. `entities/counter.rs:22-25` já transporta Selector inteiro em CounterKey.
   Porém `compiler/stdlib/counter.rs:92-112` reduz Where/Within à base e
   And/Or ao primeiro filho. Isso perde filtros no antecedente.
6. Os steps automáticos de Heading/Figure/Equation/Table/Footnote são escritos
   sob `CounterKey::Selector(Kind(...))`, em
   `compiler/introspect.rs:799-811`, `:828-834`, `:957-965`, `:996-1018`.
   Clonar Element na chave do construtor preservaria repr mas deixaria a chave
   nova sem os steps desses elementos.
7. `CounterRegistry` tem somente estado final e histórico de estados por chave
   (`entities/counter_registry.rs:24-34`). Não tem journal de ações automáticas
   e explícitas. O histórico não permite reconstruir sem ambiguidade uma ação
   a partir da diferença entre dois estados: set, step e callback podem
   produzir o mesmo estado.
8. O contrato de leitura de counter é emprestado:
   `introspector.rs:165-174`, `:734-752` retorna `Option<&[usize]>`.
   Computar Vec temporário dentro dessa implementação não produz um slice
   válido para retornar. `counter.rs:311-315`, `:522-546` consome esse contrato.
9. O callback real de counter ocorre no pós-walk já existente
   (`compiler/introspect/from_tags.rs:85-136`), lendo estado por chave antes de
   `apply_func`. O fixpoint chama esse passo antes de displays
   (`compiler/introspect/fixpoint.rs:111-126`). Mover execução de callback para
   query read-only mudaria disponibilidade de Engine, ordem e responsabilidades.
10. O antecedente não guarda Strong/Emph: `locatable.rs:157-159` classifica
    Styled/Strong/Emph como não locatáveis; `introspect.rs:1278-1363` só aloca
    Location e grava elements quando `extract_payload` retorna Some.
    `introspect.rs:1398-1401` atravessa Strong/Emph de forma transparente.
11. Na fonte ratificada, `model/strong.rs:21` e `model/emph.rs:26` declaram
    Locatable e Tagged. O recibo novo registra query contextual de ambos com
    resultado `[1]` nos casos historicamente nomeados `query_reject_strong` e
    `query_reject_emph`; os nomes desses casos não são seu veredito. Text é
    rejeitado. O recibo também mede counter filtrado por level 2 com `[0,1]`,
    quando o bare retorna `[2]`; contagem de matches não reproduz hierarquia.
12. O L0 `compiler/introspect/locatable.md:71` exige
    `is_locatable(c) == extract_payload(c).is_some()`. Tag::Start exige
    ElementInfo (`entities/tag.rs:21`), cujo payload é um enum fechado. Não há
    payload nativo Strong/Emph no antecedente. O parent index usa tags
    (`introspect.rs:1074`); convergência também usa tags
    (`fixpoint.rs:111`, `:128-130`).

## Decisão: o problema de camada tem solução interna concreta

Para os elementos que já possuem ocorrência indexada, os dados aprovados são
suficientes para filtrar query sem nova API pública. A adaptação tecnicamente
coerente é deslocar **todo** o bloco `impl Introspector for TagIntrospector`
para um consumer proprietário em compiler, mantendo trait, tipo e assinaturas
exatamente como estão. Em Rust a disponibilidade de um impl não depende do
arquivo que o contém. O local candidato natural é uma unidade de introspecção
em compiler, com L0 1:1 próprio; a escolha exata ainda exige auditoria do L0
de módulo e wiring. Este diagnóstico não nucleia esse novo consumer.

O query desse impl pode então reconhecer a identidade nativa pelo helper
estático de compiler e filtrar as entradas existentes com o comparador único
de equality. Para snapshot Some lê apenas seus campos; para None usa a
projeção legada contratada. Deve preservar as ordens anteriores das variantes
antigas; para o braço novo, percorre ocorrências em ordem documental, nunca
ordem aleatória de HashMap. Location é o identificador causal, não igualdade
de Content nem nome de função.

Mover somente o corpo de query e deixar a entidade chamar compiler, direta
ou indiretamente por um método inerente definido em compiler, não resolve a
dependência arquitetural. Duplicar values_eq nas entidades também não resolve.
Um wrapper exclusivo de native_query não satisfaz o método exaustivo nem locate.

A mudança de localização do impl, isolada, não adiciona contrato público nem
muda fase; pode ser refatoração interna em fluxo contínuo, após atualizar os
owners. Isto é avaliação de viabilidade da mecânica, não autorização para
materializar antes de L0 e gates.

## Decisão: counter exige desenho de eventos e armazenamento

Element pode ser preservado integralmente em CounterKey, inclusive grupo
vazio. A sequência correta de um counter filtrado reúne apenas ocorrências
que casam o seletor e os updates explícitos dessa chave, em ordem documental.
Um Heading correspondente aplica step no seu nível; Figure/Equation conservam
suas condições de contagem; um elemento locatável sem ação específica aplica
step simples. Esta última regra está na fonte ratificada de counter,
`introspection/counter.rs:938-955`.

Para materializar isso sem novo trait é necessário guardar antecipadamente
históricos das chaves efetivamente demandadas, incluindo as que aparecem
apenas em get/final/display. O registro deve acontecer antes da resolução
da sequência e sobreviver às substituições do introspector no fixpoint; os
callbacks explícitos precisam receber o estado já filtrado. A API pública
do CounterRegistry não precisa necessariamente mudar: campos privados e
helpers internos podem transportar demandas/eventos. Mas esse protocolo
**não existe no antecedente** e não está especificado pelos L0s aprovados.

Não recomendo declarar fechado um algoritmo que enumera só chaves presentes
em CounterUpdate: `counter(heading.where(...)).get()` pode não emitir nenhum
update explícito. Também não recomendo reconstruir ações a partir do histórico
de Kind, usar quantidade de matches, fundir grupo vazio com Kind ou usar o
Hash/PartialEq estrutural como comparador de filtros da linguagem.

Uma alternativa de helper privado em stdlib que devolva Vec próprio pode
resolver leituras da linguagem a partir de um journal completo, mas por si só
deixa os métodos públicos emprestados sem o mesmo resultado e não elimina a
necessidade de resolver callbacks na fase apropriada. Se a solução escolhida
trocar o retorno do trait para Vec ou mover callbacks de fase, há gate
ADR-0127 adicional. Não é lícito inferir esse novo contrato da aprovação dos
dois enums.

## Refutação da suficiência para a integração geral pedida

O carrier de Selector conserva a intenção do filtro. Não cria as ocorrências
Strong/Emph que o store e as tags atuais omitem. A sonda positiva e as marcas
Locatable/Tagged da fonte refutam uma aceitação geral baseada apenas em
Heading/Figure e também refutam um retorno vazio silencioso nesses elementos.

Mantendo os invariantes de indexação vigentes, promover Strong/Emph exige um
payload de evento fiel e sincronização de extração, walk, parents, convergência
e posições. Acrescentar variantes/campos ao enum público ElementPayload é
contrato adicional e requer gate; se a estratégia usar ElementKind, estendê-lo
também exige gate. A aprovação original proíbe inferir essas extensões.
Um payload existente de Metadata, Heading ou outro tipo não pode ser usado
como sentinela de Strong/Emph: alteraria identidade e observáveis desse tipo.

Existe uma alternativa conceitual sem novos campos públicos: indexar
ocorrências diretamente no store atual, mantendo ordem/ancestralidade por
estado privado do walk e revendo como posições e convergência identificam
tais ocorrências sem tags. Ela exige substituir explicitamente o invariante
de equivalência com payload e reconciliar a alocação de Location entre walk
e layout. Não foi demonstrada nesta revisão, não é um mero braço novo e não
pode ser adotada incidentalmente sob os L0s atuais. Não afirmo impossibilidade
matemática de toda solução interna; afirmo que a suficiência dos carriers para
o desenho vigente foi refutada, e que não há base para declarar esse redesenho
livre de gate sem especificá-lo e classificá-lo primeiro.

Recomendação: conservar a aprovação dos dois enums e separar a nucleação
da promoção das ocorrências e do runtime de counter. Se for mantida a
arquitetura atual de tags, apresentar ao dono o contrato adicional concreto
antes da implementação dessa promoção. Query das famílias já indexadas e show
podem ter trabalho interno próprio, mas não devem ser apresentados como
fechamento geral de function.where/query/counter. Nenhuma implementação foi
validada neste diagnóstico.
