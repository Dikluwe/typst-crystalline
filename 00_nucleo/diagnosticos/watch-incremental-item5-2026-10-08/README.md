# Item 5 — compilação incremental e watch (8 de outubro de 2026)

## Experimento e proveniência

`measure.py` lançou processos `watch` reais, um motor de cada vez, na mesma
CPU e com `RAYON_NUM_THREADS=1`. Foram três rodadas, nove amostras por condição
e motor. A ordem dos motores alternou por rodada. O import físico continha
2.000 declarações e permaneceu intacto. O documento solicitou Libertinus
Serif; `fc-match` resolveu NotoSans-Regular.ttf, com caminho/hash em
`measurements.json`. O script e a fonte principal gerada estão identificados
por SHA-256. PDFs sem tags, epoch fixo e fontes/ambiente compartilhados.

Condições no mesmo watcher: (1) tocar apenas mtime, sem alterar bytes; (2)
salvar atomicamente **os mesmos bytes**; (3) trocar uma palavra (`Alpha` ↔
`Bravo`); (4) mudar `range(8)` ↔ `range(12)` em um `#for`, acrescentando ou
retirando quatro itens visíveis. A segunda condição é o controle principal de
"documento sem alteração" que também produz evento de escrita. As outras
condições mudam a mesma fonte principal. As 36 comparações bilaterais tiveram
texto extraído idêntico, sequência de palavras idêntica e delta máximo de
caixas de **0 pt**. A fixture anterior com tabela foi descartada porque
mostrava 5,5 pt de diferença geométrica; seus números não fundamentam esta
decisão.

Medição UTC de `2026-10-09T02:04:04.614495` a `02:04:44.908130` (8 de
outubro em São Paulo). HEAD
`06848cdcb5450f2c555626bf116bc7acd82bd82b`, **working tree não
commitado**. `git-diff-head-stat.txt` e `git-status-short.txt` conservam a
lista exata naquele instante; seus SHA-256 constam de `measurements.json`.
Binários: Tekt release SHA-256
`3d6b4b21b85518c82433d48bfc64fb61dab8a1afda43708a3d3793568161201f`;
vanilla pinado `9f2b6e8715237cb086899a42873660fe744622e8`, SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
`measurements.json` contém comandos, amostras, horários, hashes e textos.

## Resultado

Medianas das nove amostras por condição. CPU soma tempo de execução das threads
do processo watcher entre o evento e a publicação (ou janela de 0,8 s para
mtime ignorado); latência é evento → PDF completo, incluindo espera do watcher.
RSS é `VmRSS` após o ciclo, em KiB. Os processos foram medidos sequencialmente.

| Condição | Publicações Tekt/vanilla | CPU Tekt/vanilla | Latência Tekt/vanilla | RSS Tekt/vanilla |
|---|---:|---:|---:|---:|
| Só mtime | 9/0 | 54,47 / 0,09 ms | 128,66 ms / — | 30.340 / 31.688 KiB |
| Mesmos bytes, escrita atômica | 9/9 | 54,38 / 1,19 ms | 129,58 / 102,57 ms | 30.340 / 31.732 KiB |
| Palavra pequena | 9/9 | 54,75 / 1,90 ms | 128,11 / 102,75 ms | 30.340 / 31.932 KiB |
| Expansão estrutural | 9/9 | 54,73 / 1,40 ms | 130,30 / 102,80 ms | 30.340 / 31.944 KiB |

O caso "só mtime" difere na **política de evento**: o vanilla filtra
`ModifyKind::Metadata` em `lab/typst-original/crates/typst-kit/src/watcher.rs`;
o Tekt inclui `modified` no fingerprint em `03_infra/src/watch.rs` e recompila.
Não atribuir essa diferença a memoização. Já a escrita dos mesmos bytes
provocou publicação nos dois; a diferença de CPU permanece.

As três rodadas Tekt começaram com RSS de 30.576/30.536/30.556 KiB e
terminaram em 30.460/30.340/30.132 KiB após 12 ciclos: não se observou
crescimento retido neste recorte. As três vanilla começaram em
31.416/30.964/31.540 KiB e terminaram em 31.988/31.720/32.168 KiB. Isto
**não** prova vantagem geral de memória: são processos curtos, uma fixture,
seleção de fontes local e RSS sujeito a política do alocador. Startup tem só
três amostras: mediana wall 265,88 ms Tekt e 276,22 ms vanilla, sem conclusão
robusta sobre partida.

## Reutilização observada e limite

`count.py` executou o release Tekt separado dos tempos sob GDB, no mesmo
modelo de fonte. Acumulados por ciclo inicial → mtime → mesmos bytes → palavra
→ estrutura:

| Entrada contada | Acumulados |
|---|---|
| `Source::new` | 2 / 2 / 2 / 3 / 4 |
| avaliação do import | 1 / 2 / 3 / 4 / 5 |
| avaliação da raiz | 1 / 2 / 3 / 4 / 5 |

`counts.json` conserva script GDB, comandos, timestamps, hashes de fonte/PDF
e SHA-256 do release. Os contadores demonstram reuso de CST da raiz/import
quando os bytes coincidem e de CST do import quando a raiz muda. Não medem
todo o trabalho dentro da avaliação. O código em
`03_infra/src/world/source_session.rs` limita essa sessão a oito entradas,
1 MiB agregado de texto e 512 KiB por entrada; `04_wiring/src/watch.rs` a
mantém durante o processo, mas cria um World novo por ciclo. Module/import e
layout não são produtos guardados por essa sessão. As entradas de avaliação
repetidas e CPU semelhante nas três condições disparadas mostram que o reuso
sintático atual não resolve o custo total desta fixture. A sonda posterior em
`correction-analysis.md` separa a validação de fontes do trabalho de avaliação.

## Decisão para a prioridade

A comparação pedida está concluída. O custo de recompilação após escrita dos
mesmos bytes, edição pequena e edição estrutural permanece aberto. A sonda
posterior em `correction-analysis.md` localizou o primeiro uso do catálogo de
fontes como principal candidato causal. Reuso de Module exige contrato de
dependências, efeitos, diagnósticos e identidade; reuso de layout exige
observação causal de conteúdo, estilos, contexto e páginas. O pacote anterior
`source-session-execucao-2026-10-08.md` limita o lote A a CST e não autoriza
esses contratos B/C. Nenhum código produtivo ou L0 foi alterado nesta
investigação.
