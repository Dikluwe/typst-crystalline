# P1225 — fechar serialização pública de Stroke e concluir a fronteira SVG

**Estado:** EXECUTADO — STROKE PRESERVED; PATHS GERAIS PARTIAL  
**Predecessor causal:** P1224  
**Cluster:** `svg-morphology`, `PARTIAL`  
**Gate de entrada:** aprovação humana da emenda P1224 em
`00_nucleo/prompts/shell/cli.md`  
**Divergência pública:** vanilla serializa `Value::Stroke` como string JSON;
cristalino rejeita o tipo por regra de escopo da primeira entrega de `eval`

**Confirmação do dono:** em 2026-08-26, após explicação da origem histórica,
o dono respondeu “Concordo escreva o passo”. Fica aprovada exclusivamente a
branch nominal `Value::Stroke -> string JSON`; fallback genérico de tipos
desconhecidos para `repr` permanece proibido.

## 1. Objetivo

Adicionar uma exceção nominal e fechada para `Value::Stroke` ao serializer JSON
do `typst eval`, sem criar fallback genérico para `repr`. Reexecutar o fluxo
público de stroke complexo ponta a ponta e, quando GREEN, terminar os ataques e
a readjudicação de paths gerais interrompidos em P1224.

Resultado esperado:

```text
PUBLIC COMPLEX STROKE PRESERVED — GENERAL PATH FRONTIER READJUDICATED
```

O gate está confirmado. Na execução, registrar esta confirmação no manifesto,
ressellar o L0 da CLI e só então iniciar o RED em L2.

## 2. Origem e alcance da decisão

Registrar no laudo:

- a restrição nasceu na especificação da primeira entrega de `typst eval`,
  commit documental `8d97b9ae7`, em 2026-08-23;
- o texto chama a entrega de “deliberadamente incompleta face ao vanilla”;
- o objetivo era limitar JSON a `none`, bool, int, float finito, string, array e
  dict e impedir fallback silencioso de qualquer tipo desconhecido para `repr`;
- o commit de implementação foi `fcbc9763f`;
- não há evidência no histórico de que a rejeição de `Stroke` fosse uma decisão
  permanente de produto ou uma divergência desejada da linguagem;
- preservar a regra nominal/exaustiva continua importante: somente tipos
  medidos e aprovados podem ganhar representação JSON.

Não reescrever essa história como decisão direta do dono sem evidência.

## 3. Baseline e resselo

Antes de código:

- registrar a confirmação humana da emenda L0;
- registrar HEAD, horário, working tree e `git diff HEAD --stat`;
- congelar hashes do P1225, L0/consumer CLI, L0/consumer repr, L0/consumer
  geometry, L0/consumer stdlib layout, L0/consumer SVG e mapa DSM;
- confirmar vanilla ratificado `a51e02804` e SHA-256 dos binários;
- ressellar somente o L0 da CLI confirmado;
- confirmar V15/V26 antes de qualquer reparo de hash;
- não ler `00_nucleo/context/` ou `00_nucleo/materialization/`.

## 4. Contrato JSON fechado

O serializer L2 deve manter branches nominais:

```text
None       -> null
Bool       -> boolean JSON
Int/Float  -> number JSON
Str/Symbol -> string JSON
Array/Dict -> JSON estrutural recursivo
Stroke     -> string JSON contendo repr pública de Stroke
outros     -> erro explícito
```

Proibido:

- transformar o wildcard final em `repr` genérico;
- serializar dict/array como string;
- permitir `Stroke` em `--format raw`;
- acessar módulo L1 `pub(crate)` a partir de L2;
- duplicar informalmente regras de repr em vários call sites.

Se for necessário expor uma função L1 nova para L2, auditar seu Prompt L0 e
aplicar ADR-0127 antes de ampliar contrato público Rust. Preferir um contrato
público estreito e proprietário para representação morfológica.

## 5. RED público antes da implementação

Congelar no mínimo:

```text
stroke(paint: red)
stroke(paint: red, cap: "round")
stroke(paint: red, join: "bevel")
stroke(paint: red, dash: "dashed")
stroke(paint: red, dash: (array: (2pt, "dot"), phase: -0.5pt))
stroke(paint: red, miter-limit: 2)
```

Para cada caso registrar stdout, stderr, exit e região diagnóstica quando
aplicável nos dois binários. O RED deve provar `vanilla exit 0` versus
`cristalino cannot serialize stroke to JSON` antes do patch L2.

Controles negativos:

- `--format raw` sobre Stroke continua exit 1;
- valor de outro tipo não-JSON continua exit 1;
- dict contendo apenas valores JSON continua estrutural;
- `--pretty` muda somente whitespace JSON.

## 6. Implementação mínima

