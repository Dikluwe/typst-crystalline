# P1342 — autoria independente de oráculos (r1)

Regime: **executado sem atestacao de isolamento**. Papel: autor independente
de oráculos, sem acesso a candidato futuro P1342. Veredito desta autoridade:
**ORACLE_AUTHORED_NOT_VERIFIED_NOT_SEALED**.

Este artefato não é contrato, ataque adversarial independente, pré-selo,
implementação, teste A/B nem certificado final. O corpus cobre somente o
fragmento focal de binding real P1342.

## Entradas e fronteira

O corpus consome o passo explícito P1342, manifesto de autoridade, inventário
candidate-free, auditoria/recibo de topologia, freeze L0 e todos os artefatos
`p1342-contract-*` r1. Seus hashes completos estão em
`p1342-oracle-corpus-r1.json` e serão repetidos no recibo de autoria.

Nenhum código candidato P1342 foi lido ou escrito. Os dez consumers pinados no
freeze L0 foram verificados por SHA-256 e permanecem exatamente nos hashes
candidate-free do binding manifest. O checker lê os pins e as âncoras diretamente
do manifesto externo; um DTO não possui campo aceito para declarar a própria
cobertura.

## DTO fechado e identidades

O DTO tem exatamente `schema`, `fixture`, `binding_manifest`,
`payload_visibility` e `attempts`. Cada tentativa tem exatamente uma célula e
um stream de eventos. Cada evento tem exatamente `seq`, `event_id`,
`prev_event_id`, `kind`, `hook`, `refs` e `data`; `refs`/`data` são fechados por
`kind`. Membro desconhecido, variante não listada ou primitivo errado é
`Violated`. O checker usa `type(x) is int`; portanto JSON `true` não satisfaz
inteiro.

IDs são não vazios e tipados por domínio (`attempt:`, `callback-with:`,
`func-callback:`, `syntax-node:`, `counter-occurrence:`, `carrier:`,
`counter-content:`, `location:` e `snapshot:`). Seus valores são locais a uma
execução. O runner remapeia todos eles em `normal`, `repeat` e `reverse`; nunca
compara um valor runtime entre execuções. Alias entre domínios é rejeitado.

O stream exige ordinais contíguos, IDs de evento únicos e predecessor imediato,
além das relações causais. Isso torna deleção, inserção no prefixo, overwrite ou
reordenação observável `Violated`; reencadear ordinais não salva inversão causal
nem cardinalidade errada.

## Coordenadas, tipos e cardinalidade

Os oráculos fixam apenas os ranges reais medidos pelo parser da fixture:

- span da ocorrência `counter.update`: `144..164`;
- `syntax-body`, espécie `SyntaxNode`, kind `CodeBlock`: `64..121`;
- Dict `witness`: `82..97`;
- Dict pré-ligado: `155..162`.

`syntax-body` nunca aceita espécie `Func`. O Dict pré-ligado é a sequência
ordenada tipada `[('a', Int(1))]`; o Dict witness é
`[('outer', Dict([('a', Int(1))]))]`. A representação é lista de pares, não
objeto JSON sem ordem. Span e contexto causal distinguem os dois Dicts; ordinal
fixo não é identidade.

`R` não existe como campo aceito no DTO: o checker o deriva da contagem de
`callback-replay-enter`. Exige `R in {0,1}`, `2*R` dispatches na ordem
`callback-with`, `func-callback`, e exatamente `R` unwraps, closure dispatches,
body enter/exit, Dicts witness e replay exits. A trajetória não incorporada
prova o ramo `R=0`; a incorporada prova `R=1`.

## Lista fechada de casos

Ordem congelada, também gravada literalmente no corpus:

| id | classe esperada | ataque/controle |
|---|---|---|
| `P01-selected-r1` | `Preserved` | cadeia inspecionável completa, incorporada, `R=1` |
| `P02-nonincorporated-r0` | `Preserved` | resultado explícito não incorporado, `R=0` |
| `O01-deliberate-witness-opacity` | `Unknown` | somente payload do Dict witness opaco após predicados não-payload |
| `N01-self-declared-coverage` | `Violated` | campo de cobertura autodeclarada |
| `N02-bool-is-not-int` | `Violated` | `true` no Int do Dict pré-ligado |
| `N03-static-hook-missing` | `Violated` | remoção de H15 no manifesto externo |
| `N04-static-hook-duplicated` | `Violated` | duplicação de H14 no manifesto externo |
| `N05-source-span-swapped` | `Violated` | `144..164` trocado por `64..121` |
| `N06-dicts-swapped` | `Violated` | payloads pré-ligado/witness trocados |
| `N07-fictitious-func-body` | `Violated` | `syntax-body` falsamente tipado como `Func` |
| `N08-with-edge-inverted` | `Violated` | aresta outer/inner invertida |
| `N09-with-edge-disconnected` | `Violated` | endpoint inner órfão |
| `N10-carrier-lost-at-walk` | `Violated` | carrier trocado após clone/walk |
| `N11-cross-domain-location-snapshot-alias` | `Violated` | Location alias de snapshot |
| `N12-ledger-order-altered` | `Violated` | Dict witness antes de body-enter, com ordinais reencadeados |
| `N13-append-prefix-deletion` | `Violated` | remoção de evento anterior |
| `N14-relational-dispatch-duplicate` | `Violated` | terceiro dispatch com `R=1`, reencadeado |
| `N15-relational-dispatch-role-order` | `Violated` | ordem wrapper/inner alterada |
| `N16-ordered-dict-collapsed-to-object` | `Violated` | pares ordenados colapsados em objeto |
| `N17-attempt-close-before-replay-exit` | `Violated` | close antecipado, com ordinais reencadeados |
| `N18-event-schema-open-member` | `Violated` | membro desconhecido em evento |
| `N19-productive-hook-substitution` | `Violated` | evento atribuído a hook estático errado |
| `N20-result-bool-is-not-int` | `Violated` | `true` no resultado `Int(1)` |

Há dois positivos, vinte negativos e exatamente um opaco deliberado. O checker
também valida o manifesto fechado com exatamente quinze hooks, seus hashes L0 e
consumer, e resolução única das âncoras no escopo declarado. Ausência ou
duplicação de hook é `Violated`, não `Unknown`.

## Normal, repeat e reverse

- `normal`: consome a lista acima na ordem escrita com namespace local fresco;
- `repeat`: repete a mesma lista a partir de DTOs e namespaces frescos;
- `reverse`: consome a lista inversa, igualmente fresca;
- relação esperada: a classificação de cada ID é idêntica nas três execuções.

O caso opaco pode resultar `Unknown` somente porque schema, bindings externos,
tipos exteriores, spans, cardinalidade, identidade, append-only, temporalidade e
causalidade continuam válidos. Qualquer outra ausência é `Violated`.

## Limite deliberado

Não se fecha lifecycle/profile P1340, NT01–NT06, retenção, descarte,
invalidação, política terminal, equivalência geral ou aceitação de implementação.
O mutation score e o selo pertencem ao adversário/verificador independentes. O
resultado aqui é exclusivamente autoria de oráculos prontos para esse gate.
