# Passo 1287 — relatório final de paridade global

**Veredito:** **PARIDADE PARCIAL**

**Instante de fechamento:** `2026-08-30T23:02:44-03:00`  
**HEAD:** `53d21c5a602f4045a769a0ab0c935baa5ecd3b88`  
**Árvore:** working tree não commitida e compartilhada  
**Regime:** materialização Tekt completa, segregada por papel, capacidade,
ordem causal e hashes; sem alegação de isolamento ambiental forte.

## Decisão

O corpus bilateral congelado encontrou diferenças de linguagem reproduzíveis.
Em particular, o candidato rejeita construções aceitas pelo vanilla em tabela/
grid com alinhamento por eixo (C05), link interno por label (C07), imagem com
argumento nomeado `format` (C08) e exportação HTML de conteúdo válido (E11).
Essas testemunhas satisfazem a regra global 1 do contrato P1287. Os resultados
`Unknown` restantes limitam cobertura, mas não apagam divergências já
demonstradas.

A matriz focal anterior continua útil como regressão local, porém os seus 17
`MATCH` e 2 `DIFF` esperados não são usados como percentagem da linguagem nem
como prova global.

## Cadeia segregada e integridade

| Papel/artefato | Resultado | SHA-256 |
|---|---|---|
| Autor de baseline | 91 fontes vanilla, três formatos, com incapacidade explícita | `3bbba47235e4865bb3c9e788bb709215dd0a53297ee587d4334a01ca42126cd6` |
| Autor de contrato | contrato congelado antes dos oráculos | `4683d23a5a489e258931ea3389be809c7b4e5b143c4f7a66ec7210c4dbd4f7a5` |
| Manifesto | `P1287-GLOBAL-PARITY-v1` | `5d4cc9a6180300c8402be4a91b30104db08874f8540c9bc8af5b895a9fdf725b` |
| Autor de oráculos | runner, 12 fixtures e baseline congelados | `b8ba1fa8d3d7bf1a305deee400ed811b5e034888ec4aeb3797392a95ea97e109` |
| Runner global | verificações de hashes, forward e reverse | `745a4c470704de54f9a550d220a3092bfa8452e3b02cb7010846758eab36bd37` |
| Baseline de oráculo | 16 observáveis válidos e 7 `Unknown` | `ff09c8146b73684b2e746c631a6103817b6df7070aa8248e2d88dbca34749488` |
| Adversário | 12 mutantes válidos mortos; restauração confirmada | `0255970c6f858d69ae733f490c645c394a230f006c7ddee3419d9a65d081f8b0` |
| Plano adversarial | decisões M01–M12 congeladas | `af92a768ab0acdab681192c0525c01a96f56b21942915b10f8b3dc204925db3f` |

O score adversarial é `12 / 12 = 1.0`. Ele atesta o poder discriminatório das
12 decisões atacadas, não uma fração da linguagem. Os hashes protegidos foram
confirmados depois da campanha.

## Corpus bilateral amplo

Execução final em `2026-08-30T23:01:01-03:00`–`23:01:13-03:00`:

```text
python3 lab/parity/matrix/p1287_global.py \
  --binary lab/parity/matrix/p1287_candidate_adapter.py
```

O runner terminou com exit `1`: 13 casos `Violated` e 10 casos `Unknown`.
O adaptador de medição, SHA-256
`152fe7ae7f65dcd015a24951e33cd682bda8590609dd20ef3fcc7ad2c8dc7679`,
remove apenas flags vanilla de controle da medição que o CLI candidato não
aceita (`--creation-timestamp 0`, `--jobs 1`, `--diagnostic-format short` e
`--ppi 144`). Não emula query, features ou comportamento do produto; executa
`target/release/typst`, SHA-256
`acc52526e1c1cfde21c4583857330a6f64540f46caf1b7fa89c666da6deee64a`.
O adaptador foi introduzido após um teste RED por `FileNotFoundError`; os seus
4 testes e os 6 testes do runner global passam.

| Caso | Cobertura | Classificação e testemunho |
|---|---|---|
| C01 | ASCII, Unicode combinante, não latino, emoji | `Violated`: morfologia PNG canônica difere |
| C02 | bidi LTR/RTL | `Violated`: morfologia PNG canônica difere |
| C03 | bold/italic/emphasis aninhados | `Violated`: morfologia PNG canônica difere |
| C04 | matemática inline/display, scripts, frações, símbolos | `Violated`: morfologia PNG canônica difere |
| C05 | tabelas/grids, span e alinhamento | `Violated`: candidato rejeita array de alinhamento aceito pelo vanilla |
| C06 | colunas, place/floats e notas | `Violated`: morfologia PNG canônica difere |
| C07 | contadores, labels, referências e links | `Violated`: candidato rejeita `link(label)` aceito pelo vanilla |
| C08 | raster, SVG/SVGZ e formatos/dependências | `Violated`: candidato rejeita `image(format:)` aceito pelo vanilla |
| C09 | linear/radial/conic/tiling | `Violated`: morfologia PNG canônica difere |
| C10 | paginação simples/múltipla | `Unknown`: digest forward/reverse do candidato diverge; observação semântica manual abaixo |
| C11 | queries metadata/heading/figure/vazia | `Unknown`: carrier HTML/CLI query exigido não está bilateralmente disponível neste runner |
| C12 | eval e diagnósticos válidos/inválidos | `Violated`: valor válido coincide, mas diagnóstico público canônico difere |

