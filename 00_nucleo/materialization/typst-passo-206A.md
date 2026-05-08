# Passo 206A — Diagnóstico-primeiro de vanilla integration (DEBT-53/54)

**Série**: 206 (sub-passo `A` = diagnóstico-primeiro
formal). 39ª aplicação consecutiva do padrão.
**Tipo**: diagnóstico-primeiro (zero código tocado) +
auditoria empírica de profundidade alta.
**Magnitude planeada**: M (M auditoria + S diagnóstico).
**Pré-condição**: P205E concluído; F3 completo final;
M8 estruturalmente fechado; ADR-0073 ACEITE; ADR-0074
ACEITE; tests 1860 verdes; 0 violations; 21 sentinelas
activas; blueprint anotado §3.0 e §3.0bis.
**Output**: 4 ficheiros (auditoria + diagnóstico +
relatório + ADR-0075 PROPOSTO se C9 for afirmativa).

---

## §1 Propósito

Fixar as decisões estruturais de vanilla integration
antes de qualquer alteração de código.

A série P206 endereça **DEBT-53/54** identificado
empíricamente em P204F (`P204F.div-1`) e P204H §6 — o
harness vanilla em `lab/parity/` está não-funcional, o
que impediu a condição 9 de ADR-0073 (sanity-check
cristalino vs vanilla observable) de ser cumprida.

P206A produz:

1. Auditoria empírica do estado actual do harness.
2. Auditoria das dependências necessárias (vanilla
   typst binário/CLI, PDF diff tools, etc.).
3. Cláusulas de decisão (C1–Cn) sem condicionais.
4. ADR-0075 PROPOSTO **se C9 for afirmativa**.
5. Plano de sub-passos `*B+` sem ramos.

P206A respeita o padrão: cada passo começa com
inventário empírico antes de qualquer decisão.

---

## §2 Tensão consciente entre os dois inputs

A clarificação inicial fixou:

- **Tipo de trabalho**: decisão no diagnóstico (reactivar
  vs construir do zero).
- **Escopo de validação**: pré-fixado em "todo o corpus
  36 ficheiros".

A tensão a registar:

- "Reactivar" pode ser S–M se pre-existing breaks forem
  triviais; pode ser L se vanilla typst binário não
  estiver acessível ou se as dependências forem
  pesadas.
- "Construir do zero" pode ser M se houver bom design
  base; pode ser XL se exigir infraestrutura nova
  (binários, fixtures, PDF diff tools).
- Escopo "36 ficheiros" pré-fixado **aumenta custo
  sob ambas as abordagens**, mas o multiplicador depende
  da abordagem.

P206A audita primeiro (C1) e fixa abordagem (C2) com
base em evidência. Se o custo agregado para 36 ficheiros
for desproporcional, P206A pode **registar `P206A.div-N`
sugerindo redução de escopo** apesar do escopo ter sido
pré-fixado.

A pré-fixação do escopo não é absoluta — é guidance
para o diagnóstico. Se a evidência empírica mostrar que
36 ficheiros é inviável sem inflação XL, o diagnóstico
deve dizê-lo honestamente em vez de inflar.

---

## §3 Cláusulas de auditoria (A1–An)

Esta secção é executada **primeiro**. Output empírico
alimenta C1+ adiante. Cada item reporta CONFIRMADO /
DIVERGÊNCIA / NÃO APLICÁVEL com evidência.

### Bloco 1 — Estado actual do harness

#### A1 — Estrutura `lab/parity/`

```
ls -la lab/parity/
ls -la lab/parity/src/
ls -la lab/parity/tests/
```

Para cada ficheiro: nome, tamanho, data de modificação,
descrição se houver doc-comment.

Critério: estrutura mapeada literalmente.

#### A2 — Pre-existing breaks identificados em P204F

P204F.div-1 listou 2 breaks em `lab/parity/`:

- `tests/layout_parity.rs:69` — `layout(content, state)`
  signature outdated (P190I migrou para
  `layout(content)`).
- `src/value_dto.rs:83` — missing `Value::Location(_)`
  arm (P179 adicionou variant).

Confirmar empíricamente:
- Os 2 breaks ainda existem?
- Há outros breaks não documentados?
- Compilam parcialmente ou falham completamente?

Critério: estado de cada break + lista exaustiva de
problemas de compilação.

#### A3 — Driver `parity-runner`

Per P204F C1: `lab/parity/Cargo.toml` declara binary
`parity-runner`. Investigar:

- Como é invocado (CLI args)?
- Que produz (output)?
- Está funcional, parcial, ou broken?

Critério: estado do driver com evidência de execução
(`cargo run --bin parity-runner -- --help` ou similar).

#### A4 — `layout_parity.rs` test harness

