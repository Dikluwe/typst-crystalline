# Índice de ADRs do Typst Cristalino

Este documento é o índice canónico dos Architectural Decision
Records (ADRs) do projecto **Typst Cristalino**. Lista os 63 ADRs
em vigor (62 números únicos; ADR-0026 tem variante -R1 por
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

11. **Tradução `Smart<T>` vanilla → `Option<T>`/default** —
    ADR-0064. Quatro casos canónicos (A: `Smart<T>` contextual
    → `Option<T>`; B: `Smart<T>` literal fixo → `T` directo
    com `Default`; C: campo `T` com default não-`Default` →
    `Option<T>`; D: `bool` com default não-`false` → `bool`
    directo). N=6 aplicações empíricas em P156D/E/G/H/I/J;
    formaliza padrão emergente da série Layout P156C-J.

12. **Inventariar primeiro — sub-passo `.1`** — ADR-0065.
    Generaliza ADR-0034: passos com decisão arquitectural
    não-trivial têm sub-passo `.1` dedicado a inventário
    pré-decisão. Critério "não-trivial" cobre naming, escolha
    de tipo, expansão de variant, atravessamento de camadas,
    scope, divergência da spec. N=5 aplicações empíricas em
    P156C/D/G/H/J; zero reformulações mid-passo.

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
| 0064 | Tradução `Smart<T>` vanilla → `Option<T>`/default | `EM VIGOR` (P156K; formaliza padrão N=6 da série P156C-J) |
| 0065 | Inventariar primeiro — sub-passo `.1` para decisão arquitectural não-trivial | `EM VIGOR` (P156K; estende ADR-0034; padrão N=5 da série P156C-J) |

**Total**: 63 ADRs (62 números únicos; ADR-0026 tem variante -R1
por revisão).

### Distribuição de status

- `PROPOSTO`: **11** ADRs (decisões em aberto: 0005, 0006,
  0008–0015, **0061**).
- `IDEIA`: 2 ADRs (0002, 0003).
- `EM VIGOR`: **28** ADRs (regras/políticas activas; 0018, 0029,
  0030, 0032–0051, 0054, 0058, 0059, **0064, 0065**).
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
- **Passo 159A — Bibliography + Cite par acoplado minimal
  (Model bibliography + cite sub-passo 1)** (décima quarta
  aplicação consecutiva de materialização desde início da
  série granular P156C; primeiro sub-passo Bibliography + Cite
  per scope decidido em diagnóstico P159 §3.5 — Estrutura A
  adaptada). **Primeira aplicação isolada concreta de ADR-0065
  critério #2** (escolha de tipo) — decisão de `BibEntry` 4
  fields minimais (key/author/title/year) é decisão arquitectural-
  chave registada. **ADR-0064 Caso A patamar cresce N=4 → 5**
  com diversidade cross-domínio reforçada (60% Layout + 40%
  Model). Substantivo M+ par funcional acoplado
  (granularidade quebrada N=13 → M+ honestamente registada com
  precedente P156C par lógico pad+hide): tipo entity novo
  `BibEntry { key, author, title, year }` em
  `01_core/src/entities/bib_entry.rs` (ficheiro novo per padrão
  P156C `sides.rs`); variants `Content::Bibliography { entries:
  Vec<BibEntry>, title: Option<Box<Content>> }` + `Content::Cite
  { key: String, supplement: Option<Box<Content>> }` adicionados
  ao enum (56 → 58 variants); stdlib `#bibliography(entries,
  title: ?)` + `#cite(key, supplement: ?)` em
  `stdlib/structural.rs` (continuação Model per P157A).
  **Naming `bibliography` e `cite` flat** (paridade decisão
  P157B). **Helper privado novo `extract_bib_entries`**
  parseia `Value::Array<Value::Dict>` para `Vec<BibEntry>`
  com validação hard de 4 fields obrigatórios.
  **Tradução ADR-0064 Caso A**: `title: Smart<Option<Content>>`
  vanilla → `Option<Box<Content>>` cristalino; `supplement`
  similar (Option<Content> vanilla passa directo). **Limitações
  aceites per ADR-0054 graded**: **sem hayagriva** (input
  cristalino literal `Vec<BibEntry>`; ADR-0062 mantém-se reserva
  sem ficheiro); **sem CSL parsing/styles** (placeholder render);
  **sem form variants** Normal/Prose/etc.; **sem validação
  cross-reference** `Cite.key ∈ Bibliography.keys` (ADR-0017
  Introspection runtime adiada — `cite("inexistente")` produz
  placeholder sem erro); **sem numbering schemes** dinâmicos.
  Layouter renderiza placeholder: Bibliography como title (se
  Some) + lista de entries formatadas como
  `"[{key}] {author}. {title} ({year})."` per linha; Cite
  como `"[{key}]"` + supplement (se Some). Cobertura exaustiva
  de **9 sítios pattern-match estruturais** (paridade P157A/B/C/
  P159A): variants + construtores + is_empty + plain_text +
  PartialEq + map_content + map_text + materialize_time + walk
  + layout arms. Validação: bib entry sem field obrigatório
  rejeitada; year negativo rejeitado; named arg desconhecido
  rejeitado em ambas funcs (mensagem inclui menção de
  scope-out per ADR-0054 graded); cite() sem key ou key vazia
  rejeitada. Tests: 1147 → **1174** typst-core lib (+27 = 11
  unit + 13 stdlib + 3 E2E layout; range esperado +18-21
  ultrapassado por par acoplado + tipo entity novo).
  Cobertura Model agregada **inalterada** (50%) — entradas
  `cite` e `bibliography` movem `ausente → parcial` (cobertura
  ampla impl+impl⁺+parcial: 22 → 24); cobertura arquitectural
  **80% → 82%** (2 variants novos; 56 → 58). Total user-facing:
  ~61.0% (inalterada agregada impl+impl⁺). Tabela B Content
  variants: 56 → **58**. **ADR-0060 mantém-se `IMPLEMENTADO`**
  (anotação P159A adicionada). **ADR-0061 mantém-se
  `PROPOSTO`**. **ADR-0062 mantém-se reserva sem ficheiro**
  (não promovida; subset minimal contornou hayagriva). README
  ADRs: total **63 inalterado**. Hash `entities/content.rs`
  mantém-se `ec58d849` — **nono passo consecutivo** (P156L →
  P159A); reinterpretação: hash refere-se ao prompt L0 (não
  ao ficheiro código); P159A adicionou variants ao código mas
  o prompt L0 `content.md` permanece inalterado. **DEBT-55
  contribuído mas NÃO fechado** — refinos futuros (hayagriva,
  CSL, form, numbering, cross-document refs) **NÃO reservados**
  per política P158. **Padrões pós-P159A**: granularidade
  **N=14** (cross-domínio reforçado mas com primeira quebra
  honestamente registada — M+ par acoplado); inventariar
  primeiro N=13 → **14** (primeira aplicação isolada concreta
  critério #2 de ADR-0065); Smart→Option N=9 → **10** (Caso A
  patamar cresce N=4 → 5); §análise de risco N=13 → **14**
  (primeiro M+ par acoplado pós-P156C); helper privado
  `extract_bib_entries` N=1 (sem promoção). **Política "sem
  novas reservas" preservada** (P158 estabeleceu; P159A
  respeita).
