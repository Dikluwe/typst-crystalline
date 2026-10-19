# Prompt — typst-passo-849: `measure()` devolve métricas diferentes das reais (achado #34 de P810/P842) — decisão arquitetural do dono

**Origem**: achado #34, sondado em P842 (`layout` define) sem correção — a causa exige atravessar a fronteira L1/L3, que é decisão de arquitetura, não bug local
**Estado**: aguardando decisão do dono — **não implementar sem essa decisão**

---

## O que já foi medido (P842, não precisa refazer)

`measure([hello])` — vanilla `(22.19pt, 7.24pt)` (métricas reais da fonte via shaping); cristalino `(33pt, 14.85pt)` (heurística monoespaçada: largura = 0.6×tamanho por caractere; altura = 1.35×tamanho, de `FixedMetrics` hard-coded).

Causa exata: `measure_content_real` (`01_core/src/engine/layout/mod.rs:1773-1804`) constrói o `Layouter` de medição com `FixedMetrics` fixo, porque `eval_func_call` (onde `measure()` é despachado) roda em L1, que por desenho não tem acesso a métricas de fonte reais (essas vivem em L3, `FallbackFontMetrics`). Isso já estava documentado no código desde P712 (comentário citando ADR-0107: "divergência mecânica documentada, não de língua").

## Por que isto não é um passo de correção comum

Toda a arquitetura do projeto separa L1 (puro, sem I/O, sem acesso a fonte real) de L3 (onde fontes de verdade vivem). `measure()` é avaliado em L1. Corrigir isto de verdade significa uma de duas coisas, e as duas são mudanças de desenho, não patches locais:

1. Injetar métricas de fonte real no caminho de `eval` (quebra a pureza de L1 — precisaria de um mecanismo específico, análogo ao que `layout_with_introspector_and_metrics` já faz para o layout principal, mas estendido até o eval).
2. Resolver `measure()` numa fase pós-eval, depois que métricas já estão disponíveis (mudança de quando/onde a função nativa `measure` de fato calcula seu resultado — provavelmente via mecanismo de re-avaliação/fixpoint, parecido com o que já existe para `layout()`/`context`).

## Passo 1 — Apresentar as opções ao dono (isto é o que este passo faz)

Resumir para decisão:
- O observável (números que `measure()` devolve) diverge do vanilla em qualquer documento que use `measure` sobre texto — não é caso de borda, é o caminho comum.
- As duas abordagens acima, com uma estimativa de esforço de cada uma (a fazer no momento da apresentação, não adivinhar aqui — abrir os dois mecanismos candidatos no código e medir o tamanho real da mudança antes de apresentar ao dono).
- Se nenhuma das duas for aceitável agora: a alternativa é formalizar isto como débito consciente (mesmo padrão de DEBT-66/67/68), com o critério de reabertura registrado.

## Passo 2 — Implementar conforme a decisão

Só depois da decisão. Se for opção 1 ou 2, é um passo de arquitetura, com sonda própria de quais outros pontos do sistema dependem do mesmo padrão de separação L1/L3 que a mudança tocaria (`layout()`, `context`, outras nativas que hoje sofrem da mesma limitação, se houver — verificar antes de implementar só para `measure`).

## Passo 3 — Validação (se implementado)

Os casos já medidos em P842 (`measure([hello])`, `measure([x])`, `measure([abcd])`, `measure([a b])`, `measure([])`, `measure([x])` com tamanho de fonte customizado) batendo com o vanilla. Suíte completa, comando + contagem antes/depois.

## Relatório

`00_nucleo/diagnosticos/typst-passo-849-relatorio.md` — se a decisão for manter como débito: só a formalização (nova entrada DEBT-XX). Se for implementar: relatório completo.
