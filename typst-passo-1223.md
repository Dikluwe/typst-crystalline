# P1223 — selar paths SVG gerais e localizar o primeiro gap de stroke complexo

**Estado:** ESCRITO — AGUARDA EXECUÇÃO  
**Predecessor causal:** P1222  
**Cluster:** `svg-morphology`, `PARTIAL`  
**Fragmento fechado:** rect, line, ellipse, circle, rounded rect, overlap e
transforms focais  
**Fronteira:** paths `A/S/Q/T`, múltiplos subpaths e stroke complexo  
**Gate transversal instável:** `p1137_watch_dependencias_recuperacao_e_filtro`

## 1. Objetivo

Ampliar o corpus selado de P1222 para paths SVG gerais e strokes com cap,
join, miter e dash, distinguindo três situações:

1. diferença apenas mecânica entre comandos equivalentes;
2. gap real de linguagem/morfologia no cristalino;
3. informação que a entidade cristalina ainda não representa.

Corrigir somente o primeiro gap produtivo causal. Se stroke complexo exigir
mudança de contrato público ou de entidade, preparar o L0 e parar no gate
ADR-0127. Não comprimir vários campos de stroke numa implementação única sem
medição individual.

Resultado esperado:

```text
SVG GENERAL PATHS SEALED — FIRST COMPLEX-STROKE GAP GREEN OR GATED
```

## 2. Baseline e proveniência

Antes de alterar qualquer artefato:

- registrar HEAD, horário, `git status --short` e `git diff HEAD --stat`;
- registrar working tree não commitado e lista completa de arquivos alterados;
- confirmar vanilla ratificado `a51e02804` e hashes dos dois binários;
- congelar hashes de P1222: manifesto, contrato, ataques, resultados,
  comparador, testes, L0 SVG, consumer e mapa DSM;
- executar a lente com o mapa e confirmar 9/9 correspondências `aplicada`;
- registrar separadamente o estado do teste watch instável;
- não ler `00_nucleo/context/` ou `00_nucleo/materialization/`.

## 3. Gate do teste watch

Antes do trabalho SVG, executar três vezes:

```text
cargo test -p typst-wiring --test cli \
  p1137_watch_dependencias_recuperacao_e_filtro -- --nocapture
```

Registrar duração, carga concorrente conhecida e resultado. Não corrigir o
watch neste passo, salvo se uma alteração P1223 o tocar causalmente.

Classificação:

- 3/3 GREEN: controle estável nesta execução;
- resultado misto: `FLAKY-PREEXISTING`;
- 3/3 RED com mesma causa: abrir cluster nominal e não chamar workspace GREEN.

## 4. Corpus de paths gerais

Medir no mínimo:

1. `M/L/H/V`, absoluto e relativo;
2. repetição implícita após `M/L/C/Q/A`;
3. `C` e shorthand `S`, incluindo reflexão do controle anterior;
4. `Q` e shorthand `T`, incluindo ausência de controle anterior;
5. `A` com quatro combinações `large-arc × sweep`;
6. arcos com rotação do eixo;
7. raios de arco que exigem normalização SVG;
8. arco degenerado e raio zero;
9. múltiplos subpaths abertos;
10. múltiplos subpaths fechados;
11. mesma geometria com ponto inicial cíclico diferente;
12. winding oposto sob `nonzero` e `evenodd`;
13. path refletido por matriz de determinante negativo;
14. path sob transforms aninhados não comutativos;
15. coordenadas próximas da tolerância `1e-8pt`;
16. path malformado como controle `Unknown`.

O corpus deve vir da linguagem Typst sempre que publicamente construível. SVG
sintético é permitido apenas para atacar o canonicalizador.

## 5. Semântica canônica de arcos

O contrato deve decidir explicitamente como comparar `A`:

- endpoint, raios, rotação e flags não bastam sem normalização normativa;
- aplicar a correção de raios exigida pelo algoritmo SVG;
- preservar sweep após reflexão;
- distinguir arco completo, degenerado e segmento de linha;
- permitir equivalência entre arco e Bézier somente se o emissor usar uma
  conversão exata conhecida; aproximação visual permanece `Unknown`;
- não usar bbox ou raster como prova primária.

Se o comparador apenas carregar `A` sem normalizá-lo, ele pode classificar
diferença, mas não pode declarar equivalência entre parametrizações distintas.

## 6. Corpus de stroke complexo

Sondar no vanilla e no cristalino:

- width default, zero e positiva;
- cap `butt`, `round`, `square`;
- join `miter`, `round`, `bevel`;
- miter limit abaixo/acima do limiar geométrico;
- dash vazio, simples e alternado;
- dash phase positiva e negativa;
- stroke com alpha;
- path aberto versus fechado;
- endpoint coincidente sem `Z` versus path fechado;
- stroke sob scale não uniforme e reflexão;
- fill+stroke com ordem observável.

Para cada campo responder:

```text
language_input | evaluated_value | entity_field | FrameItem_field |
svg_attribute | vanilla | crystalline | state | first_loss_file_line
```

Produzir:

```text
00_nucleo/diagnosticos/p1223-svg-stroke-flow.tsv
```

## 7. Contrato e artefatos protegidos

Antes de código produtivo:

```text
00_nucleo/diagnosticos/p1223-tekt-manifesto.tsv
00_nucleo/diagnosticos/p1223-svg-path-contract.tsv
00_nucleo/diagnosticos/p1223-svg-path-oraculos.tsv
00_nucleo/diagnosticos/p1223-svg-path-ataques.tsv
00_nucleo/diagnosticos/p1223-svg-path-diff.tsv
```

