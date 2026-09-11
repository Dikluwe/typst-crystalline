# P1339 — classificação A.2 após sondas, sem selo

Regime: executado sem atestação de isolamento. Autor de medição
`/root/p1339_measure_r1`, contexto inicial restrito à tarefa delegada. Nenhum
source/L0 produtivo cristalino ou candidato foi lido. As quatro fontes vanilla
foram lidas somente depois das sondas exploratórias correspondentes. Não foi
produzido contrato, implementação, selo ou veredito final.

## Proveniência e ordem causal

Autoridade: `p1339-authority-manifest-r1.json`, SHA-256
`074ec7a3cfeea34f0f6a230dcd4cc4b9b2777a860ce439bdad8c774c034f38fc`.
Passo explicitamente autorizado:
`00_nucleo/materialization/typst-passo-1339.md`, SHA-256
`817c3a1476897fb0a847c90183e9a9fe690994f60023126997c8022c4e8b86a9`.
Retificação autorizada registrada em `p1339-resume-r1.json`, SHA-256
`657f6eeeee2aa8a30a5443f930125231af38ce0088c63956ce17d38bb112fa02`.

Todos os números e exemplos abaixo procedem dos recibos `p1339-full-*`,
sobre HEAD `2f42d64253547734564513a1159ee6b584c1c4b4`, working tree com
documentos não commitados enumerados integralmente em cada manifesto/recibo.
`git diff HEAD --stat` estava vazio nas medições. Os binários pinados:

