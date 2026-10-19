---
# P618 — Resolver o benchmark completo

> **Passo:** 618
> **Data:** 2026-07-05
> **Foco:** P594 investigou o mesmo problema (benchmark a exceder o tempo limite), confirmou que era lentidão genuína, não um ciclo sem fim, e ajustou o script (`runs=2` para a categoria `macro`, em vez de 5) para caber dentro do tempo. A lista de disparidades ainda lista isto como "incerto", citando passos anteriores a P594, sem reflectir essa correcção. Muito código mudou desde então (P595 a P617). Este passo confirma se a correcção de P594 continua válida, corre o benchmark completo até ao fim, e obtém uma medição actual e completa — a primeira desde P548.
> **Tipo:** Verificação directa.
> **Tamanho:** S–M, dependendo do que a verificação encontrar.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P594 (onde a causa foi separada de ciclo sem fim vs lentidão, e o script ajustado), P595 a P617 (trabalho posterior que pode ter mudado o desempenho, para melhor ou pior).

---

## Verificação

### Confirmar se o ajuste de P594 continua no script

```bash
grep -n "runs = 2 if category" tools/perf/benchmark-p507.py
```

Se a linha já não existir (por exemplo, se o ficheiro foi reescrito ou revertido nalgum passo posterior sem se dar por isso), isso já é um achado — o ajuste de P594 perdeu-se.

### Correr o benchmark completo, com tempo generoso

```bash
time timeout 900 python3 tools/perf/benchmark-p507.py
echo "Exit code: $?"
```

Se terminar dentro do tempo: óptimo, seguir para a análise dos resultados.

Se ainda exceder: confirmar de novo, com o mesmo método de P594 (separar ciclo sem fim de lentidão), porque pode ser um bottleneck novo, introduzido por algum dos passos entre P595 e P617 — não assumir que é o mesmo problema de antes.

```bash
# Se ainda exceder, isolar qual documento está a demorar:
python3 -c "
import subprocess, time
import json
corpus = json.load(open('tools/perf/corpus_list.json'))  # ajustar ao formato real
for doc in corpus:
    start = time.time()
    subprocess.run(['./target/release/typst', doc, '/tmp/bench-out.pdf'], timeout=60)
    print(f'{doc}: {time.time()-start:.2f}s')
"
```

### Critério de fecho da verificação

- [ ] Confirmado se o ajuste de P594 continua no script.
- [ ] Benchmark corrido até ao fim, com tempo generoso.
- [ ] Se ainda exceder: causa nova identificada (ciclo ou lentidão), não assumida como igual à de P594.

---

## Análise dos resultados

Se o benchmark terminar, produzir uma tabela comparativa:

| Documento | Vanilla (ms) | Cristalino (ms) | Rácio | Rácio em P518 (histórico) |
|---|---|---|---|---|
| ... | ... | ... | ... | ... |

Comparar especificamente com os números históricos já registados (P518: mediana 1.10×, macro 0.30×; P546: macro 28.68× antes da correcção; P548: macro 11.98× depois da correcção de cache). Confirmar se o desempenho actual, depois de tudo o que mudou desde então (largura shaped para árabe em P591, tags extra de bookmarks em P606, extracção de colecções de fonte em P609, XMP em P611), se manteve razoável ou regrediu.

### Critério de fecho da análise

- [ ] Tabela completa produzida, todos os documentos do corpus de benchmark.
- [ ] Comparação com os números históricos mais relevantes (P518, P546, P548).
- [ ] Qualquer regressão nova, não explicada por nenhum passo já conhecido, investigada.

---

## Decisão

Se o benchmark correr bem e os números forem razoáveis: este item sai da lista de "incerto" e entra em "confirmado", com a data e os números desta medição.

Se houver uma regressão nova: decidir se é aceitável (com razão) ou se precisa de passo de correcção, com o mesmo método já estabelecido nesta sequência inteira (sonda, causa, correcção).

---

## Critério de fecho do passo

- [ ] Script de benchmark confirmado a funcionar, ou corrigido de novo se necessário.
- [ ] Benchmark completo corrido até ao fim, sem exceder o tempo.
- [ ] Tabela de resultados completa, comparada com o histórico.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p618.md`, com hash do commit.
- [ ] Lista de disparidades actualizada — item do benchmark sai de "incerto".
