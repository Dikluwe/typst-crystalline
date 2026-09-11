# P1339 — desenho de integração de counters filtrados

Regime: **executado sem atestação de isolamento**. Autor do diagnóstico:
`/root/p1339_counter_integration_design`. Este documento não é L0, contrato
selado, implementação, oráculo nem veredito de paridade. Recomendações abaixo
precisam de nucleação pelos owners antes de código.

## Proveniência, autoridade e limites

HEAD inspecionado: `2f42d64253547734564513a1159ee6b584c1c4b4`.
Working tree não commitado; nenhuma fonte L1 foi modificada durante as leituras
(`git diff --name-only -- 01_core/src` vazio em
`2026-09-10T01:06:47Z`). As medidas de fonte abaixo são desse antecedente,
não de candidato. A tarefa principal editava L0 em paralelo. O recorte de
proveniência em `2026-09-10T01:08:51Z`, `git diff HEAD --stat`, era:

```text
 .../prompts/compiler/eval/bindings/field_access.md | 43 +++++++++++
 .../compiler/eval/bindings/value_methods.md        | 89 ++++++++++++++++++++++
 00_nucleo/prompts/compiler/eval/call_dispatch.md   | 53 +++++++++++++
 .../prompts/compiler/eval/operators/equality.md    | 44 +++++++++++
 00_nucleo/prompts/compiler/eval/repr.md             | 38 +++++++++
 00_nucleo/prompts/compiler/eval/rules.md           | 34 +++++++++
 .../compiler/eval/selector_matching.md             | 59 ++++++++++++++
 .../prompts/compiler/introspect/extract_payload.md | 27 +++++++
 00_nucleo/prompts/compiler/introspect/locatable.md | 24 ++++++
 .../compiler/stdlib/foundations/selector.md        | 35 +++++++++
 00_nucleo/prompts/entities/element_payload.md      | 88 +++++++++++++++++++++
 00_nucleo/prompts/entities/elements/emph.md        | 25 ++++++
 00_nucleo/prompts/entities/elements/strong.md      | 26 +++++++
 00_nucleo/prompts/entities/selector.md             | 82 ++++++++++++++++++++
 00_nucleo/prompts/entities/show.md                 | 48 ++++++++++++
 15 files changed, 715 insertions(+)
```

Capacidades concedidas: ler baseline `01_core/src`, CLAUDE/AGENTS, ADRs,
L0s atuais e o diagnóstico/recibos da sonda de integração. Posteriormente,
a tarefa principal autorizou especificamente a leitura adicional de
`03_infra/src/pipeline.rs` e `00_nucleo/prompts/infra/pipeline.md`.
Única escrita autorizada/executada por este autor: este diagnóstico.
Não houve leitura de materialization/context; paths presentes dentro de
recibos/documentos são referências, não acessos. Nenhum patch candidato foi
lido. O contexto herdado contém a instrução da subtarefa e os contratos já
aprovados; não há isolamento técnico nem alegação de independência atestada.

Li a skill `tekt-materializacao-segregada` e suas duas referências. A busca
`rg -l -i 'segregad' 00_nucleo/adr --glob '*.md'` não encontrou ADR local
correspondente. Aplicação aqui: registro de capacidades/proveniência e
limitação da alegação; não execução de protocolo de selo. Não executei probes
novos, build, lint, testes ou benchmark. `jq` não está instalado; sua tentativa
de inspeção não gerou medida de linguagem.

## Medições anteriores à recomendação

### Observáveis já medidos por outro executor

Fonte pública recebida: `p1339-where-integration-probe.md` e recibos
`p1339-where-integration-probe-runs.json` /
`p1339-where-integration-probe-supplement-runs.json`. Vanilla ratificado
`a51e02804`, binário `/usr/local/bin/typst` SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
Os recibos fixam as entradas e UTC de cada execução; não os reexecutei.

- Heading numerado, seguido de `counter(heading).update(42)`: bare get
  retorna `(42,)`, `counter(heading.where()).get()` retorna `(1,)`.
- Fixture First nível 1, Child nível 2, Last nível 1: get bare/vazio/level 1
  retorna `(2,)`; level 2 retorna `(0, 1)`; level inexistente retorna `(0,)`.
- Strong/Emph são aceitos e contam suas ocorrências; filtros de body
  selecionam ou excluem. Text é rejeitado como não localizável.

