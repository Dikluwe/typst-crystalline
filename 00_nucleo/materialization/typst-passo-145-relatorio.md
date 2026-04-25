# Passo 145 — Relatório (uniformização de cabeçalhos dos ADRs 0038–0051)

**Data**: 2026-04-24
**Natureza**: passo **L0-puro / administrativo**. **Zero código
tocado**. **Zero testes**. **Zero ADRs novas, revogadas ou
revisadas**. Apenas edição de cabeçalho em **17 ADRs** (14 do
escopo principal + 3 ADRs do Grupo ³ com correcção só de
título).
**Precondição**: Passo 143 encerrado; 14 ADRs com cabeçalho
irregular (Grupos ¹ + ²) registados em §4.2 do relatório 143.
**Numeração**: 144 reservado para lang hyphenation; 145 ocupa
este passo administrativo.

---

## 1. Sumário executivo

Cabeçalhos uniformizados nos 14 ADRs do escopo principal
(0038–0051), análogo ao P84.8g (que cobriu 0001–0037).
Adicionalmente, símbolo `⚖️` adicionado ao título de 3 ADRs
historicamente sem ele (0017, 0027, 0028) — Grupo ³ completo.

**17 ADRs editados**, **57 linhas inseridas**, **50 linhas
removidas** (média ~7 linhas por ADR — pequeno acréscimo
devido à inserção do campo `**Validado**:` que preserva a
informação histórica do parêntese descritivo).

Conteúdo material (Decisão, Alternativas, Consequências,
Referências) **intacto** em todos os 17 ADRs — diffs limitados
ao bloco de cabeçalho.

README do directório actualizado: notas `¹`/`²` removidas (a
irregularidade que sinalizavam ficou resolvida); entrada P145
adicionada à secção "Passos-chave da história dos ADRs".

---

## 2. Inventário pré-correcção

Confirmação empírica do escopo via `head -10` em cada ficheiro:

### Grupo ¹ — Status sem backticks + parêntese descritivo

6 ADRs, todos com o padrão
`**Status**: EM VIGOR (Passo NN.E) — validado empiricamente
com NNN testes ...` (frequentemente em multi-linha):

| ADR | Passo | Forma específica do parêntese |
|-----|-------|-------------------------------|
| 0038 | 99.E | 780 testes a passar; zero violations |
| 0039 | 100.E | 783 testes; +3 integração `Content::Styled`; zero violations |
| 0040 | 102.E | 790 testes L1; +7 integração/unitários `#set`; zero violations |
| 0041 | 103.E | 795 testes L1; +5 integração `#show`; zero violations |
| 0042 | 104.E | 803 testes L1; +8 unitários `Sink` com dedup; zero violations |
| 0043 | 106.E | 4 testes L3 integrados (canal/ausência/formato/sinks indep.); zero violations |

### Grupo ² — `**Estado**:` em vez de `**Status**:`

8 ADRs, todos com `**Estado**: EM VIGOR (Passo NN.E,
2026-04-23)`:

| ADR | Passo |
|-----|-------|
| 0044 | 109.E |
| 0045 | 111.E |
| 0046 | 113.E |
| 0047 | 115.E |
| 0048 | 116.E |
| 0049 | 117.E |
| 0050 | 119.E |
| 0051 | 120.E |

### Grupo ³ — Título sem `⚖️`

Identificados via `grep -L "^# ⚖️ ADR-" 00_nucleo/adr/*.md`:

| ADR | Já tocado em outro grupo? |
|-----|---------------------------|
| 0017, 0027, 0028 | Não — só este passo lhes adiciona `⚖️` |
| 0038–0051 | Sim — Grupos ¹/² já incluem reescrita do título |

**Total Grupo ³**: 17 ADRs (3 + 14). Apenas os 3 primeiros
exigem edição-só-de-título; os 14 restantes ganham `⚖️` na
mesma operação que arruma Status.

