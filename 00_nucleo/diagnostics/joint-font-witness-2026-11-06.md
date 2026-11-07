# Catálogo e evidências na descoberta: captura mínima

Experimento de laboratório sobre a mesma árvore recebida. Nenhuma otimização foi integrada ao produto ou aos L0. A revisão usa o percurso nativo observado do protótipo anterior, mas elimina uma consulta de captura redundante para paths que não são links. Código e testes: [joint-font-witness](../../lab/experiments/joint-font-witness/README.md).

## Medições

Quatro controles: **bas** é o Tekt recebido; **off** é o ELF experimental com a rota original; **v01** seleciona a captura anterior; **v02** seleciona a captura mínima. off/v01/v02 usam o mesmo ELF. Valores de `TEKT_JOINT_MODE` 0/1/2 existem apenas no laboratório. bas também recebe 0. A CLI permanece na descoberta direta em todos os modos; serve de controle inativo. O novo candidato atua na primeira geração da sessão de fontes do watch.

Lote principal: 12 rounds × duas repetições × quatro fixtures × quatro variantes; **768 processos, 1.536 publicações**. Cada watch inclui startup, escrita dos mesmos bytes e edição real. Confirmação motivada pelo sinal aquecido de outline: oito rounds × duas repetições, mínimo/outline e bas/off/v02; **192 processos, 384 publicações**. Calibração separada: oito rounds × duas repetições, mínimo/outline, mesmo ELF com modos 0/1/2 e sondas off/on; **384 processos, 768 publicações**. Warmups separados e excluídos: 32/12/24, respectivamente. Ordens de fixtures, modos e variantes rotacionadas.

Wall até saída na CLI ou PDF novo e estável com EOF no watch. CPU: wait4 na CLI, deltas do relógio CPU do processo entre publicações no watch. RSS é o pico do processo inteiro, repetido nas linhas de watch; não é pico por fase. Poll 1 ms; settle 30 ms fora dos intervalos; same-bytes/edição incluem debounce e agendamento. Processo novo e fontes do host com page cache aquecido; não é boot frio da máquina. CPU0, Rayon1, stack64MiB, epoch0, UTC. Sem builds, traces ou extratores concorrentes nos lotes.

A tabela usa medianas das médias de dois processos por round. Δ é a mediana das razões pareadas por round; não é a razão das medianas absolutas. IC95% exploratório: bootstrap da mediana pareada, 10.000 reamostragens, seed0, sem correção de comparações múltiplas.

|Fixture|Modo|Tekt ms|Candidato ms|Δ contra Tekt, IC95%|Δ contra desligado, IC95%|
|---|---|---:|---:|---|---|
|imports_00.typ|cli|96.52|96.43|-0.01% [-0.97, +0.44]|+0.70% [-0.04, +1.18]|
|imports_00.typ|startup|139.04|130.43|-5.85% [-6.52, -5.68]|-7.31% [-7.47, -6.75]|
|paragraph.typ|cli|139.83|139.49|-0.36% [-1.31, +0.23]|-0.16% [-0.88, +0.94]|
|paragraph.typ|startup|182.69|170.72|-5.25% [-6.19, -4.20]|-4.88% [-5.37, -4.57]|
|imports_16.typ|cli|110.69|110.73|+0.11% [-0.30, +0.46]|-0.04% [-0.25, +0.25]|
|imports_16.typ|startup|153.42|145.96|-5.03% [-5.38, -4.35]|-5.24% [-5.95, -4.29]|
|outline.typ|cli|218.32|219.30|+0.91% [+0.15, +1.47]|+0.47% [-0.22, +0.58]|
|outline.typ|startup|260.16|255.10|-1.78% [-2.39, -0.59]|-1.37% [-1.95, -0.45]|

