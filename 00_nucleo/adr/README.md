# Índice de ADRs do Typst Cristalino

Este documento é o índice canónico dos Architectural Decision
Records (ADRs) do projecto **Typst Cristalino**. Lista os 56 ADRs
em vigor (55 números únicos; ADR-0026 tem variante -R1 por
revisão), as meta-regras que governam o projecto, o vocabulário
canónico de status, cadeias de revogação e revisão, e convenções
estruturais.

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
| 0038 | Sistema de estilos em L1 (`Style`, `Styles`, `StyleChain`) | `EM VIGOR` ¹ |
| 0039 | Forma de estilo no `FrameItem::Text` | `EM VIGOR` ¹ |
| 0040 | Activação de `#set` em eval | `EM VIGOR` ¹ |
| 0041 | Activação de `#show` — heading, strong, emph | `EM VIGOR` ¹ |
| 0042 | `Sink` materializado em L1 | `EM VIGOR` ¹ |
| 0043 | Canal de saída do `Sink` (TrackedMut + L3) | `EM VIGOR` ¹ |
| 0044 | `Engine<'a>` agregador em L1 | `EM VIGOR` ² |
| 0045 | Formato de diagnósticos (resolução L1, formatação L3) | `EM VIGOR` ² |
| 0046 | CLI mínima (compile com diagnostics) | `EM VIGOR` ² |
| 0047 | Argparsing com `clap` na CLI | `EM VIGOR` ² |
| 0048 | Cores ANSI nos diagnósticos | `EM VIGOR` ² |
| 0049 | CLI vive em L2 (correcção de 0046/0047/0048) | `EM VIGOR` ² |
| 0050 | Formatter de diagnósticos em L2 (completa 0049) | `EM VIGOR` ² |
| 0051 | Flags funcionais em L2 — pattern e `-o` | `EM VIGOR` ² |
| 0052 | Lang como tipo semântico em L1 | `IMPLEMENTADO` |
| 0053 | Font como tipo composto em L1 | `IMPLEMENTADO` |
| 0054 | Critério de fecho de DEBT-1 inclui consumo integral | `EM VIGOR` |
| 0055 | Font consumer via pipeline CIDFont existente | `IMPLEMENTADO` |

**Notas de irregularidade no cabeçalho (Passo 143)** — não
afectam o status canónico mas indicam dívida documental
candidata a passo de uniformização futura:

- **¹** ADR-0038 a 0043 declaram o status no formato
  `**Status**: EM VIGOR (Passo NN.E) — validado empiricamente
  com NNN testes`, sem backticks no valor canónico.
- **²** ADR-0044 a 0051 usam `**Estado**:` em vez de
  `**Status**:` — vocabulário antigo (pré-P84.8g) preservado
  no cabeçalho. Status canónico inferido por leitura
  conservadora (`EM VIGOR` sempre presente como prefixo).

Detalhe completo no relatório do passo:
[`materialization/typst-passo-143-relatorio.md`](../materialization/typst-passo-143-relatorio.md).

**Total**: 56 ADRs (55 números únicos; ADR-0026 tem variante -R1
por revisão).

### Distribuição de status

- `PROPOSTO`: 10 ADRs (decisões em aberto: 0005, 0006,
  0008–0015).
- `IDEIA`: 2 ADRs (0002, 0003).
- `EM VIGOR`: 24 ADRs (regras/políticas activas; 0018, 0029,
  0030, 0032–0051, 0054).
- `IMPLEMENTADO`: 17 ADRs (decisões materializadas; 0001, 0004,
  0016, 0017, 0019, 0021–0027, 0026-R1, 0031, 0052, 0053, 0055).
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
