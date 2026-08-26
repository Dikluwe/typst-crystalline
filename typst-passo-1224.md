# P1224 — materializar stroke complexo e retomar a selagem de paths SVG

**Estado:** ESCRITO — AGUARDA EXECUÇÃO E CONFIRMAÇÃO DO GATE  
**Predecessor causal:** P1223  
**Cluster:** `svg-morphology`, `PARTIAL`  
**Gate de entrada:** aprovação humana do contrato P1223 em
`00_nucleo/prompts/entities/geometry.md`  
**Primeira perda medida:** `01_core/src/compiler/stdlib/layout.rs:2333-2339`  
**Owner causal:** `01_core/src/entities/geometry.rs::Stroke`

## 1. Objetivo

Após confirmação humana do novo contrato L0, preservar `cap`, `join`, `dash` e
`miter-limit` desde a linguagem Typst até o SVG, sem misturar os quatro campos
num único salto não diagnosticável. Em seguida, retomar o corpus de paths
`A/S/Q/T` interrompido pelo gate P1223 e ampliar apenas as alegações que forem
sustentadas por oráculos e ataques.

Resultado esperado:

```text
COMPLEX STROKE PRESERVED — GENERAL PATH FRONTIER READJUDICATED
```

Se o L0 não for explicitamente confirmado, parar sem editar código produtivo.

## 2. Baseline e proveniência

Antes de qualquer alteração:

- registrar confirmação humana literal do contrato P1223;
- registrar HEAD, horário, `git status --short` e `git diff HEAD --stat`;
- declarar a árvore não commitada acumulada de P1222/P1223;
- confirmar vanilla ratificado `a51e02804` e SHA-256 dos dois binários;
- congelar hashes do passo, L0 `geometry`, consumer `geometry.rs`, L0 SVG,
  consumer SVG, comparador, testes e mapa DSM;
- repetir uma vez a lente com o mapa e guardar resumo das correspondências;
- não ler `00_nucleo/context/` nem `00_nucleo/materialization/`.

## 3. Resselo do gate L0

Com a confirmação registrada:

1. auditar se o texto P1223 do L0 continua sendo a decisão vigente;
2. calcular e registrar o hash completo do L0;
3. executar o reparo de hashes somente após validar V15/V26;
4. confirmar que `geometry.rs` continua sendo o único consumer proprietário;
5. escrever testes RED antes de adicionar qualquer campo.

Se o reparo revelar conflito de ownership ou núcleo, corrigir somente o grafo
L0 e parar; não contornar V15/V26.

## 4. Materialização incremental da entidade

Implementar no owner, nesta ordem:

1. `LineCap { Butt, Round, Square }` e `Stroke::cap`;
2. `LineJoin { Miter, Round, Bevel }` e `Stroke::join`;
3. `Stroke::miter_limit`, default `4.0`, finito e positivo;
4. `DashLength { Length(f64), LineWidth }`;
5. `DashPattern { array, phase }` e `Stroke::dash`.

Para cada incremento:

- escrever RED próprio em `geometry.rs` ou no owner de parsing;
- atualizar todos os construtores literais mecanicamente;
- usar defaults vanilla, sem inferir defaults a partir do SVG;
- provar que `paint`, `thickness` e `overhang` anteriores permanecem iguais;
- executar o conjunto focal antes de iniciar o campo seguinte.

Não substituir os enums por strings no domínio. Não ordenar, deduplicar ou
normalizar destrutivamente o dash array.

## 5. Eval e representação pública

Atualizar `native_stroke` e os caminhos equivalentes de extração para aceitar:

```typst
stroke(paint: red, cap: "round")
stroke(paint: red, join: "bevel")
stroke(paint: red, dash: "dashed")
stroke(paint: red, dash: (array: (2pt, 1pt), phase: 0.5pt))
stroke(paint: red, miter-limit: 2)
```

Cobrir:

- todos os valores válidos de cap/join;
- nome e tipo inválidos com mensagem/região comparadas ao vanilla;
- miter limit zero, negativo, infinito e NaN quando publicamente construível;
- dash `none`, presets, array simples e alternado;
- item `"dot"` preservado como `LineWidth`;
- array vazio e comprimentos inválidos;
- phase positiva e negativa;
- acesso/repr dos campos no nível da linguagem.

A aceitação é semântica e morfológica. Igualdade byte a byte do Rust não é
gate, salvo mensagens de erro observáveis.

## 6. Transporte por layout e frame

Construir uma tabela de fluxo atualizada:

```text
language_input | evaluated_value | entity_field | FrameItem_field |
svg_attribute | vanilla | crystalline | state | first_loss_file_line
```

Confirmar, para cada campo, que o `Stroke` chega intacto ao
`FrameItem::Shape`. Não duplicar os campos em `FrameItem` se ele já transporta
o `Stroke` proprietário. Não resolver `LineWidth` antes de a espessura efetiva
estar disponível.

Produzir:

```text
00_nucleo/diagnosticos/p1224-svg-stroke-flow.tsv
```

## 7. Serialização SVG tardia

No owner L3 SVG, emitir conforme necessário:

- `stroke-linecap`;
- `stroke-linejoin`;
- `stroke-miterlimit`;
- `stroke-dasharray`;
- `stroke-dashoffset`.

Regras:

- preservar a ordem do dash array;
- resolver `LineWidth` usando a espessura efetiva;
- preservar phase negativa;
- não assar transforms na espessura ou no dash sem prova da semântica SVG;
- manter alpha no paint;
- manter fill e stroke em ordem observável;
- omitir atributos default somente quando a omissão for morfologicamente
  equivalente no SVG;
