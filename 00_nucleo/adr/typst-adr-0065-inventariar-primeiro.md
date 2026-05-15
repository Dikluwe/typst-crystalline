# ⚖️ ADR-0065: Inventariar primeiro — sub-passo `.1` para qualquer decisão arquitectural não-trivial

**Status**: `EM VIGOR`
**Data**: 2026-04-26

---

## Contexto

A `ADR-0034` formalizou diagnóstico obrigatório antes de
materializar **tipo do vanilla** em L1. A série P156C-J (Layout
Fase 1 + Fase 2 + Fase 3 sub-passo 1) generalizou o padrão para
**qualquer decisão arquitectural não-trivial** — mesmo quando
não envolve materialização de tipo vanilla novo:

- **P156F (skew)**: o inventário 156F.1 revelou que
  `Content::Transform` já era unificado via `TransformMatrix`
  desde P78. Decisão de divergência da spec (não criar
  `TransformKind` enum) foi tomada com base em factos verificados,
  não em assumpção. Resultado: passo puramente aditivo, zero
  refactor, zero risco de regressão.
- **P156G (block)**: inventário 156G.1 tabulou 9 atributos
  vanilla (`width`, `height`, `inset`, `outset`, `fill`,
  `stroke`, `radius`, `clip`, `breakable`) e classificou cada
  como "incluir Fase 1" ou "scope-out per ADR-0054 graded" antes
  de declarar a estrutura do variant.
- **P156H (box)**: inventário 156H.1 detectou conflito de
  naming `Box` com `std::boxed::Box`; decisão `Boxed`
  cristalizada antes de qualquer escrita de código.
- **P156I (stack)**: inventário 156I.1 confirmou que vanilla
  stack tem só 3 atributos (`children`, `dir`, `spacing`) —
  nenhum scope-out necessário, decisão de `Arc<[Content]>` vs
  `Vec<Content>` tomada com base em paridade ADR-0026.
- **P156J (repeat)**: inventário 156J.1 revelou default vanilla
  `justify=true` (não `false` como default Rust natural) e
  decidiu diferir o algoritmo dinâmico per ADR-0054 graded antes
  de materializar.

Em todos os casos, o sub-passo `.1` (inventário) precedeu
decisões arquitecturais e produziu **zero reformulações
mid-passo** — em contraste com precedentes pré-P156C onde
inventários implícitos ou ausentes levaram a refactor mid-passo.

A regra que emergiu — **N=5 aplicações empíricas** consecutivas
em passos com decisão arquitectural não-trivial — é o objecto
deste ADR.

---

## Decisão

### Regra vinculativa

**Qualquer passo de materialização com decisão arquitectural
não-trivial deve ter sub-passo `.1` dedicado a inventário
pré-decisão.**

A regra **estende** ADR-0034 (que já obriga diagnóstico para
materialização de tipo vanilla) e **generaliza** o âmbito para:

- **Decisões de naming** (precedente P156H Box→Boxed).
- **Decisões de tipo arquitectural** (precedente P156I
  `Arc<[T]>` vs `Vec<T>`; P156J default `bool` não-padrão).
- **Expansões de variants existentes** (precedente P156F skew
  reusando `TransformMatrix`).
- **Decisões que atravessam camadas** (L0/L1/L2/L3/L4) onde
  inventário em uma camada informa decisão noutra.
- **ADRs meta** (precedente: este próprio P156K).

### Critério de "não-trivial"

Um passo tem decisão arquitectural não-trivial se ao menos uma
das condições se verifica:

1. **Decisão envolve naming** que pode colidir com convenções
   externas (Rust stdlib, vanilla typst, libraries L4).
2. **Decisão envolve escolha de tipo** (struct vs enum;
   `Vec<T>` vs `Arc<[T]>`; `Option<T>` vs `T` directo —
   classificação per ADR-0064).
3. **Decisão expande variant existente** com semântica nova
   (alterar contrato `is_empty` / `plain_text` / `PartialEq`
   já estabelecido).
4. **Decisão atravessa camadas** (e.g. variant L1 + stdlib L2
   + helper L3).
5. **Decisão de scope** (incluir N atributos vanilla; deferir
   M outros per ADR-0054 graded).
6. **Decisão de divergência da spec do passo** (precedente
   P156F: spec propunha `TransformKind` enum; inventário
   revelou unificação pré-existente; spec divergida).

### Conteúdo mínimo do sub-passo `.1`

O sub-passo `.1` produz inventário em
`00_nucleo/diagnosticos/diagnostico-<feature>-passo-<id>.md`
com (mínimo):

1. **Assinatura vanilla**: campos, tipos, defaults da feature.
2. **Comportamento observável**: semântica visível ao utilizador.
3. **Decisões de tradução** (Caso A/B/C/D per ADR-0064).
4. **Variants/tipos cristalinos a estender ou criar**.
5. **Helpers stdlib reusáveis** (citar por nome).
6. **Limitações aceites** (per ADR-0054 graded; lista explícita).
7. **Tests planeados** (range numérico esperado).

