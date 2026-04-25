# Passo 154A — Diagnóstico Model (structural): inventário + arqueologia + roadmap

**Série**: 154A (passo **L0-puro / diagnóstico-primeiro**;
quinta aplicação do padrão 131A/132A/140A/148 — agora aplicado
a uma categoria do inventário 148 em vez de a tipo único ou
infra geral).
**Precondição**: Passo 153 encerrado; matriz P2 + P3
cristalino-only baseline; 1113 tests; zero violations; 59
ADRs; 12 DEBTs abertos; inventário 148 actualizado por P149.

**Numeração**: 154A. Sufixo `A` = diagnóstico-primeiro. **Não
ocupa P154**: P154 fica reservado para P4 cristalino-only
(per §9 dos documentos de paridade, item 6 acrescentado em
P153). Coerente com prioridade tua de **sair da série paridade
para atacar gap real de cobertura**. Padrão de numeração
preservado: 154A diagnóstico → 154B+ materialização (se 154A
recomendar passo único) ou 155+/156+/... (se recomendar
múltiplos passos materializadores).

**Mudança de prioridade registada**: série paridade fica
**suspensa em P153**. Documentos de paridade não tocados;
DEBT-53 + DEBT-54 inalterados. Quando série paridade for
retomada, P154 = P4 retoma onde parou.

**Natureza**: passo **L0-puro / administrativo / diagnóstico
+ ADR proposta**. **Zero código**. **Zero testes**. **1 ADR
proposta** (status `PROPOSTO`); **0-N DEBTs novos** consoante
arqueologia (provável: 1-3 candidatos a DEBT). **Possível
actualização ao inventário 148** se arqueologia revelar
divergências de classificação.

**ADRs aplicáveis**:
- **ADR-0026** + **ADR-0026-R1** + **ADR-0038** — `Content`
  como enum fechado; novos elementos exigem variants novas
  ou re-uso via `Content::Styled`.
- **ADR-0033** — paridade funcional para cada feature
  materializada.
- **ADR-0034** — diagnóstico obrigatório para tipos vanilla
  (cumprido por este passo).
- **ADR-0036** — atomização progressiva.
- **ADR-0037** — coesão por domínio (relevante para escolha
  de localização de novos elementos).
- **ADR-0054** — perfil observacional graded.

---

## Contexto

Inventário 148 §3 mostra distribuição user-facing por
categoria. Model (structural) tem **21 entradas** com
distribuição: 4 implementado / 4 implementado⁺ / 5 parcial /
8 ausente / 0 scope-out. Cobertura **38% (impl + impl⁺) =
8/21**. Uma das duas categorias mais fracas (Layout também
38%; Introspection 17% é menor mas tem só 6 entradas).

Top divergência 7 do inventário 148 agrega "~14 elementos
`Content::*` vanilla ausentes": Bibliography, Cite,
Footnote, Quote, Terms, Table, Columns, Box, Block, Stack,
Hide, Repeat, Pad, Stroke-object. Alguns são Model
structural (Bibliography, Cite, Footnote, Quote, Terms,
Table); outros são Layout (Columns, Box, Block, Stack, Hide,
Pad, Repeat) ou Visualize (Stroke-object). Diagnóstico vai
clarificar qual é qual.

DEBT-34d e DEBT-34e mencionados em relatório 142 são
referidos como "abertos" no contexto de table — vale
verificar empiricamente em 154A.1.

**Pergunta primária do diagnóstico**: dadas as 21 entradas
de Model, qual o trabalho **realmente necessário** para
elevar cobertura de 38% para um valor target (60%? 80%?), e
em que ordem?

**Pergunta secundária**: que entradas exigem decisão
arquitectural (ADR) antes de materialização, e quais são
trabalho mecânico (consumer + tests)?

**Hipóteses a confirmar empiricamente** (não compromisso):

- `figure` é `implementado⁺` mas atributos `kind`,
  `numbering`, `supplement` podem ser `parcial`.
- `heading` é `implementado` (Passo 99/103) mas atributos
  pós-DEBT-1 podem ter limitações.
