# P1327 — auditoria independente do corpus e integração R0

Regime A/B sem atestação de isolamento. Entradas congeladas:
manifesto `cca33a6a95d40efa74d6dc8f7f910a711968a4c7bb771393ffc975bc9e515ea8`,
baseline `f8cee37f7f3556db93f935deb977790a0a13ddd232639334e3d3931cf8b504f8`,
freeze A/B `f45fa705a4f0b1edb93c6f48d3c58adea159a983a296c161a3606000acf6c076`.
Integração `p1327-test-integration.json`, SHA-256
`d55efc1a4f9074e203fc9f64822796a194ca4bbddc4669818e15072c34d3afe7`.
Proveniência: working tree do HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`
e diff/stat integral nos receipts baseline e integração. Corpus medido de
2026-09-09T10:43:56.161756 a 10:44:06.351414 UTC, freeze às 10:47:47.175605 UTC,
integração às 10:48:59.392317 UTC. Nenhum C existia ao verificar os corpos.

## Inspeção e recálculo

Li integralmente snippet, replacements e runner. Verifiquei todos os hashes
do freeze em Node, sem diferenças. A explicação aditiva
`p1327-ab-hash-clarification.md` distingue corretamente SHA bruto e SHA
normativo do L0, excluindo apenas a linha recíproca `Hash do Código`.

Reconstruí em memória `eval/tests.rs` partindo do original integral no
baseline, atualizando apenas header, aplicando os quatro replacement blocks
com preimages únicos e inserindo snippet no anchor único. O resultado é
byte-idêntico ao arquivo integrado, SHA-256
`0289e083d8ffeb6b32730d8c4a487027ac2f57d82430601814f5ee513c2914dc`.
O corpo de modules.rs ainda coincide com o original, excluído header.

Os seis casos legados que mudam estão exatamente nos quatro testes
autorizados. Os dois positivos usam helper novo estrito; os negativos e
P1306 discriminam somente as fixtures autorizadas e preservam o branch
`side.is_empty()` para as demais. Valores, erros, hints e ranges originais
continuam iguais. O helper antigo `observe` não foi relaxado.

Os seis novos testes verificam severidade, mensagem integral, count,
hints/trace vazios, range resolvível no Source correto, sucesso/erro e valor.
Cobrem Module ordinário, alias/sombra, builtin calc, nome math de arquivo,
closure, repetição no mesmo span e em spans distintos, UTF-8 deslocado,
origem importada, erro posterior e controles sem warning. A deduplicação
do mesmo span é testada como observável medido, sem alterar política do sink.

Recalculei as classificações das 112 células, cada uma medida nos dois
binários congelados (224 execuções):

- 52 correções: mesmo exit/stdout e stderr vanilla formado pelo stderr
  baseline preservado acrescido dos warnings sem efeito; expected é o
  transcript vanilla integral.
- 52 controles de paridade: exit/stdout/stderr integrais iguais nos dois
  binários; expected vanilla.
- 8 células de dívida, somente redundant-rename/type-error: expected
  baseline integral, sem alegação de paridade.

Chaves `(perfil,id)` únicas e completas; todos os exits são 0/1 previstos.
Nenhum erro de instrumentação ou observação Unknown nas células. Runner
compara transcript integral sem normalização ou remoção genérica de warning.
Os perfis HTML aqui são features no comando eval, não prova de target HTML.

O corpus e a integração R0 estão aprovados quanto a escopo e proveniência.
Formatação apontada posteriormente é problema de transporte test-only;
uma revisão aditiva estritamente mecânica não muda expectativas nem classes.
RED real ainda pendente nesta versão do relatório: compilação e execução,
com causa exclusivamente ausência de warning, são necessárias antes de C.
