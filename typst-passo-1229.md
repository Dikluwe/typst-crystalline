# P1229 — medir e selar gradients Linear/Radial não-sRGB no SVG

**Estado:** EXECUTADO — `ACCEPTED_WITH_CONTRACTUAL_UNKNOWN`  
**Predecessor causal:** P1228  
**Cluster:** `svg-morphology`, `PARTIAL`  
**Gap vigente:** Linear e Radial são nativos somente em `color.rgb`; os demais
espaços caem em primeira cor com marcador `gradient-color-space`. Default/Oklab
permanece `Unknown` por falta de contrato e orçamento de erro.

## 1. Objetivo

Descobrir o método observável usado pelo vanilla ratificado `a51e02804` para
representar gradients Linear e Radial em espaços não-sRGB no SVG, selar um
critério de aproximação reproduzível e implementar somente os espaços cuja
equivalência puder ser demonstrada.

Resultado pretendido, condicionado à medição:

```text
P1229 NON-SRGB LINEAR/RADIAL SVG INTERPOLATION SEALED — REMAINING UNKNOWNS NAMED
```

Não promover um espaço a `Preserved` apenas porque o SVG parece semelhante.

## 2. Baseline e proveniência

Antes de qualquer decisão, registrar:

- `git rev-parse HEAD`, `git status --short`, `git diff HEAD --stat` e horário;
- working tree não commitada com lista integral dos paths alterados;
- SHA-256 do cristalino e dos dois binários vanilla ratificados;
- SHA-256 deste passo, mapa DSM, comparador e fixtures;
- SHA-256 dos L0 e código dos owners que forem realmente usados.

Não ler `00_nucleo/context/` nem `00_nucleo/materialization/`.

## 3. Regra anti-deriva

O helper existente `adaptive_n_for_stops` pertence ao pipeline PDF e usa
limiares históricos `16/32/64`. Sua existência não prova que esses valores,
essa métrica ou sequer sampling uniforme sejam adequados ao SVG. Medir o
vanilla antes de reutilizá-lo.

Separar explicitamente:

- **semântica:** espaço escolhido, ordem/offset/alpha e interpolação;
- **morfologia:** Linear/Radial, geometria, servidor aplicado a fill/stroke;
- **mecânica:** quantidade literal de stops, IDs, whitespace e ordem de defs.

A contagem de stops só vira obrigação se for necessária para satisfazer um
limite de erro ou se for observável indispensável à descontinuidade declarada.

## 4. Matriz de medição do vanilla

Executar em ordem direta e inversa, duas vezes, Linear e Radial para:

| ID | Espaço | Casos mínimos |
|---|---|---|
| S1 | default/Oklab | baixo, médio e alto contraste |
| S2 | Oklch | hue curto, passagem por zero e chroma distinto |
| S3 | linear-rgb | preto→branco e cores saturadas |
| S4 | luma | alpha e luminâncias distintas |
| S5 | HSL | hue atravessando 0/360 e cor acromática |
| S6 | HSV | idem, com value distinto |
| S7 | CMYK | primárias CMYK e alpha |
| S8 | sRGB | controle nativo já `Preserved` |

Cada espaço deve incluir:

- dois stops simples;
- três stops com offsets `0%, 37%, 100%`;
- stops coincidentes `0%, 0%, 100%`;
- alpha em pelo menos um stop;
- servidor em fill e em stroke;
- Linear com ângulos `0deg`, `25deg`, `90deg`;
- Radial com center/focal-center/focal-radius não defaults.

Registrar SVG parseado como grafo de referências e lista ordenada de stops,
além do raster em resoluções pelo menos `1x`, `2x` e `4x`. IDs literais,
whitespace, posição de `<defs>` e bytes não são observáveis.

## 5. Oráculo numérico independente

Construir um oráculo fora do exportador candidato que:

1. obtém as cores esperadas por `gradient.sample(t)`/vanilla público em uma
   malha fixa que inclua todos os offsets e pontos imediatamente antes/depois
   de descontinuidades;
2. parseia o SVG produzido e calcula as cores efetivas do servidor nos mesmos
   `t`;