- `table` está bloqueado em ausência de `Content::Table`.
  DEBT-34d/e podem cobrir.
- `bibliography` + `cite` são XL (hayagriva + CSL).
- `footnote` é M (page model + área reservada).
- `quote` é S-M (estrutural + atributos).
- `terms` é S.

---

## Objectivo

Ao fim do passo:

1. **Documento de diagnóstico** em
   `00_nucleo/diagnosticos/diagnostico-model-passo-154a.md`
   com 8 secções:

   1. **Inventário detalhado** — explodir 21 entradas em
      sub-features (atributos, métodos, regras `#show`
      esperadas). Confronto com vanilla
      `lab/typst-original/crates/typst-library/src/model/`.
   2. **Estado actual em cristalino** — para cada entrada e
      sub-feature: existe? `parcial`? `ausente`? Referência
      canónica (Passo / ADR).
   3. **Tipos arquitecturais bloqueantes** — para cada
      entrada `parcial` ou `ausente`: que `Content::*`
      variants ou outros tipos faltam ou estão incompletos.
   4. **Arqueologia das ausências** — para cada `ausente`:
      passo materializador esperado nunca aconteceu, ou foi
      adiado por ADR? Razões registadas (ou ausência
      declarada).
   5. **Crates externas necessárias** — bibliography exige
      `hayagriva`; outras podem exigir crates específicas.
      Verificar por sub-feature.
   6. **Priorização proposta** — matriz custo (S/M/L/XL) ×
      valor user-facing (alto/médio/baixo). 5 a 8 entradas
      ranqueadas.
   7. **Plano de materialização** — N passos sugeridos com
      escopo cumulativo. Inclui passo dedicado ADR para
      escolhas arquitecturais (ex: bibliography exige
      autorização de `hayagriva`).
   8. **Resumo executivo** — 2-3 parágrafos.

2. **ADR proposta** em
   `00_nucleo/adr/typst-adr-0060-model-structural-roadmap.md`
   com status `PROPOSTO`:
   - Cabeçalho canónico P145 (`⚖️`, `**Status**: \`PROPOSTO\``,
     `**Validado**: Passo 154A — diagnóstico`).
   - Decisão de roadmap: priorização ranqueada do diagnóstico.
   - Decisão de zero crates novas para Fase 1 (sub-set de
     features que pode ser materializado sem autorização);
     crates novas (`hayagriva`, etc) ficam para ADRs
     subsequentes (`ADR-0061` se necessário).
   - Decisão sobre re-uso de `Content::Styled` vs novos
     `Content::*` variants (per ADR-0026 perfil): cada
     feature decide individualmente; documento dá guia.
   - Plano de materialização com 3-5 passos (ranqueamento
     S/M; XL adiados).
   - Alternativas consideradas (atacar tudo de uma vez vs
     sub-fases vs adiamento total).
   - Consequências.

3. **DEBT-34d e DEBT-34e verificados**:
   - Se abertos: confirmar conteúdo e relação com diagnóstico.
   - Se já fechados: registar.
   - Se referenciados mas não-existentes: corrigir referência
     no relatório 142 ou marcar como dívida documental.

4. **Possíveis DEBTs novos** (1-3 candidatos):
   - **DEBT-XX (XL)**: bibliography + cite — escopo
     largo; provavelmente não atacável em Fase 1.
   - **DEBT-YY**: outras features adiadas se diagnóstico
     revelar trabalho substancial não materializado e fora
     da Fase 1 proposta.
   - Critério para abrir DEBT vs deixar como item do
     roadmap: **DEBT** se trabalho exige ADR de autorização
     de crate externa OU é pré-condição para outra feature
     já priorizada. **Item do roadmap** se trabalho é
     mecânico e cabe em passo simples.

5. **Inventário 148 actualizado** se diagnóstico revelar
   classificação errada:
   - Tabela A linha "Model": ajustar contagens se
     reclassificações justificadas.
   - Tabela C: adicionar bloqueantes detectados.
   - §7 Top divergências: actualizar entrada 7 com referência
     ao diagnóstico 154A.

