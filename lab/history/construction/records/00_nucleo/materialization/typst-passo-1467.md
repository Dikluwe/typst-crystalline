# Passo 1467 — materializar `format.pdf.tagged` e `format.pdf.pretty`

**Estado:** MATERIALIZADO — confirmação ADR-0127 recebida diretamente do
dono na mesma sessão, posterior ao commit causal somente-L0
`e675e8b7acc29e272bd59939e44681a7b521acd3`. Execução segregada (Agente A/B/C)
concluída; ver secção 10.

**Data da auditoria:** 2026-09-20T17:40:24-03:00
**Data da redação dos L0s e das sondas bilaterais adicionais:** 2026-09-20T19:18-03:00

**Baseline:** `306ca50022104619447fae2625090b8d9c7b0a35`

**Tree:** `0f7f307e8ffda460270714cab0ae525237dc3433`

**Estado medido:** working tree não commitada, com 4 ficheiros rastreados
alterados (54 inserções, 19 remoções) e
`00_nucleo/materialization/typst-passo-1470.md` novo. Esses ficheiros pertencem
ao P1470 e não podem ser absorvidos por este passo.

**Vanilla ratificado:** `586e1bd43fae6c9a973218163d3165c53ab8d16d`

## 1. Objetivo

Fazer `#set pdf(tagged: ...)` e `#set pdf(pretty: ...)` influenciarem o export
PDF com a mesma semântica pública do vanilla, preservando a precedência das
flags já existentes. Este passo fecha somente os dois campos booleanos da
primeira fatia PDF.

Ficam explicitamente fora:

- `pdf.pages` e `--pages`, reservados ao P1468;
- `pdf.standard`, `--pdf-standard` e conformidade, reservados ao P1469;
- a correção morfológica do global `pdf`, reservada ao P1471;
- qualquer alteração do modo `--compact`, do conteúdo do documento ou da fase
  de layout.

## 2. Evidência anterior à decisão

O catálogo bilateral regenerado duas vezes no mesmo estado foi byte-idêntico.
Nos quatro perfis, `format.pdf.tagged == auto` e
`format.pdf.pretty == false` são acessíveis nos dois binários. Logo, as duas
rotas que o inventário bruto chama de extras são falsos positivos do
descobridor: os bindings existem; falta transportar o comportamento.

No cristalino atual:

- `compiler/stdlib/format.rs:63-66` publica os defaults nominais;
- `entities/document_format_options.rs` transporta apenas SVG, PNG e HTML;
- `compiler/layout/format_options.rs` declara chaves PDF futuras neutras;
- `compiler/eval/rules.rs` reconhece `format.svg`, `format.png` e
  `format.html`, mas não `format.pdf`;
- L2 já preserva `pdf_tagged: Option<bool>` e `pretty: Option<bool>`;
- L4 colapsa ausência de tagging para habilitado e ausência de pretty para
  falso antes que uma regra documental possa ser consultada.

No vanilla pinado, `typst-pdf/src/format.rs:102-348` declara `PdfFormat` como
elemento de formato com `tagged: auto | bool` e `pretty: bool`; a resolução em
`typst-pdf/src/lib.rs:93-152` combina opção externa e opção documental antes de
decidir tagging. Isso é comportamento da linguagem, não detalhe do encoder.

### 2.1 Sondas adicionais e proveniência (2026-09-20T19:18-03:00 a 19:36-03:00)

Repetidas nesta execução, independentemente das sondas acima, contra
`/usr/local/bin/typst` (SHA-256
`eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`, idêntico
ao pin) e contra `./target/release/typst` em `306ca50022104619447fae2625090b8d9c7b0a35`
(`git status --short`/`git diff HEAD --stat` no momento da medição: somente os
quatro ficheiros do P1470 em curso — `compiler/layout.md`,
`entities/introspector.md`, `compiler/layout/mod.rs`, `entities/introspector.rs`
— alterados e não relacionados a este passo):

