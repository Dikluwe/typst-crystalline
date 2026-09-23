# Passo 1481 — PDF/UA-1: idioma, estrutura marcada e conformidade

**Estado:** materializado em 2026-09-23 após confirmação humana dos
contratos L0; validação bilateral e gates concluídos. Commit registado
no histórico Git deste passo.

**Baseline:** auditoria limpa de 2026-09-22 no commit
`90aad760ba472cffb70063baa684c461a240e447`, contra vanilla
`586e1bd43fae6c9a973218163d3165c53ab8d16d`. Hashes dos binários e
fixture integral em `typst-passo-1480.md` §1.

## 1. Medição anterior à decisão

Na fixture P1480, `veraPDF 1.30.2 -f ua1` reportou seis regras falhadas no
cristalino e nenhuma no vanilla. Excluídas as duas de P1480, restam quatro
grupos observados: idioma do texto da página (§7.2, test 34), idioma do
outline (§7.2, test 2), idioma do título nos metadados (§7.2, test 33) e
conteúdo da página nem marcado nem declarado artifact (§7.1, test 3).
Os 12 checks falhados totais não são 12 bugs independentes. Repetir a
medição **após P1480** e usar o delta real, não presumir que os quatro
grupos ficarão invariáveis.

Fonte cristalina: `03_infra/src/export/builder.rs:3061` declara que o caminho
atual não assegura PDF/UA; `builder.rs:3509-3559` já emite
`StructTreeRoot`/`MarkInfo` parciais. O L0 `infra/export/builder.md`
§P1140.6/§P1469 distingue tagging básico de PDF/UA. Medir no vanilla
pinado a associação efetiva de idioma e conteúdo às páginas, nós da
árvore, outlines e metadados antes de escolher o ponto de emissão.

Classificação: leitura acessível de texto, idioma e árvore são semântica
do documento exportado. MCIDs, IDs e ordem dos objetos são mecânica, salvo
quando a relação estrutural que transportam é inválida. Inferência: as
quatro regras podem ser fechadas sem mudar eval/layout; uma necessidade de
mover determinação de idioma entre fases refuta essa inferência e exige
gate ADR-0127 específico.

## 2. Contrato de correção

1. Todo texto visível na fixture fica associado a uma estrutura semântica
   válida ou é declarado artifact quando sua função é apenas decorativa.
   Não mascarar texto real como artifact para satisfazer o validador.
2. Idioma definido por `#set text(lang: "en")` deve chegar às unidades
   acessíveis correspondentes. Medir controles com mudança local de idioma,
   sem idioma declarado, heading aninhado e outline gerado. Não impor `en`
   como default artificial.
3. Título/outline precisam expor o idioma que o vanilla permite deduzir;
   não confundir o idioma do documento com o de um trecho local.
4. Manter o tagging opcional dos PDFs comuns e os gates existentes de
   `tagged`, `--pages`, `a11y-extras` e PDF/A. Não alterar seleção de páginas
   nem o pipeline por conveniência.

## 3. L0, RED→GREEN e fechamento

Auditar os owners `infra/export/builder.md`, `infra/export/stream.md`,
`infra/export/mod.md` e o L0 da estrutura semântica de L1 efetivamente
tocada. Atualizar primeiro somente os L0 causais, sempre em cardinalidade
1:1. Se for necessário campo/assinatura públicos, novo default ou troca de
fase, **parar após L0 e hash para confirmação humana** (ADR-0127).

RED: quatro famílias de regras observadas, mais controles negativos para
texto sem idioma e decoração. GREEN não é apenas zero checks na fixture:

- a fixture P1480 passa integralmente em `veraPDF -f ua1` nos dois binários;
- um corpus adicional com heading multinível, 2 páginas, texto com idiomas
  distintos, ênfase, link, imagem com/sem descrição e artifact decorativo
  é comparado bilateralmente em semântica e validado; falhas do próprio
  vanilla são distinguidas de regressões cristalinas;
