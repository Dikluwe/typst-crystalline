---
# P619 — Completar a repartição de fases do `macro-10x`

> **Passo:** 619
> **Data:** 2026-07-05
> **Foco:** P618 apresentou uma tabela de fases do `macro-10x` sem `shape_ms` nem `subset_ms`, chamando aos 97,1% do tempo restante "ainda não isolado em fases instrumentadas". Isto já estava isolado desde P546, que mediu `shape_ms` como 89,6% do tempo total no mesmo documento. Este passo repete a medição de fases com o método já usado em P546/P548 (`--timings-json` completo), para confirmar se `shape_ms` continua a dominar, e se cresceu ou diminuiu desde então.
> **Tipo:** Verificação directa.
> **Tamanho:** XS.
> **ADR-0108 EM VIGOR.** Uma pergunta já respondida antes não fica "por instrumentar" só porque um relatório novo não a repetiu.

---

## Verificação

```bash
./target/release/typst tools/perf/corpus/macro-10x.typ /tmp/p619-macro.pdf --timings-json /tmp/p619-timings.json
cat /tmp/p619-timings.json
```

Confirmar todas as fases, incluindo `shape_ms` e `subset_ms`, tal como P546/P548 já mediram.

### Comparar com o histórico

| Fase | P546 (antes da correcção) | P548 (depois da correcção) | P619 (agora) |
|---|---|---|---|
| `shape_ms` | 30 564,7 ms (89,6%) | ~57 000 ms (dominante, per P548 secção 3.2) | ? |
| `layout_ms` | 1 380,8 ms | 1 484,9 ms | ? |
| `subset_ms` | 1,0 ms | ? | ? |
| `total_ms` | 34 096,0 ms | ~60 492 ms | 35 958,1 ms (de P618) |

Preencher a coluna P619, e confirmar se `shape_ms` continua a explicar a maior parte do tempo, ou se a proporção mudou — por exemplo, com a introdução de `advance_shaped` para árabe em P591, ou com o trabalho de P609 (extracção de faces de colecções de fonte), o perfil de custo pode ter mudado.

### Critério de fecho

- [ ] Repartição completa de fases obtida, incluindo `shape_ms` e `subset_ms`.
- [ ] Comparação com P546/P548 feita, com números lado a lado.
- [ ] Se `shape_ms` continuar a dominar: confirmar se a proporção é semelhante, maior, ou menor do que antes.
- [ ] Se a dominância mudou para outra fase: investigar essa mudança, não assumir que é a mesma causa de sempre.

---

## Decisão

Se `shape_ms` continuar a ser a fase dominante, sem crescimento anómalo desde P548: registar isso, e o item do benchmark completo fica confirmado com a repartição completa, não com uma lacuna.

Se `shape_ms` tiver crescido significativamente desde P548 (por exemplo, por causa do trabalho de P591 em `advance_shaped` para árabe, que passou a chamar o shaper durante a decisão de layout, não só no final): decidir se isso é esperado (o trabalho extra tem uma razão) ou se precisa de outra optimização, seguindo o mesmo padrão de cache já estabelecido em P548.

---

## Critério de fecho do passo

- [ ] Repartição de fases completa, com `shape_ms` e `subset_ms` incluídos.
- [ ] Comparação com P546/P548, número a número.
- [ ] Decisão registada.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p619.md`, com hash do commit.
- [ ] Actualizar a tabela de fases de P618, que ficou incompleta.
