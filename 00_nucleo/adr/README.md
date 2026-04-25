# Índice de ADRs do Typst Cristalino

Este documento é o índice canónico dos Architectural Decision
Records (ADRs) do projecto **Typst Cristalino**. Lista os 61 ADRs
em vigor (60 números únicos; ADR-0026 tem variante -R1 por
revisão), as meta-regras que governam o projecto, o vocabulário
canónico de status, cadeias de revogação e revisão, e convenções
estruturais.

**Reservas de números** (sem ficheiro criado, mas comprometidos):
- **ADR-0062** — autorização da crate `hayagriva` para
  bibliography + cite. Reserva originalmente em ADR-0061;
  deslocada para ADR-0062 em P156B (que reocupou ADR-0061 para
  Layout roadmap). Consumida quando DEBT-55 fechar (P159).
- **ADR-0063** — reservada para outra crate específica se
  surgir (e.g. column flow algorithm pode usar este número se
  ADR dedicada for criada quando DEBT-56 for materializado).

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

7. **Atomização progressiva** — ADR-0036. Funções de L1 declaram
   explicitamente todas as dependências na assinatura. Estado
   partilhado mutável é dívida a reduzir progressivamente.
   Primeiro pagamento concreto: DEBT-44 (integração estrutural
   do `Route<'a>`).

8. **Coesão por domínio** — ADR-0037. Ficheiros em L1 agrupam
   código por domínio conceptual ou técnico. Limite orientativo
   de 800 linhas; excepções Regra 6 documentadas no topo do
   ficheiro. Primeira aplicação concreta: Passos 96.1–96.2
   (reestruturação do `eval.rs`). Trabalho restante: DEBT-46.

9. **Sistema de estilos em L1** — ADR-0038. Toda a representação
   de estilo no domínio passa por `Style` (enum tipado de
   variantes — `Bold`, `Italic`, `Size`, `Fill`, `HeadingLevel`,
   etc.), `Styles(Vec<Style>)` e `StyleChain` (lista ligada
   imutável de deltas). `TextStyle` plano permanece como vista
   achatada para o Layouter (decisão COEX). Materializado nos
   Passos 99–102; estendido pelo DEBT-52 (Passos 136–141).

10. **Critério de fecho integral de DEBTs** — ADR-0054.
    Captura sem consumer não basta para fechar dívida estrutural.
    Aplicável a DEBT-1 explicitamente; precedente para futuros
    DEBTs fundamentais. Adopta o **perfil observacional graded**:
    paridade visível (tamanho, cor, peso, espaçamento) sem
    garantia de shaping features. Cumprida no Passo 142 (DEBT-1
    + DEBT-52 fechados).

---

## Vocabulário canónico de status

O campo `**Status**:` de cada ADR usa um dos seis valores abaixo,
todos com backticks:

