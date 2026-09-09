# P1330 — parecer final independente

**PASS_SCOPED**: aprovada a correção do overflow inteiro de calc.abs e
a preservação delimitada pelo L0 R2. Revisor `/root/p1330_review`; A/B
executado sem atestação técnica de isolamento e sem selo de refinamento.
Somente diagnósticos novos de revisão foram escritos por este revisor.

## Estado e evidência auditados

HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não commitado.
Auditoria final reproduzível em `p1330-review-final-audit-r3.cjs`, SHA-256
`2367ecb9bc194964e4486f4a5147733ca2a68225dddbeb7a5712d7c431e54309`.
Resultado `p1330-review-final-audit.json`, SHA-256
`daab7e65fed54f081bf1a26ca3200f266b3d8ab39f539b20b2a146f00b72b6c8`,
registra instante, diff/stat exatos, identidade do inventário, hashes e
horários de cada gate. A auditoria recalculou todos os hashes produtivos
dos inventários, verificou os onze recibos finais contra o estado atual
e encontrou alteração somente no par calc L0/owner perante o baseline.

Pins finais confirmados:

- Manifesto R2: `fa90e9d05855842467eb7baac85d81a157af03bb285b9c1d40df0faf1f660acf`.
- L0 normativo: `9a5d734e086297d49d80d567bdb5527919049de19b7aea11f43b4467a972d2db`.
- Owner canônico: `30ef9f29b20e45b4ed51a368bfee566469df96db1dde5705755f8ca96b97794e`.
- Binário candidato: `6f1db621bc0b2a7fe4fc9d05925fb96636f33232b8527b83c040b793970fbda0`.
- Relatório final: `6a53f4aa62d7cdaa96f150d06cea1455dad46f3749f4128988d8892618684c68`.
- Recibo A/B público: `90181b08d77a339ebb6baa1e14be733e2923525b5e79896089b3920171c50061`.

## Fundamento do veredito

Reconstrução integral do owner confirma somente braço Int novo, metadata
de linhagem e integração dos snippets aprovados. Guards, demais ramos,
outros testes e P1329 permanecem iguais. O braço Int satisfaz mensagem,
cardinalidade, severidade, ausência de hint/trace nativo e primeira origem
posicional value_span, incluindo detached; demais Int conservam tipo e
valor exatos. O erro já não satura, promove ou envolve o resultado.

RED anterior a C auditado: 23 testes, 17 passam e seis falham justamente
pela aceitação baseline do overflow. GREEN dos mesmos bytes: 23/23;
GREEN após resselo: 23/23. Workspace final: 6709 passam, zero falhas e
três ignorados, soma de 17 resumos Cargo. Build, fmt, diff, linhagem
recíproca e V5/V15/V26 estritos passam no mesmo inventário final.
Estes são recibos de execução auditados, não novos builds do revisor.

A auditoria própria comparou cada exit/stdout/stderr candidato com os
literais congelados nas três ordens: 332 células por ordem, 996 no total,
sem ausências, duplicatas ou diferenças. Não depende dos booleans do
runner. Recibo A/B aprovado verificou adicionalmente ordem efetiva,
expressões/argv, estabilidade completa das linhas e resultados BASE/vanilla
perante a captura pré-C. Todos os seis artefatos congelados da integração
e os 902 arquivos históricos inventariados conservam hashes.

Os ganhos descritos no relatório foram recalculados diretamente dos
outputs, sem somar rótulos históricos: por execução, 32 células mudam
para coincidir integralmente com vanilla; 12 mudam e mantêm dívida de
trace; 220 coincidências e 68 divergências antigas são preservadas.
Isso não estima cobertura geral nem autoriza ocultar dívidas.

Lint geral permanece com zero errors, 240 warnings e 1145 notes.
A auditoria `p1330-review-lint-preservation.md` demonstra preservação
substantiva de todos os diagnósticos perante P1329, com apenas avanço
das linhas causado pelo braço Int. Colunas não são comparáveis no
recibo textual histórico. Não se declara ausência de avisos.

## Limites e encerramento

O relatório final distingue o literal mínimo que diverge antes de abs,
precedência dos guards, nome externo calc.abs/abs e dívidas dimensionais
anteriores. Não há expansão a parser, entidades, operadores, dispatcher,
guard_float ou outra função; nenhum Unknown é aceito como cumprimento
do recorte obrigatório. Não há alegação de paridade geral de abs/calc.

O incidente de posição dos testes foi corrigido com igualdade canônica
antes de RED. Na auditoria final do revisor, R1 abortou por guard de path
demasiado amplo que confundiu prompts/_nuclei/context com a pasta proibida;
R2 manteve verificações mas seu output excedeu o limite da ferramenta.
R3 corrige o guard para as duas pastas exatas e compacta somente a saída
do inventário em hash/contagem. Nenhum desses eventos mudou produto,
contrato, oráculos ou vereditos para acomodar falhas; os scripts anteriores
permanecem registrados. Nenhum conteúdo de pasta restrita foi aberto.

Este revisor não leu o passo: sua integridade cabe ao agregador do root,
que conhece o path autorizado. Parecer aprovado para esse agregador
fechar com os recibos e pins acima. Sem stage, commit ou push pelo revisor;
nenhuma ação fora do recorte foi autorizada por este parecer.