6. **README dos ADRs actualizado** se ADR-0060 criada:
   - Tabela "Estado por ADR".
   - Distribuição.
   - Total 59 → 60.
   - Entrada em "Passos-chave da história dos ADRs".

7. **Relatório do passo** em
   `00_nucleo/materialization/typst-passo-154a-relatorio.md`.

Este passo **não**:

- Toca código em L1/L2/L3/L4.
- Toca testes.
- Implementa qualquer feature de Model.
- Materializa ADRs como `IMPLEMENTADO`.
- Resolve DEBT-53 / DEBT-54 da série paridade.
- Toca série paridade (suspensa em P153).
- Decide entre opções 2-5 da pergunta de priorização — o
  diagnóstico produz a recomendação.
- Importa `hayagriva` ou outras crates externas.
- Materializa multi-column flow ou page model novo.

---

## Decisões já tomadas

1. **Diagnóstico-primeiro**, padrão 131A/132A/140A/148. Quinto
   exemplo do padrão.
2. **Sufixo `A`** preserva semântica "diagnóstico" — coerente
   com precedentes (131A→131B, 140A→140B).
3. **Mudança de prioridade**: série paridade suspensa em
   P153; ataque a gap real de cobertura prioritário.
4. **ADR proposta `PROPOSTO`** (não `EM VIGOR` nem
   `IMPLEMENTADO`) — espelha 131A/132A/140A/148. Materialização
   em passos posteriores.
5. **Zero código tocado**.
6. **Bibliography + Cite são XL** (hipótese forte; vai à
   arqueologia). Provavelmente fora de Fase 1.

## Decisões diferidas (resolvidas neste passo)

7. **Target de cobertura post-roadmap**: 60%? 80%? 100%?
   Decisão em 154A.6 com base em distribuição custo/valor.
   **Default**: aspirar a 60-70% sem comprometer-se a um
   número exacto. Roadmap entrega "Fase 1: features
   priorizadas → cobertura X%"; futuras fases negociáveis.

8. **Localização canónica** de futuros elementos `Content::*`:
   `01_core/src/entities/content.rs` é onde vive `Content`
   enum. Adicionar variants exige edição ali. Confirmação
   em 154A.1.

9. **Re-uso `Content::Styled` vs variant novo**: ADR-0026
   permite ambos; perfil graded ADR-0054 aceita. Decisão por
   feature em 154A.7.

10. **Número da ADR proposta**: ADR-0060 (sequencial).
    Reservado: **ADR-0061** para autorização de `hayagriva`
    se diagnóstico recomendar bibliography em Fase 2.
    **ADR-0062** para autorização de qualquer outra crate
    externa específica.

11. **DEBT-34d / DEBT-34e**: 154A.1 confirma estado.
    Critério para acção:
    - Se abertos e relacionados com table/figure: integrar
      no roadmap; possivelmente referenciar de ADR-0060.
    - Se fechados: registar e seguir.
    - Se inexistentes: ajustar relatório 142 mentalmente
      (não corrigir documento histórico imutável; registar
      no relatório deste passo).

---

## Escopo

**Dentro**:

- Leitura de
  `lab/typst-original/crates/typst-library/src/model/` para
  inventário detalhado (sub-features, atributos, métodos).
- Leitura de `01_core/src/entities/content.rs`,
  `01_core/src/rules/eval.rs` (para `native_*`),
  `01_core/src/rules/stdlib/mod.rs` para confirmar estado
  cristalino.
- Cross-reference com `00_nucleo/adr/`,
  `00_nucleo/materialization/`, `00_nucleo/DEBT.md`.
- Escrita do diagnóstico (com 8 secções).
- Escrita de ADR-0060 (PROPOSTO).
- Possível abertura de 1-3 DEBTs.
- Actualização de inventário 148 (se aplicável).
- Actualização do README dos ADRs.
- Relatório do passo.

**Fora**:

- Modificação de código em L1/L2/L3/L4.
- Modificação de testes.
- Materialização de feature.
- Importação de crates novas.
- Materialização de ADR-0060 (PROPOSTO permanece).
- Decisão final sobre target de cobertura específico.
- Modificação de outros ADRs.
- Trabalho na série paridade.