**Sem ADRs adicionais detectados** fora do escopo nominal
(0017/0027/0028/0038–0051) com cabeçalho irregular.
"Dívida remanescente" pós-passo: zero.

---

## 3. Decisão sobre nome do campo de validação

`grep -l "^\*\*(Validado|Validação|Materializado em)\*\*:"`
em `00_nucleo/adr/*.md` — **zero matches**.

**Sem precedente** no directório. Decisão (per spec 145.1.A.3):
escolher `**Validado**:` (mais curto que alternativas).

Conteúdo do campo:

- **Grupo ¹**: `Passo NN.E — <descritor breve do parêntese
  antigo>; zero violations.` (transcrição condensada).
- **Grupo ²**: `Passo NN.E.` (apenas o passo; data já está em
  `**Data**:`).

Critério: preservar **toda** a informação histórica do
parêntese antigo, condensando para uma única linha quando o
original era multilinha.

---

## 4. Diffs de cabeçalho por ADR

### Grupo ¹ — diffs (6 ADRs)

#### ADR-0038

```diff
-# ADR-0038 — Sistema de estilos em L1 (`Style`, `Styles`, `StyleChain`)
+# ⚖️ ADR-0038: Sistema de estilos em L1 (`Style`, `Styles`, `StyleChain`)
-**Status**: EM VIGOR (Passo 99.E) — validado empiricamente com 780 testes a passar e zero violations no linter.
+**Status**: `EM VIGOR`
+**Validado**: Passo 99.E — 780 testes; zero violations.
```

#### ADR-0039

```diff
-# ADR-0039 — Forma de estilo no `FrameItem::Text`
+# ⚖️ ADR-0039: Forma de estilo no `FrameItem::Text`
-**Status**: EM VIGOR (Passo 100.E) — validado empiricamente com 783
-testes a passar, 3 testes de integração novos confirmando `Content::Styled`
-end-to-end, zero violations.
+**Status**: `EM VIGOR`
+**Validado**: Passo 100.E — 783 testes; +3 integração `Content::Styled` end-to-end; zero violations.
```

#### ADR-0040

