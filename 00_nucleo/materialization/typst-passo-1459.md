# Passo 1459 — eliminar o hardcode bilingue de referências de heading

**Estado:** IMPLEMENTADO — certificação segregada no commit subsequente

**Data da medição:** 2026-09-18

**Commit medido:** `c7577a031847ff38e4780044bc4132c1d92a14b9`

**Árvore medida:** `6241b3786d1f29e585bdc6b7f291305391655de5`, limpa antes da fixture

**Regime:** executado sem atestação de isolamento

## 1. Próxima unidade

`references.rs` decide suplemento de heading com `is_pt`; `labelled.rs`
repete o mesmo branch; `introspect/heading.rs` mantém uma tabela mais ampla,
mas presa ao pin anterior e com francês stale. Acrescentar `de` ao branch
seria novo hardcode, não correção arquitetural.

## 2. Medição anterior à decisão

- L0 de references SHA-256 anterior:
  `4d51cc94a6d520c87fec7094b8fb5c9b4cc0c6980f472e6ac41a9d50e6724ba1`;
- consumer references SHA-256:
  `bd7b1219657b6a01200c3b8ad3902ddc6a29237de198eef91eede2f8590f71c0`;
- vanilla ratificado SHA-256:
  `eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`;
- cristalino release SHA-256:
  `02db6b60251198a0dd78e1e09f90fa0a55df8497ee6ca18a94c3f95d50e0d432`.

Na fixture `lab/parity/p1459_reference_supplements_de.typ`, o alvo alemão
referenciado sob francês preserva `Abschnitt` no vanilla e vira `Section` no
cristalino. O alvo francês consultado por query é `Section` no pin vigente,
mas a tabela cristalina retorna `Chapitre`. Alvos espanhóis e italianos também
caem indevidamente em inglês nas referências.

## 3. Obrigação

1. Criar um único owner de fase neutra para as 45 traduções base de heading.
2. Corrigir `fr` para `Section` conforme o pin `586e1bd43`.
3. Snapshot e `Labelled` delegam ao owner, sem tabela/branch sucessor.
4. Referência implícita usa a língua causal transportada pelo alvo; a língua
   no ponto da referência não a substitui.
5. Preservar supplement explícito/vazio, NBSP, número, link, erros e fallback
   defensivo para introspector sem transporte causal.

## 4. Prova e ataques

Congelar contrato, oráculos e mutantes antes do candidato. O RED focal cobre
`de`, `es`, `it` e o controle francês. A campanha bilateral cobre alvo/ref com
línguas cruzadas, forward/backward, equation alemã como controle, override
explícito e vazio. Ataques cobrem branch atual, correção só alemã, língua da
referência/root, tabela francesa stale, correção só no layout, perda de
override/NBSP, dependência total do store, duplicação de tabela, aceite de um
único binário e promoção de Unknown.

## 5. Gate ADR-0127

Correção interna de paridade e ownership, sem API, default, fase ou quebra de
compatibilidade. L0 atualizado primeiro; fluxo contínuo autorizado.

## 6. Fecho

Executar RED→GREEN, owner completo, snapshot francês, campanha pública nas
três ordens, mutantes, testes focais, workspace, build, fmt,
`crystalline-lint .` e `git diff --check`. Produto e certificado ficam em
commits separados.

## 7. Execução

O preseal foi congelado antes do candidato em
`00_nucleo/diagnosticos/p1459-preseal.json`, SHA-256
`068cbed4cc304cf9ce231519ca1fad4e32a0c69abdaf1ef7cea79c6d0202259e`.
Contrato, oráculos e ataques permaneceram imutáveis depois desse ponto.

O primeiro RED focal observou `Section` para alemão, contra `Abschnitt`. O
primeiro candidato centralizou a tabela e fez os testes unitários passarem,
mas a fixture pública continuou vermelha: o braço `Content::Label` passava o
parâmetro de língua da raiz, não a língua da `StyleChain` causal do alvo. O
L0 de `compiler/introspect.rs` foi então atualizado antes do reparo; a emenda
pós-selo conserva essa descoberta sem reescrever o preseal.

