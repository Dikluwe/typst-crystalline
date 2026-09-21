# Passo 1469 — materializar standards PDF (escopo reduzido: contrato de linguagem)

**Estado:** L0S ESCRITOS (9 ficheiros — ver secção 7); ESCOPO REDUZIDO por
decisão explícita do dono (2026-09-21, ver secção 7.1); V15/V26 verdes;
AGUARDA COMMIT CAUSAL SOMENTE-L0 E CONFIRMAÇÃO HUMANA ADR-0127 antes de
RED/código. A confirmação de P1467/P1468 não cobre P1469.

**Data da auditoria original:** 2026-09-20T17:40:24-03:00
**Data da execução (medição + decisão de escopo + L0s):** 2026-09-21

**Baseline de redação original:** `306ca50022104619447fae2625090b8d9c7b0a35`.
**Baseline real de execução:** `85b7fc5f1eda67e89494a478c2458de36d4ee20d`
(HEAD após materialização de P1468). `git status --short` no momento da
execução: só o WIP não relacionado de P1470 (`compiler/layout.md`,
`entities/introspector.md`, `compiler/layout/mod.rs`,
`entities/introspector.rs`), preservado intocado.

**Vanilla ratificado:** `586e1bd43fae6c9a973218163d3165c53ab8d16d`, SHA-256
do binário `eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`
(confirmado idêntico ao pin). Fonte consultada diretamente:
`lab/typst-original/crates/typst-{pdf,cli}/`, e — achado desta execução — a
crate `krilla` (dependência real do vanilla para geração PDF, disponível
localmente em
`~/.cargo/git/checkouts/krilla-dfa083154a81f6b2/7772dbe/crates/krilla/`),
cujo módulo `src/configure/` contém a lógica de seleção/validação de
standards que o vanilla usa.

## 1. Objetivo

Substituir a aceitação compatível e inerte de `--pdf-standard` por um contrato
único que governe:

- `#set pdf(standard: ...)`;
- `compile --pdf-standard ...`;
- versão efetiva do PDF;
- validações PDF/A e PDF/UA;
- interações com tagging, pages, attachments, cor e metadados.

Sucesso exige conformidade observável real. Aceitar a string e continuar
emitindo um PDF 1.7 genérico é falha.

## 2. Evidência anterior à decisão

O catálogo bilateral encontra `format.pdf.standard == ("1.7",)` em ambos os
binários. O cristalino, porém, mantém `--pdf-standard` como compatibilidade
oculta sem política pública.

O pin declara 17 nomes em `typst-cli/src/args.rs:692-746`:

```text
1.4, 1.5, 1.6, 1.7, 2.0,
a-1b, a-1a, a-2b, a-2u, a-2a,
a-3b, a-3u, a-3a, a-4, a-4f, a-4e, ua-1
```

`typst-pdf/src/format.rs:554-718` aceita um valor ou array, calcula uma
configuração compatível, proíbe duas versões, mais de um PDF/A ou mais de um
PDF/UA e rejeita ranges de versão incompatíveis. O default é PDF 1.7.

O cristalino não usa a crate upstream de export; portanto a tabela acima
legitima a linguagem, mas não autoriza copiar a mecânica de `krilla` nem alegar
conformidade sem implementar seus observáveis.

## 3. Decisão arquitetural a fixar no L0

### 3.1 Tipos fechados

Criar tipos L1 puros para `PdfStandard` e conjunto validado de standards.
Strings vivem apenas nas fronteiras de cast/CLI. O conjunto efetivo conserva
versão, no máximo um perfil PDF/A e no máximo um perfil PDF/UA. O default
público representa exatamente `(\"1.7\",)`.

O set rule aceita um standard ou array; a CLI aceita lista separada por
vírgulas. Ambos delegam ao mesmo validador puro e produzem mensagens/hints
bilateralmente congelados.

### 3.2 Precedência

```text
--pdf-standard presente > pdf.standard documental > ("1.7",)
```

Uma lista externa substitui o conjunto documental como unidade; não concatenar
silenciosamente. Esta decisão deve ser confirmada por sonda de conflito antes
do texto L0 final.

### 3.3 Conformidade não nominal

Para cada perfil suportado, o export deve ajustar a versão do header/catalog e
as estruturas exigidas. PDF/A deve produzir identificação XMP e respeitar as
restrições do perfil; PDF/UA deve produzir identificação, tagging e validações
de acessibilidade. Combinações compatíveis aplicam simultaneamente as duas
famílias.

