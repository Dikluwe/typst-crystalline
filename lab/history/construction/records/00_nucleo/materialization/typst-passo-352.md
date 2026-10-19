# Passo 352 — F-realização: caso 3 (show-set), `Transformation::Style`

> **O que faz.** Implementa o **caso 3 (show-set)** — `#show heading: set text(...)` — que
> **hoje erra** no cristalino. É a fatia que a **Fase A do P352 mediu como segura, bounded e
> exigida pela fonte**, e **inverte a válvula original** (o passo propunha caso 1 primeiro; a
> medição mostrou que caso 1 colide com um limite duro e caso 3 é o seguro). **Transporte
> (Estágio 0) DISPENSADO** — reutiliza o `Content::Styled` da fatia 1 (P339); sem infra nova
> (ADR-0107). É um **arm aditivo**: **não toca** o loop α / caso 2 (recursão, fechado), o caso
> 4 (escopo, P340), o `morph_canon`/`==` (P345), a flag P350c, o DEBT-59 nem o Marco G. **Caso
> 1 (composição) NÃO entra aqui** — é decisão de desenho separada (colide com o α). Edita o L0
> e **PARA na Trava** para o hash do dono antes de qualquer código.

**Repositório de trabalho**: typst-crystalline (raiz), branch `tekt`.
**Número do passo**: P352 (a Fase A já rodou neste número e não escreveu código; este é o
escopo refinado pela medição).
**Pré-condição**: P352 Fase A fechada (relatório `typst-passo-352-relatorio.md` commitado; a
medição com `file:line` da semântica do vanilla, do estado atual do cristalino, e da
recontagem de testes). HEAD `138e0b0c2` (pós-P351), árvore limpa, lint **0/0**, suíte
**2729** (`typst-core --lib`). Caveat de stack: `RUST_MIN_STACK=33554432`. Se algo não bater,
parar e reportar.
**Tipo**: F-realização, caso 3 (show-set) — **aditivo**. O show-set que hoje **erra** passa a
renderizar (paridade com vanilla). **Zero asserção existente alterada** (há **0** testes de
show-set hoje — o caminho erra); os testes deste lote são **novos**.

---

## Base da Fase A (já medida — `typst-passo-352-relatorio.md`)

A medição que governa este escopo, para não re-herdar suposição:

- **Vanilla (oráculo)**: `lab/.../styles.rs:531-537` + `lib.rs:458-464` —
  `Transformation::Style(t) => { if !prepared { map.apply(t) } continue; }`: empurra os styles
  para a chain e **não consome o passe**; o elemento renderiza sob a chain aumentada
  (`styles.chain(&map)`, `lib.rs:358`). Show-set é **estilo-na-subárvore**, não transformação
  de conteúdo.
- **Cristalino atual**: `#show heading: set text(bold: true)` → **ERRO** ("show rule com
  selector de tipo requer função ou Content, recebeu none"). Causa (`rules/eval/rules.rs:586-668`
  + `mod.rs:524`): `eval_set_rule` **muta `engine.styles` globalmente na declaração** (escopo
  errado) e devolve `Value::None`; a `ShowRule` nasce com `transform: Value::None`
  (`show.rs:49`) e o arm `other =>` (`rules.rs:178-186`) erra. **Não há `Transformation` no
  cristalino.**
- **Transporte**: o `Content::Styled` + `Styles::push_custom` da fatia 1 (P339,
  `style.rs:225`, `style_chain.rs:175`) é o carregador; o caso 3 o reutiliza. **Estágio 0
  dispensado** (medido).
- **Testes de show-set hoje**: **0** (recontagem do P352 Fase A; não herdada do P337).

---

## Limites duros

- **Não tocar o loop α / caso 2 (recursão), o caso 4 (escopo, P340), o `morph_canon`/`==`
  (P345), a flag P350c, o DEBT-59 nem o Marco G** (`edges content→elements` = 66). O caso 3 é
  aditivo e não passa por nenhum deles.
- **Não tocar a ordem de composição (caso 1).** O conserto de ordem (inverter a iteração de
  `node_rules`) é parcial e mascara o buraco do caso canônico — fica fora deste lote, junto
  com o resto do caso 1.
- **Não mutar `engine.styles` globalmente na declaração.** O `StyleDelta` do set é **capturado**
  e carregado pelo `Content::Styled`, com escopo léxico — é o conserto do bug de escopo medido.
- **L0 primeiro, com Trava.** A edição do L0 acontece **antes** do código e **para** para o
  hash do dono (CLAUDE.md). Sem código de produto até a aprovação.

---

## Estágios

### Estágio L0 — desenho + Trava (PARA aqui para o dono)
Editar, sincronizar hashes, e **parar** para revisão/aprovação do dono antes de qualquer `.rs`:

1. **`entities/show.md`** (governa `ShowRule`/`Selector`): `ShowRule.transform` deixa de ser
   `Value` e passa a carregar uma `Transformation` com a variante `Style` (espelhando o
   `Transformation = Content | Func | Style` do vanilla, S5 do `f_fronteira_e1.md §3c`).
