# Passo 147 — Relatório (actualização dos documentos de paridade)

**Data**: 2026-04-24
**Natureza**: passo **L0-puro / administrativo**. **Zero
código**. **Zero testes**. **Zero ADRs criadas, revogadas ou
revisadas**. Apenas reescrita factual de 2 documentos
`PROPOSTO` para reflectir o estado pós-Passo 146.
**Precondição**: Passo 146 encerrado; multi-font materializado;
1113 tests; zero violations; 57 ADRs; 10 DEBTs abertos;
DEBT-1 + DEBT-52 fechados.

---

## 1. Sumário executivo

Os documentos `00_nucleo/diagnosticos/typst-paridade-definicoes.md`
e `00_nucleo/diagnosticos/typst-paridade-plano-medicao.md`
foram reescritos cirúrgicamente para reflectir o estado de
**2026-04-24 (Passo 146)**. Os documentos originais foram
escritos quando o projecto estava em "Passo 19–21" — referências
temporais ao "Passo 17 já foi", "Passo 19, actual",
"Passo 21+" estavam ~127 passos desactualizadas.

**4 secções reescritas** no plano de medição (`§Contexto`,
`§2 Existe/Falta`, `§7 Tabela de passos`, `§9 Próximas
acções`); **status canonizado** de `**Estado**: PROPOSTO`
para `**Status**: \`PROPOSTO\`` (P84.8g + P145); **aviso de
revisão** adicionado ao topo de ambos.

**Decisões conceptuais preservadas literalmente** onde são
válidas: 4 níveis P1–P4, DTOs propostos (`ValueDTO`,
`FrameDTO`, `pdf_compare`), modos de comparação (`text_content`
/ `structural` / `geometric`), tolerâncias configuráveis
(`absolute_pt`, `max_pixel_diff`, `max_diff_ratio`), corpus
categorizado, métrica como matriz.

**Materialização concreta da infra** (`frame_dto.rs`,
`value_dto.rs`, `pdf_compare.rs`, tests P3/P2/P4, expansão do
corpus, primeiro relatório `latest.md`) **fica para o Passo
148**.

---

## 2. Inventário pré-actualização (sub-passo 147.1)

### 2.1. Localização real dos ficheiros

```
00_nucleo/diagnosticos/typst-paridade-definicoes.md      (242 linhas)
00_nucleo/diagnosticos/typst-paridade-plano-medicao.md   (276 linhas)
```

Localização **`diagnosticos/`** confirmada — directório vivo
(per README dos ADRs §"Directórios relacionados"). Edição é
permitida; sem necessidade de mover. **Decisão**: preservar
localização.

### 2.2. Estado real de `lab/parity/`

```
lab/parity/
├── Cargo.toml         (433 bytes)
├── corpus/
│   ├── code/          (3 ficheiros)
│   ├── markup/        (4 ficheiros)
│   └── math/          (4 ficheiros)
│   = 11 ficheiros total
├── src/
│   ├── compact.rs
│   └── main.rs
└── tests/
    └── parse_parity.rs
```

**`lab/parity/Cargo.toml`** confirma: pacote `typst-parity`
fora do workspace cristalino, com bin `parity-runner`,
deps `typst-syntax` (do `lab/typst-original/`) +
`typst-core` (path `../../01_core`). Dev-deps:
`pretty_assertions` + `walkdir`.

**Sem ficheiros novos** desde a escrita do documento original
(P9 + ajustes posteriores). Corpus continua nos 11 ficheiros
declarados. `value_dto.rs`, `frame_dto.rs`, `pdf_compare.rs`,
`report.rs` **não existem** — coerente com `**Falta**` do
documento.

### 2.3. Referências cruzadas (sub-passo 147.7)

Probe `grep -rn "typst-paridade-..." 00_nucleo/ lab/parity/`
fora dos próprios ficheiros: **única ocorrência** em
`00_nucleo/materialization/typst-passo-147.md` (este passo).
Nenhum outro documento referencia os dois `.md` por path.
**Sem actualizações de referências cruzadas** necessárias.

ADR/Passo/DEBT mencionados nos documentos verificados como
factualmente válidos (todos existem na arquitectura actual):
ADR-0001, 0016, 0026, 0027, 0033, 0054, 0055, 0057;
Passos 9, 14, 22+, 137, 138, 139, 142, 144, 146, 148;
DEBT-1 (encerrado), DEBT-52 (encerrado).

---

## 3. Discrepâncias detectadas

