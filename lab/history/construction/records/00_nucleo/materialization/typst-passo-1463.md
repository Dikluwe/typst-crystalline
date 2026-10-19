# Passo 1463 — materializar `format.png` e transportar `ppi` até o raster

> **Nota de preservação (limpeza de 2026-09-19):** este passo é mantido como
> registro histórico. Menções a selos, recibos, certificados, manifests, hashes
> administrativos, papéis de agentes e gates do protocolo revogado não criam
> requisitos vigentes. Os artefatos removidos foram consolidados em
> relatório consolidado da limpeza; o comportamento normativo
> permanece nos Prompts L0 e nos testes funcionais atuais.


**Estado:** L0 CAUSAL ESCRITO — AGUARDA CONFIRMAÇÃO HUMANA PÓS-COMMIT
(ADR-0127)
**Data da medição:** 2026-09-19T18:30:39-03:00
**Baseline cristalino:** `23cd47e4ade4d6deb7095cb7400478feffb1e9f9`
**Tree do baseline:** `9ab3a99f61cdd3f673b7b3f2bd042e260b841430`, limpa
**Vanilla ratificado:** `586e1bd43fae6c9a973218163d3165c53ab8d16d`
**identificador administrativo do binário vanilla:**
[dado administrativo removido]
**identificador administrativo do binário cristalino:**
[dado administrativo removido]

**Regime desta escrita:** planeamento, sem alteração normativa nem código. A
execução posterior usa protocolo completo de materialização segregada, com
checkpoints reutilizáveis de protocolo de checkpoint revogado e a alegação proporcional
`executado sem atestação de isolamento` no workspace compartilhado.

## 1. Objetivo e fronteira

Completar somente o segundo filho da raiz pública `format`:

```text
format.png
└── ppi: float = 144.0
```

O passo torna `format.png` uma função/elemento settable não construtível,
valida `#set format.png(ppi: ...)`, transporta a decisão documental até o
raster PNG e acrescenta a flag CLI `--ppi`. A precedência pública é:

```text
--ppi explícito > format.png.ppi do documento > 144.0
```

P1463 não implementa `format.pdf`, `format.html`, seleção de páginas, export
PNG multipágina, templates de nome, novo encoder, novo target, aliases globais
nem feature gate. O pipeline cristalino continua exportando a primeira página,
como já faz; o veredito será limitado ao fragmento `format.png`/`ppi`.

`format.svg` materializado e certificado em P1462 deve permanecer intacto.
P1464 e P1465 continuam donos exclusivos de `format.pdf` e `format.html`.

## 2. Medição anterior à decisão

### 2.1 Superfície pública da linguagem

No vanilla ratificado, o probe:

```typ
#repr((type(format.png), format.png, format.png.ppi))
```

produz `(function, png, 144.0)`. `format.png()` falha com
`cannot be constructed manually`; alias lexical e `std.format.png` preservam
a mesma identidade nominal e namespace. No cristalino P1462, a raiz contém
somente `svg` e `format.png` falha com `module format does not contain png`.

As sondas de set rule no vanilla mediram:

| Entrada | Resultado público |
|---|---|
| `#set format.png(ppi: 72)` | aceita inteiro por coerção para float |
| `#set format.png(ppi: 1.5)` | aceita float fracionário |
| `#set format.png()` | aceita e não escreve o default |
| `#set format.png(nope: 72)` | `unexpected argument: nope` |
| `#set format.png(ppi: "72")` | `expected float, found string` |
| `#set format.png(ppi: true)` | `expected float, found boolean` |
| `#set format.png(72)` | `unexpected argument` |
| `#set format.png(ppi: 72, ppi: 144)` | `duplicate argument: ppi` |
| regra dentro de `block` | `format set rules are not allowed inside of containers` |

Uma regra vazia depois de `ppi: 72` preservou 72. Repetição nas duas ordens
confirmou last-write-wins: 72→288 terminou em 288 e 288→72 terminou em 72.
Portanto named omitido é ausência, não escrita implícita de 144.

No baseline cristalino, a mesma set rule segue ao fallback textual, emite o
warning histórico `set: target '' ainda não suportado` e não altera o raster.
Esse sucesso-com-warning é o RED: não pode ser aceito como implementação.

### 2.2 Assimetria observada entre documento e CLI

