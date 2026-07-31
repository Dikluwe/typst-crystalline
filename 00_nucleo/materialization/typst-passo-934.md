# Passo 934 — reconciliar discrepância de 27× entre a medição de vanilla em P923 e P933

**Precede este passo**: `typst-passo-923-relatorio.md`, Fase E.2 ("vanilla: ~0.3s" para
`05-utf8.typ`, base da razão "25.91×" que abriu toda a frente P925-933) e
`typst-passo-933-relatorio.md`, secção 4.2 ("vanilla: 8156ms" para o mesmo `05-utf8.typ`).

**Este passo existe só para resolver a discrepância. Não avançar nenhuma outra investigação sobre
fallback de fontes até isto estar resolvido** — toda a razão `cristalino/vanilla` medida em
P925-933 depende de um destes dois números estar certo.

**Pré-condição de árvore**: `git status`.

---

## O que precisa de ser confirmado

Dois relatórios do mesmo projecto, medindo (aparentemente) o mesmo ficheiro, com o mesmo binário
vanilla de referência, chegaram a números 27× diferentes:

| Relatório | Ficheiro testado | Tempo do vanilla medido |
|---|---|---|
| P923, Fase E.2 | `05-utf8.typ` | ~0.3s |
| P933, secção 4.2 | `05-utf8.typ` | 8156ms (~8.2s) |

## Fase A — confirmar que os dois testes eram de facto o mesmo cenário

1. Localizar o `05-utf8.typ` exacto usado em cada um dos dois passos — confirmar se é
   **literalmente o mesmo ficheiro** (mesmo conteúdo, mesmo hash) ou se algum dos dois passos
   usou uma versão diferente (por exemplo, um subconjunto menor de caracteres, ou um caminho
   diferente que apontava para outro ficheiro com o mesmo nome). Não presumir que "mesmo nome"
   significa "mesmo conteúdo" — confirmar com `sha256sum` dos dois, se ambos ainda existirem, ou
   reconstruir a partir do conteúdo citado em cada relatório.
2. Confirmar se o binário vanilla usado em cada medição era literalmente o mesmo ficheiro
   executável — os dois relatórios citam `lab/typst-original/target/release/typst`/
   `target-original/release/typst`, mas confirmar se foi recompilado entre os dois passos (P923 é
   de 2026-07-28 sessão anterior, P933 de 2026-07-30 — três dias de intervalo, outros passos
   podem ter mexido no worktree `lab/typst-original/`).
3. Confirmar as condições de ambiente de cada medição: cache de disco do sistema operativo
   (primeira execução depois de reiniciar vs. execuções repetidas com ficheiros já em cache do
   SO), número de fontes instaladas no sistema no momento de cada medição (P930 mediu 1112 faces;
   P933 mediu 2172 — **isto já é uma pista concreta**: se o número de fontes no sistema mudou
   entre os dois passos, o tempo de abrir todas elas muda proporcionalmente, o que explicaria boa
   parte da diferença).

## Fase B — remedir os dois, lado a lado, na mesma sessão

1. Confirmar o número actual de fontes do sistema (`fc-list | wc -l` ou equivalente).
2. Rodar o vanilla real, mesmo `05-utf8.typ` (confirmado idêntico pela Fase A), 10+ vezes,
   registando tempo de relógio e (`strace -c` ou equivalente) número de ficheiros de fonte
   abertos — mesmo método de P933.
3. Comparar o número de fontes abertas nesta remedição com os números já registados: 1112 (P930),
   2172 (P933). Se bater com 2172: a explicação mais provável é que o número de fontes instaladas
   no ambiente de teste **quase dobrou** entre P930 (ou P923, se usou o mesmo ambiente) e P933 —
   confirmar isso directamente, não inferir.
4. Se o número de fontes não explicar a diferença toda: procurar outra causa — verificar se P923
   mediu de facto o vanilla, ou se por engano mediu um binário diferente (por exemplo, um vanilla
   compilado sem optimizações, ou um caminho de execução que não passava pelo fallback real,
   tipo um documento de teste que não continha de facto os caracteres CJK/emoji esperados).

## Resultado esperado

- Causa da discrepância de 27× confirmada com evidência (número de fontes diferente, ficheiro de
  teste diferente, binário diferente, ou condição de cache diferente — não aceitar "não sei" como
  resposta final).
- Número de vanilla correcto e reproduzível estabelecido para `05-utf8.typ`, substituindo os dois
  números conflitantes anteriores.
- Se a causa for número de fontes do sistema ter mudado: registar isso como lição de metodologia
  — medições de performance que dependem do ambiente (fontes instaladas) precisam de fixar/
  registar esse número em cada medição futura desta frente, não só o binário e o ficheiro de
  teste.
- Confirmação (ou correcção) da conclusão de P933 de que "não há nada a copiar do vanilla" — essa
  conclusão só é válida se o número de 8156ms para o vanilla estiver correcto.
