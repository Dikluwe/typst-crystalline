# Prioridade 7 — experimento de compartilhamento de Scope

O experimento foi rejeitado e o consumer/L0 foram restaurados exatamente ao
estado inicial. Compartilhar todos os mapas reduziu drasticamente o custo de
clone, mas introduziu custo repetido em loops com escopos pequenos. Não há
mudança produtiva de Scope retida. A otimização anterior de coleta de chaves de
fontes permanece.

## Proveniência e medição anterior à decisão

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado.
Os arquivos alterados herdados e `git diff HEAD --stat` constam integralmente
em `scope-cow-prioridade7-2026-10-08/before.json.gz`, junto dos textos de fontes/L0.
Os caminhos dos recibos abaixo pertencem a essa pasta.

Binário inicial/restaurado: `17e721496fc8011021c73791293cdcf1f968aff5eba4ce322529fe3fa1e8ba8b`.
Candidato Arc: `774d1b4754137d7d1dc55c5e50cdbe5153d92504fa2210f592df4243cff27517`.
Vanilla pinado `9f2b6e8715237cb086899a42873660fe744622e8`, binário
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
Snapshot inicial em 2026-10-08 15:30:41 UTC; candidato em 15:34:27 UTC;
CLI em 15:42:14–15:42:43 UTC; decisão em 15:46:40 UTC.
Timestamps completos em recibos; `checks.json` registra comandos, ambientes,
hashes dos logs/executável de teste e snapshots associados.

Antes do experimento, `entities/scope.rs:62–65` possuía IndexMap privado e
derive(Clone); esse clone copia nomes e bindings. `compiler/eval/mod.rs:605,845`
e `compiler/eval/modules.rs:66` clonam stdlib para o namespace std. O clone de
`stdlib/sym.rs:671` é seguido de escrita; nesse percurso a cópia na escrita
não elimina a cópia. `compiler/scopes.rs:84–103` reconstrói snapshots por define,
fora do escopo deste experimento. O L0 vigente e ADR-0029/0030/0127 foram lidos.

O ensaio exploratório anterior à alteração (`micro-before.json`) mediu o Scope
real numa biblioteca release identificada por SHA, sem atribuir essa biblioteca
à identidade de linkage do executável CLI: 10.000 clones de 128 bindings em
64,09 ms e de 1.024 bindings em 479,52 ms. A hipótese de ganho na CLI permaneceu
aberta. As identidades/horas e o harness estão no recibo.

## Materialização experimental

L0 atualizado antes do código; storage privado Arc<IndexMap<...>> e
Arc::make_mut em define/get_mut existente. Busca ausente retorna None sem copiar
um mapa compartilhado. Nenhuma assinatura pública, campo público, fase ou modo
padrão mudou; experimento reversível em fluxo contínuo ADR-0127. Resselo restrito
a `scope.rs`/`entities/scope.md`, código `77b16d46`, prompt `ddf77a77`.
`arc-seals.json` guarda SHA completos. Nenhum outro owner ou núcleo mudou.

RED válido: um teste passou para isolamento semântico e um falhou porque os
bindings do clone tinham armazenamento distinto (`red.log`, `red-state.json.gz`).
O endereço é testemunho privado de cópia, nunca critério de paridade da linguagem.
GREEN: nove testes do owner passaram, incluindo isolamento dos dois escritores,
múltiplos clones, redefinição, nomes novos, ordem e lookup ausente (`green.log`).
O código candidato está preservado no snapshot `arc-final-state.json.gz`.

## Medição comparativa

O ensaio comparativo compila os dois textos exatos de Scope em harnesses iguais,
com o mesmo Value real e bibliotecas, flags rustc -O e fronteira de compilação.
Isso evita comparar bibliotecas release com configurações diferentes. Comandos,
hashes e harnesses estão em `micro-build.json`; amostras em `arc-micro-paired.json`.
Uma warmup e cinco pares de processos em ordem alternada, afinidade fixa.
A primeira coleta coincidiu parcialmente com a verificação da CLI: foi excluída,
preservada como `micro-paired-excluded-concurrent.json` e repetida isoladamente.

| Ensaio isolado | Anterior | Arc | Observação |
| --- | ---: | ---: | --- |
| 10.000 clones, 128 bindings | 67,27 ms | 0,034 ms | cópia eliminada |
| 10.000 clones, 1.024 bindings | 502,92 ms | 0,034 ms | cópia eliminada |
| 10.000 clones + leitura, 1.024 bindings | 500,97 ms | 0,103 ms | cópia eliminada |
| 10.000 clones + escrita, 1.024 bindings | 505,03 ms | 502,15 ms | cópia continua necessária |
| 100.000 criações + define/get_mut, um binding | 9,61 ms | 11,76 ms | cerca de +22% |