### Excepções

Passos **triviais** sem decisão arquitectural não-trivial **não**
requerem sub-passo `.1`:

- Propagação de hashes (`crystalline-lint --fix-hashes`).
- Correcção de typos em ADRs/prompts.
- Ajuste de cobertura na tabela `typst-cobertura-vanilla-vs-cristalino.md`
  sem nova feature.
- Renomeação trivial sem impacto semântico.
- Adição de testes ao código existente sem alterar contratos.

Em caso de dúvida, **default = inventariar**. Inventário
documentado custa ~10 min; reformulação mid-passo custa ~1-2h.

### Aplicação a passos meta

ADRs meta (como este P156K) cumprem o padrão por auto-aplicação:
o próprio passo P156K tem sub-passo `.1` (inventário do estado
dos ADRs, numeração, precedentes). Auto-aplicação valida o
padrão sem circularidade — a aplicação meta não é contada nos
N=5 do justifica este ADR (que conta apenas passos
pré-promulgação).

---

## Justificação empírica — N=5 aplicações

| Passo | Decisão arquitectural inventariada | Reformulação mid-passo? | Critério "não-trivial" |
|-------|-------------------------------------|:------------------------:|------------------------|
| P156C | Variants `Pad`/`Hide`; `Sides<T>` infraestrutura nova | ✗ | #2, #4 |
| P156D | Variants `HSpace`/`VSpace`; helper `build_spacing` | ✗ | #2, #4 |
| P156G | Variant rico vs Style cascade (decisão arquitectural-chave) | ✗ | #2, #5, #6 |
| P156H | Naming Box→Boxed (conflito std::Box) | ✗ | #1, #2 |
| P156J | Default `justify=true` (não-padrão); algoritmo dinâmico diferido | ✗ | #2, #5 |

**Total**: 5 passos consecutivos com 0 reformulações. Cobertura
de critérios #1, #2, #4, #5, #6 (apenas #3 sem precedente
recente — futuro candidato).

**Contraste pré-P156C**: passos com inventário implícito ou
ausente levaram a refactor mid-passo (ex: redefinição de
`Spacing::Strict` mid-implementação; ajuste de naming após
escrita de código). Padrão emergente é resposta directa a esse
custo.

---

## Implicações

### Sessões futuras

LLMs em sessões futuras citam **ADR-0065** em vez de re-justificar
empiricamente "porque inventário antes de código". Reduz overhead
de enunciados de passos.

### Granularidade de decisão

A regra acopla com o padrão **granularidade 1-2 features/passo**
(N=8 aplicações em P156C-J; não formalizado autonomamente —
candidato a ADR meta futura). Inventário pequeno + passo pequeno =
risco mínimo. Inventário grande + passo grande = risco grande
(atacar com sub-divisão em vez de dispensar inventário).

### Critério "default = inventariar"

Em casos limítrofes, inventariar é o default. O custo (~10 min de
inspecção vanilla + 1 ficheiro markdown) é sempre menor que o
custo de reformulação mid-passo (~1-2h de refactor + perda de
contexto).

### Compatibilidade com `granularidade 1-2 features/passo`

A regra é independente do tamanho do passo (XS/S/M/L) e do número
de features. Aplica-se sempre que decisão arquitectural não-trivial
emerge — mesmo em passos S aditivos puros se houver conflito de
naming ou divergência da spec.

---

## Relação com outros ADRs

### Estende `ADR-0034` (diagnóstico tipos vanilla)

ADR-0034 obriga diagnóstico para **materialização de tipo
vanilla**. ADR-0065 generaliza para **qualquer decisão
arquitectural não-trivial** — incluindo decisões que não
envolvem novo tipo (naming, divergência da spec, scope).
ADR-0034 mantém-se vinculativa para o seu âmbito específico;
ADR-0065 cobre o âmbito mais largo.

### Compatível com `ADR-0033` (paridade vanilla)

Inventário documenta paridade observável e divergências
estruturais aceites. Mecanismo central para ADR-0033 ser
operacional em decisões de feature.

### Reforçado por `ADR-0061` §"Aplicações cumulativas"

ADR-0061 §"Aplicações cumulativas" pós-P156J documenta o padrão
como N=5 e referencia este ADR para formalização.

### Compatível com `ADR-0064` (Smart→Option)

ADR-0064 documenta regra de tradução (Caso A/B/C/D); ADR-0065
documenta regra de processo (sub-passo .1 onde a tradução é
classificada). Os dois ADRs reforçam-se mutuamente.

---

## Consequências

### Positivas

- **Zero reformulações mid-passo** em N=5 aplicações empíricas.
- Diagnósticos persistidos permitem auditoria retrospectiva
  (mesma justificação que ADR-0034 §Consequências).
