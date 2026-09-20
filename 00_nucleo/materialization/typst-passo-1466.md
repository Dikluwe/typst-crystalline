# Passo 1466 — restaurar a baseline verde e eliminar o estouro PNG

**Estado:** L0 RETIFICADOS — AGUARDA CONFIRMAÇÃO NO GATE ADR-0127

**Data da medição:** 2026-09-20T13:26:04-03:00

**Baseline:** `23f53380fb15dcac13861099ef06a83b2484c281`

**Tree:** `193a7ab6eda64f47e10985f3f50068c2e89f85f6`

**Estado do workspace:** limpo (`git status --short` sem saída)

**Vanilla ratificado:** `586e1bd43fae6c9a973218163d3165c53ab8d16d`

## 1. Objetivo

P1466 restaura uma baseline executável verde depois da materialização de
`format.svg`, `format.png`, `format.pdf` e `format.html`. O passo corrige o
defeito produtivo que transforma o default PNG de 144 PPI em 144 pixels por
ponto, resolve duas candidatas funcionais ainda abertas e retifica testes que
continuam fiscalizando estados anteriores já substituídos por contratos mais
novos.

O encerramento exige simultaneamente:

- export PNG normal sem tentativa de alocação de dezenas de gigabytes;
- zero falhas na suíte completa;
- zero erros do Clippy;
- `cargo fmt --all -- --check` verde;
- nenhuma expectativa antiga enfraquecida apenas para obter verde.

Este passo não adiciona opção de formato, target, flag, elemento, carrier nem
fase de pipeline. Não implementa `format.pdf.tagged`, `format.pdf.pretty`,
seleção de páginas ou conformidade PDF.

## 2. Inserção no roadmap sem apagar história

O L0 vigente reservava o número P1466 para opções PDF ainda não
materializadas. A correção de estabilidade tem precedência porque o default
PNG derruba a validação e pode derrubar o processo controlador. Antes de
qualquer código, atualizar apenas o roadmap corrente:

- P1466: esta estabilização;
- P1467: `format.pdf.tagged` e `format.pdf.pretty`;
- P1468: `format.pdf.pages` e `--pages`;
- P1469: `format.pdf.standard`, `--pdf-standard` e conformidade real.

Os passos históricos P1464 e P1465 permanecem intactos. A renumeração deve ser
registrada no L0 vigente de `compiler/stdlib/format.md`, em `wiring.md` e nos
prompts de teste correntes que ainda dizem “até P1466”. Não reescrever o relato
histórico para fingir que a reserva anterior nunca existiu.

## 3. Evidência medida antes da decisão

### 3.1 Resultado global reproduzido

No conteúdo produtivo e de testes que se tornou o commit baseline, o comando

```text
cargo test --workspace --no-fail-fast -- --test-threads=1
```

percorreu todos os alvos e terminou com 20 falhas:

| Alvo | Passou | Falhou |
|---|---:|---:|
| `typst-core --lib` | 5927 | 8 |
| `typst-infra --lib` | 960 | 6 |
| `p1427_math_frac` | 4 | 2 |
| `typst-wiring --test cli` | 72 | 3 |
| `typst-wiring --test p1293_contract` | 9 | 1 |

Os mesmos nomes e observáveis anteriores ao P1465 foram reproduzidos no
commit causal `e300314e2f87e90e543c4923a8cae97f9ad978a8`; portanto não são
regressões introduzidas por `format.html`, mas continuam sendo falhas reais da
baseline e não podem permanecer aceitas.

### 3.2 Defeito crítico: PPI público usado como pixels por ponto

O carrier de `DocumentFormatOptions` guarda o valor público `ppi == 144.0`.
Na ausência de override, `03_infra/src/pipeline.rs:1833-1836` entrega esse
número diretamente a `RenderOptions.pixel_per_pt`; o override CLI, porém, é
dividido por 72. O rasterizador multiplica a página em pontos por esse fator em
`03_infra/src/export/render.rs:94-100`.

Para a página A4 default, a aritmética vigente é:

```text
round(595.2755905511812 × 144) = 85720 px
round(841.8897637795276 × 144) = 121232 px
85720 × 121232 × 4             = 41568028160 bytes
```

Quatro testes distintos observaram exatamente
`memory allocation of 41568028160 bytes failed`. Com o significado documentado
de 144 pixels por polegada, o fator correto é `144 / 72 == 2` pixels por ponto:

```text
1191 × 1684 px; 8022576 bytes RGBA antes do encoder
```

A assimetria P1463 entre regra documental e CLI é uma reprodução da mecânica
do pin que contradiz o significado público “pixels por polegada” e torna o
default inviável. Pela ADR-0107, a unidade da linguagem vence essa mecânica.

