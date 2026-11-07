# Outline aquecido: catálogo versus restante do pipeline

A confirmação sem sondas reproduziu uma regressão pequena: **+1,39% de CPU em same-bytes contra o Tekt original** e **+1,38% contra a rota original no mesmo ELF**. O perfil aponta para trabalho **fora da inicialização/validação do catálogo**; não identificou aumento relevante nesse catálogo. Eval tem o sinal mais localizado; replay físico continua hipótese, com intervalo cruzando zero. A causa interna ainda não foi provada. Nenhuma otimização foi integrada.

## Medições antes da classificação

Mesmo estado recebido da investigação anterior. **bas**: Tekt original; **off/v02**: mesmo ELF experimental, modos 0/2 do candidato anterior. CLI é controle inativo. No perfil, **b00/b01** são o original instrumentado off/on; **f00/f01**, experimental modo0 off/on; **j00/j01**, esse mesmo experimental modo2 off/on. Não há novos seletores no produto.

|Lote|Rounds × repetições|Fixtures|Pares same/edit por watch|Processos medidos|Publicações|
|---|---|---|---:|---:|---:|
|Principal, sem sondas|8×2|mínimo e outline|6|192|1.344|
|Perfil e calibração|6×2|mínimo e outline|4|288|1.440|
|Confirmação sem sondas|20×1|outline|4|120|600|

Warmups excluídos:12/24/6 processos. Rotação de fixtures, CLI/watch e variantes. Mesmo CPU0/Rayon1/stack 64MiB, epoch0, UTC, tmpfs e fontes do host aquecidas. Nenhum build, trace ou extrator concorreu com clocks. Wall é saída CLI ou nova publicação estável/EOF; warm inclui debounce/agendamento. CPU vem de wait4 CLI e deltas do relógio CPU do processo no watch. RSS é pico do processo inteiro, repetido por linha; não é pico por ciclo.

Cada processo tem vários ciclos, mas **a unidade estatística é a rodada**: médias de repetições e ciclos dentro do round antes da comparação. Não tratar seis ciclos como seis processos independentes. Absolutos são medianas dessas médias; Δ é mediana dos percentuais pareados, não razão das medianas. IC95% exploratório bootstrap por round,10.000 reamostragens/seed0, sem correção de múltiplas comparações.

O principal teve dispersão grande: CPU CLI outline/bas entre210,01 e292,98ms nas médias por round; same-bytes entre125,49 e194,84ms. É controle inativo para o candidato. v02/off same-bytes CPU:+2,10% [−3,78;+3,01]. Nenhum round foi descartado. Isso motivou a confirmação delimitada após terminar as validações offline, sem trocar CPU, ELF ou configuração. Governor observado depois do primeiro lote: powersave; não foi alterado. Não houve monitoramento de frequência por amostra, portanto não se atribui a dispersão a uma causa específica do sistema.

**Confirmação de outline**, sem sondas:

|Modo/métrica|bas ms|off ms|v02 ms|v02/bas Δ%, IC95%|v02/off Δ%, IC95%|
|---|---:|---:|---:|---|---|
|CLI CPU|214,62|214,66|217,39|+0,87 [−0,22;+1,90]|+0,22 [−0,62;+0,89]|
|same-bytes CPU|125,80|125,39|126,57|+1,39 [+0,69;+2,16]|+1,38 [+0,11;+2,80]|
|same-bytes wall|195,30|194,40|195,91|+0,83 [+0,40;+1,45]|+0,93 [+0,16;+1,65]|
|main-edit CPU|125,57|124,86|126,90|+1,16 [−0,12;+2,57]|+1,64 [+0,13;+3,97]|
|main-edit wall|194,59|194,13|195,77|+0,77 [+0,01;+1,94]|+1,10 [+0,11;+2,54]|