```text
# vanilla
repr((type(format.pdf.tagged), format.pdf.tagged,
      type(format.pdf.pretty), format.pdf.pretty))     => (type(auto), auto, bool, false)
{ set format.pdf(tagged: 1) }                            => expected boolean or auto, found integer
{ set format.pdf(pretty: 1) }                             => expected boolean, found integer
#set format.pdf(tagged: false) [root, PDF real]           => sem /StructTreeRoot, /MarkInfo
#set format.pdf(tagged: true)  [root, PDF real]           => com /StructTreeRoot, /MarkInfo (2 ocorrências)
ausência de regra                [root, PDF real]         => com /StructTreeRoot, /MarkInfo (auto == habilitado
                                                              sem seleção de páginas)
--pdf-tagged=false vence #set(tagged: true)                => sem struct tree
--pdf-tagged=true  vence #set(tagged: false)                => com struct tree
--no-pdf-tags vence #set(tagged: true) (com aviso deprecated)=> sem struct tree
--pretty vence #set(pretty: false)                          => serialização pretty (337 newlines, igual a pretty:true)
tagged:false + #set(); depois #set(pretty:true)             => tagged permanece false, pretty true (independentes)
#block[ #set format.pdf(tagged: false) ... ]                => format set rules are not allowed inside of containers

# cristalino (306ca5002)
repr((type(format.pdf.tagged), format.pdf.tagged,
      type(format.pdf.pretty), format.pdf.pretty))     => (type(auto), auto, bool, false)  [já materializado em P1464]
{ set format.pdf(tagged: true) }                          => warning: set: target '' ainda não suportado
                                                              (não reconhecido estruturalmente; diverge do vanilla)
#set format.pdf(tagged: false) [root, PDF real]           => com /StructTreeRoot, /MarkInfo (regra ignorada;
                                                              L4 já colapsa ausência de --pdf-tagged em Enabled)
--pdf-tagged=false [CLI, PDF real]                          => sem /StructTreeRoot, /MarkInfo (CLI já funciona)
```

Confirma integralmente a medição da seção 2: o cristalino já expõe os
defaults (P1464) mas nem valida a set rule, nem extrai a decisão documental,
nem a consulta na pipeline; a CLI já funciona isoladamente. O aviso genérico
`target '' ainda não suportado` hoje emitido para `format.pdf` é ele próprio
uma divergência a ser eliminada por este passo (deixa de ser alcançado
quando `format.pdf` ganha braço estrutural próprio em `rules.rs`).

## 3. Contrato a escrever no L0

### 3.1 Carrier fechado

Acrescentar um único `PdfFormatOptions` ao owner
`entities/document_format_options.md`, contendo nesta fatia:

```text
tagged: auto | bool
pretty: bool
```

O default é `auto` e `false`. `DocumentFormatOptions` ganha exatamente um campo
`pdf`. Não usar `Option<bool>` para representar `auto`: o domínio da língua tem
três estados e deve ter um tipo L1 fechado e explícito.

### 3.2 Set rule e coleta

O braço `format.pdf` aceita zero ou mais dos dois named conhecidos, rejeita
qualquer outro e grava somente os named presentes nas chaves canônicas
`format.pdf.tagged` e `format.pdf.pretty`. A regra vazia é neutra. Os casts e
diagnósticos devem ser congelados por sondas bilaterais antes da redação final
do L0.

O collector root resolve a última decisão lexical de cada chave. Qualquer
ocorrência PDF dentro de container falha com a mensagem já pública:

```text
format set rules are not allowed inside of containers
```

### 3.3 Precedência

O Núcleo Tekt `format-option-precedence` continua sendo a única regra
compartilhada. A resolução é:

```text
--pdf-tagged explícito > --no-pdf-tags > documento > auto
--pretty explícito     > documento                 > false
```

Na ausência de seleção de páginas, `auto` resolve para tagging habilitado.
`--no-pdf-tags` continua legado explícito e emite o warning vigente; não pode
apagar o tri-state moderno em L2.

`pretty` escolhe somente a serialização legível do mesmo PDF. Não muda content
streams compactos, tagging, standard, layout, metadados, ordem semântica ou
document ID.

## 4. L0s causais e ownership

Atualizar antes de código, com caminho canônico completo:

- `00_nucleo/prompts/compiler/stdlib/format.md`;
- `00_nucleo/prompts/entities/document_format_options.md`;
- `00_nucleo/prompts/compiler/eval/rules.md`;
- `00_nucleo/prompts/compiler/layout/format_options.md`;
- `00_nucleo/prompts/infra/pipeline.md`;
- `00_nucleo/prompts/shell/cli.md` somente para preservar o tri-state já
  público e registrar a nova precedência documental;
