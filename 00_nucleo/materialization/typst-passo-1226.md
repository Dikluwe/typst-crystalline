# P1226 — fechar a fresta de paths SVG gerais e arcos equivalentes

**Estado:** EXECUTADO — PATHS PÚBLICOS PRESERVED; ARC HARNESS GREEN  
**Predecessor causal:** P1225  
**Cluster:** `svg-morphology`, `PARTIAL`  
**Fresta:** o comparador já canonicaliza `M/L/H/V/C/S/Q/T/A/Z`, mas conserva
arcos em forma endpoint e, por isso, ainda não demonstra equivalência quando
vanilla e cristalino descrevem a mesma curva por comandos diferentes.

## 1. Objetivo

Ampliar a lente morfológica SVG para comparar paths gerais pela geometria da
linguagem, não pelos tokens SVG. Fechar `M/L/H/V/C/S/Q/T`, múltiplos subpaths e
arcos elípticos `A` por normalização analítica endpoint→center. Só corrigir o
produto se o novo oráculo encontrar uma divergência pública reproduzível.

Resultado esperado:

```text
GENERAL SVG PATHS ADJUDICATED — ARCS PRESERVED OR PRODUCT GAP ISOLATED
```

## 2. Baseline e proveniência

Antes da decisão:

- registrar HEAD, horário e `git diff HEAD --stat`;
- congelar SHA-256 do vanilla ratificado `a51e02804`, do cristalino e da lente;
- congelar hashes do passo, do comparador, dos testes, dos L0 afetados e do
  mapa DSM;
- executar duas vezes o corpus vigente para detectar instabilidade;
- manter recursos não modelados como `Unknown`, nunca `Preserved` por ausência
  de compreensão;
- não ler `00_nucleo/context/` nem `00_nucleo/materialization/`.

## 3. Medição pública antes da implementação

Construir fixtures Typst mínimas que produzam, quando a superfície pública
permitir:

1. `M/L/H/V` absolutos, relativos e repetição implícita;
2. `C/S`, incluindo reflexão e ausência de controle cúbico anterior;
3. `Q/T`, incluindo reflexão e ausência de controle quadrático anterior;
4. múltiplos subpaths, abertos e fechados;
5. início cíclico diferente no mesmo path fechado;
6. `fill-rule` nonzero e evenodd;
7. arcos nas quatro combinações large-arc/sweep;
8. rotação do eixo, raios desiguais e correção automática de raios;
9. arco degenerado: raios zero, endpoints iguais e raio insuficiente;
10. transformações aninhadas, reflexão e escala não uniforme.

Para construções sem API Typst pública, usar pares SVG sintéticos apenas para
testar a lente e marcá-los como `harness-only`. Não convertê-los em alegação de
paridade do produto.

Registrar stdout, stderr, exit, artefato e região diagnóstica dos dois binários.
A classificação vem depois da medição:

- `Preserved`: geometria, fechamento, winding e paints equivalentes;
- `Violated`: diferença pública no nível da linguagem;
- `Unknown`: vocabulário ainda não modelado ou caso matematicamente ambíguo.

## 4. Contrato analítico de path

Canonicalizar todo comando para coordenadas absolutas e aplicar a matriz
acumulada uma única vez. Preservar ordem de subpaths abertos. Para subpaths
fechados, permitir rotação cíclica somente dentro do mesmo subpath, sem inverter
orientação nem reordenar subpaths.

Normalizar:

- `H/V -> L`;
- `S -> C` com reflexão do último controle cúbico válido;
- `T -> Q` com reflexão do último controle quadrático válido;
- repetição implícita, incluindo `M` subsequente como `L`;
- `Z` como fechamento sem apagar a distinção aberto/fechado;
- arco endpoint SVG para uma forma canônica com centro, raios corrigidos,
  rotação normalizada, ângulo inicial e delta orientado.

Na normalização de arco seguir o algoritmo normativo SVG:

1. transformar endpoints para o referencial do eixo da elipse;
2. usar valores absolutos dos raios;
3. ampliar ambos os raios pelo mesmo fator quando forem insuficientes;
4. calcular o centro escolhendo o sinal pelas flags large-arc/sweep;
5. calcular ângulo inicial e delta orientado;
6. normalizar ângulos módulo volta completa sem apagar sweep;
7. tratar raio zero como segmento de linha;
8. tratar endpoints iguais como segmento de arco vazio, sem `NaN`;
9. comparar parâmetros com tolerância `1e-8pt` e ângulos com tolerância
   derivada explicitamente, nunca por arredondamento decimal informal.

Não converter arco em Bézier aproximada para alegar igualdade. Uma emissão em
Bézier só pode fechar paridade contra arco se existir um oráculo analítico que
prove a mesma curva dentro de um limite geométrico especificado; caso contrário
fica `Unknown`.

## 5. RED do comparador

Antes de alterar `svg_morphology.py`, adicionar testes que falhem para:

