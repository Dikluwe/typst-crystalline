# Startup do watch: controle neutro e fases de fontes

O residual pequeno sem imports não se mostrou estável na confirmação. O controle neutro e o adiado ficaram indistinguíveis; não foi demonstrado custo causal da captura, dos 16 bytes adicionais ou da lógica de reúso. O custo grande foi localizado: descoberta e aquisição de fontes, mais o trabalho de estabelecer/revalidar a prova de frescor no startup. Nenhuma otimização foi integrada.

## Estado e correção da receita

HEAD `3c6b0ce7b7632cc74b004cea6681e4746e63d28d`, working tree não commitada, congelada em `2026-11-06T03:17:30.388071+00:00`. O diffstat exato é idêntico ao [apêndice integral anterior](stdlib-reuse-2026-11-05.md#diffstat-exato-do-estado-recebido): 184 arquivos, 8388 inserções e 537 remoções. Os 53.117 arquivos recebidos foram conferidos por conteúdo e preservados. Produto, L0, configuração Cargo e experimentos anteriores não foram editados. Código/testes novos ficam em [watch-startup-neutral](../../lab/experiments/watch-startup-neutral/).

Foi encontrada uma falha na bancada anterior: a cópia RAM omitia `.cargo/config.toml`. Esse arquivo, linha 16, aplica `-C llvm-args=-align-all-functions=3` — alinhamento de funções de 8 bytes — já exigido pelo L0 `00_nucleo/prompts/wiring/main.md:431`. O L0 estava correto; a receita experimental estava incompleta. Os números anteriores qualificam sua receita sem configuração, não a configuração ativa do produto. Isso não prova que a omissão causou o residual anterior.

Nesta rodada há duas receitas separadas: **ativa**, com `.cargo` copiado e apenas `CARGO_TARGET_DIR` direcionado à RAM; **histórica**, deliberadamente sem essa configuração. Os fingerprints Cargo de Core, Infra, comemo e ecow confirmaram a flag nas três variantes ativas e sua ausência nas históricas. Não se aplicou a flag somente ao link final. Nenhuma configuração produtiva foi alterada.

Rust 1.92.0, release s/ThinLTO/uma unidade, override existente de zune-jpeg, Cargo offline/locked, um job por build. Compilações e validadores tiveram limites próprios dentro do teto conjunto de 12 GiB, swap zero. Tudo descartável ficou em `/dev/shm/tekt-watch-startup-20261106`; cache anterior somente copiado, nunca alterado. O skill Tekt foi usado com A escrevendo overlays e B testes sem receber a solução de A; C integrou e executou. A separação foi de contexto de autoria, não de filesystem.

## Contrafactual estático

- **Original:** código recebido.
- **Neutro:** mesmo helper privado, mesmo campo/tipo/posição textual e inicialização None do adiado em EvalContext, sem ler/popular o slot e sem alterar root/imports.
- **Adiado:** candidato nominal anterior, com criação do template apenas no primeiro import; helper byte a byte igual ao anterior.

Não há seletor, método extra em World, ambiente, relógio ou estado global em L1. O neutro isola o pacote de representação/contexto e grafo privado de código; não é uma prova de que somente 16 bytes explicam uma latência.

Nas duas receitas, quatro avaliações sem imports fizeram exatamente 27.940 alocações e solicitaram 8.345.276 bytes nas três variantes. EvalContext mede 2976 bytes no original e 2992 no neutro/adiado; Scope/Module não mudaram. Pedidos de alloc/realloc não são RSS, bytes copiados ou memória simultaneamente viva.

Contagens GDB nos seis ELFs sem sondas confirmaram: zero imports tem um builder e uma consulta World.inputs; com 16, original/neutro têm 17 builders e o adiado dois, todos com 17 consultas. No watch com 16, inputs e Sources somam 17/34/51, builders original/neutro 17/34/51 e adiado 2/4/6; convergência/Layouter 1/2/3 em todos. No zero, Sources somam 1/1/2 por retenção dos mesmos bytes. As contagens são execução observada, não fórmulas preenchidas por expectativa.

## Tempos sem sondas

Lote principal: seis variantes, três fixtures (mínimo sem imports, parágrafos sem imports, fan-out16), 12 rodadas × duas repetições. Confirmação: receita ativa, duas fixtures sem imports, 18 rodadas × uma repetição; inclui também os três ELFs com sondas desligadas. Binários sem sondas foram retidos antes de instalar instrumentação L3. Não houve alteração de código durante nenhum lote.

CPU fixado no primeiro permitido (0), Rayon 1, stack 64 MiB, epoch 0, UTC e fontes padrão do sistema. Startup é de processo novo, com caches do sistema naturalmente aquecidos, não I/O de disco frio. CLI usa wait4; watch termina o intervalo em nova publicação estável com EOF e usa CPU por clock do processo. Wall quente inclui eventos/debounce. Builds, contadores e extratores não concorreram com os relógios.

Absolutos são medianas das médias de repetições por rodada. Percentuais/intervalos usam deltas pareados por rodada e bootstrap de medianas, 10 mil reamostragens, seed 0. Não se combinaram lotes para ocultar resultados positivos. Intervalos descrevem esta amostra, não garantem custo zero.

Controle mínimo, receita ativa:

|Lote|Comparação|Startup baseline → variante, ms|Delta pareado, % e intervalo|
|---|---|---:|---:|
|Principal|Original → neutro|138,11 → 139,01|+0,92 [0,12; 1,17]|
|Principal|Original → adiado|138,11 → 138,90|+0,71 [−0,18; 1,05]|
|Principal|Neutro → adiado|139,01 → 138,90|−0,20 [−0,78; 0,52]|
|Confirmação|Original → neutro|139,57 → 138,83|−0,22 [−0,99; 0,65]|
|Confirmação|Original → adiado|139,57 → 138,87|−0,51 [−0,80; 0,25]|
|Confirmação|Neutro → adiado|138,83 → 138,87|−0,014 [−0,61; 0,66]|

CLI zero também ficou inconclusiva: principal adiado +0,34% [−1,02; 0,79], confirmação +0,23% [−0,09; 1,03]. Na receita histórica, startup adiado +0,15% [−0,28; 0,88] e neutro −0,36% [−0,95; 0,30]. O +2,11% do lote anterior continua registrado como resultado daquele ELF/lote; não foi reproduzido como penalidade estável aqui, mesmo na receita histórica.

O controle positivo com 16 imports preservou benefício na receita ativa: CLI 110,29 → 102,54 ms, −6,97% [−7,58; −6,14]; startup 153,81 → 144,86 ms, −5,66% [−6,35; −5,40]; CPU da edição quente 33,78 → 26,69 ms, −21,21%. O neutro continua reconstruindo por arquivo. Os CSVs conservam todos os controles e ciclos, incluindo parágrafos.

## Fases reais de fontes

Evidência de código antes da interpretação: `font_session.rs:66` coleta a prova antes de prepare; `:72` revalida o preparado; `:85–86` constrói catálogo e revalida key/prova. `fontdb/generation.rs:174` cria fast_key, `:193` processa configuração, `:380` descobre, `:389` coleta descritores e `:465` revalida identidades. `fontdb.rs:65` adquire metadados/cobertura por face. São etapas distintas já presentes no produto.

Mesmo overlay L3 nas três variantes ativas, caminho PDF padrão, sem --timings-json/oracle. Oito rodadas × duas repetições, sondas ligadas/desligadas no mesmo ELF. Relógios agregam operações inteiras, nunca faces/paths individuais. Labels são inclusivos e vários têm mais de um pai; **não somar linhas sobrepostas** como uma partição.

Original, controle mínimo; mediana das médias por rodada, ms:

|Operação inclusiva|CLI|Startup watch|
|---|---:|---:|
|Inicialização completa do catálogo|86,11|129,92|
|Descoberta fontdb|37,11|36,56|
|Aquisição seletiva, cobertura e inserção|48,49|47,45|
|Fontes embutidas|0,35|0,35|
|Composição do catálogo|0,18|0,14|
|Prova inicial completa|0|23,69|
|↳ caminhada dos roots, incluindo recheck|0|20,93|
|Coleta de stamps do preparado|0|3,31|
|Rechecks GenerationKey, três chamadas|0|8,54|
|Verificação final de fast proof|0|9,62|
|Configuração, agregado de duas chamadas|0|5,10|

A sessão adiciona aproximadamente 44 ms à inicialização em relação à CLI neste lote. Caminhada/coleção/rechecagem da evidência explicam a grande ordem de custo adicional; configuração não foi o maior componente. Descoberta e aquisição continuam dominando a CLI. O tempo de espera pelo lock foi aproximadamente 0,0001 ms; não há evidência de contenção como gargalo nesta bancada serial.

Em chamadas sequenciais do startup, os scopes superiores foram: prova inicial 23,69; prepare 42,70 (inclui descoberta/coleta/primeiro key-check); validação do preparado 2,80; build 48,11 (inclui aquisição/embutidas/composição); key-check final 2,92; fast verify final 9,62 ms. Medianas separadas não constituem soma exata do total.

Na edição quente, sessão 15,40 ms: configuração 3,44, recheck dos roots 9,21 e GenerationKey 2,71 ms. Os traços observaram reuse_fast e nenhuma nova descoberta/aquisição; a validação de frescor permanece custando. No startup, observaram cold/published_new, uma descoberta, uma aquisição e três rechecks de GenerationKey nas três variantes. Nenhum caminho de frescor foi dispensado.

No startup instrumentado, sessão original/neutro/adiado foi 129,92/129,85/130,35 ms; eval 1,53/1,50/1,51 ms. Esses pontos não localizam causalmente o residual pequeno sem sondas. `source.access` pode ser hit; não se mediu parse exclusivo nem se conclui parse gratuito. O corpo de aquisição agrega também fallbacks/cobertura, não é só tempo de read_at.

## Interferência e limite causal

Ligar as sondas no mesmo ELF aumentou startup zero em +0,93% [0,25; 1,91] no original, +0,55% [0,44; 2,14] no neutro e +0,61% [−0,08; 1,63] no adiado. O flush fica fora dos scopes, mas antes da publicação final do watch; esse I/O está incluído na calibração. Apenas adicionar o código com sondas desligadas teve intervalos cruzando zero nos três contrastes da confirmação; isso não prova interferência nula.

A instrumentação tem custo/incerteza da ordem do residual. As fases sustentam a localização do custo grande; não autorizam atribuir uma diferença submilissegundo a um trecho só porque suas medianas diferem. As contagens de operações e World.inputs permaneceram iguais no zero; não foram contados syscalls individuais por fase.

**Conclusão limitada:** não foi confirmada penalidade estável da lógica de reúso sem imports. O controle neutro não distinguiu o adiado, mas tampouco provou que o crescimento do contexto causou a latência. Representação/grafo de código, disposição das instruções e variação de execução seguem hipóteses; a causa fina permanece **Unknown**. Uma atribuição exclusiva à captura exigiria uma separação que o neutro não reproduzisse; o principal não observou essa separação. Uma atribuição estável ao slot exigiria repetição, ausente na confirmação. Esses contrastes limitam hipóteses, não provam uma causa. Não declarar regressão zero nem condenar o slot só com esses lotes.

A próxima frente sustentada pelo custo é avaliar captura de evidências durante a própria descoberta e evitar observações redundantes, sob prova equivalente para diretórios, candidatos inválidos, links, configuração e identidades. Não remover rechecks por soma de tempos, nem trocar a garantia por cache nativo/eventos. Aquisição/cobertura de aproximadamente 48 ms é a outra frente grande. Nenhum novo candidato de fontes foi implementado, e este experimento não integra a stdlib ao produto: uma futura materialização continua exigindo L0 próprio e revalidação no ELF produtivo final.

## Validação e preservação

Nas seis variantes passaram os 17 controles públicos de stdlib, alocações e tamanho. Cinco cenários públicos de fontes rodaram em duas configurações por variante: isolada e padrão com HOME/.fonts na RAM. O baseline instrumentado repetiu esses controles com sondas ligadas. Os cenários cobrem adição/remoção, inválida→válida, duas fontes reais de mesmo tamanho com mtime restaurado, symlink/ausência/reparo e Worlds vivos após substituição atômica. Catálogos completos foram comparados ao builder fresco.

A primeira bancada padrão excedeu 4 GiB por copiar bytes de todas as faces/TTCs do host. Foi corrigida para comparar bytes das famílias das fixtures controladas, derivadas dos próprios metadados; catálogo completo, ordem e asserções de frescor foram mantidos. O modo isolado não mudou. Não se transformou mmap antigo em promessa de snapshot imutável após escrita in-place.

Dez testes adversários do checker passaram. Os TSVs reais foram aceitos para todos os processos/ciclos, com short-circuit/outcomes consistentes; arquivo off ausente e corrupção de label real rejeitada. Duas mutações compilaram e foram mortas por execução: neutral-allocate violou a igualdade de alocações; session-always-reuse falhou em adição de fonte. Erro de compilação não contou como morte.

Foram conferidas 3.744 publicações, em 3.120 comparações de texto/geometria/fontes/épocas. Cento e sessenta pares de amostra tiveram todas as páginas rasterizadas a 50 dpi, pixels iguais. Não se exigiu identidade de bytes PDF. O harness público auxiliar compilou e conferiu contagens de catálogo entre fresh/session; sua pequena amostra não foi usada como ganho do watch.

Builds passaram na RAM e `crystalline-lint --quiet .` passou com zero violações no checkout ativo. Isso não certifica linhagem produtiva de um overlay experimental. Arquivos recebidos/diffstat foram preservados; logs, caches, mutantes, executáveis e imagens descartáveis foram removidos somente da área RAM desta tarefa.

Dados de domínio: [watch-startup-neutral-2026-11-06](../../lab/results/construction/diagnostics/watch-startup-neutral-2026-11-06/). Receitas/lotes e calibrações permanecem separados. Fases ausentes nos CSVs normalizados têm zero chamadas; as durações são inclusivas. Não há recibos, hashes, manifestos de execução ou logs persistentes.

Intervalos UTC amostrados, dos CSVs (aquecimentos excluídos):

- `primary`: 2026-11-06T03:44:07.057562+00:00 até 2026-11-06T03:48:26.675891+00:00.
- `probes`: 2026-11-06T03:55:31.131995+00:00 até 2026-11-06T03:58:23.626202+00:00.
- `confirmation`: 2026-11-06T04:03:37.245433+00:00 até 2026-11-06T04:05:49.517886+00:00.

As únicas mudanças de fonte entre esses lotes foram overlays L3 de instrumentação nas cópias RAM, com os ELFs originais retidos. Instalações observadas pelo mtime de `03_infra/src/lib.rs`: cbs 2026-11-06T03:49:14.882226+00:00, cne 2026-11-06T03:49:14.882102+00:00, clz 2026-11-06T03:49:14.881879+00:00. Nenhuma fonte produtiva mudou entre medições.
