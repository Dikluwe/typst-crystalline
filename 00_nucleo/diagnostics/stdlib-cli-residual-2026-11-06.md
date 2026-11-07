# Residual da CLI sem imports: decisão para a stdlib adiada

**O +0,74% anterior não se reproduziu como regressão estável do candidato. Ele deixa de ser, isoladamente, um bloqueio técnico para avançar à materialização da stdlib adiada.** Não se demonstrou custo zero: continuam possíveis diferenças submilissegundo entre ELFs. O binário produtivo final deve passar novamente pelos controles. Nenhuma otimização, alteração de produto ou L0 foi integrada nesta investigação.

Esta conclusão atualiza a condição de desempenho deixada por [stdlib-qualification-2026-11-06.md](stdlib-qualification-2026-11-06.md); o relatório anterior e suas medições foram preservados. A causa fina da amostra antiga permanece **Unknown**. Os novos dados não autorizam atribuí-la especificamente a fontes, ASLR, paths ou cache de instruções.

## Estado, receitas e controles

HEAD `3c6b0ce7b7632cc74b004cea6681e4746e63d28d`, working tree não commitada, congelada em `2026-11-06T14:26:03.456799+00:00`. O `git diff HEAD --stat=240` recebido continua exatamente igual ao apêndice de [stdlib-reuse-2026-11-05.md](stdlib-reuse-2026-11-05.md): 184 arquivos, 8388 inserções e 537 remoções. Foram protegidos por conteúdo 53292 arquivos rastreados/não ignorados recebidos, incluindo os laboratórios anteriores. A fonte produtiva permaneceu fixa durante todos os relógios.

Skill [Tekt de código/testes independentes](/home/dikluwe/.codex/skills/tekt-materializacao-segregada/SKILL.md): A escreveu a bancada, B os testes sem consultar A e C executou. Fontes novas em [stdlib-cli-residual](../../lab/experiments/stdlib-cli-residual/README.md). O driver ficou na versão de `2026-11-06T14:37:12.897645+00:00`, antes dos relógios; o último teste independente de transporte ficou pronto em `14:38:19.305503+00:00`. Não houve edição do driver durante os lotes.

Compilações, caches, ELFs, testes executáveis, PDFs, traces e mutantes ficaram em `/dev/shm/tekt-stdlib-cli-residual-20261106`. Rust 1.92.0; builds release offline/locked, um job, incremental desativado, mesmo lockfile e `.cargo/config.toml` completo, incluindo alinhamento de funções a 8 bytes; perfil recebido `opt-level=s`, thin LTO e codegen-units=1, com as exceções por pacote preservadas. Workspaces/targets separados; no máximo dois builds simultâneos, cada launcher MemoryHigh/Max=6 GiB e swap=0. O cache RAM anterior foi apenas copiado. Os artefatos do core dos três builds foram efetivamente recompilados.

ELFs nominais: `bas` produto recebido, `neu` mesmo slot vazio/helper privado sem reúso, `laz` candidato adiado **inalterado**. O root conserva o builder original; somente imports podem capturar/reutilizar o template. Não há seletor de laboratório em L1, cache entre compilações ou mudança de World/layout/fontes. Não se recompilaram os ELFs entre os lotes.

CLI PDF padrão com `--pdf-tagged=false`, stack 64 MiB, CPU0, Rayon=1, epoch=0 e fontes normais do host. As listas completas de fontes coincidiram entre os três ELFs: 3203 linhas. Inputs são os mesmos arquivos ordenados da qualificação anterior: vazio e 512 pares/74734 bytes UTF-8 de chaves+valores. São processos novos com cache do sistema operacional aquecido, não uma medição de boot frio.

O novo driver chama o relógio/publicação CLI anterior, com CPU wait4, componentes usuário/sistema, wall, PID e RSS. Cada bloco percorre todas as permutações dos rótulos. Foram usados:

- **Principal:** 36 rodadas, três ELFs, sem-imports e controle positivo de 16 imports, nos dois perfis de inputs.
- **A/A:** 48 rodadas, quatro rótulos: `bas/b02` são hardlinks do mesmo ELF baseline; `laz/l02`, do mesmo candidato. Somente sem-imports, nos dois perfis.
- **Entrada compartilhada (`sh0`):** 36 rodadas, mesmo caminho absoluto de fonte, root e cwd entre todos os ELFs; output exclusivo por processo.
- **Crossover (`sh1`):** repete esse controle trocando os ELFs dos rótulos `bas/laz`; `neu` permanece neutro. Recria-se a árvore de entrada entre os blocos no mesmo caminho absoluto, antes dos warmups. A fonte não muda durante os lançamentos de cada bloco. Assim, identidade do rótulo não é confundida com identidade do binário na análise.

