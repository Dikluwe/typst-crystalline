---
# P630 — Esclarecer `place.rs` vs `placement.rs` e o uso em `cursor.rs`

> **Passo:** 630
> **Data:** 2026-07-09
> **Foco:** P629 lista `place.rs:44` e `cursor.rs:650` como call-sites actualizados, nenhum dos dois mencionado na análise de P628 (que só falava de `grid.rs`/`placement.rs`, e dizia explicitamente que o fluxo principal estava fora de scope do helper). Antes de continuar a sequência de unificação (P630 original, `boxed.rs`), esclarecer esta discrepância — pode ser só uma omissão da sonda de P628, ou pode ser um caso de uso novo que muda o mapa.
> **Tipo:** Verificação directa.
> **Tamanho:** XS.
> **ADR-0108 EM VIGOR.** Um call-site que aparece na implementação sem ter aparecido na sonda é exactamente o tipo de coisa que a sonda devia ter apanhado — confirmar porquê.

---

## Verificação

### Confirmar se `place.rs` e `placement.rs` são o mesmo ficheiro ou dois diferentes

```bash
ls -la 01_core/src/engine/layout/place.rs 01_core/src/engine/layout/placement.rs 2>&1
```

Se os dois existirem: confirmar a diferença de propósito entre eles (por exemplo, um pode ser o elemento `#place()` em si, outro uma função auxiliar de posicionamento usada por vários sítios).

### Confirmar o uso em `cursor.rs:650`

```bash
sed -n '640,660p' 01_core/src/engine/layout/cursor.rs
```

Confirmar o que está a ser feito ali — é o layout do corpo de uma nota de rodapé (que já tinha sido identificado, noutro contexto, como um sub-layout legítimo, mesmo estando fisicamente dentro de `cursor.rs`), ou é outra coisa que P628 não tinha mapeado?

### Critério de fecho

- [ ] Confirmado se `place.rs` e `placement.rs` são ficheiros diferentes, e qual é a relação entre eles.
- [ ] Confirmado o que `cursor.rs:650` está a fazer, e se é um caso de sub-layout legítimo (por exemplo, corpo de nota de rodapé) ou uma omissão real da sonda de P628.
- [ ] Se for uma omissão real: actualizar a tabela de P628 para incluir este caso, não deixar o mapa incompleto.

---

## Decisão

Se for só uma questão de nomenclatura (dois ficheiros com nomes parecidos, ambos já geridos) ou um caso já coberto pela lógica de "sub-layout mesmo dentro de cursor.rs" (como notas de rodapé): sem acção adicional, só a clarificação registada.

Se for uma omissão genuína da sonda de P628: actualizar o mapa antes de prosseguir com P630 (a migração de `boxed.rs`, já planeada), para não repetir o mesmo tipo de surpresa nesse passo.

---

## Critério de fecho do passo

- [ ] `place.rs` vs `placement.rs` esclarecido.
- [ ] `cursor.rs:650` esclarecido.
- [ ] Mapa de P628 corrigido, se necessário.
- [ ] Relatório curto em `00_nucleo/diagnosticos/paridade-producao-p630-esclarecimento.md`.
- [ ] Só depois disto avançar para a migração de `boxed.rs` (a ser numerada a seguir).
