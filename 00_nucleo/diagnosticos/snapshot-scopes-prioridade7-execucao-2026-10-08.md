# Prioridade 7 — cópia inicial do prefixo de captura

Mantida uma otimização interna em Scopes::snapshot: copiar o primeiro mapa
completo e aplicar os overlays restantes, em vez de reconstruir esse primeiro
mapa por reinserção. A CLI com 1.000 definições de closures ficou 7,59% mais rápida
pela mediana das razões pareadas, com ganho nos três blocos. Não é aceleração
geral do compilador nem cache incremental de avaliação.

## Proveniência

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado.
Todos os recibos abaixo ficam em `snapshot-scopes-prioridade7-2026-10-08/`.
`before.json.gz` guarda lista integral de arquivos alterados, git diff HEAD --stat,
textos dos fontes/L0, UTC e binários. `final-state.json.gz` guarda o estado medido;
`scope.json` verifica delta produtivo/L0 restrito a:

- `01_core/src/compiler/scopes.rs`;
- `00_nucleo/prompts/compiler/scopes.md`.

A entidade Scope, seu L0, representação e métodos permaneceram idênticos ao
estado inicial: não reaplicamos o compartilhamento indiscriminado com Arc.
`completion.json` verifica fontes/L0/binary após os testes. Código `f460d087`,
prompt `35237067`, SHA completos em `seals.json`; sem resselo global ou núcleo novo.

Release anterior: `17e721496fc8011021c73791293cdcf1f968aff5eba4ce322529fe3fa1e8ba8b`.
Atual: `398b464d2a1062120928dfaf58351c43acd4a6769af74ac8ee1fc14f31533a14`.
Vanilla pinado `9f2b6e8715237cb086899a42873660fe744622e8`, binário
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
Não usar apenas string de versão como prova.

UTC em 2026-10-08: baseline 17:07:19; sondas 17:14:28–17:15:59;
estado candidato 17:28:14; controles finais 17:36:48–17:36:57;
CLI 17:40:40–17:41:27. Timestamps completos e comandos nos recibos.

## Medição antes da decisão

scopes.rs:84–103 no snapshot anterior reinseria todos os nomes/valores da base,
captured, pilha e top. closures.rs:196 e expression_dispatch.rs:505 capturam na
definição. Sondas externas GDB, sem editar produção, confirmaram:

| Caso | Capturas | Bindings de entrada | Bindings do primeiro prefixo |
| --- | ---: | ---: | ---: |
| 10 definições de closure | 10 | 2.030 | 2.010 |
| 100 definições de closure | 100 | 20.300 | 20.100 |
| Uma definição, 1.000 chamadas | 1 | 201 | 201 |
| 100 contextos | 100 | 20.200 | 20.100 |
| Closure aninhada | 2 | 404 | 402 |

A contagem usa DWARF do debug para ler campos/len, sem estimar offsets.
Os casos comuns concordaram com contagem de chamadas no release. A sonda release
contou 1.000 snapshots em closures1000 e uma montagem de stdlib por caso.
Logo, a medição histórica de duas passagens/montagens em outubro de 2026 não é
prova para esta árvore atual. Tempos sob GDB incluem perturbação por traps e não
foram usados para atribuir percentual da CLI ao snapshot.

`probes.json.gz`, `probe-summary.json` e scripts preservam os dados. O controle
plain inicial retornava inteiros que não podem ser concatenados: foi corrigido
para markup. A amostra GDB plain100 original foi excluída por saída prematura;
as fontes antigas e a exclusão estão em `invalid-plain-controls.json` e nos
recibos. Recheck no release congelado completou com status zero e zero snapshots;
a afinidade desse recheck não foi fixada, portanto seus tempos não foram usados.
Os outros fixtures da sonda foram qualificados como compilações válidas.

O protótipo externo comparou Scopes real e cópia inicial de prefixo, em dados
inteiros sintéticos, sem mudar produção: cinco pares por configuração, uma
warmup e 2.000 snapshots/amostra. Base201: 34,09→18,94 ms; captured201:
37,47→20,79 ms; ambos201: 37,50→20,93 ms. Redução de cerca de 44% em todos os
pares desses casos (`prototype.json`, `prototype.rs.txt`). Não confundir esses
inteiros com a biblioteca padrão completa nem o helper local com a CLI final.
A inspeção do indexmap pinado 2.13.0 confirma cópia dos índices e reserva da
capacidade correspondente (`indexmap-clone.txt`/`.json`); nenhuma suposição sobre
crescimento de Vec foi usada para prometer economia de memória.

## Materialização e aceitação

L0 atualizado antes do código. Base presente: copiar base.global e aplicar
captured, pilha antiga→nova e top. Sem base e com captured: copiar captured e
aplicar pilha/top. Sem prefixo: manter Scope::new e as inserções locais.
Continua sendo snapshot eager; nomes/valores são copiados e isolados. A mudança
evita reinserção/hashing/crescimento do primeiro mapa, não elimina as cópias dos
201 bindings por captura. Capturas não passam a observar o estado posterior.

