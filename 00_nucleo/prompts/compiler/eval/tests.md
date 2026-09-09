# Prompt L0 — `compiler/eval/tests`
Hash do Código: 0b22e216

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/core.toml sha256:e7642a709c937928333439b2a78cdb3a6dbd6b56d67fcc67728efd2a26796e58

**Camada:** L1 test-only
**Ficheiro alvo:** `01_core/src/compiler/eval/tests.rs`

## Medição e contrato

Fornece Worlds puros, helpers test-only e regressões linguísticas do eval.
Fixtures não usam filesystem real. Asserções observam semântica, sintaxe,
morfologia e mensagens, não mecânica Rust incidental.

## Aceitação

Regressões têm controles e proveniência do vanilla quando decidem paridade.

## P1325 — sucessão test-only das sentinelas de dicionário

### Medição anterior à decisão

`00_nucleo/diagnosticos/p1325-workspace-tests.json`, SHA-256
`d276d5a90995ece30bebbb2805141f80ce029254570ecaa595d9401f0c131ead`,
registra HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree
não commitado, diff/stat integral, comando e saídas entre
`2026-09-09T01:06:19.832323+00:00` e `01:08:01.106174+00:00`.
A suíte executou 5573 testes core: 5570 passaram e três falharam, todos por
exigir span total de Dict nas sentinelas P1301/P1303/P1306 deste owner.
`01_core/src/compiler/eval/tests.rs:17824,18027,19621` mantém ranges
iniciados no receiver. A medição vanilla e a obrigação pré-candidata em
`compiler/eval/bindings/field_access.md` P1325 exigem somente o field.
Não são novas mensagens, lookup ou falhas de controles PDF/Module/Float.

### Decisão e aceitação

Correção contínua ADR-0127 exclusivamente test-only: substituir somente
as três expectativas de âncora de Dict em
`p1301_controle_nao_module_dicionario_preserva_span_total`,
`p1303_sentinelas_de_span_module_e_nao_module` e
`p1306_oracles::p1306_pdf_dict_float_and_import_controls` por ranges do
identificador à direita do ponto, derivados da expressão e da obrigação
P1325, nunca da saída candidata. Permite renomear o primeiro teste para
explicitar field-only P1325; não remover testes ou relaxar comparadores.

Esta seção sucede expressamente as exigências de Dict total de P1301,
P1303 e P1306 neste prompt, somente nessas três expectativas. Preservar
expressões, mensagens, hints, perfis, cardinalidade, helpers e todos os
demais controles. O owner permanece test-only 1:1; não modifica produto,
API/default/fase nem os artefatos A/B congelados. Autor independente escreve
o delta e o revisor confere sua suficiência. A falha histórica permanece
registrada; reexecutar os três grupos e a suíte completa, sem contar a
atualização dos testes como novo RED pré-candidato de produto. Necessidade
de qualquer mudança funcional refutaria esta classificação e reabriria L0.

## P1300 — regressão dos constructors globais de cor

### Medição anterior à decisão

Em `2026-09-03T19:01:51.133010-03:00`–`19:01:55.717850-03:00`, no baseline
`1f082370e59939de7b57992e137a9f74bfb6758f`, a matriz bilateral
`00_nucleo/diagnosticos/p1300-pre-gate-measurement.json` (SHA-256
`1678b1aed89bfff7581a8fdd998a32f57df18f134226e680596c587620ea3efe`)
mediu `hsl`, `hsv`, `linear_rgb` e seus três pares sob `std` somente no
cristalino, mas preservou bilateralmente `color.hsl`, `color.hsv`,
`color.linear-rgb` e os cinco constructors globais ratificados. Os resultados
foram iguais nos perfis `default`, `html`, `a11y` e `html+a11y`, sem
`EXECUTION_UNKNOWN`.

É inferência que regressões negativas e controles positivos no mesmo teste
distinguem remoção dos aliases de apagamento das nativas ou de uma única
projeção root/`std`. Um perfil em que a disponibilidade varie, uma rota
qualificada ausente ou um global ratificado ausente refutaria a inferência.

### Decisão e aceitação

Após confirmação humana do gate ADR-0127, regressões permanentes devem:

- exigir que `hsl`, `hsv` e `linear_rgb` falhem como variáveis desconhecidas
  nos quatro perfis;
- exigir que `std.hsl`, `std.hsv` e `std.linear_rgb` falhem como fields
  ausentes nos quatro perfis;
- comparar classe, mensagem, hints e span público de cada erro negativo com o
  vanilla ratificado;
- exigir que `color.hsl`, `color.hsv` e `color.linear-rgb` continuem funções
  chamáveis, com os valores e repr vigentes;
