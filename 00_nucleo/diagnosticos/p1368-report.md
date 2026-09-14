# P1368 — relatório final do observador efetivo S02

## Veredito

**PASS_WITH_LIMITED_ATTESTATION.** O observador binding-free normaliza
exatamente um campo público `Tagged: yes|no` e o classificador aceita somente
o vetor S02 `(true, false, true)`. Entradas ausentes, duplicadas, inválidas ou
não UTF-8 ficam `Unknown`; nenhum caminho incompleto vira sucesso.

O regime foi executado sem atestação de isolamento: contrato, teste e
implementação ficaram visíveis no mesmo workspace e não houve autoridade de
verificação materialmente isolada. A alegação cobre o instrumento, não a
paridade do vanila com o cristalino.

## RED, GREEN e poder discriminatório

O teste exato inicialmente retornou exit 1 por `FileNotFoundError` do módulo
candidato ausente. Depois da implementação, retornou exit 0, executou um teste
e aprovou 1/1 em 0,000 s.

O mesmo teste rejeita os sete vetores booleanos completos distintos de
`(true,false,true)`, portanto o mutation score desse recorte é 7/7 = 1,0.
Quatro payloads opacos cobrem campo ausente, duplicado, valor desconhecido e
UTF-8 inválido, todos fail-closed.

## Materialização

Em `p1368_s02_observer.py`, `parse_tagged_field` usa decode UTF-8 estrito,
inventaria todas as linhas `Tagged:` sem depender de posição e só completa a
observação quando existe exatamente um valor reconhecido. `classify_s02`
recusa observações incompletas e valores não booleanos antes de comparar o
vetor canônico.

O P1357 histórico não foi alterado nem teve seu selo invalidado. P1368 fornece
o observador focal que um runner bilateral posterior pode alimentar com três
PDFs por produto.

## Gates e memória

- `python3 -m py_compile` nos dois arquivos P1368: exit 0;
- `git diff --check`: exit 0;
- `crystalline-lint .`: exit 0, com warnings/infos preexistentes;
- Cargo, binários Typst, `pdfinfo`, `watch`, rede e suíte ampla: zero
  execuções.

Os comandos foram sequenciais. O teste teve timeout de 30 s e saída limitada;
o RED reportou 375 tokens e o GREEN 25. O lint teve a visualização truncada
pelo limite da ferramenta. Não houve timeout, OOM, SIGKILL nem falha de
alocação.

## Próxima utilização

O observador está pronto, mas ainda não recebeu medições bilaterais. Um passo
posterior deve compilar sequencialmente os três casos em cada produto e perfil,
executar `pdfinfo` com timeout por arquivo e alimentar o classificador. Até
isso ocorrer, `S02_TOP_LEVEL_EFFECT` deixa de ser uma limitação de desenho do
observador, mas continua sem veredito novo de produto.

