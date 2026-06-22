# Passo 377 — corrigir a ADR de atomização (número + forma) + próxima fatia (atomização)

> **O que faz.** Duas coisas, na ordem: **(0) corrige a ADR de atomização** que o P376 commitou com
> dois erros — o **número** (foi gravada como `0110`; o dono indicou que o próximo livre é anterior
> — **verificar no repo, não assumir**) e a **forma canónica** (foi gravada mostrando a **Opção A**
> — a lógica no arquivo do struct — que a medição do P376 provou **errada**: cria o acoplamento
> `entities → rules` (dado→render) e o custo §3; a forma que **funcionou** é a **Opção B**, free
> function em `rules/layout/<elem>.rs`). **(1) roda a próxima fatia de atomização** — a família dos
> arms gordos restantes (Figure/Image/Shape/Transform e os do grupo) — na forma B **já provada**
> (P376: containers Block/Boxed/Stack/Pad, monólito −581 linhas, sem ciclo, content-preserving). A
> correção da ADR é **Estágio 0** porque ela **governa o rollout** — não rodar mais arms apoiados
> numa ADR cuja forma documentada está errada. **Content-preserving**: a lógica move, não muda. O
> `match` fica exaustivo, o despacho estático, os imports inalterados (`content→elements` não é
> gate). **Trava de L0 antes de mover.** **Commita ao terminar** e **não emenda o seguinte**
> (Trava 5).

**Repositório de trabalho**: typst-crystalline (raiz), branch `tekt`.
**Número do passo**: P377 (confirmar livre).
**Pré-condição**: P376 fechado (fatia containers materializada, Opção B; a ADR commitada como
`0110` com a forma da Opção A — a corrigir). Suíte verde, lint **0/0**, `content→elements` 68
(não-gate). HEAD pós-P376 (`cf204f949` + limpeza). Caveat de stack: `RUST_MIN_STACK=33554432`. Se
algo não bater, parar e reportar.
**Tipo**: correção de ADR (documentação) + atomização (content-preserving, forma B provada). A
**rede de caracterização (+11, P331)** é o oráculo. A métrica é de **leitura** (o monólito encolhe),
não a lente.

---

## Estágio 0 — corrigir a ADR de atomização (governa o rollout)

**A — o número (verificar, NÃO assumir):** listar os ADRs em `00_nucleo/adr/` e achar o **último
número usado** / o **primeiro livre**, com `file:line`. O dono indicou que o correto é **anterior a
0110** (provavelmente 0109 — **confirmar pela varredura**, não fixar de memória; foi um número que
o assistente inventou sem checar no P376). **Renumerar** a ADR para o número livre medido, atualizar
todas as referências (`claude.md`, o L0, os relatórios que a citam) e o nome do arquivo.

**B — a forma canónica (trocar a Opção A pela B medida):** a ADR foi gravada com a forma canónica =
**Opção A** (`impl XElem { fn layout }`, a lógica no arquivo do struct). A medição do P376 provou
que a A **cria o acoplamento `entities → rules::layout`** (ciclo, `pub(crate)`, genéricos — o custo
§3) e vai contra a separação dado/render (ADR-0107). Substituir a forma canónica da ADR pela **Opção
B medida**:
- a lógica de layout do elemento vai para **`rules/layout/<elem>.rs`** (ao lado do render, na camada
  de render), como **free function** `pub(super) fn layout<M,S>(layouter, e)` — acessa o estado
  privado do `Layouter` por ser **módulo descendente** (sem import reverso, sem `pub(crate)`, sem
  custo §3);
- o arm no núcleo fica magro: `Content::Heading(h) => heading::layout(self, h)`;
- **a nota de que a Opção A foi rejeitada** por criar o acoplamento dado→render (registrar como o
  erro corrigido, igual ao registro do erro do P346 — para a IA não reintroduzir a A).
- A **definição de atomização e as não-metas ficam** (estavam certas); só a **forma** muda.
Sincronizar o `claude.md` (o trecho da forma) e os hashes.