---

## Sub-passos

### 154A.1 — Inventário detalhado

**A.1.1 — Listar Model vanilla**:

```bash
ls lab/typst-original/crates/typst-library/src/model/
```

Esperado: ~22 ficheiros conforme inventário 148 §2.1.
Cada ficheiro = 1 ou mais elementos.

**A.1.2 — Para cada ficheiro, extrair elementos**:

```bash
for f in lab/typst-original/crates/typst-library/src/model/*.rs; do
  echo "=== $f ==="
  grep -E "^#\[elem\]|^pub struct \w+Elem|^pub fn \w+_elem" "$f" | head -10
done
```

Registar tabela:

| Elemento | Ficheiro | Atributos públicos | Métodos públicos | `#show` rules suportadas |
|----------|----------|---------------------|-------------------|--------------------------|
| `HeadingElem` | heading.rs | level, numbering, ... | level(), ... | show heading: ... |
| ... | ... | ... | ... | ... |

**A.1.3 — Confronto com cristalino**:

```bash
grep -nE "Heading|Figure|Footnote|Quote|Terms|Table|Cite|Bibliography" \
  01_core/src/entities/content.rs
grep -nE "native_(heading|figure|footnote|quote|table|cite)" \
  01_core/src/rules/eval/mod.rs
```

Para cada elemento vanilla, registar correspondente
cristalino (existe? `parcial`? `ausente`?) e justificar.

**A.1.4 — DEBT-34d / DEBT-34e**:

```bash
grep -A 5 "^## DEBT-34d\|^## DEBT-34e" 00_nucleo/DEBT.md
```

Confirmar abertos/fechados/inexistentes.

### 154A.2 — Estado actual cristalino

Tabela final por elemento:

| Elemento | Cristalino estado | Cobertura sub-features | Bloqueantes |
|----------|-------------------|------------------------|-------------|
| `heading` | implementado | level OK; numbering parcial; supplement ausente; outline parcial | numbering rules; show selectors |
| `figure` | implementado⁺ | kind=image OK; kind=table ausente; kind=equation ausente | Content::Table; numbering |
| ... | ... | ... | ... |

### 154A.3 — Tipos arquitecturais bloqueantes

Para `parcial`/`ausente`, listar tipos faltantes:

| Bloqueante | Quem precisa | Custo estimado | Decisão arquitectural? |
|------------|--------------|----------------|------------------------|
| `Content::Table` | table, figure(kind=table) | M | sim — variant nova |
| `Content::Footnote` | footnote | M | sim — variant ou Styled? |
| `Content::Quote` | quote | S-M | sim — variant ou Styled? |
| `Content::Bibliography` | bibliography, cite | L | sim — exige `hayagriva` |
| `Numbering` infra | heading, figure | M | precedente Passo X (verificar) |
| ... | ... | ... | ... |

### 154A.4 — Arqueologia das ausências

Para cada `ausente`, procurar:

```bash
grep -rn "ausente_feature_X" 00_nucleo/materialization/ 00_nucleo/adr/
```

Hipóteses esperadas:

- **Não materializado** porque não estava no caminho crítico
  de DEBT-1.
- **Adiado** por ADR-0017 (estratégia typst-library
  estratificar progressivamente).
- **Bloqueado** por dependência arquitectural.
- **Sem registo** — decisão de facto.

Para cada caso, classificar:
- **Decisão consciente** (referência ADR): permanece ausente
  até decisão revisitada.
- **Adiamento priorizável**: candidato a Fase 1 da ADR-0060.
- **Bloqueio externo** (crate ausente): candidato a Fase 2+
  com ADR de autorização.

### 154A.5 — Crates externas

Para cada elemento ausente, identificar deps externas:

| Elemento | Crates necessárias | Em cache? | Custo licença / autorização |
|----------|---------------------|-----------|-----------------------------|
| `bibliography` | `hayagriva` | sim (per P152) | requer ADR autorização |
| `cite` | (depende de bibliography) | — | — |
| `footnote` | nenhuma específica | — | nenhuma autorização externa |
| `table` | nenhuma específica | — | nenhuma autorização externa |
| ... | ... | ... | ... |

