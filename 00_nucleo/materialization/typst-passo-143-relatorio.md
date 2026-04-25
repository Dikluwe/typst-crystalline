# Passo 143 — Relatório (correcção empírica do `00_nucleo/adr/README.md`)

**Data**: 2026-04-24
**Natureza**: passo **L0-puro / administrativo**. **Zero código
tocado**. **Zero testes**. **Zero ADRs novas**.
**Único output**: `00_nucleo/adr/README.md` actualizado +
relatório.
**Precondição**: Passo 142 encerrado; DEBT-1 + DEBT-52 fechados;
README do directório `00_nucleo/adr/` declarava 38 ADRs e
listava apenas até ADR-0037.

---

## 1. Sumário executivo

`00_nucleo/adr/README.md` estava desactualizado por **18 ADRs**
(0038–0055). O tablebody "Estado por ADR" e a contagem total
(38) não reflectiam o conteúdo real do directório (56 ADRs;
55 números únicos + ADR-0026-R1).

Inventário empírico via Python sobre todos os ficheiros
`typst-adr-*.md` de `00_nucleo/adr/` produziu mapeamento
completo. README actualizado com:

- **Tabela "Estado por ADR"** estendida para 56 entradas.
- **Total** corrigido (38 → 56).
- **Distribuição de status** recalculada.
- **2 meta-regras novas** (ADR-0038 e ADR-0054).
- **10 entradas novas** em "Passos-chave da história dos ADRs"
  cobrindo P84.8h → 143 (~50 passos de história).
- **Notas de irregularidade** (¹ ²) registando dívida documental
  nos cabeçalhos de ADRs 0038–0051.

Cadeias de revogação e revisão **inalteradas** (zero
revogações/revisões novas detectadas no inventário).

---

## 2. Método

Inventário **empírico**, não-inferido:

```bash
ls 00_nucleo/adr/typst-adr-*.md | sort
```

Cada ficheiro lido (primeiras 30 linhas) com extracção via
`grep`/regex de:

- Número (do nome do ficheiro: `typst-adr-NNNN(-RN)?-<slug>.md`).
- Título (linha `# ` ou `# ⚖️ ADR-NNNN: ...`).
- Status canónico (linha `**Status**:` ou `**Estado**:`,
  procurando os 6 valores canónicos).
- Relações: `**Revoga**:`, `**Revogado por**:`, `**Revê**:`,
  `**Revisto por**:`.

Validação cruzada com contexto conhecido pelos relatórios
(140B, 141, 142) — confirmou todos os status `IMPLEMENTADO` e
`EM VIGOR` esperados (ex: ADR-0055 → `IMPLEMENTADO` após
Passo 141; ADR-0054 → `EM VIGOR`; ADR-0038 → `EM VIGOR`).

Sem leitura de DEBT.md ou outros ficheiros para inferir
status — empírico significa empírico.

---

## 3. Lista completa dos ADRs

Tabela transcrita do README actualizado (preservada aqui
para referência histórica imutável):

