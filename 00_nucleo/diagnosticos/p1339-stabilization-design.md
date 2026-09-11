# P1339 — desenho mínimo da estabilização seletiva

## Estado, regime e proveniência

Desenho exploratório por `/root/p1339_stabilization_design`. A autorização
de escopo para estabilizar contextos dependentes dos novos contadores
filtrados foi comunicada pelo coordenador; não equivale à aprovação das
assinaturas públicas propostas abaixo. Este documento não é L0, contrato,
oráculo, selo, implementação, prova de convergência nem veredito de paridade.

Skill `tekt-materializacao-segregada/SKILL.md` e ambas as referências lidas
integralmente. Regime: investigação de desenho, sem atestação de isolamento;
filesystem compartilhado, disciplina declarada. Nenhuma ADR de segregação
foi encontrada pela busca de `segregad` em `00_nucleo/adr/*.md`; ADR-0127
lida. Escrita restrita a este diagnóstico. Nenhum código produtivo nem L0
foi editado. Não foram lidas/listadas as pastas `materialization` ou `context`.

HEAD `2f42d64253547734564513a1159ee6b584c1c4b4`, working tree não commitado.
Inspeção iniciada em `2026-09-10T01:50:34Z`. As fontes examinadas e os L0
`compiler/eval.md` e `infra/pipeline.md` permaneciam idênticos ao HEAD no
check final dirigido. Nenhuma fixture foi executada nesta subtarefa.
Resultados experimentais citados são os do recibo prévio
`p1339-context-dependency-probe-runs.json`, SHA-256
`4da38b499916ed3d5946bb16c904f2eaa268a4ec2f788cc0149927f0d83260d1`;
esse recibo contém fontes, binários, UTC, argv e estado completo da árvore.

Hashes SHA-256 integrais dos L0 vigentes, confirmados nesta inspeção:

- `prompts/compiler/eval.md`: `80412483c8353a736f5c11842511cdef6c5e854728281a780c0183d6b73875cf`.
- `prompts/infra/pipeline.md`: `ce6da4f623a0270869606bdf42d07dae0283f3f9e5a593258609b219424e63a8`.
- `01_core/src/compiler/eval/mod.rs`: `115a34e8aa41b4ec5a55b0cec5d05927216dc6a4a6fcb98d2fd2c1edc98d262c`.
- `03_infra/src/pipeline.rs`: `72f9c080b55ca295e5b51ae45acf527c76921cd06211a64c2c43fdd010af6088`.
- `entities/func.rs`: `9bcd2e2b0c79cae5c2478b55d9d902f78983f9119d634a04a846d93210307bde`.
- `entities/content_hash.rs`: `ead6c06546e14a59d9f1eaed377d896c8247ea36b275117a602a8ea83635fdf2`.

O `git diff HEAD --stat` inicial registrou somente L0 em redação alheios às
duas entradas normativas acima. A lista exata era: compiler/eval/bindings/
field_access.md, value_methods.md; compiler/eval/call_dispatch.md,
operators/equality.md, repr.md, rules.md, selector_matching.md;
compiler/introspect.md, introspect/extract_payload.md, from_tags.md,
locatable.md; compiler/layout.md; compiler/stdlib/counter.md,
foundations/query.md, selector.md; entities/counter_registry.md,
element_payload.md, elements/emph.md, elements/strong.md, introspector.md,
selector.md, show.md, todos sob `00_nucleo/prompts/`.

## Evidência anterior à recomendação

`eval/mod.rs:148-155` torna explícito o snapshot read-only durante eval.
O contexto possui campos de execução e construtor (`:124-240`), mas nenhum
transcript de dependências. A criação de `context` captura o scope e emite
um `ContextBlock` (`:1404-1427`); não produz ainda o update dentro do corpo.

`pipeline.rs:244-278` cria um EvalContext por bloco, injeta o mesmo snapshot
e propaga imediatamente `Err` de `apply_func`. O resultado e o próprio ctx
já estão ao alcance de L3 depois da chamada; não é preciso mudar a assinatura
de `apply_func` para L3 examiná-los. A substituição ocorre em `:281` e a
reintrospecção em `:309-310`, somente se todos os blocos tiveram sucesso.
O caminho produtivo aborta antes do runtime e do layout (`:651-674`).

`introspect.rs:1023-1040` associa eventos explícitos aos counters; o walk
ignora Func nesse ponto. A execução global dos pós-processadores está em
`:414-419`, e a localização do bloco em `:1058-1065`. Nenhuma dessas rotas
informa a L3 qual bloco efetivamente consultou uma chave Element. O mapa
de blocos é um HashMap público (`entities/introspector.rs:505`), portanto
sua ordem de iteração não representa a ordem documental.