### 3.3 Falhas com contrato posterior já conhecido

Treze falhas têm oráculo antigo ou fixture inválida diante de decisões já
materializadas:

1. `p1292_c_v7_math_vec_nao_recursa_nem_duplica_markup` congela a repr
   mecânica antiga de `MathFrac`, embora o L0 de repr mande variantes
   estruturadas usarem o formatter próprio;
2. quatro testes P981/P1132r procuram `MathDelimited`, mas P1436 tornou
   `MathLr` o carrier público;
3. dois testes HTML ainda exigem ausência de `html.frame`, materializado por
   P1424;
4. três fixtures L3 passam `"red"`/`"blue"` onde a linguagem vigente exige
   valores de cor, impedindo que o teste alcance a geometria pretendida;
5. `p937_covering_all_filtra_por_coverage_exacta` compara o índice global do
   slot com o índice local denso introduzido por P1404;
6. os dois observadores falhos de P1427 já foram substituídos por P1428, cujo
   teste funcional passa 4/4: texto HTML decodificado e bits causais da fração;

Retificar esses testes não significa trocar “esperado” pelo output candidato.
Cada alteração deve apontar para o contrato posterior que a causou e manter um
observável público discriminatório.

### 3.4 Duas candidatas produtivas não classificadas por conveniência

Estas três falhas exigem medição bilateral antes de decidir:

- `p1132r_sec22_brackets_invertidos_usam_as_variantes_escalaveis`: a saída
  contém apenas o par normal; `layout_lr` reconhece `MathDelimited`, mas o
  `MathLr` público transporta cercas posicionais dentro do body. Se o vanilla
  mantém os quatro glifos, corrigir `compiler/math/layout/delimited.rs`, não o
  teste;
- `p1159_location_page_numbering_ve_func_no_relayout`: medir o valor contextual
  e sua repr no vanilla e no cristalino. Se o pin produz `(..) => ..`, reparar
  a perda na pipeline/introspecção/repr. Só atualizar o teste se a fonte
  ratificada provar que a expectativa antiga não pertence à linguagem.
- `cases_nao_panica_com_align_point`: em 2026-09-20 o vanilla pinado aceitou
  `$ cases(x &, 0 &) $`; portanto preservar a fixture e corrigir o primeiro
  ponto do eval que impede `MathAlignPoint` de chegar ao layout.

### 3.5 Análise estática e formatação

O Clippy para em seis erros, reproduzidos no baseline causal com os mesmos
arquivos e linhas:

- um `deprecated_since` não semver em `entities/layout_types.rs`;
- dois `never_loop` em `compiler/stdlib/structural/math.rs`;
- um `never_loop` em `compiler/stdlib/math_style.rs`;
- dois `absurd_extreme_comparisons` no constructor de `duration`.

`cargo fmt --all -- --check` encontra somente dois arquivos fora do diff
P1465: `03_infra/tests/p1428_math_frac.rs` e
`04_wiring/tests/p1289_float_is_infinite.rs`.

## 4. Decisões do passo

### 4.1 Unidade PNG única

O valor público e documental continua expresso em PPI e conserva default
`144.0`. A pipeline resolve ambos os caminhos pela mesma conversão:

```text
effective_ppi = ppi_override.unwrap_or(document.format_options.png.ppi)
pixel_per_pt  = effective_ppi / 72.0
```

`RenderOptions` continua recebendo pixels por ponto, com default interno 2.0.
L4 continua transportando apenas `Option<f64>` e não converte unidades. O
collector continua guardando a unidade pública, sem `Option`, sentinel ou
estado mecânico adicional.

Retificar os L0s P1463 que hoje ordenam a assimetria, atualizar a matriz IHDR
de `p1463_format_png.rs` e acrescentar um caso A4 default que prove dimensões
`1191 × 1684` sem comparar bytes comprimidos. CLI e regra documental com o
mesmo PPI devem produzir as mesmas dimensões; override explícito continua
vencendo o documento.

### 4.2 Testes antigos devem seguir o dono atual

- Repr de filho estruturado usa a repr pública vigente da variante, sem
  recuperar sintaxe mecânica antiga.
- Testes de eval reconhecem `MathLr` e inspecionam seu body; não exigem que o
  carrier volte a `MathDelimited`.
- Testes P1168/P1423 verificam seus subconjuntos históricos sem afirmar que o
  módulo global atual termina naquele lote; P1424 continua dono de `frame`.
- Fixtures de cor usam valores de linguagem válidos e conservam as mesmas
  asserções geométricas/PDF.
- O teste de fallback diferencia índice local denso de slot global e prova a
  resolução do slot correto.
