# Prompt L0 — `compiler/eval/tests`
Hash do Código: 343763db

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