Medianas por motor. Esses tempos não representam a CLI nem contagem global de
malloc. O custo de criação de mapas pequenos refuta a hipótese de benefício
uniforme da troca de storage.

CLI: três rodadas, seis pares por fixture/rodada, uma warmup por motor/bloco,
ordem alternada, processos novos, afinidade fixa, Rayon 1, PDF sem tags/pretty,
SOURCE_DATE_EPOCH=0. Build/testes/harnesses terminaram antes dessa coleta.
`arc-bench.json.gz` contém comandos, amostras, UTC, HEAD/árvore, fontes/fontconfig,
binários e fingerprints; `arc-summary.json` resulta de `analyze.py`.

| Fixture | Anterior, mediana | Arc, mediana | Variação pareada | Três blocos |
| --- | ---: | ---: | ---: | --- |
| Hello | 113,59 ms | 112,49 ms | −0,70% | −0,96%, +0,22%, −1,40% |
| 30.000 iterações | 134,27 ms | 135,09 ms | +0,70% | +0,90%, +0,62%, +0,75% |
| 3.000 chamadas de closure | 122,35 ms | 121,03 ms | −0,76% | −0,60%, +0,04%, −0,87% |
| 12 imports | 125,77 ms | 124,80 ms | −1,05% | −1,43%, −0,77%, −0,40% |

Variações são medianas das razões pareadas, não razões das medianas absolutas.
O total interno repetiu a direção nos loops (+0,91%) e imports (−0,68%).
Sem teste estatístico formal ou promessa de resultado universal. O resultado
consistente nos loops, junto do controle isolado de criação/mutação, é evidência
compatível com custo adicional de escopos pequenos; não é um perfil causal que
atribua cada milissegundo à alocação de Arc.

Decisão registrada em `decision.json`: rejeitar compartilhamento indiscriminado.
A melhoria de clone é real, mas o compilador faz muitas criações/mutações de
scopes e nem todas são seguidas de clone. Não trocar uma regressão repetida nos
loops por um ganho pequeno num subconjunto sem uma política melhor medida.

## Controles, testes e restauração

Cinco casos de sucesso preservaram texto, caixas/dimensões e raster a 36 dpi
contra o anterior; dois casos de erro conservaram diagnóstico completo. Os
cinco textos coincidiram também com vanilla. Nos erros, status e primeira linha
da mensagem coincidem com vanilla, mas igualdade de campos text ausentes não é
prova de paridade. `arc-geometry.json.gz`/`arc-controls.json` guardam os controles.
Isso não certifica paridade global.

A execução ampla core com stack padrão abortou num teste de recursão; a repetição
com 16 MiB abortou noutro. Ambos os aborts foram reproduzidos no executável de
testes com implementação anterior. Com RUST_MIN_STACK=67108864, a suíte candidata
passou: 6.644 testes, zero falhas, cinco ignorados. A baseline com os dois testes
experimentais excluídos passou: 6.642, zero falhas, cinco ignorados. Logs
`core-*.log`; nenhum limite/pilha/default do produto foi aumentado. O ajuste
valeu somente para threads do executável de testes. O release da CLI foi medido
com o ambiente habitual descrito acima.

Antes de restaurar, preservamos código, L0, selos, medições e release SHA em
`arc-final-state.json.gz`, `arc-seals.json` e `decision.json`. A restauração usa
o texto exato dos dois owners em `before.json.gz`, sem git reset/checkout ou
resselo global. O recibo de fechamento verifica todos os fontes/L0 contra o
snapshot inicial e registra builds, lint e identidade do binário restaurado.

Um novo experimento precisará distinguir scopes construídos e mutados localmente
de snapshots realmente clonados. A cópia eager em Scopes::snapshot permanece
uma candidata distinta, ainda sem medição de custo/benefício ou implementação.


Fechamento: `completion.json` verifica igualdade integral dos fontes/L0 com o
snapshot inicial e identidade byte a byte do release inicial/restaurado.
Builds dev e release offline passaram. Lint: zero errors, 535 warnings e 1.640
infos herdados; exit code zero não significa zero violations globais. Nenhum
rebuild/teste concorreu com a coleta final de tempos. A suíte infra não foi
repetida após a restauração: não há delta produtivo/L0, o release é byte-idêntico
e a suíte core baseline foi verificada. O lote anterior de recursos já tinha
1.247 testes infra aprovados; esse número não é apresentado como nova execução.
