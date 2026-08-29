# P1230 — selar e materializar Gradient Conic no SVG sem arrastar CMYK ICC

**Estado:** EXECUTADO — `ACCEPTED_WITH_CONTRACTUAL_UNKNOWN`  
**Predecessor causal:** P1229  
**Cluster:** `svg-morphology`, `PARTIAL`  
**Gap vigente:** Conic conserva fallback `conic-gradient`; CMYK ICC permanece
NO-GO vigente por ADR-0097 e não pode ser incorporado por arrasto.

## 1. Objetivo

Medir o observável do Conic SVG no vanilla ratificado `a51e02804`, selar um
contrato morfológico e numérico independente e implementar um servidor Conic
somente se a saída satisfizer esse contrato sem violar os owners, as camadas ou
as ADRs vigentes.

Resultado condicionado à evidência:

```text
P1230 SVG CONIC SEALED — REMAINING PAINT UNKNOWNS NAMED
```

Este passo não reabre CMYK ICC. ADR-0097 registra P273.14 como NO-GO por
licenciamento, crate externa e invariantes L0. Se uma solução Conic depender
disso, manter apenas Conic/CMYK `Unknown`.

## 2. Baseline e proveniência

Antes de decidir, registrar:

- `git rev-parse HEAD`, horário, `git status --short` e
  `git diff HEAD --stat`;
- working tree não commitada com lista integral dos paths alterados;
- SHA-256 do cristalino e dos dois binários vanilla ratificados;
- SHA-256 deste passo, mapa DSM, comparador, fixtures e scripts de oráculo;
- SHA-256 dos L0, ADRs e consumers efetivamente usados.

Não ler `00_nucleo/context/` nem `00_nucleo/materialization/`.

## 3. Medir antes de escolher engenharia

Confirmar no vanilla com `file:line`:

```text
typst-svg/src/paint.rs
  NUM_CONIC_SEGMENTS
  Gradient::Conic
  SVGSubGradient
  write_subgradients
  write_gradient_refs
```

A fonte ratificada atualmente mostra uma decomposição em `360` setores:
um `<pattern>` contém paths em cunha, cada um preenchido por um
`<linearGradient>` de duas cores amostradas nos extremos. A contagem literal
`360` é inicialmente mecânica/heurística, não obrigação de paridade.

Separar:

- **semântica:** progressão angular, espaço de interpolação, offsets, alpha e
  descontinuidade na volta completa;
- **morfologia:** Conic continua vetorial, centro/ângulo, fill/stroke,
  transformações e clipping;
- **mecânica:** IDs, ordem de `<defs>`, bytes, whitespace, número exato de
  setores e nomes dos subgradients.

## 4. Matriz vanilla obrigatória

Executar duas vezes em ordem direta e inversa:

| ID | Caso | Variações mínimas |
|---|---|---|
| C1 | geometria default | center default, angle `0deg` |
| C2 | centro e rotação | centers `(20%,30%)`, `(80%,65%)`; `25deg`, `-90deg` |
| C3 | aspect ratio | caixas `1:1`, `4:1`, `1:4` |
| C4 | stops | dois; `0%,37%,100%`; coincidentes `0%,0%,100%` |
| C5 | alpha | alpha em primeiro, intermediário e último stop |
| C6 | papel | fill e stroke rico no mesmo documento |
| C7 | transform | translate, rotate, scale não uniforme e composição |
| C8 | espaço | sRGB, Oklab, LinearRgb, Hsv e CMYK como controle Unknown |

Para cada caso, registrar o SVG como grafo:

- paint aplicado → pattern/ref local;
- pattern → setores;
- setor → subgradient;
- subgradient → cores/alpha;
- transforms acumulados por papel fill/stroke.

Rasterizar em pelo menos `1x`, `2x` e `4x`. Raster confirma o contrato, mas
não substitui a inspeção estrutural.

## 5. Oráculo independente

Congelar antes da implementação um oráculo fora do exportador candidato que:

1. calcula `t` pelo ângulo do pixel em relação ao centro, respeitando rotação,
   sentido e aspect ratio medidos;
2. consulta a cor pública esperada do vanilla numa malha angular que inclui
   stops, fronteiras de setor e epsilon dos dois lados;
3. compara sRGB codificado premultiplicado e alpha separadamente;
4. reporta erro máximo, p95 e posição do pior erro por caso/espaço;
5. mede continuidade dentro dos setores e a descontinuidade intencional em
   stops coincidentes;
6. refina resolução e malha até a decisão estabilizar duas vezes.

O budget nasce da medição do vanilla mais margem justificada. Não copiar
automaticamente o limiar P1229: Conic aproxima no domínio angular e pode exigir
um contrato distinto.

## 6. Opções de engenharia

Classificar somente depois da medição:

- **A — pattern vetorial de cunhas + subgradients:** primeira candidata por
  conservar a morfologia do vanilla;
- **B — tesselação adaptativa por erro angular:** preferível a uma contagem
  fixa se reduzir custo e ainda satisfizer o budget;
- **C — fallback marcado por combinação:** obrigatório para espaços ou
  geometrias que não passem;
