# Relatório P156K — ADRs meta formalizando padrões consolidados

Passo arquitectural meta. **Não materializa código**. Formaliza
dois padrões com patamar empírico forte ao fechar a série
P156C-J:
- Smart→Option/default (**N=6**) → ADR-0064.
- Inventariar primeiro (**N=5**) → ADR-0065.

---

## 1. Resumo dos dois ADRs

### 1.1 ADR-0064 — Tradução `Smart<T>` vanilla → `Option<T>`/default

**Status**: `EM VIGOR`. Data: 2026-04-26.

Formaliza regra de tradução vinculativa em **4 casos canónicos**:

| Caso | Vanilla | Cristalino | Exemplos |
|------|---------|------------|----------|
| A | `Smart<T>` semântica "auto = computa do contexto" | `Option<T>` (None ↔ Auto) | P156G/H/I width/height/spacing |
| B | `Smart<T>` semântica "auto = literal fixo" | `T` directo (Default natural) | P156I `Smart<Dir>` → `Dir` (default TTB) |
| C | campo `T` com default não-`Default::default()` | `Option<T>` (None ↔ default declarado) | P156I/J `Length` (default zero) → `Option<Length>` |
| D | `bool` com default não-`false` | `bool` directo (default documentado) | P156D weak; P156G breakable; P156J justify=true |

Documenta **subpadrão emergente**: `extract_length` reusado N=6
vezes consecutivas em `stdlib/layout.rs`. Promoção a helper
público é candidato a refactor escopo XS futuro (não materializado
neste ADR).

### 1.2 ADR-0065 — Inventariar primeiro (sub-passo `.1`)

**Status**: `EM VIGOR`. Data: 2026-04-26.

Generaliza ADR-0034: passos com decisão arquitectural não-trivial
têm sub-passo `.1` dedicado a inventário pré-decisão. Critério
"não-trivial" cobre **6 dimensões**:

1. Naming (precedente Box→Boxed P156H).
2. Escolha de tipo (precedente Arc<[T]> P156I; bool default P156J).
3. Expansão de variant existente.
4. Atravessamento de camadas (L0/L1/L2/L3/L4).
5. Scope (atributos a incluir/diferir per ADR-0054 graded).
6. Divergência da spec (precedente skew P156F).

Default em casos limítrofes: **inventariar** (~10 min vs ~1-2h
de reformulação mid-passo).

Auto-aplicação documentada: P156K cumpre o próprio padrão (este
relatório §2.1 documenta o sub-passo .1).

---

## 2. Numeração final atribuída

### 2.1 Inventário do sub-passo `.1`

Inspecção de `00_nucleo/adr/README.md` confirmou:

- **Total ADRs antes de P156K**: 61 (60 únicos + 1 -R1).
- **Reservas activas**:
  - ADR-0062: `hayagriva` (consumida quando DEBT-55 fechar em P159).
  - ADR-0063: column flow algorithm (consumida quando DEBT-56
    materializado).
- **Próxima numeração livre**: ADR-0064 e ADR-0065.
- **Precedentes de ADRs meta `EM VIGOR`**: 0029 (pureza), 0030
  (performance), 0032 (unsafe), 0033 (paridade), 0034
  (diagnóstico), 0036 (atomização), 0037 (coesão), 0038 (estilos),
  0054 (perfil graded). Estrutura canónica reusada: `# ⚖️
  ADR-NNNN: <título>` + Status + Data + Contexto + Decisão +
  Consequências + Alternativas + Referências.

### 2.2 Decisão de numeração

**ADR-0064** = Smart→Option/default.
**ADR-0065** = Inventariar primeiro.

Consecutivos. Ambos `EM VIGOR`. Nenhuma reserva (0062/0063)
afectada — mantêm-se inalteradas.

---

## 3. Confirmação de evidência empírica

### 3.1 ADR-0064 — N=6 aplicações citadas

| Passo | Caso | Aplicação |
|-------|------|-----------|
| **P156D** | D | `weak: bool` directo (HSpace/VSpace) |
| **P156E** | A | `Smart<Parity>` → `Option<Parity>` |
| **P156G** | A + D | `Smart<Rel<Length>>` → `Option<Length>` (Block.width/height) + `breakable: bool` directo |
| **P156H** | A | idem para Box.width/height/baseline |
| **P156I** | A + B | `Smart<Length>` → `Option<Length>` (spacing) + `Smart<Dir>` → `Dir` (TTB default) |
| **P156J** | C + D | `Length` (default zero) → `Option<Length>` (gap) + `bool` directo (justify=true) |

