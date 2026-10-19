# D08B3 — medição e contrato proposto, 2026-10-07

## Estado e proveniência

Somente medição e preparação de contrato neste turno; D08B3 continua aberto.
HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, **working tree não commitado**. A lista exata de alterações
anteriores e `git diff HEAD --stat` constam de `table-cell-realizacao-d08b3-2026-10-07/baseline-state.json.gz`
e de cada coleta; não usar apenas HEAD para reproduzir este estado.
UTC de cada coleta, arquivos/fontes, cwd, argv, stdout/stderr/status estão nos JSONs.
Ambiente herdado da sessão sem overrides; o ambiente completo não foi congelado.
As fontes produtivas e os L0s canônicos não mudaram durante esta etapa:
`package-audit.json` compara os hashes e o inventário com o snapshot inicial.

Alvo vanilla **9f2b6e8715237cb086899a42873660fe744622e8**, não main móvel.
Oracle `lab/typst-original/target/release/typst` SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`;
árvore/receita ratificadas no diagnóstico P1489.
Tekt `target/release/typst` SHA-256
`7c0957f45fe40290aa809e3358c9a87979aa1e28058511ad0ee978feea6713a2`.
A coleta principal usa cópias congeladas dos binários; refutadores adicionais
usam os binários originais com esses mesmos hashes. Não houve rebuild.

| Coleta | Casos | Início UTC | Fim UTC |
|---|---:|---|---|
| before.json.gz | 62 | 2026-10-07T10:11:21.313491+00:00 | 2026-10-07T10:11:45.089725+00:00 |
| additional.json.gz | 6 | 2026-10-07T10:16:40.940822+00:00 | 2026-10-07T10:16:42.887933+00:00 |
| order.json.gz | 5 | 2026-10-07T10:23:11.243430+00:00 | 2026-10-07T10:23:12.531204+00:00 |
| aliases.json.gz | 3 | 2026-10-07T10:25:04.238709+00:00 | 2026-10-07T10:25:05.295041+00:00 |

As coletas são controles diagnósticos distintos, não uma taxa de paridade global.
O refutador raw-cell-equality teve fixture corrigida e rerun UTC registrado no
próprio caso; usar essa execução. HTML completo foi recapturado com os mesmos
binários/fonte; o horário específico dessa recaptura não foi registrado, portanto
não usar diferenças de tempo como evidência. Nenhum número fecha D08B3.

## Medições anteriores à decisão

- `before/order-auto-then-fixed` rejeita colisão no vanilla; a ordem inversa passa.
  Tekt `01_core/src/compiler/layout/grid_placement.rs:91–112` prioriza explícitas
  e `:262–270` só extrai GridCell. Vanilla
  `lab/typst-original/crates/typst-library/src/layout/grid/resolve.rs:1500–1530`
  verifica conflitos na ordem documental. É semântica de coordenadas/diagnósticos.
- `before/position-gap`, `row-padding`, `zero-children`, `additional/rows-empty`
  mostram vazios em holes/rows explícitas, mas nenhuma célula quando rows e
  children são omitidos. Fixup vanilla `resolve.rs:1665–1703` confirma a fonte.
- `before/cell-overrides`, `stroke-cell-partial`, `em-inset`, `fraction-rows`
  observam None explícito, fold de stroke, unidades lexicais e breakable.
  Vanilla `resolve.rs:204–278` assenta campos e `:1945–1988` deriva breakable.
- `order/table-replacement-before-cells` não executa Cell quando Table é
  substituída; `cell-replacement-before-body` não executa Heading ocultado.
  Tekt `01_core/src/compiler/realize/mod.rs:160–200` percorre bottom-up;
  `01_core/src/compiler/eval/rules/show.rs:322–331` rejeita a função table.
  O pacote deve adicionar mapping por identidade real da função, conservando alias.
- `aliases` observa LOCAL a, OUTER LOCAL a e NEW a. **Inferência:** captura no
  ponto efetivo de inserção/join, não no let/constructor. Refutada se a futura
  execução assar OLD, perder escopo local ou duplicar efeito numa contribuição.
- `additional/raw-origin-hidden` omite heading ocultado e `generated-heading`
  inclui o heading emitido. `reused-prepared-table` observa efeito por inserção.
  `html-table` também vê coordenada lógica. Não inferir equivalência global de
  render/cache/paginação a partir desses controles.
- `order/resolved-equality` diferencia raw de preparado; raw-cell-equality
  preserva igualdade entre células cruas. Source spans/estágios mecânicos não
  legitimam igualdade da linguagem; as propriedades conhecidas são observáveis.

Não foi inferida intenção do vanilla nem copiada sua estrutura Rust. Aceitação
é semântica, sintaxe e morfologia; texto extraído de PDF sem apagar whitespace
é indício de render, não prova universal de geometria.

## Decisão proposta e gate

Pacote `00_nucleo/materialization/table-cell-realizacao-d08b3-2026-10-07.md`:
Table show → resolver malha/propriedades → Cell show → corpo emitido.
Geometria permanece no layout. Único algoritmo de ocupação continua no owner
grid_placement, com políticas fechadas Table documental/Grid legada.

Contrato concreto: cinco campos públicos, dois carriers públicos e uma fachada
pública. Candidatos L0 completos para 43 owners exclusivos, um novo helper
privado, e núcleo pinado. Manifesto SHA-256 `5fec42835e38e9a28ee2ea2714b30135f069f111c140bb5e918a0902a06cc82b`.
`package-audit.json` confirma bases/candidatos/pins/ownership/evidências e ausência
de mudanças produtivas/L0 canônico; `verify-package.py` reproduz a conferência.
Não houve compilação nem suítes cargo neste turno de documentação/medição.

**Pendente ADR-0127**, por mudança pública e de fase; nenhuma aprovação anterior
D08B2 ratifica este contrato. Após confirmação: instalar L0/núcleo, testes RED,
materializar, resselo restrito e suítes/build/lint/coletas de regressão.

D08B3-A busca os negativos primários e refutadores da tabela. D08B3-B completa
Cell isolada e decisão específica de Cell fresca recursiva. Fields/defaults
genéricos de Table, styles ricos, casts extras B2, spans multipágina e PDF/UA
continuam abertos. Não declarar paridade universal nem fechar antes dos testes.
