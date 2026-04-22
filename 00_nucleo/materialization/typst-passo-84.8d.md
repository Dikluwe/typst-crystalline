# Passo 84.8d — Refactor do anti-padrão "Diagnóstico obrigatório" em ADR-0022/0023/0025

## Estado actual antes de começar

Ler antes de começar:
- `00_nucleo/adr/typst-adr-0022-*.md` (FontBook)
- `00_nucleo/adr/typst-adr-0023-*.md` (indexmap)
- `00_nucleo/adr/typst-adr-0025-*.md` (Int == Float)
- `00_nucleo/relatorios/relatorio-auditoria-adrs-passo-84.7.md` —
  Secções 3.3 e 5.4 (motivação do refactor).
- `00_nucleo/relatorios/` — directório existente, modelo para a
  convenção paralela `00_nucleo/diagnosticos/` que este passo
  cria.

Pré-condição: `cargo test` — 911 testes, zero violations. P84.8c
concluído, ADR-0029 e ADR-0030 expandidos.

---

## Natureza deste passo

**Passo de refactor documental.** Move conteúdo entre directórios
e corta secções de ADRs. Não mexe em código, não altera DEBTs,
não cria novos ADRs.

Quatro produtos:

1. Criar directório `00_nucleo/diagnosticos/`.
2. Mover o conteúdo da secção "Diagnóstico obrigatório" de cada
   um dos 3 ADRs alvo para um ficheiro dedicado em
   `00_nucleo/diagnosticos/`.
3. Remover a secção dos 3 ADRs.
4. Adicionar em cada um dos 3 ADRs uma linha curta nas referências
   apontando para o diagnóstico movido.

---

## Decisões formalizadas neste passo

### Decisão 1 — Directório para diagnósticos

Novo directório: `00_nucleo/diagnosticos/`.

Convenção de nome: `diagnostico-adr-NNNN-<slug>.md` (paralelo à
convenção de relatórios `relatorio-<tipo>-passo-<N>.md`).

Distinção entre os três directórios de `00_nucleo/`:

- `relatorios/` — snapshots de auditoria pontuais (83.5, 84.7).
  Produzidos por passos de investigação, consultados para
  entender estado histórico do projecto.
- `diagnosticos/` — registos de verificação feita **antes** de
  uma decisão arquitectural (ADR). Citados pelo ADR
  correspondente. Produzidos durante a materialização, consultados
  para entender o que foi verificado antes de a decisão ser
  tomada.
- `prompts/` — instruções para futuras execuções (passos de
  materialização, regras do linter, etc.). Consultados para saber
  **o que fazer**.

A distinção operacional: se o conteúdo é **para executar no
futuro**, é prompt. Se é **registo do que foi executado antes
de uma decisão**, é diagnóstico. Se é **snapshot de estado em
momento específico (auditoria)**, é relatório.

### Decisão 2 — Rastreabilidade no ADR

Quando a secção de diagnóstico é removida do ADR, o ADR ganha
linha curta nas "Referências" (ou equivalente) apontando para o
ficheiro de diagnóstico movido. O texto é uniforme:

```markdown
**Diagnóstico prévio**: ver
`00_nucleo/diagnosticos/diagnostico-adr-NNNN-<slug>.md` — verificações
executadas antes desta decisão.
```

---

## Tarefa 1 — Criar directório

```bash
mkdir -p 00_nucleo/diagnosticos/
```

Verificar:
```bash
ls -ld 00_nucleo/diagnosticos/
```

Se o directório já existe (pouco provável mas possível), reportar
e parar — pode indicar que outro passo já o criou com convenção
diferente.

---

## Tarefa 2 — Mover diagnóstico do ADR-0022 (FontBook)

### Passo 2.1 — Ler o ADR-0022 inteiro e identificar a secção

```bash
cat 00_nucleo/adr/typst-adr-0022-*.md
```

