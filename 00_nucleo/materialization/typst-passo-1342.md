# Passo 1342 — reautorizar o binding contra a topologia produtiva real

## Objetivo

Corrigir exclusivamente o fragmento de binding rejeitado no Passo 1341:
substituir papéis sintéticos, coordenadas incompatíveis e ledger pós-hoc por
um contrato que nasça da topologia produtiva medida, seja ligado a callsites
reais e receba eventos append-only desses pontos sob
`cfg(p1339_observation)`.

Este passo não fecha a matriz ampla lifecycle/profile, NT01–NT06, retenção,
descarte, invalidação e decisão terminal restante de P1340. Essa matriz só
pode ser materializada num sucessor depois do certificado independente deste
fragmento. A separação é deliberada para não repetir a expansão de escopo do
P1339/P1340.

Este documento é um passo de execução, não Prompt L0. Código continua
legitimado exclusivamente pelos owners em `00_nucleo/prompts/`.

Regime: **executado sem atestacao de isolamento**. O workspace é
compartilhado; a segregação é de autoridade, entradas, ordem e artefatos, sem
alegação de isolamento técnico.

## Baseline e causa medida

Medição em `2026-09-10T20:09:54Z`:

- HEAD: `2f42d64253547734564513a1159ee6b584c1c4b4`;
- working tree não commitada;
- SHA-256 de `git status --porcelain=v1 -z`:
  `69df14fde6d9e43aaf174c99b640bb4ebb0d136f12ec076d1599230662f3ba8f`;
- SHA-256 de `git diff --binary HEAD`:
  `02f7ee9a373030a62b79fb6816a964ff282c982b6d7ee49bf1f8a6b7ae8204d9`.

Entradas causais:

- `00_nucleo/diagnosticos/p1341-execucao-r1.md`, SHA-256
  `192a834c5d24783eebdc2c143db157c2fae5cf530bd951b6f76c80250c26b949`;
- `00_nucleo/diagnosticos/p1341-verifier-candidate-r1.json`, SHA-256
  `cf63928e5bc2360b58f6f828a3e5bb5ff8c82034e7ec513e3f098186588121f2`,
  veredito `REJECTED`;
- `00_nucleo/diagnosticos/p1341-verifier-preseal-r2.json`, SHA-256
  `ead304d1bb43faf54cc2d69fad7960fc67cd7e79159bc188ff4406c008709adb`.

O verificador mediu cinco causas independentes:

1. os objetos, IDs, eventos, ordem e arestas eram montados depois da
   compilação, embora alguns fatos de apoio fossem reais;
2. o manifesto nomeava owners e callsites, mas não fixava caminho, símbolo,
   âncora nem hash de um hook alcançável;
3. `callback.with`, `func.callback` e `func.body` não tinham correspondência
   1:1 demonstrada na mecânica real;
4. Dict e span do teste divergiam das expectativas protegidas;
5. normal/repeat/reverse repetiam a mesma chamada e o checker selado não era
   alimentado pelo ledger do produto.

O GREEN R1 prova apenas que o JSON construído satisfazia suas próprias
asserções. Não recebe crédito de binding.

## Decisão de atomização

P1342 possui uma só obrigação material: **binding real do fragmento focal**.
Não acrescentar branches produtivos de estabilização, alterar política
terminal nem aproveitar o passo para corrigir a matriz restante.

O candidato R1 não é baseline nem fonte de verdade. Antes de nova autoria de
contrato, isolar seus hunks P1341 num inventário reversível e produzir uma
visão candidate-free do código, preservando integralmente as alterações
herdadas de P1339/P1340 e de outros passos. Não usar `git checkout`, reset ou
reversão ampla. O inventário deve identificar cada hunk removido e permitir
reaplicação manual; os artefatos e o veredito R1 permanecem como evidência.

## Fase A — medir a topologia antes de nomear o schema

Um auditor de topologia, sem acesso ao futuro patch, lê a visão candidate-free
e registra em diagnóstico separado:

- onde nasce cada entidade observável;
- onde ela é clonada, encapsulada, despachada, percorrida e descartada;
- qual identidade realmente existe em cada ponto (`Func`, `FuncRepr::With`,
  closure/body AST, `Content`, `CounterUpdate`, `Location`, snapshot e Dict);
- quais spans continuam disponíveis e quais já foram perdidos;
- quais owners L0 e consumers produtivos 1:1 governam esses pontos;
- se há ou não correspondência real para cada papel do manifesto R4.

Toda conclusão inclui `file:line`. Inferências são marcadas e declaram a
medição que as refutaria. É proibido conservar `func.body` como `kind=func`,
por exemplo, se a topologia só possui um corpo AST e não uma segunda função.
O contrato deve acompanhar a mecânica observada; o código não cria entidades
fictícias para salvar nomes sintéticos.

Se a identidade ou o span exigidos já não existirem no owner, registrar a
lacuna antes de decidir transporte. O transporte permitido é um carrier
test-only criado no ponto produtor e preservado por clones/mapeamentos reais.
Side table global, busca por nome final, endereço de objeto reconstruído ou
inferência a partir do output são rejeitados.

## Fase B — L0 completo antes do selo

A partir da auditoria, atualizar somente os Prompt L0 dos owners realmente
necessários. Cada cláusula deve declarar:

- callsite produtor e consumidor;
- dado observado e identidade disponível;
- forma de transporte test-only, quando necessária;
- ponto exato de emissão append-only;
- ausência de efeito no build normal;
- limite explícito do fragmento P1342.

Executar V15/V26 e resselo de `@prompt-hash`. Depois congelar os hashes dos L0
atuais no manifesto P1342. **Nenhum L0 é alterado depois do selo.** Se o
contrato revelar que a cláusula L0 está errada, invalida-se a cadeia a partir
da Fase B; não se repete o paradoxo do pré-selo P1341 R1.

Mudança necessária em API/trait/entidade normal, default, compatibilidade ou
fase de pipeline aciona ADR-0127 e exige parada humana. Campo ou carrier
presente somente sob o cfg de observação não conta como contrato público, mas
precisa continuar legitimado pelo L0 proprietário.

## Fase C — novo contrato e manifesto de binding

O contrato P1342 sucede os contratos P1341 sem editá-los. O manifesto estático
de binding é fechado e cada linha contém, no mínimo:

- papel semântico estável;
- Prompt L0 owner e seu hash;
- consumer produtivo;
- caminho do arquivo, símbolo e âncora estrutural do callsite;
- hash do trecho ou arquivo pinado;
- espécie do objeto/evento emitido;
- fixture que alcança o hook;
- cardinalidade esperada por tentativa.

O verificador confirma que cada âncora resolve uma vez, pertence ao owner
declarado e é alcançada pela fixture real. Nome simbólico sem binding
estrutural é `Violated`.

As expectativas estáveis não contêm IDs runtime. Fonte e bytes das fixtures
são inputs protegidos anteriores ao candidato. Coordenadas são obtidas do
parser/span real da fixture e só então congeladas; não se escolhe primeiro
`offset=12,length=5` para depois forçar o código a fabricá-los. O Dict esperado
é medido da intenção da fixture antes do patch, preserva tipo e ordem e deve
ser produzível pela linguagem. Divergência entre fixture e expectativa impede
o selo.

O ledger possui uma célula por tentativa e é append-only:

- o produtor registra nascimento/identidade;
- o dispatch registra a função efetivamente recebida;
- a entrada registra o corpo efetivamente executado;
- o owner de counter registra chave/ação/span no ponto em que cria ou consome
  a ocorrência, conforme a topologia medida;
- introspect registra `Location`, snapshot e conteúdo realmente percorrido;
- eval registra o Dict tipado que efetivamente produziu.

A projeção final pode serializar referências já registradas, mas não criar
objetos, arestas, ordinais, causas ou identidades ausentes do log. Toda aresta
é emitida pelo ponto que conhece seus dois extremos; cobertura não é
autodeclarada pelo DTO candidato.

`Unknown` permanece permitido apenas para payload deliberadamente opaco, após
schema, tipos, manifesto externo, alcance, cardinalidade, temporalidade e
causalidade passarem. Falta de hook, span, identidade, cobertura ou fixture é
`Violated`, nunca `Unknown`.