| Valor | Semântica | Exemplos |
|-------|-----------|----------|
| `PROPOSTO` | Decisão tomada mas ainda não em vigor nem implementada | ADR-0005, 0006, 0008-0015 |
| `IDEIA` | Direcção a considerar, pode não vir a ser implementada | ADR-0002, 0003 |
| `EM VIGOR` | Regra ou política arquitectural aceite e activa | ADR-0018, 0029, 0030, 0032, 0033, 0034, 0035, 0036, 0037 |
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
| 0035 | `ecow::EcoVec` autorizado em L1 | `EM VIGOR` |
| 0036 | Atomização progressiva — estado partilhado como dívida | `EM VIGOR` |
| 0037 | Coesão por domínio — ficheiros limitados a uma responsabilidade clara | `EM VIGOR` |
| 0038 | Sistema de estilos em L1 (`Style`, `Styles`, `StyleChain`) | `EM VIGOR` |
| 0039 | Forma de estilo no `FrameItem::Text` | `EM VIGOR` |
| 0040 | Activação de `#set` em eval | `EM VIGOR` |
| 0041 | Activação de `#show` — heading, strong, emph | `EM VIGOR` |
| 0042 | `Sink` materializado em L1 | `EM VIGOR` |
| 0043 | Canal de saída do `Sink` (TrackedMut + L3) | `EM VIGOR` |
| 0044 | `Engine<'a>` agregador em L1 | `EM VIGOR` |
| 0045 | Formato de diagnósticos (resolução L1, formatação L3) | `EM VIGOR` |
| 0046 | CLI mínima (compile com diagnostics) | `EM VIGOR` |
| 0047 | Argparsing com `clap` na CLI | `EM VIGOR` |
| 0048 | Cores ANSI nos diagnósticos | `EM VIGOR` |
| 0049 | CLI vive em L2 (correcção de 0046/0047/0048) | `EM VIGOR` |
| 0050 | Formatter de diagnósticos em L2 (completa 0049) | `EM VIGOR` |
| 0051 | Flags funcionais em L2 — pattern e `-o` | `EM VIGOR` |
| 0052 | Lang como tipo semântico em L1 | `IMPLEMENTADO` |
| 0053 | Font como tipo composto em L1 | `IMPLEMENTADO` |
| 0054 | Critério de fecho de DEBT-1 inclui consumo integral | `EM VIGOR` |
| 0055 | Font consumer via pipeline CIDFont existente | `IMPLEMENTADO` |
| 0057 | Lang hyphenation em L1 via crate `hypher` | `IMPLEMENTADO` |
| 0058 | Tipo simplificado — `type()` devolve `Value::Str` | `EM VIGOR` |
| 0059 | `Args` como tipo separado, não-variant de `Value` | `EM VIGOR` |
| 0060 | Model (structural) roadmap — Fase 1 + 2 + 3 | `IMPLEMENTADO` (Fase 1 fechada em P155; Fase 2/3 prosseguem em **P157+** após renumeração registada em P156B) |
| 0061 | Layout Fase X — page model + multi-column + footnote area roadmap | `PROPOSTO` (P156B; reocupou número antes reservado para hayagriva — passou para ADR-0062) |

**Total**: 61 ADRs (60 números únicos; ADR-0026 tem variante -R1
por revisão).

### Distribuição de status

- `PROPOSTO`: **11** ADRs (decisões em aberto: 0005, 0006,
  0008–0015, **0061**).
- `IDEIA`: 2 ADRs (0002, 0003).
- `EM VIGOR`: 26 ADRs (regras/políticas activas; 0018, 0029,
  0030, 0032–0051, 0054, 0058, 0059).
- `IMPLEMENTADO`: 19 ADRs (decisões materializadas; 0001, 0004,
  0016, 0017, 0019, 0021–0027, 0026-R1, 0031, 0052, 0053, 0055,
  0057, **0060**).
- `REVOGADO`: 2 ADRs (0007, 0028).
- `ADIADO`: 1 ADR (0020).

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
vocabulário do P84.8g (realizada em 2026-04-22) e podem conter:

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
- **Passos 99–103 — Fundação de styles tipados em L1**.
  ADR-0038 (Sistema de estilos: `Style`/`Styles`/`StyleChain`),
  ADR-0039 (forma de estilo no `FrameItem::Text`), ADR-0040
  (activação de `#set`), ADR-0041 (activação de `#show` para
  heading/strong/emph). Materializaram a fundação de DEBT-1
  fase A. DEBT-48 encerrado no Passo 100.
- **Passos 104–111 — Engine, Sink, formato de diagnósticos**.
  ADR-0042 (`Sink` materializado em L1), ADR-0043 (canal de
  saída do `Sink` via `TrackedMut`), ADR-0044 (`Engine<'a>`
  como agregador), ADR-0045 (diagnósticos resolvidos em L1,
  formatados em L3). Saída do escopo "MockWorld + helpers".
- **Passos 113–120 — Ciclo de vida da CLI**. ADR-0046 (CLI
  mínima em L4), ADR-0047 (argparsing com `clap`), ADR-0048
  (cores ANSI), ADR-0049 (correcção: CLI vive em L2),
  ADR-0050 (formatter em L2), ADR-0051 (flags funcionais com
  `-o` como pattern). Reorganização final convergiu para L2
  como home da CLI.
