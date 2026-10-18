# L0 — space
Hash do Código: 0f9aeec2

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:7b8a61eaafc8a07df12218f8f0560c33fae5c36f621b822b7350ccefc9774711
- 00_nucleo/prompts/_nuclei/layout/paragraph-hanging-indent.toml sha256:3a97c056eff81b344f46d83567274225e3519ae141fce4817e342d573a92fb41
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:79d5c48d0b9d0fb7d995207d18b1f9437209bf35c78a35cbfcedccea8b90abd2
- 00_nucleo/prompts/_nuclei/layout/text-tracking.toml sha256:3b0c535436048536ecd3133fcf8299e88c128a62c222661f36b7bcc6d98a598c
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:3ee5760ffa44b5fd889b9d6772972e3b8417be332e13726cd9eea24b32e686f8
- 00_nucleo/prompts/_nuclei/paragraph-composition.toml sha256:6eec17984449d21b626379651b9b0a511a2dfe8e1a1f0a0af230cce52a47b827


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/space.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidência anterior à decisão

A inspeção de `01_core/src/compiler/layout/text_measure.rs:85`, `word_layout.rs:89` e `space.rs:162` encontrou respectivamente fronteira textual, admissão de baseline e teste de integração de espaços iniciais em owners de outras responsabilidades. O estado exato, fontes e prompts anteriores estão em `00_nucleo/diagnosticos/separacao-tracking-2026-09-29/initial.json` e `before/`. A correção já medida encontra-se em `00_nucleo/diagnosticos/correcao-tracking-2026-09-29.md`; esta separação conserva seus resultados e limites. Posições, texto e estilos são observáveis da língua; localização das funções é mecânica. A inspeção não demonstra intenção histórica nem nova paridade bilateral.

## Contrato vigente

Possuir layout e advance para Markup/Newline/Literal. Markup/Newline resolvem estilo efetivo pelo owner `00_nucleo/prompts/compiler/layout/text/style.md`; Literal reutiliza estilo já resolvido. Medir avanço nominal por text_measure::space.

layout suprime Markup/Newline iniciais ou depois de apenas marcadores HTML. advance emite carrier quando há material ou quando Literal tem tracking resolvido diferente de zero. Aplicar text_tracking::boundary ao cursor antes do carrier, uma vez. Emitir TextSeparator append-only com origem, estilo, posição e advance nominal; advance não inclui tracking nem deve ser somado novamente pelo exportador. Aplicar largura nominal ao cursor e registrar oportunidade de justificação.

A posição y inicial usa cursor_y mais offset do estilo, sem demandar baseline. text_baseline pertence ao alinhamento posterior quando chega texto material. Literal sem tracking conserva avanço sem carrier inicial. Não inferir espaços a partir de distâncias ou texto adjacente. Em layout, quando par.justify não é true, flush somente após cruzar limite direito; com justify true, a unidade material seguinte escolhe continuação. advance literal não introduz esse flush imediato.

## Aceitação e limites

Preservar os testes causais existentes e o texto, posições, estilos, páginas e diagnósticos dos controles anteriores à separação. Ownership 1:1, pins válidos, build e lint sem erros. Movimentação interna ADR-0109/0127: funções estáticas descendentes, sem estado novo. As limitações de tracking em estilos mistos, RTL, seleção global e contagem de clusters permanecem as registradas no diagnóstico da correção.

## Continuidade lexical do tracking — 2026-09-29

### Medição anterior à decisão

`00_nucleo/diagnosticos/correcao-tracking-escopos-2026-09-29/before-pdf/results.json` e `summary.json` registram 30 fontes/90 PDFs, comandos, fontes e estado exato em initial.json e before-pdf/provenance.json, contra vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8. Escopos iguais repetidos acumulam cerca de 0.4pt e aninhados 0.8pt; wrap positivo muda a linha. `text_tracking.rs:50` compara apenas TextStyle resolvido. Vanilla `inline/collect.rs:287-295` conserva segmentos de cadeias distintas; `inline/shaping.rs:1269-1295` soma tracking apenas dentro de cada run. Isto mede fronteiras textuais observáveis, não exige copiar PartialEq ou representação da chain. Hide/Box também são fronteiras: hide.rs:21-24 e boxed.rs:157-208 substituem/restauram buffers sem preservar essa distinção.

### Contrato interno da correção

Usar boundary do State antes do carrier e record depois de emitir TextSeparator. Espaço suprimido ou avanço inicial sem carrier não registra produtor. Conserva advance nominal, origem, estilo, baseline e justificação.

