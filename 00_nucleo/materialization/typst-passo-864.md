# Prompt — typst-passo-864: listas separadas por parágrafo se fundem numa só (achado de P861, item 3.2)

**Origem**: item 2 da tabela do Item 3 de P861 — duas listas separadas por uma linha em branco (`- first` / linha vazia / `- second`) — vanilla renderiza como duas listas distintas, com espaçamento entre elas; cristalino renderiza como uma única lista compacta
**Estado**: aguardando execução

---

## Achado (medição de P861)

```
- first

- second
```

Vanilla: duas listas separadas (o `ParbreakElem` entre elas quebra o agrupamento — mecanismo de `LIST` grouping rule do vanilla, que só agrupa `ListItem` consecutivos com `SpaceElem` entre eles, não com `ParbreakElem`, conforme já documentado por P859). Cristalino: uma lista única, compacta — sugere que o cristalino não distingue quebra de parágrafo de espaço simples entre itens de lista.

---

## Regra da linha de trabalho (obrigatória)
Comando exacto + saída literal (vanilla vs cristalino). Contagem de testes de `typst-core` a bater com os testes novos.

## Passo 1 — Sonda

1. Reproduzir o caso exato de P861 nos dois binários — medir geometricamente (não só contar quantas listas aparecem) o espaçamento entre os dois grupos no vanilla, para saber o que replicar.
2. Testar o caso de controle: itens de lista separados só por quebra de linha simples (sem linha em branco) — confirmar que isso continua formando uma lista única nos dois binários (não regredir o caso comum).
3. Localizar no cristalino onde a lista é formada a partir dos itens (`ListItem` consecutivos) e confirmar se a distinção entre `Space`/quebra de linha simples e `Parbreak`/linha em branco está disponível nesse ponto, ou se ela se perde antes (possível relação com o achado #41/P862 — texto fundido pode ter apagado a distinção entre tipos de separador; confirmar se há relação antes de assumir que são a mesma causa).

## Passo 2 — Implementação

Ajustar a formação de lista para respeitar `Parbreak` como quebra de agrupamento, replicando a regra `LIST` do vanilla (consecutivos com espaço simples agrupam; com parbreak, não).

## Passo 3 — Validação

1. Recompilar. O caso do achado produzindo duas listas separadas, com espaçamento batendo com o vanilla.
2. O caso de controle (sem linha em branco) continuando a formar lista única.
3. Testar o mesmo padrão para `enum` e `terms`, já que o vanilla tem regras de grouping análogas para os três (`LIST`, `ENUM`, `TERMS`, conforme P859) — confirmar se a mesma lacuna existe nos três ou só em `list`.
4. Suíte completa, comando + contagem antes/depois, discriminada por crate.

## Relatório

`00_nucleo/diagnosticos/typst-passo-864-relatorio.md` com medição antes, código identificado, diff, medição depois (incluindo `enum`/`terms` se aplicável), contagem de testes.