- **Passos 131B/132B — Tipos semânticos `Lang` e `FontList`**.
  ADR-0052 (Lang com validação ISO + erro hard) e ADR-0053
  (FontList com paridade parcial: string + array; dict
  deferido por ausência de `regex` em L1). Captura completa do
  `StyleDelta` canónico.
- **Passo 135 — Diagnóstico de shaping; ADR-0054**. Diagnóstico
  detectou que captura sem consumer não cumpre paridade.
  ADR-0054 redefiniu critério de fecho de DEBT-1 (perfil
  observacional graded). DEBT-52 aberto como rastreador.
- **Passos 136–139 — DEBT-52 Fases A e B**. Consumer
  `tracking` (137), `leading` (138), `weight` faux-bold (139).
  `TextStyle` estendido (136). Primeiro efeito visível desde
  o Passo 102 (`fill`).
- **Passo 140A — Diagnóstico font infra; ADR-0055 `PROPOSTO`;
  ADR-0019 anotada**. Inventário revelou infra CIDFont
  embedding pronta. ADR-0055 propõe consumer integral via
  pipeline existente. ADR-0019 ganhou nota factual sobre
  `rustybuzz` declarado-sem-uso (não revogação).
- **Passos 140B + 141 — DEBT-52 Fase C básica**. ADR-0055
  materializada (single-font no Passo 140B; array fallback no
  Passo 141 → ADR-0055 transitou para `IMPLEMENTADO`). Gaps 5
  e 6 do DEBT-52 fechados.
- **Passo 142 — Fecho formal de DEBT-1**. ADR-0054 cumprida:
  9 de 10 campos de `StyleDelta` com consumer activo; `lang`
  em scope-out por perfil observacional graded. DEBT-1 e
  DEBT-52 movidos para Secção 2 do `DEBT.md`. Relatório formal
  em `relatorios/fecho-debt-1-passo-142.md`.
- **Passo 143 — Correcção empírica deste README**. Inventário
  empírico do directório (56 ADRs) revelou gap de 18 ADRs em
  falta na tabela "Estado por ADR" desde P84.8h. Adicionadas
  meta-regras 9 e 10 (ADR-0038 e ADR-0054). Anomalias de
  formato no cabeçalho de ADRs 0038–0051 (Status sem
  backticks ou `**Estado**:` em vez de `**Status**:`)
  registadas como dívida documental no relatório do passo;
  status canónico inferido por leitura conservadora.
- **Passo 145 — Uniformização de cabeçalhos dos ADRs 0038–0051**
  (análogo P84.8g, que cobriu 0001–0037). Em 14 ADRs:
  `**Estado**:` → `**Status**:`; valor com backticks
  (`` `EM VIGOR` ``); informação de validação preservada
  numa nova linha `**Validado**:`. Símbolo `⚖️` e separador
  `:` adicionados aos títulos de 17 ADRs (0017, 0027, 0028
  e 0038–0051). Conteúdo material dos ADRs intacto. Notas
  `¹`/`²` da tabela "Estado por ADR" removidas (a
  irregularidade que sinalizavam ficou resolvida).
- **Passo 144 — Lang hyphenation** (gap 7 do DEBT-52
  reaberto pós-fecho voluntariamente). **ADR-0057** autoriza
  crate `hypher` em L1 (pure-data, no_std, zero deps).
  Consumer integrado no algoritmo greedy de quebra de linha:
  quando palavra não cabe e `style.lang` é `Some(lang)`,
  tenta-se quebra com hífen literal antes do flush. `lang`
  muda de scope-out total para parcialmente consumido —
  hyphenation activo; shaping (rustybuzz) continua ausente.
  Reduz superfície de scope-out de ADR-0054 sem invalidar o
  perfil observacional graded. Numeração 144 vs 145: 144
  estava reservado para hyphenation desde os relatórios 142
  e 143; 145 (uniformização documental) precedeu por
  priorização operacional. Sem conflito.