- PDF/A sem anexo e com anexo, PDF comum tagged/untagged, HTML/SVG/PNG e
  combinações permitidas de standards não regridem.

Se o corpus novo revelar requisitos adicionais, o passo permanece aberto;
não declarar “PDF/UA completo” a partir de uma única fixture. Executar
`cargo build`, testes relevantes, `crystalline-lint .`, V5/V15/V26. Registrar
commit/tree ou diff exato, hora quando pertinente, binários, versão/comando
do veraPDF e resultados por regra/check. Não usar igualdade de bytes PDF.

## 4. Execução iniciada: medição, L0 e gate ADR-0127 (2026-09-23)

**Proveniência:** HEAD `e18fd39c4e32e417192f83a6a67dfcc26227af18`,
working tree não commitada com P1480 já materializado nos arquivos listados
em `typst-passo-1480.md` §4. Medição repetida em
`2026-09-23T14:03:05-03:00`. Binário cristalino release SHA-256
`cdbd7da3465d846eab862c5c1299e8e390d480487d7a488cebf5b2ec19f469a4`;
vanilla pinado `/usr/local/bin/typst` SHA-256
`eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`.

`veraPDF 1.30.2 -f ua1` na fixture P1480 repetiu cristalino
102 regras aprovadas/4 falhadas e 172 checks aprovados/10 falhados;
vanilla 106/0 e 262/0. As falhas cristalinas continuam §7.2 testes
2/33/34 e §7.1 teste 3. No QDF vanilla, catálogo `/Lang (en)`,
H1/P/Strong, spans com MCID e idioma local são observáveis; o
`StructTreeRoot` cristalino tem `/Document /K []` nessa fixture.

Controles adicionais do vanilla: `p1481-no-lang.typ` gera `/Lang (en)`;
`p1481-global-fr.typ` e `p1481-local-first.typ` geram `/Lang (fr)`.
`p1481-no-text-fr.typ` (SHA-256
`8e1b27cf49c91d2edbe132aa69f893bb554bca7141121fe87a544ed219455b38`)
gera `/Lang (fr)` sem texto: a língua documental não pode ser inferida
dos glifos. O corpus de 2 páginas `p1481-corpus.typ` (SHA-256
`90475d9f6bd2b6964495861203c480af064b7273c14dc0feb492f79444dbd7d4`)
com H1/H2, idiomas locais, Strong/Em, link, imagem descrita e artifact
passa `veraPDF -f ua1` no vanilla com 106/0 regras e 623/0 checks.
O cristalino rejeita `image(alt:)` na avaliação. Sem `alt` (SHA-256
`7d3b364ffe1e979d127cf303a2f13b55ad969d7d844aea831c549d6f058ae504`),
o vanilla recusa compilar: `PDF/UA-1 error: missing alt text`, hint
`make sure your images and equations have alt text`.

**Gate:** para preservar esses observáveis, os L0 abaixo propõem campos
públicos em `PagedDocument`, `TextStyle`, `ImageElem` e `FrameItem::Image`,
além de um novo diagnóstico sob `ua-1`. O passo §3 e ADR-0127 exigem
paragem após L0. Nenhum código P1481 foi escrito. A aprovação anterior
de P1480 cobre apenas o título obrigatório e não cobre estes contratos.

Prompts L0 P1481 propostos e SHA-256 completos (working tree não commitada):