## Fase D — segregação e gates antes do código

1. Autor da intenção: este passo e os L0 atualizados; não aprova a própria
   materialização.
2. Autor do contrato: recebe passo, L0, baseline candidate-free e auditoria de
   topologia; não lê o futuro patch.
3. Autor dos oráculos: recebe intenção, contrato e fixtures congeladas; não lê
   implementação.
4. Adversário: produz mutantes de omissão, alias, inversão, reparenting,
   fabricação pós-hoc, span reconstruído, Dict não injetivo, ordem ignorada,
   hook morto e manifesto autodeclarado; não corrige a solução.
5. Verificador de pré-selo: exige positivos `Preserved`, todos os negativos
   `Violated`, opaco `Unknown`, score `1.0`, repetição e reordenação.
6. Testador A/B: após o selo, escreve RED contra o contrato e fixtures, sem
   ler o patch candidato.
7. Implementador: recebe apenas L0, contrato/oráculos selados e RED congelado;
   não edita entradas protegidas.
8. Verificador final: lê inputs selados e outputs candidatos, mas não modifica
   nenhum artefato julgado.

O manifesto de autoridade registra allowlists, hashes, contexto herdado,
outputs e custos. A frase de regime é sempre
`executado sem atestacao de isolamento` enquanto o workspace for comum.

## Fase E — implementação autorizável

Somente depois de `SEALED_FOR_IMPLEMENTATION` e RED focal demonstrado:

- substituir os hunks rejeitados, não construir uma segunda camada paralela;
- emitir registros nos callsites reais pinados;
- preservar identidades e origem ao longo do transporte real;
- não executar callback adicional para observar;
- não alterar semântica, output, warnings ou custo do build normal;
- manter o registro compilado exclusivamente sob
  `cfg(p1339_observation)`/teste.

O teste candidato entrega o ledger real ao checker selado ou a uma fachada
mínima que chama exatamente o checker. Não reimplementa suas regras em
asserções Rust adaptáveis ao output.

`normal`, `repeat` e `reverse` devem ser operações distintas e observáveis:
normal percorre a lista congelada; repeat executa a mesma lista novamente com
World/registro frescos; reverse percorre a lista em ordem inversa. O recibo
registra a ordem realmente consumida e compara classificações, sem congelar
IDs runtime entre execuções.

## Aceitação

- visão candidate-free e auditoria de topologia pinadas;
- L0 atualizado antes do manifesto/selo; V15/V26 verdes;
- manifesto com arquivo/símbolo/âncora/hash e alcance demonstrado para todo
  papel obrigatório;
- fixture, coordenadas e Dict mutuamente reproduzíveis antes do candidato;
- mutation score `1.0`, sem sobreviventes ou negativos `Unknown`;
- RED demonstra ausência de pelo menos um hook produtivo real;
- GREEN alimenta o checker selado com ledger real inspectable e opaque;
- nenhum objeto, identidade, aresta ou causa nasce na serialização final;
- normal/repeat/reverse consomem ordens realmente distintas e concordam;
- teste focal repetido, build normal, rustfmt, `git diff --check`, V5/V15/V26
  e `crystalline-lint .` sem violações novas;
- certificado independente `ACCEPTED`, com escopo somente no binding P1342.

O certificado P1342 não fecha P1340/P1339 nem a matriz lifecycle/profile.
Ele apenas torna legítimo começar o passo posterior dessa matriz.

## Budget e condições de parada

- até 2 revisões de contrato após a primeira versão;
- até 2 revisões focais por nova classe de falha;
- 1 execução completa do corpus antes do selo e 1 na verificação final;
- parar após duas revisões com o mesmo vetor/causa;
- parar se uma classificação depender de informação ausente da fixture e do
  ledger binding-free;
- parar se o modelo de papéis continuar incompatível com a topologia medida;
- parar e pedir decisão humana diante de mudança ADR-0127.

Não reduzir score, adaptar oráculo ao patch, converter ausência em `Unknown`
ou promover GREEN local para ultrapassar uma condição de parada.