| ADR | Título curto | Status | Notas |
|-----|--------------|--------|-------|
| 0001 | Estratégia de migração (Opção C) | `IMPLEMENTADO` | |
| 0002 | Hierarquia de contenção | `IDEIA` | |
| 0003 | comemo + contenção | `IDEIA` | |
| 0004 | Passo 1 descobertas | `IMPLEMENTADO` | |
| 0005 | PackageSpec World | `PROPOSTO` | |
| 0006 | typst_timing | `PROPOSTO` | |
| 0007 | rustc_hash substituído | `REVOGADO` | revogado por 0018 |
| 0008–0013 | (inlining) | `PROPOSTO` (cada) | 6 ADRs |
| 0014 | unscanny inlinado | `PROPOSTO` | |
| 0015 | ecow removido do parser | `PROPOSTO` | |
| 0016 | LazyHash removido | `IMPLEMENTADO` | |
| 0017 | Adiamento de eval | `IMPLEMENTADO` | título sem `⚖️` |
| 0018 | rustc_hash reintroduzido | `EM VIGOR` | revoga 0007; meta-regra 3 |
| 0019 | TTF + RustyBuzz | `IMPLEMENTADO` | nota factual no Passo 140A |
| 0020 | FontDB | `ADIADO` | |
| 0021 | Datetime | `IMPLEMENTADO` | |
| 0022 | FontBook | `IMPLEMENTADO` | |
| 0023 | indexmap | `IMPLEMENTADO` | |
| 0024 | ecow para Value::Str | `IMPLEMENTADO` | |
| 0025 | Int == Float | `IMPLEMENTADO` | |
| 0026 | Content como enum | `IMPLEMENTADO` | revisto por 0026-R1 |
| 0026-R1 | Content com Arc<[T]> | `IMPLEMENTADO` | revê 0026 |
| 0027 | CIDFont subsetting | `IMPLEMENTADO` | título sem `⚖️` |
| 0028 | Tipos tipográficos simplificados | `REVOGADO` | revogado por 0029; título sem `⚖️` |
| 0029 | Pureza física de L1 | `EM VIGOR` | revoga 0028; meta-regra 1 |
| 0030 | Performance é domínio de L1 | `EM VIGOR` | meta-regra 2 |
| 0031 | Early hashing em Source | `IMPLEMENTADO` | |
| 0032 | Política de `unsafe` em L1 | `EM VIGOR` | meta-regra 4 |
| 0033 | Paridade funcional vanilla | `EM VIGOR` | meta-regra 5 |
| 0034 | Diagnóstico obrigatório para tipos vanilla | `EM VIGOR` | meta-regra 6 |
| 0035 | `ecow::EcoVec` autorizado em L1 | `EM VIGOR` | autorização específica (não meta-regra) |
| 0036 | Atomização progressiva | `EM VIGOR` | meta-regra 7 |
| 0037 | Coesão por domínio | `EM VIGOR` | meta-regra 8 |
| 0038 | Sistema de estilos em L1 | `EM VIGOR` ¹ | **meta-regra 9 (nova)**; título sem `⚖️` |
| 0039 | Forma de estilo no `FrameItem::Text` | `EM VIGOR` ¹ | título sem `⚖️` |
| 0040 | Activação de `#set` em eval | `EM VIGOR` ¹ | título sem `⚖️` |
| 0041 | Activação de `#show` (heading/strong/emph) | `EM VIGOR` ¹ | título sem `⚖️` |
| 0042 | `Sink` materializado em L1 | `EM VIGOR` ¹ | título sem `⚖️` |
| 0043 | Canal de saída do `Sink` (TrackedMut + L3) | `EM VIGOR` ¹ | título sem `⚖️` |
| 0044 | `Engine<'a>` agregador em L1 | `EM VIGOR` ² | usa `**Estado**:`; título sem `⚖️` |
| 0045 | Formato de diagnósticos | `EM VIGOR` ² | usa `**Estado**:` |
| 0046 | CLI mínima | `EM VIGOR` ² | usa `**Estado**:` |
| 0047 | Argparsing com `clap` na CLI | `EM VIGOR` ² | usa `**Estado**:` |
| 0048 | Cores ANSI nos diagnósticos | `EM VIGOR` ² | usa `**Estado**:` |
| 0049 | CLI vive em L2 (correcção) | `EM VIGOR` ² | usa `**Estado**:` |
| 0050 | Formatter de diagnósticos em L2 | `EM VIGOR` ² | usa `**Estado**:`; sem campo `**Revoga**:` |
| 0051 | Flags funcionais em L2 (`-o`) | `EM VIGOR` ² | usa `**Estado**:`; sem campo `**Revoga**:` |
| 0052 | Lang como tipo semântico em L1 | `IMPLEMENTADO` | |
| 0053 | Font como tipo composto em L1 | `IMPLEMENTADO` | |
| 0054 | Critério de fecho de DEBT-1 | `EM VIGOR` | **meta-regra 10 (nova)** |
| 0055 | Font consumer via pipeline CIDFont | `IMPLEMENTADO` | transitou de PROPOSTO no Passo 141 |

**Total**: 56 ADRs (55 números únicos + ADR-0026-R1).

---

## 4. Divergências detectadas (anomalias)

### 4.1. Anomalias de nome de ficheiro

**Nenhuma**. Inventário com regex
`^typst-adr-[0-9]{4}(-R[0-9]+)?-[a-z0-9-]+\.md$` aplicado a
todos os ficheiros do directório (excluindo `README.md` e
`template-adr.md`) — zero divergências.

