# Custo das duas passagens de paginação na tabela simples

## Medição e proveniência

Coleta em **2026-10-05 09:01:36–09:02:11 America/Sao_Paulo**
(`12:01:36–12:02:11 UTC`), HEAD
`06848cdcb5450f2c555626bf116bc7acd82bd82b`. Código produtivo e L0
sem alterações: `git diff HEAD --stat` vazio no início e no fim.
O único arquivo não rastreado durante a medição era
`00_nucleo/diagnosticos/grid-pagination-cost-2026-10-05/probe.py`.
O relatório e as cópias dos resultados foram adicionados depois da coleta.

Build Tekt: `cargo build --release --locked --offline -p typst-wiring --bin typst`,
exit zero; SHA-256 do executável
`eec9edd5ded17b0cd58c3297d313ff5796694899b39d5b87f6c5426f5dbc8e49`.
Vanilla pinado `9f2b6e8715237cb086899a42873660fe744622e8`, executável
`lab/typst-original/target/release/typst`, SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
Este último corresponde ao build ratificado no P1489; não se usou a string
de versão como prova de identidade.

O [probe reproduzível](grid-pagination-cost-2026-10-05/probe.py), o
script GDB gerado em `/tmp/tekt-grid-pagination-cost-2026-10-05/layout.gdb` e os
[resultados individuais](grid-pagination-cost-2026-10-05/results.json.gz)
acompanham este diagnóstico. O runner gera fixtures, PDFs, timings públicos
e logs em `/tmp/tekt-grid-pagination-cost-2026-10-05/`. Reexecutar:

```sh
cargo build --release --locked --offline -p typst-wiring --bin typst
python3 00_nucleo/diagnosticos/grid-pagination-cost-2026-10-05/probe.py
```

A sonda usa GDB 15.1 e requer permissão de `ptrace`; o sandbox inicial
bloqueou esse acesso, e a execução externa foi autorizada pelo revisor
automático. Não há ablação nem alteração do compilador.

Mesmas fixtures simples das medições anteriores: quatro colunas Auto,
Libertinus Serif 10pt, página de 500 × 3500pt com margem 30pt. SHA-256 para
1, 20 e 80 linhas, respectivamente:

- `d5739642bf8c6da1967d8665bb9b45f8b0c522ca4e1a463eb59725447e63e9d2`
- `de5ce30a77ca2c3bab2c92661ed924175d1b4fb366638faef0ae35a345bf1753`
- `902986bd28ace4b39144ce567592229c5a5621453cbb661a408414c803a58d6d`

Por escala: duas warmups e nove amostras de cada modo (Tekt normal, vanilla
normal e Tekt sob GDB), com ordem alternada. CPU 0,
`SOURCE_DATE_EPOCH=0`, `RAYON_NUM_THREADS=1`, PDF sem tags, `--pretty=false`;
vanilla também `--jobs 1`. Os comandos completos constam de cada amostra.

## Escopo da sonda

Em `03_infra/src/pipeline/paged.rs:322` ocorre o layout inicial; `:336`
entra no ciclo de PageStore, expansão contextual, introspecção e realização
de outlines; `:397` chama o layout seguinte. Os wrappers convergem para
`layout_with_realized_math_callbacks_from_with_metrics` em `:555`, cujo
símbolo release foi localizado automaticamente por `nm`.

Breakpoints de entrada e retorno nesse símbolo medem cada chamada inteira,
incluindo a preparação local e o orçamento de callbacks. Nas fixtures
simples não há callbacks de usuário; todas as amostras produziram exatamente
duas chamadas. A contagem não é uma prova geral sobre documentos contextuais.

Além do tempo de parede, a sonda lê o primeiro campo de
`/proc/<pid>/task/<tid>/schedstat` com a thread do inferior parada em cada
extremidade. A diferença mede tempo de CPU dessa thread, excluindo a CPU
consumida pelo debugger e o tempo sem execução por disputa de CPU. É uma
medição da thread que executa o layout, não uma soma de eventuais workers.
O intervalo entre o retorno inicial e a entrada seguinte mede a preparação
do candidato; não separa suas operações internas.

## Resultado das passagens

Medianas de CPU da thread, em ms. A coluna ciclo é a mediana das somas
por amostra, não a soma das medianas das outras colunas.

| Células | Primeiro layout | Preparação entre layouts | Segundo layout | Preparação + segundo layout |
| --- | ---: | ---: | ---: | ---: |
| 4 | 1,088 | 0,126 | 0,373 | 0,484 |
| 80 | 6,613 | 0,145 | 3,853 | 4,023 |
| 320 | 23,648 | 0,273 | 15,152 | 15,466 |

