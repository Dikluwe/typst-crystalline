# Passo 889 — `04-math`: diagnóstico combinado (divergência visual + 18.85× mais lento)

**Precede este passo**: `typst-passo-885-relatorio.md` (achado 1 original, descartado por
comparação com ficheiro vanilla desactualizado), e a tabela de benchmark de `typst-passo-888-
relatorio.md`, secção 8 (`04-math`: 18.48× no estado pós-P888, sem melhoria desde P873).

**Este é um passo de diagnóstico. Não implementar correcção nenhuma aqui** — só confirmar o que
diverge, medir onde o tempo vai, e decidir se é um problema ou dois.

---

## Erro a não repetir

O achado 1 original caiu porque a comparação usou um `vanilla-04-math.pdf` de uma sessão anterior
(P872) contra um `cristalino-04-math-p884.pdf` gerado de um `.typ` já reescrito — as duas metades
não vinham da mesma fonte. **Neste passo, gerar os dois PDFs (vanilla e cristalino) no mesmo
momento, a partir do mesmo `04-math.typ`, e confirmar isso explicitamente no relatório** (por
exemplo, hash do `.typ` usado, ou pelo menos o conteúdo colado no relatório) antes de tirar
qualquer conclusão sobre divergência.

**Pré-condição de árvore**: confirmar `git status`. Trabalho de P888 (`grid.rs`, `layout.md`, testes)
pode continuar por commitar — decidir e registar como nos passos anteriores, não presumir.

---

## Parte 1 — Divergência visual

1. Confirmar qual é o `04-math.typ` **actual** no repositório (o mesmo usado por P888 para o
   benchmark, não um ficheiro em `/tmp` que possa ter sido alterado entre passos). Se o dono do
   projecto tiver um ficheiro `.typ` mais rico em mente (com fração, radical, símbolos gregos,
   delimitadores grandes — não só `sum_(i=0)^n i^2 = alpha + beta`), confirmar qual é antes de
   prosseguir; a investigação de P885/seguimento só cobriu a versão simplificada de uma equação.
2. Compilar esse `.typ` nos dois binários, no mesmo momento, e comparar:
   - Extração de texto dos dois PDFs, lado a lado.
   - Render visual a 150dpi dos dois, lado a lado — não confiar só em extração de texto para
     matemática (motivo já registado em P885: extração pode reordenar/cortar mesmo com PDF
     correto).
3. Se houver glifo(s) ausente(s) no cristalino: identificar exactamente qual símbolo/carácter, e
   se é sistemático (por exemplo, toda uma classe de símbolos, tipo delimitadores extensíveis ou
   letras gregas maiúsculas) ou pontual (um caso específico). Cruzar com
   `01_core/src/rules/math/symbols.rs` e `01_core/src/entities/glyph_variants.rs` — confirmar se o
   símbolo em falta está coberto por `ident_to_unicode`/`shorthand_to_unicode` e se a fonte MATH
   tem a variante de tamanho necessária (`GlyphVariants::select`), antes de presumir onde está o
   problema.
4. Revisitar também a anomalia de espaçamento já registada (relatório de seguimento de P885,
   secção 5: cristalino insere espaço extra em `i=0` → `i  =0` e em `i²` → `i  2`) — confirmar se
   ainda ocorre com o `.typ` actual e se está relacionada com a mesma área de código que qualquer
   glifo ausente encontrado agora, antes de tratar como dois achados sem relação.

## Parte 2 — Tempo (18.48× mais lento que o vanilla)

1. Confirmar se o diagnóstico de P873 (fallback de fonte relendo `.ttc` CJK inteiro sem filtro;
   subsetting CFF desligado) ainda é a causa dominante em `04-math`, ou se esse cenário passa por um
   caminho de código diferente dos outros 6 (fonte MATH é tipicamente um ficheiro `.otf` separado da
   fonte de texto normal, com tabela MATH própria — pode não ser afectado pelas correcções de
   P874–P880, que focaram em fontes de texto/CJK).
2. Perfilar a compilação de `04-math.typ` no binário cristalino (ferramenta disponível no ambiente —
   `perf record`/`flamegraph` ou equivalente já usado em P873) e identificar as funções que dominam
   o tempo. Não presumir que é a mesma causa dos outros cenários sem medir.
3. Comparar contra o tempo do vanilla para o mesmo `.typ` — confirmar se o vanilla também degrada
   com mais equações (o benchmark actual usa `#for i in range(100) { ... }` — cem repetições) ou se
   a curva é aproximadamente linear num e superlinear no outro, o que apontaria para uma estrutura
   de dados errada (ex: busca O(n) repetida em vez de cache) em vez de custo fixo por equação.
4. Se a Parte 1 encontrar um glifo ausente: verificar se buscar (e falhar a encontrar) esse glifo
   está a contribuir para o tempo — por exemplo, fallback repetido tentando várias fontes antes de
   desistir, por equação, 100 vezes. Isto ligaria os dois problemas com uma causa comum; não
   presumir que estão ligados sem essa verificação.

---

## Resultado esperado

Relatório de diagnóstico com:
- Confirmação explícita de que os PDFs comparados vêm do mesmo `.typ`, gerados no mesmo momento
  (hash ou conteúdo do `.typ` colado).
- Veredicto da Parte 1: há ou não divergência visual real; se houver, qual símbolo, sistemático ou
  pontual, e localização candidata no código (sem implementar correcção).
- Veredicto da Parte 2: onde o tempo é gasto (perfil, não suposição), se é a mesma causa de P873 ou
  uma nova, e se a curva é linear ou superlinear com o número de equações.
- Veredicto sobre se a divergência visual (Parte 1) e o tempo (Parte 2) partilham causa raiz ou são
  independentes — com evidência, não suposição, seguindo o mesmo padrão já usado em P886/887 para a
  pergunta equivalente.
- Recomendação explícita: um passo de correcção ou dois, e prioridade sugerida entre eles.

**Não implementar nada neste passo** — só diagnóstico. A correcção fica para um `typst-passo-890`
(ou mais, se a Parte 1 e a Parte 2 se confirmarem independentes) a escrever depois de ler este
relatório.