- não usar raster ou bbox como prova primária.

## 8. Corpus A/B de stroke complexo

Executar no vanilla e cristalino, no mínimo:

1. width default, zero e positiva;
2. cap `butt`, `round`, `square` em path aberto;
3. join `miter`, `round`, `bevel` em ângulo agudo;
4. miter limit abaixo e acima do limiar;
5. dash ausente, preset, array e `dot`;
6. phase positiva e negativa;
7. stroke com alpha;
8. path aberto, `Z` e endpoint coincidente sem `Z`;
9. scale uniforme e não uniforme;
10. reflexão por matriz de determinante negativo;
11. fill+stroke e stroke sem fill;
12. rounded rect P1222 como controle de não regressão.

Executar o corpus duas vezes sobre o mesmo estado da árvore.

## 9. Retomar paths gerais

Somente após stroke focal GREEN, selar no comparador:

- `M/L/H/V` absoluto e relativo;
- repetição implícita após `M/L/C/Q/A`;
- reflexão normativa de controle em `S` e `T`;
- múltiplos subpaths abertos e fechados;
- ponto inicial cíclico diferente em path fechado;
- winding oposto sob `nonzero` e `evenodd`;
- `A` com as quatro combinações `large-arc × sweep`;
- rotação de eixo, correção normativa de raios e raio zero;
- reflexão e transforms aninhados não comutativos;
- tolerância `1e-8pt` e input malformado.

Se a normalização de arco ainda carregar apenas parâmetros sem aplicar o
algoritmo SVG endpoint→center, parametrizações distintas ficam `Unknown`, não
`Preserved`. Equivalência arco↔Bézier aproximada também permanece `Unknown`.

## 10. Ataques segregados

Selar ataques antes de cada correção produtiva. Rejeitar pelo menos os 25
mutantes enumerados em P1223, acrescentando estes controles de entidade:

1. trocar `Round` por `Square`;
2. trocar `Bevel` por `Miter`;
3. forçar `miter_limit = 4.0`;
4. ordenar o dash array;
5. substituir `LineWidth` por zero;
6. zerar phase negativa;
7. descartar alpha;
8. remover fechamento `Z`;
9. promover `Unknown` de arco a `Preserved`.

Gate: `mutation_score = 1.0` sobre mutantes válidos. Registrar mutantes
inviáveis separadamente; não incluí-los no denominador.

## 11. Artefatos obrigatórios

Produzir antes e depois do código conforme a autoridade correspondente:

```text
00_nucleo/diagnosticos/p1224-tekt-manifesto.tsv
00_nucleo/diagnosticos/p1224-svg-stroke-contract.tsv
00_nucleo/diagnosticos/p1224-svg-stroke-oraculos.tsv
00_nucleo/diagnosticos/p1224-svg-stroke-ataques.tsv
00_nucleo/diagnosticos/p1224-svg-stroke-flow.tsv
00_nucleo/diagnosticos/p1224-svg-stroke-resultados.tsv
00_nucleo/diagnosticos/p1224-svg-path-contract.tsv
00_nucleo/diagnosticos/p1224-svg-path-oraculos.tsv
00_nucleo/diagnosticos/p1224-svg-path-ataques.tsv
00_nucleo/diagnosticos/p1224-svg-path-resultados.tsv
00_nucleo/diagnosticos/p1224-svg-engineering-choices.tsv
00_nucleo/diagnosticos/p1224-tekt-certificado.tsv
00_nucleo/diagnosticos/typst-p1224-complex-stroke-and-paths.md
```

## 12. Readjudicação DSM e fila

Após os resultados:

- adicionar evidências P1224 a `svg-solid-analytic-shapes` sem ampliar seu
  texto além do corpus provado;
- criar `svg-complex-stroke` somente se os owners formarem correspondência
  estrutural válida e a lente aplicar o mapa;
- criar `svg-general-paths` somente para o fragmento efetivamente selado;
- manter arcos não normalizados, paint servers, clip/mask, imagens e links
  como `Unknown`/`PARTIAL`;
- registrar extensões exclusivas do cristalino separadamente;
- atualizar `p1213-fila-implementacao.tsv` com o primeiro gap material seguinte.

A lente documenta correspondências estruturais; o fechamento de paridade vem
dos testes A/B e oráculos, nunca do pareamento mecânico isolado.

## 13. Gates finais

Executar e registrar:

```text
python3 -m unittest discover -s lab/parity/matrix -p 'test*.py'
python3 lab/parity/matrix/runner.py --case P1138-X-001
cargo test -p typst-core p1224
cargo test -p typst-infra p1224
cargo test -p typst-wiring p1224
cargo test --workspace
cargo build --workspace
cargo fmt --all -- --check
git diff --check
crystalline-lint .
```

Repetir isoladamente o teste watch três vezes se a suíte integral falhar nele.
Não declarar workspace GREEN com falha, ainda que classificada como preexistente.

## 14. Critério de conclusão

O passo fecha somente se:

- a confirmação do L0 estiver registrada;
- `cap`, `join`, `dash` e `miter-limit` forem aceitos e preservados ponta a ponta;
- cada campo tiver RED próprio, A/B público e ataque independente;
- o corpus stroke passar duas vezes;
- paths forem classificados sem promover parametrização não normalizada;
- mapa DSM e fila refletirem exatamente o alcance provado;
- V15, V26 e todos os gates aplicáveis estiverem verdes;
- o certificado declarar honestamente se houve ou não isolamento entre papéis.

Se stroke fechar mas paths/arcos permanecerem `Unknown`, concluir como:

```text
COMPLEX STROKE PRESERVED — GENERAL PATHS REMAIN PARTIAL
```
