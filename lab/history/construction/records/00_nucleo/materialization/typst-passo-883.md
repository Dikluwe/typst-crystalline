# Prompt — typst-passo-883: teste de regressão para o embedding CFF1 (P882) — e ver se dá para melhorar mais

**Origem**: P882 corrigiu o embedding de fontes CFF1 (removeu o wrapper OpenType/SFNT desnecessário, ~1.5KB por ocorrência), mas fechou sem nenhum teste automatizado novo cobrindo especificamente esse comportamento — só o rename de um teste existente. Mesma lacuna que já causou uma regressão silenciosa antes neste projeto (P875 quebrando algo que só apareceu no benchmark manual, não numa suíte automática).
**Estado**: aguardando execução — parte 1 é obrigatória (teste de regressão); parte 2 é exploratória (ver se dá pra melhorar mais o tamanho, sem compromisso de fechar tudo num passo só).

---

## Parte 1 — Teste de regressão para o formato de embedding CFF1 (obrigatório)

1. Adicionar um teste automatizado (unitário, em `03_infra/src/export/builder.rs` ou `tests.rs`, junto dos testes já existentes de `p560`/`p772u`) que confirme, sem depender de comparação manual de bytes: uma fonte CFF1 embutida produz stream PDF com assinatura CFF pura (não `OTTO`/SFNT), `/Subtype /CIDFontType0C`, e uma fonte CFF2 continua com o wrapper `/OpenType` (controle de não-regressão para o caso que **deve** manter o wrapper).
2. Se possível, um teste que meça o tamanho do stream gerado para uma fixture de fonte conhecida e trave um limite superior razoável (não exato, mas que capture se o wrapper voltar a aparecer por engano — a diferença de ~1.5KB medida por P882 é grande o suficiente para não ser confundida com ruído de compressão).

## Parte 2 — Ver se dá para melhorar mais (exploratório, sem compromisso de fechar tudo aqui)

P882 já identificou duas frentes seguintes, não corrigidas por decisão de escopo (não por esquecimento):

1. **Compressão FlateDecode do stream de fonte** — o vanilla comprime, o cristalino não. P882 mediu que isso é quase toda a diferença restante em `02-lorem` (8018 bytes cristalino vs 6113 bytes vanilla, mesmo conteúdo CFF). Avaliar o esforço: se o exportador já usa FlateDecode em outros streams do PDF (imagens, por exemplo — confirmar), aplicar o mesmo mecanismo ao stream de fonte pode ser barato. Se for barato, implementar e medir o efeito em `02-lorem`. Se não for trivial, registrar como achado separado com o esforço estimado, não forçar.
2. **Content streams grandes em `06-long`** — P882 identificou que a redução de fonte é irrelevante frente aos ~850KB de content stream (múltiplas páginas, ~19KB de operadores PDF por página). Isso é uma frente bem maior e provavelmente merece um passo de investigação próprio (não uma correção improvisada aqui) — se o tempo permitir, fazer uma sonda rápida (o que compõe esses 19KB por página — operadores redundantes, falta de compressão do content stream em si, formatação verbosa de números) só para saber o tamanho do problema, sem se comprometer a resolver neste passo.

## Passo 3 — Validação

1. Suíte completa, comando + contagem antes/depois, discriminada por crate.
2. Se a compressão FlateDecode do stream de fonte foi implementada (item 2.1): repetir a medição de tamanho de `02-lorem` de P882, confirmando a redução.
3. Se a sonda de content streams (item 2.2) foi feita: relatar o que foi encontrado, mesmo sem implementar correção — vira insumo para um passo futuro dedicado.

## Relatório

`00_nucleo/diagnosticos/typst-passo-883-relatorio.md` com: o teste de regressão novo (Parte 1, obrigatório), e o que foi encontrado/feito na Parte 2 — deixando claro o que foi implementado versus o que só foi sondado e fica registrado para depois. Não é necessário fechar as duas frentes da Parte 2 neste passo; é aceitável fechar só a Parte 1 e reportar a Parte 2 como sondada, se o escopo mostrar que qualquer uma das duas merece passo próprio.
