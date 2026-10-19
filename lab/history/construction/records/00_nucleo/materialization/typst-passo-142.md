# Passo 142 — Fecho formal de DEBT-1 (cumprimento de ADR-0054)

**Série**: 142 (passo **XS** L0-puro; fecho administrativo pós
Fase C básica). Número escolhido por continuidade directa com
140B/141 — multi-font per document, anteriormente pensada como
"142 opcional", passa a **142A** se/quando priorizada, para
preservar o uso tradicional do número nesta série de fecho.
**Precondição**: Passo 141 encerrado; 1116 total tests; zero
violations; **55 ADRs** com ADR-0055 em `IMPLEMENTADO`;
DEBT-52 em 6/8 gaps resolvidos (gaps 7 e 8 opcionais segundo
ADR-0054); hash L0 `prompts/infra/pipeline.md` = `00e4ebd3`.

**ADRs aplicáveis**:
- **ADR-0054** (critério fecho DEBT-1) — **cumprimento é o
  objectivo do passo**. Relatório final confirma que cada
  critério da ADR é satisfeito.
- **ADR-0055** (`IMPLEMENTADO` após 141) — referência de
  materialização.
- **ADR-0033** (paridade funcional) — invocada para
  "observacional equivalente" no critério 2 de ADR-0054.

**Natureza**: passo **L0-puro**. **Zero código**. **Zero
testes**. **Zero ADRs novas**. Único output: relatório
formal + actualização de `DEBT.md` + actualização de
`README.md` se tiver índice de DEBTs.

**Tamanho estimado**: **XS**, ~30min.

---

## Contexto

DEBT-1 está aberto desde o início do projecto. Passou por
múltiplas actualizações parciais (Passos 30, 33, 83.5, 84.1,
94, 95, 99, 100, 101, 102, 136–139, 140B, 141). A linha de
pendências foi progressivamente drenada:

- **Scoping de `#set` por bloco** — resolvido implicitamente
  (Passos 33 + 94).
- **Arquitectura partilhada de `styles`** — resolvida (Passo
  94, ADR-0036).
- **Wrappers `Content::Strong/Emph` no layout** — removidos
  (Passo 101).
- **`#set text(...)` validado + `fill` activado** — Passo 102
  + ADR-0040.
- **Consumer `tracking`/`leading`/`weight` faux-bold** —
  Passos 137/138/139 (Fase B de DEBT-52).
- **Consumer `font` string** — Passo 140B (gap 5).
- **Consumer `font` array fallback** — Passo 141 (gap 6).

ADR-0054 (criada no Passo 135) formalizou o critério de fecho:

1. **Cada campo inerte do `StyleDelta` tem consumer activo
   OU é explicitamente scope-out com ADR de suporte**.
2. **Output PDF observacionalmente equivalente ao vanilla
   para inputs de teste documentados**.
3. **DEBT-1 pode fechar**.

Após Passos 140B + 141, as condições 1 e 2 são verificáveis.
Este passo **certifica o cumprimento** e fecha DEBT-1
formalmente.

---

## Objectivo

Ao fim do passo:

1. **Relatório formal de fecho** em
   `00_nucleo/relatorios/fecho-debt-1-passo-142.md` com:
   - Tabela campo-a-campo do `StyleDelta` mapeando cada
     campo ao consumer activo **ou** ao scope-out com ADR de
     suporte.
   - Evidência de "observacional equivalente" (inputs de
     teste + sítios no código que produzem output comparável
     a vanilla).
   - Confirmação explícita de cumprimento de cada critério de
     ADR-0054.

2. **DEBT-1 movido para Secção 2** ("DEBTs encerrados") em
   `00_nucleo/DEBT.md`. Texto histórico preservado; adicionada
   nota de fecho com referência a este passo e ao relatório.

3. **DEBT-52 movido para Secção 2** com nota "rastreador
   cumprido; gaps 7 e 8 opcionais conforme ADR-0054". Os
   gaps 7 (lang hyphenation) e 8 (font dict) **não
   reabrem** DEBTs novos neste passo — ficam documentados
   no relatório como "potenciais passos futuros 143/144
   quando priorizados".

4. **Contagem de DEBTs abertos**: 13 → **11** (DEBT-1 e
   DEBT-52 encerrados simultaneamente).

5. **README.md** de `00_nucleo/` ou equivalente actualizado
   se tiver referência explícita a DEBT-1/DEBT-52 como
   abertos. (Verificar em 142.1.)

Este passo **não**:

- Toca código em L1, L2, L3, L4.
- Toca testes.
- Cria ADRs novas.
- Materializa gap 7 (hyphenation) ou gap 8 (font dict).
- Abre DEBT novo para trabalho opcional remanescente.
- Move outros DEBTs para encerrados.

