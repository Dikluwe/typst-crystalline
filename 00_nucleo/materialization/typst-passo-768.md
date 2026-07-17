---
# P768 — Auditoria sistemática: classificação comportamental (`rules.rs` do vanilla) vs cristalino

> **Passo:** 768
> **Data:** 2026-07-15
> **Foco:** P763h→P767c revelaram que `Content::Shape` foi implementado (Passo 76, numeração antiga) sem nunca confirmar contra `rules.rs` do vanilla se a primitiva era bloco ou inline — a disciplina de sonda obrigatória (ADR-0108/0114) só entrou em vigor depois de uma quantidade substancial de código já estar escrita, e foi usada para **corrigir bugs encontrados**, não para **auditar retroactivamente** decisões de classificação já tomadas sem sonda. Isto pode não ser um caso isolado: `rules.rs` do vanilla não classifica só bloco/inline — também define `Behaviour` (`Weak`, `Ignorable`, `Destructive`, etc.), ordem de `show`, e outras propriedades que afectam como um elemento interage com o que está à sua volta. Este passo audita, elemento a elemento já implementado no cristalino, se a classificação comportamental foi alguma vez confirmada contra o vanilla — não corrige tudo de uma vez, prioriza por risco.
> **Tipo:** Sonda exaustiva + Decisão registada (regra 1) + Correcção condicional para achados de alto risco.
> **Tamanho:** L — pode gerar uma lista grande; a implementação de cada achado fica para passos seguintes, não todos aqui.
> **ADR-0108 EM VIGOR.** **Regra 1 do handoff** — nenhuma correcção sem decisão explícita registada; para achados que exigirem mudança grande (como P767 exigiu), parar e devolver para decisão antes de implementar. **ADR-0107** — distinguir diferença de linguagem (resultado observável diverge) de diferença de implementação aceitável.
> **Dependências:** P767c (linha de trabalho `cetz`/shapes fechada, origem da metodologia). P763h (onde a técnica de ler `rules.rs` directamente foi usada pela primeira vez nesta linha de trabalho).

---

## Parte A — Levantar todas as classificações do vanilla

```bash
grep -n "^const.*_RULE\|ShowFn<" lab/typst-original/crates/typst-layout/src/rules.rs
wc -l lab/typst-original/crates/typst-layout/src/rules.rs
```

Para cada `_RULE` encontrada, extrair:
1. O elemento (`XxxElem`).
2. Se produz `BlockElem::single_layouter`, `InlineElem`, ou outra coisa (`Content` directo, `SequenceElem`, etc.).
3. Qualquer `Behaviour` associado (procurar noutros ficheiros de `typst-library`/`typst-layout` se `rules.rs` não os contiver directamente):

```bash
grep -rn "Behaviour::\|fn behaviour\b" lab/typst-original/crates/typst-library/src/foundations/*.rs lab/typst-original/crates/typst-layout/src/*.rs 2>/dev/null | head -60
```

Produzir uma tabela completa (todos os `_RULE` de `rules.rs`, não uma amostra):

| Elemento vanilla | Classificação (block/inline/outro) | Behaviour |
|---|---|---|
| ... | ... | ... |

## Parte B — Confirmar o estado actual de cada elemento no cristalino

Para cada elemento da tabela da Parte A que já tenha equivalente implementado no cristalino:

```bash
grep -rln "native_<elemento>\|<Elemento>Elem" 01_core/src/rules/stdlib/*.rs 01_core/src/entities/*.rs 2>/dev/null
```

Confirmar, por leitura de código (não pela existência do nome), como esse elemento é tratado hoje: passa por `block::layout`? Fica no fluxo contínuo como `Content::Shape` estava antes de P767c? Tem alguma marcação de comportamento equivalente a `Behaviour`?

Preencher a mesma tabela com uma coluna "cristalino (actual)":

| Elemento vanilla | Classificação vanilla | Cristalino (actual) | Confirmado por sonda antes de implementar? |
|---|---|---|---|
| `LineElem` | Block | (Parte B) | Não — achado em P767 |
| `RectElem` | Block | (Parte B) | Não — achado em P767 |
| `HeadingElem` | ? | ? | ? |
| `ListElem`/`EnumElem` | ? | ? | ? |
| `TableElem` | ? | ? | ? |
| `FigureElem` | ? | ? | ? |
| `QuoteElem` | ? | ? | ? |
| `RawElem` | ? | ? | ? |
| ... (todos os `_RULE` de `rules.rs`) | | | |

A coluna "confirmado por sonda antes de implementar" não é sempre verificável com certeza (regra 9 — não reescrever passos antigos), mas pode ser inferida da mesma forma que P767 fez: ler o prompt/relatório do passo original e confirmar se menciona a classificação comportamental explicitamente.

## Parte C — Priorizar por risco

Não é viável medir todas as combinações para todos os elementos neste passo. Priorizar por:

1. **Elementos que combinam com outros no mesmo fluxo com frequência real** (texto + elemento, como `line`+texto foi o caso de `cetz`) — maior risco de repetir o padrão de P763-P767.
2. **Elementos cuja classificação no cristalino diverge visivelmente da vanilla já na Parte A/B**, sem precisar medir — esses vão direto para a lista de achados confirmados.
3. **Elementos com uso real no corpus do projecto** (mesmo critério já usado em P766 para símbolos) — não investigar a fundo elementos sem nenhum documento de teste ou uso conhecido.

Para os itens de maior risco (topo da priorização), medir uma amostra pequena (2-3 elementos) com o mesmo rigor de P767b (coordenadas via `mutool trace`, não só AE), para validar se a lacuna de facto se traduz em divergência visual real, ou se por acaso o comportamento actual já é suficiente mesmo sem ter sido decidido conscientemente.

---

## Decisão a registar (regra 1)

Com a tabela completa e a amostra medida:

| Achado | Classificação | Decisão |
|---|---|---|
| (por achado) | Divergência confirmada / Sem uso real, scope-out / Já correcto por coincidência | Corrigir agora (passo dedicado) / Backlog priorizado / Não corrigir |

Não implementar nada neste passo além da amostra de validação da Parte C — registar a lista completa e a decisão de prioridade, devolvendo para decisão antes de abrir passos de correcção em massa.

---

## Critério de fecho do passo

- [ ] Todas as `_RULE` de `rules.rs` do vanilla classificadas (não uma amostra).
- [ ] Estado actual do cristalino levantado para cada elemento com equivalente implementado.
- [ ] Coluna "confirmado por sonda antes de implementar" preenchida com a melhor informação disponível (leitura dos prompts/relatórios originais).
- [ ] Priorização por risco aplicada, com justificação.
- [ ] Amostra de 2-3 itens de maior risco medida com coordenadas (`mutool trace`), não só AE.
- [ ] Decisão registada por achado (corrigir agora / backlog / não corrigir), sem implementação em massa neste passo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p768.md`, com a tabela completa.

---

## Próximo passo

Para cada achado marcado "corrigir agora": abrir passo dedicado (numeração própria, fora de P763/P767), com a mesma disciplina de coordenadas usada em P767b/P767c — sonda de classificação já feita aqui, não repetir.
Para achados em backlog: registar em local próprio (ex: `00_nucleo/diagnosticos/achados-classificacao-comportamental.md`), com prioridade, para revisão em ciclo futuro.
