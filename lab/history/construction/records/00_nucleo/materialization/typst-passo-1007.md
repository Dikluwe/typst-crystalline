# Passo 1007 — Verificação: `apply_show_rules` é puro (`Content → Content`) ou fecha sobre contexto externo?

**Tipo**: Verificação — ler e responder, **sem propor nem implementar nada de detecção de
ciclo ainda**. Este passo é pré-condição para qualquer L0 futuro sobre o mecanismo de
paragem de `eval::rules` (a proposta revista por Qwen: `HashMap<MorphCanon, (passo,
Vec<RuleId>)>`, ponto-fixo/ciclo-nomeado/limite).
**Motivo**: se `apply_show_rules` (ou o loop que a chama) lê ou muta algo fora do
`Content` que está a transformar — contador, posição, estado de layout, `Engine<'a>` —
então dois `Content` estruturalmente idênticos em pontos diferentes da cadeia **não são o
mesmo estado**, e qualquer detecção de ciclo por forma repetida (`MorphCanon`) fica
incorrecta: pode reportar "ciclo" onde não há (dois estados iguais em forma mas diferentes
em efeito), ou pode nunca detectar um ciclo real disfarçado por uma diferença de contexto
irrelevante. Esta pergunta tem de ter resposta **antes** de desenhar o mecanismo, não
depois.

**Pré-condição**: `git status` limpo.

---

## Fase A — Localizar o mecanismo exacto

```bash
grep -n 'fn apply_show_rules\|fn eval_show_rule' 01_core/src/engine/eval/rules.rs
```
(ajustar para `compiler/` se o Passo 1005 já tiver corrido)

Ler a função `apply_show_rules` na íntegra, e o loop que a invoca repetidamente (o
mecanismo de P348 — revisitação local até `morph_canon` ou tecto 64).

## Fase B — Assinatura e captura

1. **Assinatura de `apply_show_rules`**: que parâmetros recebe, além do `Content` a
   transformar? Se receber `&mut Engine<'a>`, `&EvalContext`, `&StyleChain`, ou qualquer
   referência a estado fora do nó — is that state **lido** (consultado, sem influenciar o
   resultado de forma que dependa de *quando* é chamado) ou **mutado** (o resultado
   depende de chamadas anteriores, não só do `Content` de entrada)?
2. Para cada `Transformation` (`Func`, `Content`, `Str`, `Style` — per `entities/show.md`):
   verificar se a aplicação de cada uma toca algo fora do nó:
   - `Transformation::Func` — a closure chamada (`apply_func`) pode capturar/ler
     `counter`/`state`/`query` do documento? Se sim, o resultado de aplicar a mesma regra
     ao "mesmo" `Content` (mesma forma) pode diferir consoante a posição no documento ou
     chamadas anteriores.
   - `Transformation::Style` (show-set) — já sabemos que embrulha em `Content::Styled`
     **sem mutar `engine.styles` globalmente** (achado já registado, `f_fronteira_e1.md`).
     Confirmar se isto significa que é seguro (o styles fica **dentro** do `Content`
     resultante, logo faz parte da "forma" comparada por `morph_canon`) — ou se
     `morph_canon` ignora/normaliza esse campo ao calcular a forma canónica (o que
     tornaria dois `Content` com styles diferentes indistinguíveis para o mecanismo de
     ciclo, mesmo sendo semanticamente diferentes).
3. **`morph_canon` em si**: o que exactamente normaliza/ignora ao calcular a forma
   canónica? Ler a implementação. Isto decide se "forma repetida" é uma noção
   semanticamente segura para este propósito ou se descarta informação que importa.
4. **`active_guards`**: como é que o guard actual (anti-recursão durante a chamada) se
   relaciona com o loop de revisitação — são mecanismos independentes ou o guard já dá
   alguma garantia que reduz o problema (ex.: já impede uma classe de ciclos antes de
   chegar ao loop)?

## Fase C — Casos a testar directamente (não só ler código)

Escrever (ou usar, se já existirem) casos mínimos que exercitem show-rules envolvendo
`counter`/`state`/`query` dentro da closure, e observar empiricamente se chamar a mesma
regra sobre `Content` de forma idêntica, em dois pontos diferentes de uma cadeia de
revisitação, produz resultados diferentes. Se sim — está confirmada a impureza relevante.

## Fase D — Resposta directa, sem ambiguidade

O relatório final tem de responder, com evidência (`file:line` + resultado do teste da
Fase C), a uma destas três:

1. **Puro para este propósito**: `apply_show_rules` não lê nem muta nada que afecte o
   resultado de forma dependente de quando/quantas vezes é chamada, dentro do que
   `morph_canon` compara. A proposta de detecção de ciclo por forma pode avançar como
   desenhada.
2. **Impuro, mas de forma inofensiva para este mecanismo especificamente**: toca contexto
   externo, mas esse contexto não pode produzir a mesma forma canónica para `Content`
   semanticamente diferentes (explicar porquê, com exemplo concreto verificado).
3. **Impuro de forma relevante**: existe um caso real onde a mesma forma canónica pode
   surgir com efeito/significado diferente consoante o contexto — a proposta de detecção
   de ciclo por forma **não é segura** como está, e precisa de ser revista (ex.: incluir
   uma componente do contexto relevante na chave do `HashMap`, não só `MorphCanon`).

## O que este passo NÃO faz

- Não desenha nem implementa nenhum mecanismo de detecção de ciclo.
- Não altera `apply_show_rules` nem `morph_canon`.
- Não decide se a proposta de Qwen (documento 19) avança — só produz a evidência que
  falta para essa decisão.

## Resultado esperado

Resposta fechada (1, 2, ou 3 da Fase D), com `file:line` e teste empírico, pronta para
condicionar o L0 do mecanismo de paragem de `eval::rules` quando esse passo for escrito.
