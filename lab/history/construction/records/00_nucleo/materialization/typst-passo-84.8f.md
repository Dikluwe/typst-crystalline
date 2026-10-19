# Passo 84.8f — Resolver ADR-0026 duplicado (renomear revisão para -R1)

## Estado actual antes de começar

Ler antes de começar:
- `00_nucleo/adr/typst-adr-0026-content-divergencia.md` — original.
- `00_nucleo/adr/typst-adr-0026-revisao-content-arc.md` — revisão.
  Nome exacto pode divergir — verificar `ls`.
- `00_nucleo/relatorios/relatorio-auditoria-adrs-passo-84.7.md` —
  Secção 5.2 (motivação).

Pré-condição: `cargo test` — 911 testes, zero violations. P84.8e
concluído, 35 ADRs no total.

---

## Natureza deste passo

**Passo de refactor estrutural pequeno.** Renomeia um ficheiro,
ajusta conteúdo dos dois ADRs-0026 e actualiza qualquer
referência cruzada no resto do projecto.

Nenhum código tocado. Nenhum DEBT. Nenhum ADR novo.

Quatro produtos:

1. Renomeação: `typst-adr-0026-revisao-content-arc.md` →
   `typst-adr-0026-R1-content-arc.md`.
2. ADR-0026-R1 ganha campo `**Revê**: ADR-0026` logo após Status.
3. ADR-0026 original ganha campo `**Revisto por**: ADR-0026-R1`
   logo após Status, e mantém `IMPLEMENTADO` com nota sobre a
   revisão (Opção C da Pergunta 3).
4. Qualquer referência no resto do projecto a "0026-revisao" ou
   "ADR-0026 (revisao)" é actualizada para "ADR-0026-R1".

---

## Decisão formalizada neste passo

### Convenção `-RN` para revisões de ADRs

Quando um ADR é revisto (com ou sem revogação), o novo ficheiro
recebe sufixo `-R1`, `-R2`, etc., preservando o número original.
A cadeia histórica fica na mesma família numérica.

Diferença entre "revisão" e "revogação":
- **Revogação** (convenção `**Revogado por**:`): decisão
  anterior deixa de estar em vigor; ADR novo com **número novo**
  substitui. Exemplo: ADR-0028 revogado por ADR-0029.
- **Revisão** (convenção `-R1`, `**Revisto por**:`, `**Revê**:`):
  decisão anterior continua em vigor no seu núcleo, mas um ADR
  posterior no mesmo número refina algum aspecto (forma interna,
  nova optimização, etc.). Exemplo: ADR-0026 revisto por
  ADR-0026-R1.

Esta distinção formaliza-se **neste passo** e fica documentada
aqui porque é a primeira aplicação.

---

## Tarefa 1 — Confirmar nomes exactos dos dois ficheiros

```bash
ls 00_nucleo/adr/typst-adr-0026*
```

Esperado:
```
00_nucleo/adr/typst-adr-0026-content-divergencia.md
00_nucleo/adr/typst-adr-0026-revisao-content-arc.md
```

Se os nomes divergirem do esperado, ajustar as Tarefas seguintes
aos nomes reais encontrados, mantendo a lógica (o segmento "revisao"
é o que vira "R1").

---

## Tarefa 2 — Grep de referências existentes ao ficheiro a renomear

Antes de renomear, identificar **todas** as referências ao ficheiro
actual no projecto:

```bash
# Referências ao nome do ficheiro "revisao-content-arc"
grep -rn "0026-revisao\|revisao-content-arc" \
  00_nucleo/ \
  01_core/src/ \
  02_shell/src/ \
  03_infra/src/ \
  04_wiring/src/ \
  lab/ \
  2>/dev/null
```

```bash
# Referências ao texto "ADR-0026 (revisao)" ou variantes
grep -rn "ADR-0026\s*(revisao)\|ADR-0026-revisao\|ADR 0026 revisao" \
  00_nucleo/ \
  01_core/ \
  02_shell/ \
  03_infra/ \
  04_wiring/ \
  lab/ \
  2>/dev/null
```

```bash
# Referências genéricas a ADR-0026 (para saber se há ambiguidade
# em citações existentes — algumas podem referir o original, outras
# a revisão)
grep -rn "ADR-0026" \
  00_nucleo/ \
  01_core/ \
  02_shell/ \
  03_infra/ \
  04_wiring/ \
  lab/ \
  2>/dev/null
```

**Reportar o resultado do terceiro grep antes de prosseguir.** Cada
referência genérica `ADR-0026` precisa de ser analisada para
determinar se se refere ao original ou à revisão — só depois
dessa análise se pode decidir se essa referência precisa de ser
actualizada para `ADR-0026-R1` ou deixada como está.

### Critério de desambiguação

- Se o contexto da referência fala de **`Content` como enum** ou
  **estrutura de Content** de forma geral: refere-se ao **original**
  (`ADR-0026`). **Não alterar**.
- Se o contexto fala de **`Arc<[Content]>`**, **partilha**, ou
  **clone O(1) de Content**: refere-se à **revisão** (`ADR-0026-R1`).
  **Actualizar** para `ADR-0026-R1`.