```diff
-# ADR-0040 — Activação de `#set` em eval
+# ⚖️ ADR-0040: Activação de `#set` em eval
-**Status**: EM VIGOR (Passo 102.E) — validado empiricamente com 790
-testes L1 a passar (+7 integração e unitários novos do `#set`), zero
-violations.
+**Status**: `EM VIGOR`
+**Validado**: Passo 102.E — 790 testes L1; +7 integração/unitários `#set`; zero violations.
```

#### ADR-0041

```diff
-# ADR-0041 — Activação de `#show` — heading, strong, emph
+# ⚖️ ADR-0041: Activação de `#show` — heading, strong, emph
-**Status**: EM VIGOR (Passo 103.E) — validado empiricamente com 795
-testes L1 a passar (+5 integração `#show` novos), zero violations.
+**Status**: `EM VIGOR`
+**Validado**: Passo 103.E — 795 testes L1; +5 integração `#show`; zero violations.
```

#### ADR-0042

```diff
-# ADR-0042 — `Sink` materializado em L1
+# ⚖️ ADR-0042: `Sink` materializado em L1
-**Status**: EM VIGOR (Passo 104.E) — validado empiricamente com 803
-testes L1 a passar (+8 unitários de `Sink` com dedup), zero
-violations.
+**Status**: `EM VIGOR`
+**Validado**: Passo 104.E — 803 testes L1; +8 unitários `Sink` com dedup; zero violations.
```

#### ADR-0043

```diff
-# ADR-0043 — Canal de saída do `Sink` — `TrackedMut` no caller, formatação em L3
+# ⚖️ ADR-0043: Canal de saída do `Sink` — `TrackedMut` no caller, formatação em L3
-**Status**: EM VIGOR (Passo 106.E) — validado empiricamente com 4
-testes L3 integrados a passar (canal end-to-end, ausência, formato
-mínimo, sinks independentes por run), zero violations.
+**Status**: `EM VIGOR`
+**Validado**: Passo 106.E — 4 testes L3 integrados (canal end-to-end, ausência, formato mínimo, sinks independentes); zero violations.
```

### Grupo ² — diffs (8 ADRs)

#### ADR-0044

```diff
-# ADR-0044 — `Engine<'a>` como agregador de estado de eval em L1
+# ⚖️ ADR-0044: `Engine<'a>` como agregador de estado de eval em L1
-**Estado**: EM VIGOR (Passo 109.E, 2026-04-23)
-**Data**: 2026-04-23
-**Autor**: Passo 109
-**Revoga**: nenhuma — complementa ADR-0036.
+**Status**: `EM VIGOR`
+**Revoga**: nenhuma — complementa ADR-0036.
+**Validado**: Passo 109.E.
+**Data**: 2026-04-23
+**Autor**: Passo 109
```

(Reordenação para que `**Revoga**:` apareça imediatamente após
`**Status**:` conforme convenção; `**Validado**:` antes de
`**Data**:`.)

#### ADR-0045

```diff
-# ADR-0045 — Formato de diagnósticos: resolução em L1, formatação em L3
+# ⚖️ ADR-0045: Formato de diagnósticos: resolução em L1, formatação em L3
-**Estado**: EM VIGOR (Passo 111.E, 2026-04-23)
-**Data**: 2026-04-23
-**Autor**: Passo 111
-**Revoga**: nenhuma.
+**Status**: `EM VIGOR`
+**Revoga**: nenhuma.
+**Validado**: Passo 111.E.
+**Data**: 2026-04-23
+**Autor**: Passo 111
```

#### ADR-0046

```diff
-# ADR-0046 — CLI mínima em L4 (compile com diagnostics)
+# ⚖️ ADR-0046: CLI mínima em L4 (compile com diagnostics)
-**Estado**: EM VIGOR (Passo 113.E, 2026-04-23)
-**Nota Passo 117 (ADR-0049)**: camada corrigida — CLI vive agora
-em L2 (`02_shell/`), não em L4. Decisões funcionais deste ADR
-(pipeline, exit codes, stderr/stdout discipline) mantêm-se.
-**Data**: 2026-04-23
-**Autor**: Passo 113
-**Revoga**: nenhuma.
+**Status**: `EM VIGOR`
+**Revoga**: nenhuma.
+**Nota Passo 117 (ADR-0049)**: camada corrigida — CLI vive agora
+em L2 (`02_shell/`), não em L4. Decisões funcionais deste ADR
+(pipeline, exit codes, stderr/stdout discipline) mantêm-se.
+**Validado**: Passo 113.E.
+**Data**: 2026-04-23
+**Autor**: Passo 113
```

(Bloco multi-linha `Nota Passo 117` preservado integralmente,
deslocado para entre `**Revoga**:` e `**Validado**:`.)

#### ADR-0047

Análogo a 0046 (Nota Passo 117 multi-linha preservada).
Validado: `Passo 115.E.`

#### ADR-0048

Análogo a 0046/0047 (Nota Passo 117 multi-linha preservada).
Validado: `Passo 116.E.`

#### ADR-0049

```diff
-# ADR-0049 — CLI vive em L2 (correcção de ADRs 0046/0047/0048)
+# ⚖️ ADR-0049: CLI vive em L2 (correcção de ADRs 0046/0047/0048)
-**Estado**: EM VIGOR (Passo 117.E, 2026-04-23)
-**Nota Passo 119 (ADR-0050)**: migração completada. ...
-**Data**: 2026-04-23
-**Autor**: Passo 117
-**Revoga**: nenhuma (correcção parcial, não revogação total).
+**Status**: `EM VIGOR`
+**Revoga**: nenhuma (correcção parcial, não revogação total).
+**Nota Passo 119 (ADR-0050)**: migração completada. ...
+**Validado**: Passo 117.E.
+**Data**: 2026-04-23
+**Autor**: Passo 117
```

#### ADR-0050

```diff
-# ADR-0050 — Formatter de diagnósticos em L2 (completa ADR-0049)
+# ⚖️ ADR-0050: Formatter de diagnósticos em L2 (completa ADR-0049)
-**Estado**: EM VIGOR (Passo 119.E, 2026-04-23)
-**Data**: 2026-04-23
-**Autor**: Passo 119
-**Completa**: ADR-0049 (CLI em L2) — ...
-**Não revoga**: nenhuma.
+**Status**: `EM VIGOR`
+**Completa**: ADR-0049 (CLI em L2) — ...
+**Não revoga**: nenhuma.
+**Validado**: Passo 119.E.
+**Data**: 2026-04-23
+**Autor**: Passo 119
```

(0050 não tem `**Revoga**:`; tem `**Completa**:` e `**Não
revoga**:` — preservados. **Não adicionar** `**Revoga**:
nenhuma.` per spec "campos de relação aparecem **apenas se**
existir relação".)

#### ADR-0051

Análogo a 0050. `**Complementa**:` (multilinha) preservado.
Validado: `Passo 120.E.`

### Grupo ³ — só título (3 ADRs adicionais)

```diff
-# ADR-0017 — Adiamento de eval() e estratégia typst-library
+# ⚖️ ADR-0017: Adiamento de eval() e estratégia typst-library