O sinal warm sobrevive ao contraste no mesmo ELF; uma diferença apenas entre os códigos gerados de bas/off não basta como explicação. Não há regressão de CLI estabelecida nesse controle. Startup outline continua separado: wall bas 259,41/off 258,24/v02 254,25ms; v02/bas−1,06% [−1,99;−0,14], v02/off−0,91% [−2,37;+0,67]. Não usar essa tabela para generalizar ganho de startup.

## Fontes separadas por estágio

Um coletor L3 atribui cada `catalog.initialize` a exatamente um estágio da mesma thread: font.eval/introspect/context/layout/finish/outside. Calls/ns somados dessas labels devem reproduzir initialize exatamente. RAII restaura estágios após escopos aninhados/erro; workers sem estágio ficam outside. Só depois de validar limites deriva-se `stage_without_catalog = stage − font.stage`. Isso retira **o catálogo/frescor**, não todo trabalho de shaping, seleção ou subsetting relacionado a fontes.

As **720 publicações instrumentadas** passaram no checker independente. Nos288 ciclos warm de outline observados nos três modos com sondas:

- `font.context=1`, font.eval/outside=0; um hit validado por ciclo, zero prepare/build/aquisição seletiva.
- Três tentativas contextuais: índices1,2,3 uma vez cada,4/5 zero.
- Três chamadas reais discovery e três replay da fachada `layout_with_page_running`; seis Complete, zero Pending. Nenhum aumento no número dessas tentativas/chamadas.
- Same-bytes: zero Source constructors medidos em L3; edição real: um. Construção inclui parse, não mede parse puro nem todo possível construtor interno de outros programas.

**Outline usa o canal físico dentro de context_stabilization.** O helper legado layout.round/passN não foi chamado. O primeiro smoke identificou essa lacuna; a instrumentação foi ampliada para page_running antes dos lotes. Não houve uso dos tempos de smoke nas conclusões. `pipeline.layout≈0,04ms` é apenas a fase ordinária posterior, e não significa que o documento fez layout em0,04ms.

Fases same-bytes, **off/on do candidato no mesmo ELF instrumentado** (f01/j01): medianas de médias por round; Δ pareado em ms. Timers físicos são inclusivos e sobrepõem catálogo/contexto; não somar a tabela como partição.

|Escopo|off ms|v02 ms|Δ pareado ms, IC95%|
|---|---:|---:|---|
|Catálogo/frescor|12,334|12,347|+0,013 [−0,184;+0,380]|
|Eval, sem catálogo|23,738|24,730|+0,578 [+0,046;+1,643]|
|Contexto, sem catálogo|58,031|58,401|+0,906 [−0,671;+3,111]|
|Paged inteiro, sem catálogo|87,845|89,241|+1,552 [−0,909;+4,996]|
|Discovery físico, inclusivo|36,440|36,258|−0,199 [−0,548;+1,328]|
|Replay físico, inclusivo|24,701|25,078|+0,747 [−0,358;+1,877]|
|Exportação PDF|20,113|20,096|−0,017 [−0,281;+0,794]|

Contra o original instrumentado, catálogo12,360→12,347ms e Δ pareado+0,0002 [−0,335;+0,421]; eval+0,657 [+0,211;+1,906]. Atribuição observada coloca a primeira inicialização/validação de catálogo **depois de eval**, dentro da primeira tentativa contextual. Logo, as consultas desse ciclo não explicam diretamente o sinal anterior em eval. Estado retido/allocações/cache após o cold path são hipóteses, não causas demonstradas.

Detalhe de fontes f01/j01, ms: fast.config2,495/2,481; roots_recheck7,117/7,110; fast.verify9,637/9,619; GenerationKey2,691/2,722. São inclusivos, não parcelas para somar indiscriminadamente. Tentativas contextuais1/2/3 f01:19,200/26,574/24,083ms; j01:19,168/26,734/24,644ms. A primeira inclui fontes. Os intervalos mais amplos impedem culpar definitivamente a terceira tentativa/replay.