| # | Discrepância | Resolução |
|---|--------------|-----------|
| 1 | "**§Contexto**: Passos 19–21 (layout, início de export PDF)" — ~127 passos desactualizado. | Reescrito em §Contexto do plano para reflectir Passo 146. |
| 2 | "**§2 Existe**: Pipeline end-to-end em L1 (Passo 19)" — pipeline actual é L1+L3 (eval/layout em L1; export em L3 via `compile_to_pdf_bytes`). | Reescrito com facto factual; export PDF + multi-font + hyphenation listados em "Existe". |
| 3 | "**§7 Ligação com a sequência de passos**": tabela com "Passo 17 já foi", "Passo 19, actual", "Passo 21+" — passos passados como futuros. | Tabela inteira reescrita; coluna "Quando" passa a referir Passo 148+ (futuros) e "Estado em 2026-04-24" (actual). |
| 4 | "**§9 Próximas acções**": refere "depois do Passo 21" — desactualizado. | Reescrito apontando Passo 148 (próximo), 149+ (P2), 150+ (P4). Numeração indicativa. |
| 5 | `**Estado**: PROPOSTO` (vocabulário antigo, pré-P84.8g). | Migrado para `**Status**: \`PROPOSTO\`` (canónico P145). |
| 6 | Definições não tem campo `Status` algum (omissão). | Adicionado `**Status**: \`PROPOSTO\`` + `**Data**: 2026-04-24` no topo. |
| 7 | Sem aviso de revisão em ambos. | Aviso adicionado em ambos os topos descrevendo a actualização. |

**Discrepâncias herdadas registadas** (não corrigidas por
estarem fora do escopo P147):

- Documento referencia `lab/parity/src/value_dto.rs` (etc.)
  como ficheiros propostos. Realidade: não existem. Coerente
  com `**Falta**`.
- Corpus mínimo: documento dizia "11 ficheiros". Inventário
  confirma exactamente 11. **Sem discrepância**.

---

## 4. Reescritas aplicadas

### 4.1. `typst-paridade-plano-medicao.md`

#### Cabeçalho + §Contexto

```diff
- **Estado**: PROPOSTO
- **Data**: 2026-04-24
- **Contexto**: o projeto está entre os Passos 19–21 (layout, início de export PDF). A estimativa inicial era de ~10 passos no ADR-0001; o número actual cresceu para 146 porque a análise revelou que `typst-library` tem de ser estratificada antes de `eval()` poder migrar (ADR-0016, ADR-0026). A pergunta "em que percentual de paridade estamos?" não tem hoje uma resposta numérica porque o projeto mede testes acumulados, não paridade comparativa.
+ **Status**: `PROPOSTO`
+ **Data**: 2026-04-24
+
+ > **Revisto no Passo 147 (2026-04-24)**: ...
+
+ **Contexto**: o projecto tem **146 passos executados**.
+ Pipeline end-to-end ... está **estável** desde o Passo 22+;
+ multi-font, hyphenation e consumer integral de `StyleDelta`
+ (DEBT-52: 6/8 gaps materializados; gap 7 fechado em 144)
+ existem. **DEBT-1 fechado** no Passo 142 com cumprimento de
+ ADR-0054 (perfil observacional graded). 57 ADRs vigentes;
+ 10 DEBTs abertos. ...
```

#### §2 — Existe / Falta

"Existe" estendido com:
- Pipeline end-to-end em L1+L3 (não apenas L1).
- Export PDF estável + multi-font (Passo 146) + hyphenation
  (Passo 144) + consumer integral 9/10 campos.
- 57 ADRs vigentes; DEBT-1 + DEBT-52 encerrados.

"Falta" preservado conceptualmente; clarificado que a
materialização é o Passo 148.

#### §7 — Tabela

Substituída tabela de 6 entradas (referindo Passos 17/19/20+/21+
como condições temporais). Nova tabela com 5 entradas:
"Estado em 2026-04-24" + "Passo 148" + "Passo 149+" +
"Passo 150+" + "A cada N passos".

#### §9 — Próximas acções

Lista de 6 itens reescrita para 4 itens (item 5 e 6 do
original mesclados pois `eval()` e `export_pdf()` já
estabilizaram):

1. Passo 148 — `frame_dto.rs` + `tests/layout_parity.rs`
   + `latest.md`.
2. Passo 149+ — P2 (`value_dto.rs`).
3. Passo 150+ — P4 Opção B; Opção A futura.
4. Decisão sobre corpus (oficial vs próprio vs ambos).

### 4.2. `typst-paridade-definicoes.md`

Adicionado no topo:

```diff
+ **Status**: `PROPOSTO`
+ **Data**: 2026-04-24
+
+ > **Revisto no Passo 147 (2026-04-24)**: ...
```

