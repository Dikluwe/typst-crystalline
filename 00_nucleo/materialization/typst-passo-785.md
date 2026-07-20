---
# P785 — Triagem em lote: 15 módulos restantes de `lacuna-inventario`

> **Passo:** 785 (primeiro lote de uma nova fase de triagem, 15 módulos por vez)
> **Data:** 2026-07-19/20
> **Foco:** Depois da série P765a-P784 (varredura item a item, um módulo por passo), restam ~150 itens em ~60+ módulos de `lacuna-inventario`, majoritariamente classificados a priori como mecânica de parsing/CST/utils (baixa prioridade por ADR-0107). Em vez de continuar um módulo por passo (custo alto por item de baixo volume), este passo faz uma triagem mais rápida em lote — os 15 maiores módulos restantes — para confirmar rapidamente quais são mecânica pura (sem abrir passo dedicado) e quais têm sinal real que justifique investigação própria depois.
> **Tipo:** Sonda de triagem em lote (nível de confiança mais raso que os passos anteriores) + Implementação directa só para achados óbvios e baratos.
> **Tamanho:** L — 15 módulos, mas profundidade menor por módulo que os passos P772a-m/P772f-y.
> **ADR-0108 EM VIGOR** — mesmo em triagem rápida, não classificar como "mecânica" só pelo nome do módulo; abrir pelo menos 1-2 itens de cada módulo para confirmar.
> **ADR-0107** — critério de classificação: resultado observável de um documento Typst diverge (linguagem) vs estrutura interna de implementação sem efeito (mecânica).
> **Dependências:** P772e/P772t (metodologia de reconfirmação e priorização), toda a série P765a-P784 (padrão de classificação item a item, aplicado aqui em profundidade reduzida).

---

## Sonda — identificar os 15 módulos

```bash
awk -F'\t' '$1=="lacuna-inventario"' 00_nucleo/diagnosticos/lente-lista-B-2026-07-15.txt \
  | cut -f5 | sed 's/::[^:]*$//' | sort | uniq -c | sort -rn
```

Excluir todos os módulos já tratados (lista completa: `foundations::calc`, `foundations::ops`, `diag`, `math::style`, `layout::grid::resolve`, `image::raster`, `pdf::accessibility`, `syntax::span`, `syntax::package`, `image::svg`, `foundations::scope`, `text::font::*`, `foundations::target_`, `foundations::plugin_`, `image::pdf`, `layout::frame`, `math` — a série P765a-P784 inteira). Pegar os 15 maiores módulos restantes por contagem de itens.

---

## Metodologia de triagem (mais rápida que os passos anteriores, por módulo)

Para cada um dos 15 módulos:

1. Listar os itens do módulo (`awk`/`grep` no `lente-lista-B`).
2. Ler o nome/assinatura de 1-2 itens representativos no código-fonte do vanilla (não todos, ao contrário dos passos anteriores) — suficiente para confirmar a categoria (mecânica Rust interna vs símbolo/comportamento exposto à língua Typst).
3. Se claramente mecânica (ex: struct/enum interno de parsing, helper de CST, tipo de erro Rust sem exposição): classificar como "mecânica, ADR-0107" e não abrir mais.
4. Se houver qualquer sinal de exposição à língua (função nomeada como a que o utilizador chamaria, comportamento observável no documento): fazer 1 teste rápido real (documento `.typ` mínimo, comparar vanilla vs cristalino) antes de classificar.
5. Registar cada módulo like uma linha da tabela final — não é preciso o detalhe de código completo que os passos P772a-m tinham, mas cada classificação precisa de pelo menos uma linha de evidência (não "parece que sim").

### Achados óbvios e baratos

Se algum item revelar um bug real, simples e isolado (mesmo padrão de "achado incidental" que apareceu várias vezes na série anterior — `.first()` de P772i, `cannot_mutate_constant` de P772l): corrigir directamente neste passo, com teste. Se for maior ou incerto: registar para passo dedicado, não forçar.

---

## Tabela de saída esperada

| Módulo | Itens | Classificação | Evidência (1 linha) | Ação |
|---|---:|---|---|---|
| ... | ... | mecânica / risco confirmado / bug real | ... | nenhuma / passo dedicado / corrigido aqui |

---

## Validação

```bash
cargo test --workspace
crystalline-lint .
```

Para qualquer correcção feita neste passo: teste dedicado, mesma disciplina de sempre.

---

## Critério de fecho do passo

- [x] 15 módulos identificados e triados.
- [x] Cada módulo com pelo menos 1 linha de evidência real (código lido ou teste executado), não suposição pelo nome.
- [x] Bugs óbvios e baratos corrigidos com teste; achados maiores/incertos registados para passo dedicado, não forçados.
- [x] Tabela de saída completa no relatório.
- [x] Contagem: quantos dos 15 módulos tinham sinal real vs mecânica pura — este número é o dado que decide se vale continuar em lotes de 15.
- [x] `cargo test --workspace` verde.
- [x] `crystalline-lint .` zero violações.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p785.md`.

---

## Próximo passo

Com a taxa deste primeiro lote (quantos dos 15 tinham sinal real), decidir: continuar com lotes de 15 (P786, P787...) até cobrir o restante, ou ajustar o tamanho do lote conforme o rendimento observado. Se a taxa for baixa (a maioria mecânica, como esperado pela composição já conhecida da lista restante), considerar reduzir o esforço por lote ainda mais nos próximos.
