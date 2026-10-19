# Recon P370 — F-5b: por que a ADR-0026 rejeita o vtable, e a solução que satisfaz a razão dela

**Tipo:** read-only (leitura de ADRs + código + vanilla em `lab/`). **Não decide o conflito;
não escreve código.** Termina numa decisão de modelo **proposta ao dono**. HEAD pós-P369
(3f5008fe3). Suíte 2742, lint 0/0 (não re-rodados — sem código).

> **Veredito antecipado: o conflito é APARENTE, não real.** A ADR-0026 rejeita o **vtable como
> mecanismo** (proc-macro + `unsafe` + dispatch dinâmico), **não** a distinção semântica entre
> elementos. Ao contrário: a própria ADR-0026 **prescreve** `Strong`/`Emph` como **variantes
> distintas do enum**, e a ADR-0105 (modelo D, complementa 0026) confirma "uma variante por
> elemento". O que criou a divergência foi o **colapso do P101** (strong/emph → `Content::Styled`),
> válido sob o critério **funcional** de 0026 da época, mas que virou divergência **semântica** sob
> a 0107 (posterior). A distinção via **variante/discriminante** satisfaz **todas** as cláusulas de
> 0026 e dá à 0107 a sua fidelidade — **sem emenda de ADR** (no máximo uma nota de que o P101 é
> superado). **A decisão é do dono.**

---

## 1. A razão da ADR-0026, decomposta em cláusulas (`file:line`, [medido])

ADR-0026 (`00_nucleo/adr/typst-adr-0026-content-divergencia.md`). O original usa
`struct Content(RawContent)` **com vtable unsafe** + `NativeElement` por proc-macro `#[elem]`
(`:16-17`). "Replicar esta implementação em L1 exigiria" (`:21-25`):

| Cláusula | Citação (`file:line`) | O que proíbe |
|---|---|---|
| **A — sem proc-macro em L1** | "`typst_macros` como dependência de L1 (crate de proc macros)" (`:22`) | a metaprogramação `#[elem]` |
| **B — sem `unsafe` em L1** | "Código `unsafe` em L1 (violação do princípio de domínio puro)" (`:23`) | o fat-pointer/vtable manual |
| **C — sem cadeia prematura** | "Toda a cadeia `NativeElement → Styles → StyleChain` antes de ter sequer texto básico" (`:24-25`) | bootstrapping (histórico) |
| **(decisão)** | "A paridade de **implementação** não é um objectivo — o original usa vtable, o cristalino usa enum" (`:48-49`); "enum linear declarativo … sem a metaprogramação" (`:27-28`) | a **mecânica** vtable, não o conceito |

**A cláusula que NÃO existe:** 0026 **não** proíbe distinguir elementos. Pelo contrário —
**"Implicações futuras"** (`:57-70`) **prescreve**:
```rust
pub enum Content { … Strong(Box<Content>), Emph(Box<Content>), … }   // :63-69 [medido]
```
e "O enum pode crescer linearmente **sem vtable**, mantendo a arquitectura clara" (`:78`). ∴ a
razão de 0026 é **"a distinção sem o mecanismo vtable/macro/unsafe"** — e o seu modelo
**positivo** é exatamente **variantes distintas por elemento**, strong/emph incluídos. [medido]

**Exaustividade do compilador:** a ADR-0105 (`:8` "ADR-0026 … complementada, não revogada";
`:19-31`) adota o **modelo D** (enum fino com delegação, `Nome(Arc<nome::Nome>)`, uma variante
por elemento) e grava a **cláusula 3** (restaurar a exaustividade do compilador, `:65`). ∴ o
`match` exaustivo é um **valor protegido** — o `dyn`/vtable tira-o; o enum/discriminante dá-o.
[medido]

---

## 2. O que a ADR-0107 exige (`file:line`, oráculo vanilla)