Assinaturas/campos públicos, Scope, Capturer, precedência, ordem, diagnósticos,
fases/defaults e loops sem captura permanecem sob o contrato vigente.
Um helper privado recebe FnOnce estático para construir a cópia inicial; não é
callback da linguagem. Sem I/O, unsafe, globals ou crate novo em L1. Alteração
interna reversível em fluxo contínuo ADR-0127, owner 1:1.

RED válido: dois testes falharam porque o builder de cópia inicial era chamado
zero vezes onde exigiam uma, após os asserts funcionais de ordem/valores/isolamento
passarem (`red.log`, `red-state.json.gz`). O adapter privado no RED preservava o
algoritmo anterior. GREEN: 25 testes de escopos passaram, incluindo os dois novos
para base+captured, captured sem base, overlays, isolamento e fallback local.
Esse testemunho conta construção do prefixo; não conta malloc global nem prova
paridade por igualdade de ponteiros.

## CLI completa

Três rodadas de seis pares por fixture, uma warmup por motor/bloco, ordem
alternada, processos novos, CPU fixada, Rayon 1, SOURCE_DATE_EPOCH=0, PDF sem tags
ou pretty. Builds/testes/GDB terminaram antes da coleta. `bench.json.gz` contém
216 amostras, comandos, UTC, HEAD/árvore e fingerprints de fontes/fontconfig/
configuração/binários. `analyze.py` produz `summary.json`.

| Fixture | Parede anterior, mediana | Atual, mediana | Variação pareada | Três blocos |
| --- | ---: | ---: | ---: | --- |
| 1.000 definições de closure | 137,17 ms | 126,45 ms | −7,59% | −7,68%, −7,76%, −7,45% |
| 300 contextos | 318,03 ms | 319,56 ms | −0,77% | −1,46%, −0,77%, +3,55% |
| Loop sem captura | 113,61 ms | 112,90 ms | −0,43% | −1,05%, −0,23%, −0,30% |
| Hello | 112,62 ms | 112,03 ms | −0,38% | +0,23%, −0,18%, −0,77% |
| Uma closure, 1.000 chamadas | 113,98 ms | 114,56 ms | +0,37% | +0,67%, +0,19%, +0,22% |
| Markup sem contexto | 134,59 ms | 134,35 ms | −0,38% | +0,17%, −0,07%, −0,84% |

Percentuais são medianas das razões pareadas, não razões de medianas absolutas.
Em context300 essas duas estatísticas têm direções distintas; não certificar ganho
total nesse caso. Avaliação de closures1000: 125,55→115,04 ms, −8,22% pareado,
três blocos melhores. Avaliação de context300: 119,61→116,32 ms, −2,78% pareado,
três blocos melhores. O timer eval inclui outros serviços, não é tempo exclusivo
de snapshot. O caso de definição única teve pequena piora repetida de 0,37% na
parede e 0,31% em eval; não apresentar como ganho nem como prova causal da mudança.

RSS não demonstrou redução: context300 200.940→201.190 KiB, definição única
43.566→43.878 KiB e closures1000 43.758→43.684 KiB. Não são perfis causais de
retenção/alocação. `sizes.json` mede somente tamanhos físicos (Value/Binding 272,
Scope 56 e Content 120 bytes nesta compilação), não heap ou origem do RSS.
Sem teste estatístico formal; não somar percentuais a lotes anteriores.

Decisão: manter pelo ganho forte/repetido de definição de closures, redução
repetida de eval em contextos e ausência de regressão relevante nos controles.
Não declarar melhora para chamadas já capturadas, toda compilação, memória ou
paridade com vanilla.

## Verificação final

18 controles: 16 sucessos conservaram texto, caixas/dimensões, raster a 36 dpi e
identidade de fontes embutidas; dois erros conservaram diagnóstico completo.
`geometry.json.gz` e `completion.json` guardam a comparação anterior/atual.
Com vanilla, 12 sucessos coincidiram textualmente; context100/context300 e
plain100/plain300 já divergem no anterior. Os dois erros coincidem em status e
primeira linha da mensagem; campos text ausentes não são prova de paridade.
Não certificamos paridade global ou bytes PDF.

Core: 6.644 passaram, zero falhas, cinco ignorados, com RUST_MIN_STACK=67108864
somente nas threads de teste, como no lote anterior. Infra: 1.247 passaram,
zero falhas, dois ignorados. Builds dev/release offline passaram. Logs e comandos
em `completion.json`; o ajuste de pilha não muda defaults do produto.
`crystalline-lint .`: zero errors, 535 warnings e 1.640 infos herdados, sem
certificado de zero violations globais. Fontes/L0 e release medidos permaneceram
idênticos até o fechamento; nenhum outro owner foi alterado neste lote.
