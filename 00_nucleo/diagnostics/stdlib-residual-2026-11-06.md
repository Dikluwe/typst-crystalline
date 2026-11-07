# Controle sem imports e candidato adiado de stdlib

O snapshot antecipado custa uma alocação de 136 bytes por avaliação sem imports, mas os experimentos não atribuíram a ele o residual de startup em milissegundos. O candidato nominal adia o template até o primeiro import e preserva as alocações do controle. Seu ganho com imports é claro; o controle de startup continua sendo um limite de aceitação, não um item encerrado. Nenhuma otimização foi integrada ao produto.

## Estado e método

HEAD `3c6b0ce7b7632cc74b004cea6681e4746e63d28d`, working tree não commitada, congelada em `2026-11-06T02:18:37.074309+00:00`. O `git diff HEAD --stat=240` é exatamente igual à lista integral do [apêndice do experimento anterior](stdlib-reuse-2026-11-05.md#diffstat-exato-do-estado-recebido): 184 arquivos, 8388 inserções e 537 remoções. Os 53.103 arquivos originalmente recebidos foram conferidos por conteúdo e preservados. Nenhum arquivo produtivo ou L0 foi editado. As medições atravessaram 5–6 de novembro no fuso local; os CSVs de pipeline registram cada instante em `sample_epoch`.

Usei o skill Tekt de materialização segregada: A produziu overlays/helper/sondas, B escreveu testes sem receber a solução de A, C integrou e executou. Código e testes estão em [lab/experiments/stdlib-residual](../../lab/experiments/stdlib-residual/). O isolamento foi de contexto de autoria, não de filesystem. A solução foi aplicada somente em cópias RAM. Os controles de alocação, microbancadas e mutantes também foram executados ali.

Rust 1.92.0; Cargo offline/locked, release `opt-level=s`, thin LTO, codegen-units=1, um job por build, limites configurados de 8/12 GiB e swap zero. Cada variante teve workspace/target separado em `/dev/shm/tekt-stdlib-residual-20261105`; o cache antigo foi apenas copiado. Não houve alteração de código durante os relógios. A variante nominal foi construída depois dos lotes diagnósticos; seus resultados permanecem separados.

O lote diagnóstico alternou seis variantes nas mesmas fixtures 0/16/64 imports: baseline `bas`, antecipado anterior `old` e políticas no mesmo ELF: `p00` original, `p01` captura sem reúso, `p02` captura com reúso, `p03` adiado. Foram dez rodadas × duas repetições. Outro lote comparou `bas/old/p03/laz`, sendo `laz` o candidato estático sem instrumentação. CPU fixado no primeiro permitido (0), Rayon 1, stack 64 MiB, epoch 0, UTC, fontes padrão do sistema. CLI usa wait4; watch registra startup, mesmos bytes e edição de main, com CPU por clock do processo. Wall quente inclui eventos/debounce. RSS é pico do processo, não de uma fase.

As tabelas usam mediana das médias das repetições por rodada; percentuais e intervalos vêm dos deltas pareados por rodada, bootstrap de medianas, 10 mil reamostragens, seed 0. A diferença dos absolutos não precisa coincidir com a mediana dos percentuais. Os intervalos são descritivos do lote; não certificam generalização nem custo zero.

## Residual sem imports

Em quatro avaliações pré-parseadas com descarte completo, o controle e o adiado fazem 27.940 alocações e solicitam 8.345.276 bytes. Captura e reúso antecipados fazem 27.944 / 8.345.820: +1 alocação e +136 bytes por avaliação. O teste independente repetido contra as rlibs nominal e baseline confirmou a mesma igualdade de alocações. Pedidos de alloc/realloc não são bytes copiados, RSS ou memória simultaneamente viva.

O adiado não mantém o objeto snapshot no caso zero, mas não torna toda a execução idêntica: `EvalContext` cresce de 2976 para 2992 bytes; Scope permanece 56 e Module 8. Esse aumento de stack é observado; atribuir a ele qualquer latência específica continua sendo inferência não demonstrada.

Onze rodadas, 200 avaliações por janela, Sources pré-parseadas, sem fontes/layout/exportação:

|Comparação, zero imports|Eval por chamada, ms|Delta pareado e intervalo, %|
|---|---:|---:|
|Baseline → nominal adiado|0,8853 → 0,8914|+0,57 [0,20; 1,12]|
|Original → captura, mesmo ELF|0,8924 → 0,8917|−0,17 [−0,22; 0,19]|
|Original → reúso antecipado, mesmo ELF|0,8924 → 0,8899|−0,19 [−0,70; 0,11]|
|Original → adiado, mesmo ELF|0,8924 → 0,8935|+0,11 [−0,13; 0,69]|

Não surgiu custo temporal estável da captura no contrafactual do mesmo ELF. O nominal teve diferença positiva pequena no eval isolado, da ordem de microssegundos; ela não explica automaticamente o residual em milissegundos do watch. O antecipado anterior foi mais rápido nesse eval e ainda apresentou startup mais lento. Portanto, remover a alocação não demonstra eliminar todo o residual.

No primeiro lote, o antecipado anterior teve CLI zero +0,01% [−0,37; 0,70] e startup +0,89% [0,37; 1,53]. No lote nominal, o adiado ficou assim:

|Controle zero|Baseline → nominal, ms|Delta pareado e intervalo, %|
|---|---:|---:|
|CLI wall|96,10 → 96,29|+0,22 [−0,37; 0,70]|
|Startup watch wall|138,30 → 139,25|+1,11 [0,005; 1,96]|
|Watch edição, wall|88,77 → 88,69|−0,33 [−0,60; 0,56]|

Esse startup motivou a confirmação focada descrita adiante. Não se descartou esse lote nem se combinou lotes para esconder a diferença.

## Fontes e limites da atribuição

Sondas inclusivas L3, no caminho PDF padrão, mostraram no controle original cerca de 86,00 ms na inicialização do catálogo na CLI e 129,11 ms no startup do watch. Eval ficou em 1,56 / 1,50 ms; export dispatch em 0,74 / 0,62 ms. As faixas se sobrepõem: fontes podem estar dentro de paged; não somá-las como partição. Paged menos eval também não seria layout puro.

A calibração ligado/desligado, seis rodadas × duas repetições, detectou na política original CLI +0,55% [0,13; 1,14]; outros pares cruzaram zero. A instrumentação tem interferência comparável ao residual pequeno. Essas sondas sustentam a dominância de fontes, mas não a localização precisa de uma diferença submilissegundo.

O harness de APIs públicas usa duas Worlds novas e independentes: configuração → Source → primeiro book → eval/drop; e configuração → Source → PDF padrão/drop. O segundo repete fontes/eval e não é uma partição do primeiro. Em nove blocos de cinco amostras, as medianas individuais do baseline/antecipado foram: catálogo 80,72/80,75 ms; eval/drop 1,018/0,993 ms; PDF inclusivo 82,86/82,55 ms. Esse harness não reproduziu a perda de startup; tampouco mede a sessão de fontes do watch.

`World::source` foi predominantemente um hit de cache. Sua faixa não mede parse exclusivo; aquisição/parse do main não foram separados exclusivamente da configuração/aquisição neste experimento. Não se conclui parse gratuito. A microbancada exclui parse por construir Sources antes do relógio.

**Classificação:** custo direto de captura identificado; causa fina do residual de startup ainda **Unknown**. Variações de código gerado, disposição de memória e do trabalho de fontes são hipóteses, não causas provadas. A medição não autoriza uma nova otimização de Fontconfig, filesystem, layout ou representação de Module.

## Ganho e fronteira do candidato

Evidência de código anterior à escolha: os builders originais estão em `01_core/src/compiler/eval/mod.rs:640`, `:848` e `compiler/eval/modules.rs:60`; `entities/module.rs:66` compara identidade de Arc. O guard e o fallback medidos estão em `lab/experiments/stdlib-residual/snapshot.rs:35` e `:77`; o memo de instanciação começa em `:99`. O owner atual de Module continua especificando Scope owned, e o candidato não o altera.

|Fan-out|CLI baseline → nominal, ms|Delta CLI, %|Startup baseline → nominal, ms|Delta startup, %|
|---|---:|---:|---:|---:|
|16 imports|110,79 → 102,20|−7,63 [−8,25; −6,97]|153,92 → 145,35|−5,07 [−5,88; −4,88]|
|64 imports|155,14 → 122,52|−21,10 [−21,39; −20,70]|201,47 → 170,31|−15,46 [−16,08; −15,20]|

O candidato seguro em correção fica limitado a um template privado da stdlib pinada, criado no primeiro import. Root e expressão mantêm os builders originais e Scope owned. EvalContext começa com slot vazio. Inputs continuam consultados na ordem original; o guard exige todos os pares na mesma ordem e todas as features. Mismatch recebe seed para o próprio subtree e não substitui o seed do caller. Cada arquivo ganha identidades Module/Element próprias, com aliases internos preservados. Funções With produzidas durante eval continuam locais; o corpus não certifica uma clonagem geral de With no template. Não se memoizam Modules avaliados, nem se guardam estilos/show-rules/scopes locais, nem se reutiliza entre compilações. Closure/Plugin não são templates gerais autorizados.

Contagens fora dos relógios confirmaram: zero imports 1 → 1 builder; 16 imports 17 → 2; 64 imports 65 → 2. No watch com imports, o nominal soma 2/4/6 builders em startup/mesmos bytes/edição. Sources permanecem 17/34/51 ou 65/130/195; convergência e Layouter somam 1/2/3 em ambos. No controle zero, Source soma 1/1/2 em ambos (hit nos mesmos bytes). Layout permanece separado e sem otimização.

O ELF nominal foi conferido sem seletor/env diagnóstico, método adicional em World, portais, RootLibrary ou sondas. A futura materialização deve ter helper com owner L0 próprio, atualizar os owners de eval/imports antes do código, restringir o template à composição estática e revalidar o ELF final. Não há proposta de mudar o contrato de World, igualdade de Module/Func, representação de Module ou fases eval/layout.

## Confirmação focada e decisão

Novo lote, mesmos ELFs baseline/nominal, dez rodadas × duas repetições, controle mínimo e parágrafos sem imports. Nenhum código mudou; não houve sondas, builds ou contadores concorrentes.

|Controle|CLI baseline → nominal, ms|Delta CLI, %|Startup baseline → nominal, ms|Delta startup, %|
|---|---:|---:|---:|---:|
|Mínimo sem imports|95,28 → 95,83|+0,63 [0,20; 0,94]|137,77 → 140,24|+2,11 [1,39; 2,48]|
|Parágrafos sem imports|138,83 → 137,67|−0,95 [−1,28; −0,53]|181,41 → 179,23|−1,28 [−1,88; −0,13]|

O residual do controle mínimo não foi eliminado. Também não é penalidade uniforme em qualquer documento sem imports. CLI/Startup desse controle têm perda reproduzida na confirmação; o ganho no parágrafo não a cancela. A captura não é explicação suficiente: ela foi retirada do nominal, as alocações igualaram o baseline e o residual permaneceu.

**Candidato delimitado:** adiado, dois builders com imports estáveis, sem mudar root/World/layout/identidades. **Não liberado para integração ainda.** É compatível com o corpus testado e tem benefício em imports, mas não há evidência para declará-lo livre de regressão no controle mínimo. Não se presume um orçamento de regressão aceito pelo usuário.

O próximo experimento deve confrontar um controle estático neutro com o mesmo slot vazio em EvalContext contra baseline/nominal, e isolar descoberta, aquisição e evidências de frescor na sessão inicial de fontes do watch, com os mesmos caminhos/opções do produto. O objetivo é distinguir efeito de representação/código gerado de trabalho real adicional de fontes. Primeiro medir; não implementar uma nova otimização nem enfraquecer frescor. A causa segue Unknown até um contrafactual reproduzível. Após resolver esse gate, materializar apenas o helper/imports sob seus L0s e repetir o lote no ELF produtivo, incluindo pacotes reais e inputs grandes. Nenhum desses passos posteriores foi executado.

## Validação e dados

Passaram 17 controles públicos no baseline, antecipado anterior e nominal; 23 testes no diagnóstico, incluindo a matriz das quatro políticas, mais o gate independente de alocações. Cobrem identidades/aliases, imports repetidos/aninhados/pacote, inputs alterados/reordenados, features, roots/Worlds intercalados, fallback, show-rules/registry/host de plugins, erros/traces e fachadas de expressão. Isso limita a conclusão ao corpus, não é prova geral da stdlib futura.

Quatro mutações compilaram e foram mortas em execução por asserções: compartilhar identidades, ignorar inputs, ignorar features e antecipar a captura no modo adiado. Não se contou erro de compilação como morte. O teste de features rejeitou explicitamente policy 2, seed 0, pedido Bundle: faltava o binding exigido.

Foram conferidas as 3296 publicações medidas, em 2560 comparações com referência: texto, geometria, fontes e marcadores de época. Quarenta e oito pares de amostra tiveram todas as páginas rasterizadas a 50 dpi, com pixels iguais. Não se exigiu identidade binária de PDF. Nove watches adicionais observaram edição apenas da dependência: fan-out/nested16 Total 16 → 17 e repeated16 → 32, em baseline, diagnóstico adiado e nominal. As verificações ficaram fora dos relógios.

Builds passaram em RAM. `crystalline-lint --quiet .` passou com zero violações no checkout ativo; isso não certifica linhagem de um overlay de lab ainda sem L0 produtivo. Os arquivos recebidos e o diffstat original foram preservados. Executáveis, caches, logs e mutantes ficaram somente na área RAM exclusiva e foram removidos ao final.

Dados numéricos e o controle de parágrafos: [stdlib-residual-2026-11-06](../../lab/results/construction/diagnostics/stdlib-residual-2026-11-06/). CSVs de pipeline conservam instante por amostra, variante, fixture e ciclo; resumos de pares ficam separados por lote. Sondas inclusivas, calibração, microbancadas, tamanhos, alocações e contagens têm arquivos próprios. Não há logs, manifestos de execução, mutantes ou recibos persistentes.

Intervalos UTC das publicações amostradas, calculados dos próprios CSVs (não incluem aquecimentos):

- `primary`: 2026-11-06T02:42:32.709675+00:00 até 2026-11-06T02:46:10.110762+00:00.
- `probe`: 2026-11-06T02:46:16.807518+00:00 até 2026-11-06T02:47:35.936005+00:00.
- `nominal`: 2026-11-06T02:55:06.249136+00:00 até 2026-11-06T02:57:28.568512+00:00.
- `focus`: 2026-11-06T03:09:23.596309+00:00 até 2026-11-06T03:10:12.282014+00:00.
