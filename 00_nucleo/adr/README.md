# Índice de ADRs do Typst Cristalino

Este documento é o índice canónico dos Architectural Decision
Records (ADRs) do projecto **Typst Cristalino**. Lista os 64 ADRs
em vigor (63 números únicos; ADR-0026 tem variante -R1 por
revisão), as meta-regras que governam o projecto, o vocabulário
canónico de status, cadeias de revogação e revisão, e convenções
estruturais.

**Reservas de números** (sem ficheiro criado, mas comprometidos):
- ~~**ADR-0062**~~ **CONSUMIDA** em passo `ADR-0062-create`
  (2026-04-27) — ficheiro
  `typst-adr-0062-hayagriva-bibliography-parsing.md` criado com
  status `PROPOSTO`. Promoção a `IMPLEMENTADO` ocorre em passo
  futuro de materialização hayagriva real (P159G ou equivalente).
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
| 0061 | Layout Fase X — page model + multi-column + footnote area roadmap | `IMPLEMENTADO` (P156B PROPOSTO → P221 IMPLEMENTADO 2026-05-12; Fase 1+2+3 cumpridas; refinos `measure`/`place` Fase 4 candidata NÃO-reservada) |
| 0062 | Autorização crate `hayagriva` para bibliography + cite (CSL parsing) | `PROPOSTO` (passo `ADR-0062-create`; promoção a IMPLEMENTADO em passo futuro materialização hayagriva real) |
| 0064 | Tradução `Smart<T>` vanilla → `Option<T>`/default | `EM VIGOR` (P156K; formaliza padrão N=6 da série P156C-J) |
| 0065 | Inventariar primeiro — sub-passo `.1` para decisão arquitectural não-trivial | `EM VIGOR` (P156K; estende ADR-0034; padrão N=5 da série P156C-J) |
| 0066 | Introspection runtime — promoção da reserva conceptual (referida historicamente como "ADR-0017 Introspection runtime adiada") a ficheiro PROPOSTO | `PROPOSTO` (passo `P160A`; resolve confusão de numeração — reserva conceptual usava 0017 mas slot 0017 já IMPLEMENTADO para outro tópico; promoção a IMPLEMENTADO em passo futuro materialização Introspection runtime real — P160B subset minimal) |
| 0082 | Promoções reais de scope-outs ADR-0054 graded — 4 critérios operacionais | `PROPOSTO` (passo `P249` administrativo XS; formaliza pattern empírico N=8 cumulativo granular pós-P248 — P242 radius+clip + P247 outset+fill+stroke + P248 breakable+height+cell_overflow; promoção a EM VIGOR pendente N=3 aplicações consecutivas citantes; **nota numeração**: `P249.div-2` registado — ADR-0067 já ocupada por `attribute-grammar-scoping`; ADR-0082 escolhido como próximo slot disponível após ADR-0081) |

**Total**: 65 ADRs (64 números únicos; ADR-0026 tem variante -R1
por revisão; **+ADR-0082 PROPOSTO P249** + entradas históricas
pós-P156K não-recapitatuladas nesta tabela — ver passos-chave
abaixo).

### Distribuição de status

- `PROPOSTO`: **11** ADRs (decisões em aberto: 0005, 0006,
  0008–0015, 0062, **0066**, ~~0079 P253~~, ~~0082 P254~~)
  + **+1 ADR-0083 PROPOSTO P257.B → IMPLEMENTADO P257.D**
  (Color paridade vanilla com subset materializado;
  PROPOSTO+IMPLEMENTADO no mesmo passo via Cenário B1 pattern)
  — pós-P221: ADR-0061 e ADR-0078 transitam PROPOSTO →
  IMPLEMENTADO; pós-P226: +2 ADRs PROPOSTAS criadas (ADR-0079
  Layout Fase 5 roadmap + ADR-0080 L0 minimal para refactors
  aditivos pós-M9c N=7 meta documental); **pós-P229: ADR-0080
  transita PROPOSTO → EM VIGOR** (promoção administrativa XS
  dedicada pós-N=9 validação cumulativa); **pós-P249: +1
  ADR-0082 PROPOSTO** (Promoções reais scope-outs ADR-0054
  graded); **pós-P253: ADR-0079 transita PROPOSTO →
  IMPLEMENTADO** (Cenário A scope-out formal humano; paridade
  ADR-0061 P221); **pós-P254: ADR-0082 transita PROPOSTO → EM
  VIGOR** (passo administrativo XS dedicado pós-N=3 citantes
  P250+P251+P252; paridade pattern P229 ADR-0080); **pós-P257:
  +1 ADR-0083 IMPLEMENTADO** (Color paridade vanilla 8 espaços
  materializados + 4 scope-outs documentados per ADR-0029
  §"Simplificações aceites apenas com ADR explícita").
  **PROPOSTO 11 preservado** (ADR-0083 entra e sai no mesmo
  passo via promoção P257.D).
- `IDEIA`: 2 ADRs (0002, 0003).
- `EM VIGOR`: **30** ADRs pós-P254 (regras/políticas activas;
  0018, 0029, 0030, 0032–0051, 0054, 0058, 0059, **0064, 0065**,
  **0080** P229, **0082** P254).
- `IMPLEMENTADO`: **25** ADRs pós-P257 (decisões materializadas;
  0001, 0004, 0016, 0017, 0019, 0021–0027, 0026-R1, 0031,
  0052, 0053, 0055, 0057, **0060**, **0061** P221, **0078**
  P221, **0079** P253, **0083** P257).
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
- **Passo 160A — Criar ADR-0066 PROPOSTO Introspection runtime
  (administrativo XS)** (sub-passo administrativo de P160 para
  formalizar reserva conceptual pré-existente "Introspection
  runtime adiada" como ficheiro ADR concreto com status
  `PROPOSTO`; **não materializa código**; **não promove a EM
  VIGOR ou IMPLEMENTADO** — promoção fica para passo futuro
  P160B subset minimal materialização real). **Subpadrão
  emergente N=1 → 2** "passo administrativo XS criar ADR
  PROPOSTO a partir de reserva pré-existente" — segunda
  aplicação após `ADR-0062-create` (atinge meio-caminho limiar
  formalização N=3-4). **Decisão de numeração crítica registada**:
  reserva conceptual usava "ADR-0017" mas slot 0017 já
  IMPLEMENTADO desde 2026-03-26 para tópico distinto
  ("adiamento de eval()"); **resolução: usar próximo número
  disponível ADR-0066** (slot 0063 reservado conceptualmente
  para column flow; 0064/0065 já usados; 0066 livre); ficheiro
  novo `00_nucleo/adr/typst-adr-0066-introspection-runtime-adiada.md`
  com nota explícita sobre divergência de numeração. Ficheiro
  estrutura canónica (Status / Data / Nota numeração / Contexto /
  Decisão / Análise pureza paridade ADR-0029 / Consequências /
  Alternativas / Plano promoção futuro / Precedentes citáveis /
  Referências / Próximos passos). **Subset minimal pós-promoção**:
  state(key, init) M (P160B); metadata(value) S+ (P160C);
  here()/locate() M (P160D); query(target) M+ (P160E);
  position(target) S+ (P160F). **Cobertura esperada pós-Bloco B
  subset minimal**: ~17% → ~50%. **Status PROPOSTO** —
  autorização arquitectural concedida em princípio mas **não
  em vigor** até P160B materializa subset minimal. **Sem código
  alterado**. **Hash `entities/content.rs` preservado** `ec58d849`
  — **19º passo consecutivo** via L0-baseline interpretation.
  **Total ADRs 64 → 65** (ADR-0066 adicionada). Distribuição:
  PROPOSTO 12 → **13** (+0066); EM VIGOR 28 inalterado;
  IMPLEMENTADO 19 inalterado. Padrões pós-P160A: granularidade
  N=21 (inalterada — administrativo); inventariar primeiro
  N=24 → **25** (ADR-0065 critério #1 naming convention +
  critério #5 inventário trivial em passo administrativo);
  §análise de risco N=24 → **25** (passo administrativo XS
  muito baixo risco); subpadrão "passo administrativo XS criar
  ADR PROPOSTO" N=1 → **2** (atinge meio-caminho limiar
  formalização). **Política "sem novas reservas" preservada** —
  passo formaliza reserva conceptual pré-existente, não cria
  nova; reservas pré-existentes (slot 0063 column flow)
  mantêm-se documentadas mas não reforçadas. **Implicação**:
  Bloco B do diagnóstico P160 (5 candidatos P160B-F) agora
  pode iniciar com referência concreta a ADR-0066 PROPOSTO.
