# Passo 345 — o `==` de conteúdo vira semântico/morfológico (a ADR-0107 exercida pela primeira vez)

> **O que faz.** Torna o operador `==` da **linguagem** (eval) uma comparação de
> **morfologia** — dois conteúdos de mesma forma de linguagem (texto, markup, estilo
> semântico) casam; estilo de **render** (assado/transporte) **não** entra. Fecha o
> Achado 2 na camada certa (linguagem, não de-bake) e é a **primeira aplicação da
> ADR-0107**. Lote **completo**: cobre **todas** as variantes de `Content` cuja
> fronteira morfologia-vs-render é **clara da fonte**; variante **ambígua** para na
> trava para o dono decidir a definição (a ADR sendo testada). **Não toca** o
> `PartialEq` do Rust (dois sistemas, ADR-0025).

**Repositório de trabalho**: typst-crystalline (raiz).
**Número do passo**: P345 (confirmar livre).
**Pré-condição**: P344 fechado — **ADR-0107 EM VIGOR** + regra no `claude.md`
(confirmar que a secção e a linha na tabela de ADRs vigentes existem **no `claude.md`**,
não só no relatório). HEAD pós-P344; suíte **2719** (`typst-core --lib`) / **3238**
(workspace), lint 0/0, árvore limpa. Se não bater, parar.
**Tipo**: **lote completo** (todas as variantes claras numa corrida) — **NÃO
content-preserving**: muda a semântica do `==` da linguagem de propósito. Regra do
P340 + **ADR-0107**: a paridade e o critério de aceitação são **morfológicos** (mesma
morfologia → iguais), **não** "o booleano bate" nem diff de bytes. O vanilla compilado
é oráculo da **semântica** quando ambígua, não de saída.
**Objetivo**: o `==` do Typst sobre `Content`/`Value` comparar **morfologia** — texto,
markup estrutural, estilo semântico (`*bold*`/`_italic_`) — e **ignorar** estilo de
render (o `TextStyle` assado; o transporte `Style::Custom` do β1). Resultado: dois
conteúdos de mesma morfologia e render diferente casam; de morfologia diferente, não.
`it.body == [a]` casa **como consequência** disso, não como alvo.
**Limites duros**: **não** tocar o `PartialEq` derivado do Rust (testes/coleções/
`IndexMap` — ADR-0025, dois sistemas de propósito); **não** des-assar nada (o de-bake é
F-5, concern separado de render/duplicação); **não** consertar a recursão/guard (P341b,
fatia posterior — só remedir o `m1` no fim).
**Fontes**: **ADR-0107** (a definição: semântica/sintaxe/morfologia; morfologia =
conteúdo de linguagem vs estilo de render), **ADR-0025** (a igualdade do Typst é
semântica, separada da do Rust; o `==` do Typst vive no `eval_binary_op`), relatório
P342 (o `==` de conteúdo caindo no `PartialEq` do Rust, `content.rs:1711` via
`value.rs:17`; o wildcard de `eval_binary_op`), relatório P343 (o inventário do bake:
quais variantes carregam render assado/transporte), relatório P341b (o `m1`),
`lab/typst-original/` (vanilla — oráculo da semântica do `==` de conteúdo onde a fonte
for ambígua; `file:line`).
**Commits** (isoláveis): "Passo 345 — caronas" · "Passo 345 — Fase A: tabela
morfologia-por-variante" · "Passo 345 — `==` morfológico (eval)" · "Passo 345 —
evolução de testes vs semântica" · "Passo 345 — remedição do m1 (P341b)".

---

## Caronas (commit próprio)
- **C0 — base exata**: confirmar 2719 / 3238 no HEAD pós-P344; confirmar a secção
  ADR-0107 e a linha na tabela **dentro do `claude.md`** (o relatório P344 marcou a
  inserção; verificar que não ficou só no relatório).