- `00_nucleo/prompts/wiring.md`;
- os prompts de teste exclusivos que forem realmente alterados.

Cada consumer continua com um único Prompt L0 proprietário. Não criar um
segundo carrier PDF em L3 nem apontar código ao Núcleo Tekt.

## 5. Gate ADR-0127

Este passo adiciona campo público a `DocumentFormatOptions` e muda o
comportamento por defeito quando o documento contém `#set pdf(...)`. A parada é
obrigatória:

1. executar as sondas bilaterais de presença, casts, regra vazia, ordem lexical
   e container;
2. redigir/retificar os L0s acima;
3. ressellar hashes e obter V5/V15/V26 verde;
4. criar commit causal somente de L0;
5. **parar e aguardar confirmação humana**;
6. somente depois escrever testes RED e produto.

Confirmações de P1466, P1470 ou de outro passo não autorizam P1467.

## 6. RED → GREEN

O RED deve provar, no mínimo:

- defaults `auto` e `false` no carrier;
- `#set pdf(tagged: false)` remove a estrutura de tags sem depender de bytes
  idênticos;
- `tagged: true` e `auto` produzem estrutura tagueada no caso sem páginas;
- `#set pdf(pretty: true)` altera somente whitespace/forma de serialização;
- `--pdf-tagged` e `--pretty` vencem o documento em ambos os valores;
- flags ausentes deixam o documento decidir;
- duas regras root usam a última decisão lexical por campo;
- regra vazia não restaura default;
- regra dentro de container falha com a mensagem canônica;
- `tagged`, `pretty`, `compact` e feature `a11y-extras` são ortogonais.

Inspecionar o PDF por estrutura parseada, catálogo/tag tree e modo do writer;
não usar igualdade integral de bytes como prova de linguagem.

## 7. Gates finais

```text
cargo test -p typst-core format_pdf
cargo test -p typst-infra format_pdf
cargo test -p typst-wiring --test cli
cargo test --workspace --no-fail-fast -- --test-threads=1
cargo clippy --workspace --all-targets --all-features
cargo build --workspace
cargo fmt --all -- --check
crystalline-lint .
crystalline-lint --checks v5,v15,v26 --fail-on warning .
git diff --check
```

Registrar no próprio passo o commit ou o estado não commitado exato, hora,
comandos e contagens. Não criar recibo ou relatório paralelo.

## 8. Execução segregada (Agentes A/B/C)

Confirmação humana explícita recebida em 2026-09-20, na mesma sessão,
posterior ao commit causal somente-L0 `e675e8b7acc29e272bd59939e44681a7b521acd3`.
Os três papéis da skill relida partiram todos desse mesmo baseline, em
worktrees isoladas:

- **Agente A (implementação):** commit
  `11138de552ad8dc51543bd1dfbbf0a408da3d144`
  ("feat(format): implement pdf tagged/pretty resolution (P1467, agente A)").
  Implementou o carrier `PdfTagged`/`PdfFormatOptions`, o braço `format.pdf`
  em `rules.rs`, a extração root/exaustiva em `format_options.rs`,
  `resolve_pdf_tags`/`resolve_pdf_pretty` e as três entry points
  `_tagged_and_pretty_override` em `pipeline.rs` (as 10 entry points de
  compatibilidade preexistentes passaram a delegar, preservando byte a byte
  o comportamento anterior), e o fold CLI-only em `main.rs`. Não escreveu
  nenhum teste.
- **Agente B (testes):** commit `1ead07a35477e7eb423ee559fd3a5d244cea4858`
  ("test(format): freeze pdf tagged/pretty red contract (P1467, agente B)").
  Escreveu 51 testes (6 em `document_format_options.rs`, 9 em
  `format_options.rs`, 15 em `rules.rs`, 11 em `pipeline.rs`, 10 em
  `04_wiring/tests/p1467_format_pdf_tagged_pretty.rs`) e o L0 de teste
  `wiring/tests/p1467-format-pdf-tagged-pretty.md`, sem ver o código de A.
  Confirmou RED por ficheiro contra o mesmo baseline (ver §9.1).
- **Agente C (integração, o próprio dono desta execução):** extraiu os dois
  patches, aplicou-os em sequência sobre o HEAD `Tekt` (sem conflito),
  reconciliou os `@prompt-hash`/"Hash do Código" finais com
  `crystalline-lint --fix-hashes .`, corrigiu formatação (`cargo fmt --all`)
  e mediu os gates finais (§9).

