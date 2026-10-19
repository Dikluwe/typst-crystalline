# Emenda a `auditar-fatiamento.md` — proveniência obrigatória para constantes geométricas/tipográficas

**Origem**: achado `typst-achado-math-cases-gutter-width.md` (Passo 1041) — `padding =
style.size * 0.1` escrito em `matrix.md` e herdado por `cases.md` como se fosse fórmula
correcta, quando é aproximação que coincide com uma fonte de teste específica, não com o
mecanismo real (o glifo do delimitador esticado já inclui a folga nas suas próprias
métricas OpenType MATH — a soma manual duplica).

---

## Regra nova

**Nenhuma constante geométrica/tipográfica num L0 pode aparecer como valor nomeado sem
proveniência.** Duas formas aceitáveis, nenhuma outra:

1. **Citação ao campo da fonte** — `file:line` de onde o vanilla lê o campo real da tabela
   `MathConstants`/OpenType MATH (ex.: `delim_gap`, `axis_height`, `radicalKernBeforeDegree`).
   Isto é o valor correcto por construção, porque varia com a fonte activa em vez de estar
   fixo.
2. **Aproximação heurística explicitamente marcada** — se por alguma razão não for
   possível ler o campo real ainda, o L0 tem de dizer isso: "aproximação, verificada só
   contra `<fonte X>`, pode divergir com outra fonte" — nunca apresentada como fórmula
   definitiva.

**Proibido**: um número (`0.1em`, `0.5em`, etc.) escrito como se fosse a regra, sem
nenhuma das duas proveniências acima. É precisamente isto que aconteceu em `matrix.md`,
copiado depois para `cases.md` — um valor que "funcionava" contra a única fonte usada nos
testes, tratado como verdade geral.

## Porquê isto importa mais em tipografia do que noutras áreas do código

Uma tabela de fonte é desenhada pelo tipógrafo que a fez — os valores **variam por
fonte**, não são propriedade universal da matemática nem do layout. Um `L0` que hardcoda
um múltiplo de `size` em vez de ler o campo da fonte produz um bug latente: continua a
"funcionar" (produz número, não crasha) com qualquer fonte, mas erra por quantidade
diferente consoante a fonte — nunca detectável nos testes actuais, que usam sempre a
mesma fonte de referência.

## Aplicação retroactiva

Ao rever ou fatiar qualquer L0 de `compiler/math/layout/`, verificar todas as constantes
geométricas presentes contra esta regra — não presumir que só `cases.md`/`matrix.md`
tinham este problema; confirmar caso a caso ao tocar em cada ficheiro, per a mesma
disciplina já aplicada a outras lições deste método.
