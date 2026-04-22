# Passo 84.8h — Canonizar ADR-0002/0003 e criar README.md de índice de ADRs

## Estado actual antes de começar

Ler antes de começar:
- Todos os 35 ADRs em `00_nucleo/adr/typst-adr-*.md`.
- `00_nucleo/adr/typst-adr-0002-*.md` e
  `00_nucleo/adr/typst-adr-0003-*.md` — alvos da Tarefa 1.
- `00_nucleo/relatorios/relatorio-auditoria-adrs-passo-84.7.md`,
  Secção 7 (proposta original de índice).

Pré-condição: `cargo test` — 911 testes, zero violations. P84.8g
concluído, 9 ADRs com status uniformizado, vocabulário canónico
de 6 valores em vigor.

---

## Natureza deste passo

**Passo final da série 84.8.** Dois produtos:

1. Canonizar status de ADR-0002 e ADR-0003 (`IDEIA — não
   implementar ainda` → `IDEIA` puro, qualificador movido para o
   corpo).
2. Criar `00_nucleo/adr/README.md` como índice canónico dos ADRs
   do Typst Cristalino.

Nenhum ADR novo. Nenhum DEBT. Nenhum código tocado.

Com este passo, a série 84.8 fecha. Próximo trabalho do projecto
retoma o ataque a DEBTs em aberto.

---

## Tarefa 1 — Canonizar ADR-0002 e ADR-0003

### Ler o cabeçalho actual

```bash
head -15 00_nucleo/adr/typst-adr-0002-*.md
head -15 00_nucleo/adr/typst-adr-0003-*.md
```

Localizar a linha:
```markdown
**Status**: `IDEIA — não implementar ainda`
```

### Edição

Substituir por:
```markdown
**Status**: `IDEIA`
**Nota**: direcção registada para consideração futura; não
materializar em L1 sem reavaliação prévia.
```

Aplicar a edição em ambos (ADR-0002 e ADR-0003).

### Verificar

```bash
grep "^\*\*Status\*\*" \
  00_nucleo/adr/typst-adr-0002-*.md \
  00_nucleo/adr/typst-adr-0003-*.md
# Esperado: ambos retornam **Status**: `IDEIA`

grep "^\*\*Nota\*\*" \
  00_nucleo/adr/typst-adr-0002-*.md \
  00_nucleo/adr/typst-adr-0003-*.md
# Esperado: ambos retornam a nota.

# Confirmar que o grep de valores únicos agora retorna 6 valores
# canónicos puros, sem qualificadores
grep -h "^\*\*Status\*\*" 00_nucleo/adr/typst-adr-*.md | sort -u
```

Esperado — exactamente 6 linhas:
```
**Status**: `ADIADO`
**Status**: `EM VIGOR`
**Status**: `IDEIA`
**Status**: `IMPLEMENTADO`
**Status**: `PROPOSTO`
**Status**: `REVOGADO`
```

Se houver qualquer 7º valor, **parar e reportar**.

---

## Tarefa 2 — Criar `00_nucleo/adr/README.md`

Criar o ficheiro com a estrutura abaixo. Esta é a estrutura final,
**não template** — o conteúdo abaixo é para inserir literalmente.

