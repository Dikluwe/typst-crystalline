# Passo 84.8b — Correcções de status nos ADRs (Prioridade ALTA do P84.7)

## Estado actual antes de começar

Ler antes de começar:
- `00_nucleo/relatorios/relatorio-auditoria-adrs-passo-84.7.md` —
  Secção 2 (status desalinhado) e Secção 8 (Prioridade ALTA).
- `00_nucleo/adr/typst-adr-0032-unsafe-em-l1.md` — ADR mais recente,
  referência de formato canónico do status.
- Os 6 ADRs alvo:
  - `00_nucleo/adr/typst-adr-0001-...md`
  - `00_nucleo/adr/typst-adr-0007-...md`
  - `00_nucleo/adr/typst-adr-0017-...md`
  - `00_nucleo/adr/typst-adr-0018-...md`
  - `00_nucleo/adr/typst-adr-0027-...md`
  - `00_nucleo/adr/typst-adr-0028-...md`

Pré-condição: `cargo test` — 911 testes, zero violations. P84.8a
concluído, ADR-0032 existe, DEBTs 40-42 em Secção 1 do DEBT.md.

---

## Natureza deste passo

**Passo puramente mecânico de correcção.** Edita 6 ADRs para
alinhar status declarado com estado real. Nenhum ADR novo, nenhum
DEBT novo, nenhum código tocado.

Os desalinhamentos foram auditados no P84.7 (Secção 2) com
evidência grep. Este passo consome essa auditoria como instrução
de execução.

**Escopo das alterações**: 1-2 linhas por ADR. Total estimado:
10-15 linhas de edições distribuídas por 6 ficheiros.

---

## Duas decisões menores formalizadas neste passo

### Decisão 1 — Formato canónico do status

Valor canónico: `**Status**: ` `` `VALOR` `` em português, com
backticks à volta do valor. Os valores possíveis são:

- `PROPOSTO` — decisão tomada mas não implementada.
- `IDEIA` — direcção a considerar, pode não ser implementada.
- `IMPLEMENTADO` — decisão tomada e materializada em código.
- `REVOGADO` — decisão superseded por ADR posterior.
- `UPDATED` — revisão de decisão anterior (ex: ADR-0026-revisao).
- `ADIADO` — decisão tomada mas implementação adiada para data
  futura.

Formatos **a corrigir** neste passo:

- `**Estado**: IMPLEMENTADO` (sem backticks, "Estado" em vez de
  "Status") — encontrado em ADR-0017.
- `\*\*Status\*\*: ` `` `IMPLEMENTADO` `` (backslashes literais
  a escapar asteriscos) — encontrado em ADR-0018.
- `**Status:** Accepted` (dois pontos antes do espaço, valor em
  inglês sem backticks) — encontrado em ADR-0027 e ADR-0028.

### Decisão 2 — Convenção para revogações

Convenção introduzida neste passo:

- O ADR que **revoga** declara: `**Revoga**: ADR-NNNN`.
- O ADR **revogado** declara: `**Revogado por**: ADR-NNNN`.

Ambos os campos ficam **logo após** o campo `**Status**` no
cabeçalho. Isto dá simetria e permite navegar a cadeia de
revogações em qualquer direcção.

Exemplo de ADR revogado após edição:

```markdown
# ADR-0007: rustc_hash (substituição)

**Status**: `REVOGADO`
**Revogado por**: ADR-0018
**Data**: 2026-XX-XX
```

Exemplo de ADR que revoga (já tem campo `**Revoga**`, sem
alteração):

```markdown
# ADR-0018: rustc_hash reintroduzido

**Status**: `IMPLEMENTADO`
**Revoga**: ADR-0007
**Data**: 2026-XX-XX
```

---

## Tarefa 1 — Verificar formato dos ADRs recentes (baseline)

Antes de corrigir, confirmar que ADR-0032 (o mais recente) segue
o formato canónico:

```bash
head -10 00_nucleo/adr/typst-adr-0032-unsafe-em-l1.md
```

