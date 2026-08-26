# P1215 — fechar os spans diagnósticos dos métodos públicos de `bytes`

**Estado:** ESCRITO — AGUARDA EXECUÇÃO  
**Predecessor causal:** P1214  
**Mapa:** `language-bytes`, atualmente `parcial`  
**Fila:** `bytes-methods`, atualmente `PARTIAL`

## 1. Objetivo

Fechar a única lacuna nominal deixada pelo P1214: os 21 casos de erro de
`bytes.len`, `bytes.at` e `bytes.slice` já preservam exit status e mensagem
central, mas o cristalino não reproduz integralmente a localização mostrada
pelo vanilla.

Este passo deve descobrir primeiro onde o span se perde e corrigir somente o
owner causal. Não deve alterar valores, mensagens já verdes, superfície de
métodos ou representação de `Bytes`.

Resultado terminal pretendido:

```text
BYTES DIAGNOSTIC SPANS GREEN — LANGUAGE-BYTES DECLARED-CLOSED
```

Se qualquer posição continuar diferente:

```text
BYTES DIAGNOSTIC SPANS PARTIAL — LANGUAGE-BYTES REMAINS PARTIAL
```

## 2. Baseline protegido

Usar como entradas imutáveis:

- vanilla ratificado `a51e02804`;
- `00_nucleo/diagnosticos/p1214-bytes-oraculos.tsv`;
- `00_nucleo/diagnosticos/p1214-bytes-resultados.tsv`;
- `00_nucleo/diagnosticos/typst-p1214-bytes-paridade.md`;
- relação `language-bytes` do mapa DSM;
- L0s vigentes de `entities/args`, `compiler/eval/call_dispatch`,
  `compiler/eval/bindings/value_methods`, `compiler/eval/bindings/method_dispatch`,
  `compiler/stdlib/collections` e do renderer diagnóstico que a medição apontar.

Congelar HEAD, `git diff HEAD --stat`, horário e SHA-256 dos dois binários,
mapa, matriz P1214 e lente antes de decidir.

## 3. Medição antes da decisão

Não assumir que `Args::span` está detached. O código vigente declara
`eval_args(...): Args { span: args_node.span() }`; portanto, o laudo P1214 é
evidência do observável final, não prova da camada onde a informação se perde.

Para uma amostra mínima representativa, executar nos dois binários:

1. `bytes((1,2,3)).at()` — ausência de argumento;
2. `bytes((1,2,3)).at("1")` — tipo do primeiro argumento;
3. `bytes((1,2,3)).at(3)` — limite;
4. `bytes((1,2,3)).at(0,foo:1)` — named desconhecido;
5. `bytes((1,2,3)).slice()` — ausência de start;
6. `bytes((1,2,3)).slice(0,"2")` — tipo de end;
7. `bytes((1,2,3)).slice(0,4)` — fim fora do limite;
8. `bytes((1,2,3)).slice(0,1,2)` — posicional extra;
9. `bytes((1,2,3)).first()` — método inexistente.

Medir cada expressão por dois caminhos:

- `typst eval`, preservando stdout, stderr e status;
- um ficheiro `.typ` compilado, preservando linha, coluna, extensão sublinhada,
  hints e trace.

Repetir pelo menos um caso em linha 1 e linha 4, com indentação distinta. Uma
coincidência de mensagem sem acompanhar o deslocamento não prova span correto.

## 4. Instrumentação causal temporária

Antes de escrever a correção, observar sem alterar o contrato:

- `call.args().span()` no AST;
- `Args::span` logo após `eval_args`;
- span entregue por `try_dispatch_collection_method`;
- span dentro de `bytes_len`, `bytes_at` e `bytes_slice`;
- span contido no `SourceDiagnostic` devolvido;
- resolução desse span contra `Source` antes da renderização no shell.

Usar teste/instrumentação descartável ou assertions sob `#[cfg(test)]`; não
deixar logging produtivo. Publicar
`00_nucleo/diagnosticos/p1215-span-pipeline.tsv` com um registro por fronteira.

Classificar a primeira fronteira que divergir:

| Classe | Owner provável | Condição |
|---|---|---|
| `AST-SPAN-GAP` | parser/AST | `call.args().span()` já está detached/incorreto |
| `ARGS-PROPAGATION-GAP` | `eval/call_dispatch` | AST correto, `Args::span` diferente |
| `METHOD-ANCHOR-GAP` | dispatcher/collections | `Args` correto, diagnóstico ancora região errada |
| `SOURCE-RESOLUTION-GAP` | pipeline/source | diagnóstico correto, resolução falha |
| `RENDERER-GAP` | shell diagnóstico | linha/coluna existem antes do renderer e somem na saída |
| `EVAL-ONLY-GAP` | caminho CLI `eval` | compile está correto e somente eval perde o source |

É proibido escolher o owner pelo nome do sintoma. A medição `file:line` deve
preceder o veredito.

## 5. Oráculo de posição

Criar `00_nucleo/diagnosticos/p1215-bytes-span-oraculos.tsv` antes do patch com:

```text
id | modo | expressão/arquivo | exit | linha | coluna_inicial | coluna_final | mensagem | hints | trace
```

Extrair as posições do vanilla ratificado. Não congelar ANSI, caminhos
temporários ou desenho gráfico do terminal quando esses bytes forem apenas
mecânica; congelar a região da linguagem apontada e os complementos
diagnósticos semanticamente observáveis.

Separar explicitamente:

- erro da chamada inteira;
- erro do argumento posicional específico;
- erro do nome de argumento;
- método inexistente no field access.