A fonte vanilla
`lab/typst-original/crates/typst-render/src/format.rs:81-115` declara
`ppi: Scalar` com default 144, mas armazena o campo documental diretamente em
`PngFormatOptions.pixel_per_pt`. Já
`lab/typst-original/crates/typst-cli/src/compile.rs:686-692` converte a flag
externa com `ppi / 72.0` antes da resolução. O raster lê o valor resolvido como
pixels por ponto (`typst-render/src/lib.rs:29-34,61-66`).

O comportamento do binário ratificado confirmou a assimetria numa página de
72 pt × 36 pt:

| Documento | CLI | PNG observado |
|---|---|---:|
| sem regra | ausente | 10368 × 5184 |
| `ppi: 72` | ausente | 5184 × 2592 |
| `ppi: 2` | ausente | 144 × 72 |
| `ppi: 1.5` | ausente | 108 × 54 |
| `ppi: 288` | ausente | 20736 × 10368 |
| `ppi: 288` | `--ppi 72` | 72 × 36 |
| sem regra | `--ppi 288` | 288 × 144 |

Isto é contraintuitivo, mas é o observável do vanilla pinado. P1463 não pode
dividir o valor documental por 72 nem passar a flag crua ao raster em nome de
uma unidade mais conveniente. Uma sincronização upstream futura pode reabrir
a decisão; este passo não antecipa essa mudança.

Para evitar consumo desnecessário, os testes finais usam páginas de 1 pt e
relações dimensionais equivalentes. Os números acima são evidência da fixture
medida, não exigência de tamanho de corpus.

### 2.3 Flag pública e domínio numérico medido

O vanilla anuncia:

```text
--ppi <PPI>
    The PPI (pixels per inch) to use for PNG export.
    [default: 144]
```

O valor sintático é `Option<f64>` e a ausência fica distinguível. Inteiros,
frações, zero e negativos finitos são aceitos pelo parser; zero e negativos
produziram bitmap 1 × 1 no corpus medido. A grafia negativa requer
`--ppi=-72` por regra normal do parser CLI. Texto não numérico falha em L2 com
exit 2.

`+inf` fez o vanilla abortar dentro do raster, portanto não é oráculo positivo
seguro e fica fora do corpus certificável de P1463. Crash, OOM ou timeout não
serão convertidos em `Preserved`; serão `Unknown` e não darão crédito. O passo
não alega paridade para não finitos.

A flag é aceita mas ignorada quando o target não é PNG. No probe SVG, com e
sem `--ppi 72`, o identificador administrativo foi identicamente
[dado administrativo removido].

O CLI cristalino atual não anuncia `--ppi` e devolve exit 2 para a flag.
Sem regra documental, sua página de 72 pt × 36 pt usa o
`RenderOptions::default()` vigente e produz 144 × 72.

### 2.4 Estado cristalino e owners vigentes

- `compiler/stdlib/format.rs` expõe somente `svg`;
- `compiler/eval/rules.rs` reconhece somente `format.svg`;
- `compiler/layout/format_options.rs` coleta somente
  `format.svg.pretty`;
- `DocumentFormatOptions` contém somente `svg`;
- `CompileIntent` não possui `ppi`;
- o braço PNG de L4 não transporta override;
- a pipeline PNG constrói sempre `RenderOptions::default()` nos ramos com e
  sem fontes;
- `infra/export/render.rs` já recebe `pixel_per_pt: f32` e não precisa aprender
  precedência, set rules ou CLI.

O Núcleo Tekt
`00_nucleo/prompts/_nuclei/export/format-option-precedence.toml` é
semanticamente correto para precedência, mas sua redação atual fixa
`Some(bool)` e o default específico de SVG. O identificador administrativo bruto medido é
[dado administrativo removido].
P1463 deve generalizá-lo para valor explícito tipado, sem mover para o Núcleo
a conversão PNG `/72`; essa regra pertence ao owner da pipeline.

Na materialização L0 causal, o Núcleo generalizado possui identificador administrativo bruto
[dado administrativo removido]
e identificador administrativo efetivo V26
[dado administrativo removido].
Os dois prompts consumidores foram repinados juntos; V26 deve permanecer
verde antes do commit.

Os L0s vigentes foram lidos integralmente antes desta proposta:

| Prompt | identificador administrativo atual |
|---|---|
| `compiler/stdlib/format.md` | [dado administrativo removido] |
| `compiler/eval/rules.md` | [dado administrativo removido] |
| `compiler/layout/format_options.md` | [dado administrativo removido] |
| `entities/document_format_options.md` | [dado administrativo removido] |
| `shell/cli.md` | [dado administrativo removido] |
| `infra/pipeline.md` | [dado administrativo removido] |
| `wiring.md` | [dado administrativo removido] |
| `infra/export/render.md` | [dado administrativo removido] |

`infra/export/render.md` permanece inalterado: o seu contrato já recebe
`pixel_per_pt`; a resolução da origem desse valor pertence à pipeline.

### 2.5 Proveniência das fontes medidas

| Fonte | identificador administrativo |
|---|---|
| `lab/typst-original/crates/typst-render/src/format.rs` | [dado administrativo removido] |
| `lab/typst-original/crates/typst-render/src/lib.rs` | [dado administrativo removido] |
| `lab/typst-original/crates/typst-library/src/format.rs` | [dado administrativo removido] |
| `lab/typst-original/crates/typst-cli/src/args.rs` | [dado administrativo removido] |
| `lab/typst-original/crates/typst-cli/src/compile.rs` | [dado administrativo removido] |

Todos os números desta seção vieram do HEAD/tree registrados no cabeçalho;
`git status --short` e `git diff HEAD --stat` estavam sem saída.

## 3. Classificação ADR-0107/0108

São língua e superfície pública: existência/tipo/nome/default de
`format.png`, campo público `ppi`, coerção numérica, validação e diagnósticos,
construção proibida, restrição a root, cascata lexical, precedência da flag e
dimensões resultantes do raster.

São mecânica livre: macros/traits upstream, `Scalar`, coleção parcial,
downcast, forma Rust do collector, número de passes, conversão final f64→f32,
encoder escolhido, bytes comprimidos e hash do PNG, desde que dimensões e
morfologia pedidas permaneçam corretas.

Inferência marcada: os carriers e o collector criados em P1462 bastam para
adicionar um campo PNG sem tocar `PagedDocument`, `layout/mod.rs`,
`entities/mod.rs` ou `infra/export/render.rs`. Refutam a inferência a perda de
ordem entre chaves SVG/PNG, incapacidade de representar float, divergência
entre os ramos com/sem fontes ou necessidade real de nova fase. Qualquer
refutador exige L0 novo antes de ampliar o diff.

## 4. Decisão L0 proposta

Antes de teste ou código produtivo:

1. `compiler/stdlib/format.md` acrescenta `png` ao mesmo módulo, com namespace
   `ppi: 144.0` e constructor manual sempre falho;
2. `compiler/eval/rules.md` reconhece estruturalmente `format.png`, aceita
   exatamente um named `ppi`, converte `integer|float` para `f64` e grava
   `format.png.ppi` somente quando presente;
3. `compiler/layout/format_options.md` coleta a nova chave com a mesma regra
   root/nested, last-write-wins e falha fechada de tipo interno;
4. `entities/document_format_options.md` acrescenta
   `PngFormatOptions { pub ppi: f64 }` e
   `DocumentFormatOptions { pub png: PngFormatOptions }`, default 144.0;
5. `shell/cli.md` acrescenta `--ppi <PPI>` e
   `CompileIntent::ppi: Option<f64>`, preservando ausência;
6. `infra/pipeline.md` acrescenta uma entry point PNG explícita com
   `ppi_override: Option<f64>`; fachadas antigas delegam `None`;
7. `wiring.md` transporta o `Option<f64>` somente ao braço PNG;
8. o Núcleo `format-option-precedence` passa a falar de valor explícito
   genérico e defaults por formato; `pipeline.md` e `wiring.md` atualizam o pin
   efetivo juntos;
9. criar `wiring/tests/p1463-format-png.md`, proprietário exclusivo do novo
   teste CLI `04_wiring/tests/p1463_format_png.rs`.

O valor efetivo entregue ao `RenderOptions` é deliberadamente:

```text
Some(cli_ppi) -> cli_ppi / 72.0
None          -> document.format_options.png.ppi
```

