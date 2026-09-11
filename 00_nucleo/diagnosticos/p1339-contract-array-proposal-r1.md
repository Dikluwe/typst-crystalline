# P1339 — proposta prospectiva de sucessão estreita Array ← Bytes

Estado: proposta de admissibilidade, **não selada**, aguardando medição e pins
L0 novos. Autor `/root/p1312_tests`, sem leitura de fonte candidata. Protocolo
completo executado sem atestação de isolamento técnico. O contexto histórico
P1312 e a autoria exploratória show P1339 são declarados; não fornecem evidência
para Array. Esta autoridade escreve somente `diagnosticos/p1339-contract-array-*`.

## Causa anterior à decisão

O recibo independente `p1339-verifier-array-prerequisite-gap-r1.json`, SHA-256
`1f8c66dd50aa592d0e1924580b5569458bde971d13ee8b6395301e788a34957e`, registra
que 70 programas públicos positivos congelados observam bytes através de
`array(b)`. A execução C do antecedente falhou antes, na rota float ausente;
isso não provou disponibilidade do constructor Array posterior. Uma inspeção
unitária direta de Bytes é útil, mas não substitui esses programas públicos.

Predecessores imutáveis: `p1339-contract-r3.json`, SHA-256
`c0cd1826679ddaf77e937a677839c5cbe64fdae6ec71bfa34e635d584d9c3e17`, e
`p1339-seal.json`, SHA-256
`35f00c4b9e15a010692017f5083ea4f451f4104a3730022136972e151ac0a8ee`.
As contagens históricas pertencem à proveniência completa desses recibos, não
a uma medição nova deste autor. A direção explícita do usuário, transmitida
pelo operador, autoriza ampliar L0 para o pré-requisito Array ← Bytes, preservar
os 70 oráculos públicos e corrigir os testes. Não é autorização de conversor
Array geral nem uma exceção mecânica de resselo.

## Admissibilidade semântica e ownership

Um novo L0 `compiler/stdlib/primitives-constructors/array.md` deve legitimar um
único consumer proprietário de conversão Bytes em Array. Os hubs somente
declaram/reexportam esse owner. O dispatch de `Type::Array` deve encaminhar
somente a rota cujo primeiro positional seja Bytes; nenhuma fórmula/conversão
fica no dispatch, e nenhum registry ou nova API de entidade é necessário.

Observar Array de inteiros por byte, preservando integralmente comprimento,
ordem, multiplicidade, extremos sem sinal e entrada vazia. Bytes UTF-8 são
bytes, não caracteres Unicode. Esta obrigação não é satisfeita por Bytes,
String, valores Float, apenas repr semelhante ou um round-trip que compense
dois erros. Casts e precedência de aridade/named da rota Bytes precisam vir da
medição anterior e do L0 pinado, sem presumir mensagens nesta proposta.

Preservar integralmente comportamento antecedente fora da guarda: Array ←
Version, Array ← Array, inteiros, strings, none e demais nonBytes; argumento
ausente e named sem positional Bytes não ganham constructor por acidente.
Um Bytes posterior a primeiro positional nonBytes não basta para abrir a rota.
Não alterar a descoberta pública de membros de Array ou habilitar outras
conversões. Os 18 scope-outs de P1339 continuam fora da alteração.

## Conjunto mínimo proposto para autoria independente de oráculos

Os exemplos abaixo são fontes candidatas, não expectativas diagnósticas seladas.
O autor dos oráculos deve medi-las nos binários pinados e fixar fontes, argv,
exit/stdout/stderr, erro primário, hints, span e traces antes do patch Array.

- Bytes vazio; Bytes literal com `0, 1, 127, 128, 254, 255`; sequência assimétrica
  e repetida; e espectro completo de 256 valores em ordem conhecida. Exigir tipo
  Array e tipo inteiro dos elementos, além do conteúdo integral.
- `array(bytes("é"))` deve testar a fronteira entre octetos e caracteres; sua
  expectativa concreta é confirmada pela medição independente antes do freeze.
- Alias de `array`, spread de Array/Args com Bytes como primeiro positional e
  named anterior a esse positional distinguem a guarda lexical da causal.
