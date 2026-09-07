# P1307-r2 — auditoria do transporte de argumentos e spans

Conclusão: **não está demonstrado um desenho completo restrito a
`call_dispatch` e sem novo carrier persistente**. Capturar o span antes de
`merge_with_args` resolve um fragmento simples, mas argumentos encaminhados,
closures que devolvem funções e named duplicados expõem informação que já
foi perdida. A reabertura autorizada permitiu esta auditoria; não autorizou
alterações públicas, L0, testes RED ou implementação.

## Proveniência e leitura

Auditoria de fonte em `2026-09-07T16:38:27Z`, HEAD
`b303f1f15b610e09872b567027e0d806387fde8c`, working tree P1306 certificado
não commitado, conforme baseline P1307 SHA-256
`418443cb00f8733ae034cbf098662869835a1235f2112ef423f900cf9b56e48e`.
O `git diff HEAD --stat` observado permaneceu:

```text
 .../prompts/compiler/eval/bindings/field_access.md |  94 +++++++++-
 00_nucleo/prompts/compiler/eval/tests.md           |  76 +++++++-
 01_core/src/compiler/eval/bindings/field_access.rs |   4 +-
 01_core/src/compiler/eval/tests.rs                 | 198 ++++++++++++++++++++-
 4 files changed, 360 insertions(+), 12 deletions(-)
```

O preflight desta revisão é
`00_nucleo/diagnosticos/p1307-r2-preflight.json`, SHA-256
`f09eb44503af474a02f8f4823756e1672bb7d3266610c8805f28c9448536c257`.
Autoria deste documento: papel `span_contract_audit`, sem candidato e sem
escrita de oracle/veredito. Regime: executado sem atestação de isolamento
técnico. Nenhum artefato P1306/P1307 anterior foi alterado.

Medição independente sucessora consultada, congelada por `/root/p1307_oracle`:
`p1307-r2-measurement.json`, SHA-256
`847faabad41df603a82f7fc5c5d0435180cdec66c33ae9b3a1fd55d7ee320fa2`;
script `p1307-r2-observe.py`, SHA-256
`ebf05054b737357c9ee80bbef764fe4c1cac37f8209af4af3d59b88108dcce62`.
A matriz começou em `2026-09-07T16:44:21.062280+00:00` e terminou em
`2026-09-07T16:44:45.243137+00:00`, no mesmo HEAD e diff acima. Conserva
48 casos × quatro perfis × dois binários × três ordens = 1.152 execuções,
sem Unknown ou instabilidade dentro de cada lado/perfil/caso. O relatório
independente `p1307-r2-observability.md`, SHA-256
`5b76a7899714ac169f7b6e127c2da09a2ccb97b66e72f92e981690de2146410c`,
documenta também as tentativas focais preservadas. Este autor não escreveu
nem ajustou esses resultados. A rota contextual agora é observável; isso
remove a lacuna do adaptador anterior, não a perda de argumentos auditada aqui.

Os quatro L0 seguintes foram lidos integralmente; os três primeiros já
tinham sido lidos na fase anterior e seus hashes foram reconfirmados:

| Owner relativo a `00_nucleo/prompts/` | SHA-256 |
|---|---|
| `compiler/eval/call_dispatch.md` | `e09f8d04c91c009af2bacadd2d6a06b3b314e07b42322b0e84370e7c1767f9b1` |
| `entities/args.md` | `3a33f1f2628348e3484b8346ab55aa82474b789e6836c62303b24b43297f5d56` |
| `entities/func.md` | `52b6f6e7ca7714027d4e3a883f6e5ae888a48e4a5c67e45651b63b86d9811908` |
| `compiler/eval/closures.md` | `13579610a2aa7b640b1423ce273700f6005dd1831ca8f1df63af5d366b5498fc` |

Hashes dos consumers auditados, respectivamente:

```text
01_core/src/compiler/eval/call_dispatch.rs
75399c76d99dfaeb9b515d3152f824dcb7dc8815e693f45a2be10b3d394c2759
01_core/src/entities/args.rs
c4390cee53010c891656e33dc08159a597f99b10f8f9a1a54e9eb67acd15155a
01_core/src/entities/func.rs
0287c1e122c15af282e98162d3f38f8bcd7e5747d5529b08d5186fc0bc8e80b9
01_core/src/compiler/eval/closures.rs
0869b2aa44e000b5a76eb9a8dfb77abe3c4c8a1291ae885d7ef785f427b6168e
```

## Medição da informação disponível antes de decidir

### 1. A representação inicial já descarta informação

`01_core/src/entities/args.rs:20-32` guarda `items: Vec<Value>`,
`named: IndexMap<..., Value>` e um único `span`. Não guarda ordem conjunta,
ocorrências named anteriores nem spans por argumento/valor.

Em `01_core/src/compiler/eval/call_dispatch.rs:389-430`, `eval_args` ainda
tem AST e spans individuais, mas retorna somente essa estrutura:

- `:403-408`: positional perde seu span; named é inserido no mapa, perdendo
  argumento completo e expressão-valor como âncoras distintas;
- `:410-419`: spread de Array/Dict/Args estende os containers de valores;
  no braço `Value::Args`, até o `args.span` original é descartado;
- `:430`: o span final é o da lista sintática atual.

No vanilla ratificado, a fonte
`lab/typst-original/crates/typst-eval/src/call.rs:412-460` mantém o span do
argumento completo e o span de sua expressão-valor. Spread Array/Dict usa
o span do spread (`:438-450`); spread de Args conserva os itens originais,
inclusive os spans (`:452`). Isso é uma diferença de observáveis
diagnósticos, não uma obrigação de copiar a estrutura Rust.

### 2. `.with` conserva somente o Args que recebeu

`01_core/src/compiler/eval/call_dispatch.rs:1325-1330` avalia os argumentos
antes de criar a aplicação parcial. `01_core/src/entities/func.rs:39-42,237-238`
guarda exatamente `(Func, Args)` no `With`; aliases clonam esse valor, sem
recuperar informação anteriormente descartada.

Na chamada, `call_dispatch.rs:457-459` combina os Args. O merge de
`:1016-1023` escolhe `new.span`, concatena posicionais e sobrepõe named por
`IndexMap::extend`. O span antigo ainda pode ser consultado imediatamente
antes do merge, mas não revela origens de Args encaminhados já apagadas
em `eval_args`.

No vanilla, `foundations/func.rs:372-374` (mesma raiz ratificada do parágrafo
anterior) concatena todos os itens prebound com os novos, conservando
ocorrências e spans. O recibo P1307 já mede
`yaml.encode.with(pretty:false)(...)` com erro ancorado no named pré-ligado,
em `17..30`, e não na chamada final.

### 3. Closures prolongam a vida da informação e consomem argumentos

`01_core/src/compiler/eval/closures.rs:72-114` consome named e posicionais
ao ligar parâmetros. O sink em `:117-131` guarda somente os valores
restantes e o span agregado da chamada original. Depois da retirada de
parâmetros, a posição dentro desse Args já não corresponde diretamente à
posição sintática original.

O corpo e snapshot de uma closure são guardados em `entities/func.rs:52-81`;
isso não significa que uma função nativa `With` devolvida por uma factory
guarde o ambiente local dessa factory. `FuncRepr::With` contém apenas sua
função interna e Args. Um wrapper closure que chama diretamente o encoder
com um parâmetro simples cria um novo argumento na AST do corpo; um sink
que encaminha `..args` conserva as origens externas no vanilla. As duas
rotas não podem receber a mesma política de ancoragem por conveniência.

### 4. Named anterior pode mudar o erro, além do span

A fonte ratificada
`lab/typst-original/crates/typst-library/src/foundations/args.rs:218-235`
converte cada ocorrência do named para o tipo solicitado e só então conserva
o último resultado. Uma ocorrência anterior com cast inválido não é
apagada por outra válida posterior. `:259-266` aponta o primeiro argumento
remanescente quando rejeita named desconhecido.

Consequência prevista diretamente da fonte, enviada para medição independente:

```typst
json.encode.with(pretty: "bad")((a: 1), pretty: true)
```

Deve falhar no `"bad"` pré-ligado, embora o named final seja booleano.
O merge cristalino `:1021-1022` elimina esse valor antes de a nativa recebê-lo.
Portanto um mapa `nome → span final` não basta. A informação necessária
inclui ocorrências anteriores e seus valores, além das âncoras. A observação
de que `pretty:false` seguido por `pretty:true` funciona não prova que todo
named anterior seja semanticamente descartável.

A medição independente R2 confirmou a previsão: os casos
`with-json-invalid-first-occurrence` e
`with-toml-invalid-first-occurrence`, em `extra_focal.rows` de
`00_nucleo/diagnosticos/p1307-r2-measurement.json`, retornam
`error: expected boolean, found string` em `<input-expression>:1:25`,
destacando o `"bad"` original. No YAML, que não aceita `pretty`, o caso
equivalente retorna `error: unexpected argument: pretty` em `1:17`,
destacando o argumento inteiro. O recibo conserva stdout/stderr literais,
fonte exata e argv; não se infere um cast booleano inexistente no YAML.
Na source congelada JSON, o range UTF-8 half-open do primeiro valor inválido
é `25..30`; o controle da última ocorrência inválida aponta `58..63`.

## Viabilidade do desenho sem alterar entidades

### Fragmento em que é plausível

Para chamada direta sem spread, o dispatch já tem todas as âncoras na AST.
Uma captura privada, selecionada pela identidade resolvida da nativa,
poderia seguir os precedentes atuais de spans. Para `With` cujas listas
pré-ligadas são sintaticamente diretas e cujas Sources continuam acessíveis,
examinar a cadeia antes da fusão pode fornecer as âncoras originais.
Um alias de uma função parcialmente aplicada não elimina por si só
`pre.span`; o bloqueio não deve ser exagerado como se todo alias o fizesse.

Essa proposta é limitada e refutável: uma origem sem AST recuperável, spread
dinâmico, valor named sobrescrito relevante ou Args retornado de sink/factory
que exija origem anterior a `pre.span` impede tratá-la como solução geral.
O contrato P1307 não autorizou retirar essas rotas nem aceitar fallback
detached para transformá-la em suficiente.

### Contraprova de perda por factory e spread

Programa para medição independente, com a mesma fonte exceto a seleção
da chamada final `f((:))` ou `g((:))`:

```typst
{
  let make(which) = {
    let a = arguments(nope: 1)
    let b = arguments(nope: 1)
    yaml.encode.with(..(if which { a } else { b }))
  }
  let f = make(true)
  let g = make(false)
  f((:))
}
```

Antes de avaliar `.with`, `a` e `b` possuem origens sintáticas distintas.
Depois de `eval_args` expandir o Args escolhido, ambos os caminhos entregam
ao With os mesmos valores e o mesmo span da linha `.with(...)`.
O ambiente local da factory já não pertence ao objeto devolvido. O vanilla
preserva o span de `nope:1` originado em `a` ou `b`.

Essa é uma inferência de fluxo de dados, não uma afirmação de execução
cristalina dos encoders ausentes. A observação vanilla foi confirmada pelos
casos R2 `with-spread-arguments-f/g`: erro `unexpected argument: nope`
em `3:22` para f e `4:22` para g, ambos com trace da chamada final em `9:2`.
Os casos `with-spread-closure-f/g`, que obtêm Args por sink de closure,
distinguem igualmente `3:27` de `4:27`. Os programas exatos e todos os
traces permanecem em `extra_focal.rows`; o exemplo acima é ilustrativo,
não substitui os fixtures literais medidos.
Os ranges UTF-8 half-open das fontes congeladas são `46..53` e `78..85`
para Args direto, `51..58` e `88..95` para sink, sempre cobrindo `nope: 1`.
A identidade incidental dos Arcs não
reconstitui a origem sem um novo armazenamento causal. Reavaliar a factory,
consultar nomes do exemplo ou procurar um valor igual na fonte seria uma
heurística, não prova de origem.

### Cross-source e Source de expressão

