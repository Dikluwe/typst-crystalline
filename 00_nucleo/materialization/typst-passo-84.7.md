# Passo 84.7 — Auditoria dos ADRs do Typst

## Estado actual antes de começar

Ler antes de começar:
- `00_nucleo/adr/typst-adr-*.md` — todos os ADRs do Typst, de
  `typst-adr-0001` a `typst-adr-0031`.
- `00_nucleo/adr/template-adr.md` — template usado para novos ADRs.
- `00_nucleo/DEBT.md` — estado actual pós-Passo 84.6 para cross-reference.
- Passos 84.1 a 84.6 deste bloco — contêm decisões de design que podem
  revelar lacunas nos ADRs.

Pré-condição: `cargo test` — 911 testes (737 L1 + 174 L3, 6 ignorados
pré-existentes), zero violations. Passo 84.6 concluído. DEBT-37
encerrado.

---

## Âmbito

**Auditar apenas os ADRs do Typst** (prefixo `typst-adr-` no nome do
ficheiro). ADRs do `crystalline-lint` que existam no mesmo directório
por razões históricas não são tocados neste passo — pertencem a outro
projecto que partilha o padrão arquitectural mas tem ciclo de vida
próprio.

Critério de identificação: ficheiros em `00_nucleo/adr/` cujo nome
começa com `typst-adr-`. Ficheiros em `00_nucleo/adr/` sem esse prefixo
(ex: `0001-tree-sitter-intermediate-repr.md`) são explicitamente
**ignorados** neste passo — mesmo que mencionem ADRs por número
genérico sem prefixo, a auditoria resolve "ADR-NNNN" como sempre
referindo `typst-adr-NNNN.md`.

---

## Natureza deste passo

Replicar o padrão do Passo 83.5 (auditoria dos DEBTs):

1. **Auditar** cada ADR do Typst com comandos concretos de verificação.
2. **Produzir relatório** em formato estruturado listando:
   - ADRs onde o estado documentado não corresponde à realidade do
     código.
   - ADRs com lacunas de conteúdo.
   - Oportunidades de consolidação (meta-regras implícitas que
     deveriam ser explícitas, índice de precedência, etc.).
3. **Não alterar** os ADRs neste passo. As correcções serão
   executadas num passo dedicado subsequente (análogo ao 84.1 em
   relação ao 83.5).

**Regra absoluta**: Claude Code **não edita nenhum ficheiro em
`00_nucleo/adr/`** neste passo. Apenas lê e produz relatório. Se
durante a auditoria descobrir um ADR com erro óbvio (ex: typo no
número de versão), anotar no relatório, não corrigir.

---

## Inputs explícitos — achados desta série 84.x

Os seguintes achados emergiram durante os Passos 84.1–84.6 e **devem**
ser incluídos no relatório, independentemente do que a auditoria
encontrar adicionalmente:

### Input 1 — `Alignment` em falta em ADR-0029

ADR-0029 lista explicitamente `Length, Abs, Rel, Angle, Ratio, Color`
como tipos tipográficos que seguem a arquitectura vanilla sem
simplificações. O Passo 84.5 (DEBT-36, `Value::Align`) revelou que
`Alignment` é do mesmo género mas não está na lista.

Resposta da auditoria deve incluir: recomendação de actualizar
ADR-0029 para listar explicitamente `Alignment` (ou uma formulação
genérica "qualquer tipo tipográfico").

### Input 2 — Distinção entre clone profundo e `Arc::clone`

Durante o Passo 84.4 (DEBT-22), identificou-se que a ambiguidade da
palavra "clone" em Rust causa confusão em LLMs:

- `Vec<T>::clone()` → copia profunda O(n) — proibida no hot path.
- `Arc<T>::clone()` → incremento de contador O(1) — obrigatória e
  exigida em L1.

O ADR-0030 diz textualmente "Um compilador que copia árvores O(n)
quando podia partilhá-las via `Arc` não é mais puro — é
incorrectamente lento." Isto implica a distinção mas não a
formaliza explicitamente. Um LLM que lê ADR-0030 pode aplicar a
regra "evitar clone" a `Arc::clone` por engano.

