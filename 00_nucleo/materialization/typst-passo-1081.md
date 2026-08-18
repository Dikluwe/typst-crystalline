# L0 — Passo 1081: Auditoria de Estado — Lotes 1/2 da Reclassificação N16[α/β/γ]

**Gate**: nenhum — auditoria pura, sem código alterado.

**Base**: P1070 recomendou 3 lotes (`entities/`; `stdlib/`/`eval/`; `introspect/`
`layout/`/`export/`). O Lote 3 foi confirmado executado e fechado no P1080
(36/36 casos). **Não há, nesta conversa, nenhum relatório de execução dos
Lotes 1 ou 2** — preciso de confirmar o estado real antes de presumir que
ficaram pendentes ou que já foram feitos noutro lugar.

---

## 1. O que verificar

Para `entities/` e `stdlib/`/`eval/`: quantos `// neutro:` ainda estão no
formato antigo (prosa livre, sem tag `N16[α/β/γ]`) vs quantos já têm a tag
formal.

```bash
# Total de // neutro: em cada domínio
grep -rc "// neutro:" 01_core/src/entities/ | awk -F: '{s+=$2} END {print s}'
grep -rc "// neutro:" 01_core/src/compiler/stdlib/ 01_core/src/compiler/eval/ \
  | awk -F: '{s+=$2} END {print s}'

# Quantos já têm a tag formal N16[
grep -rc "// neutro: N16\[" 01_core/src/entities/ | awk -F: '{s+=$2} END {print s}'
grep -rc "// neutro: N16\[" 01_core/src/compiler/stdlib/ 01_core/src/compiler/eval/ \
  | awk -F: '{s+=$2} END {print s}'
```

## 2. Reconciliar contra os números do P1070

P1070 estimou "26 casos" em `entities/` e "~30 casos" em `stdlib/`/`eval/`
(números aproximados, o próprio P1070 usa "~" para o segundo). Confirmar os
números reais agora, mesma disciplina de reconciliação já aplicada em todo o
resto desta conversa — não aceitar a estimativa antiga sem verificar.

## 3. Três estados possíveis, tratar cada um diferente

- **Já 100% tageado** — nada a fazer, só actualizar o registo desta conversa
  para reflectir isso (o P1070/P1080 ficariam desactualizados ao sugerir que
  ainda falta).
- **Parcialmide tageado** — alguém já começou (talvez noutra sessão não
  visível aqui); levantar quantos faltam e continuar a partir daí, não do
  zero.
- **0% tageado** — Lotes 1/2 continuam exactamente como o P1070 deixou,
  prontos para execução conforme recomendado (aplicação assistida, já que a
  amostra mostrou serem quase 100% mecânicos).

## 4. Se houver casos parcialmente tageados — verificar consistência

Se algum caso já tiver tag, conferir uma amostra pequena (5-10) contra os
critérios reais do `ADR-0017` (mesmos usados no P1070/P1080) — não presumir
que quem tageou usou o critério certo só porque a tag está no formato correcto
sintacticamente.

## Critério de conclusão

- Números reais de `entities/` e `stdlib/`/`eval/` obtidos, não estimados.
- Estado classificado (§3) para cada um dos dois domínios, podendo ser
  diferente entre eles.
- Se houver tags já aplicadas, amostra de consistência conferida (§4).
- Nenhuma anotação nova aplicada neste passo — isto é só levantamento, a
  execução (se necessária) é passo seguinte.