Esperado: `**Status**: ` `` `ACCEPTED` `` ou `**Status**: `
`` `IMPLEMENTADO` `` com backticks. Se divergir, **reportar antes
de prosseguir** — o formato canónico pode precisar de revisão.

```bash
# Verificar também 0029, 0030, 0031
grep -A 0 "^\*\*Status\*\*" 00_nucleo/adr/typst-adr-0029-*.md \
  00_nucleo/adr/typst-adr-0030-*.md \
  00_nucleo/adr/typst-adr-0031-*.md
```

Todos devem ter a forma `**Status**: ` `` `VALOR` ``. Se houver
divergência entre 0029-0032, tratar a forma mais recente (0032)
como canónica e anotar as divergências para possível correcção
em passo futuro.

---

## Tarefa 2 — ADR-0001 (Estratégia de Migração)

**Estado actual**: `**Status**: ` `` `PROPOSTO` ``

**Evidência de desalinhamento** (Secção 2 do relatório P84.7):
- `comemo` em `crystalline.toml` linha 73 (`[l1_allowed_external]`
  com comentário `# decisão intencional, ver ADR-0001`).
- `grep -rn "comemo" 01_core/src/` retorna 5+ ocorrências de
  `#[comemo::track]` em `world_types.rs`.

Opção C do ADR-0001 foi implementada no Passo 4. Opção B
(isolamento em L3) continua em aberto como direcção futura.

### Edição

Substituir:

```markdown
**Status**: `PROPOSTO`
```

Por:

```markdown
**Status**: `IMPLEMENTADO`
**Opção escolhida**: C (comemo em `[l1_allowed_external]`, Passo 4)
**Nota**: Opção B (isolamento em L3) continua em aberto como
direcção futura, não registada como DEBT por não ter plano de
execução.
```

**Não alterar** o resto do conteúdo do ADR. A análise das opções
A/B/C no corpo do documento continua a ser valor histórico útil.

---

## Tarefa 3 — ADR-0007 (rustc_hash substituído)

**Estado actual**: `**Status**: ` `` `PROPOSTO` ``

**Evidência de desalinhamento**:
- `grep -n "Revoga" 00_nucleo/adr/typst-adr-0018-*.md` retorna
  `Revoga: ADR-0007` na linha 5.
- Texto literal "ADR-0007 — decisão revogada" no corpo do ADR-0018.

### Edição

Substituir:

```markdown
**Status**: `PROPOSTO`
```

Por:

```markdown
**Status**: `REVOGADO`
**Revogado por**: ADR-0018
```

**Não alterar** o conteúdo do ADR-0007. A análise original
permanece como registo histórico da decisão revogada.

---

## Tarefa 4 — ADR-0017 (Adiamento da migração de eval)

**Estado actual**: `**Estado**: IMPLEMENTADO` (formato não-canónico)

**Evidência de desalinhamento**:
- Campo chama-se `**Estado**` em vez de `**Status**`.
- Valor sem backticks.
- `pub fn eval(` confirmado em `01_core/src/rules/eval.rs:250` —
  o conteúdo `IMPLEMENTADO` está correcto, só o formato está
  errado.

### Edição

Substituir:

```markdown
**Estado**: IMPLEMENTADO
```

Por:

```markdown
**Status**: `IMPLEMENTADO`
```

**Nota sobre conteúdo**: o ADR-0017 documenta adiamento que já
não se aplica (eval já foi implementado). O conteúdo do corpo
pode estar desactualizado em relação à realidade. **Este passo
não mexe no corpo** — eventual reescrita do conteúdo fica para
passo posterior se o utilizador decidir.

---

## Tarefa 5 — ADR-0018 (rustc_hash reintroduzido)

**Estado actual**: `\*\*Status\*\*: ` `` `IMPLEMENTADO` ``
(backslashes literais a escapar os asteriscos)