**Commit do Estágio 0** (a correção da ADR é coesa por si): `git commit -m "Passo 377 estágio 0 —
ADR atomização: número <NNNN> + forma canónica Opção B (A rejeitada: acoplamento dado→render)"`.

---

## Fase A — a próxima fatia (a fonte vence; `file:line`)

1. **Reler** a ADR corrigida (a forma B), as **Travas anti-deriva**, e o P376 (os arms gordos
   restantes: Figure 34 · Image 41 · Shape 33 · Transform 49 · Overline 93 · Place 66 · Columns 44 ·
   Heading 44 — confirmar quais já têm precedente flat `figure.rs`/`image.rs` e quais faltam).
2. **Escolher a fatia** (uma família coerente — ex.: Figure/Image/Shape, ou Transform/Overline/Place,
   ou Heading sozinho se for o caso de prova de um elemento que lê estado diferente): medir, por arm,
   quanta lógica move e o que ela lê do `Layouter` (para a free function receber tudo). **A fatia sai
   da medição** (Trava 1) — não fixar a família de memória.
3. **Confirmar as não-metas** (ADR): o `match` fica exaustivo (sem wildcard); o despacho estático
   (sem `dyn`); `entities/` **não** é tocado (`content→elements` inalterado). Se mover um arm exigir
   `dyn` ou tocar `entities/`, **parar e reportar**.
4. **Os precedentes**: alguns elementos já têm arquivo flat em `layout/` (figure/image/grid — os 6
   precedentes). Confirmar a forma exata deles e segui-la (consistência — o desvio do P376 foi para
   alinhar com eles).

---

## Limites duros (ADR de atomização)

- **`match` exaustivo MANTIDO** (sem wildcard) — a garantia do compilador fica.
- **Despacho ESTÁTICO** — sem `dyn`/vtable. Se um arm exigir, **parar e reportar**.
- **`entities/` não tocado** — `content→elements` inalterado; não é gate, não tentar reduzi-lo.
- **Forma B** — free function em `rules/layout/<elem>.rs`, módulo descendente (sem ciclo, sem
  `pub(crate)`, sem custo §3). **Não usar a Opção A** (a forma rejeitada).
- **Content-preserving** — a rede de caracterização passa **sem alteração**; se virar, a lógica
  mudou ao mover → **investigar, não mascarar**.
- **Não tocar** o α/caso 2, o `morph_canon`/`==`, o caso 4, a flag, o F-5b.

---

## Estágios

### Estágio L0 — desenho (com a medição) + Trava (PARA aqui para o dono)
Com a fatia medida, registrar no L0 (`rules/atomizacao_elementos.md`) os elementos da fatia, a forma
B, e as não-metas confirmadas. Sincronizar hashes. **TRAVA**: para no chat — a ADR corrigida (já
commitada no Estágio 0) + a medição da fatia + o escopo + os hashes para o dono aprovar. Nenhum
arm movido antes.

### Estágio 1 — mover a fatia (após aprovação)
Por elemento da fatia: a lógica de layout **muda** do `match` para `rules/layout/<elem>.rs` (free
function, forma B); o arm do núcleo vira a delegação de uma linha. Content-preserving.

### Estágio Teste
- A **rede de caracterização (+11)** e a suíte passam **sem alteração** (content-preserving).
- **Leitura**: o `layout_content` encolhe mais X linhas; cada elemento da fatia legível no seu arquivo.
- Confirmar: `match` exaustivo, despacho estático, `entities/` intacto (`content→elements` 68), e o
  α/caso 2/caso 4/flag/F-5b intactos.

### Estágio F — linhagem
`@updated`; `--fix-hashes`; V7 limpa.

### Estágio de fecho — COMMIT (padrão)
Com os gates verdes, **commitar** a fatia: `git add -A && git commit -m "Passo 377 — atomização
fatia <família>: layout para rules/layout/<elem>.rs (forma B)"`. Árvore limpa e commitada.

---

## Verificação (gates)