São semântica/morfologia da linguagem. A forma do log, uma tabela de demandas
ou um método de leitura owned são escolhas mecânicas, não observáveis exigidos.
Não inferir intenção adicional a partir desses resultados. Em particular,
as sondas recebidas **não** medem ordem de callbacks de update entre chaves,
callbacks que leem outro counter ou erro de get no primeiro contexto.

### Estado e ações no antecedente cristalino

| Medida | Consequência observada na fonte |
|---|---|
| `01_core/src/entities/counter.rs:21-25` | CounterKey já transporta Selector integral; não precisa de novo campo/variante para Element aprovado. |
| `01_core/src/compiler/stdlib/counter.rs:100-120` | selector_to_key reduz Where à base e And/Or ao primeiro filho; transporte atual perde filtros. |
| `01_core/src/compiler/introspect.rs:793-804` | Ação automática de heading é Step hierárquico no depth, condicionada a numbering_active; não equivale a contar resultados de query. |
| `01_core/src/compiler/introspect.rs:815-834,953-959,992-1018` | Figure/Equation/Table/Footnote aplicam ações próprias/gates; figure também mantém chave interna por kind. |
| `01_core/src/compiler/introspect.rs:1023-1040` | Update explícito conserva a chave exata; Step/Set aplicados no walk; Func adiado. |
| `01_core/src/entities/counter_registry.rs:25-36` | Registry guarda estado atual e snapshots, não os eventos que os originaram. |
| `01_core/src/entities/counter_registry.rs:69-84,99-109` | Step trunca níveis inferiores, preenche zeros e incrementa nível; Set substitui o vetor inteiro. |
| `01_core/src/entities/counter_registry.rs:138-145,166-191` | value_at devolve último snapshot inserido cujo loc <= alvo; mutadores acrescentam snapshots em ordem de inserção. |
| `01_core/src/compiler/introspect/from_tags.rs:85-136` | Pós-pass só visita Func, lê value_at, executa callback e acrescenta Set em sua Location. Não reexecuta Steps/Sets posteriores. |
| `01_core/src/compiler/introspect.rs:1337-1363` | Payload recebe gates lexicais; store elements guarda conteúdo/snapshot por Location no mesmo walk. |

Inferência verificável, **não execução de teste**: o mecanismo Func atual é
insuficiente para `Step; Func; Step`. O último Step foi calculado antes da
Func; acrescentar o Set da Func ao fim não recalcula o Step posterior. Trocar
value_at por busca da maior Location tampouco conserta o estado desse Step.
Refutação: descobrir replay posterior que substitua ambos os estados; não há
tal replay nos paths runtime/fixpoint lidos. Deltas de snapshots não recuperam
a operação perdida: um Set pode produzir o mesmo número de um Step.

### Demanda real, borrowing e pipeline de produção

`counter_get` (`stdlib/counter.rs:293-315`), `counter_final` (`:534-546`) e
`counter_at_location` (`:517-527`) recebem `&EvalContext`. O trait retorna
`Option<&[usize]>` em `entities/introspector.rs:165-174`, por delegação ao
registry (`:734-753`). Uma computação local `Vec<usize>` não pode ser
devolvida como referência ao introspector. Transferir o impl concreto para
compiler, como investigado na tarefa principal, muda onde o matching pode
viver; não elimina esse limite de lifetime.

`EvalContext.introspector` é concreto e documentado como read-only no eval
(`compiler/eval/mod.rs:148-155`). `native_counter` tem `&mut EvalContext`
(`stdlib/counter.rs:36-41`), mas isso só dá oportunidade de registrar demanda
quando o construtor executa. Uma consulta pode receber Counter já capturado
em closure/array ou surgir somente dentro do contexto; não há no código
medido um inventário obrigatório de todos esses valores.

Produção, confirmada em `03_infra/src/pipeline.rs`:

1. `:644`: introspecção **pura** do conteúdo para localizar contextos.
2. `:652-664` chama expansão; `:245-248` cria EvalContext novo por bloco e
   clona o snapshot recebido; `:272-279` executa a closure.
3. `:310`: reintrospecção pura da árvore expandida.
4. `:666`, via helper `:316-347`: só então chama introspecção com runtime.
5. `:723-731`: ciclo de páginas reexpande contextos e repete runtime; sai
   quando o número de páginas é igual (`:755-760`).