- exigir que `rgb`, `luma`, `cmyk`, `oklab` e `oklch`, tanto bare quanto sob
  `std`, continuem funções nos quatro perfis;
- impedir que a correção apague `native_hsl`, `native_hsv` ou
  `native_linear_rgb`, esconda qualquer rota qualificada, ou corrija somente
  uma das projeções root/`std`.

Esses testes observam superfície da linguagem e diagnósticos públicos, não
ordem interna de inserção, endereço de function pointer ou estrutura Rust.
Antes da confirmação humana, este contrato não autoriza escrever o teste RED.

## P1301 — retificação independente dos diagnósticos `Module`

### Medição anterior à decisão

O verificador P8 demonstrou que o GREEN P1300 era insuficiente: os testes
esperavam a mensagem cristalina `module 'std' does not contain field "…"` e o
span da expressão inteira, enquanto a matriz bilateral registrou `12`
`DIFFERENT_DIAGNOSTIC`. A medição P1301 em
`00_nucleo/diagnosticos/p1301-pre-gate-measurement.json` ampliou o controle a
`calc.nope`, `sym.nope` e `color.map.nope`: todos usam no vanilla a mensagem
``module `<nome>` does not contain `<field>` `` e ancoram apenas o field.

É inferência que testar a categoria `Module`, e não apenas os três aliases de
cor, discrimina uma correção sem blacklist. Sobreviver a mensagem antiga, ao
span total ou ao nome público `std` refutaria a suficiência do teste. A
divergência separada de `repr(std)` não é critério deste consumer.

### Decisão e aceitação

Os testes P1300 negativos continuam a exigir ausência dos seis aliases nos
quatro perfis, mas a expectativa de `std.hsl`, `std.hsv` e `std.linear_rgb`
passa a ser derivada do oracle vanilla:

- mensagens exatas ``module `global` does not contain `<field>` ``;
- hints vazios;
- span interno somente no identificador à direita do ponto;
- mesma classe em `default`, `html`, `a11y` e `html+a11y`.

Adicionar controles de field inexistente em módulos independentes `calc`,
`sym` e `color.map`, esperando os nomes públicos `calc`, `sym` e `map`, além
de ao menos um lookup de módulo existente que preserve valor/kind. Preservar
um controle não-`Module` cuja âncora continue a expressão inteira para impedir
generalização indevida contra P1293.

O teste RED deve falhar no baseline pré-candidato pelas expectativas novas de
mensagem/span, sem ler o patch de implementação. O GREEN só é válido se os
casos `Module` e todos os controles passarem; teste interno isolado não
substitui a matriz bilateral CLI selada. Mutantes mínimos obrigatórios:
mensagem antiga, span total, `std` em vez de `global`, regra restrita aos três
fields de cor, alteração do sucesso e generalização do span para targets não
`Module`.

Esta retificação é test-only e não autoriza mudar API, entidade, default,
fase, `repr(std)` nem os owners dos módulos. O dono autorizou a reabertura em
`2026-09-03`; a cadeia segregada P1301 começa de novos hashes e não ressela ou
reinterpreta os artefatos P1300.

P1250B retifica o oracle P744 depois de P1253: as constantes públicas
nomeadas preservam literais `f32`, portanto `red.mix(blue, space: rgb)` expõe
`#805b87` no vanilla ratificado. O valor `#805a88` pertence ao caso distinto
construído por canais inteiros `rgb(255,65,54).mix(rgb(0,116,217), ...)`.

P1252 protege pela superfície pública alpha em `luma(l, alpha: alpha)`,
`luma(color)` e na normalização de stops Linear/Radial/Conic com
`space: luma`. O teste exige alpha preservado nos três variants e não afirma
paridade de luminância nem promoção SVG.

P1239 fecha a luminância Luma na superfície pública: `luma(red)` e os
endpoints vermelhos de `gradient.linear(..., space: luma)` e
`gradient.radial(..., space: luma)` devem expor `54.02%`, como o vanilla
ratificado. O teste continua a exigir alpha preservado segundo P1252 e não
autoriza promoção SVG.

P1269-owner fixa a fronteira estrutural de stops Linear coincidentes em
offsets não diádicos: em Oklab e Linear RGB, `sample` e `samples` recebem a
razão pública devolvida por `stops()`; a coincidência exata escolhe o primeiro
stop daquele offset e um epsilon positivo escolhe o ramo à direita. O controle
usa `sharp(3)` para exercer `1/3` e `2/3` sem depender da representação textual
decimal.