Nas 320 células, o segundo layout variou de **14,963 a 16,708 ms de CPU**;
preparação mais segundo layout variou de **15,232 a 17,058 ms**. A mediana
de parede desse ciclo foi **16,249 ms**, incluindo interrupções do debugger.
O primeiro layout é mais caro: o segundo já recebe caches aquecidos.
Portanto não se deve estimar o custo repetido dividindo o layout total por dois.

## CLI e controle de instrumentação

Estes tempos são de subprocessos normais, sem GDB. Tempo inclui inicialização,
compilação, recursos, exportação e escrita; CPU inclui usuário e sistema.

| Células | Parede Tekt | Parede vanilla | Delta pareado mediano | CPU Tekt | CPU vanilla |
| --- | ---: | ---: | ---: | ---: | ---: |
| 4 | 211,25 ms | 141,74 ms | +64,31 ms | 208,32 ms | 139,39 ms |
| 80 | 194,67 ms | 138,32 ms | +55,17 ms | 192,40 ms | 136,01 ms |
| 320 | 229,37 ms | 149,85 ms | +79,94 ms | 227,21 ms | 147,71 ms |

O desktop não é um ambiente estável: a CLI de quatro células variou de
193,47 a 270,23 ms no Tekt, maior que a escala seguinte. Não usar essa
inversão para concluir regressão por tamanho ou reproduzir o delta fixo
anterior. Uma coleta exploratória inicial só de parede apresentou ainda mais
variação, motivando a segunda coleta com CPU; ela permanece em
`/tmp/tekt-grid-pagination-cost-2026-10-05/wall-only-exploratory.json`
e não fundamenta as tabelas deste relatório.

O timing público `layout_ms` do Tekt normal nas 320 células teve mediana
**39,355 ms**, contra **44,527 ms** sob GDB. O debugger altera os tempos:
seu intervalo público não deve ser usado como o custo normal de produção.
O tempo de CPU por thread reduz esse problema, mas não elimina efeitos de
breakpoints sobre caches e execução. Os 15,15 ms são uma estimativa medida
do segundo layout sob instrumentação, não um ganho causal já validado.

Todas as saídas medidas tiveram uma página. Texto e ordem extraídos por
`pdftotext -raw` coincidiram entre os três modos; as coordenadas de palavras
extraídas por `pdftotext -bbox` coincidiram entre Tekt normal e instrumentado.
Esses controles verificam texto, ordem e geometria textual da sonda; não
constituem uma prova completa de paridade de toda a linguagem ou do render.

## Decisão após a medição

**A segunda passagem tem um custo relevante e quantificado nesta fixture,
mas não explica a maior parte da diferença total.** Há aproximadamente
15 ms de CPU no layout repetido da tabela de 320 células, enquanto a diferença
de CLI normal continua perto de 80 ms nesta coleta. Não dividir esses números
para anunciar uma percentagem causal: os instrumentos e escopos diferem.
Na fixture de quatro células, o ciclo adicional custa menos de 1 ms de CPU,
portanto sua remoção não explica o delta de dezenas de ms do documento pequeno.

O intervalo de preparação é pequeno nas fixtures medidas. Além disso, parte
dele cria o PageStore final, necessário mesmo se um candidato pudesse
reutilizar o documento inicial. Não tratar os 15,47 ms do ciclo inteiro como
trabalho integralmente removível.

Próximo experimento causal: definir no L0 owner um predicado conservador de
ausência de dependências que exigem relayout e comparar a execução atual com
essa variante, preservando PageStore, posições, warnings, callbacks e opções
do documento. A inferência de ganho será refutada por ausência de redução
repetível ou por mudança na saída e nos diagnósticos. Uma página, ausência
de numeração visível e igualdade mecânica de `Content` não provam o predicado.
O L0 vigente foi lido e não foi alterado por este estudo.

Separadamente, investigar o intervalo hoje classificado como `eval_ms` e
os custos gerais do documento pequeno. Neste estudo ele inclui o tempo do
helper de avaliação (`paged.rs:201–211`); seu nome não autoriza atribuir todo
esse custo a L1, parsing ou fontes sem subdividir a medição.

## Verificação

Build release e `crystalline-lint .` terminaram com exit zero. O linter
emitiu avisos preexistentes, inclusive V5 em cópias históricas e V20 em testes;
não se declara uma saída sem avisos. Log conservado em
`/tmp/tekt-grid-pagination-lint-2026-10-05.log`. Nenhum código produtivo,
contrato público ou prompt foi modificado; não houve ciclo RED→GREEN porque
este trabalho é somente diagnóstico externo.