Um span já contém FileId (`01_core/src/entities/span.rs:95-103`), e
`World::source(id)` existe (`01_core/src/contracts/world.rs:43-44`).
Portanto uma solução que retenha o span original pode resolver sua fonte
de origem, inclusive importada. Deve usar esse FileId, não o arquivo da
chamada final nem o nome do alias.

Contudo recuperar AST depois não é um contrato universal: em
`01_core/src/compiler/eval/mod.rs:343-362`, `eval_expression_with_features`
cria uma Source code local e mantém no Engine o World recebido; não instala
a nova Source no World. Um World pode não devolver aquela Source por id.
O dispatcher recebe a AST atual diretamente, mas não uma garantia de
recuperação futura de toda Source por `world.source`.
`Source::span_byte_range` (`entities/source.rs:183-193`) também devolve
None para nó ausente ou FileId diferente.

Uma função pré-ligada exportada por módulo, renomeada no import, passada por
closure e chamada noutro arquivo precisa conservar causalmente sua origem.
Reabrir somente a fonte da chamada final não resolve essa rota; embutir
uma dependência de I/O de recuperação em serializer puro também não resolve
os Args de origem já colapsada.

## Desenho mínimo a desenvolver sob nova autorização

A obrigação de transporte precisa ser anterior a `eval_args`, acompanhar
valores `arguments`, sobreviver a sinks/clone/With e só ser consumida depois
da seleção do erro. O desenho deve preservar:

1. ordem das ocorrências posicionais/named relevantes;
2. valor de cada ocorrência necessária ao cast, inclusive named sobrescrito;
3. span do argumento completo e da expressão-valor;
4. origem de Args spread e regras distintas para Array/Dict spread;
5. consumo por parâmetros/sinks e prebinding sem trocar a proveniência;
6. nenhum novo observável nos encoders, `repr(arguments)`, igualdade ou
   lookup decorrente de metadata interna.

Uma proposta concreta para redigir o próximo L0 é acrescentar a Args um
carrier opcional, não exposto pela linguagem, com uma sequência imutável de
ocorrências. Cada ocorrência registra categoria positional/named, nome
quando houver, valor avaliado, span do argumento completo e span do valor.
O carrier também conserva a distinção entre ausência real de origem e
origem sintática conhecida. Os containers `items`/`named` atuais podem
continuar como projeções compatíveis para os consumidores antigos; a nova
validação dos encoders usa as ocorrências completas. Não se exige copiar
o tipo ou o algoritmo vanilla.

A proposta tem obrigações concretas de atualização: `eval_args` gera as
ocorrências no instante em que avalia uma vez cada expressão; `arguments`
transporta-as; spread de Args concatena-as; spread de Array/Dict cria
ocorrências no span do spread; With concatena pre/new sem descartar as
ocorrências antigas; o sink retira exatamente os argumentos consumidos;
clone preserva a sequência. A nativa escolhe o erro pela assinatura
congelada, tendo acesso tanto aos valores necessários ao cast como às duas
classes de âncora. O serializer em si permanece puro e sem recuperação de AST.

O campo poderia usar compartilhamento imutável em RAM, permitido em L1;
o detalhe de alocação não é requisito de linguagem. Não aprovar ainda uma
assinatura Rust: adicionar campo à struct pública, inclusive campo privado
que impeça literais externos, muda seu contrato de construção e exige gate.
Também é necessário decidir como as projeções legadas e o carrier se mantêm
coerentes quando um helper transforma Args; metadata stale não pode ser usada
como se fosse prova. Preservar somente spans e descartar valores de named
anteriores seria insuficiente, assim como guardar metadata apenas no stack
de uma chamada que termina antes de uma factory retornar.

