# Aspas semânticas — materialização confirmada

Estado: pacote confirmado pelo humano e materializado. Identidade e campos
SmartQuote são preservados em eval; glyph é resolvido no render Paged/HTML.
A conclusão é restrita aos observáveis medidos abaixo, com negativos explícitos.

## Proveniência

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
Snapshots baseline-state, partial-state, ratified-before-suites,
delivered-state e nominal-final-state em smartquote-2026-10-06 guardam UTC,
diff HEAD --stat, lista exata de alterações, conteúdo e hashes das fontes.
Cada coleta bilateral guarda o estado, argv/cwd/ambiente, fontes e hashes dos
binários; cópias imutáveis evitam mistura com builds posteriores.
Coleta final: 2026-10-06T19:46:01.724376+00:00 a 19:46:12.345664+00:00.
Vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8,
SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea.
Tekt release medido:
3ac264d718706c32d8ace9aa1d3ac596de98da6534433b278c1deeaa0395e0cc.

## Medição anterior à decisão

As coletas before, fields-before, partial-and-html e proposal-controls guardam
os testemunhos anteriores. Upstream typst-eval/markup.rs:141–146 cria SmartQuote;
typst-layout/inline/collect.rs:196–211 escolhe glyph após styles e contexto;
typst-library/text/smartquote.rs:119–158 distingue primes, apostrophe e nesting.
Tekt markup_stream.rs:79–128 realizava Text antecipadamente; o constructor
apagava identidade com enabled:false e o layout alternava flags isolados.
Reuso após mudar idioma/disabled, primes numéricos e composição com funções
expuseram diferenças da linguagem. HTML rejeitava a folha programática.
Esses observáveis legitimam preservar identidade e resolver tarde; não se
copia a mecânica upstream nem se usa igualdade de bytes PDF como aceitação.

## Ratificação e implementação

Confirmação humana recebida para o pacote smartquote-semantica-2026-10-06.md.
Receipt: materialization/smartquote-semantica-aprovacao-2026-10-06.json.
Os 24 L0s completos e o núcleo proposto foram promovidos antes de testes/código;
pins foram ordenados canonicamente e cláusulas conflitantes foram supersedidas.
SmartQuoteElem.double/enabled agora distinguem None de Some(false).
Constructor Typst mantém assinatura e identidade quando desabilitado.
Markup conserva double explícito; show usa a fronteira estrutural existente.
Resolve_smart_quote e SmartQuoteContext são helper/contexto puro L1, por fluxo,
compartilhados por PDF/HTML. Estado inclui texto precedente e nesting e é
restaurado inteiro em checkpoint. Parbreak reseta; soft wrap não reseta;
linebreak é newline sem perder nesting. O hub mantém delegação estática das
unidades, sem novo NodeKind, I/O, globals, registry ou target HTML paginado.
Campos/JSON preservam presença. Getters semânticos mantêm defaults para filtros,
mas acesso direto usa Set/Unset/Undeclared sem fabricar campos conhecidos.
HTML conserva composição e escaping; títulos usam resolução contextual e
script/style mantêm o guard de string/raw text.

Duas adaptações de owners de testes e uma correção interna de acesso direto
ampliaram o pacote para 27 owners/consumers exclusivos, sem novo contrato público
ou fase além do gate ratificado. Field-access-before.json e field-access-red.log
registram true incorreto para double omitido; final coleta dá o erro vanilla.
Quotes-key-order-red.json e upstream smartquote.rs:394 medem keys() double,
single; Value canônico agora preserva essa ordem observável da linguagem.
A aspa adjacente francesa reabre após NBSP no oracle (fr-adjacent-oracle.json);
o teste histórico de alternância foi atualizado para esse testemunho.

## Verificação

RED genuíno: fields-red.log, html-red.log, field-access-red.log e
quotes-key-order-red.json. Semantic-red.log também registra uma tentativa
inválida de usar children() na linguagem; não conta como prova RED da correção.
GREEN final: nominal-core.log — 6561 testes passaram, 5 ignorados.
Delivered-core.log — 8 testes de integração e 3 doctests passaram.
Delivered-infra.log — 1192 passaram; delivered-cli.log — 126 passaram.
Nominal-release.log e nominal-build-debug.log: builds passaram.
Nominal-lint.sarif: zero erros, 528 avisos, 1585 notas; nenhum V5/V6/V15/V26
nos 27 owners. Contagem anterior de avisos foi preservada. Git diff --check passou.
Final-preservation.json comprova fontes/L0s alheios preservados e todos os selos
batendo. Final-checks.json registra hashes dos receipts. Não houve resselo global.
Uma rodada intermediária de doctests encontrou rlib ausente durante rebuilds;
a rodada entregue passou integralmente. Nenhuma falha intermediária fecha item.

## Resultado bilateral e limites

Delivered.json.gz: 44 controles, 41 coincidem integralmente em status,
stdout/stderr, texto PDF e HTML conforme target. Todos os testemunhos de
SmartQuote, campos, reuso/idioma/disabled, primes, custom, nesting, show,
set double, título e acesso direto coincidem nos observáveis de aspas.
Bilateral-results.json lista os três negativos que impedem paridade universal:

- eval-markup-disabled: wrapper Styled geral é achatado na serialização Tekt;
  as folhas SmartQuote e o render desabilitado estão preservados.
- compile-show-text: ambos rejeitam a receita, mas Tekt ainda rejeita o selector
  text enquanto vanilla reporta profundidade de show; não é controle positivo.
- html-reused-german: glyph „Hallo“ coincide; root lang permanece en no Tekt
  e de no vanilla. Metadados de idioma da raiz são débito separado.

Query-compilada-2026-10-06/after-smartquote.json.gz: 36/36 controles coincidem,
incluindo os 23 originais e os dois testemunhos de corpo com aspas antes
abertos. Não se certifica biblioteca inteira, Styled geral, metadados HTML,
show text, watch ou math phase. Os dois primeiros limites estavam explicitamente
fora do pacote ratificado; show text permanece negativo preexistente.