Resposta da auditoria deve incluir: recomendação de acrescentar
secção à ADR-0030 (ou criar ADR-0032) que formaliza explicitamente
os dois sentidos de "clone" e autoriza/exige `Arc::clone` em L1.

### Input 3 — Meta-regra do ADR-0018 sub-aproveitada

ADR-0018 contém a meta-regra:

> "O critério para `[l1_allowed_external]` não é 'é externo?' mas
> 'viola pureza funcional de L1?'."

Esta frase é a correcção explícita de um padrão errado de raciocínio
(observado em ADR-0007, ADR-0015 pre-0024, e noutros). Está embutida
no corpo de um ADR específico em vez de destacada como princípio
geral.

Resposta da auditoria deve incluir: recomendação de criar um índice
em `00_nucleo/adr/README.md` ou equivalente que destaque as
meta-regras do projecto, incluindo esta.

### Input 4 — Regra "sem `unsafe` em L1" não formalizada

Identificada no Passo 84.2. A regra é aplicada consistentemente em
toda a série 84.x (rejeita `ptr::addr_of`, `Box::leak`, casts de
ponteiro bruto) mas nunca foi escrita num ADR. Está implícita em:

- ADR-0004 (remove `Box::leak()` por ser "exceção silenciosa").
- ADR-0005 (confirma remoção de `Box::leak()`).
- ADR-0019 (autoriza `unsafe` em L3, por implicação excluindo-o de L1).
- ADR-0029 (V13 proibe `UnsafeCell` em statics de L1).

Resposta da auditoria deve incluir: recomendação de criar
ADR explícito "sem `unsafe` em L1" consolidando as implicações
dispersas.

### Input 5 — Numeração duplicada entre projectos

Durante esta conversa, identificou-se que `00_nucleo/adr/` contém
simultaneamente ADRs do `crystalline-lint` (prefixo numérico simples:
`0001-*.md`) e do Typst (prefixo `typst-adr-`). Há sobreposições
reais de números (ex: ADR-0006 existe nos dois prefixos).

Resposta da auditoria deve incluir: recomendação de política clara
— manter os dois prefixos, ou mover os ADRs do `crystalline-lint`
para um directório separado.

### Input 6 — ADR-0026 com dois ficheiros

Dentro dos ADRs do Typst, `typst-adr-0026-content-divergencia.md` e
`typst-adr-0026-revisao-content-arc.md` usam ambos o número 0026.
A revisão deveria ter nome distinto (ex: `typst-adr-0026-R1-...`) ou
um número novo para não colidir com a procura por "ADR-0026".

Resposta da auditoria deve incluir: recomendação de política para
revisões — usar `-R1`, `-R2` no sufixo, ou criar ADR novo com número
distinto que referencia o original como "Revoga/Revê".

---

## Tarefa 1 — Inventário

### 1.1 — Listar todos os ADRs do Typst

```bash
# Listar ADRs do Typst, ordenados por número
ls 00_nucleo/adr/typst-adr-*.md 2>/dev/null | sort

# Contagem
ls 00_nucleo/adr/typst-adr-*.md 2>/dev/null | wc -l
```

Esperado: 31 ficheiros (ADR-0001 a ADR-0031). Se a contagem for
diferente, reportar — pode indicar ADR com nome inconsistente ou
ADR em falta.

### 1.2 — Extrair status declarado de cada ADR

```bash
# Para cada ADR, o status (linha "**Status**: `...`")
for f in 00_nucleo/adr/typst-adr-*.md; do
    echo "=== $(basename "$f") ==="
    grep -m1 "^\*\*Status\*\*:" "$f"
done
```

Esperado: valores como `PROPOSTO`, `ACEITO`, `IMPLEMENTADO`,
`IDEIA — não implementar ainda`, `REVOGADO`, `UPDATED`, etc.

