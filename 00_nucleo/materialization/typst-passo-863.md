# Prompt — typst-passo-863: `#show par: ...` rejeitado como regra de elemento (achado de P861, item 3.1)

**Origem**: item 1 da tabela do Item 3 de P861 — `#show par: it => strong(it)` — vanilla aceita e aplica; cristalino: `error: show par: apenas 'set block(spacing: ..)' é reconhecido`
**Estado**: aguardando execução

---

## Achado (medição de P861)

`#show par: it => strong(it)` seguido de `hello world` — vanilla aceita a show rule e a aplica ao parágrafo inteiro; cristalino rejeita com uma mensagem que sugere que `par` só é reconhecido dentro de `#set`, não como alvo de `#show`.

---

## Regra da linha de trabalho (obrigatória)
Comando exacto + saída literal (vanilla vs cristalino). Contagem de testes de `typst-core` a bater com os testes novos.

## Passo 1 — Sonda

1. Reproduzir o caso exato de P861 nos dois binários, confirmar a mensagem.
2. Testar variações: `#show par: it => it` (identidade, só para confirmar que o mecanismo básico funciona antes de testar transformações), e `#show par.where(...)` se essa forma de seletor fizer sentido para `par` no vanilla.
3. Localizar no cristalino onde `#show` despacha por tipo de elemento (provavelmente `01_core/src/engine/eval/rules.rs`, mesmo arquivo que trata `#set par`) e confirmar por que `par` não está na lista de alvos válidos de `#show`, mesmo já existindo tratamento de `par` para `#set`.
4. Confirmar se essa lacuna é relacionada à ausência de `Content::Par` como elemento de primeira classe (mencionada en passant no relatório de P806, que implementou `par()` como função sem criar um `Content::Par` dedicado, decisão registrada na época) — se for, a implementação de `#show par` pode depender de revisar aquela decisão.

## Passo 2 — Implementação

Depende do que o Passo 1.4 encontrar. Se `#show par` puder ser suportado sem precisar de `Content::Par` como elemento dedicado (interceptando no ponto onde parágrafos são formados/layoutados, mesmo sem um nó de content próprio), implementar assim. Se depender de `Content::Par` existir de verdade, isso é uma mudança maior — registrar como achado que depende de outro (P806) e decidir com o dono se vale reabrir aquela decisão agora ou registrar como scope-out formal até `Content::Par` ser criado por outro motivo.

## Passo 3 — Validação

1. `#show par: it => strong(it)` aplicando a regra, batendo com o vanilla.
2. Confirmar que `#set par(...)` (já existente) continua funcionando sem regressão.
3. Suíte completa, comando + contagem antes/depois, discriminada por crate.

## Relatório

`00_nucleo/diagnosticos/typst-passo-863-relatorio.md` com medição antes, código identificado (incluindo a relação com a decisão de P806, se aplicável), diff (ou decisão de escopo formal se depender de `Content::Par`), medição depois, contagem de testes.
