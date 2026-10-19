# Passo 909 — atomizar `layout_accent`/`layout_cancel`/`layout_underover`/`layout_op` para fora de `mod.rs`

**Precede este passo**: `00_nucleo/adr/typst-adr-0109-atomizacao.md` (definição de atomização,
Forma B) e `00_nucleo/prompts/rules/math/layout/_comum.md` (núcleo partilhado do subsistema math).
Ler os dois antes de começar — este passo replica o padrão já aplicado a `frac.rs`/`root.rs`/
`stretchy.rs`/`assembly.rs`/`matrix.rs`/`cases.rs`/`delimited.rs`, não inventa um novo.

**Este é um passo de refactor puro — content-preserving (ADR-0107). Nenhuma mudança de
comportamento é esperada nem aceite.** Se qualquer teste mudar de resultado durante este passo,
parar e investigar antes de continuar — isso significaria que a lógica não foi só movida, foi
alterada.

**Pré-condição de árvore**: `git status`. Confirmar estado de P893 (Fase B) e do restante backlog
(P906-908 se ainda não commitados) — este passo mexe em `math/layout/mod.rs`, que P906 também
tocou (adicionou `layout_stretchy_or_node`, dispatch de `underbrace`/etc.) — confirmar que não há
conflito de merge antes de começar.

---

## O que está fora do padrão (medido, não presumido — confirmar antes de mover)

`math/layout/mod.rs` contém hoje, ao lado do núcleo partilhado (`MathLayouter`, `layout_node`,
`layout_equation`, `hconcat`, etc.), quatro handlers que deveriam ter seguido para arquivos
próprios quando `frac`/`root`/`stretchy`/`assembly`/`matrix`/`cases`/`delimited` foram fatiados em
P314 (ADR-0104), mas ficaram para trás (adicionados depois, em P296-298, "em paralelo a
`layout_frac`", sem nunca serem movidos):

- `layout_accent` (`Content::MathAccent`, P296)
- `layout_cancel` (`Content::MathCancel`, P296)
- `layout_underover` (`Content::MathUnderover`, P297) — inclui o dispatch de
  `underbrace`/`overbrace`/`underbracket`/`overbracket` adicionado por P906, e o esticamento
  horizontal (`layout_stretchy_or_node`, também P906)
- `layout_op` (`Content::MathOp`, P298)

## Fase A — confirmar antes de mover

1. Ler os quatro handlers directamente no `mod.rs` actual (linhas exactas, não presumir a partir
   dos prompts L0 antigos de P296-298, que podem estar desatualizados face ao código real pós-
   P906/907) — confirmar exactamente o que cada um consome do `MathLayouter`/`FontMetrics`.
2. Confirmar dependências cruzadas entre os quatro (por exemplo, `layout_underover` chama
   `layout_stretchy_or_node`, que também é usado por `layout_accent` per P906 — "guard de
   1-carácter partilhado" — confirmar se esse guard deve virar uma função em `_comum.md`/`mod.rs`
   partilhada pelos dois arquivos novos, ou se cada um a duplica) — decidir e registar antes de
   mover, mesmo padrão de decisão explícita já usado nos passos anteriores.
3. Seguir a Forma B (ADR-0109): free function na camada de render, arm magro no `mod.rs`
   (`Content::MathAccent{..} => accent::layout(self, ...)`), sem `dyn`, sem import reverso.
4. Confirmar se `layout_op` tem alguma interacção especial com `MathAttach` (o L0 antigo de P298
   menciona "cross-variant interaction" como a verdadeira inovação desse passo) que precise de
   ficar documentada explicitamente no novo `op.md`, não perdida na mudança de arquivo.

## Fase B — implementação

1. Criar `01_core/src/rules/math/layout/{accent,cancel,underover,op}.rs`, cada um com a lógica
   movida (não reescrita) do handler correspondente, seguindo a forma B.
2. Criar os L0s correspondentes (`00_nucleo/prompts/rules/math/layout/{accent,cancel,underover,
   op}.md`), mesmo formato dos já existentes (`frac.md`/`root.md`/etc.: cabeçalho, camada, alvo,
   origem "fatiado de `math/layout/mod.rs` neste passo (P909), completando o padrão de P314", link
   para `_comum.md`, critério de uma linha).
3. `mod.rs` fica só com os arms magros de despacho para os quatro, mais o núcleo partilhado que já
   lá estava.
4. Suíte completa verde, discriminada por crate — **sem nenhuma mudança de resultado esperada**,
   só de organização.
5. `crystalline-lint --fix-hashes .` — sincronizar hashes dos ficheiros novos e do `mod.rs`
   alterado. `crystalline-lint .` — confirmar 0 drift novo (o V7 pré-existente de
   `package_version_resolution.md` continua não relacionado).
6. Recompilar o `.typ` de 30 secções (mesmo hash de sempre) e confirmar **byte-a-byte idêntico** ao
   PDF gerado antes deste passo (não só "sem regressão visual" — comparar hash do PDF de saída
   antes/depois, já que isto é refactor puro, sem mudança de layout esperada).

## Fase C — Regressão

Benchmark completo, 7 cenários, `--warmup 5 -m 20`. Não se espera diferença (é reorganização de
código, não lógica nova) — medir por disciplina de qualquer forma.

## Resultado esperado

- 4 arquivos novos (`accent.rs`/`cancel.rs`/`underover.rs`/`op.rs`) + 4 L0s novos.
- `mod.rs` reduzido, só núcleo partilhado + arms de despacho.
- PDF de saída do `.typ` de 30 secções **byte-idêntico** ao gerado antes deste passo — prova mais
  forte de "content-preserving" do que só "suíte verde".
- `crystalline-lint`: 0 drift.
- Relatório registando a decisão da Fase A ponto 2 (guard partilhado — onde vive).

---

## Registrado para depois, não para este passo

O dono pediu para, **depois** deste passo, pensar numa nova engenharia de testes de layout —
algo que compare posição/geometria de glifos entre cristalino e vanilla directamente (via
`pdftotext -bbox` ou equivalente) de forma automatizada, em vez de depender de leitura de código ou
de extração de texto simples para decidir se algo está certo. Não desenhar isso agora — só marcar
como próximo passo depois de P909 fechar.
