# Limpeza dos artefatos Tekt iniciados em 2026-08-26

## Fronteira e decisão sobre o histórico

A primeira inclusão inequívoca do protocolo nocivo é o commit
`4d738fe931f265a49a8e7473cfb5b8169ccbfbbb`, de
2026-08-26T16:55:18-03:00. O predecessor imediato
`89835fd0629cbcddf00331bb3b8e2eff4ffb15c5`, de
2026-08-26T15:54:00-03:00, é a fronteira anterior usada para selecionar apenas
artefatos protocolares criados depois da contaminação. Os passos de execução
foram preservados como memória histórica, ainda quando descrevem aquele período.

A cadeia alcançou `origin/Tekt` em `8ceea20f92dff06b6ca5a34828135c0fd3e35248`.
Por isso a limpeza é aditiva, por commits novos: o histórico publicado, os
reflogs e os objetos antigos não são reescritos nem coletados sem autorização
explícita.

## Valor técnico preservado

- Todo o código produtivo em `01_core`–`04_wiring`, seus Prompts L0 e Núcleos
  Tekt continuam no repositório.
- Testes pequenos de linguagem, DOM, SVG, PDF e diagnósticos foram mantidos.
  Hooks de mutação, catálogos, hashes de runners e repetições administrativas
  foram retirados desses testes.
- O modelo reutilizável `lab/parity/matrix/svg_morphology.py` e seu teste foram
  mantidos porque o runner geral de paridade ainda os usa.
- As configurações de inventário `p1283-probes.json`, `p1284-probes.json` e
  `extra_seeds.json`, além do extrator de color maps, foram mantidas. Baselines
  congelados e runners one-shot não foram mantidos.
- Fixtures pequenas de comportamento em `lab/parity/corpus` e a árvore vanilla
  ratificada permanecem disponíveis.
- As dez fixtures `.typ/sec_*` e o workspace `benches/` foram restaurados
  depois que a compilação demonstrou seu uso funcional. `benches/` data de
  P441, em 2026-06-24, portanto é anterior à fronteira de contaminação.

## Conclusões úteis condensadas

- A referência vanilla ratificada é
  `586e1bd43fae6c9a973218163d3165c53ab8d16d`, árvore
  `2847920ee80c9101a8a122d5c22f6700826b6d6e`; o binário medido em
  `8ceea20f92dff06b6ca5a34828135c0fd3e35248` tinha SHA-256
  `eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`.
- O inventário público histórico medido em
  `06e136d19fdc6d5f83fbbe44414b8b91e4dfd658` encontrou 8.724 células:
  8.012 presentes nos dois lados, 544 somente vanilla e 168 somente
  cristalino. Isso é cobertura nominal, não prova de paridade semântica.
- A amostra focal do mesmo commit tinha 54 células ativas: 45 `MATCH`, 3
  diferenças e 6 desconhecidas; 45 de 48 casos decididos coincidiam (93,75%).
- A medição de 66 rotas em
  `613721db562436ff6e16ff2de911e2f1537613e6` obteve 65 coincidências. A única
  diferença, `html.frame`, era uma projeção SVG estrutural excessivamente
  estrita, não um defeito demonstrado da linguagem.
- Em `1cbe98caef0be8958558d1fefaa7bc46d976cbde`, ficou demonstrado que `watch`
  não deve ser executado como subprocesso bloqueante com stdout/stderr
  capturados. O parsing deve ser exercido em processo; subprocessos de teste
  precisam ser finitos.
- Em `23cd47e4ade4d6deb7095cb7400478feffb1e9f9`, `format.svg` preservou identidade,
  set rule, `pretty`, precedência `caller explícito > documento > false` e
  neutralidade em targets não-SVG. As suítes focais registradas foram 7/7 em
  wiring, 7/7 em L1 e 6/6 em L3.

## Escopo removido

Foram selecionados pela fronteira temporal e removidos 2.680 arquivos
rastreados: 2.458 diagnósticos brutos, 221 executáveis/fixtures/baselines
laboratoriais one-shot e o ADR administrativo 0131. Os 2.660 arquivos textuais
somam 1.850.437 linhas; outros 20 eram binários, sem contagem de linhas pelo
Git. Nenhum passo de execução foi removido. O total final deve ser lido do diff
do commit de limpeza, pois também inclui a poda cirúrgica dentro de arquivos
funcionais preservados.

Os passos rastreados de 1140 a 1463 foram preservados. Os números 1301 e 1355
jamais existiram como arquivos `typst-passo-N.md` no histórico alcançável; seus
assuntos existem em prompts, testes e relatórios próprios, portanto não foram
fabricados retroativamente. Os `mtime` dos passos rastreados foram reconstruídos
a partir da data do último commit que tocou cada caminho, a aproximação
auditável disponível porque o Git não armazena o `mtime` original do filesystem.

## Temporários removidos

Foram removidos aproximadamente 37,8 GiB de artefatos ignorados e
descartáveis: quatro árvores `target`, três diretórios `__pycache__` e seus
bytecodes. A varredura final não encontrou `target`, `__pycache__`, caches de
pytest/mypy/ruff, `.pyc`, `.pyo`, swaps ou temporários de editor dentro do
projeto. Toda a validação desta limpeza usou `CARGO_HOME`, `CARGO_TARGET_DIR`,
`TMPDIR`, caches XDG e bytecode Python sob `/dev/shm`, sem fallback para o HD.

## Validação

- `crystalline-lint .`: exit 0, sem erros; 286 avisos históricos.
- `crystalline-lint --checks v5,v7,v15,v26 .`: zero violações.
- Python: 21/21 testes de `surface-inventory` e 25/25 de `svg_morphology`.
- Testes focais saneados: P1250 1/1, P1428 4/4, P1286 4/4, P1289 2/2 e
  P1462 7/7. P1293 terminou 9/10; o caso restante reproduziu o timeout
  produtivo preexistente na compilação PNG com serialização HTML cristalina.
- `cargo test --workspace`: compilou e executou 5.924 testes com sucesso;
  oito falhas históricas permaneceram em `repr/math.lr`, alinhamento e
  inventário HTML.
- `cargo clippy --workspace --all-targets --all-features`: executado; manteve
  oito erros históricos (atributo `since`, três loops que não iteram, duas
  comparações extremas e dois literais aproximados de PI), além de avisos.
- `cargo build --workspace`: GREEN.
- Referências exatas aos diagnósticos/laboratórios removidos: zero; referências
  ao ADR administrativo removido: zero. O scanner de links encontrou somente seis links quebrados antigos para
  `relatorios/fecho-debt-1-passo-142.md`, todos em arquivos não modificados.
- `git diff --check`: GREEN.

Esta consolidação foi preparada sobre o HEAD
`a1286f958f15a539dca17a3696ef83ef9e5b1b78`, em working tree não commitado, em
2026-09-19T22:28:19-03:00. O estado exato final e os comandos de validação são
registrados no commit da limpeza, não em recibos ou cadeias autorreferentes.
