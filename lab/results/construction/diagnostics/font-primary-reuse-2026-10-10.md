# Reuso da resolução primária de fontes — 2026-10-10

## Resultado

**Mantido** o cache de uma entrada da cadeia primária em `FallbackFontMetrics`, compartilhado entre clones da mesma instância e vazio em cada nova instância. A entrada usa igualdade do FontList completo e da FontVariant efetiva; não usa hashes como identidade. Em miss, o algoritmo anterior resolve a cadeia e substitui a entrada somente se a resolução for completa e não-vazia. Falhas de leitura/parse continuam sendo tentadas. Os candidatos guardam somente índices e unidades por em; tamanho, eixos e identidade final da fonte permanecem no caminho atual. Adições MATH são feitas fora do cache.

Não houve assinatura pública, flag, fase ou representação do enum alterada. O carrier pertence a `03_infra/src/font_metrics.rs`; chave, entrada e algoritmo pertencem a `03_infra/src/font_metrics/resolution.rs`. Os dois L0 consomem o Núcleo `fonts/primary-resolution-cache.toml` com pin efetivo `b18027804dddf2f9eb87a3d01168d95dffaa34ad6839bad618e758ffbff080b4`.

## Medição que precedeu a intervenção

O perfil anterior em [content-cost-profile-2026-10-10.md](content-cost-profile-2026-10-10.md) mostrou 48.019 buscas no catálogo e 6.401 resoluções para uma tabela de 1.600 células; `text_edges`/`vertical_metrics` repetiam essa resolução. Os L0 foram escritos antes dos testes/implementação. A hipótese foi reduzir os scans repetidos, preservando seleção e limitando retenção a uma entrada. Os limiares foram fixados em `acceptance.json` antes do tempo pareado do candidato: ganho local de pelo menos 3%, nenhum caso com regressão robusta de 2% em tempo ou 3% em RSS e redução de pedidos de alocação nos seis alvos.

## Desempenho na mesma janela

Três rodadas, 12 pares medidos por fixture, motores embaralhados, CPU fixada e um worker. Os valores de tempo são medianas externas; porcentagens são medianas das razões pareadas, não a razão entre duas medianas.

| Caso | Antes → depois (ms) | Tempo pareado | Alocações antes → depois | Redução de pedidos |
|---|---:|---:|---:|---:|
| local-set-400 | 126,35 → 114,66 | −9,5% | 176.439 → 162.048 | −8,2% |
| local-set-1600 | 209,29 → 163,30 | **−22,3%** | 418.092 → 360.501 | **−13,8%** |
| no-fill-1600 | 193,65 → 145,14 | **−25,1%** | 342.821 → 285.230 | **−16,8%** |
| inline-set-1600 | 232,91 → 183,61 | −20,2% | 549.175 → 491.584 | −10,5% |
| families-1600 | 234,99 → 162,45 | −31,1% | 508.622 → 423.073 | −16,8% |
| variants-1600 | 999,22 → 940,34 | −6,1% | 1.202.828 → 1.134.037 | −5,7% |

Todas as medianas de rodada foram favoráveis. Em local-set-1600: −21,68%, −22,82%, −22,38%. O gate quantitativo passou sem regressões robustas nos alvos/refutadores.

O RSS não caiu de maneira demonstrada. As medianas antes/depois em local-set foram 42.150/42.252 KiB; no-fill, 38.014/38.296 KiB; inline-set, 47.676/47.576 KiB. O cache limita o número de resoluções retidas, mas isso não torna constante o tamanho da última FontList nem elimina outros stores do pipeline.

O refutador `variants-1600`, com pesos nominalmente diferentes, já tinha RSS de aproximadamente **513 MiB** antes e manteve esse nível depois (525.060/525.024 KiB). Essa retenção preexistente continua aberta; não foi atribuída ao novo cache nem corrigida por ele. O refutador de famílias alternadas também melhorou. Não há promessa de ganho para todo padrão de troca de identidade.

## Relação com vanilla

Na mesma janela, o local-set-1600 passou de **+20,25% para −6,43%** de tempo pareado contra vanilla. No-fill passou de +14,58% para −14,11%; inline-set passou de +31,30% para **+3,73%**, ainda mais lento.

Os documentos clássicos dessa matriz já tinham conteúdo/páginas e geometria próximos no perfil anterior, que registrou diferença de caixa de até 0,000024 pt e raster distinto. Esta intervenção preserva o release anterior; não fecha paridade global de render. Os controles ampliados de matemática, RTL, variações e muitas variantes também conservam diferenças anteriores contra vanilla. Em particular, `font-controls` tem tokenização/ordem de palavras extraídas distinta contra vanilla; não se certifica esse controle como linguagem plenamente equivalente entre motores. Os tempos dos refutadores descrevem principalmente a comparação Tekt anterior × candidato.

## Causa verificada

O mesmo censo GDB foi repetido no binário entregue:

| Entrada em tabela de 1.600 células | Antes | Depois |
|---|---:|---:|
| FontBook::select_pattern | 48.019 | **9.619** |
| resolve_primary | 6.401 | 6.401 |
| text_edges | 3.200 | 3.200 |
| vertical_metrics | 3.201 | 3.201 |
| layout_styled, caso local | 1.602 | 1.602 |
| layout_styled, controle | 2 | 2 |

