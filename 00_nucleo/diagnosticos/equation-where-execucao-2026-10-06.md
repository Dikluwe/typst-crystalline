# D05 — math.equation.where corrigido

Estado: concluído no escopo medido. Seletores de equação aceitam filtros e aliases,
funcionam em query/show e conservam a separação de bloco quando show produz texto.
Não se certifica paridade universal de math.

## Proveniência

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado.
Evidências em `equation-where-2026-10-06/`. As coletas guardam UTC, HEAD,
`git diff HEAD --stat`, lista exata de alterações, hashes das fontes,
argv/cwd/ambiente e hashes dos binários congelados. `final-code.json.gz`
guarda também o conteúdo das fontes modificadas. `before.json.gz` é a coleta
anterior e contém hashes das fontes; não é um snapshot integral de conteúdo.

Coleta final: 2026-10-06T20:25:31.543054+00:00 a
2026-10-06T20:25:39.905466+00:00.
Vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`, SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
Tekt release SHA-256
`c2fd3dfa765e2f1135036b20bfd78d4d776011d4821a49a9a5a52dc87db747b1`.
A regressão query foi medida com os mesmos binários, entre
2026-10-06T20:25:31.543115+00:00 e 2026-10-06T20:25:40.048365+00:00.

## Medição anterior à decisão

Na árvore upstream pinada, `crates/typst-library/foundations/func.rs:433–462`
valida campos de elementos; `crates/typst-library/math/equation.rs:58–170`
distingue campos públicos e internos. A coleta before registra a rejeição de
math.equation.where como função não-elemento no Tekt.

No estado anterior, `01_core/src/compiler/eval/bindings/value_methods/element_where.rs:63–490`
omitia equation do catálogo e `01_core/src/compiler/eval/selector_matching.rs:269–310`
omitia sua identidade. `compiler/introspect/from_tags/equations.rs:161–166`
produzia a vista Styled sem snapshot; a coleta after revelou queries vazias após
corrigir apenas a admissão. `compiler/eval/expression_dispatch.rs:270–281`
criava o literal sem interceptar a receita lexical existente, ao contrário do
caminho de inserção em `compiler/eval/markup_stream.rs:119`.

A coleta delivered revelou o resultado textual de show de bloco na mesma linha
anterior. `compiler/layout/mod.rs:1172–1175` emitia apenas o visual de Realized;
`compiler/layout/paragraph.rs:114–130` já oferecia a fronteira de parágrafo.
As linhas citadas são das fontes na medição anterior, preservadas nas evidências.

## Decisão e materialização

São diferenças de linguagem: admissão de campos, identidade observável do
seletor, aplicação de show e papel de bloco do resultado. Não se exige igualdade
de estruturas Rust, passos internos ou bytes PDF. Correção de paridade em fluxo
contínuo ADR-0127: L0 atualizado primeiro, testes RED, implementação, resselo.
Não há assinatura pública, enum, flag, default ou transferência de fase nova.

O catálogo estático admite block, numbering, number-align, supplement, alt e body,
além do label comum; size interno é rejeitado. Aliases usam identidade tipada
da função, sem inferir pelo nome nem invocar o constructor. O bridge da stdlib
é privado ao compiler (`pub(in crate::compiler)`).

Matching reconhece Equation e sua vista Styled, restrita à identidade de equation,
e conserva projeções causais existentes. O literal captura show lexical pela
interceptação existente, avaliando o corpo uma vez. Query consulta o índice de
label apenas quando o snapshot legado é None; Some, inclusive vazio, permanece
autoritativo. Não se fabrica snapshot nem se muda ordem de resultados.

Na unidade Realized, original Equation de bloco com visual textual certificado
e não vazio usa as fronteiras de parágrafo existentes, preservando as âncoras.
Inline mantém seu fluxo. Empty, espaços apenas, visual opaco e Realized aninhado
não recebem essa certificação; seguem o fallback existente. Não se reavalia o
original nem se cria Content ou Location.

## Verificação

RED genuíno: valid-red.log, wrappers-show-red.log, label-valid-red.log e
block-red.log. As tentativas red.log e label-red.log falharam na compilação e
não são usadas como prova RED. GREEN final: block-green.log, cinco testes
focados aprovados.

`final.json.gz`: 29/29 controles bilaterais coincidem em status, stdout, stderr e
texto extraído de PDF. Incluem campos públicos, rejeições, aliases, filtros,
label, corpo, estilos e show inline/bloco. O controle composto com operador `|`
é rejeitado pelos dois e não comprova composição válida; o controle `.or(...)`
comprova composição aceita. Bytes PDF não são critério de aceitação.

`../query-compilada-2026-10-06/final-equation-where.json.gz`: 36/36 controles
coincidem, inclusive os 23 originais e os corpos de aspas corrigidos anteriormente.
`final-results.json` resume as duas coletas e suas proveniências.

- Núcleo: 6566 aprovados, zero falhas, cinco ignorados (final-core.log).
- Infraestrutura: 1192 aprovados, zero falhas (final-infra.log).
- CLI query: dez aprovados, zero falhas, 116 filtrados (final-cli-query.log).
- cargo build --offline e build release de typst-wiring aprovados.
- crystalline-lint: zero erros, 527 warnings e 1585 notes (final-lint.sarif).
  V5/V6/V15/V26 sem erros; warnings não são certificados como resolvidos.

Resselo restrito a dez owners/consumers 1:1, registrado em seals.json.
`preservation.json` confirma todos os selos e nenhuma mudança de fonte/L0 fora
desses owners em relação à coleta inicial. Alterações anteriores foram preservadas;
não foi usado resselo global. `final-checks.json` registra hashes dos receipts.

D06 (math bare automático), D07 (morfologia de matriz) e D08 permanecem separados.
Os negativos anteriores de Styled geral, show text e root lang HTML também não
são encerrados por esta correção.
