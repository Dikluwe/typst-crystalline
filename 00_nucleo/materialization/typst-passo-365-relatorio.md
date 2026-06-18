# Relatório P365 — F-5a de-bake: figura (fonte única) + colapso da assinatura partilhada

> **Desfecho.** O 3º caminho duplo (figura) está **fechado**: o campo assado
> `FigureElem.numbering` foi **removido**; o padrão de numeração vive **só na chain**
> (`custom("figure.numbering")`, transportado por `Content::Styled`). E a **assinatura
> partilhada `figure_numbering: Option<&str>`** — o último parâmetro do tipo `NativeFn`,
> **141 ocorrências em 16 ficheiros**, morta após o de-bake — foi **colapsada** do ABI no
> mesmo lote (escopo aprovado na Trava). **Content-preserving**: a rede de caracterização
> (+11) e a suíte inteira passam (**2738**, 0 falhas). lint **0/0**. Estado final limpo, sem
> parâmetro morto interino.

**HEAD**: pós-P363+364 (d87258796). **Branch**: Tekt. **Justificativa = princípio (fonte
única / atomização)**, decidido pelo dono (P362). **Escopo aprovado na Trava**: tudo no P365
(campo + assinatura).

---

## Fase A — a varredura (medida; `file:line`)

- `FigureElem.numbering: Option<String>` (`figure.rs:26`) — o **padrão**. Consumidores:
  `layout/mod.rs:870-872` (gate = `Some`/`None`; o **valor** ignorado, `_pattern`),
  `figure.rs:78` `to_payload` (`is_counted`), `morph_canon` (`content.rs:2096`).
- **Gate ≠ número confirmado:** layout usa `numbering` só como gate; o **número**
  (`figure_progress` + `figure_number_at_index`, `mod.rs:874-886`) — **intacto**.
- **Produção [transporte]:** `native_figure` (`figure_image.rs:54`) via dispatch
  (`closures.rs:79-83`), sob o `Content::Styled` da fatia-1. Zero roteamento.
- **A assinatura `figure_numbering: Option<&str>`:** **141 ocorrências, 16 ficheiros**,
  último param do tipo `NativeFn` (`func.rs:78/97`), **morta em todas menos `native_figure`**.

---

## Estágio L0 (Trava aprovada pelo dono)

`f_fronteira_e1.md §3a.9` — subsecção **Figura**: de-bake do campo (consumidor lê a chain) +
colapso da assinatura partilhada; gate ≠ número; fixtures roteados; addendum pós-execução
(o aninhamento label×transporte; o `compute_labelled` por contador). Hash sincronizado ANTES
do código. Lint 0/0. Trava aprovada ("Aprovado").

---

## Estágio 1 — de-bake do campo (`file:line`)

- **Campo + clones removidos:** `figure.rs` (`numbering` field, `map_content`/`map_text`).
- **`to_payload`:** `is_counted` = placeholder `caption.is_some()` — o gate de padrão é ANDado
  pela chain no walk.
- **`Content::figure(.., Some(pat))`** (`content.rs:1266`) produz a **forma de transporte**
  (`Styled(Figure, custom("figure.numbering", Str(pat)))`), a forma de produção — usada pelos
  fixtures. `None` → figura simples. `native_figure` passa `None` (produção simples; a
  fatia-1 carrega o gate).
- **`morph_canon`:** arm `Figure` removido (subsumido pelo arm `Styled` transparente —
  mecanismo único, P345 N1).
- **Consumidores leem a chain:** `layout/mod.rs:870` (`self.chain.custom("figure.numbering")`
  = `Some(Str)`); introspect — `is_counted` patcheado da chain no walk top; **`compute_labelled`
  passa a confiar só no contador** (`flat_counter_at`, n>0, espelho do arm Equation), sem
  leitura de campo; `materialize_time` reconstrói com `numbering=None`.

## Estágio 2 — colapso da assinatura partilhada (`file:line`)

- **Tipo `NativeFn`** (`func.rs:78/97`) + accessor `native_fn_addr` (`func.rs:154`): param
  `Option<&str>` **removido**.
- **Dispatch** (`closures.rs:83`): deixa de ler `custom("figure.numbering")` e de passar o
  arg.
- **~141 definições de native** (16 ficheiros: calc, structural, foundations, layout,
  math_style, shapes, transforms, gradients, text, assert, figure_image, introspect,
  from_tags): `_figure_numbering: Option<&str>` removido.