`pipeline.rs:423-441` conserva marcador de contexto junto ao resultado.
`:641-642,721-725` preserva e reutiliza a árvore contextual original.
Entretanto `:754-757` só compara quantidade de páginas. O L0 vigente
`infra/pipeline.md:502-504` exige também snapshots, objetos crus e conteúdo
realizado; `:515-518` exige partir da árvore original sem acumular marcadores.

Há dois perigos opostos ao tentar aproveitar comparadores atuais:

- `Func::PartialEq` usa Arc para closures (`entities/func.rs:384-396`);
  `Func::Hash` usa endereço (`:401-415`). `CounterUpdate` deriva ambos
  (`entities/counter_update.rs:15-23`). Recriar a mesma closure em cada
  tentativa pode impedir igualdade mecânica indefinidamente.
- `Func::Debug` é apenas `<function>` (`entities/func.rs:369-372`).
  `hash_content` usa Debug (`entities/content_hash.rs:25-29`). Duas
  callbacks com capturas diferentes podem parecer iguais nesse hash.
  `ClosureRepr` guarda body e scope capturado (`entities/func.rs:53-82`);
  ignorá-los perde dados que alteram a chamada futura.

O recibo anterior mostra falha inicial tanto para Set como para Func gerada
em contexto, e para final anterior ao produtor. Isso sustenta repetir com
novo snapshot; não autoriza reparar os controles de chaves legadas.

O coordenador acrescentou como entrada permitida o recibo
`p1339-stabilization-boundaries-runs.json`, SHA-256
`4d82648d895ec5dba23a9775c2bf09dfae84a175603f3ce6d742c8c7f2a09241`.
Foi lido integralmente. Medição exploratória em
`2026-09-10T01:53:33.885664+00:00`–`01:53:34.524627+00:00`, mesmo HEAD
mais working tree com status/stat completos no recibo; vanilla ratificado
`a51e02804`, `/usr/local/bin/typst`, SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
Em ambas as ordens de execução, oscilação e crescimento retornam exit 0,
respectivamente `[[0]]` e `[[4]]`, com aviso de não convergência após cinco
tentativas. NaN retorna `[[0]]` sem aviso de convergência; closure recriada
e produtor aninhado retornam `[[12]]`; erro independente permanece exit 1
com `unrelated-error`. O aviso de depreciação de query aparece separadamente.

Isso refuta **hard error universal ao teto como regra de paridade**. A
recomendação anterior em `p1339-context-dependency-review.md` e `-gate.md`
de não exportar provisório precisa distinguir erro pendente de avaliação,
ausência de convergência e a política medida de resultado com warning.
Não se pode inferir autorização para uma nova política de produto; o L0
concreto deve decidir essa fronteira antes de implementação. Exit 0 com
warning de não convergência nunca é prova de que um fixpoint foi atingido.

## Fronteira pública mínima recomendada

Para **seleção e comparação das leituras**, a proposta mínima é armazenamento
privado no EvalContext e dois métodos públicos no seu owner existente:

```rust
pub fn has_filtered_counter_reads(&self) -> bool;
pub fn filtered_counter_observations_match(&self, previous: &Self) -> bool;
```

O primeiro responde se houve demanda real por CounterKey cuja variante
Selector seja Element. O segundo compara observações de execuções reais já
terminadas; é puro, não recebe Engine e não executa nenhuma callback.
Informação incompleta/ambígua resulta em `false`, nunca em igualdade otimista.
Seu nome e L0 precisam afirmar que compara **observações filtradas**, não
convergência do documento. O retorno bool basta para essa pergunta limitada;
L3 conserva separadamente o motivo de término por teto, que não atesta
convergência e cuja política pública precisa seguir a decisão acima.

O armazenamento começa vazio em `EvalContext::new`. Registro e fecho de cada
leitura são helpers internos de L1. Registrar a demanda depois de identificar
o valor Counter/Selector::Element, **antes** de consultar localização, resolver
label, executar o fold ou fazer cast do resultado. Assim `Err` conserva a
dependência; uma leitura em alias, função importada, `.with` ou ramo realmente
executado funciona sem reconhecer o texto da expressão. Construir counter ou
update não registra leitura; ramo não executado também não. Registrar antes
do callback evita perder a marca se ele falhar.

