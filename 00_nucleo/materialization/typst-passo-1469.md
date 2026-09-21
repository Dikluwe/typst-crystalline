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

