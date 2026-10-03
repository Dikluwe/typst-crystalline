# Grid: limitar o cache de largura a conteúdo já compartilhado

## Problema e variante

Na [medição anterior](grid-prototypes-2026-10-03.md), o cache estrutural de
largura copiou as palavras para criar uma chave e ficou 28,4% mais lento em
320 células repetidas e 74,5% mais lento em 320 células únicas. A largura
intrínseca do modelo é barata: máximo e soma de pequenas listas de palavras.

O protótipo agora usa `Arc<[u16]>` para a lista de palavras. A fixture repetida
**já compartilha** a mesma alocação entre células equivalentes; a fixture
única não compartilha. O cache é consultado somente quando `Arc::strong_count`
indica compartilhamento. A chave é endereço da alocação, comprimento e escala
da fonte. Assim, criar e comparar a chave não percorre as palavras; células
com conteúdo igual em alocações distintas calculam a largura normalmente.
Todos os objetos da fixture vivem durante `run`, então o endereço não pode ser
reciclado dentro da medição. O teste adicional cobre conteúdo estruturalmente
igual, porém não compartilhado, e escala diferente sobre a mesma alocação.

Antes dessa variante, testei chave estrutural emprestada e `FxHashMap`. Isso
reduziu a regressão, mas o custo de percorrer a chave ainda superou a conta
intrínseca nos textos curtos, e células únicas continuaram pagando pela
consulta. Esses resultados foram exploratórios; a decisão abaixo usa a
variante final e seus dados brutos.

## Medição e proveniência

Em 2026-10-03, aproximadamente 02:21–02:22 UTC, HEAD
`169578af7d117720230b2ba6fed1d7c74f25c3eb`, **working tree não
commitado**. Os arquivos modificados no momento da medição eram
`lab/grid-prototypes/Cargo.lock`, `lab/grid-prototypes/Cargo.toml`,
`lab/grid-prototypes/README.md`, `lab/grid-prototypes/src/lib.rs` e
`lab/grid-prototypes/src/measure.rs`. O `git diff HEAD --stat` era:

```text
 lab/grid-prototypes/Cargo.lock     |  9 ++++
 lab/grid-prototypes/Cargo.toml     |  3 ++
 lab/grid-prototypes/README.md      | 17 ++++++--
 lab/grid-prototypes/src/lib.rs     | 85 ++++++++++++++++++++++++++++++--------
 lab/grid-prototypes/src/measure.rs | 16 ++++---
 5 files changed, 104 insertions(+), 26 deletions(-)
```

Binário de medição SHA-256
`b558b03e336cc0d15c0e7ae1e7f5e4c2a178d86092bae7174f566a7477bc4464`.
Compilação: `cargo build --release --offline --manifest-path
lab/grid-prototypes/Cargo.toml --bin grid-prototypes-measure`. Execução:
`taskset -c 0` com 80 linhas, 31 blocos e 200 iterações por bloco, modos
intercalados. `--word-repeat 16` multiplica as palavras da célula; a
construção da fixture permanece fora do cronômetro. Os tempos abrangem `run`
inteiro. Os 16 modos produziram saída igual à do baseline **dentro do modelo**.

| Conteúdo | Baseline | `width` por identidade | Diferença | Acertos |
| --- | ---: | ---: | ---: | ---: |
| Curto, compartilhado | 26,073 µs | 26,052 µs | −0,1% | 158/160 |
| Curto, único | 34,285 µs | 34,305 µs | +0,1% | 0/160 |
| Longo, compartilhado | 149,852 µs | 148,891 µs | −0,6% | 158/160 |
| Longo, único | 205,479 µs | 205,632 µs | +0,1% | 0/160 |

Com `reuse+height+window` no texto longo compartilhado, acrescentar `width`
reduziu a mediana de 87,406 para 85,361 µs (−2,3%). Os percentis 10/90 dos
blocos foram 87,079–87,706 µs sem `width` e 85,014–85,617 µs com ele. Para
conteúdo curto, as diferenças são pequenas perante a variação dos blocos.

Dados completos: [curto compartilhado](grid-width-cache-2026-10-03/grid-width-final-short-repeated.csv),
[curto único](grid-width-cache-2026-10-03/grid-width-final-short-unique.csv),
[longo compartilhado](grid-width-cache-2026-10-03/grid-width-final-long-repeated.csv) e
[longo único](grid-width-cache-2026-10-03/grid-width-final-long-unique.csv).

## Decisão

A consulta por identidade **evitou a regressão grande do modelo**, mas o
ganho isolado é pequeno. A fixture repetida constrói o compartilhamento antes
do cronômetro. Não medimos o custo de criar compartilhamento em Tekt, nem
demonstramos que células repetidas reais apontem para o mesmo objeto. Se não
apontarem, esta variante quase sempre contornará o cache e não trará ganho.
Também não há alteração ou ganho comprovado no L1; o binário de produção
permanece o mesmo da medição anterior. O próximo gate para `width` é medir
identidade compartilhada no caminho real e então comparar custo total, incluindo
qualquer criação dessa identidade, com os caches já existentes.