O helper L1 `run_fixpoint` (`compiler/introspect/fixpoint.rs:69-135`) possui
outra ordem e convergência por hash de tags. Seu L0 reconhece explicitamente
que ele não é o pipeline L3 de produção. Não é válido propor demandas nesse
helper e assumir que a CLI passará a usá-las.

Consequência: se o primeiro contexto toma uma decisão falível com base num
update Func anterior, ele pode abortar antes de `:666`. O ciclo de páginas
posterior não pode recuperar esse erro. Isso é inferência causal da ordem,
não resultado de probe; a fixture discriminante precisa ser medida nos
binários antes de virar contrato. O L0 `infra/pipeline.md`, cláusula
P1140.4-A2, exige runtime depois de contextos/Locations estabilizados.

## Recomendação de desenho interno e seu alcance

### Registro de ações reais, separado de identidade de chave

Recomendo como substrato **um log privado no CounterRegistry**, composto
somente de dados L1. Cada evento conserva ordem, Location, origem automática
ou manual, ação real e, para manual, CounterKey exata. Pode usar tuplas/tipo
privado e métodos `pub(crate)`; não requer novo campo público de
TagIntrospector, CounterKey, ElementPayload ou Introspector.

Evento automático: Location de ocorrência e ação opcional efetivamente
capturada pelo walk. Heading numerado gera Step(depth); heading sem
numeração não gera avanço. Strong/Emph próprios geram Step(1) somente depois
de sua promoção de ocorrência aprovada; a unidade NativeElement não revela
identidade, obtida do store canônico. Gates de outros elementos permanecem
os dos owners. O registro não deve duplicar como novos eventos a escrita
auxiliar figure:kind e a escrita global que representam a mesma ocorrência.

Evento manual: CounterKey + Step(level), Set(vetor) ou Func, exatamente como
emitido pelo Content. Não o classificar como automático por a chave ser Kind.
Para obter uma chave filtrada, selecionar eventos automáticos pelo predicado
linguístico sobre a ocorrência/snapshot, e incluir eventos manuais somente
quando a chave inteira coincide. Grupo vazio conserva identidade própria.

Assim get até Location aplica o prefixo; final aplica a sequência completa.
O vetor começa em zero e cada ação hierárquica é aplicada de verdade. Level 2
isolado produz `(0, 1)` pela ação, não por tamanho de query. Bare update 42
não entra na sequência da chave Element com grupo vazio. Filtros com campos
em outra ordem podem selecionar os mesmos eventos automáticos e ainda
constituir chaves distintas para updates.

A aplicação de igualdade entre **chaves** precisa seguir o contrato de
identidade aprovado; matching de valores de campo usa igualdade da linguagem.
Não adotar incidentalmente o derive como prova de igualdade Typst nem usar
hash como identidade. `CounterKey::Eq` tem comentário antigo sobre ausência
de NaN (`counter.rs:28-31`), que não vale como proibição de filtros NaN; essa
fronteira deve ser conciliada com o owner P1339 de igualdade, sem alterar
derives públicos por este diagnóstico.

### Callbacks: replay na etapa que já possui Engine

Para uma chave cujo log contém Func, a própria ocorrência do update já
declara a demanda exata. No pós-processador existente, fazer replay da
sequência completa dessa chave na ordem dos eventos: automáticos
selecionados + manuais exatos. Cada callback recebe o vetor imediatamente
anterior como argumentos posicionais separados, como já exigido por P1149;
resultado válido vira Set resolvido no mesmo evento. Em seguida aplicar
Steps/Sets posteriores e reconstruir snapshots em ordem. Erro segue
SourceResult, não vira zero. A função de elemento no Selector não é callback.

Chaves distintas com callbacks devem conservar a ordem global medida antes
de otimizar por chave. Não há ainda sonda suficiente para decidir callbacks
que fazem consultas contextuais a outros counters. O helper runtime L3 cria
ctx novo (`pipeline.rs:326`), e o pós-processador não o transforma num contexto
de cada evento. Fazer essa transformação seria decisão adicional a medir.

Clonar o log junto ao registry preserva causalidade por iteração; não existe
cache global. Eventos originais/resolvidos precisam de distinção interna
para que replay não execute a mesma Func duas vezes na mesma construção nem
regrave o log enquanto reconstrói snapshots. Não usar um Set sintético
inserido como evento do documento para simular a declaração de uma demanda.