Per P204F: harness é cristalino-only baseline (per
doc-comment). Investigar:

- Que testa actualmente?
- Que assertions tem?
- Como itera sobre o corpus?
- Onde estaria o ponto de comparação vanilla (que
  está deferred per DEBT-53)?

Critério: estrutura do harness + ponto de extensão
vanilla identificado.

### Bloco 2 — Dependências vanilla

#### A5 — Vanilla typst binário

Investigar:

- `lab/typst-original/` está em quarentena (per
  CLAUDE.md / blueprint).
- É possível compilar vanilla typst nessa quarentena?
- Existe binário pré-construído?
- Que versão de vanilla é (release tag)?

Critério: estado do vanilla com evidência (`cargo build
--manifest-path lab/typst-original/Cargo.toml` ou
similar).

#### A6 — Workspace setup vs quarentena

DEBT-54 é "vanilla workspace setup". Investigar
literalmente:

- Vanilla é workspace member ou external?
- Há conflitos de Cargo.lock se ambos forem
  workspace members?
- Como cristalino e vanilla coexistem actualmente?

Critério: arquitectura literal documentada.

#### A7 — PDF diff tools

Investigar:

- Que ferramentas estão disponíveis (ImageMagick,
  pdftocairo, poppler, mupdf)?
- Cristalino tem dependência declarada para alguma?
- Convenção de tolerância (pixel-perfect, fuzzy match)?

Critério: ferramentas confirmadas com versão; convenção
de tolerância identificada (ou registado que precisa
ser fixada em P206).

#### A8 — Comparação estrutural (queries)

Cristalino expõe `Introspector::query_*` métodos
(P204B). Vanilla expõe `typst query` CLI. Investigar:

- Output formato do `typst query` (JSON, plaintext)?
- Output formato do cristalino (precisa novo CLI ou
  test helper)?
- Como comparar (JSON diff, custom)?

Critério: viabilidade de comparação estrutural com
ferramentas existentes.

### Bloco 3 — Corpus actual

#### A9 — Listagem dos 36 ficheiros corpus

```
ls -R lab/parity/corpus/
```

Por categoria (`code/`, `markup/`, `math/`, `semantic/`,
`visual/`):
- Quantos ficheiros.
- Features cobertas.
- Quais foram adicionados em P204F (6 introspection).
- Quais são pre-existing P155–P204.

Critério: tabela 36 linhas × categoria × pre-P204F /
P204F.

#### A10 — Companions `.toml` actuais

Para cada `.typ`, confirmar se há companion `.typ.toml`
e que metadados expõe. Per P204F: companions têm
`features`, `modo_p3`, `notes`. P204F adicionou
`[expectations.cristalino]`.

Critério: formato das companions actual; gap para
expectations vanilla.

#### A11 — Features que cristalino ainda não suporta

Per P204F: `here()`/`locate()` ainda não em stdlib
cristalino (SKIP `here-locate.typ`). Auditar:

- Que outras features `.typ` no corpus podem falhar
  em cristalino?
- Que outras features podem falhar em vanilla (versão
  diferente, etc.)?

Critério: lista honesta de gaps por ficheiro.

### Bloco 4 — Caminhos viáveis

#### A12 — Custo "Reactivar harness"

Estimar:

- Custo de fixar `layout_parity.rs:69` (signature
  update).
- Custo de fixar `value_dto.rs:83` (Location arm).
- Outros breaks descobertos.
- Custo de adicionar vanilla CLI invocation.
- Custo de adicionar PDF diff.
- Custo de cobrir 36 ficheiros (multiplicador).

Critério: estimativa S/M/L/XL com decomposição.

#### A13 — Custo "Construir do zero"

Estimar:

- Custo de design base (driver + harness + fixture).
- Custo de vanilla CLI invocation.
- Custo de PDF diff.
- Custo de cobrir 36 ficheiros.
- Reuso vs reescrita.

Critério: estimativa S/M/L/XL com decomposição.

#### A14 — Comparação A12 vs A13

Critério: tabela comparativa com pontos fortes e fracos
de cada caminho.

### Bloco 5 — Estado pós-M8+F3 e contexto

#### A15 — Condição 9 de ADR-0073 (referência)