Implementar uma única branch nominal `Value::Stroke` no owner do serializer.
A string deve vir do contrato morfológico de repr selado, não de `Debug`.

Verificar:

- stroke simples permanece equivalente ao vanilla;
- campos complexos aparecem separadamente;
- defaults não criam campos espúrios;
- ordem do dash e phase negativa são preservadas;
- escaping JSON ocorre uma única vez;
- newline final segue o formatter JSON existente.

## 7. Fluxo público até SVG

Compilar fixtures Typst públicas que usem stroke complexo em `line`, `curve`,
`rect`/rounded rect e path fechado quando disponível. Comparar morfologia SVG:

- `stroke-linecap`;
- `stroke-linejoin`;
- `stroke-miterlimit`;
- `stroke-dasharray`;
- `stroke-dashoffset`;
- alpha, fill+stroke e ordem de paints;
- transforms uniformes, não uniformes e reflexão.

O teste unitário que constrói `FrameItem` diretamente é controle interno, não
substitui o A/B público de linguagem.

## 8. Ataques de stroke

Completar os mutantes P1223/P1224 ainda não executados:

1. ignorar cap;
2. ignorar join;
3. forçar miter limit 4;
4. ordenar dash array;
5. zerar phase;
6. resolver `LineWidth` como zero;
7. descartar alpha;
8. tratar path aberto como fechado;
9. alterar stroke sob scale não uniforme;
10. inverter ordem fill/stroke;
11. promover tipo não-JSON desconhecido a repr;
12. permitir Stroke em raw.

Gate: `mutation_score = 1.0` para mutantes válidos, com inviáveis fora do
denominador e justificados individualmente.

## 9. Paths gerais

Depois do fluxo público stroke GREEN, retomar:

- `M/L/H/V` absoluto, relativo e repetição implícita;
- `C/S` e reflexão do controle anterior;
- `Q/T` e ausência de controle anterior;
- múltiplos subpaths abertos/fechados e ponto inicial cíclico;
- winding `nonzero`/`evenodd`;
- `A` nas quatro combinações de flags, rotação e correção de raios;
- arco degenerado, raio zero, reflexão e transforms aninhados;
- tolerância `1e-8pt` e path malformado.

Arco sem normalização endpoint→center permanece `Unknown`. Não declarar
equivalência arco↔Bézier por aproximação visual.

## 10. Artefatos

Produzir:

```text
00_nucleo/diagnosticos/p1225-tekt-manifesto.tsv
00_nucleo/diagnosticos/p1225-stroke-json-contract.tsv
00_nucleo/diagnosticos/p1225-stroke-json-oraculos.tsv
00_nucleo/diagnosticos/p1225-stroke-json-ataques.tsv
00_nucleo/diagnosticos/p1225-stroke-json-resultados.tsv
00_nucleo/diagnosticos/p1225-svg-stroke-public-resultados.tsv
00_nucleo/diagnosticos/p1225-svg-path-contract.tsv
00_nucleo/diagnosticos/p1225-svg-path-ataques.tsv
00_nucleo/diagnosticos/p1225-svg-path-resultados.tsv
00_nucleo/diagnosticos/p1225-svg-engineering-choices.tsv
00_nucleo/diagnosticos/p1225-tekt-certificado.tsv
00_nucleo/diagnosticos/typst-p1225-public-stroke-and-paths.md
```

## 11. DSM e fila

- adicionar `svg-complex-stroke` somente após A/B público GREEN;
- limitar a alegação aos campos e elementos medidos;
- criar/ampliar `svg-general-paths` somente pelo corpus selado;
- manter arcos não normalizados e recursos opacos como `Unknown`;
- preservar extensões cristalinas separadas da paridade vanilla;
- atualizar a fila com o primeiro gap material restante.

## 12. Gates finais

```text
python3 -m unittest discover -s lab/parity/matrix -p 'test*.py'
python3 lab/parity/matrix/runner.py --case P1138-X-001
cargo test -p typst-core p1225
cargo test -p typst-shell p1225
cargo test -p typst-infra p1225
cargo test -p typst-wiring p1225
cargo test --workspace
cargo build --workspace
cargo fmt --all -- --check
git diff --check
crystalline-lint .
```

Se o watch falhar dentro do workspace, repetir três vezes isoladamente e
registrar como controle independente; não chamar a suíte integral de GREEN.

## 13. Critério de conclusão

Fechar somente se:

- L0 da CLI estiver confirmado e ressellado;
- JSON de Stroke coincidir morfologicamente com o vanilla nos casos focais;
- raw e tipos desconhecidos continuarem fechados;
- linguagem→entidade→frame→SVG estiver comprovado publicamente;
- todos os mutantes válidos forem rejeitados;
- paths forem classificados sem promoção indevida de `Unknown`;
- DSM, fila, linter e gates refletirem exatamente o alcance comprovado.