- `entities/layout_types.md` — `460b925415bed13a8bb23b23e229acdadd8080a8dc08f626e02b8e365c362e3f`;
- `entities/elements/image.md` — `0ad198933d394868d47658c77bd196c2d17be14d9b04d00ff21fff62c2afdc11`;
- `compiler/stdlib/figure_image.md` — `9d0ef1889c9c65a85c485d9ec08a21809b80fd6c3d7aa8f24aa2c740126d028b`;
- `compiler/layout-image.md` — `9136621402161443da80d37f4bb52531faf3ebd5a1cb10683e5374bf31fe83f2`;
- `compiler/layout.md` — `cfcbc530ade1db3a80fd28cda790265065198e7d6fc2787dfde9f161157e6fee`;
- `entities/style_chain.md` — `6b9451170150de64ea864d6055b0f918e5bf3abefe7af22c6dcfc0939e823eba`;
- `infra/export/builder.md` — `76a63bdef4984709c222f251aebb3baa0c23bc138b5a477592b33ea9bb2fe2e2`;
- `infra/export/stream.md` — `c84453541d355f4c168b41ca44769ef36945ac8a2bef506d6c50ef6f6be4624f`;
- `infra/pipeline.md` — `95ee9eb5b4a3da8ce594a1a0852d0e0a90d1964c1129e7b577ab17e2bb2aaf86`;
- `entities/content.md` — `7f772e1298f2f92739c72f1ff0d1157556fff1dea5fcde09026737037d4ca961`;
- `compiler/layout/text.md` — `9e01c3c13b41be797616ca4a3d18bab88f9b9ce3187befff9e54029f3d68059f`.

Após confirmação humana destes contratos, a continuação obrigatória é
testes RED, implementação, resselo dos hashes de linhagem, corpus
bilateral completo, controles de não-regressão e gates do §3. Até lá,
V5 detectará drift esperado porque os L0 propostos precedem o código.
## 5. Fechamento após confirmação humana (2026-09-23)

O dono confirmou os contratos públicos L0 e autorizou o commit. Os testes
P1481 foram escritos antes da implementação e observados em RED; depois,
passaram em GREEN. Os hashes de linhagem foram ressellados com
`crystalline-lint --fix-hashes .` (zero drift). A implementação transporta
idioma efetivo e local, ênfase semântica e `image(alt:)` até a árvore PDF/UA-1.
Texto e imagem reais recebem MCIDs e nós semânticos; decoração vira artifact.
A árvore agrupa fragmentos por bloco e inclui links, figuras, Alt, OBJR e
ParentTree. Imagem real sem `alt` produz o diagnóstico do vanilla; artifact
é isento. PDF comum permanece fora do gate UA1.

**Proveniência:** medição final em `2026-09-23T15:30:27-03:00`, HEAD
`e18fd39c4e32e417192f83a6a67dfcc26227af18`, working tree não
commitada. A lista exata dos arquivos alterados está em `git show --stat`
do commit deste relatório; P1482–P1488 não pertencem a ele. Binário
cristalino release SHA-256
`ba164438e499d0ae1980993bdfc751539f43bbedf029875a102495a508ef3442`;
vanilla pinado SHA-256
`eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`.
Corpus de duas páginas SHA-256
`90475d9f6bd2b6964495861203c480af064b7273c14dc0feb492f79444dbd7d4`.

`veraPDF 1.30.2 -f ua1` no corpus: cristalino 106 regras aprovadas/0
falhadas e 772 checks aprovados/0 falhados; vanilla 106/0 e 623/0.
As contagens de checks diferem por mecânica de emissão. O QDF de ambos
contém H1=1, H2=1, P=2, Strong=1, Em=1, Link=1, Figure=1. A fixture
P1480 passou no cristalino com 106/0 regras e 456/0 checks. Controles
sem idioma, francês global, idioma local e página sem texto passaram em
UA1 (75, 85, 120 e 28 checks, zero falhas). O corpus PDF/A-2u+UA1 passou
`veraPDF -f 2u` com 146/0 regras e 2645/0 checks; `-f ua1` com
106/0 e 772/0.

`cargo build` passou; testes P1480–P1481 13/13, série P147 37/37,
P1248 9/9, `native_image` 7/7, SVG 7/7, PNG 7/7, HTML 4/4
(`TMPDIR=/dev/shm`), PDF tagged pretty 10/10 e PDF standard 10/10.
`crystalline-lint .` terminou com exit 0 e V5/V15/V26 sem violações.
O conjunto integral `cargo test -p typst-infra --lib` foi interrompido
após mais de seis minutos em teste raster P1431 não relacionado; não é
apresentado como aprovado.