**Conteúdo material (P1/P2/P3/P4 sections)** preservado
literalmente — todas as decisões conceptuais (modos,
tolerâncias, convenções, DTOs propostos) continuam válidas
e são input directo para o Passo 148.

---

## 5. Aviso de revisão acrescentado

Ambos os documentos ganham bloco no topo (após cabeçalho
Status/Data):

```markdown
> **Revisto no Passo 147 (2026-04-24)**: o documento original
> foi escrito durante a fase precoce do projecto (referências
> a "Passos 19–21" eram contemporâneas). O projecto está
> agora em Passo 146; pipeline end-to-end estável; DEBT-1
> fechado por ADR-0054 (perfil observacional graded).
> Decisões conceptuais do documento (4 níveis P1–P4, DTOs
> propostos, modos de comparação) **permanecem válidas** e
> são alvo de materialização em Passo 148. Discrepâncias de
> contexto temporal foram corrigidas neste passo. Vocabulário
> de status migrado de `**Estado**: PROPOSTO` para o canónico
> `**Status**: \`PROPOSTO\`` (P84.8g + P145).
```

Variação no aviso de `definicoes.md` para reflectir que o
documento não tem `§Contexto` separado e que mais APIs
referenciadas reflectem o pós-146.

---

## 6. Decisão de localização canónica

**Preservada**: `00_nucleo/diagnosticos/`. Razão:

- `diagnosticos/` é "vivo (cresce; cada ficheiro é imutável
  após criação)" segundo o README dos ADRs.
- Os documentos de paridade não são prompts L0 (não governam
  código com hash); não são relatórios (não são snapshots
  imutáveis); não são contexto (não são análises de momento
  específico).
- Mais próximos de **diagnóstico operacional**: factuais +
  estruturais + perenes mas editáveis quando estado real
  evolui.

**Ressalva**: o README dos ADRs declara `diagnosticos/` como
"cada ficheiro é imutável após criação". Os documentos de
paridade contradizem essa imutabilidade (foram editados
neste passo). Esta tensão fica registada como **dívida
documental** — possível distinção futura entre
"diagnósticos snapshot" (imutáveis) e "diagnósticos
operacionais" (editáveis), ou movimentação para um
sub-directório dedicado (ex: `diagnosticos/operacional/`).
**Não corrigido neste passo** — escopo é actualização
factual, não redefinição arquitectural de
`00_nucleo/diagnosticos/`.

---

## 7. Próximo passo: 148

**Passo 148** materializa o que estes documentos descrevem
como "Falta":

1. `lab/parity/src/frame_dto.rs` com `LayoutTolerance` e
   modo `text_content=true` (mínimo viável).
2. `lab/parity/tests/layout_parity.rs` invocando o corpus
   actual (11 ficheiros).
3. `lab/parity/reports/latest.md` — primeira matriz de
   paridade real.
4. Decisão sobre corpus (oficial vs próprio vs ambos).

Passos 149+ e 150+ ficam como **trabalho condicional**
(numeração indicativa). Se o Passo 148 crescer em escopo,
ramifica em sub-passos.

A pergunta que motivou esta série ("em que paridade
estamos?") passa a ter resposta numérica concreta a partir
do Passo 148 — em formato de matriz, não de percentual
único.

---

## 8. Verificação final

| Item | Estado |
|------|--------|
| `§Contexto` actualizado em `plano-medicao.md` | ✅ |
| `§2 Existe/Falta` reescrito | ✅ |
| `§7 Ligação com sequência de passos` reescrito | ✅ |
| `§9 Próximas acções` reescrito | ✅ |
| Aviso de revisão no topo de ambos | ✅ |
| Decisões conceptuais (P1–P4, DTOs, modos) preservadas | ✅ |
| Vocabulário canónico aplicado (`**Status**: \`PROPOSTO\``) | ✅ |
| Localização canónica preservada (`diagnosticos/`) | ✅ |
| Referências cruzadas verificadas (zero impacto externo) | ✅ |
| ADR/Passo/DEBT internos verificados como válidos | ✅ |
| Nenhum ficheiro em `lab/parity/` tocado | ✅ |
| Nenhum ficheiro em `01_core/`, `02_shell/`, `03_infra/`, `04_wiring/` tocado | ✅ |
| Nenhuma ADR criada / revogada / revisada | ✅ |
| `DEBT.md` e `00_nucleo/adr/README.md` intactos | ✅ |
| `cargo test --workspace --lib`: inalterado (1113) | ✅ |
| `crystalline-lint .`: zero violations | ✅ |
| Relatório do passo escrito | ✅ |

**Pós-147**: documentos coerentes com estado de 2026-04-24.
Passo 148 tem âncoras documentais correctas e pode
materializar a infra de medição de paridade sem ambiguidade.
