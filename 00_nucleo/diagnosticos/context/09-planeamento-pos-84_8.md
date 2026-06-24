# Planeamento pós-Passo 84.8 — próximos passos e revisão da estimativa de 120

**Data**: 2026-04-22
**Estado do código**: 911 testes (737 L1 + 174 L3), zero violations, Passo 84.8 concluído.
**Documento de referência**: `typst-migracao-estado.md` (pronto para Passo 85).

---

## 1. Revisão da estimativa de 120 passos

A estimativa original veio de um parecer externo (Gemini) em 2026-03-29,
quando o projecto estava no Passo 23. Esse parecer mapeou 5 fases e
previu 100–120 passos para paridade total com o Typst vanilla.

### 1.1. O que já aconteceu desde então

A numeração real divergiu do mapa original por duas razões:

1. **Sub-passos dentro de passos maiores** — a série 83.5, 84.1–84.8 e
   84.8a–84.8h introduziram granularidade que o mapa de 120 não
   previa. Cada sub-passo conta como uma unidade de execução, mas
   corresponde a fracções de um "passo" no sentido do mapa original.

2. **Reorganização de fases** — o motor de equações, originalmente
   previsto para os Passos 56–75 (Fase 3), foi antecipado para os
   Passos 34–40. A Fase 1 (pagamento de dívida) estendeu-se para a
   série 84.x por descoberta de dívidas não previstas (DEBTs 33–42).

### 1.2. Mapeamento do estado actual face ao mapa de 120

| Fase original (mapa de 120) | Passos previstos | Estado real |
|-----------------------------|------------------|-------------|
| Fase 1 — Dívida e estrutura | 24–35            | Concluída na essência. DEBTs 1–6 pagos. Dívida residual paga até 84.8. |
| Fase 2 — Motor de layout    | 36–55            | Parcialmente concluída. Line breaking básico, grids (84.2), tabelas (Content::Grid), page breaking funcional. Falta: Knuth-Plass, floats, hifenização. |
| Fase 3 — Motor matemático   | 56–75            | Concluída na essência entre 36–40 (antecipada). Falta: fontes OpenType MATH, kern matemático, MathPrimes (DEBT-8). |
| Fase 4 — Introspecção       | 76–95            | Parcialmente concluída. Counters, state, labels, TOC com fixpoint (DEBTs 10–18 resolvidos). Falta: `locate`, `query`, `measure` completos. |
| Fase 5 — Gráficos e stdlib  | 96–120           | Parcialmente concluída. Imagens (DEBT-24b, DEBT-28, DEBT-29), shapes (DEBTs 30–33), cores (DEBT-27). Falta: SVG, gradientes, stdlib completa. |

### 1.3. Estimativa actualizada

A estimativa de 120 como **limite superior** continua razoável, mas a
interpretação mudou:

- **Passos "macro"** (1–84): 84 executados. Cada um concluiu uma
  unidade de trabalho do mapa original, por vezes antecipando fases
  posteriores.
- **Sub-passos** (84.1–84.8h): 14 executados. Não contam como "passos
  macro" mas representam trabalho real de pagamento de dívida e
  consolidação arquitectural.
- **Restante até paridade total**: 30–40 passos macro. Dependem da
  granularidade adoptada para a stdlib restante, gráficos SVG, e o
  trabalho transversal de paridade.

A estimativa continua dentro da faixa "100–120" do parecer original,
com o entendimento de que os sub-passos são trabalho adicional não
contado naquela estimativa.

### 1.4. O que falta para um motor "viável para produção"

O ponto de viabilidade (não paridade total) é mais próximo. Do parecer
original: "um motor de produção viável no Passo 50 (Estilos, Grids,
Tabelas e Imagens)". Esse marco foi atingido na prática pelo Passo 84
— o que falta são refinamentos, não funcionalidades bloqueantes.

---

## 2. Candidatos para o Passo 85

Quatro direcções identificadas no documento de estado, ordenadas por
prontidão (não por prioridade):

### 2.1. DEBT-41 — Sealed traits no scanner

**Tipo**: refactor mecânico.
**Dependências**: nenhuma.
**Risco**: baixo.
**Duração estimada**: 1 sub-passo (85a ou passo 85 único).

Substituir 6 `unsafe impl Sealed<T>` pelo padrão "private module" da
stdlib. Custo de performance: zero. Primeira convergência da política
ADR-0032 (`unsafe` zero em L1).

