# L0 — Passo 1077: `len` Global — Extensão do Cristalino, não Bug de Valor — Achado #12 do P1031

**Gate**: `ADR-0127` — mas de natureza diferente dos passos anteriores desta
fila. **Requer decisão do dono, não só confirmação de código.**

**Base**: P1031, Achado #12 (citação literal confirmada no P1071): `#len("ação")`
→ vanilla `error: unknown variable \`len\``, cristalino `4` (conta codepoints).
`"ação".len()` (método) → ambos `6` (conta bytes) — o método já bate com o
vanilla; só a função global é a diferença. Código citado:
`01_core/src/compiler/eval/mod.rs:1471`,
`01_core/src/compiler/stdlib/foundations/len.rs:19`.

---

## 1. Isto não é o mesmo tipo de achado que P1072-1076

Todos os passos anteriores desta fila corrigiram um **valor errado** para um
comportamento que já devia existir (indentação, supplement, itálico, captura de
regex, arredondamento). Este é diferente: `len` como função global **não existe
no vanilla de todo** — não é um valor divergente de algo que os dois têm, é uma
funcionalidade que só o cristalino tem.

Isso muda a natureza da decisão. Não é "consertar para bater com o vanilla" —
é escolher entre:

**(a) Remover `len` global** — paridade estrita. `#len("ação")` passa a dar erro
`unknown variable`, igual ao vanilla. Risco: se algum documento real já usa
`len` global (mesmo sendo extensão não-oficial), isto quebra esse documento —
mudança que remove funcionalidade, não só corrige comportamento.

**(b) Manter `len` como extensão documentada** — per `ADR-0107` ("paridade é
comportamental, não estrutural"), uma função adicional que não colide com nada
do vanilla (o identificador `len` não existe lá, logo não há ambiguidade nem
sombra de significado) pode ser aceitável como extensão intencional, desde que
**documentada como tal**, não escondida como se fosse comportamento padrão do
Typst.

**Este L0 não recomenda uma das duas** — é decisão do dono, não uma correcção
técnica onde só uma resposta é "certa".

## 2. Se a decisão for (a) — remover

Ler antes de codificar: `00_nucleo/prompts/compiler/stdlib/foundations.md` (ou
o prompt real, a confirmar caminho), `01_core/src/compiler/eval/mod.rs`,
`01_core/src/compiler/stdlib/foundations/len.rs`.

Verificar se `len` global é usado em algum lugar do próprio corpus de testes ou
documentos de exemplo do projecto antes de remover — se for, esses usos também
precisam de ser actualizados para `.len()` (método), não só a função removida.

## 3. Se a decisão for (b) — manter como extensão documentada

Não requer mudança de código nenhuma. Requer:
- Documentar explicitamente em `foundations/len.rs` (comentário) e no prompt L0
  correspondente que `len` global é extensão do cristalino, sem equivalente no
  vanilla — para que ninguém no futuro confunda isto com paridade e tente
  "corrigir" na direcção errada.
- Confirmar se a contagem por codepoints (em vez de bytes, que é o que `.len()`
  método já faz) é intencional para a extensão, ou se devia ser consistente com
  o método — decisão separada dentro da opção (b), também do dono.

## 4. Nota — não é prioridade Baixa por acaso

O P1031 classificou este achado como Baixa prioridade, e a natureza da decisão
(extensão vs remoção, não bug) reforça isso — não há urgência, porque não há
comportamento "errado" a propagar erro para o usuário; há uma pergunta de
política de produto em aberto.

## Critério de conclusão

- Decisão do dono registada explicitamente: (a) remover ou (b) manter como
  extensão documentada.
- Se (a): código real lido, usos existentes de `len` verificados antes de
  remover.
- Se (b): documentação adicionada, pergunta sobre codepoints-vs-bytes na
  extensão respondida.
