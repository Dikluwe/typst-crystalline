# Desempenho do binário atual — comparação C→C2 e vanilla

O binário atual não demonstrou ganho geral de velocidade frente ao release C imediatamente anterior. Em seis dos sete documentos, a mediana da diferença pareada ficou entre −1.21% e +0.96%, pequena frente à variação entre rodadas. O caso Table com Cells Styled locais ficou +7.52% mais lento e usou cerca de +1.05MiB, mas também introduziu uma regressão de tamanho/posições: não executa trabalho equivalente ao anterior ou ao vanilla. Esse controle funcional precisa de correção antes de otimização. Nenhum código/L0 foi alterado nesta medição.

## Método e proveniência

Três rodadas sequenciais, doze trios por fixture/rodada, seis permutações da ordem de engines; sete documentos, 756 invocações medidas e126 warmups. CPU4, Rayon1, vanilla jobs1, mesmos arquivos/opções/fontes/fontconfig, PDF tagging false e pretty=false. Processos novos com cache de filesystem aquecido. Wall inclui launch/exec, catálogo/fontes, parsing/eval/layout/exportação, escrita e teardown; não mede recompilação incremental ou cold-disk. Hashing, verificação de saída e produção do relatório ficam fora do timer.

Os cinco fixtures gerais são cópias byte a byte da baseline: Hello, texto denso, contexto, imagem JPEG e formas posicionadas. Foram adicionadas Table com100 aliases bare e Table com100 Cells locais sob set text anterior. Há um controle de páginas/dimensões/palavras/bboxes/família/tamanho/cor por trio de engines e por bloco; não usamos bytes PDF como paridade.

HEAD06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado. Coleta wall:2026-10-07T19:48:27.583810+00:00 até2026-10-07T19:53:20.161287+00:00. Coleta RSS separada:19:53:22.462563+00:00 até19:56:12.725391+00:00. Quiet gate30s em ambas; monitor em todos os blocos. Nenhum build/test/benchmark/lint concorrente foi observado na visão de processos disponível. Todos os42 blocos passaram estabilidade de fontes e executáveis. A amostragem não garante ausência de processos breves ou ruído do desktop.

Executáveis congelados, SHA-256 verificado antes/depois de cada invocação:

- Atual C2: ac889581d6423d2fcfdd77d2f7e5cf57562dbf9adb40e73aebc25f959864c04f.
- Anterior C:23eb4227d17b332af08414506b81c685c63cfa440d18b18e0c2ec69fd23c675c.
- Vanilla pin9f2b6e8715237cb086899a42873660fe744622e8, árvore/receita P1489, binário7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea.

Receipts das builds/estados C/C2 foram copiados, com consumidores/inputs completos em consumer-inputs.json.gz. state e memory/state conservam HEAD, hashes integrais, git status expanded, diff HEAD --stat e patch binário comprimido em cada bloco. identity.json registra fontes integrais, argv/ambiente, fontes de projeto, host/CPU/governor e hashes. O source fingerprint permanece igual antes/depois; não se usou string de versão como prova. Esta comparação delimita o efeito C→C2. Os números históricos de setembro mediram outra árvore e condições; não calcular ganho causal contra eles.

## Tempo CLI — medianas em ms

| Documento | Anterior C | Atual C2 | Vanilla | Δ atual/anterior pareado |
|---|---:|---:|---:|---:|
| Texto simples | 157.51 | 156.40 | 129.88 | +0.07% |
| Texto denso | 176.34 | 175.40 | 133.47 | -0.28% |
| Contexto | 157.05 | 155.72 | 129.33 | -0.70% |
| Imagem JPEG | 11.72 | 11.76 | 13.20 | -0.99% |
| Formas | 14.85 | 14.67 | 13.86 | -1.21% |
| Table: cem aliases | 162.70 | 164.10 | 132.18 | +0.96% |
| Table: cem Cells locais | 163.97 | 176.56 | 133.79 | +7.52% |

