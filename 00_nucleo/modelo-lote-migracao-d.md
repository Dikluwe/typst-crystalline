# Modelo de Lote — Migração D (ADR-0104/0105)

Artefato reutilizável. Instanciar um lote = preencher os **parâmetros** e
seguir as fases. Gravado no P317 como entregável; usado pelos lotes 3+ sem
retrabalho. Não é L0 (não materializa código): é a *receita* de como migrar
variantes do `Content` para o modelo D (`Nome(Arc<NomeElem>)` + `impl Element`).

Referências de forma: `entities/elements/_comum.md` (o trait `Element`),
`entities/elements/math_styled.md` (precedente de handler math),
`entities/elements/heading.md` (precedente de **locatável**:
`element_kind`/`to_payload`), `entities/content.md` (o hub).

---

## Parâmetros (preencher por lote)

- `N_LOTE` — número do lote (e do passo que o executa).
- `LOTE` — lista de variantes a migrar, **ordenada por largura de uso
  crescente** (medir com Pre-2; a mais barata primeiro).
- `NOTAS_FAMÍLIA` — particularidades da família (ex.: math é structural,
  terminal em `map_text`; locatável segue Heading; etc.).

A **largura de uso** de cada variante (nº de sites de construção/match fora
de `content.rs`) é o dimensionador e o preditor de custo:

```sh
grep -rnE "Content::Nome([^A-Za-z0-9]|$)" 01_core 02_shell 03_infra 04_wiring \
  --include='*.rs' | grep -v "entities/content.rs" | wc -l
```

> **Critério de elegibilidade (achado P317, guideline):** o modelo D só
> compensa naturalmente para variantes **element-shaped** — com corpo(s)/campos
> próprios que justifiquem um struct + `impl Element`. Primitivos de AST (folhas
> como `Text`/`MathIdent`, contentores como `Sequence`/`MathSequence`,
> marcadores unit como `MathAlignPoint`) tendem a não compensar: embrulhá-los em
> `Arc<…Elem>` é overhead e a largura de uso costuma ser enorme (paralela a
> `Text`/`Sequence`, também não migrados). **Default: excluir e registar.**
> **A composição é decisão do dono por lote** e pode em princípio incluir
> primitivos — mas a guideline pesa contra. No P317 o dono **confirmou a
> exclusão**: o Lote 2 migrou as **11** `Math*` element-shaped e diferiu
> `{MathSequence, MathText, MathIdent}` como decisão futura própria (medir
> performance se migrar). Se um lote incluir primitivos, registar a ressalva no
> L0 da variante **e** no relatório, com a medição que a justifique.

---

## Fase A — L0 (redigir e PARAR no checkpoint)

1. **Um prompt fino por variante**: `entities/elements/<nome>.md`.
   Content-preserving do que já existir nos matches do hub; spec nova só
   do desenho do trait (que métodos do `Element` a variante implementa /
   usa default). Precedente: `math_styled.md`.
2. **Atualizar `entities/content.md`**: variante `Nome { … }` →
   `Nome(Arc<NomeElem>)`; braços dos 6 matches → dispatch (`m.metodo()`).
3. **`_comum.md` só muda se o trait mudar — e o trait NÃO muda em lote.**
   Mudança de trait = passo próprio, fora do modelo.
4. **CHECKPOINT humano (Trava Arquitetural).** Apresentar: decisões de
   composição, os L0s redigidos, e o **plano de toque** (a lista de sites do
   grep, por variante). Prosseguir para a Fase B **só com confirmação humana**
   (o humano guarda os L0 e calcula os hashes).

## Fase B — implementação (ordem: largura de uso crescente)

1. **Testes primeiro**: testes unitários do `impl Element` de cada variante,
   no módulo próprio (`#[cfg(test)]`). Confirmar que falham.
2. **Migrar variante a variante** (a mais barata primeiro):
   - `NomeElem` struct no módulo próprio `entities/elements/<nome>.rs`
     (`#[derive(Debug, Clone, PartialEq, Hash)]`), absorvendo os campos.
   - Variante do enum: `Nome(Arc<NomeElem>)`.
   - Construtor ergonómico em `content.rs` (`Content::nome(...)` → embrulha
     `Arc::new(NomeElem { … })`), preservando a assinatura de chamada onde
     der.
   - Os 6 matches do hub viram dispatch (`plain_text`, `is_empty`,
     `map_content`, `map_text`, `get_field`, `PartialEq`).
   - Sites de construção/match externos atualizados para o construtor/Arc.
3. **Locatável** (se a variante for): segue o precedente Heading —
   implementar `element_kind`/`to_payload`; o estado misto dos enums de
   introspecção permanece e esvazia lote a lote.
4. **Linhagem**: header `@prompt`/`@prompt-hash`/`@layer`/`@updated` em cada
   módulo novo; `crystalline-lint --fix-hashes .` após guardar os L0.

## Validação (critérios fixos)

- `cargo build` verde.
- `RUST_MIN_STACK=33554432 cargo test --workspace` verde. A contagem **cresce
  só pelos testes unitários novos**; **nenhuma asserção existente alterada**
  (a sintaxe de construção pode mudar; a asserção não).
- `crystalline-lint .` → **ZERO violations**. Qualquer violação = regressão
  do lote: parar e corrigir antes de seguir (desde o P317 "zero" vale sem
  asterisco).
- Ressalva conhecida da stack (`recursao_infinita_*` com stack default)
  pode reaparecer; **registrar, não é regressão** (correr com `RUST_MIN_STACK`).

## Medições (fecham o lote — métrica ADR-0104)

- **`content.rs`**: linhas antes/depois. Esperado: **encolhe** (o setup do
  trait foi pago no P316). Lote que não encolha o hub exige explicação no
  relatório.
- **Custo-por-variante**: ficheiros/linhas fora do módulo próprio,
  **comparado à previsão da tabela de largura** — o lote valida o preditor.
- **Parte atómica**: linhas dos módulos novos (`elements/<nome>.rs`).

## Relatório (`typst-passo-<N>-relatorio.md` + resumo no chat)

Decisões, medições vs previsão, `content.rs` antes/depois, contagem da suíte,
**proposta do lote seguinte derivada da tabela de largura** (decisão humana),
fora-de-escopo confirmado.

## Regras permanentes

- O trait `Element` **não muda em lote**; quebra de desenho → parar e voltar
  ao L0.
- **Conserto oportunista proibido** (nada fora do escopo do lote).
- Toda medição acompanhada do **comando** que a produz.
- Primitivos de AST não entram **por default**; inclusão só por decisão
  explícita do dono, com a ressalva no L0 da variante e a medição que a
  justifique (ver critério de elegibilidade acima).
- **L0 de variante deferida não fica em `00_nucleo/prompts/`** (geraria V7
  órfão permanente e quebraria o "zero violations"): é anexado à entrada de
  DEBT que regista o deferimento, e volta a `prompts/` no passo que o
  materializar.
- Se a Fase B precisar **editar** um prompt grosso (`rules/eval.md`,
  `rules/parse.md`, `rules/layout.md` ou outro com linhagem larga), o imposto
  morde: **fatiar primeiro** pela receita do P314 (partição content-preserving,
  `_comum.md` por área, `git rm` do prompt velho — a trilha fica no git).
- **Um commit isolável por lote** (pré-tarefas separadas do lote principal).
  Lição medida no diagnóstico P313: o P298 não era isolável no histórico e
  custou à medição.