- **Passo 160 — Diagnóstico Introspection (módulo mais fraco
  17%)** (passo arquitectural de diagnóstico precedendo
  materialização Introspection; **não materializa código**;
  **quarto diagnóstico de módulo focado** após P157/P158/P159
  base; **primeira mudança de módulo cross-domínio Model →
  Introspection** desde início da série granular P156C;
  **décima segunda aplicação concreta de ADR-0065 critério #5**
  com diversidade cross-domínio nova). Diagnóstico em
  `00_nucleo/diagnosticos/diagnostico-introspection-passo-160.md`
  com 6 secções: §1 ADRs/DEBTs Introspection (ADR-0017
  confirmada como reserva sem ficheiro pré-existente; sem DEBT
  formal — ADR-0017 captura tudo); §2 inventário código actual
  (`introspect.rs` 1108 linhas + `counter_state.rs` 333 linhas
  com 14 fields públicos; subpadrão #15 N=3 demonstra single-pass
  state lookup viável); §3 features Introspection vanilla
  (3983 LOC totais; 1/13 implementado counter; 1/13 parcial
  measure; 11/13 ausentes — todas exigem ADR-0017); §4 análise
  tecto (**tecto puro saturado em ~17%** — counter() já cobre
  o atingível sem ADR-0017; refinos qualitativos não movem
  cobertura agregada; tecto pós-ADR-0017 ~50-83%); §5 sequência
  candidata (**Bloco A VAZIO**; Bloco B 5 candidatos pós-ADR-
  0017 P160A-E; Bloco C cross-módulo measure/cross-doc); §6
  recomendação primária **`ADR-0017-create`** XS administrativo
  paridade `ADR-0062-create` (alternativa Opção C: Layout Fase 3
  columns/colbreak se prioridade Layout for maior). **Decisão
  arquitectural-chave §4**: tecto Introspection puro
  trivialmente saturado por counter() já implementado;
  refinos qualitativos R1/R2/R3 (state.figure_kinds, state.heading_levels_present,
  state.equations_count) são infraestrutura sem features
  observable user-facing — adiamento até prioridade clara
  razoável. **Sem código alterado** — passo puramente
  documental. **Hash `entities/content.rs` preservado** `ec58d849`
  — **18º passo consecutivo** via L0-baseline interpretation
  (sem alteração esperada de hashes em passo documental).
  Padrões pós-P160: granularidade N=21 (inalterada — passo
  diagnóstico); inventariar primeiro N=23 → **24** (ADR-0065
  critério #5 décima segunda aplicação concreta com diversidade
  cross-domínio nova); §análise de risco N=23 → **24** (P160
  baixo risco — diagnóstico documental); subpadrão #15
  (infraestrutura state lookup) N=3 confirmado como
  infraestrutura única materializável sem ADR-0017; subpadrões
  #14/#16/P155 cross-feature/refactor field para Option N
  inalterados. **Política "sem novas reservas" preservada** —
  recomendações §6 são para validação humana, não compromissos.
  ADR-0017 estado factual confirmado (reserva sem ficheiro
  mantida; promoção a PROPOSTO via XS administrativo
  recomendada mas sujeita a validação humana).
- **Passo 159G — `BibEntry` 6 fields restantes (refino família
  159 fora Bloco A)** (**segundo sub-passo família 159 fora
  Bloco A** — Bloco A esgotado pós-P159F; **refino estrutural
  de tipo entity** sem alteração ao variant Content; **subpadrão
  #16 cresce N=2→3 atinge limiar formalização N=3-4** "refino
  tipo entity sem alteração Content"; **pattern P159D replicado
  pela terceira vez** — patamar consolida-se). Struct entity
  `BibEntry` extendido em `01_core/src/entities/bib_entry.rs`
  com **6 fields opcionais restantes mais comuns hayagriva**
  (`editor`/`series`/`note`/`isbn`/`location`/`organization`)
  — listados em P159D §9.3 como diferidos por menor universalidade.
  **Builder pattern fluente** extendido (6 novos `with_*`
  métodos paridade P159D/E). Constructor `new(4 args)` original
  preservado (backwards compat trivial via fields novos default
  `None`). Helper `extract_bib_entries` (P159A+P159D+P159E)
  extendido em `01_core/src/rules/stdlib/structural.rs`: helper
  inline `optional_str` reusado para os 6 fields — **cumulativo
  N=4 P159D + N=2 P159E + N=6 P159G = N=12 usos** (largamente
  acima do limiar promoção N=3-4; promoção a `pub(super)` ou
  helper público diferida em passo administrativo XS futuro NÃO
  reservado). Layout `format_bib_entry` extendido em
  `01_core/src/rules/layout/mod.rs` com decisões diagnóstico
  §8.2 ordem + §9 formatos individuais: editor `(Ed. {editor})`
  após title; series `({series})` após title; location: antes
  de publisher; organization substitutivo a publisher quando
  publisher ausente; isbn antes de url/doi com prefixo lowercase
  `isbn:` (paridade P159E doi); note ao final entre brackets
  `[{note}]`. **Sem alteração ao variant `Content::Bibliography`
  ou `Content::Cite`**. **Decisões cosméticas registadas**:
  ordem APA-like; formatos individuais paridade APA estilo prose.
  Tests +11 (1230 → 1241; 4 unit bib_entry P159G + 4 stdlib
  parse + 3 layout E2E formato extendido/regression/organization
  substitutivo; range esperado +8-12). Cobertura Model agregada
  **inalterada** (~50%) — refino tipo entity. Cobertura
  arquitectural **inalterada** 82%. Hashes `entities/content.rs`
  e `entities/bib_entry.rs` ambos preservados via L0-baseline
  interpretation (**17º passo consecutivo content.rs**).
  **BibEntry pós-P159G: 16 fields total** (4 obrigatórios +
  12 opcionais; cobertura ~70-75% hayagriva universais).
  Padrões pós-P159G: granularidade N=20 → **21**; inventariar
  primeiro N=22 → **23** (ADR-0065 critério #5 décima primeira
  aplicação concreta com pattern P159D replicado pela terceira
  vez); §análise de risco N=22 → **23** (P159G baixo risco —
  refino pattern validado N=2); ADR-0064 NÃO directamente
  aplicável (Optional<String> directo); subpadrão #15
  (infraestrutura state lookup) N=3 inalterado; **subpadrão
  #16 (refino tipo entity sem alteração Content): N=2 → 3
  atinge limiar formalização N=3-4**; helper `optional_str`
  cumulativo N=4 → **12** (largamente acima limiar promoção);
  P155 cross-feature N=1 inalterado; refactor de field para
  Option N=1 inalterado.
  **Sequência alfabética identificadores família 159
  não-monótona**: A → B → C → D → F → E → G (facto histórico
  registado; preserva slot E para refinos família 159 que
  surgiram após P158C ocupar identificador alternativo).
  **Política "sem novas reservas" preservada** — restantes
  fields vanilla (booktitle/address/chapter/type/institution/
  etc.), tipos estruturados, ISBN validation, hyperlinks
  (Bloco C), promoção `optional_str` a helper público, ADR
  meta subpadrão #16 (N=3 atinge limiar) permanecem candidatos
  NÃO-reservados.