Anomalias a detectar:
- ADR sem linha de status.
- Status não-canónico (ex: `Accepted` em inglês, misturado com o
  português dos outros).
- Status `PROPOSTO` em ADRs que foram claramente implementados
  (ADR-0001 Typst é caso suspeito — ver Tarefa 2).

### 1.3 — Extrair relações entre ADRs

```bash
# Quais ADRs revogam/revêem/actualizam outros
for f in 00_nucleo/adr/typst-adr-*.md; do
    revoga=$(grep -m1 "^\*\*Revoga\*\*\|^\*\*Actualiza\*\*\|^\*\*Substituído por\*\*\|^\*\*Complementa\*\*" "$f" 2>/dev/null)
    if [ -n "$revoga" ]; then
        echo "$(basename "$f"): $revoga"
    fi
done
```

Esperado: mapa de revogações e revisões. Esta informação é a base
para o "índice de precedência" que Input 3 e 4 recomendam.

---

## Tarefa 2 — Auditoria por ADR

Para cada ADR em `typst-adr-XXXX.md`, executar verificações específicas
e classificar o estado em uma de quatro categorias:

### Categorias de classificação

- **OK** — Estado declarado corresponde à realidade; conteúdo
  actualizado.
- **STATUS DESALINHADO** — Conteúdo correcto mas status declarado
  desactualizado (ex: `PROPOSTO` em ADR implementado).
- **CONTEÚDO INCOMPLETO** — ADR é válido mas tem lacunas
  identificáveis (ex: falta entrada para `Alignment` em ADR-0029).
- **CONTRADIÇÃO COM CÓDIGO** — ADR diz X, código faz Y, e Y é que é
  correcto. Implica alteração do ADR.

### 2.1 — ADR-0001 (Estratégia de Migração)

```bash
# Ver status actual
head -20 00_nucleo/adr/typst-adr-0001-estrategia-migracao.md

# Confirmar decisão sobre comemo — está em [l1_allowed_external]?
grep "comemo" crystalline.toml 2>/dev/null

# Passo 10 mencionado como gatilho para isolamento de comemo — foi
# alcançado? (estamos no passo 84+)
grep -rn "comemo" 01_core/src/ | head -10
```

Verificar:
- Status declarado (esperado `PROPOSTO`).
- `comemo` em `[l1_allowed_external]` conforme Opção C.
- Opção B (isolamento em L3) não foi realizada — o "Passo 10" nunca
  aconteceu porque a numeração de passos seguiu outro caminho.

**Avaliação esperada**: STATUS DESALINHADO. ADR-0001 deveria estar
`IMPLEMENTADO` (Opção C foi executada) com nota de que a Opção B
futura (isolamento em L3) continua em aberto como dívida conhecida.

### 2.2 — ADR-0002 e ADR-0003 (Hierarquia de contenção, coexistência com comemo)

```bash
head -10 00_nucleo/adr/typst-adr-0002-hierarquia-contencao.md
head -10 00_nucleo/adr/typst-adr-0003-comemo-contencao.md
```

Ambos marcados `IDEIA — não implementar ainda`. Verificar que:
- O status continua válido (nenhum passo entre 1 e 84 implementou
  hierarquia de contenção).

**Avaliação esperada**: OK, status correcto.

### 2.3 — ADR-0004 (Descobertas Passo 1)

```bash
head -20 00_nucleo/adr/typst-adr-0004-passo1-descobertas.md

# FileId sem interner global
grep -n "AtomicU16\|static.*INTERNER" 01_core/src/entities/file_id.rs

# ecow fora de L1 (Opção C aplicada)
grep "ecow" crystalline.toml 2>/dev/null
```

Verificar:
- Status `IMPLEMENTADO`.
- FileId sem interner global em L1.
- `ecow` inicialmente removido (mas ADR-0024 reintroduziu para
  `Value::Str` — isto é documentado?).

### 2.4 — ADR-0005 (PackageSpec DTO + World)

