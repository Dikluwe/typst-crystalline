# Passo 886 — `#context` com valor de retorno não convertido em conteúdo

**Precede este passo**: `typst-passo-885.md` (achados originais) e
`typst-passo-885-relatorio.md` (confirmação visual — achado 2, secção 3).
Ler os dois antes de começar.

**Pré-condição de árvore**: confirmar `git status` antes de tocar em qualquer
ficheiro. O relatório de confirmação (P885, seguimento) foi medido numa
working tree **não limpa** (31 ficheiros, `+1106/-405`, tocando font/export/
shaper/world). Se essas alterações ainda estiverem por commitar quando este
passo começar, decidir explicitamente uma de duas coisas antes de escrever
código novo — e registar a decisão no relatório deste passo:
1. Commitar ou reverter esse trabalho pendente primeiro, ou
2. Continuar por cima dele deliberadamente, registando no relatório que o
   binário final deste passo carrega também essas alterações.
Não continuar em silêncio sem uma das duas.

---

## Sintoma confirmado

Fonte de teste: `#for i in range(200) { context measure[lorem(10)] }`
(`07-context.typ`).

- **Vanilla**: cada iteração produz `(width: ..., height: ...)` como texto
  visível na página — o dicionário retornado por `measure()` é convertido em
  conteúdo quando é o valor de retorno de um bloco `context` a nível de
  markup.
- **Cristalino**: o content stream da página fica vazio (`/Length 0`,
  confirmado nos bytes do PDF). O valor é presumivelmente calculado, mas
  descartado — nada é emitido.

## Fase A — Diagnóstico (obrigatória antes de qualquer edição)

1. Confirmar onde o cristalino avalia um bloco `context { ... }` a nível de
   markup e o que faz com o valor de retorno da closure/bloco. Ponto de
   partida razoável, não confirmado: `01_core/src/engine/eval/` ou
   `01_core/src/engine/introspect/` — procurar pela variante `Content` ou
   pelo enum que representa blocos de contexto, e seguir o caminho do valor
   de retorno até ao ponto onde deveria virar `Content` renderizável.
2. Confirmar como o vanilla faz essa conversão (`Value` → `Content` no
   retorno de um bloco `context`/`show` a nível de markup) em
   `lab/typst-original/`. Não presumir — ler o código.
3. Verificar se este sintoma tem a mesma causa raiz do achado 3 (P887,
   `table()` sem stroke) antes de tratar como bug isolado. O relatório de
   confirmação já levantou a hipótese de um padrão comum "computado mas não
   emitido" nos dois — confirmar ou descartar essa hipótese aqui, com
   evidência (não com suposição), e registar o resultado no relatório deste
   passo independentemente da conclusão.
4. Verificar se isto é regressão do achado #34 do handoff pós-P884
   (`measure()`/`Content::Context`, fechado em P860) ou um caso que #34 nunca
   cobriu. O achado #34 tratava especificamente de `measure()` devolver
   largura/altura correctas para uso interno (ex: cálculo de layout); este
   sintoma é sobre o valor de retorno de `measure()` virar texto visível
   quando `context` é usado directamente a nível de markup — pode ser um
   caminho de código diferente que nunca foi coberto, não uma regressão do
   que #34 fechou. Confirmar qual dos dois é, com base no código, e registar.

**Não avançar para a Fase B até a Fase A estar escrita no relatório**, mesmo
que a resposta seja "não determinei a causa raiz com confiança" — nesse caso
registar isso explicitamente e decidir com o dono do projecto se se avança
mesmo assim ou se este passo pára aqui como diagnóstico puro.

## Fase B — Implementação (TDD, per `CLAUDE.md`)

Só depois da Fase A identificar o ponto exacto no código onde o valor de
retorno do bloco `context` deveria ser convertido em `Content` e não está a
ser.

1. Escrever teste(s) que falhem primeiro, reproduzindo o sintoma a um nível
   mais baixo do que "compilar 07-context.typ inteiro e olhar para o PDF" —
   idealmente um teste de unidade sobre a função de avaliação de `context`
   que verifica que o `Content` resultante não está vazio quando o corpo do
   bloco retorna um valor não-`None`. **Verificar que o teste falha antes de
   escrever código de produção** (Fase 1 do fluxo TDD de `CLAUDE.md`).
2. Implementar a correcção.
3. Verificar que os testes novos passam e que a suíte completa continua
   verde, **discriminada por crate** (`typst-core`, `typst-infra`,
   `typst-shell`, `typst-wiring` — não aceitar um número "workspace" sem
   saber de onde vem, per regra reforçada do handoff pós-P884).
4. Recompilar `07-context.typ` (fonte actual, não um PDF antigo de
   referência) nos dois binários e confirmar visualmente (não só por
   extração de texto) que o cristalino agora produz texto visível
   equivalente ao vanilla.
5. `cargo run -- .` (ou equivalente) — confirmar zero violations do
   `crystalline-lint`.

## Fase C — Regressão

Este achado foi descoberto durante a frente de performance de P872–P884, que
mexeu em fontes e export. A Regra 4 do handoff pós-P884 exige que qualquer
correcção nesta área repita o benchmark completo original (7 cenários,
`/tmp/p872-bench/`), não só o caso que a correcção visava — um "corrigido"
sem isso já escondeu uma regressão uma vez neste projecto (P875). Correr o
benchmark completo e reportar os 7 números, comparando com o estado
pós-P884, antes de considerar este passo fechado.

## Resultado esperado

- Header de linhagem actualizado no(s) ficheiro(s) tocado(s), apontando para
  este prompt.
- Testes novos cobrindo o caso (unidade + confirmação E2E via PDF).
- Relatório do passo com: veredicto da Fase A (causa raiz + relação com
  achado 3 e com #34), diff resumido da Fase B, números do benchmark
  completo da Fase C, e a decisão tomada sobre a árvore não limpa na
  pré-condição.