O gate final usa parser e validadores independentes disponíveis no ambiente,
além de inspeção estrutural própria. Se um perfil não puder ser implementado de
forma real nesta fatia, o L0 deve reduzir explicitamente o conjunto anunciado;
não aceitar e ignorar.

### 3.4 Interações obrigatórias

- `a-1a`, `a-2a`, `a-3a` e `ua-1` exigem tagging compatível;
- pages implica tagging false e por isso conflita com esses perfis;
- `tagged: false` explícito conflita com perfil acessível;
- attachments são permitidos ou proibidos conforme o perfil efetivo;
- transparência, fontes, espaços de cor, metadata e versão mínima devem ser
  validados no owner que emite cada observável, sem lógica de negócio em L4.

## 4. L0s e cardinalidade

Atualizar antes de código os owners de `format`, opções documentais, rules,
collector, CLI, pipeline, wiring e todos os emissores PDF afetados. Se as
invariantes de standard forem partilhadas por mais de um Prompt L0, criar um
Núcleo Tekt declarativo pinado; não fazer um prompt possuir vários consumers e
não apontar código ao núcleo.

## 5. Gate ADR-0127

Este passo cria tipos/campos públicos, torna uma flag antes inerte funcional,
muda o default/forma do PDF para seleções explícitas e pode rejeitar documentos
antes aceitos. Após a medição bilateral:

1. escrever L0 completo e matriz de compatibilidade;
2. ressellar hashes;
3. validar V5/V15/V26;
4. criar commit causal somente documental;
5. **parar para confirmação humana**;
6. depois executar RED→GREEN por família de standard.

## 6. Testes e validação

O RED cobre todos os 17 nomes, valor/array, CLI múltipla, conflitos, default,
precedência e interações com tagged/pages/attachments. O GREEN só fecha uma
família quando:

- o header/version e identificadores estruturais são corretos;
- um validador externo aceita um fixture válido;
- o mesmo validador ou o produto rejeita fixture incompatível;
- a saída continua abrível e semanticamente equivalente;
- PDF 1.7 default sem regra não regride.

Depois, executar suíte completa, Clippy all-features, build, fmt, lint total,
V5/V15/V26 fatal e `git diff --check`. Registrar ferramenta, versão, comando,
resultado e proveniência; “o arquivo abriu” não prova conformidade.

## 7. Execução: decisão de escopo, medição e L0s (2026-09-21)

### 7.1 Escopo reduzido — decisão do dono

Antes de escrever qualquer L0, a auditoria de ambiente encontrou uma
limitação que muda a natureza deste passo: **não há validador de
conformidade PDF/A ou PDF/UA disponível neste ambiente** (sem veraPDF, sem
`pikepdf`; só `qpdf`/`mutool`, que validam estrutura genérica de PDF, não
conformidade A/UA) e **o vanilla delega toda a mecânica de conformidade real
à crate `krilla`** (não só a seleção de perfil — o módulo
`configure/validate.rs` sozinho tem 1237 linhas, e a imposição real de cada
perfil está espalhada pelo resto da crate), que o cristalino não usa. A
secção 3.3 deste passo já previa isto: "se um perfil não puder ser
implementado de forma real nesta fatia, o L0 deve reduzir explicitamente o
conjunto anunciado; não aceitar e ignorar."

Apresentadas três opções ao dono (contrato de linguagem apenas / tentar
instalar veraPDF agora / parar o passo), a decisão foi: **materializar
apenas o contrato de linguagem** — parsing dos 17 nomes, validação de
conflitos entre standards, cálculo da versão efetiva do PDF e ajuste do
header `%PDF-{version}`. **Não implementa nem alega conformidade PDF/A ou
PDF/UA real.** Um passo futuro, quando houver biblioteca/validador
disponível, pode reabrir esse eixo sem quebrar este contrato.

### 7.2 Medição do contrato de linguagem

A tabela de compatibilidade de versão por perfil (min/max), a resolução da
versão efetiva (explícita > máximo do range dos validators > default 1.7),
as mensagens de erro de conflito e a interação com `tagged`/`pages` foram
medidas com precisão lendo diretamente `krilla::configure::{mod,validate}.rs`
(fonte real do comportamento, não inferência) e confirmadas experimentalmente
contra `/usr/local/bin/typst` via `typst eval`/`typst compile`. Ver as
secções "Medição anterior à decisão" de cada L0 (§7.3) para os dados
completos e citações `file:line`. Achado notável: `standard` **não aceita
`Value::None`** como os outros três campos de `format.pdf` — a única forma
de "documento não decide" é a ausência do named (diferente de `pages`, que
tem `none` como decisão explícita).

