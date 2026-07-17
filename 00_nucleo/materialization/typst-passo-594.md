---
# P594 — O benchmark excede o tempo por ser grande, ou por não convergir?

> **Passo:** 594
> **Data:** 2026-07-05
> **Foco:** O benchmark completo (`tools/perf/benchmark-p507.py`) excede o tempo limite desde P546, repetido em P563, P565, e P593 — sempre aceite como "é grande, demora", nunca confirmado se é isso ou se ficou preso num ciclo sem fim. As mudanças recentes na cascata de largura (P591 shaping com cache, P565/P567 reflow de parágrafos RTL, P592 recálculo de posições) são exactamente o tipo de código onde um ciclo sem terminar pode aparecer — por exemplo, se a lógica de "juntar linhas se couberem" nunca conseguir decidir que já não cabe mais nenhuma, ou se o cache tiver uma dependência circular. Este passo separa as duas hipóteses antes de continuar a aceitar o tempo limite como normal.
> **Tipo:** Verificação directa.
> **Tamanho:** S.
> **ADR-0108 EM VIGOR.** Um processo que excede o tempo limite quatro vezes seguidas não se aceita como "normal" sem se confirmar que termina, só devagar.

---

## Verificação

### Confirmar se o processo avança, ou está parado

```bash
timeout 60 python3 tools/perf/benchmark-p507.py &
PID=$!
sleep 10
ps -o pid,pcpu,pmem,etime,cmd -p $PID
sleep 20
ps -o pid,pcpu,pmem,etime,cmd -p $PID
wait $PID
```

Se o uso de CPU ficar constante e alto (perto de 100%) ao longo do tempo, sem o processo terminar, isso não distingue por si só entre "muito trabalho" e "ciclo sem fim" — mas se ficar preso exactamente no mesmo ponto (mesmo documento a compilar, sem avançar para o seguinte), é sinal de ciclo.

### Isolar o documento `macro-10x` sozinho, com tempo limite curto e trace

```bash
timeout 30 ./target/release/typst lab/parity/corpus/macro-10x.typ /tmp/p594-macro.pdf
echo "Exit code: $?"
```

Um `exit code` de 124 (do comando `timeout`) confirma que não terminou dentro do tempo dado. Repetir com um tempo maior (por exemplo, 120 segundos) para confirmar se termina eventualmente, ou nunca:

```bash
time timeout 120 ./target/release/typst lab/parity/corpus/macro-10x.typ /tmp/p594-macro.pdf
echo "Exit code: $?"
```

### Se ainda não terminar, instrumentar o ponto mais provável de ciclo

Dado que P565/P567 introduziram `reflow_rtl_paragraphs` (fundir linhas RTL consecutivas quando cabem) e P591 introduziu cache de shaping:

```bash
grep -n "loop\|while\|fn reflow_rtl_paragraphs\|fn try_fuse_paragraph" 03_infra/src/layout_bidi.rs
```

Confirmar se existe algum ciclo (`while`, `loop`) nesta função sem uma condição de saída clara, ou sem um limite de iterações. Se existir, adicionar temporariamente uma contagem de iterações com `eprintln!`, e correr o documento `macro-10x` de novo com tempo limite curto, para ver se a contagem cresce sem parar.

### Critério de fecho

- [ ] Confirmado se o processo avança (documentos diferentes a compilar ao longo do tempo) ou fica preso no mesmo ponto.
- [ ] `macro-10x` isolado, testado com tempo limite curto e depois longo.
- [ ] Se não terminar mesmo com tempo longo (por exemplo, 120 segundos, para um documento que devia demorar segundos): confirmado como ciclo sem fim, não só lentidão.
- [ ] Se terminar, mas devagar: confirmado que é lentidão genuína, com o tempo real registado, não só "excedeu o limite".
- [ ] Se for ciclo: localizado com `file:line`, seguindo o mesmo método já usado nesta sequência inteira.

---

## Decisão

Se for ciclo sem fim: corrigir com prioridade alta — isto pode estar a acontecer também em produção, não só no benchmark, para qualquer documento com a combinação de condições que dispara o ciclo.

Se for só lentidão: medir o tempo real (mesmo que grande), decidir se é aceitável, e ajustar o tempo limite do script de benchmark para um valor que permita medir até ao fim, em vez de desistir a meio.

---

## Critério de fecho do passo

- [ ] Confirmado com prova: ciclo sem fim, ou lentidão genuína.
- [ ] Se ciclo: causa localizada com `file:line`.
- [ ] Se lentidão: tempo real medido, registado, e decisão sobre o tempo limite do script.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p594.md`, com hash do commit.
