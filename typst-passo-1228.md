# P1228 — localizar e fechar a perda pré-SVG de gradient com alpha e stops coincidentes

**Estado:** PRONTO PARA EXECUÇÃO  
**Predecessor causal:** P1227  
**Cluster:** `svg-morphology`, `PARTIAL`  
**Fresta medida:** a fixture pública `linear-fill-alpha-coincident.typ` é aceita
pelo vanilla ratificado e produz uma forma com gradient, mas o cristalino final
produz somente o fundo da página. A perda acontece antes do exportador SVG.

## 1. Objetivo

Encontrar a primeira fronteira em que desaparece a forma contendo gradient com
alpha e stops coincidentes, corrigir somente a obrigação pública comprovada e
levar a fixture RED→GREEN sem misturar este trabalho com aproximação Oklab,
conic ou tiling de conteúdo.

Resultado esperado:

```text
P1228 PRE-SVG PAINT LOSS CLOSED — COINCIDENT STOPS AND ALPHA PRESERVED
```

## 2. Baseline e proveniência obrigatória

Antes da primeira sonda, registrar:

- `git rev-parse HEAD`;
- `git status --short` e `git diff HEAD --stat`;
- horário ISO-8601;
- SHA-256 dos binários cristalino e vanilla ratificado `a51e02804`;
- SHA-256 da fixture, do comparador SVG, do mapa DSM e dos L0 relevantes;
- working tree não commitada como tal, com todos os paths alterados listados.

Não ler `00_nucleo/context/` nem `00_nucleo/materialization/`.

## 3. Hipóteses — não decidir antes da medição

As hipóteses iniciais são independentes e nenhuma pode ser assumida como causa:

1. despacho de método `red.transparentize(50%)` falha ou devolve tipo incorreto;
2. parser de stop não aceita tupla `(Color, Ratio)` após chamada de método;
3. dois offsets iguais são rejeitados, ordenados ou deduplicados;
4. o constructor cria o `Gradient`, mas `rect(fill:)` perde o `Paint`;
5. eval/layout preservam a forma, mas alguma filtragem anterior ao SVG a remove;
6. a fixture depende de uma segunda lacuna não relacionada.

Cada conclusão deve declarar o que foi medido, o que permanece inferido e qual
evidência a refutaria, conforme ADR-0108.

## 4. Matriz mínima de isolamento causal

Executar em ambos os binários, duas vezes e em ordem alternada:

| ID | Construção focal | Pergunta |
|---|---|---|
| A | `rect(fill: gradient.linear(red, blue, space: color.rgb))` | baseline de forma + gradient |
| B | `red.transparentize(50%)` via `repr`/`type` | método público funciona isoladamente? |
| C | gradient com `rgba(...)` direto, offsets `0%, 100%` | alpha sem method dispatch |
| D | gradient com `transparentize`, offsets `0%, 100%` | alpha via método sem coincidência |
| E | gradient opaco, offsets `0%, 0%, 100%` | coincidência sem alpha |
| F | gradient com alpha, offsets `0%, 0%, 100%` | composição das duas dimensões |
| G | fixture P1227 original em espaço default | reprodução integral |
| H | mesma fixture com `space: color.rgb` | separa perda pré-SVG de `Unknown` cromático |

Para cada caso registrar eval/repr, exit, stdout/stderr, quantidade e tipos de
`FrameItem`, SVG normalizado e hash do artefato. IDs SVG, whitespace e posição
textual de `<defs>` não são observáveis.

## 5. Localizar a primeira perda

Instrumentar por testes, não por logs produtivos, as fronteiras:

```text
source
  → AST/call de transparentize
  → Value::Color
  → parse_stops
  → Value::Gradient
  → ShapeElem.fill: Paint
  → FrameItem::Shape.fill: Paint
  → SVG PaintDefs/fallback
```

O diagnóstico deve apontar `file:line` para o último valor correto e o primeiro
valor incorreto. É proibido alterar o SVG para mascarar uma perda em eval ou
layout.

## 6. L0 e gate ADR-0127

Antes do código, ler integralmente e confirmar ownership/hash dos prompts dos
módulos efetivamente afetados, no mínimo entre:

- `00_nucleo/prompts/compiler/stdlib/color.md`;
- o Prompt L0 proprietário de `compiler/stdlib/gradients.rs`;
- `00_nucleo/prompts/compiler/stdlib/shapes.md`;
- `00_nucleo/prompts/compiler/layout/shape_block_behaviour.md`;
- `00_nucleo/prompts/infra/export/svg.md`.

