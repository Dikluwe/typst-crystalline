# Passo 125 — Auditoria profunda dos DEBTs abertos

**Série**: 125 (passo de análise + potencial fecho trivial; quebra
linha de CLI após 12 passos consecutivos).
**Precondição**: Passo 124 encerrado; 1042 total tests; zero
violations; 51 ADRs activas; **11 DEBTs abertos**.
**ADRs aplicáveis**: ADR-0017 (registo de DEBT), precedente do
Passo 105.
**ADR nova**: **não por default**. Se algum DEBT fechar com
decisão arquitectural explícita (ex: "fecha porque ADR-0050
resolveu canal paralelo"), citar a ADR relevante em vez de criar
nova.

---

## Natureza deste passo

Análise profunda dos 11 DEBTs abertos com **grep empírico em
L1/L3** para cada um, seguida de fecho dos que forem
**trivialmente não aplicáveis** (ex: código que originava o
DEBT já foi removido, bloqueio externo dissolveu, ADR
posterior resolveu canal paralelo).

Precedente: Passo 105 (auditoria rápida, tudo M). Desde aí:
- **DEBT-45 fechou** (Passo 110).
- **DEBT-49 fechou** (Passo 107).
- **DEBT-51 fechou** (Passo 106).

Padrão: "M" do 105 pode evoluir silenciosamente para "fechável"
quando o contexto muda. Auditoria periódica paga-se.

Diferença vs 105:
- **Profundidade maior** (grep por DEBT em vez de só ler notas).
- **Fecho oportuno** se o DEBT for já não aplicável.

Este passo **não**:
- Corrige DEBTs que exigem trabalho real (novo código).
- Adiciona funcionalidade.
- Toca estrutura de camadas.
- Cria ADR excepto para documentar fecho de DEBT se ADR
  existente não cobre.

---

## Objectivo

Ao fim do passo:

1. **Inventário profundo** de cada DEBT: estado actual do código
   relevante, se o bloqueio persiste, classificação.
2. **DEBTs trivialmente não aplicáveis** fechados no próprio
   passo (remover entrada de `DEBT.md` ou equivalente;
   documentar o fecho).
3. **DEBTs com fecho não-trivial** permanecem abertos; a
   análise regista o que falta e estima tamanho.
4. **DEBTs claramente M** permanecem abertos com nota "M
   confirmado" + razão factual.
5. **Relatório 125.E** consolida: matriz antes/depois + lista
   de candidatos de fecho dedicado.

---

## Decisões já tomadas

1. **Profundidade**: grep empírico por DEBT + inspecção de código
   relevante.
2. **Fecho oportuno**: só se DEBT for trivialmente não aplicável
   (≤ 5 linhas de mudança, zero código novo excepto remoção).
3. **Fecho não-trivial → sair do escopo**: se fechar exige
   código novo, documentar como candidato e NÃO executar.
4. **Ordem**: numérica (1, 2, 8, 9, 33, 34d, 34e, 35b, 42, 43,
   50). No final, destaque dos potencialmente fecháveis.

---

## Escopo

**Dentro**:
- `view`/`grep` extensivo em `01_core/`, `03_infra/` por cada
  DEBT.
- `view` em `DEBT.md` (ou equivalente).
- `view` em ADRs recentes que possam ter alterado canal paralelo.
- Produção de ficheiros de diagnóstico por DEBT ou agregados.
- **Remoção** de entradas DEBT triviais (pós-análise).

**Fora**:
- Qualquer código novo para fechar DEBT.
- Refactor estrutural.
- Mudança em ADRs excepto para registar fecho.
- Alteração de testes.

---

## Sub-passos

### 125.A — Inventário de meta-dados

**Parte 1 — Ficheiro de registo**:

1. `view` em `00_nucleo/DEBT.md` (ou equivalente onde DEBTs
   são registados).
2. Confirmar lista de 11 abertos com IDs exactos. Se a
   contagem divergir (ex: DEBT-52 criado que esqueci),
   registar.

**Parte 2 — Mudanças relevantes desde o 105**:

Listar ADRs criadas entre 105 e hoje (~25 passos). Identificar
quais afectam domínios potencialmente relacionados com DEBTs:

- ADR-0038+ — Style, StyleChain (afecta DEBT-1? DEBT-2?).
- ADR-0042 — Sink dedup (afecta DEBT relacionado com warnings?).
- ADR-0043 — canal Sink → L3 (afecta DEBT-49 que já fechou;
  outros?).
- ADR-0044 — Engine<'a> (afecta DEBT-2 closures? DEBT-35b
  preventivo?).
