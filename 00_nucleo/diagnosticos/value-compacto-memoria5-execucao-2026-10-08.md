# Memória — prioridade 5: Value compacto, execução ratificada

## Medição anterior à decisão

O pacote `value-compacto-memoria5-pacote-2026-10-08.md` mediu `Value` e `Binding` com 272 bytes e identificou o payload inline de `Value::Stroke` como o maior discriminante. O modelo isolado previu 144 bytes com `Arc<Stroke>`. Esse custo físico não constituía prova de RSS nem critério de paridade da linguagem. O utilizador confirmou o manifesto SHA-256 `6badf07a4f58f08e7c60ad2e34dfb9a14a3a6ba55db90045afe046be8e988237`; a mudança do payload Rust público foi, portanto, ratificada conforme ADR-0127.

## Implementação e linhagem

Os 14 L0 aprovados foram importados antes dos testes/código; os 14 consumers correspondentes constam em `owners.json`. `Value::Stroke` passou a carregar `Arc<Stroke>`, e `From<Stroke> for Value` preservou a assinatura, envolvendo o valor com `Arc::new`. Os consumidores que precisam de geometria própria clonam `stroke.as_ref()`; reconstruções/folds produzem payload independente. Clones de `Value` compartilham o stroke imutável. Arrays continuam com sua representação anterior.

Clientes Rust que constroem a variante diretamente devem fornecer `Arc<Stroke>` ou usar `Value::from(stroke)`. A semântica Typst de stroke, os campos/getters e o comportamento padrão permanecem cobertos pelos testes. Nenhuma nova dependência, I/O ou estado global foi introduzido em L1. O resselo foi limitado aos 14 owners, com validação dos pins/núcleos; `seals.json` registra os hashes. `productive.diff` e `changes.json` registram as 28 diferenças desta intervenção, sem incorporar alterações anteriores ao passo.

## Proveniência

HEAD: `06848cdcb5450f2c555626bf116bc7acd82bd82b`, **working tree não commitado**. `before.json.gz`, `red-state.json.gz` e `candidate.json.gz` conservam os conteúdos integrais das fontes/L0/configurações, status e `git diff HEAD --stat` nos respectivos instantes. A conferência final verificou que todos os arquivos do candidato permaneceram idênticos durante a medição e o fechamento.

Baseline, cristalino após memória4: `/tmp/typst-before-value-memory5`, SHA-256 `564b1fe69f92880736485e882e2e1409a7a9df59712632783e6eae5f74470784`.

Atual: `target/release/typst`, SHA-256 `89d01f159b7244d8684b8644400b7673fff34ee278894e906dabcde73c17b450`.

Bateria UTC: `2026-10-08T23:30:49.649626+00:00` até `2026-10-08T23:31:50.893074+00:00`. Foram 15 fixtures, 3 blocos × 4 pares alternados, 360 execuções medidas, com um aquecimento por motor/caso/bloco. Processos novos, filesystem aquecido, CPU fixada 0, Rayon=1, epoch=0, locale C e fontes comuns de `performance-baseline/fonts`. `bench.json.gz` conserva todos os comandos, stderr, amostras e SHA-256 das entradas/fontes/configurações/fontconfig/assets/binários; `measure.py` registra o monitor de exclusão de builds/testes/lint/GDB concorrentes. RSS é `%M` de `/usr/bin/time`, em KiB; wall inclui o wrapper comum. Não é perfil de bytes vivos do heap nem bateria de cache frio.

Não houve nova medição contra vanilla. O pin de referência continua `9f2b6e8715237cb086899a42873660fe744622e8`; este relatório compara somente os dois estados cristalinos acima, sem somar percentuais de passos anteriores.

## RED → GREEN e validação

Os dois testes novos falharam antes da implementação: tamanho 272 acima do teto e clone sem compartilhar payload. Após a alteração, ambos passaram; asserções semânticas verificam igualdade, representação e independência da reconstrução. Endereços e sizeof são testemunhos de custo de RAM, separados da aceitação de linguagem (ADR-0107). Uma primeira compilação GREEN detectou tentativa de mover `dash` de dentro do Arc; o acesso foi corrigido para `as_ref()`, sem relaxar asserções. Os logs RED, primeira compilação e GREEN foram preservados.

- Core: 6646 testes passaram, 5 ignorados.
- Infra: 1253 passaram, 2 ignorados; shell: 90 passaram.
- `cargo build --offline` e `cargo build --offline --release`: sucesso.
- `crystalline-lint .`: 0 erros, 535 warnings e 1640 infos, mesmas contagens passivas do baseline.

