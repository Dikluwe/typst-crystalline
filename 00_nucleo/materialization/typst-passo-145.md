# Passo 145 — Uniformização de cabeçalhos dos ADRs 0038–0051

**Série**: 145 (passo **L0-puro / administrativo**;
uniformização de cabeçalhos análoga a P84.8g, que cobriu
0001–0037).
**Precondição**: Passo 143 encerrado; README dos ADRs
canónico (56 ADRs listados); 14 ADRs com cabeçalho irregular
registados em §4.2 do relatório 143.

**Numeração**: este passo ocupa **145**. **144** está
reservado para o candidato de lang hyphenation (gap 7 de
DEBT-52, conforme relatório 143 §9). 145 é o número seguinte
disponível.

**Natureza**: passo **L0-puro / administrativo**. **Zero
código**. **Zero testes**. **Zero ADRs novas**. **Zero ADRs
revogadas/revisadas**. Apenas **edição de cabeçalho** em 14
ADRs existentes — preserva conteúdo material (Decisão,
Alternativas, Consequências, Referências) intacto.

**ADRs aplicáveis**:
- Nenhuma directamente. O passo **executa** as convenções
  documentadas na secção "Cabeçalho canónico" do
  `00_nucleo/adr/README.md`. Não cria, revê, nem revoga
  ADRs.

---

## Contexto

Relatório 143 §4.2 detectou três grupos de divergências de
cabeçalho nos ADRs 0038–0051:

**Grupo ¹** — 6 ADRs (0038–0043), Passos 99–106:

`**Status**: EM VIGOR (Passo NN.E) — validado empiricamente
com NNN testes a passar e zero violations no linter.`

Status sem backticks; parêntese descritivo + sufixo de
validação. Forma divergente da convenção `**Status**:
\`<valor canónico>\``.

**Grupo ²** — 8 ADRs (0044–0051), Passos 109–120:

`**Estado**: EM VIGOR (Passo NN.E, 2026-04-23)`

Campo nomeado `**Estado**:` em vez de `**Status**:`.
Vocabulário pré-P84.8g ressuscitado em passos posteriores.

**Grupo ³** — ≥14 ADRs (subset que inclui 0017, 0027, 0028 e
todos os 0038–0051): título sem `⚖️` no `# `.

P84.8g uniformizou 0001–0037 (eliminou `ACCEPTED`/`UPDATED`,
canonizou os 6 valores de status). Os passos 99–120 criaram
ADRs novas mas não seguiram a convenção uniformizada —
acumulou-se dívida documental que o relatório 143 explicitou.

Este passo é o equivalente de P84.8g para o intervalo
0038–0051, mais o cleanup de títulos sem `⚖️`. Trabalho é
**mecânico**: substituir cabeçalhos sem tocar no conteúdo.

---

## Objectivo

Ao fim do passo:

1. **6 ADRs do Grupo ¹** (0038–0043) com cabeçalho
   uniformizado:
   - `**Status**: \`EM VIGOR\``.
   - Informação de validação (Passo, número de testes)
     **preservada** numa nova linha `**Validado**:` ou
     equivalente, **abaixo** do cabeçalho canónico.

2. **8 ADRs do Grupo ²** (0044–0051) com cabeçalho
   uniformizado:
   - `**Estado**:` → `**Status**:`.
   - Valor com backticks: `\`EM VIGOR\``.
   - Informação de validação preservada análogamente ao
     Grupo ¹.

3. **Títulos com `⚖️`** adicionado a todos os ADRs do escopo
   que não o tenham (ADRs 0017, 0027, 0028 e 0038–0051).
   - Padrão: `# ⚖️ ADR-NNNN: <título>`.
   - Se actualmente é `# ADR-NNNN — <título>` (com travessão),
     normalizar para `:` (dois pontos), conforme o exemplo
     canónico do README.