Nos exportadores, E01 e E03 são `Violated`; E02 e E04–E10 permanecem
`Unknown`; E11 é `Violated`. Não houve promoção de observação unilateral,
saída malformada ou carrier incompleto.

## Adjudicação PNG

O focal plain produz imagens `1191 × 1684` RGBA. Existem exatamente 11 pixels
divergentes, todos em cobertura de borda do mesmo glyph; o delta máximo por
canal é 1 e a média absoluta por canal é `4.113392e-06`. As coordenadas são:

```text
(232,147), (233,147), (233,148), (233,149), (233,150), (246,150),
(233,151), (233,152), (233,153), (233,154), (233,155)
```

Em dez coordenadas o vanilla contém `(2,2,2,255)` e o candidato
`(1,1,1,255)`; em `(246,150)` ocorre `(1,1,1,255)` versus `(2,2,2,255)`.
Os streams PDF do mesmo focal posicionam o texto em `y=763.78564` no vanilla
e `y=763.78562` no candidato, diferença de `0.00002 pt`; a borda de caixa
difere pelo mesmo quantum. A implementação de cobertura/blend observada é a
mesma. A causa é quantização subpixel de borda raster, mecânica permitida pela
lente ADR-0107, sem mudança de conteúdo, ordem, cor semântica ou geometria
intencional.

Contudo, o autor independente não congelou antes do candidato a máscara
excepcional exigida pelo contrato. Portanto E01 permanece `Violated` pelo
comparador RGBA exato. Nenhuma tolerância geral foi criada e a adjudicação não
é transferível a outra fixture, dimensão, PPI ou exportador.

## Adjudicação PDF

No focal plain, ambos os lados têm uma página A4, texto extraído
`Hello, parity.`, boxes equivalentes, fonte embedded/subset/Unicode e encoding
`Identity-H`, além de `/Marked true` e `/StructTreeRoot`. A diferença nominal é:

```text
vanilla:    LibertinusSerif-Regular-Identity-H
candidato:  LibertinusSerif-Regular
```

Esse sufixo nominal é metadata mecânica: não muda o texto, a seleção de face,
o subset, o mapeamento Unicode, a paginação ou a estrutura acessível observada.
Ele fica registrado como diferença de nome, sem ser convertido em divergência
de linguagem e sem generalizar acessibilidade para outros PDFs.

No carrier amplo E02, o candidato gera PDF recuperável, mas `qpdf --check`
reporta múltiplas entradas xref com offset zero e termina com aviso. O contrato
congelado trata saída que o parser não aceita limpidamente como `Unknown`; este
resultado não recebe crédito. A evidência de dano xref é um defeito de formato
público separado a corrigir, mas não foi necessária para decidir o veredito.
C10 também fica `Unknown` no runner por instabilidade de digest; a inspeção
manual conservadora observou texto, páginas, tamanho e tagging equivalentes e
somente a diferença nominal de fonte acima.

## Adjudicação SVG

| Eixo | Evidência disponível | Decisão P1287 |
|---|---|---|
| Plain determinístico | XML bilateral produzido, mas grafo canônico difere | E03 `Violated` |
| Linear/Radial × Luma/CMYK | carrier congelado não separa o produto completo | E04 `Unknown` |
| Conic por espaço suportado | apenas Conic/RGB no carrier | E05 `Unknown` |
| Tiling conteúdo/imagem/gradiente | somente conteúdo no carrier congelado; vanilla de conteúdo contém NUL | E06 `Unknown` |
| links mesma página/cross-page/bundle | não há carrier cross-file congelado bilateral | E07 `Unknown` |
| PNG/JPEG/GIF/WebP/SVG/SVGZ/PDF/external | produto de formatos/dependências incompleto | E08 `Unknown` |
| colisões de glyph/fonte ausente/fontless | só aviso de fonte ausente está no carrier | E09 `Unknown` |
| alfa/máscaras/nested clip/even-odd | há alfa e rounded clip, faltam máscara/nested/even-odd | E10 `Unknown` |

Certificados anteriores pinados pelo manifesto preservam fragmentos já
medidos — gradientes polares promovidos, tiling por conteúdo, links locais,
imagens comuns/nested SVG, glyph direto e clipping geométrico nonzero. Eles não
preenchem automaticamente os produtos residuais acima. A saída vanilla com NUL
em tiling por conteúdo é defeito do próprio baseline e nunca conta como
equivalência bilateral.

