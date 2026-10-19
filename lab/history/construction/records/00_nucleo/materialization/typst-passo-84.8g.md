# Passo 84.8g — Uniformizar vocabulário de status nos ADRs

## Estado actual antes de começar

Ler antes de começar:
- Todos os ADRs em `00_nucleo/adr/typst-adr-*.md` (35 ficheiros).
- `00_nucleo/relatorios/relatorio-auditoria-adrs-passo-84.7.md`,
  Secção 5.3 (inconsistências de formato identificadas).
- ADRs recentes como referência de formato canónico: 0032, 0033,
  0034.

Pré-condição: `cargo test` — 911 testes, zero violations. P84.8f
concluído, ADR-0026 duplicado resolvido, bidirecionalidade
formalizada.

---

## Natureza deste passo

**Passo de uniformização de vocabulário.** Introduz novo valor de
status (`EM VIGOR`) para distinguir regras/políticas de decisões
técnicas materializadas. Actualiza status em ADRs afectados.
Remove `UPDATED` da lista canónica — mudança desses ADRs para
`IMPLEMENTADO`.

Nenhum ADR novo. Nenhum DEBT. Nenhum código tocado.

Três produtos:

1. Actualizar status de ~6 ADRs (regras/políticas) para `EM VIGOR`.
2. Actualizar status de ~6 ADRs em inglês (`ACCEPTED`) para o
   equivalente em português (`EM VIGOR` ou `IMPLEMENTADO`
   conforme classificação).
3. Actualizar status do ADR-0026-R1 de `UPDATED` para `IMPLEMENTADO`,
   eliminando `UPDATED` do vocabulário canónico.

---

## Decisões formalizadas neste passo

### Decisão 1 — Lista canónica de valores de status

**Seis valores canónicos** para o campo `**Status**:` em ADRs do
Typst Cristalino:

| Valor | Semântica | Exemplo |
|-------|-----------|---------|
| `PROPOSTO` | Decisão tomada mas ainda não em vigor nem implementada | ADR-0005, 0006 |
| `IDEIA` | Direcção a considerar, pode não vir a ser implementada | ADR-0002, 0003 |
| `EM VIGOR` | Regra ou política arquitectural aceite e activa (não se "implementa", aplica-se) | ADR-0029, 0030 |
| `IMPLEMENTADO` | Decisão técnica concreta materializada em código | ADR-0016, 0021 |
| `REVOGADO` | Superseded por ADR posterior com número novo | ADR-0007, 0028 |
| `ADIADO` | Decisão tomada com implementação diferida por prazo/condição | ADR-0020 |

**`UPDATED` é removido** do vocabulário — era o único valor em
inglês usado em ADR português, e tornou-se redundante depois da
convenção `-R1` formalizada no P84.8f (a relação "é revisão" fica
em `**Revê**`/`**Revisto por**`, não no status).

### Decisão 2 — Distinção `EM VIGOR` vs `IMPLEMENTADO`

Critério:

- **`EM VIGOR`** — o ADR formaliza regra, política, invariante
  arquitectural, ou critério de decisão. Não há "código que
  implementa" o ADR; o ADR aplica-se a todo o código presente e
  futuro. Perguntar: "este ADR seria alguma vez marcado como
  concluído pela materialização de um tipo ou função?" Se não,
  é `EM VIGOR`.

- **`IMPLEMENTADO`** — o ADR documenta decisão técnica concreta
  (escolher crate X, estrutura de dados Y, optimização Z). Há
  código que materializa a decisão. Perguntar: "este ADR descreve
  uma mudança específica que foi feita ao código?" Se sim, é
  `IMPLEMENTADO`.

### Decisão 3 — Formato de exibição

Todos os valores ficam com backticks:

```markdown
**Status**: `EM VIGOR`
**Status**: `IMPLEMENTADO`
```

Padrão já estabelecido no P84.8b; este passo apenas aplica a
novos valores.

---

## Tarefa 1 — Classificação dos ADRs

Tabela completa de classificação com decisão:

| ADR | Título curto | Status actual | Status final | Acção |
|-----|--------------|---------------|--------------|-------|
| 0001 | Estratégia de migração | `IMPLEMENTADO` | `IMPLEMENTADO` | Manter |
| 0002 | Hierarquia de contenção | `IDEIA` | `IDEIA` | Manter |
| 0003 | comemo + contenção | `IDEIA` | `IDEIA` | Manter |
| 0004 | Passo 1 descobertas | `IMPLEMENTADO` | `IMPLEMENTADO` | Manter |
| 0005 | PackageSpec World | `PROPOSTO` | `PROPOSTO` | Manter |
| 0006 | typst_timing | `PROPOSTO` | `PROPOSTO` | Manter |
| 0007 | rustc_hash substituído | `REVOGADO` | `REVOGADO` | Manter |
| 0008 | (inlining) | `PROPOSTO` | `PROPOSTO` | Manter |
| 0009 | (inlining) | `PROPOSTO` | `PROPOSTO` | Manter |
| 0010 | (inlining) | `PROPOSTO` | `PROPOSTO` | Manter |
| 0011 | (inlining) | `PROPOSTO` | `PROPOSTO` | Manter |
| 0012 | (inlining) | `PROPOSTO` | `PROPOSTO` | Manter |
| 0013 | (inlining) | `PROPOSTO` | `PROPOSTO` | Manter |
| 0014 | unscanny inlinado | `PROPOSTO` | `PROPOSTO` | Manter |
| 0015 | ecow removido parser | `PROPOSTO` | `PROPOSTO` | Manter |
| 0016 | LazyHash removido | `IMPLEMENTADO` | `IMPLEMENTADO` | Manter |
| 0017 | Adiamento eval | `IMPLEMENTADO` | `IMPLEMENTADO` | Manter |
| 0018 | rustc_hash reintroduzido | `IMPLEMENTADO` | **`EM VIGOR`** | **Alterar** |
| 0019 | TTF + RustyBuzz | `IMPLEMENTADO` | `IMPLEMENTADO` | Manter |
| 0020 | FontDB | `PROPOSTO — adiada` | `ADIADO` | **Alterar** (formato) |
| 0021 | Datetime | `IMPLEMENTADO` | `IMPLEMENTADO` | Manter |
| 0022 | FontBook | `IMPLEMENTADO` | `IMPLEMENTADO` | Manter |
| 0023 | indexmap | `IMPLEMENTADO` | `IMPLEMENTADO` | Manter |
| 0024 | ecow Value::Str | `IMPLEMENTADO` | `IMPLEMENTADO` | Manter |
| 0025 | Int == Float | `IMPLEMENTADO` | `IMPLEMENTADO` | Manter |
| 0026 | Content enum | `IMPLEMENTADO` | `IMPLEMENTADO` | Manter |
| 0026-R1 | Content Arc<[T]> | `UPDATED` | **`IMPLEMENTADO`** | **Alterar** |
| 0027 | CIDFont subsetting | `IMPLEMENTADO` | `IMPLEMENTADO` | Manter |
| 0028 | Tipos tipográficos | `REVOGADO` | `REVOGADO` | Manter |
| 0029 | Pureza física | `ACCEPTED` | **`EM VIGOR`** | **Alterar** |
| 0030 | Performance domínio L1 | `ACCEPTED` | **`EM VIGOR`** | **Alterar** |
| 0031 | Early hashing Source | `ACCEPTED` | **`IMPLEMENTADO`** | **Alterar** |
| 0032 | unsafe em L1 | `ACCEPTED` | **`EM VIGOR`** | **Alterar** |
| 0033 | Paridade vanilla | `ACCEPTED` | **`EM VIGOR`** | **Alterar** |
| 0034 | Diagnóstico tipos vanilla | `ACCEPTED` | **`EM VIGOR`** | **Alterar** |

**Total de alterações**: 9 ADRs.
- 5 para `EM VIGOR` (0018, 0029, 0030, 0032, 0033, 0034 — 6 ADRs,
  corrigir contagem: são 6).
- 2 para `IMPLEMENTADO` (0026-R1, 0031).
- 1 para `ADIADO` (0020, correcção de formato).

Correcção: **9 alterações totais** (6 + 2 + 1).

### Classificação justificada

**`EM VIGOR`** (regras/políticas/invariantes):
- **ADR-0018**: critério "viola pureza?" para autorizar externos.
  Regra de decisão, não implementação concreta.
- **ADR-0029**: pureza física em L1. Regra arquitectural
  fundamental.
- **ADR-0030**: performance é domínio. Regra.
- **ADR-0032**: política de `unsafe`. Política.
- **ADR-0033**: paridade vanilla. Invariante.
- **ADR-0034**: diagnóstico obrigatório. Política de processo.

**`IMPLEMENTADO`** (decisões técnicas concretas):
- **ADR-0031**: early hashing em Source. Optimização concreta
  materializada em código.
- **ADR-0026-R1**: `Arc<[Content]>` em `Content::Sequence`.
  Mudança concreta implementada.