4. **Relatório do passo** em
   `00_nucleo/materialization/typst-passo-145-relatorio.md`
   documentando:
   - Cada ADR tocado com diff do cabeçalho (antes/depois).
   - Decisão sobre preservação da informação de validação.
   - Confirmação de que conteúdo material (Decisão,
     Alternativas, Consequências, Referências) ficou
     intacto.

5. **README dos ADRs**: notas `¹` e `²` na tabela "Estado por
   ADR" **removidas**, dado que a irregularidade que
   sinalizavam foi corrigida. Notas no rodapé do README que
   explicavam `¹`/`²` removidas.

Este passo **não**:

- Toca código em L1, L2, L3, L4.
- Toca testes.
- Modifica conteúdo material de qualquer ADR (Decisão,
  Alternativas, Consequências, Referências, datas).
- Altera status de qualquer ADR.
- Cria, revoga ou revisa ADRs.
- Renomeia ficheiros.
- Toca ADRs fora do escopo 0038–0051 (excepto 0017, 0027 e
  0028 para o cleanup de `⚖️`, **se** o relatório 143
  confirmar que estão no Grupo ³).
- Toca ficheiros em `prompts/`, `relatorios/`, `context/`.
- Modifica `DEBT.md`.

---

## Decisões já tomadas

1. **Escopo restrito a cabeçalho**. Conteúdo material dos
   ADRs é tabu. Se algum ADR tiver erro material no corpo,
   regista-se no relatório como "candidato a passo dedicado
   futuro" e **não corrige-se**.
2. **Sem alteração de status**. Cada ADR mantém o status
   declarado (todos `EM VIGOR` no escopo). A correcção é
   apenas de **forma** (`**Estado**:` → `**Status**:`,
   adicionar backticks).
3. **Informação de validação preservada**. Os parênteses
   descritivos `(Passo NN.E) — validado empiricamente com
   NNN testes` contêm informação real que tem valor
   histórico. Não se elimina; movida para linha `**Validado**:`
   abaixo do cabeçalho canónico (ver 145.2).
4. **Sem ADR de revisão**. Uniformização de cabeçalho não
   é mudança de decisão arquitectural — é correcção de
   forma. Análogo ao P84.8g, que também não criou ADRs.

## Decisões diferidas (resolvidas neste passo)

5. **Nome exacto do campo de validação**: candidatos
   `**Validado**:`, `**Validação**:`, `**Materializado em**:`.
   Decisão em 145.1: confirmar se algum ADR já usa um destes;
   se sim, seguir precedente; se não, escolher
   `**Validado**:` (mais curto). Registar a escolha no
   relatório.
6. **Formato exacto do conteúdo do campo `**Validado**:`**:
   - Grupo ¹: `Passo NN.E — NNN testes; zero violations.`
     (transcrever os números reais do parêntese antigo).
   - Grupo ²: `Passo NN.E, 2026-04-23.` (transcrever a data
     existente).
7. **Travessão `—` vs dois pontos `:` no título do `#`
   heading**: convenção do README usa `:`. Normalizar para
   `:`. Se algum ADR usa `—` por design, registar.
8. **Confirmação de quais ADRs estão no Grupo ³** (título
   sem `⚖️`): inventário em 145.1 confirma. Relatório 143 §4.2
   diz "≥14" e nomeia 0017, 0027, 0028 + 0038–0051 (= 17 ADRs);
   o "≥" indica que pode haver mais. Empírico resolve.

---

## Escopo

**Dentro**:

- Edição de cabeçalho (linhas até `**Data**:` inclusive) dos
  14 ADRs do Grupo ¹+² (0038–0051).
- Adição de `⚖️` ao título dos ADRs identificados no
  inventário do Grupo ³ (esperado: subset de 0017, 0027,
  0028, 0038–0051).
- Adição de linha `**Validado**:` (ou equivalente) nos
  ADRs do Grupo ¹+² para preservar informação histórica.
- Edição do `00_nucleo/adr/README.md` para remover notas
  `¹` e `²` da tabela e do rodapé.
- Escrita do relatório do passo.

**Fora**:

- Modificação de **qualquer** secção do corpo dos ADRs
  abaixo do cabeçalho.
- Modificação de ADRs fora do escopo declarado.
- Modificação de status (todos `EM VIGOR` permanecem `EM
  VIGOR`).
- Modificação de relações (`Revoga`, `Revogado por`, `Revê`,
  `Revisto por`).
- Modificação da convenção do README (secção "Cabeçalho
  canónico" preservada literalmente — este passo executa-a,
  não a altera).
- Modificação de ADRs com cabeçalho já canónico (mesmo
  dentro do intervalo 0038–0051; verificar caso a caso em
  145.1).
- Renomeação de ficheiros.
- Modificação de datas.

---

## Sub-passos

### 145.1 — Inventário pré-correcção

**A.1.1 — Confirmar lista exacta dos Grupos ¹ e ²**:

```bash
for f in 00_nucleo/adr/typst-adr-{0038,0039,0040,0041,0042,0043,0044,0045,0046,0047,0048,0049,0050,0051}-*.md; do
  echo "=== $f ==="
  head -10 "$f"
  echo
done
```

Confirmar:
- ADRs 0038–0043 usam `**Status**: EM VIGOR (Passo ...)` sem
  backticks (Grupo ¹).
- ADRs 0044–0051 usam `**Estado**: EM VIGOR (...)` (Grupo ²).
- Detectar ADRs nesse intervalo que **já** estejam canónicos
  (e portanto fora do escopo deste passo). Esperado: zero,
  mas confirmar.

**A.1.2 — Confirmar lista exacta do Grupo ³**:

```bash
grep -L "^# ⚖️ ADR-" 00_nucleo/adr/typst-adr-*.md
```

Saída lista todos os ADRs que **não** têm `⚖️` no título.
Filtrar para o escopo deste passo (0017, 0027, 0028,
0038–0051) — ADRs fora desse intervalo que apareçam ficam
**fora do escopo** e são registados no relatório como dívida
remanescente, não corrigidos aqui.

**A.1.3 — Detectar precedente de campo de validação**:

```bash
grep -l "^\*\*Validado\*\*:\|^\*\*Validação\*\*:\|^\*\*Materializado em\*\*:" 00_nucleo/adr/*.md
```

Se algum ADR **fora** do escopo deste passo usa um destes
campos, seguir esse precedente. Se nenhum, escolher
`**Validado**:` (decisão tomada acima, item 5).

**A.1.4 — Detectar formato de título irregular**:

```bash
grep "^# ADR-" 00_nucleo/adr/*.md
```

Detectar se algum título usa `—` (travessão) em vez de `:`
(dois pontos). Esperado: alguns ADRs do Grupo ² podem usar
`—`. Normalizar para `:` na correcção.

### 145.2 — Correcção do Grupo ¹ (ADRs 0038–0043)

Para cada ADR do Grupo ¹, edição do cabeçalho.

**Forma actual** (exemplo ADR-0038):

```
# ADR-0038: Sistema de estilos em L1

**Status**: EM VIGOR (Passo 99.E) — validado empiricamente com 780 testes a passar e zero violations no linter.

**Data**: 2026-...
```

**Forma após edição**:

```
# ⚖️ ADR-0038: Sistema de estilos em L1

**Status**: `EM VIGOR`
**Validado**: Passo 99.E — 780 testes; zero violations.
**Data**: 2026-...
```

**Regras mecânicas**:

- Linha do título: prefixar `⚖️ ` após `# `, antes do
  `ADR-NNNN`. Se já tem `⚖️`, manter intacto.
- Linha do `**Status**:`: substituir tudo após `**Status**:`
  por ` \`EM VIGOR\`` (com backticks).
- Linha nova `**Validado**:`: inserida imediatamente abaixo
  do `**Status**:`. Conteúdo extraído do parêntese antigo:
  `Passo NN.E — NNN testes; zero violations.`. Manter os
  números exactos do ADR original.