```bash
head -10 00_nucleo/adr/typst-adr-0005-packagespec-world.md

# World com B3 + blanket impl
grep -A 5 "pub trait World\|pub trait TrackedWorld\|impl.*TrackedWorld for T" \
  01_core/src/contracts/world.rs 2>/dev/null
```

### 2.5 — ADR-0006 (typst_timing)

```bash
head -10 00_nucleo/adr/typst-adr-0006-typst-timing.md

# Macro timing_scope existente e sem chamadas reais
grep -rn "timing_scope" 01_core/src/
```

### 2.6 — ADR-0007 (rustc_hash substituído)

```bash
head -10 00_nucleo/adr/typst-adr-0007-rustc-hash.md
# Esperado: PROPOSTO e depois REVOGADO por ADR-0018
```

Verificar:
- ADR-0007 ainda marcado `PROPOSTO` mas ADR-0018 já o revoga.
- Se o status não reflecte a revogação, é STATUS DESALINHADO.

### 2.7 — ADR-0008 a ADR-0014 (inlinings e grupo Unicode)

```bash
for n in 08 09 10 11 12 13 14; do
    head -5 00_nucleo/adr/typst-adr-00${n}-*.md
    echo "---"
done
```

Verificar cada um:
- Status declarado vs. realidade no código.
- Se o ADR descreve decisão que foi totalmente aplicada.

### 2.8 — ADR-0015 (ecow removido do parser)

```bash
head -10 00_nucleo/adr/typst-adr-0015-ecow.md
# Importante: ADR-0024 reverte parte do espírito
```

Verificar:
- Status do ADR-0015.
- Se menciona que ADR-0024 actualiza/complica a posição filosófica.

### 2.9 — ADR-0016 (LazyHash removido)

```bash
head -10 00_nucleo/adr/typst-adr-0016-lazyhash.md

# Hash/Eq em Source — já existe ou adiado?
grep -n "impl.*Hash for Source\|impl.*PartialEq for Source" 01_core/src/entities/source.rs
```

### 2.10 — ADR-0017 (adiamento de eval)

```bash
head -10 00_nucleo/adr/typst-adr-0017-adiamento-eval-typst-library.md

# eval() já foi implementado?
grep -rn "pub fn eval\b" 01_core/src/rules/ | head -5
```

Verificar se `eval()` já migrou em passos intermédios. Se sim, ADR-0017
está desactualizado.

### 2.11 — ADR-0018 (rustc_hash reintroduzido)

```bash
head -15 00_nucleo/adr/typst-adr-0018-rustc-hash.md
# Deveria ter a meta-regra "não é externo, é pureza?" (Input 3)
```

Verificar:
- Status `IMPLEMENTADO`.
- A meta-regra está destacada ou embutida em texto corrido?

### 2.12 — ADR-0019, ADR-0020 (fontes L3)

```bash
head -5 00_nucleo/adr/typst-adr-0019-ttf-rustybuzz.md
head -5 00_nucleo/adr/typst-adr-0020-fontdb.md
```

### 2.13 — ADR-0021 (time crate)

```bash
head -5 00_nucleo/adr/typst-adr-0021-datetime.md

# time crate em [l1_allowed_external]
grep "time" crystalline.toml 2>/dev/null
```

### 2.14 — ADR-0022 (FontBook)

```bash
head -15 00_nucleo/adr/typst-adr-0022-fontbook.md
# Este ADR tem secção "Diagnóstico obrigatório" — mistura decisão com
# execução. Anti-padrão identificado nesta conversa.
```

Verificar:
- Se o diagnóstico foi executado em algum passo subsequente.
- Se o status reflecte o resultado.

**Avaliação esperada**: se o ADR tem secção de diagnóstico, isso é
anti-padrão (diagnóstico pertence ao passo de materialização, não
ao ADR). Registar no relatório como oportunidade de correcção
(remover secção de diagnóstico, manter apenas decisão).

### 2.15 — ADR-0023 (indexmap)