- **Passo 159 — Diagnóstico Bibliography + Cite**
  (passo arquitectural de diagnóstico; **não materializa código**;
  análogo estrutural a P156B/P156K/P157/P158). **Terceira
  aplicação concreta de ADR-0065 critério #5** (scope determinado
  por inventário) após P157 e P158 — auto-validação cumulativa
  do ADR meta P156K com diversidade cross-feature confirmada
  (P157 multi-passo divisão; P158 subset selection; P159 par
  acoplado). Diagnóstico em
  `00_nucleo/diagnosticos/diagnostico-bibliography-cite-passo-159.md`
  com 5 secções: §1 ADRs lidas e resumidas (ADR-0060 declara
  P159 XL com `hayagriva` autorizada via ADR-0062 reservada
  mas **NÃO criada como ficheiro**; ADR-0017 Introspection
  runtime adiada não bloqueia subset minimal — Cite resolve
  walk single-pass; precedentes ADR-0024/0023/0057 cobrem
  autorização de crate externa); §2 estado factual em código
  (Bibliography + Cite **completamente ausentes** — zero
  matches grep; DEBT-55 documenta plano completo XL com
  hayagriva; vanilla bibliography.rs 1226 linhas integra
  hayagriva profundamente — `Bibliography` interno usa
  `Arc<ManuallyHash<IndexMap<Label, hayagriva::Entry>>>`); §3
  scope com avaliação de **3 estruturas**: A multi-passo
  análogo P157 (P159A bibliography + P159B cite + P159C
  hayagriva); B minimal análogo P158 (par num único M+ sem
  hayagriva); C diferimento total per ADR-0054 graded;
  **recomendação Estrutura A adaptada** (par acoplado
  Bibliography+Cite num único passo M+ sem hayagriva — input
  cristalino literal Vec<BibEntry>; refinos futuros
  hayagriva/CSL/form/numbering NÃO reservados); §4 dependências
  (zero bloqueios hard para subset minimal; hayagriva
  contornada com input literal; ADR-0017 não bloqueia walk
  single-pass; ADR-0062 confirmada como reserva sem ficheiro;
  promoção a IMPLEMENTADO só necessária se P159B+ futuros
  integrarem hayagriva); §5 esboço de **P159A** (tipo `BibEntry`
  novo em `entities/bib_entry.rs` + 2 variants Content
  `Bibliography` + `Cite` + 2 stdlib funcs `native_bibliography`
  + `native_cite` + layout arms + introspect arms; M+ par
  funcional acoplado análogo a P156C par lógico pad+hide;
  granularidade quebrada N=13 → M+ honestamente registada com
  precedente; tests ~15-20; **quebra esperada padrão
  "preservação hash content.rs"** após 7-8 passos consecutivos
  — variants novos serão adicionados ao enum em P159A).
  **Política "sem novas reservas"** preservada (P158 estabeleceu;
  P158A respeitou; P159 respeita) — refinos pós-P159A
  (hayagriva, CSL, form variants, numbering schemes,
  cross-document forward refs) permanecem candidatos
  **NÃO-reservados**. Padrões pós-P159: granularidade N=13
  (inalterada — diagnóstico); inventariar primeiro N=12 →
  **13** (terceira aplicação concreta critério #5; diversidade
  cross-feature: table multi-passo + figure subset + bibliography
  par acoplado); §análise de risco N=12 → **13** (passo
  diagnóstico baixo risco; XL declarado torna inventário
  particularmente importante). Total ADRs **63 inalterado**
  (ADR-0062 continua reserva sem ficheiro); sem novas crates;
  sem novos DEBTs; sem alteração de hashes (passo documental;
  `entities/content.rs` mantém `ec58d849` — **oitavo passo
  consecutivo**); cobertura Layout/Model/arquitectural
  inalteradas (78%/50%/80%).