Ambas as worktrees dos agentes nasceram inicialmente desalinhadas (HEAD no
pin vanilla `586e1bd43` em vez do tip `Tekt`); os dois agentes detectaram e
corrigiram sozinhos com `git reset --hard` depois de confirmar árvore limpa
e zero commits exclusivos a perder. Registado aqui como causa-raiz de
infraestrutura a reportar (não é defeito de nenhum dos dois papéis).

## 9. Medição final e proveniência

**Instante:** 2026-09-20T20:44-03:00. **Estado:** working tree não commitada
sobre o commit causal `e675e8b7acc29e272bd59939e44681a7b521acd3`, resultado
da integração dos patches A (`11138de55`) e B (`1ead07a35`) mais os ajustes
de C (reseal de hash e `cargo fmt`). `git diff HEAD --stat` nesse instante
cobria exatamente os 12 ficheiros de produto/teste listados no §8 mais os 7
`.md` L0 (hash reseal) — nenhum ficheiro do P1470 em curso foi tocado.

### 9.1 RED → GREEN por ficheiro

| Ficheiro | RED (baseline `e675e8b7a`) | GREEN (após integração) |
|---|---|---|
| `entities/document_format_options.rs` | erro de compilação (`E0609`/`E0433`, campo/tipo inexistente) | 8/8 (6 novos + 2 irmãos) |
| `compiler/layout/format_options.rs` | erro de compilação (idem) | 16/16 (9 novos + 7 irmãos) |
| `compiler/eval/rules.rs` | 4/15 (deteção de duplicado e regra vazia já eram genéricas; 11 exigiam o braço) | 15/15 |
| `03_infra/src/pipeline.rs` | erro de compilação (`E0425`, entry point inexistente) | 79/79 (11 novos + 68 pré-existentes, incl. SVG/PNG/HTML) |
| `04_wiring/tests/p1467_format_pdf_tagged_pretty.rs` | 6/10 (só precedências CLI puras e o caso onde `auto` já coincidia com o default) | 10/10 |

### 9.2 Gates finais medidos

```text
cargo build --workspace                                     -> sucesso
cargo check --workspace --tests                              -> sucesso
cargo test -p typst-core --lib                                -> 5965/5965
cargo test -p typst-infra --lib                               -> 977/977
cargo test -p typst-shell --lib                                -> 69/69
cargo test -p typst-wiring --test p1467_format_pdf_tagged_pretty
    (TMPDIR=/dev/shm, --test-threads=1)                        -> 10/10
cargo test -p typst-wiring --test p1462_format_svg
    --test p1463_format_png --test p1464_format_pdf
    --test p1465_format_html (--test-threads=1)                -> 22/22 (regressão zero)
cargo fmt --all -- --check                                    -> sucesso (após 1 `cargo fmt --all`
                                                                   nos testes novos de B; sem mudança
                                                                   semântica, testes reconfirmados GREEN)
cargo clippy --workspace --all-targets --all-features          -> 0 erros; 0 avisos novos nos 6
                                                                   ficheiros tocados (confirmado por
                                                                   grep de nome de ficheiro no relatório)
crystalline-lint .                                              -> 0 erros (avisos V16/V19/V20
                                                                   pré-existentes, fora do diff)
crystalline-lint --checks v5,v15,v26 .                          -> 0 violações
git diff --check                                                -> sucesso
```

Não foi corrido `cargo test --workspace` completo nem a suíte de export PNG
em massa de `typst-wiring`, pela mesma razão documentada em P1464 (pico
histórico de ~40 GB de alocação por processo nesse caminho, não relacionado
com este diff); a cobertura acima (todas as libs dos três crates tocados,
mais os 5 testes de integração `format.*`) é proporcional ao raio do diff.

O veredito final é: `format.pdf.tagged` e `format.pdf.pretty` resolvem
corretamente a precedência `CLI explícito > legado --no-pdf-tags > documento
> default`, ortogonais entre si e aos demais formatos, sem regressão em
`format.svg`/`format.png`/`format.html`/módulo `pdf`. Não se declara
paridade de `pdf.pages`, `pdf.standard`, PDF/A ou PDF/UA — reservados a
P1468/P1469.

