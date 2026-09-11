# P1339 — resolução de desenho das observações e tentativas

Estado: **desenho revisável, sem selo nem veredito**. Autor: agente
`/root/p1339_observation_design`; autoridade de escrita restrita a este
diagnóstico. Não escreveu Rust, L0, contrato, oráculo ou teste. Regime:
executado sem atestação de isolamento. Skill Tekt e suas duas referências
lidas; não se atribui independência atestada ao filesystem compartilhado.
A autorização comunicada pelo coordenador — “Autorizo tudo até o fim do
1339” — permite integrar APIs estritamente necessárias ao escopo; não
dispensa o selo ou o RED.

## Proveniência da inspeção

HEAD `2f42d64253547734564513a1159ee6b584c1c4b4`; working tree não commitado,
inspeção de fontes produtivas ainda iguais ao baseline. Captura de hashes e
`git diff HEAD --stat`: `2026-09-10T02:27:18Z`. L0s estavam sendo redigidos
concorrentemente, logo referências normativas abaixo são sugestões para o
coordenador, não uma declaração de que o texto presente está congelado.

SHA-256 das entradas principais nessa captura:

| Caminho | SHA-256 |
|---|---|
| `00_nucleo/prompts/compiler/eval.md` | `a0ba87212163227d1ea51d38a401ca5277ee4e1d266993b826d81275924eaa76` |
| `00_nucleo/prompts/infra/pipeline.md` | `e326323e0b1e59310c24ce29c1d7cd2b49c4cc0b67815dde023f2ac8adc110c6` |
| `01_core/src/compiler/eval/mod.rs` | `115a34e8aa41b4ec5a55b0cec5d05927216dc6a4a6fcb98d2fd2c1edc98d262c` |
| `03_infra/src/pipeline.rs` | `72f9c080b55ca295e5b51ae45acf527c76921cd06211a64c2c43fdd010af6088` |
| `01_core/src/entities/value.rs` | `f792590f48799067f2f00767319c97aa498a519f207a154332ffc4deb6c8712b` |
| `01_core/src/entities/func.rs` | `9bcd2e2b0c79cae5c2478b55d9d902f78983f9119d634a04a846d93210307bde` |
| `01_core/src/entities/content.rs` | `34838354261fa472c5efbc73c39ee6587b8b77be4d4606ca7587537818ed4a98` |
| `01_core/src/entities/plugin_func.rs` | `5d58aac17741c4c507b90b6ca3f4d43ef06859bec34bfbd0973abd1c2ca33a20` |
| `01_core/src/entities/elements/dynamic.rs` | `503a3604c26b45fcf58fbb30a7c91742664ad8b820383ba6406f94103f512575` |
| `lab/typst-original/crates/typst/src/lib.rs` | `07946a1266c2bbdf8b7301d740e95819b84e7136c58711580e682b8964ea80a5` |
| `lab/typst-original/crates/typst-library/src/introspection/convergence.rs` | `7963e54a9eb2748f081fc15de4a487de4752159781bb486ea515120be809a07b` |

O diff/stat da captura registrou somente os seguintes L0s, todos sob
`00_nucleo/prompts/`: `compiler/eval.md`,
`compiler/eval/bindings/field_access.md`,
`compiler/eval/bindings/value_methods.md`, `compiler/eval/call_dispatch.md`,
`compiler/eval/operators/equality.md`, `compiler/eval/repr.md`,
`compiler/eval/rules.md`, `compiler/eval/selector_matching.md`,
`compiler/introspect.md`, `compiler/introspect/extract_payload.md`,
`compiler/introspect/from_tags.md`, `compiler/introspect/locatable.md`,
`compiler/layout.md`, `compiler/stdlib/counter.md`,
`compiler/stdlib/foundations/float.md`,
`compiler/stdlib/foundations/query.md`,
`compiler/stdlib/foundations/selector.md`,
`compiler/stdlib/primitives-constructors/version.md`,
`compiler/stdlib/state.md`, `entities/counter_registry.md`,
`entities/element_payload.md`, `entities/elements/emph.md`,
`entities/elements/strong.md`, `entities/introspector.md`,
`entities/selector.md`, `entities/show.md`, `entities/version.md`,
`infra/pipeline.md`. O comando totalizou 28 arquivos, 1822 inserções e
4 remoções; estes números identificam a árvore, não certificam trabalho.