```markdown
# Índice de ADRs do Typst Cristalino

Este documento é o índice canónico dos Architectural Decision
Records (ADRs) do projecto **Typst Cristalino**. Lista os 35 ADRs
em vigor, as meta-regras que governam o projecto, o vocabulário
canónico de status, cadeias de revogação e revisão, e convenções
estruturais.

Para ADRs do projecto `crystalline-lint` (guardião arquitectural
deste projecto), consultar o repositório separado desse projecto
— não vivem aqui.

---

## Meta-regras em vigor

As seguintes regras arquitecturais, políticas, e invariantes estão
em vigor no projecto. São citadas por passos de materialização e
por outros ADRs:

1. **Pureza física de L1** — ADR-0029 (revoga ADR-0028). L1 não
   faz I/O de sistema (filesystem, rede, relógio, env). RAM é
   domínio; computação pura é esperada.

2. **Performance é domínio de L1** — ADR-0030. Gestão eficiente
   de RAM (`Arc`, `EcoString`, alocação planeada, `Arc::clone`
   como partilha O(1)) é comportamento correcto, não optimização
   especulativa. Inclui secção "Clone profundo vs `Arc::clone`".

3. **Critério de autorização externa** — ADR-0018. Crate em
   `[l1_allowed_external]` é autorizada se não viola pureza
   funcional (determinismo, sem I/O), independentemente de ser
   externa. Exemplo: `rustc_hash`, `comemo`.

4. **Política de `unsafe` em L1** — ADR-0032. Zero `unsafe` como
   objectivo; excepções apenas com prova de custo medido. DEBT-40
   (ImportGuard), DEBT-41 (sealed traits) e DEBT-42 (get_unchecked,
   bloqueado por benchmark) registam o trabalho pendente.

5. **Paridade funcional com vanilla** — ADR-0033. Para o mesmo
   input, output observável é idêntico ao Typst vanilla.
   Divergência permitida em forma estrutural (struct vs enum,
   `Vec` vs `Arc<[T]>`); divergência proibida em semântica
   observável (operadores, mensagens de erro, ordem de execução
   visível).

6. **Diagnóstico obrigatório para tipos vanilla** — ADR-0034.
   Antes de materializar tipo do Typst vanilla em L1, produzir
   diagnóstico estruturado em `00_nucleo/diagnosticos/`. Regra
   vinculativa, com 7 itens de conteúdo mínimo.

---

## Vocabulário canónico de status

O campo `**Status**:` de cada ADR usa um dos seis valores abaixo,
todos com backticks:

| Valor | Semântica | Exemplos |
|-------|-----------|----------|
| `PROPOSTO` | Decisão tomada mas ainda não em vigor nem implementada | ADR-0005, 0006, 0008-0015 |
| `IDEIA` | Direcção a considerar, pode não vir a ser implementada | ADR-0002, 0003 |
| `EM VIGOR` | Regra ou política arquitectural aceite e activa | ADR-0018, 0029, 0030, 0032, 0033, 0034 |
| `IMPLEMENTADO` | Decisão técnica concreta materializada em código | ADR-0001, 0004, 0016, 0017, 0019, 0021-0027, 0026-R1, 0031 |
| `REVOGADO` | Superseded por ADR posterior com número novo | ADR-0007, 0028 |
| `ADIADO` | Decisão tomada com implementação diferida por prazo ou condição | ADR-0020 |

**Distinção `EM VIGOR` vs `IMPLEMENTADO`**: um ADR é `EM VIGOR`
se formaliza regra/política/invariante sem código específico que
"implemente" a regra (aplica-se a todo o código presente e
futuro). É `IMPLEMENTADO` se documenta decisão técnica concreta
que corresponde a mudança específica no código.

---

## Estado por ADR

| ADR | Título curto | Status |
|-----|--------------|--------|
| 0001 | Estratégia de migração (Opção C) | `IMPLEMENTADO` |
| 0002 | Hierarquia de contenção | `IDEIA` |
| 0003 | comemo + contenção | `IDEIA` |
| 0004 | Passo 1 descobertas | `IMPLEMENTADO` |
| 0005 | PackageSpec World | `PROPOSTO` |
| 0006 | typst_timing | `PROPOSTO` |
| 0007 | rustc_hash substituído | `REVOGADO` (por ADR-0018) |
| 0008 | (inlining) | `PROPOSTO` |
| 0009 | (inlining) | `PROPOSTO` |
| 0010 | (inlining) | `PROPOSTO` |
| 0011 | (inlining) | `PROPOSTO` |
| 0012 | (inlining) | `PROPOSTO` |
| 0013 | (inlining) | `PROPOSTO` |
| 0014 | unscanny inlinado | `PROPOSTO` |
| 0015 | ecow removido do parser | `PROPOSTO` |
| 0016 | LazyHash removido | `IMPLEMENTADO` |
| 0017 | Adiamento de eval | `IMPLEMENTADO` |
| 0018 | rustc_hash reintroduzido | `EM VIGOR` (revoga ADR-0007) |
| 0019 | TTF + RustyBuzz | `IMPLEMENTADO` |
| 0020 | FontDB | `ADIADO` |
| 0021 | Datetime | `IMPLEMENTADO` |
| 0022 | FontBook | `IMPLEMENTADO` |
| 0023 | indexmap | `IMPLEMENTADO` |
| 0024 | ecow para Value::Str | `IMPLEMENTADO` |
| 0025 | Int == Float | `IMPLEMENTADO` |
| 0026 | Content como enum | `IMPLEMENTADO` (revisto por ADR-0026-R1) |
| 0026-R1 | Content com Arc<[T]> | `IMPLEMENTADO` (revê ADR-0026) |
| 0027 | CIDFont subsetting | `IMPLEMENTADO` |
| 0028 | Tipos tipográficos simplificados | `REVOGADO` (por ADR-0029) |
| 0029 | Pureza física de L1 | `EM VIGOR` (revoga ADR-0028) |
| 0030 | Performance é domínio de L1 | `EM VIGOR` |
| 0031 | Early hashing em Source | `IMPLEMENTADO` |
| 0032 | Política de `unsafe` em L1 | `EM VIGOR` |
| 0033 | Paridade funcional vanilla | `EM VIGOR` |
| 0034 | Diagnóstico obrigatório para tipos vanilla | `EM VIGOR` |

**Total**: 35 ADRs (34 números únicos; ADR-0026 tem variante -R1
por revisão).

### Distribuição de status

- `PROPOSTO`: 13 ADRs (decisões em aberto).
- `IDEIA`: 2 ADRs.
- `EM VIGOR`: 6 ADRs (regras/políticas activas).
- `IMPLEMENTADO`: 12 ADRs (decisões materializadas).
- `REVOGADO`: 2 ADRs.
- `ADIADO`: 1 ADR.

---

## Cadeia de revogações

Revogação: decisão anterior deixa de estar em vigor; ADR novo
com **número novo** substitui. Campos simétricos:
`**Revoga**` (no ADR novo) e `**Revogado por**` (no ADR revogado).

```
ADR-0007 (rustc_hash substituído)    ◄─revogado por─  ADR-0018 (rustc_hash reintroduzido)
ADR-0028 (tipos tipográficos)        ◄─revogado por─  ADR-0029 (pureza física)
```

## Cadeia de revisões

Revisão: decisão anterior continua em vigor no núcleo; ADR
posterior **no mesmo número** com sufixo `-RN` refina algum
aspecto (forma interna, nova optimização). Campos simétricos:
`**Revê**` (no R) e `**Revisto por**` (no original).

```
ADR-0026 (Content como enum)  ◄─revisto por─  ADR-0026-R1 (Content com Arc<[T]>)
```

---

## Convenções estruturais

### Ficheiros de ADR

- Localização: `00_nucleo/adr/`.
- Nome: `typst-adr-NNNN-<slug-kebab-case>.md`.
  - `NNNN`: número de 4 dígitos com zeros à esquerda.
  - `<slug>`: descrição curta em kebab-case.
- Revisões: sufixo `-RN` após o número: `typst-adr-NNNN-RN-<slug>.md`.

### Cabeçalho canónico

```markdown
# ⚖️ ADR-NNNN: <título>

