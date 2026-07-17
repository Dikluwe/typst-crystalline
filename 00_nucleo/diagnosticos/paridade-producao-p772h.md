---
# Diagnóstico — P772h: arqueologia de `header:`/`footer:` inventados e do código órfão de `align`+`place`

> **Passo:** 772h
> **Data:** 2026-07-16
> **Tipo:** Sonda arqueológica. Sem implementação — decisões ficam para depois.
> **Dependências:** P772f (achados originais, commit `2c7025a9a949ecafda8a2550c8c3692db3c9ad0b`).

---

## Parte A — Origem de `header:`/`footer:` como parâmetros nomeados

### Commit identificado

```
git log --all --oneline -S '"header"' -- 01_core/src/engine/stdlib/layout.rs
git log --all --oneline -S '"footer"' -- 01_core/src/engine/stdlib/layout.rs
```

Ambos apontam para um único commit: **`0af55e792` — "Passo 222 - 224"** (13 maio 2026).
Não há nenhum outro commit que toque `_header`/`_footer` em `01_core/src/engine/layout/grid.rs`
além deste e de `a5bdb96ae` (P380, atomização — apenas moveu o código de `mod.rs` para
`grid.rs`, sem alterar a semântica; confirmado via `git show a5bdb96ae -- grid.rs` — só
a linha `e.header.as_ref(), e.footer.as_ref()` foi deslocada).

### O que o commit `0af55e792` realmente fez (não é "sem justificação" — é uma
inconsistência interna entre plano e implementação, dentro do mesmo commit)

O ficheiro de planeamento desse commit,
`00_nucleo/materialization/typst-passo-224.md` (secção C4), **especifica correctamente**
a extracção de header/footer a partir dos filhos do `grid()`, não de argumentos
nomeados:

```rust
// P224.B — extract header/footer from children Vec.
let (header, footer, cells) =
    split_header_footer(children_resolved)?;
```

Isto está alinhado com "Decisão 1 — Opção α" do mesmo documento (variants `Content`
`GridHeader`/`GridFooter`, paridade vanilla literal — vanilla usa `grid.header(...)`
como elemento-filho, nunca como argumento nomeado; confirmado contra
`lab/typst-original/crates/typst-library/src/layout/grid/mod.rs:580,608`,
`#[elem(name = "header")]`/`#[elem(name = "footer")]`).

**Mas o mesmo documento de planeamento, na mesma secção C4, contradiz-se a si próprio**:
a lista de argumentos nomeados aceites por `grid()` inclui `"header", "footer"` lado a
lado com `"gutter", "align", "inset"`:

```rust
if !["gutter", "align", "inset", "header", "footer"].contains(&key.as_str()) {
    return Err(...)
}
```

`split_header_footer` **nunca existiu no código** (`git log --all -S
"split_header_footer" -- .` só encontra o próprio texto do plano, nunca uma
implementação). O que foi implementado — e é o código ainda em vigor hoje, verificado
com `git show 0af55e792 -- 01_core/src/engine/stdlib/layout.rs` — foi o caminho mais
simples sugerido pela própria lista contraditória: tratar `header`/`footer` como mais
dois argumentos nomeados de `Value::Content`, no mesmo padrão de `gutter`/`align`/`inset`:

```rust
// P224.B — extract header/footer (Content opcional).
let header = match args.named.get("header") {
    Some(Value::Content(c)) => Some(Box::new(c.clone())),
    ...
```

Ou seja: **não foi uma decisão deliberada e justificada** de divergir do vanilla — foi
uma inconsistência já presente no próprio plano (a secção de extracção correcta e a
whitelist de argumentos nomeados discordam entre si), e a implementação seguiu o
caminho mais simples da whitelist em vez do `split_header_footer` mais trabalhoso, sem
que o relatório de fecho assinalasse a divergência como intencional.

### O relatório de fecho da época já documentava o gap como conhecido, não como bug novo

`00_nucleo/materialization/typst-passo-224-relatorio.md:117` (mesmo commit) regista
explicitamente:

> `_align`/`_inset`/`_header`/`_footer` ignorados graded