Nenhuma sonda funcional foi executada nesta subtarefa. As medições vanilla
citadas vêm de `p1339-stabilization-boundaries-runs.json`, SHA-256
`4d82648d895ec5dba23a9775c2bf09dfae84a175603f3ce6d742c8c7f2a09241`,
com upstream ratificado `a51e02804`, binário e UTC no próprio recibo.
Também foram consultados `p1339-observation-integration.md` (SHA-256
`0bdc275d03cfecf4691f4dae762590743edbfd1105410a0bbee7f437cdfbc36d`),
`p1339-stabilization-design.md` (SHA-256
`d853d22302c15534b10d40357adc3a360211df787c84812df7b223ee3c97a06d`)
e recibo de dependências (SHA-256
`264a4a6fb76d64738714ab528295667491da53d20af6c93e2916f4762e139894`).

## Medição que delimita o comparador

`value.rs:26-44` conserva Content e fields separadamente no carrier;
`:59-189` é enum fechado. Portanto locations e campos superficiais não
cobrem o valor recebido por query. `Content::PartialEq`, em
`content.rs:3495-3500`, nem sequer cobre Metadata/State/StateUpdate e
termina em false. Não serve como comparador reflexivo pronto.

`func.rs:22-43,53-98` dá acesso interno a variantes, body, defaults,
patterns e captured scope. `Func::repr` em `:271-274` é acesso ao enum,
não a função de repr da linguagem; `Scope::iter` permite ler bindings.
`syntax_node.rs:25-36` possui igualdade estrutural derivada. Há dados para
examinar closures sem Debug e sem novo registry. Entretanto igualdade
estrutural do executável não resolve toda igualdade observável:
`func.rs:384-395` usa identidade de Arc para closures/With/Plugin e
`eval/operators/equality.rs:129` delega a `Value::PartialEq`. Comparar uma
função consultada com outra função capturada pode distinguir duas instâncias
com body/defaults/capturas estruturalmente iguais. Este achado refuta usar
essa estrutura como substituição universal de identidade na validação.

`func.rs:65-75` declara captura eager como snapshot; conservar a mesma
ClosureRepr conserva o mesmo captured Scope. A saída já retida pelo produtor
não precisa ser recriada para depois tentar provar que a nova closure é a
antiga. A condição causal é anterior ao teste de ponteiro.

`plugin_func.rs:28-34` contém host, module e export; `:85-95` exclui host
de sua igualdade. `contracts/plugin_host.rs:85-96` declara que transition
cria módulo derivado e mantém original inalterado. Mesmos module/name sem
mesmo host e versão lógica não constituem prova. `elements/dynamic.rs:45-69`
fornece trait aberto, campos individuais e dyn_eq, mas não um snapshot
exaustivo de estado interno nem garantia geral contra mutação interior.
`element_registry.rs:37` também contém um ctor opaco `Arc<dyn Fn>`.

## Decisão mínima implementável para comparação

Manter a reutilização causal já escolhida no L0. O comparador é privado de
eval, fechado por variante de resultado e distingue internamente
`Same`, `Different` e `Unproven`. O bool público vale true somente para
`Same` de todas as leituras e das precondições sob responsabilidade de L3.
Não introduzir igualdade global nova de Value/Func/Content.

1. Primitivos possuem comparação exata por variante. Inteiro e Float não
   colapsam. Floats usam representação IEEE preservada (`to_bits`), inclusive
   nas folhas geométricas: o mesmo NaN torna-se reflexivo; sinais de zero
   permanecem distintos. Isso não altera membership/associação de updates.
2. Arrays, dicts, Args, Selector, State e Counter percorrem conteúdo e
   campos em ordem. Incluem presença/ausência e os metadados de Args que
   afetam despacho/diagnóstico. LocatedContent inclui Location, distinção
   de carrier, fields completos e Content. Comparar somente `.fields()`
   também é insuficiente: o valor pode ser renderizado ou inspecionado.
3. Content estático é percorrido pelo enum e pelos dados dos elementos;
   inclui filhos, payload cru, estilos, callbacks, campos de presença e
   dados semânticos de execução. Nada de morph_canon/Debug/repr/hash_content.
   Não usar `PartialEq` genericamente em containers com Value/Func/floats.
   Folhas fechadas sem esses campos podem delegar a igualdade exata já
   auditada. O match exaustivo deve impedir novos braços silenciosamente
   aceitos; a desestruturação explícita das structs evita omitir campo novo.
4. Para Func e Module, o caminho mínimo reconhece a MESMA instância
   imutável preservada, com recursos/capturas da mesma geração causal.
   Isto é prova limitada de identidade, não equivalência de funções novas.
   Native exige a mesma variante, executável, nome e namespace preservado;
   nome isolado não prova captura. With inclui função base e Args retidos.
   Closure nova invalida conservadoramente, mesmo se a auditoria de body,
   patterns, defaults e captured Scope não detecta diferença estrutural.
   Não precisa executar a closure nem examinar código para adivinhar reads.