- Vanilla ratificado `a51e02804`: `/usr/local/bin/typst`, SHA-256
  `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
- Cristalino antecedente P1338: `/tmp/p1338-target.vlNAmp/release/typst`, SHA-256
  `f7c8085f8453ecb6b10841b698092d41e7fbec64d9d46f9341b9b9b538bfefa1`.

Manifestações de presença, tipo, nome, `repr`, valores, erros e spans foram
medidas como observáveis da linguagem. `Preserved` nas comparações significa
apenas igualdade dos canais públicos do caso nos dois binários pinados.
Não significa que toda a família ou toda a intenção esteja cumprida.

## Medições antes de classificação

### Ângulo

No recibo final vanilla, `angle.deg(90deg)` devolve float `90.0`,
`angle.deg(1rad)` devolve `57.29577951308232`; `-0deg` devolve `-0.0` e seu
`signum()` é `-1.0`. Os pares estáticos/ligados foram medidos separadamente.
Inteiros e floats sem unidade são rejeitados. Extrair `(90deg).deg` como
valor não é suportado: `cannot access fields on type angle`.

Fonte: `lab/typst-original/crates/typst-library/src/layout/angle.rs:142`
documenta conversão para radianos e `:148` conversão para graus; assinaturas
públicas retornam `f64` em `:144` e `:150`. `:63` e `:247` explicam a unidade.
Os construtores Rust `:43` e `:48` têm nomes semelhantes, mas não são essas
rotas da linguagem. A classificação é linguagem para conversão, tipo,
rejeições e resultado; `Scalar`, `AngleUnit`, macros e organização são mecânica.

O suplemento `boundaries` mediu três construções candidatas de NaN, tanto
em graus como em radianos: `float("nan") * 1unit`, `0unit * float("inf")`
e subtração de ângulos infinitos. Todas viram `0deg` no vanilla e Angle NaN
no cristalino antecedente. A divisão pública por `1deg` e a autoigualdade
confirmam a distinção. O caso que chama conversão após `nan * 1rad` mede
conversão de zero no vanilla; ele não testemunha conversão de Angle NaN.
Infinito Angle, por sua vez, é construível bilateralmente.

Inferência: não há receiver Angle NaN comum demonstrado no universo testado.
Refutação: uma fixture pública, independente da conversão a testar, que
construa Angle NaN em ambos os binários. Não se afirma impossibilidade
universal e não se autoriza alterar a aritmética de Angle. A obrigação de
Angle NaN permanece `Unknown`; a regra geral de A.1 faz esse `Unknown`
obrigatório bloquear a progressão, apesar da cláusula especial instruir
precisamente essa classificação quando não há construção bilateral.

### Float e bytes

`float.inf` e `float.nan` são floats; NaN não compara igual a si mesmo.
`signum` devolve `1.0` para positivo e `+0.0`, `-1.0` para negativo e
`-0.0`, e NaN para NaN. Inteiros são aceitos pela forma estática, booleanos
e strings não. Formas ligadas e extração como valor foram medidas; a
extração de método em float é rejeitada pelo vanilla.

Fonte: `foundations/float.rs:12` documenta a coerção de inteiro quando se
espera float, `:20` as constantes, `:97` especifica signum com zero assinado
e NaN. Não confundir casts de parâmetro float com o constructor `float`,
que documenta outros tipos em `:41`.

`to-bytes(1.5)` devolve `bytes` cujo array é
`(0, 0, 0, 0, 0, 0, 248, 63)`; `size: 4` sem endian devolve
`(0, 0, 192, 63)`. Foram medidos little/big, tamanhos 4/8, zero assinado,
finito positivo/negativo, infinito e NaN, com round-trips e entradas de bytes
independentes. Comprimentos 0/3/5/7/9 são rejeitados com
`bytes must have a length of 4 or 8`. Tamanhos fora de 4/8 têm o erro
`size must be either 4 or 8`, exceto casts anteriores como tamanho negativo
(`number must be at least zero`) ou acima do u32 (`number too large`).

Fonte: `foundations/float.rs:114` e `:151` documentam a API; `:124` declara
os comprimentos e binary32/64; `:131` e `:162` declaram little; `:172`
declara tamanho default 8. `:180` mostra redução para binary32. A fonte
motivou sondas adicionais de arredondamento, subnormal, underflow e overflow:
por exemplo `0.1` em binary32 vira `0.10000000149011612`.

Os bytes são o resultado dessa API da linguagem, portanto sua igualdade
exata é pertinente. A estrutura de dados e as operações Rust usadas para
produzi-los não prescrevem a arquitetura cristalina. Inferência de defaults
é refutada por uma chamada sem named correspondente que devolva outro valor;
essa fronteira foi medida, não apenas deduzida da fonte.

### Função e Selector

As medições de `with` cobrem closure, nativa e elemento; argumentos
posicionais, named, mistos, espalhados e múltiplas aplicações. Argumentos
pré-aplicados aparecem antes dos da chamada: o caso variádico medido resulta
em posicionais `(1, 3, 5)` após duas aplicações. Named posterior substitui
anterior: o caso `a:1` pré-aplicado e `a:2` na chamada resulta em `2`.
Named duplicado escrito diretamente dá erro; duplicado vindo de spreads
foi aceito e o último prevaleceu. A extração `calc.pow.with` como valor foi
rejeitada, enquanto a chamada ligada funciona.

Fonte: `foundations/func.rs:395` declara pré-aplicação; `:397` recebe `Args`
sem reconstruir os argumentos públicos. `:372` explica a concatenação.
O uso de `FuncInner::With` e `Arc` é mecânica, não obrigação estrutural.
Ordem, efeito e spans são linguagem. Os casos com `panic` mostram que
argumentos são avaliados antes de casts da chamada estática; o receiver
inválido na forma ligada pode falhar antes dos argumentos. Não unificar
essas duas prioridades por conveniência.

`where` produz Selector para heading, figure, strong, emph, raw, text e
table; os casos cobrem campos vazios, campos válidos, campo desconhecido
e duplicado por spread. `strong.where(body:[Hi])`, `emph.where(body:[Hi])`
e `text.where(text:"Hi")` produzem Selectors no vanilla. Closure e nativa
não-elemento são rejeitadas sem executar o corpo. `where` sobre um elemento
embrulhado por `.with()` também foi rejeitado, mesmo sem pré-argumentos.
`heading.where(level:"bad")` é aceito como filtro: valores de campos não
são convertidos para os tipos dos parâmetros do constructor nessa operação.

Fonte: `foundations/func.rs:412` documenta identidade do elemento e valores
de campos. `:433` exige elemento direto; `:310` só extrai essa variante,
portanto `With(element)` não passa. `:440` valida nome do campo; não há cast
do valor aqui. `:430`/`:431` preservam posicionais para validação posterior;
o positional extra foi medido como erro, com span da ocorrência original.

O suplemento `show-final` força realização no vanilla: uma show rule com
esse selector emite `metadata("MATCH")`, lida com `typst query`. Para
strong/emph/text, vazio e matching produzem `["MATCH"]`; mismatch produz
`[]`, nas duas formas, quatro perfis e duas ordens. Isso comprova aplicação
real no vanilla; os casos
`eval` com styles sozinhos comprovam apenas morfologia, pois a realização
é diferida. No perfil default o cristalino antecedente rejeita essas
fixtures por falta da rota ou suporte ligado. Nos três perfis com features,
o subcomando `query` rejeita a flag `--features` antes de carregar a fixture:
as 108 células correspondentes são `Unknown` de infraestrutura, não
testemunhas de rejeição semântica. O recibo original foi preservado.

O recorte corretivo `show-focal-r1` tentou a alternativa `eval --in` sugerida
pela própria warning do binário: vanilla realizou a mesma fixture nos
quatro perfis, mas cristalino rejeitou `--in` em todos. A consulta posterior
a `--help`, registrada em `p1339-full-cli-capabilities.json`, confirmou essa
limitação e indicou features no subcomando compile. Um recorte compile de
uma única fixture, `show-compile-focal`, já havia começado e terminado
quando o operador mandou interromper novas tentativas. Seus recibos foram
preservados; nenhuma rodada adicional foi iniciada. Compile só observa
aceitação/diagnóstico, não o valor de metadata. Esses recortes não
substituem nem reclassificam as 108 células Unknown da rodada original.

Esses resultados exigem uma avaliação arquitetural pelo operador sobre o
gate público de Selector; esta autoridade de medição não leu a representação
cristalina e não conclui qual campo ou assinatura deve mudar. A testemunha
de linguagem é independente dessa escolha. Refutação: os mesmos selectors
deveriam aceitar, filtrar e produzir o efeito observado numa medição pública
equivalente. Plugin WASM específico não foi ensaiado; a alternativa nativa
não-elemento da expressão “plugin/nativa” foi coberta, sem alegar cobertura
de plugin.

### Versão

`version.at` estático e `.at` ligado foram medidos sobre versões vazias,
curtas e com componentes intermediários. Índice positivo além do comprimento
devolve zero; índice negativo é relativo ao comprimento explicitamente
fornecido. Fora desse comprimento produz
`component index out of bounds (index: ..., len: ...)`, com índice original.
O literal mínimo i64 precisou ser escrito como `(-9223372036854775807 - 1)`:
a escrita decimal direta é rejeitada antes de chamar a rota.

Fonte: `foundations/version.rs:109` documenta a operação, `:116` o índice
negativo, `:124` o erro e `:130` o zero-padding positivo. `:11` a `:24`
documentam os componentes e sua extensão semântica.

Os controles individuais de `major`, `minor`, `patch`, `repr`, display e
componentes explícitos preservam o antecedente como informação separada.
Há tensão entre comentário e comportamento na própria fonte: `:38` diz
zero para componente nomeado ausente, mas `:42` a `:47` e a medição de
`version().major`/`version(1).minor` retornam `unknown version component`.
Não se infere intenção do comportamento nem se autoriza corrigir esse
contraste como parte de `version.at`. Refutação para a medição: executar as
mesmas expressões no binário pinado e obter zero; refutação para intenção
requer documentação normativa adicional do alvo, não uma suposição.

## Limites e correções de sonda

A exploração original tinha dois defeitos locais: auxiliar
`calc.is-infinite` inexistente nos observáveis de Angle e literal mínimo
i64 rejeitado pelo parser. O runner original foi preservado em
`p1339-full-probe-explore-r0.py`; recibos exploratórios conservam seus
hashes. Uma revisão focal substituiu o auxiliar por signum público e
construiu i64 mínimo por subtração, sem mudar resultados esperados depois
de candidato. O recorte passou no vanilla antes da rodada final.

O manifesto `focal-r1` lista o catálogo completo disponível naquela revisão;
o runner arquivado aplica filtro explícito e o recibo contém os IDs
efetivamente executados. Isso é uma limitação de descrição daquele
manifesto exploratório; os manifestos finais listam exatamente seus casos.
O suplemento `boundaries` usa o runner base por import; seu ponto de entrada
é `python3 00_nucleo/diagnosticos/p1339-full-boundaries.py boundaries` e
ambos os hashes constam do recibo de entrega. Um bytecode Python temporário
foi gerado por esse import e removido do caminho exato; nenhum artefato
material foi apagado.

Nenhuma destas medições resolve o `Unknown` de Angle NaN, emite selo,
autoriza mudança pública, prova isolamento técnico ou fecha P1339.

## Estado da medição entregue

O catálogo contém 539 casos: 479 no corpus principal, 42 fronteiras
adicionais motivadas pela fonte e 18 fixtures de realização. Isso corresponde
a 4.312 comparações de células bilateralmente observadas em duas ordens e
quatro perfis: 656 `Preserved`, 3.548 `Violated` e 108 `Unknown` de transporte.
Não houve instabilidade normal/inversa. As 54 diferenças entre default e
outro perfil, por ID e binário, procedem da mesma rejeição CLI em query.
Essas contagens estão em `p1339-full-comparison.json`, vinculadas por SHA
ao catálogo, manifestos, recibos e estado Git de cada execução.

Separadamente das células, a obrigação de receiver Angle NaN comum é
`Unknown`: sua impossibilidade de construção demonstrada não foi usada
para marcar conversão como sucesso. Plugin WASM específico não foi medido;
o catálogo o identifica como limite opcional, cobrindo nativa não-elemento
como representante da redação “plugin/nativa”.

A.1 está executada no recorte descrito, mas inconclusiva para progressão.
O bloqueio de intenção/observabilidade de Angle NaN é distinto da lacuna
do transporte de show e distinto da decisão arquitetural de Selector.
Nenhum deles autoriza correção fora das dez rotas ou mudança silenciosa
da política de `Unknown`.