- **Passo 146 — Multi-font per document**. Materialização
  voluntária da decisão 5 de ADR-0055 (declarada opcional na
  própria ADR; relatório 142 §9 listava como candidata
  futura). `collect_fonts_from_doc` + `resolve_fonts` (silent
  drop) + `export_pdf_multifont` (resource dict `/F1..N`
  apontando para `/CrystallineFont1..N`). Cada
  `FrameItem::Text` selecciona `/F{i+1}` por match
  estrutural contra a sua `style.font`. Single-font
  preservado por dispatch (`[(_, b)] => export_pdf_with_font`
  mantém o caminho 140B/141). ADR-0055 **anotada
  pós-IMPLEMENTADO** (modelo ADR-0019 + 140A); status
  inalterado. Numeração 146 (não 142A) registada no
  relatório do passo. Test do 140B
  `font_wiring_segunda_font_diferente_primeira_vence`
  renomeado para
  `font_wiring_segunda_font_diferente_ambas_embebidas` com
  assertion ajustada de 1 → 2 `/Subtype /Type0` (regressão
  deliberada do MVP 140B documentada).
- **Passos 147–148 — Série paridade (actualização + inventário)**.
  P147 actualizou documentos de paridade (`typst-paridade-*.md`)
  para reflectir o estado de 2026-04-24 pós-P146 (estavam
  ~127 passos desactualizados). P148 produziu **inventário
  factual de cobertura vanilla vs cristalino** em
  `diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`
  (~138 user-facing + ~105 arquitectural; 5 classes:
  implementado/implementado⁺/parcial/ausente/scope-out).
  Cobertura declarada: 54% user-facing; 70% arquitectural
  (pré-149). Reformulação da série paridade: inventário
  antes de medição.
- **Passo 149 — Arqueologia e formalização de divergências
  `Value::Type` e `Value::Args`**. Inventário 148 §7
  sinalizou 2 divergências não-formalizadas. Arqueologia em
  `diagnosticos/divergencias-value-passo-149.md` localizou
  Passos materializadores (13 e 16) e razões (ADR-0017 +
  ADR-0036 espírito). Classificação: **ADR-0058** (`type()`
  devolve `Value::Str`, não `Value::Type(Type)` rico) e
  **ADR-0059** (`Args` como tipo separado em
  `entities/args.rs`, passado como `&Args` às nativas, não
  variant de `Value`). Zero DEBTs novos; zero código tocado.
  Inventário 148 actualizado: Tabela B reclassificou
  `Value::Type` e `Value::Args` de `parcial` para
  `implementado⁺`. Cobertura arquitectural pós-149: 72%.
- **Passo 154A — Diagnóstico Model + ADR-0060 PROPOSTO**.
  Quinto exemplo do padrão diagnóstico-primeiro
  (131A/132A/140A/148/154A); primeira aplicação a categoria
  user-facing ampla. Inventário detalhado dos 22 elementos
  Model em `lab/typst-original/.../typst-library/src/model/`
  revelou cobertura empírica **32-36%** (revisão para baixo
  do 38% declarado). Cristalino tem 7-8 materializados, 5
  parciais, 10 ausentes. **ADR-0060 PROPOSTO** com roadmap
  Fase 1 (terms/divider/quote — passos 154B/155), Fase 2
  (table foundations/figure kinds/bibliography+cite —
  passos 156/157/158), Fase 3 (asset/document/title —
  condicional). **DEBT-55 aberto** para
  `bibliography`+`cite` (XL; bloqueado por ADR-0061 a
  criar para autorizar `hayagriva`). Inventário 148
  actualizado (Tabela A linha "Model" recontada para 22
  entradas; cobertura user-facing total 54% → 53%). Série
  paridade fica suspensa em P153 enquanto Fase 1 do roadmap
  Model arranca.