Cada observação privada precisa conservar a operação efetiva, seu alvo e
ordem, os eventos selecionados em ordem documental e o resultado do fold
demandado, inclusive erro. Para get/at, guardar apenas o prefixo retornado é
insuficiente: a demanda do mesmo counter também pode falhar num callback
posterior. O transcript pode reutilizar entidades existentes internamente;
não necessita exportar um novo enum/struct para L3. Um ctx novo por bloco
dispensa método público reset/take/set e evita vazamento de marcas entre
blocos. Contextos temporários de execução de callbacks não apagam o registro
do demandante.

L3 retém `(id, location, ctx, resultado, diagnósticos, warnings)` da tentativa.
As fachadas `expand_context_blocks` e
`expand_context_blocks_and_reintrospect` mantêm assinaturas; um helper privado
em pipeline.rs pode devolver tentativas parciais ao orquestrador interno.
Nenhum campo novo em `TagIntrospector`, trait alterado, callback injetada por
trait, mutação do snapshot durante eval ou reentrada por layout decorre
necessariamente desse transporte.

**Gate:** os métodos públicos são adições concretas de superfície L1 e devem
constar da decisão ADR-0127 antes do código. Campo privado novo no EvalContext
também merece auditoria de compatibilidade: a struct atual tem campos públicos;
adicionar um campo privado pode quebrar construção por literal fora do crate.
Preservar assinaturas existentes não demonstra, sozinho, compatibilidade
de todas as formas de construção. O coordenador examina essa fronteira.

## Estabilidade: o que esses métodos resolvem e o que não resolvem

O comparador do transcript não usa a igualdade de linguagem dos seletores
como igualdade de estado de execução. São perguntas diferentes. NaN pode
continuar não reflexivo ao decidir membership/associação de updates e,
simultaneamente, uma leitura repetida sem matches e resultado `[0]` ter
observações estáveis. Comparar `CounterKey == CounterKey` diretamente nesse
gate pode declarar instabilidade artificial. O mínimo comparável é a decisão
efetivamente observada: alvo, roster ordenado de eventos/matches, vetores
numéricos resolvidos e sucesso/erro. Caso se guarde também a requisição crua,
sua representação para estabilidade deve ser reflexiva e não substituir a
igualdade de associação da linguagem.

Tampouco se compara identidade Arc dos callbacks executados. Compara-se sua
contribuição realmente observada, incluindo o fold completo da chave
demandada. Não executar callbacks de chaves nunca consultadas para preencher
um fingerprint. Não construir equivalência geral de funções a partir de
retornos iguais em um argumento.

**Limite material:** getter + comparador filtrado não são condição suficiente
de convergência. Um bloco pode observar o mesmo counter Element e outro valor
contextual diferente — query, page, metadata — e emitir conteúdo diferente,
inclusive uma closure com captura diferente. O transcript filtrado permanece
igual, as páginas podem permanecer iguais e o Debug da closure também. Outro
exemplo é final igual com histórico intermediário diferente: consumidores at
podem observar a diferença.

Há duas formas legítimas de completar o gate, ainda a especificar, e nenhuma
deve ser escondida dentro de uma afirmação de suficiência dos dois métodos:

1. Validar também todos os outros inputs contextuais efetivamente usados
   pelo bloco selecionado; conservar a saída daquele bloco quando a execução
   original, seus inputs e sua posição continuam válidos. A justificação
   seria determinismo sob inputs iguais, não igualdade de ponteiro. Isso
   exige observabilidade que o baseline não possui e pode ampliar os owners
   além desta subtarefa. Ler somente Element não satisfaz essa alternativa.
2. Comparar o resultado contextual completo por uma relação de estabilidade
   própria, sem mudar `Func::PartialEq` da linguagem. Closures diferidas
   exigem identificação exata do executável e comparação completa dos valores
   capturados relevantes; With exige função e argumentos. Nomes, span sozinho,
   Debug e endereço novo não bastam. Usar a representação imutável completa
   do body para comparar executáveis seria comparação estrutural, não scan de
   AST para adivinhar dependências; a distinção deve ficar explícita no L0.
   Valores opacos ou representação não coberta impedem afirmar estabilidade.

A alternativa 2 permitiria uma fachada pública pura adicional, se o
coordenador a escolher, por exemplo
`context_realizations_match(&Content, &Content) -> Option<bool>` no owner
apropriado. `None` significa insuficiência observacional. **Não se recomenda
aprovar essa assinatura isolada:** falta o contrato exaustivo dos conteúdos,
funções e capturas que compara, e esta allowlist não contém todos os L0
desses owners. Não há aqui instrução para escrever esse comparador nem
declaração de que é menor que registrar os demais inputs.

