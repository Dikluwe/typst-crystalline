# P1339 — premissa de ângulo incompatível com o vanilla

Estado: **bloqueado antes de L0, contrato, selo e implementação**. A sonda
A.1 foi executada em recorte focal; não se declara A.1 integral, RED completo,
mutation score, PASS_SCOPED ou fechamento das dez rotas.

## Medição antes da decisão

Proveniência: HEAD `2f42d64253547734564513a1159ee6b584c1c4b4`, branch
`Tekt`, em 2026-09-09. `git diff HEAD --stat` vazio durante a medição.
O único arquivo inicialmente não rastreado era o passo fornecido pelo dono.
Foram adicionados apenas diagnósticos P1339, sem alteração produtiva.

O congelamento A0 ocorreu entre `21:52:45.623474+00:00` e
`21:52:48.769138+00:00`. Seus 3.912 hashes produtivos coincidiram com o
inventário certificado no P1338; a mudança de HEAD decorre dos vinte commits
de organização da cadeia, não de alteração dos bytes medidos. V5/V15/V26
passaram sem violações. Não há motivo para emitir baseline-mismatch.

Binários efetivamente executados:

- Vanilla ratificado `a51e02804`: `/usr/local/bin/typst`, SHA-256
  `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
- Cristalino certificado P1338: `/tmp/p1338-target.vlNAmp/release/typst`,
  SHA-256 `f7c8085f8453ecb6b10841b698092d41e7fbec64d9d46f9341b9b9b538bfefa1`.

A sonda usa 48 expressões: descoberta das dez rotas e chamadas/controles de
ângulo. Cada binário executou 384 comandos: quatro perfis de feature
(`default`, `html`, `a11y`, `html+a11y`), em ordens normal e inversa.
Esses perfis não significam quatro targets de render.

Vanilla: `21:54:32.046593+00:00` até `21:54:34.070380+00:00`.
Cristalino: `21:54:34.119384+00:00` até `21:55:10.036131+00:00`.
Os recibos guardam todos os comandos, cwd, canais integrais, exit codes,
ambiente controlado e duração individual. Não houve instabilidade entre
ordens. Comparação focal de canais: 16 Preserved, 368 Violated, zero Unknown.
Essas contagens não são métricas de paridade global nem de fechamento.

Resultados vanilla, reproduzidos em todos os perfis e ordens:

| Expressão submetida a `repr` | Resultado |
|---|---|
| `angle.deg(90deg)` | `90.0` |
| `(90deg).deg()` | `90.0` |
| `angle.rad(90deg)` | `1.5707963267948966` |
| `(90deg).rad()` | `1.5707963267948966` |
| `angle.deg(1)` | `expected angle, found integer` |
| `angle.rad(1)` | `expected angle, found integer` |
| `angle.deg(1) == 1 * 1deg` | erro no argumento `1` |
| `angle.rad(1) == 1 * 1rad` | erro no argumento `1` |

O cristalino antecedente ainda rejeita as duas descobertas e os respectivos
métodos ligados; a ausência não torna correta a equivalência proposta.

## Fonte declarativa, classificação e inferência

Após executar a sonda, foi lido
`lab/typst-original/crates/typst-library/src/layout/angle.rs`:

- linhas 140–152: o escopo público declara conversão de um ângulo para
  radianos/graus, métodos `to_rad(self) -> f64` e `to_deg(self) -> f64`,
  expostos na linguagem com nomes curtos `rad` e `deg`;
- linhas 43–50: `Angle::rad(f64) -> Self` e `Angle::deg(f64) -> Self` são
  construtores internos Rust, fora desse escopo público;
- linhas 63–65: a conversão numérica divide pela escala da unidade.

Entrada Angle, saída numérica, identidade pública e diagnóstico são
**linguagem**. A organização em impl, macros e nomes dos construtores Rust é
**mecânica**, não especificação automática da API Typst (ADR-0107/0108).
A documentação dos métodos públicos confirma a intenção, não apenas o
comportamento acidental do executável.

Inferência: a formulação do passo parece confundir os construtores Rust com
os métodos públicos de conversão. Não se afirma conhecer a causa de autoria
do texto. Essa inferência causal seria refutada por outra origem documentada;
a incompatibilidade semântica medida permanece independentemente dela.

## Decisão e correção proposta — ainda não aplicada

`00_nucleo/materialization/typst-passo-1339.md:103` exige equivalência de
`angle.deg(x)` com `x * 1deg`, e o mesmo para radianos. Isso conflita com o
objetivo do próprio passo de coincidir com o vanilla.

É necessário o dono confirmar a retificação para:

- tratar `angle.deg(a)` e `angle.rad(a)` como conversões **Angle → float**;
- usar ângulos como casos válidos, incluindo sinal e fronteiras numéricas;
- usar inteiros/floats sem unidade como casos de rejeição de tipo;
- exigir equivalência estática/ligada: `angle.deg(a)` com `a.deg()` e
  `angle.rad(a)` com `a.rad()`;
- conservar as mesmas dez rotas e os demais requisitos, sem antecipar os
  lotes sucessores.

Não foi alterado o passo do dono. A skill `tekt-materializacao-segregada`
exige parar e solicitar decisão quando as interpretações são incompatíveis;
não seria legítimo selar a fórmula literal nem corrigi-la silenciosamente.

## Artefatos e limites

- `p1339-a0.json`: SHA-256
  `c70ca7d1f22aa7df222a081da4feddbc056b628eca505449eedb153acd71c361`.
- `p1339-authority-manifest.json`: SHA-256
  `36d64dc36333f97de2a3dc96a35ce7b70675389248238ecacd03a3da29d782f4`.
- `p1339-probe-manifest.json`: SHA-256
  `712fb949a5ed8725c6322157bbe06a8ef1cd3313007d02eb92fabeeb95bd8e21`.
- `p1339-vanilla-runs.json`: SHA-256
  `8a02ee948ad3e9f2cc73f0607dd6fcc894386cdbc2f6efcb4fc8610619e0d49f`.
- `p1339-crystalline-before-runs.json`: SHA-256
  `e140c37fb7cfd84398a8c13571b2428c5542ce9206ae0f21601869e4d51042ae`.
- `p1339-comparison-before.json`: SHA-256
  `39b861734c32c64ecc288825b24d55cda3a20f8aea1be2f36a51493d08577c4b`.

Regime: protocolo completo solicitado, **executado sem atestação de
isolamento**, limitado neste turno ao preflight e à medição focal. Root
operou a sonda; o revisor `/root/p1339_preflight_review` recebeu contexto
novo e somente pode escrever `p1339-review-*`, sem corrigir material julgado.
Essas restrições são declaradas, não capacidades tecnicamente atestadas.
O recibo independente deve ser consultado separadamente; este diagnóstico
do operador não substitui o veredito do revisor.

Ainda faltam a retificação autorizada, o restante de A.1/A.2, auditoria L0
completa, atribuição efetiva dos autores de contrato/oráculos/ataques, selo,
RED independente, implementação, GREEN, mutantes, workspace e rebaseline
global. Nenhum desses gates foi contornado e nenhum commit P1339 foi feito.