P1271-C2 protege a morfologia pública de uma cor Luma normalizada para Oklab.
O stop `white` de um gradient Oklab deve expor os mesmos quatro componentes do
vanilla ratificado — inclusive os pequenos canais `a` e `b` não nulos — e o
controle sRGB vermelho continua no caminho P1253. O teste não observa a
estrutura das matrizes nem autoriza qualquer alteração de budget SVG.

P1271-C3 exige que `repeat(2)` preserve a resolução `f64` dos offsets
automáticos. Linear, Radial e Conic com quatro stops devem expor exatamente
`0, 1/6, 1/3, 1/2, 1/2, 2/3, 5/6, 1`; `sharp` é scope-out deste teste.

P1271-C4 fixa o sampling Radial em `37.123456789%`: Oklab `black/white` e
Linear RGB `red/blue` devem expor os componentes medidos no vanilla. O
controle nos endpoints continua idêntico; Conic e outros espaços são
scope-out, assim como qualquer observável SVG/PDF.

## P1215

Os testes de `eval_expression` reconstroem a `Source` code com o mesmo
`FileId` e exigem ranges exatos para chamada inteira, positional, named e
deslocamento por linhas. Mensagem sem range resolvível não satisfaz a prova.

## P1293 — retificação test-only da regressão P1105

### Medição anterior à decisão

O recibo segregado `p1293-implementation-receipt-b.md`, SHA-256
`3772126a3880a174c588dd62897e77e89bbf955fdd917dcd6218215132044eef`,
mediu em `2026-09-01T17:14:34-03:00`, no
`HEAD 7dd25ff0e222b6c7c640d6bc7957b98f94227507` e working tree não
commitada (`git status --short` SHA-256
`e7ce2e33d6683603012059deaf3e8254dbedee2a93c9b25663cf779b097df55a`;
`git diff HEAD --stat` SHA-256
`26d25cac81995d45a9c667fa181ba17259eb9f65327117d1fbaeffa0d1435b7e`):

- `20/20` spans negativos B coincidentes e suites de math/repr/call verdes;
- o filtro `p1105` com `5 passed / 1 failed / 5379 filtered`, exit `101`;
- a única falha é
  `p1105_attach_zero_ou_multiplos_args_posicionais_erro`: para zero
  posicionais o teste ainda exige a frase histórica portuguesa
  `attach espera exactamente 1 argumento, recebeu 0`, enquanto a superfície
  P1293/vanilla exige `missing argument: base`; para dois posicionais o teste
  ainda exige a frase histórica com contagem, enquanto a superfície exige
  `unexpected argument`.

O consumer test-only mantém header 1:1 para este prompt. A inferência é que
a falha é expectativa regressiva obsoleta, não falha de produto. Refutaria
esta inferência qualquer falha adicional nos outros cinco controles P1105,
mudança das mensagens congeladas no vanilla/contrato P1293 ou divergência
semântica, morfológica, de layout ou span no lote B; nenhuma foi medida.

### Decisão e aceitação

Retificar somente as duas expectativas de mensagem do teste P1105 citado:

- zero posicionais exige exatamente `missing argument: base`;
- dois posicionais exigem exatamente `unexpected argument`.

As mensagens portuguesas históricas não são alternativa aceita e o produto
não deve ser revertido nem receber fallback para satisfazê-las. Os outros
cinco controles do filtro P1105 e todas as demais asserções permanecem
inalterados e obrigatórios.

Mensagens diagnósticas são observáveis da linguagem (ADR-0107). Esta é uma
correção interna test-only de paridade em fluxo contínuo (ADR-0127): não
autoriza mudança de produto, API/campo/entidade/trait/assinatura pública,
default, compatibilidade, fase eval/layout, ordem de validação, spans,
morfologia ou layout. O ownership permanece exatamente este prompt para
`01_core/src/compiler/eval/tests.rs`; não se cria owner 1:N.

## P1303 — regressões dos spans dos gates `pdf.*`

### Medição fresca anterior à decisão

Em `2026-09-04T00:38:07.847942-03:00`–`00:38:10.279557-03:00`, no HEAD
`5b4a0d0438a535c54fdb5e74b28903c1313f5bc2` e working tree não commitada
sem diff tracked ou staged, a matriz bilateral fresca
`00_nucleo/diagnosticos/p1303-pre-measurement.json` (SHA-256
`ef3a4eb0b6fcb3fb0e9b1d8ec54bfbfa8f1a4f7b58dd7c675cbf677bada573d4`)
executou `48` runs e `24` comparações em ordem normal e invertida. Nos perfis
`default` e `html`, cada um de `pdf.data-cell`, `pdf.header-cell` e
`pdf.table-summary` coincidiu bilateralmente em exit `1`, stdout vazio, um erro
primário, zero laterais, mensagem e dois hints ordenados, mas divergiu somente
no span: vanilla field-only `14..23`, `14..25`, `14..27`; cristalino total
`10..23`, `10..25`, `10..27`. Nos perfis `a11y` e `html+a11y`, os três casos
foram `MATCH_VALUE` com kind `function` e nomes/`repr` `data-cell`,
`header-cell`, `table-summary`. O vetor normal foi `3` divergências em cada
perfil negativo e `3` matches em cada positivo; nas duas ordens houve `12`
`DIFFERENT_DIAGNOSTIC`, `12` `MATCH_VALUE`, `0` Unknown e `0` divergências de
repetição.

