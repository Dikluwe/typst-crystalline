# Passo 140A — Diagnóstico amplo de infra PDF para font embedding

**Série**: 140A (passo **S-M** em L0; diagnóstico inicial da
Fase C do roadmap DEBT-1).
**Precondição**: Passo 139 encerrado; 1100 total tests; zero
violations; 54 ADRs activas; 12 DEBTs abertos (DEBT-52 com 4
gaps — Fase B completa). Consumer `font` ainda não existe.

**ADRs aplicáveis**:
- **ADR-0019** (TTF+RustyBuzz `IMPLEMENTADO`) — **estado empírico
  a confirmar**. Relatório 135 sugeriu que `rustybuzz` não está
  no repo. Este diagnóstico resolve a divergência.
- **ADR-0033** (paridade funcional).
- **ADR-0034** (diagnóstico obrigatório para materialização).
- **ADR-0054** (critério fecho DEBT-1).
- **ADR-0053** (FontList materializado; covers deferido).

**Natureza**: passo L0-puro. **Sem código**. **Sem testes**.
Produz documento de inventário alargado + ADR proposta + potencial
ajuste de ADR-0019 + roadmap de Fase C.

**Pausa após encerramento**: conversa nova começa com output
deste passo como contexto. Razão: Fase C é suficientemente
diferente do que veio antes para justificar reset de contexto.

---

## Contexto

Roadmap 135 previu Fase C em 4-5 passos (140-143). Relatório
139 sugeriu que font exige diagnóstico dedicado antes. Decisão
confirmada (padrão 131/132/135).

`font` é qualitativamente diferente de tracking/leading/weight:
são parâmetros contínuos; font é **selecção de recurso externo**.
Requer:

- Mecanismo de lookup (`FontBook::select` — existe segundo 135).
- Representação de `Font` runtime (com bytes) — a confirmar.
- Descoberta de fontes no sistema (L3 I/O).
- PDF embedding (Font dict, FontDescriptor, FontFile2, CIDFont,
  ToUnicode).
- Possível subsetting (optimização).
- Fallback quando fonte não disponível.

Exporter actual só sabe F1/F2/F3 Helvetica hardcoded. Não há
registo, embedding, ou resolução real.

---

## Contexto estratégico

Fase C do roadmap revisto:

- **140A** (este): diagnóstico amplo.
- **Pausa** — conversa nova com output deste passo como
  contexto.
- **140B / 141 / 142 / ...**: materialização conforme o
  roadmap saído de 140A.

Número de sub-passos de materialização desconhecido até 140A
encerrar.

---

## Objectivo

Ao fim do passo:

1. **Diagnóstico** em
   `00_nucleo/diagnosticos/diagnostico-font-infra-passo-140a.md`
   cobrindo 4 áreas:
   - L1 — domínio font runtime.
   - L3 — descoberta de fontes no sistema.
   - L3 — PDF embedding (resource dict + font programs).
   - Vanilla referência — como vanilla resolve.

2. **ADR proposta** em
   `00_nucleo/adr/typst-adr-NNNN-font-infra.md` em status
   `PROPOSTO` com:
   - Decisão sobre autorização de crates (ttf-parser, fontdb,
     subsetter, outras).
   - Estratégia de materialização (tipo `Font` em L1? loader
     em L3? embedding path?).
   - Plano de sub-passos de materialização.

3. **Decisão sobre ADR-0019** (TTF+RustyBuzz `IMPLEMENTADO`):
   - Se confirmada: anotar com referência a sítios reais no
     código.
   - Se divergente: abrir ADR de revisão (ADR-0019-R1) ou
     revogação.

4. **Roadmap de Fase C** estimado em número de passos e
   complexidade cumulativa.

5. **Potencial abertura de novos DEBTs** se diagnóstico revela
   trabalho adjacente (ex: shaping engine completo).

Este passo **não**:

- Toca código em L1, L2, L3, L4.
- Toca testes.
- Implementa qualquer parte de font embedding.
- Fecha DEBT-52 (ataca apenas preparação de gaps 5-7).

---

## Decisões já tomadas

1. **Escopo amplo**: cobrir descoberta + embedding + CIDFont +
   subsetting. Se escopo revelar-se excessivamente grande,
   registar no relatório mas **não reduzir o diagnóstico** —
   decisão de redução tomada na conversa nova.
