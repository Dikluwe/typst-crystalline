# Comparação do conjunto atual com baseline P1 e vanilla

Nenhum código/L0 alterado. Três engines frozen/pinados, sete fixtures byte a byte
da coleta C2: baseline ac889, atual 398b, vanilla 9f2b/bin7fa9. Mesmos fonts/fontconfig
e font-path explícito da baseline. Três rodadas, seis trios por fixture, todas as seis
permutações de ordem; uma warmup/motor/bloco. Processos novos, cache FS aquecido,
afinidade única min allowed, Rayon1/vanilla jobs1, epoch0, PDF sem tags/pretty.
GNU time mede pico RSS do filho sem footprint Python; wall inclui launcher/wrapper
idêntico. Fingerprints antes/depois, monitor de bloqueadores em cada invocação;
sem build/test/GDB/lint concorrente. Verificação de saída fora dos tempos.
Medianas/razões pareadas por caso e soma construída por ciclo; sem bootstrap formal.
Mostrar variação entre blocos; aviso exploratório de drift>3%, inspirado no critério
do scanner ADR-0115, não uma extensão normativa dessa ADR a toda CLI.
Table local baseline tem erro de estilo: excluída do ganho acumulado equivalente.
Qualificar texto/geometry/fontes contra vanilla e sinalizar diferenças; saída PDF
em bytes/raster entre implementações não é gate de paridade da linguagem.