ADR-0107 (`typst-adr-0107-…md`): o estilo **semântico** (`*bold*`, `_italic_`) é **distinto** do
estilo **resolvido de render** (`:40-41`); a morfologia é a fronteira conteúdo-de-linguagem vs
render (`:42-46`, `:115`). A paridade mede-se por **semântica/sintaxe/morfologia**, com o
**vanilla como oráculo** quando a fonte é ambígua (`:55-57`). E **a própria 0107 difere o
conserto**: "o `==` … devia ser semântico/morfológico. **Esse conserto é um passo seguinte, não
esta ADR**" (`:118-120`). [medido]

**O oráculo vanilla (P366, re-confirmado):** `StrongElem`/`EmphElem` são **elementos distintos**
(`lab/.../model/strong.rs`, `emph.rs`); `StyledElem::eq` compara só o `child`
(`content/mod.rs:763`); `Packed::eq` compara id de elemento (`packed.rs:144`). ∴ no vanilla
`*bold* X ≠ _italic_ X ≠ #set text X ≠ X` — **distinção semântica observável por TIPO de
elemento**, não por vtable (o vtable é só a mecânica Rust do original). [medido]

---

## 3. O P101 (o colapso) e o critério da época (`file:line`)

No código: `Content::strong(body)` = `Content::Styled([Style::Bold(true)], body)`
(`content.rs:1040`); `Content::emph` = `Styled([Italic(true)])` (`:1047`) — strong/emph
**colapsados** em `Content::Styled`, sob a **ADR-0038** (sistema de estilos L1; "5 L1
consolidados em 1", `adr-0038:340`; `Bold(bool)=text.bold`, `Italic(bool)=text.italic`,
`:60-61`). [medido]