**`ADIADO`** (correcção de formato):
- **ADR-0020**: actualmente `PROPOSTO — adiada`, formato não
  canónico. Fica `ADIADO` (valor canónico único).

---

## Tarefa 2 — Verificar estado actual antes de alterar

Antes de executar as 9 alterações, confirmar o status actual de
cada ADR alvo:

```bash
for n in 0018 0020 0026 0026-R1 0029 0030 0031 0032 0033 0034; do
  f=$(ls 00_nucleo/adr/typst-adr-$n*.md 2>/dev/null | head -1)
  if [ -n "$f" ]; then
    echo "=== $(basename $f) ==="
    grep -m1 "^\*\*Status\*\*" "$f"
  fi
done
```

Esperado:
- `0018` → `**Status**: `IMPLEMENTADO` ``
- `0020` → `**Status**: `PROPOSTO — adiada` ` (ou variante)
- `0026` (sem -R1) → `**Status**: `IMPLEMENTADO` ` (referência, não
  alterado)
- `0026-R1` → `**Status**: `UPDATED` `
- `0029-0034` → `**Status**: `ACCEPTED` `

Se algum ADR alvo tiver status diferente do esperado, **reportar
antes de prosseguir**. Pode ter havido alteração imprevista desde
o P84.8f.

---

## Tarefa 3 — Aplicar as 9 alterações

### ADR-0018 — `IMPLEMENTADO` → `EM VIGOR`

```markdown
**Status**: `EM VIGOR`
```

O ADR-0018 já tem campo `**Revoga**: ADR-0007`. **Não alterar** o
resto do cabeçalho.

### ADR-0020 — `PROPOSTO — adiada` → `ADIADO`

Substituir a linha de status pelo valor canónico único:

```markdown
**Status**: `ADIADO`
```

Se o corpo do ADR contém contexto sobre a razão do adiamento, **não
alterar** o corpo. O valor `ADIADO` já comunica o estado; a razão
continua documentada no corpo.

### ADR-0026-R1 — `UPDATED` → `IMPLEMENTADO`

```markdown
**Status**: `IMPLEMENTADO`
**Revê**: ADR-0026
```

Manter o campo `**Revê**` adicionado no P84.8f.

### ADR-0029 — `ACCEPTED` → `EM VIGOR`

```markdown
**Status**: `EM VIGOR`
**Revoga**: ADR-0028 (Representação Simplificada dos Tipos Tipográficos)
```

Manter o campo `**Revoga**`.

### ADR-0030 — `ACCEPTED` → `EM VIGOR`

```markdown
**Status**: `EM VIGOR`
```

### ADR-0031 — `ACCEPTED` → `IMPLEMENTADO`

```markdown
**Status**: `IMPLEMENTADO`
```

### ADR-0032 — `ACCEPTED` → `EM VIGOR`

```markdown
**Status**: `EM VIGOR`
```

### ADR-0033 — `ACCEPTED` → `EM VIGOR`

```markdown
**Status**: `EM VIGOR`
```

### ADR-0034 — `ACCEPTED` → `EM VIGOR`

```markdown
**Status**: `EM VIGOR`
```

---

## Tarefa 4 — Verificar alterações

### Verificar cada ADR alvo

```bash
for n in 0018 0020 0026-R1 0029 0030 0031 0032 0033 0034; do
  f=$(ls 00_nucleo/adr/typst-adr-$n*.md 2>/dev/null | head -1)
  if [ -n "$f" ]; then
    echo "=== $(basename $f) ==="
    grep -m1 "^\*\*Status\*\*" "$f"
  fi
done
```

Esperado:
- `0018` → `**Status**: `EM VIGOR` `
- `0020` → `**Status**: `ADIADO` `
- `0026-R1` → `**Status**: `IMPLEMENTADO` `
- `0029` → `**Status**: `EM VIGOR` `
- `0030` → `**Status**: `EM VIGOR` `
- `0031` → `**Status**: `IMPLEMENTADO` `
- `0032` → `**Status**: `EM VIGOR` `
- `0033` → `**Status**: `EM VIGOR` `
- `0034` → `**Status**: `EM VIGOR` `

### Verificar que `UPDATED` e `ACCEPTED` desapareceram

```bash
# Zero ocorrências de ACCEPTED em qualquer ADR (como valor de status)
grep -l "^\*\*Status\*\*:.*ACCEPTED" 00_nucleo/adr/typst-adr-*.md
# Esperado: vazio.

# Zero ocorrências de UPDATED em qualquer ADR (como valor de status)
grep -l "^\*\*Status\*\*:.*UPDATED" 00_nucleo/adr/typst-adr-*.md
# Esperado: vazio.