Na entrada mínima, CPU do startup também cai de 136,68 para 128,68 ms: −5,78% [−6,85; −5,62]. A confirmação sem sondas repetiu o startup mínimo: **140,60→130,79 ms**, −6,10% [−8,15; −5,02] contra bas; −6,44% [−7,85; −5,15] contra off. Sua CLI: 95,99→96,16 ms, +0,45% [−1,20; +0,99]. Não há ganho de CLI demonstrado.

**Outline não confirmou ganho de startup:** no segundo lote, medianas 257,82→261,55 ms e Δ pareado +0,27% [−1,88; +1,55] contra bas; −0,88% [−2,32; +0,96] contra off. Não juntar os lotes para esconder essa divergência. No same-bytes de outline, v02/bas foi +2,10% [+1,59; +3,46] no principal e +1,80% [+0,85; +2,90] na confirmação. Contra off, a confirmação foi +0,64% [−2,27; +1,53]. A entrada mínima aquecida também tem sinais pequenos positivos contra bas na confirmação. Isso impede declarar ausência geral de regressão. Não foi estabelecida a causa desses resíduos.

## Consultas e fases

Strace separado, uma execução de cada rota na entrada mínima, até startup publicado e encerramento controlado; contagens de kernel, sem usar a duração sob strace como benchmark:

|Rota|getdents64|statx|openat|read|pread64|
|---|---:|---:|---:|---:|---:|
|CLI, todos os modos|144|4.678|2.334|119|11.182|
|Startup bas/off|290|19.342|2.684|673|11.182|
|Startup v01|148|19.779|2.668|783|11.182|
|Startup v02|148|17.521|2.668|783|11.182|

v02 elimina **142 getdents64 e 1.821 statx líquidos** contra o original. Contra v01, são exatamente **2.258 statx a menos**. Permanecem 110 reads extras e a mesma quantidade de pread64; este candidato não funde as leituras/extratores das fontes. Não contar menos enumerações como ganho de tempo por si só.

As 384 publicações medidas com sondas passaram pelo validador independente de schema, hierarquia inclusiva, contagens e barreiras. Nas 64 inicializações conjuntas medidas (32 por modo), houve uma descoberta conjunta aceita, G1/G2/G3 e política; nenhum fallback. No controle, uma `fast.walk_roots`; nos conjuntos, zero. Os ciclos seguintes entram na rota standard e reutilizam com validação, sobre a chave produzida pela captura conjunta.

Por descoberta, ambos os modos fizeram **2.452 tentativas lstat** na captura. As tentativas stat caíram de **2.451 para 193**. São contadores de operações efetivamente tentadas pelo engine durante captura, não contagens do kernel e não incluem as validações posteriores.

Fases do startup mínimo: medianas das médias por round, ms. Clocks inclusivos, com sobreposição; **não somar as linhas como partição**.

|Escopo|Original no ELF de sondas|v01|v02|
|---|---:|---:|---:|
|Catálogo inteiro|133,79|130,63|126,73|
|Percurso independente `fast.walk_roots`|21,73|0|0|
|Descoberta fontdb original|37,71|—|—|
|Captura conjunta completa|—|60,68|55,58|
|Descoberta observada, incluindo callbacks|—|52,27|47,17|
|Selfcheck da evidência|—|7,89|7,91|
|G1 / prepare_from|—|6,29|6,29|
|G2|—|2,89|2,85|
|G3|—|3,10|3,19|
|Política final, com reparse|—|9,86|10,28|
|Aquisição seletiva + cobertura|48,99|44,07|44,43|

A economia local mais clara da revisão é captura v01→v02, cerca de **5,1 ms**, com selfcheck e demais barreiras próximos. A aquisição/cobertura continua grande. Sua diferença entre original e rota conjunta não é uma otimização do extrator: o código permaneceu original; cache/estado de entrada/codegen podem influenciar o número. Não atribuir todo o ganho líquido aos statx removidos.

