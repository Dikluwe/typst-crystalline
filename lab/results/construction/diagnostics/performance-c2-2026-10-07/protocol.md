# Protocolo — CLI atual versus pré-C2 e vanilla pinado

Preparado antes de medir. Diagnóstico, sem alteração de código/L0 ou otimização.

Comparar o release atual ac889581d6423d2fcfdd77d2f7e5cf57562dbf9adb40e73aebc25f959864c04f, o release C pré-C2 congelado 23eb4227d17b332af08414506b81c685c63cfa440d18b18e0c2ec69fd23c675c e o vanilla pinado 9f2b6e8715237cb086899a42873660fe744622e8 (binário 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea). Copiar executáveis para /tmp e verificar hashes antes/depois. O C tem snapshot e receita registrados em table-cell-styles-d08b3c-execucao-2026-10-07.md; o atual em table-origens-d08b3c2-execucao-2026-10-07.md. Comparação de velocidade isola o delta C→C2, não todas as intervenções desde setembro. Os tempos históricos de setembro não serão comparados como A/B causal.

Cinco fixtures da baseline sem alterar fonte: Hello, texto denso, contexto, JPEG e formas posicionadas. Acrescentar Table com cem aliases bare e Table com cem Cells Styled locais. Mesmas fontes/projeto/opções, PDF sem tagging e pretty=false. Math excluído, conforme qualificação de fontes anterior. Guardar bytes/fontes integrais e hashes.

Três rodadas sequenciais, doze trios por fixture/rodada, seis permutações de ordem repetidas, duas compilações não medidas por engine/fixture/rodada. Afinidade CPU4, Rayon1 e vanilla jobs1; processos novos com cache de filesystem aquecido, sem cold-disk ou compilação incremental. Tempo wall launch→exit inclui startup/catalog/fontes/I/O/exportação/teardown; também medir CPU user/system e RSS máximo por processo com wait4. Nenhuma fase interna cronometrada nesta população. Sem seleção do menor tempo ou remoção de outliers.

Quiet gate de trinta segundos sem builds/testes/benchmarks/lint/Typst concorrentes observados; monitor durante os blocos. Nenhum processo externo será interrompido. Receipts com fingerprint de L1–L4/manifests/L0/fixtures/fontes/fontconfig, HEAD, dirty/untracked inventory, diffstat, patches e conteúdos. Antes/depois iguais e hashes de binários iguais exigidos; bloco contaminado rejeitado inteiro e preservado, sem misturá-lo às rodadas válidas. A amostragem de processos não garante ausência de concorrência muito breve nem de carga não detectada do desktop.

Conferir páginas, dimensões, palavras/bboxes e família/tamanho/cor em um trio por bloco, fora do timer; nunca bytes PDF. Publicar separadamente correspondência da saída e limites de qualificação. Falha de qualificação impede tratar uma redução de tempo como melhoria para a mesma saída.

Relatar medianas, p05/p95, razões e deltas pareados por trio, CPU e RSS; média/mediana <3% ADR-0115 e faixa de medianas entre rodadas <3% separadamente. Falhar repetibilidade torna resultado indicativo, não prova de aceleração/regressão. Bootstrap da mediana pareada é apenas intervalo descritivo condicionado aos blocos, não elimina variação sistemática do host. Não atribuir causalidade a fonte/layout isolados sem instrumentação própria.
