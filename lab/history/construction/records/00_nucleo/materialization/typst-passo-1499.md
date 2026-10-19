# Passo 1499 — round-trip posicional de stroke

**Estado:** IMPLEMENTADO; gate humano ADR-0127 confirmado em 2026-09-25
para o L0 `00_nucleo/prompts/compiler/stdlib/layout.md` com SHA-256
`b8cb933cfce796ad4b2494209d21deb4d72313a14b8c6a96a2d1830c89c09da1`.

## Medição antes da decisão

P1497 sondou o vanilla pinado `9f2b6e8715237cb086899a42873660fe744622e8`
e o cristalino pós-P1496. O diagnóstico
`00_nucleo/diagnosticos/typst-p1497-triagem-linguagem.md` fixa HEAD,
working tree, UTC e hashes dos binários. A sonda
`typst eval 'repr(stroke(eval(repr(stroke()))))' --format json`
devolve `"stroke()"` no vanilla; no cristalino, erro
`stroke() não aceita argumentos posicionais`. `repr(stroke(1pt))`
também vale `"1pt"`
no vanilla e falha no cristalino. O round-trip de um stroke com paint
e thickness explícitos diverge da mesma forma. Como controle,
`repr(stroke(paint: red, thickness: 2pt))` coincide bilateralmente;
`stroke(42)` falha em ambos, sem exigir mensagem idêntica.

Fonte upstream: `lab/typst-original/crates/typst-library/src/visualize/stroke.rs:227-243`.
O L0 pré-P1499 (`00_nucleo/prompts/compiler/stdlib/layout.md:625-651`)
declarava “sem posicionais” e `stroke(1pt)` como erro. A medição refuta
esse contrato para o pin novo; o P1490 já separava esta lacuna da correção
de `repr(stroke())`.

## Plano e gate (executados)

1. Revalidar a sonda com binários pinados e controles de tipo, inclusive
   stroke automático, paint/thickness explícitos e argumento inválido.
2. Atualizar primeiro o L0 proprietário
   `00_nucleo/prompts/compiler/stdlib/layout.md`, delimitando exatamente
   os posicionais aceitos, precedência frente a named args e erros. Auditar
   `compiler/eval/call_dispatch.md`, `compiler/eval/repr.md` e
   `entities/geometry.md` somente se os respectivos consumers mudarem.
3. **PARAR** para confirmação humana do novo contrato público e seu hash
   (ADR-0127). P1497 não autoriza implementação por antecipação.
4. Após confirmação: testes RED no owner, implementação mínima,
   resselo após V15/V26, `cargo build`, testes, `crystalline-lint .`,
   `git diff --check` e sonda bilateral GREEN.

**Aceitação:** round-trip conserva valor e morfologia de `Stroke` nos
estados medidos, named args permanecem válidos, argumento inválido
continua a falhar, sem inferir paridade de representação Rust.

## Execução e validação

O RED inicial (`CARGO_BUILD_JOBS=2 cargo test -p typst-core p1499_stroke
--offline -- --nocapture`) compilou: quatro testes falharam por
`stroke() não aceita argumentos posicionais` e o controle negativo passou.
Snapshot imediatamente após o RED: 2026-09-25T11:09:01Z, HEAD
`c9d0162163f80f105f022ff9081f1e079f63f266`, working tree não
commitado com exatamente o passo (4 linhas alteradas),
`prompts/compiler/stdlib/layout.md` (71) e
`01_core/src/compiler/stdlib/mod.rs` (168 linhas adicionadas). O controle
adicional de `Dict` também foi RED antes da mudança produtiva.

Após o ramo posicional, os seis testes P1499 passaram; os 107 testes
filtrados por `stroke` e os 10 por `native_stroke` passaram. A suíte
integral `typst-core` passou: 6196 unitários, mais 8 e 3 testes
adicionais, sem falhas.

Na sonda bilateral após `cargo build --offline --quiet`, os 12 casos
válidos coincidiram: round-trip automático/explícito, Length, Color,
Dict simples/complexo, shorthand aritmético, named de controle,
comprimentos zero/negativo e tipo dos valores Gradient/Tiling. Os três
controles inválidos (`stroke(42)`, segundo posicional e posicional+named)
falharam em ambos, sem exigir texto idêntico. Snapshot pré-sonda:
2026-09-25T11:16:48Z, mesmo HEAD não commitado; `git diff HEAD --stat`
incluía só o passo, L0, `layout.rs` e `mod.rs` (respectivamente 4, 71,
245 e 208 linhas de delta exibidas). O vanilla pinado tinha SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`;
o cristalino `target/debug/typst`, SHA-256
`904afaaf52ab67b4f749d26bbcd2530a9248f5efab756f9c9c0ae314c516fbc1`.
Receita por caso: `NO_COLOR=1 <bin> eval '<expressão>' --format json`.

V15/V26 passaram antes do resselo; `crystalline-lint --fix-hashes
--dry-run .` apontou somente `layout.rs`, e o resselo aplicado removeu
o drift. `crystalline-lint .` terminou exit 0; V5/V6/V15/V26 com
`--fail-on warning` terminaram sem violações. `cargo build --offline
--quiet` e `git diff --check` passaram. Snapshot antes deste registro:
2026-09-25T11:21:27Z, mesmo HEAD e quatro arquivos modificados
(passo, L0, `layout.rs`, `mod.rs`); SHA-256 do L0 pós-resselo
`05d58ebb0e7d0b6de276bbdd6cddd5215884f3f11c876bf7638b09fcd4048938`.
Não houve mudança produtiva em `repr`, dispatch ou entidades; as lacunas
named de paint Gradient/Tiling e thickness não positivo permanecem
fora do P1499, como declarado no L0 confirmado.
