# L0 — Passo 1072: Default de `body_indent` — `0pt` → `0.5em` em `list`/`enum`

**Gate**: `ADR-0127` — mudança de comportamento por defeito (todo documento com
lista ou enumeração sem `body-indent` explícito é afectado). **Requer confirmação
do dono antes de codificar.**

**Base**: P1031, Achado #8 (citação literal confirmada no P1071): "vanilla `• Um`,
cristalino `•Um`; com `body-indent: 0.5em` explícito coincidem" — prioridade
Média. Prompts relevantes já identificados: `compiler/layout/list_item.md`,
`compiler/layout/enum_item.md`, `entities/elements/enum_item.md`.

---

## 1. Ler antes de codificar (Manifesto — prompt antes de código)

Não tenho, nesta conversa, o conteúdo real de nenhum dos três prompts citados
acima, nem de `list_item.rs`/`enum_item.rs`. Pedir:

- `00_nucleo/prompts/compiler/layout/list_item.md`
- `00_nucleo/prompts/compiler/layout/enum_item.md`
- `00_nucleo/prompts/entities/elements/enum_item.md`
- `01_core/src/compiler/layout/list_item.rs`
- `01_core/src/compiler/layout/enum_item.rs`

Confirmar, a partir destes, onde exactamente o default `0.0pt` está definido —
o P1031 aponta os arquivos, mas não a linha exacta.

## 2. Verificar se `TermItem` tem o mesmo padrão

O P1031 só lista `list`/`enum` no Achado #8, não `term` (glossário/definição).
**Não presumir que `TermItem` está isento** só por não estar no achado original —
pode ser que o P1031 simplesmente não o tenha testado. Verificar
`entities/elements/term_item.md`/`.rs` (se existir) e medir separadamente. Se
`TermItem` também divergir, isso é achado novo — registar como tal, não misturar
silenciosamente no mesmo passo sem dizer.

## 3. Medição antes de fixar o valor exacto

O P1031 já tem evidência de que `0.5em` explícito faz o cristalino bater com o
vanilla — mas confirmar isto directamente antes de codificar, mesma disciplina já
usada em todos os passos anteriores desta conversa (nunca aceitar valor sem
número real):

```
#set list(body-indent: auto)  // ou omitir, para ver o default puro
- Um
- Dois
```

Medir `pdftotext -bbox-layout` nos dois binários, confirmar Δx do início do texto
"Um"/"Dois" face ao marcador. Repetir para `enum`.

## 4. Mecanismo (a confirmar contra o código real, §1)

Alterar o valor por defeito de `body_indent` de `Length::pt(0.0)` para o
equivalente de `0.5em` — resolvido contra o tamanho de fonte activo no ponto de
layout (não um valor absoluto fixo em pt, já que `em` escala com o texto). Forma
exacta depende do que `list_item.rs`/`enum_item.rs` real mostrar — não presumir a
partir do padrão de outro arquivo (`heading.rs`, por exemplo) sem confirmar que
`body_indent` já não tem alguma resolução própria diferente.

## 5. Critérios de verificação

1. Lista sem `body-indent` explícito — Δx alvo: 0.0000pt (mesma disciplina de
   paridade sub-pixel já usada em todo o resto desta conversa).
2. Enumeração sem `body-indent` explícito — mesmo critério.
3. Lista/enumeração **com** `body-indent` explícito diferente de `0.5em` —
   confirmar que o valor explícito do usuário continua a ter precedência (não
   regressão do caso já funcional, citado no próprio P1031: "com `body-indent:
   0.5em` explícito coincidem").
4. `TermItem`, se aplicável (§2) — mesmo critério, ou registado como scope-out
   explícito se não for tocado neste passo.
5. `crystalline-lint .` — 0 erros.
6. `cargo test --workspace` — 100% pass.

## Critério de conclusão

- Arquivos do §1 lidos, linha exacta do default confirmada.
- §2 respondido (TermItem incluído ou explicitamente scope-out).
- Medição do §3 feita antes da mudança de código.
- 6 critérios de verificação do §5 confirmados.