- **Passo 154B — Fase 1 Model (sub-passo 1: terms + divider)**.
  Primeira materialização do roadmap ADR-0060: `Content::Divider`,
  `Content::Terms { items: Vec<Content> }`, `Content::TermItem
  { term, description }` adicionados ao enum (39 → 42 variants);
  `native_terms` e `native_divider` registadas em `make_stdlib`
  (forma `#terms(key: [desc], ...)` e `#divider()`). Cobertura
  exaustiva de arms `match` em ~7 sítios L1 (`plain_text`,
  `is_empty`, `PartialEq::eq`, `map_content`, `map_text` em
  `entities/content.rs`; `materialize_time`, `walk` em
  `rules/introspect.rs`; `layout_content` em `rules/layout/mod.rs`).
  L0 prompt `entities/content.md` ganhou secção dedicada;
  hash propagado via `--fix-hashes` (`85fae9b9` →
  `43745b5d`). ADR-0060 anotada com nota de progresso —
  **status `PROPOSTO` preservado** (Fase 1 fecha após P155
  = `quote`). Inventário 148 actualizado: Tabela A Model
  3/4/5/10/0 → 5/4/5/8/0 (cobertura **32-36% → 41%**);
  Tabela B Content cristalino 39 → 42 variants; vanilla
  extra ausentes ~14 → ~12. Sem ADR nova; sem DEBT criado
  ou fechado; sem regressão em 1113 → 1123 testes
  (+5 unit content.rs + +3 eval + +2 implícitos). Padrão
  diagnóstico-primeiro (P154A) → materialização (P154B)
  replica precedentes 131A→131B, 132A→132B, 140A→140B.
- **Passo 156E — Layout Fase 1 sub-passo 3: pagebreak manual**
  (terceira aplicação consecutiva de **ADR-0061**; **halfway
  point Fase 1** atingido — 50% cobertura Layout). Substantivo
  S+ agregado: `Content::Pagebreak { weak: bool, to:
  Option<Parity> }` adicionado ao enum (47 → 48 variants);
  tipo `Parity { Even, Odd }` novo em
  `01_core/src/entities/parity.rs` (infraestrutura genérica
  análoga a `Sides<T>` de P156C; reuso futuro previsível em
  refino Page rico); stdlib `#pagebreak(weak: false, to: ?)`
  (sem posicionais; `to` aceita `"even"`/`"odd"` via helper
  `extract_parity`; named arg desconhecido + weak não-bool
  + posicional + `to` inválido rejeitados). Cobertura
  exaustiva de arms em `Content` (is_empty `false`, plain_text
  vazio, PartialEq 2-fields, map_* terminais), `introspect.rs`
  (no-op), `layout/mod.rs::layout_content` (flush_line + reusa
  `Layouter::new_page` de `cursor.rs:128`; se `to: Some(parity)`
  não bate `pages.len()+1`, insere `new_page()` extra para
  ajustar paridade). Layouter scope-outs declarados: `weak`
  collapse adiado (atributo armazenado); página vazia inserida
  não tem header/footer (Page actual não tem); `to` aceita só
  string em stdlib (não Symbol). Tests: 1192 → **1214**
  (+22 = 3 unit parity + 5 unit content + 10 stdlib + 4
  layout E2E). Cobertura Layout: 44% → **50%** (8/18 → 9/18);
  total user-facing: 56% → **57%**. **ADR-0061 mantém-se
  `PROPOSTO`** (anotação cumulativa após Fase 1 completa).
  README ADRs: total e distribuição inalterados (61 ADRs;
  PROPOSTO 11). L0 `entities/parity.md` criado; secção
  pagebreak adicionada a `entities/content.md`; hashes
  propagados (`content.rs` → `b632e841`, `parity.rs` →
  `af8490cb`).