- ADR-0045 — formato diagnósticos.
- ADR-0046-0051 — CLI (pouco provável afectar DEBTs L1/L3).
- Outras.

Escrever em
`00_nucleo/diagnosticos/auditoria-debts-passo-125-contexto.md`.

### 125.B — Revisão profunda por DEBT

Para **cada** DEBT dos 11, aplicar template:

```markdown
## DEBT-N — <título>

**Abertura**: Passo M (data).
**Bloqueio original**: [citação curta do DEBT.md]
**Contexto arquitectural actual**: [o que mudou em ADRs/código
 desde abertura]

### Grep empírico

Comandos executados:
- `grep -rn "<palavra-chave>" 01_core/src/`
- `grep -rn "<tipo>" 01_core/src/`
- ...

### Estado actual do código

[sumário: o código que originava o DEBT ainda existe? Mudou?
 Foi removido?]

### Bloqueio persiste?

[sim/não/parcial — com razão factual]

### Classificação

[M (manter) / F (fechar agora) / P (preparar fecho em passo
 dedicado)]

### Acção neste passo

- Se **F**: remover entrada de DEBT.md; documentar em relatório.
- Se **M**: nota "M confirmado 125" em DEBT.md.
- Se **P**: adicionar candidato ao registo de passos futuros.

### Se P: tamanho estimado do passo de fecho

[XS / S / M / L; interdependências]
```

**Grep suggestions por DEBT** (para guiar — ajustar conforme
código real):

- **DEBT-1** (parcial): texto original fala de... [consultar
  DEBT.md]. Grep por palavras-chave do ticket.
- **DEBT-2** (closures eager vs lazy): grep em `rules/eval/` por
  `Closure`, `capture`.
- **DEBT-8** (equações): grep em `rules/math/` ou equivalente.
- **DEBT-9** tracking: idem.
- **DEBT-33** (bézier b-box): grep em `layout/` por `bbox`,
  `bounding_box`, `cubic`.
- **DEBT-34d, DEBT-34e**: grep em layout grid por `colspan`,
  `rowspan`, `auto_shrink`.
- **DEBT-35b** preventivo: ler DEBT.md para entender o preventivo.
- **DEBT-42** (bloqueado externo): ver qual é o bloqueio — pode
  ter dissolvido.
- **DEBT-43** (linter externo): idem — linter pode ter ganho a
  feature.
- **DEBT-50** (condicional com canário): o canário é
  `#set text(font: "X")`? Se sim, está activo.

**Escrever** cada DEBT em
`00_nucleo/diagnosticos/auditoria-debt-<N>-passo-125.md` (11
ficheiros) ou agregar em 1 ficheiro grande.

**Preferência**: 1 ficheiro agregado `auditoria-debts-passo-125.md`
com secções por DEBT. Mais fácil de ler/diff.

### 125.C — Fecho trivial (se aplicável)

Para cada DEBT classificado **F**:

1. **Remover entrada** do `DEBT.md` (ou marcar `[FECHADO — Passo
   125]` se o registo preserva histórico).
2. **Documentar** no relatório com:
   - Estado na abertura.
   - O que mudou.
   - Por que fecha agora.
   - ADRs/Passos relevantes.
3. Se o DEBT tem testes `#[ignore]` associados (alguns DEBTs
   criam teste `ignored` como marcador), **remover o
   `#[ignore]`** se o teste agora passa, ou manter se
   funcionalidade ainda depende de código futuro.

