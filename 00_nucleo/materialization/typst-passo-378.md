# Passo 378 — atomização do layout: próxima fatia (rumo a fechar o monólito)

> **O que faz.** Continua a atomização do `layout_content` (ADR-0109) na forma **B** já provada em
> três tipos de caso (containers P376; Heading que lê o Introspector + visuais P377). Move a próxima
> fatia dos arms restantes — os **visuais/decorações que faltam** (Place, Image, Figure, Overline, e
> o que a Fase A medir na família) — do `match` monolítico para `engine/layout/<elem>.rs` (free
> function `pub(super) fn layout`, módulo descendente: sem import reverso, sem `pub(crate)`, sem
> custo §3). **Objetivo declarado: seguir até o `layout_content` fechar** (todos os arms não-math
> distribuídos); os arms **math** ficam para fatia própria (descem a `rules/math/layout/`).
> **Content-preserving**: a lógica move, não muda. O `match` fica exaustivo, o despacho estático, os
> imports inalterados (`content→elements` não é gate — ADR-0109). **Trava de L0 antes de mover.**
> **Commita ao terminar** — e, por causa do mecanismo externo que apaga `.md` não-commitados neste
> repo (medido no P376/P377), **commita o L0/ADR ANTES de mover código**, não só ao fim. **Não
> emenda o seguinte** (Trava 5).

**Repositório de trabalho**: typst-crystalline (raiz), branch `tekt`.
**Número do passo**: P378 (confirmar livre).
**Pré-condição**: P377 fechado (ADR renumerada 0109; forma B; fatia visuais — `layout_content`
1857→1126, −731 acumulado). Suíte verde, lint **0/0**, `content→elements` 68 (não-gate). HEAD
pós-P377 (`aff0e257d`). Caveat de stack: `RUST_MIN_STACK=33554432`. Se algo não bater, parar e
reportar.
**Tipo**: atomização (content-preserving, forma B provada). A **rede de caracterização (+11, P331)**
é o oráculo. A métrica é de **leitura** (o monólito encolhe rumo a 0 arms gordos), não a lente.

---

## Nota de processo (o mecanismo que apaga `.md` não-commitados)

O P376/P377 mediram que arquivos `.md` não-commitados **somem da working tree** por um mecanismo
externo (o conteúdo ficou íntegro em `git`, restaurado de `HEAD`). **Mitigação obrigatória neste
lote:** commitar o **L0 (e qualquer doc novo) ANTES** de mover código — não deixar L0/ADR/relatório
não-commitados entre estágios. Se um `.md` sumir, restaurar de `HEAD` e registrar. (Isto é do
ambiente do dono; o passo só se protege.)

---

## Fase A — medir os arms restantes + escolher a fatia (a fonte vence; `file:line`)

1. **Reler** a ADR-0109 (forma B, não-metas), as **Travas anti-deriva**, e os P376/P377 (a forma
   provada; os arms já movidos: Block/Boxed/Stack/Pad/Heading/Transform/Shape/Columns).
2. **Inventariar os arms restantes** do `layout_content` (`engine/layout/mod.rs`), com `file:line` e
   linhas por arm: quais não-math ainda estão gordos no `match` (Place, Image, Figure, Overline, e
   os demais), e quais já têm arquivo flat (os 6 precedentes + os movidos). **Medir o que falta para
   fechar.**
3. **Escolher a fatia** (uma família coerente; rumo a fechar): os visuais/decorações restantes.
   Medir, por arm, o que a lógica lê do `Layouter` (estado, chain, introspector) — para a free
   function receber tudo, como o Heading provou. **A fatia sai da medição** (Trava 1).
4. **Confirmar as não-metas** (ADR-0109): `match` exaustivo (sem wildcard); despacho estático (sem
   `dyn`); `entities/` não tocado (`content→elements` inalterado). Se um arm exigir `dyn` ou tocar
   `entities/`, **parar e reportar**.
5. **Os arms math**: confirmar que são o grupo que desce a `rules/math/layout/` e **deixá-los fora**
   (fatia própria) — registrar quantos são, para saber o que falta após esta fatia.

---

## Limites duros (ADR-0109)

- **`match` exaustivo MANTIDO** (sem wildcard).
- **Despacho ESTÁTICO** — sem `dyn`/vtable. Se um arm exigir, **parar e reportar**.
- **`entities/` não tocado** — `content→elements` inalterado, não-gate.
- **Forma B** — free function em `engine/layout/<elem>.rs`, módulo descendente (sem ciclo, sem
  `pub(crate)`). **Não usar a Opção A** (rejeitada).
- **Content-preserving** — a rede de caracterização passa **sem alteração**; se virar, a lógica
  mudou ao mover → **investigar, não mascarar**.
- **Não tocar** o α/caso 2, o `morph_canon`/`==`, o caso 4, a flag, o F-5b.

---

## Estágios

