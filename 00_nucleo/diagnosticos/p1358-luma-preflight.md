# P1358 — pré-medição de `luma` contra o vanilla ratificado

## Resultado

O `native_luma` cristalino implementa o contrato do snapshot histórico
`a51e02804`, enquanto o único vanilla ratificado `586e1bd43` mudou a forma
pública do construtor em três commits. A divergência não é uma simples troca
de fórmula: afeta cardinalidade, posição de `alpha`, aceitação de chamadas e
diagnósticos públicos.

O lote seguro para P1358 é o protocolo de argumentos e diagnósticos de
`luma(lightness[, alpha])`. A conversão `luma(color)` deve permanecer fora do
lote porque contém uma decisão distinta já registrada no L0: o cristalino
preserva alpha, enquanto o vanilla medido o perde ao converter uma cor não
Luma. Misturar os dois assuntos esconderia uma escolha semântica sob uma
correção de assinatura.

## Proveniência

- UTC: `2026-09-12T20:11:47Z`.
- HEAD: `8ceea20f92dff06b6ca5a34828135c0fd3e35248`.
- Árvore: `c3c2fa834e6ec318690ddb88e181277ff2e25b27`.
- Working tree: não commitado; `git diff HEAD --stat` registrava mudanças em
  `typst-passo-1356.md` e nos dois enumeradores de `lab/`, 178 inserções e 31
  remoções. Os artefatos P1356/P1357 ainda não rastreados também estavam
  presentes.
- Vanilla: commit `586e1bd43fae6c9a973218163d3165c53ab8d16d`, binário
  `lab/typst-original/target/release/typst`, SHA-256
  `eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`.
- Cristalino: `target/release/typst`, SHA-256
  `8d9bee2a51d3a309c7459904e619972c608a311f6531c3956c1c635ad2e0bba4`.
- L0 vigente: `00_nucleo/prompts/compiler/stdlib/foundations/color.md`,
  SHA-256 `64d925b20110c53bbeef1e0c7aa6804e2b75206cdb4b1ce51a81e64bdff726db`.
- Consumer: `01_core/src/compiler/stdlib/foundations/color.rs`, SHA-256
  `d23a00c2751b64b8ec7be2d6a37ad2e06319727ffec3e4214c339bb16256e258`.

Comandos principais: `git diff a51e02804..586e1bd43 --
crates/typst-library/src/visualize/color.rs`; leitura com linhas numeradas da
fonte/testes; e `typst eval <expressão> --format json` bilateral para cada caso
abaixo. Cada processo foi independente; não se usou igualdade Rust ou bytes de
render como prova de linguagem.

## Fonte vanilla antes da decisão

Em `lab/typst-original/crates/typst-library/src/visualize/color.rs:341-374`,
`luma` recebe `Args` e declara três alternativas externas: `lightness:
Component`, `alpha: Component` e `color: Color`. A execução procura primeiro
uma cor; sem cor, exige o primeiro `Component`, consome um segundo componente
posicional opcional e usa alpha 100% quando ausente.

O intervalo entre o snapshot histórico e o vanilla contém três mudanças
focais:

1. `fdfb8da09`: remove o fallback branco de lightness inválida/ausente e passa
   a propagar diagnósticos;
2. `afe812ed7`: muda alpha de `RatioComponent` para `Component`, admitindo
   inteiros 0–255;
3. `65ee10716`: acrescenta ao caso vazio o hint para `white` ou `luma(100%)`.

Os testes vanilla em
`lab/typst-original/tests/suite/visualize/color.typ:172-177` congelam alpha
ratio e inteiro; `:212-231` congelam ausência, tipos inválidos, named arg
inesperado e positional excedente.

## Medição bilateral da linguagem

| Chamada | Vanilla `586e1bd43` | Cristalino atual | Classe |
|---|---|---|---|
| `luma()` | erro `missing argument: lightness component` + hint | `luma(100%)` | diferença |
| `luma(128)` | `luma(50.2%)` | `luma(50.2%)` | match |
| `luma(128, 0)` | `luma(50.2%, 0%)` | erro de cardinalidade | diferença |
| `luma(128, 255)` | `luma(50.2%)` | erro de cardinalidade | diferença |
| `luma(128, 40%)` | `luma(50.2%, 40%)` | erro de cardinalidade | diferença |
| `luma(128, alpha: 40%)` | erro `unexpected argument: alpha` | `luma(50.2%, 40%)` | diferença |
| `luma("bad")` | erro de tipo | fallback `luma(100%)` | diferença |
| `luma(300)` | erro de range inteiro | fallback `luma(100%)` | diferença |
| `luma(150%)` | erro de range ratio | fallback `luma(100%)` | diferença |
| `luma(128, -1)` / `256` | erro de range inteiro | erro genérico de cardinalidade | diferença |
| `luma(128, 0.5)` | erro `expected integer or ratio, found float` | erro genérico de cardinalidade | diferença |
| `luma(128, 101%)` | erro de range ratio | erro genérico de cardinalidade | diferença |
| `luma(25%, 50%, "val")` | erro `unexpected argument` no terceiro | erro genérico de cardinalidade | diferença |
| `luma(255, key: "val")` | erro `unexpected argument: key` | diagnóstico cristalino diferente | diferença |

O P1357 já repetiu S04 nos perfis `default`, `html`, `a11y-extras` e
`html+a11y`: 4 matches para `luma(128)` e 20 diferenças nos cinco casos
restantes. Logo, feature flags não explicam a divergência.

## Conversão de cor deliberadamente separada

`luma(rgb(255, 0, 0, 40%)).components()` mediu `(49.85%, 100%)` no vanilla e
`(49.85%, 40%)` no cristalino. O L0 vigente chama a preservação cristalina de
correção de bug do upstream. Como isso é comportamento público e não mecânica,
uma decisão posterior deve escolher explicitamente entre paridade estrita e a
divergência deliberada. P1358 não altera esse eixo e não pode alegar paridade
total de `luma(color)`.

## Estado do L0 e do consumer

O L0 em `color.md:20-33` está desatualizado: ainda legitima `luma()`, fallback
branco, alpha nomeado e a referência ao snapshot histórico como vanilla. Os
critérios `:165-168` repetem essas expectativas. O consumer em
`color.rs:138-191` materializa exatamente esse contrato antigo.

Portanto, o P1358 deve atualizar primeiro o L0. A mudança remove chamadas
antes aceitas (`luma()` e `alpha:`) e muda fallback para erro; isso é quebra de
compatibilidade pública. Pelo ADR-0127, a atualização de L0 deve ser seguida de
uma única confirmação humana antes de escrever RED/código. Esse checkpoint é
uma escolha real de produto, não uma política de bloqueio metodológico.

## Hipótese e refutação

Hipótese: substituir o parser especial de `native_luma` pelo mesmo cast
`Component` já usado por `rgb`, com primeiro positional obrigatório e segundo
opcional, fecha o lote sem alterar `Color` nem `luma(color)`.

Refutariam a hipótese: um caso vanilla ratificado em que alpha nomeado seja
aceito; um caso em que lightness inválida ainda devolva branco; necessidade de
alterar entidade pública `Color`; ou regressão observável de `luma(color)`.

