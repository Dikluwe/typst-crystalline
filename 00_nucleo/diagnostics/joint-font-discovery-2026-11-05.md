# Catálogo e evidências na mesma descoberta

**Resultado:** o candidato de laboratório eliminou um percurso independente de diretórios no startup do watch, preservando as barreiras existentes nos controles executados. **Não demonstrou ganho líquido consistente no pipeline. Não integrar com base nesta rodada.** A CLI permaneceu na rota original. Código e testes persistem em `lab/experiments/joint-font-discovery/`; produto e L0 recebidos foram preservados.

## Medições antes da decisão

Três variantes: **original** é o Tekt recebido; **controle** é o binário experimental com a rota conjunta desligada; **conjunto** é esse mesmo binário com `E_JOINT_DISCOVERY=1`. Essa variável só existe no overlay de laboratório. Nenhuma flag ou caminho padrão foi acrescentado ao produto.

Doze rounds × cinco fixtures × CLI/startup × três repetições × três variantes = **1.080 processos medidos**, mais 30 warmups. Ordens de variantes e fixtures rotacionadas. Mesmo CPU0, uma thread Rayon, stack 64MiB, entradas em tmpfs, fontes do sistema mantidas. Sem builds, traces ou extratores concorrentes nos tempos. Cada startup usa processo novo. Não houve limpeza de page cache; são inicializações de processo em sistema aquecido.

Wall até saída na CLI ou primeiro PDF com trailer completo no watch. CPU é wait4 utime+stime; RSS é ru_maxrss Linux KiB×1024. Watch: poll 1ms, estabilização 30ms e SIGTERM/wait4; CPU/RSS incluem esse encerramento, wall não. Valores absolutos abaixo são medianas das 12 médias de três processos. Percentuais são calculados por round antes da mediana; não equivalem à divisão dessas medianas. Colchetes são IC95% bootstrap da mediana pareada (10.000 reamostragens, seed 0), exploratório, sem correção de comparações múltiplas.

|Fixture|Modo|Original wall ms|Controle wall ms|Conjunto wall ms|Wall conjunto/controle, IC %|Wall conjunto/original, IC %|CPU conjunto/controle, IC %|
|---|---|---:|---:|---:|---|---|---|
|plain|cli|93.87|93.78|94.09|+0.41% [-3.13, +1.02]|+0.77% [-0.22, +1.71]|+0.33% [-3.17, +1.04]|
|plain|startup|138.80|140.23|134.16|-2.90% [-4.80, -0.22]|-1.57% [-2.52, +0.88]|-2.51% [-4.73, -0.39]|
|import_syntax|cli|94.19|94.76|94.82|+0.17% [-0.76, +0.56]|+0.14% [-0.55, +0.92]|+0.24% [-0.59, +0.64]|
|import_syntax|startup|138.76|138.52|138.44|+0.54% [-0.13, +0.70]|+1.36% [-0.20, +1.80]|+0.48% [+0.03, +1.13]|
|import_compute|cli|94.39|94.91|96.05|+0.71% [-1.02, +1.28]|-0.45% [-0.99, +0.97]|+0.79% [-0.57, +1.30]|
|import_compute|startup|140.93|141.03|140.37|+0.71% [-0.75, +2.52]|-0.13% [-1.65, +0.44]|+0.80% [-0.34, +2.54]|
|layout|cli|112.58|112.42|115.16|+1.74% [+0.08, +4.28]|+1.71% [+0.53, +3.55]|+1.88% [+0.08, +4.34]|
|layout|startup|161.79|160.51|160.17|-1.39% [-3.10, +3.77]|-1.68% [-2.84, -0.07]|-1.28% [-3.08, +3.74]|
|outline|cli|214.79|207.93|214.91|+1.74% [-0.08, +3.32]|+0.06% [-0.92, +1.05]|+1.71% [-0.02, +3.31]|
|outline|startup|258.50|257.32|260.13|+1.20% [-7.04, +1.78]|+0.32% [-1.20, +1.20]|+1.26% [-6.74, +1.71]|

A entrada curta mostra diferença absoluta descritiva de 138,80→134,16ms no startup. Seu intervalo pareado contra o original cruza zero; contra o controle há redução local. Layout apresenta redução local contra o original, mas não contra o controle. Imports e outline não confirmam melhora. A rota CLI é idêntica por construção e por trace; mesmo assim layout CLI variou para pior nos tempos. Isso impede atribuir pequenas diferenças universais à descoberta conjunta. **Ganho geral permanece Unknown**; não usar a economia de enumerações como substituto de tempo medido.

RSS de mediana original/conjunto variou de aproximadamente 24–52MiB; diferenças absolutas de medianas ficaram abaixo de 0,4MiB em todas as combinações. Isso descreve este lote, não estabelece limite universal de memória. CPU, wall, RSS e intervalos completos constam nos CSVs.

## Percursos e consultas

Trace de plain separado dos relógios, original e controle idênticos:

|Modo / rota|getdents64 total|statx total|openat total|pread64 total|read total|
|---|---:|---:|---:|---:|---:|
|CLI, três variantes|144|4678|2334|11182|119|
|Startup original/controle|290|19342|2684|11182|673|
|Startup conjunto|148|19779|2668|11182|783|

São **142 getdents64 a menos**, mas **437 statx e 110 reads a mais** no startup. getdents64 conta chamadas, inclusive finais de diretório, não diretórios distintos. O subconjunto textual contendo `/fonts` e excluindo `/etc/fonts` caiu de 264 para 132 getdents64; esse filtro não representa todos os arquivos de fontes. Contagens totais e esse filtro estão em consultas.csv. O detalhamento de statx por path evita confundir menos percursos com menos verificações.

Exemplos do detalhamento: `/usr/share/fonts` passou de 9.966 para 9.372 statx; `/etc/fonts`, de 116 para 134; `/usr/share/fontconfig`, de 222 para 259. Alguns destinos canônicos MathJax passaram de 8 para 14 por path. São contagens observadas, não atribuição de tempo por fase. A coleta de caminhos auxiliares e a checagem de configuração são hipóteses a decompor, não custos medidos separadamente nesta bancada.

A descoberta observada do host teve 72 tentativas de diretório, 2.265 callbacks de entrada, 1.086 tentativas de arquivos, 2.456 visitas e 139.156 bytes de paths; catálogo de 1.112 faces e evidência aceita com 1.351 stamps, oito raízes. A sequência de todas as faces coincidiu com os métodos nativos originais em source/path, índice, famílias, estilo, peso, stretch e monospace. Isso não é uma comparação independente de toda a cobertura Unicode/FontInfo do Tekt; o extrator produtivo permaneceu inalterado.

## O que o candidato preserva e o que a evidência não prova

O fork fontdb 0.23.0 retém os métodos nativos originais como oracle e acrescenta variantes observadas. Os callbacks colhem observações lexicais e auxiliares durante o percurso nativo; preservam política, ordem, índices TTC, regras de seen e erros. Não há uma segunda read_dir para montar a prova. Limites globais e falhas de evidência recusam retenção sem truncar o catálogo nativo.

O overlay recebe a descoberta e as evidências juntas. Mantém `prepare_from` e sua autovalidação G1, a validação G2 antes de construir, G3 depois de construir e a função original `fast_key_unchanged`, incluindo **reparse efetivo da configuração**. Recusa, erro, mutex envenenado ou geração existente retornam à rota original; não se publica evidência parcial. O prefixo original inteiro de generation.rs e o prefixo nativo de fontdb foram conferidos byte a byte.

**A evidência do engine é um fragmento de entradas observadas, não uma prova completa da política Fontconfig.** Includes opcionais ausentes e novos membros de um diretório de includes fora das raízes podem mudar a política sem alterar os arquivos antes conhecidos. Dois testes explicitam que validate(fragmento) pode continuar true e um parser fresco mudar raízes/aliases/arquivos. Por isso a revalidação original de configuração é indispensável após construir e nos hits. Esses dois casos são testes de composição, não execuções reais que modificaram `/etc/fonts` durante a construção.

A garantia continua baseada nos oito campos Unix, incluindo inode, modo completo e mtime/ctime com nanossegundos, mais configuração, contexto e barreiras. Detectar alterações nos casos testados não significa snapshot atômico nem prova para qualquer sequência concorrente de filesystem.

## Verificação

Skill Tekt com A implementação, B testes independentes e C integração/execução. **29 testes adversariais passaram**, abrangendo catálogo/ordem/TTC, raízes ausentes, links/ciclos, paths não UTF-8, arquivos inválidos, limites, contexto/configuração e mutações em fronteiras da descoberta. **11/11 mutações dirigidas mortas**: oito no engine (ctime, modo, inode, nanossegundos, limite, selfcheck, bytes de config e identidade lexical) e três nas guardas G2/G3/política da sessão.

Seis controles executaram os métodos reais do overlay de sessão com serviços substituídos deterministicamente: sucesso/hit validado, recusa, G2, G3/curto-circuito, política pós-build e poison. São controles isolados, não seis testes end-to-end do produto. As três mutações das guardas foram rejeitadas nesse modelo.

Watch real, baseline e conjunto, quatro ciclos cada em HOME/XDG/fontes privados: fonte ausente→adicionada→removida→recolocada. Cada edição da entrada publicou PDF selecionando a fonte esperada. Não se exigiu que somente mudar a fonte dispare recompilação. Configuração customizada inelegível também manteve o resultado do baseline na CLI e no startup.

Trinta PDFs de smoke e **1.080 PDFs medidos** coincidiram em texto, bbox, páginas e fontes; **20 pares rasterizados a 72dpi** coincidiram em todas as páginas. Preserva o comportamento atual, inclusive outline; não é nova certificação de paridade com vanilla. Builds release original e candidato passaram, assim como `crystalline-lint .` (exit 0, notas informativas existentes). Não foi executada a suíte completa core/infra.

## Delimitação após medir

