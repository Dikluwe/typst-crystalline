# Headings HTML através de estilos e raiz explícita

Corrigidos html-headings e html-show da amostra macro. **13/13 controles DOM
passam**, incluindo níveis 1–6, regra show, heading gerado seletivamente, alias
duplo, parágrafos estilizados e html/body explícitos. Não declarar paridade HTML
integral a partir desse recorte.

## Proveniência

Working tree não commitado, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, coleta final UTC `2026-10-06T13:47:52.056855+00:00`.
Tekt SHA-256 `d448be2e5831441836dba19333c0d74989a64c16c6cb72bcdd17075fe7ac80b8`, vanilla pinado
9f2b6e8715237cb086899a42873660fe744622e8, SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`. before.json conserva fonte/L0 anterior,
stat/status; execution.json.gz conserva fonte/L0 final, hashes, stat/status e
logs de RED/compilação/GREEN/build. intermediate-measurement.json.gz conserva
as duas falhas de raiz da candidata intermediária; os logs de erro de compilação
não são contados como RED comportamental. Fonte, comando, HTML bruto e DOM
em measurement.json.gz/measure.py. RAYON_NUM_THREADS=1, SOURCE_DATE_EPOCH=0,
NO_COLOR=1. O L0 de show proposto depois da implementação está explicitamente
registrado como não materializado; o código de show permanece inalterado.

## Medição antes da decisão

`html/document.rs` classificava blocos apenas diretamente. Styled em torno de
Heading caía no serializador inline, que rejeita blocos. A emissão de Heading
direto também usava o nível sem offset. No oracle, typst-html/src/rules.rs:223-260
reserva h1 para título, mapeia seção nível 1 a h2 e nível 6 a div role=heading
aria-level=7. A baseline macro confirma erro do Tekt contra h2/h3 do vanilla.

Os testes de wrapper/nível foram RED; o controle de estilo inline já passava.
O refutador adicional de Sequence estilizada exige manter texto anterior no
mesmo parágrafo até a fronteira do heading. A primeira candidata revelou raízes
duplicadas: Styled ocultava html/body de explicit_document_root. red-root.log
confirma a falha antes de projetar estilos/sequências na classificação de raiz.

## Decisão e implementação

O owner document classifica blocos através de estilos/sequências transparentes.
Um buffer de parágrafo continua através desses wrappers; flush ocorre na
fronteira real do bloco. Chain lexical é transportada ao serializar cada folha;
origens consumidas dos carriers continuam projetadas para o visual. Heading
é emitido por helper único, com offset/fallback de nível semanticamente medido.

A classificação de raiz explícita coleta nós relevantes sob Styled/Sequence
e conserva a chain do nó selecionado. Não insere segundo html/body automático,
nem aceita raiz com irmãos relevantes; o diagnóstico de exclusividade permanece.
Não procura raízes dentro do body de HtmlElem arbitrário. Compacto é montado
antes do pretty, como antes; nenhuma lógica de layout paginado foi transferida.

L0 primeiro e hashes locais; correção interna de paridade ADR-0127, sem nova
API/fase/default/unsafe/dependência. Cinco testes de regressão locais validam
blocos, níveis, continuidade do parágrafo, estilo inline e raiz/exclusividade.

## Validação e limites

**1.183 testes L3 passaram**, sem ignorados, incluindo as suites de frames/raízes
já existentes. Builds debug/release e diff-check passaram. Hash final do consumer
e CodeHash do owner conferidos; V15/V26 íntegros. Lint antes do novo L0 de show:
0 erros, 531 avisos, 1.577 notas; nenhum erro/aviso novo pelos reparos. Formas introduziu uma nota de alternativas
e mudou a descrição de uma nota de profundidade; HTML removeu duas notas de
cobertura, como registrado em lint-delta.json. Após salvar o L0 de show para gate, há um V5 esperado nesse
owner não materializado; não ressellar o código de show para ocultar a proposta.

As 13 sondas comparam DOM do body (tags/atributos/texto), não bytes do documento.
Raw HTML e stderr foram preservados. A equivalência DOM de nível 6 não certifica
os avisos de heading profundo; numbering e restante superfície HTML também não
foram medidos. **228/228 controles PDF de layout** foram reexecutados no mesmo
binário final, sem regressões. Macro atual **31 MATCH e 5 DIFF em 36 sondas**;
quatro paths de diagnóstico e recursão de show continuam abertos. Não há
percentual global nem ganho de tempo declarado.
