# P1291 — recibo de implementação B

## Papel, entradas e limites

- Regime: Tekt A/B; papel `Implementador B`.
- Executor: agente `/root/implementador_b`, na árvore partilhada do coordenador.
- Predecessor causal: selo RED de A em
  `00_nucleo/diagnosticos/p1291-red-tests-receipt.md`, SHA-256
  `d73fe204f3ebf3aafd2640989287922f640a13966408c8437a2c8824b095f00a`.
- Manifesto recebido: SHA-256
  `7ca6445f85cc697039849a6869a4fac62f9da0ca96c42dda56a52de8894d59e2`.
- L0s recebidos: `structural/math.md`
  `c36f2e77550e9d4fd79221063d63cf4527033d21948bb6425db8bcdb4f1bf175`,
  `math_style.md`
  `6325f034976a93a326a9293b626c441b74b8b413a792b7349d60586690a0406d`
  e `math/layout/_comum.md`
  `a4760755ea54621fd5c44c24804665f31d821b1658d73fa544e25240ddc3dbee`.
- Escrita exercida: código produtivo em `structural/math.rs`,
  `math_style.rs` e `math/layout/mod.rs`, mais este recibo. Nenhum L0,
  manifesto, matriz, recibo A ou bloco `#[cfg(test)]` foi editado.
- `cancel`, `underline` e `vec` não foram registrados nem implementados;
  continuam no gate ADR-0127.

HEAD durante a execução: `53d21c5a602f4045a769a0ab0c935baa5ecd3b88`;
árvore não commitada. Instante final: `2026-08-31T11:22:07-03:00`.

## Implementação candidata

- `math.bb`, `math.frak`, `math.inline` e `math.serif` foram inseridos no
  scope fechado com os mesmos function pointers das quatro nativas donas.
- `math.scripts` usa o adapter privado `native_math_scripts`, valida
  aridade/tipo/named args e delega a
  `Content::math_limits_override(body, false, true)`.
- A validação comum das nativas com `cramped` passou a rejeitar named args
  diferentes de `cramped`; isto fecha a aceitação silenciosa observada pelo
  teste A sem criar wrapper alternativo para `math.inline`.
- A composição de `MathStyled` resolve glyph/bold/italic com precedência do
  setter interno e preserva wrappers de tamanho/`cramped` para o handler de
  layout, mantendo os eixos ortogonais.

Hashes finais do código produtivo:

- `01_core/src/compiler/stdlib/structural/math.rs`:
  `585c555ffc4d7c0a0dfa9ff85bd1281d0c0db7b69e5910525fa2869d36b8d0b8`;
- `01_core/src/compiler/stdlib/math_style.rs`:
  `eba3a5f64b17964d7b362eb1d0d2a1e17a6b18139c96fbdc555fd1a58d113038`;
- `01_core/src/compiler/math/layout/mod.rs`:
  `7f280b09b35be4d3ba90388bba3cf6dcab6cfadf17c2a52cd15dee07eaf7df10`.

## Preservação dos testes protegidos

Os hashes whole-file selados por A mudam legitimamente porque os mesmos
ficheiros contêm código produtivo. Para provar a preservação, foi calculado o
SHA-256 da concatenação de todos os blocos iniciados por `#[cfg(test)]` em
cada consumer, antes e depois do patch:

```text
structural/math.rs  d2f0a2b47cb4271daa736ecdb626ee1d134799b51b38f81846ba7d3eff2f6157
math/layout/mod.rs  d20a9f57fd76b91a5d97956d5014e6ee78c8e533b84782e14cf2cc6f995ca30f
```

Os dois valores foram idênticos antes/depois. Comando reproduzível, aplicado
separadamente a cada ficheiro:

```text
perl -0777 -ne 'while (/(#\[cfg\(test\)\].*?)(?=\n#\[cfg\(test\)\]|\z)/sg) { print $1 }' <ficheiro> | sha256sum
```

## Gate executado

```text
cargo test -p typst-core p1291_ -- --nocapture
```

Resultado final: `exit 0`; `9` testes executados, `9` passed, `0` failed,
`5328` filtered out. O comando foi repetido após o último rename mecânico e
permaneceu GREEN.

```text
git diff --check -- 01_core/src/compiler/stdlib/structural/math.rs \
  01_core/src/compiler/stdlib/math_style.rs \
  01_core/src/compiler/math/layout/mod.rs
```

Resultado: `exit 0`, sem saída.

Por instrução do coordenador, não foram executados gates completos do
workspace nem filtros adicionais. O veredito integrado e a verificação dos
hashes L0/lineage pertencem ao Verificador.

## Veredito do papel B

`Implementação candidata GREEN no contrato focal P1291, com testes A
preservados.` A atestação final continua pendente do coordenador/verificador;
este recibo não reivindica equivalência funcional fora dos cinco membros
contínuos e da composição de estilos coberta.
