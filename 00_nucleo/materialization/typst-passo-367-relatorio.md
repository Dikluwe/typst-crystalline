# Relatório P367 — F-6 (3 folhas): medido como **sem de-bake** (folhas de math já vanilla-faithful)

> **Desfecho.** A Fase A (fronteira-F-5b + varredura + oráculo vanilla) mede que **F-6 não tem
> de-bake de fonte única a fazer**: `MathText`/`MathIdent` são **folhas nuas** (sem campo de
> estilo assado; estilo do contexto/chain) — **já fonte única por construção**, e o vanilla
> confirma o mesmo modelo (`#[ghost]` na `StyleChain`); `Content::Text` é a única folha com
> estilo **assado**, mas isso é o **F-5b** (adiado, DEBT-61). **F-6 fecha sem código** —
> consistente com a auditoria P362 ("F-6 sem divergência observável"). **Nenhum `.rs` tocado.**

**HEAD**: pós-P366 (b74f901bd). **Branch**: Tekt. Justificativa = princípio (fonte única).

---

## Fase A — fronteira-F-5b primeiro, depois a varredura (medida; `file:line`)

**As 3 folhas (`content.rs`):**

| Folha | Def | Estilo | Veredito F-6 |
|---|---|---|---|
| `Content::Text` | `Text(EcoString, **TextStyle**)` (`:122`) | **assado** | **É o F-5b** (adiado, DEBT-61) → **fora do F-6** (regra de fronteira) |
| `MathText` | `MathText(EcoString)` (`:170`) | **nenhum campo** | **nua — sem caminho duplo** |
| `MathIdent` | `MathIdent(EcoString)` (`:167`) | **nenhum campo** | **nua — sem caminho duplo** |

**Fronteira-F-5b:** o de-bake de uma folha tocaria o `TextStyle`/`morph_canon` que o F-5b adiou?
- `Content::Text` → **é** o `TextStyle` adiado → fora do F-6.
- `MathText`/`MathIdent` → **nuas**, sem campo de estilo; o estilo vem do **contexto de math**
  (`MathStyled` transforma o glyph, `math/layout/mod.rs:711-723`), não de campo da folha →
  **não tocam** o bloqueio do F-5b, mas **também não têm caminho duplo a remover**.

**Veredito da varredura: não há de-bake.** As folhas de math já são fonte única (nuas +
contexto); a única folha assada é o F-5b.

---

## Oráculo vanilla (re-medição pedida pelo dono; `lab/`)

Confirma que as folhas de math nuas são **vanilla-faithful**, não uma lacuna:
- Math text do vanilla = `TextElem` (`math/equation.rs:177` `TextElem::packed`); layout
  `typst-layout/math/text.rs:18`: `layout_text(text, styles: StyleChain)` → `TextElem::packed(text)`
  (só a string) estilizado **pela chain**.
- `TextElem` (`text/mod.rs:95+`): campos de estilo (`font`/`size`/`fill`/`weight`/…) são
  **`#[ghost]`** — **resolvidos da `StyleChain`, não guardados no elemento**.
- ∴ folha de math vanilla = **string nua + estilo-da-chain** = **idêntica** ao
  `MathText`/`MathIdent` nu do cristalino. **Sem divergência observável.**
- **Bônus:** o `TextStyle` **assado** do `Content::Text` é justamente a divergência do modelo
  `#[ghost]` do vanilla → **confirma** que o F-5b (não o F-6) é o trabalho real. Reforça DEBT-61.

Dar às folhas de math os "campos do vanilla" seria **adição de feature** (e o vanilla os põe na
chain, não no elemento) — **não** de-bake de fonte única, e sem divergência observável. Fora do F-6.

---

## Estado / gates

```
build/suíte/lint: INALTERADOS — nenhum .rs tocado. Suíte 2738, lint 0/0.
ACEITAÇÃO: F-6 não tem alvo de de-bake (medido) — folhas de math já fonte única (nuas +
  chain/contexto, confirmado vs vanilla #[ghost]); Content::Text é F-5b (adiado).
INTACTOS: TextStyle/F-5b (não tocado); α/caso 2, morph ==/morph_canon, caso 4, flag P350c,
  Marco G; os 3 numbering (P364/P365). Tudo intacto (nenhum código).
lente/perf: n/a (sem código).
L0: f_fronteira_e1.md §3b.7 (nota F-6 + confirmação vanilla); hash sincronizado.
commit: só L0 + relatório (padrão medido-sem-trabalho).
```

**Tocados:** `f_fronteira_e1.md` (§3b.7 nota F-6) + backings (hash sync); este relatório. Nenhum `.rs`.

**Item (2) coberto** (medido: as folhas que o F-6 alcançava já são fonte única; a que não, é F-5b).

**Próximo (decisão do dono):** **item (3) — `#set <elemento-de-usuário>(prop:)`** (a
extensibilidade que depende da chain como fonte única, já estabelecida). O `TextStyle`/F-5b
permanece como lote arquitetural dedicado (DEBT-61). **Termino aqui — não emendo o seguinte
(Trava 5).**