Atualizar primeiro apenas o L0 do primeiro consumer que perde a informação.
Se a correção mudar campo/assinatura pública, comportamento por defeito, fase do
pipeline ou compatibilidade, parar no gate ADR-0127. Se for correção interna de
paridade, seguir em fluxo contínuo com RED→GREEN e resselo.

V15/V26 devem passar antes de qualquer `--fix-hashes`.

## 7. Testes RED independentes

Congelar antes da implementação:

1. `transparentize` preserva RGB e reduz alpha conforme o vanilla;
2. `parse_stops` preserva dois stops no mesmo offset e a ordem declarada;
3. gradient conserva alpha em cada `GradientStop`;
4. `ShapeElem.fill` conserva o `Paint::Gradient` integral;
5. layout emite `FrameItem::Shape` mesmo com stops coincidentes;
6. SVG sRGB resolve `url(#id)`, mantém ambos os offsets `0` e separa alpha;
7. espaço default continua `Unknown`/fallback marcado até existir contrato de
   interpolação — este passo não pode promovê-lo a `Preserved`.

Confirmar que ao menos um teste focal falha pela causa medida antes de escrever
a correção.

## 8. Ataques obrigatórios

O verificador deve rejeitar implementações que:

- troquem `transparentize` por cor opaca;
- removam o primeiro ou segundo stop coincidente;
- ordenem ou dedupliquem stops;
- convertam alpha para cor RGB sem opacidade;
- reduzam gradient a primeira cor antes de L3;
- emitam `url(#id)` quebrado;
- considerem IDs literais parte da paridade;
- classifiquem Oklab/default como `Preserved` por aparência;
- consertem somente a fixture por reconhecimento textual;
- movam lógica de render para entidades, contrariando ADR-0109.

Quando a segregação material for alegada, contrato, testes/ataques,
implementação e veredito devem ter autoridades e capacidades separadas. Se o
ambiente continuar compartilhado, executar como ensaio e declarar
`EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO`.

## 9. Implementação mínima

Corrigir exclusivamente a primeira perda comprovada. Preservar:

- enum fechado e despacho estático;
- ordem e multiplicidade dos stops;
- alpha como parte da cor do stop;
- `Paint` íntegro até L3;
- separação entre linguagem e mecânica de SVG;
- fallback explicitamente marcado para espaços ainda `Unknown`.

Não implementar neste passo:

- sampling adaptativo Oklab/CMYK/HSL/etc.;
- aproximação conic;
- tiling com corpo de conteúdo;
- transforms/relative ainda não contratados;
- igualdade byte a byte com o SVG vanilla.

## 10. Adjudicação e mapa DSM

Após GREEN, executar a fixture original e a variante sRGB em ambos os
binários. Classificar separadamente:

- **Preserved:** forma, papel fill/stroke, multiplicidade/ordem/offset/alpha;
- **Unknown:** interpolação de espaço sem contrato, conic e conteúdo opaco;
- **Violated:** qualquer perda semântica ou morfológica restante.

Atualizar:

- `p1228-pre-svg-paint-sondas.tsv`;
- `p1228-pre-svg-paint-ataques.tsv`;
- `p1228-pre-svg-paint-resultados.tsv`;
- `typst-p1228-pre-svg-paint-loss.md`;
- `dsm/typst-correspondencias-v1.toml`;
- `p1213-fila-implementacao.tsv`.

Somente fechar o subgap no mapa; `svg-paint-servers` permanece `parcial` até
que todos os `Unknown` nomeados tenham contratos próprios.

## 11. Gates finais

```text
cargo test -p typst-core p1228
cargo test -p typst-infra p1228
cargo test -p typst-wiring p1228
cargo test -p typst-infra export::svg::tests
cargo build
cargo fmt --all -- --check
git diff --check
crystalline-lint --checks v5,v15,v26 --fail-on warning .
crystalline-lint .
../tekt-cargo-dsm/target/release/lente --comparar \
  --antes lab/typst-original --depois . \
  --mapa-correspondencia 00_nucleo/diagnosticos/dsm/typst-correspondencias-v1.toml
```

Repetir os testes focais e a comparação pública duas vezes. Registrar a
proveniência de toda contagem usada para decidir encerramento.

## 12. Critério de encerramento

O passo termina quando:

- a primeira perda possui prova `file:line` reproduzível;
- a matriz A–H separa alpha, coincidência e espaço cromático;
- testes RED tornam-se GREEN pela correção mínima;
- a fixture sRGB conserva forma, stops coincidentes, ordem e alpha;
- a fixture default deixa de desaparecer, ainda que permaneça fallback
  explicitamente `Unknown` quanto à interpolação;
- mapa e fila registram exatamente o que fechou e o que continua aberto;
- build, formatação, V5/V15/V26 e lente passam;
- nenhuma alegação excede o fragmento público medido.