- **Passo 159E — `url` + `doi` em `BibEntry` (refino família
  159 fora Bloco A)** (**primeiro sub-passo família 159 fora
  Bloco A** do diagnóstico P159B — Bloco A esgotado pós-P159F;
  **refino estrutural de tipo entity** sem alteração ao variant
  Content; **subpadrão #16 cresce N=1→2** "refino tipo entity
  sem alteração Content" — patamar atinge meio-caminho do limiar
  formalização N=3-4; pattern P159D replicado fielmente).
  Struct entity `BibEntry` extendido em
  `01_core/src/entities/bib_entry.rs` com 2 fields opcionais
  identificadores digitais (`url`/`doi`) — par natural
  identificado em P159D §9 como candidato a sub-passo M futuro.
  **Builder pattern fluente** extendido (`with_url`/`with_doi`)
  paridade P159D. Constructor `new(4 args)` original preservado
  (backwards compat trivial via fields novos default `None`).
  Helper `extract_bib_entries` (P159A+P159D) extendido em
  `01_core/src/rules/stdlib/structural.rs`: helper inline
  `optional_str` reusado para url/doi — **cumulativo N=2 P159D
  + N=2 P159E = N=4** (atinge limiar promoção a `pub(super)` ou
  helper público N=3-4; reavaliação em passo administrativo XS
  futuro NÃO reservado). Layout `format_bib_entry` extendido em
  `01_core/src/rules/layout/mod.rs` com concatenação condicional
  APA-like (**Opção C diagnóstico §8.2**: url/doi após `(year).`
  per paridade APA + backwards compat — quando ambos `None`,
  output P159D preservado exactamente). **Formato decidido**
  diagnóstico §9: URL plaintext literal `https://example.com/paper`;
  DOI prefixo `doi:10.1234/abc` (paridade APA estilo prose).
  **Sem alteração ao variant `Content::Bibliography` ou
  `Content::Cite`**. Hyperlinks NÃO suportados — plaintext
  simples per ADR-0033 + ADR-0054 graded; depende Layout/PDF
  infra cross-módulo (Bloco C — NÃO reservado). Tests +8
  (1222 → 1230; 3 unit bib_entry url/doi + 3 stdlib parse +
  2 layout E2E formato extendido/regression; range esperado
  +5-8). Cobertura Model agregada **inalterada** (~50%) —
  refino tipo entity. Cobertura arquitectural **inalterada** 82%.
  Hashes `entities/content.rs` e `entities/bib_entry.rs` ambos
  preservados via L0-baseline interpretation (**16º passo
  consecutivo content.rs**; bib_entry.rs preservado paridade
  P159D resultado). Padrões pós-P159E: granularidade N=19 → **20**;
  inventariar primeiro N=21 → **22** (ADR-0065 critério #5
  décima aplicação concreta com pattern P159D replicado);
  §análise de risco N=21 → **22** (P159E baixo risco — refino
  pattern idêntico a P159D validado); ADR-0064 NÃO directamente
  aplicável (Optional<String> directo); subpadrão #15
  (infraestrutura state lookup): N=3 inalterado; **subpadrão
  #16 (refino tipo entity sem alteração Content): N=1 → 2**
  (atinge meio-caminho limiar formalização); helper
  `optional_str` cumulativo N=2 → **4** (atinge limiar promoção);
  P155 cross-feature N=1 (inalterado); refactor de field para
  Option N=1 (inalterado).
  **Política "sem novas reservas" preservada** — restantes fields
  vanilla (`editor`/`series`/`note`/`isbn`/`location`/`organization`),
  tipos estruturados, URL/DOI validation, hyperlinks (Bloco C),
  promoção `optional_str` a helper público permanecem candidatos
  NÃO-reservados.