**Evidência de desalinhamento**:
- Markdown renderiza literalmente `\*\*Status\*\*` em vez de
  aplicar negrito.
- Valor (`IMPLEMENTADO`) está correcto, só a formatação está
  partida.

### Edição

Substituir:

```markdown
\*\*Status\*\*: `IMPLEMENTADO`
```

Por:

```markdown
**Status**: `IMPLEMENTADO`
```

**Verificar** que não há mais backslashes literais no resto do
documento:

```bash
grep "\\\\\\*" 00_nucleo/adr/typst-adr-0018-*.md
```

Se retornar outras ocorrências, reportar antes de prosseguir —
podem ser intencionais (ex: mostrar asteriscos literais num
exemplo de código).

---

## Tarefa 6 — ADR-0027 (CIDFont subsetting)

**Estado actual**: `**Status:** Accepted` (formato inglês, dois
pontos mal colocados)

**Análise**:
- Dois pontos antes do espaço (`Status:` + espaço) em vez do
  canónico (`Status**` + dois pontos + espaço).
- Valor em inglês (`Accepted`) em vez do canónico em português
  (`IMPLEMENTADO`).
- Sem backticks à volta do valor.

### Edição

Substituir:

```markdown
**Status:** Accepted
```

Por:

```markdown
**Status**: `IMPLEMENTADO`
```

**Nota**: `Accepted` em ADRs estilo Michael Nygard tradicionalmente
significa "decisão tomada e em vigor", que no vocabulário deste
projecto corresponde a `IMPLEMENTADO` (decisão materializada) ou
`PROPOSTO` (decisão tomada mas pendente). ADR-0027 é sobre CIDFont
subsetting — verificar estado real no código:

```bash
grep -rn "CIDFont\|cid_font\|subset" 03_infra/src/ 01_core/src/ \
  | head -20
```

Se há implementação real: `IMPLEMENTADO`. Se a implementação está
pendente mas a decisão está tomada: `PROPOSTO`. Reportar o
resultado do grep e escolher conforme evidência.

---

## Tarefa 7 — ADR-0028 (Tipos tipográficos simplificados)

**Estado actual**: `**Status:** Accepted` (mesmo padrão de formato
que ADR-0027)

**Evidência de desalinhamento adicional**:
- `grep -n "Revoga\|ADR-0028" 00_nucleo/adr/typst-adr-0029-*.md`
  retorna "ADR-0028 é revogada na sua totalidade" na linha 60 do
  ADR-0029.
- Nome do ficheiro do ADR-0029 inclui literalmente "revoga-adr-0028".

### Edição

Substituir:

```markdown
**Status:** Accepted
```

Por:

```markdown
**Status**: `REVOGADO`
**Revogado por**: ADR-0029
```

**Não alterar** o corpo do ADR-0028. Fica como registo histórico
da decisão revogada.

**Verificar** que o ADR-0029 (que revoga) tem campo inverso:

```bash
grep "Revoga" 00_nucleo/adr/typst-adr-0029-pureza-fisica-*.md
```

Esperado: `**Revoga**: ADR-0028`. Se o campo não existe no formato
canónico, reportar — pode ser alvo de correcção futura (tarefa
secundária do P84.8c).

---

## Tarefa 8 — Verificação final