---

## Decisões já tomadas

1. **Passo curto dedicado** (em vez de entrada directa em
   DEBT.md). Razão: cumprimento de ADR-0054 justifica
   relatório formal, análogo ao Passo 84.5 (fecho DEBT-36) e
   Passo 92 (fecho DEBT-44/40).
2. **Gap 7 e gap 8 permanecem abertos como "potenciais passos
   futuros"** no texto do relatório. **Não** se tornam DEBTs
   separados. ADR-0054 já os declara opcionais — a abertura
   de DEBTs para trabalho opcional polui o inventário.
3. **Número 142** usado para este passo de fecho. Se
   multi-font per document for priorizada no futuro,
   numeração é **142A** (sub-série, paralela ao padrão 140A).
4. **Sem ADR de revisão de ADR-0054**. A ADR continua em
   vigor — apenas constata-se que foi cumprida.

## Decisões diferidas (resolvidas neste passo)

5. **Nome exacto do relatório**: `fecho-debt-1-passo-142.md`
   proposto, alinhado com convenção de `00_nucleo/relatorios/`
   (ex: `relatorio-auditoria-adrs-passo-84.7.md`). Confirmar
   em 142.1 se há convenção diferente já estabelecida para
   relatórios de fecho.
6. **Mapeamento exacto campo-a-campo**: obtido em 142.2 via
   leitura directa da definição actual de `StyleDelta`. Campos
   esperados (pós-139 + ADR-0053): `bold`, `italic`, `size`,
   `fill`, `heading_level`, `tracking`, `leading`, `weight`,
   `lang`, `font`. Confirmar estrutura em 142.2.

---

## Escopo

**Dentro**:

- Leitura de `01_core/src/entities/style_delta.rs` (ou
  equivalente) para obter lista exaustiva de campos.
- Leitura de `00_nucleo/adr/typst-adr-0054-*.md` para os
  critérios exactos.
- Leitura de `00_nucleo/DEBT.md` (secções DEBT-1 e DEBT-52)
  para confirmar histórico.
- Escrita de relatório de fecho.
- Edição de `DEBT.md` (mover DEBT-1 e DEBT-52 para Secção 2).
- Edição de `00_nucleo/README.md` ou outros índices **se**
  referenciarem DEBT-1/DEBT-52 como abertos.

**Fora**:

- Qualquer ficheiro em `01_core/`, `02_shell/`, `03_infra/`,
  `04_wiring/`.
- Testes (pré-existentes e novos).
- ADRs (criação, revisão, revogação).
- Prompts L0 em `00_nucleo/prompts/`.
- Implementação de gap 7 ou gap 8.
- Abertura de novos DEBTs.

---

## Sub-passos

### 142.1 — Inventário pré-fecho

**A.1 — Confirmar critérios actuais de ADR-0054**:

`view 00_nucleo/adr/typst-adr-0054-*.md`. Extrair lista
numerada de critérios. Transcrever literalmente para o
relatório.

**A.2 — Confirmar campos actuais de `StyleDelta`**:

`grep -rn "struct StyleDelta\|pub bold\|pub italic\|pub size\|pub fill\|pub heading_level\|pub tracking\|pub leading\|pub weight\|pub lang\|pub font" 01_core/src/entities/`.

Registar cada campo + tipo + passo de materialização.

**A.3 — Confirmar estado actual de DEBT-1 e DEBT-52**:

`view 00_nucleo/DEBT.md` — localizar Secção 1 (abertos),
procurar cabeçalhos DEBT-1 e DEBT-52. Registar:
- Último estado documentado de DEBT-1.
- Lista de gaps fechados/abertos em DEBT-52 (esperado 6/8).

**A.4 — Procurar referências cruzadas**:

`grep -rn "DEBT-1\b\|DEBT-52\b" 00_nucleo/ --include="*.md"`.