Per P204H §1: cond 9 está PARCIAL ("Saída cristalino
sanity-check vs vanilla nos 5-7 ficheiros corpus
paridade — sem regressões observable"). P206 endereça
essa condição.

Confirmar:
- Cond 9 ainda PARCIAL pós-F3.
- Que mudou em F3 que pode afectar a forma da validação
  (Position concrete, sealing, etc.).
- Se cumprir cond 9 transita ADR-0073 de "estruturalmente
  fechado" para "completo final" — ou se a transição
  é mais subtil.

#### A16 — Outras condições/expectativas pendentes

Investigar:

- Há outras pendências da série M8+F3 que P206 deve
  cobrir?
- Há expectativas implícitas (ex: paridade observable
  desktop vs CI) que merecem documentação?

---

## §4 Cláusulas de decisão (C1–Cn)

Estas cláusulas são fixadas **depois** da auditoria, com
base no output empírico. Cada uma é fixada sem
condicionais.

### C1 — Tipo de trabalho

Decisão fixada com base em A12 + A13 + A14:

- **Caminho A — Reactivar** — fixar pre-existing breaks
  + estender com vanilla CLI invocation.
- **Caminho B — Construir do zero** — design novo;
  legacy harness mantido como referência.
- **Caminho C — Híbrido** — fixar breaks principais;
  reescrever partes críticas.

C1 fixa **uma** alternativa.

### C2 — Escopo concreto de validação

Pré-fixado pela clarificação inicial: **todo o corpus
36 ficheiros**.

Mas C2 pode registar **excepções honestas**:

- Ficheiros que cristalino não suporta (SKIP
  documentado, não silenciado).
- Ficheiros que vanilla não suporta na versão
  acessível.
- Ficheiros que requerem features fora do escopo de
  P206 (ex: features de stdlib não materializadas).

Critério: lista das 36 entradas com etiqueta INCLUDE /
SKIP-documentado / DEFERRED + razão.

C2 não reduz o escopo nominal; documenta excepções.

### C3 — Mecanismo de comparação observable (PDF diff)

Com base em A7:

- **Mecanismo A — pixel-perfect** com tool específico
  (ImageMagick `compare`).
- **Mecanismo B — fuzzy match** com tolerância.
- **Mecanismo C — sem comparação observable** (apenas
  estrutural; cristalino-only).

C3 fixa **uma**.

### C4 — Mecanismo de comparação estrutural (queries)

Com base em A8:

- **Mecanismo D — vanilla `typst query` + cristalino
  test helper**.
- **Mecanismo E — sem comparação estrutural** (apenas
  observable).
- **Mecanismo F — comparação dual** (D + observable
  per C3).

C4 fixa **uma**.

### C5 — Vanilla typst binário acesso

Com base em A5 + A6:

- **Caminho a — Compilar vanilla na quarentena** durante
  CI/local.
- **Caminho b — Pre-built binário versionado**.
- **Caminho c — Vanilla como workspace member**
  (controverso; resolveria DEBT-54 mas pode introduzir
  conflitos).

C5 fixa **uma**.

### C6 — Tolerância de comparação

Se C3 = A ou B:

- Pixel-perfect (zero diff): para regression detection
  estricto.
- Fuzzy threshold: para rendering differences aceitáveis.

C6 fixa valor concreto se C3 ≠ C.

### C7 — Categorias de SKIP documentado

Per A11: que ficheiros são SKIP por feature missing?

Lista literal + razão por ficheiro. Documentado em
companion `.typ.toml` ou em manifest dedicado.

### C8 — Tratamento de DEBT-53/54

Decisão sobre como DEBT-53/54 são fechadas:

- Fechadas em P206 (uma série única).
- Fechadas em séries separadas (P206 = harness; P207 =
  workspace setup, etc.).
- Mantidas como DEBT documentada (se P206 = parcial).

C8 fixa fecho.

### C9 — ADR-0075 PROPOSTO?

Decisão: criar ADR-0075 para vanilla integration ou
não.

Critério:

- **Sim** — vanilla integration é decisão arquitectural
  com alternativas reais (reactivar vs construir;
  workspace member vs quarentena; tolerâncias).
- **Não** — documentação inline em ADR-0073/0074
  (extensão).

Hipótese mais provável: criar ADR-0075. Cada marco
arquitectural com decisão estrutural ganhou ADR
dedicada (0072 M7, 0073 M8, 0074 F3). Vanilla
integration é estrutural.

### C10 — Magnitude agregada

Soma das decisões C1–C9. Range plausível: M (escopo
mínimo Caminho A simples) a XL (Caminho B + 36
ficheiros + workspace setup).

C10 é output. Não pré-fixada.

Hipótese específica: se A12 ou A13 mostrarem custo XL,
C10 obriga a sub-séries (P206/P207/...) em vez de série
única.

### C11 — Sub-passos `*B+`

Plano sem ramos. Quantidade depende de C1–C10.

### C12 — Sem cláusulas condicionais

C1–C11 fixadas com valores concretos.

### C13 — Possível `P206A.div-N` sobre escopo

Se A12 + A13 + C10 mostrarem que "36 ficheiros" é
inviável sem inflação XL, registar `P206A.div-N`:

- Recomendar redução de escopo a 6 ficheiros (P204F
  introspection) inicialmente.
- Documentar fundamento empírico (comparação custo vs
  valor).
- Solicitar decisão ao humano antes de prosseguir
  P206B.

A pré-fixação do escopo na clarificação inicial não
absorve obrigação de inflar quando empírico mostra
inviabilidade.

---

## §5 Outputs concretos

Quatro ficheiros (3 sempre + 1 condicional em C9):

### Ficheiro 1 — Auditoria empírica

Localização:
`00_nucleo/diagnosticos/typst-passo-206A-auditoria-vanilla.md`.

### Ficheiro 2 — Diagnóstico

Localização:
`00_nucleo/diagnosticos/typst-passo-206A-diagnostico.md`.

### Ficheiro 3 — Relatório

Localização:
`00_nucleo/materialization/typst-passo-206A-relatorio.md`.

### Ficheiro 4 — ADR-0075 PROPOSTO (condicional em C9)

Localização (se C9 = afirmativa):
`00_nucleo/adr/typst-adr-0075-vanilla-integration.md`.

Estado: PROPOSTO. Estrutura per `template-adr.md`.

---

## §6 Critério de progressão para `*B`

P206A só transita para `*B` quando:

- A1–A16 todos com etiqueta CONFIRMADO ou DIVERGÊNCIA.
- C1–C12 instanciadas com valores concretos.
- ADR-0075 PROPOSTO escrito (se C9 afirmativa).
- Magnitude calibrada (C10).
- Plano `*B+` sem condicionais.
- (Se C13) Divergência sobre escopo registada com
  recomendação ao humano.

Em caso de divergência empírica relevante, registar em
`P206A.div-N` e:

- Resolver dentro de P206A se trivial (ex: ferramenta
  alternativa disponível).
- Recuar para humano se afecta escopo (ex: 36 ficheiros
  inviável; sugerir redução).

---

## §7 Convenções mantidas

- Sem código nas specs.
- Sem condicionais.
- Cada passo começa com inventário empírico.
- 4 outputs (3 sempre + 1 condicional).
- Localização canónica.
- Sem inflação retórica.

---

## §8 Não-objectivos

P206A não:

- Toca em código.
- Fixa pre-existing breaks (P206B+).
- Compila vanilla typst.
- Implementa harness novo.
- Executa comparação cristalino vs vanilla.
- Modifica corpus existente (P204F já fechou).
- Promove ADR-0073 ou 0074 (já ACEITES).
- Pré-define sub-passos `*B+` (esses emergem de C11).
- Decide tipo de trabalho antes da auditoria — C1
  fixa-se com base em A12+A13+A14.
- Reduz escopo abaixo de 36 ficheiros sem registar
  divergência empírica fundamentada.

---

## §9 Erro a não repetir

Da série P204F — `P204F.div-1` revelou empíricamente
que harness vanilla está broken. Eu (na spec P204F)
assumi observable harness funcional sem verificar.
P206A não repete: A1–A4 audita o estado actual antes
de qualquer decisão.

Risco específico de P206A: **inflar escopo para honrar
"36 ficheiros" pré-fixado quando empírico mostra
inviabilidade**. C13 antecipa isto e legitima a
divergência. Pré-fixar não é constrangimento absoluto
— é guidance.

Outro risco: **subestimar custo de "Construir do zero"
porque parece mais limpo**. Reactivar é geralmente mais
barato que reconstruir, mesmo quando o legacy é
imperfeito. C1 fixa-se com base em estimativa
empírica, não preferência estética.

Hipótese mais provável: C1 = Caminho A (Reactivar) ou
C1 = C (Híbrido). Caminho B (Construir do zero) só se
A2 revelar que `lab/parity/` está irrecuperável.

Hipótese específica: A5 pode revelar que vanilla typst
não compila trivialmente na quarentena (pode exigir
features específicas, dependencies pesadas). Isto pode
empurrar C5 para "pre-built binário" em vez de
"compilar na quarentena".

---

## §10 Particularidade — execução

P206A é diagnóstico de profundidade alta:

- Estrutura `lab/parity/` com tests + driver.
- Vanilla typst quarentena.
- PDF diff tools disponibilidade.
- 36 ficheiros corpus + companions.
- Estimativa custo Caminho A vs B.

Volume médio-alto. Magnitude M.

Recomendado Claude Code dado:

- Volume de leitura para A1–A14 (lab/parity inteiro +
  vanilla quarentena).
- Tentativa de compilação vanilla (A5) que pode falhar
  empíricamente.
- Decisão arquitectural em C1 que beneficia de
  comparação detalhada.

Sessão actual viável apenas se houver tempo
significativo + acesso a ferramentas externas (PDF
diff). Caso contrário, Claude Code é mais apropriado.
