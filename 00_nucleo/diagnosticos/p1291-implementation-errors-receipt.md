# P1291 — recibo de implementação B dos diagnósticos públicos

## Papel, regime e limites

- Regime: Tekt A/B; papel `Implementador B` do sublote de diagnósticos.
- Entrada autoritativa: `00_nucleo/materialization/typst-passo-1291.md`, lido
  por autorização explícita, os L0s `compiler/stdlib/math_style.md` e
  `compiler/stdlib/structural/math.md`, e o recibo RED
  `p1291-red-errors-receipt.md`.
- Escrita B: somente a implementação produtiva em
  `01_core/src/compiler/stdlib/math_style.rs`, o braço produtivo de `scripts`
  em `01_core/src/compiler/stdlib/structural/math.rs` e este recibo.
- Os testes A, os L0s e os headers de linhagem não foram editados por B. As
  diferenças de header já visíveis contra `HEAD` nos dois consumers existiam
  antes desta implementação, numa árvore partilhada com autores concorrentes.
- O ambiente partilha filesystem e histórico de conversa operacional; por
  isso o resultado é **executado sem atestação de isolamento**, não uma prova
  de isolamento técnico.

## Estado e entradas congeladas

- Medição final: `2026-08-31T11:46:05-03:00`.
- HEAD: `53d21c5a602f4045a769a0ab0c935baa5ecd3b88`.
- Árvore não commitada. No instante da medição final,
  `git status --short | sha256sum` produziu
  `c21d0c490988483545ff30bfc780d2fc95d2e3cce0ab72967e20bba36f257173`
  e `git diff HEAD --stat | sha256sum` produziu
  `2b83af170293d57c66e51dca044c4588cae6b4c9291e33c06da3a0ac89032d44`.
- L0 `math_style.md`, antes/depois:
  `fd33cd0c921eec632e122aa475aa28d845a1e6cb4345458bfe68a5e78c53390f`.
- L0 `structural/math.md`, antes/depois:
  `6a4dc569a729792871e6d4a317143260be605c9f8f9aa567d72d4d9e24751646`.
- Consumer `math_style.rs`: antes
  `2338fdb586204773e82eccb79c4227b33929050d6e7c46266657a16c231ec514`;
  depois
  `8ed331c0424dbd2c224ca8bb4e1eb00cfa5473f8f294c36db576951ef3176976`.
- Consumer `structural/math.rs`: antes
  `f47133b7e90e46dc1f9090eafe7c2c6c525280ba944d077d7573451862c3c59d`;
  depois
  `8e86e8db6f53b84b6a731def462e3fd5d25dff128b84b3f3ad0d9301729b37cf`.

## Materialização B

O helper único `wrap_math_style` passou a usar o formatador L1 comum
`vanilla_type_name`. Assim, `bb`, `frak`, `inline` e `serif` emitem
`unexpected argument` para excesso, `unexpected argument: <nome>` para named
desconhecido, `expected content, found <tipo vanilla>` para body inválido e,
onde `cramped` existe, `expected boolean, found <tipo vanilla>`. A conversão
positiva de `Value::Str` para `Content::MathText` foi preservada, e os mesmos
function pointers continuam expostos globalmente e sob `math`, sem wrappers de
namespace.

O adapter privado `native_math_scripts` passou a reutilizar
`vanilla_type_name_class`, helper já pertencente ao mesmo owner estrutural,
para produzir nomes longos (`integer`, `string`, `boolean`, e o nome público
vigente para os restantes). Não houve mudança de assinatura ou contrato Rust
público.

## Integridade dos testes A

Os seis intervalos textuais protegidos foram recalculados antes da
implementação e novamente depois dos dois gates de teste. As linhas mudaram de
posição por alterações produtivas anteriores ao módulo de testes, mas os bytes
e hashes permaneceram idênticos:

| bloco | SHA-256 antes | SHA-256 depois |
|---|---|---|
| helper `diagnostic_message` | `f082a19945daa7a2d8c403a35546d96d532596a6c57dab46d71e41bef92c769b` | `f082a19945daa7a2d8c403a35546d96d532596a6c57dab46d71e41bef92c769b` |
| excesso nos cinco membros | `9b8a1b21aa3bfdd98020e4727ccc82ea635215343a35517da5884dff87abf5e0` | `9b8a1b21aa3bfdd98020e4727ccc82ea635215343a35517da5884dff87abf5e0` |
| named `nope` nos cinco membros | `a7fcd04cf81dfc6d39c8a7d859d28942b8e59274f166fb6253dcdb4bf65534dd` | `a7fcd04cf81dfc6d39c8a7d859d28942b8e59274f166fb6253dcdb4bf65534dd` |
| body inteiro nos cinco membros | `4dd4134aeb24ffef5fbb17ef5b0b4b1fe1e2532ab5b4459d4cff6fa9d8f63d97` | `4dd4134aeb24ffef5fbb17ef5b0b4b1fe1e2532ab5b4459d4cff6fa9d8f63d97` |
| `inline(cramped: 1)` | `74a03e9399c3fbb4ea04000f21c1c69ba376255f546deef202bc2cb29d3880e5` | `74a03e9399c3fbb4ea04000f21c1c69ba376255f546deef202bc2cb29d3880e5` |
| strings aceites pelos quatro estilos | `6bd91ad2dd08f3380ca6529b21d54d26ec29be51f532bf6629557685cf766a7f` | `6bd91ad2dd08f3380ca6529b21d54d26ec29be51f532bf6629557685cf766a7f` |

## Gates

```text
cargo test -p typst-core p1291_red_errors_ -- --nocapture
```

Resultado: `exit 0`; `5` passaram, `0` falharam, `5.337` filtrados.

```text
cargo test -p typst-core p1291_ -- --nocapture
```

Resultado: `exit 0`; `14` passaram, `0` falharam, `5.328` filtrados. O filtro
inclui diagnósticos, namespace, identidade dos function pointers, morfologia,
composição de estilos e attachments laterais de `scripts`.

```text
git diff --check
```

Resultado: `exit 0`, sem saída. Os testes emitiram warnings preexistentes na
árvore partilhada; não houve erro de compilação ou de teste.

## Veredito B

**GREEN no fragmento observável selado dos diagnósticos P1291.** Os testes A e
os L0s permaneceram byte-idênticos; `Unknown` não foi convertido em sucesso.
Este recibo atesta somente o sublote de diagnósticos e não reivindica
equivalência funcional geral dos oito membros de P1291.