Identificar se `00_nucleo/README.md`, outros relatórios, ou
ADRs referenciam DEBT-1/DEBT-52 de forma que o fecho torne
desactualizadas. Referências em `materialization/`,
`relatorios/`, `context/` **não são** actualizadas (imutáveis
— ver README.md secção "Aviso sobre vocabulário em documentos
históricos").

### 142.2 — Escrever relatório de fecho

Ficheiro: `00_nucleo/relatorios/fecho-debt-1-passo-142.md`.

Estrutura proposta:

```markdown
# Fecho formal de DEBT-1 — Passo 142

**Data**: 2026-04-24
**ADR de referência**: ADR-0054 (critério fecho DEBT-1).
**Natureza**: relatório formal. DEBT-1 encerrado após
cumprimento dos critérios de ADR-0054. DEBT-52 (rastreador)
encerrado simultaneamente.

---

## 1. Histórico comprimido

[Lista dos passos-chave: 30, 33, 83.5, 84.1, 94, 95, 99,
100, 101, 102, 136-139, 140A, 140B, 141. 1-2 linhas por passo.]

## 2. Critérios de ADR-0054

[Transcrever literalmente os critérios extraídos em 142.1.A.1.]

## 3. Mapeamento campo-a-campo de `StyleDelta`

| Campo | Tipo | Consumer | Passo | Estado |
|-------|------|----------|-------|--------|
| bold | bool | eval → TextStyle | 30 | activo |
| italic | bool | eval → TextStyle | 30 | activo |
| size | Length | eval → TextStyle | 30 | activo |
| fill | Paint | layout + export | 102 | activo |
| heading_level | u8 | show rules | 99 | activo |
| tracking | Length | word_width + Tc operator | 137 | activo |
| leading | Length | flush_line | 138 | activo |
| weight | FontWeight | faux-bold | 139 | activo (aprox) |
| lang | Lang | (hyphenation opcional) | — | scope-out (ADR-0054 §N) |
| font | FontList | first_font_from_doc + resolve_font | 140B + 141 | activo |

[Cada linha justificada em subsecção se não trivial.]

## 4. Cumprimento do critério 1

[Cada campo inerte tem consumer OU scope-out documentado em
ADR-0054. `lang` é o único em scope-out → referência directa
à secção da ADR-0054 que o declara opcional.]

## 5. Cumprimento do critério 2

[Inputs de teste documentados:
- `#set text(bold: true); Hello` → PDF com font Bold/faux-bold.
- `#set text(font: "Inria Serif"); Hello` → PDF com CIDFont.
- `#set text(font: ("FontQueNaoExiste", "Inria Serif")); Hello`
  → PDF com segunda família embutida.
- etc.

Sítios no código que produzem output: pipeline →
first_font_from_doc → resolve_font → export_pdf_with_font.

Paridade observacional: ADR-0033 invocada; divergências
residuais (weight faux-bold em vez de font Bold dedicada;
subsetting ausente) documentadas como "out-of-scope DEBT-1"
e cobertas por ADR-0055 + limitações registadas.]

## 6. Cumprimento do critério 3

[DEBT-1 pode fechar: critérios 1 e 2 cumpridos; este passo
executa o fecho.]

## 7. DEBT-52 encerramento simultâneo

[DEBT-52 é rastreador de consumers do `StyleDelta`. 6/8 gaps
fechados; gaps 7 e 8 opcionais segundo ADR-0054. Rastreador
cumpre a sua função — encerra junto com DEBT-1.]

## 8. Gaps 7 e 8 como potenciais passos futuros

[
- **Gap 7** (lang hyphenation): Passo 143 candidato. Requer
  crate autorizada (ex: `hyphenation`) via ADR separada.
- **Gap 8** (font dict): ADR-0054bis condicional; requer
  `regex` em L1. Baixa prioridade.

Nenhum DEBT novo aberto. Trabalho opcional.]

## 9. Limitações conhecidas preservadas

[Lista canonizada das limitações registadas em 140B/141:
variant-aware (ADR-0055bis candidata), multi-font per
document (Passo 142A candidato), subsetting (ADR-0056
candidata), shaping rustybuzz (DEBT-53 candidato), fixture
de fonts em CI (passo dedicado futuro).]

## 10. Verificação

- [x] Critérios de ADR-0054 transcritos.
- [x] Mapeamento campo-a-campo completo.
- [x] Sem campo inerte não documentado.
- [x] Cada scope-out tem referência ADR.
- [x] Tests 1116 inalterados (zero código tocado).
- [x] `crystalline-lint .` zero violations.
- [x] DEBT-1 movido a Secção 2.
- [x] DEBT-52 movido a Secção 2.
```

### 142.3 — Actualizar `DEBT.md`

**A.3.1 — Mover DEBT-1**:

Cortar secção inteira de DEBT-1 da Secção 1. Colar em Secção 2
("DEBTs encerrados"). Adicionar no topo do bloco:

```markdown
## DEBT-1 — StyleChain — ENCERRADO (Passo 142) ✓

**Fechado em**: 2026-04-24.
**Relatório formal**:
[`relatorios/fecho-debt-1-passo-142.md`](relatorios/fecho-debt-1-passo-142.md).
**Cumprimento**: ADR-0054 (critérios 1, 2, 3 satisfeitos).