5. Substituir produtor, captura ou chain descarta o certificado de
   retenção e seus descendentes. Produtor congelado sem demanda filtrada
   conserva a MESMA saída; produtor selecionado com allreads válidos também.
   Assim a fixture de callback contextual recriada na fonte não exige criar
   uma nova callback em cada validação. É o reaproveitamento da execução,
   não a igualdade de Arcs frescos, que evita instabilidade artificial.
6. Sucesso/erro e diagnóstico completo são parte do resultado da operação;
   mudança de qualquer lado invalida. Mesma leitura com erro pode ser
   estável; o erro do corpo continua pendente, nunca convertido em sucesso.

Este desenho exige um percurso fechado dos dados observados; não exige
comparação estrutural universal de executáveis e capturas. A auditoria
estrutural de Func serve para localizar a diferença e desenhar testes, não
para anular a identidade que a linguagem atual consegue observar.

### Limite preciso de opacidade

Plugin pode reutilizar identidade quando se conserva a mesma função, host
e módulo original sob o contrato de imutabilidade do host. Um módulo
derivado é input diferente. A igualdade `(module,name)` sozinha é vedada.
Plugin reconstituído por host desconhecido, ElementCtor e Dynamic sem
garantia de snapshot completo permanecem `Unproven`; dyn_eq/Debug/Arc
isolado não curam ausência de contrato de imutabilidade.

`Unproven` é uma classificação do gate de evidência. NÃO é autorização
para introduzir um erro genérico novo da linguagem, nem para retornar
diagnóstico vanilla de não convergência, nem para afirmar que o documento
convergiu. Durante tentativas, tanto Different quanto Unproven podem
invalidar a execução e provocar nova avaliação; conservam-se internamente
as causas separadas. No teto, a política de produto para Unproven NÃO está
determinada pelos controles numéricos vanilla. O selo de um fragmento que
exige preservar esses opacos precisa de prova adicional de identidade
retida/imutabilidade, observação mais fina já autorizada, ou reconhecer
explicitamente essa incompatibilidade. Não eliminar o caso do contrato nem
inventar erro para fazer o gate passar.

Refutador concreto do caminho otimista: o mesmo dyn Arc altera estado
interior exposto por dyn_get_field; ponteiro e nome permanecem, saída muda.
Refutador do comparador estrutural de funções: função consultada é comparada
por `==` com função capturada, e a consulta troca a identidade por uma
closure estruturalmente igual. Ambos precisam ser tratados como fronteiras
reais, não como paridade já obtida. Outro caso a medir é callback de state
que cria Func nova em cada replay; não se prova estabilidade dessa identidade
com a simples comparação estrutural da callback produtora.

## Medição que fixa as tentativas

Vanilla `typst/src/lib.rs:140-143` usa EmptyIntrospector antes do primeiro
documento; `:156-160` produz documento e valida contra seu introspector;
`:163-180` conserva documento atual e monta história com seed e documentos;
`:184` só guarda documento anterior se vai repetir. `:187-190` promove os
erros atrasados do sink retido. `convergence.rs:16` fixa cinco tentativas;
`:56-67` só emite resumo se existem detalhes; `:240-249` compara os dois
últimos outputs da consulta; `:267-278` nomeia seed como run 1 e último
snapshot produzido como final.

O recibo de boundaries, `:13-18,38-40`, registra oscilação `[[0]]` com
história `0,1,0,1,0,1`; crescimento registra `[[4]]` e história até 5.
A saída mostra o valor LIDO pela última avaliação, enquanto o diagnóstico
final mostra o valor no snapshot que essa avaliação PRODUZIU. São distintos.

## Decisão de correspondência e texto sugerido para pipeline L0