- **Passo 156D — Layout Fase 1 sub-passo 2: h + v spacing**
  (segunda aplicação consecutiva de **ADR-0061**, modelo
  granular confirmado). Substantivo S agregado:
  `Content::HSpace { amount: Length, weak: bool }` e
  `Content::VSpace { ... }` adicionados ao enum (45 → 47
  variants); stdlib `#h(amount, weak: false)` e
  `#v(amount, weak: false)` (amount aceita Length/Float-pt/
  Int-pt; amount negativo rejeitado per perfil ADR-0054
  graded; named arg desconhecido + weak não-bool rejeitados;
  Fraction scope-out per ADR-0061 §6.3). Cobertura exaustiva
  de arms em `Content` (is_empty proxy zero, plain_text
  vazio, PartialEq 2-fields, map_content/map_text terminais),
  `introspect.rs` (no-op em walk; preserve em
  materialize_time), `layout/mod.rs` (HSpace avança cursor.x;
  VSpace força flush_line + avança cursor.y). Layouter
  scope-outs declarados: `weak` collapse adiado (atributo
  armazenado mas semantic não implementada); h fora-de-largura
  não força wrap; v no início de página não colapsa contra
  margem. Helper `build_spacing` partilhado por h/v evita
  duplicação. Tests: 1172 → **1192** (+20 = 7 unit content +
  11 stdlib + 2 layout E2E). Cobertura Layout: 33% → **44%**
  (6/18 → 8/18); total user-facing: 55% → **56%**.
  **ADR-0061 mantém-se `PROPOSTO`** (anotação cumulativa após
  Fase 1 completa, per decisão humana). README ADRs: total e
  distribuição inalterados (61 ADRs; PROPOSTO 11). L0
  `entities/content.md` ganha secção HSpace+VSpace; hash
  propagado (`content.rs` → `4a7e0a99`).
- **Passo 156C — Layout Fase 1 sub-passo 1: pad + hide**
  (primeira aplicação concreta de **ADR-0061**). Substantivo
  S agregado: `Content::Pad { body, padding: Sides<Length> }`
  e `Content::Hide { body }` adicionados ao enum
  (43 → 45 variants); `Sides<T>` genérico criado em
  `01_core/src/entities/sides.rs` (reusable por `PageConfig`
  refino futuro, `Block`, `Box`); stdlib `#pad(body, left:?,
  right:?, top:?, bottom:?, x:?, y:?, rest:?)` (precedência
  específico>eixo>rest; padding negativo rejeitado per perfil
  ADR-0054 graded) e `#hide(body)`. Cobertura exaustiva de
  arms em `Content` (is_empty, plain_text, PartialEq,
  map_content, map_text), `introspect.rs` (materialize_time,
  walk), `layout/mod.rs` (layout_content, measure_content_constrained).
  Layouter: pad reserva top/left + flush + bottom; right é
  scope-out (refino com refactor multi-region — DEBT-56 + Fase
  3); hide drena items para buffer e descarta após body
  (preserva avanço de cursor). Tests: 1145 → **1172**
  (+27 = 8 unit content + 4 unit sides + 13 stdlib + 2 layout
  E2E). Cobertura Layout: 22% → **33%** (4/18 → 6/18); total
  user-facing: 53% → **55%**. **ADR-0061 mantém-se `PROPOSTO`**
  (anotação cumulativa após Fase 1 completa, per decisão humana
  2026-04-25). README ADRs: total e distribuição inalterados
  (61 ADRs; PROPOSTO 11). L0 `entities/sides.md` criado;
  `entities/content.md` ganha secção pad+hide; hashes
  propagados (`content.rs` → `daf00164`, `sides.rs` → `c47b14e6`).
- **Passo 156A — Historiograma do projecto**. Passo
  L0-puro / administrativo (sétima aplicação do padrão
  diagnóstico-primeiro). Zero ADRs criadas. Output material:
  `00_nucleo/diagnosticos/historiograma-passos.md` (linha
  temporal + padrões + análise + conclusões metodológicas dos
  155+ passos) +
  `00_nucleo/diagnosticos/ideias-projecto-blueprint-tool.md`
  (ferramenta automática esboçada, não-comprometida).
  Confirmou empiricamente padrão diagnóstico-primeiro com
  retorno alto (6/6 aplicações descobriram informação
  relevante).