2. **ADR em `PROPOSTO`**, não `EM VIGOR` — padrão 131A/132A
   (implementação materializa depois).
3. **Zero código tocado**: único output é documentação.
4. **Pausa após encerramento**: conversa nova começa com este
   diagnóstico como contexto.

## Decisões diferidas (resolvidas pelo diagnóstico)

5. **Nome do tipo `Font` em L1**: já existe? Outro nome?
6. **Que crates autorizar**: `ttf-parser` (leitura), `fontdb`
   (descoberta system), `subsetter` (optimização). Cada uma
   tem custo de deps transitivas.
7. **Embedding full-font vs subset**: full é maior mas
   simpler. Subset é menor mas exige identificar glyphs usados.
8. **ToUnicode CMap**: para texto copiável do PDF. Vanilla
   faz. Cristalino?
9. **Fallback quando fonte não existe**: warning? erro? usar
   Helvetica?

---

## Escopo

**Dentro**:
- Leitura de `01_core/src/` (procurar `Font`, `FontBook`,
  embedding).
- Leitura de `03_infra/src/` (procurar font loading, export PDF).
- Leitura de `lab/typst-original/` (vanilla referência).
- Leitura de `Cargo.toml` do workspace e de cada crate (L1/L3
  autorizadas).
- Escrita de diagnóstico + ADR.

**Fora**:
- Código fonte.
- Testes.
- Shaping completo (rustybuzz integration) — âmbito de outro
  passo/ADR se for necessário.
- Font matching avançado (italic/weight variants via
  `FontBook::select` — se existe, documentar; se não,
  registar como gap).

---

## Sub-passos

### 140A.1 — Inventário L1 domínio font

**A.1.1 — `FontBook` actual**:

`grep -rn "FontBook\|struct Font\b\|pub fn select" 01_core/src/`.

Registar:
- Se `FontBook` existe.
- Se tem método `select(family, weight, style) -> Option<...>`.
- O que devolve (`FontId`? `Font`? índice?).
- Onde é inicializado (quem popula o catálogo?).
- Relação com `font_list.rs` (132B) e `font_book.rs` (existe com
  `FontWeight`/`FontStretch`).

**A.1.2 — Tipo `Font` runtime**:

Vanilla tem `Font` com bytes. Cristalino tem equivalente?
`grep -rn "struct Font\b" 01_core/src/`.

Registar:
- Se existe, forma interna (bytes? path? reference counted?).
- Métodos públicos.
- Integração com `FontBook`.

**A.1.3 — `StyleDelta.font` → resolução**:

Campo é `Option<FontList>`. Como passa de `FontList` (lista de
nomes) para `Font` runtime (bytes)?

`FontList::single("Arial")` → ??? → bytes para embedding.

Registar o gap. Provavelmente:
- `FontList` → iterate families → `FontBook::select(name)` →
  `FontId` → `FontBook::get(id)` → `Font` → bytes.

Cada passo confirmar se existe.

### 140A.2 — Inventário L3 descoberta

**A.2.1 — Descoberta de fontes no sistema**:

`grep -rn "fontdb\|FontDb\|system_fonts\|walk_fonts\|\\.ttf\\b\|\\.otf\\b" 03_infra/src/ 01_core/src/ Cargo.toml`.

Registar:
- Se `fontdb` crate está autorizada.
- Se há código que descobre fontes do sistema.
- Se há fontes embutidas no binário (ex: Computer Modern via
  `include_bytes!`).
- Se existe CLI flag `--font-path` (resumo pós-127 menciona) —
  como é usada?

**A.2.2 — CLI `--font-path`**:

Já existe (resumo pós-127). Verificar:
- O que o flag faz em código.
- Se as fontes descobertas atravessam para `FontBook`.
- Relatórios 117/119 refactor em camadas — `--font-path` vive
  em L3.

**A.2.3 — fontes embutidas**:

Typst vanilla embute Computer Modern, DejaVu, etc. como
defaults. Cristalino tem equivalente?

`grep -rn "include_bytes!\|DEFAULT_FONTS\|embedded.*font" 01_core/src/ 03_infra/src/`.

### 140A.3 — Inventário L3 PDF embedding

**A.3.1 — Resource dict actual**:

`grep -rn "F1\|F2\|F3\|/Font\|Resources\|FontDict" 03_infra/src/export.rs`.

