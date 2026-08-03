# Passo 952 — vanilla reserva 7-17pt a mais de espaço vertical para equações multi-linha (achado sistemático)

**Precede este passo**: achado do dono via `tools/geometry/compare.py` — deltas entre as 44
equações numeradas do documento não mostram pico isolado; mostram um padrão consistente: equações
de uma linha ficam com delta próximo de zero, equações multi-linha (fração/integral/matriz) ficam
consistentemente +7 a +17pt acima do que o cristalino produz. Um ponto (entre as equações 41-42)
levemente fora do padrão, dentro de margem não-anômala. **Isto é uma regra de layout diferente,
sistemática, distinta do bug pontual que motivou P948.**

**Pré-condição de árvore**: `git status`. Confirmar P944-950 (os que já tiverem sido executados)
presentes.

---

## Fase A — confirmar o mecanismo real do vanilla para espaçamento de equação multi-linha

1. Ler o código do vanilla que decide o espaço reservado **acima/abaixo** de um bloco de equação,
   em função da sua altura/número de linhas — candidatos a procurar:
   `typst-layout/src/math/`, o ponto onde uma equação de bloco é inserida no fluxo do documento
   (não dentro da própria equação — isto é sobre o espaço **entre** equações consecutivas no
   documento, ou entre a equação e o parágrafo ao redor).
2. Confirmar se é uma constante fixa (tipo `above`/`below` de `equation`, per `EquationElem`) que
   já existe mas é aplicada de forma diferente conforme a altura do conteúdo, ou se é uma fórmula
   que escala com a altura da equação (explicando por que só equações multi-linha mostram o
   desvio, e por que o desvio varia entre +7 e +17pt em vez de ser um valor fixo).
3. Confirmar a implementação actual do cristalino no mesmo ponto — `above`/`below` de equação de
   bloco, ou o que decide o espaço entre um bloco de equação e o conteúdo seguinte.
4. Isolar um caso mínimo: uma equação de uma linha e uma equação de duas linhas (por exemplo,
   `frac(a,b)`), ambas numeradas, medir o espaço antes/depois de cada uma nos dois compiladores
   com `compare.py`/`mutool trace` — confirmar a magnitude do efeito isoladamente, fora do
   documento de 30 secções.

## Fase B — Implementação (protocolo de dois agentes de P898 — geometria, mexe no espaçamento de
qualquer equação de bloco no documento inteiro, risco de regressão amplo)

1. Agente A escreve testes com o valor esperado derivado da fórmula real do vanilla (Fase A),
   cobrindo pelo menos: equação de uma linha (delta ≈ 0, guarda de não-regressão), equação de duas
   linhas, equação de três ou mais linhas (confirmar se a relação é linear com o número de linhas
   ou tem outro comportamento).
2. Agente B implementa.
3. Revisão do orquestrador — testar um documento com várias equações de alturas diferentes em
   sequência, confirmar que o espaçamento acumulado ao longo do documento bate com o vanilla, não
   só par a par.
4. Suíte completa verde, discriminada por crate.
5. `cargo run -- .` — zero violations.

## Fase C — Revalidação com `compare.py`

1. Rodar a ferramenta (com a extensão de P951, se já disponível) no documento de 30 secções
   inteiro — confirmar que o padrão sistemático (+7 a +17pt) desaparece, e que a classificação
   passa de "sistemático" para próximo de zero/sem padrão.
2. Confirmar que o ponto fora do padrão (equações 41-42) continua dentro de margem normal, ou
   investigar se virou um outlier real depois da correção do padrão geral (pode ter estado
   mascarado pelo efeito sistemático maior).
3. Benchmark completo, 7 cenários canônicos, `depois/antes`, zero regressão.

## Resultado esperado

- Fórmula real do vanilla para espaçamento de equação multi-linha confirmada por leitura de
  código, não inferida da magnitude observada.
- Correção aplicada, testada com casos de 1/2/3+ linhas.
- `compare.py` confirmando o padrão sistemático fechado no documento completo.
- Benchmark completo, zero regressão.