- **Passo 158A — Model figure-kinds sub-passo 1: auto-detecção
  de `kind` em `native_figure`** (décima terceira aplicação
  consecutiva de materialização desde início da série granular
  P156C; primeiro sub-passo Model figure-kinds per scope decidido
  em diagnóstico P158 §3.2 — subset minimal). Refino qualitativo
  de infraestrutura existente (`Content::Figure` e counters por
  kind já funcionais desde P75/ADR-0041). Substantivo S+ /
  M-: helper privado novo `infer_kind_from_body(body: &Content)
  -> Option<String>` em `stdlib/figure_image.rs` (~10 linhas)
  cobrindo Image/Table/Raw + **recursão limitada a
  `Content::Sequence`** per decisão P158A §8 (paridade vanilla
  parcial per ADR-0033 — vanilla usa `query_first_naive`
  recursivo profundo; cristalino limita a Sequence; outros
  containers Block/Box/Pad/Styled scope-out per ADR-0054 graded).
  Modificação trivial em `native_figure` para fallback chain
  3 níveis: `kind:` explícito > inferência > default `"image"`
  (precedência absoluta para `kind:` explícito preserva tests
  pré-existentes). **Sem alteração ao variant `Content::Figure`**
  (estrutura inalterada; `kind: String` continua directo —
  refactor para `Option<String>` per ADR-0064 Caso A NÃO
  reservado per política P158). **Sem alteração a `introspect.rs`
  ou layout** — counters por kind continuam funcionar inalterados;
  refino vive só na origem do valor `kind` antes de o passar
  ao variant. **ADR-0064 NÃO directamente aplicável** em P158A
  (kind continua String directo); aplicação futura potencial
  em refactor não reservado. **ADR-0065 critério #1 (naming
  `infer_kind_from_body`) + critério #5 (scope) implícitos**.
  Validação: `kind:` explícito vence (precedência absoluta);
  Image/Table/Raw direct detectam correctamente; Sequence
  recurse no primeiro child detectável; body Text fallback
  para `"image"` default. Tests: 1141 → **1147** typst-core
  lib (+6 = 5 auto-detect por kind + 1 Sequence handling;
  range esperado +6-8). **Política "sem novas reservas" preservada**
  (P158 estabeleceu; P158A respeita) — supplement automático
  por lang, show selectors `figure.where(kind:)`, refactor
  `kind: String → Option<String>` permanecem candidatos
  **NÃO-reservados**. Cobertura Model agregada **inalterada**
  (~50%) — refino qualitativo. Total user-facing: ~61.0%
  (inalterada). Tabela B Content variants: 56 (inalterada).
  **ADR-0060 mantém-se `IMPLEMENTADO`** (anotação P158A
  adicionada). **ADR-0061 mantém-se `PROPOSTO`**. README ADRs:
  total **63 inalterado**; reservas P159/ADR-0062 mantidas
  (não reforçadas). **Hash `entities/content.rs` mantém-se
  `ec58d849`** — **sétimo passo consecutivo** (P156L → P157 →
  P157A → P157B → P157C → P158 → P158A) sem alteração ao
  variant Content. Padrão "passos aditivos / refino sem
  alteração de variant Content" estabilizado. **Padrões
  pós-P158A**: granularidade **N=13** (cross-domínio fortalecido
  com refino Model); inventariar primeiro N=11 → **12** (P158A
  reforça critério #5 com decisão Sequence handling); §análise
  de risco N=11 → **12** (primeiro passo Model com refino
  comportamental sem alteração estrutural — risco muito baixo);
  Smart→Option N=9 (inalterado — não aplicável directamente em
  P158A); helpers `extract_*` reusos inalterados. **Helper novo
  `infer_kind_from_body` N=1** (sem candidato a reuso até agora;
  promoção diferida per política consistente N=3-4).
- **Passo 158 — Diagnóstico Model figure-kinds**
  (passo arquitectural de diagnóstico; **não materializa código**;
  análogo estrutural a P156B/P156K/P157). **Segunda aplicação
  concreta de ADR-0065 critério #5** (scope determinado por
  inventário) — auto-validação cumulativa do ADR meta P156K
  (P157 foi a primeira aplicação concreta; P158 reforça padrão).
  Diagnóstico em
  `00_nucleo/diagnosticos/diagnostico-model-figure-kinds-passo-158.md`
  com 5 secções: §1 ADR-0060 sobre figure-kinds (declarado em
  §"Decisão 2" Fase 2 como "extension" M; sem detalhes concretos
  na ADR — scope decidido por inventário); §2 estado factual em
  código (`Content::Figure { body, caption, kind: String,
  numbering: Option<String> }` já existe completo desde P75/
  ADR-0041; field `kind: String` arbitrário aceita image/table/
  raw/custom; counters por kind funcionam em
  `introspect.rs:279-292`; toda infraestrutura — Image P71, Table
  P157A, Raw P156C — já existe); §3 scope determinado (3 subsets
  avaliados — minimal §3.2 auto-detecção apenas + máximo §3.3
  auto-detecção + supplement + intermédio §3.4; **recomendação
  subset minimal §3.2 — auto-detecção de kind baseada no body**
  preservando granularidade N=13); §4 dependências bloqueantes
  (zero hard; toda infraestrutura existe; ADR-0017 Introspection
  runtime adiada não bloqueia — counters resolvem em walk
  single-pass); §5 esboço de **P158A** (helper privado
  `infer_kind_from_body(body: &Content) -> Option<String>` em
  `stdlib/figure_image.rs`; modificação trivial de
  `native_figure` para fallback `kind explícito > infer > "image"`;
  S+/M-; sem alteração a variant ou layout; tests ~6-8;
  granularidade preservada N=13). **Política nova "sem novas
  reservas"** explícita: P158 NÃO cria reservas para passos
  pós-P158A. Reservas pré-existentes (P159 = bibliography + cite;
  ADR-0062 = hayagriva) respeitadas mas não reforçadas;
  candidatos a refino futuro (supplement automático, show
  selectors `figure.where(kind:)`, refactor `kind: String →
  Option<String>` per ADR-0064 Caso A) **NÃO reservados** —
  decisões ficam para sessões futuras com informação acumulada.
  Padrões pós-P158: granularidade N=12 (inalterada — P158 é
  diagnóstico, não materialização); inventariar primeiro N=10
  → **11** (segunda aplicação concreta critério #5); §análise
  de risco N=10 → **11** (passo diagnóstico baixo risco;
  estabelece precedente "sem novas reservas"). Total ADRs **63
  inalterado**; sem novas crates; sem novos DEBTs; sem
  alteração de hashes (passo documental); cobertura Layout/
  Model/arquitectural inalteradas (78%/50%/80%).
- **Passo 157C — Model Fase 2 sub-passo 3: table header + footer
  (fecha "table foundations")** (décima segunda aplicação
  consecutiva de materialização desde início da série granular
  P156C; **terceiro e último sub-passo Model Fase 2** per
  ADR-0060 §"Decisão 1" sub-passo 3). **Primeira aplicação
  concreta de ADR-0064 Caso D em domínio Model** — Casos A/B/C/D
  agora todos validados em Layout E em Model (**saturação
  cross-domínio cross-caso atingida**; ADR meta P156K atinge
  maturidade empírica). Substantivo M com **par simétrico**:
  variants `Content::TableHeader { body, repeat: bool }` +
  `Content::TableFooter { body, repeat: bool }` adicionados ao
  enum (54 → 56 variants; +2 par); stdlib `#table_header(body,
  repeat: true)` + `#table_footer(body, repeat: true)` em
  `stdlib/structural.rs` (continuação P157A/B). **Naming
  `table_header`/`table_footer` flat** (paridade decisão P157B
  — FieldAccess actual cristalino não suporta namespacing de
  funcs `Value::Func.subname`). **Tradução ADR-0064 Caso D**:
  `bool` directo com default `true` (paridade vanilla
  `#[default(true)]`; divergência intencional do default Rust
  `bool::default() == false`). **Limitação aceite per ADR-0054
  graded**: `repeat` armazenado mas **ignorado em layout** —
  algoritmo de repetição em page breaks diferido em **DEBT-56**
  (refactor multi-region; column flow + header/footer repeat;
  permanece aberto). **Divergência aceite per ADR-0033**:
  `body: Box<Content>` em vez de vanilla `#[variadic] children:
  Vec<TableItem>` para uniformidade com containers cristalinos
  existentes; `level: NonZeroU32` (Header hierarquia),
  `repeat-rows: Smart<usize>` scope-out per ADR-0054 graded.
  Helper privado novo `extract_bool_with_default(args, fn,
  field, default: bool)` em `stdlib/structural.rs`
  parametrizado (N=2 usos imediatos no mesmo passo —
  TableHeader.repeat + TableFooter.repeat; subpadrão emergente
  análogo a `extract_usize_or_none_min` em P157B). Distinção
  vs `extract_weak` (em `stdlib/layout.rs`): genérico no key e
  no default — preserva separação de domínios per ADR-0037.
  Cobertura exaustiva de **9 sítios pattern-match estruturais
  com par simétrico em entradas adjacentes** (paridade
  visualmente óbvia em todos os arms). Validação simétrica:
  body required em ambos; repeat=Int rejeitado em ambos; named
  arg desconhecido rejeitado em ambos; defaults vanilla
  preservados. Tests: 1115 → **1141** typst-core lib (+26 = 12
  unit + 8 stdlib + 3 E2E + 1 integrativo Table+Header+Cell+Footer;
  range esperado +18-23 ultrapassado por par simétrico que
  duplica naturalmente). **"Table foundations" declarado em
  ADR-0060 fica integralmente fechado** com P157A + P157B +
  P157C (3 sub-passos M cada; granularidade preservada
  N=10/11/12). Cobertura Model agregada **inalterada** (50%);
  cobertura arquitectural **78% → 80%** (variants Content
  vanilla extra ausentes desce de ~1 a 0). Total user-facing:
  **~61.0% (inalterada)** — ganhos qualitativos cumulativos.
  Tabela B Content variants: 54 → **56**. **ADR-0060 mantém-se
  `IMPLEMENTADO`** (Fase 1 fechada P155 não muda; Fase 2 fechada
  P157C; anotação P157C adicionada — promoção a R1 candidata se
  decisão humana for prioritária). **ADR-0061 mantém-se
  `PROPOSTO`** (Layout inalterado por P157C). README ADRs:
  total **63 inalterado**; reservas P158/P159/ADR-0062 mantidas.
  Hash `content.rs` mantém-se `ec58d849` (`crystalline-lint
  --fix-hashes` reportou "Nothing to fix"; refactor aditivo).
  **DEBT-56 permanece aberto** — P157C contribui via storage de
  `repeat: bool`. **Padrões pós-P157C**: granularidade **N=12**
  (cross-domínio fortalecido — 3 sub-passos Model consecutivos
  sem reformulação fecham conjunto coerente); inventariar
  primeiro N=9 → **10** (P157C reforça critério #6 divergência
  da spec); Smart→Option N=8 → **9** (Caso D primeiro Model);
  §análise risco N=9 → **10**; reuso template containers N=4
  (inalterado); reuso `Sides<T>` N=2 (inalterado); reuso
  `extract_length` N=7 (inalterado — P157C não usa);
  `extract_tracks` N=2 (inalterado); `extract_usize_or_none_min`
  N=4 (inalterado); **novo subpadrão helper privado
  parametrizado `extract_bool_with_default` N=2 usos no mesmo
  passo**; **novo subpadrão "par simétrico em pattern-match"
  N=2 aplicações concretas** (P156D HSpace+VSpace + P157C
  Header+Footer). **Saturação cross-domínio cross-caso
  ADR-0064**: 4/4 casos validados em Layout; 3/4 (A,C,D) em
  Model — Caso B só Layout (candidato futuro Model). **ADR meta
  P156K atinge maturidade empírica.**
- **Passo 157B — Model Fase 2 sub-passo 2: table cell**
  (décima primeira aplicação consecutiva de materialização;
  segundo sub-passo Model Fase 2 per ADR-0060 §"Decisão 1"
  sub-passo 3). **Primeira aplicação concreta de ADR-0064
  Caso A em domínio Model** (P156G/H/I aplicaram-no em Layout);
  **terceira aplicação global de Caso C** com **primeira variação
  `usize`** (anteriores eram `Length`). Substantivo M: variant
  `Content::TableCell { body, x: Option<usize>, y: Option<usize>,
  colspan: Option<usize>, rowspan: Option<usize> }` adicionado
  ao enum (53 → 54 variants); stdlib `#table_cell(body, x: ?,
  y: ?, colspan: ?, rowspan: ?)` em `stdlib/structural.rs`
  (continuação P157A). **Decisão arquitectural de naming P157B**:
  `table_cell` flat (snake_case) **divergência intencional vs
  vanilla `table.cell`** per ADR-0033 — FieldAccess cristalino
  actual em `bindings.rs:eval_field_access` suporta apenas
  `Value::Dict.field` e `Value::Content.field` (`get_field`);
  não suporta `Value::Func.subname` (namespacing de funcs);
  refactor para `table.cell` exigiria introdução de
  `Value::Module`/`Value::ScopedFunc` — fora de scope (futuro
  refactor pode adicionar alias `table.cell` sem breaking
  change). **Tradução ADR-0064**:
  - x/y: `Smart<usize>` vanilla → `Option<usize>` (Caso A;
    None ↔ Auto auto-placement);
  - colspan/rowspan: `NonZeroUsize` default 1 → `Option<usize>`
    com `None` ↔ default 1 (Caso C; zero rejeitado em stdlib
    paridade `NonZeroUsize`).
  Helper privado novo `extract_usize_or_none_min(val, fn,
  field, min: usize)` em `stdlib/structural.rs` parametrizado
  (min=0 para x/y; min=1 para colspan/rowspan) — combina 4 usos
  num único helper para evitar duplicação. **Limitação aceite
  per ADR-0054 graded**: `x/y/colspan/rowspan` armazenados mas
  **ignorados em layout** — algoritmo de placement diferido em
  **DEBT-34e** (refactor dedicado a placement Grid completo;
  permanece aberto). 6 atributos vanilla scope-out
  (align/stroke/fill/inset/breakable + internals
  kind/is_repeated). Cobertura exaustiva de **9 sítios
  pattern-match estruturais** (paridade P157A): variant +
  construtor + is_empty + plain_text + PartialEq + map_content +
  map_text + materialize_time + walk + layout arm. Validação:
  colspan/rowspan = 0 rejeitado (paridade NonZeroUsize); int
  negativo em qualquer field rejeitado; named arg desconhecido
  rejeitado. Tests: 1097 → **1115** typst-core lib (+18: 7
  unit TableCell + 9 stdlib + 2 layout E2E incluindo cell
  dentro de Table). **Auto-validação cross-domínio ADR-0064**:
  Caso A passa de N=3 (100% Layout) para N=4 (75% Layout +
  25% Model); Caso C passa de N=2 (Length 100%) para N=3
  (Length 66% + usize 33%). Cobertura Model agregada
  **inalterada** (50% — `table.cell` é sub-entrada de `table`
  per padrão P154A); ganho qualitativo via expansão estrutural.
  Total user-facing: **~61.0% (inalterada)**. Tabela B Content
  variants: 53 → **54**. **ADR-0060 mantém-se `IMPLEMENTADO`**
  (Fase 1 fechada P155 não muda; Fase 2 prossegue per roadmap;
  anotação P157B adicionada). **ADR-0061 mantém-se `PROPOSTO`**
  (Layout inalterado por P157B). README ADRs: total **63
  inalterado**; reservas P158/P159/ADR-0062 mantidas. Hash
  `content.rs` mantém-se `ec58d849` (`crystalline-lint
  --fix-hashes` reportou "Nothing to fix"; refactor aditivo).
  **DEBT-34e permanece aberto** — P157B contribui ao armazenar
  fields necessários ao algoritmo de placement, mas não fecha.
  **Padrões pós-P157B**: granularidade **N=11** (cross-domínio
  reforçado — 2 sub-passos Model consecutivos sem reformulação);
  inventariar primeiro N=8 → **9** (P157B aplicação concreta
  critério #1 naming + #6 divergência da spec); Smart→Option
  N=7 → **8** (Caso A primeiro Model; Caso C primeira variação
  usize); §análise risco N=8 → **9**; reuso template containers
  N=4 (inalterado); reuso `Sides<T>` N=2 (inalterado); reuso
  `extract_tracks` N=2 (inalterado — P157B não usa); **novo
  subpadrão helper privado parametrizado `extract_usize_or_none_min`
  N=4 usos no mesmo passo** (combinação via param em vez de
  duplicação).
- **Passo 157A — Model Fase 2 sub-passo 1: table minimal**
  (décima aplicação consecutiva de materialização desde início
  da série granular P156C; **primeiro sub-passo Model Fase 2**
  per ADR-0060 §"Decisão 1" sub-passo 3). Substantivo M:
  variant `Content::Table { columns: Vec<TrackSizing>, rows:
  Vec<TrackSizing>, children: Vec<Content> }` adicionado ao
  enum (52 → 53 variants); stdlib `#table(columns: ?, rows: ?,
  ..children)` em **`stdlib/structural.rs`** (decisão arquitectural
  P157A: módulo Model existente, não novo `stdlib/model.rs` —
  per diagnóstico P157A §8). Subset minimal per ADR-0054 graded:
  3 fields críticos; **9+ atributos vanilla scope-out**
  (gutter/inset/align/fill/stroke/summary; cells estruturadas
  P157B; header/footer P157C; HLine/VLine cosmetic). Layouter
  delega a `layout_grid` clone simples per ADR-0060 §"Decisão 4"
  (sem modificação de `grid.rs` — confirmado por diff vazio).
  Helper `extract_tracks` promovido a `pub(super)` para reuso
  cross-módulo `stdlib/layout.rs` → `stdlib/structural.rs`
  (N=2; subpadrão emergente análogo a `extract_length` N=7).
  Cobertura exaustiva de **9 sítios pattern-match estruturais**
  (paridade P156I Stack / P156J Repeat / P156L Pad). Validação:
  named arg desconhecido rejeitado com diagnóstico claro;
  child Int rejeitado; defaults columns/rows omitidos caem em
  `[Auto]`. Tests: 1081 → **1097** typst-core lib (+16:
  6 unit Table + 8 stdlib Table + 2 layout E2E incluindo
  paridade estrutural Table↔Grid). **Decisão arquitectural
  P157A**: variant dedicado per ADR-0060 §Decisão 4 (não Styled);
  field `children` (não `cells` como Grid) per nomenclatura
  vanilla `Vec<TableChild>`. **ADR-0064 NÃO aplicável directamente**
  (subset minimal sem Smart<T>; futuras aplicações em P157B
  Caso A para `TableCell.x/y`; P157C Caso D para `repeat`
  default true). **ADR-0065 critério #1 (naming módulo) +
  critério #5 (scope) implícitos** em decisão de módulo
  `stdlib/structural.rs` continuação. Cobertura Model 45% →
  **50%** (entrada `table` transita `ausente → implementado`;
  contagem Model 6/4/5/7/0=22 → 7/4/5/6/0=22). Total user-facing:
  ~60.3% → **~61.0%**. Tabela B Content variants: 52 → **53**.
  **ADR-0060 mantém-se `IMPLEMENTADO`** (Fase 1 fechada P155
  não muda; Fase 2 prossegue per roadmap; anotação P157A
  adicionada). **ADR-0061 mantém-se `PROPOSTO`** (Layout
  inalterado por P157A). README ADRs: total **63 inalterado**;
  reservas P158/P159/ADR-0062 mantidas. Hash `content.rs`
  mantém-se `ec58d849` (`crystalline-lint --fix-hashes`
  reportou "Nothing to fix"). **Padrões pós-P157A**:
  granularidade **N=10** (cross-domínio confirmado — Layout
  P156C-L → Model P157A sem reformulação); inventariar primeiro
  N=7 → **8**; Smart→Option N=7 (inalterado); §análise risco
  N=7 → **8**; reuso template containers N=4 (inalterado);
  reuso `Sides<T>` N=2 (inalterado); **novo subpadrão reuso
  `extract_tracks` N=2** (análogo a `extract_length` em fase
  inicial); reuso `extract_length` N=7 (inalterado).
- **Passo 157 — Diagnóstico Model Fase 2 (table foundations)**
  (passo arquitectural de diagnóstico; **não materializa código**).
  **Primeira aplicação concreta de ADR-0065 critério #5**
  (scope determinado por inventário) — auto-validação cumulativa
  do ADR meta P156K (P156L já validou critério #3; P157 valida
  critério #5). Diagnóstico em
  `00_nucleo/diagnosticos/diagnostico-model-fase-2-passo-157.md`
  com 5 secções: §1 ADR-0060 lida e resumida (status
  `IMPLEMENTADO`; Fase 1 fechada P155; subset declarado
  `Content::Table` + `TableCell` + `TableHeader` + `TableFooter`
  M+); §2 estado de Model em código (factualmente confirmado:
  `table` ausente; `grid` parcial mas funcional 272 linhas;
  `Figure.kind: "table"` slot já existe — preparação P158
  figure-table); §3 scope determinado (subset máximo M+ rejeitado
  por violar granularidade N=9; **recomendação: dividir em 3
  sub-passos M cada — P157A/B/C** preservando granularidade
  N=10/11/12 com cobertura cumulativa Model análoga a Layout
  P156C-L); §4 dependências bloqueantes (zero hard; limitações
  scope-out per ADR-0054 graded em P157B colspan/rowspan via
  DEBT-34e e P157C repeat real via DEBT-56); §5 esboço de
  **P157A** (variant `Content::Table` minimal reusando Grid;
  M; tests esperados ~10-15; granularidade preservada N=10).
  **Reservas P158/P159/ADR-0062 mantêm-se inalteradas**.
  Padrões pós-P157: granularidade N=9 (inalterada — P157 é
  diagnóstico, não materialização); inventariar primeiro N=6
  → **7** (primeiro critério #5); §análise de risco N=6 → **7**
  (passo diagnóstico baixo risco); ADR-0064 Caso D ganha
  patamar N=4/5 implícito (TableHeader/Footer.repeat default
  true). Total ADRs **63 inalterado**; sem novas crates; sem
  novos DEBTs; sem alteração de hashes (passo documental).
- **Passo 156L — Layout Fase 3 sub-passo 2: pad refino sides
  individualizadas** (nona aplicação consecutiva de **ADR-0061**;
  **primeira aplicação concreta de ADR-0065 critério #3** —
  expansão de variant existente; **segunda aplicação concreta de
  ADR-0064 Caso C** — `Length` default zero → `Option<Length>`).
  Substantivo M com **refactor real** (primeiro após série
  aditiva P156C-J; risco médio, não baixo). Variant
  `Content::Pad { body, padding: Sides<Length> }` refactorado
  para `Content::Pad { body, sides: Sides<Option<Length>> }` —
  cada side `None ↔ default vanilla zero` resolvido em momento
  de uso no Layouter (em vez de em `native_pad`). Field renomeado
  `padding → sides` para alinhar com naming vanilla. Helper
  privado novo `extract_sides_lengths` em `stdlib/layout.rs`
  (não-genérico per pré-decisão; promoção a genérico/público
  diferida até segundo reuso). Cobertura exaustiva de **12
  sítios pattern-match estruturais** + adaptação de **7 tests
  pré-existentes** (1 unit em entities + 6 stdlib + 1 layout
  E2E). Validação: padding negativo continua rejeitado em qualquer
  side declarado; named arg desconhecido rejeitado; precedência
  específico > eixo > rest preservada (paridade vanilla
  pad.rs:20-24). **Divergência da spec do passo P156L
  §"Verificação" #5** detectada via inventário ADR-0065:
  spec assumia pad como `parcial` (factualmente `implementado`
  desde P156C); cobertura **não passa para 84%** como spec
  previa — pad ganha sufixo ⁺ (`implementado⁺`) indicando
  refino qualitativo sem ganho quantitativo. Documentado em
  diagnóstico §6.1. Tests: 1077 → **1081** typst-core lib
  (+4 novos: 1 sides individuais, 1 some_zero≠none, 1 x-axis
  isolado, 1 cadeia precedência); 7 tests pré-existentes
  adaptados sem mudança de contagem. Cobertura Layout: 78% →
  **78%** (refino qualitativo: 14/0/3/1=18 → 13/1/3/1=18).
  **Reusos consolidados**: `extract_length` **N=7**;
  `Sides<T>` infraestrutura **N=2**; `extract_length` aproxima-se
  de promoção formal a helper público (refactor escopo XS
  candidato). **Padrões metodológicos pós-P156L**: granularidade
  N=9; inventariar primeiro N=6 (primeiro critério #3 de
  ADR-0065); Smart→Option N=7 (segundo Caso C); §análise risco
  N=6 (primeiro com peso real, não cerimonial); reuso template
  containers N=4 (inalterado); reuso `Sides<T>` N=2 (novo
  subpadrão). **Status `PROPOSTO` mantido**; promoção a
  `IMPLEMENTADO` continua diferida. Hash inalterado
  (`crystalline-lint --fix-hashes` reportou "Nothing to fix" —
  refactor preserva o hash do prompt L0). Reservas ADR-0062/63
  inalteradas; total ADRs **63** (inalterado).
- **Passo 156K — ADRs meta formalizando padrões consolidados
  da série P156C-J** (passo arquitectural meta; **não materializa
  código**). Dois ADRs novos `EM VIGOR`: **ADR-0064** (Tradução
  `Smart<T>` → `Option<T>`/default — formaliza padrão N=6 com
  4 casos canónicos A/B/C/D; documenta subpadrão emergente
  `extract_length` reuso N=6) e **ADR-0065** (Inventariar
  primeiro — sub-passo `.1` para decisão arquitectural
  não-trivial; estende ADR-0034 generalizando para naming /
  tipo / variant / camadas / scope / divergência da spec; padrão
  N=5 com critério "default = inventariar"). Auto-aplicação:
  P156K cumpre o próprio padrão a ser formalizado (sub-passo
  .1 dedicado ao inventário do estado dos ADRs antes de redigir
  os novos). Numeração escolhida: 0064 + 0065 (consecutivos;
  0062 reservada hayagriva, 0063 reservada column flow).
  Contagens: total 61 → **63** ADRs; EM VIGOR 26 → **28**.
  Reservas ADR-0062/ADR-0063 inalteradas. Sem alteração de
  código (`crystalline-lint` zero violations; sem hash a
  propagar). **Benefício**: sessões futuras citam ADRs
  explicitamente em vez de re-justificar empiricamente cada
  passo — reduz overhead de enunciados e garante rastreabilidade
  formal dos padrões consolidados.
- **Passo 156J — Layout Fase 3 sub-passo 1: repeat (primeira
  Fase 3)** (oitava aplicação consecutiva de **ADR-0061**;
  **activa caminho 1** dos 3 documentados em §"Aplicações
  cumulativas"). Substantivo M: **decisão arquitectural
  reusada** de P156G/H/I (variant rico) sem nova decisão.
  `Content::Repeat { body, gap: Option<Length>, justify:
  bool }` adicionado ao enum (51 → 52 variants); stdlib
  expõe `#repeat(body, gap: ?, justify: true)` (paridade
  vanilla — default `justify == true` é divergência intencional
  do default Rust `bool::default() == false`). **Limitação
  aceite per ADR-0054 graded**: algoritmo dinâmico de
  "quantidade-para-encher" (vanilla calcula `floor(available
  / (body_width + gap))`) diferido — Layouter executa
  single-render do body; suficiente para paridade estrutural
  (variant disponível em todo o pipeline; counters/labels
  descem via walk; medição estática via measure_content).
  Cobertura exaustiva de arms em `Content` (3 fields adaptados),
  `introspect.rs` (materialize_time + walk recurse no body),
  `layout/mod.rs` (single-render em layout_content; recurse em
  measure_content_constrained). Validação: gap negativo
  rejeitado; gap não-length rejeitado; justify não-bool
  rejeitado; named arg desconhecido rejeitado; body posicional
  obrigatório (Content ou Str). **Helper `extract_length`
  reusado N=6** vezes consecutivas (P156C/D/G/H/I/J) — emergiu
  como vocabulário canónico (subpadrão dentro de "reuso de
  template containers" N=4). **Padrão Smart→Option/default
  atinge N=6** aplicações consecutivas (P156D weak; P156E to;
  P156G/H width; P156I spacing; P156J gap) — patamar empírico
  reforçado, candidato a ADR meta P156K-meta. Tests:
  1296 → **1315** (+19 = 6 unit Repeat + 11 stdlib + 2 layout
  E2E + regression Stack/Block/Box/Pad/Hide). Cobertura Layout:
  72% → **78%** (13/18 → 14/18) — **target ADR-0061
  ultrapassado**; total user-facing: 60% → **~60.3%**.
  **ADR-0061 §"Aplicações cumulativas" actualizada** para
  pós-P156J: tabela slope cumulativo (8 passos), padrões N
  incrementados (Granularidade N=8, Inventariar N=5,
  Smart→Option N=6, §análise risco N=5, Reuso template N=4),
  estado pós-P156J (4 entradas restantes — columns/colbreak
  bloqueadas por DEBT-56, refinos pad/place/measure).
  **Status `PROPOSTO` mantido** — caminho 1 50% concluído
  (1/2 features Fase 3); promoção a `IMPLEMENTADO` continua
  diferida (decisão humana sobre columns/colbreak ou scope-out
  formal). README ADRs: total e distribuição inalterados
  (61 ADRs; PROPOSTO 11). Secção Repeat adicionada a
  `entities/content.md`. Hash propagado (`content.rs` →
  `ec58d849`). **Fase 3 iniciada: +19 tests; +6 pontos
  percentuais Layout; zero reformulações em N=8 aplicações
  consecutivas.**
- **Passo 156I — Layout Fase 2 sub-passo 3: stack compositivo
  (último Fase 2; atinge target 72%)** (sétima aplicação
  consecutiva de **ADR-0061**; **fechamento de série
  P156C-I**). Substantivo M agregado: **decisão arquitectural
  reusada** de P156G/H (variant rico) com adaptação para
  `Arc<[Content]>` (clone O(1) per ADR-0026 revisão,
  consistente com `Sequence`/`MathSequence`).
  `Content::Stack { children: Arc<[Content]>, dir: Dir,
  spacing: Option<Length> }` adicionado ao enum (50 → 51
  variants); **tipo `Dir` novo** em `01_core/src/entities/dir.rs`
  (4 direcções LTR/RTL/TTB/BTT; análogo a `Parity` P156E e
  `Sides<T>` P156C — infraestrutura genérica reusável); stdlib
  expõe `#stack(dir: ?, spacing: ?, ..children)` (children
  variádicos posicionais; helper `extract_dir` para parsing
  de string). **Decisão local 156I.2** (per inventário 156I.1):
  spacing implementado real (trivial via cursor advance per
  pattern P156D HSpace/VSpace). Cobertura exaustiva de arms
  em `Content` (3 fields adaptados para Vec), `introspect.rs`
  (recurse em cada child), `layout/mod.rs::layout_content`
  (4 direcções: TTB/BTT empilham verticalmente com
  `cursor_y += spacing`; LTR/RTL inline com `cursor_x +=
  spacing`; BTT/RTL implementadas como reverse iteration per
  ADR-0054 graded) + `measure_content_constrained` (sum
  heights + (n-1)*spacing para vertical; sum widths para
  horizontal). Validação: `dir` rejeita strings inválidas;
  spacing negativo rejeitado; named arg desconhecido
  rejeitado; children não-Content/Str rejeitados (estricto).
  Sem atributos vanilla scope-out (vanilla stack tem só
  estes 3). Tests: 1271 → **1296** (+25 = 4 unit Dir + 6 unit
  Stack + 12 stdlib + 3 layout E2E). Cobertura Layout:
  67% → **72%** (12/18 → 13/18) — **target ADR-0061
  atingido**; total user-facing: 59% → **60%**. **ADR-0061
  ganha §"Aplicações cumulativas"** documentando a sequência
  P156C-I com slope cumulativo, padrões metodológicos
  consolidados (granularidade N=7; inventariar primeiro
  N=4; Smart→Option N=5; reuso template N=3),
  variants/stdlib funcs adicionados, e estado pós-target.
  **Status `PROPOSTO` mantido** — promoção a `IMPLEMENTADO`
  requer Fase 3 (columns DEBT-56) ou decisão humana de
  scope-out formal. README ADRs: total e distribuição
  inalterados (61 ADRs; PROPOSTO 11). Secção Stack adicionada
  a `entities/content.md`; L0 `entities/dir.md` criado.
  Hashes propagados (`content.rs` → `b9ca52c4`, `dir.rs` →
  `657f9389`). **Fechamento de série granular P156C-I:
  +151 tests acumulados; +50 pontos percentuais Layout;
  zero reformulações em N=7 aplicações consecutivas.**
- **Passo 156H — Layout Fase 2 sub-passo 2: box inline container**
  (sexta aplicação consecutiva de **ADR-0061**; segunda
  Fase 2). Substantivo M agregado: **decisão arquitectural
  reusada** de P156G (variant rico) sem nova decisão.
  `Content::Boxed { body, width: Option<Length>, height:
  Option<Length>, inset: Sides<Length>, baseline: Length }`
  adicionado ao enum (49 → 50 variants); naming `Boxed`
  evita conflito com `std::boxed::Box`; stdlib expõe
  `#box(body, width: ?, height: ?, inset: ?, baseline: ?)`
  (paridade vanilla). **Distinção material face a Block
  (P156G)**: posicionamento **inline** (não força flush_line);
  atributo único `baseline` (vs `breakable`); largura default
  content-based (vs full page). 6 atributos vanilla scope-out
  (outset, fill, stroke, radius, clip, stroke-overhang) per
  ADR-0054 graded — rejeitados com erro hard. Cobertura
  exaustiva de arms em `Content` (5 fields), `introspect.rs`
  (recurse body), `layout/mod.rs::layout_content` (apenas
  inset.left + body + inset.right como avanço cursor.x;
  width/height/baseline/inset.top/bottom armazenados mas
  semantic real adiada — cursor.rs sem mecânica baseline
  mid-linha; refactor multi-region exigido per DEBT-56) +
  `measure_content_constrained` (dimensões correctas com
  todos os atributos). Validação: width/height/inset
  negativos rejeitados; **baseline negativo aceito** (semantic
  legítima — move para cima). Tests: 1250 → **1271** (+21 =
  6 unit Boxed + 13 stdlib + 2 layout E2E + regression
  Block/Pad/Hide). Cobertura Layout: 61% → **67%** (11/18 →
  12/18); total user-facing: 58% → **59%**. **ADR-0061
  mantém-se `PROPOSTO`** (anotação cumulativa após Fase 1+2
  completas). README ADRs: total e distribuição inalterados
  (61 ADRs; PROPOSTO 11). Secção Boxed adicionada a
  `entities/content.md`; hash propagado (`content.rs` →
  `5bb6e3d2`). **Padrão emergente Fase 2 confirmado**:
  reaplicação directa do template Block reduziu custo de
  pensamento; P156I (stack) provavelmente segue mesmo modelo.
- **Passo 156G — Layout Fase 2 sub-passo 1: block container**
  (quinta aplicação consecutiva de **ADR-0061**; **primeira
  aplicação Fase 2** — containers ricos). Substantivo M+
  agregado: **decisão arquitectural** em 156G.2 escolheu
  **variant rico** (Opção A modificada) sobre Style cascade
  per inventário 156G.1 — `Style` enum cobre só propriedades
  de texto (Bold/Italic/Size/Fill/HeadingLevel), vocabulário
  não-encaixa para width/height/inset/breakable de container.
  `Content::Block { body, width: Option<Length>, height:
  Option<Length>, inset: Sides<Length>, breakable: bool }`
  adicionado ao enum (48 → 49 variants); stdlib `#block(body,
  width: ?, height: ?, inset: ?, breakable: true)` (subset
  Fase 1 per ADR-0054 graded; **9 atributos vanilla scope-out**:
  outset, fill, stroke, radius, clip, spacing, above/below,
  sticky — todos rejeitados com erro hard até refino futuro).
  Cobertura exaustiva de arms em `Content` (5 fields),
  `introspect.rs` (recurse body), `layout/mod.rs::layout_content`
  (force flush_line + inset top + offset line_start_x para
  body + flush_line final + inset bottom + height mínimo
  forçado se Some) + `measure_content_constrained` (dimensões
  para grid). Layouter scope-outs: `inset.right` (mesma razão
  Pad em P156C — refino multi-region); `width` armazenado mas
  não impõe limite real (refino multi-region); `breakable: false`
  semantic real adiada. Tests: 1230 → **1250** (+20 = 6 unit
  Block + 12 stdlib + 2 layout E2E + regression Pad/Hide).
  Cobertura Layout: 56% → **61%** (10/18 → 11/18); total
  user-facing: 57% → **58%**. **ADR-0061 mantém-se `PROPOSTO`**
  (anotação cumulativa após Fase 1+2 completas, per decisão
  humana). README ADRs: total e distribuição inalterados
  (61 ADRs; PROPOSTO 11). Secção Block adicionada a
  `entities/content.md` documentando decisão arquitectural;
  hash propagado (`content.rs` → `5702d2e3`). **Padrão
  emergente Fase 2**: containers ricos preferem variants
  explícitos quando atributos não são propriedades de texto;
  Box (P156H) e Stack (P156I) provavelmente seguem mesmo
  modelo.
- **Passo 156F — Layout Fase 1 sub-passo 4: skew via TransformMatrix**
  (quarta aplicação consecutiva de **ADR-0061**; **divergência
  consciente da spec** baseada em descoberta empírica). Substantivo
  S agregado: spec propunha refactor com `enum TransformKind
  { Move, Rotate, Scale, Skew }` para "unificar" os 4 elementos
  vanilla, mas inventário 156F.1 revelou que `Content::Transform
  { body, matrix: TransformMatrix }` **já era unificado** via
  matriz cm desde P78. Decisão deste passo: adicionar apenas
  método estático `TransformMatrix::skew(ax_rad, ay_rad)` em
  `entities/layout_types.rs` (forma da matriz cm:
  `{a:1, b:tan(ay), c:tan(ax), d:1, tx:0, ty:0}`) + `native_skew`
  em `stdlib/transforms.rs` (ao lado de move/rotate/scale,
  coesão por domínio). **Zero refactor de variant** — `Content`
  enum mantém 48 variants inalterado. **Risco de regressão zero**
  (puramente aditivo). Validação: ângulos com magnitude ≥
  `π/2 - 1e-3` rejeitados (tan diverge); aceita `Angle` ou
  `Float` (radianos directos, consistente com `native_rotate`);
  named arg desconhecido rejeitado; `origin` scope-out (alinhado
  com move/rotate/scale actuais). Tests: 1214 → **1230** (+16 =
  4 unit TransformMatrix::skew + 9 stdlib native_skew + 3
  regression de move/rotate/scale). Cobertura Layout: 50% →
  **56%** (9/18 → 10/18); total user-facing mantém 57%
  (arredondamento; 60 vs 59 implementado). **ADR-0061 mantém-se
  `PROPOSTO`** (anotação cumulativa após Fase 1 completa). README
  ADRs: total e distribuição inalterados (61 ADRs; PROPOSTO 11).
  Secção skew adicionada a `entities/content.md` documentando
  divergência da spec; hash propagado (`content.rs` → `4321258d`).
  **Hipótese granular reforçada**: P156F era teste de risco de
  regressão (primeiro passo modificador esperado); inventário
  empírico simplificou para puramente aditivo, eliminando o risco.
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
