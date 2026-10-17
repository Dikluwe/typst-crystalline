# Correção do custo recorrente de fontes no watch

## Medição e decisão

Em 2026-10-09 03:11:32–03:12:06 UTC, três rodadas pareadas de 12 ciclos de `typst watch` compararam o binário Tekt anterior (`3d6b4b21…`) e o corrigido (`4c2b720e…`) sobre documentos e alterações equivalentes. As amostras completas, comandos, ambiente, hashes dos binários e caixas de palavras estão em [measurements.json](measurements.json); o script reproduzível está em [measure.py](measure.py). O commit base era `06848cdcb5450f2c555626bf116bc7acd82bd82b`, com working tree não commitado: [git-diff-head-stat.txt](git-diff-head-stat.txt) e [git-status-short.txt](git-status-short.txt) registram o estado exato no início da medição. A comparação do item 5 com o vanilla está no diagnóstico `watch-incremental-item5-2026-10-08/` e não foi refeita aqui.

| Alteração | CPU mediana antes → depois | Latência mediana antes → depois | Publicações |
|---|---:|---:|---:|
| somente metadados | 54,41 → 22,89 ms | 128,69 → 95,97 ms | 9/9 em ambos |
| mesmos bytes | 54,05 → 22,37 ms | 129,74 → 95,76 ms | 9/9 em ambos |
| texto pequeno | 54,19 → 23,16 ms | 128,45 → 97,04 ms | 9/9 em ambos |
| estrutura | 54,88 → 23,17 ms | 129,50 → 97,18 ms | 9/9 em ambos |

Texto extraído idêntico nas 36 comparações; sequências de palavras e caixas também coincidiram (`max_bbox_delta_pt = 0`). A mediana de arranque foi **266,01 → 298,75 ms**. O RSS mediano após ciclos ficou aproximadamente **30,68 → 30,86 MiB**, conforme o tipo de alteração. Essas grandezas são de uma máquina e deste documento, não projeções universais.

A primeira variante, que repetia a travessia integral em cada ciclo, reduziu CPU apenas para cerca de 40–42 ms; [measurements-full-scan.json](measurements-full-scan.json) preserva essa medição. A segunda ainda atravessava duas vezes ao criar a geração; [measurements-double-probe.json](measurements-double-probe.json) preserva a medição que mostrou custo de arranque de aproximadamente 314 ms. A versão final faz uma travessia inicial e revalida configuração/identidades após construir.

## Implementação

`infra/fontdb/generation` mantém o preparador fresco como fallback e acrescenta uma prova rápida para Linux com fontconfig padrão e sem fontes de projeto explícitas. Ao estabelecer a geração, percorre todas as entradas dos diretórios efetivos. Retém identidades de diretórios, candidatos a fonte e symlinks, inclusive destinos e ausências. Em ciclos estáveis, reanalisa a configuração e revalida essas identidades e os stamps das fontes carregadas, sem redescobrir o catálogo. Mudança, erro, configuração personalizada ou limite força o preparador fresco. `infra/system-world/font_session` guarda a prova somente com o catálogo pareado e não a transporta para outra geração. Os dois L0 foram atualizados e selados antes da medição final e da validação; [seals.json](seals.json) registra os hashes.

A redução de CPU é real, mas o watch ainda recompila e publica no toque de metadados, e conserva uma diferença de execução para o vanilla. Evitar essa publicação exige outra decisão sobre invalidação de entradas e recursos. A prova rápida não acelera configurações personalizadas ou fontes de projeto e aumenta o custo de criação da sessão.

## Validação

O teste RED inicial falhou antes da implementação ([excerto](test-red-excerpt.txt)). Depois passaram quatro testes da prova rápida ([resultado](test-proof-green-excerpt.txt): diretório aninhado com mtime restaurado, escrita de mesmo tamanho com mtime restaurado, symlink, raiz ausente, orçamento e configuração personalizada) e os 12 testes da sessão ([resultado](test-session-green-excerpt.txt), incluindo mudança durante a construção e concorrência). `cargo build --offline` e `crystalline-lint .` concluíram com código de saída 0; os dois consumers alterados não têm avisos do linter. A árvore compartilhada já contém avisos em outros arquivos.
