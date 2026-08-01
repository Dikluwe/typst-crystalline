# Passo 945 — delimitador duplicado/desencontrado no caminho de `assembly` (matrizes/casos de 3+ linhas)

**Precede este passo**: `typst-passo-944-relatorio.md` — corrigiu a fonte das equações
(`New Computer Modern Math` forçada), o que fez matrizes/casos de conteúdo alto passarem a usar
o caminho de **assembly** (montagem de delimitador por várias peças, de P906/913) pela primeira
vez de forma consistente num documento real. **Isso expôs um bug pré-existente no assembly**,
nunca visto antes porque o caminho nunca tinha sido exercitado ponta a ponta com a fonte certa.

**Sintoma confirmado**: comparando `test_crystalline.pdf` regenerado (pós-P944) com o texto
extraído, aparecem caracteres de delimitador **duplicados/extra** especificamente em conteúdo de
3+ linhas — matriz 3×3 (seção 5), coluna de 3 elementos (seção 5), vetor `[...]` de 3 elementos
(seção 5), coeficiente binomial (seção 7), `cases()` de 3 ramos (seção 9), matriz 3×3/matriz
aumentada/matriz de reticências 4×4 (seção 21). **Matrizes de 2 linhas (2×2) não têm o problema**
— o `det(...)` 2×2 e a matriz 2×2 simples saem limpos. A contagem de traços extra parece escalar
com o número de linhas/peças de extensor usadas (matriz 4×4 de reticências mostra mais duplicação
que a 3×3).

**Confirmado visualmente pelo dono**: traços de parêntese/chave desencontrados, não alinhados —
bate com "linhas desencontradas" já reportado.

**Pré-condição de árvore**: `git status`. Confirmar correção de P944 commitada e presente.

---

## Fase A — isolar a causa exata no caminho de assembly

1. Reproduzir isoladamente: `$ mat(1,2,3;4,5,6;7,8,9) $` (3×3, o caso mais simples que já mostra
   o defeito) com `#set text(font: "New Computer Modern")` (a mesma configuração do documento de
   teste, agora com a fonte forçada correta dentro da equação por P944).
2. Confirmar, por `mutool trace`, exatamente quantos operadores de desenho de glifo/traço saem
   para o delimitador nesse caso — comparar com o vanilla real para o mesmo `.typ`. Se o
   cristalino desenha mais traços que o vanilla para a mesma matriz, isso confirma duplicação.
3. Ler `layout_assembly`/`layout_assembly_horizontal` (`assembly.rs`, P906/913) e o ponto que
   invoca o assembly a partir de `layout_stretchy_delimiter` (`stretchy.rs`) — confirmar se há
   algum caminho que desenha tanto o resultado do assembly quanto um glifo de delimitador de
   fallback/base **ao mesmo tempo**, em vez de um ou outro exclusivamente. Candidato óbvio a
   confirmar: alguma condição que deveria ser `else if` virou dois blocos independentes que
   ambos executam.
4. Confirmar se a contagem de peças do assembly (repetição de extensor, algoritmo de P913) está
   a gerar peças a mais do que o necessário para a altura pedida — o que produziria traços extra
   sem ser exatamente "duplicação", mas com o mesmo sintoma visual (mais linhas do que devia).
5. Confirmar se `Content::MathDelimited` (o wrapper que envolve o corpo com delimitadores,
   `delimited.rs`) está a aplicar a própria lógica de desenho de delimitador **além** do que
   `layout_stretchy_delimiter` já produziu — dois pontos de código tentando desenhar o mesmo
   delimitador seria a explicação mais direta para "duplicado".

## Fase B — Implementação (protocolo de dois agentes de P898 — geometria, risco alto, mexe em
código já usado por muitos outros casos que hoje parecem correctos com conteúdo mais baixo)

1. Agente A escreve testes cobrindo: matriz 2×2 (deve continuar sem desenhar peça extra, guarda
   de não-regressão), matriz 3×3 (não deve duplicar), matriz 4×4/reticências (não deve duplicar,
   caso com mais peças de extensor). Contagem de operadores de traço como asserção, não só
   "compila".
2. Agente B implementa a correção no ponto exacto confirmado pela Fase A.
3. Revisão do orquestrador — testar `cases()` de 3 ramos e o coeficiente binomial (seção 7)
   também, não só matrizes — confirmar que a causa raiz é realmente comum aos dois consumidores
   de `layout_stretchy_delimiter`/assembly, não uma coincidência de dois bugs parecidos.
4. Suíte completa verde, discriminada por crate.
5. `cargo run -- .` — zero violations.

## Fase C — Revalidação completa (mesma disciplina que P944 já estabeleceu)

1. Recompilar o `.typ` de 30 seções inteiro, comparar visualmente antes/depois desta correção
   (nenhuma regressão nova) e contra o vanilla real (paridade nos casos corrigidos).
2. `mutool trace`/attestation (`L11`) para os casos da Fase A — números reais, não "parece certo".
3. Benchmark completo, 7 cenários canônicos, `depois/antes`, zero regressão.

## Resultado esperado

- Causa exacta da duplicação confirmada por leitura de código + `mutool trace`, não suposição.
- Correção aplicada ao ponto certo, sem afetar matrizes/delimitadores de 2 linhas que já
  funcionavam.
- Revalidação visual completa do documento de 30 seções, registando qualquer achado novo.
- Benchmark completo, zero regressão.

---

## Nota — os outros itens reportados pelo dono, não tratados neste passo

- **Gregas como palavra literal** (`zeta`/`Gamma`/`Psi`/`chi`/`omega`/`alpha`/`tau`) — já
  catalogado em P944 §8.3 item 2. Candidato a passo próprio, provavelmente rápido (mapeamento de
  símbolo, mesma classe de correção de P895/902).
- **Espaçamento/tamanho de fonte em sub/sobrescrito** — parcialmente já catalogado em P944 §4
  (termo `lower_limit_baseline_drop_min`/`upper_limit_baseline_rise_min` em falta em `attach.rs`).
  Candidato a passo próprio.
- **Centralização em parênteses e linha de fração fora do centro** — pode estar relacionado ao
  mesmo bug deste passo (se o delimitador está malformado, o conteúdo dentro dele pode parecer
  descentralizado por reflexo) — **reconfirmar depois deste passo fechar**, antes de abrir
  investigação separada, para não duplicar esforço se a causa for a mesma.
