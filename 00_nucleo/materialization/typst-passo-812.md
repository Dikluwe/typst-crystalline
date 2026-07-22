# Prompt — typst-passo-812: resto do achado #13 (`math::style`) — display/script/sscript, itálico em wrappers, `scr`, letras ausentes

**Origem**: achado #13 de P810 (`math::style`), parte que ficou fora do escopo de P811 (que resolveu só a corrupção do PDF por `frak()`/documento sem páginas)
**Estado**: aguardando execução

---

## Achado (texto do relatório de P810, parte restante)

> `display`/`inline`/`script`/`sscript` **sem efeito geométrico**; itálico P809 perdido em wrappers de tamanho; scr bloco errado + sem variation selectors; `NN`/`RR`/`ZZ`/`QQ`/`CC` ausentes

São quatro sub-achados dentro do mesmo módulo, com causas prováveis distintas. Este prompt trata os quatro em sequência, cada um com sonda própria — não assumir que a correcção de um resolve os outros sem medir.

---

## Regra da linha de trabalho (obrigatória)

Não aceitar "corrigido" sem execução mostrada. Cada afirmação do relatório tem de vir com comando exacto + saída literal, comparando vanilla (`lab/typst-original/target/release/typst`) e cristalino (`./target/release/typst`). A contagem de testes da suíte `typst-core` tem de aparecer e bater com os testes novos declarados. Onde a lição de P811 se aplicar (um sintoma pode ter causa mais ampla do que o caso citado), isolar a variável antes de corrigir só o caso literal do achado.

---

## Sub-achado A — `display`/`inline`/`script`/`sscript` sem efeito geométrico

### Sonda
1. Compilar `$display(x/y)$`, `$inline(x/y)$`, `$script(x/y)$`, `$sscript(x/y)$` (ou a sintaxe exacta usada no `.typ` original do achado #13 — confirmar no relatório de materialização de P810 antes de assumir) com os dois binários. Medir geometricamente (`mutool trace`, tamanho de fonte efectivo dos elementos, não só extracção de texto) se há alguma diferença de escala/posição entre os quatro estilos no cristalino — o achado diz que não há nenhuma.
2. Localizar no vanilla (`lab/typst-original/`) onde `MathSize`/estilo de contexto (display/text/script/scriptscript) altera o factor de escala aplicado a glifos e espaçamento — normalmente uma propriedade de `MathContext`/`StyleChain` consumida no layout.
3. Localizar no cristalino o ponto equivalente: é provável que `display()`/`inline()`/`script()`/`sscript()` estejam registados como funções que envolvem o conteúdo (ex.: `Content::Styled` com uma custom key), mas que o layout matemático nunca leia essa key para ajustar escala — verificar se a key existe e se é consumida em algum lugar de `01_core/src/engine/math/`.

### Implementação
Aplicar o factor de escala correcto por nível de estilo no layout matemático, lendo a custom key (ou criando-a, se não existir) nos pontos onde tamanho de fonte e espaçamento matemático são calculados.

### Validação
`mutool trace` mostrando tamanho/posição de glifo diferente entre os quatro níveis, batendo com o vanilla (proporções, não necessariamente pixels exactos — registar a proporção medida no vanilla, ex. script ≈ 0.7× text, scriptscript ≈ 0.5×, conforme o valor real encontrado no Passo 2).

---

## Sub-achado B — itálico de P809 perdido dentro de wrappers de tamanho

### Sonda
1. Compilar `$display(x)$`/`$script(x)$` etc. com os dois binários e comparar se o `x` continua itálico (P809) dentro do wrapper — o achado diz que perde o estilo.
2. Confirmar a hipótese já registada em P810 (`letter_base` retorna `None` para size variants): localizar `letter_base` (ou equivalente) e ver se o caminho de resolução de estilo de letra único não reconhece o conteúdo quando embrulhado por `display`/`script`/etc.

### Implementação
Ajustar o caminho para que a detecção de "letra única" (que dispara o itálico por defeito de P809) atravesse o wrapper de tamanho, em vez de parar nele.

### Validação
Repetir o Passo 1 da sonda deste sub-achado, extracção idêntica ao vanilla (`𝑥` dentro do wrapper, não `x`).

---

## Sub-achado C — `scr` com bloco Unicode errado + sem variation selectors

### Sonda
1. Compilar `$scr(A)$` (ou a sintaxe usada no achado) com os dois binários, comparar o codepoint exacto extraído.
2. Localizar no vanilla a tabela de mapeamento para `scr` (script/calligraphic) — confirmar o bloco Unicode correcto e se o vanilla emite variation selector (U+FE00/FE01) para desambiguar entre script e calligraphic no mesmo bloco base.
3. Localizar no cristalino `map_glyph` (tocado em P809 para grego) e confirmar o bloco usado para `scr` hoje.

### Implementação
Corrigir o bloco Unicode de `scr` em `map_glyph`. Para os variation selectors: o achado de P810 já registou que isto pode ser uma limitação estrutural (`map_glyph` devolve 1 char, o codex do vanilla usa `[char; 2]`) — se for esse o caso, decidir entre (a) estender `map_glyph` para devolver até 2 chars, replicando a estrutura do vanilla, ou (b) registar como scope-out formal (nova entrada de dívida) se o esforço for desproporcional. Não decidir sozinho por (b) sem apresentar a opção (a) com estimativa de esforço, seguindo o mesmo padrão usado em P807 para decisões de escopo.

### Validação
Codepoint do bloco corrigido, extracção idêntica ao vanilla para o caso sem variation selector. Se (a) for escolhido, também o caso com selector.

---

## Sub-achado D — `NN`/`RR`/`ZZ`/`QQ`/`CC` ausentes

### Sonda
1. Compilar `$NN$`, `$RR$`, `$ZZ$`, `$QQ$`, `$CC$` com os dois binários — confirmar que o cristalino não reconhece esses identificadores (provavelmente `unknown variable`) enquanto o vanilla os resolve para os símbolos de conjunto numérico em blackboard bold (ℕ, ℝ, ℤ, ℚ, ℂ).
2. Confirmar se estes já existem como símbolos nomeados na tabela de símbolos do cristalino sob outro nome, ou se estão totalmente ausentes.

### Implementação
Registar os cinco identificadores na tabela de símbolos matemáticos do cristalino, mapeando para os codepoints correctos (blackboard bold — mesma família Unicode já tratada para `frak`/`bb` em P811).

### Validação
Os cinco compilam e extraem o codepoint correcto, idêntico ao vanilla.

---

## Relatório final

Produzir `00_nucleo/materialization/typst-passo-812-relatorio.md` cobrindo os quatro sub-achados (A, B, C, D) separadamente, cada um com:
- Comando + saída literal antes/depois.
- Trecho do código vanilla e cristalino relevante.
- Diff da correcção.
- Testes novos, nomeados por sub-achado (ex.: `p812a_...`, `p812b_...`).
- Se o sub-achado C resultar em scope-out formal em vez de implementação, documentar a decisão e a nova entrada de dívida, seguindo o padrão de DEBT-66.
- Contagem de testes antes/depois da suíte `typst-core` (e infra, se algum sub-achado tocar export).
