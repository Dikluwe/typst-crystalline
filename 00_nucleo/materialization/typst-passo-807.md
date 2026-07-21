# Prompt — typst-passo-807 (achado P798 #11): `pdf::attach` — `#pdf.attach()` rejeitado, decisão de escopo pendente

**Origem**: P798 (lote 3 de triagem em lote, corrigido), tabela "Achados de P798, aguardando passo dedicado"
**Handoff**: `00_nucleo/handoff-novo-chat-p798.md`
**Módulo afectado**: `pdf::attach`
**Estado**: aguardando decisão do dono do projecto — **não implementar sem essa decisão**

---

## Achado (texto exacto do handoff)

> `#pdf.attach()` rejeitado (scope-out explícito) vs vanilla que embute — decisão de escopo pendente

Nota do handoff, secção "Débitos grandes, decisão consciente de não implementar": este achado pode ser o mesmo tipo de decisão de escopo já tomada para SVG (`image::svg`, scope-out desde P772k) e PDF-como-imagem (`#image()` com PDF como fonte, rejeitado por peso de dependência, P781).

---

## Diferença deste prompt em relação aos outros achados de P798

Este não é um passo de correcção directa. `#pdf.attach()` foi **rejeitado deliberadamente** no cristalino (scope-out explícito, não bug de omissão). Antes de qualquer implementação, é preciso decidir se o scope-out se mantém.

## Passo 1 — Sonda (levantar a informação para a decisão)

1. Confirmar o comportamento actual de `#pdf.attach()` nos dois binários: comando exacto usado no achado original de P798 e saída literal de cada um (vanilla embute o anexo no PDF; cristalino rejeita).
2. Localizar no código do vanilla (`lab/typst-original/`) o mecanismo de `pdf.attach` — que estrutura PDF gera (embedded file stream, `/EmbeddedFiles` name tree, etc.) e que dependências usa.
3. Localizar no cristalino o ponto exacto da rejeição (mensagem, condição) e, se existir, a ADR ou nota que registou esse scope-out — procurar em `00_nucleo/adr/` e em `00_nucleo/DEBT.md` por menções a `pdf.attach`, "anexo", "embedded file".
4. Se não existir ADR/DEBT registando o scope-out de `pdf.attach` especificamente, isso é em si um achado a registar no relatório (decisão que existia só implicitamente no código).
5. Estimar o peso de implementação: complexidade da estrutura PDF necessária, se já existe infra reutilizável em `03_infra/src/export/` (o projecto já lida com streams e dicionários PDF noutros contextos), e se há dependência externa nova necessária ou não.

## Passo 2 — Decisão (do dono do projecto, não automática)

Apresentar ao dono do projecto, com base no levantamento do Passo 1:
- Opção A: implementar `pdf.attach()` agora (se a infra PDF já existente cobrir a maior parte do trabalho).
- Opção B: manter o scope-out, mas formalizá-lo com uma entrada em `00_nucleo/DEBT.md` (ou ADR, se o padrão do projecto para esse tipo de decisão pedir ADR — comparar com como SVG e PDF-como-imagem foram registados).

Este passo termina aqui até a decisão ser tomada. Não avançar para implementação sem confirmação explícita.

## Passo 3 — Implementação (só se a Opção A for escolhida)

Implementar `pdf.attach()` replicando a estrutura PDF do vanilla (embedded file stream + name tree), usando a infra de export já existente em `03_infra/src/export/` sempre que possível.

## Passo 4 — Validação (só se a Opção A for escolhida)

1. Recompilar o cristalino.
2. Repetir o comando do Passo 1.1 e mostrar a saída literal — comparar a estrutura do PDF gerado (não só ausência de erro) com o vanilla.
3. Adicionar casos de teste cobrindo `pdf.attach()` com pelo menos um ficheiro anexado.
4. Correr a suíte `typst-core` completa (e a suíte de export/infra, se separada) e mostrar o comando e a contagem de testes antes/depois.

## Passo 5 — Relatório

Se Opção B (manter scope-out): produzir apenas a entrada em `00_nucleo/DEBT.md` formalizando a decisão, com referência ao levantamento do Passo 1.

Se Opção A (implementar): produzir `00_nucleo/materialization/typst-passo-807-relatorio.md` com:
- Comando + saída literal do Passo 1 (antes da correcção).
- Trecho do código vanilla e do código cristalino identificados no Passo 1.
- Diff da implementação.
- Comando + saída literal do Passo 4 (depois da correcção).
- Contagem de testes antes/depois.