**Total**: 6 passos consecutivos com 0 reformulações. Cobertura
de todos os 4 casos canónicos (A, B, C, D). Patamar empírico
forte que justifica `EM VIGOR` (não `PROPOSTO` para período
adicional de observação — conforme pré-decisão do passo).

### 3.2 ADR-0065 — N=5 aplicações citadas

| Passo | Decisão arquitectural inventariada | Critério "não-trivial" |
|-------|-------------------------------------|------------------------|
| **P156C** | Variants `Pad`/`Hide` + `Sides<T>` infraestrutura | #2, #4 |
| **P156D** | Variants `HSpace`/`VSpace` + helper `build_spacing` | #2, #4 |
| **P156G** | Variant rico vs Style cascade (decisão arquitectural-chave) | #2, #5, #6 |
| **P156H** | Naming Box→Boxed (conflito std::Box) | #1, #2 |
| **P156J** | Default `justify=true` (não-padrão); algoritmo dinâmico diferido | #2, #5 |

**Total**: 5 passos consecutivos com 0 reformulações. Cobertura
de critérios #1, #2, #4, #5, #6. Apenas critério #3 (expansão
de variant existente) sem precedente recente — futuro candidato.

P156I deliberadamente **não** contado (apesar de ter inventário
.1) porque a decisão de `Arc<[T]>` enquadra-se mais directamente
em ADR-0064 Caso A (tipo paralelo), dispensando o critério
estendido de ADR-0065. Alternativa de contar P156I como sexta
aplicação foi desconsiderada para preservar rigor: N=5 com
diversidade de critérios é mais informativo que N=6 com
sobreposição.

---

## 4. Análise de risco (padrão N=5; **sexta aplicação consecutiva**)

P156K é **passo arquitectural meta** sem alteração de código.
Mesmo assim, documentar risco preserva o precedente N=5
(P156F/G/H/I/J) e mantém a disciplina — auto-aplicação de
ADR-0065 §"Critério de não-trivial #6" (divergência da spec
documentada via inventário).

### 4.1 Riscos identificados

| Risco | Avaliação | Mitigação aplicada |
|-------|-----------|---------------------|
| ADRs serem citados sem leitura por sessões futuras | Médio | ADRs auto-contidos (definições + casos + exemplos no próprio ficheiro) — não obrigam leitura cruzada de relatórios para entender |
| Caso E emergir e regra Smart→Option ficar desactualizada | Baixo | ADR-0064 documenta excepções e prevê expansão futura ("Caso E pode emergir") |
| Inventário superficial degradar valor de ADR-0065 | Médio | ADR-0065 referencia ADR-0034 §"Conteúdo mínimo" (7 itens canónicos) como baseline; lista de excepções explícita |
| Ambiguidade entre ADR-0064 e ADR-0034 sobre âmbito | Baixo | ADR-0064 §Relação documenta explicitamente: ADR-0034 cobre tipo vanilla; ADR-0064 cobre tradução de campos dentro do tipo |
| Ambiguidade entre ADR-0065 e ADR-0034 sobre âmbito | Baixo | ADR-0065 §Relação documenta explicitamente: ADR-0034 obriga diagnóstico para tipo vanilla; ADR-0065 generaliza para qualquer decisão arquitectural não-trivial |
| Decisão de status `EM VIGOR` (vs `PROPOSTO`) ser prematura | Baixo | N=6 + N=5 = patamar empírico forte com zero reformulações; pré-decisão do passo confirma `EM VIGOR` |
| Numeração 0064/0065 colidir com reservas futuras | Nulo | Reservas 0062/0063 documentadas e preservadas; próximas numerações 0066+ ficam livres |

### 4.2 Riscos não-aplicáveis

- **Refactor de código**: zero (passo puramente documental).
- **Quebra de contrato API**: zero (sem código alterado).
- **Drift de hashes L0/L1**: zero (sem código → sem hash a
  propagar).

### 4.3 Conclusão de risco

**Risco residual: muito baixo.** Padrão "ADR meta documental +
patamar empírico forte (N≥5) + casos canónicos exaustivos +
referências cruzadas explícitas" replica tratamento bem-sucedido
de ADR-0034 (que formalizou padrão similar com N=4 precedente).
Zero alteração de código → zero risco de regressão.

---

## 5. Verificações (numeradas per spec do passo)