- dois arcos equivalentes com raios originais diferentes após correção;
- forma absoluta versus relativa;
- flags que escolhem o mesmo arco após normalização angular;
- arco versus linha quando um raio é zero;
- endpoints iguais sem erro numérico;
- transform aninhado aplicado exatamente uma vez;
- mesma curva com início cíclico distinto;
- controles refletidos de `S/T`.

Adicionar também controles que obrigatoriamente permaneçam diferentes:

- sweep invertido;
- large-arc invertido quando seleciona outra trajetória;
- rotação do eixo alterada;
- raio, endpoint ou controle alterado acima da tolerância;
- path aberto versus fechado;
- winding ou ordem de subpaths alterados.

## 6. Implementação e fronteira de produto

Primeiro implementar apenas a normalização na lente e seus testes. Reexecutar o
corpus público congelado.

Se todos os casos públicos forem `Preserved`, não alterar L1–L4. Atualizar
somente diagnóstico, mapa e corpus.

Se aparecer `Violated`:

1. localizar a primeira fase onde a geometria diverge (`eval`, `layout` ou
   export SVG);
2. ler o Prompt L0 proprietário dessa fase e medir `file:line` antes de decidir;
3. classificar linguagem versus mecânica conforme ADR-0107;
4. atualizar L0 antes do código;
5. parar para confirmação somente se surgir contrato público, comportamento
   por defeito, mudança de fase ou incompatibilidade, conforme ADR-0127;
6. caso contrário seguir RED→GREEN em fluxo contínuo.

É proibido alterar o produto para facilitar a lente ou forçar ambos os lados a
emitirem os mesmos tokens SVG.

## 7. Ataques obrigatórios

O conjunto mínimo de mutantes válidos deve incluir:

1. ignorar relative;
2. não promover argumentos extras de `M` para `L`;
3. refletir `S` a partir de controle inexistente;
4. refletir `T` a partir de controle cúbico;
5. apagar `Z`;
6. permitir rotação cíclica em path aberto;
7. reordenar subpaths fechados;
8. inverter winding;
9. ignorar uma das flags de arco;
10. trocar o sinal do centro;
11. omitir correção de raios;
12. corrigir somente um raio;
13. ignorar rotação do eixo;
14. converter raio zero em arco;
15. produzir `NaN` para endpoints iguais;
16. normalizar sweep para módulo sem sinal;
17. aplicar transform duas vezes;
18. aceitar diferença acima de `1e-8pt`;
19. converter `Unknown` em `Preserved`;
20. aceitar path malformado.

Gate: `mutation_score = 1.0` entre mutantes válidos; inviáveis ficam fora do
denominador com justificativa individual.

## 8. Artefatos

Produzir:

```text
00_nucleo/diagnosticos/p1226-tekt-manifesto.tsv
00_nucleo/diagnosticos/p1226-svg-path-contract.tsv
00_nucleo/diagnosticos/p1226-svg-path-oraculos.tsv
00_nucleo/diagnosticos/p1226-svg-path-fixtures.tsv
00_nucleo/diagnosticos/p1226-svg-path-ataques.tsv
00_nucleo/diagnosticos/p1226-svg-path-resultados.tsv
00_nucleo/diagnosticos/p1226-svg-engineering-choices.tsv
00_nucleo/diagnosticos/p1226-tekt-certificado.tsv
00_nucleo/diagnosticos/typst-p1226-svg-general-paths.md
```

## 9. DSM e fila

- promover `svg-general-paths` somente para o subconjunto público medido;
- registrar separadamente casos `harness-only`;
- manter paint servers, clips, imagens, links e tags não modeladas como
  `Unknown`;
- não misturar extensões cristalinas com paridade vanilla;
- atualizar `p1213-fila-implementacao.tsv` com a primeira fresta material que
  restar;
- rodar a lente DSM com o mapa e anexar o resumo reproduzível ao laudo.

## 10. Gates finais

```text
python3 -m unittest discover -s lab/parity/matrix -p 'test*.py'
python3 lab/parity/matrix/runner.py --case P1138-X-001
cargo test -p typst-core p1226
cargo test -p typst-infra p1226
cargo test -p typst-wiring p1226
cargo test --workspace
cargo build --workspace
cargo fmt --all -- --check
git diff --check
crystalline-lint --checks v5,v15,v26 --fail-on warning .
crystalline-lint .
../tekt-cargo-dsm/target/release/lente --comparar \
  --antes lab/typst-original --depois . \
  --mapa-correspondencia 00_nucleo/diagnosticos/dsm/typst-correspondencias-v1.toml
```

## 11. Critério de encerramento

O passo fecha quando o corpus público repetido possui veredito reproduzível,
todos os mutantes válidos são rejeitados e o mapa declara exatamente o
subconjunto demonstrado. Se um caso continuar sem oráculo matemático, ele fica
explicitamente `Unknown`; isso é uma fronteira honesta, não uma falha a ocultar.

EXECUTAR SEM ATESTAÇÃO DE ISOLAMENTO, salvo se houver ambientes independentes
reais para os papéis Tekt.