Isto confirma que, em Maio de 2026, quem fechou P224 **sabia** que o motor de layout
não lia `header`/`footer` — classificou como "graded" (simplificação deliberada,
destinada a ser retomada depois). O mesmo relatório também lista, na mesma secção
(§4), os variants `Content::GridHeader`/`GridFooter` como "Decisão 1 — Opção α"
implementada — ou seja, **as duas vias (variants correctos + argumento nomeado
inventado) foram introduzidas na mesma sessão**, aparentemente sem que se tenha notado
que geram dois caminhos paralelos e nenhum dos dois fica completo: o caminho
correcto (`grid.header(...)` como filho) cai num braço genérico do loop de
`grid()` que não distingue `Content::GridHeader` de uma célula normal (achado #16
de P772f); o caminho inventado (`header:` nomeado) fica plumbed até `GridElem` mas
nunca lido pelo motor de layout.

### Nenhum passo posterior retomou o gap

`git log --all --oneline -S "header" -- 01_core/src/engine/layout/grid.rs` só devolve
os dois commits já citados (`0af55e792`, `a5bdb96ae`). Nenhum passo entre P224 (maio)
e P772f (este mês) tentou ligar `_header`/`_footer` ao motor de layout, nem removeu o
argumento nomeado inventado.

### Tabela de rastreabilidade (regra 9 — registo, sem reescrever)

| Passo (numeração da época) | Commit | O que decidiu (de facto) |
|---|---|---|
| P224 (`typst-passo-224.md`, secção C4) | `0af55e792` | Planeou `split_header_footer` a partir dos filhos (correcto), mas a mesma secção listou `header`/`footer` na whitelist de argumentos nomeados — inconsistência já no plano. |
| P224 (implementação, `stdlib/layout.rs`) | `0af55e792` | Implementou o caminho mais simples da whitelist: `header`/`footer` como `args.named.get(...)`, nunca `split_header_footer`. |
| P224 (`typst-passo-224-relatorio.md:117`) | `0af55e792` | Registou explicitamente `_header`/`_footer` como "ignorados graded" no motor de layout — gap conhecido, não descoberto agora. |
| P380 (atomização) | `a5bdb96ae` | Moveu `layout()`/`layout_grid` de `mod.rs` para `grid.rs` (ADR-0109); `_header`/`_footer` deslocados sem alteração semântica. |
| P772f | `2c7025a9a` | Redescobriu o gap (não sabia da nota "graded" de P224 na altura), classificou como achado incidental, não corrigiu. |
| P772h (este passo) | — | Arqueologia — confirma que o gap é conhecido desde P224, nunca retomado; a inconsistência de origem está no plano de P224, não numa decisão isolada posterior. |

---

## Parte B — Origem do código órfão não commitado (`Content::Place` a envolver
`effective_align` em `grid.rs`)

### Verificação directa

```
git stash list
```
→ **1 stash**: `stash@{0}: On Tekt: P571: salvar working tree para testes de
isolacao de causa`. Isto é de um passo muito anterior (P571) e não tem relação de
conteúdo com o bloco `P772f — aplicar align efectivo`; `git stash show -p` não foi
necessário para o descartar — a mensagem do stash já identifica um propósito e uma
época diferentes.

```
git fsck --no-reflog --unreachable --dangling | grep "^dangling commit"
```
→ **nenhum resultado**. Não há nenhum commit "perdido" (feito e depois desfeito por
`reset`) que contenha este código — confirma que o bloco **nunca foi commitado em
nenhum momento** nesta repo (não é um caso de "commit + reset --soft/mixed" a
apagar o histórico).

```
git reflog --all | head -50
```
→ mostra apenas a sequência normal de commits da linha P767–P772e; nenhuma entrada de
`stash` ou `checkout` que sugira uma sessão paralela a introduzir e depois abandonar
este código por via de reflog.

### Evidência circunstancial (não é prova por git, mas é directa e consistente)

O próprio bloco de código já se auto-refere:

```rust
// P772f — aplicar align efectivo da célula (herdado do Grid
// ou override per-cell) envolvendo o body num Place in-place
// (float: false, scope: Column) para ancorar ao bbox da célula.
```

E o cabeçalho do ficheiro `00_nucleo/materialization/typst-passo-772f.md` — o mesmo
documento que dirigiu a execução de P772f — dizia explicitamente, antes de ser reescrito
no início da execução de P772f:

> **Passo:** 772f (continuação da série P765a→P772e; **substitui/valida a versão
> auto-gerada por Claude Code**)

Estas duas peças, lidas juntas, apontam para a mesma conclusão sem precisar de
inferência adicional: **uma sessão anterior de Claude Code já tinha começado a
trabalhar em P772f** (ao ponto de escrever código que já se rotula "P772f") antes da
sessão que reescreveu `typst-passo-772f.md` e o executou como o P772f que gerou o
relatório `paridade-producao-p772f.md`. Essa sessão anterior parece ter começado a
implementar per-cell alignment via `Content::Place`, mas **nunca chegou a: (a)
completar/validar o mecanismo, (b) escrever ou actualizar um L0 correspondente em
`00_nucleo/prompts/engine/layout.md`, nem (c) commitar o trabalho** — ficou como
alteração pendente no working tree, e a sessão seguinte (que reescreveu o `.md` como
"substitui a versão auto-gerada") tratou o **documento** de coordenação como
substituído, mas não removeu nem revisitou o **código** já deixado no working tree.

### Conclusão explícita da Parte B

A origem exacta (que sessão, quando, com que prompt do operador) **não é rastreável**
por git — não há commit, não há stash correspondente, não há entrada de reflog. A
única fonte é a auto-referência textual no comentário do próprio código, corroborada
pela nota de substituição no cabeçalho do `.md`. Isto é suficiente para situar o código
no tempo (sessão de P772f anterior à que fechou o passo), mas não para recuperar a
intenção completa ou validar se o mecanismo tinha uma direcção de desenho acordada
com o humano.

**Consequência para a decisão de mérito (P772f §3.3):** como a intenção original não é
recuperável com certeza, a decisão sobre reverter ou formalizar este código **tem de
assentar apenas no mérito observável hoje** — e esse mérito já foi medido em P772f:
diverge do vanilla em `#grid(align: center, ...)` por ~13.5pt, não tem L0
correspondente, e não foi coberto por nenhum teste. Nenhum facto novo desta arqueologia
muda essa medição; apenas confirma que não há contexto adicional escondido em git que a
contradiga ou justifique.

---

## Critério de fecho — checklist

- [x] Commit de introdução de `header:`/`footer:` como argumentos nomeados
      identificado (`0af55e792`, "Passo 222 - 224").
- [x] Tabela de rastreabilidade da Parte A preenchida, sem reescrever passos antigos.
- [x] Origem do código órfão da Parte B investigada em todas as fontes plausíveis
      (stash, reflog, dangling commits, timestamps, comentário completo).
- [x] Não encontrada por git — registado explicitamente como não rastreável por essa
      via; evidência textual (comentário + cabeçalho do `.md`) situa a origem numa
      sessão de P772f anterior à que fechou o passo, mas não recupera a intenção de
      desenho completa. Essa conclusão não substitui a decisão de mérito já registada
      em P772f — apenas informa que não há contexto adicional em git que a mude.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p772h.md`.

---

## Próximo passo

Com a arqueologia feita:
- **`header:`/`footer:` nomeados**: o gap é conhecido desde P224 (maio 2026), não uma
  regressão recente. Decisão pendente (fora do âmbito deste passo): remover o
  parâmetro nomeado (bater com vanilla, que rejeitaria `#grid(header: ..)`) ou
  implementar finalmente `split_header_footer` como o plano original de P224 já
  especificava — o que também resolveria o scope-out #16 de P772f (`ResolvableGridChild`)
  ao mesmo tempo, já que ambos os gaps partilham a mesma causa-raiz (loop de `grid()`
  não distingue `Content::GridHeader`/`GridFooter` de células normais).
- **Código órfão de `align`+`place`**: sem intenção de desenho recuperável e sem L0.
  Decisão pendente (fora do âmbito deste passo): reverter (mais seguro, dado
  divergir do vanilla e não ter testes) ou abrir um passo dedicado que redija o L0
  primeiro e refaça a implementação com testes, seguindo o Protocolo de Nucleação.
- Retomar a varredura da stdlib (`visualize::image::svg`, `foundations::scope`,
  `text::font::*`) depois de fechadas as duas decisões, ou em paralelo com P772g
  (correcção de `layout_place`), que é independente destes dois achados.