O default 144.0 já vive no carrier documental; não há colapso anterior em L2
ou L4. A conversão para `f32` ocorre na fronteira do raster, como no vanilla.
Os ramos com fontes e sem fontes recebem o mesmo `RenderOptions` resolvido.

Como `f64` não implementa `Eq`, o L0 de entidade deve declarar
`PngFormatOptions` com `PartialEq` e autorizar a remoção de `Eq` do derive de
`DocumentFormatOptions`; `SvgFormatOptions` pode conservar `Eq`. Não criar
igualdade total artificial apenas para conservar uma mecânica Rust que não é
observável da linguagem.

As linhas narrativas ainda marcadas como “P1462 aguarda selo” nos prompts
tocados devem ser normalizadas para “P1462 certificado” antes do resselo. Isso
não reabre o contrato P1462.

## 5. Ownership 1:1 e limites do diff

| Prompt L0 | Consumer exclusivo após confirmação |
|---|---|
| `compiler/stdlib/format.md` | `01_core/src/compiler/stdlib/format.rs` |
| `compiler/eval/rules.md` | `01_core/src/compiler/eval/rules.rs` |
| `compiler/layout/format_options.md` | `01_core/src/compiler/layout/format_options.rs` |
| `entities/document_format_options.md` | `01_core/src/entities/document_format_options.rs` |
| `shell/cli.md` | `02_shell/src/cli.rs` |
| `infra/pipeline.md` | `03_infra/src/pipeline.rs` |
| `wiring.md` | `04_wiring/src/main.rs` |
| `wiring/tests/p1463-format-png.md` | `04_wiring/tests/p1463_format_png.rs` |

O Núcleo não é owner de código e nenhum header `@prompt` pode apontar para
ele. Não alterar `entities/layout_types.rs`, `entities/mod.rs`,
`compiler/layout/mod.rs`, `infra/export/render.rs`, `infra/export/mod.rs`,
PDF, SVG ou HTML sem refutador medido e atualização L0 anterior.

## 6. Gate ADR-0127

P1463 adiciona elemento público de linguagem, campo público, flag CLI, campo
em `CompileIntent`, carrier público e entry point L3. Também muda o default
efetivo do export PNG quando a flag está ausente. Logo a paragem é obrigatória.

Sequência:

1. escrever os L0s e generalizar o Núcleo, sempre L0-first;
2. ressellar hashes e validar V15/V26;
3. commitar passo + L0 + Núcleo, sem teste RED nem código produtivo;
4. **PARAR** e obter confirmação humana posterior ao commit causal;
5. somente depois congelar contrato/oráculos/ataques, escrever RED e
   implementar.

O pedido que originou este passo não substitui a confirmação posterior ao
commit L0. A confirmação de P1462 não autoriza P1463.

## 7. Contrato observável congelável

Nos quatro perfis (`default`, `a11y-extras`, `html`, `html+a11y`), exigir:

- `type(format.png) == function`, nome `png`, `ppi == 144.0` e estabilidade
  por alias e `std`;
- construção manual proibida;
- integer e float aceitos para `ppi`, erros exatos para string/bool, named
  desconhecido, positional e duplicado;
- regra vazia neutra, repetição/inversão e last-write-wins;
- regra em container rejeitada sem publicar artefato;
- matriz CLI/documento/default completa, incluindo a assimetria `/72` apenas
  no caller CLI;
- igualdade de dimensões entre ramos com e sem fontes para a mesma página;
- `--ppi` ignorado por PDF/SVG/HTML e `format.png.ppi` sem efeito nesses
  exporters;
- `format.svg` e sua precedência P1462 preservados;
- `format.pdf`/`format.html` continuam ausentes.

Comparar dimensões, exit, stdout/stderr, Unicode, spans e mensagens
pertinentes. Não congelar bytes PNG, metadados de compressão, ordem interna de
hash ou endereço de objetos.

## 8. RED, mutantes e segregação

O RED deve provar separadamente:

1. `format.png` ausente na superfície pública;
2. set rule aceita falsamente pelo fallback com warning e não muda dimensões;
3. `--ppi` rejeitado por L2;
4. pipeline PNG usa sempre 2 pixels por ponto.

Mutantes mínimos:

1. omitir `png`, registrar nome errado ou criar módulo em vez de função;
2. default/namespace com `ppi == 2`, `72`, inteiro ou tipo errado;
3. permitir construção manual;
4. aceitar/ignorar named inválido, positional, duplicado ou tipo errado;
5. fazer regra vazia escrever 144;
6. first-write-wins ou ordem lexical invertida;
7. aceitar nested ou rejeitar root;
8. colapsar `None` em L2/L4;
9. dividir o valor documental por 72;
10. deixar de dividir a flag CLI por 72;
11. fazer documento vencer CLI explícito;
12. aplicar opções somente no ramo com fontes ou somente no ramo sem fontes;
13. vazar `ppi` para PDF/SVG/HTML;
14. antecipar `format.pdf`/`format.html` ou falsificar a contagem P1357;
15. corromper ownership/hash de prompt;
16. apontar código diretamente para o Núcleo;
17. atualizar o Núcleo sem atualizar ambos os pins consumidores;
18. alterar `format.svg` ao acrescentar PNG.

Contrato, oráculos, implementação, ataques e veredito têm autores segregados
quando a execução começar. Artefatos protegidos ficam em
`00_nucleo/diagnosticos/p1463-*`; o implementador não os reescreve para fazer
o candidato passar.

## 9. Inventário público e correção do observador

O baseline canônico herdado do veredito P1462 é:

| Perfil | BOTH | VANILLA_ONLY | CRYSTALLINE_ONLY normalizado |
|---|---:|---:|---:|
| default | 2000 | 21 | 42 |
| a11y-extras | 2003 | 24 | 42 |
| html | 2115 | 136 | 42 |
| html+a11y | 2118 | 139 | 42 |

P1463 acrescenta exatamente o owner `format.png`, portanto a projeção, ainda
não usada como prova, é:

| Perfil | BOTH esperado | VANILLA_ONLY esperado | CRYSTALLINE_ONLY normalizado |
|---|---:|---:|---:|
| default | 2001 | 20 | 42 |
| a11y-extras | 2004 | 23 | 42 |
| html | 2116 | 135 | 42 |
| html+a11y | 2119 | 138 | 42 |

O enumerador vanilla representa `ppi` em `format.png.params`; o enumerador
cristalino tende a publicar `format.png.ppi` como rota separada, tal como
ocorre com `format.svg.pretty`. Antes do candidato, um autor independente deve
medir bilateralmente essa representação e congelar a extensão da correção do
observador P1357. Se confirmada, o raw cristalino sobe de 43 para 44 e o overlay
canônico coalesce somente as duas rotas provadas, mantendo 42 normalizado.
Nenhum sibling pode ser normalizado por semelhança de nome.

## 10. Checkpoints protocolo de checkpoint revogado

A campanha não recomeça do zero após interrupção:

1. cada unidade atômica registra entradas, comando, outputs, hashes, estado
   Git, hora, resultado e `Interrupted` distinto de `Unknown`;
2. uma ordem completa discriminatória, com P00 preservado, todos os mutantes
   válidos violados, score `1.0` e zero `Unknown` semântico, autoriza o
   candidato;
3. estados parciais não recebem crédito, mas checkpoints completos continuam
   reutilizáveis depois de auditoria dos pins;
4. após o candidato, repetir P00, conversão documental/CLI, precedência,
   ambos os ramos de fontes, nested, vazamento de target, P1357 e gates
   arquiteturais; não rerodar universalmente todas as ordens sem sinal de
   risco;
5. duas revisões consecutivas sem ganho na mesma causa exigem redesenho antes
   de nova campanha completa.

Redução de orçamento ou interrupção nunca transforma ausência de evidência em
sucesso.

## 11. Gates finais planejados

Após L0, confirmação, RED e implementação:

```text
cargo test -p typst-core <testes focais P1463>
cargo test -p typst-shell <testes focais de --ppi>
cargo test -p typst-infra <testes focais P1463>
cargo test -p typst-wiring --test p1463_format_png
cargo test --workspace
cargo build
crystalline-lint .
git diff --check
```

Executar também os probes bilaterais congelados e o inventário P1357 nos
quatro perfis. As oito falhas históricas P1450/P1451 ainda registradas na suíte
global não recebem crédito verde nem podem mascarar regressão P1463.

O veredito final só pode alegar o fragmento `format.png`/`ppi` provado. Não
alega paridade PNG geral, multipágina, bytes do encoder ou superfícies futuras
de `format.pdf`/`format.html`.