### Consultas novas em contexto: alternativas e limites

**A — helper interno de consulta owned.** Um helper em compiler recebe o
TagIntrospector, chave e limite de Location e devolve vetor próprio, fazendo
fold puro do log com callbacks já resolvidos. get/final/at/display e wrappers
estáticos chamam esse helper; suas assinaturas públicas retornam Value e não
precisam mudar. Isso atende uma demanda criada somente dentro do contexto e
não depende de materializar infinitas chaves de filtro antecipadamente.
O custo é proporcional aos eventos examinados; não foi medido.

O trait borrowed pode continuar servindo suas chaves materializadas, mas
isso **não** prova que uma chamada Rust direta
`counter_values_at(Element(...))` funciona para qualquer chave arbitrária.
O L0 precisa dizer explicitamente qual interface é a leitura materializada e
qual calcula a consulta da linguagem. Não declarar transparência universal
do trait. Além disso, o helper não resolve uma Func ainda não executada no
snapshot inicial; devolvê-la como zero seria resultado incorreto, não solução.

Essa alternativa é a recomendada para reduzir superfície pública, **condicional
à decisão de fase**: é consulta pura de ações capturadas, mas parte do cálculo
de estado passa a ocorrer na leitura de eval. O L0 atual descreve leitura de
snapshots e construção de estado na introspecção; este diagnóstico não se
autoautoriza a reinterpretar essa fronteira como mera fórmula interna.

**B — demanda no construtor.** Registrar a chave em native_counter tem
acesso mutável, porém os ctx novos são descartados por bloco e o pipeline
não devolve demandas (`pipeline.rs:245-281`). Capturas que pulam o construtor
na iteração de expansão refutam sua suficiência geral. Transportar demandas
para nova iteração exigiria armazenamento/canal e condição de convergência;
pré-avaliar contexto para descobri-las pode falhar antes da descoberta.
Não é alternativa resolvida com as assinaturas e o fluxo vigentes.

**C — ampliar getter para retorno owned.** Tornaria natural a consulta
derivada também para clients do trait. Porém alterar
`counter_values_at`/`counter_final_values` para `Option<Vec<usize>>` é gate
explícito ADR-0127 de contrato público, com inventário de implementações e
consumers. Não resolve sozinho runtime atrasado de Func.

**D — cache mutável sob &self.** RefCell não permite retornar &[usize] de um
borrow temporário como referência estável; OnceLock simples não modela chaves
arbitrárias acrescentadas continuamente. Uma arena append-only com política
de lifetimes/memória é desenho novo ainda não investigado. Não propor unsafe,
leaks, estado global ou enum público especulativo para contornar o obstáculo.
Esse limite refuta as implementações triviais, não todas as estruturas
imutáveis concebíveis; não há prova de impossibilidade universal.

## Menor mudança de fase concreta a levar a decisão

Se o contrato público incluir get/final correto **já no primeiro contexto**
após callbacks anteriores, a fase atual não fornece o dado. A alternativa
concreta preferível à execução de callbacks dentro dos getters é preparar
as ações de counter antes de expandir contextos:

```text
walk puro (inclui log e Locations de ContextBlock)
  -> resolver somente ações Func de counter em replay ordenado
  -> expandir ContextBlocks lendo esse snapshot
  -> runtime final e layout nos pontos atuais
```

O preparador fica em L1 compiler/introspect/from_tags ou no owner
introspect, recebe o introspector/log e o Engine/EvalContext construído por
L3. L3 só compõe. Não antecipar implicitamente state displays, equação ou
numbering para usar indiscriminadamente `introspect_with_runtime` antes dos
contextos: esse entrypoint executa todos eles (`introspect.rs:414-420`).

Uma entrada L1 pública estreita, por exemplo preparação de counters sobre
`&mut TagIntrospector` com Engine/ctx e SourceResult, seria a forma clara de
ligação entre crates. O nome/assinatura final não está fixado neste diagnóstico.
Há **gate de fase ADR-0127, ponto 3**, porque Func passa a ficar disponível
antes da expansão; se for criada essa entrada pública, há também **gate de
assinatura, ponto 1**. A aprovação de Selector/ShowSelector/NativeElement não
cobre esses gates. Não se provou que uma nova assinatura é inevitável em
todas as alternativas; ela é exigida pela ligação estreita descrita aqui.

