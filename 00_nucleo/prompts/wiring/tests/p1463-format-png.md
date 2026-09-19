# Prompt L0 — `wiring/tests/p1463-format-png`

**Estado:** CONTRATO DE TESTE P1463 PROPOSTO — AGUARDA CONFIRMAÇÃO ADR-0127;
sem consumer e sem `Hash do Código` até o selo posterior ao gate.

**Camada:** L4 (testes de integração)
**Alvo planejado:** `04_wiring/tests/p1463_format_png.rs`
**Consumer futuro exclusivo:** `04_wiring/tests/p1463_format_png.rs`
**Vanilla ratificado:** `586e1bd43fae6c9a973218163d3165c53ab8d16d`
**ADRs:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0131

## Responsabilidade

Executar `CARGO_BIN_EXE_typst` como processo externo e provar o fragmento
integrado `format.png`/`ppi`. O harness não importa módulos privados, não
implementa precedência produtiva e não legitima código L1–L3.

## Fixture e observável

Usar página quadrada de 1 pt, margem zero e forma opaca determinística. Ler
largura/altura diretamente do IHDR PNG; não comparar bytes comprimidos,
metadados, tamanho do ficheiro ou hash como semântica.

A matriz positiva mínima é:

| Documento | CLI | Dimensão esperada |
|---|---|---:|
| sem regra | ausente | 144 × 144 |
| `ppi: 72` | ausente | 72 × 72 |
| `ppi: 2` | ausente | 2 × 2 |
| `ppi: 1.5` | ausente | 2 × 2 após arredondamento vanilla |
| sem regra | `--ppi 72` | 1 × 1 |
| sem regra | `--ppi 288` | 4 × 4 |
| `ppi: 288` | `--ppi 72` | 1 × 1 |

Essa matriz congela a assimetria medida: documento segue cru e caller CLI é
dividido por 72. Valores zero e negativo finito produzem dimensão mínima 1 ×
1; não usar `+inf`, NaN ou corpus que possa causar OOM.

## Linguagem, cascata e diagnósticos

Cobrir também:

- `type(format.png)`, nome, `ppi == 144.0`, alias e `std.format.png`;
- chamada direta falha com `cannot be constructed manually`;
- integer e float são aceitos;
- regra `ppi: 72` seguida de regra vazia preserva 72;
- 72→288 e 288→72 provam last-write-wins;
- nested falha com `format set rules are not allowed inside of containers` e
  não publica artefato;
- named desconhecido, string, boolean, positional e duplicado falham sem o
  warning legado de target vazio;
- `--ppi texto` falha no parser com exit 2.

Os asserts verificam exit, stdout/stderr, existência do artefato, assinatura
PNG e IHDR. Spans são comparados apenas onde o contrato público os congela.

## Controles de fronteira

- com `--ppi` ausente, `format.svg.pretty` conserva toda a matriz P1462;
- `--ppi 72` não altera bytes SVG da mesma fixture nem comportamento PDF/HTML;
- `format.png.ppi` não altera SVG/PDF/HTML;
- `format.pdf` e `format.html` continuam ausentes;
- `--pretty`, `--compact`, tags, features e timings permanecem ortogonais;
- aliases `compile`/`watch`/legado aceitam a flag, mas comandos alheios não.

Testes unitários L3, não esta suíte, distinguem os ramos PNG com e sem fontes.
P1357 e V15/V26 são gates externos e não são simulados pelo harness.

## Mutação e Unknown

O conjunto discriminatório deve rejeitar: documento dividido por 72, CLI cru,
precedência invertida, `None` colapsado, default 2/72, regra vazia destrutiva,
nested aceito, vazamento de target e sibling antecipado. Timeout do caso,
crash, PNG inválido, binário não pinado ou saída insuficiente é `Unknown`, não
sucesso. Interrupção externa é `Interrupted` conforme ADR-0131.

## Gate

O teste RED só pode ser criado depois da confirmação humana explícita
posterior ao commit causal P1463 e depois do selo discriminatório completo.
Então o header apontará exclusivamente para este prompt, RED precederá o
candidato e o verificador final repetirá os casos de maior risco.
