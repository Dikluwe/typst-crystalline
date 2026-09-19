# Passo 1456 — substituir a reescrita textual de ranges BibTeX

**Estado:** EXECUÇÃO AUTORIZADA — correção interna de paridade
**Data da medição:** 2026-09-18T22:04:00-03:00
**Commit medido:** `f1cfce3d75113960ccd31c8e17bc328fcf6509d6`
**Árvore medida:** `ee983fbe2789321e4477df83d4fec2dde1d4ebc5`, limpa
**Regime:** executado sem atestação de isolamento

## 1. Próxima unidade

P1454 removeu uma substituição textual de cor e P1455 eliminou a paleta
string duplicada. A varredura seguinte encontrou outra transformação produtiva
ad hoc: `bib_csl::bib_entry_to_hayagriva` executa
`pages.replace("--", "-")` antes de entregar o range ao hayagriva.

O objetivo não é eliminar constantes normativas da linguagem, mas substituir
uma reescrita não estrutural que depende de como `str::replace` particiona
runs sobrepostos.

## 2. Medição anterior à decisão

Baseline cristalino:

- L0 `compiler/layout/bib_csl.md` SHA-256
  `796559c807fa4ed68a832eb75d1db808e4dde352b9947b7c729d106295d0b5cd`;
- consumer `compiler/layout/bib_csl.rs` SHA-256
  `e7c113d2c0653c75a951f51cdccba0f7b031264ee0c78dc604a30b99ad027a11`;
- binário release cristalino SHA-256
  `cfbfd03361331ee422c97007d4cf222e265d5922c0a86c2c16ba711b6f541ecc`;
- vanilla ratificado SHA-256
  `eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`.

Um corpus IEEE com `pages = {45---67}` produz `pp. 45–67` no vanilla e
`p. 45––67` no cristalino. A causa é reproduzível: `replace("--", "-")`
consome os dois primeiros hífens e deixa o terceiro, entregando `45--67` ao
parser intermediário. Runs de quatro ou mais hífens têm a mesma classe de
falha. O caso canônico `45--67`, o hífen simples `45-67`, prefixos
`S12--S14` e ranges múltiplos devem permanecer válidos.

## 3. Obrigação

1. Normalizar cada run ASCII de dois ou mais hífens para exatamente um hífen
   antes do YAML intermediário.
2. Preservar hífen simples, caracteres não ASCII, prefixos, separadores,
   whitespace e texto fora do campo `pages` byte a byte.
3. Não usar `replace`, regex, tabela de valores, catálogo de páginas ou
   pós-processamento do documento formatado.
4. Preservar o parser BibTeX, `BibEntry`, o hayagriva, styles CSL, locale e
   todos os demais campos.

## 4. Prova e ataques

Congelar contrato e oráculos antes do candidato. O RED deve exercer um run de
três hífens pelo `BibEntry → hayagriva`. Positivos cobrem zero/um/dois/três/
quatro hífens, range prefixado, múltiplos ranges e Unicode. Mutantes devem
reter `replace`, remover todos os hífens, alterar hífen simples, normalizar
globalmente, truncar após o primeiro range, aceitar somente o resultado de um
binário ou promover `Unknown`. Exigir score `1.0`, três ordens estáveis e zero
promoção de `Unknown`.

## 5. Gate ADR-0127

É correção de fórmula interna e paridade com o vanilla: não altera entidade,
assinatura pública, comportamento padrão, fase do pipeline nem quebra
compatibilidade documentada. O L0 é atualizado primeiro e o passo segue em
fluxo contínuo conforme ADR-0127.

## 6. Fecho

Executar RED→GREEN focal, corpus público bilateral, mutantes, testes do owner,
`cargo build`, `cargo fmt --all -- --check`, `crystalline-lint .` e
`git diff --check`. Produto e certificado ficam em commits separados.

## 7. Execução e resultado

**Estado medido:** working tree não commitado sobre
`f1cfce3d75113960ccd31c8e17bc328fcf6509d6`, em
`2026-09-18T22:12:32-03:00`. Os ficheiros alterados nesta medição são o L0
e consumer `bib_csl`, este passo e os sete artefactos `p1456-*` em
`00_nucleo/diagnosticos/`.

O teste focal foi introduzido antes do candidato e falhou com o valor
intermédio `45--67` em vez de `45-67`. A implementação substituiu a
reescrita textual por uma única varredura que consome cada run maximal de
hífens ASCII, sem tabela de valores. A busca por
`replace\("--",\s*"-"\)` no consumer ficou vazia.

Resultados reproduzidos:

- `cargo test -q -p typst-core p1456_ -- --nocapture`: 2/2;
- `cargo test -q -p typst-core compiler::layout::bib_csl::tests -- --nocapture`:
  34/34;
- corpus público IEEE: vanilla e cristalino produzem `pp. 45–67` para
  `45---67`, e preservam `45--67` e `S12--S14` em paridade;
- o caso múltiplo continua `pp. 1–37–9` no vanilla e `pp. 1–3, 7–9` no
  cristalino, diferença já presente antes do P1456 e fora desta correção;
- `python3 00_nucleo/diagnosticos/p1456-mutation-runner.py`: 12/12 mutantes
  mortos, score `1.0`, zero sobreviventes e sem promoção de `Unknown`;
- `cargo build`, `cargo fmt --all -- --check`, `crystalline-lint .` e
  `git diff --check`: sucesso; o linter manteve zero violations;
- `cargo test --workspace`: 5913/5921 testes passaram. As oito falhas são as
  regressões históricas fora de `bib_csl`: P1292 `math_vec`, quatro casos
  P981/P1132R de delimitadores `math lr`, `cases` com align point e duas
  contagens HTML P1423/P1168.

O resselo obrigatório atualizou somente o campo mecânico `Hash do Código` do
L0 após o candidato. O preseal conserva o hash integral do L0 semântico
anterior ao candidato (`44a3a15a...`); o L0 final tem SHA-256
`b17c8ef1911dbf6a7d71d1833369970094c9dd02308b4c1576b76e86a1fdfbfe`,
o consumer final
`2e14ab8de2a4c617ec68c55578979e48b0fdeb7742a5db3f6ca616649a3c8c74`
e o binário release
`c2022276ae8bb7d2053a80df91b4ccabb32c1fdcd7d4ff9ef916df8749e39b75`.
Não se alega isolamento entre papéis: executado sem atestação de
isolamento.