Esses controles não tornam a execução imune a ruído nem isolam, sozinhos, a causa de uma diferença de nanosegundos. Não se subtraiu o A/A para fabricar ganho. Os lotes nominais somaram 1248 processos medidos entre `14:39:52.984507` e `14:42:09.628554` UTC, em 2026-11-06. Nenhum build, teste, profiler, trace ou extrator de PDF correu durante esses relógios.

## Resultado principal: o residual não acompanhou o candidato

CPU pareada, candidato menos baseline, **512 inputs**; mediana por rodada e IC bootstrap de 95% (10000 reamostragens, seed=0):

| Controle | Delta em ms [IC] | Delta em % [IC] |
| --- | ---: | ---: |
| Principal, 36 rodadas | −0,036 [−0,346; +0,262] | −0,037 [−0,355; +0,276] |
| A/A, candidato vs baseline, 48 rodadas | +0,010 [−0,474; +0,514] | +0,010 [−0,498; +0,533] |
| Entrada compartilhada, 36 rodadas | −0,188 [−0,608; +0,109] | −0,197 [−0,635; +0,115] |
| Crossover, identidade real dos ELFs, 36 rodadas | +0,228 [−0,195; +0,589] | +0,241 [−0,205; +0,631] |

Sem inputs, os deltas correspondentes foram −0,107%, −0,086%, −0,063% e +0,047%, todos com intervalos abrangendo zero. Os CSVs também conservam wall; não se escolheu apenas CPU para omitir um efeito consistente em latência.

O próprio **mesmo ELF** baseline, lançado sob os rótulos A/A com 512 inputs, apresentou +0,259 ms [−0,278; +0,520], ou +0,270% [−0,284%; +0,551%]. No candidato contra ele mesmo: +0,093 ms [−0,206; +0,253]. Isso evidencia a escala de variação da bancada, mas não prova que a observação antiga era somente ruído.

Há resultados pequenos positivos que devem permanecer visíveis: no principal/512, candidato vs **neutro** deu +0,442 ms [+0,092; +0,832], ou +0,467% [+0,094%; +0,866%]. O efeito não se manteve contra esse neutro com entrada compartilhada (+0,041%, IC abrangendo zero). O neutro também possui os 16 bytes extras de contexto; portanto esses dados não sustentam culpar apenas o slot ou remover o guard de frescor da stdlib.

Os intervalos são exploratórios, sem correção de múltiplas comparações ou teste de equivalência com margem previamente aprovada. Não demonstram desempenho idêntico, nem estabelecem um limite universal de +0,63%. Demonstram que a penalidade de +0,74% não foi um efeito repetido desta reconstrução nominal.

O controle positivo continuou funcionando: com 16 imports, a CLI economizou **6,89 ms/−6,28%** sem inputs e **7,95 ms/−7,15%** com 512 inputs. No segundo caso, IC de CPU [−7,42%; −6,60%]; wall −7,10%. A economia com imports não foi usada para compensar uma regressão aceita no controle zero.

## Onde o custo pode estar — e o que não foi provado

### Avaliação sem I/O, fontes, parse, layout ou exportação

Mesmo Source já construído e mesmo World, 18 janelas independentes de processo por ELF/perfil, 200 avaliações com descarte completo por janela, aquecimento fora do relógio: 21600 avaliações cronometradas. O harness observa e verifica os pares de inputs fora da janela, e confirma uma consulta World.inputs por avaliação. Não há memoização de Module. Os relógios ocorreram entre `14:42:22.357572` e `14:42:45.319557` UTC.

| Perfil | Eval baseline → candidato, ms/chamada | Delta pareado [IC], ms |
| --- | ---: | ---: |
| Vazio | 0,8609 → 0,8635 | +0,00346 [−0,00214; +0,00464] |
| 512 pares | 1,1180 → 1,1189 | +0,00504 [−0,00379; +0,00684] |

Contra o neutro, o candidato teve um pequeno efeito positivo nesta bancada: aproximadamente +2,91 µs no vazio e +4,94 µs com 512 pares, com ICs positivos. Não foi ocultado. A magnitude medida é de **microssegundos**, sem demonstrar a penalidade antiga de cerca de 0,70 ms na CLI. Esse eval aquecido não é um limite matemático para o primeiro eval de um processo frio.

### Pipeline público cronometrado, em bancada separada

Outros 30 processos por ELF/perfil usaram a flag existente `--timings-json`, **sem sondas novas no produto**. Essa flag seleciona uma rota instrumentada distinta (`04_wiring/src/compile.rs:482`), por isso seus tempos não foram misturados aos relógios nominais. Com 512 pares:

- Eval: baseline 1,652 ms, candidato 1,661 ms; delta pareado +0,015 ms [−0,018; +0,028].
- `layout_ms`: aproximadamente 85,7/85,4 ms; delta +0,035 ms [−0,406; +0,404].
- Pipeline total: delta +0,065 ms [−0,460; +0,406]. No perfil vazio, houve total positivo +0,307 ms [+0,033; +0,498]; não foi descartado.