Mensagens, hints, cardinalidade, spans, disponibilidade por feature, kind,
nome/`repr` e morfologia de chamada são observáveis da linguagem sob ADR-0107;
estrutura Rust, helper, layout do enum, ponteiros e passos do algoritmo não são
critérios. É inferência que regressões negativas exatas, controles positivos
e sentinelas de fronteira distinguem a correção local de uma generalização ou
de uma mudança do gate. Refutam-na um mutante aplicável sobrevivente,
`Unknown`, divergência por ordem/repetição, falha positiva, ou regressão de
mensagem/hints/cardinalidade/sentinela.

### Classificação e obrigações

Classificação: `ADR-0127_CONTINUOUS_DIAGNOSTIC_PARITY`. As regressões são
test-only e protegem uma correção interna de paridade; não acrescentam API,
entidade, trait, assinatura, default, compatibilidade ou fase. Após L0-first e
resselo, o gate é RED→GREEN mais revalidação, sem nova paragem humana.

As regressões negativas devem cobrir os três fields nos perfis `default` e
`html` e, para cada um, exigir cumulativamente: falha bilateral; stdout vazio;
exatamente um erro primário de severidade `error`; zero diagnósticos laterais;
mensagem exata
``cannot access field `<field>` because the `a11y-extras` feature is not enabled``;
exatamente os dois hints seguintes, nessa ordem:

```text
try enabling the `a11y-extras` feature
see https://typst.app/help/compiler-features for more details
```

Devem ainda exigir span half-open resolvível e field-only `14..23`, `14..25`
ou `14..27`, stderr CLI byte-idêntico ao vanilla e classificação
`MATCH_DIAGNOSTIC`. O span não pode incluir `pdf`, o ponto ou qualquer byte
vizinho.

As regressões positivas devem cobrir os três fields nos perfis `a11y` e
`html+a11y` e exigir exit `0`, stderr vazio, nenhum diagnóstico/hint, kind
`function`, nome/`repr` exato e `MATCH_VALUE`. Uma chamada representativa de
cada função preserva a morfologia P1288: `table-summary` substitui somente o
summary semântico; `header-cell` e `data-cell` aceitam conteúdo cru ou
`table.cell`, preservam os demais fields e mantêm suas classificações/defaults.

### Sentinelas e poder discriminatório

As mesmas regressões devem preservar:

- `std.nope`, `calc.nope`, `sym.nope` e `color.map.nope` com mensagens
  P1301r2, zero hints e span somente em `nope`, inclusive o módulo global real
  alcançado por `std` publicando `global`; P1306 distingue esse módulo do
  import ordinário `std.typ` que publica `std`;
- um lookup existente em cada módulo — `std.rgb`, `calc.abs`, `sym.alpha` e
  `color.map.viridis` — com valor e kind preservados;
- o dicionário ausente com mensagem
  `dictionary does not contain key "nope"` e span da expressão completa;
- `float("NaN").is-nan` sem chamada com
  `cannot access fields on type float` e span somente em `is-nan`;
- `pdf.attach` e `pdf.artifact` acessíveis sem `a11y-extras`, com kind,
  nome/`repr`, valor e chamada preservados;
- o trio inacessível sem `a11y-extras` e acessível com a feature nos dois
  perfis positivos;
- `html` sem a feature HTML como `EXPECTED_FEATURE_DISABLED`.

O corpus deve produzir o mesmo mapa por chave `(perfil, probe)` em ordem
normal, repetição da normal e ordem integralmente invertida. `Unknown` e
`EXECUTION_UNKNOWN` nunca contam como sucesso. Os testes devem matar, com
testemunha observável, span total, span sobre `pdf`, inclusão do ponto, início
um byte à esquerda, fim um byte à direita, correção parcial por field ou
perfil, hints removidos/reordenados, mensagem alterada, exposição sem feature,
ocultação com feature, generalização ao dicionário, regressão `global` →
`std` e remoção do sucesso de `pdf.attach`/`pdf.artifact`. O gate adversarial
exige `14/14` mutantes aplicáveis mortos, `mutation_score = 1.0`.

