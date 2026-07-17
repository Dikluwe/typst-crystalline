# Passo 380 — atomização do layout, Fatia 1/2: fluxo de bloco e estrutura

> **O que faz.** Primeira de **duas fatias** (decisão do dono) que atomizam os elementos de domínio
> restantes do `layout_content` (929 linhas, pós-P378) na forma **B** provada 15× (P376-378). Esta
> fatia move os **elementos de fluxo de bloco e estrutura**: Listas (EnumItem/ListItem/Terms/
> TermItem), Tabelas/Grid (**juntos** — o P379 mediu que partilham `self.layout_grid`), Breaks
> (Pagebreak/Colbreak), Spacing (VSpace/HSpace/Repeat) — do `match` monolítico para
> `engine/layout/<elem>.rs` (free function `pub(super) fn layout`, módulo descendente: sem import
> reverso, sem `pub(crate)`). **Deixa FORA, por medição (P379):** o **Text** (fatia própria — grande,
> chain-pesado, caminho quente); a **máquina do layouter** (Sequence/Styled/Dynamic/SetPage — **não
> é elemento de domínio**, orquestra/reconfigura — fica onde está); a fatia **math** (final, path
> próprio). **Content-preserving**: a lógica move, não muda. O `match` fica exaustivo, o despacho
> estático, os imports inalterados (`content→elements` não é gate — ADR-0109). **L0 commitado ANTES
> de mover** (mecanismo externo que apaga `.md`). **Trava de L0**, **commit ao fim**, **não emenda o
> seguinte** (Trava 5).

**Repositório de trabalho**: typst-crystalline (raiz), branch `tekt`.
**Número do passo**: P380 (confirmar livre).
**Pré-condição**: P379 fechado (recon: acoplamento baixo → fatiar é barato; Grid+Table cluster;
Text atomizável; Sequence/Styled/Dynamic/SetPage = máquina; ~27 arms não-math, ~444 linhas). Suíte
verde, lint **0/0**, `content→elements` 68 (não-gate). HEAD pós-P379 (`26c822677`). Caveat de stack:
`RUST_MIN_STACK=33554432`. Se algo não bater, parar e reportar.
**Tipo**: atomização (content-preserving, forma B provada). A **rede de caracterização (+11, P331)**
é o oráculo. A métrica é de **leitura** (o monólito encolhe), não a lente.

---

## Escopo desta fatia (medido no P379)

| Família | Arms | Nota |
|---|---|---|
| **Listas** | EnumItem `@834`, ListItem `@820`, Terms `@1165`, TermItem `@1172` | fluxo-texto + recursão; TermItem lê `chain` |
| **Tabelas/Grid** | Grid `@928`, Table `@957` (+ Cells/Headers/Footers se inline) | **juntos** — partilham `self.layout_grid` (cluster medido) |
| **Breaks** | Pagebreak `@1322`, Colbreak `@1347` | `new_page`, `regions`, `flush_line`; Pagebreak lê `pages` |
| **Spacing** | VSpace `@1218`, HSpace `@1214`, Repeat `@1291` | `regions`/`font_size_pt`; Repeat só recursa |

~13 arms. **Grid e Table na mesma fatia** (cluster). O destino de cada um:
`engine/layout/<elem>.rs` (ou um arquivo por família coerente — ex.: `lists.rs` para os de lista, se
partilharem helper; a Fase A confirma a granularidade).

---

## Fase A — confirmar e desenhar (a fonte vence; `file:line`)

1. **Reler** a ADR-0109 (forma B, não-metas), as **Travas anti-deriva**, e o P379 (o acoplamento, o
   cluster Grid+Table, a granularidade).
2. **Confirmar, por arm**, com `file:line`: a lógica e o que lê do `Layouter` (estado/métodos/tipos)
   — para a free function receber tudo. Confirmar o cluster Grid+Table (`self.layout_grid`).
3. **A granularidade**: um arquivo por elemento (como os 15 já feitos) ou por família onde
   partilham helper (ex.: as listas). **Sai da medição** (Trava 1) — seguir os precedentes do repo.
4. **Confirmar as não-metas** (ADR-0109): `match` exaustivo (sem wildcard); despacho estático (sem
   `dyn`); `entities/` não tocado. Se um arm exigir `dyn` ou tocar `entities/`, **parar e reportar**.
5. **Confirmar o que fica FORA**: Text (fatia própria), Sequence/Styled/Dynamic/SetPage (máquina),
   math (final) — **não** tocar nesta fatia.

---

## Limites duros (ADR-0109)

- **`match` exaustivo MANTIDO** (sem wildcard).
- **Despacho ESTÁTICO** — sem `dyn`/vtable. Se um arm exigir, **parar e reportar**.
- **`entities/` não tocado** — `content→elements` inalterado, não-gate.
- **Forma B** — free function em `engine/layout/<elem>.rs`, módulo descendente. **Não a Opção A.**
- **Não tocar o Text, a máquina (Sequence/Styled/Dynamic/SetPage), nem a math** — fora desta fatia.
- **Content-preserving** — a rede de caracterização passa **sem alteração**; se virar, a lógica
  mudou ao mover → **investigar, não mascarar**.
- **Não tocar** o α/caso 2, o `morph_canon`/`==`, o caso 4, a flag, o F-5b.

---

## Estágios