**`layout_ms` não significa layout puro.** O intervalo abre em `03_infra/src/pipeline/paged.rs:853`, inclui o trabalho de métricas/recursos durante layout, e fecha em `:1130`. A resolução chama `world.book()` (`font_metrics/resolution.rs:56`), que acessa o catálogo lazy (`world/world_impl.rs:23`). Não se atribuem os cerca de 85 ms ao algoritmo geométrico. Essa bancada não separa o catálogo do restante desse intervalo. `parse_ms=0` aqui significa campo não preenchido pela rota (`compile.rs:378` carrega Source antes do pipeline), **não parse gratuito**.

### Filesystem e código gerado

Fora dos relógios, duas execuções por ELF/perfil sob `strace -f -c`: todas as contagens de syscalls coincidiram, inclusive erros, dentro de cada perfil. Com 512 pares: 2334 `openat`, 4678 `statx`, 11182 `pread64`, 144 `getdents64`, 2685 `readlink`, 119 `read`. Não se observou trabalho adicional de consultas do filesystem causado pelo candidato nesses casos. Contagem igual não comprova identidade de ordem, conteúdo, page faults ou custo de cada chamada; os tempos perturbados pelo tracer não foram usados como ganho.

O controle público de tamanho confirmou `EvalContext`: **2976 → 2992 bytes**, igual em neutro/candidato. Inspeção dos ELFs mostrou diferenças reais de codegen/disposição: `EvalContext::new` tem 890/900/900 bytes (bas/neu/laz); a entrada completa de eval tem 8391/8533/8299 bytes. Assim, conservar o builder root não produz um ELF idêntico. Esses tamanhos/endereços são mecânica, não requisitos de linguagem, e **não provam causalidade de cache de instruções**. Não há medição de hardware que atribua o residual antigo a esse mecanismo.

## Correção, preservação e decisão

Os **14 testes Python** passaram, incluindo transporte compartilhado com compiler fake (não confundir com compilação Typst). Os **quatro testes Rust públicos passaram em cada um dos três ELFs**: até 4096 pares/mais de 1 MiB, ordem/último par, A/B/A entre raízes, identidades builtin/raw.line/With e features/targets. Foram mortas por asserções executadas **4/4 mutações da bancada**: omitir igualdade CPU/wait4, ignorar mtime da publicação, inverter delta pareado e não aplicar o redirecionamento para o root compartilhado. Não são quatro mutações novas do helper da stdlib; esse helper não mudou.

As 1428 publicações reais (1248 nominais + 180 diagnósticas) passaram na validação independente de frescor/marcadores e em 984 comparações entre variantes de texto, posições e fontes. Uma seleção de 18 pares/36 páginas também coincidiu em rasterização a 50 dpi. São controles de regressão no mesmo compilador, sem redefinir paridade de linguagem como igualdade dos bytes de PDF. Builds release e `crystalline-lint --quiet .` passaram; não foi executada toda a suíte core/infra nem um novo lote vanilla/watch/pacotes. A qualificação anterior desses cenários permanece separada.

**Decisão:** remover o bloqueio específico baseado em “+0,74% é penalidade reproduzida da stdlib”. Não implementar uma nova otimização para um mecanismo causal ainda não demonstrado. O candidato adiado, restrito ao registry builtin qualificado, pode avançar para L0/materialização quando isso for solicitado. Mantêm-se as fronteiras já qualificadas: root original, template apenas no primeiro import, todos os inputs ordenados/Features no guard, identidades novas por arquivo e nenhum Module avaliado compartilhado.

Isso não equivale a aceitar qualquer regressão de performance: a materialização altera o ELF. Seu gate final deve repetir o controle CLI vazio/512 com entrada compartilhada e A/A se necessário, o controle positivo de imports e os testes de pacotes/startup/watch já qualificados. Se aparecer uma penalidade positiva consistente no ELF final, ela volta a ser um item concreto a resolver ou decidir. Não é necessário continuar perseguindo o antigo ponto de +0,74% como se sua causa estivesse estabelecida.

[Medições de domínio e análises pareadas](../../lab/results/construction/diagnostics/stdlib-cli-residual-2026-11-06/) conservam CSVs, incluindo rótulos brutos do crossover e sua análise com referência correta (`sh1-{empty,large}/vs-laz.csv`). `sample_epoch` registra cada instante. Foram preservados todos os arquivos recebidos e o diffstat; os artefatos RAM próprios são descartados ao terminar, mantendo o cache RAM anterior. Nenhum hashfile, recibo de execução, log, ELF, PDF ou mutante foi adicionado ao repositório.

Conferência final em `2026-11-06T14:50:03.829727+00:00`: os 53292 arquivos protegidos mantinham seus bytes e o diffstat recebido era idêntico. O diretório RAM exclusivo desta tarefa foi removido; o cache anterior permaneceu intacto.
