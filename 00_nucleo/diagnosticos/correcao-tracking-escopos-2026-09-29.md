# Correção de tracking em escopos lexicais — 2026-09-29

Corrigidas as fronteiras de tracking entre escopos explícitos, inclusive quando repetem o mesmo valor, entradas/saídas aninhadas e as fronteiras de caixas e conteúdo oculto. O owner text_tracking conserva a regra de continuidade; o hub armazena seu estado e os produtores apenas registram emissão efetiva. Não há fatores de compensação numéricos, flags ou contrato público novo.

## Medição anterior à decisão

No binário anterior, os controles equal-local/adjacent-equal com ±0.2pt diferem cerca de 0.4pt; nested acumula cerca de 0.8pt. O wrap positivo muda a linha, com diferença de posição de 20.272001pt. Os valores exatos por entrada estão em `correcao-tracking-escopos-2026-09-29/before-pdf/summary.json`; foram medidos antes de redigir a alteração L0, em 30 fontes e 90 PDFs.

Fonte causal: `01_core/src/compiler/layout/text_tracking.rs:50` comparava só o TextStyle resolvido. `lab/typst-original/crates/typst-layout/src/inline/collect.rs:287-295` preserva segmentos de cadeias distintas; `inline/shaping.rs:1269-1295` acrescenta tracking dentro de cada run, até o último cluster. O observável requerido é ausência/presença do intervalo na fronteira lexical; não a igualdade Rust, ponteiros ou representação de dados do upstream. Não inferimos intenção de desenvolvimento a partir dessa falha.

Estado: HEAD `17f13b1df79b87ea065ee62e78164b18e477da3d`, working tree não commitada. `initial.json`, `before/` e `before-pdf/provenance.json` registram UTC, fontes, lista exata das alterações, diff HEAD, binários, comandos e hashes. O snapshot completo de before-pdf inclui também Boxed, acrescentado ao manifesto após confirmar sua participação na fronteira. Vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`, binário SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`. O caminho do oráculo é lab/typst-original/target/release/typst.

## Implementação e organização

State privado em `01_core/src/compiler/layout/text_tracking.rs` distingue escopo ativo e último produtor textual com identidades opacas locais. Ao entrar em Styled não vazio, Strong ou Emph, muda a identidade ativa; ao sair, restaura apenas essa identidade. Texto/separador emitido registra seu escopo. Assim a saída de um escopo com tinta interrompe o intervalo, enquanto um escopo vazio conserva a continuidade anterior. A elegibilidade uniforme/LTR/math e o cálculo assinado permanecem na função pura existente. Nenhuma identidade é serializada.

Word/Space usam State::boundary e registram o produtor após emissão. Wrap continua projetando o intervalo antes da seleção e removendo-o no início da próxima linha. Hide e Box interrompem o intervalo por serem objetos, mesmo quando o percurso interno deixa folhas textuais no buffer. Checkpoint captura/restaura o State; subframe isolado e colunas irmãs o restauram junto à linha exterior. A troca temporária para alinhar cauda inline preserva a proveniência do body.

Nove owners foram atualizados, com L0 primeiro e resselo exclusivamente manifestado em scope.json. O Núcleo de tracking permanece inalterado: a proibição de ampliar sem controles foi respeitada pelas medições. Funções e estado permanecem na camada de render; entidades, dispatcher exaustivo e composição estática foram preservados. `review.diff.txt`, `scoped-seals.json` e os scripts arquivados permitem revisão da mudança.

## Verificação

- RED causal: escopo que repete tracking falha antes da implementação, escopo vazio passa. `red.log`. GREEN causal e teste adicional de restauração pelo checkpoint passam; suíte final em core-tests.log.
- Core: 6.372 unitários, 8 integração e 3 doctests; infra: 1.137 unitários e 34 integração. Total 7554 passaram, zero falhas; 2 testes existentes ignorados no core.
- Builds offline de desenvolvimento e release passaram, em build.log e release-build.log.
- Lint: zero erros, 503 warnings e 1557 notes, em lint-final.sarif. Não se declara ausência de avisos.
- 42 controles bilaterais/126 PDFs, incluindo os 30 causais e 12 adicionais de fonte/tamanho/idioma repetidos e adjacência sem override. Mesma sequência de glyphs visíveis e páginas; diferença máxima de origem 0.000031000pt, tolerância 0.001pt. Entre padrão/compacto, máximo 0.000018000pt. Evidência em after-pdf/results.json, summary.json e mode-summary.json. Os 12 adicionais foram controles posteriores, não usados para alegar RED anterior.
- 36 controles uniformes/108 PDFs continuam alinhados ao vanilla, máximo 0.000030pt. Evidência em uniform/; o runner também valida a coincidência dos modos.
- 42 controles ampliados/126 PDFs recompilados. 38 preservam palavras/caixas e dimensões dentro de 0.015pt; mixed-tracking passa agora. As quatro divergências remanescentes estão listadas abaixo, sem removê-las do lote.
- 401 entradas anteriores recompiladas: texto, geometria de páginas/caixas e diagnósticos anteriores preservados, zero falhas, tolerância 0.001pt. A referência aqui é o estado corrigido da rodada anterior, não igualdade nova com vanilla. Evidência em regressions/results.json e provenance.json.

Novo binário SHA-256 `89f9fc6765e760c011b38f8a9520db135b3dd3f19cb4c3750377a1d64cd1d756`. after-pdf/provenance.json conserva hashes completos das fontes e working tree da medição. final-receipt.json confirma que as fontes não mudaram durante a verificação. Não foram regenerados goldens; não se usa igualdade de bytes PDF como critério de paridade.

## Limites ainda abertos

Quatro entradas do lote ampliado divergem: auto-spaces-only com ±0.2pt preserva palavras mas diverge nas dimensões automáticas da página; rtl-explicit com ±0.2pt diverge 0.8pt nas caixas. Esses caminhos permanecem fora da elegibilidade desta correção.

Preparação global ainda recusa tracking: a distribuição paginada de 3+1 contra 2+2, registrada no relatório anterior, permanece aberta. Fallback de medição por codepoints e seleção completa por clusters também não foram alterados. Não se declara paridade geral em RTL, clusters, paginação ou estado da arte.

## Histórico

[Separação dos owners](separacao-tracking-2026-09-29.md) → esta correção. [Correção de precisão e intervalos uniformes](correcao-tracking-2026-09-29.md) conserva a evidência anterior e suas qualificações. Próximas frentes: direção RTL, dimensão auto de espaços isolados e seleção/paginação global com tracking, cada uma precedida por medição própria.

## Seguimento posterior

[A correção RTL incremental](correcao-tracking-rtl-2026-09-29.md) fecha fronteiras e linhas continuadas nos controles latinos/árabes medidos. O quadro acima registra o estado anterior; hebraico, espaços literais iniciais, dimensão auto e preparação global continuam separados.