## Inventários padrão e HTML

Os inventários foram regenerados com o enumerador release SHA-256
`448798225102458f64206387179b1dfab9a305174c900bdc4aa32df228482c21`.
Os catálogos vanilla contêm 2.015 entradas no perfil padrão e 2.130 no HTML;
os catálogos candidatos contêm 1.983 e 2.047, respectivamente.

| Perfil | MATCH | MISSING_MEMBER | UNVERIFIED_METADATA | EXTRA_BINDING | MISSING_BINDING | WRONG_KIND | UNKNOWN | SHA-256 |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---|
| padrão | 1552 | 71 | 386 | 45 | 0 | 0 | 0 | `a9233711b83be0af44464271dfda9479412569fe791a585a7e75c9e4aa1489b8` |
| HTML | 1553 | 122 | 449 | 45 | 0 | 0 | 0 | `68efbabc8cc19c3b0bbcd52ab752ad55561b0544727d9e51b483d612fc846b7b` |

Os probes padrão executaram 111 casos: 72 iguais e 39 diferentes ou
desabilitados; SHA-256
`fdd50f61f4d0c5555e30ed80b3e0548540bed7f84ea90623417a39bc2975d953`.
Os probes HTML executaram 115 casos: 77 iguais e 38 diferentes ou
desabilitados; SHA-256
`1c4e772745edbda4dff1266d486e6dea46aab2607290955d0bbed32e936fcbc1`.
As contagens são um inventário de superfície e nunca uma percentagem de
paridade da linguagem.

## Matriz focal e validação final

A repetição final da matriz focal ocorreu entre
`2026-08-30T23:01:38-03:00` e `23:01:53-03:00`:

```text
python3 lab/parity/matrix/runner.py \
  --vanilla /usr/local/bin/typst \
  --crystalline target/release/typst \
  --output /tmp/p1287-matrix-final.json
```

Resultado: 17 `MATCH`, 2 `DIFF` esperados, 19 expectativas satisfeitas; exit
`0`. Runner SHA-256
`25e025b80d9c19fb1bce2e3d3c6f7caa9d0a680c63a17e9d6e5c6e27e0da8d5b`;
manifesto focal SHA-256
`9cdcb75527d5c0f2ae5c69cb2d7238c0c558c5fc27f65ef149b8e98c3fee241e`.
O JSON desta repetição tem SHA-256
`becd9e9fc83714dcb5a6b95c079a5af936e5783edd24d46b4d56d0883b422ded`;
o seu timestamp torna o hash específico desta execução.

| Gate | Resultado |
|---|---|
| `cargo test --workspace` | exit 0; suíte integral passou |
| testes Python de runners/inventário | 90 testes, zero falhas |
| `run_p1286_oracles.py --binary target/debug/typst` | exit 0; 58 `Preserved`, 2 `BaselineOnly`, 1 `NotApplicable`, zero `Unknown`/`Violated` |
| inventários padrão e HTML | executados bilateralmente; resíduos classificados acima |
| corpus amplo P1287 | exit 1 deliberado; diferenças e incapacidade classificadas |
| matriz focal | exit 0; expectativas satisfeitas |
| `cargo build --workspace --bin typst` | exit 0 |
| `cargo fmt --all -- --check` | exit 0 |
| `crystalline-lint .` | exit 0; sem violações fatais, com warnings preexistentes |
| `crystalline-lint --checks v5,v15,v26 --fail-on warning .` | exit 0; `No violations found` |
| `git diff --check` | exit 0 |

## Proveniência reproduzível

O baseline executável é `/usr/local/bin/typst`, SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
A associação independente desses bytes ao upstream/main `a51e02804` permanece
`Unknown`; isso bloquearia uma alegação global positiva, mas não refuta as
divergências bilaterais observadas contra estes bytes pinados.

No snapshot de `2026-08-30T23:02:44-03:00`, `git diff HEAD --stat` registrou
`124 files changed, 647273 insertions(+), 1789 deletions(-)`, SHA-256
`8b22c5e3e6a3427dde83ddce564dfa45986b4d49f341a92be0619104f4ec1d5c`.
O SHA-256 de `git status --short` foi
`4cb24cbb3847760781336fb72c04da84aeca1ba88b57bb47c1abf4fbdc683b71`.
Os arquivos P1287 são novos e a árvore inclui mudanças dos passos 1281–1286;
nenhuma autoria isolada da árvore inteira é alegada.

O Passo 1287 não alterou código produtivo nem Prompt L0. A única implementação
nova é o adaptador de laboratório e seu teste RED→GREEN; portanto não houve
mudança de contrato público, comportamento padrão, fase de pipeline ou quebra
de compatibilidade que acionasse uma nova parada ADR-0127.