Registar forma actual do resource dict PDF:
- Como F1/F2/F3 são declarados.
- Se há estrutura para adicionar fontes dinâmicamente.
- Se é hardcoded ou tem build-step.

**A.3.2 — Capacidade de embedding**:

Procurar se há código que escreve:
- `Font dictionary` (PDF object type Font).
- `FontDescriptor`.
- `FontFile2` stream (TrueType bytes).
- `Widths` array.
- `ToUnicode` CMap.
- `CIDFont` (para Unicode além de WinAnsi).

Esperado: **nada disto existe hoje**. Confirmar.

**A.3.3 — PDF writer utilizado**:

O exporter actual escreve PDF como strings? Usa crate
(`pdf-writer`, `printpdf`, `lopdf`)?

`grep -rn "pdf-writer\|printpdf\|lopdf" Cargo.toml 03_infra/src/`.

Se usa crate: qual? tem suporte para embedding? documentado?
Se manual: cada Font precisa de ser construído manualmente —
mais trabalho.

### 140A.4 — Inventário vanilla

**A.4.1 — Pipeline vanilla font → PDF**:

Leitura de `lab/typst-original/crates/typst-pdf/src/` (ou
caminho real):
- Ficheiro principal de font embedding.
- Tipos usados.
- Se usa `ttf-parser`, `fontdb`, `subsetter`.

Registar estrutura do code — pode servir de template para
cristalino (com adaptações de arquitectura crystalline).

**A.4.2 — Crates que vanilla usa para fontes**:

`cat lab/typst-original/Cargo.toml` ou `find . -name Cargo.toml`:
- `ttf-parser` — leitura de TTF/OTF.
- `fontdb` — descoberta de sistema.
- `subsetter` ou `fonttools-rs` — subsetting.
- `rustybuzz` — shaping.
- Outras.

Para cada crate, registar: versão, feature flags, tamanho
típico de deps transitivas.

**A.4.3 — Paridade observacional para font**:

Vanilla produz PDFs com fontes embutidas. Cristalino alvo é
output observacionalmente equivalente (ADR-0054 perfil
graded). Questão: o quão próximo precisa ser?

- **Paridade total**: subsetting + CIDFont + ToUnicode + ...
- **Paridade básica**: 1 fonte embutida inteira + Widths
  correctas. Texto renderiza correcto, PDF maior que vanilla
  mas observacionalmente equivalente a olhar.
- **Paridade mínima**: fallback para Helvetica se font não
  match; warning.

### 140A.5 — Inventário de ADR-0019

**A.5.1 — Confirmar estado IMPLEMENTADO**:

ADR-0019 diz TTF+RustyBuzz `IMPLEMENTADO`. Relatório 135
sugeriu que `rustybuzz` não está no repo.

- `grep -rn "rustybuzz" workspace/`.
- `grep -rn "ttf-parser\|ttf_parser" workspace/`.

Três resultados possíveis:
- **Ambas ausentes**: ADR-0019 é obsoleta ou incorrecta.
  Revogar ou abrir ADR-0019-R1.
- **TTF presente, RustyBuzz ausente**: ADR-0019 parcialmente
  implementada. Documentar.
- **Ambas presentes mas não integradas em export**: ADR-0019
  cobre leitura e não embedding. Expandir âmbito em ADR
  complementar.

**A.5.2 — Se ADR-0019 está errada, acção**:

Não é trabalho deste passo resolver. Registar no diagnóstico
com recomendação para conversa nova decidir.

### 140A.6 — Decisão sobre crates

Com base em A.2.1 + A.3.3 + A.4.2, propor lista de crates a
autorizar em L1 e L3:

**L1 candidates** (domínio font puro):
- `ttf-parser` (leitura pura, sem I/O). Provável.
- `subsetter` ou similar (optimização). Opcional.

**L3 candidates** (descoberta + embedding):
- `fontdb` (system discovery). Requer I/O (filesystem).

Para cada crate, registar:
- Nome exacto, versão.
- Deps transitivas (quantas, quais).
- Feature flags que minimizam superfície.
- ADR necessária para autorização (precedente: ecow, rustc-hash,
  indexmap, ecow:EcoVec, comemo).

### 140A.7 — Escrever diagnóstico

Ficheiro:
`00_nucleo/diagnosticos/diagnostico-font-infra-passo-140a.md`.