Esta preparação cobre eventos já presentes na árvore inicial. Updates e
elementos que nascem dentro de ContextBlock exigem iteração de expansão com
snapshot coerente e critério semântico de convergência, ou escopo público
explicitamente menor. O loop atual condicionado apenas a páginas iguais não
prova estabilidade desses valores. Não há desenho fechado para contexto
cíclico, callbacks consultando outros counters ou queries sobre elementos
criados na mesma expansão; tratar esses casos como não investigados, não
como impossíveis ou aprovados. Também falta medir qual primeiro snapshot
correto o vanilla fornece quando há forward dependency contextual.

## Conclusão delimitada e trabalho restante

O carrier aprovado é suficiente para identidade/filtros. O baseline não
possui a maquinaria necessária para aplicar ações automáticas filtradas a
uma chave criada tardiamente. Recomenda-se log privado de ações + consulta
owned interna + replay real de callbacks. Isso é um desenho candidato útil,
**não integração pronta** sob todos os contratos e fases atuais.

Antes de fixar L0, medir as fronteiras que discriminam as alternativas:
get/final criados dentro de context; Counter capturado antes dele; intercalar
Set/Func/Step hierárquico; bare versus where vazio após update; callback antes
de primeiro contexto falível; update criado dentro do contexto; estabilidade
de resultado com mesmo número de páginas. As expectativas não medidas não
devem ser congeladas por inferência deste relatório. Os números da sonda
recebida continuam evidência do seu baseline, não resultado destes cenários.

Owners de futura nucleação: entities/counter_registry (dados e replay puro),
compiler/introspect (captura de ações e consulta),
compiler/introspect/from_tags (resolução de Func), compiler/stdlib/counter
(transporte e leituras), e infra/pipeline somente se a fase for alterada.
entities/introspector exige atualização se a responsabilidade do impl/leitura
materializada mudar; entities/counter apenas se houver obrigação nova de
identidade além do carrier já existente. Owner fixpoint acompanha a lógica
quando aplicável, sem promovê-lo silenciosamente a produção. Nenhum código
é legitimado por este diagnóstico nem por um owner alheio (ADR-0129).

## Hashes das entradas consultadas

SHA-256 bruto dos bytes lidos; não são resselo Tekt nem prova de isolamento.
Alguns L0s históricos ainda contêm interfaces sucedidas; a leitura considerou
as seções posteriores e o código medido, não os primeiros exemplos isolados.