- Excesso positional depois de Bytes, named desconhecido e conflitos de erros
  na rota Bytes distinguem validação pública de simples extração do primeiro
  valor. Ordem de avaliação com `panic` é conservada pelo caminho vigente;
  não pode surgir avaliação duplicada ou antecipação artificial do cast.
- Preservações: `array()`, somente named, Array literal, Version, string,
  integer, none, primeiro nonBytes com Bytes posterior e diagnóstico dos quatro
  controles Version já congelados. Usar expectativa baseline onde ela diverge
  do vanilla: esses casos não expandem o escopo autorizado.
- Reexecutar os 70 programas públicos anteriores sem alterar ID, fonte,
  argumentos, type/bytes esperados ou predicado. O caso de extração de campo
  que falha antes de `array` mantém sua atribuição causal distinta.

Discriminação mínima por mutantes reais e compiláveis, sob autoridade adversária:
stub que rejeita Bytes; saída de tipo errado apesar de dados semelhantes;
conversão com sinal/truncamento; inversão de ordem; perda do último/primeiro
byte; deduplicação; tratamento por caracteres; aceitação de excesso/named
inválido; guarda que promove Array/Version/nonBytes; guarda que busca qualquer
Bytes posterior ao primeiro positional. Cada família exige testemunha concreta.
Uma única fixture pode discriminar várias famílias; nenhuma família recebe
crédito somente por um nome no manifesto. Mutante inválido não é rejeição.

## RED, integração e reutilização de evidência

O RED independente do novo pré-requisito deve executar `array(bytes(...))`
construído sem `float.to-bytes`: a falha pública deve alcançar Array. Erro de
compilação de teste ou ausência anterior de float não contam. Positivos do
vanilla e controles de construção Bytes no baseline legitimam a fixture.
Entradas sem identidade, ausência de canal ou parser opaco são Unknown e
bloqueiam a fase devida; nunca viram sucesso por default.

A mudança normativa reinicia a aceitação do recorte afetado desde B/C; o selo
R3 e seus resultados C/D permanecem registros históricos. A sucessão deve
listar dependências para justificar reutilização dos artefatos não afetados.
Preservar integralmente todas as obrigações W/F e vinte famílias negativas
anteriores. Não transferir automaticamente PASS C para runtime F.

Depois do focal válido e do gate discriminatório aceito pelo verificador,
executar o conjunto afetado com quatro perfis e ordens normal/repeat/reverse,
incluindo os 70 positivos intactos, controles fronteiriços e novas testemunhas.
O rebaseline global mantém seu universo e denominador de 4.718 probes; qualquer
delta de linguagem Array agora autorizado deve ser identificado, sem editar
matrizes antigas ou atribuir uma equivalência geral.

## Budget prospectivo, antes de novas execuções

Os históricos continuam gastos: três revisões de contrato, dois lotes focais
adicionais e uma execução C completa de duas. Nenhum contador é reiniciado.

Propõe-se **um único sucessor semântico estreito** para Array ← Bytes e **no
máximo dois ciclos focais novos**, exclusivos deste pré-requisito. Cada ciclo
deve registrar hipótese, entradas, positivos/negativos/Unknown, custo e ganho
discriminatório. Duas tentativas sem ganho na mesma causa exigem nova direção;
não há terceiro ciclo automático nem quarta revisão geral disfarçada.

Nenhuma execução ampla antes de o focal afetado passar. A validação ampliada
é somente do recorte afetado e não renova a cota global C restante. Se o
verificador considerar necessária outra C completa, ela consome a segunda e
última vaga histórica; essa vaga não dispensa focal. Gates finais F e rebaseline
continuam obrigatórios segundo seus budgets vigentes, sem crédito antecipado.

## Pendências para vincular o suplemento

Receber medição anterior e L0 novo pinado; ler integralmente os owners afetados;
confirmar a exceção explícita à exclusão anterior de constructors; registrar
hashes normativos/raw e política estrita de metadado; obter plano de oráculos,
negativos e RED independente; e submeter a sucessão ao verificador. Nenhum
código Array ou veredito fica autorizado apenas pela existência deste rascunho.
