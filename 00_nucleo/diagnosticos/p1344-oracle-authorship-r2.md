# P1344 — revisão focal da autoria independente do oráculo R2

## Veredito limitado

`FOCAL_R2_AUTHORED_NOT_VERIFIED_NOT_SEALED`.

Regime: **executado sem atestacao de isolamento**. A revisão preserva a
separação de papéis e o closed world congelado, mas o workspace continua
compartilhado. Esta autoridade não leu nem executou o candidato produtivo, não
alterou contrato, L0, passo ou código, não emitiu selo e não executou o corpus
completo.

## Raízes autorais R2

- corpus: `p1344-oracle-corpus-r2.json`, SHA-256
  `1246920352da5678603dadcd07efd2f1e2364e2e397d2d26fdd36b8b7ef757e5`;
- source verifier: `p1344-source-verifier-r2.py`, SHA-256
  `62824f399b34659b81a2ab6ff538ad0747b4bb76aed6c5af4cfc78fd17a20e82`;
- checker: `p1344-oracle-checker-r2.py`, SHA-256
  `62a9ebb7241fc6b2ff1495417b5a5f8b3d4ad547dc2e770cba6da8c60e118d5f`;
- authority root segundo a fórmula compilada no checker:
  `4ef44241165fef855f6d5d8ea646d70dec324817c9aa619764547c4435dbba27`.

Os artefatos R1 permanecem byte a byte preservados. O checker R2 pina por
path e SHA-256 o corpus, o verifier, o checker R1 e o runner adversarial R1;
o corpus também fecha os quatro artefatos adversariais R1. JSON estrito
rejeita chaves decodificadas duplicadas em qualquer profundidade e números
não finitos.

## Revisões públicas, uma por classe

1. `OWNER_HELPER_PRODUCTION_IS_TOKEN_PRESENCE_NOT_CLOSED_GRAMMAR`: as
   produções de `Content` e `CounterUpdate` passaram a exigir assinatura,
   retorno e `match` exatos, braços fechados, encaminhamento único e ausência
   de corpos laterais, callbacks, storage ou controle alternativo.
2. `INHERITED_AND_FUNC_PRODUCTIONS_NOT_CLOSED`: `Func`, `EvalContext`, os 12
   hooks diretos, carriers e façade passaram a ter inventário, ordem,
   receiver, retorno, argumentos, efeitos, storage e expressões fechados. A
   gramática proíbe loop, storage lateral, métodos extras, projeção mutável,
   substituição de argumentos e callee apenas comentado.
3. `UNKNOWN_LAUNDERED_WITHOUT_WITNESS`: `Unknown` exige um objeto estruturado
   com sete chaves em ordem e valores canônicos; ausência, forma escalar,
   chave extra ou valor divergente é `Violated`.

Cada delta preservou todos os negativos anteriores e incorporou os 23 ataques
R1 no recorte focal. Não houve segunda revisão de nenhuma destas causas nem
sobrevivente repetido. Durante a primeira execução foi corrigido apenas um
erro do parser do controle positivo, que cortava incorretamente o receiver
`&mut self`; nenhum negativo ou produção normativa foi enfraquecido.

## Cobertura e resultado focal

O recorte R2 contém 29 identidades: 6 controles e os 23 ataques adversariais
R1. A composição declarada contém os 139 casos R1 protegidos mais os 29 R2,
total de 168. A rota `--full` encadeia R1 e R2 em normal, repeat e reverse,
mas não foi chamada por esta autoridade.

Na execução focal final houve concordância 29/29: 23/23 negativos válidos
foram `Violated`, 5/5 controles transparentes foram `Preserved`, o único
controle opaco foi `Unknown` mediante witness canônico, zero ataques
sobreviveram e o mutation score focal foi `1.0`. `full_corpus_runs = 0`.

O verifier é exclusivamente de fonte. Runtime sintético existe apenas como
controle autoral e não pode ser fornecido por `--candidate-root` ou
`--runtime-evidence`; qualquer prova produtiva de reachability, freshness,
identidades e cardinalidades pertence à autoridade externa.

## Próximo gate

Um adversário independente deve atacar estes hashes exatos sem editar os
artefatos julgados. Só depois de zero sobreviventes uma autoridade de pré-selo
pode executar a passagem full normal/repeat/reverse e emitir ou negar o selo.