**Status**: `<valor canónico>`
**Revoga**: ADR-XXXX        ← apenas se revoga outro
**Revogado por**: ADR-YYYY  ← apenas se foi revogado
**Revê**: ADR-NNNN          ← apenas em ADRs -RN
**Revisto por**: ADR-NNNN-R1 ← apenas se tem revisão
**Data**: YYYY-MM-DD
```

Campos de relação (Revoga, Revogado por, Revê, Revisto por)
aparecem imediatamente após `**Status**`, antes de `**Data**`.

### Corpo canónico

Secções típicas (ordem sugerida, não obrigatória):
- Contexto
- Decisão
- Alternativas Consideradas (tabela)
- Consequências (positivas, negativas, neutras)
- Referências

Secções **proibidas** no ADR (identificadas no P84.7 Secção 5.4):
- `## Diagnóstico obrigatório antes de qualquer código` — anti-padrão
  eliminado no P84.8d. Diagnósticos vivem em
  `00_nucleo/diagnosticos/`; o ADR referencia via linha
  `**Diagnóstico prévio**: ver <caminho>`.

---

## Directórios relacionados

Estrutura de `00_nucleo/`:

| Directório | Mutabilidade | Conteúdo |
|------------|--------------|----------|
| `adr/` | Vivo (editável por passos de correcção) | Architectural Decision Records |
| `prompts/` | Vivo | Instruções para execução futura (passos de materialização, regras do linter) |
| `diagnosticos/` | Vivo (cresce; cada ficheiro é imutável após criação) | Registos factuais executados antes de decisões arquitecturais |
| `relatorios/` | Imutável após produção | Snapshots de auditoria (83.5, 84.7, etc.) |
| `materialization/` | Imutável | Enunciados dos passos executados pelo Claude Code |
| `context/` | Imutável | Artefactos de análise em momentos específicos do desenvolvimento |
| `DEBT.md` | Vivo | Inventário da dívida técnica |