Manter o candidato em laboratório. O percurso compartilhado funciona nos controles, mas o benefício quantitativo não está consolidado e há aumento de consultas. Antes de outra implementação, decompor por barreira as chamadas extras (observação lexical/canônica, validação e configuração), usando os mesmos paths/fixtures, e definir qual conjunto de observações é necessário sem enfraquecer as guardas. Só um novo resultado líquido repetível justificará propor integração e atualizar os L0 owners. Não retirar reparse de configuração, campos de identidade ou barreiras com base neste experimento.

## Proveniência e reprodução

HEAD `3c6b0ce7b7632cc74b004cea6681e4746e63d28d`, **uncommitted working tree**. Estado recebido congelado em `2026-11-05T15:44:15.654068+00:00`; os **52.896 arquivos recebidos** foram conferidos e preservados. Diffstat rastreado permaneceu byte a byte idêntico à [lista de 182 arquivos](font-discovery-acquisition-2026-11-05.md#proveniência-e-reprodução), também usada na rodada anterior. Nenhum consumer produtivo ou L0 foi alterado. Além deste diagnóstico e CSVs de domínio, os acréscimos são exatamente:

- `lab/experiments/joint-font-discovery/Cargo.toml`
- `lab/experiments/joint-font-discovery/apply_overlay.py`
- `lab/experiments/joint-font-discovery/fixtures/DejaVuSans.LICENSE`
- `lab/experiments/joint-font-discovery/fixtures/DejaVuSans.ttf`
- `lab/experiments/joint-font-discovery/fixtures/LICENSE-NotoSans.md`
- `lab/experiments/joint-font-discovery/fixtures/NotoSans-Regular.ttf`
- `lab/experiments/joint-font-discovery/overlays/03_infra/src/fontdb/generation.rs`
- `lab/experiments/joint-font-discovery/overlays/03_infra/src/world/font_session.rs`
- `lab/experiments/joint-font-discovery/src/lib.rs`
- `lab/experiments/joint-font-discovery/tests/adversarial.rs`
- `lab/experiments/joint-font-discovery/vendor/fontdb/Cargo.toml`
- `lab/experiments/joint-font-discovery/vendor/fontdb/LICENSE`
- `lab/experiments/joint-font-discovery/vendor/fontdb/src/lib.rs`

Estado definitivo do candidato conferido contra sua cópia RAM em `2026-11-05T16:29:12.048124+00:00`. Os dois testes finais de includes foram acrescentados antes desse congelamento; não alteraram o binário release. Esta tarefa não alterou fontes, fixtures ou binários durante o lote `2026-11-05T16:29:19.433839+00:00` a `2026-11-05T16:32:21.991875+00:00`. As cópias mutadas e as fontes privadas foram usadas em controles separados, fora dos tempos. Não foi coletado um inventário completo de metadados das fontes do host antes/depois do lote; a igualdade dos PDFs e a comparação nativa são os controles observados, sem prova de snapshot atômico.

Baseline SHA-256 `902cfde0215fe0a4c5e4e9713820a1eae1b339e7aaa0d8005439849c2fdaa84c`; candidato `8bd4852c814ef0c952686da5619df9ab31eef4b7faae265beca327dc501adf10`. Rust 1.92.0, fontdb 0.23.0; dependências normais sem atualização de versões. O fork remove apenas o dev-dependency env_logger não disponível offline. Builds `cargo build --offline --locked --release --bin typst`, jobs 1, incremental 0, TYPST_COMMIT_SHA igual ao HEAD, MemoryHigh 8GiB/MemoryMax 12GiB/MemorySwapMax 0. Crate isolado testado com `cargo test --offline --test adversarial -- --test-threads=1`, TMPDIR/CARGO_TARGET_DIR em RAM; resolve a partir do lock recebido. Sem Cargo.lock experimental persistente.

Mesmas [fixtures herdadas](../../lab/results/construction/diagnostics/font-discovery-acquisition-2026-11-05/inputs/), `compile|watch entrada saida --root fixtures --pdf-tagged=false`, RAYON_NUM_THREADS=1, SOURCE_DATE_EPOCH=0, TZ=UTC, NO_COLOR=1. Controle com E_JOINT_DISCOVERY ausente, conjunto com valor 1. apply_overlay.py aplica somente numa cópia separada RAM; não usar no checkout produtivo. Não foi reexecutado vanilla.

Todo build, cache, PDF, trace, log, controle e mutante ficou no diretório exclusivo `/dev/shm/tekt-joint-discovery-20261105`, removido ao final. Nenhuma outra bancada foi limpa. Os dados de domínio ficam em [amostras.csv](../../lab/results/construction/diagnostics/joint-font-discovery-2026-11-05/amostras.csv), [resumo.csv](../../lab/results/construction/diagnostics/joint-font-discovery-2026-11-05/resumo.csv), [consultas.csv](../../lab/results/construction/diagnostics/joint-font-discovery-2026-11-05/consultas.csv) e [statx-por-path.csv](../../lab/results/construction/diagnostics/joint-font-discovery-2026-11-05/statx-por-path.csv), sob ignore existente de resultados, sem force-add. Sem staging ou commit.
