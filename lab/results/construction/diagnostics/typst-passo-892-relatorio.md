# Relatório — typst-passo-892: `italic_correction` para `𝑖` — hipótese testada e descartada

**Data:** 2026-07-24T15:20:09Z
**Executor:** Claude (Sonnet 5)
**Commit base:** `ccbe6816c5c26b84a780cc9aa15e2950cd5a6587` (HEAD do ramo `Tekt`; inclui o trabalho de
P891 já commitado nesse commit — `chore: relatórios P889 a P891 e atualizações em layout, math, e
entities`).
**Pré-condição de árvore (exigida pelo passo)**: `git status --short` confirma que o trabalho de
P891 (`font_metrics.rs`, os 5 L0s, etc.) **já estava commitado** antes do início deste passo — nada a
decidir sobre pendência. Working tree no início: `M .gitignore` (não relacionado, pré-existente),
mais os ficheiros untracked deste próprio passo (`typst-passo-892.md`, `typst-passo-893.md` — este
último não lido, fora do âmbito, per a restrição de leitura de `materialization/`) e dois PDFs soltos
(`output_vanilla.pdf`, `test_vanilla.pdf`) não relacionados com este passo, não tocados.

**Passo puramente diagnóstico — Fase A refutou a hipótese antes da Fase B. Nenhum código de produção
foi alterado.**

---

## Fase A — Confirmar se `italic_correction` é não-zero para `𝑖` (U+1D456)

### Ferramenta e método

`fontTools` (Python, v4.63.0, já instalado) em vez de `ttx` diretamente — leitura directa da tabela
`MathItalicsCorrectionInfo` via API estruturada (`Coverage.glyphs` + `ItalicsCorrection[i].Value`),
mais preciso que fazer parsing de XML do `ttx`. Fonte usada: os 3 ficheiros `NewCMMath-{Regular,
Book, Bold}.otf` do checkout local de `typst-assets` (`~/.cargo/git/checkouts/typst-assets-525e6d15e
f7950cb/c0ae970/files/fonts/`) — `c0ae970` é exactamente o `rev` pinado em `Cargo.toml` (`typst-assets
= { git = ..., rev = "c0ae970", features = ["fonts"] }`), confirmando que são bit-a-bit os mesmos
bytes embutidos no binário cristalino (mesmo mecanismo de proveniência usado em P891 para o teste de
`math_kern`).

### Resultado — as 3 variantes

```
NewCMMath-Regular.otf: glyph=u1D456 in_italic_coverage=False value=None
NewCMMath-Book.otf:    glyph=u1D456 in_italic_coverage=False value=None
NewCMMath-Bold.otf:    glyph=u1D456 in_italic_coverage=False value=None
```

`𝑖` (glyph `u1D456`) **não está na tabela de cobertura de `italic_correction`** em nenhuma das 3
variantes — por definição OpenType MATH, um glifo ausente da `Coverage` da `MathItalicsCorrectionInfo`
tem correcção itálica **zero**. Isto vale inclusive para `NewCMMath-Book`, a variante que o vanilla
usa de facto para este benchmark (confirmado em P891, secção "Confirmação visual").

### Sanidade do método (a tabela em si está populada, o zero é real)

Para descartar a hipótese de que a leitura estava simplesmente errada (ex.: API mal usada, tabela
vazia por engano), confirmei contra glifos vizinhos da mesma classe (itálico matemático) na mesma
fonte (`NewCMMath-Book`):

```
total glyphs com italic correction: 1204
f (1D453) -> glyph=u1D453 value=90
D (1D437) -> glyph=u1D437 value=4
j (1D457) -> glyph=u1D457 value=13
x (1D465) -> glyph=u1D465 NOT covered (=0)
```

A tabela tem 1204 entradas populadas (não está vazia), e glifos próximos de `𝑖` na mesma família
visual (itálico matemático de uma letra) têm valores reais não-zero (`f`→90, `D`→4, `j`→13) — a
leitura funciona correctamente. `𝑖` estar genuinamente ausente da cobertura (e portanto com correcção
zero) é um facto da fonte, não um artefacto do método de leitura.

---

## Conclusão — hipótese descartada, per a condição de paragem explícita do passo

O passo instruía: *"Não avançar para a Fase B sem a Fase A confirmar que `italic_correction` é de
facto não-zero para o caso observado"* e *"Se a Fase A refutar a hipótese (valor zero também aqui):
registar isso como achado fechado por descarte, e o gap de `i²` volta a ficar sem causa confirmada —
não inventar uma quarta hipótese sem evidência nova."*

**`italic_correction` para `𝑖` é zero nas 3 variantes de `NewCMMath` embutidas — mesma conclusão que
`kern_infos` (P891).** A hipótese levantada no final de P891 está **descartada por evidência directa**,
não por omissão. Fases B e C deste passo **não foram executadas** — não há implementação, não há
mudança de assinatura de trait, não há novo teste, não há benchmark. Nenhum L0 foi tocado.

### Estado do sintoma `i²`

O gap visual original de `i²` reportado em P889 **continua sem causa confirmada**. Duas hipóteses já
testadas e descartadas (P891: `kern_infos`; P892: `italic_correction`). Por disciplina (per a
instrução explícita do passo), **não se levanta uma quarta hipótese sem evidência nova** — a
investigação deste sintoma específico pára aqui até que surja um indício concreto (nova leitura de
código do vanilla, nova medição) que aponte para outro mecanismo. Fica registado como aberto, não
fechado por adivinhação.

Nota residual (já registada em P891, não re-investigada aqui): a comparação vanilla-vs-cristalino
mais recente (P891) já não reproduzia o gap original de forma alguma — ambos produziam kern/posição
idênticos (zero) para `i²` numa comparação fresca e sincronizada. Isto reforça a possibilidade,
também já registada em P891 e não decidida, de que o sintoma original de P889 tenha sido, em parte
ou no todo, um artefacto de comparação não sincronizada (mesmo padrão dos achados 1/4 de P885) — mas
esta é uma hipótese sobre a *medição*, não sobre o *mecanismo*, e não foi confirmada nem refutada
aqui.
