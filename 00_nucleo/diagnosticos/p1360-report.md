# P1360 — relatório final de `compile --pretty`

## Veredito

**PASS_WITH_LIMITED_ATTESTATION.** O fragmento P1360 está materializado: o
CLI conserva o tri-state, PDF/SVG/HTML recebem formatação por target, PNG fica
neutro, os eixos PDF permanecem independentes e os processos usados na prova
foram contidos. O veredito cobre somente este fragmento; não declara
equivalência funcional geral com o vanila.

O protocolo completo foi tentado, mas o workspace partilhado não permite
atestar isolamento forte. A formulação precisa é **executado sem atestação de
isolamento**, com autoria e gate estático segregados por autoridade declarada.

## Correção causal durante a execução

A primeira obrigação dizia que `--pretty false` deveria ser rejeitado. Um
teste que realmente executou — depois de dois filtros iniciais terem casado
zero testes — mostrou que o cristalino aceitava a forma. A medição corretiva
do vanila ratificado confirmou o mesmo comportamento: `--pretty` fica
`Some(true)` e `false` é consumido pelo positional `output`; o processo só
falha mais tarde ao não inferir o formato do caminho `false`.

Por isso, não se alterou a implementação para satisfazer uma hipótese falsa.
Foram regenerados preflight, L0 de CLI, contrato, manifesto, oráculos,
mutantes, gate, selo e teste. O selo vigente é
`58e0ee80bcffbe60d3b234cce15da334cce6488f8b26e07d8d6b76269cf12182`.
O novo M03 possui dois ataques: consumir o token como `Some(false)` e rejeitá-lo
artificialmente como booleano. Ambos são `Violated`.

## Matriz de evidência

| Obrigação | Evidência | Resultado |
|---|---|---|
| Entradas protegidas | hashes atuais de preflight, passo, 7 L0, contrato, manifesto, oráculos e mutantes coincidem com o selo revisado | Preserved |
| Poder discriminatório | 43 positivos, 6 rejeições válidas e 10 opacos; 36/36 mutantes `Violated`; `Unknown` recebe crédito zero | score 1.0 |
| CLI tri-state | `p1360-reseal-shell-tristate.stdout`: 1 executado, 1 passou | PASS |
| CLI token separado/inválidos | `p1360-reseal-shell-separated.stdout`: 1 executado, 1 passou | PASS |
| PDF page stream | teste focal do builder: 1 executado, 1 passou; sondas mostram Flate no default/false e stream de página claro em pretty | PASS |
| HTML | teste focal preserva phrasing, `title`, `>` em atributo e dois modos de escaping: 1 executado, 1 passou | PASS limitado |
| SVG | teste focal compara conteúdo sem whitespace e confirma indentação: 1 executado, 1 passou | PASS |
| PNG | artefactos default/true têm 4344 bytes e SHA-256 idêntico | PASS |
| Ortogonalidade PDF | `compact+pretty` continua sem Flate no page stream; `untagged+pretty` não possui marcadores de tags; recursos continuam comprimidos | PASS no fixture |
| Build | recibo termina em `Finished dev profile`, exit 0 informado | PASS |
| Arquitetura | `p1360-lint-reseal.stdout`, exit 0 informado, nenhum `error:`; stderr vazio | PASS |
| Formatação | `p1360-fmt-final.{stdout,stderr}`, executado após o último rustfmt: `cargo fmt --all -- --check`, exit 0, ambos vazios | PASS |
| Diff textual | `p1360-diff-check-final.{stdout,stderr}`, executado após o último rustfmt: `git diff --check`, exit 0, ambos vazios | PASS |

Os cinco testes creditados têm recibos que mostram explicitamente `running 1
test` e `1 passed`. Os recibos anteriores que mostravam `running 0 tests` e o
teste da obrigação obsoleta não recebem crédito.

## Implementação observada

- L2 guarda `CompileArgs.pretty`/`CompileIntent.pretty` como `Option<bool>`.
- L4 resolve ausência para `false`; PDF recebe `PdfSerializationMode`, SVG
  recebe `SvgOptions.pretty`, HTML recebe a variante pretty e PNG não recebe o
  parâmetro.
- `PdfSerializationMode` é distinto de `StreamMode` e `PdfTags`; somente
  content streams de página deixam de ser comprimidos em `Pretty`.
- HTML formata a string compacta com scanner de tags sensível a aspas e
  contextos protegidos; SVG reutiliza o writer existente.
- Wrappers de compatibilidade continuam a delegar com pretty desativado.

## Contenção do estouro de memória

Nenhum `watch` ou alias `w` foi iniciado. Esses comandos apareceram somente no
parser in-process. As sondas executaram apenas `compile`, uma por vez, com
`timeout` máximo de 20 segundos e stdout/stderr em ficheiros. Cargo usou
`CARGO_BUILD_JOBS=1`; não houve suíte ampla nem filtro capaz de iniciar o
teste watch perigoso.

Os 14 artefactos candidatos somaram 74.216 bytes; o maior teve 8.402 bytes,
abaixo do teto de 4 MiB. No preflight, o maior controlo teve 936.603 bytes,
também abaixo do teto. Não houve timeout, processo persistente nem violação de
tamanho observada. Esta prova demonstra contenção de processos e artefactos;
não houve telemetria direta de RSS/heap, portanto não se afirma um teto de
memória residente.

## Limitações de atestação

- Os papéis partilharam filesystem gravável; as allowlists foram disciplina
  declarada, não isolamento tecnicamente imposto.
- Um rascunho do autor de testes começou antes do primeiro selo e foi removido.
- O teste A/B do builder foi revisto depois do primeiro RED.
- A primeira hipótese de parsing estava errada e invalidou a cadeia antiga;
  somente a cadeia revisada é válida.
- Os recibos dos três testes de infra antecedem uma troca HTML mecanicamente
  equivalente (`slice` para `raw.get`) feita para o parser do linter. O teste
  HTML foi reportado como revalidado, mas não há recibo novo distinguível.
- O mutation score é de um gate sintético binding-free; os 36 mutantes não
  foram aplicados ao código produtivo.
- A suíte ampla foi deliberadamente omitida por segurança de memória.

Os detalhes reproduzíveis e hashes estão em
`diagnosticos/p1360-verification.json`.
