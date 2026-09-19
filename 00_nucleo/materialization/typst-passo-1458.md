# Passo 1458 — eliminar o hardcode bilingue de referências de figure

**Estado:** EXECUÇÃO AUTORIZADA — correção interna de paridade  
**Data da medição:** 2026-09-18  
**Commit medido:** `f6463768d8bb093a59577ff6f3262a9c5f8eb448`  
**Árvore medida:** `1a598261ead2448fd5ec5b4a2381930ed729e3bc`, limpa antes da fixture  
**Regime:** executado sem atestação de isolamento

## 1. Próxima unidade

`references.rs` replica os supplements de figure com `is_pt` e literais para
image/table/raw. O owner linguístico já existe e já é consumido pelas legendas
e pela introspecção; ampliar a duplicação seria novo hardcode.

## 2. Medição anterior à decisão

- L0 SHA-256 anterior: `d52d9d1559c8145ec586394287fde48e74c0c0afe47bab5b224e409198d1dad2`;
- consumer SHA-256: `5f154629f028d3658a4978dcc0cd1a74c30a1b3ef72e90c21077bf7c93f4a118`;
- vanilla ratificado SHA-256: `eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`;
- cristalino release SHA-256: `b24463da898a2fa98669e118954fa5bf622fc762c5cd6c7f1741613d96c7f4ff`.

Na fixture `lab/parity/p1458_figure_ref_de.typ`, o vanilla extrai
`Abbildung 1: Bild` e `Siehe Abbildung 1.`. O cristalino extrai a legenda
correta, mas `Siehe Figure 1.`. A divergência localiza-se na cópia rígida em
`default_supplement_for_key`, não no owner linguístico.

## 3. Obrigação

1. Toda chave `figure:{kind}` consulta `figure_supplement_for_lang`.
2. O fallback legacy de figure consulta o mesmo owner com kind `image`.
3. Remover decisões `is_pt`, matches de kind e literais linguísticos de figure
   deste consumer; não criar tabela sucessora.
4. Preservar supplement explícito, NBSP, número, link, erros e fallbacks do
   owner.

## 4. Prova e ataques

Congelar contrato, oráculos e mutantes antes do candidato. O RED focal cobre
image/de, table/de e raw/it. A fixture bilateral deve provar duas ocorrências
de `Abbildung 1`, em três ordens. Ataques cobrem retenção do ramo atual,
always-English, always-Portuguese, tradução só da legenda, solução só para
image, match duplicado, perda de supplement explícito/NBSP, aceitação de um
único binário e promoção de Unknown.

## 5. Gate ADR-0127

Correção interna de paridade sem mudança de API, default, fase ou quebra de
compatibilidade. L0 atualizado primeiro; fluxo contínuo autorizado.

## 6. Fecho

Executar RED→GREEN, campanha bilateral pública, mutantes, testes do owner,
workspace, build, fmt, `crystalline-lint .` e `git diff --check`. Produto e
certificado ficam em commits separados.

## 7. Execução e resultado

**Estado medido:** working tree não commitado sobre
`f6463768d8bb093a59577ff6f3262a9c5f8eb448`, em
`2026-09-18T22:41:26-03:00`. O L0 foi atualizado antes do candidato. O RED
focal falhou em `image/de`: esperado `Abbildung`, obtido `Figure`.

O candidato importou `figure_supplement_for_lang` e substituiu por uma chamada
ao owner tanto o braço `figure:{kind}` como o fallback legacy `image`. Não
entrou tabela, literal linguístico, match de kind ou pós-processamento novo.
O resselo atualizou apenas os campos mecânicos `@prompt-hash` e
`Hash do Código`.

Resultados reproduzidos:

- GREEN focal: 1/1;
- owner linguístico: 8/8; regressões P1073: 4/4;
- campanha pública nas ordens forward, reverse e forward-repeat: ambos os
  binários compilaram; cada extração contém `Abbildung 1` na legenda e na
  referência, sem `Figure 1`;
- ataques: a primeira execução encontrou um mutante malformado
  (`delegate-image-only` também delegava table/raw). O runner foi corrigido
  depois do preseal para representar o ataque declarado; a nova execução
  matou 10/10, score `1.0`, zero sobreviventes e nenhuma promoção de Unknown;
- `cargo build --release --bin typst`, `cargo build`,
  `cargo fmt --all -- --check`, `crystalline-lint .` e `git diff --check`:
  sucesso, linter com zero violations;
- `cargo test --workspace`: 5916/5924 passaram; permanecem as mesmas oito
  falhas históricas (P1292 `math_vec`, quatro P981/P1132R `math lr`, align
  point e duas contagens HTML P1423/P1168), fora do escopo P1458.

Identidade final do recorte:

- L0: `4d51cc94a6d520c87fec7094b8fb5c9b4cc0c6980f472e6ac41a9d50e6724ba1`;
- consumer: `bd7b1219657b6a01200c3b8ad3902ddc6a29237de198eef91eede2f8590f71c0`;
- fixture: `8890c287edf359d5a6a62fa658ed5a2138b5e7df4fdd53ba6cc1bf5144020825`;
- runner corrigido: `d88c87c49b96ffeea16830fe0e01e67c01216d0b4d2904dc82c73a4490ca36a8`;
- binário release: `02db6b60251198a0dd78e1e09f90fa0a55df8497ee6ca18a94c3f95d50e0d432`.

Não se alega isolamento entre papéis: executado sem atestação de isolamento.
