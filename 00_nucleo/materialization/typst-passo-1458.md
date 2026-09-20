# Passo 1458 — eliminar o hardcode bilingue de referências de figure

> **Nota de preservação (limpeza de 2026-09-19):** este passo é mantido como
> registro histórico. Menções a selos, recibos, certificados, manifests, hashes
> administrativos, papéis de agentes e gates do protocolo revogado não criam
> requisitos vigentes. Os artefatos removidos foram consolidados em
> relatório consolidado da limpeza; o comportamento normativo
> permanece nos Prompts L0 e nos testes funcionais atuais.


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

- L0 identificador administrativo anterior: [dado administrativo removido];
- consumer identificador administrativo: [dado administrativo removido];
- vanilla ratificado identificador administrativo: [dado administrativo removido];
- cristalino release identificador administrativo: [dado administrativo removido].

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

- L0: [dado administrativo removido];
- consumer: [dado administrativo removido];
- fixture: [dado administrativo removido];
- runner corrigido: [dado administrativo removido];
- binário release: [dado administrativo removido].

Não se alega isolamento entre papéis: executado sem atestação de isolamento.