Se o vanilla apontar regiões diferentes conforme a classe de erro, um único
`Args::span` não pode ser declarado suficiente sem prova.

## 6. Gate L0

Auditar os L0s listados na seção 2 e atualizar primeiro somente o owner causal.

Fluxo contínuo ADR-0127 é permitido se a solução apenas corrige propagação ou
ancoragem interna de spans. Parar para confirmação antes do código se a solução
exigir:

- novo campo público em `Args` ou outra entidade;
- mudança de assinatura pública/trait;
- alteração geral do comportamento diagnóstico por default fora do fragmento;
- mudança de fase do pipeline;
- quebra de compatibilidade.

Não atualizar `entities/bytes.md`: o valor `Bytes` não é owner de localização
de fonte. Não usar `Span::detached()` como fallback para fazer testes passarem.

## 7. Testes RED

Escrever testes no owner causal, antes da implementação, que comprovem:

- span não detached atravessa AST → args → diagnóstico;
- linha e coluna mudam quando a mesma chamada é deslocada;
- tipo inválido aponta a região vanilla medida;
- named desconhecido aponta a região vanilla medida;
- limite de índice preserva mensagem e região;
- método inexistente continua pelo caminho genérico correto;
- um método de coleção não-`bytes` permanece inalterado;
- pelo menos um diagnóstico nativo fora de collections permanece inalterado.

Confirmar RED e registrar comando, falha e predecessor causal. Teste que apenas
compara a mensagem não serve para este passo.

## 8. Implementação mínima permitida

Aplicar a menor correção compatível com a classe medida:

- se o span já chega correto ao diagnóstico, corrigir somente resolução/render;
- se a chamada usa âncora larga incorreta, transportar a âncora AST necessária
  pelo caminho já existente, sem tabela global;
- se apenas `typst eval` perde source, corrigir somente esse adaptador;
- se a lacuna for genérica e comprovada, extrair helper puro no owner vigente,
  acompanhado por regressões fora de `bytes`.

Não:

- especializar o shell por texto de mensagem ou nome `bytes`;
- recalcular linha/coluna contando caracteres do source no L1;
- inventar source, path ou span sintético;
- alterar `bytes.len/at/slice` ou implementar novos métodos;
- corrigir simultaneamente `operator-diagnostics` sem novo fragmento;
- promover o mapa com base apenas em mensagens centrais.

## 9. Ataques obrigatórios

As sondas devem rejeitar estes mutantes:

1. substituir todo span por `Span::detached()`;
2. ancorar todos os erros na lista completa de argumentos;
3. ancorar todos no primeiro argumento;
4. ancorar no callee/field access quando vanilla aponta argumento;
5. preservar linha e errar coluna por um byte;
6. contar bytes UTF-8 como colunas visuais;
7. corrigir `compile` e deixar `eval` divergente;
8. corrigir `eval` e deixar `compile` divergente;
9. apagar hints/trace para obter igualdade textual parcial;
10. alterar mensagens já verdes do P1214;
11. regressar spans de `array`/`str` no mesmo dispatcher;
12. hardcode por nome do método ou conteúdo da mensagem.

Todos os mutantes válidos precisam de testemunha. Sem motor externo, relatar
apenas cobertura dirigida, não mutation score independente.

## 10. Verificação A/B

Produzir `00_nucleo/diagnosticos/p1215-bytes-span-resultados.tsv` e repetir:

- os 21 diagnósticos P1214;
- os 22 casos de valor P1214, que devem continuar byte a byte verdes;
- a matriz deslocada de linha/coluna deste passo;
- as regressões de collections e de um diagnóstico fora do fragmento.

Critério por diagnóstico:

```text
exit + mensagem + região (linha/coluna/extensão) + hints/trace aplicáveis
```

Diferença cosmética do renderer deve ser classificada segundo ADR-0107; região
ou complemento que muda a informação entregue ao utilizador continua sendo
paridade da linguagem.

## 11. Mapa e fila

Somente após todos os valores e diagnósticos passarem:

```toml
id = "language-bytes"
alegacao = "declarada-fechada"
```

Adicionar as evidências P1214/P1215 e limitar a nota à superfície medida de
`Bytes`. Na fila P1213, mudar `bytes-methods` de `PARTIAL` para `RESOLVED`, sem
apagar o histórico do gap.

Se qualquer modo/posição divergir, manter `parcial`/`PARTIAL` e registrar a
fronteira causal restante. Não usar a lente estrutural como substituto da
sonda funcional.

## 12. Gates finais

Executar, no mínimo:

```text
cargo test -p typst-core <testes focais de span>
cargo test -p typst-shell <testes focais>, se o owner for L2
cargo test --workspace
cargo build --workspace --quiet
cargo build --release --bin typst
crystalline-lint --fix-hashes .
crystalline-lint .
cargo fmt --all -- --check
git diff --check
lente --comparar ... --mapa-correspondencia ...
```

Executar a matriz e a lente duas vezes; registrar hashes e determinismo.
Falha temporal de workspace deve ser preservada e repetida isoladamente, nunca
apagada do laudo.

## 13. Separação de autoridades

Aplicar o protocolo completo da materialização segregada:

- A congela oráculos e não lê o patch;
- B ataca spans, posições e renderização;
- C implementa contra L0/oráculos selados;
- D executa A/B sem decidir promoção;
- E verifica e promove ou recusa.

Se tudo ocorrer numa única sessão, declarar expressamente
`EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO`.

## 14. Próximo cluster

Depois do fechamento ou laudo parcial deste passo, retornar à fila P1213. O
próximo cluster funcional é `operator-diagnostics`, preservando operador e
representação dos operandos nas mensagens já identificadas em P1212.