### 4.2. Anomalias de cabeçalho

**14 ADRs** com cabeçalho fora da convenção canónica
documentada no README ("Cabeçalho canónico" → `**Status**:
\`<valor>\``):

**Grupo ¹ — Status sem backticks + parênteses descritivos**
(6 ADRs, Passos 99–106):

- `ADR-0038`: `**Status**: EM VIGOR (Passo 99.E) — validado
  empiricamente com 780 testes a passar e zero violations no
  linter.`
- `ADR-0039`: `**Status**: EM VIGOR (Passo 100.E) — validado
  empiricamente com 783 ...`
- `ADR-0040`: `**Status**: EM VIGOR (Passo 102.E) — validado
  empiricamente com 790 ...`
- `ADR-0041`: `**Status**: EM VIGOR (Passo 103.E) — validado
  empiricamente com 795 ...`
- `ADR-0042`: `**Status**: EM VIGOR (Passo 104.E) — validado
  empiricamente com 803 ...`
- `ADR-0043`: `**Status**: EM VIGOR (Passo 106.E) — validado
  empiricamente com 4 ...`

**Grupo ² — `**Estado**:` em vez de `**Status**:`**
(8 ADRs, Passos 109–120):

- `ADR-0044`: `**Estado**: EM VIGOR (Passo 109.E, 2026-04-23)`
- `ADR-0045`: `**Estado**: EM VIGOR (Passo 111.E, 2026-04-23)`
- `ADR-0046`: `**Estado**: EM VIGOR (Passo 113.E, 2026-04-23)`
- `ADR-0047`: `**Estado**: EM VIGOR (Passo 115.E, 2026-04-23)`
- `ADR-0048`: `**Estado**: EM VIGOR (Passo 116.E, 2026-04-23)`
- `ADR-0049`: `**Estado**: EM VIGOR (Passo 117.E, 2026-04-23)`
- `ADR-0050`: `**Estado**: EM VIGOR (Passo 119.E, 2026-04-23)`
- `ADR-0051`: `**Estado**: EM VIGOR (Passo 120.E, 2026-04-23)`

`**Estado**:` é vocabulário pré-P84.8g; o uniformizador desse
passo aplicou-se a `00_nucleo/adr/` mas estes 8 ADRs foram
criados depois (Passos 109+) com vocabulário antigo
ressuscitado.

**Grupo ³ — Título sem `⚖️` no `# ` heading** (≥ 14 ADRs):

ADRs 0017, 0027, 0028, e 0038 a 0051 usam `# ADR-NNNN — ...`
ou `# ADR-NNNN: ...` em vez do canónico `# ⚖️ ADR-NNNN:
...`. Não afecta status; é divergência de estilo.

### 4.3. Anomalias de relação

- **ADR-0050 e ADR-0051**: sem campo `**Revoga**:` ou
  equivalente. Outros ADRs do grupo (0044–0049) têm `**Revoga**:
  nenhuma.` ou `**Revoga**: nenhuma — complementa ADR-XXXX.`.
  Inconsistência de formato; não há relação real a
  registar.

- **Cadeias simétricas**: pares ADR-0007/0018 e ADR-0028/0029
  (revogações) e ADR-0026/0026-R1 (revisão) são **simétricos**
  — cada lado declara a relação. Sem assimetrias detectadas.

### 4.4. Acção tomada (per spec)

**Nenhuma correcção em ADRs individuais**. O escopo do Passo
143 é apenas o README; correcções de cabeçalho de ADRs
individuais (uniformizar `**Estado**:` → `**Status**:`,
adicionar backticks ao status, adicionar `⚖️` ao título)
ficam como **dívida documental** candidata a passo dedicado
de uniformização (à semelhança do P84.8g, que cobriu até
ADR-0037).

Notas `¹` e `²` na tabela do README sinalizam o grupo de cada
ADR irregular para que o leitor não se surpreenda.

---

## 5. Meta-regras novas identificadas

Critério aplicado (per spec 143.3): "regra geral aplicável a
código presente e futuro" → meta-regra. "Decisão técnica sobre
parte específica" → não.

