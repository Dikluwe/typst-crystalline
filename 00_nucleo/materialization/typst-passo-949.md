# Passo 949 — chave de `cases()`: peça do meio não repete para cobrir a altura (gap de 9.5pt medido por posição real)

**Precede este passo**: achado do dono via `tools/geometry/compare.py` (P948) — medição directa de
posição, não pixel nem texto. Seção 21 (matriz com chave), peças `⎧⎨⎩`: espaço `⎧→⎨` = 1.2pt
(normal), espaço `⎨→⎩` = **9.5pt** (quase uma peça inteira sem nada desenhado). Padrão espelhado
nos dois lados da chave. O parêntese vizinho na mesma seção (`⎛⎜⎜⎜⎜⎜⎝`) repete a peça do meio com
sobreposição de propósito (6pt), sem buraco — a chave é a única que não segue esse padrão.

**Duas coisas já confirmadas, não repetir a investigação**:
1. **Não é fonte diferente** — P946 já confirmou charstring idêntica (`uni23A7`/`uni23A8`/
   `uni23A9`) contra a fonte `NewCMMath` original e o PDF vanilla, byte a byte.
2. **Ausência de `⎧⎨⎩` no texto extraído do vanilla não prova mecanismo diferente** — P946 já
   documentou que o vanilla usa `ToUnicode` diferente (agrupa a assembly inteira num carácter de
   origem só, não emite codepoint real por peça — divergência deliberada nossa, não dele).
   Confirmar isto antes de tratar a ausência como evidência de mecanismo.

**Pré-condição de árvore**: `git status`. Confirmar P945-948 commitados.

---

## Fase A.0 — confirmar que o subset embutido no PDF não corrompeu o glifo (antes de tudo)

O dono mediu, corretamente, que o arquivo `FontFile3` inteiro do cristalino (`CrystallineFont2`,
40.831 bytes) tem hash diferente do `NewCMMath-Book` do vanilla (46.294 bytes) e nome genérico
(`CrystallineFont1-4`). Isso é esperado — PDF sempre faz subsetting, e dois subsetters diferentes
(o do cristalino vs `krilla` no vanilla) produzem contêineres diferentes mesmo partindo da mesma
fonte-fonte original; nome de recurso `CrystallineFontN` é convenção nossa pré-existente, não
sinal de fonte feita sob encomenda. P946 já confirmou, por charstring, que o glifo `uni23A8` bate
com a fonte original `NewCMMath-Book.otf` — mas essa comparação foi feita contra a **fonte
original**, não necessariamente contra o **subset real dentro do PDF gerado**.

1. Extrair a charstring de `uni23A8` (peça do meio da chave) de **dentro do `CrystallineFont2`
   embutido no PDF gerado** (não da fonte original) — via `pikepdf`/`fontTools`, mesmo método de
   P946.
2. Comparar essa charstring com a mesma extraída do `NewCMMath-Book` embutido no PDF do vanilla, e
   com a da fonte original.
3. Se as três baterem: confirma definitivamente que o subsetting não alterou o desenho, e a Fase A
   abaixo (investigação do algoritmo de repetição) é a única coisa que falta.
4. Se a charstring de dentro do PDF cristalino divergir da fonte original: isto seria um achado
   maior e diferente — bug no processo de subsetting do cristalino, não no algoritmo de
   composição. Tratar como achado novo, não presumir que é o mesmo problema do gap de posição.

---

## Fase A — confirmar se a chave passa pelo mesmo caminho de repetição que o parêntese

1. Ler o código que decide, para cada delimitador, se a peça do meio é tratada como extensor
   repetível — confirmar se `{`/`}` e `(`/`)`/`[`/`]` passam pela **mesma** função
   (`layout_assembly`/`resolve_assembly_repeat`, P906/913) ou se a chave tem um caminho próprio,
   mais antigo, com posicionamento fixo de 3 peças (topo/meio/base, sem repetição).
2. Se for a mesma função: confirmar, via `fontTools`, se a peça do meio da chave
   (`uni23A8`/equivalente) está marcada como `isExtender=true` na tabela `MathGlyphPartRecord` de
   `NewCMMath` — se não estiver marcada (ou se o cristalino não estiver lendo essa flag
   corretamente para este glifo específico), o algoritmo de repetição trataria a peça como
   não-repetível, mesmo passando pela função certa, produzindo exatamente o sintoma medido (3
   peças fixas, sem repetição, buraco na altura).
3. Se for caminho diferente (código legado, específico de chave, nunca migrado para o mecanismo
   geral de P906/913): confirmar isso por leitura directa, `file:line` dos dois caminhos.
4. Confirmar se o mesmo problema afeta outros delimitadores com formato "gancho-meio-gancho"
   (não retos) — já catalogado antes como candidato: `⌊⌋`/`⌈⌉` não têm esse formato, mas `{`/`}`
   têm; confirmar se há mais algum símbolo na mesma família que possa ter o mesmo problema.

## Fase B — corrigir a causa exata (só depois da Fase A confirmar qual das duas é)

1. Se for flag `isExtender` não lida/aplicada: corrigir a leitura da flag para este glifo,
   reaproveitando o mecanismo de repetição já validado para parênteses (P913) — não reescrever o
   algoritmo, só corrigir por que a chave não o está a usar.
2. Se for caminho de código separado: migrar a chave para o mecanismo geral de assembly
   (`layout_assembly`), removendo o caminho fixo de 3 peças.
3. TDD directo ou protocolo de dois agentes conforme o tamanho da mudança revelado pela Fase A.
4. Teste com medição de posição real (mesmo método do dono — distância entre peças consecutivas),
   confirmando que o espaço entre peças da chave fica consistente com o espaço entre peças do
   parêntese (mesma sobreposição/gap, não mais 9.5pt vs 1.2pt).

## Fase C — Revalidação com `tools/geometry/compare.py` (P948)

1. Rodar a ferramenta de novo na seção 21 e em qualquer outra seção com `cases()`/chave (seção 9)
   — confirmar que o gap desaparece, com números, não impressão visual.
2. Rodar contra o `.typ` de 30 seções inteiro — confirmar que a correção não introduziu desvio
   novo noutro lugar (mesma disciplina de revalidação completa já estabelecida em P944-947).
3. Benchmark completo, 7 cenários canónicos, `depois/antes`, zero regressão.

## Resultado esperado

- Causa exacta confirmada: flag de extensor não lida/aplicada, ou caminho de código separado da
  chave nunca migrado — não presumida.
- Correção aplicada, reaproveitando o mecanismo já validado de P913 sempre que possível.
- Gap de 9.5pt eliminado, confirmado por medição de posição real (`compare.py`), não visual.
- Revalidação completa das 30 seções, benchmark sem regressão.

---

## Nota sobre o pedido de registo em `00_nucleo`

Sim, registar — mas como achado de investigação (`00_nucleo/diagnosticos/` ou equivalente, ligado
a este passo), não como exemplo avulso solto na spec de `GlyphInstance`. Se a causa confirmada for
genuinamente sobre como `GlyphInstance`/`GlyphAssembly` representa a flag de extensor, a
actualização da spec correspondente acontece como parte da Fase B (L0 antes do código, mesmo
protocolo de sempre), não como registo separado.