Crates de uso geral (`rustc-hash`, `comemo`, etc) já estão
autorizadas; não contam aqui.

### 154A.6 — Priorização proposta

Matriz **custo** × **valor user-facing**:

```
      Alto valor      Médio valor      Baixo valor
S     [F1: terms]     [F1: pad/hide]   [F2: stroke-obj]
M     [F1: footnote]  [F1: quote]      [F2: stack]
M+    [F1: table]     [F2: columns]    [F3: box/block]
L     [F2: figure-kinds] [F3: outline-improvements] [-]
XL    [F2: bibliography+cite] [-] [-]
```

(F1 = Fase 1; F2 = Fase 2; F3 = adiada/condicional.)

**Fase 1 proposta** (decisão default):

- `terms` (S, alto valor — listas de definições aparecem
  em texto técnico).
- `footnote` (M, alto valor).
- `table` (M+, alto valor — exige `Content::Table` + cell
  layouting).
- `quote` (M, médio valor).
- `pad` / `hide` (S, médio valor).

Aspiração de cobertura post-Fase 1: **60-65%** (8/21 → 13-14/21).

**Fase 2** (proposta):

- `figure` kinds (alto valor, depende de table/equation).
- `columns`, `stack` (médio valor).
- `bibliography + cite` (alto valor mas XL — passo dedicado).

**Fase 3** (condicional):

- `box`, `block` (granularidade fina; talvez via
  `Content::Styled`).
- `stroke-object`, `repeat` (visualize-adjacent).

### 154A.7 — Plano de materialização

ADR-0060 propõe N passos:

1. **154B** — `terms` + `pad` + `hide` (S agregado, ~baseline
   familiar).
2. **155** — `footnote` (M; page model + área reservada).
3. **156** — `quote` (M; estrutural).
4. **157** — `table` + `Content::Table` (M+; cell
   layouting básico).
5. **158** — figure kinds (depende de 157 para table-figure).
6. **ADR-0061 + 159+** — bibliography + cite (XL; autorização
   `hayagriva`).
7. Restantes — Fase 3 condicional.

**Relação com ADR-0026/0026-R1**: cada feature decide entre
`Content::*` variant nova (se semântica é distinta) ou
`Content::Styled` com `Style` específica (se é estrutural).
Diagnóstico fornece recomendação por feature; ADR-0060
formaliza a regra de decisão.

### 154A.8 — Resumo executivo

2-3 parágrafos no diagnóstico:

> Model (structural) tem cobertura 38% (8/21). Diagnóstico
> revela X entradas como Fase 1 viável, Y como Fase 2
> dependente, Z como Fase 3 condicional.
>
> Ataque proposto: 5 passos (154B–158) elevam cobertura
> para ~60-65% sem novas crates. Fase 2 (figure kinds +
> bibliography) exige ADR-0061 para `hayagriva`; passo
> dedicado.
>
> Trabalho restante (~8-10 entradas) é XL ou condicional;
> registado no roadmap mas não obriga a executar.

### 154A.9 — Escrever ADR-0060

Cabeçalho:

```markdown
# ⚖️ ADR-0060: Model (structural) roadmap — Fase 1

**Status**: `PROPOSTO`
**Data**: 2026-04-25
**Validado**: Passo 154A — diagnóstico.
**Diagnóstico prévio**:
[`00_nucleo/diagnosticos/diagnostico-model-passo-154a.md`](...)
```

Estrutura: Contexto, Decisão (5 itens — Fase 1 features,
Fase 2 conditional, Fase 3 deferred, regra Content::Styled
vs variant, número de passos), Alternativas (tabela),
Consequências, Plano de materialização (5 sub-passos),
Referências (ADRs 0017, 0026, 0026-R1, 0033, 0034, 0036,
0038, 0054).

### 154A.10 — Possíveis DEBTs novos

Critério explícito (per Decisão diferida 11):

