# Passo 1502 — restaurar a expansão de repeat com prefixo após isolamento de box

**Estado:** IMPLEMENTADO. Lacuna de morfologia separada dos sete RED do P1489;
aberta pelo fecho restrito P1498 e materializada neste passo.

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

## Plano e gate executados

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

## Execução e proveniência P1502

No HEAD inicial `f6d4f890141203bbdbbb91dd0ac8d5009c757abc`, worktree limpa,
a auditoria confirmou os owners L0 1:1 de `boxed.rs` e `pipeline.rs`.
Os L0s de repeat, cursor e outline já especificavam a forma correta e não
foram alterados. Trata-se de correção interna de paridade sem contrato,
default, fase ou compatibilidade novos: fluxo contínuo ADR-0127. L0 foi
atualizado antes dos testes e do consumer; V15/V26 passaram antes do
resselo. Hashes finais: `@prompt-hash 71023ebf` e `fd0702f8`;
`Hash do Código ed5effdb` e `c1aa1309`.

RED em 2026-09-25T14:44:50Z, HEAD inicial não commitado: quatro testes
novos `p1502_*` falharam (0/4), assim como os cinco `p1407_*` (0/5).
Nesse momento, `git diff HEAD --stat` listava somente `boxed.md` (+44),
`infra/pipeline.md` (+23) e `pipeline.rs` (+102): 169 inserções e
nenhuma remoção; `git diff --check` passou. O controle sem prefixo foi
persistido em `lab/parity/corpus/p1407/control-no-prefix.typ`, SHA-256
`b990216334a6b73f69b309e7c88184aa9396fbdec8aa022b59e6a43cedb3cc02`.

A correção em `boxed.rs` retira com a linha do pai os pendentes `fr`,
larguras naturais e repeats. O body usa buffers locais; na reunião,
seus índices são transladados pelo comprimento do prefixo e os buffers
são restaurados/anexados na ordem dos itens. O isolamento P1491 fica.
`p1502_*` passou 4/4, `p1407_*` 5/5, `p1491_measure_tests` 1/1,
`p1493` L1 11/11 e L3 6/6. A suíte L1 ampla passou 6194 testes com
dois testes de recursão antigos filtrados: sem filtro,
`recursao_infinita_retorna_err_sem_crash` e
`recursao_profunda_retorna_err` abortam por stack overflow. Os dois
testes estão inalterados neste passo. Builds debug e release offline
passaram.

Sonda bilateral em 2026-09-25T14:54:03Z–14:54:57Z, mesmo HEAD inicial
com worktree não commitada: quatro ficheiros alterados em
`git diff HEAD --stat` (199 inserções, 4 remoções) e fixture de controle
ainda não rastreado. Binário vanilla pinado SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`;
cristalino release SHA-256
`c0663911190b0afcc9aad86ee78fb65abcd11687943611c6a47f27d3a0944d48`.
Fontes `repeat.typ`, `outline.typ`, `narrow.typ`: SHA-256
`4242ed2345e44d8babc48ac7f51b5dd174804944bec67ffa746fad6487b9e90a`,
`0de527c4ae84e84a164656ff1a346b1f29cae4b106cbe6eace700e5c205264ef`,
`58ce5ae0af11ad2c3f69b1edcde54e8a4643c5ddea33a370c82af718b3f8158d`.
Vanilla/cristalino compilaram com exit 0 e stderr vazio;
`pdftotext -layout` contou 23/23, 76/76 e 16/16 pontos; em repeat, A/B únicos.
O controle sem prefixo também deu 23/23. Texto bruto foi idêntico
em repeat, narrow e controle; outline diferiu apenas na indentação
de um item e coincidiu após normalizar whitespace. PDF byte-a-byte
não é gate. A CLI cristalina trata `-` como nome de ficheiro, não stdin;
por isso o controle final usa fixture persistido, sem mudar a CLI.

Matriz no mesmo release: `P1489-L-001` (default),
`P1493-I-001/002` (html) e `P1493-I-003` (html+bundle) deram MATCH
nas ordens direta e inversa. `crystalline-lint .`, verificação estrita
`--checks v5,v6,v15,v26 --fail-on warning`, `--fix-hashes --dry-run`,
`git diff --check` e `rustfmt --check` do consumer passaram.

Verificação L3 ampliada no mesmo código: `cargo test --offline
-p typst-infra --lib -- --test-threads=2 --skip p1431_line_cap_tests`
passou 1072 testes, quatro filtrados, em 65,38s. A tentativa sem filtro
foi interrompida após mais de cinco minutos com um teste P1431 em CPU;
seu resultado é inconclusivo e não foi contado como PASS. Nessa medição,
o HEAD continuava `f6d4f890...`, com o passo, os dois L0s, `boxed.rs` e
`pipeline.rs` modificados e o fixture de controle ainda não rastreado.