2. **`entities/f_fronteira_e1.md §3c`** (S5): registrar que o show-set é `Transformation::Style`,
   **não consome passe**, e carrega o `StyleDelta` via `Content::Styled` (o transporte da fatia
   1). Sincronizar o hash.

**TRAVA**: o passo termina aqui no chat — L0 + hashes para o dono aprovar. Nenhum código antes.

### Estágio 1 — captura do set sem mutação global (após aprovação)
`eval_show_rule` detecta o transform `Expr::SetRule` e **captura o `StyleDelta`** desse set
**sem mutar `engine.styles`**; a `ShowRule` nasce com `Transformation::Style(delta)` em vez de
`Value::None`. Corrige o bug de escopo medido.

### Estágio 2 — o arm aditivo em `apply_show_rules`
Quando uma show-set casa o elemento: **embrulhar em `Content::Styled(elem, styles)`** (o
carregador da fatia 1) e **`continue`** — **não consome passe** —, espelhando o `map.apply`
do vanilla (`lib.rs:458-464`). O elemento renderiza sob a chain aumentada.

### Estágio Teste
- **Novo**: `#show heading: set text(bold: true)` sobre `= titulo` renderiza sob a chain
  aumentada, casando com o vanilla (antes: **ERRO**).
- **Novo**: a show-set **confina** à subárvore que embrulha (escopo léxico — não vaza para o
  irmão), igual ao caso 4.
- Confirmar por construção/teste que o loop α / caso 2, o caso 4, o `morph_canon`/`==` e a
  flag P350c **ficaram intactos** (o caso 3 não passa por eles).

### Estágio F — linhagem
`@updated`; `--fix-hashes`; V7 limpa.

---

## Verificação (gates)

```
build: limpo por estágio.
suíte (RUST_MIN_STACK=33554432): 2729 + os testes NOVOS de show-set. ZERO asserção existente
  alterada (não havia teste de show-set; o caminho errava).
lint: crystalline-lint . = 0/0.

ACEITAÇÃO (observável, contra a harness de paridade — o oráculo):
  - `#show heading: set text(bold: true)` sobre `= titulo`: renderiza sob a chain aumentada,
    igual ao vanilla (antes: erro).
  - a show-set confina à subárvore (não vaza ao irmão).

INTACTOS (confirmar): loop α / caso 2 (recursão), caso 4 (escopo P340), morph ==/morph_canon
  (P345), flag P350c, DEBT-59, Marco G (edges content→elements = 66).
Trava ADR-0105 cl.3: presente para o caminho dinâmico de show (já construída em F-1/F-2/F-3).

lente (critério 3): edges(content→elements::*) = 66 INALTERADO; edges(elemento→elemento) = 0;
  par --comparar antes/depois com o delta na camada que o show-set toca.
perf (critério 4): antes = 0.6518 s ± 0.0057 (P330); reportar o depois (≥10 execuções).
L0 (critério 5): show.md + f_fronteira_e1.md §3c editados e hashes sincronizados ANTES do
  código, com a Trava cumprida (aprovação do dono registrada).
```

---

## O que NÃO fazer

- **Não implementar o caso 1 (composição).** Colide com o limite duro do α (medido na Fase A);
  é decisão de desenho separada (L0/ADR que reconcilie composição com α, ou divergência
  declarada). Sem o desenho, sem código.
- **Não fazer o conserto parcial de ordem** (inverter `node_rules`) — mascara o caso canônico.
- **Não tocar o loop α, o caso 4, o `morph_canon`/`==`, a flag P350c, o DEBT-59 nem o Marco G.**
- **Não mutar `engine.styles` globalmente.** Capturar o `StyleDelta` e carregá-lo escopado.
- **Não pular a Trava.** L0 + hash do dono antes de qualquer `.rs`.
- **Não importar a quarentena.** `lab/` é leitura de semântica, nunca import.

---

## Relatório (`typst-passo-352b-relatorio.md` + resumo no chat)

O L0 editado (show.md + f_fronteira_e1.md §3c) com hashes e a aprovação da Trava; os dois
estágios de código (captura sem mutação global; o arm aditivo); os testes novos de show-set
com a justificativa de paridade; a prova de que loop α / caso 2 / caso 4 / morph `==` / flag
ficaram intactos; os números da lente (edges 66 inalterado, par `--comparar`) e da perf
(antes/depois); `git status` limpo por estágio fora de `lab/` e docs; lint 0/0; o caveat de
stack.

## Fora de escopo (confirmado)

**Caso 1 (composição)** — decisão de desenho separada (reconciliar multi-apply innermost-first
com o α, ou divergência declarada); o conserto parcial de ordem; F-5 (de-bake); F-6 (3 folhas);
Marco G; exposição da flag na CLI (DEBT-59); qualquer mudança no loop α / `morph_canon` / `==`
ou na flag de diagnóstico.
