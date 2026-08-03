# Passo 953 — vocabulário PDF diferente (`Tm`/`Td`, `q`/`cm`/`Q`, `cs`/`scn`, `Tr`): lacuna real ou codificação equivalente?

**Precede este passo**: achado do dono — contagem de operadores do content stream mostra o vanilla
usando `Tm` (não `Td`), `q`/`cm`/`Q` ao redor de quase todo bloco de texto, `BDC`/`EMC` (conteúdo
marcado), `cs`/`scn` repetido, `Tr` explícito — o cristalino não usa nenhum destes da mesma forma.

**Duas categorias, não uma — separar antes de investigar**:

1. **PDF tagueado/acessibilidade (`BDC`/`EMC`)**: já registado como scope-out deliberado desde
   muito antes desta frente (lista original de itens de baixo impacto, junto com CJK vertical,
   pacotes `@preview`). **Não é objecto deste passo** — se o dono quiser reconsiderar a
   prioridade, é uma decisão de produto separada, não um bug a corrigir aqui.
2. **`Tm` vs `Td`, `q`/`cm`/`Q` por bloco, `cs`/`scn` repetido, `Tr` explícito**: podem ser
   escolhas de codificação equivalentes (o documento de teste não precisa de rotação/escala/
   mudança de cor dentro de um bloco, logo o caminho mais simples é suficiente e não pior), ou
   podem ser lacuna real de capacidade. **Isto é o que este passo testa.**

**Pré-condição de árvore**: `git status`. Confirmar P950-952 presentes.

---

## Fase A — testar capacidades reais, não inferir da ausência num documento que não as exercita

Para cada capacidade candidata, criar um documento `.typ` mínimo que a exercite explicitamente e
confirmar se o cristalino produz o resultado visual correcto:

1. **Rotação/escala de texto**: `#rotate(45deg)[texto]`, `#scale(150%)[texto]`, texto dentro de
   um `#box()` transformado — confirmar visualmente (`mutool draw`) que o resultado é o esperado.
   Se o cristalino não usa `Tm` com componentes de rotação/escala nunca, mesmo para este caso,
   confirmar como ele produz o efeito (matriz `cm` a envolver o bloco? posições pré-calculadas
   glifo a glifo, sem depender de `Tm`/`cm` para a transformação?) — as duas abordagens podem ser
   válidas, mas precisa de se confirmar qual é usada e se o resultado bate com o vanilla.
2. **Cor mudando dentro do mesmo parágrafo/bloco**: `#text(fill: red)[a] #text(fill: blue)[b]` na
   mesma linha — confirmar que o cristalino emite `cs`/`scn` (ou equivalente) na transição, não só
   uma vez no início do documento. Se não emitir, isto é bug real (cor errada visível), não
   escolha de codificação.
3. **Conteúdo aninhado com transformação herdada**: `#box(fill: ..., )[#rotate(...)[texto]]` ou
   equivalente — confirmar que transformações aninhadas (múltiplos níveis) produzem o resultado
   geométrico correcto, não só o caso de um nível.
4. Para cada um dos três: se o resultado visual estiver correcto, mas usando uma codificação
   diferente do vanilla (por exemplo, posições absolutas pré-calculadas em vez de `cm`/`Tm`),
   registar isso explicitamente como "codificação equivalente, sem defeito" — não é preciso
   igualar o vocabulário do vanilla se o resultado é correcto e não há razão prática (tamanho de
   arquivo, compatibilidade com ferramentas externas) para mudar.

## Fase B — se algum caso da Fase A falhar

Se algum dos testes acima mostrar resultado visual incorrecto (não só vocabulário diferente):
tratar como bug real, com o mesmo rigor de sempre — Fase A de diagnóstico (ler o código do
vanilla para o mecanismo real), gate se mudar contrato público, TDD ou dois agentes conforme o
risco.

## Resultado esperado

- Para cada uma das três capacidades testadas: veredicto claro — "funciona, codificação diferente
  mas equivalente" ou "lacuna real, corrigida neste passo" (com a correcção, se for o caso).
- Decisão separada e explícita sobre PDF tagueado/acessibilidade: mantido como scope-out (decisão
  já tomada, só reconfirmada), ou promovido a prioridade activa — decisão do dono, não deste
  passo tecnicamente resolver sozinho.
- Nenhuma mudança de vocabulário PDF só por "parecer mais com o vanilla" sem uma razão funcional
  ou prática concreta — igualar operadores por igualar não é objectivo deste projecto (`ADR-0107`:
  paridade é com a língua/resultado, não com a mecânica interna).