- **Passo 159F — Numbering numérico Bibliography (Model
  bibliography+cite sub-passo 4 — **último candidato Bloco A**)**
  (quarto sub-passo substantivo de Bibliography + Cite após
  P159A par acoplado + P159C cite.form + P159D BibEntry fields;
  **refino comportamental** com counter local de bib entries +
  render numerado em Cite Normal/None; **subpadrão #15
  cresce N=2→3** "infraestrutura state lookup"; **marca conceptual**:
  Bloco A do diagnóstico P159B esgotado pós-P159F; tecto Model
  puro estimado (~55-60%) atingido empiricamente). Field aditivo
  `pub bib_numbers: HashMap<String, u32>` em
  `01_core/src/entities/counter_state.rs::CounterState` (paridade
  aditiva infraestrutura state lookup — Sides P156C → Parity
  P156E → Dir P156I → BibEntry P159A → CitationForm P159C +
  state.lang P158B + state.bib_entries P159C +
  **state.bib_numbers P159F**; patamar #15 atinge limiar de
  candidato a formalização ADR meta N=3-4). Walk arm
  `Content::Bibliography` em `rules/introspect.rs` popula
  contínuamente: `state.bib_numbers.entry(key).or_insert(len + 1)`
  — multi-Bibliography preserva primeiro número (paridade
  HashMap; decisão diagnóstico §9 contínua vs independente).
  Layout arm `Content::Cite { form: Normal/None }` em
  `rules/layout/mod.rs` faz lookup `state.bib_numbers.get(key)`
  → `[N]` ou fallback `[key]` (regression P159A). Forms
  diferenciadas (Prose/Author/Year) inalteradas — numeração só
  em Normal/None preserva semântica forms (decisão diagnóstico
  §10). **Decisão arquitectural-chave Opção C** (Cite.form
  interaction sem field user-facing) escolhida com matriz
  multi-critério (decisão diagnóstico §8): vs Opção A
  (substituir sempre; rejeitada por quebrar tests P159A/C);
  vs Opção B (Bibliography.style field novo; rejeitada por
  alteração estrutural sem ganho proporcional). Multi-Bibliography
  contínua (paridade vanilla numeric style). **Sem alteração
  ao variant `Content::Cite` ou `Content::Bibliography`**.
  Helper inline trivial — sem helper público novo (promoção
  diferida N=3-4). Tests +8 (1214 → 1222; 2 unit counter_state
  bib_numbers + 6 layout E2E numbering — `cite_normal_renderiza_numero_quando_bib_populada`,
  `cite_normal_fallback_placeholder_quando_bib_vazia`,
  `cite_normal_multiple_entries_numeradas_em_ordem`,
  `cite_form_prose_inalterada_com_bib_numerada` (regression
  P159C), `cite_unknown_key_fallback_placeholder` (regression
  P159A), `cite_normal_multi_bibliography_continua`; range
  esperado +10-15 ligeiramente abaixo por helper inline
  trivial). Cobertura Model agregada **inalterada** (~50%) —
  refino comportamental. Cobertura arquitectural **inalterada**
  82%. Hash `entities/content.rs` preservado `ec58d849` —
  **15º passo consecutivo** (P156L → P159F via L0-baseline
  interpretation). Padrões pós-P159F: granularidade N=18 → **19**;
  inventariar primeiro N=20 → **21** (ADR-0065 critério #5
  nona aplicação concreta com matriz multi-critério Opção A/B/C
  mais elaborada do passo); §análise de risco N=20 → **21**
  (P159F baixo-médio risco — refino comportamental + extensão
  infraestrutura state lookup + decisão arquitectural-chave
  multi-critério); ADR-0064 NÃO directamente aplicável (Opção C;
  sem field novo); subpadrão #15 N=2 → **3** (state.lang +
  state.bib_entries + state.bib_numbers; patamar atinge limiar
  formalização N=3-4); tipo entity em ficheiro próprio N=5
  (inalterado); P155 cross-feature N=1 (inalterado); refino
  tipo entity sem alteração Content N=1 (inalterado); refactor
  de field para Option N=1 (inalterado).
  **Política "sem novas reservas" preservada** — outras styles
  (alphanumeric, author-date, CSL), `Bibliography.style` field
  user-facing, numeração independente multi-Bibliography
  permanecem candidatos NÃO-reservados.
  **Marca conceptual**: Bloco A do diagnóstico P159B esgotado
  pós-P159F. Próximas direcções exigem Bloco B (hayagriva
  ADR-0062 promovida), Bloco C (cross-módulo), refinos Model
  fora do Bloco A original, mudança de módulo (Layout Fase 3
  columns/colbreak ou Introspection P160), ou passos
  administrativos XS.
- **Passo 158C — Refactor `kind: String → Option<String>` em
  `Content::Figure`** (quarto sub-passo Model figure-kinds após
  P158A auto-detect + P158B supplement por lang; **refactor
  cosmético** per ADR-0064 Caso A estrito; **subpadrão emergente
  N=1 NOVO** "refactor de field para Option" — primeiro Caso A
  em refactor não em variant aditivo). Field
  `Content::Figure.kind: String → Option<String>` em
  `01_core/src/entities/content.rs` (vanilla `Smart<Str>` →
  cristalino `Option<String>`; None ↔ Auto; default `"image"`
  resolvido em uso por callers via
  `kind.as_deref().unwrap_or("image")`, não em construção).
  **ADR-0064 Caso A patamar N=6 → 7** com primeiro Caso A "estrito"
  em refactor; distribuição cross-domínio passa de 50/50 para
  43/57 favorecendo Model (3 Layout + 4 Model). Stdlib
  `native_figure` (`figure_image.rs`) adaptado para retornar
  `Option<String>` directamente; `infer_kind_from_body` (P158A)
  já retornava `Option<String>` — sem alteração. ~10 sítios
  callers em `introspect.rs` (counters por kind) e
  `layout/mod.rs` (figure_progress + figure_numbers lookup)
  adaptados via `kind.as_deref().unwrap_or("image")` em uso.
  **Sem alteração observable** — backwards compat trivial via
  fallback nos callers; tests pré-existentes P157A/P158A/B
  preservados após adaptação de destructuring
  `Some("image".to_string())` em vez de `"image".to_string()`.
  **Decisão arquitectural-chave**: default resolvido em uso (não
  em construção) per Caso A canónico — alternativa
  ("default em construção") rejeitada por divergir do pattern
  estabelecido N=6. Tests +2 (1212 → 1214; 1 novo
  `figure_kind_auto_explicito_devolve_none` + 1 novo
  `introspect_figure_kind_none_resolve_para_image_no_counter`;
  range esperado +2-4) + ~5 tests existentes adaptados (asserts
  `kind.as_deref() == Some(...)` em vez de `kind == "..."`).
  Cobertura Model agregada **inalterada** (~50%) — refactor
  cosmético. Cobertura arquitectural **inalterada** 82%. Hash
  `entities/content.rs` preservado `ec58d849` — **14º passo
  consecutivo** (P156L → P158C; L0-baseline interpretation;
  spec previa preservação como regra default — lição P159A/C/D
  internalizada). Padrões pós-P158C: granularidade N=17 → **18**;
  inventariar primeiro N=19 → **20** (ADR-0065 critério #5
  oitava aplicação concreta com diversidade reforçada em refactor
  cosmético); §análise de risco N=19 → **20** (P158C baixo risco —
  refactor cosmético com pattern já validado N=6); ADR-0064
  Caso A N=6 → **7** (primeiro estrito em refactor; distribuição
  43/57 Layout/Model); tipo entity em ficheiro próprio N=5
  (inalterado); infraestrutura state lookup N=2 (inalterado);
  P155 cross-feature N=1 (inalterado); refino tipo entity sem
  alteração Content N=1 (inalterado); **subpadrão novo #17
  N=1** "refactor de field para Option" (precedente novo).
  **Política "sem novas reservas" preservada** — refactor
  análogo de outros String fields em Content variants, helper
  público `kind_or_default`, documentação completa de variants
  no L0 prompt content.md permanecem candidatos NÃO-reservados.
