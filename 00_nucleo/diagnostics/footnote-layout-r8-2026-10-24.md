# R8 — geometria e layout das notas de rodapé — 2026-10-24

**Concluído nos dez casos originais e controles adicionais certificados, com limites remanescentes explícitos.**
Os contratos públicos foram aprovados antes da materialização. RED→GREEN,
validação completa, replays A–H/R1–R7 e recursos passaram. Não é uma declaração
de paridade universal de notas ou sobrescritos.

## Referência, estado e recibos

Oráculo: commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, árvore
`cac836c07e4dec2a89219370428292393a3216b5`, executável SHA-256
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.
A versão textual não foi usada como identidade.

[Evidência R8](../../lab/results/construction/diagnostics/footnote-layout-r8-2026-10-24/):
`baseline.json` e `received-source.json.gz` preservam HEAD
`cf21f47a3b63f0e9fc5308925e1d0b67925e7227`, árvore de trabalho não commitada,
UTC, lista exata `git diff HEAD --stat`, status, 2162 hashes de entradas e binários
recebidos. Cada medição posterior registra sua própria fonte, UTC e executáveis.

O [recibo de fechamento](../../lab/results/construction/diagnostics/footnote-layout-r8-2026-10-24/closing-audit.json)
confere todos os gates e resultados. A fonte release está em
`revalidation/final-build/source.json.gz`; o recibo de build contém receita,
Rust, entradas e SHA do executável. Binários finais:

- Release: `9f40d753bb9ef8a05d34242f938a94885515639f193797f3de34a64f661dc7bc`.
- Debug: `62459254119b46ab950216df21f2cbb691b612aeb01d4997117e71df4dbdcd48`.

## Medição antes da decisão e correções

`red-geometry.json` reproduz dez falhas reais, com ambos os compiladores saindo
com sucesso: note, note-successor, two, custom, literal, columns, page-columns,
nested, deferred e split. Faltavam INNER na nota aninhada e END na continuação;
chamada/corpo divergiam de página. O contrato original `contract.py` verifica
páginas físicas, palavras e todos os boxes a 0,001 pt, conservando a extração bruta.

Tabelas reais em `font-tables.json`, fonte fixado `text/shift.rs` e
`model/footnote.rs:242–303,334–341` fundamentam as correções:

- Métricas de sobrescrito vêm de OS/2 da face de texto efetiva; `sups` exige
  cobertura completa e eixos corretos. A escolha resolvida acompanha fragmentos,
  medição, shaping e caches. Deslocamento horizontal não altera o avanço.
  O PDF conserva cinco casas quando o tamanho fracionário exige precisão.
- Entrada usa tamanho 0,85, leading 0,5 em, indentação/gap explícitos, separador e
  clearance. O gutter padrão considera a largura útil das colunas.
- Captura única preserva corpos, prefixos, filas aninhadas e escopos de colunas.
  O preflight da linha portadora aceita apenas conteúdo/regiões certificados;
  não repete callbacks nem dispatch de fonte.
- Cortes acompanham linhas e espaços certificados. Texto, decoração e alinhamentos
  seguem juntos; leading entre regiões é removido e a cauda não é drenada duas
  vezes na mesma região física. `Linebreak` preserva o certificado do commit real.

## L0 e contratos aprovados

`FontMetrics::text_superscript_metrics(&self, text, size, style) -> (Pt, Pt, Pt, bool)`
retorna tamanho, deslocamento horizontal, deslocamento de baseline para baixo e
ativação nativa. O método provido tem fallback `(size×0,6, 0, size×−0,5, false)`; o provider dinâmico
emprestado encaminha os overrides. Aprovação e drafts: `approval.json`,
`l0-proposal.json` e dono `core/compiler/layout/metrics.md`.

`TextStyle::superscript_native: Option<bool>`, default `None`, foi aprovado no draft
SHA-256 `4b14829d1eb38475d1985a9707a6e6ce40166196dfe524afe512d99b9cabf4d0`
(`native-carrier-approval.json`). `Some(false)` impede reativar `sups` num fragmento
sintético; `Some(true)` ainda verifica cobertura/eixos; `None` resolve automaticamente.
A identidade `superscript` permanece. Não foi criada propriedade de StyleDelta.
O bridge de StyleChain inicializa `None`.

Os 26 pares exclusivos L0/consumidor estão em `canonical-final7.json`. V15/V26 foram
validados antes do reseal; o linter aplicou os hashes forward canônicos e o gate
reverso verificou 1003 consumidores. Nenhum contrato público adicional foi criado.

## RED→GREEN e validação