> Nomear I0 o seed das observações e A_k a tentativa de produção k. A_k
> lê I_(k-1), produz árvore C_k e documento D_k; seu snapshot candidato
> completo é I_k. Reintrospecção, posições e PageStore de D_k compõem I_k.
> Validar os transcripts retidos de A_k contra I_k depois de completar
> essas projeções. Reutilizar um resultado previamente validado equivale
> a conservá-lo em A_k, com seu transcript/sink e precondições; não criar
> execução fictícia. Produtores não selecionados conservam sua contribuição
> ordinária enquanto existir a mesma geração causal.
>
> Quando houver invalidação, A_(k+1) reavalia somente os selecionados
> invalidados e descendentes cujo produtor mudou, contra I_k. Novos blocos
> reais são expandidos recursivamente contra o snapshot da tentativa atual;
> descobri-los não consome sozinho uma nova tentativa. Refazer C_k desde
> a árvore de origem e contribuições atuais. Corpo com erro contribui só
> seu marcador; seu sucesso anterior não permanece no documento candidato.
>
> Ao validar todos os registros, terminar: devolver o erro pendente original
> se existir, senão D_k. Ao esgotar A5 sem erro final, a saída é D5, que
> foi calculada lendo I4. Não executar A6, não reaplicar consultas a I5
> para substituir metadata/Content de D5 e não retornar D4 por confundir
> último snapshot lido com último documento produzido.
>
> Diagnose recebe [I0,I1,I2,I3,I4,I5] e SOMENTE as requisições efetivas
> retidas na tentativa final. Resolve essas requisições contra a história
> com seus próprios argumentos/capturas/estilos. Uma requisição não precisa
> ter sido executada em todas as tentativas: a história é replay da consulta
> final, não invenção de execução do corpo. Comparar suas projeções I4/I5
> decide se cabe detalhe; formato de history nomeia I0 como run 1, I4 como
> run 5 e I5 como final. Emitir resumo só se existem detalhes comprovados.
>
> Publicar sinks das contribuições efetivamente retidas e da tentativa
> final; descartar sinks de validação e execuções substituídas. Warnings
> globais de eval/CLI não são repetidos pelo laço. Erros independentes sem
> demanda Element alcançada permanecem legados. No teto, erro pendente
> continua erro e impede exportação como sucesso; warnings de não convergência
> não o removem. Ordenação/deduplicação de diagnósticos deve seguir o owner
> e o oráculo, nunca a iteração de HashMap.

O orçamento dessas tentativas deve ser único para dependências contextuais
e pages/positions: não abrir cinco rodadas internas por relayout. O ciclo
de math já delimitado permanece distinto. Tratar valor igual de página como
condição suficiente perderia campos de query/state/numbering.

### Seed: incompatibilidade concreta a resolver na integração

`pipeline.rs:644-654` usa introspecção estática já populada antes da expansão,
em contraste com o EmptyIntrospector vanilla. Os controles sem headings
estáticos não discriminam essa diferença. Se o counter inicial observa um
heading, começar com um valor adiantado pode mudar a saída no teto.

Há uma separação implementável compatível com o congelamento legado:
executar a passagem ordinária de descoberta e conservar suas contribuições
sem demanda filtrada; para a geração que EFETIVAMENTE alcançou demanda
Element, descartar sua realização de descoberta e iniciar A1 com vistas
observacionais vazias de I0, conservando a Location do bloco proveniente
da topologia de origem. A descoberta é mecânica preliminar, não entra na
história run 1..5 nem publica sink dos selecionados. Os produtores sem
leitura filtrada entram em C1 com sua saída congelada; I0 não contém seus
updates. A seleção fica associada à geração, mesmo se um ramo posterior
não consulta o counter. Esta política não executa callbacks nunca demandadas.

Isso é proposta concreta, NÃO escolha já congelada: a descoberta ordinária
pode falhar antes de alcançar Element ou seguir ramo diferente do seed vazio.
Pelo escopo vigente, esse bloco permanece legado. Logo não se pode prometer
equivalência com a seleção dinâmica global do vanilla para qualquer programa
misto. O refutador é query/state antes da primeira demanda Element decidir
se ela é alcançada, com resultado diferente no seed vazio e no seed estático.
A integração precisa fixar e medir essa fronteira; a autorização para todos
os passos do P1339 não transforma esse programa legado em obrigação de reparo.

Para o fragmento que alcança demanda Element na passagem ordinária, o
desenho fecha a correspondência A5/I4/I5 e permite executar os gates com
seed explicitado. Para pretender paridade de todo programa misto, os dois
requisitos — congelar a descoberta legada e usar seleção global vanilla
desde EmptyIntrospector — são incompatíveis sem ampliar o escopo. Não
mascarar a diferença alterando a contagem de tentativas.

## Entrega ao coordenador

Há mecanismo implementável de retenção causal + comparação fechada dos
valores + cinco documentos/história de seis snapshots, sem nova igualdade
global nem registry. Não basta bool Element, PartialEq de Content, Arc de
closure nova ou Debug. Permanecem duas fronteiras precisamente identificadas
para o contrato: imutabilidade/identidade de valores opacos realmente exigidos
e seleção no seed de programas mistos. Nenhuma delas autoriza runtime error
inventado ou PASS parcial disfarçado de equivalência geral. Este diagnóstico
não sela esses pontos nem executa seus testes.