- **Passo 159D — `BibEntry` fields adicionais (Model
  bibliography+cite sub-passo 3)** (terceiro sub-passo
  substantivo de Bibliography + Cite após P159A par acoplado +
  P159C cite.form; **refino estrutural de tipo entity** sem
  alteração ao variant Content; **subpadrão emergente N=1**
  "refino de tipo entity sem alteração ao variant Content";
  **ADR-0065 critério #2 patamar N=2→3**). Struct entity
  `BibEntry` extendido em `01_core/src/entities/bib_entry.rs`
  com 4 fields universais opcionais (`volume`/`pages`/`journal`/
  `publisher`) per ADR-0065 critério #2 (terceira aplicação
  isolada concreta — selecção de fields universais; 4 fields
  escolhidos por universalidade cross-style + cobertura de
  classes de publicação distintas — journals/papers/books/
  manuals — vs alternativas `url`/`doi`/`editor` diferidas).
  **Builder pattern fluente** `with_volume()`/`with_pages()`/
  `with_journal()`/`with_publisher()` (Opção C diagnóstico §8;
  legibilidade superior + backwards compat trivial via
  `new()` original com 4 args preservado). Helper
  `extract_bib_entries` (P159A) extendido para parsing dos 4
  fields opcionais com validação tipo `Value::Str` e mensagem
  mencionando field específico. Helper privado novo
  `format_bib_entry` em `rules/layout/mod.rs` para concatenação
  condicional APA-like (`[key] author. title journal vol. volume,
  pp. pages. publisher (year).`). Backwards compat trivial —
  fields opcionais default `None` preservam output P159A
  exactamente. **Sem alteração ao variant `Content::Bibliography`**
  ou `Content::Cite`. **Decisão arquitectural-chave**: builder
  pattern (Opção C) vs `new_full(8 args)` (Opção B; rejeitada
  por verbosidade) vs field assignment directo (Opção A;
  rejeitada por construtor incompleto). Tests +8 (1204 → 1212;
  3 unit bib_entry incluindo builder pattern + PartialEq cobre
  8 fields + backwards compat + 3 stdlib parse + 2 layout E2E
  entry completa/mínima; range esperado +5-8). Cobertura Model
  agregada **inalterada** (~50%) — refino qualitativo. Cobertura
  arquitectural **inalterada** 82%. Hashes `entities/content.rs`
  e `entities/bib_entry.rs` ambos preservados via L0-baseline
  interpretation (**13º passo consecutivo content.rs**; spec
  P159D previa quebra bib_entry.rs mas extensão via doc-comment
  preserva). Padrões pós-P159D: granularidade N=16 → **17**;
  inventariar primeiro N=18 → **19** (ADR-0065 critério #5
  sétima aplicação concreta + critério #2 N=2→**3** terceira
  aplicação isolada concreta — selecção de fields universais);
  §análise de risco N=18 → **19** (P159D baixo risco — refino
  estrutural de tipo entity sem alteração de variant Content
  + reuso pattern); ADR-0064 NÃO directamente aplicável (fields
  são `Option<String>` directos sem mapping `Smart<T>`); tipo
  entity em ficheiro próprio N=5 (inalterado — `BibEntry`
  expande mas continua em `bib_entry.rs`); infraestrutura
  state lookup N=2 (inalterado); P155 cross-feature N=1
  (inalterado); **subpadrão novo #16 N=1** "refino de tipo
  entity sem alteração ao variant Content" (precedente novo).
  **Política "sem novas reservas" preservada** — fields
  restantes vanilla, tipos estruturados, CSL real, estilo
  configurável permanecem candidatos NÃO-reservados.
- **Passo 159C — `Cite.form` variants (Model bibliography+cite
  sub-passo 2)** (segundo sub-passo substantivo de Bibliography
  + Cite após P159A par acoplado; **refino estrutural-
  comportamental** de variant existente; **5ª aplicação
  consecutiva** do padrão "tipo entity em ficheiro próprio";
  **ADR-0064 Caso A patamar N=5→6 atinge equilíbrio cross-domínio
  50/50 Layout/Model**). Enum entity novo
  `CitationForm { Normal, Prose, Author, Year }` em
  `01_core/src/entities/citation_form.rs` (ficheiro novo;
  paridade Sides P156C → Parity P156E → Dir P156I → BibEntry
  P159A). Field `form: Option<CitationForm>` em `Content::Cite`
  per ADR-0064 Caso A — terceira aplicação Model do Caso A
  após P157B/P159A. **13 sítios pattern-match Content
  actualizados** (variant declaration + construtor + is_empty
  + plain_text + PartialEq + map_content + map_text +
  introspect.materialize_time/walk + layout arm + 2 stdlib
  testes + 2 content testes existentes). Helper privado novo
  `extract_citation_form` em `stdlib/structural.rs` (strict
  matching case-sensitive; 4 forms válidos; mensagem lista
  válidos). Field novo `pub bib_entries: Vec<BibEntry>` em
  `CounterState` para lookup Bibliography (**subpadrão
  emergente N=2** "infraestrutura state lookup" — P158B
  `state.lang` + P159C `state.bib_entries`); populado por
  introspect walk; multi-Bibliography concatena. Layout placeholder
  melhorado por form: `Normal/None ↔ [key]` (regression P159A);
  `Prose ↔ Author (Year)`; `Author ↔ Author`; `Year ↔ Year`;
  fallback `[key]` se key não encontrada. **Decisão arquitectural-
  chave**: lookup via CounterState (Opção C; reusa infraestrutura
  P158B) em vez de Layouter field ou second-pass — registada em
  diagnóstico §9. Sem alteração ao algoritmo cross-document
  (ADR-0017 bloqueia refs cross-document). L0 prompt novo
  `00_nucleo/prompts/entities/citation_form.md`. Tests +15
  (1189 → 1204; 8 unit + 2 cite com form + 6 stdlib parse +
  4 layout E2E forms; range esperado +12-17). Cobertura Model
  agregada **inalterada** (~50%) — refino qualitativo. Cobertura
  arquitectural **inalterada** 82%. Hash `entities/content.rs`
  preservado `ec58d849` — **décimo segundo passo consecutivo**
  (P156L → P159C; L0-baseline interpretation: prompt content.md
  não modificado; spec previa quebra mas interpretação L0 mantém
  preservação). Padrões pós-P159C: granularidade N=15 → **16**;
  inventariar primeiro N=17 → **18** (ADR-0065 critério #5
  sexta aplicação concreta + critério #2 N=2 segunda aplicação
  isolada concreta); §análise de risco N=17 → **18** (P159C
  baixo-médio risco — refino estrutural com decisão de tipo
  enum + lookup arquitectural); ADR-0064 Caso A patamar N=5→6
  com equilíbrio cross-domínio 50/50; tipo entity em ficheiro
  próprio N=5; infraestrutura state lookup N=1→2 (subpadrão
  novo); P155 cross-feature N=1 (inalterado).
  **Política "sem novas reservas" preservada** — forms vanilla
  adicionais (Full, CSL-specific), CSL render real (depende
  hayagriva ADR-0062), `style: Str` per-Cite, cross-document refs
  (bloqueado ADR-0017), promoção helper a público permanecem
  candidatos NÃO-reservados.
