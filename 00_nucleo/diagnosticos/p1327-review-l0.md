# P1327 — revisão do L0 congelado e linhagem inicial

Regime A/B, sem atestação técnica de isolamento. Mesmo revisor e permissões
de `p1327-review-preliminary.md`; somente este relatório foi escrito.
Nenhuma leitura de materialization/context, nenhuma edição de produto/oracle.

## Entradas e observações

Manifesto verificado por SHA-256:
`cca33a6a95d40efa74d6dc8f7f910a711968a4c7bb771393ffc975bc9e515ea8`.
Baseline verificado por SHA-256:
`f8cee37f7f3556db93f935deb977790a0a13ddd232639334e3d3931cf8b504f8`.
O baseline registra HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`,
working tree não commitado, diff/stat, originais integrais dos quatro
arquivos autorizados e binários. Seus runs começam em
`2026-09-09T10:40:09.540621+00:00`; a fotografia causal começa em
`2026-09-09T10:40:09.228317+00:00`. O binário vanilla corresponde ao hash
ratificado `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.

Li as novas seções P1327 de ambos os L0 e o manifesto. O baseline demonstra
warning ausente para std/alias/calc, emissão vanilla antes de erro posterior,
dois imports em spans distintos e closure, além dos controles de as/items/
wildcard/field/bare dinâmico/resolução/tipo e rename redundante. Os últimos
dois permanecem dívida explicitamente delimitada.

O manifesto congela hashes do corpo normativo, removendo apenas a linha
recíproca `Hash do Código`. Recalculei independentemente em Node esses
hashes e os Hash B, usando as fontes atuais e os originais do baseline:

| Owner | SHA-256 normativo L0 | Hash B canônico |
|---|---|---|
| modules | `790a041521e4d358f9e1ba76d9128ba554d6050eeda5dda50573d2f371086abe` | `58ce1f7d8cee386aa258d43101560439880421633ab9cd9acd3c77a5fd1da499` |
| tests | `fa0c9d6c0d2f473cc16c281ebf4e780f1f2b511aac73d04a56a9a2353643f900` | `0b22e2167e25bb24b9d63fcfb45edc3d826e3c58fac3fd87355b137cc7cd9e53` |

Ambos os corpos Rust atuais, excluída apenas a linha `@prompt-hash`, ainda
são byte-idênticos aos originais P1327. Não existe candidato funcional nesta
revisão. Assim o Hash B anterior `589ad79e` em modules era preexistente e
não canônico para o corpo atual; `58ce1f7d` é sua correção verificável, não
indício de implementação antecipada.

`p1327-l0-preflight.json` contém apenas falha de instrumentação: CLI rejeitou
`--severity`, exit 2. Não é RED nem gate arquitetural. O sucessor r1 utiliza
`--checks v5,v15,v26`: acusa somente os dois V5 esperados após editar L0,
sem V15/V26. Seu exit zero não é declaração de zero warnings.

`p1327-l0-reseal.json`, executado de 10:42:24.806367 a
10:42:30.928746 UTC, registra aplicação somente aos dois pares autorizados.
Comparei os inventários before/after: mudaram apenas modules.md e os dois
headers Rust; tests.md já tinha seu Hash B válido. A reanálise reporta zero
drift. `p1327-l0-lineage.json` verifica explicitamente A/B e pin do núcleo,
com exit zero, entre 10:43:17.259797 e 10:43:18.203969 UTC. Os Hash A
registrados são `11d38a08…` para modules e `4470777c…` para tests.

## Decisão

L0 e manifesto estão suficientes e consistentes para avançar à autoria
independente/freeze dos testes e RED pré-candidato. O owner set permanece
um produtivo mais um test-only, com relações 1:1. O texto sucede
explicitamente só a dívida bare identifier P1305/P1306, preserva rename,
bindings e demais condições, e incorpora erro posterior, UTF-8, origem,
perfis e cardinalidade. A classificação contínua ADR-0127 permanece válida.

O protocolo é corretamente apresentado como ensaio A/B sem isolamento e
sem selo de refinamento; o manifesto não dá crédito a Unknown nem autoriza
paridade geral. O campo `step_sha256` foi tratado somente como referência
opaca: a materialização não foi lida nem utilizada como autoridade L0.

Ainda não há veredito RED/contrato A/B/candidato. Congelar e classificar
todo o corpus baseline antes de C; depois verificar sucessores exatos,
teste RED sem falhas de build/fixture, GREEN e gates finais. Não reiniciar
medição histórica válida nem reescrever recibo de instrumentação.
