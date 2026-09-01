# P1291 — recibo RED dos diagnósticos públicos

## Papel, regime e isolamento

- Regime: Tekt A/B; papel `Testador A` do sublote de erros públicos.
- Entradas lidas: o passo explicitamente autorizado
  `00_nucleo/materialization/typst-passo-1291.md`, os L0s vigentes
  `compiler/stdlib/math_style.md` e `compiler/stdlib/structural/math.md`, o
  recibo/testes P1291 preexistentes e a API `SourceDiagnostic` de testes.
- Escrita: somente novos blocos `#[cfg(test)]` em
  `01_core/src/compiler/stdlib/structural/math.rs` e este recibo.
- Não foram lidos nem alterados o corpo da implementação produtiva candidata,
  L0s ou headers. Nenhum outro ficheiro de `materialization/` ou `context/` foi
  listado ou lido.
- O ambiente usa uma árvore partilhada e não fornece atestação técnica de
  isolamento de leitura. A autoria foi segregada por allowlist operacional e
  ordem causal, portanto o resultado é **executado sem atestação de isolamento**.

## Entradas e estado da medição

- Resultado capturado em `2026-08-31T11:38:40-03:00`.
- HEAD: `53d21c5a602f4045a769a0ab0c935baa5ecd3b88`.
- Árvore: não commitada. A lista exata congelada antes dos testes está em
  `00_nucleo/diagnosticos/p1291-baseline-status.txt`, capturada por
  `git status --short` em `2026-08-31T10:47:23-03:00`, SHA-256
  `46007bc74479c66497b0c1ccd94f1585ca3d0f0997b9ba58207f80c6eb4a794a`.
- No instante do resultado final, `git status --short | sha256sum` produziu
  `234d0982741f76ac8454cd8f159b65084c93df3a07be50c7fa30d422117e09f3`
  e `git diff HEAD --stat | sha256sum` produziu
  `b4893f66b0a6dd6e008c8ffad3ce46179396f7df95bcf1420b236c5fa296bb98`.
  Como há autores concorrentes na árvore partilhada, estes dois hashes são
  marcadores de deriva; a identidade reproduzível desta execução é dada pelos
  hashes integrais abaixo.
- L0 `math_style.md`: SHA-256
  `fd33cd0c921eec632e122aa475aa28d845a1e6cb4345458bfe68a5e78c53390f`.
- L0 `structural/math.md`: SHA-256
  `6a4dc569a729792871e6d4a317143260be605c9f8f9aa567d72d4d9e24751646`.
- Consumer com testes no instante da execução: SHA-256
  `f47133b7e90e46dc1f9090eafe7c2c6c525280ba944d077d7573451862c3c59d`.

## Blocos de teste selados

Os hashes abaixo são dos intervalos textuais inclusivos no estado acima; cada
bloco de teste inclui o respetivo `#[test]`:

| bloco | linhas no instante do selo | SHA-256 |
|---|---:|---|
| helper `diagnostic_message` | 865–870 | `f082a19945daa7a2d8c403a35546d96d532596a6c57dab46d71e41bef92c769b` |
| excesso nos cinco membros | 962–981 | `9b8a1b21aa3bfdd98020e4727ccc82ea635215343a35517da5884dff87abf5e0` |
| named `nope` nos cinco membros | 982–1003 | `a7fcd04cf81dfc6d39c8a7d859d28942b8e59274f166fb6253dcdb4bf65534dd` |
| body inteiro nos cinco membros | 1004–1023 | `4dd4134aeb24ffef5fbb17ef5b0b4b1fe1e2532ab5b4459d4cff6fa9d8f63d97` |
| `inline(cramped: 1)` | 1024–1030 | `74a03e9399c3fbb4ea04000f21c1c69ba376255f546deef202bc2cb29d3880e5` |
| strings aceites pelos quatro estilos | 1031–1047 | `6bd91ad2dd08f3380ca6529b21d54d26ec29be51f532bf6629557685cf766a7f` |

Os três testes matriciais avaliam `bb`, `frak`, `inline`, `scripts` e `serif`
antes da asserção final, de modo que uma primeira divergência não oculta os
demais membros. O teste positivo de string cobre apenas os quatro estilos, pois
`scripts` exige `Content` segundo o L0 estrutural.

## Comando focal e RED

```text
env CARGO_TARGET_DIR=/tmp/p1291-testador-a-errors-target cargo test -p typst-core p1291_red_errors_ -- --nocapture
```

Resultado final: `exit 101`; `5` testes executados, `1` passou e `4` falharam;
`5337` ficaram filtrados. As falhas observadas foram:

- excesso: `scripts` já devolveu `unexpected argument`; `bb`, `frak`,
  `inline` e `serif` ainda devolveram `<nome>() espera 1 argumento, recebeu 2`;
- named desconhecido: `scripts` já devolveu `unexpected argument: nope`; os
  quatro estilos ainda devolveram `<nome>() não aceita o argumento nomeado
  'nope'`;
- body inteiro: os quatro estilos ainda devolveram
  `<nome>() espera content ou string, recebeu int`; `scripts` devolveu
  `expected content, found int`; todos deveriam devolver exatamente
  `expected content, found integer`;
- `inline(cramped: 1)` devolveu `cramped deve ser bool, recebeu int`, em vez de
  `expected boolean, found integer`;
- o controlo positivo passou: `String` continua aceite por `bb`, `frak`,
  `inline` e `serif` e é convertida em `MathText`.

`git diff --check -- 01_core/src/compiler/stdlib/structural/math.rs` terminou
sem saída e com `exit 0`.

## Veredito

**RED demonstrado antes de qualquer correção B deste sublote.** Os testes
discriminam as mensagens públicas vanilla verbatim pedidas. `Unknown` não foi
convertido em sucesso, e este recibo não atribui GREEN nem equivalência geral.