**Gate 125.C**: se fechar um DEBT exige > 5 linhas de mudança,
> 1 ficheiro tocado, ou qualquer código novo (não remoção),
**não fechar neste passo**. Registar como P (candidato futuro).

### 125.D — Criar ADR de fecho (só se justificar)

Se um DEBT fecha com decisão arquitectural que ADR existente
**não cobre**, ADR nova é justificada. Ex: "fecho de DEBT-X
confirma que pattern Y é definitivo".

Na maioria dos casos, DEBT fecha porque ADR posterior já
resolveu. Nesse caso, basta citar a ADR no relatório — sem ADR
nova.

**Preferência**: **sem ADR nova**. Só se explicitamente
justificado.

### 125.E — Encerramento

1. `cargo test --workspace` passa (total inalterado ou
   marginalmente maior se algum `#[ignore]` foi removido).
2. `crystalline-lint` zero violations.
3. Relatório `typst-passo-125-relatorio.md`:
   - Tabela sumário: DEBT-N | antes | depois | nota.
   - Detalhe por DEBT fechado.
   - Candidatos de fecho dedicado registados.
   - Comparação com Passo 105 (quantos mudaram de M para F ou P?).
4. `DEBT.md` (ou equivalente) actualizado.

---

## Critério de conclusão

1. Inventário 125.A + 125.B escrito (todos os 11 DEBTs
   analisados).
2. Zero ou mais DEBTs fechados trivialmente em 125.C.
3. Candidatos de fecho dedicado registados (0 ou mais).
4. `cargo test --workspace` passa.
5. `crystalline-lint` zero violations.
6. Zero ADR nova (ou 1 se justificada).
7. `DEBT.md` reflecte estado pós-passo.
8. Relatório 125.E escrito.

---

## O que pode sair errado

- **Contagem de DEBTs abertos diverge de 11**: o meu resumo
  pode estar desactualizado. Inventário 125.A confirma. Se
  divergir, ajustar escopo em vez de parar.
- **Fecho trivial não é trivial**: aparência enganadora. Se ao
  tentar remover entrada do DEBT.md descobre que há teste
  `#[ignore]` ou código que assume o DEBT aberto, reverter e
  registar como P.
- **Todos os 11 são M**: resultado válido. Registar "M
  confirmado 125" com razão factual. Passo continua a ter
  valor: auditoria documenta estado, facilita futuras
  revisões.
- **Grep por DEBT dá muitos hits**: filtrar por directório
  (01_core/ ou 03_infra/), excluir tests, excluir comentários
  se não são relevantes.
- **DEBT tem link para issue externo**: ver se o issue ainda
  está aberto; se fechou, DEBT pode acompanhar.
- **Tentação de corrigir "enquanto cá estou"**: resistir.
  Este passo é auditoria + fecho trivial. Código novo é outro
  passo.
- **Ambiguidade em classificação**: se não for claro M vs P,
  preferir M + nota "revisto 125, bloqueio persiste".

---

## Notas operacionais

- Este passo espelha o 105 em forma. O 105 demorou moderadamente
  e fechou zero; este passo pode fechar 1-3 DEBTs se as
  conjecturas sobre ADRs recentes dissolvendo bloqueios estão
  correctas.
- Se nenhum DEBT for F, o passo tem valor como "marca no tempo"
  — próxima auditoria arranca do estado documentado.
- Se > 3 DEBTs forem F, surge pergunta se a convenção "DEBT" está
  a ser usada correctamente (talvez algumas notas devessem ser
  "follow-up informal" em vez de DEBT formal). Registar mas não
  agir.
- Ordem numérica permite paralelizar análise (cada DEBT é
  independente). Se fizer-se tudo em sequência, ~1-2h de leitura
  + 30min de grep + 30min de escrita. ~3h total.
- O ficheiro agregado `auditoria-debts-passo-125.md` tem ~11
  secções; espera-se 3-5 linhas de grep output por secção + 5-10
  de análise. Total ~200-400 linhas. Manejável.
- Gate 125.C evita "enquanto cá estou, também fecho este com
  10 linhas de código". Disciplina — se 10 linhas, é passo
  dedicado.