A decisão mais estreita já concretizável é aprovar o transporte dos dois
métodos e manter a convergência normativa aberta até escolher uma dessas
relações completas. Não selar o contrato com igualdade de transcript Element
ou com hash de Content como substituto. A insuficiência é demonstrável pelos
dados omitidos acima, não uma hipótese de performance.

## Tentativas, erros legados e documento misto

Na primeira tentativa, cada bloco possui seu próprio ctx e seu resultado.
Um erro sem leitura Element efetivamente alcançada é erro legado definitivo
para essa tentativa de expansão: conservar seu diagnóstico original, span e
trace. A existência de outro bloco Element no documento não transforma esse
erro em provisório. Em particular, um bloco que falha **antes** de alcançar
uma leitura Element não ganha autorização por ter uma leitura depois do erro
no código fonte. Não é necessário nem permitido inferir esse trecho do AST.

Erro de bloco com demanda Element pode ser provisório. Continuar produtores
irmãos permite construir uma árvore candidata e reintrospectar; não elimina
o erro nem autoriza exportação. Blocos legados bem-sucedidos conservam sua
contribuição já calculada durante esse ciclo adicional; reexecutá-los contra
novos snapshots para fazer o documento inteiro passar repararia os controles
legados por acidente. O ciclo ordinário pré-existente após layout permanece
distinto dessa preservação durante as tentativas adicionais.

Caso haja erro legado e erro Element provisório no mesmo documento, conservar
o erro legado em um conjunto separado. Ele não desaparece quando a leitura
Element estabiliza. Ordem final deve vir da ordem documental/Location, não
da iteração aleatória de HashMap. A precedência entre vários erros deve ser
medida/congelada pelo coordenador; esta proposta não inventa equivalência de
stderr de múltiplos erros sem observação.

Um bloco que contém leitura Element e leitura legada pertence ao conjunto
selecionado por dependência real; isso não autoriza alterar a semântica das
operações legadas que ele chama. Se a preservação pretendida exigir congelar
também **o valor observado** por leitura legada dentro desse mesmo bloco,
será necessária uma vista composta de snapshots ou rastreio mais fino,
decisão adicional que um bit por bloco não consegue representar. Essa
fronteira precisa ser explicitada pelo contrato, não inferida do nome do bit.

Em cada tentativa, partir da árvore original com resultados da tentativa
atual e as contribuições legadas conservadas. Nunca acumular updates. Se um
bloco selecionado que tinha sucesso passa a falhar, seu resultado anterior
não pode permanecer como sucesso atual e sustentar a própria dependência.
Manter o marcador e o erro pendente; reintrospectar apenas a contribuição
válida atual. Novos blocos/IDs e mudança de Location invalidam comparações
anteriores; colisões/identidade ambígua não podem virar convergência.

Declarar a tentativa convergida exige nenhum erro pendente e validação
completa perante o snapshot produzido pela própria tentativa. Persistência
de erro em estado estável devolve o diagnóstico da tentativa estável;
oscilação ou teto esgotado é estado separado de não convergência, com política
de resultado/warning ainda a fixar conforme a medição nova. Não descartar
um erro pendente para imitar o exit 0 dos controles que não têm esse erro.
Warnings pertencem à
tentativa que os produziu e precisam de política para não duplicar avisos
descartados. A estabilidade contextual também participa do ciclo que reinjeta
pages/positions; páginas iguais não encerram dependência contextual pendente.

## Validação necessária antes de qualquer alegação de GREEN

Casos discriminatórios a derivar do L0 escolhido: produtor contextual depois
de final; Set e Func; alias/With; callback posterior inválida com get anterior;
callback de chave nunca consultada; erro estável depois de assert provisório;
erro legado em bloco irmão antes/depois de bloco Element; erro anterior à
demanda dentro do mesmo bloco; NaN sem matches estável; fold intermediário
mudando com final igual; closure recriada com mesma captura; closure emitida
com captura que muda sem mudar páginas; oscilação; mudança de Location;
bloco que alterna sucesso/erro sem reter contribuição antiga.

Os casos de captura e input não Element distinguem um gate completo do
atalho bool+hash. Um caso obrigatório opaco não pode ser removido só para
fechar o contrato; precisa de observabilidade ou permanece bloqueador.
As medições exploratórias existentes não substituem esse contrato nem seu
gate discriminatório. Entrega atual: transporte público mínimo proposto,
tratamento por bloco delimitado e insuficiência do gate de convergência
explicitada, sem código ou selo.
