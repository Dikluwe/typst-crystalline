# Passo 1003 — Auditoria macro: candidatos de corte, cristalino vs vanilla, com DSM Tekt

**Tipo**: Auditoria — catalogar candidatos, não decidir nem cortar. Complementar ao
Passo 1002 (que resolve o nível micro, dentro de um ficheiro já escolhido).
**Âmbito**: nível de módulo/ficheiro — "que hubs existem, quão grandes, quão parecidos ou
diferentes da fronteira vanilla". Não decide onde cortar dentro de cada um — isso é o
método do Passo 1002, aplicado depois, hub a hub.

---

## Aviso de âmbito, já registado no projecto (não repetir o erro de `ADR-0109`)

O `tekt-cargo-dsm` mede arestas ao nível de módulo (`edges(A→B)`). Já foi tentado como gate
de decisão de atomização de elemento em `ADR-0109` e foi **explicitamente rejeitado** para
essa finalidade ("a métrica da lente é irrelevante — não a use como gate"). Este passo usa
o DSM só para **encontrar candidatos** (triagem macro), nunca para decidir sozinho se um
corte específico está certo — essa decisão fina precisa dos 4 critérios do Passo 1002,
aplicados caso a caso.

---

## Fase A — Correr a lente DSM sobre o cristalino

1. Confirmar como invocar `tekt-cargo-dsm` (procurar a sessão/ferramenta —
   `00_nucleo/diagnosticos/typst-dsm.html` já apareceu como ficheiro untracked em relatórios
   recentes; confirmar se é gerado por esta ferramenta e como regenerar).
2. Correr sobre `01_core/src/engine/` — obter a matriz de arestas por módulo (fan-in/fan-out
   por ficheiro).
3. Listar os N ficheiros com maior fan-in (mais módulos dependem deles) e maior fan-out
   (dependem de mais módulos) — candidatos naturais a hub inchado.

## Fase B — Repetir no vanilla, para comparação

1. Correr o mesmo tipo de análise sobre `lab/typst-original/crates/` — se `tekt-cargo-dsm`
   não for aplicável a um projecto externo (Cargo workspace diferente), usar `cargo modules`
   ou equivalente disponível; registar qual ferramenta foi de facto usada, não presumir que
   é a mesma.
2. Produzir a mesma lista (fan-in/fan-out) para o vanilla.

## Fase C — Cruzamento (só listar, não julgar ainda)

Tabela: ficheiro cristalino × fan-in/out cristalino × módulo(s) vanilla equivalente(s) ×
fan-in/out vanilla × nota.

Já temos ponto de partida do Passo 1000 (`eval.md`, `bindings.rs`, `rules.rs` como os hubs
mais carregados no cristalino, com correspondência vanilla mais fragmentada). Este passo
**confirma isso com números da lente**, em vez de ficar só na leitura manual do Passo 1000,
e **estende a todo o `engine/`**, não só a `eval/`.

## Fase D — Candidatos a protótipo seguinte (após `operators.rs`)

Ordenar por: (fan-in alto) + (divergência grande face ao vanilla, i.e., vanilla tem o
equivalente fragmentado em vários ficheiros pequenos, cristalino tem um só grande). Isto dá
a lista de próximos candidatos ao mesmo tratamento do Passo 1002 — não os cortar ainda,
só ordenar por onde o ganho é maior.

## O que este passo NÃO faz

- Não decide onde cortar dentro de nenhum ficheiro (isso é o método do Passo 1002, hub a
  hub, depois deste catálogo existir).
- Não usa a métrica da lente como critério de correcção de um corte específico — só como
  triagem de candidatos, per o aviso de `ADR-0109`.
- Não corrige nada.

## Resultado esperado

Lista ordenada de candidatos a próximo protótipo (depois de `operators.rs` fechar), com
números reais de fan-in/out cristalino vs vanilla — não opinião, não a lista manual que já
temos do Passo 1000, mas a mesma pergunta respondida com a ferramenta que o projecto já
tem para isto.
