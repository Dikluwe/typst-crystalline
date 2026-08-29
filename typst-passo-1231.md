# P1231 — fechar Linear/Radial multi-space não-CMYK no SVG

**Estado:** EXECUTADO — REJEITADO PELO CONTROLE R01  
**Predecessor causal:** P1229 e P1230  
**Cluster:** `svg-morphology`, `PARTIAL`  
**Gap focal:** nove combinações Linear/Radial não-CMYK ainda conservam
fallback `gradient-color-space` e classificação `Unknown`.

## 1. Objetivo

Medir novamente, contra o vanilla ratificado `a51e02804`, as combinações
Linear/Radial que P1229 não promoveu e fechar mecanicamente todas as que
satisfizerem um orçamento independente mediante refinamento adaptativo,
conversão ou geometria corrigida no owner SVG.

O universo fechado é:

| Espaço | Linear | Radial |
|---|---|---|
| Oklab | já `Preserved` | medir |
| Oklch | medir | medir |
| LinearRgb | já `Preserved` | medir |
| Luma | medir | medir |
| Hsl | medir | medir |
| Hsv | medir | já `Preserved` |

São nove casos candidatos. sRGB e as três promoções P1229 são controles de
regressão. CMYK Linear/Radial é controle `Unknown-ADR0097`, não candidato.
Conic P1230 e tiling não pertencem a este passo.

Resultado condicionado à evidência:

```text
P1231 SVG MULTI-SPACE FRONTIER SEALED — NON-CMYK CASES DECIDED
```

## 2. Por que este gap vem antes de tiling opaco

O mapa P1230 deixa três famílias abertas: CMYK, as combinações de cor não
seladas e tiling de conteúdo. CMYK continua NO-GO por ADR-0097. Tiling de
conteúdo arbitrário não cabe hoje no contrato público cristalino:

- `entities/tiling.md` limita `TilingBody` a Image/Gradient/Color;
- `tiling-stdlib.md` rejeita Gradient e não aceita Content arbitrário;
- offset e angle públicos também estão ausentes.

Corrigir tiling exigiria contrato público e comportamento do produto, logo
gate ADR-0127 e um passo próprio. P1231 permanece correção interna de paridade
no consumer L3 já existente.

## 3. Baseline e proveniência

Antes de decidir, registrar:

- `git rev-parse HEAD`, horário, `git status --short` e
  `git diff HEAD --stat` integral;
- working tree não commitada e todos os paths alterados;
- SHA-256 deste passo, mapa DSM, L0, ADRs, scripts, fixtures, comparador e
  binários cristalino/vanilla;
- confirmação dos dois binários vanilla ratificados e do pin `a51e02804`;
- hashes das evidências P1229 e P1230 usadas como predecessor.

Não ler `00_nucleo/context/` nem `00_nucleo/materialization/`.

## 4. Medir antes de escolher engenharia

Reabrir a medição P1229, sem herdar sua decisão como verdade permanente:

1. localizar no vanilla `typst-svg/src/paint.rs` o sampling e a emissão de
   subgradients/stops;
2. medir `Gradient::sample` público para cada espaço e variante;
3. separar erro de cor, erro de geometria radial, alpha e descontinuidade;
4. identificar por `file:line` se cada falha anterior veio de:
   - profundidade/cap insuficiente;
   - função de erro inadequada;
   - conversão do espaço para sRGB;
   - interpolação polar/hue;
   - offsets coincidentes ou alpha;
   - geometria radial/focal;
5. marcar inferências e o witness que as refutaria.

Não exigir igualdade de stops, bytes, IDs, ordem de defs, profundidade de
recursão ou algoritmo do vanilla. Esses itens são mecânica, conforme ADR-0107.

## 5. Matriz de fixtures

Para cada uma das nove combinações candidatas, executar isoladamente:

- dois stops opacos saturados;
- três stops em `0%, 37%, 100%`;
- stops coincidentes em `0%, 0%, 100%`;
- alpha no primeiro, intermediário e último stop;
- caixa quadrada, larga e alta;
- fill e stroke;
- Linear com ângulos `0deg`, `25deg`, `-90deg`;
- Radial com center/focal/radius default e não-default;
- hue cruzando a costura, inclusive rota curta/longa observada no vanilla.

Executar duas rodadas em ordem direta e inversa. Rasterizar em `1x`, `2x` e
`4x`, além de comparar amostras públicas nos stops, midpoints e epsilon dos
dois lados de cada fronteira.

## 6. Contrato e orçamento independentes

Congelar antes do candidato:

- grafo SVG local resolvido e variante Linear/Radial correta;
- geometria, papel fill/stroke, offsets, ordem, alpha e descontinuidades;
- erro em sRGB codificado premultiplicado e alpha separado;
- bbox/máscara derivada do grafo e geometria local antes dos deltas;
- layout/grid/gutter/flow externo como `Unknown`, sem servir para perdoar
  falha numa fixture isolada.

O orçamento nasce do vanilla e deve estabilizar em duas resoluções/refinamentos.
Não copiar cegamente P1229 ou P1230. Se o orçamento tiver de variar por espaço,
registrar a medição que justifica cada limite antes da classificação.

## 7. Opções de engenharia

Classificar depois da medição:

- **A — ampliar o refinamento adaptativo:** mudar cap/profundidade somente se
  o orçamento justificar o custo e houver limite determinístico;
- **B — corrigir métrica/subdivisão:** incluir alpha premultiplicado, hue ou
  pontos críticos que o midpoint sozinho não detecta;
- **C — conversão local correta:** corrigir fórmula interna no owner de cor ou
  adaptive somente quando o L0 proprietário legitimar a mudança;