- **Passo 158B — Supplement automático por lang em figure
  (Model figure-kinds sub-passo 2)** (refino qualitativo
  consecutivo de `figure` após P158A; **segundo refino
  consecutivo de mesma feature Model**; **primeiro reuso
  explícito cross-feature do padrão P155** `localize_quotes`).
  Helper novo `figure_supplement_for_lang(kind: &str, lang:
  Option<&Lang>) -> String` em `01_core/src/rules/lang/
  figure_supplement.rs` (ficheiro novo paralelo a `quotes.rs`)
  cobrindo 6 langs (pt/en/de/fr/es/it) × 3 kinds (image/table/
  raw) = 18 entradas + fallback PT por kind + capitalização
  para kind desconhecido. Field novo `pub lang: Option<Lang>`
  em `entities/counter_state.rs::CounterState` para lang
  resolution (default `None` → fallback PT, paridade backwards
  compat com tests pré-existentes que esperam "Figura").
  Modificação trivial em `rules/introspect.rs` linha 334:
  `Some(format!("Figura {}", n))` →
  `Some(format!("{} {}", figure_supplement_for_lang(kind,
  lang), n))`. **Sem alteração ao variant `Content::Figure`**
  (estrutura inalterada). **Decisão arquitectural-chave**:
  fallback PT (não EN) para preservar backwards compat com
  tests pré-existentes — registada em diagnóstico P158B §2 +
  §8.2. **Subpadrão emergente N=1**: "padrão P155 i18n reusado
  cross-feature" — primeiro reuso (quotes → figure supplement);
  estrutura paralela tabela estática + lookup linear + fallback;
  candidato a formalização N=3-4 mínima. Tests +15 (1174 →
  1189; 8 unit + 7 integration; range esperado +12-15). Cobertura
  Model agregada **inalterada** (~50%) — segundo refino
  qualitativo consecutivo. Cobertura arquitectural **inalterada**
  82%. Hash `entities/content.rs` preservado `ec58d849` —
  **décimo primeiro passo consecutivo** (P156L → P158B).
  Padrões pós-P158B: granularidade N=14 → **15**; inventariar
  primeiro N=16 → **17** (ADR-0065 critério #5 quinta
  aplicação concreta); §análise de risco N=16 → **17** (P158B
  muito baixo risco); P155 cross-feature N=1 (subpadrão novo).
  **Política "sem novas reservas" preservada** — `supplement:
  Option<Content>` field user-facing, mais langs, CSL-aware
  format, region-specific supplements permanecem candidatos
  NÃO-reservados.
- **Passo `ADR-0062-create` — Criar ADR-0062 PROPOSTO
  (administrativo XS)** (passo administrativo formaliza reserva
  pré-existente de ADR-0062 — autorização de crate `hayagriva`
  para bibliography + cite CSL parsing — como ficheiro ADR
  concreto com status `PROPOSTO`; **não materializa código**;
  **não promove a `EM VIGOR` ou `IMPLEMENTADO`** — promoção
  ocorre em passo futuro de materialização hayagriva real
  (P159G ou equivalente)). **Subpadrão emergente N=1**: "passo
  administrativo XS criar ADR PROPOSTO a partir de reserva
  pré-existente" — primeiro do tipo nesta sessão; candidato
  a precedente para outras reservas se aplicável (e.g. ADR-0063
  reservada column flow). Ficheiro novo
  `00_nucleo/adr/typst-adr-0062-hayagriva-bibliography-parsing.md`
  com estrutura canónica (Status / Data / Contexto / Análise
  de pureza / Decisão / Precedentes citáveis / Crate hayagriva
  informação técnica / Consequências / Alternativas considera-
  das / Plano de promoção futuro / Referências). **Precedentes
  citáveis** (autorização crate externa em L1): ADR-0023
  (`indexmap`); ADR-0024 (`ecow`); ADR-0057 (`hypher`); padrão
  consolidado de "cada crate externa em L1 tem ADR dedicada
  com análise de pureza + justificação técnica + critério de
  promoção". **Justificação técnica primária**: vanilla integra
  `hayagriva` profundamente em `model/bibliography.rs` (1226
  linhas); CSL parsing cristalino do zero é trabalho
  desproporcionado vs reuso de hayagriva existente; hayagriva
  é mantida pela mesma organização que typst (autoridade
  máxima). **Alternativas consideradas**: implementar CSL
  cristalino (rejeitada — desproporcionalidade); manter subset
  minimal (P159A já implementou — insuficiente para paridade
  ADR-0060 ~68%); usar outra crate biblatex (rejeitada — menos
  mainstream). **Plano de promoção futuro**: P159G ou
  equivalente adiciona Cargo.toml + crystalline.toml; ADR-0062
  transita PROPOSTO → IMPLEMENTADO. **Política "sem novas
  reservas" preservada** — passo formaliza reserva pré-existente,
  não cria nova; reservas pré-existentes (ADR-0017 Introspection
  runtime) mantêm-se documentadas mas não reforçadas.
  Padrões pós-ADR-0062-create: granularidade N=14 (inalterada
  — administrativo); inventariar primeiro N=15 → **16** (ADR-
  0065 critério #1 naming + critério #5 inventário trivial em
  passo administrativo); §análise de risco N=15 → **16** (passo
  administrativo XS muito baixo risco). **Total ADRs 63 → 64**
  (paridade incremento P156K que adicionou ADR-0064/0065).
  Distribuição: PROPOSTO 11 → **12** (+0062); EM VIGOR 28
  inalterado; IMPLEMENTADO 19 inalterado. Hash
  `entities/content.rs` mantém `ec58d849` — **décimo primeiro
  passo consecutivo** (P156L → ADR-0062-create) sem alteração
  ao prompt L0 do content. **Implicação**: Bloco B do
  diagnóstico P159B agora pode iniciar com referência concreta
  a ADR-0062 PROPOSTO (em vez de referência a reserva sem
  ficheiro).
- **Passo 159B — Diagnóstico amplo expansão série 159 + tecto
  realista Model** (passo arquitectural de diagnóstico amplo;
  **não materializa código**; análogo estrutural a P156B —
  diagnóstico Layout amplo). **Quarta aplicação concreta de
  ADR-0065 critério #5** (scope determinado por inventário)
  com **diversidade ampliada multi-feature** após P157/P158/
  P159 (que inventariaram uma feature cada) — auto-validação
  cumulativa do ADR meta P156K continua a ganhar evidência
  empírica. Diagnóstico em
  `00_nucleo/diagnosticos/diagnostico-expansao-159-passo-159b.md`
  com 6 secções: §1 ADRs/DEBTs por família 159/158/157
  (DEBT-55 plano 6/10 itens pendentes pós-P159A; ADR-0062
  reserva sem ficheiro confirmada; ADR-0017 cite cross-document
  forward refs bloqueado mas walk single-pass viável; DEBT-34e
  + DEBT-56 cross-módulo); §2 inventário código pendente
  (campos diferidos vanilla por família — BibEntry adicional
  fields, Cite.form/style, Bibliography.style/full/lang,
  supplement automático, table cells refinos); §3 matriz de
  dependências cruzadas com 5 categorias (Introspection /
  multi-region / crate externa / ADR pendente / outro módulo);
  §4 tecto realista Model (cobertura agregada 50% → ~55-60%
  alcançável com Bloco A 5 sub-passos sem dependência cruzada
  hard; ~68% pós-resolver hayagriva ADR-0062; difícil estimar
  pós-cross-módulo); §5 sequência candidata sub-passos com 3
  blocos: **Bloco A** (5 refinos puramente Model — supplement
  figure/cite.form/BibEntry fields/kind refactor/bib numbering
  simples), **Bloco B** (5 refinos com hayagriva — pré-requisito
  ADR-0062 promovida), **Bloco C** (cross-módulo — DEBT-34e/
  DEBT-56/ADR-0017/show selectors); §6 recomendação concreta
  para passo seguinte: **P158B Supplement automático por lang
  em figure** (M; reuso padrão P155 quotes; hash content.rs
  preservado; funcionalidade visível "Figura"/"Figure"/
  "Abbildung"; sem dependência cruzada hard). **Recomendação
  secundária**: P159D BibEntry fields adicionais (S+; refino
  struct sem variant). **Recomendação terciária**: criar
  ADR-0062 PROPOSTO + actualizar L0 prompt content.md (passos
  administrativos XS). **Decisão crítica registada**: tecto
  Model puro vs pós-resolver dependências documentado com
  estimativas em §4 (Bloco A apenas: ~55-60%; +Bloco B
  hayagriva: ~68%; +Bloco C cross-módulo: difícil). **Política
  "sem novas reservas"** preservada (P158/A/P159/A/P159B
  respeitam) — recomendações §5/§6 são para validação humana,
  não compromissos. Padrões pós-P159B: granularidade N=14
  (inalterada — diagnóstico); inventariar primeiro N=14 →
  **15** (quarta aplicação concreta critério #5 com diversidade
  multi-feature); §análise de risco N=14 → **15** (passo
  diagnóstico baixo risco; escopo amplo M-). Total ADRs **63
  inalterado**; sem novas crates; sem novos DEBTs; sem alteração
  de hashes (passo documental amplo); cobertura Layout/Model/
  arquitectural inalteradas (78%/50%/82%); hash
  `entities/content.rs` mantém `ec58d849` — **décimo passo
  consecutivo** (P156L → P159B).
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