As sondas não são gratuitas. No mesmo ELF v02, startup mínimo on/off deu +1,56% [+0,63; +2,50]; em outline, −0,002% [−0,77; +0,70]. Há também ruído em controles inativos. O flush ocorre na cauda de publicação, fora dos escopos de fontes, mas dentro do wall do watch. Portanto os tempos sem sondas governam a decisão; as fases localizam custos grandes. No same-bytes de outline, catálogo original/v02 ficou 12,88/13,01 ms, enquanto `pipeline.paged` inclusivo ficou 107,44/111,97 ms. Isso não prova a causa da regressão nem autoriza retirar uma validação de fontes.

## Correção e candidato delimitado

Para path que não é symlink, a captura usa o mesmo resultado de lstat para identidade lexical e alvo. Links continuam consultando o destino separadamente. **validate mantém lstat e stat independentes**, inclusive nos negativos. Preservam-se oito campos Unix, inode/ctime/modo/nanossegundos, caminhos lexicais/canônicos, limites, contexto, erros, catálogo/ordem/TTC e descoberta nativa. Há um único percurso de diretórios para descoberta e captura; continuam existindo observações e validações posteriores.

O controlador real `establish`, usado pelo overlay da sessão, executa descoberta/selfcheck/G1→G2→key→build→G3→política. Só publica o conjunto completo depois de todos os checks. Warm/refusal/poison soltam o lock antes de retornar à rota original. A evidência observada continua sendo um fragmento: **includes opcionais ausentes e novos membros de diretórios de configuração exigem o reparse Fontconfig original**. Nenhuma barreira foi dispensada; não há promessa de snapshot atômico nem de detectar alterações que preservem todos os metadados do contrato atual.

Skill Tekt: A escreveu implementação; B produziu testes sem consultar A; executor C integrou e executou. Passaram **29 testes herdados + 12 dos modos de captura + quatro do controlador = 45**. Incluem catálogo/ordem/TTC, não UTF-8, links/dangling/ciclos, inválidas→válidas, tamanho+mtime restaurados com ctime diferente, permissões, limites e mutações nas fronteiras. Dois erros iniciais eram fixtures incompatíveis com o contrato: arquivo comum não retido passou a candidato `.ttf` inválido; FD para escrita foi aberto antes de chmod0400. As asserções de frescor e contexto foram preservadas.

**Seis mutações dirigidas mortas / seis tentadas:** ignorar ctime, omitir evidências, omitir selfcheck e retirar G2, G3 ou política. Todas compilaram e foram rejeitadas por teste executado, não por erro de compilação. As mutações das barreiras exercitam o controlador efetivamente usado pelo overlay, com predicados controlados; não são seis ensaios completos de processos watch.

O harness público com SystemWorld/FontCatalogSession passou em bas0 e diag0/1/2, configurações explícita e system-roots: cinco cenários por combinação, **40 cenários**, repetidos também após compilar sondas. Compara FontInfo completo/ordem com builder fresco; verifica bytes/mundos retidos nas famílias controladas. Não exige imutabilidade de mmap antigo após escrita in-place. Elegibilidade da rota conjunta foi observada nas fases do pipeline do host; não se infere elegibilidade de cada cenário público apenas por passar.

Os seis testes do checker de fases passaram. **2.688 PDFs medidos** foram comparados em texto/bbox/páginas/fontes, com marcadores distintos por época; **32 pares rasterizados, todas as 376 páginas renderizadas**, coincidiram a 50dpi. Isso valida preservação do estado recebido, não nova paridade com vanilla. Builds release original, candidato e sondas passaram; `crystalline-lint --quiet .` terminou com zero violações no checkout. Não foi executada a suíte completa core/infra; o lint do produto preservado não certifica linhagem dos overlays lab.

**Decisão:** manter em laboratório. Há candidato quantitativamente sustentado para o **cold path elegível Linux da sessão do watch**, sobretudo entrada curta, com preservação das guardas nos controles. Não há candidato de ganho para CLI nesta revisão e não há ganho geral consolidado para outline. Antes de propor integração: isolar a regressão aquecida contra o ELF original/controle e confirmar o alcance em diferentes catálogos/configurações, mantendo reparse e todas as barreiras. Só então atualizar os L0 owners de geração/sessão/fontdb e revalidar conforme ADR-0127; este experimento não autoriza essa integração.