### Estágio L0 — desenho + Trava + **commit do L0** (PARA aqui para o dono)
Registrar no L0 (`rules/atomizacao_elementos.md`) os elementos da Fatia 1, a granularidade
(por-elemento ou por-família), a forma B, as não-metas, e o inventário do que resta (Fatia 2 +
Text + máquina + math). Sincronizar hashes. **Commitar o L0 agora** (mecanismo externo). **TRAVA**:
para no chat — a medição + o desenho + a granularidade + os hashes para o dono aprovar. Nenhum arm
movido antes.

### Estágio 1 — mover a Fatia 1 (após aprovação)
Por elemento/família: a lógica de layout **muda** do `match` para `engine/layout/<elem>.rs` (free
function, forma B); o arm do núcleo vira a delegação de uma linha; imports mortos em `mod.rs`
removidos. Grid+Table juntos. Content-preserving.

### Estágio Teste
- A **rede de caracterização (+11)** e a suíte passam **sem alteração** (content-preserving).
- **Leitura**: o `layout_content` encolhe X linhas; o acumulado desde 1857; quantos arms de domínio
  restam (a Fatia 2 + Text).
- Confirmar: `match` exaustivo, despacho estático, `entities/` intacto; o Text/máquina/math não
  tocados; o α/caso 2/caso 4/flag/F-5b intactos.

### Estágio F — linhagem
`@updated`; `--fix-hashes`; V7 limpa.

### Estágio de fecho — COMMIT (padrão)
Com os gates verdes, **commitar**: `git add -A && git commit -m "Passo 380 — atomização Fatia 1/2:
fluxo de bloco e estrutura (listas/tabelas/breaks/spacing)"`. Árvore limpa e commitada.

---

## Verificação (gates)

```
build: limpo por estágio.
suíte (RUST_MIN_STACK=33554432): inalterada em número (content-preserving) — rede +11 sem asserção
  virada. Se virar, investigar.
lint: crystalline-lint . = 0/0.

ACEITAÇÃO (leitura — ADR-0109):
  - o layout_content encolhe (−X; acumulado desde 1857); cada elemento da fatia legível no seu arquivo.
  - content-preserving: comportamento idêntico (rede de caracterização).

NÃO-METAS confirmadas: match exaustivo (0 wildcards); despacho estático (0 dyn); entities/ intacto
  (content→elements = 68 não-gate); forma B.

FORA (não tocados): Text, Sequence/Styled/Dynamic/SetPage (máquina), math.
INTACTOS: α/caso 2, morph ==/morph_canon, caso 4, flag P350c, F-5b; os 3 numbering; o #set de props.
lente (instrumento, NÃO gate): content→elements = 68 inalterado.
perf: free function inlinável → sem regressão; medir.
L0 (critério 5): atomizacao_elementos.md + hash COMMITADO ANTES de mover, Trava aprovada.
commit: L0 no Estágio L0; fatia no fecho; árvore limpa.
```

---

## Válvula declarada

Se a Fatia 1 (~13 arms) não couber num lote, reduzir (ex.: listas+tabelas num, breaks+spacing
noutro). Cada arm migra independente — `match` magro coexiste com arms gordos, exaustividade
intacta. Grid+Table ficam **sempre** juntos (cluster).

---

## O que NÃO fazer

- **Não usar a forma A** nem `dyn`/wildcard (ADR-0109).
- **Não tocar `entities/`** nem tentar reduzir `content→elements`.
- **Não tocar o Text, a máquina (Sequence/Styled/Dynamic/SetPage), nem a math** (fora da fatia).
- **Não mudar comportamento ao mover** — content-preserving; se a rede virar, investigar.
- **Não tocar** o α/caso 2, o `morph_canon`/`==`, o caso 4, a flag, o F-5b.
- **Não deixar L0/doc não-commitado** entre estágios (mecanismo externo).
- **Não pular a Trava** (medição + L0 commitado + hash do dono antes de mover).
- **Não emendar nem iniciar o passo seguinte** (Trava 5).
- **Não importar a quarentena.**

---

## Relatório (`typst-passo-380-relatorio.md` + resumo no chat)

A **confirmação por arm** (`file:line`, o que lê do `Layouter`, o cluster Grid+Table); a
**granularidade** escolhida; o L0 commitado (hash) e a Trava aprovada; a lógica movida por
elemento/família (`file:line` do `engine/layout/<elem>.rs` + o arm magro); a paridade (rede +11 sem
alteração); a **métrica de leitura** (o `layout_content` −X; o acumulado; quantos arms de domínio
restam = Fatia 2 + Text); as não-metas confirmadas; a confirmação de que o Text/máquina/math
ficaram fora e o α/caso 2/caso 4/flag/F-5b intactos; a lente (= 68, não-gate); a perf; **os commits**
(L0 + fatia); `git status` limpo; lint 0/0; o caveat de stack. **A nota do que resta: a Fatia 2
(refs/avulsos), o Text (fatia própria), e a math (final).** Termina aqui — não emenda o seguinte.

## Fora de escopo (confirmado)

A **Fatia 2** (refs/citações + avulsos — próximo lote); o **Text** (fatia própria); a **máquina**
(Sequence/Styled/Dynamic/SetPage — fica, não é elemento); a fatia **math** (final, path próprio); os
**displays de counter/state** ([a-decidir], fronteira — decisão do dono perto do fim); a atomização
do **`introspect.rs`** (após o layout); varredura/crates (depois); Marco G/desacoplamento
(descartado); DEBT-59; DEBT-60.
