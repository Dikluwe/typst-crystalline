# P1325 — gate anterior ao candidato

Veredito `PASS_PREPATCH_REAL_RED`. Regime A/B sem atestação técnica de
isolamento e sem selo. Revisor `/root/p1325_review`; não escreve L0, produto,
snippets ou expectativas julgados. Detalhes de assertions não foram
comunicados ao implementador.

## Evidência e proveniência

Baseline e HEAD são os pinados pelo manifesto P1325, conferidos em
`p1325-review-l0-audit.json`. Recibos julgados preservam seus estados
before/after com HEAD, diff/stat e inventário produtivo.

- R0 `p1325-unit-red.json` continua falha de instrumentação, não RED.
- R1 `p1325-unit-red-r1.json`, SHA-256
  `401db0efe0cb7c0459a9fb1e1c8c78532c96efbbfbfe309386622e8f496c6101`,
  execução de `2026-09-09T00:59:16.484264+00:00` a
  `2026-09-09T01:00:38.845788+00:00`: compilou, executou quatro testes,
  três falharam em assertions de âncora (uma categoria por teste) e o
  controle positivo passou. Exit 101 é RED semântico real desta execução.
- `p1325-unit-red-located-baseline.json`, SHA-256
  `2ffb0a36b9aef648a3a7a42f86541dc0670fbd4226c9ac6e04fcd47315bbd27d`,
  execução de `2026-09-09T01:01:20.165070+00:00` a
  `2026-09-09T01:01:20.547581+00:00`: o teste de fronteira LocatedContent
  foi efetivamente executado e passou para snapshot Some e None. Não foi
  confundido com o filtro anterior que não o selecionava.

O snippet R1 SHA-256
`0b97482bad70e0a14f97f425559110668c5dabeb96b7244109ac72bb7efffa1e`
é sufixo literal do consumer integrado. Ao retirá-lo e normalizar somente o
header @prompt-hash, o consumer reproduz o baseline exatamente: ainda não
existia implementação candidata. O diff R0→R1 corrige o import de Route e
acrescenta a fronteira Located; não relaxa as assertions originais.

## Contrato A/B anterior a C

Freeze `p1325-ab-freeze.json`, SHA-256
`b5a2e3b5697243780734caf58c38ad78e5f18b5b8fb1fa42c1342c3ecc8f0853`,
está presente e todos os hashes protegidos conferem. O hash normativo de L0
permanece o do manifesto. Classificação integral do baseline CLI:
44 diferenças de âncora, quatro residuais de disponibilidade, 52 controles
coincidentes e 12 residuais externos preservados. Proveniência desses números:
`p1325-ab-baseline-cli.json`, SHA-256
`e5251512b942c1df1e02e9fb66695e03aa1c92291c8ccb789341d338781d2b10`.

Expectativas `84d19c0cf924eca410f28f23341899290ce792faf31218f8cda373c22b9b4b78`
e comparador `9c2e542f35455b66e15eed5c4c475f01bdbbc9eef679bd1e430d26b2ed1748aa`
foram inspecionados pelo revisor. O comparador exige exit/stdout/stderr
inteiros para cada caso/perfil/produto e ocorrência única nas três ordens;
ausência ou duplicação falha. `[x].text` mantém erro cristalino com âncora
corrigida e não recebe crédito de paridade. Os residuais integer/string/
closure mantêm a saída baseline integral. Não há comparação que converta
Unknown em sucesso.

O recorte está pronto para C sob a norma congelada. Depois de C ainda serão
necessários julgamento do diff produtivo, testes GREEN abrangentes, corpus
bilateral com repetição/inversão, verificação dos protegidos e linhagem final.
Este recibo não antecipa esses gates nem afirma paridade geral.