## Estado e reprodução

HEAD `3c6b0ce7b7632cc74b004cea6681e4746e63d28d`, **uncommitted working tree**, congelado em `2026-11-06T10:12:18.377815+00:00`. Os 53.133 arquivos recebidos foram conferidos; nenhum foi alterado. O diffstat original permaneceu idêntico à [lista exata de 184 arquivos, 8.388 inserções/537 remoções](stdlib-reuse-2026-11-05.md#diffstat-exato-do-estado-recebido). Novos arquivos desta tarefa limitam-se ao diretório do experimento, este diagnóstico e dados de domínio sob resultados ignorados; sem staging/commit.

As cópias baseline/diag preservaram `.cargo/config.toml` **inteiro**, incluindo `-C llvm-args=-align-all-functions=3` recebido. Rust1.92.0; release opt-level=s, thin LTO, codegen-units1 e exceção recebida zune-jpeg3. `cargo build --offline --locked --release -p typst-wiring --bin typst --message-format=json`, jobs1, incremental0, SHA do HEAD explícito. Lock delta restrito à crate lab, aresta infra e path do fontdb0.23; nenhuma versão dependente atualizada. RAM exclusiva `/dev/shm/tekt-joint-witness-20261106`: fontes copiadas, targets/caches, tmp, executáveis, PDFs, traces, logs e mutantes. Builds com teto conjunto de até12GiB/swap0; etapas paralelas limitadas a6GiB cada. Cache anterior recebido foi apenas copiado e preservado.

Original compilado antes de `10:17:00.415858Z`; candidato antes de `10:28:47.610249Z`. Sondas alteraram apenas as cópias RAM em `10:41:36.103259Z`/`10:41:36.104751Z`; rebuild concluído antes de `10:45:20.750120Z`. Os ELFs sem sondas foram retidos. Não houve mudança de código dentro de lote. Intervalos abaixo abrangem o início da primeira à última amostra medida (warmups excluídos):

- Principal: `2026-11-06T10:36:51.661680Z` a `10:41:20.849309Z`.
- Calibração/fases: `10:46:24.976072Z` a `10:48:53.746267Z`.
- Confirmação: `10:48:58.584141Z` a `10:50:11.923432Z`.

Fixtures exatas e CSVs ficam em [resultados](../../lab/results/construction/diagnostics/joint-font-witness-2026-11-06/): `inputs/`, `primary-samples.csv`, `confirmation-samples.csv`, `profile-samples.csv`, resumos `*-vs-*.csv`, `phases.csv`, `phase-summary.csv` e `syscalls.csv`. Sem inventário atômico completo das fontes do host antes/depois; PDFs, descoberta nativa e cenários privados são os controles observados.

`apply_overlay.py` e `apply_probes.py` só aceitam cópias RAM preparadas; instruções em README. `measure.py --fixtures inputs --ram-dir RAM_NOVA --rounds N --repeats 2 --variant bas=ELF:0 --variant off=ELF_EXPERIMENTAL:0 --variant v01=ELF_EXPERIMENTAL:1 --variant v02=ELF_EXPERIMENTAL:2 --fixture imports_00.typ --fixture paragraph.typ --fixture imports_16.typ --fixture outline.typ` reproduz o principal. Compilações e Cargo tests são feitos dentro da RAM; harness público recebe externs dos artefatos do build; testes engine selecionam adversarial/capture_modes/establish_contract. Instrumentação usa três modos com sondas off/on no mesmo ELF e `--probe-variant`; checker independente valida cada ciclo. Não há Cargo.lock lab persistido, trace bruto, log, executável ou mutante no repositório.

A RAM exclusiva da tarefa é removida após a conferência final; nenhuma outra bancada é limpa.
