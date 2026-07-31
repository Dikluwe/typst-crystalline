# Passo 943 — reconciliar a aritmética do 1.2-1.3× restante (passo curto, exploratório)

**Precede este passo**: `typst-passo-942-relatorio.md`, seção 6 — descoberta (~71ms) + coverage
lazy (~196ms) = ~267ms, contra ~256ms do vanilla (num único passo combinado) — diferença de só
~11ms nesse componente. Mas o tempo total medido tem um gap maior: `utf8-cjk` 331ms (cristalino)
vs 282.7ms (vanilla) = ~48ms de diferença, sem contar `layout_ms` (só 25ms, já pequeno).

**Passo explicitamente limitado — per pedido do dono: "só um pouco para saber se dá para melhorar
ou vai ser difícil".** Não é compromisso de fechar o 1.2× por completo. Se a Fase A já mostrar que
o resíduo é ruído/overhead de processo difícil de reduzir, parar e registar, não forçar mais
investigação.

**Pré-condição de árvore**: `git status`. Confirmar estado P942 presente.

---

## Fase A — reconciliar os números (rápido, sem instrumentação nova se possível)

1. Somar os componentes já medidos e decompostos por P942 (`Face::parse` 15.3ms, iteração `cmap`
   ~30ms, `Coverage::from_codepoints` 151.1ms, descoberta ~71ms, `layout_ms` residual ~25ms) e
   confirmar se a soma bate com o tempo total medido pelo `hyperfine` (~331ms para `utf8-cjk`) —
   se não bater, a diferença é overhead não contabilizado por nenhuma das medições anteriores
   (arranque do processo, I/O não capturado pelos timings internos, etc.).
2. Medir o tempo de arranque "vazio" de cada binário (por exemplo, `--version` ou o menor trabalho
   possível) para confirmar quanto do tempo total é overhead de processo puro, não relacionado a
   fontes — comparar cristalino vs vanilla nesse baseline mínimo.
3. Se sobrar uma diferença não explicada depois dos pontos 1-2: decidir, com esse número em mãos,
   se vale a pena instrumentar mais (Fase B) ou se é pequeno demais para justificar o esforço.

## Fase B — só se a Fase A encontrar algo concreto e não-trivial

Se a Fase A isolar uma causa específica (não "overhead de processo genérico, difícil de mudar"):
medir com mais precisão, decidir com o dono se vale corrigir, mesma disciplina de sempre (TDD,
benchmark antes/depois). Não abrir escopo maior que isso sem check-in.

## Resultado esperado

- Aritmética reconciliada: quanto do tempo total é explicado pelos componentes já medidos, quanto
  é overhead de arranque de processo comum aos dois binários, e quanto (se houver) continua sem
  explicação.
- Veredicto claro: "dá para melhorar mais, com esta causa específica" ou "o resíduo é overhead de
  processo, não vale o esforço" — resposta à pergunta do dono, não mais um passo aberto.
- Se a resposta for "não vale o esforço": este é o ponto de fechar a frente de performance de
  fontes (P925-943) de vez, sem deixar a pergunta em aberto.
