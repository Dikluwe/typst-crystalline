# Passo 1021 — Critério D em todo o corpus: ambiguidade/omissão, usando `auditar-spec.md`

**Tipo**: Auditoria — catalogar, não corrigir. Primeira execução real do Critério D desde
que foi definido no P999 (bloqueado nesse passo por o workflow não existir; escrito
depois, nunca aplicado).
**Motivo**: risco concreto já materializado uma vez (Passo 996 — generalização "`\` é
**apenas** quebra de linha" além do que a documentação citada dizia), só apanhado por
acaso, numa investigação com outro objectivo. Sem esta auditoria, ambiguidades
equivalentes só aparecem quando um bug as expõe, não antes.
**Pré-condição**: `git status` limpo. HEAD ≥ Passo 1020.

---

## Fase A — Inventário e partição

```
find 00_nucleo/prompts -name '*.md' | sort
```

Excluir os ~25 já fatiados desde P999 (`operators`, `eval::rules` e os 2 nós dele,
`closures` e o nó, `bindings` e os 5 nós, `structural` e os 9 nós, `font_dict`,
`selector_matching`) — **não** por presunção de que estão limpos (não foram verificados
contra `auditar-spec.md` formalmente, só escritos com mais cuidado), mas para não
duplicar esforço nesta primeira passagem; marcar como "a confirmar depois" se sobrar
tempo, não como "aprovado".

Particionar o resto por directório (mesma lógica do P998: um agente por partição,
`entities/`, `compiler/` restante, `infra/`, o que existir).

## Fase B — Aplicar os 4 blocos do `auditar-spec.md`, por prompt

Ler `00_nucleo/prompts/auditar-spec.md` (ou o caminho actual, confirmar) na íntegra
primeiro — não presumir o conteúdo a partir de memória desta conversa, o ficheiro é a
fonte.

Para cada prompt: Bloco 1 (completude estrutural — já parcialmente coberto por Critério C
do P999, não repetir se já catalogado), Bloco 2 (ambiguidade de conteúdo — afirmações não
verificáveis, termos sem definição, limites de escopo implícitos, contradição interna),
Bloco 3 (fundamentação — toda afirmação sobre comportamento do Typst tem citação de
documentação ou medição; **atenção especial a generalizações que ultrapassam a própria
citação**, o caso do Passo 996 é o padrão a procurar), Bloco 4 (zero referência a passo —
já medido no P999 para a maioria; só remedir se o prompt não estiver na lista já
catalogada).

## Fase C — Catálogo

Um achado por problema, com: prompt, bloco de origem, localização, descrição curta, **sem
propor correcção** (mesma disciplina de sempre). Agregar por severidade: ambiguidade que
afecta comportamento gerado (grave) vs. ambiguidade de prosa sem efeito em código (leve).

Priorizar na leitura do catálogo os prompts de `compiler/math/` e `compiler/eval/`
restantes (ainda não fatiados) — são a área onde já sabemos que existe pelo menos um caso
real (o `\` do P996), e é a área mais activa desta frente.

## O que este passo NÃO faz

- Não corrige nenhum prompt.
- Não decide prioridade de correcção — só cataloga, com severidade indicativa.
- Não re-mede o Critério A/B/C onde o P999 já o fez, salvo prompts fora do âmbito
  original do P999 (novos desde então).

## Resultado esperado

Catálogo de ambiguidades/omissões reais no corpus, cobertura completa pela primeira vez.
Decisão de prioridade de correcção fica para depois deste catálogo existir.
