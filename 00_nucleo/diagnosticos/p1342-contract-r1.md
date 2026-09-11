# P1342 — contrato fechado de binding real (r1)

Regime: **executado sem atestacao de isolamento**. Papel: autor independente
do contrato. Este artefato não é selo, oráculo, ataque, teste, implementação ou
veredito final.

## Resultado

O contrato sucede, sem editar, o veredito `REJECTED` de
`p1341-verifier-candidate-r1.json` (`cf63928e...`). Fecha somente o binding
P1342 da fixture congelada. Corrige a ontologia rejeitada: `callback-with` é o
`FuncRepr::With` real, `func-callback` é o seu `inner: Func`, e o antigo
`func.body` passa obrigatoriamente a `syntax-body`, espécie `SyntaxNode`.
Nenhuma função fictícia pode representar o corpo.

O JSON normativo é `p1342-contract-r1.json`. Objetos e enums nele declarados
são closed-world: membro desconhecido, primitivo de tipo errado ou variante
não listada é `Violated`. Em particular, `bool` nunca satisfaz inteiro.

## Fixture e coordenadas medidas antes do candidato

`p1342-contract-fixture-r1.typ` tem 178 bytes, somente LF, LF final e SHA-256
`98159f5ac529520590a197521dfb23383cec0ba6373b431b8b33f426cfc3a714`.
O parser real candidate-free (`Source` + `LinkedNode`) produziu raiz `Markup
0..178`, sem nó `Error`. As coordenadas são ranges de bytes com fim exclusivo:

| papel | kind real | range | bytes |
|---|---|---:|---:|
| closure callback `step` | `Closure` | `47..121` | 74 |
| `syntax-body` | `CodeBlock` | `64..121` | 57 |
| Dict `witness` | `Dict` | `82..97` | 15 |
| contextual | `Contextual` | `123..177` | 54 |
| corpo contextual | `ContentBlock` | `131..177` | 46 |
| chamada `c.update` | `FuncCall` | `136..164` | 28 |
| origem agregada `args.span()` | `Args` | `144..164` | 20 |
| chamada `step.with` | `FuncCall` | `145..163` | 18 |
| Dict pré-ligado | `Dict` | `155..162` | 7 |

A compilação real da fixture com o binário candidate-free terminou com exit 0.
O span da ocorrência é `144..164`; não é a definição de `step`, o texto
`step` nem qualquer coordenada herdada de P1341.

Os Dicts esperados são tipados e ordenados. O pré-ligado é
`[("a", Int(1))]`. O `witness` é
`[("outer", Dict([("a", Int(1))]))]`. A multiplicidade de dois Dicts impede
aceitar “algum Dict visto”: o evento do `witness` precisa ocorrer em
`82..97` durante a execução causal do `syntax-body`.

## Ledger e fechamento causal

Existe um ledger compartilhado append-only, com uma célula por tentativa real
de `Session::execute`. Cada append acrescenta um evento imutável ao fim. É
proibido apagar, sobrescrever, inserir no prefixo ou reordenar. O DTO final só
serializa o que já foi registrado; não cria objetos, IDs, ordinais, spans,
Locations, snapshots, Dicts, causas ou arestas. Cada aresta nasce no hook que
conhece simultaneamente os dois endpoints reais.

O carrier é o mesmo através dos clones reais de `Func`, ação,
`CounterUpdateElem`, payload e `EvalContext` filho. Side table global, endereço,
nome, repr, igualdade final ou output não podem reconstruí-lo. IDs runtime são
locais, tipados por domínio e não vazios, mas nenhum valor concreto é congelado.

Para cada tentativa há exatamente um `attempt-open`, um `context-dispatch`, um
Dict pré-ligado, uma criação de ocorrência e um `attempt-close`. O walk ocorre
uma vez quando o conteúdo dessa tentativa entra num snapshot candidato; se não
entrar, ocorre zero vezes e o resultado não incorporado precisa ser explícito.
Se `R` é o número de visitas reais do resolver que selecionam a ocorrência
focal nessa tentativa, então `R` é 0 ou 1 para esta fixture e vem dos hooks, não
do DTO. Há `R` replay-enter/exit, With edges, closure dispatches, body
enter/exit e Dicts witness, e exatamente `2*R` dispatches de Func: wrapper e
inner, nessa ordem.

A ordem causal mínima é:

```text
attempt-open
  < context-dispatch
  < dict-prebound
  < counter-occurrence-created
  < counter-occurrence-walked (se incorporada)
  < callback-replay-enter
  < dispatch(callback-with)
  < with-unwrapped
  < dispatch(func-callback)
  < closure-dispatch
  < syntax-body-enter
  < dict-witness
  < syntax-body-exit
  < callback-replay-exit
  < attempt-close
```

Não executar callback adicional para observar. `Location`, span de fonte e
snapshots ocupam domínios distintos. Falta, duplicação, alias entre domínios,
span errado, Dict errado, hook morto, carrier perdido, aresta desconectada ou
inversão causal são `Violated`.

## Política de classificação

- `Preserved`: todos os predicados de schema, binding estático, fixture,
  alcance, tipos, cardinalidade, identidade, append-only, tempo e causalidade
  passam, e o payload obrigatório é inspecionável.
- `Violated`: qualquer desses predicados falha ou falta informação necessária
  para prová-lo.
- `Unknown`: somente payload deliberadamente opaco, depois de todos os
  predicados estruturais e causais passarem. `Unknown` não é sucesso e não sela
  caso positivo.

Normal consome a lista futura de casos na ordem congelada; repeat consome a
mesma lista com World/ledger frescos; reverse consome a lista inversa também
com World/ledger frescos. As classificações por caso devem coincidir sem
comparar IDs entre runs.

## Limite e estado

Estado: `AUTHOR_CONTRACT_COMPLETE_NOT_SEALED`. Um autor independente de
oráculos e um adversário ainda devem produzir casos; o pré-selo exige score
1.0, opaco somente `Unknown`, repetição/reordenação estáveis, alcance de todas
as âncoras e hashes protegidos intactos.

Este contrato não autoriza lifecycle/profile P1340, NT01–NT06, retenção,
descarte, invalidação geral, política terminal, paridade geral, mudança pública,
default ou fase do pipeline.