- Bibliography + Cite → **DEBT-XX (próximo número
  disponível)**: "Bibliography + Cite — XL; exige hayagriva
  + CSL parser". Aberto neste passo. Plano: ADR-0061 +
  passo dedicado em Fase 2.
- Outros candidatos identificados durante 154A.4–6:
  conforme arqueologia.

Se DEBT-34d ou DEBT-34e cobrem `Content::Table`, **não
duplicar**. Referenciar em ADR-0060.

### 154A.11 — Actualizar inventário 148

Em
`00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`:

- Tabela A linha "Model": se diagnóstico revelar
  reclassificação, ajustar contagens.
- Tabela C: adicionar bloqueantes detectados (cells,
  numbering, page model footnote area).
- §7 Top divergências entrada 7: substituir "~14 elementos
  agregados" por referência ao diagnóstico 154A com
  breakdown.

### 154A.12 — Actualizar README dos ADRs

Se ADR-0060 criada (esperado):

- Tabela "Estado por ADR": linha nova ADR-0060 (`PROPOSTO`).
- Distribuição: `PROPOSTO` 10 → 11.
- Total: 59 → 60.
- Passos-chave: entrada P154A.

### 154A.13 — Relatório do passo

Ficheiro:
`00_nucleo/materialization/typst-passo-154a-relatorio.md`.

Secções:
1. Sumário.
2. Inventário detalhado (resumo da Tabela 154A.1).
3. Estado actual cristalino (resumo 154A.2).
4. Bloqueantes arquitecturais (resumo 154A.3).
5. Arqueologia (resumo 154A.4).
6. Crates externas (resumo 154A.5).
7. Priorização (matriz 154A.6).
8. Plano de materialização proposto (referência a ADR-0060).
9. ADR-0060 produzida.
10. DEBTs novos (se houver).
11. Inventário 148 actualizado.
12. README dos ADRs actualizado.
13. Próximo passo: 154B (Fase 1, primeira sub-fase) ou
    decisão humana entre alternativas.
14. Verificação final.

---

## Verificação

1. ✅ Diagnóstico em
   `00_nucleo/diagnosticos/diagnostico-model-passo-154a.md`
   com 8 secções factuais.
2. ✅ ADR-0060 criada em status `PROPOSTO` (cabeçalho
   canónico P145).
3. ✅ Cada elemento Model classificado com referência
   canónica.
4. ✅ Bloqueantes arquitecturais identificados para cada
   `parcial`/`ausente`.
5. ✅ Crates externas listadas por elemento.
6. ✅ Priorização ranqueada (matriz custo × valor).
7. ✅ Plano de materialização com N passos sugeridos.
8. ✅ DEBT-34d / DEBT-34e estado verificado (e referenciado
   se aberto).
9. ✅ Possíveis DEBTs novos abertos consoante critério.
10. ✅ Inventário 148 actualizado se aplicável.
11. ✅ README dos ADRs actualizado.
12. ✅ Nenhum ficheiro em L1/L2/L3/L4 tocado.
13. ✅ Nenhum ficheiro em `lab/parity/` tocado.
14. ✅ `cargo test --workspace --lib`: 1113 inalterado.
15. ✅ `crystalline-lint .`: zero violations.
16. ✅ Relatório do passo escrito.

---

## Critério de conclusão

1. Cada uma das 21 entradas Model classificada com base
   empírica.
2. ADR-0060 propõe roadmap com Fase 1 / Fase 2 / Fase 3.
3. DEBTs novos abertos só consoante critério explícito.
4. Próximo passo (154B = Fase 1 primeira sub-fase) tem
   âncora documental.
5. Sem código tocado.
6. Relatório do passo escrito.

---

## O que pode sair errado

- **Inventário detalhado revela mais de 21 sub-features**:
  esperado. Cada elemento pode ter 5-15 atributos. Documento
  cresce; aceitável (~400-500 linhas).

- **Crates exigidas além de `hayagriva`**: ex, `csl-rs`
  para Citation Style Language. Listar como dep transitiva
  ou directa; se directa, ADR adicional.

- **DEBT-34d / DEBT-34e referidos mas não-existentes**: P142
  pode ter erro factual. Ajustar nota mental + relatório
  154A; documentos passados (P142) ficam imutáveis.