Histórico preservado abaixo na forma em que vivia antes do
fecho. Actualizações de 30, 33, 83.5, 84.1, 94, 95, 99,
100, 101, 102 continuam como documento; acrescentam-se as
entradas de Fase B (137/138/139) e Fase C (140B/141).
```

Preservar todas as "Actualizações Passo N" existentes. Adicionar
duas novas:

```markdown
### Actualização Passo 140B — Consumer `font` string

- [x] `TextStyle.font` capturado por eval flui até ao
  exporter. `first_font_from_doc` procura primeira `FontList`;
  `resolve_font` consulta `FontBook::select` +
  `World::font`; `compile_to_pdf_bytes` despacha para
  `export_pdf_with_font` quando resolve. Gap 5 DEBT-52
  fechado.

### Actualização Passo 141 — Consumer `font` array fallback

- [x] `resolve_font` itera todas as famílias de `FontList`;
  primeira que resolve vence. Gap 6 DEBT-52 fechado.
  ADR-0055 transita `IMPLEMENTADO`.

### Fecho — Passo 142

- [x] Critérios de ADR-0054 cumpridos. Relatório formal
  produzido. DEBT-52 encerrado simultaneamente. Limitações
  conhecidas preservadas como candidatos futuros (não DEBTs).
```

**A.3.2 — Mover DEBT-52**:

Cortar secção inteira de DEBT-52 da Secção 1. Colar em Secção 2
(imediatamente após DEBT-1 — ordem cronológica dentro de
"encerrados" não é regra dura, mas agrupa pares). Adicionar
cabeçalho:

```markdown
## DEBT-52 — Consumer integral de `StyleDelta` — ENCERRADO (Passo 142) ✓

**Fechado em**: 2026-04-24 (simultaneamente com DEBT-1).
**Relatório formal**:
[`relatorios/fecho-debt-1-passo-142.md`](relatorios/fecho-debt-1-passo-142.md).
**Gaps fechados**: 6/8 (gaps 7 e 8 são opcionais segundo
ADR-0054 e permanecem como candidatos futuros, sem reabertura
como DEBTs novos).

Rastreador cumpriu a sua função: guiou as Fases A (Passo 136),
B (Passos 137-139) e C básica (Passos 140B + 141). Histórico
preservado abaixo.
```

**A.3.3 — Actualizar cabeçalho de Secção 1**:

Se a Secção 1 tiver contagem de DEBTs abertos no topo,
actualizar: 13 → **11**.

### 142.4 — Actualizar referências cruzadas

Para cada ficheiro identificado em 142.1.A.4 (excluindo
`materialization/`, `relatorios/` anteriores a 142, e
`context/`):

- Se refere DEBT-1 como aberto: substituir por referência ao
  relatório de fecho.
- Se refere DEBT-52 como aberto: mesmo tratamento.
- Se refere gap 5 ou gap 6: pode permanecer (descreve
  trabalho histórico correctamente).

Esperado: poucos matches fora de `DEBT.md`. Se forem muitos,
pausar e reconsiderar escopo.

### 142.5 — Verificação final

- `cargo test --workspace --lib` — confirmar **1116**
  (inalterado).
- `crystalline-lint .` — zero violations.
- `git diff --stat` — apenas `.md` files tocados.
- Nenhum ficheiro em `01_core/`, `02_shell/`, `03_infra/`,
  `04_wiring/` com diff.
- `grep -rn "DEBT-1\b" 00_nucleo/adr/` — ADRs que
  referenciam DEBT-1 ainda são consistentes (DEBT-1 existe
  no histórico; referência como conceito continua válida).

---

## Verificação

1. ✅ Relatório `fecho-debt-1-passo-142.md` criado em
   `00_nucleo/relatorios/`.
2. ✅ Mapeamento campo-a-campo completo de `StyleDelta`.
3. ✅ Cada critério de ADR-0054 explicitamente confirmado.
4. ✅ DEBT-1 movido para Secção 2 de `DEBT.md` com nota de
   fecho + referência a relatório.
5. ✅ DEBT-52 movido para Secção 2 com nota análoga.
6. ✅ Contagem de DEBTs abertos: 13 → 11.
7. ✅ Gaps 7 e 8 documentados no relatório como candidatos
   futuros; nenhum DEBT novo aberto.
8. ✅ Limitações conhecidas preservadas (variant-aware,
   multi-font, etc.) sem se tornarem DEBTs.
9. ✅ `cargo test --workspace --lib`: 1116 (inalterado).
10. ✅ `crystalline-lint .`: zero violations.
11. ✅ Nenhum ficheiro em L1/L2/L3/L4 tocado.

---

## Critério de conclusão

1. Relatório formal de fecho escrito.
2. DEBT-1 formalmente fechado em `DEBT.md`.
3. DEBT-52 formalmente fechado em `DEBT.md`.
4. ADR-0054 registada como cumprida no relatório.
5. Limitações preservadas; gaps 7/8 como candidatos futuros.
6. Testes inalterados; lint zero; zero código tocado.
7. Nenhum DEBT novo aberto.
8. Contagem de DEBTs abertos actualizada.

---

## O que pode sair errado

- **`StyleDelta` tem campo que não está no inventário
  esperado**: em 142.1.A.2, lista real pode ter campos extra
  adicionados em passos entre 139 e 141 que eu não recordo.
  Nesse caso, cada campo extra entra na tabela com consumer
  identificado ou scope-out documentado. Se algum campo não
  tiver consumer nem ADR de scope-out, **pausar** — o fecho
  de DEBT-1 não é justificado até se resolver.

- **ADR-0054 tem critério que não foi antecipado**: ao
  transcrever critérios em 142.1.A.1, pode aparecer condição
  adicional (ex: "documentar desvios de performance"). Nesse
  caso, adicionar secção dedicada ao relatório. Se critério
  não puder ser cumprido, pausar e abrir discussão.

- **Referência cruzada a DEBT-1 em ADR activa**: ADRs podem
  referenciar DEBT-1 como precedente ou contexto. Essas
  referências permanecem (descrevem trabalho real); não se
  actualizam. Se uma ADR usar DEBT-1 como **pré-condição
  aberta** (ex: "quando DEBT-1 fechar, fazer X"), esse X
  pode ficar pendente — registar no relatório como
  observação, sem acção imediata.

- **Gap 8 (font dict) aparece como bloqueante implícito**:
  ADR-0054 declara opcional, mas utilizadores podem ter
  expectativa. Relatório deve ser explícito: **gap 8 é
  opcional**, não foi materializado, não bloqueia fecho.
  Registar forma actual aceite de `#set text(font: "X")`
  (string) e `#set text(font: ("A","B"))` (array); forma dict
  `#set text(font: ("name": "A", ...))` não é suportada e é
  out-of-scope.

