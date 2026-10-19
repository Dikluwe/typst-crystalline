# Passo 1490 — corrigir a morfologia de `repr(stroke())`

**Estado:** CONCLUÍDO em 2026-09-24. Dependência: diagnóstico P1489. Este passo não altera o contrato público de `Stroke`. Evidência final em `00_nucleo/diagnosticos/typst-p1490-stroke-repr.md`.

## Medição anterior à decisão

No vanilla pinado `9f2b6e8715237cb086899a42873660fe744622e8`, `repr(stroke())` devolve `stroke()` (`lab/typst-original/crates/typst-library/src/visualize/stroke.rs:310-319`); o cristalino devolve `1pt + black` (`01_core/src/compiler/eval/repr.rs:409-455`). A sonda `P1489-B-001` está RED. O commit upstream `5d5ed38af` classifica a mudança como correção da representação de stroke totalmente automático; não se deve perpetuar o bug antigo. O relatório `00_nucleo/diagnosticos/typst-p1489-resync-vanilla.md` registra HEAD, worktree não commitado, hashes dos binários e receita desta observação. Repetir e registrar a proveniência atual antes de executar.

`01_core/src/entities/geometry.rs:75-105` já preserva presença explícita em `StrokeFields`. `00_nucleo/prompts/entities/geometry.md:187-214` ainda fixa a expectativa antiga. Inferência: o formatter pode distinguir automático de explícito sem novo campo; refutação: um controle explícito ou round-trip do vanilla que o modelo atual não represente.

## Contrato e sequência

1. Auditar e atualizar primeiro os L0 proprietários `00_nucleo/prompts/compiler/eval/repr.md` e `00_nucleo/prompts/entities/geometry.md`; não alterar `entities/geometry.rs` se a presença existente basta. Resselar hashes só depois de manter V15/V26 íntegros.
2. Acrescentar testes RED de linguagem: `stroke()`; `stroke(thickness: 1pt)`; `stroke(paint: black)`; ambos explícitos; casos compostos de `repr` e `eval(repr(stroke()))` onde o constructor atual já aceita a forma. O round-trip completo da suite vanilla `tests/suite/visualize/stroke.typ:160-173` inclui argumentos posicionais que o constructor cristalino ainda rejeita: registrar como lacuna separada, não exigir GREEN aqui sem L0/gate ADR-0127. Não aceitar apenas igualdade do valor resolvido, porque ela apaga a morfologia.
3. Corrigir `repr_stroke`, atualizar a expectativa unitária antiga em `repr.rs:2047` e confirmar GREEN da sonda `P1489-B-001`; só então mudar seu `expected_state` de `VIOLATED` para `MATCH` com rebaseline. Render/layout continuam a consumir os valores resolvidos.
4. Rodar testes focais, `CARGO_BUILD_JOBS=1 cargo build`, `crystalline-lint .` e `git diff --check`. Registrar estado exato do código e dos dois binários usados.

**Gate ADR-0127:** fluxo contínuo se for apenas fórmula de `repr` e L0, com RED→GREEN. Se surgir necessidade de novo campo de entidade, assinatura pública ou fase, parar após redigir o L0 e pedir confirmação. Não usar igualdade de bytes de PDF como aceitação (ADR-0107).