```text
bc50c0c6d54c0e301a5fe3c5c5869dbeef8fdf624096b0fa20185122c64d7da0 CLAUDE.md
d358e69f77ceb88bfd7f64bbf64f90b88b6e97c0bac88566f0eb94315b48829f 01_core/CLAUDE.md
66990d349a9e89851686cd94590a84711c69364f76b6df501230f64daf3b0c48 skill SKILL.md
16db4af3a8a21a27e1bfc4a5dd00c976f0fc946ba46dc8df1a663171d823f72d skill references/artefatos-e-gates.md
f59f44c4e53e89651963115c582872b4d3cd59d89689d103baa9ef8b464d2417 skill references/papeis-e-capacidades.md
e680d22bbf4486cf93f5bfb4ec85f4ae965e6db788c2a18c48f3be000029d49d 00_nucleo/adr/typst-adr-0107-paridade-linguagem-nao-mecanica.md
31daec5ae9e84cb5bbdcb806e9a2b6cb9160b7e90519df53e6e0bd8809076405 00_nucleo/adr/typst-adr-0108-disciplina-anti-deriva.md
0cbd3049418073e9b8efd0be1a19030c9ecde905a25ebf3d20922e3cef02676e 00_nucleo/adr/typst-adr-0109-atomizacao.md
5e8581b5f9ebb0798d4213e59e39ee8dfcd4f41b34d4ca639b00b1f287699ad9 00_nucleo/adr/typst-adr-0127-gate-l0-paragem-vs-fluxo.md
64756b81ce58ca62e1a166b3776303759bc7af507a1c97a4e3ad91a8dc5b906e 00_nucleo/adr/typst-adr-0129-nucleos-tekt-l0-compartilhado.md
83f677859fe3c3aac95b6f635366106e77b480d8664b75f08cd0443ebc98d633 00_nucleo/prompts/entities/counter.md
3d833b89782e5bb2c38dbf227c046680caa00e92047d2986154be18ff6c69872 00_nucleo/prompts/entities/counter_registry.md
20ce64c0e7eeddb1dca4cd491858e88fc0259929ae625bd2bce50458f6272187 00_nucleo/prompts/entities/counter_update.md
78ab7cd909b137a786865daac1c18d8cbc74cafd7ddda5458e09dd3bedf8ed2f 00_nucleo/prompts/entities/introspector.md
b9d3a63c899641645cae6e50ba1e4fa76555d6e8f02cd3e7b284af80d788370a 00_nucleo/prompts/compiler/introspect.md
13c5a5680f9a03dcf8626e06bc9013946bff3a821dc414fa63a916386a47fd81 00_nucleo/prompts/compiler/introspect/from_tags.md
9a0fb90452c91a75fe1f091af514f782b2c16a3c6d3b0b95a52b137748b76b1b 00_nucleo/prompts/compiler/introspect/fixpoint.md
3435825c70539ec4c0c03d4d15ee9fa19698f1c7f66808723bf4e9c596e9a243 00_nucleo/prompts/compiler/stdlib/counter.md
f02618e88c4c60a5d52f4f06997ed052b2eba93d3a3d9576e12844c8817e9813 00_nucleo/prompts/compiler/stdlib/_comum.md
8b6ebfea3455c6191455b007b461270765b8b07726911f5830835dfd259239df 00_nucleo/prompts/entities/selector.md
777e6aecdad7bb23ba69b82471fb4bc9c9b7636c8a2273e50232c248fa522e73 00_nucleo/prompts/entities/show.md
44994d49505a501d657e413ffff6d69ae68d1feb9044c863505f40ccf1da77f5 00_nucleo/prompts/entities/element_payload.md
48cc0bacd9b84391df60f5065f4e834c21c6832df175337cfc98f54d33a20e5d 00_nucleo/prompts/compiler/eval/selector_matching.md
09029233b75b358953024023dd11b060544890ac791b3151129afd7687a3066e 01_core/src/compiler/introspect.rs
04b5d1fd225f6df7c33b6ea5c4798d0cad2f18b2d8e8bd447053201189f6fee4 01_core/src/compiler/introspect/from_tags.rs
52383410ca023b303e28b4df718e82f882d7d5b44afce226bb5035367c1c0fe6 01_core/src/compiler/introspect/fixpoint.rs
c9058627737fb4726f3fd627e84f5050972b20f817752d01e050d3188f47f270 01_core/src/compiler/stdlib/counter.rs
2b81a9e35eac6b2547dc3136d63eb285e410d48ccea02753fcadf2ecc64d1f4a 01_core/src/entities/counter_registry.rs
553f602a5b3c38f2b7c8ff9add6495fe8feeeb37b3a6fc409fcebd2084568f02 01_core/src/entities/introspector.rs
d85f2151f08b6f9adc966f3611728b75fa1ce270b3904461155dcd76f634053e 01_core/src/entities/counter.rs
412c80370dd64e983582c4a580cedaa823e6bad23203ab1c3b5a13b85f0d9f05 01_core/src/entities/counter_update.rs
115a34e8aa41b4ec5a55b0cec5d05927216dc6a4a6fcb98d2fd2c1edc98d262c 01_core/src/compiler/eval/mod.rs
72f9c080b55ca295e5b51ae45acf527c76921cd06211a64c2c43fdd010af6088 03_infra/src/pipeline.rs
ce6da4f623a0270869606bdf42d07dae0283f3f9e5a593258609b219424e63a8 00_nucleo/prompts/infra/pipeline.md
9a4c5da77740fb73eda167e70f869173a4f281f2ec356f6eb3083d322892c4e9 00_nucleo/diagnosticos/p1339-where-integration-probe.md
ae4656f58a0a03cd62a56ecacbf99202f440e4df8073ad231b6b2b257b6e84b7 00_nucleo/diagnosticos/p1339-where-integration-probe-runs.json
5f76638911404da4569c45719b0670e7ab53d873966fc34f49059f1653300d78 00_nucleo/diagnosticos/p1339-where-integration-probe-supplement-runs.json
```