Este consumer test-only não autoriza alterar o produto, o contrato funcional
de `pdf`, API/entidades/traits/assinaturas, defaults, features, pipeline, CLI,
exportadores, wiring, lab, `color.map`, extensões cristalinas, `repr(std)` ou
membros residuais P1299. A aceitação limita-se aos três spans e sentinelas
enumeradas; não prova equivalência funcional geral. Se o RED falhar por motivo
além dos seis spans previstos, o teste/contrato volta à medição; não se adapta
o produto nem se converte estado opaco em PASS.

## P1305-r2 — regressões de arrays, nomes públicos e imports

### Medição anterior à decisão

A medição independente `00_nucleo/diagnosticos/p1305-r2-pre-measurement.json`,
SHA-256 `fd6354824e500e61f18b14116dd54b4f4838691289226a7f7e04236258dd9da0`,
preserva fixtures, argv/cwd, binários, stdout/stderr completos, exit e horários
entre `2026-09-07T13:48:28.580684+00:00` e
`2026-09-07T13:51:19.266604+00:00`. Proveniência: HEAD
`8eb41b769eb840c7ab1063f981f98fdd4047952b` mais diff P1303 não commitado,
status/diff/stat registrados no artefato; baseline fresco SHA-256
`4a4e1bd46c053dd19bfcae2230537c9478fbfb2ff26a94dfd87d9f29fee2835d` e
vanilla ratificado `a51e02804` SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.

O baseline representa integralmente arrays longos (`repr.rs:36-41`) e
Module como `module(nome)` (`:56`). O global usa nome `std` nos caminhos
`eval/mod.rs:324,562` e `eval/modules.rs:66`. O bare import Module usa esse
nome público como ligação (`eval/modules.rs:184-213`), enquanto a fonte
vanilla `typst-eval/src/import.rs:82-105` e os controles medidos exigem nome
lexical. A medição também distingue bare dinâmico inválido de fonte dinâmica
com rename/items/wildcard válidos. Essas diferenças são linguagem e
morfologia; identidade de ponteiro não é critério de paridade.

### Decisão e aceitação

Após L0-first e resselo dos owners autorizados, regressões independentes
P1305 devem exigir:

- arrays de 0, 1, 39, 40, 41, 42, 81 e 256 itens; limite geral 40, marcador
  exato `.. (N items omitted)`, quantidade exata, pontuação, singleton,
  forma curta/multilinha e reindentação; números, strings, nesting, item
  multiline e item distinto no índice 40;
- dados integrais preservados depois de repr: comprimento, ordem, sequência
  completa, índices 39, 40, penúltimo e último quando existentes; o marker
  não serve de prova de igualdade dos dados. A serialização estruturada
  pública continua integral; o fallback textual de Module usa a nova repr;
- P1290 ASCII 49/50/51, escaping, conteúdo e listas genéricas integrais;
  não exigir mudança da dívida de forma em args/dict/sequences já medida;
- global com nome público `global` nos três caminhos reais de construção,
  exercidos nos perfis default/html/a11y/html+a11y; aliases conservam nome,
  kind, valores e lookup. `color.map`, calc, sym, pdf e módulos ordinários
  usam seu nome próprio e wrapper `<module nome>`;
- imports de `std.typ`, `global.typ`, `map.typ`, outro nome e reexport
  integral `std: *`, com aliases, não são reconhecidos pelo conteúdo;
- bare import de identificador/alias/field Module liga sob nome lexical,
  sem introduzir `global`; literal-file, `as`, items e wildcard permanecem
  válidos e preservam valores. Fonte dinâmica com `as`, items ou wildcard
  não recebe o guard do modo bare;
- bare Module dinâmico sem nome/lista falha na expressão fonte com
  `dynamic import requires an explicit name`, hint único
  ``you can name the import with `as` `` e span exato medido. Não aceitar
  erro posterior `unknown variable` como equivalente nem sucesso;
- sentinelas P1300/P1301r2/P1303 e os três gates pdf permanecem preservados
  nos quatro perfis; sem novo diagnóstico lateral ou warning.

Para bare identifiers, o warning vanilla `this import has no effect` já
ausente no baseline é dívida explícita. Exigir stdout/resultado ou erro
primário/hints/span exatos e preservar os laterais cristalinos do baseline;
não remover warnings genericamente do comparador. Os casos dinâmicos medidos
não têm essa exceção: exigir diagnóstico integral correspondente.