**Calibração:** no ELF conjunto, CPU same-bytes on/off+0,61% [−0,80;+3,93]; main-edit−0,21% [−0,49;+0,50]. No original, same-bytes+0,11% [−1,57;+2,90]. Flush fica fora dos escopos de fontes, na cauda de publicação, mas dentro do wall do watch. Instrumentação/incerteza pode alcançar a ordem do residual. Tempos sem sondas sustentam a regressão; fases sustentam sua localização aproximada e hipóteses, não uma função causal definitiva.

## Consultas de filesystem

Dois traces independentes, fora dos clocks, quatro ciclos warm por variante em outline. Intervalos delimitados pela escrita da entrada e novo PDF, usando timestamps do trace; sem usar tempo sob strace como benchmark. Contagens totais do processo no intervalo, não somente fontes:

|Rota, por ciclo|statx|openat|read|getdents64|pread64|
|---|---:|---:|---:|---:|---:|
|bas/off|3.924|115|226|2|0|
|v02|3.936|115|226|2|0|

Os quatro ciclos de cada execução e a repetição deram as mesmas contagens. São **12 statx extras**, não uma nova descoberta de fontes nem aquisição de bytes. Após normalizar aliases pelo destino canônico, o saldo é+2 em cada um de seis diretórios: MathJax otf e Poppler Adobe-CNS1/GB1/Japan1/Japan2/Korea1. Trocas de paths lexicais/canônicos nas fontes MathJax cancelam na contagem normalizada; os caminhos observados não foram descartados da prova. Essa normalização é análise posterior do trace, não alteração do contrato.

Esse custo adicional é real, mas os relógios do catálogo não mostram aumento capaz de atribuir a ele a regressão total. Remover uma evidência/guarda para corrigir o residual não é sustentado por esta rodada.

## Delimitação após medir

A regressão warm foi reproduzida em magnitude menor que os2% anteriores. **O catálogo não é o componente que os dados apontam como explicação principal.** O alvo seguinte deve ser o estado/custo de eval e o replay físico, preservando as fontes. Não há evidência de mais tentativas nem reparse em same-bytes. A causa por função/linha e a contribuição de alocação, cache e estado retido continuam **Unknown**; diferenças de medianas ou símbolos não bastam para prová-las.

Próximo experimento delimitado, ainda lab: contar alocações/bytes por eval e replay em bancada separada; comparar uma descoberta conjunta neutra que observa mas conserva a geração original, antes de alterar qualquer mecanismo. Esse controle testa se o estado deixado pelo cold path, em vez da validação corrente, reproduz o sinal. Só propor mudança depois de contraste pareado, preservação de saída e delimitação dos owners L0. Não implementar agora memoização, arena, dedup de evidências ou alteração de budgets/frescor.

## Verificações e estado medido

Skill Tekt: A implementou; B escreveu testes independentes; C compilou/executou. **19 testes Python e dois Rust** passaram, incluindo subprocessos do coletor off/on, restauração aninhada/unwind/thread, igualdade de atribuição, schema/ciclos/CPU/marcadores e publicação antiga/incompleta. **3/3 mutações dirigidas rejeitadas por execução**: atribuição de fontes errada, perda de restore (ambas compilaram) e aceitação de publicação antiga no driver. O checker também rejeitou um trace real com outcomes físicos removidos. Patcher recusou checkout, frozen e reaplicação. Falha inicial de import do oracle na cópia de testes foi corrigida copiando o teste final com caminho read-only explícito, sem mudar asserções.

Harness público de SystemWorld/FontCatalogSession: **30 cenários** passaram em original0 e experimental0/2, config explícita e system-roots. Catálogo/ordem completos e bytes/mundos retidos das famílias controladas comparados com builder fresco. Sem prometer snapshot imutável de mmap antigo após escrita in-place. Engine e guardas do candidato anterior foram reutilizados; não se repetiu sua suíte completa de 45 testes.

