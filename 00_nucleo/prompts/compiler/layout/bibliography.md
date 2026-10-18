# L0 — bibliography
Hash do Código: 3914a705

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/bibliography-plan.toml sha256:cec5b93a8888658ac90461deea96799dfd56393be3d63c874e6cf93f4d57032c
- 00_nucleo/prompts/_nuclei/layout/bibliography-prefix-admission.toml sha256:64d61513bab0293f918958404cf4fe0387dea84cc53f5442faad2b467fa4e077
- 00_nucleo/prompts/_nuclei/layout/paragraph-hanging-indent.toml sha256:3a97c056eff81b344f46d83567274225e3519ae141fce4817e342d573a92fb41

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/bibliography.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/bibliography.rs:25`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Render an optional title and close its line. When the precomputed CSL cache supplies bibliography Content, dispatch it directly. Otherwise stably order entries by first citation, leave uncited entries in original order, add sequential `[N]` prefixes and the existing back-reference arrows. Own the `format_bib_entry_body` compatibility helper and its conditional field composition. The hub delegates fallback formatting here; preserve the existing field order and organization-as-publisher rule. This fallback is a local compatibility renderer, not evidence of complete vanilla CSL parity. Citation rendering belongs only to `00_nucleo/prompts/compiler/layout/cite.md`; cache construction belongs to `00_nucleo/prompts/compiler/layout/bib_csl.md`.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

## A2 — fronteira de bloco e validação de hanging-indent (2026-10-18)

### Medição anterior à decisão

00_nucleo/diagnosticos/bibliography-indent-2026-10-18/before.json.gz contém
60 pares de sondas no HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c,
working tree não commitado/UTC/status/diff HEAD --stat, fontes/argv/binários,
texto e caixas. before-sources.json.gz preserva fontes/L0. Oracle pinado
bd5ebec285505889e8ff1d4f68804ab147a699f3, árvore
cac836c07e4dec2a89219370428292393a3216b5, executável SHA-256
9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9;
Tekt baseline 97f6fa3078bdd6c40aabb80d6f93f3fd019b405ce70fbf2266686d8fb42271cb.

MLA sem título começa na linha da citação no Tekt (Silva x41.84/y5.168)
contra x8/y27.463995 no vanilla. Com título, o Tekt mistura título e citação
(y6.7568/y8.108) e intercala a extração no CSL customizado; o vanilla os
separa. Oito positivos de par medem recuo não implementado. Dez negativos
none/auto/int/ratio/string de set e constructor são aceitos no Tekt, mas o
oracle rejeita expected length. bibliography.rs:106–123 despacha o título sem
fechar a linha anterior e o cache como fluxo inline. set.rs:1180–1192 trata
hanging-indent como desconhecido; structural/par.rs:173 ignora o valor.

No pin, typst-layout/src/rules.rs:463–531 realiza título e blocos de entradas;
sem prefixo, o recuo é inset mais HElem compensador; prefixos usam duas colunas.
model/par.rs:431 declara Length e inline/collect.rs:146–148 conserva avanço
inicial absoluto. São fatos de linguagem/posição/erro; não obrigação de copiar
seu grafo Rust, bytes ou algoritmo de armazenamento. Hipótese causal desta
subetapa: ausência de fronteira de bloco mistura título/citação/corpo, e falta
de validação admite tipos inválidos. Medições de posições, tipos/hints/spans e
os controles restantes podem refutá-la; não concluir que um offset único resolve
recuo ou que toda diferença de extração PDF é só mecânica.

### Classe e divisão explícita

Correção interna de paridade em fluxo contínuo ADR-0127: sem campo/trait ou
assinatura pública nova, default/flag adicional, compatibilidade Rust ou fase
nova. O L0 precede testes/código. Esta revisão é incompleta para A2:
A2-Structure deve preservar hanging_indent/prefix/body do CSL e compor entradas
com suas margens/colunas; A2-HangingTransport deve transportar overrides por
ocorrência e resolver prioridades; A2-ParIndent deve completar composição de
parágrafos comuns (set/constructor), inclusive estratégias e direções. A2-Multi
continua atribuição/cache por bibliografia. Não silenciar warnings de suporte
incompleto antes de materializar seus consumidores, nem considerar positivos
hanging-indent corrigidos pela rejeição dos negativos.

### Responsabilidade exclusiva

Antes de emitir título ou corpo não vazio, fechar o segmento anterior pelo helper paragraph::close_in_chain, usando text_segment_chain quando disponível e a chain atual como fallback. Isso conserva o escopo causal da citação anterior mesmo quando o show da bibliografia muda a fonte; o helper continua governado por 00_nucleo/prompts/compiler/layout/paragraph.md e delega a parbreak. Despachar o corpo bibliográfico dentro de um Block breakable de largura/altura automáticas e inset zero, delegando toda geometria/margens ao owner 00_nucleo/prompts/compiler/layout/block.md. O corpo cacheado continua a ser o Content fornecido pelo cache; não mudar sua API ou morfologia pública. O fallback monta as mesmas linhas, ordem/campos/arrows e quebras anteriores, e usa a mesma fronteira de corpo. Não emitir Block para corpo vazio; título continua sua ocorrência existente, sem tags falsas, nova realização ou interpretação CSL. Não inventar constante vertical para ajustar um caso. Os recuos/per-entry/prefixos ainda requerem A2-Structure/A2-HangingTransport; global block defaults e controles de paginação não são presumidos equivalentes pelo template.

### Aceitação

Testes RED funcionais antes da implementação; GREEN de fronteira título/citação/corpo/fallback e negativos com controles de zero/negativo/affine e vazio. Medir novamente caixas PDF/extração, manter os REDs de recuo e múltiplas bibliografias explícitos, não atribuir vantagem ao Tekt sem fonte/controle. Revalidar build/workspace/doctests, Clippy/Rustdoc estritos, fmt, linhagem V1/V5/V15/V26 e reversa, SARIF sem erro/warning novo. Teto 12 GiB/High8 GiB/swap0/jobs1/debug0/incremental0; comandos pesados sequenciais. Sem commit/push/release.

## A2-Structure — entradas CSL e colunas (2026-10-18)

### Medição anterior à decisão

00_nucleo/diagnosticos/bibliography-structure-2026-10-18/before.json.gz registra
12 pares, fontes/argv/UTC/HEAD/status/diff HEAD --stat, textos/caixas/binários;
before-sources.json.gz conserva as fontes e L0s anteriores. HEAD
b1b486f3d42431f8ea360bc5712637ee1e3d4f6c, árvore não commitada, Tekt SHA-256
c2b1b2e93cf34f3c597a0a0fbbd51c8f8ac96a3c6bc0a4d9bd72373599acf899;
oracle bd5ebec285505889e8ff1d4f68804ab147a699f3, árvore
cac836c07e4dec2a89219370428292393a3216b5, binário SHA-256
9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9.
IEEE12 põe A. em x25.124 contra29.924 e a segunda entrada em y89.936
contra112.232. IEEE18 usa x33.686 contra40.886. No CSL sem prefixo, a segunda
entrada começa em y184.424 contra191.024. As sondas com par.spacing0/2em
separam margens de espaçamento dentro das linhas. Há também diferença de
agrupamento de citações adjacentes (MLA Silva/Doe), fora da composição da lista;
manter o RED explícito, não chamar igualdade só da lista de paridade geral.

bib_csl.rs:342–355 concatena first_field, espaço literal, content e Linebreak,
perdendo a fronteira de cada entrada e o hanging_indent. convergence.rs:58–82
transporta só BibRenderCache público; bibliography.rs:107–151 emite um Block
único. O pin typst-layout/src/rules.rs:463–531 compõe duas colunas Auto com
gutter0.65em e row-gutter par.spacing, ou um Block por entrada sem prefixo;
o formato autoral do código Rust não é requisito. Observáveis são posição,
ordem, conteúdo/estilo e escopo. Hipótese: transportar fronteiras reais e
prefixos corrige a coluna e a separação sem reformatar; controles de fontes,
ordem, campos vazios, sem prefixo, zero/2em, show-grid e estilo podem refutá-la.

### Classe e escopo explícito

Correção interna/de paridade ADR-0127 em fluxo contínuo, dentro do layout,
sem API pública, tipo público novo, flag/default, compatibilidade ou fase nova.
L0 antes de testes/código. A2-Structure preserva hanging no discriminante do
plano, mas sua aplicação física, overrides por ocorrência e composição
simple/optimized/RTL continuam em A2-HangingTransport/A2-ParIndent. Não forçar
fallback greedy para contornar o suporte de avanço inicial ainda ausente da
preparação. Margens de título, CSL completos, atribuição A2-Multi e agrupamento
de citações adjacentes não ficam certificados por esta etapa.

### Refutador de paginação antes da revisão

pagination-before.json.gz / pagination-grid.json.gz e grid-freeze.json.gz no
mesmo diagnóstico preservam o candidato Grid e a contraprova: IEEE em página
200×90pt/margem8 exige duas páginas no oracle e no Tekt anterior, mas o
candidato usa três, iniciando toda a entrada na página2 e OUTSIDE na3.
Gutter/coluna e separação em página auto estavam corretos (A. x29.924 e
segunda entrada y112.232), mas não bastam. grid/row_pagination.rs:23–31
admite overflow de linha e move row inteira; não é a quebra de um parágrafo
longo. Isto refuta a composição Grid desta subetapa e exige trocá-la antes
de fechar, sem enfraquecer o teste. A linguagem pede coluna estável e entrada
quebrável; não exige usar o mesmo Grid Rust do vanilla (ADR-0107/0109).
Nova hipótese usa Block/Place existentes; testes de primeira região, conteúdo
completo, fontes, alinhamento e caixas podem refutá-la, inclusive RTL.

### Responsabilidade exclusiva

Quando há plano privado, fechar a entrada como já prescrito e despachar o título existente. Lista com prefixos mede somente os prefixos puros fornecidos pelo CSL pelo helper público existente measure_content_real de 00_nucleo/prompts/compiler/layout/measurement.md, sob a chain e métricas atuais, uma vez por ocorrência de lista, para obter a largura intrínseca máxima comum. Cada entrada usa seu próprio Block breakable, inset na borda inicial igual a max_prefix_width + 0.65em; corpo segue o fluxo normal quebrável. Prefixo é um Place técnico sem float, alinhado à borda inicial/top do referencial interno do Block com dx que cancela o inset (negativo à esquerda, positivo à direita), emitido antes do corpo e só na primeira região da entrada. Não criar Grid nem reexecutar show/CSL/callback. O owner de Block resolve as margens fracas par.spacing entre entradas e sua continuação; não inventar offset vertical. Os owners 00_nucleo/prompts/compiler/layout/place.md e 00_nucleo/prompts/compiler/layout/placement.md já definem ancoragem ao block_place_region interno. Containers são detalhes privados de render, sem ocorrências/tags/Location sintetizada. Sem prefixos, despachar cada body em seu próprio Block breakable, conservando divisão plain/hanging no plano (ambos usam Block nesta subetapa incompleta). Sem plano, conservar cache programático e fallback anteriores. Não modificar Content público nem usar props genéricas para transportar o plano. Não alterar estratégia de quebra para simular hanging. As normas novas substituem o Block único de A2-fronteiras somente quando o plano está disponível.

### Aceitação

RED funcional de coluna com gutter relativo e margem entre entradas antes de
implementar. Controles de cache público (Content anterior e assinaturas), CSL
sem prefixo, preservação de modo hanging/plain, vazio, ordenação/citações e
rollback. PDF bilateral com caixas e texto da lista, conservando divergências
de recuo e agrupamento de citações no relatório. Build/workspace/doctests,
Clippy e Rustdoc estritos, fmt, V1/V5/V15/V26 e reversa; SARIF sem erro/warning
novo. Resselo só após ownership/grafo válido. MemoryMax12GiB/High8GiB/swap0,
jobs1/debug0/incremental0, comandos pesados sequenciais; sem commit/push/release.

### Controle de entrada somente com prefixo

Place é fora do fluxo (placement.rs:432) e Block usa fim causal do corpo. Uma entrada CSL pode ter prefixo e corpo vazio; exigir que seu prefixo continue ocupando uma linha e que a próxima entrada não se sobreponha. Esse controle de geometria precede qualquer correção especial de corpo vazio, sem tratar prefixo ausente como prefixo vazio.

## A2 — prefixo admitido e região inline de Block (2026-10-18)

### Medição anterior à decisão — admissão e continuação

pagination-block.json.gz e block-freeze.json.gz em
00_nucleo/diagnosticos/bibliography-structure-2026-10-18 preservam estado completo,
UTC/HEAD/fontes/binários/argv e caixas do candidato Block/Place c1c222ae54ab3b67487471326012a7f6c4ed86c1e6bd61b1830ec6a987c2ccea.
Em IEEE/200×90pt ele recupera duas páginas, mas a continuação começa em x8
contra29.924 no oracle bd5ebec285505889e8ff1d4f68804ab147a699f3 (binário
9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9).
region_progress.rs:75–76 reinstala a margem física ao transitar, descartando o
inset ativo. O teste corrigido para70pt também mostrou prefixo na primeira
página e corpo admitido na segunda: paragraph_pagination.rs:87–105 pode adiar
o corpo para respeitar widow/orphan depois que o Place já foi emitido.

A exigência inicial de FIRSTWORD sempre na primeira página da fixture FixedMetrics
não é o contrato correto: group_size em paragraph_pagination.rs:69 pode manter
três linhas juntas. Substituir esse critério pelo prefixo acompanhar a página
real da primeira linha; preservar o RED PDF90/duas páginas e a distinção entre
coluna estável e estratégia de quebra. Não forçar greedy nem desabilitar
widow/orphan para satisfazer um assert. Posições geométricas, conteúdo e agrupamento
são observáveis da língua; slots/representação são mecânica privada.

Correção interna ADR-0127 dentro do layout, sem assinatura/campo/trait público,
flag/default novo, compatibilidade ou execução eval/show. A primeira tinta do
prefixo continua antes da primeira tinta do corpo, após a admissão física do
parágrafo. Não é nova ordem de realização de show nem transferência de fase.
A hipótese é refutável por prefixo órfão, recuo perdido, emissão dupla, efeitos
novos, estratégia alterada ou regressão dos controles de Block/columns/subframe.

### Responsabilidade exclusiva

Trocar a emissão antecipada de Place por PendingPrefix privado com Content, chain, TextStyle, indent e direção. Instalar ao redor do Block do body e restaurar o contexto externo ao terminar a entrada. Corpo vazio com prefixo usa Block textual comum, sem deixar estado pendente para a próxima entrada. O helper privado emit_pending_prefix toma o contexto antes de renderizar (evita recursão), ancora um Place técnico à baseline da folha real e ao line_start_x/largura corrente, suspende temporariamente chain/style/block_place_region do corpo e os restaura. A página/coluna é a admitida, sem capturar Location ou deduzir posição de query por tinta. Prefixo continua CSL puro. Preparação mede só prefixos e não reexecuta o body. Preservar custos e backends de quebra. A sintaxe anterior emitido antes do corpo significa antes da tinta, não antes da admissão.

Aceitação: RED→GREEN de prefixo/corpo na mesma página e inset nas continuações, inclusive saída para OUTSIDE, simple/optimized e controles existentes. Medir PDF90/150/250, RTL, prefix-only/formatado, cores e regressões; build, suites/doctests/Clippy/Rustdoc/fmt/linhagem/reversa/diferencial Tekt. MemoryMax12GiB/High8GiB/swap0/jobs1; sem publicação. Hanging genérico/overrides e múltiplas bibliografias continuam frentes abertas.

### V17/V19 nos controles e no modo incompleto — 2026-10-18

A varredura pós9005 testes (lint-before-fixture-fix.json.gz no diagnóstico do
lote) encontrou um V17 no guard OR do teste prefix-only e uma nota V19 no ramo
Hanging|Plain. A seleção dos prefixos da fixture usa o predicado nominal
starts_with('['), com as duas identidades [1]/[2] verificadas explicitamente;
não usar guard lógico composto nem supressão. O agrupamento Hanging|Plain é
intencional e temporário: ambos despacham Blocks por entrada nesta subetapa,
como declarado, mas hanging físico permanece pendente. O corpus contém MLA
(hanging) e CSL custom (plain) separadamente. Conservar a nota V19 informativa,
sem converter execução de um modo em prova do outro ou encerrar hanging.

## Recuo francês — aplicação física (2026-10-18)

### Medição anterior à decisão

00_nucleo/diagnosticos/bibliography-hanging-2026-10-18/before-sources.json.gz
preserva HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c, árvore não commitada,
status/diffstat/UTC e2099 fontes/L0s. before.json.gz contém92 pares;
extra-before.json.gz contém26; boundary-before.json.gz contém10 e
repeat-before.json.gz contém4. Oracle bd5ebec285505889e8ff1d4f68804ab147a699f3,
binário9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9;
Tekt inicial2cc69a555e9738dc659c06ed6d933853ab981cbb73e93c388a5b43c55afad095.
Em160pt/margem8/font12, hanging10 mantém a primeira linha x8 mas move
continuações x8→18; negativo-5 exige x3. RTL mantém primeira borda direita152
mas continuações vão para142. O Tekt ignora o comprimento. MLA tem default
18pt (1.5em), set ambiente3em não o troca, show-set3em exige36pt; IEEE/Plain
não adquirem recuo de hanging por esse show-set. bibliography.where(style:"mla")
não casa na sonda vanilla: conservar a decisão do seletor, não forçar override.

Block/Box com body apenas inline ignoram hanging10; Par explícito dentro
deles e Block com Parbreak aplicam. Heading/Raw/List/Quote simples não o
aplicam. Declaração parcial, zero, repetida e escopo vazio fecham fronteiras:
FIRST/MIDDLE/LAST em5.168/27.464/49.76, contra todos5.168 no Tekt. A primeira
sonda lexical usou colchetes literais: foi refutada, corrigida para #[...] e
preservada como extra-before-initial; não é evidência de escopo Styled.
Valores extremos geram tinta recortada pelo extrator: não equiparar ausência
em pdftotext a apagamento de Content. Fonte parcial24pt não muda o em do
comprimento compartilhado12pt, como partial-font registra.

Fontes pinadas: typst-realize/src/lib.rs:1061–1085,1190–1219 distinguem
fragmento/Par e interrupção; typst-layout/src/flow/collect.rs:72–134 escolhe
fluxo, inline/mod.rs:174,216–221 e collect.rs:141–147 resolvem forma;
inline/line.rs:496–503 posiciona; rules.rs:510–523 faz inset/compensação CSL.
typst-library/src/model/bibliography.rs:112–127,367 determina o default e
precedência. Isso mede língua/geometria; não exige a estrutura Rust vanilla
nem atribui intenção histórica. Refutação por fronteira perdida, em de run,
recuo duplo, reset em página, fallback greedy imposto, cópia perdida, callbacks
reexecutados ou regressões dos controles impede fechamento.

### Classificação e limites

Correção interna/de paridade ADR-0127, fluxo contínuo: sem campo ou assinatura
pública, flag/default externo novo, fase ou ordem de realização diferente.
A nova forma inicial/continuação supersede somente a limitação correspondente
de largura constante; RTL latino certificado segue as condições do núcleo,
sem BidiCursor ativo/tracking e sem inventar BidiSpan. Os demais limites
anteriores continuam. Os valores/cópia fora da página requerem inspeção dos
frames ou SVG, não igualdade de extração recortada. Não antecipar R3-T1.

### Responsabilidade exclusiva deste owner

Separar Hanging de Plain. Hanging resolve h do override privado realmente marcado por show-set ou default1.5em, preserva Length até tamanho corrente, usa inset inicial h e fila privada de compensação -h na primeira linha do Block. Não usar HSpace/fake Par nem tags/Location novos. Shadow par.hanging-indent com zero somente no corpo técnico, para evitar dupla aplicação ambiente; Columns/Plain conservam geometria, cache público e fallback. Restaurar chain/fila/contexto após cada entrada e antes de OUTSIDE.

### Aceitação

RED→GREEN antes da implementação; controles fonte→layout e bilaterais dos sinais,
fronteiras, variantes e restauração pertinentes. Não deduzir cobertura de todas
as alternativas por executar um arm agrupado: classificações nominais fechadas
do contexto inline/fluxo são intencionais e exigem testes por classe/variante.
Resselo apenas após V15/V26 válidos; testes, build, Clippy/Rustdoc/fmt, reversa
e diferencial Tekt do pin1279707, sob12GiB/High8GiB/swap0/jobs1. Sem publicação.
