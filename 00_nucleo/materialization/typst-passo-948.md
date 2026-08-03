# Passo 948 — ferramenta de comparação geométrica automatizada (posição de glifo/traço, cristalino vs vanilla)

**Precede este passo**: toda a sequência P944-947 — cada achado exigiu alguém decidir manualmente,
por `mutool trace`/inspeção visual, se uma divergência era real ou aparente. Isso já produziu pelo
menos uma contradição entre passos consecutivos (P946 concluiu "sem defeito", P947 teve que
refazer a medição do zero para confirmar). **Este passo constrói a ferramenta para não depender
mais disso.**

**Isto é uma ferramenta de tooling/verificação, não código do compilador** — vive em
`tools/geometry/` (paralelo a `tools/perf/`, que já não passa pelo gate de Nucleação de L0 neste
projecto, mesmo padrão a seguir aqui).

---

## Objectivo

Dado o mesmo `.typ` compilado pelos dois binários (cristalino e vanilla, hash confirmado dos dois
lados, mesmo padrão de proveniência já exigido em toda esta frente), a ferramenta:

1. Extrai, de cada PDF, a posição absoluta (x, y), o texto/codepoint (via `ToUnicode`), e a fonte/
   tamanho de **cada** operador de desenho de glifo no content stream — não só o texto (`pdftotext`
   já mostrou ser insuficiente e confuso), a posição real.
2. **Emparelha** os glifos correspondentes entre os dois PDFs — não por posição absoluta na página
   (que pode divergir por razões legítimas, como tamanho de página diferente), mas por **ordem de
   leitura dentro de cada construção matemática**, usando o texto/codepoint como âncora principal
   e a estrutura de secções do documento de teste (que já é numerado, 1 a 30) para dividir o
   trabalho em unidades comparáveis.
3. Para cada par emparelhado, calcula o delta de posição **relativo à origem da própria construção**
   (por exemplo, relativo ao canto superior esquerdo da equação em que o glifo está, não à página
   inteira) — isto evita falsos positivos de "está tudo deslocado" quando na verdade só a largura
   total da página mudou.
4. Reporta, por secção do documento de 30 seções, uma tabela de deltas (Δx, Δy) por glifo
   emparelhado, com um limiar configurável (por exemplo, 0.5pt) acima do qual a divergência é
   marcada para revisão.

## Fase A — desenhar antes de implementar

1. Confirmar a forma mais fiável de extrair posição real de glifo do content stream — `mutool
   trace` já usado em toda esta frente dá posição, mas confirmar se dá para automatizar o parsing
   do output dele, ou se é preferível usar uma biblioteca (`pikepdf`, já usada em P946, ou
   `pdfminer.six`) para ler os operadores `Tj`/`TJ`/`cm` directamente em Python, com mais controlo
   sobre o formato de saída.
2. Confirmar como emparelhar glifos entre os dois PDFs quando a **ordem de emissão** no content
   stream diverge entre cristalino e vanilla (já visto nesta frente — a ordem de peças de uma
   assembly, por exemplo, pode não ser idêntica mesmo quando o resultado visual é). Desenhar uma
   heurística de emparelhamento robusta a isso — por exemplo, ordenar por posição (x,y) dentro de
   cada secção antes de comparar, não pela ordem bruta de emissão.
3. Decidir o formato de saída — uma tabela (CSV/JSON) por secção, e um resumo agregado (quantos
   glifos por secção excedem o limiar, qual o maior desvio) para permitir triagem rápida das 30
   secções sem ter que abrir todas.
4. Confirmar se a ferramenta precisa de conhecer a estrutura do `.typ` de teste (os marcadores de
   secção "N.") para dividir o documento, ou se consegue inferir unidades comparáveis só a partir
   da geometria (espaços em branco grandes entre blocos) — mais robusto se não depender do
   ficheiro de teste específico, mas confirmar se vale o custo extra de generalidade agora ou se
   fica para depois.

## Fase B — Implementação

1. Escrever a ferramenta (`tools/geometry/compare.py` ou nome equivalente), com teste próprio
   (não precisa do rigor de TDD do compilador, mas precisa de confirmar que funciona: rodar contra
   dois PDFs conhecidos — por exemplo, o mesmo PDF comparado consigo mesmo deve dar delta zero em
   tudo — e um par com uma divergência conhecida injectada manualmente, para confirmar que a
   ferramenta a detecta).
2. Documentar o uso (README dentro de `tools/geometry/` ou secção no próprio script).

## Fase C — rodar contra o `.typ` de 30 seções, produzir o primeiro relatório real

1. Rodar a ferramenta contra o par actual (`test_crystalline.pdf`/`test_vanilla.pdf`, gerados do
   binário mais recente commitado — confirmar qual é, mesmo cuidado de proveniência de P947).
2. Produzir o relatório de 30 secções, triado por maior desvio.
3. **Não corrigir nada neste passo** — o resultado deste passo é o relatório, que se torna o
   ponto de partida objectivo para decidir quais das 30 secções precisam de investigação (em vez
   de continuar a descobrir uma de cada vez, por acaso, como tem acontecido).

## Resultado esperado

- Ferramenta funcional em `tools/geometry/`, testada contra casos conhecidos (delta zero
  auto-comparação; divergência conhecida detectada).
- Relatório completo das 30 secções do documento de teste, com deltas reais, não impressão visual.
- Lista priorizada (por magnitude de desvio) de onde investigar a seguir — a decisão de por onde
  continuar deixa de ser "o que alguém notou olhando", passa a ser "o que a ferramenta mediu como
  maior".