```
build: limpo por estágio.
suíte (RUST_MIN_STACK=33554432): inalterada em número (content-preserving) — rede +11 sem asserção
  virada. Se alguma virar, a lógica mudou ao mover → investigar.
lint: crystalline-lint . = 0/0.

ESTÁGIO 0 (ADR): número renumerado para o livre medido (file:line da varredura); forma canónica =
  Opção B (A registrada como rejeitada); claude.md + referências sincronizados.

ACEITAÇÃO (leitura — ADR de atomização):
  - o layout_content encolhe (−X linhas); cada elemento da fatia legível no seu arquivo.
  - content-preserving: comportamento idêntico (rede de caracterização).

NÃO-METAS confirmadas: match exaustivo (0 wildcards); despacho estático (0 dyn de elemento);
  entities/ não tocado (content→elements = 68 inalterado, não-gate); forma B (sem ciclo/pub(crate)).

INTACTOS: α/caso 2, morph ==/morph_canon, caso 4, flag P350c, F-5b; os 3 numbering; o #set de props.
lente (instrumento, NÃO gate): content→elements registrado = 68 (a atomização não o move).
perf: free function inlinável → sem regressão; medir.
L0 (critério 5): ADR corrigida (Estágio 0) + atomizacao_elementos.md + hash sincronizado ANTES de
  mover, Trava aprovada.
commit: Estágio 0 (ADR) e Estágio 1 (fatia) commitados; árvore limpa ao fim.
```

---

## Válvula declarada

Se a fatia escolhida não couber num lote, reduzir (menos elementos). Cada arm migra independente — o
`match` magro coexiste com arms ainda-gordos, e a exaustividade fica intacta o tempo todo. Os arms
**math** (agrupados, descem a `rules/math/layout/`) ficam para uma fatia própria.

---

## O que NÃO fazer

- **Não assumir o número da ADR** — varrer `00_nucleo/adr/` e medir o livre (o erro do P376 foi
  inventar 0110 sem checar).
- **Não usar a forma da Opção A** (acoplamento dado→render) — a forma é a B.
- **Não usar `dyn`/vtable** nem remover a exaustividade (seria desacoplamento, fora da ADR).
- **Não tocar `entities/`** nem tentar reduzir `content→elements`.
- **Não mudar comportamento ao mover** — content-preserving; se a rede virar, investigar.
- **Não tocar** o α/caso 2, o `morph_canon`/`==`, o caso 4, a flag, o F-5b.
- **Não pular a Trava** (ADR + medição + L0 + hash do dono antes de mover a fatia).
- **Não emendar nem iniciar o passo seguinte** (Trava 5).
- **Não importar a quarentena.**

---

## Relatório (`typst-passo-377-relatorio.md` + resumo no chat)

**Estágio 0**: a ADR renumerada (o número livre medido com `file:line` da varredura; as referências
sincronizadas) + a forma canónica trocada para a Opção B (a A registrada como rejeitada, com o
porquê: acoplamento dado→render); o commit do Estágio 0 (hash). **A fatia**: a medição dos arms
escolhidos (`file:line`, linhas por arm); o L0 com hash e a Trava aprovada; a lógica movida por
elemento (`file:line` do `rules/layout/<elem>.rs` + o arm magro); a paridade (rede +11 sem
alteração); a métrica de leitura (o monólito −X linhas); as não-metas confirmadas (exaustivo,
estático, `entities/` intacto); a prova de que o α/caso 2/caso 4/flag/F-5b ficaram intactos; a
lente (registrada, não gate); a perf; **o commit de fecho** (hash); `git status` limpo; lint 0/0; o
caveat de stack. **Termina aqui — não emenda o seguinte.**

## Fora de escopo (confirmado)

Os arms restantes além da fatia (lotes futuros); os arms **math** (fatia própria, descem a
`rules/math/layout/`); a **varredura do projeto inteiro** (depois dos elementos); a **decisão de
crates** (depois); o **Marco G / desacoplamento** (descartado — ADR não-meta); DEBT-59; DEBT-60.