Correção interna de paridade ADR-0127 em fluxo contínuo. Aceitação RED→GREEN, controles bilaterais de texto/posições/linhas e preservação dos controles de tracking uniforme; build e lint sem erros, resselo somente destes owners. Demais limites abertos mantidos.

## Fronteiras entre unidades em RTL explícito — 2026-09-29

### Medição anterior à decisão

`00_nucleo/diagnosticos/correcao-tracking-rtl-2026-09-29/before-pdf/results.json`, `summary.json`, `provenance.json` e `initial.json` registram 66 entradas e 198 PDFs contra o vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8, com fontes, comandos, binários, HEAD e working tree completos. Em `dir:rtl`, três termos latinos deixam de receber quatro intervalos: com ±0.2pt o primeiro termo difere 0.8pt; 0pt coincide. Texto árabe explícito difere 0.4pt nos dois intervalos entre palavras para ±0.2pt. `text_tracking.rs:65` recusa indiscriminadamente RTL; `word_layout.rs:34,72` e `space.rs:25` já projetam/adicionam a fronteira assinada; `cursor.rs:150-186` ancora a linha pelo limite direito real. No vanilla `inline/shaping.rs:1269-1295`, tracking aplica-se antes de cada próximo cluster distinto também em direção negativa. Isso mede posições e escolha de linha, não a intenção histórica.

Hebraico (`dir:rtl`) e espaço literal inicial apresentam diferenças mesmo em 0pt; não são prova de defeito da fronteira. Ligaduras, mistura de direção e paginação global ainda exigem suas próprias sondas.

### Contrato da correção

Em RTL explícito, avançar para a próxima unidade textual pelo mesmo State::boundary e conservar advance do TextSeparator exclusivamente nominal. Origin/style/baseline/justificação e supressão de espaço inicial permanecem. Não somar outro intervalo em PDF ou inverter seu sinal pela direção.

Correção interna de paridade ADR-0127 em fluxo contínuo: sem contrato público, default, quebra de compatibilidade ou fase nova. A restrição histórica a LTR fica substituída somente por esta fatia medida. Aceitação RED→GREEN, controles bilaterais em ambas as direções/valores, preservação LTR, build e lint sem erros, resselo restrito.

## Geometria auto de separadores literais — 2026-09-29

### Medição anterior à decisão

`00_nucleo/diagnosticos/correcao-auto-espacos-2026-09-29/before/results.json`, `provenance.json` e `initial.json` medem 30 fontes/60 PDFs, font-path, comandos, UTC, HEAD, working tree e binários contra vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8. Só espaços literais (`#text("   ")`) com tracking 0/±0.2 produzem no Tekt `/MediaBox [0 0 0 0]` (Poppler apresenta 612×792), enquanto o vanilla mede respectivamente 9.36/9.76/8.96×8.568pt. Separadores literais finais após texto perdem a sua largura em páginas auto; os iniciais seguidos de texto já coincidem. Escopo com tamanho 18pt termina em 14.44×12.852pt vanilla para +0.2pt. Texto vazio e markup sem tinta revelam outra falta de baseline/mínimo; newline final tem divergência vertical própria e não é fechado por esta fatia.

Causa fonte: `space.rs:22-35` não transporta Literal inicial sem tracking; `page_geometry.rs:104-135` usa `helpers::line_content_right` que filtra todos os TextSeparator, por necessidade correta do alinhamento RTL, e a altura de linha só com carrier não recebe topo tipográfico; `builder.rs:291` e `builder/fonts.rs:633,1167` escrevem MediaBox zero. Vanilla `crates/typst-pdf/src/convert.rs:108-118` impõe mínimo físico de 3×3 na saída PDF. Isso justifica projeção auto separada do alinhamento RTL e mínimo compartilhado entre os três caminhos exportadores. Não inferir intenção histórica do escopo antigo.

### Contrato interno da correção

Transportar TextSeparator Literal desde o primeiro espaço mesmo com tracking zero, sem criar baseline ou tinta. O avanço/carrier continua nominal; tracking anterior é aplicado apenas quando existe unidade anterior elegível, sem duplicação. Markup/Newline iniciais continuam suprimidos. Preservar estilo literal resolvido para a projeção auto, sem introduzir estado global nem contagens codificadas.