- **P249 (administrativo XS) — Criar ADR-0082 PROPOSTO
  "Promoções reais de scope-outs ADR-0054 graded — 4 critérios
  operacionais"** (paridade P156K + P160A + ADR-0062-create
  + P244 administrativos XS precedentes). **Sétima aplicação
  cumulativa "passo administrativo XS"** N=6 → 7 cumulativo
  (P156A historiograma + P156K ADRs meta + ADR-0062-create
  + P160A + P238 + P244 + **P249**). **Terceira aplicação
  cumulativa "ADR meta formalizar pattern N≥4 cumulativo"** N=2
  → 3 cumulativo (P156K Smart→Option N=6 + ADR-0064; P156K
  inventariar primeiro N=5 + ADR-0065; P234 L0 minimal N=7
  + ADR-0080; **P249 promoções reais N=8 + ADR-0082**).
  **Pattern empírico N=8 cumulativo granular** formalizado
  (P242 radius+clip + P247 outset+fill+stroke + P248
  breakable+height+cell_overflow); 4 critérios operacionais
  fixados (storage prévio + consumer Layouter graded + paridade
  vanilla referência + backward compat literal). **Status ADR-
  0082 PROPOSTO inicial** (paridade ADR-0065/ADR-0066/ADR-0080);
  promoção a EM VIGOR pendente N=3 aplicações consecutivas
  citantes (decisão humana). Anotação ADR-0054
  §"Promoções reais cumulativas" sub-secção nova adicionada
  (status `EM VIGOR` preservado literal — refino interno).
  Anotação ADR-0080 §"Lição refinada P249" N=12 cumulativo
  ("ADR meta administrativo XS exige audit empírico das N≥4
  aplicações concretas antes de formalizar pattern"). **Nota
  numeração**: `P249.div-2` registado — spec previa ADR-0067
  mas já ocupada por `attribute-grammar-scoping`; ADR-0082
  escolhido como próximo slot disponível após ADR-0081.
  ADRs distribuição: PROPOSTO → **+1** (ADR-0082); EM VIGOR
  preservado; IMPLEMENTADO preservado. **Sem código tocado**
  (zero L1/L2/L3/L4). **Sem L0 prompts tocados**. Tests
  workspace **preservado**. **Sem hashes propagados**.
  Cobertura Layout per metodologia **preservada**. 41ª
  aplicação cumulativa anti-inflação pós-P205D. Padrão
  emergente "Promoção real scope-out ADR-0054 graded" granular
  **N=8 preservado P249** (P249 administrativo XS não
  materializa nova promoção; apenas formaliza pattern).
  Patterns emergentes pós-P249 (3): "Passo administrativo XS"
  N=7 cumulativo; "ADR meta formalizar pattern N≥4
  cumulativo" N=3 cumulativo; "Spec C1 audit obrigatório
  bloqueante" N=11 → 12 cumulativo.

- **P253 (administrativo XS) — Promoção ADR-0079 Layout Fase 5
  roadmap `PROPOSTO → IMPLEMENTADO`** via Cenário A (scope-out
  formal humano paridade pattern ADR-0061 P221 + ADR-0060 P155
  precedentes). **Oitava aplicação cumulativa "passo
  administrativo XS"** N=7 → 8 cumulativo (P156A historiograma
  + P156K ADRs meta + ADR-0062-create + P160A + P238 + P244 +
  P249 + **P253**). **Cumprimento cumulativo P227-P252 ~14
  sub-passos granulares**: Categoria A 5/5 reforçada (Block A.4
  COMPLETO 10/10 + Boxed A.4 COMPLETO 6/6); Categoria B 1/3
  (B.2 placement P234); Categoria C 1.5/2 (C.1 P245 cumprida +
  C.2 P251 parcial cell-level γ-Items); Categoria D ~4/5-6
  (P236/P237/P240/P241 state+counter parciais). **Scope-outs
  formais P253** (decisão humana paridade ADR-0061 P221): C.2
  multi-region completo (Reabertura 2+3; DEBT-56b candidato
  não-aberto P158) + D.2-D.6 restantes prosseguem como roadmap
  pós-IMPLEMENTADO (paridade columns/colbreak ADR-0061; Fase
  2/3 ADR-0060). **Sub-padrão "ADR Fase X roadmap → IMPLEMENTADO
  via scope-out formal humano" N=2 → N=3 cumulativo P253**
  (ADR-0060 + ADR-0061 + **ADR-0079** — limiar formalização
  interno atingido; candidato a ADR meta futura). Anotação
  ADR-0061 §"Paridade pattern P221 IMPLEMENTADO precedente";
  anotação ADR-0080 §"Lição refinada P253" N=15 → 16 cumulativo
  ("promoção ADR roadmap → IMPLEMENTADO exige audit empírico
  cumulativo de sub-passos materializados antes de declarar
  critério satisfeito"); ADR-0082 §"Aplicações citantes" N=3
  limiar atingido P252 preservado (P253 administrativo
  não-citante; promoção EM VIGOR humana possível sequente).
  ADRs distribuição: PROPOSTO 13 → **12** (ADR-0079 sai);
  IMPLEMENTADO 23 → **24** (ADR-0079 entra); EM VIGOR 29
  preservado; **total 69 preservado**. **Sem código tocado**
  (zero L1/L2/L3/L4). **Sem L0 prompts tocados**. Tests
  workspace **2304 verdes preservado**. **Sem hashes
  propagados** ("Nothing to fix"). Cobertura Layout per
  metodologia **~98-99% preservado**. **45 aplicações cumulativas
  anti-inflação** pós-P205D. Patterns emergentes pós-P253 (3):
  "Passo administrativo XS" N=8 cumulativo; "ADR Fase X roadmap
  → IMPLEMENTADO via scope-out formal humano" N=3 cumulativo
  (limiar interno atingido); "Spec C1 audit obrigatório
  bloqueante" N=15 → 16 cumulativo. **Marco P253**: terceira
  ADR roadmap a transitar IMPLEMENTADO (ADR-0060 + ADR-0061 +
  ADR-0079); patamar conceptual máximo Layout Fase 5 atingido.

- **P254 (administrativo XS) — Promoção ADR-0082 (Promoções
  reais scope-outs ADR-0054 graded — 4 critérios operacionais)
  `PROPOSTO → EM VIGOR`** via passo administrativo XS dedicado
  pós-N=3 citantes consecutivos (paridade pattern P229 ADR-0080
  PROPOSTO→EM VIGOR precedente directo). **Nona aplicação
  cumulativa "passo administrativo XS"** N=8 → 9 cumulativo
  (P156A historiograma + P156K ADRs meta + ADR-0062-create +
  P160A + **P229 ADR-0080→EM VIGOR** + P238 + P244 + P249 +
  P253 + **P254**). **Segunda aplicação cumulativa sub-padrão
  "ADR meta PROPOSTO → EM VIGOR via passo admin XS dedicado"**
  N=1 → 2 cumulativo (P229 ADR-0080 + **P254 ADR-0082**).
  **N=3 citantes ADR-0082 atingidos P252** (P250 N=1 Block 4
  scope-outs + P251 N=2 TableCell row break γ-Items + P252 N=3
  Boxed stroke-overhang sequente consecutivo). Critério literal
  N=3 satisfeito; **paridade ADR-0065 P156K via P156J/P157A/
  P157B sequente** validação retroactiva (ADR-0082 P249
  inverteu template: PROPOSTO inicial + critério literal N=3
  citantes explícito documentado no próprio ADR meta antes de
  promover). **Status pós-P254 ADR-0082**: `EM VIGOR` — pattern
  "Promoções reais scope-outs ADR-0054 graded" formalizado como
  regra vinculativa operacional cristalina. Aplicações futuras
  citam ADR-0082 EM VIGOR (em vez de re-justificar empíricamente
  os 4 critérios operacionais). Anotação ADR-0080 §"Lição
  refinada P254" N=16 → 17 cumulativo ("promoção ADR meta
  PROPOSTO → EM VIGOR exige confirmação empírica de critério
  N=3 citantes documentado literal no próprio ADR meta antes
  de promover") + sub-padrão "ADR meta PROPOSTO → EM VIGOR via
  passo admin XS dedicado" N=2 cumulativo; anotação ADR-0065
  preservativa "pattern N=3 citantes ADR-0082 inspirado em
  validação retroactiva ADR-0065". ADRs distribuição: PROPOSTO
  12 → **11** (ADR-0082 sai); EM VIGOR 29 → **30** (ADR-0082
  entra); IMPLEMENTADO 24 preservado; **total 69 preservado**.
  **Sem código tocado** (zero L1/L2/L3/L4). **Sem L0 prompts
  tocados**. Tests workspace **2304 verdes preservado**.
  **Sem hashes propagados** ("Nothing to fix"). Cobertura
  Layout per metodologia **~98-99% preservado**. **46 aplicações
  cumulativas anti-inflação** pós-P205D. Patterns emergentes
  pós-P254 (3): "Passo administrativo XS" N=8 → 9 cumulativo;
  "ADR meta PROPOSTO → EM VIGOR via passo admin XS dedicado"
  N=1 → 2 cumulativo; "Spec C1 audit obrigatório bloqueante"
  N=16 → 17 cumulativo. **Marco P254**: ADR-0082 EM VIGOR —
  pattern "Promoções reais scope-outs ADR-0054 graded"
  formalizado como regra vinculativa cristalina; segunda
  aplicação cumulativa sub-padrão "ADR meta PROPOSTO → EM VIGOR
  via passo admin XS dedicado" (P229 + P254 = N=2 cumulativo).

- **P257 — Color paridade vanilla (Leitura B funcional)** via
  refactor cross-cutting `Color { Rgb, Rgba }` (P25 simplificado)
  → enum tagged 8 variantes paridade vanilla literal
  (Srgb/Luma/LinearRgb/Oklab/Oklch/Hsl/Hsv/Cmyk). **Cumpre
  ADR-0029 §"Diagnosticar primeiro" + §"Simplificações aceites
  apenas com ADR explícita"** via ADR-0083 nova (PROPOSTO P257.B
  → IMPLEMENTADO P257.D no mesmo passo). **Magnitude real M
  (~3-4h)** face M-L estimada. Sub-passos sequenciais:
  P257.A diagnóstico vanilla imutável
  (`diagnostico-color-vanilla-passo-257.md`); P257.B ADR-0083
  PROPOSTO com 4 scope-outs documentados (PDF native CMYK +
  operadores lighten/darken/mix + ColorSpace runtime +
  constantes nomeadas extras); P257.C L0 prompt
  `entities/color.md` + 8 variantes em
  `01_core/src/entities/color.rs` (~500 LoC com PartialEq
  bitwise + conversões Oklab/HSL/HSV/etc. + 22 unit tests) +
  remoção Color de `layout_types.rs` (re-export) + 5 adaptações
  consumers + 7 stdlib funcs novas (`native_oklab`,
  `native_oklch`, `native_linear_rgb`, `native_cmyk`,
  `native_hsl`, `native_hsv` + refactor `native_luma` para
  `Color::Luma`) registadas em eval scope + 8 unit stdlib tests;
  P257.D promoção ADR-0083 IMPLEMENTADO. **Tests: 2304 → 2334
  verdes (+30)**. **Paridade observable estricta preservada**:
  `Color::rgb(255,0,0).to_srgb() == (255, 0, 0, 255)`; PDF
  output bit-equivalente. **PDF exporter intocado**
  estructuralmente (4 caminhos `to_rgba_f32` preservados; novos
  espaços convertem para sRGB transparentemente via
  `Color::to_srgb()`). Anotação implícita ADR-0028 (REVOGADA;
  P257 confirma revogação via materialização da regra ADR-0029).
  ADRs distribuição: PROPOSTO 11 preservado (ADR-0083 entra e
  sai); EM VIGOR 30 preservado; IMPLEMENTADO 24 → **25**
  (+ADR-0083); **total 69 → 70**. Hash propagado
  (`entities/color.md` → `7188e8d9`). **45 aplicações cumulativas
  anti-inflação** pós-P205D preservadas (Opção α nova ADR
  PROPOSTO+IMPLEMENTADO mesmo passo via Cenário B1; Opção β L0
  prompt novo dedicado; Opção α subset materializado per
  ADR-0029). Patterns emergentes pós-P257 (2): "ADR PROPOSTO+
  IMPLEMENTADO no mesmo passo via Cenário B1" N=2 cumulativo
  (P257 cumpre paridade pattern P253 cenário A — ADR cria-se
  PROPOSTO e logo promove-se quando materialização cumpre
  critério); "Refactor cross-cutting entity primitivo
  Color/Stroke" N=1 → N=2 cumulativo (P252 Stroke + **P257
  Color**). **Marco P257**: Color paridade vanilla materializada
  cumprindo regra ADR-0029 literal; Visualize ganha primeira
  expansão substantiva user-facing pós-Fase 5 Layout
  IMPLEMENTADO P253.