- Remover somente os dois observadores P1427 formalmente substituídos por
  P1428, preservando os quatro casos P1427 ainda úteis e executando P1428 como
  gate obrigatório.
- A fixture `cases` só pode ser trocada depois de medir sintaxe aceita no pin;
  a medição P1466 confirmou que a fixture vigente é válida. Ela deve continuar
  alcançando layout com align points, não virar um teste vazio.

### 4.3 Candidatas produtivas

Para cercas invertidas, preservar o carrier `MathLr` e ensinar o layout a
classificar a primeira e a última cerca pela posição, inclusive quando suas
classes naturais são Closing/Opening. Não baixar novamente para
`MathDelimited`, não fabricar texto e não alterar `math.mid` ou tamanhos.

Para `page-numbering`, registrar a saída bilateral e o primeiro ponto onde o
valor se perde. A correção, se necessária, fica no owner já responsável pela
perda; não adicionar nova fase nem executar relayout apenas para satisfazer o
teste.

### 4.4 Clippy e formato

- usar versão semver válida no atributo deprecated sem alterar a mensagem;
- substituir loops que retornam sempre na primeira iteração por seleção
  explícita do primeiro named inválido, preservando ordem e diagnóstico;
- remover comparações impossíveis de um total já representado em `i128`, sem
  estreitar aritmética nem suprimir overflow real;
- aplicar `cargo fmt` somente às diferenças mecânicas apontadas e ressellar os
  hashes dos consumers afetados.

## 5. L0s que devem ser atualizados antes de código

| Prompt L0 | Motivo |
|---|---|
| `entities/document_format_options.md` | retirar a alegação de que PPI documental é pixels por ponto |
| `compiler/layout/format_options.md` | afirmar que o collector preserva PPI, não escala de raster |
| `infra/pipeline.md` | resolver documento e CLI por `/72` e cobrir A4 default |
| `wiring/tests/p1463-format-png.md` | substituir a matriz IHDR assimétrica |
| `compiler/math/layout/delimited.md` | cercas posicionais do `MathLr` |
| `compiler/eval/tests.md` | observador P981/P1132r no carrier atual |
| `compiler/eval/repr.md` | repr de filho estruturado de vec |
| `compiler/layout/tests.md` | fixture válida de `cases` |
| `compiler/eval/math.md` | preservar `MathAlignPoint` nos argumentos de `cases` |
| `compiler/stdlib/html.md` e `compiler/stdlib/html/p1423-tests.md` | remover negativos obsoletos contra P1424 |
| `infra/integration_tests.md` | fixtures de cor e witness P1132r |
| `infra/shaper.md` | índice local denso versus slot global |
| `infra/tests/p1427-tests.md` | registrar substituição dos dois observadores por P1428 |
| `infra/export/render.md` | documentar que recebe pixels por ponto já resolvidos |
| `entities/layout_types.md` | atributo deprecated semver |
| `compiler/stdlib/structural/math.md` | validação named sem loop fictício |
| `compiler/stdlib/math_style.md` | validação named sem loop fictício |
| `compiler/stdlib/primitives-constructors/duration.md` | aritmética `i128` sem comparação impossível |
| `wiring/tests/cli.md` e `wiring/tests/p1293_contract.md` | regressões PNG default |
| `wiring/tests/p1465-format-html.md` | deslocar a reserva PDF para P1467 |

Os prompts de P1428 e P1289 só precisam de novo `Hash do Código` se a
formatação alterar seus consumers; não inventar mudança normativa.

## 6. Gate ADR-0127

A correção muda o comportamento por defeito do target PNG de uma tentativa de
alocação de aproximadamente 40,5 GB para export normal a 144 PPI. Portanto a
paragem é obrigatória:

1. escrever e revisar este passo;
2. atualizar os L0s da seção 5, inclusive o deslocamento do roadmap PDF;
3. ressellar hashes, validar V5/V15/V26 e criar commit causal somente de
   documentação;
4. **parar para confirmação humana posterior ao commit causal**;
5. depois da confirmação, produzir RED focal, corrigir produto e testes e
   revalidar a suíte completa.

Nenhuma confirmação de P1465 autoriza a implementação P1466.

## 7. Ordem de execução após o gate

1. Corrigir unidade PNG e executar primeiro os quatro testes que hoje abortam.
2. Corrigir as duas candidatas produtivas conforme a medição bilateral.
3. Retificar os treze testes obsoletos, um cluster causal por vez.
4. Corrigir os seis erros do Clippy e os dois desvios de formatação.
5. Executar focais de cada owner e só então a suíte completa sem fail-fast.
6. Atualizar este passo com resultados e proveniência; não criar relatório,
   recibo, selo, certificado, manifest ou catálogo paralelo.