- Decisões arquitecturais ganham rastreabilidade explícita
  (qual inventário motivou qual decisão).
- Acopla naturalmente com cadência granular (1-2 features/passo).

### Negativas

- Passos triviais podem ser falsamente classificados como
  "não-triviais" e gerar inventário desnecessário (~10 min
  overhead). Mitigação: lista de excepções acima é explícita.
- Risco de inventário superficial ("checklist sem rigor") —
  mitigação: critério mínimo de 7 itens (per ADR-0034 já
  estabelecido) e revisão humana pós-passo.

### Neutras

- Regra **não** dita formato exacto do diagnóstico fora do
  conteúdo mínimo. ADR-0034 cobre formato em detalhe.
- Regra **não** obriga sub-passo `.1` separado se inventário for
  trivial e couber em 5 linhas — pode ficar inline na spec do
  passo. Mas o **registo** (em diagnóstico ou inline) é sempre
  obrigatório.

---

## Alternativas Consideradas

| Alternativa | Prós | Contras |
|-------------|------|---------|
| Manter ADR-0034 sem extensão (apenas materialização de tipo vanilla) | Menor overhead documental | Não capta padrão emergente; passos com decisão arquitectural não-trivial mas sem novo tipo ficam sem regra explícita (P156F skew, P156G variant rico) |
| Revisão `-R1` de ADR-0034 em vez de ADR-0065 nova | Centraliza regras de inventário | Mistura âmbitos; ADR-0034 fica longa e perde foco em "tipo vanilla" |
| ADR meta única "padrões metodológicos da série P156" | Coloca múltiplos padrões juntos | Mistura âmbitos diferentes (tradução vs processo); dificulta citação precisa |
| **Decisão adoptada: ADR nova autónoma estendendo ADR-0034** | **Foco preservado em ADR-0034; novo âmbito tem ADR próprio para citação precisa** | **Aumento da contagem de ADRs; aceitável dado patamar empírico N=5** |

---

## Referências

- ADR-0033 — Paridade funcional vanilla.
- ADR-0034 — Diagnóstico obrigatório para tipos vanilla (regra
  estendida por este ADR).
- ADR-0054 — Critério perfil observacional graded (limitações
  aceites em inventário item #6).
- ADR-0061 §"Aplicações cumulativas" pós-P156J — patamar
  empírico documentado.
- ADR-0064 — Tradução `Smart<T>` → `Option<T>`/default (regra
  de tradução aplicada dentro do sub-passo `.1`).
- Diagnósticos em `00_nucleo/diagnosticos/diagnostico-*-passo-156*.md`
  — inventários da série P156C-J.
- Relatórios em `00_nucleo/materialization/typst-passo-156*-relatorio.md`
  — evidência empírica de zero reformulações.

---

## Anotação preservativa P254 — pattern N=3 citantes ADR-0082 inspirado em validação retroactiva ADR-0065

**P254 (2026-05-14)** promoveu ADR-0082 (Promoções reais
scope-outs ADR-0054 graded) PROPOSTO → EM VIGOR pós-N=3
citantes consecutivos (P250+P251+P252). Sub-padrão "ADR meta
PROPOSTO → EM VIGOR via passo admin XS dedicado" N=1 → N=2
cumulativo (P229 ADR-0080 + **P254 ADR-0082**).

**Pattern N=3 citantes ADR-0082 inspirado em validação
retroactiva ADR-0065** via P156J/P157A/P157B sequente — ADR-0065
**não transitou** por PROPOSTO intermediário (criada EM VIGOR
directo 2026-04-26 P156K). A "validação N=3 sequente" foi
**retroactiva** (aplicações concretas pós-promoção EM VIGOR
fornecidas como evidência cumulativa). ADR-0082 P249 invertou
o template: **PROPOSTO inicial + critério literal N=3 citantes
explícito** documentado no próprio ADR meta antes de promover.

**Distinção formal**:
- ADR-0065 (P156K): criada EM VIGOR directo; validação N=3
  retroactiva (aplicações concretas pós-promoção).
- ADR-0080 (P229): PROPOSTO P226 → EM VIGOR P229 pós-N=9
  validação cumulativa (N≥8 documentado no próprio ADR).
- **ADR-0082 (P254)**: PROPOSTO P249 → EM VIGOR P254 pós-N=3
  citantes consecutivos (P250+P251+P252; **critério literal
  N=3 citantes** documentado no próprio ADR meta).

**Marco P254**: ADR-0082 inaugura sub-padrão "ADR meta com
critério literal N=3 citantes documentado no próprio ADR
antes de promover" N=1 (distinto de ADR-0080 "N=9 cumulativo
documentado" N=1). Candidato a formalizar futuro quando N=3
sub-padrões cumulativos atingir limiar.
