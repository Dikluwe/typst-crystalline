# Passo 1499 — round-trip posicional de stroke

**Estado:** PLANEADO; gate humano ADR-0127 pendente antes de código produtivo.

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
O L0 vigente `00_nucleo/prompts/compiler/stdlib/layout.md:625-651`
declara “sem posicionais” e `stroke(1pt)` como erro. A medição refuta
esse contrato para o pin novo; o P1490 já separava esta lacuna da correção
de `repr(stroke())`.

## Trabalho futuro e gate

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