### 7.3 L0s escritos (9 ficheiros)

`entities/document_format_options.md`, `compiler/eval/rules.md`,
`compiler/layout/format_options.md`, `shell/cli.md`, `infra/pipeline.md`,
`infra/export/mod.md`, `infra/export/builder.md`, `wiring.md` (todos com
contrato substantivo novo, todos rotulados "ESCOPO REDUZIDO — contrato de
linguagem"), e `compiler/stdlib/format.md` (nota de scope, sem contrato
novo — `format.pdf.standard == ("1.7",)` já é default nominal materializado
em P1464).

### 7.4 Gate ADR-0127

Após `crystalline-lint --fix-hashes .` e validação V15/V26, commit causal
somente-L0 (sem código produtivo, sem RED) e parada para confirmação humana
explícita sobre os L0s específicos — a confirmação de P1467/P1468 (e a
confirmação já recebida sobre o *escopo*, §7.1) não substitui a confirmação
do *contrato* escrito.

## 8. Execução segregada e integração (2026-09-21)

Confirmação humana explícita recebida em 2026-09-21, na mesma sessão,
posterior aos commits causais somente-L0 `65e68ad45` (L0s originais) e
`e14002d55` (correção das mensagens exatas `NoOverlappingRange`/
`VersionOutOfRange` medidas contra o binário — ver §7.2). Os três papéis
partiram desse baseline em worktrees isoladas; ambos os agentes sofreram
pelo menos uma interrupção por limite de taxa da sessão a meio do trabalho
e foram retomados via `SendMessage`, com instrução explícita de verificar
primeiro o que já estava no worktree antes de continuar, sem perda nem
repetição de progresso.

- **Agente A (implementação):** commit `99917beacc82bec6b913cac1aa1bf4b73d985429`
  ("feat(format): implement pdf standard language contract (P1469, agente
  A)"), parent único `e14002d55`. Implementou `PdfVersion`/`PdfArchival`/
  `PdfAccessibility`/`PdfStandard`/`PdfStandards`/`PdfValidator`/
  `PdfStandardsConflict` em `document_format_options.rs`, a extração em
  `format_options.rs`, o braço `format.pdf.standard` em `rules.rs`, `--pdf-
  standard` em `cli.rs`, a resolução de precedência e a interação com
  `tagged` em `pipeline.rs`, as variantes `_and_standard` em `export/mod.rs`,
  o campo `standard: PdfStandards` e o header `%PDF-{version}` dinâmico em
  `export/builder.rs`, e os gates CLI-only em `main.rs` — sem ver os testes.