```bash
head -10 00_nucleo/adr/typst-adr-0023-indexmap.md
grep "indexmap" crystalline.toml 2>/dev/null
```

### 2.16 — ADR-0024 (ecow reintroduzido para Value::Str)

```bash
head -15 00_nucleo/adr/typst-adr-0024-ecow-value.md
# Relação com ADR-0015 deve estar explicitada
```

Verificar:
- Se ADR-0024 menciona explicitamente a relação não-revogatória com
  ADR-0015 (contextos diferentes: parser vs eval).

### 2.17 — ADR-0025 (Int == Float)

```bash
head -15 00_nucleo/adr/typst-adr-0025-int-eq-float.md

# Comportamento actual em eval_binary_op
grep -B 2 -A 10 "Value::Int.*Value::Float\|Value::Float.*Value::Int" 01_core/src/rules/eval.rs
```

Verificar se a Opção B foi aplicada conforme ADR.

### 2.18 — ADR-0026 + revisão

```bash
head -15 00_nucleo/adr/typst-adr-0026-content-divergencia.md
head -15 00_nucleo/adr/typst-adr-0026-revisao-content-arc.md

# Content::Sequence — Vec<Content> ou Arc<[Content]>?
grep -B 1 -A 3 "Content::Sequence" 01_core/src/entities/content.rs
```

Verificar:
- Se a migração para `Arc<[Content]>` foi realizada.
- Se o nome do segundo ficheiro (`0026-revisao-*`) é aceitável ou
  merece renomear para `0026-R1-*` (Input 6).

### 2.19 — ADR-0027, ADR-0028

```bash
head -10 00_nucleo/adr/typst-adr-0027-cidfont-subsetting.md
head -15 00_nucleo/adr/typst-adr-0028-typographic-types.md
# ADR-0028 foi revogado por ADR-0029
```

Verificar:
- ADR-0028 status `REVOGADO` ou similar?
- Se não, é STATUS DESALINHADO (ADR-0029 revoga ADR-0028
  explicitamente).

### 2.20 — ADR-0029 (pureza física — revoga 0028)

```bash
head -20 00_nucleo/adr/typst-adr-0029-pureza-fisica-revoga-adr-0028.md

# Lista explícita de tipos tipográficos — Alignment está lá?
grep "Alignment\|Length.*Abs.*Rel.*Angle.*Ratio.*Color" \
  00_nucleo/adr/typst-adr-0029-pureza-fisica-revoga-adr-0028.md
```

Verificar:
- A lista actual de tipos tipográficos (Input 1).
- **Esperado**: `Alignment` ausente. CONTEÚDO INCOMPLETO.

### 2.21 — ADR-0030 (performance é domínio)

```bash
head -20 00_nucleo/adr/typst-adr-0030-performance-dominio-l1.md

# Distinção explícita clone profundo vs Arc::clone?
grep -i "arc::clone\|arc.clone\|incremento\|refcount" \
  00_nucleo/adr/typst-adr-0030-performance-dominio-l1.md
```

Verificar:
- Se o ADR distingue explicitamente os dois sentidos de "clone"
  (Input 2).
- **Esperado**: menciona `Arc<T>` mas não formaliza "Arc::clone é
  O(1), não é cópia". CONTEÚDO INCOMPLETO.

### 2.22 — ADR-0031 (early hashing)

```bash
head -15 00_nucleo/adr/typst-adr-0031-early-hashing-source.md

# Source com content_hash?
grep "content_hash" 01_core/src/entities/source.rs 2>/dev/null
```

Verificar:
- Se a Opção C da ADR-0016 foi aplicada via ADR-0031.
- Status actual.

---

## Tarefa 3 — Identificação de lacunas arquitecturais

Para além da auditoria por ADR, identificar **regras implícitas** que
deveriam ser ADRs mas não são:

### 3.1 — "Sem `unsafe` em L1"

Já identificada como Input 4. A auditoria confirma com:

```bash
# Ocorrências de unsafe em L1
grep -rn "unsafe" 01_core/src/ | grep -v "//"

# Se retornar zero, a regra é seguida na prática
```