### Estágio L0 — desenho + Trava + **commit do L0** (PARA aqui para o dono)
Com a fatia medida, registrar no L0 (`rules/atomizacao_elementos.md`) os elementos da fatia, a forma
B, as não-metas, e **o inventário do que falta para fechar o monólito** (quantos arms não-math
restam após esta fatia). Sincronizar hashes. **Commitar o L0 agora** (mitigação do mecanismo
externo). **TRAVA**: para no chat — a medição + o desenho + o escopo + os hashes para o dono
aprovar. Nenhum arm movido antes.

### Estágio 1 — mover a fatia (após aprovação)
Por elemento: a lógica de layout **muda** do `match` para `engine/layout/<elem>.rs` (free function,
forma B); o arm do núcleo vira a delegação de uma linha; imports mortos em `mod.rs` removidos.
Content-preserving.

### Estágio Teste
- A **rede de caracterização (+11)** e a suíte passam **sem alteração** (content-preserving).
- **Leitura**: o `layout_content` encolhe mais X linhas (registrar o acumulado desde 1857 e quantos
  arms gordos não-math restam — o progresso rumo a fechar).
- Confirmar: `match` exaustivo, despacho estático, `entities/` intacto, e o α/caso 2/caso 4/flag/
  F-5b intactos.

### Estágio F — linhagem
`@updated`; `--fix-hashes`; V7 limpa.

### Estágio de fecho — COMMIT (padrão)
Com os gates verdes, **commitar** a fatia: `git add -A && git commit -m "Passo 378 — atomização
fatia <família>: layout para engine/layout/<elem>.rs"`. Árvore limpa e commitada.

---

## Verificação (gates)

```
build: limpo por estágio.
suíte (RUST_MIN_STACK=33554432): inalterada em número (content-preserving) — rede +11 sem asserção
  virada. Se virar, investigar.
lint: crystalline-lint . = 0/0.

ACEITAÇÃO (leitura — ADR-0109):
  - o layout_content encolhe (−X; acumulado desde 1857); cada elemento da fatia legível no seu arquivo.
  - inventário: quantos arms gordos não-math restam (progresso rumo a fechar).
  - content-preserving: comportamento idêntico (rede de caracterização).

NÃO-METAS confirmadas: match exaustivo (0 wildcards); despacho estático (0 dyn); entities/ intacto
  (content→elements = 68 não-gate); forma B.

INTACTOS: α/caso 2, morph ==/morph_canon, caso 4, flag P350c, F-5b; os 3 numbering; o #set de props.
lente (instrumento, NÃO gate): content→elements = 68 inalterado.
perf: free function inlinável → sem regressão; medir.
L0 (critério 5): atomizacao_elementos.md + hash sincronizado e COMMITADO ANTES de mover, Trava aprovada.
commit: L0 commitado no Estágio L0; fatia commitada no fecho; árvore limpa.
```

---

## Válvula declarada

Se a fatia não couber num lote, reduzir. Cada arm migra independente — o `match` magro coexiste com
arms gordos, exaustividade intacta o tempo todo. **Para fechar o monólito** podem ser precisos mais
lotes desta forma (cada um uma fatia) — registrar, ao fim de cada um, quantos arms restam, até
chegar a **0 arms gordos não-math**. Os math são a fatia final própria.

---

## O que NÃO fazer

- **Não usar a forma A** (acoplamento dado→render) nem `dyn`/wildcard (ADR-0109).
- **Não tocar `entities/`** nem tentar reduzir `content→elements`.
- **Não mudar comportamento ao mover** — content-preserving; se a rede virar, investigar.
- **Não tocar** o α/caso 2, o `morph_canon`/`==`, o caso 4, a flag, o F-5b.
- **Não deixar L0/doc não-commitado** entre estágios (o mecanismo externo apaga `.md`).
- **Não pular a Trava** (medição + L0 commitado + hash do dono antes de mover).
- **Não emendar nem iniciar o passo seguinte** (Trava 5).
- **Não importar a quarentena.**

---

## Relatório (`typst-passo-378-relatorio.md` + resumo no chat)

A **medição dos arms restantes** (`file:line`, linhas por arm; quantos não-math faltam); a **fatia
movida** por elemento (`file:line` do `engine/layout/<elem>.rs` + o arm magro); a paridade (rede +11
sem alteração); a **métrica de leitura** (o `layout_content` −X; o acumulado desde 1857; **quantos
arms gordos não-math restam** — o progresso); as não-metas confirmadas; a prova de que o α/caso 2/
caso 4/flag/F-5b ficaram intactos; a lente (registrada = 68, não-gate); a perf; **os commits**
(L0 no Estágio L0; a fatia no fecho); a nota do mecanismo externo se ocorreu; `git status` limpo;
lint 0/0; o caveat de stack. **A nota de quanto falta para fechar o monólito (e que os math são a
fatia final).** Termina aqui — não emenda o seguinte.

## Fora de escopo (confirmado)

Os arms **math** (fatia final própria, descem a `rules/math/layout/`); os arms não-math além desta
fatia (lotes seguintes, mesma forma, até fechar); a atomização do **`introspect.rs`** (os 43 arms —
após o layout fechar); a **varredura do projeto** e os **crates** (depois); o **Marco G /
desacoplamento** (descartado — ADR-0109 não-meta); DEBT-59; DEBT-60.