Δ é a mediana das razões dos trios, não o quociente das medianas das colunas. Valores negativos indicam menos tempo na sonda, não aceleração certificada. Distribuições p05/p95, médias, medianas por rodada, bootstrap descritivo e dados individuais permanecem em summary.json e samples/*/samples.json; nenhuma amostra foi removida.

Todas as populações tiveram faixa de medianas entre rodadas acima de3% (aproximadamente3.07–12.02%). Alguns blocos também excederam3% de média/mediana. Portanto os números são indicativos, sem certificação de aceleração/regressão de pequena magnitude ADR-0115. A deriva foi compartilhada: no texto simples, atual154.83/156.18/161.11ms e anterior154.34/156.76/161.75ms. Repetir no mesmo desktop até obter uma rodada conveniente não seria prova válida. Na Table local, a diferença pareada foi +7.39%, +7.31%, +8.16% por rodada; sinal consistente, mas com saída diferente.

## Qualificação da saída

Atual e anterior coincidem em páginas/dimensões/palavras/fontes e posições até0.0001pt nos seis controles gerais/bare. Apenas a Table local falha esse controle.

Contra vanilla, texto simples, imagem, formas e Table bare passaram o controle disponível. Contexto e texto denso conservam páginas/dimensões/palavras/fontes, mas têm delta vertical máximo0.658006pt; já ocorre no anterior, sem novidade C2. Estes resultados não certificam renderização global. Para imagem/formas, bboxes de palavras são vazios: os controles não provam identidade pixel a pixel ou todas as primitivas. Tempos vanilla permanecem como referência do lifecycle, sem atribuição a uma função privada.

### Nova regressão C2-R1 — Table com Styled local e estilo de texto anterior

Fonte medida antes de classificar:

```typst
#set page(width: 400pt, height: 600pt, margin: 24pt)
#set text(font: "Libertinus Serif", size: 10pt)
#let c = { set table.cell(fill: red); table.cell[Local] }
#table(columns: 4, ..range(100).map(_ => c))
```

Em todos os três controles, os três engines retornam sucesso e100 palavras Local, com a mesma família e cor. pdftohtml publica fontspec size17 no atual versus15 no anterior/vanilla (unidades dessa ferramenta, não pontos Typst). A largura do primeiro Local cresce de21.880pt para24.068pt,10%. Seu bbox atual é x29.000–53.068; a segunda palavra começa em38.999, sobrepondo a primeira. Anterior/vanilla têm primeiro x29.000–50.880 e segundo começa em60.880. Delta máximo de bbox65.643pt horizontal/16.038pt vertical. Não é whitespace ou igualdade Rust: tamanho e posição reais divergem, e a saída atual perde a equivalência com o controle anterior. Não inferimos qual função é a causa sem auditoria L0/fonte adicional.

A medição original C2 principal/adicional permanece válida para seus fixtures; este novo recorte amplia a cobertura e refuta qualquer conclusão universal de estilos locais. O caso agora é C2-R1, prioritário antes do C3 misto e de otimizações de performance. Witness e resultado completo: table-local-regression.json; PDFs e controles em samples/r{1,2,3}-table-local/.

## Memória máxima CLI — medianas em MiB

A primeira população wait4 também gravou ru_maxrss, mas incluía a memória do Python antes do exec (cerca de105MiB em todos os engines). Esses valores foram excluídos de conclusões. Não foram apagados dos dados brutos.

A população adicional usa GNU time após o helper executar, medindo RSS do processo taskset/Typst sem carregar o footprint Python. Três rodadas/seis trios por fixture:378 medidas e126 warmups, mesmas opções/fontes. Somente RSS dessa população é usado; seus tempos/CPU não são misturados à população wall principal. O pequeno footprint taskset antes do exec pertence ao lifecycle observado. RSS mede pico do processo, não alocações exclusivas, memória retida ou cache incremental.

| Documento | Anterior C | Atual C2 | Vanilla | Δ pareado atual−anterior MiB |
|---|---:|---:|---:|---:|
| Texto simples | 42.50 | 42.64 | 29.22 | +0.064 |
| Texto denso | 44.19 | 44.21 | 30.46 | +0.014 |
| Contexto | 44.13 | 44.30 | 29.71 | +0.354 |
| Imagem JPEG | 18.42 | 18.45 | 21.36 | -0.029 |
| Formas | 19.94 | 19.63 | 21.22 | -0.371 |
| Table: cem aliases | 45.00 | 45.36 | 30.78 | +0.260 |
| Table: cem Cells locais | 45.21 | 46.17 | 30.93 | +1.049 |

Nos seis controles equivalentes, o delta pareado mediano de pico RSS ficou entre−0.371 e+0.354MiB, sem evidência de mudança material nesta sonda. Table local acrescenta+1.049MiB com saída diferente; não é comparação de memória para trabalho equivalente. Tekt com texto usa aproximadamente42.6–46.2MiB contra29.2–30.9MiB vanilla nestes inputs. Não extrapolar para documentos grandes ou watch.

## Decisão

As intervenções C2 corrigiram os testemunhos de origem/morfologia já medidos, mas não mostraram aceleração geral. O binário atual continua aproximadamente20–32% acima do vanilla no tempo destes documentos com texto, como mediana pareada do lifecycle CLI; contexto/texto denso têm o limite geométrico explícito acima. Antes de qualquer otimização, corrigir o novo C2-R1, conservar testes RED→GREEN e repetir o mesmo witness. Catálogo de fontes, memória e recompilação continuam candidatos separados para investigação; esta população sem instrumentação não atribui causalidade a eles.

Evidência reproduzível e verificação:00_nucleo/diagnosticos/performance-c2-2026-10-07/summary.json, identity.json, protocol.md, protocol-memory.md, snapshots/patches/archives, samples e memory/samples. Nenhum arquivo produtivo/L0 ou binário foi alterado; sem commit.
