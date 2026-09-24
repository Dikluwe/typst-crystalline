# Passo 1494 — origem de newline e morfologia CJK na realização

**Estado:** PLANEADO; gate ADR-0127 antes de código se o desenho exigir carrier público ou mudar fase. Dependência: P1489.

## Medição anterior à decisão

O vanilla pinado mantém `had_newline` na avaliação (`lab/typst-original/crates/typst-eval/src/markup.rs:98-107`) e decide o descarte durante a realização (`lab/typst-original/crates/typst-realize/src/spaces.rs:180-205`), em interação com regras `show`/regex; a ordem exata ainda deve ser medida. O cristalino preserva texto no token (`01_core/src/compiler/lexer/mod.rs:164-177`), mas `01_core/src/compiler/eval/mod.rs:6951` converte `SyntaxKind::Space` em `Content::Space` sem origem; `01_core/src/entities/content.rs:175` não tem payload. `P1489-S-001` mede forma textual no HTML: newline entre ideogramas junta, espaço explícito permanece. Baseline pinado/hashes/working tree: relatório P1489; repetir bilateralmente antes da spec.

Inferência: a proveniência precisa sobreviver até o ponto em que a realização e `show` textual/regex a observam, em ambos os targets. A fonte `typst-realize/src/lib.rs:1205-1215,1263-1270,1285-1325` sugere normalização entrelaçada; medir a ordem, inclusive `tests/suite/text/space.typ:144-162`, antes de fixar fase. Refutação: controles que provem outra ordem ou tratamento exclusivo do HTML. O patch **não** deve ser um `trim` do exporter, `split_whitespace`, nem uma regra só do lexer.

## Proposta L0 e sequência

1. Medir controles da suite vanilla `tests/suite/text/space.typ:83-213`: CJK newline, espaço explícito, latim, coreano, pontuação, comentário, strong, regra `show`/regex, texto dinâmico, smart quotes e vizinhança não textual. Comparar morfologia do conteúdo e saídas HTML/PDF quando relevantes, não bytes de render.
2. Auditar o owner da avaliação `00_nucleo/prompts/compiler/eval.md` e `00_nucleo/prompts/entities/content.md`; localizar o consumer real da realização/normalização e seu L0 proprietário (ou criar L0 1:1 se novo). `00_nucleo/prompts/compiler/eval/rules.md` e `00_nucleo/prompts/compiler/eval/selector_matching.md` são owners da segunda sublacuna de regex entre nós; `00_nucleo/prompts/compiler/eval/markup.md` e `00_nucleo/prompts/compiler/lexer/mod.md` só mudam se seus consumers mudarem. Especificar origem interna até a normalização causal na realização, sem expô-la em `repr`/igualdade da linguagem.
3. Se o desenho adicionar campo/variante pública de `Content` ou mover normalização entre eval/realização/layout, **parar após L0** para confirmação do dono (ADR-0127). Na dúvida, parar; não implementar antes desse gate.
4. Após autorização quando necessária: testes RED separados para (A) origem/normalização sem regex e (B) regex sobre texto realizado que cruza nós `Text`; `rules.rs:982-1059` hoje trata regex por nó e seu L0 fixa essa fronteira como scope-out. Implementar ambos nos owners corretos. Antes de exigir GREEN de `P1489-S-001`, trocar o comparador `semantic_tree` total por observável focal da forma textual/sentinelas `CJK_JOINED` e `CJK_EXPLICIT_SPACE`, com controles negativos; não aceitar só presença de artefato nem tolerância arbitrária. Após GREEN integral, mudar `expected_state` para `MATCH` com rebaseline, resselo, build com um job, lint, diff-check e relatório com proveniência.

Não copiar algoritmo do vanilla por reflexo: a obrigação é a morfologia demonstrada, inclusive sob `show` (ADR-0107/0108).