- Se ambíguo, **reportar individualmente** antes de alterar.

---

## Tarefa 3 — Renomear o ficheiro da revisão

```bash
git mv 00_nucleo/adr/typst-adr-0026-revisao-content-arc.md \
       00_nucleo/adr/typst-adr-0026-R1-content-arc.md
```

**Uso de `git mv`** (não `mv` simples) preserva o historial do
ficheiro para `git log --follow`.

Verificar:
```bash
ls 00_nucleo/adr/typst-adr-0026*
# Esperado: dois ficheiros, original + R1.

git status | grep "0026"
# Esperado: mostra rename detectado (R100 ou similar indicando
# renomeação sem mudança de conteúdo).
```

---

## Tarefa 4 — Adicionar bidirecionalidade — ADR-0026-R1 (ficheiro renomeado)

Ler o cabeçalho actual:
```bash
head -15 00_nucleo/adr/typst-adr-0026-R1-content-arc.md
```

Verificar se já existe campo `**Revê**:` ou equivalente. Se não
existe, adicionar imediatamente após a linha `**Status**:`.

### Edição

Após a linha:
```markdown
**Status**: `UPDATED`
```

Adicionar (se não presente):
```markdown
**Revê**: ADR-0026
```

Resultado final do cabeçalho:
```markdown
# ⚖️ ADR-0026-R1: Content com Arc<[Content]>

**Status**: `UPDATED`
**Revê**: ADR-0026
**Data**: <data original>
```

**Não alterar** o resto do ficheiro neste passo. O título do ADR
pode conter "Revisão" ou "Revisão de ADR-0026" — se contiver,
pode ser actualizado para incluir o sufixo `-R1` no número, mas
isso é decisão estética do utilizador. Alternativamente, manter
título tal como está.

### Verificar

```bash
grep -E "^\*\*Status\*\*|^\*\*Revê\*\*" \
  00_nucleo/adr/typst-adr-0026-R1-content-arc.md | head -4
# Esperado:
# **Status**: `UPDATED`
# **Revê**: ADR-0026
```

---

## Tarefa 5 — Adicionar bidirecionalidade — ADR-0026 (original)

Ler o cabeçalho actual:
```bash
head -15 00_nucleo/adr/typst-adr-0026-content-divergencia.md
```

Confirmar status actual (espera-se `IMPLEMENTADO` conforme
auditoria do P84.7).

### Edição

Após a linha `**Status**:`, adicionar campo `**Revisto por**:` e
nota explicativa:

```markdown
**Status**: `IMPLEMENTADO`
**Revisto por**: ADR-0026-R1
**Nota**: decisão original continua em vigor (Content como enum
em L1). ADR-0026-R1 refina a forma interna de `Content::Sequence`
para `Arc<[Content]>` — ver esse ADR para detalhes da
implementação corrente.
```

**Não alterar** o resto do ficheiro. O status permanece
`IMPLEMENTADO` (Opção C da Pergunta 3 confirmada — a decisão não
foi revogada, só refinada).

### Verificar

```bash
grep -E "^\*\*Status\*\*|^\*\*Revisto por\*\*|^\*\*Nota\*\*" \
  00_nucleo/adr/typst-adr-0026-content-divergencia.md | head -4
# Esperado:
# **Status**: `IMPLEMENTADO`
# **Revisto por**: ADR-0026-R1
# **Nota**: ...
```

---

## Tarefa 6 — Actualizar referências identificadas na Tarefa 2

Para cada referência identificada como "refere-se à revisão"
(critério na Tarefa 2), executar substituição pontual.

Recomenda-se **não** usar `sed` em massa. Cada substituição deve
ser verificada no contexto. Abordagem:

1. Para cada ficheiro identificado, abrir o ficheiro.
2. Localizar a referência específica.
3. Aplicar substituição:
   - `0026-revisao` → `0026-R1`
   - `revisao-content-arc.md` → `typst-adr-0026-R1-content-arc.md`
   - `ADR-0026 (revisao)` → `ADR-0026-R1`
   - `ADR-0026-revisao` → `ADR-0026-R1`

Para cada ficheiro alterado, reportar:
- Caminho do ficheiro.
- Linhas alteradas (antes → depois).

Se não houver nenhuma referência a actualizar (caso possível se os
diagnósticos, relatórios e passos anteriores só citaram `ADR-0026`
genericamente), **reportar como "nenhuma actualização necessária"**
e passar à Tarefa 7.

### Ficheiros esperados que possam conter referências

Conhecidos do contexto da série 84.8:
- `00_nucleo/adr/typst-adr-0030-performance-dominio-l1.md` — secção
  "Clone profundo vs Arc::clone" (adicionada no 84.8c) cita
  `ADR-0026-revisao`.
- `00_nucleo/adr/typst-adr-0033-paridade-funcional-vanilla.md` —
  Exemplo 2 cita `ADR-0026 + ADR-0026-revisao`.
- `00_nucleo/adr/typst-adr-0034-diagnostico-tipos-vanilla.md` —
  possível referência.