Uma alternativa com estado lateral por avaliação também exigiria prova
de transporte por clones, modules, retornos e repetição. Não é uma solução
restrita ao dispatch com capacidades vigentes: faltaria um owner/carrier
para guardar esse estado. Estado global mutável é proibido em L1; metadata
escondida em argumentos de usuário ou namespaces públicos altera a linguagem.
Assim, “sem mudar entidades” não pode ser prometido por trocar o nome do
armazenamento sem auditar sua vida útil e observabilidade.
Isso não demonstra impossibilidade matemática de todo sidecar concebível:
um desenho causal persistente, com outro owner explicitamente autorizado,
poderia ser proposto e refutado pelos mesmos casos. A recomendação de carrier
em Args é uma escolha arquitetural a desenvolver, distinta da necessidade
observada de conservar informação.

## Owners exatos e gate

| Owner | Necessidade demonstrada ou condicional |
|---|---|
| `compiler/eval/call_dispatch.md` → `01_core/src/compiler/eval/call_dispatch.rs` | Captura/expansão antes da perda, fusão e entrega da origem correta. Necessário para o desenho local auditado. |
| `entities/args.md` → `01_core/src/entities/args.rs` | Primeiro owner a reabrir se a proveniência acompanhar Args. Seu contrato atual declara agregado-only e dívida de spans por argumento. Mudança ainda não autorizada. |
| `compiler/eval/closures.md` → `01_core/src/compiler/eval/closures.rs` | O sink reconstrói Args e consome parâmetros; precisa preservar a correspondência no desenho proposto. Não alterar captura eager nem semântica dos parâmetros. |
| `compiler/stdlib/collections.md` → `01_core/src/compiler/stdlib/collections.rs` | `arguments.filter/map` e delegações que removem receiver reconstróem Args; precisam atualizar as ocorrências coerentemente. |
| `compiler/eval/operators/join.md` → `01_core/src/compiler/eval/operators/join.rs` | O braço Args+Args precisa concatenar também a proveniência, conservando o comportamento público já autorizado. |
| `entities/func.md` → `01_core/src/entities/func.rs` | Auditar clone/With; alteração não é automaticamente necessária se Args transportar metadata. Necessária somente se o desenho mudar a representação de With ou sua interface. |
| `compiler/stdlib/loading.md` → `01_core/src/compiler/stdlib/loading.rs` | Continua dono de casts, mensagens e serialização dos encoders; não recebe responsabilidade por inferir origem perdida. |
| `compiler/eval/tests.md` → `01_core/src/compiler/eval/tests.rs` | Futuras regressões públicas independentes, somente após confirmação/gate aplicável. |

Há dívida documental prévia a sanear na redação futura: `entities/args.md:7`
ainda chama spread de adiado e `:95-102` descreve nativas que recebem
`&[Value]`, enquanto `call_dispatch.rs:410-419,461-468` já usa spread e
entrega `&Args`. `entities/func.md:203-219` também duplica uma interface Args
histórica. Essas descrições devem ser reconciliadas com o owner único de
Args, sem declarar que uma duplicação textual seja uma violation V15
atualmente detectada. O desenho de Func continua condicional: saneamento
documental não torna necessária uma mudança de sua representação Rust.

O inventário lexical abaixo foi executado nesta auditoria, no mesmo estado
de código registrado acima:

```text
rg -n '\bArgs\s*\{' 01_core/src 02_shell/src 03_infra/src 04_wiring/src --glob '*.rs'
rg -n 'Value::Args' 01_core/src/compiler/eval/operators --glob '*.rs'
```

Ele foi depurado: `02_shell/src/cli.rs:129` é outro tipo Args, e os hits
`fn ... -> Args {` são assinaturas, não construção por literal. Não usar
a contagem bruta como custo ou promessa de escopo. Há reconstruções de
produto em `compiler/eval/math.rs:341,596,1351`,
`compiler/stdlib/numbering.rs:36,103`,
`compiler/stdlib/foundations/int.rs:149,264`, além dos owners semânticos
acima. Seus L0 (`compiler/eval/math.md`, `compiler/stdlib/numbering.md`,
`compiler/stdlib/foundations/int.md`) precisam de auditoria antes de
qualquer adaptação; esta rodada não propõe arquitetura nesses módulos.