O autor independente também retifica a expectativa existente
`repr_value_module` em `repr.rs` para `<module mylib>`. Testes não podem
calcular suas expectativas pela função candidata. RED tem causa semântica
contratada; erro de build, harness, fixture ou import inválido num positivo
não conta. Mutantes obrigatórios cobrem limites 39/41, elisão ausente ou só
de mapas, quantidade errada, dados truncados/reordenados/omitido alterado,
singleton/indentação, helper genérico elidido, wrapper antigo, global `std`,
todo módulo global, rename nominal de std.typ, rota global parcialmente
corrigida, ligação pelo nome público, guard dinâmico ausente ou aplicado a
modo válido, e alteração de lookup/feature gate. Mutantes válidos devem
ser rejeitados semanticamente; `Unknown` e execuções inválidas não são
sucesso. Oráculos ficam congelados antes de ler candidato; este owner
test-only não autoriza editar os outros owners ou ampliar escopo.

## P1306 — regressões da identidade diagnóstica de módulos nomeados

### Medição independente anterior à decisão

O recibo `00_nucleo/diagnosticos/p1306-contract-measurement.json`, SHA-256
`d3066b1bf00ff3120190e329752df7547758f5b03f367a22c6cded83f31310fb`, registra
304 execuções válidas entre `2026-09-07T15:07:28.245613+00:00` e
`2026-09-07T15:07:53.110165+00:00`, HEAD
`b303f1f15b610e09872b567027e0d806387fde8c`, sem diff tracked/staged e com
untracked exatos no status antes/depois. Baseline certificado P1305 SHA-256
`be51045f1df75ac42ee081801ddf7f738f709029e3694b9205473ba5fa8b31d4`; vanilla
ratificado `a51e02804` SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.

As rotas ordinárias direta/alias/sombra/reexport/nesting de `std.typ`
publicaram `global` no erro cristalino contra `std` no vanilla, nos quatro
perfis; os positivos pareados confirmaram import, repr e lookup válidos.
O global real e aliases já publicaram `global`. A fonte pré-candidata
`01_core/src/compiler/eval/bindings/field_access.rs:376-382` revela conversão
nominal remanescente, embora `eval/mod.rs:324,562` e `eval/modules.rs:66` já
guardem `global`. A fonte vanilla
`lab/typst-original/crates/typst-library/src/foundations/module.rs:139-150`
projeta o nome guardado. A medição mantém a falha anterior de opção CLI como
instrumentação inválida, nunca RED. O dict total preexistente segue controle
cristalino de preservação, não alegação de igualdade com vanilla.

### Classificação e obrigações de teste

Diagnósticos e lookup são linguagem; estrutura Rust não é oracle. É inferência
que casos pareados e adversariais distinguem identidade real de coincidência
nominal; um mutante válido sobrevivente, positivo inválido ou observável
obrigatório opaco refutaria a suficiência. Esta correção test-only acompanha
`ADR-0127_CONTINUOUS_DIAGNOSTIC_PARITY`, preservando API/default/fase.

Os testes devem separar explicitamente global real de arquivo ordinário
`std.typ`. O primeiro, inclusive alias, exige nome `global`; o segundo exige
`std`, independentemente de alias, sombra lexical, nesting ou reexport integral
`std: *`. Cobrir também arquivos `global.typ`, `map.typ`, nome não reservado,
calc/sym/color.map e acessos existentes com valor/kind/chamada/repr preservados.
As expectativas vêm da fonte/intenção/vanilla congelados antes do candidato.

Cada erro de módulo nomeado exige mensagem exata
``module `<nome-guardado>` does not contain `<field>` ``, severidade error,
zero hints, um erro primário, ausência de laterais novos, stdout vazio, exit
correto e span half-open resolvível apenas sobre o field. Cobrir `nope` e
outro identificador, com offsets de linha e coluna diferentes, nos perfis
default/html/a11y/html+a11y. Import inválido, arquivo ausente e falha de parser
ou adaptador não contam como RED. Uma origem importada exige resolver a âncora
na fonte real; apresentação de path não autoriza apagar diferenças semânticas.

Preservar cumulativamente P1300/P1301/P1303/P1305: gates PDF, mensagens e hints
ordenados, spans, funções presentes, dict com span total, float/is-nan
field-only, imports e repr. Warning preexistente de bare import é dívida
individualizada por fixture/perfil antes do selo; nunca retirar warnings
genericamente. Testes não corrigem wrappers, arrays, constructors ou encoders.

O corpus e seus controles devem rejeitar nove famílias: projeção nominal
antiga; todo módulo global; global real chamado std; identidade pelo alias;
correção somente de um field; correção somente de um perfil; span sobre alvo
ou expressão; diagnóstico acrescentado/perdido; lookup existente
apagado/alterado. Aplicação/compilação inválida de mutante não é morte; exige
instrumentação corrigida com preservação da tentativa e restauração verificada.