3. compara sRGB codificado premultiplicado e alpha separadamente, espelhando
   `to_rgb().premultiply()` do vanilla; `LinearRgb` continua sendo apenas um
   dos espaços de interpolação da matriz, não a métrica de erro;
4. reporta erro máximo, percentil 95 e posição do pior erro;
5. refina a malha até que a decisão não mude em duas iterações sucessivas.

O contrato deve definir os limites numéricos **a partir da medição do vanilla**,
com margem justificada. É proibido escolher uma tolerância depois de observar
somente o candidato. Raster serve como confirmação; não substitui o oráculo de
stops/interpolação.

Se o SVG vanilla usa sampling ou transformação que não possa ser modelada com
segurança, manter o espaço como `Unknown` e registrar a evidência que falta.

## 6. Localização e opções de engenharia

Medir, com `file:line`, estes caminhos antes de decidir:

```text
Gradient::{Linear,Radial}.sample(t)
  → interpolate_in_space
  → multispace_sample_stops{,_radial}
  → adaptive_n_for_stops
  → PaintDefs::supports_native
  → write_definition / write_stops
```

Classificar as opções separadamente:

- **A — stops amostrados em `<linearGradient>/<radialGradient>`:** preferível
  se conserva geometria nativa e satisfaz o orçamento medido;
- **B — subdivisão adaptativa por erro do segmento:** usar somente se A com N
  fixo não satisfizer o contrato; o critério deve depender do erro medido, não
  apenas da distância entre stops originais;
- **C — fallback marcado:** obrigatório para qualquer espaço ainda não selado.

Não usar primeira cor como resultado `Preserved`. Não introduzir rasterização
do shape, pois ela altera a morfologia e resolução do conteúdo.

## 7. L0 e ADR-0127

Antes do código, ler integralmente e confirmar ownership/hash de:

- `00_nucleo/prompts/infra/export/svg.md`;
- `00_nucleo/prompts/infra/export/gradients/adaptive.md`;
- prompts de `gradients/linear.rs` e `gradients/radial.rs` se forem alterados;
- `00_nucleo/prompts/entities/gradient.md` apenas se a medição exigir mudança
  na entidade;
- ADR-0107, ADR-0108, ADR-0109, ADR-0127 e ADR-0129.

Atualizar primeiro somente o L0 de cada consumer efetivamente alterado. Se a
solução exigir novo campo/método público, mudança de fase, novo default ou
compatibilidade pública, parar no gate ADR-0127. Sampling interno de paridade
em L3 segue fluxo contínuo: L0 → resselo → RED→GREEN.

V15/V26 devem passar antes de `--fix-hashes`.

## 8. Testes RED independentes

Congelar antes da implementação:

1. default/Oklab Linear satisfaz o orçamento medido e não usa primeira cor;
2. default/Oklab Radial preserva center, radius, focal-center e focal-radius;
3. cada espaço aprovado preserva offsets, ordem e alpha;
4. stops coincidentes continuam coincidentes e em ordem;
5. fill e stroke apontam para servidores válidos e separados quando preciso;
6. transformação por rename de IDs mantém o veredito;
7. baixo/médio/alto contraste exercitam todos os níveis adaptativos aceitos;
8. um caso adversarial entre stops originais força refinamento adicional;
9. espaço não aprovado conserva fallback explícito `Unknown`;
10. sRGB nativo não sofre sampling nem regressão.

Confirmar RED por ausência do servidor não-sRGB, erro acima do limite ou
fallback focal — nunca por comparação byte a byte.

## 9. Ataques obrigatórios

O verificador deve rejeitar mutantes que:

- promovam primeira cor ou média das cores a gradient preservado;
- amostrem sempre `16`, `32` ou `64` sem verificar erro intermediário;
- calculem erro apenas nos stops originais;
- percam alpha ou o comparem somente depois de compositing em branco;
- ordenem, dedupliquem ou desloquem stops coincidentes;
- interpolem em sRGB quando o espaço declarado é outro;
- tratem quantidade exata de stops vanilla como semântica sem justificativa;
- quebrem focal geometry do Radial;
- emitam `url(#id)` sem definição correspondente;
- considerem IDs literais ou bytes como paridade;
- promovam todos os espaços porque um único caso visual passou;
- alterem o helper PDF sem prova de que o contrato compartilhado é válido;
- incluam Conic ou tiling opaco por arrasto;
- movam render para entities ou introduzam despacho dinâmico contra ADR-0109.