- **Casts `as fn(_,_,_,_,_)`** (`rules.rs:701-709`, 5) → 4-arg; helper de teste
  (`mod.rs:8106`) e ~677 sítios de chamada de teste (5º arg `None`) ajustados.

---

## Asserções de fixture que viraram (declaradas, S5b)

Nenhuma asserção de **comportamento** mudou. As mudanças de fixture:
1. `figure.rs` tests: literais `FigureElem { .. }` sem `numbering`; `to_payload`/`is_counted`
   testam o **placeholder** (caption-only; gate de numbering no walk). Renomeado
   `is_counted_falso_sem_numbering` → `..._sem_caption`.
2. `eval/tests.rs` `collect_figure_numbering`: threada o padrão do `Styled` custom (verifica
   o transporte de produção).
3. `figure_tem_kind_e_numbering`: desembrulha o `Styled` (transporte) para checar kind +
   custom.
4. **~14 fixtures de figura rotulada** (introspect/layout): usavam `labelled(figure(.., Some))`
   = `Labelled{Styled{Figure}}` (inverso). Roteadas à forma de produção via o helper de teste
   `labelled_prod` (levanta o transporte para fora → `Styled{Labelled{Figure}}`), como a
   fatia-1 embrulha a cauda. Mesma resolução do P364 (sem código de produção especulativo).
5. `p203b_lacuna_..._4_casos`: parte (a) (`extract_payload` direto) desembrulha o `Styled`
   via helper `fig_inner`; `is_counted` aí é o placeholder (caption-only). Partes (b)/(c)
   (via walk) inalteradas.

---

## Gates (todos verdes)

```
build: workspace limpo (warnings de import = 4, pré-existentes em ficheiros não tocados pelo
  de-bake; baseline 5 — P365 não adicionou nenhum).
suíte (RUST_MIN_STACK=33554432): typst-core 2738 (0 falhas); demais 472/24/2/21, 0 falhas.
lint: crystalline-lint . = 0/0.

ACEITAÇÃO (oráculo = rede de caracterização +11 / vanilla):
  - figura: output idêntico ao de antes (paridade), lendo o padrão SÓ da chain.
  - fonte única: FigureElem.numbering REMOVIDO; só a chain carrega o padrão.
  - assinatura: figure_numbering colapsado do ABI (NativeFn + 141 defs + dispatch + calls).

INTACTOS (confirmado): o NÚMERO (figure_number_at_index, figure_progress) e o contador P335;
  α/caso 2, caso 4, morph ==/morph_canon (só o arm figura sai), flag P350c, Marco G;
  heading + equation (P364) — não regridem.

lente (instrumento): não re-corrida (tekt-cargo-dsm externa, fora da árvore). Analiticamente:
  troca leitura-de-campo por leitura-de-chain (ambas em rules/), remove um param do ABI;
  nenhuma aresta content→elements nova. content→elements = 66 esperado inalterado.
perf (depois): 0.6911 s ± 0.0091 (n=19); P364 0.7063, P363 0.6809 — behavior-neutral, deriva
  ambiental. Sem regressão.
L0 (critério 5): f_fronteira_e1.md §3a.9 (figura) + addendum, hash sincronizado ANTES do
  código, Trava aprovada.
commit: árvore commitada no estágio de fecho (hash abaixo).
```

---

## Estado / próximo

**Tocados:** `f_fronteira_e1.md` (§3a.9 figura + addendum, L0); `figure.rs`, `content.rs`,
`func.rs`, `closures.rs`, `figure_image.rs`, `layout/mod.rs`, `introspect.rs`,
`eval/tests.rs`, `layout/tests.rs`, + ~14 ficheiros de stdlib/introspect (colapso da
assinatura). Árvore limpa fora de docs.

**Os 4 caminhos duplos da auditoria P362:** heading + equation (P364) + **figura (P365)** =
**3 fechados**. O 4º — `Content::Text` `TextStyle` (`#set text` assa) — é o **F-5b**
(arrasta o bold do heading), escopo separado.

**Próximo (decisão do dono):** o **F-5b** (`TextStyle`), ou outra frente. **Termina aqui — não
emendo o passo seguinte (Trava 5).**

## Fora de escopo (confirmado)

O ponto 4 / `TextStyle` (F-5b); o F-6; o mapa aberto p/ `#set` de user-props; o Marco G;
DEBT-59 (flag CLI); DEBT-60 contador; qualquer toque no α / `morph_canon` / `==` ou na flag.
