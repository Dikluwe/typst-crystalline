# Prompt L0 — `compiler/stdlib/structural/title` — `title`
Hash do Código: ae6363e0

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/structural/title.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/stdlib/structural.md` — dono de
`structural/mod.rs` e da história acumulada. Este L0 especifica **a superfície do nó**.
**Convenções partilhadas**: `00_nucleo/prompts/compiler/stdlib/_comum.md`
**Vanilla**: `typst-library/src/model/title.rs` — ficheiro próprio.

**Fronteira medida**: nó de uma nativa só. `native_title` tem **1** commit de corpo real —
`f36ca1abe` (2026-07-16), o que a criou. Os três commits que partilha com `heading`,
`divider` e o núcleo `outline` são `0661aef91` (`cargo fmt`) e os dois renames de módulo.

> Este nó existe por causa de um artefacto: o cluster `native_outline`+`native_title`
> citado como justificação do nó `sectioning` **não existia**. Em `f36ca1abe` o único bloco
> de corpo é `@@ -374,0 +375,64 @@ pub fn native_outline(` — inserção pura de `title`
> depois de `outline`, cujo corpo ficou intacto. Medido e desfeito em 2026-08-13.

**Distinção de pureza — este nó é o único do grupo que lê contexto**: ver Restrições.

---

## Contexto

`title(...)` emite o título do documento como conteúdo. É a única nativa deste grupo que
tem **duas fontes** para o seu corpo: o argumento, ou o metadado `document.title` definido
por `#set document(title: …)`. Essa segunda fonte é o que a torna diferente das vizinhas em
mecânica, não só em superfície.

## Instrução

`title(body?)` — corpo por **named `body`** ou por **1.º posicional**; os dois ao mesmo
tempo são erro (`title(): não pode usar body posicional e named 'body' simultaneamente`).
O named tem prioridade quando o posicional está ausente.

Cadeia de resolução do corpo, nesta ordem:

1. named `body` (`Content | Str`);
2. 1.º posicional (`Content | Str`);
3. metadado `ctx.document_info.title` → `Content::text(t)`;
4. nada disso → `title() exige body ou metadado document.title`.

Tipo errado → `title(body:): espera content ou string, recebeu {tipo}` (named) ou
`title(): body espera content ou string, recebeu {tipo}` (posicional).

Emite `Content::Title(TitleElem::new(body))`.

## Restrições Estruturais

- L1 puro — mas **não pass-through**: `ctx` é usado de verdade (`ctx.document_info.title`).
  É leitura de estado já resolvido do `EvalContext`, sem I/O e sem escrita — classe
  "contexto real" do critério de pureza, medida em `title.rs` na cadeia de resolução do
  corpo. As outras seis nativas do antigo `sectioning`/`flow` são pass-through; esta não, e
  é por isso que a distinção está escrita aqui em vez de no hub.
- A precedência da cadeia é contrato: inverter a ordem (metadado antes do argumento) muda
  comportamento observável do documento.
- Nenhum helper privado.

## Critérios de Verificação

```
#title[T]                                    → Title com body T
#title("T")                                  → body coagido de Str
#title(body: [T])                            → body named
#title[A](body: [B])                          → Err (posicional + named)
#set document(title: "D") + #title()         → Title com body "D"
#title()  (sem body e sem metadado)          → Err "title() exige body ou metadado document.title"
#title(1)                                    → Err "title(): body espera content ou string, recebeu integer"
#title(body: 1)                              → Err "title(body:): espera content ou string, recebeu integer"
```