# Zero ocorrências de "Accepted" (variante inglês dois pontos — não
# deveria existir após P84.8b, mas confirmar)
grep -l "Accepted\|accepted" 00_nucleo/adr/typst-adr-*.md
# Pode retornar ocorrências em corpo de ADR (contexto textual),
# mas nenhuma como valor de status. Se retornar, inspecionar e
# confirmar que não é status.
```

### Verificar lista canónica aplicada uniformemente

```bash
# Listar todos os valores de status usados
grep -h "^\*\*Status\*\*" 00_nucleo/adr/typst-adr-*.md | sort -u
```

Esperado: apenas valores da lista canónica, algo como:
```
**Status**: `ADIADO`
**Status**: `EM VIGOR`
**Status**: `IDEIA`
**Status**: `IMPLEMENTADO`
**Status**: `PROPOSTO`
**Status**: `REVOGADO`
```

Se houver qualquer outro valor, **reportar para análise**.

---

## Tarefa 5 — Verificação global

```bash
# Contagem de ADRs inalterada (35)
ls 00_nucleo/adr/typst-adr-*.md | wc -l

# Código intacto
git status 01_core/ 02_shell/ 03_infra/ 04_wiring/

# DEBT.md intacto
git diff --stat 00_nucleo/DEBT.md

# Diagnósticos intactos
git diff --stat 00_nucleo/diagnosticos/

# Relatórios intactos
git diff --stat 00_nucleo/relatorios/

# Materialization intacto
git diff --stat 00_nucleo/materialization/

# Context intacto
git diff --stat 00_nucleo/context/

# Apenas 9 ADRs modificados
git diff --stat 00_nucleo/adr/ | grep -c "^ typst-adr"
# Esperado: 9

# Testes
cargo test

# Linter
crystalline-lint .
```

---

## Critérios de conclusão

- [ ] Tarefa 2 executada — status actual de cada ADR alvo
  confirmado antes de alterar.
- [ ] 9 alterações aplicadas conforme tabela da Tarefa 1.
- [ ] Nenhum ADR adicional modificado.
- [ ] Nenhum corpo de ADR alterado (apenas linhas de status e
  formato `PROPOSTO — adiada` de ADR-0020).
- [ ] Campos `**Revê**`, `**Revoga**`, `**Revogado por**`,
  `**Revisto por**` preservados onde existiam.
- [ ] Grep final da Tarefa 4 retorna apenas 6 valores únicos de
  status.
- [ ] Zero ocorrências de `ACCEPTED` ou `UPDATED` como valor de
  status em qualquer ADR.
- [ ] Nenhum código-fonte tocado.
- [ ] Nenhum DEBT aberto ou fechado.
- [ ] `cargo test` mantém 911 testes.
- [ ] `crystalline-lint .` zero violations.

---

## Ao terminar, reportar

- Output completo do grep da Tarefa 4 (lista de valores únicos).
- Confirmação das 9 alterações com status antes/depois para cada.
- Confirmação de que relatórios, materialization, context,
  diagnósticos e DEBT.md ficaram intocados.
- Qualquer anomalia encontrada na Tarefa 2 (ADR com status
  diferente do esperado).

**Go/No-Go para P84.8h** (README de índice):

- **GO**: 9 alterações aplicadas, vocabulário uniforme (6 valores
  canónicos), código e documentos históricos intocados.
- **NO-GO — valor não canónico aparece**: se o grep final mostrar
  qualquer valor fora dos 6 canónicos, identificar qual ADR está
  errado e corrigir antes de fechar o passo.
- **NO-GO — corpo de ADR alterado**: se algum `git diff` mostrar
  alterações fora do cabeçalho (linha de status), reverter a
  alteração de corpo. Este passo é estritamente sobre status.

---

## Nota sobre referências em relatórios históricos

Os ficheiros em `00_nucleo/relatorios/` (notavelmente o
`relatorio-auditoria-adrs-passo-84.7.md`) contêm a distribuição
de status como era antes desta uniformização (7 valores
misturados, incluindo `ACCEPTED` e `UPDATED`). Estes ficheiros
**não são actualizados** — são snapshots históricos.

O README de índice (P84.8h) terá nota explicando que relatórios
anteriores a 2026-04-22 (data aproximada deste passo) usam
vocabulário anterior.

Análoga salvaguarda aplica-se a `00_nucleo/materialization/` e
`00_nucleo/context/`, que são também directórios imutáveis.

---

## Nota sobre caminho de ficheiro do relatório

Este passo **não produz relatório**. O produto são 9 cabeçalhos
de ADR actualizados.