Literais em testes também exigiriam ajustes de construção sob seus owners:
`compiler/stdlib/shapes.md`, `compiler/stdlib/plugin.md`,
`compiler/stdlib/structural.md`, `compiler/stdlib/_comum.md` e nos próprios
owners já enumerados. Nenhum desses ajustes está autorizado por este
inventário. `Args::positional(...)` não exige por si uma mudança no caller
se sua assinatura for preservada e o constructor inicializar origem
sintética; ainda assim, cada transformação de Args existente precisa
ser classificada como transporte, consumo ou síntese.

O owner de collections foi lido integralmente, SHA-256
`f474e4b654509de4c5dda75223499463388d2e683be36b4eff11dbaf732839e5`.
A fonte `collections.rs:545-549` remove o receiver; `:2402-2452` filtra
argumentos e `:2455-2487` mapeia valores, reconstruindo Args.
O owner de join também foi lido integralmente, SHA-256
`e48c2c98e12fd7e76c26f8b3de5ead13b7b9d5812be28eda9605d9f90d68f092`;
seu consumer `operators/join.rs:73-76` funde os containers atuais.

Há uma fronteira adicional medida na fonte ratificada de Args:
`foundations/args.rs:414-418` preserva argumento e valor no filter;
`:438-445` preserva o span do argumento completo no map, mas cria o novo
valor com span detached. Portanto a proposta não deve impor origem de valor
inventada após map. Diagnóstico vanilla detached é observável legítimo,
distinto de Unknown do adaptador. A exigência geral de range no contrato
sucessor deve ser condicionada ao range que o vanilla efetivamente publica.

O hub `compiler/stdlib/_comum.md` só ganha ligação semântica adicional se
novos reexports forem necessários. A fachada `compiler/eval.md` só ganha
obrigação de transporte adicional se o desenho escolher contexto/entrypoints
como carrier. Esta auditoria prefere a proposta associada a Args justamente
para evitar mapa lateral cuja identidade e vida útil ainda não foram provadas.

O gate humano permanece obrigatório para a nova superfície dos encoders e
os defaults `pretty:true` já declarados em P1307. Adicionar campo ao Args
público ou alterar contrato de Func/contexto cria também gatilho explícito
ADR-0127 de contrato público/compatibilidade. Corrigir somente uma âncora
privada seria paridade interna, mas não elimina o gate já existente nem
autoriza retirar os casos que exigem informação persistente.

Antes de propor aprovação de implementação, produzir um desenho concreto
com todos os owners e contraprovas: direto, alias, cadeia `.with`, named
pré-ligado inválido sobrescrito por válido, duas ocorrências desconhecidas,
factory devolvendo With, sink com parâmetros consumidos, Args spread,
Array/Dict spread, closure com chamada direta e import cross-source.
Cada erro deve ter mensagem completa, hints/traces e source/range exatos;
dois positivos devem diferenciar validação adiada de rejeição indevida no
momento de `.with`. Unknown obrigatório continua bloqueante.

Este relatório propõe um carrier para a próxima especificação, sem o
materializar nem declarar sua aceitação. A autorização necessária é para
redigir o L0 de Args e dos pontos de transporte/transformação inventariados,
com as obrigações e limites acima, antes de pedir confirmação do contrato
público e dos encoders. A mudança pública e a implementação continuam
bloqueadas. A promessa de cobertura geral com o Args vigente recebido
somente pelo serializer foi refutada pela auditoria de informação.

## Retificação do diagnóstico predecessor

A afirmação de `p1307-contract.md` de que a função `encoder.with()` tem
repr `encode` foi erro de transcrição deste autor. O próprio recibo anterior
`p1307-contract-measurement.json` já registrava, nos casos
`json-encoder-with-identity`, `toml-encoder-with-identity` e
`yaml-encoder-with-identity`, o valor vanilla
`["function", "(..) => .."]`. Não houve mudança de alvo ou observável entre
as rodadas. Os casos R2 `with-{json,toml,yaml}-encoder-with-identity`
reconfirmam o resultado. A função original `encoder` tem repr `encode`;
sua aplicação parcial `.with()` tem repr `(..) => ..`. O predecessor fica
imutável como trilha de auditoria; nenhum L0 futuro pode herdar a frase errada.
