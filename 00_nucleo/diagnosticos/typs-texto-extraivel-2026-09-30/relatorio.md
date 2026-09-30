# Identidade textual dos `.typ` locais — sec_09, sec_10, sec_12, sec_27

Medição final: 2026-09-30, UTC registrado em `results-final.json`, HEAD `17f13b1df79b87ea065ee62e78164b18e477da3d`, working tree não commitada. `working-tree-status.txt` e `working-tree-stat.txt` registram o checkout compartilhado no momento da verificação. `results-final.json` contém hash SHA-256 do binário debug usado, entradas, PDFs, extrações `pdftotext -raw`/`-layout` e comparação raster 120 dpi. O vanilla é o binário pinado em `lab/typst-original/target/release/typst` descrito no relatório original `typs-teste-2026-09-30/relatorio.md`. `results-before-nested.json` preserva a primeira sonda após a correção simples.

## Causas e classificação

- **sec_09, defeito de identidade:** `cases.rs` compunha a chave pelo seletor visual e deixava suas peças (`⎧`, barra, `⎨`, `⎩`) chegarem à extração. O sinal da linguagem é `{`. Um `TextReplacement` no delimitador conserva `{` uma vez, com a mesma geometria e os mesmos filhos visuais. O vanilla omite parte da chave em `-layout`; isso não é obrigação semântica do Tekt. Em `-raw`, ambos preservam a chave, com diferença de quebra de linha do extrator.
- **sec_10, defeito de identidade:** `accent.rs` emitia base e marca combinante separadas, e `underover.rs` expunha as peças do under/overbracket como `_` extra. A base atômica e a marca agora formam uma ocorrência; acentos aninhados usam um envelope externo com marcas em ordem lógica. Suportes elásticos guardam um único `⎵` ou `⎴`. `pdftotext -raw` agora extrai `𝑥̂`, `𝑥̃`, `𝑥̇̇` e um suporte sem peças extras; `-layout` ainda reorganiza linhas de under/overbracket pela geometria, mas não perde nem inventa caracteres.
- **sec_12, defeito de morfologia e identidade:** `apply_math_default` em `math/layout/mod.rs` pulava `MathText` sob `MathAccent`, embora o mesmo símbolo grego tivesse itálico matemático fora do acento. `hat(mu)` agora desenha `𝜇` e a extração é `𝜇̂` em ordem lógica. O vanilla extrai `𝜇` e a marca em linhas distintas. O raster Tekt×vanilla passou de **0,20284%** dos pixels com diferença de canal >20 no PDF inicial para **0,00047%** na sonda final, mesma dimensão 434×493 px a 120 dpi. A diferença residual é de um pixel; não se atribui identidade de bytes ao requisito.
- **sec_27, não defeito de conteúdo:** o raster Tekt×vanilla é idêntico, e `pdftotext -raw` é idêntico. Apenas `pdftotext -layout` insere um espaço antes do expoente final no vanilla. Nenhum espaço foi fabricado no L1 ou no PDF para imitar a heurística do extrator.

## Implementação e verificação

L0 atualizado primeiro nos owners `accent.md`, `cases.md`, `underover.md`, `_comum.md` e `tests.md`. A mudança é correção interna de paridade/identidade; não acrescenta contrato público, modo por defeito, fase ou incompatibilidade (ADR-0127, fluxo contínuo). Cinco testes causais em `math/layout/tests.rs` passaram RED→GREEN: acento simples, dois acentos aninhados, chave de cases, underbracket e `hat(mu)`. A suíte matemática passou **247/247** no checkout compartilhado; `cargo build -q`, `crystalline-lint .` e `git diff --check` saíram com status zero. Os quatro `.typ` compilaram com status zero. Os rasters de sec_09, sec_10 e sec_27 são byte a byte iguais aos PDFs Tekt do relatório inicial; sec_12 mudou para o itálico correto.

`scoped-seals.json` registra os quatro consumers produtivos ressellados. O par compartilhado `math/layout/tests.rs`/`tests.md` será ressellado pela integração principal após as duas frentes terminarem. Nenhum arquivo de seleção ou estiramento de delimitadores de sec_22 foi alterado nesta frente.

## Integração dos testes independentes

A suíte integrada encontrou RED adicional em quatro testes de `p1429_tests.rs`
e `p1430_tests.rs`: os helpers buscavam `Text`/`TextShaped` apenas no nível
superior, ignorando os filhos do novo `TextReplacement`. Os L0 desses dois
módulos foram atualizados antes dos helpers. `p1430` passou **22/22** após a
travessia transparente. Em `p1429`, a travessia revelou outra expectativa
antiga: `dotless=false` procurava `i`/`j` ASCII, mas a base matemática
realizada é `𝑖`/`𝑗` após a correção de `hat(mu)`. As assertivas passaram a
observar o itálico matemático, preservando as comparações de orientação,
tamanho e precedência; `p1429` passou **9/9**. Recibos escopados em
`scoped-test-seals.json`. Nenhum código produtivo ou teste compartilhado
`math/layout/tests.rs` foi alterado nesta integração.
