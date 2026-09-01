# P1291 — recibo dos testes A em RED

## Papel e isolamento

- Regime: Tekt A/B, papel `Testador A`.
- Executor: agente `/root/testador_a`, sem leitura de implementação candidata.
- Entradas: passo autorizado `typst-passo-1291.md`, manifesto e matriz P1291,
  L0s ressellados, baseline vanilla/fonte e consumers anteriores ao candidato.
- Escrita concedida: somente blocos `#[cfg(test)]` em
  `01_core/src/compiler/stdlib/structural/math.rs`, testes existentes em
  `01_core/src/compiler/math/layout/mod.rs` e este recibo.
- Não foram escritos código produtivo, L0 ou headers.
- `cancel`, `underline` e `vec` permanecem gated por ADR-0127 e não são
  aceitos como GREEN nesta suíte.

## Proveniência

- Instante do selo dos testes: `2026-08-31T11:09:15-03:00`.
- HEAD: `53d21c5a602f4045a769a0ab0c935baa5ecd3b88`.
- Árvore: não commitada. O estado congelado anterior aos testes está em
  `00_nucleo/diagnosticos/p1291-baseline-status.txt`, SHA-256
  `46007bc74479c66497b0c1ccd94f1585ca3d0f0997b9ba58207f80c6eb4a794a`.
- Manifesto: SHA-256
  `7ca6445f85cc697039849a6869a4fac62f9da0ca96c42dda56a52de8894d59e2`.
- Matriz: SHA-256
  `649cb0a0a0816dc2ff61997cc9fb5c069f67da7055a8915f8753c1bfd145c41e`.
- L0 `structural/math.md`: SHA-256
  `c36f2e77550e9d4fd79221063d63cf4527033d21948bb6425db8bcdb4f1bf175`.
- L0 `math_style.md`: SHA-256
  `6325f034976a93a326a9293b626c441b74b8b413a792b7349d60586690a0406d`.
- Vanilla ratificado: `/usr/local/bin/typst`, SHA-256
  `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`,
  revisão `a51e02804` (do manifesto congelado).

## Testes protegidos

SHA-256 calculado depois da última edição de testes:

- `01_core/src/compiler/stdlib/structural/math.rs`:
  `89cba52435e0e47ba5c5e43fde8808a44643a9f0f5af8ae92ff2f6e40d65168a`;
- `01_core/src/compiler/math/layout/mod.rs`:
  `d5063329e4dc6c65c7b122c38b55264f69d753361bf128bcaefb7a6cf32e03ee`.

Cobertura protegida:

- os cinco bindings são `Func`, têm nome curto e os quatro estilos apontam
  exatamente para as nativas donas;
- morfologia de `bb`, `frak`, `inline`, `serif`, default/override de
  `inline.cramped`, aridade, tipos e named args;
- `scripts` produz `MathLimitsOverride { limits: false, inline: true }`,
  valida argumentos e mantém attachments laterais;
- o scope não copia funções globais alheias;
- nesting de estilo usa inner-wins no mesmo eixo;
- um `inline(cramped: true)` interior não perde `MathSize::Text` nem
  `cramped` quando envolvido por estilo de glifo.

## Execuções e veredito RED

Tentativa inválida, registrada para não ser confundida com prova:

```text
cargo test -p typst-core p1291_namespace_expoe_cinco_funcoes_com_nome_curto -- --exact --nocapture
```

Resultado: `exit 0`, porém `running 0 tests`; o nome exato não incluiu o path
completo do módulo. Não conta como gate.

Primeiro RED válido:

```text
cargo test -p typst-core p1291_namespace_expoe_cinco_funcoes_com_nome_curto -- --nocapture
```

Resultado: `exit 101`, `1` teste executado, `0` passed, `1` failed. Testemunha:
`math.bb must be callable` em `structural/math.rs`.

Filtro focal agregado:

```text
cargo test -p typst-core p1291_ -- --nocapture
```

Resultado observado antes de remover um teste redundante de itálico e de
corrigir a sentinela falsa-positiva `rect` (símbolo legítimo): `exit 101`,
`10` testes, `1` passed e `9` failed. As testemunhas ainda pertencentes à
suíte protegida final foram:

- `inner cal must win over outer bb`: obteve `math.ident("𝕩")`, esperado
  `math.ident("𝓍\\u{fe00}")`;
- `inner inline size was lost`: obteve `Display`, esperado `Text`;
- ausência de callable `math.bb`/`math.scripts`, causando RED nos testes de
  presença, owner, morfologia, argumentos e adapter.

O teste focal de efeito de `scripts` em attachments passou, como guarda de
que o owner/layout preexistente discrimina sequência comum de
`MathLimitsOverride`. Após remover `rect` da lista de extras, o teste de scope
fechado foi executado isoladamente e passou (`1 passed`); isto evita selar um
falso positivo causado pelo espelho legítimo `sym → math`.

`git diff --check -- 01_core/src/compiler/stdlib/structural/math.rs
01_core/src/compiler/math/layout/mod.rs` terminou sem saída (`exit 0`).

## Veredito

`RED demonstrado`. A implementação B ainda precisa satisfazer as sete falhas
atuais observadas. `Unknown` não foi convertido em sucesso e nenhum dos três
membros gated recebe crédito.