- **D — fallback por combinação:** manter `Unknown` quando nenhuma opção
  vetorial satisfizer o contrato;
- **E — rasterização:** proibida, pois altera a morfologia SVG.

Preferir uma regra comum apenas se os nove casos a sustentarem. Não promover
um espaço/variante por analogia com outro. Não adicionar crate, ICC, download,
`dyn`, vtable ou mudança no helper PDF.

## 8. L0 e gate ADR-0127

Ler integralmente antes de código:

- `00_nucleo/prompts/infra/export/svg.md`;
- `00_nucleo/prompts/infra/export/gradients/adaptive.md`;
- `00_nucleo/prompts/entities/gradient.md` somente se a entidade for tocada;
- ADR-0097, ADR-0107, ADR-0108, ADR-0127 e ADR-0129.

Auditar Prompt L0 ↔ consumer `1:1` e pins V15/V26. Atualizar primeiro somente
o L0 dos consumers realmente alterados e ressellar hashes.

Fluxo contínuo é permitido para fórmula interna, tabela de combinações e
correção de paridade. Parar se surgir método/campo público, comportamento por
default, mudança de fase, quebra de compatibilidade ou necessidade de alterar
o contrato público de cor/gradient.

## 9. Testes RED→GREEN

Antes da implementação, congelar RED que provem ao menos:

1. cada combinação promovida deixa de emitir fallback marcado;
2. variante e grafo locais fecham sem URL pendente;
3. geometria Linear/Radial permanece correta;
4. alpha e stops coincidentes satisfazem o oráculo;
5. hue cruza a costura pelo lado medido no vanilla;
6. fill e stroke passam separadamente;
7. ordem direta/inversa e repetição são determinísticas;
8. sRGB, P1229 e Conic P1230 não regridem;
9. CMYK continua explicitamente `Unknown-ADR0097`.

Registrar o RED real antes de implementar. GREEN unitário não substitui a
matriz numérica independente.

## 10. Ataques obrigatórios

O verificador deve rejeitar mutantes que:

- promovam todas as combinações por uma única aprovação;
- usem primeira/última cor ou média global;
- amostrem apenas endpoints ou midpoint;
- escondam pico estreito entre pontos de amostragem;
- invertam hue ou escolham a volta errada na costura;
- descartem alpha ou façam compositing antecipado;
- dedupliquem/reordenem stops coincidentes;
- ignorem focal point, aspect ratio, fill ou stroke;
- aceitem por contagem literal de stops ou igualdade de bytes/IDs;
- derivem crop/máscara de cor, alpha ou mapa de erro;
- aumentem cap sem limite de custo reproduzível;
- promovam CMYK, incorporem ICC ou alterem PDF/L1 por arrasto;
- convertam caso sem evidência em `Preserved`.

Exigir mutation score `1.0` sobre todos os mutantes negativos válidos. O
controle CMYK deve receber `Unknown`, fora do denominador negativo.

## 11. Materialização segregada

Usar protocolo Tekt completo com autoridades segregadas para:

1. manifesto e obrigações;
2. contrato e budgets;
3. oráculo vanilla;
4. implementação;
5. ataques discriminativos;
6. verificação e veredito final.

Contrato, oráculo e ataques ficam protegidos antes da leitura do candidato.
Em filesystem compartilhado, o certificado declara literalmente:

```text
EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO
```

## 12. Adjudicação e documentação

Classificar cada par espaço/variante/papel:

- `Preserved`: grafo, geometria e budget passam;
- `Unknown`: combinação não selada, método esgotado ou bloqueio vigente;
- `Violated`: obrigação contratada falha na fixture local.

Produzir:

- `p1231-svg-multispace-frontier-contract.tsv`;
- `p1231-svg-multispace-frontier-oraculos.tsv`;
- `p1231-svg-multispace-frontier-error-budget.tsv`;
- `p1231-svg-multispace-frontier-ataques.tsv`;
- `p1231-svg-multispace-frontier-resultados.tsv`;
- `p1231-tekt-manifesto.tsv` e `p1231-tekt-certificado.tsv`;
- `typst-p1231-svg-multispace-frontier.md`;
- atualização limitada de `svg-paint-servers` no mapa DSM e da fila P1213.

O mapa deve nomear exatamente quais das nove combinações fecharam e quais
permaneceram `Unknown`. O cluster continua `PARTIAL` enquanto CMYK ou tiling
de conteúdo estiverem abertos.

## 13. Gates finais

```text
cargo test -p typst-infra p1231
cargo test -p typst-infra p1229
cargo test -p typst-infra p1230
cargo test -p typst-infra export::svg::tests
cargo test -p typst-core gradient
cargo build
cargo fmt --all -- --check
git diff --check
crystalline-lint --checks v5,v15,v26 --fail-on warning .
crystalline-lint .
../tekt-cargo-dsm/target/release/lente --comparar \
  --antes lab/typst-original \
  --depois . \
  --mapa-correspondencia \
  00_nucleo/diagnosticos/dsm/typst-correspondencias-v1.toml
```

Aceitar somente com gates verdes, proveniência completa e veredito segregado.

## 14. Stop conditions

Parar sem promover se:

- contrato/oráculo/ataques não discriminarem todos os mutantes válidos;
- orçamento só passar após observar e afrouxar para o candidato;
- a correção exigir mudança pública sem novo gate ADR-0127;
- houver necessidade de ICC/CMYK, rasterização ou alteração PDF/L1;
- surgir regressão P1229/P1230 ou violação V5/V15/V26;
- não for possível separar erro local de layout externo sem fitting por erro.