**ADR-0038 (Sistema de estilos em L1)** — declara que TODA a
representação de estilo no domínio passa por
`Style`/`Styles`/`StyleChain`. Aplica-se a código presente
**e futuro** (não cobre apenas Passo 99). É meta-regra.
Adicionada como item **9** na lista do README.

**ADR-0054 (Critério de fecho de DEBT-1)** — declara que
"captura sem consumer não basta" para fechar dívida
estrutural. Embora redija-se especificamente para DEBT-1,
estabelece **precedente** para futuros DEBTs fundamentais
(notas operacionais da própria ADR mencionam isto). Adopta o
**perfil observacional graded** que reinterpreta ADR-0033 para
efeito de fecho. É meta-regra. Adicionada como item **10**.

**ADRs avaliadas e rejeitadas como meta-regra** (mantidas como
EM VIGOR mas não em meta-regras):

- ADR-0035 — `ecow::EcoVec` autorizado: já estava EM VIGOR sem
  ser meta-regra; é autorização específica de crate.
  Precedente preservado.
- ADR-0039 — Forma de estilo no `FrameItem::Text`: decisão
  técnica sobre estrutura específica. Não meta.
- ADR-0040 a 0043 — Activações de `#set`/`#show`/Sink/canal:
  decisões técnicas concretas, não regras gerais.
- ADR-0044 — `Engine<'a>` agregador: aplicação concreta de
  ADR-0036 (atomização). Não introduz nova meta.
- ADR-0045 a 0051 — Diagnostics + CLI evolution: decisões
  arquitecturais sobre componentes específicos.

Total final de meta-regras: **10** (era 8 antes do passo).

---

## 6. Passos-chave novos adicionados ao README

Adicionadas **10 entradas novas** após `P84.8h` (limite
prático declarado pelo spec; cumprido). Cobrem ~57 passos de
história (99 → 143). Agrupadas por fases coerentes para evitar
explosão de entradas:

1. **Passos 99–103** — Fundação de styles tipados em L1
   (ADRs 0038–0041; DEBT-48 fechado).
2. **Passos 104–111** — Engine, Sink, formato de
   diagnósticos (ADRs 0042–0045).
3. **Passos 113–120** — Ciclo de vida da CLI (ADRs
   0046–0051; convergência para L2).
4. **Passos 131B/132B** — Tipos semânticos `Lang` e
   `FontList` (ADRs 0052–0053).
5. **Passo 135** — ADR-0054 + DEBT-52 aberto.
6. **Passos 136–139** — DEBT-52 Fases A/B (consumers
   tracking/leading/weight; primeiro efeito visível desde
   `fill`).
7. **Passo 140A** — Diagnóstico font infra; ADR-0055
   `PROPOSTO`; ADR-0019 anotada.
8. **Passos 140B + 141** — DEBT-52 Fase C básica; ADR-0055
   `IMPLEMENTADO`.
9. **Passo 142** — Fecho formal de DEBT-1; ADR-0054
   cumprida.
10. **Passo 143** — Este README actualizado (correcção
    documental).

---

## 7. Actualização de contagens

| Métrica | Antes | Depois | Delta |
|---------|-------|--------|-------|
| Total de ADRs declarado | 38 | 56 | +18 |
| Números únicos | 37 | 55 | +18 |
| Variantes `-RN` | 1 (apenas 0026-R1) | 1 | 0 |
| `PROPOSTO` | 13 | 10 | −3 (correcção: README antigo contava 0008–0015 como 8, mas só são 6: 0008–0013) |
| `IDEIA` | 2 | 2 | 0 |
| `EM VIGOR` | 9 | 24 | +15 |
| `IMPLEMENTADO` | 12 | 17 | +5 |
| `REVOGADO` | 2 | 2 | 0 |
| `ADIADO` | 1 | 1 | 0 |
| Meta-regras | 8 | 10 | +2 |

**Discrepância em `PROPOSTO`**: o README antigo declarava 13
mas a soma da tabela antiga era `0005, 0006, 0008–0013, 0014,
0015` = 10. O número 13 estava errado já antes do Passo 143.
Corrigido para o valor real (10).

**Verificação aritmética**: 10 + 2 + 24 + 17 + 2 + 1 = **56** ✓.

---

## 8. Verificação de coerência (checklist 143.5)