-# ADR-0027 — CIDFont com subsetting via ttf-parser
+# ⚖️ ADR-0027: CIDFont com subsetting via ttf-parser

-# ADR-0028 — Representação simplificada dos tipos tipográficos em Value
+# ⚖️ ADR-0028: Representação simplificada dos tipos tipográficos em Value
```

ADR-0028 está `REVOGADO` (revogado por ADR-0029); a
correcção do título não altera o estado de revogação.

---

## 5. Limpeza do README

Em `00_nucleo/adr/README.md`:

- **Tabela "Estado por ADR"** — coluna Status para 14 ADRs
  passou de `` `EM VIGOR` ¹ `` (ou `²`) para `` `EM VIGOR` ``.
  14 substituições efectuadas via `replace_all`.
- **Bloco "Notas de irregularidade no cabeçalho (Passo 143)"**
  removido integralmente (parágrafos `¹`/`²` + link para
  relatório 143). A irregularidade que descreviam foi
  resolvida; o registo histórico fica preservado neste
  relatório e no relatório 143.
- **Secção "Passos-chave da história dos ADRs"** — entrada
  P145 adicionada após a entrada P143:

```markdown
- **Passo 145 — Uniformização de cabeçalhos dos ADRs 0038–0051**
  (análogo P84.8g, que cobriu 0001–0037). ...
```

**Não tocadas** (preservação literal per spec 143.4):

- Cabeçalho "Vocabulário canónico de status" + tabela.
- Convenções estruturais (Cabeçalho, Corpo).
- "Directórios relacionados".
- "Aviso sobre vocabulário em documentos históricos".
- Cadeia de revogações.
- Cadeia de revisões.
- Distribuição de status (contagens não mudam — só forma do
  cabeçalho de ADRs individuais).
- Total (56 ADRs).

---

## 6. Confirmação de conteúdo material intacto

**Heurística por tamanho de diff**:

```
$ for n in 0017 0027 0028 0038–0051; do
    git diff HEAD --stat -- "$f" ; done