Localizar a secção `## Diagnóstico obrigatório antes de qualquer
código`. Identificar os limites exactos:
- Linha de início (o cabeçalho `## Diagnóstico obrigatório...`).
- Linha de fim (o próximo cabeçalho de nível igual ou superior,
  ou fim do ficheiro).

Todo o conteúdo entre estas linhas (inclusive o cabeçalho) é o
que será movido.

### Passo 2.2 — Identificar o slug do ficheiro de destino

O nome do ficheiro de ADR-0022 é algo como
`typst-adr-0022-fontbook-crossroads.md` ou similar. O slug
(segmento após `0022-`) vira parte do nome do diagnóstico.

```bash
ls 00_nucleo/adr/typst-adr-0022*.md
```

Ficheiro de diagnóstico: `00_nucleo/diagnosticos/diagnostico-adr-0022-<mesmo-slug>.md`.

### Passo 2.3 — Criar ficheiro de diagnóstico

Conteúdo do ficheiro de destino:

```markdown
# Diagnóstico prévio à ADR-0022

**Contexto**: registo do diagnóstico executado antes da decisão
formalizada em `00_nucleo/adr/typst-adr-0022-<slug>.md`.

**Data do diagnóstico**: [data do commit que adicionou a secção
ao ADR — ver git blame].

**Natureza**: este ficheiro é registo histórico. Os comandos
abaixo foram executados antes da decisão do ADR-0022 ser tomada;
o estado do código pode ter mudado desde então. Consultar este
ficheiro para entender o contexto factual da decisão, não para
re-executar.

---

[CONTEÚDO DA SECÇÃO "## Diagnóstico obrigatório" DO ADR-0022,
 SEM O CABEÇALHO "## Diagnóstico obrigatório antes de qualquer
 código" — o cabeçalho é substituído pelo título do ficheiro.]
```

O cabeçalho original `## Diagnóstico obrigatório antes de qualquer
código` é **removido** (substituído pelo título `# Diagnóstico
prévio à ADR-0022`). Os sub-cabeçalhos internos (ex: `### Passo 1`,
`### Verificar grep X`) são preservados tal qual.

**Determinar a data do diagnóstico** via `git blame` sobre a linha
do cabeçalho da secção:

```bash
git blame 00_nucleo/adr/typst-adr-0022-*.md | grep -B 2 "Diagnóstico obrigatório"
```

Pegar no SHA do commit, executar:

```bash
git show <sha> --format=%ci --no-patch
```

Usar a data retornada (formato `YYYY-MM-DD`) no cabeçalho do
diagnóstico. Se `git blame` não retornar nada utilizável (raro),
usar `[data não identificável]` e passar ao próximo.

### Passo 2.4 — Remover a secção do ADR-0022

No ficheiro do ADR-0022, eliminar **exactamente** as linhas
entre o cabeçalho `## Diagnóstico obrigatório...` e o próximo
cabeçalho de nível igual ou superior (inclusive o cabeçalho de
diagnóstico; exclusive o cabeçalho seguinte).

### Passo 2.5 — Adicionar referência no ADR-0022

Identificar a secção de "Referências" do ADR-0022 (se existe) ou
criar uma se não existe (no fim do documento, antes de qualquer
linha final ou separador).

Adicionar linha:

```markdown
**Diagnóstico prévio**: ver
`00_nucleo/diagnosticos/diagnostico-adr-0022-<slug>.md` —
verificações executadas antes desta decisão.
```

A linha vai no topo da secção "Referências" ou imediatamente
antes das referências a outros ADRs, para ter destaque.

### Passo 2.6 — Verificar

