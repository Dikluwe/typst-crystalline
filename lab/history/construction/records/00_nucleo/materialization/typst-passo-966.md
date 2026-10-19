# Passo 966 — conteúdo de função de utilizador (`#let bra(x) = [...]`) dentro de math não recebe default matemático

**Precede este passo**: achado de P964 §3 — `φ`/`ψ` dentro de `bra(phi)`/`ket(psi)` (funções de
markup definidas pelo utilizador, invocadas dentro de contexto matemático) ficam no bloco grego
comum, sem o mapeamento para o plano itálico matemático que o resto do documento já recebe
corretamente desde P964. Causa suspeita: `apply_math_default` só recursa em containers nativos de
math (`Content::Math*`), não em conteúdo produzido por função de utilizador.

**Recomendação do dono**: passo dedicado, não catalogar para depois — o caso de uso é comum
(notação `bra`/`ket` de mecânica quântica vive de funções de utilizador), e a fronteira
markup-em-math já mostrou ter armadilhas (mesma classe de interação de P961 Parte B e P962, por
lados opostos).

**Pré-condição de árvore**: `git status`. Confirmar P964/965 presentes.

---

## Fase A — mapear o mecanismo (roteiro já proposto pelo dono)

1. Confirmar que tipo de `Content` os templates de markup produzem quando invocados dentro de
   contexto matemático — `#let bra(x) = [⟨#x|]` invocado como `bra(phi)` dentro de `$...$`: o
   resultado é `Content::Sequence`, `Content::MathText`, ou outra variante? Confirmar por
   inspecção directa (debug print da árvore de `Content`), não presumir.
2. Ler onde o vanilla aplica o default matemático neste caminho — o dono já aponta que a resolução
   do vanilla acontece em `ir/resolve.rs`, não no layout (diferente de onde `apply_math_default`
   do cristalino vive hoje). Confirmar isso por leitura directa, `file:line`.
3. Confirmar se a diferença de camada (resolve vs layout) é a causa raiz — se o vanilla aplica o
   default **antes** do conteúdo de utilizador ser expandido/avaliado (na fase de resolução), e o
   cristalino aplica **depois** (no layout, só em containers já nativamente math), isso explicaria
   por que conteúdo de função de utilizador escapa: no momento em que `apply_math_default` do
   cristalino roda, o conteúdo já não está mais marcado como "dentro de math" de forma que o
   recursor reconheça.

## Fase A.1 — decidir a direção da correção (gate — mudança arquitectural, mesmo que a
implementação final seja pequena)

1. Avaliar as duas direções que o dono já esboçou:
   - **(a)** Estender `apply_math_default` para recursar em mais variantes de `Content` (incluindo
     o que templates de markup produzem) — mudança contida, mas pode não resolver se a causa raiz
     for a ordem de fases (ponto 3 da Fase A).
   - **(b)** Mover a aplicação do default matemático para a fase de `eval` (mais cedo, antes do
     conteúdo de utilizador perder a marcação de contexto) — mais fiel à arquitectura do vanilla,
     mas mudança maior, toca o pipeline de avaliação, não só o layout.
2. Registar a decisão com razão, editar L0s afectados, **parar para confirmação do dono antes da
   Fase B** — isto é mudança de contrato/arquitectura per o critério de P965 (paragem obrigatória),
   não fluxo contínuo.

## Fase B — Implementação (protocolo de dois agentes de P898 — mudança arquitectural, risco de
efeito em cascata sobre todo conteúdo matemático que usa função de utilizador, não só bra/ket)

1. Agente A escreve testes cobrindo: `bra(phi)`/`ket(psi)` (o caso motivador); pelo menos mais um
   template de utilizador diferente dentro de math, para confirmar que a correção generaliza, não
   é específica a `bra`/`ket`; um caso de função de utilizador **fora** de math (guarda de
   não-regressão — não deve ganhar itálico matemático onde não devia).
2. Agente B implementa a direção decidida na Fase A.1.
3. Revisão do orquestrador — testar função de utilizador aninhada (uma função de utilizador que
   chama outra, ambas dentro de math) para confirmar que a correção não é superficial.
4. Suíte completa verde, discriminada por crate.
5. `cargo run -- .` — zero violations.

## Fase C — Revalidação

1. Recompilar o `.typ` de 30 secções, `compare.py`, confirmar melhoria na seção 26 (mecânica
   quântica, onde `bra`/`ket` aparecem).
2. Confirmação visual/glifo dos casos de teste.
3. Benchmark completo, 7 cenários canónicos, `depois/antes`, zero regressão.

## Resultado esperado

- Mecanismo confirmado: diferença de camada (resolve vs layout) confirmada ou refutada como causa.
- Decisão de direção da correção registada e confirmada antes de implementar.
- Correção generalizando além de `bra`/`ket` (testada com outro template de utilizador).
- Benchmark sem regressão.