**Distinção operacional**:
- **Prompts**: "o que fazer no futuro".
- **Diagnósticos**: "o que foi verificado antes de uma decisão".
- **Relatórios**: "snapshot de estado num momento de auditoria".
- **Materialization**: "histórico do que foi feito".
- **Context**: "análises em pontos do desenvolvimento".

---

## Aviso sobre vocabulário em documentos históricos

Documentos em `relatorios/`, `materialization/` e `context/` são
**imutáveis**. Alguns foram produzidos antes da uniformização de
vocabulário do P84.8g (realizada em 2026-04-XX) e podem conter:

- Valores de status em inglês: `ACCEPTED`, `UPDATED`.
- Formatos alternativos: `**Estado**:`, `**Status:**` (dois
  pontos interiores), `\*\*Status\*\*:` (backslashes literais).
- Nome antigo do ADR-0026-R1: `ADR-0026-revisao`,
  `typst-adr-0026-revisao-content-arc.md`.

Estas ocorrências são **esperadas** nesses directórios e **não
devem** ser corrigidas — os documentos históricos preservam o
vocabulário da sua data de produção.

O vocabulário canónico actual (documentado neste README) aplica-se
apenas a `00_nucleo/adr/` e a documentos produzidos a partir do
P84.8g.

---

## Passos-chave da história dos ADRs

- **P84.5 (DEBT-36)** — Align2D materializado. Primeira aplicação
  consciente da paridade semântica (ADR-0033 formalizou depois).
- **P84.7** — Auditoria completa dos ADRs. Relatório em
  `00_nucleo/relatorios/relatorio-auditoria-adrs-passo-84.7.md`
  identificou desalinhamentos, lacunas, e regras implícitas.
  Input directo para a série 84.8.
- **P84.8a** — ADR-0032 criado. DEBTs 40, 41, 42 abertos.
- **P84.8b** — 6 ADRs corrigidos (status desalinhado). Convenção
  `**Revogado por**:` introduzida.
- **P84.8c** — ADR-0029 expandido (20 tipos tipográficos do
  vanilla). ADR-0030 ganhou secção "Clone profundo vs
  `Arc::clone`".
- **P84.8d** — Anti-padrão "Diagnóstico obrigatório" eliminado de
  ADR-0022/0023/0025. Directório `00_nucleo/diagnosticos/` criado.
- **P84.8e** — ADR-0033 e ADR-0034 criados.
- **P84.8f** — ADR-0026-revisao renomeado para ADR-0026-R1.
  Convenção `-RN` formalizada.
- **P84.8g** — Vocabulário de status uniformizado (6 valores
  canónicos). `ACCEPTED` e `UPDATED` eliminados como valores de
  status.
- **P84.8h** — Este README criado. Canonização residual em
  ADR-0002 e ADR-0003.