Esperado: zero ocorrências. Regra seguida mas não documentada.

### 3.2 — "Clone profundo no hot path é proibido; `Arc::clone` é obrigatório"

Já identificada como Input 2. Confirmar se há ADR que formaliza a
distinção além de ADR-0030.

### 3.3 — "Paridade funcional com vanilla é invariante; performance
interna pode divergir"

Mencionada em ADR-0026 (Content como enum vs vtable). Vale a pena
formalizar como princípio geral? Diferentes ADRs aplicam o princípio
mas não há ADR-raiz que o estabeleça.

### 3.4 — "Diagnóstico obrigatório antes de materializar tipo
tipográfico vanilla"

Emergiu como prática nos passos 84.5 e 84.6 (diagnóstico de
`Alignment` e `PlaceElem`). Está implícita no espírito da ADR-0029
mas não explicitada como método.

---

## Tarefa 4 — Recomendação de estrutura de índice

Input 3 recomenda criar `00_nucleo/adr/README.md` ou equivalente.
A auditoria deve propor a estrutura concreta. Proposta preliminar
(a refinar com base no relatório):

```markdown
# Índice de ADRs do Typst

## Meta-regras em vigor
- **Pureza física (ADR-0029)**: L1 não faz I/O de sistema. RAM é domínio.
- **Performance é domínio (ADR-0030)**: gestão eficiente de RAM é
  comportamento correcto, não optimização prematura.
- **Critério de autorização externa (ADR-0018)**: a pergunta é
  "viola pureza?", não "é externo?".
- **Sem `unsafe` em L1** (implícita — formalizar).
- **`Arc::clone` é partilha, não cópia** (implícita — formalizar).

## Status por ADR
| ADR | Título curto | Status |
|-----|--------------|--------|
| 0001 | Estratégia de migração | PROPOSTO (mas Opção C implementada) |
| 0002 | Hierarquia de contenção | IDEIA |
| 0003 | comemo + contenção | IDEIA |
| ... | ... | ... |

## Cadeia de revogações/revisões
- ADR-0007 → revogado por ADR-0018
- ADR-0015 → filosofia corrigida por ADR-0030 (código intacto)
- ADR-0016 → Opção C formalizada em ADR-0031
- ADR-0028 → revogado por ADR-0029
- ADR-0026 → revisto por ADR-0026 revisão (ambos vivos)
```

A proposta vai para o relatório como sugestão. Decisão sobre se
criar este índice e qual o nome final fica para o passo de
correcção.

---

## Tarefa 5 — Formato do relatório

Produzir um único documento `relatorio-auditoria-adrs-passo-84.7.md`
em `/mnt/user-data/outputs/` (ou local equivalente no projecto) com
estrutura:

```markdown
# Relatório de auditoria dos ADRs — Passo 84.7

## 1. Inventário

- Total de ADRs do Typst: N
- Distribuição por status: X `PROPOSTO`, Y `IMPLEMENTADO`, Z `IDEIA`, ...
- Relações registadas: lista de "ADR-A revoga/revê/actualiza ADR-B"

## 2. ADRs com STATUS DESALINHADO

Para cada ADR onde o status declarado não corresponde à realidade:

### ADR-NNNN — [título]
- Status declarado: `PROPOSTO`
- Realidade: implementado no Passo X (justificação)
- Acção sugerida para passo de correcção: actualizar status para
  `IMPLEMENTADO` com nota de data.

## 3. ADRs com CONTEÚDO INCOMPLETO

### ADR-NNNN — [título]
- Lacuna: [descrição]
- Acção sugerida: [alteração concreta de texto]

Incluir obrigatoriamente:
- ADR-0029 sem `Alignment` na lista de tipos tipográficos (Input 1).
- ADR-0030 sem distinção explícita clone profundo vs Arc::clone
  (Input 2).

## 4. ADRs com CONTRADIÇÃO COM CÓDIGO

### ADR-NNNN — [título]
- ADR afirma: X
- Código faz: Y
- Decisão correcta (X ou Y) e justificação.
- Acção sugerida.

## 5. Anomalias estruturais

- Numeração duplicada entre projectos (Input 5).
- ADR-0026 com dois ficheiros (Input 6).
- ADR-0022 com secção "Diagnóstico obrigatório" — anti-padrão
  (decisão vs execução).

## 6. Regras implícitas a formalizar

Cada regra implícita candidata a tornar-se ADR:

### Regra — Sem `unsafe` em L1 (Input 4)
- Onde está implícita: ADRs 0004, 0005, 0019, 0029.
- Evidência de aplicação: grep `unsafe` em `01_core/` retorna 0.
- Acção sugerida: criar ADR-0032 "Proibição de `unsafe` em L1",
  consolidando as implicações dispersas.

### Regra — `Arc::clone` é partilha, não cópia (Input 2)
### Regra — Paridade funcional com vanilla é invariante (Tarefa 3.3)
### Regra — Diagnóstico obrigatório antes de materializar tipo vanilla (Tarefa 3.4)

## 7. Proposta de índice de ADRs

Proposta de estrutura para `00_nucleo/adr/README.md` — ver Tarefa 4.

## 8. Sumário para o passo de correcção

Lista priorizada de alterações:

### Prioridade alta (informação factualmente errada)
- [ADR-XXXX: status desactualizado]
- [ADR-YYYY: contradição com código]

### Prioridade média (lacunas com impacto de decisão)
- [ADR-0029: adicionar Alignment]
- [ADR-0030: formalizar clone vs Arc::clone]

### Prioridade baixa (melhorias estruturais)
- [criar README.md como índice]
- [formalizar regras implícitas em novos ADRs]
- [resolver anomalias de numeração]

## 9. Anexo — comandos executados

Para reprodutibilidade, listar todos os comandos bash/grep executados
durante a auditoria. Sem output dos comandos — só os próprios comandos.
```

---

## Tarefa 6 — Verificação

```bash
# Confirmar que nenhum ADR foi alterado neste passo
git status 00_nucleo/adr/

# Esperado: zero ficheiros modificados em 00_nucleo/adr/.
# O único ficheiro alterado deve ser o relatório em /mnt/user-data/outputs/
# ou local equivalente no projecto.

# Confirmar que testes continuam a passar (auditoria não muda código)
cargo test

# Linter
crystalline-lint .
```

---

## Critérios de conclusão

- [ ] Tarefa 1 (inventário) executada e reportada.
- [ ] Tarefa 2 (auditoria por ADR) executada para todos os 31 ADRs
  do Typst — nenhum saltado.
- [ ] Tarefa 3 (regras implícitas) executada — pelo menos as 4
  regras dos Inputs 2, 4 e as 2 adicionais identificadas na Tarefa
  3.3 e 3.4.
- [ ] Tarefa 4 (proposta de índice) incluída no relatório.
- [ ] Relatório produzido seguindo o formato da Tarefa 5.
- [ ] Os 6 inputs explícitos incluídos no relatório (não perdidos).
- [ ] Nenhum ficheiro em `00_nucleo/adr/` alterado.
- [ ] `cargo test` continua a passar (911 testes, zero violations).
- [ ] `crystalline-lint .` sem violações.

---

## Ao terminar, reportar

- Número total de ADRs auditados.
- Contagens por categoria:
  - OK: X
  - STATUS DESALINHADO: Y
  - CONTEÚDO INCOMPLETO: Z
  - CONTRADIÇÃO COM CÓDIGO: W
- Lista de regras implícitas identificadas (mínimo 4 dos inputs).
- Lista de anomalias estruturais (mínimo 2 dos inputs).
- Link/path para o relatório produzido.

Este passo não tem Go/No-Go para um passo seguinte específico — o
relatório é o produto. O passo de correcção subsequente (provavelmente
84.8, 85, ou numeração à escolha do utilizador) consumirá o relatório
e executará as alterações priorizadas.
