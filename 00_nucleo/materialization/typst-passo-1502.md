# Passo 1502 — restaurar a expansão de repeat com prefixo após isolamento de box

**Estado:** PLANEADO. Lacuna de morfologia separada dos sete RED do P1489;
aberta pelo fecho restrito P1498. Nenhuma implementação produtiva é autorizada
por P1498.

## Medição antes da decisão

No HEAD `7424d5eaa808463b32b3a7f668df7055f599c5e8`, worktree limpa,
os binários pinados vanilla SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`
e cristalino release SHA-256
`2620ffb09b9f1ed2f51bdb412cd883cc1bf944c5f0a6a7433926eee7146412e3`
compilaram bilateralmente `lab/parity/corpus/p1407/{repeat,outline,narrow}.typ`
com exit 0. O texto extraído por `pdftotext -layout` teve 23/1 pontos
(vanilla/cristalino) em `repeat`, 76/18 em `outline` e 16/5 em
`narrow`. Em `repeat`, o cristalino emite `AA.` onde o vanilla
repete o ponto entre A e B; um controle sem prefixo A produziu 23
pontos e texto idêntico nos dois. As cinco provas Rust `p1407_*`
falharam no cristalino (0/5), após 5/5 GREEN no P1407 histórico.
O diagnóstico P1494 já reproduzia 0/5 no archive do commit P1493,
sem código P1494. Horários, hashes das fontes/PDFs, comandos e limites
estão em `00_nucleo/diagnosticos/typst-p1498-fecho-p1489.md`.
Esta regressão não é causada simplesmente pelo novo pin vanilla.

Fonte atual: `01_core/src/compiler/layout/boxed.rs:157-179` retira
temporariamente `current_line`, desce no body e restaura o prefixo;
`repeat.rs:32-51` registra `start/end` relativos à linha vazia;
`cursor.rs:489-519` usa esses índices após a restauração.
`git blame` atribui a retirada/restauração de `boxed.rs` ao P1491.
Os L0s vigentes `00_nucleo/prompts/compiler/layout/boxed.md:48-76`,
`00_nucleo/prompts/compiler/layout/repeat.md:24-42`,
`00_nucleo/prompts/compiler/layout/cursor.md:432-455` e
`00_nucleo/prompts/compiler/layout_outline.md:195-210` exigem simultaneamente isolamento
do body, repetição finita e fill dentro do link.

## Hipótese e refutação

**Inferência causal, ainda não prova por bisect:** o estado
`PendingRepeat` guarda índices locais ao body; ao anexar o body
depois do prefixo sem rebasing, o Cursor clona o prefixo ou lê o
suffix errado. A hipótese seria refutada por um teste que mostre
`start/end` corretamente transladados após a restauração e ainda
reproduza `AA.`, ou por um build imediatamente anterior à mudança
P1491 que apresente a mesma divergência. Não remover o isolamento
P1491 para forçar GREEN: ele protege baseline e prefixo de body bloco.

## Trabalho futuro

1. Auditar primeiro os Prompts L0 1:1 de `boxed.rs`
   (`00_nucleo/prompts/compiler/layout/boxed.md`),
   `repeat.rs` (`00_nucleo/prompts/compiler/layout/repeat.md`),
   `cursor.rs` (`00_nucleo/prompts/compiler/layout/cursor.md`)
   e, somente se necessário,
   `outline.rs` (`00_nucleo/prompts/compiler/layout_outline.md`).
   Medir índices/forma antes de alterar a decisão; atualizar cada L0
   proprietário antes de seu consumer e ressellar após V15/V26.
2. Escrever RED focal para prefixo A + box repeat + suffix B,
   controle sem prefixo, duas fronteiras, gap/justify, teto e
   `outline.entry` link; confirmar que os cinco testes P1407 atuais
   falham pelas causas medidas.
3. Materializar somente a correção de índices/escopo comprovada,
   preservando o isolamento físico do P1491 e o match estático da
   forma B (ADR-0109). Revalidar `P1489-L-001`, os testes de
   baseline/box de P1491 e o target HTML P1493 para detectar regressão.
4. Exigir GREEN 5/5 P1407 e sonda bilateral de morfologia 23/23,
   76/76 e 16/16 com A/B únicos, controle sem prefixo invariável,
   além de build, lint, V5/V15/V26 e diff-check.

**Gate ADR-0127:** esta parece correção interna de paridade com L0
primeiro e RED→GREEN em fluxo contínuo. Se a auditoria exigir mudança
de contrato público, comportamento por defeito novo, fase de pipeline
ou quebra de compatibilidade, parar após L0 e pedir confirmação humana.
Não usar igualdade de bytes PDF ou igualdade de `FrameItem` como
aceitação; a forma textual e o escopo do link são observáveis da língua
(ADR-0107/0108).