Correção interna de paridade em fluxo contínuo ADR-0127, sem campo público, assinatura/flag/fase nova. Aceitação RED→GREEN para espaços 0/± e PDF mínimo, casos causais bilaterais, corpus anterior, build e lint sem erros, resselo estritamente no manifesto. Texto vazio explícito e newline final permanecem para rodada própria.

## Migração do carrier bidi ratificado (2026-10-05)

Medição anterior à decisão: o inventário `00_nucleo/diagnosticos/bidi-contexto-2026-10-05/inventory.json` identifica as referências desta unidade e os owners 1:1. O contrato do manifesto 1ae7513e foi confirmado; nesta unidade ajustar constructors não certificados com None e patterns de leitura com `..`, preservando os observáveis de linguagem existentes. Transformações obedecem ao núcleo pinado de origem; sem ampliar fase/API.

## Transporte lógico textual — contrato ratificado em 2026-10-05

Medição anterior à decisão: `00_nucleo/diagnosticos/bidi-contexto-2026-10-05/probe.json.gz` refuta resolver 456 789 isoladamente; a medição de fonte e os locais afetados estão no inventário e initial.json.gz daquele diagnóstico. A API imutável e os três campos opcionais foram ratificados pelo humano (manifesto 1ae7513e). Correção interna de paridade sem mudar fase.

Esta unidade participa do transporte por Cursor privado, input Arc e spans exatos. Preparar antes de emitir o parágrafo textual inteiro, base lexical já certificada; conservar offsets por ordem de visita, não matching de strings. Sequence/Par criam escopo, nested Sequence herda; checkpoint conserva cursor e span corrente; subframe isola e restaura. Text fornece subspans para palavras e espaços literais; Space consome seu segmento mesmo quando fraco; Linebreak consome fronteira. Word emite span somente se texto exato não foi transformado.

Escopo inicial: incremental simple, fora de células/colunas/subframes, sem tracking/hifenização/justificação/smallcaps, Text sem newline/tab/segmentação linguística. Região recusada tem None. Preparação global anterior permanece para parágrafos que não precisam do contexto bidi; para o subconjunto certificado com base RTL ou caracteres RTL/override oposto, encaminhar pelo incremental sem mudar os breaks permitidos. Não interferir em optimized explicitamente escolhido. O owner privado bidi_origin prepara/valida; esta unidade só coordena o seu papel. Testes de origem por ocorrências, UTF8, restauração e parbreak; validação bilateral de wraps.

## Colapso de espaços fracos adjacentes entre escopos

Medição anterior à decisão: verification.json.gz registra desvio de 3.4966pt na borda oposta de rtl_terminal_space/rtl_wrap e no DEF final de ltr_wrap. A árvore possui um Space fraco no fim do Styled e outro depois do envelope; space.rs:31 avança ambos. Correção interna de paridade: depois de consumir o segmento lógico, quando o último item já é TextSeparator Markup/Newline e o novo Space é fraco, não emitir/avançar a segunda ocorrência visual. Não colapsar Literal, não procurar igualdade de string, não fundir spans. Conservar o carrier sobrevivente e seu span; espaços fracos omitidos continuam no input lógico e não são reasignados. Aceitação RED→GREEN com Markup/Newline adjacentes e controle Literal, matriz fonte→PDF com mesmas larguras/fontes e escopos.

Fonte primária complementar: vanilla `typst-realize/src/spaces.rs:25–31,68–99,128–132` conserva os estilos do primeiro espaço do grupo. A primeira suíte após a correção encontrou uma asserção mecânica antiga que esperava wrap provocado pelo segundo espaço fraco; a fixture passa a inserir um novo B antes de testar wrap, conservando a obrigação de quebra após material. O novo teste usa tolerância numérica de 1e-9pt para soma de floats; diferença medida foi ~1e-14pt, não de geometria.

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

Manter regras de origem/duplicação e avanço; ao verificar overflow, usar a borda final efetiva do parágrafo. Não fabricar espaço para representar recuo.

### Aceitação

RED→GREEN antes da implementação; controles fonte→layout e bilaterais dos sinais,
fronteiras, variantes e restauração pertinentes. Não deduzir cobertura de todas
as alternativas por executar um arm agrupado: classificações nominais fechadas
do contexto inline/fluxo são intencionais e exigem testes por classe/variante.
Resselo apenas após V15/V26 válidos; testes, build, Clippy/Rustdoc/fmt, reversa
e diferencial Tekt do pin1279707, sob12GiB/High8GiB/swap0/jobs1. Sem publicação.