**Critério da época:** sob a 0026, a paridade é **funcional** ("mesmo output de texto para o
mesmo input", `0026:47-48`, `:100-101`). Colapsar strong/emph em `Styled[Bold/Italic]` produz o
**mesmo output** (texto bold/italic) → **válido sob 0026** (funcional). A 0107 (paridade de
**linguagem**, posterior) torna-o **divergência semântica** (`strong` deixou de ser distinguível
de `#set text(weight:bold)`). [medido] **Datação:** 0026 = 2026-03-27 (`0026:9`); 0107 é
posterior (fecha "a outra metade" de 0033/P342/P329, `0107:22-24`). O colapso **antecede** a
0107 — não a violou quando foi feito; tornou-se divergência **retroativamente**. [medido]

---

## 4. Tabela: candidato de modelo × cláusula da 0026 × impacto no α

| Candidato | A (proc-macro) | B (`unsafe`) | C/exaust. (match) | Clareza | Impacto no α-fixpoint |
|---|---|---|---|---|---|
| **(1) Variantes próprias** `Content::Strong(Box)`/`Emph(Box)` (reverte P101; = modelo prescrito por 0026 `:63` + 0105-D) | **não viola** (enum) | **não viola** | **preserva/melhora** (match exaustivo, 0105 cl.3) | **é o modelo 0026/0105** | morph_canon ganha arms Strong/Emph **mantidos** (morfologia), equivalente ao atual `Styled[Bold]` mantido → α inalterado [inferido] |
| **(2) Discriminante em `Styled`** `Styled{ kind: StyleKind, … }` | não viola | não viola | preserva (match no discriminante) | medir (menos limpo que (1)?) | idem (1) [inferido] |
| **(3) vtable/`dyn`** (como o original) | **VIOLA A** | **VIOLA B** | **perde** (sem match) | viola | n/a — rejeitado por 0026 |

**Leitura:** os candidatos **(1)** e **(2)** — a distinção por **tipo estático** (variante ou
discriminante) — **não violam nenhuma cláusula de 0026**; o candidato **(3)** (o vtable) é o único
que viola, e é o que 0026 rejeitou. ∴ **a distinção semântica que a 0107 exige NÃO precisa do
vtable** — obtém-se pelo enum, que é o modelo de 0026/0105. [medido para A/B/exaustividade;
inferido para o α]

**O α (limite duro do P366):** o `morph_canon` serve dois consumidores — o `==` da linguagem
(`operators.rs:79`) e o α-fixpoint do `#show` (`rules.rs:229`). Distinguir strong/emph como
variantes próprias faz o `morph_canon` **mantê-las** (morfologia), **como já mantém** o
`Styled[Bold]` hoje (não-vazio). A **auto-igualdade** (`Strong(x) canon == Strong(x) canon`)
preserva-se → a terminação do α preserva-se. **A distinção em si não reabre o caso 2.** [inferido;
refutável por: implementar (1) e rodar os ~testes do caso 2 / a rede +11]. **Atenção:** o
**de-bake do render `#set text`** (a OUTRA metade do F-5b) é o que toca o `custom`-vs-tipado do
`morph_canon` (P366 #1) — **separável** do candidato de distinção; é sub-problema próprio.

---

## 5. Veredito e decisão proposta ao dono

**Veredito: conflito APARENTE.** A 0026 nunca proibiu a distinção semântica strong/emph/styled —
proibiu o **vtable** (proc-macro+`unsafe`+dispatch). A distinção por **variante/discriminante**
(candidatos 1/2) satisfaz **todas** as cláusulas de 0026 e **é o modelo que 0026 (`:63`) e 0105-D
prescrevem**. A divergência atual nasceu do **colapso P101** (válido sob a paridade *funcional*
de 0026; divergência *semântica* sob a 0107 posterior). A 0107 já marcou o conserto como "passo
seguinte" (`:120`). **Sem emenda de ADR** — no máximo uma **nota** em 0038/no DEBT de que o
colapso P101 é **superado** pelo retorno ao modelo de variantes (0026 `:63` / 0105-D).

**Recomendação marcada:** o F-5b é viável como **lote arquitetural multi-fatia**, fiel à 0107 e à
0026:
1. **Distinção de tipo** — strong/emph deixam de ser `Styled[Bold/Italic]` e voltam a **variantes
   próprias** (`Content::Strong`/`Emph`, modelo D/0105) **ou** um discriminante em `Styled`. Dá à
   0107 a fidelidade (`*bold* ≠ _italic_ ≠ #set text`). Candidato **(1)** é o mais aderente
   (modelo 0026/0105). [medir clareza/custo-por-variante na Fase A do lote]
2. **De-bake do render `#set text`** (a outra metade, P366) — o estilo de render viaja
   transparente à morfologia; o `morph_canon` distingue render (transparente) de morfologia
   (Strong/Emph mantidos). **Separável** da fatia 1.
3. **Verificação do α** — provar (rodar caso 2 + rede +11) que a distinção não muda a terminação.

**Alternativa (se o dono não quiser o lote agora):** registrar o F-5b como **divergência
consciente medida** no DEBT-61 (como o DEBT-60 (a) fez para o contador) — `*bold*`/`#set text`
conflatados em `Styled[Bold]`, divergência da 0107 conhecida, com o porquê (custo do lote
arquitetural vs demanda observável nula até aqui). **A decisão é do dono.**

---

## Estado / limites

Read-only: nenhum código de produto, nenhum L0 funcional tocado (só este diagnóstico). α/caso 2,
`morph_canon`/`==`, caso 4, flag P350c, Marco G, os 3 numbering, o `#set` de props de usuário
(P368) — **não tocados**. Árvore limpa fora deste doc. lente: content→elements = 66 (instrumento;
não re-corrida — sem código). **Termina aqui — não emenda o passo seguinte (Trava 5); a decisão
do conflito/execução é do dono.**

## Fora de escopo

A **execução** do F-5b (nasce da decisão do dono); o **Marco G** (não-F; também toca a 0026, mas é
decisão separada); DEBT-59 (flag CLI); DEBT-60 (contador).
