# Passo 1494 — origem de newline e morfologia CJK na realização

**Estado:** MATERIALIZADO na working tree P1494; validação focal GREEN em 2026-09-24; gate ADR-0127 confirmado pelo dono. Dependência: P1489; baseline e controles no diagnóstico `00_nucleo/diagnosticos/typst-p1494-gate-cjk-newline.md`.

## Medição anterior à decisão

No HEAD `6da06e849d75972651a342ad6aad337313451c79` (working tree limpa à medição), o vanilla pinado mantém `had_newline` na avaliação (`lab/typst-original/crates/typst-eval/src/markup.rs:98-107`) e decide o descarte durante a realização (`lab/typst-original/crates/typst-realize/src/spaces.rs:180-205`). O cristalino preserva o texto do token (`01_core/src/compiler/lexer/mod.rs:163-177`), mas `01_core/src/compiler/eval/mod.rs:6970` emite `Content::Space` sem origem. A matriz `P1489-S-001` continua VIOLATED: vanilla `CJK_JOINED`, cristalino `水 果`; espaço explícito é preservado por ambos. Sondas bilaterais adicionais confirmaram também a fronteira de regex entre nós; fontes, SHA-256 dos binários, controles HTML/PDF, inferências e refutadores estão no diagnóstico P1494.

Inferência: a proveniência precisa sobreviver até o ponto em que a realização e `show` textual/regex a observam, em ambos os targets. A fonte `typst-realize/src/lib.rs:1205-1215,1263-1270,1285-1325` sugere normalização entrelaçada; medir a ordem, inclusive `tests/suite/text/space.typ:144-162`, antes de fixar fase. Refutação: controles que provem outra ordem ou tratamento exclusivo do HTML. O patch **não** deve ser um `trim` do exporter, `split_whitespace`, nem uma regra só do lexer.

## L0 ratificado e sequência

1. Medir controles da suite vanilla `tests/suite/text/space.typ:83-213`: CJK newline, espaço explícito, latim, coreano, pontuação, comentário, strong, regra `show`/regex, texto dinâmico, smart quotes e vizinhança não textual. Comparar morfologia do conteúdo e saídas HTML/PDF quando relevantes, não bytes de render.
2. Auditar o owner da avaliação `00_nucleo/prompts/compiler/eval.md` e `00_nucleo/prompts/entities/content.md`; localizar o consumer real da realização/normalização e seu L0 proprietário (ou criar L0 1:1 se novo). `00_nucleo/prompts/compiler/eval/rules.md` e `00_nucleo/prompts/compiler/eval/selector_matching.md` são owners da segunda sublacuna de regex entre nós; `00_nucleo/prompts/compiler/eval/markup.md` e `00_nucleo/prompts/compiler/lexer/mod.md` só mudam se seus consumers mudarem. Especificar origem interna até a normalização causal na realização, sem expô-la em `repr`/igualdade da linguagem.
3. Se o desenho adicionar campo/variante pública de `Content` ou mover normalização entre eval/realização/layout, **parar após L0** para confirmação do dono (ADR-0127). Na dúvida, parar; não implementar antes desse gate.
4. Após autorização quando necessária: testes RED separados para (A) origem/normalização sem regex e (B) regex sobre texto realizado que cruza nós `Text`; `rules.rs:982-1059` hoje trata regex por nó e seu L0 fixa essa fronteira como scope-out. Implementar ambos nos owners corretos. Antes de exigir GREEN de `P1489-S-001`, trocar o comparador `semantic_tree` total por observável focal da forma textual/sentinelas `CJK_JOINED` e `CJK_EXPLICIT_SPACE`, com controles negativos; não aceitar só presença de artefato nem tolerância arbitrária. Após GREEN integral, mudar `expected_state` para `MATCH` com rebaseline, resselo, build com um job, lint, diff-check e relatório com proveniência.

Não copiar algoritmo do vanilla por reflexo: a obrigação é a morfologia demonstrada, inclusive sob `show` (ADR-0107/0108).

## Preparação e ratificação do gate

A medição bilateral, a classificação língua/mecânica e os refutadores estão
em `00_nucleo/diagnosticos/typst-p1494-gate-cjk-newline.md`. O L0 ratificado
abrange cinco owners centrais: `entities/content`, `compiler/eval`,
`compiler/eval/rules`, `compiler/eval/selector_matching` e
`compiler/eval/repr`. A auditoria do enum público encontrou mais dez owners
com braços exaustivos ou equivalência estrutural: `compiler/introspect`,
`compiler/introspect/locatable`, `compiler/layout`,
`compiler/layout/sequence`, `compiler/columns`,
`compiler/math/layout/spacing`, `infra/query-helpers`,
`infra/export/html`, `infra/pipeline` e
`infra/pipeline/context_stabilization`. O owner produtivo indireto
`entities/content_hash` também recebeu seção P1494: sua serialização por
Debug deve produzir o mesmo hash para as duas origens. Os dezesseis L0s
produtivos foram ratificados após a medição, com gate confirmado em
2026-09-24. `compiler/eval/tests` é um décimo sétimo L0, test-only,
sem consumer produtivo adicional.

O comparador focal `html_paragraph_text` substitui `semantic_tree`
somente em `P1489-S-001`; exige dois parágrafos exatos e distingue defeito
do oráculo de divergência do cristalino. Nesta preparação, o estado esperado permaneceu
`VIOLATED` até implementação e rebaseline. O predicado Unicode do codex
vanilla git rev `2e43ab9` não existe no `codex = 0.3.0` cristalino;
o L0 ratificado exige `icu_properties 2.2.0` direto em L1 e whitelist
estrita, sem I/O. A confirmação específica da nova variante pública e da ordem
show/realização foi recebida em 2026-09-24 (ADR-0127). Testes RED,
código L1/L3, headers/resselo e GREEN estão autorizados.

## Fechamento factual da implementação

No HEAD `6da06e849d75972651a342ad6aad337313451c79` com working tree
não commitada, a suíte L1 passou integralmente (6174/6174), os builds
`dev` e `release` passaram e os quatro testes L3 focais P1494 passaram.
O comparador focal da matriz `P1489-S-001` passou a `MATCH`: oráculo e
cristalino produziram `[CJK_JOINED,CJK_EXPLICIT_SPACE]`, com digest
determinístico `a36fe2787915d2a4d76549abff8f6f2bb285519b59fd803ab10597986bd58bc8`
e saída 0 após o rebaseline. O diagnóstico P1494 guarda hashes dos
binários, janela temporal, inventário exato da working tree e comandos.

A suíte infra integral registrou 1067/1072 com cinco falhas concentradas
em P1407 repeat/outline/justify. O integrador extraiu `git archive` do
HEAD P1493 exato, sem alterações P1494, e reproduziu as mesmas cinco
falhas e valores (0/5, saída 101) no binário SHA-256
`08362babbbe0581a55f19285f579febed7188d4265f4c898fead2106ea42a663`.
Logo, essas falhas já existem no commit P1493; a receita integral está
no diagnóstico. O lint crítico e `git diff --check` passaram. A verificação
global de `cargo fmt --all -- --check` falha por dívida ampla
preexistente e não foi usada como gate deste passo.

Dívida distinta: em ContentBlock bruto com `show` local de `水果` sobre
`水`-newline-`果`, o vanilla emite `<p>X</p>`. O cristalino ainda não
tem carrier de `show` diferido para esse fragmento; a paridade desse
caso combinado não é alegada por este fechamento.