Há RED de API ausente (`red-api.log`, `red-carrier-api.log`) e REDs comportamentais
de métricas, entrada, filas, continuação, colunas, Tf, forwarding, separador e cache.
`red-carrier-behavior.log` mede `123`/Nimbus com síntese solicitada ainda em 10,989 pt
contra 18,348 pt normal; o GREEN preserva a escolha através da fragmentação.

Os 22 testes R8 passam na suíte final. Os 28 controles PDF e 22 SVG passam em cada
build, debug e release: dez originais, doze numerações personalizadas e seis
combinações fonte/tamanho (estas seis verificadas em PDF). Maior delta PDF:
0,000016 pt; SVG: aproximadamente 10⁻⁹ pt. Tolerância permanece 0,001 pt.
Recibos: `final-debug-r8.json`, `final-release-r8.json` e seus outputs pareados.

`validation-complete7/` passou nos 12 gates, sem mudança nas 2162 entradas durante a
execução: 42 testes Python; estrutura; V1/V5/V15/V26; linhagem reversa; build;
testes de domínio/infra, wiring e doctests; todos os targets; Clippy com
`-D warnings -D unfulfilled_lint_expectations`; fmt; Rustdoc com `-D warnings`.
Rust: 9900 aprovados e 7 ignorados. `lint-close.log` e `reverse-close.log` repetem
os gates finais, sem erros/avisos. `git diff --check` passou.

Tentativas inválidas de fixture não contam como RED comportamental e permanecem
preservadas. `validation-complete3` revelou também o defeito de certificação de
Linebreak. A hipótese intermediária de igualar a linha com marcador à linha simples
da cauda foi refutada em `validation-complete4`; o teste exige a top-edge do provider
e a translação conjunta de texto/decoração. O aviso de nota indivisível continua
coberto; texto longo divisível tem teste separado de START/END uma vez.
Falhas de reseal e Clippy também foram conservadas: canonicalização, enumeração
explícita no coletor, referências das fórmulas, alias privado FootnoteRegion e
busca reversa no teste corrigiram os problemas sem supressões.

## Preservação e recursos

`revalidation/replays.json` tem 21 estágios aprovados; `supplement.json` tem cinco.
A auditoria `revalidation/audit.json` verifica A–H/R1–R5 com os gates anteriores,
substituindo apenas a exigência de geometria antiga de R1 pela paridade R8 com o
oráculo. Ambos os binários preservam 198 símbolos: 162 MATCH e 36 EXTENSION.
R2–R5 mantêm suas qualificações e limites anteriores, inclusive gradientes,
ordens mecânicas qualificadas separadamente e PNG nativo 11 pixels/max 1.

R6 passou com a receita release preservada: tamanho normalizado 42.724.288 bytes,
contra 52.271.120 do oráculo; 85,3938% da referência R5. Média geométrica RSS:
94,8958% de R5 nos sete workloads. Todos os gates de repetibilidade, preservação,
limites individuais e três controles históricos de estresse passaram. Isso não
promete velocidade universal. Receita executada com um job, teto 12 GiB,
MemoryHigh 8 GiB, swap 0; `final-run-envelope.json` registra o lançamento.

`preservation-audit-close.json` confirma exatamente 52 entradas alteradas em R8
(26 consumidores e 26 donos L0), 2110 outras entradas recebidas inalteradas e 19599
arquivos históricos preservados. Nenhum commit, reset, novo pin ou mudança da
receita de otimização foi realizado. Os relatórios congelados anteriores permanecem.

## Divergências adicionais remanescentes

`remaining-final-debug.json` repete 16 controles adicionais na fonte final: nove
passam; sete ficam explicitamente não qualificados. Seis são controles gerais de
sobrescrito: novas linhas literais nas três fontes, quebra explícita Nimbus e
fallbacks mistos Nimbus/Liberation. Espaços Nimbus passaram após o novo carrier.

O sétimo é uma nota de cinco linhas explícitas em página de 80 pt: corpo distribuído
1/2/2 no produto e 0/2/3 no vanilla, sem perda de palavras. Fonte/outputs estão em
`linebreak-final-debug/receipt.json`; SVG anterior também foi preservado.
Prevenção de órfãs/viúvas e dimensionamento das regiões futuras têm suporte em
`flow/collect.rs:196–231` e `flow/compose.rs:650–777` como hipóteses causais, sem
certificar uma explicação completa. A correção atual usa um certificado finito;
corpos opacos, estilos/regiões recusados e paginação arbitrária de notas não são
cobertos pelos dez casos originais. Esses limites não foram reclassificados como
paridade nem usados para afrouxar gates.