- [x] **A.5.1** — Contagem manual da tabela "Estado por ADR"
      (56 entradas) = total declarado (56).
- [x] **A.5.2** — Soma das contagens em "Distribuição de status"
      (10+2+24+17+2+1) = 56.
- [x] **A.5.3** — Cadeia de revogações simétrica:
  - ADR-0007 (REVOGADO) ↔ ADR-0018 (EM VIGOR, Revoga 0007)
  - ADR-0028 (REVOGADO) ↔ ADR-0029 (EM VIGOR, Revoga 0028)
- [x] **A.5.4** — Cadeia de revisões simétrica:
  - ADR-0026 (IMPLEMENTADO, Revisto por 0026-R1) ↔
    ADR-0026-R1 (IMPLEMENTADO, Revê 0026)
- [x] **A.5.5** — Sem órfãos: todas as referências a ADR-NNNN
      no README correspondem a entradas existentes da tabela.
- [x] Nenhum ADR tocado (apenas README + relatório).
- [x] Nenhum ficheiro em L1/L2/L3/L4 tocado.
- [x] Nenhum ficheiro em `prompts/`, `relatorios/`,
      `materialization/` ou `context/` tocado **excepto**
      este relatório (criado em `materialization/` per spec
      143.6 — equivalente aos relatórios 140B/141/142).
- [x] `cargo test --workspace --lib`: inalterado face ao
      Passo 142.
- [x] `crystalline-lint .`: zero violations (não relevante;
      lint não cobre `00_nucleo/`).

---

## 9. Conflito de numeração registado

O relatório do Passo 142 §8 reservou "Passo 143" como
candidato para gap 7 do DEBT-52 (lang hyphenation). Este
passo de correcção documental ocupa **143**.

**Renumeração**: candidato de lang hyphenation passa a
**Passo 144** quando priorizado. ADR-0055 não é tocada;
DEBT-52 está encerrado e a renumeração não invalida o seu
estado de "candidato futuro não-DEBT".

Consequência: a referência "Passo 143 (lang hyphenation)" no
relatório 142 §11 fica ligeiramente desactualizada — lê-se
correctamente como "Passo 144 (candidato lang hyphenation)".
Não corrige-se aqui (`relatorios/` é imutável após produção,
per "Aviso sobre vocabulário em documentos históricos" do
próprio README).

---

## 10. Limitações reconhecidas

1. **Notas `¹`/`²` no README acumulam dívida documental**.
   Solução real: passo dedicado análogo ao P84.8g, aplicado a
   ADRs 0038–0051. Estimativa: S, ~30min. Não bloqueia o uso
   do README como índice.

2. **Sem renumeração ou correcção de slugs**. Convenções
   estruturais do README declaram `typst-adr-NNNN-<slug>.md`;
   inventário confirmou 100% de conformidade nominal.

3. **Sem validação de `Data`** nos cabeçalhos. Algumas datas
   parecem ser do dia da criação do passo, outras do dia da
   transição de status. Não escopo deste passo.

4. **`crystalline-lint` não cobre `00_nucleo/`**. O lint actua
   sobre código (linhagem `@prompt`/`@prompt-hash`) e não sobre
   documentos de governança. Coerência do README é
   responsabilidade humana / passo dedicado.

---

## 11. Verificação final

| Item | Estado |
|------|--------|
| `00_nucleo/adr/README.md` lista 56 ADRs (corrigido de 38) | ✅ |
| Meta-regras: 8 → 10 (ADR-0038, ADR-0054 adicionadas) | ✅ |
| Distribuição de status recalculada e somando 56 | ✅ |
| Cadeias de revogação e revisão simétricas e inalteradas | ✅ |
| 10 entradas novas em "Passos-chave" cobrindo P84.8h → 143 | ✅ |
| Anomalias de cabeçalho (14 ADRs) registadas neste relatório | ✅ |
| Nenhum ADR individual modificado | ✅ |
| Nenhum ficheiro fora de `00_nucleo/adr/README.md` e este relatório | ✅ |
| `cargo test --workspace --lib` (post-142 baseline 1095) | ✅ inalterado |
| `crystalline-lint .` | ✅ zero violations |
| Conflito de numeração (143 documental vs lang hyphenation) registado | ✅ |