**Template**:

```markdown
# Diagnóstico de infra PDF para font embedding — Passo 140A

**Data**: 2026-MM-DD
**ADR alvo**: ADR-NNNN — "Font embedding real em L1/L3".
**Motivação**: Fase C do roadmap DEBT-1 precisa de consumer
real para `text.font`. Cristalino actual só tem F1/F2/F3
Helvetica hardcoded. Este diagnóstico mapeia trabalho de infra.

---

## 1. L1 — Domínio font runtime

### 1.1 FontBook actual
### 1.2 Tipo Font runtime
### 1.3 Resolução StyleDelta.font → bytes

## 2. L3 — Descoberta de fontes

### 2.1 Descoberta no sistema
### 2.2 CLI --font-path
### 2.3 Fontes embutidas

## 3. L3 — PDF embedding

### 3.1 Resource dict actual
### 3.2 Capacidade de embedding
### 3.3 PDF writer

## 4. Vanilla referência

### 4.1 Pipeline font → PDF no vanilla
### 4.2 Crates do vanilla
### 4.3 Níveis de paridade

## 5. ADR-0019 estado empírico

### 5.1 Confirmação ou divergência
### 5.2 Acção recomendada

## 6. Crates a autorizar

### 6.1 L1 candidates
### 6.2 L3 candidates
### 6.3 ADRs necessárias

## 7. Roadmap proposto

### 7.1 Níveis de paridade e sub-passos
### 7.2 Estimativa por sub-passo
### 7.3 Dependências entre sub-passos
### 7.4 Alternativas (reduzir escopo)

## 8. DEBTs adjacentes

### 8.1 Relacionamento com DEBT-52
### 8.2 Novos DEBTs propostos (se houver)

## 9. Resumo executivo

[1-2 parágrafos com as conclusões principais]
```

### 140A.8 — Escrever ADR proposta

Ficheiro: `00_nucleo/adr/typst-adr-NNNN-font-infra.md`.