São **38.400 buscas evitadas, cerca de 80%**, sem eliminar chamadas de métricas ou escopos de estilo. Contagens do debugger não foram usadas como tempo. A redução é coerente com a hipótese medida; não afirma que 80% do tempo total desapareceu nem que todo trabalho restante é seleção de fontes.

## Testes e preservação

- RED real: o teste de chamadas repetidas, inclusive via clone e métricas, registrava 36 consultas ao catálogo após as chamadas adicionais, contra as 6 da primeira resolução. Após o cache, permaneceu em 6.
- Quatro testes focais passaram: hits compartilhados; família/regex/ordem/variante em ambas as ordens contra instâncias frescas; face inicialmente ilegível e posterior retry, com Worlds independentes; MATH/variações aplicadas após reuso da base.
- Suíte completa de infraestrutura: **1.273 passaram, 0 falharam, 2 ignorados**. L1 não foi modificado; não se repetiu a suíte core já validada no passo anterior.
- **36 documentos** conservaram texto, palavras, páginas/tamanhos, coordenadas e raster entre os dois releases Tekt: 25 tabelas, 6 controles de origem/contexto/show/colunas/espaços, 4 controles de fontes/refutadores e 1 de efeitos/query.
- Seis sondas query(table) mantiveram contagem 1 nos dois Tekt e vanilla; o controle de efeitos manteve três ocorrências.
- `cargo build`, `cargo build --release` e `crystalline-lint .` retornaram zero. Avisos globais anteriores permanecem; o lint final não indicou V5/V15/V26 nos owners/núcleo afetados.

Uma tentativa de controle com `text(font: (regex(...): ...))` falhou também no release anterior com `font expects a string, array of strings, or dict`. O documento e o erro foram preservados fora do corpus aceito (`font-regex-constructor-preexisting.typ` e `preexisting-regex-error.txt`). O teste de regex usa FontFamily::new_regex diretamente; não se declarou essa superfície do frontend corrigida ou certificada.

A primeira redação dos metadados do Núcleo tinha chave TOML inválida e pins fora da ordem; ambos foram corrigidos e revalidados antes da coleta pareada. `allocation-preformat.json.gz` pertence a um build anterior à correção de indentação do trecho produtivo e foi substituído por `allocation-green.json.gz`, com o SHA do binário final. Não foi usado para fechar os números acima.

## Proveniência e reprodução

Receipts em `00_nucleo/diagnosticos/font-primary-reuse-2026-10-10/`:

- HEAD **06848cdcb5450f2c555626bf116bc7acd82bd82b**, **working tree não commitado**. before/candidate/final-validation guardam estado Git completo, UTC, fontes/L0/núcleo e hashes; `owner-diff.patch` isola os dois owners frente ao início da experiência.
- Antes: `9ae0a3d9285b8d0b4bae0da1db058483e7f5543003fabff46bb7d93b17e474f2`, cópia `/tmp/tekt-before-primary-reuse-2026-10-10`.
- Depois: **783572ba02f8b423935179dd7c81f62895bcf21db9144d48d2fa924421249ef0**, `target/release/typst`.
- Vanilla pinado **9f2b6e8715237cb086899a42873660fe744622e8**, árvore/receita P1489, binário `lab/typst-original/target/release/typst` SHA-256 **7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea**.
- Tempo: **2026-10-10T02:34:37.143159+00:00 → 02:35:50.615000+00:00**.
- Alocações baseline: **02:30:39.712342 → 02:30:46.380495 UTC**; candidato final: **02:32:48.050403 → 02:32:56.410469 UTC**; mesma data.
- Contagens candidato: **02:36:46.886667 → 02:37:03.270603 UTC**. O receipt antes é o `resolution-calls.json.gz` do perfil precedente, com mesmo baseline e fixtures de conteúdo idêntico.
- Drivers registram argv, ambiente, CPU, hashes/textos de fixtures, fontes tipográficas e binários; stack 64 MiB em todos os motores. O interpositor mede pedidos cumulativos glibc, não heap vivo. GDB requer ptrace e sua execução é separada dos benchmarks.

Reproduzir com `python3 .../allocation_gate.py red` (exit 1 esperado), `... green`, `measure.py`, `performance_gate.py`, `qualify_*`, `count_resolution.py` e `capture_final.py`. A cópia baseline precisa estar no caminho registrado; cada execução sobrescreve seu receipt. Tests: `cargo test -p typst-infra --lib primary_reuse_` e `cargo test -p typst-infra --lib -- --test-threads=1`. Selo limitado aos dois owners: `seal.py`.

## Próxima frente

O acréscimo de estilo local frente ao controle Tekt ainda é aproximadamente 18 ms nesta janela. O censo manteve 1.600 escopos adicionais, confirmando que o cache resolveu o custo comum de fontes. A próxima hipótese deve medir reconstrução/colapso de estilo e o caso inline-set, sem retirar herança, efeitos ou origem. A retenção de muitas variantes é uma frente independente.


### Continuação em 2026-10-10

A transferência de buffers plurais no PDF reduziu o caso de muitas variantes
de aproximadamente 513 para 286 MiB de pico e cerca de 16% do tempo, com
argv padronizado e qualificação antes/final. Ver
[pdf-font-buffer-reuse-2026-10-10.md](pdf-font-buffer-reuse-2026-10-10.md).
A captura direta de estilos ficou como experimento rejeitado, com fonte e
L0 restaurados; ver style-prefix-reuse-2026-10-10.md.