- **Passo 156B — Diagnóstico Layout (Fase X)**. Oitava
  aplicação do padrão diagnóstico-primeiro; primeira a categoria
  Layout. **ADR-0061 criada** em status `PROPOSTO` (Layout
  roadmap — page model + multi-column + footnote area).
  **Reocupação**: ADR-0061 estava reservada para `hayagriva`
  (per blueprint pré-P156A); reocupada por Layout. **Reserva
  hayagriva passa para ADR-0062** (sem ficheiro criado).
  **Renumeração de Fase 2 Model** (anotada em ADR-0060):
  P156→P157 table, P157→P158 figure-kinds, P158→P159
  bibliography. **DEBT-56 aberto** (column flow Fase 3 Layout
  L+; refactor multi-region do Layouter exigido). DEBT-55
  actualizada para reflectir P159 + ADR-0062. Cobertura Layout
  recalculada empiricamente: 38% declarado → **22% implementado
  puro** (4/18; +2 entradas adicionadas — `h`/`v` e `skew`).
  Distribuição ADRs: `PROPOSTO` 10→11; `IMPLEMENTADO` 19→19;
  total 60→61. Sem código tocado.
- **Passo 155 — Fase 1 Model fechada (sub-passo 2: quote +
  smart-quotes)**. Segunda e última materialização da Fase 1
  do roadmap ADR-0060: `Content::Quote { body, attribution,
  block, quotes }` (4 atributos vanilla `QuoteElem`) adicionado
  ao enum (42 → 43 variants); `native_quote` registado em
  `make_stdlib` (`#quote(body, attribution: ?, block: ?,
  quotes: ?)`). **Módulo novo `01_core/src/rules/lang/quotes.rs`**
  expondo `localize_quotes(lang) → (open, close)` para 6 idiomas
  (`pt`/`en`/`de`/`fr`/`es`/`it`) + default ASCII (precedente
  `localize_*(lang)` para futuras features lang-aware; ADR-0057
  hyphenation continua em `rules/layout/`). `eval_markup`
  actualizado para tratar `SyntaxKind::SmartQuote` via alternância
  open/close por sequência markup, emitindo glyph localizado
  como `Content::Text`. **Distinção contextual code vs markup
  preservada** (regression test obrigatório:
  `eval_markup_aspas_em_codigo_continua_string_literal_regression`
  — `"..."` em código continua a ser `Value::Str`). Cobertura
  exaustiva de arms (~7 sítios L1: `plain_text`, `is_empty`,
  `PartialEq::eq`, `map_content`, `map_text` em
  `entities/content.rs`; `materialize_time`, `walk` em
  `rules/introspect.rs`; `layout_content` em
  `rules/layout/mod.rs` — block + inline + smart-quote insertion).
  **ADR-0060 transita `PROPOSTO → IMPLEMENTADO`** (Fase 1
  fechada). Distribuição ADRs: `PROPOSTO` 11→10, `IMPLEMENTADO`
  18→19. Inventário 148 actualizado: Tabela A Model 5/4/5/8/0
  → 6/4/5/7/0 (cobertura **41% → 45%**); Tabela B Content
  42 → 43 variants; vanilla extra ausentes ~12 → ~11.
  L0 prompts: `entities/content.md` ganhou secção Quote;
  novo `prompts/rules/lang.md` regista smart-quotes mecanismo.
  Hashes propagados via `--fix-hashes`: `content.rs`
  `43745b5d → 8413bb8d`; novos `lang/mod.rs` e `lang/quotes.rs`
  partilham `4426dbc0`. Sem ADR nova; sem DEBT tocado.
  Tests: 1123 → 1145 (+22 = 7 lang/quotes + 7 unit content +
  8 eval/parse). Cobertura crista do enum Content
  pós-P155: 31/43 = 72% (mais 9 implementado⁺ + 3 parcial).
  Próxima fase (Fase 2): P156 = table foundations; P157 =
  figure kinds; ADR-0061 + P158 = bibliography + cite (XL,
  bloqueado por DEBT-55).