As suites completas de biblioteca core/infra/shell foram executadas com `RUST_MIN_STACK=67108864`, necessário para testes recursivos existentes; isso não altera o produto nem o ambiente da bateria. Os logs conservam as execuções. A cobertura existente inclui SVG/PNG/HTML, folds de tabelas, strokes/shapes, augment de matrizes, context e diagnósticos. Não foi feita comparação bilateral nova de todas as exportações.

A sonda ligada ao rlib release real mediu **Value=144 bytes e Binding=144 bytes**, redução de **47,06%** do tamanho físico neste alvo de 64 bits. `real-sizes.json` registra horário, comandos, fonte e SHA-256 do rlib e executável. O teste usa teto de 160 bytes; os 144 são a medição deste build, não uma promessa de ABI portátil.

## Resultados pareados

As colunas antes/depois são medianas das 12 amostras por motor; o percentual é a mediana das diferenças percentuais de cada par, portanto não necessariamente coincide com a razão dessas medianas. Valores negativos significam redução. `summary.json` conserva também resultados por bloco.

| Caso | Pico RSS MiB antes → depois | RSS pareado | Wall ms antes → depois | Wall pareado |
|---|---:|---:|---:|---:|
| cloned-strokes | 45.54 → 43.82 | -3.97% | 121.53 → 118.19 | -2.66% |
| context | 43.19 → 43.35 | +0.24% | 117.84 → 118.49 | +0.19% |
| context10 | 43.08 → 43.16 | +0.14% | 117.58 → 117.68 | +0.02% |
| context100 | 55.16 → 52.68 | -4.39% | 136.44 → 134.54 | -1.19% |
| context300 | 132.42 → 125.00 | -5.59% | 226.98 → 223.19 | -2.09% |
| dictionary | 44.16 → 43.30 | -1.89% | 120.66 → 119.20 | -1.02% |
| image | 18.81 → 18.53 | -2.02% | 10.81 → 10.64 | -1.22% |
| numeric-array | 52.47 → 47.54 | -9.19% | 127.00 → 121.99 | -3.65% |
| pipeline-single-font | 41.69 → 41.84 | +0.36% | 116.47 → 116.25 | -0.40% |
| plain300 | 46.35 → 46.31 | -0.21% | 138.65 → 137.97 | +0.07% |
| shapes | 20.15 → 19.70 | -2.16% | 14.35 → 13.81 | -4.50% |
| table-alias | 44.18 → 44.09 | -0.16% | 123.73 → 122.81 | -0.68% |
| table-local | 44.67 → 44.19 | -0.99% | 134.45 → 134.85 | +0.45% |
| text-dense | 43.95 → 44.17 | +0.56% | 136.99 → 137.58 | -0.17% |
| unique-strokes | 45.67 → 43.86 | -3.99% | 126.56 → 123.63 | -2.93% |

Arrays numéricos reduziram RSS em 9,19%; context300 em 5,59%; strokes únicos e repetidos em aproximadamente 4%. Os ganhos de RSS e tempo desses casos apareceram nos três blocos. Context300 passou de 132,42 para 125,00 MiB de pico mediano; trata-se de ganho adicional ao estado memória4 usado como baseline.

Há contrapartidas pequenas: texto denso aumentou RSS em 0,56%, nos três blocos; tabela local aumentou wall em 0,45%, também nos três. Context simples e texto simples apresentaram pequenos aumentos de RSS. Os casos de imagem/shapes duram poucos milissegundos, tornando seus percentuais mais sensíveis a variação. Não há fundamento para anunciar redução uniforme de memória ou ganho global de velocidade.

## Qualificação e decisão

Os 15 PDFs finais preservaram texto extraído, sequência de palavras, dimensões de página, estilos de fonte e raster a 36 dpi; a maior diferença de bounding box de palavra foi zero. Os 180 pares medidos preservaram stderr. `qualification.json.gz` conserva os dados extraídos e hashes dos PDFs/raster. Isso qualifica este corpus entre baseline e candidato; não fecha a paridade global com vanilla nem exige igualdade de bytes PDF.

**Decisão: conservar a mudança**, ratificada, por reduzir os carriers reais e o RSS dos principais casos visados sem falhas semânticas detectadas. A alocação adicional por stroke único não piorou o corpus de 5000 strokes medido, mas outros tamanhos/distribuições podem diferir. O aumento pequeno de RSS no texto denso e de tempo na tabela local permanece registrado, sem ocultar esses controles. Não foram feitas alterações adicionais de representação de arrays.

Artefatos deste relatório: `value-compacto-memoria5-execucao-2026-10-08/`. O fechamento em `completion.json` registra a conferência final e hashes dos arquivos de evidência.
