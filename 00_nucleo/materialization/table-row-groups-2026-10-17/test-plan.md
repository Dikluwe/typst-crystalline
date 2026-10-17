# Plano de aceitação RG1 — testes antes do código

Estes são gates futuros; a coleta RED da implementação corrente não equivale a implementação aprovada. Cada teste novo tem de compilar e falhar funcionalmente antes da mudança.

## R1 — Transporte, tipos e casts

Headers/footers vazios, aliases, um argumento Sequence versus duas Cells, linhas no grupo e rejeição de Grid/nested group, origem causal; getters/has/at/fields/func/repr/JSON/TOML/YAML e máscaras.

## R2 — Colspan e rowspan

header/footers largura2, altura2, área2x2, holes reais, rows mínimas e nenhum padding inserido por quantidade de entradas.

## R3 — Ordem documental

Header após linha completa/parcial de Body, sequências de Header/Body, mesmos níveis, níveis ascendentes/descendentes e alias repetido; comparar células/papéis/efeitos.

## R4 — Coordenadas parciais

x-only,y-only,x+y em todos os grupos; grupo fixo sem Cell fully-auto não avança cursor do Body; primeiras origens livres e intervalos reservados.

## R5 — Faixas e conflitos

Header em y2 permite Body y0; Body y1 colide com Header rowspan2; grupos que tentam expandir por Body/outro grupo geram erro/span/hint, não sobreposição nem falso conflito.

## R6 — Footer final

Corpo lexical depois de Footer fixo pode caber antes; auto ou x/y após não é rejeitado por parsing. Footer antecipado/rows mínimas/Cellspans posteriores inválidos diagnosticados pela geometria final; footer duplicado com erro causal.

## R7 — Nível, repeat e papéis

Nível de grupo independente de kind Header/Data explícito, máscaras true/false e level1 omitido/assente, Source e clone/map/time/canonicalização preservados. Grupo vazio/line-only sintetiza uma linha. Raw Header.children.first()==Cell equivalente e identidade de func são verdadeiros, como medido em metadata.json.gz. Inferência de padding é verificada separadamente, não prova PDF/UA.

## R8 — Cursores e linhas

Linhas antes/depois das Cells, no topo/fim de grupos, y/x auto/explícito0 e após spans completos; invalid y/x/bottom/end border e end<start diagnosticados; linhas não ocupam células.

## R9 — Validação antes de Table show

Casos hidden-invalid-{cell-panic,empty,text,table-panic,header,footer,duplicates,footer-end} devem ter o erro estrutural pinado antes de callback Table. A Table crua não inserida continua sem síntese/effects.

## R10 — Ocultação e efeitos

hidden-valid-cell-panic continua sucesso; Table substituída não roda Cell/body, Cell substituída não roda body. Contar ordem/efeitos no programa real, incluindo alias/it/produto fresco.

## R11 — Show-set e ambiente

set-cell-hidden, showset-before-hide e showset-after-hide falham por colspan, não por panic; defaults de Cell capturados antes da síntese, Table children continua raw; snapshots local/context e restauração em erro.

## R12 — Plano uma vez

Transição pending→complete, is_complete, alias sem recomputar ocupação/receitas, sobrevivente com receita it, raw Child getters sem coordenadas/defaults assados, zero emissão/indexação de pending.

## R13 — HTML sem paginação

thead só prefixo, th em Header médio/fixo, tbody/td e tfoot com intervalos reais; linhas de rowspan cobertas existem sem origem falsa, tags/attrs/texto são comparados sem normalizar spans ou whitespace para esconder erros.

## R14 — Fonte, consultas e derivados

Maps retornando None reusam filhos, erros param irmãos, máscaras e Source conservados; introspecção/counter/outline/citações veem apenas visual completo, legenda/summary e coords corretos sem duplicação.

## R15 — Stroke/overhang SVG

Cinco casos sem texto em svg.json.gz; None remove top e azul anteriores, parcial deixa só intervalo remanescente, overhang/junções corretos. Comparar geometria/visuais, não bytes ou contagem de paths entre engines.

## R16 — Render/exportação

Mesmos casos em PDF/SVG/PNG, caixas/tracks/traços/texto/estilos, nested tables/Source/style capture, tiling/FrameStroke preservados. Nenhuma promessa de PDF/UA ou repetição multipágina por teste de uma página.

## R17 — Fallback cru e Grid

Callers de layout cru e fixtures Table usam o mesmo motor sem Engine/Func; controles GridExplicitFirst, índices/auto e geometria legada não mudam inadvertidamente. Erros de setup/compilação não contam como RED.

## R18 — Futuros e lint

Matches de novos enums fechados exigem decisão ao crescer; nenhum novo V16/V20/Clippy/allow. V19 por grupos com mesma política e L0 específico, sem inferir cobertura por executar só um corpo. Sondas reais em Cargo target isolado.

## R19 — Gates completos

cargo build/workspace tests, Clippy all-targets -D warnings/-D unfulfilled_lint_expectations, Rustdoc -D warnings/quatro índices, fmt, V1/V5/V15/V26 e reversa, replay/SARIF completo/diff-check; registrar HEAD+working tree+diffstat+UTC/hash.

## R20 — Fronteiras abertas

Conservar refutadores de campos gerais Table/HTML Grid/rich properties/paginação como pendentes; não alterar seus esperados para declarar paridade. Manter math-stretch-composite e as duas pendências matching separadas e o objetivo geral aberto.