- **Agente B (testes):** commit `c38712e31` ("test(format): freeze pdf
  standard red contract (P1469, agente B)"), parent `65e68ad45`. 64 testes
  novos, RED confirmado por ficheiro (compilação falha nos tipos/módulos
  ainda inexistentes; runtime falha nos 17 novos de `rules.rs` e nos 10
  novos do teste de integração de binário) — sem ver a implementação.
- **Agente C (integração, o próprio dono desta execução):** extraiu os dois
  patches, aplicou sem conflito de merge, e fez as correções mecânicas de
  integração: (1) três testes do Agente B usavam a forma antiga de
  `PdfStandardsConflict` (sem o campo `validators`, acrescentado pela
  correção `e14002d55` que chegou depois do commit do Agente B) — corrigidos
  manualmente com os dados reais medidos (§7.2); (2) dois `clone()`
  redundantes em tipos `Copy` (`Option<PdfStandards>` em `pipeline.rs` e
  `PageRange` num teste de `document_format_options.rs`) detectados por
  clippy — removidos; (3) após a suíte completa GREEN, dois avisos clippy
  genuinamente novos (não pré-existentes ao baseline `e14002d55`, confirmado
  por comparação em worktree isolada) — `impl Default for PdfVersion`
  manual (convertido para `#[derive(Default)]` com `#[default]` no variant
  `V1_7`) e uma closure redundante `.then(|| label())` em `main.rs`
  (substituída por `.then(label)`) — corrigidos sem mudança de
  comportamento e reconfirmados GREEN. Os `42` avisos clippy
  `too_many_arguments`/`type_complexity` que também apareceram novos em
  `pipeline.rs`/`export/mod.rs` **não** foram tratados como defeito: são a
  consequência direta e documentada (`infra/export/mod.md`/`infra/
  pipeline.md` §P1469) de `standard` ser o quinto eixo acrescentado como
  novo último parâmetro às mesmas entry points que já acumulavam
  `StreamMode`/`PdfTags`/`PdfSerializationMode`/`PageRanges` — o limiar
  clippy de 7 argumentos, nunca cruzado pelos quatro eixos anteriores, é
  cruzado pela primeira vez nesta série de passos. Comparação feita via
  `git worktree add` no commit baseline `e14002d55` (sem tocar a árvore de
  trabalho), clippy corrido com o mesmo comando dos dois lados.

### 8.1 RED → GREEN por ficheiro

| Ficheiro | Novos testes (RED no baseline `e14002d55`) | GREEN (após integração) |
|---|---|---|
| `entities/document_format_options.rs` | 13 novos — erro de compilação (`PdfVersion`/`PdfStandard`/... inexistentes) | 30/30 (13 novos + 17 irmãos) |
| `compiler/layout/format_options.rs` | 10 novos — erro de compilação (idem) | 37/37 (10 novos + 27 irmãos) |
| `compiler/eval/rules.rs` | 17 novos — compilava isolado; falhava em runtime (17/17) | 52/52 (17 novos + 35 irmãos) |
| `02_shell/src/cli.rs` | 6 novos — erro de compilação (`--pdf-standard` inexistente) | 83/83 na suíte typst-shell (6 novos) |
| `03_infra/src/pipeline.rs` | 8 novos — erro de compilação (entry points `_and_standard` inexistentes) | 97/97 (8 novos + 89 irmãos) |
| `04_wiring/tests/p1469_format_pdf_standard.rs` | 10 novos — 1/10 (só o caso trivial "não crasha") | 10/10 |

Total: **64/64 testes novos GREEN** (13+10+17+6+8+10).

### 8.2 Gates finais medidos (2026-09-21, working tree sobre `e14002d55`)

```text
cargo build --workspace                                     -> sucesso
cargo test -p typst-core --lib                                -> 6045/6045
cargo test -p typst-shell --lib                                -> 83/83
cargo test -p typst-infra --lib                                -> 995/995
cargo test -p typst-wiring --test p1469_format_pdf_standard
    (TMPDIR=/dev/shm, --test-threads=1)                        -> 10/10
cargo test -p typst-wiring --test p1462_format_svg
    --test p1463_format_png --test p1464_format_pdf
    --test p1465_format_html --test p1467_format_pdf_tagged_pretty
    --test p1468_format_pdf_pages (--test-threads=1)            -> 44/44 (regressão zero:
                                                                    7+7+4+4+10+12)
cargo fmt --all -- --check                                     -> sucesso (após 1 `cargo fmt --all`;
                                                                    sem mudança semântica, testes
                                                                    reconfirmados GREEN)
cargo clippy --workspace --all-targets --all-features          -> 0 erros; 2 avisos novos
                                                                    identificados e corrigidos
                                                                    (derive de PdfVersion, closure
                                                                    redundante em main.rs); 42 avisos
                                                                    novos aceites como consequência
                                                                    documentada da acumulação de eixos
                                                                    (ver §8); resto é ruído
                                                                    pré-existente confirmado por
                                                                    comparação em worktree baseline
crystalline-lint .                                              -> 0 erros (só info V20
                                                                    pré-existente, fora do diff)
crystalline-lint --checks v5,v15,v26 .                          -> 0 violações
git diff --check                                                -> sucesso
```

Não foi corrido `cargo test --workspace` completo nem a suíte de export PNG
em massa de `typst-wiring`, pela mesma razão documentada em P1464/P1467/
P1468 (pico histórico de RAM no export PNG, não relacionado com este diff);
a cobertura acima (todas as libs dos quatro crates tocados, mais os 7
testes de integração `format.*`) é proporcional ao raio do diff.

O veredito final é: `format.pdf.standard`/`--pdf-standard` parseiam os 17
nomes de perfil PDF/A e PDF/UA, validam conflitos entre standards (mensagens
exatas medidas contra `/usr/local/bin/typst`, §7.2), resolvem a versão
efetiva do PDF pela precedência explícita > máximo do range dos validators >
default `1.7`, e ajustam o header `%PDF-{version}` gerado — com a mesma
interação `standard`+`tagged` medida e replicada (dois canais de erro
distintos, hint condicional quando a causa raiz é o colapso por `--pages`).
**Não se implementa nem se alega conformidade estrutural PDF/A ou PDF/UA
real** (sem structure tree adicional, sem validação além do parsing e do
conflito de linguagem) — escopo reduzido por decisão explícita do dono
(§7.1), documentado como reabrível quando houver validador disponível
(veraPDF, a instalar como próximo passo desta sessão, fora do escopo de
código deste passo).