Exigir RED pré-candidato pela divergência de nome, GREEN dos mesmos casos,
controles preservados, todos os mutantes válidos rejeitados, repetição normal
e ordem invertida estáveis. `Unknown` obrigatório bloqueia; não inventar caso
opaco para preencher matriz. Este owner permanece exclusivamente test-only,
sem autorização para modificar o consumer de produto ou as provas históricas.

## P1307-R3 — regressões de encoders e transporte causal

### Medição anterior à decisão

Os recibos P1307 SHA-256
`f1191d3de029dabf6f66f87872bee8146aa43106ae54e84facc22952afd238d0`
e R2 SHA-256
`847faabad41df603a82f7fc5c5d0435180cdec66c33ae9b3a1fd55d7ee320fa2`
em `00_nucleo/diagnosticos/` medem a superfície inexistente no baseline,
repr parcial anônima, casts de named anteriores e origens f/g distintas.
Baseline R3 SHA-256
`b50e726c5830c0f91a6875d2d0a758903bc93b0bd719f4b5357828522a999293`
conserva HEAD `b303f1f15b610e09872b567027e0d806387fde8c`, diff/stat completo
P1306 e fontes antes dos novos L0. A referência permanece upstream ratificado
`a51e02804`, nunca a string de versão isolada. R2 resolveu observabilidade
contextual com compile e features reais; uso anterior de `eval --in` inválido
não conta como RED nem ausência do tipo.

### Obrigações após aprovação ADR-0127

Este consumer é exclusivamente `01_core/src/compiler/eval/tests.rs`.
Seu autor independente congela expectativas antes do candidato. Só depois
da aprovação pública e auditoria dos writers de Args pode escrever RED.
Reutilizar evidência válida, mas completar previamente as mudanças de ordem
de Args, filter/map/join, repr With de outras nativas e deltas de fallback
CBOR que ainda não foram medidos. Fonte sustenta a proposta, não GREEN.

Exigir strings integrais JSON/TOML/YAML em default/pretty/compact quando
válido, dados e escaping completos, ordem TOML escalares/tables, multiline,
None, não finitos, -0.0, Bytes/Symbol/Content/LocatedContent e fallbacks de
cada classe pública. Não usar o candidato como gerador de expected, nem
round-trip, substring, tamanho ou bytes truncados como oracle suficiente.
Location e LocatedContent usam rota contextual com fixture válida, resultado
tipado e observação do valor real, nunca marker copiado do eco da source.

Cobrir pais chamáveis, namespaces exatos, membros ausentes de csv/xml/read,
alias/With/shadow/reexport e default/html/a11y/html+a11y. Repr do membro é
encode; repr de With é `(..) => ..`. Parent With não transmite prebindings
ao membro. Cast inválido anterior não desaparece com named válido posterior.
Encadeamentos, spread de Array/Dict/Args, sink/factory, import com source
resolvível e iguais valores com origens distintas devem discriminar perdas.
Filter conserva spans individuais; map destaca value_span; ambos destacam
o agregado. Join elimina named LHS presentes RHS, diferente de With.
Callbacks de arguments usam ordem causal/repetições; pos/named projetam as
views, len de linguagem conta ocorrências e len Rust mantém posicionais.

Cada negativo compara mensagem completa, severidade, exit, stdout, quantidade,
hints ordenados, laterais e ranges UTF-8 half-open na source real. Detached
previsto pelo contrato é distinto de Unknown do adaptador e de perda indevida
de span. Não remapear mensagem para selecionar âncora nem apagar diferenças.

Preservar cumulativamente as sentinelas P1300/P1301/P1303/P1305/P1306, os
decoders e os controles CBOR já congelados. Symbol/Content CBOR continuam
dívida, não equalizar silenciosamente ao vanilla. A repr canônica corrigida
altera explicitamente os fallbacks de State/Counter/Location/With/Args;
esses casos precisam de expectativas sucessoras e comparação integral,
não da afirmação falsa de preservação geral de CBOR.

Ataques devem discriminar no mínimo: binding/perfil omitido; pai quebrado;
namespace herdado com args indevidos; identidade parcial incorreta; pretty
ignorado; ordem/escaping/multiline adulterados; perda de dados; classe
especial tratada como repr; TOML aplicado aos outros formatos; None/não
finito/-0.0 errados; fallback Debug; erro/hint/lateral/range incorreto;
ocorrência named apagada; origem f/g fundida; carrier stale por view mutada;
filter/map com spans trocados; join tratado como With; sink que retém
consumidos; callback reordenado; alteração CBOR fora da exceção contratada.
Não substituir as famílias obrigatórias do plano independente já congelado
por esta lista resumida. Mutante inválido não é morto; Unknown obrigatório
bloqueia, e cada controle novo precisa de decisão nova. Os artefatos antigos
permanecem imutáveis; este L0 não declara testes executados ou aceitação.