Política:

- `Preserved`: geometria normalizada, ordem e paint equivalentes;
- `Violated`: qualquer observável modelado diverge;
- `Unknown`: parametrização não normalizada, paint server, clip/mask ou
  informação sem contrato suficiente;
- `Unsupported`: somente para input que a linguagem rejeita em ambos os lados;
  nunca usar como sinônimo de `Unknown`.

## 8. Ataques obrigatórios

Rejeitar no mínimo 25 mutantes:

1. trocar relativo por absoluto sem atualizar cursor;
2. perder repetição implícita;
3. refletir `S` no controle errado;
4. refletir `T` no controle errado;
5. reutilizar controle depois de comando incompatível;
6. trocar flags de arco;
7. ignorar rotação do arco;
8. não corrigir raios insuficientes;
9. tratar raio zero como curva;
10. inverter sweep sob reflexão incorretamente;
11. reordenar subpaths;
12. ignorar winding;
13. fechar path aberto;
14. abrir path fechado;
15. ignorar cap;
16. ignorar join;
17. ignorar miter limit;
18. ordenar dash array;
19. zerar dash phase;
20. ignorar alpha;
21. ignorar scale do stroke;
22. ordenar paints;
23. arredondar acima da tolerância;
24. aceitar path malformado;
25. promover `Unknown` a `Preserved`.

Gate: `mutation_score = 1.0` para mutantes válidos.

## 9. Medição e escolha do primeiro gap

Com o contrato selado:

1. executar corpus duas vezes;
2. localizar o primeiro `Violated` por fluxo de dados;
3. confirmar onde a informação é perdida: eval, entidade, layout ou L3;
4. ler o L0 exato desse owner;
5. atualizar L0 primeiro;
6. escrever RED no owner e um A/B público;
7. implementar somente esse gap;
8. repetir controles P1220-P1222.

Se o primeiro gap for campo ausente em `Stroke`, isso altera entidade/contrato
público interno compartilhado: aplicar gate ADR-0127 e parar após o novo L0.

## 10. Escolhas de engenharia permitidas

É permitido preparar seams internos que facilitem stroke complexo e issues
upstream, desde que:

- tenham consumidor imediato no gap atual;
- a saída padrão permaneça `Preserved`;
- não criem flag ou API especulativa;
- não adicionem I/O a L1;
- serialização e sanitização permaneçam em L3;
- não introduzam despacho dinâmico ou registry global;
- sejam registradas em:

```text
00_nucleo/diagnosticos/p1223-svg-engineering-choices.tsv
```

Preferir preservação de informação no domínio e decisão tardia de
serialização, sem mudar o observável padrão.

## 11. Readjudicação DSM

Após os resultados:

- manter as 9 correspondências aplicáveis sem diagnóstico;
- ampliar `svg-solid-analytic-shapes` somente pelo corpus comprovado;
- criar `svg-general-paths` se a cardinalidade e owners forem distintos;
- criar `svg-complex-stroke` como `parcial`, `unknown` ou
  `declarada-fechada` conforme evidência;
- não misturar extensão cristalina com paridade vanilla;
- atualizar a fila com o primeiro `Unknown` material restante.

## 12. Resultados e laudo

Produzir:

```text
00_nucleo/diagnosticos/p1223-svg-path-resultados.tsv
00_nucleo/diagnosticos/p1223-svg-stroke-resultados.tsv
00_nucleo/diagnosticos/p1223-watch-estabilidade.tsv
00_nucleo/diagnosticos/p1223-tekt-certificado.tsv
00_nucleo/diagnosticos/typst-p1223-svg-paths-and-stroke.md
```

O laudo separa:

- paths comprovadamente Preserved;
- primeiro gap de stroke e owner causal;
- diferenças mecânicas normalizadas;
- campos ainda não representados;
- `Unknown` restantes;
- estado independente do teste watch;
- alcance exato das alegações DSM.

## 13. Gates finais

```text
python3 -m unittest discover -s lab/parity/matrix -p 'test*.py'
python3 lab/parity/matrix/runner.py --case P1138-X-001
cargo test -p typst-core p1223
cargo test -p typst-infra p1223
cargo test -p typst-wiring p1223
cargo build --workspace --quiet
cargo test --workspace
cargo build --release --bin typst
crystalline-lint --fix-hashes .
crystalline-lint .
cargo fmt --all -- --check
git diff --check
lente --comparar --antes lab/typst-original --depois . \
  --mapa-correspondencia \
  00_nucleo/diagnosticos/dsm/typst-correspondencias-v1.toml
```

Repetir corpus e lente duas vezes. Não declarar `cargo test --workspace` verde
se o watch falhar, mesmo que todos os gates SVG passem.

## 14. Separação de autoridades

Usar protocolo Tekt completo:

- A congela baseline e watch;
- B escreve contrato de paths/arcos;
- C cria oráculos e mutantes;
- D sela o comparador;
- E mede sem editar produção;
- F identifica o primeiro ponto de perda de stroke;
- G atualiza L0 e implementa somente o gap autorizado;
- H executa A/B final;
- I readjudica DSM e fila.

Em sessão única declarar:

```text
EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO
```

## 15. Condição de encerramento

O passo encerra quando:

- corpus geral de paths tiver vereditos reproduzíveis;
- o primeiro gap de stroke estiver GREEN ou formalmente parado no gate L0;
- nenhuma mutação válida sobreviver;
- a lente repetir o mesmo mapa duas vezes sem diagnóstico;
- a instabilidade watch estiver registrada sem contaminar o veredito SVG;
- o próximo `Unknown` estiver nomeado na fila.