- **D — rasterização:** proibida neste passo, pois altera morfologia e torna o
  resultado dependente da resolução.

Não introduzir `dyn`, vtable, render em `entities`, import reverso ou mudança
no helper PDF. Deduplicação só pode usar igualdade morfológica completa.

## 7. L0 e gate das ADRs

Antes de código, ler integralmente:

- `00_nucleo/prompts/infra/export/svg.md`;
- `00_nucleo/prompts/infra/export/gradients/conic.md`;
- `00_nucleo/prompts/infra/export/gradients/adaptive.md` se reutilizado;
- `00_nucleo/prompts/entities/gradient.md` somente se a entidade for tocada;
- ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 e ADR-0097.

Auditar ownership `1:1` e pins antes de resselo. Atualizar primeiro somente o
L0 dos consumers alterados.

Parar no gate ADR-0127 se surgir novo campo/método público, comportamento por
default, mudança de fase ou quebra de compatibilidade. Uma implementação
interna L3 de paridade segue fluxo contínuo L0 → resselo → RED→GREEN.

## 8. Testes RED

Congelar RED independentes que provem:

1. Conic aprovado emite pattern vetorial, não primeira cor;
2. centro, ângulo, sentido e aspect ratio satisfazem o oráculo;
3. fill e stroke resolvem para servidores válidos;
4. alpha permanece separado e numericamente correto;
5. offsets e stops coincidentes preservam ordem/descontinuidade;
6. transforms não são assados duas vezes;
7. rename de IDs e reorder de defs não mudam o veredito;
8. caso adversarial entre setores excede o budget de uma tesselação
   insuficiente;
9. combinação não aprovada mantém `conic-gradient`/Unknown;
10. Linear/Radial P1229 e tiling existente não regridem.

## 9. Ataques obrigatórios

O verificador deve rejeitar mutantes que:

- usem primeira cor, última cor ou média global;
- invertam sentido ou ignorem angle/center;
- assumam caixa quadrada ou percam correção de aspect ratio;
- usem setores fixos sem verificar o budget;
- deixem fendas angulares ou sobreposições observáveis;
- percam alpha, coincidência ou fronteira `0/1`;
- emitam `url(#id)` pendente;
- comparem bytes, IDs ou contagem literal como semântica;
- promovam todos os espaços por um único caso aprovado;
- promovam CMYK apesar do budget ou contornem ADR-0097;
- alterem PDF, rasterizem shapes ou movam render para L1;
- transformem `Unknown` em `Preserved` sem evidência.

Contrato, oráculo, ataques, implementação e veredito devem usar autoridades
segregadas. Em filesystem compartilhado, declarar literalmente:

```text
EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO
```

## 10. Implementação mínima condicionada

Se A ou B passar:

- manter a lógica em `03_infra/src/export/svg.rs` ou em owner L3 exclusivo
  chamado estaticamente por ele;
- emitir pattern, cunhas e subgradients com grafo local resolvido;
- preservar centro/ângulo/aspect ratio/transforms e papéis fill/stroke;
- serializar alpha sem compositing antecipado;
- parametrizar/adaptar setores pelo erro contratado se a contagem literal do
  vanilla não for necessária;
- manter fallback explícito para cada combinação não aprovada.

Se nenhuma opção vetorial satisfizer o contrato, produzir diagnóstico NO-GO
e conservar o fallback atual; isso é resultado válido, não autoriza relaxar o
budget depois de observar o candidato.

## 11. Adjudicação e mapa

Classificar por espaço e papel:

- `Preserved`: grafo, geometria, alpha, transforms e erro dentro do budget;
- `Unknown`: método/budget não selado ou bloqueio ADR vigente;
- `Violated`: obrigação contratada com perda ou erro acima do limite.

Produzir:

- `p1230-svg-conic-contract.tsv`;
- `p1230-svg-conic-oraculos.tsv`;
- `p1230-svg-conic-error-budget.tsv`;
- `p1230-svg-conic-ataques.tsv`;
- `p1230-svg-conic-resultados.tsv`;
- `p1230-tekt-manifesto.tsv` e `p1230-tekt-certificado.tsv`;
- `typst-p1230-svg-conic.md`;
- atualização limitada do mapa DSM e da fila P1213.

`svg-paint-servers` permanece `parcial` enquanto tiling opaco ou combinações
de cor declaradas `Unknown` tiverem owners futuros nomeados.

## 12. Gates finais

```text
cargo test -p typst-infra p1230
cargo test -p typst-infra export::svg::tests
cargo test -p typst-infra p1229
cargo test -p typst-core gradient
cargo build
cargo fmt --all -- --check
git diff --check
crystalline-lint --checks v5,v15,v26 --fail-on warning .
crystalline-lint .
../tekt-cargo-dsm/target/release/lente --comparar \
  --antes lab/typst-original --depois . \
  --mapa-correspondencia 00_nucleo/diagnosticos/dsm/typst-correspondencias-v1.toml
```

Repetir matriz, oráculo, rasters e grafo duas vezes em ordens inversas. Toda
contagem ou erro usado para promover uma combinação deve registrar HEAD,
working tree e horário reproduzíveis.