Número: verificar `ls 00_nucleo/adr/typst-adr-*.md | sort |
tail -5`. Candidato base: **ADR-0055** (0054 é "critério fecho
DEBT-1").

Template (seguindo ADR-0052 e ADR-0053 como precedentes):

```markdown
# ⚖️ ADR-NNNN: Font embedding real em L1/L3

**Status**: `PROPOSTO`
**Data**: 2026-MM-DD
**Diagnóstico prévio**:
`00_nucleo/diagnosticos/diagnostico-font-infra-passo-140a.md`

## Contexto

[Parágrafo: Fase C DEBT-1, F1/F2/F3 hardcoded, trade-off
paridade ADR-0033 vs custo de infra]

## Decisão

1. Crates autorizadas em L1: [lista].
2. Crates autorizadas em L3: [lista].
3. Tipo `Font` em L1 com [forma interna].
4. Pipeline de descoberta: [descrição].
5. Pipeline de embedding PDF: [descrição].
6. Nível de paridade escolhido: [total / básica / mínima].
7. Plano de materialização em N sub-passos.

## Alternativas consideradas

| Alternativa | Prós | Contras |
|-------------|------|---------|
| Paridade total (subsetting + CID) | Mais próximo vanilla | Grande trabalho |
| Paridade básica (full font embed) | Observacional OK | PDFs maiores |
| Paridade mínima (fallback Helvetica) | XS | Utilizador frustrado |
| Usar crate X vs Y | ... | ... |

## Consequências

**Positivas**:
- Consumer font real.
- DEBT-52 Fase C avança.

**Negativas**:
- Crates novas autorizadas.
- Custo de materialização maior que Fase B.

**Neutras**:
- ADR-0019 pode precisar revisão (A.5).

## Referências

- ADR-0019 (RustyBuzz, estado a confirmar).
- ADR-0053 (FontList).
- ADR-0054 (critério fecho DEBT-1).
- DEBT-52.
- Passo 140A (este).
```

### 140A.9 — Se ADR-0019 revisão é necessária

Se A.5.1 revela divergência clara, **registar no diagnóstico**
e **não criar revisão neste passo**. A criação de ADR-0019-R1
(ou revogação) fica para conversa nova — não é trabalho do
diagnóstico.

### 140A.10 — Abertura potencial de DEBTs novos

Se diagnóstico revela que shaping completo (rustybuzz) é
trabalho paralelo significativo **fora do âmbito de DEBT-1**,
abrir DEBT novo (candidato **DEBT-53** ou próximo número) com
scope dedicado.

Critério: se trabalho é pré-requisito para fechar DEBT-1,
fica em DEBT-52. Se é trabalho adjacente que ADR-0054 explicitou
como "fora de DEBT-1", fica em DEBT novo.

---

## Verificação

1. Diagnóstico criado com 9 secções preenchidas com factos
   concretos (não placeholders).
2. ADR-NNNN criada com `Status: PROPOSTO`.
3. ADR-0019 estado documentado (confirmado, divergente, ou
   parcial).
4. Decisão de crates a autorizar registada.
5. Roadmap de Fase C com número de sub-passos estimado.
6. Nenhum ficheiro em `01_core/`, `02_shell/`, `03_infra/`,
   `04_wiring/` tocado.
7. `cargo test --workspace` continua em 1100, 6 ignorados.
8. `crystalline-lint` zero violations.

---

## Critério de conclusão

1. Diagnóstico com 9 secções factuais.
2. ADR-NNNN proposta com 7 decisões.
3. ADR-0019 estado registado; acção recomendada na
   documentação.
4. Roadmap de Fase C estimado em passos + horas cumulativas.
5. Decisão sobre DEBT novo (ou não) explicitada.
6. Zero código tocado.
7. `cargo test --workspace` inalterado (1100).
8. `crystalline-lint` zero.
9. Relatório 140A.K escrito.

---

## O que pode sair errado

- **Inventário revela que `Font` runtime é fundamentalmente
  diferente de vanilla** (ex: cristalino tem arquitectura
  própria para gerir fontes que não mapeia trivialmente): ADR
  proposta precisa de redesenhar, não copiar. Mais trabalho
  no 140A.

- **Nenhuma crate de embedding é autorizável sem ADR que
  autorize múltiplas**: pode exigir meta-ADR de "bloco de
  crates font". Baixo risco mas possível.

- **ADR-0019 é totalmente obsoleta**: não bloqueia este passo,
  mas obriga decisão na conversa nova (revogar vs reabrir).

- **Diagnóstico chega a > 500 linhas**: aceite. Escopo amplo
  foi decisão consciente. Registar no relatório como "passo
  de retorno diferido — conversa nova tem contexto denso".

- **Vanilla font pipeline usa optimizações complexas
  (CFF subsetting, composite glyphs, ligatures features)**: não
  replicar neste passo; registar como "paridade total requer X
  passos adicionais, paridade básica requer Y".

- **Crates vanilla não autorizáveis (ex: `rustybuzz` exige `std`
  + C deps)**: alternativa é implementação própria (XXL) ou
  redução de escopo (Opção "paridade mínima"). Registar.

- **Conversa corrente fica "gorda" após este passo**: esperado.
  Pausa explícita e reinício em nova conversa resolve.

---

## Notas operacionais

- **Escopo amplo intencional**. 131A e 132A tiveram escopo
  bem delimitado (1 tipo cada). 135 expandiu para 5 áreas. 140A
  continua a expandir para 4 áreas L0 + 1 ADR + 1 revisão
  potencial de ADR anterior. É o mais alargado de todos.

- **Pausa explícita**: conversa nova começa com diagnóstico
  + ADR como contexto. Este passo é investimento pesado em
  documentação para reduzir risco em múltiplos passos de
  materialização.

- **ADR-0019 divergência é bonus**: se diagnóstico apanha
  erro em documentação antiga, vale o esforço mesmo sem
  Fase C.

- **Três aplicações do padrão "só diagnóstico"**: 131A (Lang),
  132A (FontList), 135 (shaping), 140A (font infra). Quatro
  aplicações. Pattern consolidado. Candidato para formalizar
  em ADR-0034 continua pendente — adicionar ao relatório como
  candidato futuro.

- **Próximo passo (140B ou 141) escreve-se em conversa nova**
  com base no diagnóstico. Se diagnóstico revela que
  materialização exige ADR-0019-R1 antes, essa é a sequência
  correcta — não começar por 140B sem infra arquitectural
  alinhada.

- **Pausa não é fim**: é transição. Todo o trabalho de 126-139
  continua válido. O DEBT-52 continua aberto. Conversa nova
  continua o fecho de DEBT-1.