```bash
# Todos os 6 ADRs corrigidos têm formato canónico
for f in \
  00_nucleo/adr/typst-adr-0001-*.md \
  00_nucleo/adr/typst-adr-0007-*.md \
  00_nucleo/adr/typst-adr-0017-*.md \
  00_nucleo/adr/typst-adr-0018-*.md \
  00_nucleo/adr/typst-adr-0027-*.md \
  00_nucleo/adr/typst-adr-0028-*.md; do
  echo "=== $f ==="
  grep -m1 "^\*\*Status\*\*:" "$f"
done

# Esperado: cada linha mostra `**Status**:` com um valor em
# backticks. Nenhuma linha vazia, nenhum `\*\*`, nenhum `Estado:`.

# ADR-0007 tem Revogado por
grep "Revogado por" 00_nucleo/adr/typst-adr-0007-*.md
# Esperado: **Revogado por**: ADR-0018

# ADR-0028 tem Revogado por
grep "Revogado por" 00_nucleo/adr/typst-adr-0028-*.md
# Esperado: **Revogado por**: ADR-0029

# Nenhum backslash literal a escapar asteriscos em nenhum ADR
grep -l "\\\\\\*\\\\\\*Status" 00_nucleo/adr/typst-adr-*.md
# Esperado: vazio (nenhum ficheiro listado)

# Ninguém usa **Estado**: em vez de **Status**:
grep -l "^\*\*Estado\*\*:" 00_nucleo/adr/typst-adr-*.md
# Esperado: vazio

# Ninguém usa **Status:** com dois pontos interiores
grep -l "^\*\*Status:\*\*" 00_nucleo/adr/typst-adr-*.md
# Esperado: vazio

# Código intacto
git status 01_core/ 02_shell/ 03_infra/ 04_wiring/
# Esperado: vazio (este passo não mexe em código)

# DEBT.md intacto (não adicionamos nem fechamos DEBTs)
git diff --stat 00_nucleo/DEBT.md
# Esperado: vazio

# Testes
cargo test

# Linter
crystalline-lint .
```

---

## Critérios de conclusão

- [ ] ADR-0001: `PROPOSTO` → `IMPLEMENTADO` com campo `**Opção
  escolhida**: C` e nota sobre Opção B.
- [ ] ADR-0007: `PROPOSTO` → `REVOGADO` com campo `**Revogado por**:
  ADR-0018`.
- [ ] ADR-0017: `**Estado**: IMPLEMENTADO` → `**Status**: ` `` `IMPLEMENTADO` ``.
- [ ] ADR-0018: `\*\*Status\*\*` → `**Status**` (remoção de
  backslashes).
- [ ] ADR-0027: formato canonizado; valor decidido pela evidência
  do grep sobre implementação real.
- [ ] ADR-0028: `Accepted` → `REVOGADO` com campo `**Revogado por**:
  ADR-0029`.
- [ ] Nenhum corpo de ADR alterado (só o cabeçalho de status).
- [ ] Nenhum código-fonte tocado.
- [ ] Nenhum DEBT aberto ou fechado.
- [ ] `cargo test` mantém 911 testes.
- [ ] `crystalline-lint .` zero violations.

---

## Ao terminar, reportar

- Resultado do grep da Tarefa 6 sobre CIDFont — decisão adoptada
  para ADR-0027 (`IMPLEMENTADO` ou `PROPOSTO`).
- Confirmação de que ADR-0018 não tinha outros backslashes
  literais (ou, se tinha, quais).
- Confirmação de que o grep da Tarefa 7 sobre ADR-0029 retornou
  `**Revoga**: ADR-0028` no formato esperado.
- Lista dos 6 ADRs editados com confirmação de alteração 1-a-1.

**Go/No-Go para P84.8c** (lacunas de conteúdo):

- **GO**: os 6 status estão alinhados, convenção `**Revogado por**:`
  introduzida em ADR-0007 e ADR-0028, código intacto.
- **NO-GO — corpo alterado**: se alguma Tarefa terminou a mexer no
  corpo do ADR além do cabeçalho de status, reportar e reverter.
  O escopo deste passo é estritamente o cabeçalho.
- **NO-GO — formato canónico divergiu**: se a Tarefa 1 revelou que
  ADR-0029/0030/0031/0032 têm formatos diferentes entre si,
  reportar — o formato canónico precisa de ser escolhido antes
  de continuar.

---

## Nota sobre caminho de ficheiro do relatório

Este passo **não produz relatório** — o produto são 6 edições de
cabeçalho em ADRs existentes. Segue o padrão estabelecido no P83.6
(a regra "relatório persistente" aplica-se a passos de auditoria
ou investigação, não a passos de execução directa).