- `00_nucleo/relatorios/relatorio-auditoria-adrs-passo-84.7.md` —
  relatório histórico; ver Tarefa 8.

---

## Tarefa 7 — Não alterar relatórios históricos

**Regra**: ficheiros em `00_nucleo/relatorios/` **não** são
actualizados neste passo, mesmo que contenham referências ao nome
antigo.

Justificação: relatórios são snapshots. O relatório do P84.7 foi
produzido quando o ficheiro se chamava `typst-adr-0026-revisao-...`;
alterar o relatório agora reescreve história. A convenção (já
estabelecida no P83.6) é que relatórios são imutáveis após produção.

Se alguém ler o relatório do P84.7 no futuro e encontrar
`ADR-0026-revisao`, o README.md de índice (a criar no P84.8h) deve
ter nota explicando a renomeação. O relatório mantém-se tal como
foi escrito.

**Verificação negativa**:
```bash
git diff --stat 00_nucleo/relatorios/
# Esperado: vazio. Se aparecer qualquer ficheiro de relatório
# modificado, reverter.
```

---

## Tarefa 8 — Verificação global

```bash
# Os dois ADRs-0026 existem com nomes correctos
ls 00_nucleo/adr/typst-adr-0026*
# Esperado:
# typst-adr-0026-content-divergencia.md
# typst-adr-0026-R1-content-arc.md

# Zero referências ao nome antigo em ADRs, prompts, diagnósticos
grep -rn "0026-revisao\|revisao-content-arc" \
  00_nucleo/adr/ \
  00_nucleo/prompts/ \
  00_nucleo/diagnosticos/ \
  2>/dev/null
# Esperado: vazio.

# Referências antigas em relatórios SÃO esperadas (imutáveis)
grep -rn "0026-revisao" 00_nucleo/relatorios/ 2>/dev/null
# Pode retornar ocorrências — aceitável.

# Bidirecionalidade presente
grep "^\*\*Revisto por\*\*" \
  00_nucleo/adr/typst-adr-0026-content-divergencia.md
# Esperado: **Revisto por**: ADR-0026-R1

grep "^\*\*Revê\*\*" \
  00_nucleo/adr/typst-adr-0026-R1-content-arc.md
# Esperado: **Revê**: ADR-0026

# Contagem de ADRs continua 35 (rename não altera contagem)
ls 00_nucleo/adr/typst-adr-*.md | wc -l

# Código intacto
git status 01_core/ 02_shell/ 03_infra/ 04_wiring/

# DEBT.md intacto
git diff --stat 00_nucleo/DEBT.md

# Diagnósticos intactos
git diff --stat 00_nucleo/diagnosticos/

# Testes
cargo test

# Linter
crystalline-lint .
```

---

## Critérios de conclusão

- [ ] Tarefa 2 executada — grep reportado com classificação das
  referências (refere-se a original, refere-se a revisão,
  ambígua).
- [ ] Ficheiro renomeado via `git mv` (histórico preservado).
- [ ] ADR-0026-R1 tem `**Revê**: ADR-0026` após Status.
- [ ] ADR-0026 original tem `**Revisto por**: ADR-0026-R1` + nota,
  mantendo `IMPLEMENTADO`.
- [ ] Referências identificadas na Tarefa 6 actualizadas (ou
  reportado "nenhuma").
- [ ] Relatórios em `00_nucleo/relatorios/` **não** modificados.
- [ ] Nenhum código-fonte tocado.
- [ ] Nenhum DEBT aberto ou fechado.
- [ ] Testes 911.
- [ ] Linter limpo.

---

## Ao terminar, reportar

- Outputs dos 3 greps da Tarefa 2 (primeiros dois completos; o
  terceiro com classificação por referência: original vs revisão
  vs ambígua).
- Confirmação de renomeação `git mv` com hash da operação.
- Lista de ficheiros cuja Tarefa 6 alterou, com linhas antes e
  depois.
- Confirmação de que relatórios em `00_nucleo/relatorios/` ficaram
  intocados.
- Qualquer ambiguidade de desambiguação (referências a ADR-0026
  que não foi claro se eram ao original ou à revisão).

**Go/No-Go para P84.8g** (uniformizar vocabulário de status):

- **GO**: renomeação feita, bidirecionalidade formalizada em ambos
  os ADRs-0026, referências no projecto vivo actualizadas,
  relatórios imutáveis preservados.
- **NO-GO — referência perdida**: se o grep da Tarefa 8 final
  retornar ocorrências de `0026-revisao` em directórios vivos
  (adr/, prompts/, diagnosticos/), há referência que ficou
  desactualizada. Identificar e corrigir antes de fechar passo.
- **NO-GO — git mv falhou**: se `git status` mostrar os dois
  ficheiros como "novo + apagado" em vez de "renomeado",
  reverter e usar `git mv` correctamente. Caso contrário,
  historial fica partido.

---

## Nota sobre caminho de ficheiro do relatório

Este passo **não produz relatório**. Produtos são a renomeação e
as edições nos dois ADRs-0026.