- Linha do `**Data**:` e seguintes: **intactas**.

Repetir para 0039, 0040, 0041, 0042, 0043, ajustando os
números (Passo 100.E/102.E/103.E/104.E/106.E e contagens de
testes 783/790/795/803/4).

### 145.3 — Correcção do Grupo ² (ADRs 0044–0051)

Para cada ADR do Grupo ².

**Forma actual** (exemplo ADR-0044):

```
# ADR-0044 — Engine<'a> agregador em L1

**Estado**: EM VIGOR (Passo 109.E, 2026-04-23)
**Revoga**: nenhuma — complementa ADR-0036.
**Data**: 2026-04-23
```

**Forma após edição**:

```
# ⚖️ ADR-0044: Engine<'a> agregador em L1

**Status**: `EM VIGOR`
**Revoga**: nenhuma — complementa ADR-0036.
**Validado**: Passo 109.E.
**Data**: 2026-04-23
```

**Regras mecânicas**:

- Título: prefixar `⚖️ `; substituir `—` por `:` se aplicável.
- `**Estado**:` → `**Status**:`. Valor `\`EM VIGOR\``.
- `**Revoga**:` (se existe): manter intacto, posicionado
  conforme convenção (entre Status e Data — ver "Cabeçalho
  canónico" do README).
- Linha nova `**Validado**:` antes de `**Data**:`. Conteúdo:
  `Passo NN.E.` (data já está em `**Data**:`, não duplicar).
- `**Data**:` intacta.

**Caso especial**: ADR-0050 e ADR-0051 não têm campo
`**Revoga**:`. A convenção do README é "campos de relação
aparecem **apenas se** existir relação". Não-presença é
correcta — não adicionar campo vazio. A nota do relatório
143 §4.3 sobre "inconsistência de formato" refere-se à
falta de `**Revoga**: nenhuma.` explícito noutros ADRs do
grupo; isso é estilo, não erro. **Não tocar** estes campos.

Repetir para 0045–0051 com os passos correspondentes
(111.E/113.E/115.E/116.E/117.E/119.E/120.E).

### 145.4 — Cleanup do Grupo ³ (títulos sem `⚖️`)

ADRs 0038–0051 já são tocados em 145.2/145.3 (incluindo o
`⚖️`). Restam **possivelmente** 0017, 0027, 0028.

Para cada um confirmado em 145.1.A.2:

- Adicionar `⚖️ ` após `# ` no título.
- **Não tocar** mais nada. Status, datas, relações, corpo
  intactos.

ADR-0028 está `REVOGADO`. A correcção do título não
altera o estado de revogação. O `⚖️` aplica-se a todos os
ADRs independentemente de status.

Se 145.1.A.2 detectar ADRs adicionais fora do intervalo
canónico (ex: ADR-0001 também sem `⚖️`), **registar mas não
corrigir** — fora do escopo. Documentar no relatório como
"dívida remanescente para passo futuro".

### 145.5 — Limpeza do README

Em `00_nucleo/adr/README.md`:

**A.5.1** — Remover notas `¹` e `²` da coluna "Notas" da
tabela "Estado por ADR" para os ADRs corrigidos. Substituir
por nota neutra ou esvaziar a célula consoante o caso (a
nota tinha mérito de assinalar irregularidade; corrigida a
irregularidade, a nota perde sentido).

**A.5.2** — Remover do rodapé do README a explicação dos
símbolos `¹` e `²` (parágrafos que descreviam os Grupos ¹ e
² da §4.2 do relatório 143).

**A.5.3** — Adicionar entrada à secção "Passos-chave da
história dos ADRs":

```markdown
- **P145** — Uniformização de cabeçalhos para ADRs 0038–0051
  (análogo P84.8g). `**Estado**:` → `**Status**:`; backticks
  no valor; informação de validação preservada em
  `**Validado**:`. Símbolo ⚖️ adicionado a títulos dos ADRs
  0017, 0027, 0028 e 0038–0051 (subset do Grupo ³).
```

**A.5.4** — Recálculo de contagens **não é necessário**.
Status e relações dos ADRs ficam inalterados; só a forma
muda.

### 145.6 — Verificação automatizada

Comandos para confirmar correcção:

```bash
# Grupo ¹+² — todos têm Status canónico com backticks?
grep -L "^\*\*Status\*\*: \`EM VIGOR\`$" \
  00_nucleo/adr/typst-adr-{0038..0051}-*.md
# Saída esperada: vazia.

# Grupo ¹+² — nenhum usa Estado em vez de Status?
grep -l "^\*\*Estado\*\*:" \
  00_nucleo/adr/typst-adr-{0038..0051}-*.md
# Saída esperada: vazia.

# Grupo ³ — todos os ADRs do escopo têm ⚖️?
grep -L "^# ⚖️ ADR-" \
  00_nucleo/adr/typst-adr-{0017,0027,0028,0038..0051}-*.md
# Saída esperada: vazia.

# Conteúdo material intacto — comparar tamanhos de Decisão/etc.
# (heurística: se número de linhas mudou >5% num ADR, pausar)
```

### 145.7 — Relatório

Ficheiro:
`00_nucleo/materialization/typst-passo-145-relatorio.md`.

Secções:

1. Sumário executivo.
2. Inventário pré-correcção (resultado de 145.1).
3. Decisão sobre nome do campo de validação (resultado de
   145.1.A.3).
4. Diffs de cabeçalho por ADR — uma subsecção por ADR
   tocado, mostrando antes/depois das primeiras ~6 linhas.
5. ADRs do Grupo ³ corrigidos (lista exacta).
6. Limpeza do README (notas removidas, entrada
   "Passos-chave" adicionada).
7. Confirmação de que conteúdo material ficou intacto
   (heurística de tamanho + spot check em 2-3 ADRs).
8. Anomalias detectadas que ficam fora do escopo (dívida
   remanescente).
9. Verificação automatizada (saídas dos comandos de 145.6).

---

## Verificação

1. ✅ ADRs 0038–0043 com cabeçalho canónico (`**Status**:
   \`EM VIGOR\``).
2. ✅ ADRs 0044–0051 com cabeçalho canónico (`**Estado**:` →
   `**Status**:`; backticks; valor canónico).
3. ✅ Informação de validação preservada em `**Validado**:`.
4. ✅ Títulos do Grupo ³ com `⚖️` adicionado.
5. ✅ Conteúdo material (Decisão, Alternativas,
   Consequências, Referências) intacto em todos os ADRs.
6. ✅ Status e relações de ADRs inalterados.
7. ✅ README com notas `¹`/`²` removidas e entrada
   "Passos-chave" para P145 adicionada.
8. ✅ Comandos de 145.6 com saída vazia (todas as
   irregularidades resolvidas).
9. ✅ Nenhum ficheiro em L1/L2/L3/L4 tocado.
10. ✅ Nenhum ficheiro em `prompts/`, `relatorios/`,
    `context/` tocado.
11. ✅ `cargo test --workspace --lib`: inalterado.
12. ✅ `crystalline-lint .`: zero violations.
13. ✅ Relatório do passo escrito.

---

## Critério de conclusão

1. Cabeçalhos dos 14 ADRs do escopo conformes à convenção
   "Cabeçalho canónico" do README.
2. Títulos do Grupo ³ com `⚖️` adicionado.
3. Informação de validação preservada (não eliminada).
4. README sem notas `¹`/`²` redundantes.
5. Verificação automatizada de 145.6 com saída vazia.
6. Conteúdo material dos ADRs intacto.
7. Relatório do passo escrito.

---

## O que pode sair errado

- **ADR já em estado canónico no escopo**: se 145.1
  detectar que algum dos 14 ADRs já está conforme, **saltar**
  esse ADR. Registar no relatório.

- **Ficheiros com encoding não-UTF-8 no `⚖️`**: improvável
  (relatório 143 já leu os ficheiros e produziu tabela), mas
  se acontecer, gravar em UTF-8 explícito. Não introduzir
  outras alterações de encoding.

- **`**Revoga**: nenhuma.` vs ausência do campo**: convenção
  ambígua. Decisão: **preservar como está**. Se o ADR tem o
  campo explícito com "nenhuma", manter. Se não tem, não
  adicionar. Eliminação ou adição de campo é decisão fora
  deste passo.

- **Conteúdo do parêntese descritivo do Grupo ¹ é
  multilinha**: improvável (relatório 143 mostra-os em
  linha única), mas se ocorrer, condensar para uma única
  linha em `**Validado**:`.

- **Inventário detecta ADR fora do escopo com mesmo
  problema** (ex: ADR-0017 sem `⚖️` mas também com
  `**Estado**:` em vez de `**Status**:`): regista-se no
  relatório como dívida remanescente. **Não corrigir** —
  escopo é fixo.

- **Diff do cabeçalho introduz drift de hash em código
  L1/L2/L3/L4 que cite o ADR**: ADRs não têm hash em si; o
  `@prompt-hash` do linter aplica-se a prompts L0 em
  `00_nucleo/prompts/`, não a ADRs. Edição de cabeçalho de
  ADR **não** activa V5 (`PromptDrift`). Confirmar com
  `crystalline-lint` em 145.6.

- **Relatório 143 sub-contou Grupo ³**: relatório diz "≥14"
  e nomeia 17 ADRs específicos. Se 145.1.A.2 detectar mais
  do que isso (ex: 20+), o passo cresce. Pausa e
  redimensionamento se forem >25 ADRs.

- **Algum ADR tem cabeçalho com formato terceiro não-previsto**
  (ex: campo extra `**Hash**:` ou similar): preservar
  intacto. Convenção do README permite extensões — só
  uniformiza-se o que está explicitamente documentado como
  canónico.

---

## Notas operacionais

- **Modelo: P84.8g**. Aquele passo cobriu 0001–0037 e
  uniformizou `ACCEPTED`/`UPDATED` → vocabulário canónico.
  Este passo é o equivalente para 0038–0051. Se a frequência
  de criação de ADRs com cabeçalho não-canónico continuar,
  política de "cada novo ADR usa convenção canónica" devia
  ser meta-decisão (ADR ou nota no README) — fora do escopo
  deste passo.

- **Trabalho mecânico, baixo risco**. Edição de cabeçalho
  preserva conteúdo material por construção. Se algum ADR
  tiver erro material, este passo **não o corrige nem o
  expõe**.

- **Notas `¹`/`²` removidas**: a tabela do README perde a
  marca visual de irregularidade. Quem ler a tabela vai
  assumir conformidade — está garantida pela verificação
  145.6. Se for desejável manter rastreabilidade histórica
  ("estes ADRs tiveram cabeçalho irregular até P145"), a
  entrada nova em "Passos-chave" cobre esse registo.

- **Sem ADR de revisão de ADR**: P84.8g também não criou
  ADR. Uniformização de forma não é mudança de decisão.

- **`**Validado**:` como campo novo**: introduz um campo
  não documentado na secção "Cabeçalho canónico" do README.
  Decisão: aceitar como **extensão de facto**, sem alterar
  o README na parte de convenções. Se quisermos formalizar
  o campo, é decisão futura (passo dedicado ou nota no
  README). A informação preservada é histórica e não-crítica
  para uso operacional dos ADRs.

- **Numeração 145**: 144 reservado para lang hyphenation;
  145 é o seguinte. Se mais passos administrativos forem
  intercalados antes de hyphenation, a numeração progride
  naturalmente — o reservar de 144 é hint, não bloqueio.

- **Após este passo**: índice do README + cabeçalhos dos
  ADRs estão coerentes. A próxima dívida documental
  conhecida é o registo de meta-decisão sobre criação
  futura de ADRs (acima). Não bloqueia trabalho substantivo.