O transporte final captura `chain.lang()` no alvo e só recorre ao parâmetro
legacy quando a chain não declara idioma. A suíte integral revelou que
remover esse fallback quebrava um helper sintético de figure em português;
`target_lang.as_ref().or(lang)` restaurou a compatibilidade sem permitir que
o fallback sobrescreva uma língua causal presente.

O owner novo contém as 45 traduções base do pin ratificado e fallback inglês.
Snapshot, transporte labelled e layout delegam a ele. O layout prefere o
prefixo causal já resolvido para o alvo, conserva o número no caminho
numérico e mantém U+00A0 no join público. Supplement explícito, inclusive
vazio, continua soberano.

## 8. Provas finais

Estado medido em `2026-09-19T02:38:11Z`: working tree não commitado derivado
de `c7577a031847ff38e4780044bc4132c1d92a14b9`, árvore
`6241b3786d1f29e585bdc6b7f291305391655de5`. `git diff HEAD --stat` tinha
10 ficheiros tracked, 247 inserções e 86 remoções; `git status --short`
acrescentava os 14 ficheiros novos P1459 listados no manifesto/emenda,
incluindo owner, L0, fixture, passo e dez artefactos diagnósticos.

- binário vanilla: SHA-256
  `eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`;
- binário cristalino release: SHA-256
  `57644d2ef726a8eebc934fe485e0eff1a96e9b1f84e289913928eae827297990`;
- campanha pública bilateral: vanilla e cristalino retornaram sucesso e o
  texto extraído coincidiu byte a byte nas ordens forward, reverse e
  forward-repeat;
- testes focais `cargo test -p typst-core p1459_`: 3 aprovados, 0 falhas;
- suíte `cargo test --workspace --quiet`: 5919 aprovados, 8 falhas, exatamente
  as oito falhas históricas do baseline (P1292, quatro P981/P1132R, align
  point e duas contagens HTML), sem falha P1459;
- ataques estáticos/de modelo: 14/14 rejeitados, score discriminatório 1,0;
  esse número não é apresentado como mutation testing compilado;
- `cargo-mutants 27.1.0`: 3 mutantes gerados, 3 apanhados, 0 sobreviventes,
  score compilado 1,0 no owner focal. Comando reproduzível abaixo.

```sh
cargo mutants --no-config -p typst-core \
  -f '01_core/src/compiler/lang/heading_supplement.rs' \
  -F 'heading_supplement_for_lang' --baseline skip \
  --timeout 180 --build-timeout 300 -j 1 --colors never -- --lib p1459_
```

- `cargo build --workspace`, `cargo fmt --all -- --check` e
  `git diff --check`: sucesso;
- `crystalline-lint .`: exit 0, sem violations e sem drift V5/V15/V26;
- todos os JSON P1459 válidos e o runner estático reproduziu exatamente o
  artefacto de resultados.

## 9. Veredito e limites honestos

**VERIFIED no recorte P1459:** referências ordinárias de heading no compiler
L1 usam um owner canónico, preservam a língua causal transportada do alvo e
mantêm overrides, NBSP, número e link.

Não é alegada paridade causal universal. O probe
`#show heading: set text(lang: "de")` ainda produz `Section` no cristalino e
`Abschnitt` no vanilla porque o carrier de show-set só existe na árvore
renderizada, enquanto P498 introspecta a árvore pré-show. Corrigir isso muda a
fronteira de fase e exige L0 próprio mais confirmação ADR-0127.

Também não é alegada eliminação global de todo literal: o serializer raw de
L2 ainda contém `Content::text("Section")` em `02_shell/src/cli.rs`, dívida já
registada em `shell/cli.md` P1307-R5. Eliminá-la é a próxima unidade e requer
resolver explicitamente o gate público, sem reexportar silenciosamente o
owner interno nem duplicar a tabela.

Toda a execução e revisão deste passo ocorreu **sem atestação de isolamento**.