## 8. Gates finais

```text
cargo fmt --all -- --check
cargo test -p typst-core --lib -- --test-threads=1
cargo test -p typst-infra --lib -- --test-threads=1
cargo test -p typst-infra --test p1427_math_frac -- --test-threads=1
cargo test -p typst-infra --test p1428_math_frac -- --test-threads=1
cargo test -p typst-wiring --test cli -- --test-threads=1
cargo test -p typst-wiring --test p1293_contract -- --test-threads=1
cargo test -p typst-wiring --test p1463_format_png -- --test-threads=1
cargo test --workspace --no-fail-fast -- --test-threads=1
cargo clippy --workspace --all-targets --all-features
cargo build --workspace
crystalline-lint .
crystalline-lint --checks v5,v15,v26 --fail-on warning .
verificação de referências quebradas
git diff --check
```

Sucesso exige zero testes falhos e zero erros do Clippy. Aviso, timeout, OOM,
processo morto, ausência de artefato ou teste não executado não contam como
verde.

## 9. RAM, temporários e proveniência

Toda compilação, cache, fixture, clone causal, log e temporário descartável
usa exclusivamente `/dev/shm`, sem fallback para o HD. Cada lote usa:

```text
ulimit -v 25165824
CARGO_BUILD_JOBS=1
CARGO_INCREMENTAL=0
RUST_TEST_THREADS=1
```

Verificar espaço de `/dev/shm` e `MemAvailable` antes de cada lote. O limite de
24 GiB fica abaixo do teto de 32 GiB acordado e deve alcançar também
subprocessos. Se faltar espaço ou o limite for atingido, parar e reportar
`Unknown`; não elevar o teto para repetir a alocação de 41.568.028.160 bytes.

Qualquer novo número usado para decidir fechamento deve registrar commit ou
working tree exato, `git diff HEAD --stat` e hora quando o estado puder ter
mudado. Ao terminar, remover somente as raízes P1466 criadas em `/dev/shm` e
confirmar workspace limpo.

## 10. Integridade dos arquivos medidos

| Arquivo | SHA-256 no baseline |
|---|---|
| `01_core/src/entities/document_format_options.rs` | `18407c191d40d54624b852ddcfdac75ef18724d8c8178e4057801a4b3fbd60cb` |
| `01_core/src/compiler/layout/format_options.rs` | `46cc0d82fbefdb6a74dd760c7c2e01b99f83cd2ce68b2b2000c7478a1bf23f18` |
| `03_infra/src/pipeline.rs` | `7ccdd84e785d16bfba4f1aeaf4b8dcfe4670cd60864cfce5d83966c8b28f60b2` |
| `03_infra/src/export/render.rs` | `74d94a63e1ee1cc8bd4875a5e915305bc6a6bf35e77f5f93e17d15f6e9e70901` |
| `04_wiring/tests/p1463_format_png.rs` | `686a52d7f68b95242b600b4a1b3b809062d19b4ed7ddf34977fdd7b4218ea5fb` |
| `01_core/src/compiler/eval/tests.rs` | `5d4b8a00fa8d942de7160780ded4734a0ae4bce616ac44e367bd8f5640c9bb59` |
| `01_core/src/compiler/eval/repr.rs` | `fb54cb46c06a2567621d2a94bcf7cddc675a2af0978e2116f2a61a129214d5ee` |
| `01_core/src/compiler/layout/tests.rs` | `091c7cdc7c425b9ff1a5be690dd7fa8a8b05744a7ef9439af16158903f5f0698` |
| `01_core/src/compiler/stdlib/html.rs` | `6a1085e58997912e7c4fba055307b3f86ed63a20b963f6796c739310aee69632` |
| `01_core/src/compiler/stdlib/html/p1423_tests.rs` | `eba0351ceb0e126cb27c54da96df5a05c1ee406820a014ad026a427856d8e91b` |
| `03_infra/src/integration_tests.rs` | `c19ee54fd5df53af1e1bce7f5d6edd137dffaf0c6b2b766a3e9f5eb60cd37509` |
| `03_infra/src/shaper.rs` | `56f244dfda6e735787c316fe3fb2eb9ac5373dbed18d0af31a5a291fe1e4664c` |
| `03_infra/tests/p1427_math_frac.rs` | `708a39bc4da195ecfab55bdac0c780a9161a353fb1889375cf1b6ef950626ff7` |
| `04_wiring/tests/cli.rs` | `59c66cc635bbeaeaf8aef6789aaf2cd451121e8533935b883e3929bb9c683375` |
| `04_wiring/tests/p1293_contract.rs` | `ffd152c0373f5efd435f54d4a77f60fd24713d6125597424fad0e88737cde6ab` |
