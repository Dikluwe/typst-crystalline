# Identidade de documento e expressão — 2026-10-06

A correção separa a identidade de diagnóstico da expressão da identidade do
seu documento contextual. `eval --in` preserva arquivo, texto e posições do
documento; a expressão aparece como `<input-expression>` e stdin como `<stdin>`.
Imports e callbacks conservam seus próprios Sources e traces. A base inicial
para read/import continua `world.main()`. Não há nova execução do documento.

## Evidência e proveniência

A auditoria inicial está em
`diagnostic-paths-2026-10-06/identity-next-audit.json`. A medição final está em
`origem-fontes-expressao-2026-10-06/measurement.json.gz`, com todas as fontes,
comandos, saídas completas, cwd, UTC, HEAD, estado não commitado e diff stat.
HEAD: `06848cdcb5450f2c555626bf116bc7acd82bd82b`. O receipt `execution.json.gz` preserva fontes/L0s,
seus hashes e os logs; `macro.json.gz` preserva a matriz independente.

Vanilla pin: `9f2b6e8715237cb086899a42873660fe744622e8`.
SHA-256 dos binários:

- Baseline anterior: `7c61e9b4cad5aee8995f201ce721dcce66775d222e02ccbb0aedf9e4f74a23ce`.
- Tekt final: `8af04155a7bacfa7dec3bb94cfc6faaafb74ed1a46abc010ac18a3a43144f668`.
- Vanilla: `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.

A comparação usa o baseline congelado de caminhos físicos, o release final
congelado em `/tmp/tekt-source-identity/typst-final` e o oracle ratificado,
sem projeção de nomes, stderr, espaços, hints ou posições. `measure.py` registra
cada caso separadamente, inclusive entradas inválidas e controles de sucesso.

## Resultado

| Coleta contextual/isolada | Antes | Depois |
|---|---:|---:|
| stdout, stderr e status integralmente iguais | 5/16 | 14/16 |

Os casos agora coincidentes incluem erro de documento, erro de parser de
ambas as fontes, erro de expressão, stdin, import e callback cujo erro vem
do documento e cujo trace vem da expressão. Query contextual, leitura e
import relativos preservam os controles antes coincidentes.

Os dois DIFF restantes são `document-warning` e
`warning-and-expression-error`: os caminhos/fontes agora coincidem, mas o
warning para `**` marca um caractere no Tekt e dois no vanilla. A ordem das
mensagens nesses dois controles coincide; não atribuir sua diferença à ordem.
Caret é débito de range separado. A depreciação/ordem de warnings em query e
compile stdin permanecem lacunas da coleta anterior, fora desta implementação.

A matriz macro conserva **36 MATCH/36** nos mesmos critérios anteriores:
valores de query, DOM HTML, texto/páginas e bbox com tolerância inferior a
0,05 pt, raster com vizinhança de um pixel e stderr completo nos casos de erro.
Não significa paridade global nem igualdade de bytes de todos os PDFs.
Não foi medido nem é afirmado ganho de desempenho neste reparo.

## Implementação e contrato

O gate ADR-0127 foi confirmado pelo dono com `confirmo`. O manifesto original
aprovado e seus hashes permanecem em
`00_nucleo/materialization/origem-fontes-expressao-l0-manifest-2026-10-06.json`;
a aprovação está no arquivo irmão `origem-fontes-expressao-aprovacao-2026-10-06.json`.
Após confirmação, os L0s passaram a registrar o gate realizado e foram selados
localmente. O núcleo compartilhado é
`00_nucleo/prompts/_nuclei/expression-source-identity.toml`.

- L3 registra expressão e stdin em RAM com Source e bytes resolvíveis. Fontes
  transientes não entram em dependencies nem no mapa canônico de arquivos.
  Arquivos reais com nomes `<stdin>`/`<input-expression>` não compartilham seus ids.
- O alocador por World não recicla IDs. Exaustão da fábrica transiente retorna
  FileError; `register_file` mantém assinatura e emite panic explícito para uma
  identidade nova sem capacidade, soltando locks antes, sem sobrescrever fontes
  ou envenenar os caches. Identidades conhecidas continuam disponíveis.
- L1 recebe Source code pronta, sem inventar ids, reparse ou I/O. A base inicial
  da Engine/Route continua main; o overlay só atende a identidade da expressão.
- L3 coordena a compilação contextual existente e reaproveita seu introspector
  final. A fachada string contextual delega à mesma coordenação Source-based.
- L4 adquire stdin/cwd, compõe as entradas e entrega somente a expressão como
  primary ao formatter existente. A identidade distinta permite resolver as
  demais fontes pelo World.

São cinco entradas públicas aditivas: for_stdin, source_for_expression,
eval_source_with_features_and_introspector, eval_source_with_sink_features e
eval_source_with_document_features. As entradas string antigas conservam
assinaturas e sua identidade legada; callers que precisam de Sources independentes
usam as entradas novas. A CLI foi migrada. Não foram alterados traits, entidades
públicas, fases, flags ou serialização, e não foi acrescentado unsafe.

A ordem de publicação transiente é next_id → read_cache → slots, compatível
com o leitor de bytes existente. O detalhe foi corrigido no L0 antes do código,
pois slots → read_cache permitiria inversão com o leitor.

## Validação

Dois controles CLI foram escritos antes do código e falharam por ausência
real de origem; `red-cli.log`/`green-cli.log` preservam RED→GREEN. A tentativa
inicial usou nome de package errado; foi corrigida para typst-wiring antes das
falhas significativas. `red.json` distingue o snapshot coletado posteriormente
da linhagem produtiva pré-correção preservada em `specification-state.json`.

Passaram:

- 111 testes da CLI completos, serialmente, incluindo watch/HTTP local.
- Um teste L1 cobrindo as novas fontes, erros sintáticos/semânticos e os quatro
  perfis de features, com controles das fachadas antigas.
- Cinco testes focados L3: fontes/caches, nomes físicos iguais aos marcadores,
  concorrência, exaustão e os dois controles do pipeline.
- 39 testes da família World; os três novos testes World acima estão incluídos,
  portanto não somar essas contagens como uma suíte distinta.

O teste do pipeline conta o efeito real de read do programa comum: exatamente
uma leitura na compilação contextual; uma leitura posterior da expressão é
observada separadamente e verifica main como sua base, embora seu Source tenha
outro id. Não confundir esse controle com número de passagens de layout.

A suíte CLI completa usou o binário debug com SHA-256
`ac1a6853a54dad9e40e3d4c214ba9f34304db36f976a128fcbcd29e9d2663fd4`.
Depois dela houve apenas atualização de headers de linhagem nos cinco consumers
produtivos e refinamento do teste de base no pipeline. O receipt verifica que
os corpos produtivos são iguais excluindo prompt-hash/updated. Os cinco testes
focados L3 foram repetidos após esse refinamento; release final e matriz bilateral
foram medidos após o resselo final.

Build debug/release e crystalline-lint passaram. O linter reportou **0 erros,
531 warnings e 1577 notes**, o estado global anterior; os nove owners deste
pacote não têm V5/V6/V15/V26. Não afirmar zero violations globais.

## Atualização — carets vazios corrigidos, 2026-10-06

O alcance de **/__ foi corrigido pela relação real entre o marcador de warning
e seu construct na Source numerada, sem alterar a CST ou fases. A coleta de
identidade passou de 14/16 para 16/16 integrais; seis controles adicionais e a
matriz macro de 36 mantiveram seus critérios. Ver
`00_nucleo/diagnosticos/empty-markup-warning-2026-10-06.md` e receipt. Estes dois
negativos estão fechados; demais frentes permanecem conforme seus próprios
diagnósticos. Não é prova de paridade global nem de ganho de tempo.