**3.384 PDFs medidos** coincidiram em texto/bbox/páginas/fontes nos 2.496 pares comparados; marcadores alternados foram checados offline pelo validador independente. **30 pares rasterizados, 492 páginas renderizadas**, coincidiram a 50dpi. Valida o estado recebido, não nova paridade com vanilla. Seis builds release passaram: original/conjunto, sondas gerais e complemento físico em ambos. `crystalline-lint --quiet .`: zero violações no checkout preservado; não certifica os overlays lab. Sem suíte completa core/infra.

HEAD `3c6b0ce7b7632cc74b004cea6681e4746e63d28d`, **uncommitted working tree**, snapshot em `2026-11-06T11:38:41.568241+00:00`. Todos os 53.158 arquivos recebidos foram conferidos e preservados. Diffstat original permaneceu idêntico à [lista exata de 184 arquivos/8.388 inserções/537 remoções](stdlib-reuse-2026-11-05.md#diffstat-exato-do-estado-recebido). Novos arquivos limitam-se a [outline-warm-residual](../../lab/experiments/outline-warm-residual/README.md), este diagnóstico e medições/fixtures de domínio sob resultados ignorados. Nenhum produto, L0, experimento anterior, staging ou commit foi alterado.

Rust 1.92.0; `.cargo/config.toml` inteiro preservado, inclusive `-C llvm-args=-align-all-functions=3`; release opt-level=s/thin LTO/codegen-units1 e exceção zune-jpeg3 recebida. `cargo build --offline --locked --release -p typst-wiring --bin typst --message-format=json`, jobs1/incremental0/SHA do HEAD explícito. Apenas path do fontdb0.23/crate lab/aresta infra no lock; nenhuma atualização de versões. Cada build sob MemoryHigh/Max6GiB, swap0; no máximo dois builds simultâneos. Todos os sources copiados/targets/tmp/executáveis/testes/logs/traces/mutantes/PDFs em RAM exclusiva `/dev/shm/tekt-outline-warm-20261106`; cache anterior somente copiado e preservado.

ELFs sem sondas retidos antes de instrumentar. Builds original/conjunto concluídos antes de 11:43:55.427874Z/11:49:57.182077Z. Sondas físicas instaladas apenas nas cópias RAM em 11:54:18.841655Z/11:56:38.798898Z; builds finais antes de 11:58:03.862287Z/12:00:09.771465Z. Não houve mudança de código em nenhum lote. Horários de início da primeira e da última amostra medida, UTC 2026-11-06:

- Principal:12:02:29.569271Z–12:06:41.322680Z.
- Perfil/calibração:12:07:02.768821Z–12:11:16.923334Z.
- Confirmação:12:15:47.608203Z–12:18:08.574444Z.

Dados em [outline-warm-residual-2026-11-06](../../lab/results/construction/diagnostics/outline-warm-residual-2026-11-06/): `*-samples.csv`, resumos pareados/primeiro-versus-posteriores, phases/phase-summary/phase-pairs/cycle-summary, consultas e diferenças canônicas; fixtures exatas em inputs. Fontes do host não tiveram inventário atômico antes/depois; outputs, ramos observados e cenários privados são os controles.

Reprodução: novo patcher `apply.py` instala variantes base/joint e sondas; `--physical-probes` é complemento separado antes dos lotes. `measure.py --fixtures inputs --ram-dir RAM_NOVA --rounds N --repeats R --cycles P --variant bas=ELF:0 --variant off=ELF_CONJUNTO:0 --variant v02=ELF_CONJUNTO:2 --fixture outline.typ` reproduz confirmação com N20/R1/P4. Principal acrescenta imports_00 com N8/R2/P6. Perfil usa os dois ELFs instrumentados on/off com N6/R2/P4 e `--probe-variant`. Checkers e instruções em tests/README.md. O leitor final de traces gerou inadvertidamente dois caches .pyc fora da RAM; ambos foram removidos na conferência, sem alterar fontes. Use python -B também nos leitores auxiliares. Sem lockfile experimental, logs, manifests, traces brutos, executáveis, bytecode ou mutantes persistidos no repositório. RAM exclusiva removida após a conferência final; nenhuma outra bancada é limpa.