- **Relatório cresce > 500 linhas**: aceite, mas nesse caso
  considerar subdivisão em secções separadas. Alternativa:
  manter relatório compacto e referenciar 140A/140B/141 para
  detalhe.

- **`cargo test` degrada de 1116**: impossível por construção
  (zero código tocado). Se degradar, algo fora do escopo do
  passo aconteceu — pausar e investigar antes de fechar.

- **DEBT-52 gap 7 ou gap 8 reabrem discussão**: se conversa
  futura levantar um deles, é matéria de passo novo (143/144
  ou ADR-0054bis). Este passo apenas regista que são
  opcionais; não decide sobre priorização.

---

## Notas operacionais

- **Passo administrativo puro**. Cumprimento de ADR-0054 é
  certificação, não trabalho. Toda a substância foi paga em
  140B + 141.

- **Modelo de fecho análogo a Passos 84.5, 92, 95** (fechos
  formais de DEBT-36, DEBT-44, DEBT-39 respectivamente). Este
  é o primeiro fecho de DEBT **maior** (DEBT-1 é base do
  projecto) — justifica relatório mais detalhado que fechos
  anteriores.

- **Gap 7 e gap 8 não viram DEBTs**. Decisão consciente de
  manter o inventário limpo. Candidatos futuros são trabalho
  de priorização humana, não acumulação administrativa.

- **Numeração**: 142 para o fecho; multi-font per document
  torna-se **142A** se e quando for priorizada. O padrão
  "NxxA" já é usado em 131A, 132A, 140A para diagnósticos;
  reutilizar para sub-séries de materialização preserva a
  grelha.

- **ADR-0055bis continua candidata** para variant-aware.
  Fecho de DEBT-1 não bloqueia abertura desta ADR; são
  trabalhos independentes.

- **Após este passo**: projecto fica com 11 DEBTs abertos,
  55 ADRs (54 activas + ADR-0055 `IMPLEMENTADO`). Próxima
  decisão é humana: priorizar 142A (multi-font), 143
  (hyphenation), ADR-0055bis (variant-aware), ou áreas
  orgonais.

- **O marco é real mas modesto**. DEBT-1 fechou; nem tudo o
  que era sonhado em 2024/2025 foi implementado. Faux-bold
  substitui font Bold dedicada; subsetting ausente; shaping
  não existe. O critério cumprido é o de ADR-0054, não um
  ideal absoluto. Registar esta humildade no relatório.
