# Prioridade 7 — construção de chaves de fontes

Aplicada uma redução interna de cópias na coleta textual de fontes. O caminho
isolado ficou mais rápido de forma repetida; a CLI completa não demonstrou ganho.
Este lote cobre a coleta de chaves, não encerra todas as oportunidades de
alocações e travessias do compilador.

## Estado e proveniência

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado.
A lista integral de arquivos alterados e `git diff HEAD --stat` estão em
`font-keys-prioridade7-2026-10-08/before.json.gz`, `final-state.json.gz` e
`completion.json`. Os snapshots guardam o texto dos fontes/L0, não apenas HEAD.
O delta deste lote é exclusivamente:

- `03_infra/src/pipeline/resources.rs`;
- `00_nucleo/prompts/infra/pipeline/resources.md`.

`scope.json` verifica ausência de mudanças produtivas fora desse par. O recibo
final confirma que fontes/L0 não mudaram depois do snapshot usado nas medições.
Nenhum hash global foi ressellado; ownership e pins existentes foram preservados.
Código `f4c1f68a`, prompt `459e60be`; SHA-256 completos em `seals.json`.

Binários comparados:

- Anterior, após prioridade 6: `d76926491304bb6d2291be082542f3199619a67757916c3f11485052d18d5371`.
- Atual: `17e721496fc8011021c73791293cdcf1f968aff5eba4ce322529fe3fa1e8ba8b`.
- Vanilla pinado `9f2b6e8715237cb086899a42873660fe744622e8`,
  binário `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.

Snapshot anterior: 2026-10-08 14:46:13 UTC. Final produtivo: 14:54:06 UTC.
Microensaio: 14:54:32 UTC. Saída: 14:56:04–14:56:09 UTC.
CLI: 14:58:25–14:59:27 UTC. Os recibos guardam timestamps completos, comandos,
fontes, fixtures, opções, afinidade e identidades; todos os caminhos deste relatório
referem-se à pasta `font-keys-prioridade7-2026-10-08/`.

## Evidência anterior à decisão

No snapshot anterior, `resources.rs:127–134` construía um key owned com clones de
FontList e variações antes de procurar um duplicado. Ambos contêm Vec; ocorrências
repetidas descartavam a construção. Essa cópia é mecânica interna (ADR-0107).
A ordem e a seleção por família/variante/eixos continuam sob o contrato vigente.
O L0 foi atualizado antes do código; experimento privado e reversível em fluxo
contínuo (ADR-0127), sem novo contrato público ou comportamento padrão.

## Mudança e teste causal

O helper privado compara por empréstimo e executa a construção apenas no miss.
Mantém primeira ocorrência, igualdade de listas/variante/eixos, None versus lista
vazia, zeros com sinal e NaN. Não usa HashSet, normalização, estado global ou
callback da linguagem. Glyph conserva a resolução da fonte efetiva de matemática;
a recursão e os percursos background/items/foreground permanecem.

RED válido: os dois testes novos falharam por excesso de construções
(`red.log`/`red-state.json.gz`); o controle repetido encontrou 1.000 construções onde
exigia uma. GREEN: os mesmos testes passaram, incluindo ordem, variantes/eixos,
vazio, zero e NaN. Isso conta construção de keys owned, não chamadas globais de
malloc, memória total ou todas as alocações do documento. Erros de compilação
anteriores à execução dos testes não foram usados como prova RED.

## Tempo isolado e CLI

Microensaio release, 10.000 itens preparados fora do cronômetro, uma warmup e
30 pares por execução, três execuções isoladas com ordem alternada. O lado legado
reproduz a construção/comparação anterior; o atual executa o collector produtivo.
Os asserts verificam a cardinalidade. Executável, SHA, afinidade, snapshot e
amostras estão em `collection-micro.json`; a sonda cfg(test) é ignorada na suíte
normal e não instrumenta produção.

| Coleta isolada | Anterior, mediana | Atual, mediana | Resultado repetido |
| --- | ---: | ---: | --- |
| Uma família, sem eixos | 0,443 ms | 0,114 ms | cerca de 74% menor |
| Uma família, dois eixos | 0,558 ms | 0,138 ms | cerca de 75% menor |
| 24 famílias, dois eixos | 1,216 ms | 0,796 ms | cerca de 35% menor |

CLI: três rodadas de dez pares por fixture, processos novos, ordem alternada,
uma warmup por bloco/motor, Rayon 1, afinidade única, PDF sem tags/pretty e
SOURCE_DATE_EPOCH=0. Nenhum build/teste concorreu com os pares. Receitas e todos os
fingerprints de fontes/fontconfig estão em `bench.json.gz`; `analyze.py` produz
`summary.json`. As variações abaixo são medianas das razões pareadas, não razões
das medianas absolutas.

| Fixture | Anterior, mediana de parede | Atual, mediana de parede | Variação pareada |
| --- | ---: | ---: | ---: |
| Hello | 113,36 ms | 113,72 ms | +0,42% |
| Texto repetido | 258,90 ms | 259,39 ms | +0,20% |
| Eixos | 327,73 ms | 329,84 ms | +0,26% |
| Famílias alternadas | 169,71 ms | 170,24 ms | +0,57% |

Os tempos totais internos também não demonstram ganho. Não há teste estatístico
formal nem atribuição causal da pequena piora à mudança: fases não alteradas,
como layout, também variaram. RSS ficou próximo e não comprova economia de memória.
Não somar os percentuais isolados às prioridades anteriores ou extrapolar à CLI.

Decisão: conservar a redução pequena e privada porque elimina construções
descartáveis, tem benefício isolado repetido e preserva os controles de saída.
A prioridade 7 não recebe certificado de aceleração geral da compilação.

## Verificação e limites

Seis fixtures preservaram texto, caixas/dimensões, raster a 36 dpi, identidade de
fontes embutidas e diagnósticos contra o binário anterior. Incluem famílias, eixos,
matemática, camadas, link e bloco. `geometry.json.gz` contém dados por motor;
`completion.json` compara colunas de fontes sem prefixo de subset/IDs de objeto.
A comparação textual vanilla coincide apenas em Hello; os outros cinco controles
já divergem no binário anterior. Estes ensaios demonstram preservação, não fecham
essas divergências nem certificam paridade global.

Builds dev e release offline passaram. Suíte infra: 1.247 passaram, zero falhas,
dois ignorados (inclui a sonda de tempo). Testes causais: dois passaram, uma sonda
ignorada. Logs `dev-build.log`, `release-build.log`, `infra.log`, `green.log`.
`crystalline-lint .`: zero errors, 535 warnings e 1.640 infos já presentes na árvore;
o sucesso de exit code não significa zero violations globais (`lint.log`).

## Próximo candidato auditado, ainda sem implementação

`Scope` usa IndexMap privado e derive(Clone) (`entities/scope.rs:62–65`). Há clones
da stdlib em `compiler/eval/mod.rs:605,845` e `compiler/eval/modules.rs:66`, além de
espalhamento dos bindings. `entities/scope.md` mantém ordem/lookup e contrato de
binding; os hashes dos arquivos auditados estão em `completion.json` e os textos
no snapshot final. Compartilhamento com cópia na escrita é uma hipótese para
reduzir esses clones. Esta auditoria não mede seu peso na CLI e não legitima
implementá-la como ganho confirmado. Antes de materializar: medir incidência/custo,
atualizar L0 e controlar isolamento das mutações, ordem, imports e closures,
incluindo o custo adicional em scopes pequenos/vazios. Nenhum código de Scope
foi alterado neste lote.