Contrato, oráculos, ataques, implementação e veredito devem ter autoridades e
capacidades segregadas. Em filesystem compartilhado, declarar literalmente:

```text
EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO
```

## 10. Implementação mínima condicionada

Implementar somente depois do contrato numérico selado:

- gerar stops sRGB equivalentes a partir do sampling no espaço declarado;
- preservar endpoints e inserir explicitamente offsets originais;
- preservar descontinuidades sem interpolar através de stops coincidentes;
- preservar alpha como `stop-opacity` separado;
- reutilizar a geometria SVG nativa Linear/Radial existente;
- deduplicar definições apenas por igualdade morfológica completa;
- manter fallback marcado para espaços que não satisfizerem o contrato.

Se for necessário compartilhar uma nova invariância entre SVG e PDF, criar ou
atualizar Núcleo Tekt somente após provar consumo `1:N`; não fazer dois prompts
apontarem ao mesmo consumer nem código apontar diretamente para Núcleo.

Fora de escopo:

- Conic;
- tiling com conteúdo opaco;
- rasterização do gradient;
- mudanças no eval ou na entidade sem RED próprio;
- igualdade byte a byte com o SVG vanilla.

## 11. Adjudicação e mapa DSM

Classificar por espaço e variante, sem um veredito agregado enganoso:

- `Preserved`: geometria, papel, offsets, ordem, alpha e interpolação dentro do
  contrato numérico;
- `Unknown`: método/orçamento ainda não selado;
- `Violated`: perda ou erro acima do limite em obrigação já contratada.

Produzir:

- `p1229-svg-multispace-contract.tsv`;
- `p1229-svg-multispace-oraculos.tsv`;
- `p1229-svg-multispace-ataques.tsv`;
- `p1229-svg-multispace-resultados.tsv`;
- `p1229-svg-multispace-error-budget.tsv`;
- `p1229-tekt-manifesto.tsv` e `p1229-tekt-certificado.tsv`;
- `typst-p1229-svg-multispace.md`;
- atualização limitada de `dsm/typst-correspondencias-v1.toml` e
  `p1213-fila-implementacao.tsv`.

`svg-paint-servers` só deixa `parcial` se todos os `Unknown` restantes tiverem
outro cluster proprietário e nenhuma obrigação Linear/Radial contratada ficar
aberta.

## 12. Gates finais

```text
cargo test -p typst-core gradient
cargo test -p typst-infra p1229
cargo test -p typst-infra export::svg::tests
cargo test -p typst-infra p274
cargo build
cargo fmt --all -- --check
git diff --check
crystalline-lint --checks v5,v15,v26 --fail-on warning .
crystalline-lint .
../tekt-cargo-dsm/target/release/lente --comparar \
  --antes lab/typst-original --depois . \
  --mapa-correspondencia 00_nucleo/diagnosticos/dsm/typst-correspondencias-v1.toml
```

Repetir matriz, oráculo numérico, rasters focais e comparação pública duas
vezes em ordens inversas. Registrar proveniência de toda contagem ou erro usado
para fechar um espaço.

## 13. Critério de encerramento

O passo termina quando:

- o vanilla tem método e erro medidos por espaço/variante;
- tolerâncias foram fixadas antes de observar o patch candidato;
- pelo menos default/Oklab Linear e Radial têm veredito reproduzível, mesmo que
  esse veredito ainda seja `Unknown` por evidência insuficiente;
- todo espaço promovido passa o orçamento em casos normais e adversariais;
- offsets coincidentes, ordem, alpha e focal geometry permanecem intactos;
- sRGB nativo não regride;
- espaços não selados mantêm fallback explicitamente `Unknown`;
- mapa e fila dizem exatamente o que fechou e o que continua aberto;
- gates finais e lente passam;
- nenhuma alegação inclui Conic ou tiling opaco.