### P1307-R4 — refinamento independente antes de RED

O focal encerrado em `2026-09-07T17:37:05.017565+00:00`, baseline R4 SHA-256
`52df1c661c20d9eb612bbd5c89ae3cae8c44735aeab27c11e4a1da0d4d145e57`,
conservado integralmente em `00_nucleo/diagnosticos/p1307-r4-contract-refinement.json`,
mede que o sink agrupa named restantes antes dos positional, sem apagar
duplicatas; o constructor arguments continua na ordem lexical. Expectativas
devem distinguir esses caminhos, conforme `compiler/eval/closures.md` R4,
inclusive consumed named, default omitido e duas origens iguais distintas.
Não exigir o intercalamento incorreto inferido em R3 para o resultado do sink.
A dívida de binding de sink não-terminal permanece controle baseline explícito.

O mesmo focal `args.join-duplicates` mede sucesso vanilla e erro baseline
`cannot add arguments and arguments`. Testar o operador + real, não apenas
o helper de join: o roteamento de `compiler/eval/operators/arithmetic.md`
também é necessário à semântica Args+Args já aprovada. As outras combinações
aritméticas permanecem controles; None-identidade de join não implica nova
aceitação de Args+None pelo operador +. São correções medidas de contrato e
roteamento antes do candidato, não expectativas escolhidas para um patch.

## P1308 — regressões independentes dos quatro limites autorizados

Medição anterior à decisão: recibo `p1308-measure.json` em diagnosticos,
SHA-256 `ce758b5c2a18288bf9c8433178f577b50c52df80cedcddcb1f4daa4e785573fc`,
fixa os dois binários e mede Args+None, None+Args, callback/panic, traces e
repr longa. Os L0 proprietários P1308 determinam os resultados, não o patch.

Testar identidades Args/None sem perda de origem, âncoras de closure
direta/alias/With/spreads, mensagem completa do cast boolean, panic em call
inteiro e Source/trace pela entrada pública. `filter(str)` mantém origem
detached da função sintetizada a partir do tipo. Erros named de panic
conservam dívida baseline explicitamente, sem alegar paridade.
Nesses controles, baseline fixa mensagem e span primário; o trace segue a
regra vigente com Source resolvível, conforme o owner panic, não a omissão
acidental de trace no baseline.
Migrar independentemente somente o expected inline longo de
`p1305_args_fields_remain_integral` para a forma multiline integral medida.
Acrescentar fronteiras 50/51 bytes, sem truncamento. Nenhum teste é removido;
falhas adicionais exigem diagnóstico e decisão, nunca adaptação ao candidato.

## P1308-R2 — transcripts de preservação com Source resolvível

Medição anterior à decisão: `p1308-verification-final.json` em diagnosticos,
SHA-256 `06a57344f4e8039c1cd96a1df1aa2b2b2c159797e8080f1841844541e012ac60`,
reclassifica os 1982 registros públicos sem modificar o observador: 76
violações novas em 19 casos, todas exclusivamente por um trace anexado ao
stderr esperado. Mensagens, severidades, spans primários, hints e valores
continuam iguais. O recibo fixa código, binário e estado exatos. A Source
local passou a ser resolvível; não houve correção de validação CBOR/decoders.

O dono autorizou explicitamente migrar esses transcripts e commitar em
“Faça o commite do que falta e autorizo”, após conhecer esse bloqueio.
São observáveis diagnósticos, não igualdade mecânica. O contrato sucessor
preserva a dívida primária e acrescenta o trace causal, sem alegar paridade
vanilla dos erros portugueses/detached.

Autor independente pode criar sucessor dos 19 casos enumerados nesse
recibo (quatro cbor.encode, quinze decoders/read), nos quatro perfis,
substituindo somente o stderr esperado por sua extensão literal autorizada.
Não apagar traces, não criar comparador tolerante, não alterar os demais
casos, fixtures, Unknown, hints ou erros primários. Artefatos R6 e P1308
anteriores continuam imutáveis, inclusive resultados Violated históricos.
Os 18 testes P1308 permanecem intactos; esta revisão não exige código de
produto nem novos testes internos, só legitima a exceção estreita de
preservação do corpus associado. Revalidar o fragmento, a matriz completa
e a ordem inversa; alteração além do trace invalida a migração.