| # | Verificação | Resultado |
|---|-------------|-----------|
| 1 | README lista 63 ADRs (61 + 2 novos) | **✓ 63** (estado actualizado) |
| 2 | Status dos dois novos ADRs: ambos EM VIGOR | **✓ EM VIGOR** ambos |
| 3 | Numeração consecutiva confirmada | **✓ 0064 + 0065** (consecutivos) |
| 4 | ADR-0064 cita N=6 aplicações (P156D/E/G/H/I/J) | **✓** (tabela §3.1 deste relatório + ADR-0064 §"Justificação empírica") |
| 5 | ADR-0065 cita N=5 aplicações (P156C/D/G/H/J) | **✓** (tabela §3.2 deste relatório + ADR-0065 §"Justificação empírica") |
| 6 | Linter markdown (se aplicável): zero erros | **✓ N/A** (sem linter markdown configurado no projecto; sem ferramenta a invocar) |
| 7 | ADR-0034 não modificada (estendida por referência) | **✓ inalterada** (apenas referenciada em ADR-0064 §Relação e ADR-0065 §Estende) |
| 8 | `crystalline-lint`: zero violations | **✓ No violations found** |

---

## 6. Estado pós-P156K

- **Total ADRs**: 61 → **63** (60 únicos + 1 -R1 → 62 únicos +
  1 -R1).
- **Distribuição actualizada**:
  - `PROPOSTO`: 11 (inalterado).
  - `IDEIA`: 2 (inalterado).
  - `EM VIGOR`: 26 → **28** (+2 = ADR-0064 + ADR-0065).
  - `IMPLEMENTADO`: 19 (inalterado).
  - `REVOGADO`: 2 (inalterado).
  - `ADIADO`: 1 (inalterado).
- **Reservas**: ADR-0062 (hayagriva) e ADR-0063 (column flow)
  **mantidas inalteradas**.
- **Padrões agora citáveis explicitamente**:
  - Smart→Option/default: cite **ADR-0064**.
  - Inventariar primeiro: cite **ADR-0065**.
- **Padrões NÃO formalizados neste passo** (candidatos futuros):
  - Granularidade 1-2 features/passo (N=8) — candidato se
    patamar continuar a crescer ou se for desafiado por passo
    M+/L (e.g. columns/colbreak).
  - §análise de risco no relatório (N=5) — já parcialmente
    coberto por convenções de processo.
  - Reuso de template containers / variant rico (N=4) —
    candidato se aplicado fora de Layout.
  - Promoção formal de `extract_length` a helper público —
    refactor escopo XS, mencionado em ADR-0064 §Implicações.

### 6.1 Sem alteração de código

- `cargo build`: não relevante (sem código alterado).
- `cargo test`: não relevante (sem código alterado).
- `crystalline-lint`: **zero violations**.
- Hashes: nenhum a propagar.

---

## 7. Decisão pós-P156K

Per spec do passo §"Pós-passo", as candidatas para próximo passo
mantêm-se as mesmas que pós-P156I/J:

1. **Continuar Fase 3** — columns + colbreak (DEBT-56 column
   flow L+; quebra granularidade; provavelmente 3-5 passos).
2. **Mudar para Model Fase 2 P157** (table foundations).
3. **Footnote area** (sub-fase prioritária ADR-0061 Decisão 5).
4. **Promover ADR-0061 a IMPLEMENTADO** (3 caminhos documentados
   em ADR-0061 §"Aplicações cumulativas"; agora 1 caminho a 50%
   pós-P156J).
5. **Investigar discrepância DEBTs** (9 vs 13) — passo
   administrativo XS.
6. **Retomar paridade** (DEBT-54/53 suspensos desde P153).
7. **Atacar Introspection** (17% cobertura — mais fraca per
   inventário 148).
8. **Promover `extract_length` a helper público** — refactor
   escopo XS sugerido por ADR-0064 §Implicações.

ADR-0061 mantém-se `PROPOSTO`. Padrão granularidade 1-2
features/passo (N=8) **NÃO** foi formalizado neste passo (per
pré-decisão).

Decisão humana sobre próxima direcção tem máxima informação
acumulada após série completa P156C-K.

---

## 8. Fechamento

P156K fecha como **passo arquitectural meta documental**, sem
modificação de código. Dois ADRs novos `EM VIGOR` (0064 + 0065)
formalizam padrões consolidados na série P156C-J com patamar
empírico forte (N=6 + N=5) e zero reformulações mid-passo.

Auto-aplicação documentada: P156K cumpre ADR-0065 (sub-passo
`.1` de inventário) ao redigir-se. **Sexta aplicação consecutiva**
do padrão §análise de risco preservada (P156F/G/H/I/J/**K**).

Sessões futuras citam **ADR-0064** e **ADR-0065** explicitamente
em vez de re-justificar empiricamente cada passo. Reduz overhead
de enunciados; garante rastreabilidade formal dos padrões.

**Pausa natural após P156K — fecha série meta P156C-K com
patamar arquitectural máximo. Próxima decisão humana sobre
direcção (8 candidatas documentadas) tem máxima informação.**