```bash
# A secção foi removida
grep -c "Diagnóstico obrigatório" 00_nucleo/adr/typst-adr-0022-*.md
# Esperado: 0

# A referência foi adicionada
grep "diagnostico-adr-0022" 00_nucleo/adr/typst-adr-0022-*.md
# Esperado: 1 linha

# O ficheiro de diagnóstico existe
ls -l 00_nucleo/diagnosticos/diagnostico-adr-0022-*.md

# O ficheiro tem o cabeçalho correcto
head -5 00_nucleo/diagnosticos/diagnostico-adr-0022-*.md
# Esperado: "# Diagnóstico prévio à ADR-0022" seguido de contexto.

# Não há duplicação — o conteúdo está num sítio só
grep -c "\bgrep\b" 00_nucleo/adr/typst-adr-0022-*.md
# (Esperado: valor baixo, só mencões contextuais, não blocos de comandos.)
```

---

## Tarefa 3 — Mover diagnóstico do ADR-0023 (indexmap)

Repetir o procedimento da Tarefa 2, com as seguintes mudanças:

- Alvo: `00_nucleo/adr/typst-adr-0023-*.md`.
- Ficheiro de destino: `00_nucleo/diagnosticos/diagnostico-adr-0023-<slug>.md`.
- Título do diagnóstico: `# Diagnóstico prévio à ADR-0023`.
- Mesma linha de referência (ajustada para 0023).

Verificar no fim:
```bash
grep -c "Diagnóstico obrigatório" 00_nucleo/adr/typst-adr-0023-*.md
grep "diagnostico-adr-0023" 00_nucleo/adr/typst-adr-0023-*.md
ls -l 00_nucleo/diagnosticos/diagnostico-adr-0023-*.md
```

---

## Tarefa 4 — Mover diagnóstico do ADR-0025 (Int == Float)

Repetir o procedimento, com:

- Alvo: `00_nucleo/adr/typst-adr-0025-*.md`.
- Ficheiro de destino: `00_nucleo/diagnosticos/diagnostico-adr-0025-<slug>.md`.
- Título: `# Diagnóstico prévio à ADR-0025`.
- Referência ajustada para 0025.

Verificar no fim:
```bash
grep -c "Diagnóstico obrigatório" 00_nucleo/adr/typst-adr-0025-*.md
grep "diagnostico-adr-0025" 00_nucleo/adr/typst-adr-0025-*.md
ls -l 00_nucleo/diagnosticos/diagnostico-adr-0025-*.md
```

---

## Tarefa 5 — Verificação global

```bash
# O directório existe e tem exactamente 3 ficheiros
ls 00_nucleo/diagnosticos/
# Esperado: 3 ficheiros (diagnostico-adr-0022-*.md,
# diagnostico-adr-0023-*.md, diagnostico-adr-0025-*.md).

# Zero ocorrências de "Diagnóstico obrigatório" em qualquer ADR
grep -l "Diagnóstico obrigatório" 00_nucleo/adr/typst-adr-*.md
# Esperado: vazio (nenhum ficheiro listado).

# Os 3 ADRs alvo agora referenciam diagnósticos
for n in 0022 0023 0025; do
  grep "diagnostico-adr-$n" 00_nucleo/adr/typst-adr-$n-*.md
done
# Esperado: 3 linhas (uma por ADR).

# Nenhum outro ADR foi tocado
git diff --stat 00_nucleo/adr/
# Esperado: 3 ficheiros modificados (typst-adr-0022, 0023, 0025).
# Se aparecer qualquer outro ADR modificado, reverter.

# Código intacto
git status 01_core/ 02_shell/ 03_infra/ 04_wiring/
# Esperado: vazio.

# DEBT.md intacto
git diff --stat 00_nucleo/DEBT.md
# Esperado: vazio.

# Testes
cargo test

# Linter
crystalline-lint .
```

---

## Tarefa 6 — Verificar integridade do conteúdo movido

Cada ficheiro de diagnóstico deve ter **todo** o conteúdo da
secção original. Verificar por amostragem:

```bash
# O conteúdo dos 3 ficheiros não é trivial (> 20 linhas)
wc -l 00_nucleo/diagnosticos/*.md

# Cada ficheiro tem pelo menos uma ocorrência de comando bash
grep -c "grep\|ls\|cat\|awk" 00_nucleo/diagnosticos/*.md

# Cada ficheiro tem o cabeçalho correcto
head -1 00_nucleo/diagnosticos/*.md
# Esperado: 3 linhas iguais a:
# # Diagnóstico prévio à ADR-0022
# # Diagnóstico prévio à ADR-0023
# # Diagnóstico prévio à ADR-0025
```

Se algum ficheiro tem < 20 linhas ou nenhum comando, a extracção
falhou parcialmente. Reportar antes de concluir.

---

## Critérios de conclusão

- [ ] Directório `00_nucleo/diagnosticos/` criado.
- [ ] 3 ficheiros `diagnostico-adr-NNNN-<slug>.md` em
  `00_nucleo/diagnosticos/`, cada um com cabeçalho canónico e
  conteúdo integral da secção original.
- [ ] Secção `## Diagnóstico obrigatório...` removida dos 3 ADRs
  alvo (0022, 0023, 0025).
- [ ] Cada ADR alvo tem linha de referência ao ficheiro de
  diagnóstico correspondente.
- [ ] Nenhum outro ADR modificado.
- [ ] Nenhum ficheiro de código modificado.
- [ ] DEBT.md inalterado.
- [ ] `cargo test` mantém 911 testes.
- [ ] `crystalline-lint .` zero violations.

---

## Ao terminar, reportar

- Caminhos dos 3 ficheiros de diagnóstico criados.
- Tamanho de cada ficheiro de diagnóstico (linhas).
- Redução de tamanho de cada ADR alvo (linhas antes/depois).
- Data identificada por `git blame` para cada secção movida (ou
  `[data não identificável]` se aplicável).
- Confirmação de que a referência no ADR foi colocada na secção
  correcta (Referências ou equivalente).

**Go/No-Go para P84.8e** (novos ADRs: 0033 paridade vanilla, 0034
diagnóstico de tipos vanilla):

- **GO**: 3 diagnósticos movidos, 3 ADRs alvo sem a secção, 3
  linhas de referência adicionadas, código intacto.
- **NO-GO — conteúdo perdido**: se algum ficheiro de diagnóstico
  tem menos linhas do que o conteúdo original da secção no ADR
  (verificável por `git diff` ao ADR vs `wc -l` do diagnóstico),
  a extracção falhou. Reverter tudo, executar novamente com
  cuidado extra na Tarefa 2.1 (identificação dos limites da
  secção).
- **NO-GO — referência em sítio errado**: se a linha `**Diagnóstico
  prévio**: ...` foi colocada numa secção aleatória do ADR (ex:
  Contexto, Decisão), reposicionar para Referências.

---

## Nota sobre a cadência do projecto

Este é o sexto passo da série 84.8 (8a, 8b, 8c, 8d agora, 8e e 8f
planeados). A série inteira desce do P84.7 (auditoria) e materializa
as correcções identificadas.

Após 84.8d a situação dos ADRs é:
- 32 → 33 ADRs (0032 criado no 8a).
- 6 ADRs com status corrigido (8b).
- 2 ADRs com conteúdo expandido (8c).
- 3 ADRs sem anti-padrão "Diagnóstico obrigatório" (este passo).

Restante:
- 84.8e: 2 ADRs novos (0033 paridade vanilla, 0034 diagnóstico de
  tipos vanilla). ADR-0033 `Arc::clone` não será criado (absorvido
  em ADR-0030 no P84.8c — Opção α confirmada).
- 84.8f: índice `00_nucleo/adr/README.md`, resolução do ADR-0026
  duplicado, uniformização de vocabulário `ACCEPTED`/`IMPLEMENTADO`
  com introdução de `EM VIGOR` (Opção b pendente do P84.8b).
