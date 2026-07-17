---
# P670 — Remedir o benchmark completo desde P657

> **Passo:** 670
> **Data:** 2026-07-10
> **Foco:** A última medição completa e confirmada é de P657 (`macro-10x` a 5,78×). Desde então, P659 (chave de cache), P662/P665 (reversões de sintaxe), e P666-669 (instanciação de fontes variáveis, incluindo chamadas a Python nalguns casos) alteraram código sem que o efeito no desempenho tivesse sido medido. Este passo corre o benchmark completo de novo, dando o estado actual, não o de há treze passos.
> **Tipo:** Verificação directa.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.** Um número de desempenho de um passo anterior não é o estado actual só porque ninguém o contradisse — precisa de ser medido de novo depois de mudanças no código.

---

## Verificação

### Confirmar que o script de benchmark continua a funcionar

```bash
grep -n "runs = 2 if category" tools/perf/benchmark-p507.py
```

### Correr o benchmark completo

```bash
time timeout 900 python3 tools/perf/benchmark-p507.py
```

### Produzir a tabela comparativa

| Documento | Categoria | Vanilla (ms) | Cristalino (ms) | Rácio | Rácio em P618 | Rácio em P657 |
|---|---|---|---|---|---|---|
| ... | ... | ... | ... | ... | ... | ... |

Preencher com todos os documentos do corpus, não só o resumo.

### Repartição de fases do `macro-10x`

```bash
./target/release/typst tools/perf/corpus/macro-10x.typ /tmp/p670-macro.pdf --timings-json /tmp/p670-timings.json
cat /tmp/p670-timings.json
```

Comparar com P619 (repartição original) e P657 (depois da cache de shaping), confirmando se `shape_ms` continua a dominar na mesma proporção, ou se mudou.

### Confirmar se documentos com fontes variáveis entraram no benchmark

Verificar se o corpus de benchmark (`tools/perf/corpus/`) inclui algum documento que use fontes variáveis — se não incluir, os passos P666-669 não estão a ser medidos por este benchmark, e vale a pena considerar adicionar um caso de teste próprio para isso.

```bash
grep -l "weight:\|style:.*italic\|font.*variant" tools/perf/corpus/*.typ 2>/dev/null
```

---

## Decisão

Se o rácio geral se mantiver estável ou melhorar face a P657: confirmar e actualizar o registo histórico.

Se houver regressão: investigar qual dos passos entre P657 e agora a causou, com a mesma disciplina de bissecção já usada em P620.

Se o corpus de benchmark não cobrir fontes variáveis: considerar adicionar um documento de teste específico para isso, dado que P666-669 introduziram um caminho de execução novo (incluindo, nalguns casos, uma chamada a um processo Python) que nunca foi incluído em nenhuma medição de desempenho até agora.

---

## Critério de fecho do passo

- [ ] Benchmark completo corrido até ao fim.
- [ ] Tabela comparativa completa, todos os documentos, com rácios de P618, P657, e agora lado a lado.
- [ ] Repartição de fases do `macro-10x` comparada com P619/P657.
- [ ] Confirmado se o corpus cobre fontes variáveis; se não cobrir, decisão registada sobre adicionar ou não.
- [ ] Qualquer regressão nova investigada, não só registada.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p670.md`, com hash do commit e a tabela completa.