### 2.2. DEBT-40 — ImportGuard::drop

**Tipo**: refactor com decisão de design.
**Dependências**: nenhuma.
**Risco**: médio (escolha entre 3 opções com trade-offs).
**Duração estimada**: 1–2 sub-passos (decisão + execução).

As 3 opções propostas no DEBT:
- Opção 1: `Rc<RefCell<Vec<FileId>>>` — custo runtime baixo, simples.
- Opção 2: eliminar o guard, push/pop manual — perde RAII.
- Opção 3: índice + verificação com `len()` — sem custo, perde garantia
  estrutural.

### 2.3. DEBT-1 — StyleChain save/restore (pendências remanescentes)

**Tipo**: materialização de funcionalidade vanilla.
**Dependências**: ADR-0034 (diagnóstico obrigatório antes de código).
**Risco**: médio-alto (trabalho de paridade com vanilla).
**Duração estimada**: 3–5 sub-passos.

Pendências listadas em DEBT-1:
- Propriedades adicionais (fill, font-family, weight numérico).
- Paridade total com o sistema de styles do original.
- Remover wrappers `Content::Strong/Emph` do layout.

### 2.4. DEBT-9 — Cobertura de `lab/parity/`

**Tipo**: infraestrutura transversal de testes.
**Dependências**: nenhuma.
**Risco**: baixo.
**Duração estimada**: contínua (DEBT aberto por natureza).

Expandir testes de paridade. Relevante para verificação de ADR-0033
(paridade funcional com vanilla) em todos os refactors futuros.

---

## 3. Análise de trade-offs entre candidatos

| Candidato | Previsibilidade | Impacto arquitectural | Desbloqueia quê |
|-----------|-----------------|-----------------------|-----------------|
| DEBT-41   | Alta            | Baixo                 | Convergência ADR-0032 |
| DEBT-40   | Média           | Baixo-médio           | Elimina último `unsafe` em `eval.rs` |
| DEBT-1    | Baixa           | Médio-alto            | Paridade de styles |
| DEBT-9    | Alta            | Baixo                 | Detecção precoce de regressões em qualquer passo futuro |

**Recomendação de sequência** (sujeita à decisão do utilizador):

1. **Passo 85 = DEBT-41**. Mecânico, fecha uma decisão aberta, produz
   momentum. Sub-passo único.
2. **Passo 86 = DEBT-40**. Continua a convergência ADR-0032. Decisão
   arquitectural pequena, execução curta.
3. **Passo 87 = DEBT-9 (primeiro alargamento)**. Antes de atacar
   DEBT-1, ter mais testes de paridade reduz risco da materialização.
4. **Passos 88+ = DEBT-1**. Série dedicada com diagnósticos por
   propriedade (ADR-0034).

DEBT-42 (get_unchecked) continua bloqueado por infra de benchmark — só
volta à fila quando essa infra for decidida em passo próprio.

---

## 4. Candidatos bloqueados ou adiados (referência)

- **DEBT-42** — get_unchecked no scanner. Bloqueado por infra de
  benchmark inexistente.
- **DEBT-8** — OpenType MATH primes/align points. Requer fontes math.
- **DEBT-34d** — TrackSizing::Auto min/max-content. Decisão de design
  pendente.
- **DEBT-34e** — colspan/rowspan em Grid. Algoritmo de placement
  novo.
- **DEBT-35b** — cache preventivo de available_width. Abrir só se
  cache for adicionado.
- **DEBT-39** — active_guards push/pop. Aberto sem evidência empírica
  de problema; pode fechar-se sem resolução.

---

## 5. Critério para fechar a série principal

O projecto pode declarar "paridade funcional suficiente" quando:

- Todos os DEBTs em aberto estão encerrados ou documentados como
  "fora de escopo" com razão registada.
- A cobertura de `lab/parity/` inclui pelo menos os casos canónicos
  do Typst vanilla (a definir em sub-passo próprio).
- Os ADRs `EM VIGOR` não têm lacunas (auditoria análoga à do P84.7).
- O código compila sem `unsafe` em L1 (ADR-0032 atingida ou com
  excepções documentadas).

Esse ponto não é "Passo 120" — é um critério de qualidade, não um
número. A estimativa de 120 é útil como ordem de grandeza, não como
contador preciso.
