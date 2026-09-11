# P1339 — desenho do contrato independente, revisão 1

Autor: `/root/p1316_review`, somente contrato. Manifesto de autoridade r2
SHA-256 `842d6526739014022c073800148a47b3c886831e2198ab65bbfdbe73ce59411b`;
freeze L0 SHA-256
`397c136fc8710d44b2f7537193fe5b9c44ab89d40a99bf6b994296e05e7c4d04`.
Contexto herdado: revisão encerrada P1316, sem relação causal com candidato
P1339. Neste trabalho foram recebidos L0, baseline e sondas anteriores;
nenhum candidato P1339 foi lido. Regime completo, executado sem atestação
de isolamento; permissões são procedimentais num filesystem compartilhado.

## Decisões para a discriminação anterior ao candidato

O antecedente cristalino não oferece as dez rotas e não é um positivo-base
honesto para suas mutações. A fase de discriminação pode usar uma cópia
isolada da referência vanilla ratificada com programas de entrada públicos
do contrato. A fonte de referência, `/usr/local/bin/typst` e o lab permanecem
somente leitura. Registrar origem da cópia, hashes, patch, comando de build,
exit, binário e ambiente de cada modo. Os negativos são execuções reais de
programa alterado, nunca alteração de JSON, stdout gravado ou expectativas.

Um único build instrumental multiplexado por modo externo explícito é
admissível para mutações semânticas se: modo 0 reproduz exatamente a
referência não instrumental nos casos do contrato; cada modo ativa somente
uma mutação identificada; modo inválido falha fechado; seleção não depende
de nome de fixture, mensagem esperada, inspeção do runner ou resultado que
se queira obter; todos os modos têm fonte/binário/env pinados. A seleção de
modo pertence ao aparato de ataque, não ao futuro produto cristalino.
Usar o mesmo binário em vários modos não multiplica famílias: cada uma
precisa de testemunha própria e elegibilidade verificável. Compilação de
branches não alcançados não prova que o modo semântico foi executado.

A família 12 é diferente. Duplicação da fórmula pode manter todas as saídas
iguais; exige testemunha arquitetural sobre o código e grafo de chamadas,
não apenas blackbox. Um branch duplicado presente no mesmo binário do
controle já violaria a propriedade física de causa única. Portanto o
controle arquitetural e o negativo de duplicação devem ter artefatos de
compilação distintos, ou configurações de compilação verificadas que
excluam a segunda definição do controle. Um modo de ambiente não apaga
duplicação compilada. O verificador deve confirmar definição/caminhos
efetivos com localização de fonte e resolução de chamadas, não confiar
numa afirmação autodeclarada no registro do adversário. Para a referência
vanilla, provar compartilhamento do ponto semântico; para o candidato
cristalino, acrescentar propriedade L0/consumer e owners congelados.

Na família 20, `Unknown` bruto é retido. O predicado do contrato para caso
obrigatório reprova com `mandatory_unknown`; não o converte em Preserved.
Um erro normal com span ausente é diferença diagnóstica conhecida, portanto
`Violated/diagnostic_origin`, não deve ser renomeado Unknown para simular
a mutação. Um ataque real pode tornar a execução obrigatória inobservável
por terminação anormal ou por indisponibilidade controlada da fronteira de
observação. Compilação e execução devem existir e seus canais/exit/sinal
ser preservados. O caso obrigatório permanece obrigatório. Não alterar
expectativa, recibo, classificação exigida ou texto raw como atalho.

Mutation score usa famílias válidas discriminadas, com 20 famílias exigidas;
nenhum mutante inválido, redundante, equivalente sem testemunha arquitetural
ou não executado recebe crédito. Uma família pode precisar de mais de um
negativo, mas eles não substituem outra família. `Unknown` obrigatório não
explicado pelo controle negativo M20 com proveniência completa,
positivo não preservado, modo não isolável ou família faltante impede selo.
Budget: até três revisões e duas execuções completas pré-selo, focal primeiro;
duas revisões consecutivas sem ganho exigem redesenho, sem reduzir gates.

## Fronteiras observáveis já estabelecidas

O contrato canônico referencia casos e recibos imutáveis por hash/ID; o autor
dos oráculos produzirá expectativas independentes e o verificador executará
os predicados. A autoria deste documento não sela nem executa mutantes.

- Dez rotas: presença, tipo, nome/repr, chamada, cast, aridade, named,
  prioridade de avaliação, diagnóstico integral e equivalência estática/ligada.
- Bytes IEEE são valores da API: comparação exata, incluindo tamanho/endian,
  zero negativo, binary32, infinito e NaN conforme a entrada declarada.
- `where` inclui grupo ordenado, identidade nativa, igualdade de linguagem,
  show real, query/counter, ocorrência Strong/Emph e estabilização seletiva
  autorizada nos L0; descoberta isolada não fecha a obrigação.
- Os 108 Unknown históricos de transporte show continuam históricos;
  a ponte canônica os relaciona às observações sucessoras de compile, sem
  contá-los como sucesso ou apagar os registros.
- Angle NaN conserva Unknown condicional sem crédito, segundo resolução
  e revisão r2 pinadas; infinitos construíveis continuam obrigatórios.
- Seed legado sem demanda Element alcançada é preservação do baseline,
  não promessa de consertar programas mistos. Registro incompleto ou valor
  obrigatório Unproven não recebe essa exceção.

## Metadado derivado e transporte

A inspeção mecânica do linter confirma a separação: `prompt_io.rs:164-235`
remove somente a linha canônica validada no preâmbulo; `hash_writer.rs:15-19`
deriva o hash do consumer sem `@prompt-hash`, e `:63-87` prepara o par;
`nucleus.rs:200-215` calcula o hash efetivo do prompt sem `Hash do Código`.
São fontes em `/repos/Antigravity/tekt-linter/03_infra/`, pinadas no contrato,
assim como o executável instalado. Nenhum consumer P1339 foi lido para isso.

O freeze raw permanece o retrato histórico. Depois da implementação, somente
os oito hexadecimais daquela linha derivada podem mudar, com raw antes/depois,
campo antes/depois, hash normativo idêntico e verificação independente da
derivação e V5/V15/V26. O hash normativo remove somente a linha validada;
nenhum espaço, pin de núcleo, exemplo no corpo ou outro metadado é ignorado.
Qualquer outra alteração invalida o selo. Isso evita o ciclo de hash sem
permitir mudança normativa escondida sob resselo. Skill e passo exigem
congelamento da obrigação/proveniência; não impõem imutabilidade impossível
do valor derivado após o novo código.

Os suplementos ainda precisam de ponte bilateral congelada antes do selo:
compile usa arquivo nomeado quando stdin não funciona em ambos. Wrapper de
metadata pode observar o argumento já calculado e devolver o metadata
original, sem acrescentar context/query. Serializabilidade não prova que
asserts transientes serão descartados até A5: isso exige sonda focal própria,
valor final inteiro, histórico/warnings e mapa de spans. Transporte ou
projeção obrigatória não demonstrados continuam bloqueios do selo, não
exceções tácitas. A autoria do contrato não fornece os valores esperados.

Os hashes completos adicionais e as obrigações executáveis serão registrados
em `p1339-contract.json`; este desenho não é autorização de implementação.