```

### Verificar após criar

```bash
ls -la 00_nucleo/adr/README.md
wc -l 00_nucleo/adr/README.md
# Esperado: ~200-250 linhas.

# Grep às secções principais
grep "^## " 00_nucleo/adr/README.md
# Esperado: Meta-regras em vigor, Vocabulário canónico, Estado por
# ADR, Cadeia de revogações, Cadeia de revisões, Convenções
# estruturais, Directórios relacionados, Aviso sobre vocabulário,
# Passos-chave.

# Tabela de estado tem 35 entradas (34 linhas de ADR + cabeçalho
# dividido em 0026 e 0026-R1 = 35 linhas de dados)
grep -c "^| 00[0-9][0-9]" 00_nucleo/adr/README.md
# Esperado: 35.
```

---

## Tarefa 3 — Verificação global

```bash
# Contagem de ADRs inalterada
ls 00_nucleo/adr/typst-adr-*.md | wc -l
# Esperado: 35.

# README.md existe em 00_nucleo/adr/
ls 00_nucleo/adr/README.md

# ADR-0002 e ADR-0003 agora têm IDEIA puro
grep "^\*\*Status\*\*" 00_nucleo/adr/typst-adr-0002-*.md
grep "^\*\*Status\*\*" 00_nucleo/adr/typst-adr-0003-*.md

# Grep final de valores únicos
grep -h "^\*\*Status\*\*" 00_nucleo/adr/typst-adr-*.md | sort -u
# Esperado: exactamente 6 linhas correspondentes aos 6 valores
# canónicos.

# Código intacto
git status 01_core/ 02_shell/ 03_infra/ 04_wiring/

# DEBT.md intacto
git diff --stat 00_nucleo/DEBT.md

# Relatórios, diagnósticos, materialization, context intactos
git diff --stat 00_nucleo/relatorios/ \
                00_nucleo/diagnosticos/ \
                00_nucleo/materialization/ \
                00_nucleo/context/

# Apenas 3 ficheiros tocados: ADR-0002, ADR-0003, README.md novo
git status 00_nucleo/adr/

# Testes
cargo test

# Linter
crystalline-lint .
```

---

## Critérios de conclusão

- [ ] ADR-0002 e ADR-0003 têm `**Status**: \`IDEIA\`` puro +
  campo `**Nota**` explicativo.
- [ ] `00_nucleo/adr/README.md` criado com todas as 9 secções
  principais.
- [ ] Tabela "Estado por ADR" tem 35 entradas.
- [ ] Grep final de valores únicos retorna exactamente 6 valores
  canónicos.
- [ ] Nenhum outro ADR alterado.
- [ ] Nenhum código-fonte tocado.
- [ ] Nenhum DEBT aberto ou fechado.
- [ ] Directórios `relatorios/`, `diagnosticos/`,
  `materialization/`, `context/` intactos.
- [ ] `cargo test` mantém 911 testes.
- [ ] `crystalline-lint .` zero violations.

---

## Ao terminar, reportar

- Confirmação da canonização de ADR-0002 e ADR-0003.
- Tamanho do README.md criado (linhas).
- Confirmação de que a tabela tem 35 linhas de dados.
- Lista final dos 6 valores únicos de status após este passo.
- Confirmação de que todos os directórios imutáveis ficaram
  intocados.
- Total de ficheiros modificados em `00_nucleo/adr/` (esperado:
  3 — ADR-0002, ADR-0003, README.md novo).

**Fecho da série 84.8**: este é o último passo da série. Após a
sua execução, a série fecha e o projecto retoma trabalho sobre
os DEBTs em aberto:

- **Prioridade "prontos para atacar"**: DEBT-40 (ImportGuard),
  DEBT-41 (sealed traits no scanner), DEBT-1, DEBT-9, outros
  DEBTs em Secção 1 do DEBT.md.
- **Bloqueados**: DEBT-42 (get_unchecked, depende de benchmark).

---

## Nota sobre caminho de ficheiro do relatório

Este passo **não produz relatório**. Os produtos são as 2
correcções em ADR-0002/0003 e o README.md canónico.
