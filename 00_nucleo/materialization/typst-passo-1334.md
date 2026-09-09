# Passo 1334 — completar diagnósticos de argumentos da nativa calc.abs

A medição `00_nucleo/diagnosticos/p1334-baseline-public.json` registra HEAD,
árvore não commitada, comandos/UTC e binários. Missing ainda usa aridade
portuguesa sem origem primária; named value carece de hint; named depois
de extra posicional indevidamente vence a primeira sobra.

Atualizar primeiro os L0 `00_nucleo/prompts/compiler/stdlib/calc.md`,
`00_nucleo/prompts/compiler/eval/call_dispatch.md` e
`00_nucleo/prompts/compiler/stdlib/_comum.md`. Depois implementar:

- calc decide ausência, named value/hint e primeira sobra conjunta;
- dispatcher transporta a chamada inteira somente pela identidade nativa
  de abs, inclusive With, sem validar argumentos;
- fachada reexporta calc_abs apenas dentro da crate, sem wrapper.

Preservar P1333, fórmulas, origens, nomes, avaliação eager e outras funções.
Sem entidades/API pública/fase nova. Resolução de abs importado em math
continua dívida separada; não reconhecer spelling para contorná-la.

Correção contínua ADR-0127, L0 antes de código. Skill
tekt-materializacao-segregada em A/B sem atestação de isolamento: testes
e migrações congelados por autor independente, revisão separada, RED
compilado → implementação → GREEN. Novos testes usam Args coerente, origens
reais/detached/cross-source, duplicatas legais, ordem e controles anteriores.
Após focal verde: build/workspace, fmt, linhagem/V15/V26, linter e CLI
normal/repetido/invertido. Unknown obrigatório bloqueia; duas revisões
focais sem ganho reabrem desenho. Relatório substantivo em
`00_nucleo/diagnosticos/p1334-final-report.md`. Histórico preservado.
Sem commit, stage ou push.