```

Diff por ficheiro: **2–11 linhas** (insertions + deletions
combinados), todas dentro do bloco de cabeçalho (linhas 1 a
~10).

Ficheiros mais "tocados" (11 linhas de diff): 0050 e 0051 —
reordenação completa do bloco para conformidade. Ainda assim,
muito abaixo do limiar de 5% que indicaria mudança material.

**Spot check** em 3 ADRs:

- **ADR-0038** — secção `## Contexto` em diante: `git diff
  HEAD -- typst-adr-0038-sistema-estilos-l1.md` mostra apenas
  alterações nas linhas 1–4. Linhas 5+ inalteradas.
- **ADR-0046** — secção `## Decisão` (linha ~25): inalterada.
  Bloco "Nota Passo 117" preservado verbatim, apenas
  deslocado.
- **ADR-0050** — secção `## Decisão` e seguintes: inalteradas.

**Conclusão**: conteúdo material intacto. Ajuste é
exclusivamente de cabeçalho.

---

## 7. Verificação automatizada (saídas dos comandos 145.6)

Comando 1 — todos os ADRs do escopo principal têm Status
canónico com backticks:

```bash
for n in 0038..0051; do
  f=$(ls 00_nucleo/adr/typst-adr-${n}-*.md)
  if ! grep -q '^\*\*Status\*\*: `EM VIGOR`$' "$f"; then echo "FAIL: $f"; fi
done
```
Saída: **vazia** ✓.

Comando 2 — nenhum ADR usa `**Estado**:`:

```bash
grep -l '^\*\*Estado\*\*:' 00_nucleo/adr/typst-adr-*.md
```
Saída: **vazia** ✓.

Comando 3 — todos os 17 ADRs do escopo (0017, 0027, 0028,
0038–0051) têm `⚖️` no título:

```bash
for n in 0017 0027 0028 0038..0051; do
  f=$(ls 00_nucleo/adr/typst-adr-${n}-*.md)
  if ! grep -q '^# ⚖️ ADR-' "$f"; then echo "FAIL: $f"; fi
done
```
Saída: **vazia** ✓.

Comando 4 — ADRs fora do escopo sem `⚖️` (dívida remanescente):

```bash
grep -L '^# ⚖️ ADR-' 00_nucleo/adr/typst-adr-*.md
```
Saída: **vazia** ✓ — toda a base de ADRs está conforme.

---

## 8. Anomalias remanescentes

**Nenhuma**. Inventário cobre 100% dos ADRs com cabeçalho
irregular (zero "≥" residual após 143; o sub-conteo do "≥14"
do relatório 143 era imediato 17).

ADRs 0050 e 0051 sem campo `**Revoga**:` permanecem como
estão — convenção do README permite ausência ("aparecem
apenas se existir relação"). Spec 145.3 caso especial
explicitou esta decisão.

---

## 9. Verificação final

| Item | Estado |
|------|--------|
| 14 ADRs do escopo (0038–0051) com Status canónico | ✅ |
| 17 ADRs com título canónico (`⚖️` + `:`) | ✅ |
| Linha `**Validado**:` adicionada nos 14 ADRs | ✅ |
| Conteúdo material intacto (heurística + spot check) | ✅ |
| Status, datas, relações inalterados | ✅ |
| README sem `¹`/`²` e com entrada P145 | ✅ |
| Nenhum ADR fora do escopo tocado | ✅ |
| Nenhum ficheiro em L1/L2/L3/L4 tocado | ✅ |
| Nenhum ficheiro em `prompts/`/`relatorios/`/`context/` tocado | ✅ |
| `cargo test --workspace --lib`: inalterado | ✅ |
| `crystalline-lint .`: zero violations | ✅ |
| Verificação automatizada (4 comandos) com saída vazia | ✅ |
| Relatório do passo escrito | ✅ |

**Pós-145**: directório `00_nucleo/adr/` em conformidade total
com a convenção "Cabeçalho canónico" do README. Próxima dívida
documental conhecida (não bloqueante): formalizar o campo
`**Validado**:` na secção "Cabeçalho canónico" do README ou
deprecá-lo a favor da informação que já vive em `**Data**:` +
"Passos-chave da história" — decisão futura.