- **Diagnóstico conclui que Model não é prioridade real**:
  ex, footnote depende de page model que não está
  materializado (footnote area é parte da page layout, não
  só content). Roadmap fica condicional a Layout Fase X
  (categoria 38% também). Se isto acontecer, ADR-0060
  proposta inclui ressalva.

- **Surge dependência circular**: ex, `figure` exige
  `Content::Table` que exige `figure` para table-figure.
  Resolver com ordem topológica em 154A.7.

- **Bibliography + cite tem ramificações de scope-out**:
  ex, vanilla integra hayagriva profundamente; cristalino
  pode optar por subset. Se decisão é "subset", ADR-0061
  formaliza scope-out parcial. Mais trabalho de design.

- **Scopes de `#show` revelam dependências de stdlib**:
  algumas regras `show` exigem `Selector` extensions ou
  similar. Documentar; pode forçar passo separado.

- **`Content::Styled` vs variant novo**: regra ADR-0026 pode
  ser ambígua para alguns casos (ex: footnote semântica
  versus structural; é variant ou styled?). Diagnóstico
  recomenda; ADR-0060 formaliza por feature.

- **Volume excede passo único**: aceitável.
  Diagnóstico-primeiro pode crescer para 600+ linhas. Se
  exceder ~800 linhas, considerar dividir em 154A1
  (foundations + heading + figure) + 154A2 (table +
  bibliography + restantes). **Pausar** em 800 linhas e
  consultar.

- **Algum elemento revelar-se já materializado mas
  classificado como `ausente` no inventário 148**:
  classificação corrige-se; inventário 148 ganha
  reclassificação no relatório.

- **`enum`/`list` aparecem como dependência**: list/enum não
  são Model strict mas podem ser referenciados (ex:
  bibliography lista referencias). Registar como dep não
  fazendo parte do escopo Model.

---

## Notas operacionais

- **Modelo: diagnóstico-primeiro aplicado a categoria
  ampla** (não a tipo único como 131A `Lang` ou 132A
  `FontList`). Quinta aplicação; primeira a categoria com
  21 elementos. Padrão escala: o passo cresce
  proporcionalmente ao número de entradas.

- **Reformulação 7 da série paridade efectivamente
  encerrada em P153**. Mudança de prioridade humana é parte
  do método — projectos longos não seguem caminho linear.

- **ADR-0060 é a primeira ADR pós-paridade**. Status
  `PROPOSTO` reservado até primeira sub-fase materializar
  (provavelmente 154B). Modelo análogo a ADR-0055 +
  Passos 140A/140B/141.

- **ADR-0061 reservada** para autorização `hayagriva`
  quando bibliography for atacada. **ADR-0062 reservada** se
  outra crate específica surgir do diagnóstico.

- **Numeração 154A vs 154B+**: depende do plano
  proposto. Se 5 sub-fases, 154B + 155 + 156 + 157 + 158.
  Se trabalho cresce, sub-passos `A`/`B` mantêm-se.
  Numeração final decidida em ADR-0060.

- **DEBTs vs items do roadmap**: critério explícito
  (Decisão diferida 11). Conservador: trabalho de Fase 1
  é roadmap; XL ou condicional vira DEBT.

- **Inventário 148 actualizado se necessário**: precedente
  P149 + P153. Padrão coerente: novos diagnósticos
  refinam inventário com referências cruzadas.

- **Pós-154A**:
  - Documento de diagnóstico.
  - ADR-0060 PROPOSTA.
  - Eventual DEBT-XX (bibliography).
  - Inventário 148 com referências cruzadas.
  - Roadmap claro.
  - **Próximo substantivo**: 154B (Fase 1 primeira sub-fase)
    ou outra prioridade humana.

- **Quarentena vanilla**: continua opção 3 (princípio sem
  regra absoluta) por defeito. Sem mudança neste passo.

- **Série paridade continua suspensa** após P154A. Se
  retomada futuramente, P154 = P4 cristalino-only baseline
  herda a numeração reservada.
