# Cobertura de fontes por intervalos — experimento, 2026-10-05

O experimento indica uma oportunidade na construção do catálogo: consumir os segmentos declarados em `cmap` em vez de enumerar cada ponto e reconstruir seus intervalos. A extração e construção isoladas passaram de aproximadamente 53,19 para 10,85 ms, redução de 79,6%. **Não é tempo da CLI nem ganho já incorporado ao produto.** Nenhum código produtivo foi alterado nesta etapa; o L0 público está preparado e aguarda ratificação.

## Medição antes da decisão

Working tree não commitado, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`. O estado inicial, a lista exata de arquivos alterados (`git diff HEAD --stat`), fontes e hashes estão em `cmap-intervalos-2026-10-05/before-state.json.gz`. Cada coleta tem timestamps UTC, comandos, CPU, fontes, hashes dos arquivos de fontes, Cargo.lock e texto integral do programa experimental nos recibos. O executável produtivo permanece com SHA-256 `f8fc2599bc599b39678575df1eea3fd71b807f1ffd257abbb0ce3413b2344915`.

São três processos release, presos à CPU 0, sete amostras por face por processo, alternando a ordem dos algoritmos. O baseline usa o extrator ponto a ponto e o `Coverage::from_codepoints` atual. A alternativa faz leitura direta verificada dos formatos 4/12/13 e união pura dos intervalos. Parsing inicial da face e descarte do resultado ficam fora do trecho cronometrado. Os valores abaixo somam as durações das faces e dividem pelas sete amostras.

| Experimento independente | Baseline, ms por processo | Alternativa, ms por processo |
|---|---|---|
| Intervalos diretos, sem compactação inicial | 50,878 / 51,240 / 51,005 | 20,043 / 20,070 / 20,031 |
| Intervalos diretos, compactação antes de ordenar | 53,044 / 53,405 / 53,130 | 10,803 / 10,883 / 10,863 |

A segunda coleta ocorreu de `2026-10-05T18:47:39.623062+00:00` a `2026-10-05T18:47:44.491955+00:00`. Os recibos `measurement-direct.json.gz` e `measurement.json.gz` preservam os dois programas e suas medições separadamente. O binário da segunda medição tem SHA-256 `f97eb1bcf2f8538388c3972b2953db8e78a5ee0f12a05212640dfdb5d2fea180`; depois da inclusão do módulo de testes e recompilação, o binário experimental tem SHA-256 `60e735888f74199fb4c00973af765b2ae68d0849e3a6f057f81532c4df321e78`. Não se atribui a medição ao segundo binário.

Nas 1.112 faces de sistema, a representação canônica coincidiu em todas as amostras. Foram 1.520.598 intervalos de entrada em 3.582 subtabelas nativas, sem callbacks pontuais nem fallback de face nesta coleção. A coleta anterior enumerava 28.689.553 ocorrências de pontos nesse catálogo, conforme `custo-fixo-2026-10-05.md`. A equivalência da representação privada serve de controle do experimento; não substitui os critérios de paridade da linguagem ou validação de PDF.

## Semântica e controles

A leitura do código fixado de `ttf-parser` 0.25.1 antecedeu a escolha: `tables/cmap/format4.rs:103–115`, `format12.rs:68–75` e `format13.rs:41–48` enumeram limites declarados, sem filtrar IDs de glifo zero. O formato 4 para no segmento singleton FFFF. O iterador de subtabelas termina no primeiro registro inválido. O programa experimental preserva essas decisões. O vanilla pinado `9f2b6e8715237cb086899a42873660fe744622e8`, em `lab/typst-original/crates/typst-library/src/text/font/info.rs:119–132`, usa builder incremental e união; ainda enumera pontos. A leitura direta dos segmentos é a alternativa aqui investigada, não uma característica atribuída ao vanilla.

Sete testes sintéticos passaram tanto em debug quanto em release: união com ordem arbitrária, intervalos vazios/invertidos e fronteira Unicode; formato 4 com glifo zero e sentinela; formatos 12/13 com sobreposição e buracos; subtable não Unicode e fallback formato 6; primeiro registro inválido; limite exclusivo não representável; formato 4 truncado. Fontes sintéticas derivadas da fixture servem somente ao parser experimental. Logs integrais, timestamps de conclusão e fontes finais estão em `execution.json.gz`.

O risco principal é duplicar parte do parsing. O adapter experimental usa a classificação Unicode do parser tipado, valida o formato bruto, lê bytes com limites e aritmética verificados, e volta ao callback existente se a leitura direta não for aplicável. Um limite exclusivo não representável sinaliza fallback para o extrator legado da face inteira. A coleção instalada não exercita esse fallback, por isso há controle sintético. Não foi adicionado `unsafe`.

## Decisão e próxima materialização

Inferência: eliminar a enumeração e o vetor intermediário pode reduzir também o custo real do catálogo. A hipótese será refutada ou dimensionada pela medição da CLI após a implementação produtiva; não se deduz uma redução de 42 ms da CLI a partir deste trecho isolado.

A união e codificação pertencem ao domínio puro. A proposta é acrescentar `Coverage::from_ranges(impl IntoIterator<Item = Range<u32>>)` em L1 e manter leitura de bytes/eligibilidade/fallback em L3. O contrato usa intervalos semiabertos, ignora intervalos vazios ou invertidos e aceita sobreposições, repetições e qualquer ordem. `from_codepoints` mantém seu contrato. Não há alteração de seleção de fonte, campos de entidade, fases, política de cache ou comportamento padrão.

O L0 proprietário salvo é `00_nucleo/prompts/entities/font_book.md`, SHA-256 **`1ef2d9719a79f848d473e074ecee3fea8f79c2bcd56e87c957f70bf8abcd5e9d`**. A ADR-0127, seção 2, exige “PARAGEM OBRIGATÓRIA antes de escrever código (confirmar com o dono)” para contrato público. O novo construtor amplia esse contrato. O header do consumer permanece no L0 anterior deliberadamente: não foi ressellado antes da ratificação.

Após ratificação: testes RED→GREEN do construtor produtivo com oráculo independente; L0 exclusivo do adapter privado L3 antes de sua implementação, preservando cardinalidade 1:1; testes sintéticos do parser e regressões existentes; build/lint; medições CLI pareadas com fontes e variantes, mais controles de texto/geometria/PDF. Os testes produtivos da etapa anterior não são apresentados como execução desta etapa. Ainda não há conclusão nova de lint ou ganho de produção.