- **C1 — registro do gatilho β1 (P339 §3a.8)**: o gatilho ("se igualdade-de-`Content`
  virar requisito, reavaliar o wrapper") é **realizado** aqui na camada certa — o `==`
  morfológico trata o transporte `Custom` como render (transparente), não como
  morfologia. Anotar na §3a.8 que o gatilho foi resolvido no P345 pela via da
  linguagem (não pelo de-bake).

---

## Fase A — a tabela da morfologia por variante (o coração do lote; `file:line`)

A ADR-0107 deu o **exemplo** (`it.body`="a"; bold=render); este lote produz a
**definição operacional**: para **cada variante** que o `==` de conteúdo compara, o
que entra na morfologia e o que é render. Sem código de produção nesta fase.

Para cada variante de `Content` (e os campos de `Value` que delegam a ela), classificar
cada campo em **morfologia** (entra no `==`) ou **render** (sai), com `file:line` e a
razão:
- **Texto/markup estrutural** (o texto de `Text`; `body`/`level` de heading; sequência;
  listas; etc.) → **morfologia**.
- **Estilo semântico** (`Style::Bold`/`Italic` de `*bold*`/`_italic_`; o `Styled`
  produzido por markup, ADR-0038) → **morfologia** (faz parte do que o conteúdo *é* na
  linguagem; o `#show strong` o vê).
- **Render assado** (o `TextStyle` em `Content::Text` — P343 #1) → **render, sai**.
- **Transporte** (o `Style::Custom` de numbering do β1 em `Content::Styled` — P343/P342)
  → **render, sai** (transparente; descer no body).
- **Campos de numbering assados** (`numbering_active` de heading/equation/figure — P343
  #2/#3/#4) → decidir: são **morfologia** (estado de linguagem que o usuário setou via
  `#set`) ou **render**? Medir contra o vanilla: o `==` do vanilla sobre dois headings
  iguais em texto mas um numerado e outro não — casa ou não? A resposta da fonte decide.
- **Casos de fronteira** (ex.: o glyph de `SmartQuote` resolvido na criação — P343):
  classificar com a razão.

**Onde a fonte for ambígua** (não dá para dizer da leitura se um campo é morfologia ou
render, e o vanilla não desambígua claramente): **marcar como AMBÍGUO** com a pergunta
precisa — não decidir. Essas variantes **param na trava**.

Saída: a tabela **variante × campo × morfologia|render|AMBÍGUO × razão × `file:line` ×
(vanilla, se medido)**. É a definição operacional da morfologia, e é o que o `==`
semântico vai consultar.

---

## TRAVA ARQUITETURAL — revisão da definição operacional da morfologia

Esta trava **não** é decisão de desenho (a ADR-0107 já a fixou) — é a **ADR exercida
pela primeira vez**, e você disse que o ajuste da definição volta ao dono. Emitir:
- a tabela completa (todas as variantes classificadas);
- a lista de **AMBÍGUOS** com a pergunta de cada um (estes esperam — não entram no
  código desta corrida);
- a posição proposta para os campos de numbering (morfologia vs render), com a medição
  do vanilla.

**Parar** se houver AMBÍGUO ou se a tabela revelar que a definição de morfologia da
ADR-0107 precisa de refino (aí é ajuste de ADR, decisão do dono, antes de código).
**Seguir sem parar** se a tabela for inteiramente clara e bater com a ADR — nesse caso
o lote executa tudo. (Onde a fronteira é clara, não há checkpoint de decisão; a parada
é só na ambiguidade.)

---

## Fase B — o `==` morfológico (após a trava; só as variantes claras)

### Estágio C1 — a comparação morfológica de `Content`
Implementar a comparação semântica de conteúdo que o `==` da linguagem usa, sobre a
tabela da Fase A: compara os campos **morfologia**, ignora os **render** (desce
transparente nos wrappers de transporte; ignora o `TextStyle` assado). **Onde** ela
vive: no caminho do `==` do Typst (`eval_binary_op` — ADR-0025), **não** no
`#[derive(PartialEq)]` (que fica intacto para testes/coleções). Se o `eval_binary_op`
hoje cai no wildcard `(Eq, a, b) => a == b` para conteúdo (P342), substituir esse
caminho, para conteúdo, pela comparação morfológica; os demais tipos seguem como estão.

### Estágio C2 — `Value::Content` e a cadeia de delegação
Garantir que o `==` da linguagem sobre `Value::Content` usa a comparação morfológica
(não o `PartialEq` derivado de `Value`). O `PartialEq` derivado do Rust **permanece**
para uso interno; só o caminho de eval muda.

### Estágio E — evolução de testes pela semântica (não pelo booleano)
Os testes que asseveram o `==` de conteúdo: classificar pela **semântica** (o que o
vanilla/a linguagem manda), não pela mecânica. Cada teste que a semântica morfológica
**muda** evolui **um a um**, com a justificativa morfológica escrita (e a saída do
vanilla colada onde o vanilla foi o oráculo). Os testes do `PartialEq` do Rust
(estrutural, para coleções/debug) ficam **intactos** — são o outro sistema.

### Estágio M — remedição do `m1` (P341b)
Com `it.body == [a]` casando por morfologia, reavaliar o `m1`: deve sair de "a" para
"b" (o guard trunca no nível 1). Se ainda divergir do vanilla, a causa restante é o
**guard** (mecanismo de render), **fatia posterior** — registrar o novo resultado e
apontar, **não** consertar aqui.

### Estágio F — linhagem
`@updated` nos ficheiros tocados; `--fix-hashes`; V7 limpa.

---

## Verificação (gates) — paridade morfológica, não booleana

```
build: limpo a cada estágio.
suíte (RUST_MIN_STACK=33554432): C0 (2719) ± N. Asserções alteradas SÓ as do == da
  linguagem que a morfologia muda (cada uma justificada pela semântica + vanilla onde
  oráculo). Os testes do PartialEq do Rust intactos. Reportar evoluídas e novas.
lint: crystalline-lint . = 0 violations, 0 warnings.

ACEITAÇÃO (morfológica — ADR-0107; NÃO "o booleano bate"):
  - dois conteúdos de MESMA morfologia e render DIFERENTE → o == da linguagem casa
    (ex.: it.body de `= a` vs [a]; texto igual, bold de heading difere → casa);
  - dois de morfologia DIFERENTE → não casa (ex.: [a] vs [b]; *a* vs a — estilo
    semântico É morfologia, então difere);
  - o estilo semântico (*bold*) permanece observável (== o distingue; #show strong
    intacto) — morfologia, não render.
  it.body == [a] casar é CONSEQUÊNCIA medida, registrada como tal, não o critério.

dois sistemas preservados: o PartialEq do Rust sobre Content/Value inalterado
  (testes de estrutura e coleções/IndexMap não mudam de comportamento). Confirmar.

lente (tekt-cargo-dsm@98d8f9e, --comparar antes/depois): delta de aresta no megaciclo
  ou ~nulo; content→elements 66; elem→elem 0; ciclos [90,4]. Registrar.

perf (par back-to-back): a comparação morfológica vs o == estrutural — esperar ~nulo;
  reportar o par.
```

---

## O que NÃO fazer
- **Não tocar o `#[derive(PartialEq)]` do Rust** sobre `Content`/`Value` — é o outro
  sistema (ADR-0025), para testes/coleções. Só o `==` da linguagem (eval) muda.
- **Não des-assar** o `TextStyle` nem nada — o de-bake é F-5 (render/duplicação),
  concern separado; o `==` morfológico **ignora** o render sem removê-lo.
- **Não consertar a recursão/guard** — só remedir o `m1`; o resíduo é fatia posterior.
- **Não executar variantes AMBÍGUAS** — esperam a decisão do dono na trava.
- **Não medir aceitação pelo booleano** — a métrica é morfológica (ADR-0107).
- **Não confiar em relatório** sobre o que é morfologia/render — **medir da fonte**
  (a tabela da Fase A é da fonte, `file:line`).
- **Não estimar com "~"**.

---

## Relatório (`typst-passo-345-relatorio.md`)
- Fase A: a tabela morfologia-por-variante completa; os AMBÍGUOS (se houver) com a
  pergunta; a posição dos campos de numbering com a medição do vanilla.
- Fase B: o diff por estágio; **onde** o `==` morfológico vive (eval) e a prova de que
  o `PartialEq` do Rust ficou intacto.
- Evolução de testes: cada um, pela justificativa morfológica (vanilla colado onde
  oráculo); os do `PartialEq` intactos.
- O `m1` do P341b remedido (novo resultado; resíduo de guard apontado como fatia
  posterior).
- Aceitação morfológica: os três critérios (mesma morfologia casa; morfologia diferente
  não; estilo semântico observável) com os exemplos medidos. `it.body == [a]` como
  consequência.
- Verificação: suíte, lint, dois-sistemas-preservados, lente, perf.
- **Mapa de filtro (campo) — duas notas:**
  1. **Lugar lógico desta fatia:** o `==` morfológico mora junto da definição de
     igualdade da linguagem (ADR-0025), como a aplicação ao conteúdo do que os números
     já tinham; é a ADR-0107 exercida.
  2. **Nota de filtragem futura (registrar, NÃO executar nesta branch):** a ADR-0107
     deveria ser uma das **primeiras** ADRs (fundadora, ao lado do P329). **Reorganizar/
     renumerar as ADRs** para as fundadoras virem primeiro é **objetivo de melhoria do
     projeto** — mas é **fora desta branch**: mexe em todas as referências cruzadas
     (`ver ADR-00NN`) do repo e quebraria citações no meio do F. Registrado como item
     nomeado do mapa de filtro, para a versão destilada; **não** é ação agora.
- Item aberto carregado: `content→elements → 0` (fora da fila, sem dono).
```
