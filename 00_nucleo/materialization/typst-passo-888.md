# Passo 888 — `grid()`/`table()` desenham 4 segmentos por célula em vez de 1 por linha partilhada

**Precede este passo**: `typst-passo-887-relatorio.md`, secção 7 (achado colateral, registado como
"fora de escopo desta correcção" na nota `P887` de `00_nucleo/prompts/engine/layout.md`). Ler antes
de começar.

**Este NÃO é achado de correcção — é achado de optimização.** A diferença entre 4003 e 371
operadores `S` não produz output visual incorrecto (P887 já confirmou isso — linhas na posição
certa, nas duas versões). É puramente mecânico: tamanho de ficheiro e (potencialmente) tempo de
export. Antes de investir neste passo, confirmar com o dono do projecto se vale a pena face às
outras coisas em aberto — não é obrigatório nem urgente, ao contrário de P886/P887, que corrigiam
comportamento visível errado.

---

## Sintoma confirmado (P887, secção 7)

Para a mesma tabela (`05-tables.typ`, 4 páginas, 20 tabelas 5×10), o cristalino emite **4003**
operadores de stroke (`S`) contra **371** do vanilla — cerca de 10.8×. `layout_grid`
(`01_core/src/engine/layout/table.rs`, comentário P227/P230/P234 referenciado no relatório) desenha
4 segmentos de borda por célula (top/bottom/left/right individuais), em vez de 1 segmento por linha
de grelha, partilhado entre as duas células adjacentes que o tocam.

Efeito prático de desenhar 4× por célula em vez de 1× por linha partilhada:
- Toda linha interior da grelha é desenhada **duas vezes** sobrepostas (a borda direita da célula N
  e a borda esquerda da célula N+1 ocupam a mesma posição).
- O ficheiro fica maior sem ganho visual (dois traços sobrepostos rendem igual a um, assumindo
  mesmo stroke — mas não é garantido se o `stroke` variar por célula via `Celled`, ver Fase A ponto
  3).

## Fase A — Diagnóstico

1. Confirmar exactamente onde em `layout_grid` (ou equivalente pós-reorganização de camadas — P887
   já usou `01_core/src/engine/layout/table.rs`, confirmar que ainda é o ficheiro certo) as 4 bordas
   por célula são emitidas, e se há alguma razão arquitectural para isto ser deliberado (por exemplo,
   suporte a `stroke` por célula individual via `table.cell(stroke: ...)`, que o vanilla também
   suporta via `Celled<Sides<...>>` — ver referência em P887 secção 1.3). **Se `stroke` puder variar
   célula a célula**, colapsar para 1 segmento por linha partilhada pode não ser trivialmente
   correcto quando as duas células adjacentes têm strokes diferentes — o vanilla decide isso de
   alguma forma (conferir `crates/typst-library/src/layout/grid/lines.rs` ou equivalente no
   `lab/typst-original/`, não presumir).
2. Confirmar como o vanilla decide desenhar uma linha só uma vez quando é partilhada, incluindo o
   que faz quando os dois lados têm strokes diferentes (precedência, mistura, ou desenha os dois
   parcialmente). Isto é o comportamento de referência a replicar — ler o código, não assumir "o
   vanilla deve estar a fazer a coisa óbvia".
3. Medir se este é só um problema de tamanho de ficheiro ou também de tempo de export. P887 (secção
   9) mostrou `05-tables` subir de 113.9ms para 122.5ms depois da correcção do stroke (+7.5%,
   atribuído a "desenhar linhas que antes não eram desenhadas"). Não está isolado quanto disso é
   trabalho novo legítimo (as linhas que faltavam) vs quanto é o excesso de segmentos duplicados
   deste achado. Não é preciso decompor com precisão, mas vale registar a suspeita.

## Fase B — Implementação (TDD, per `CLAUDE.md`)

**Só depois da Fase A confirmar que colapsar para 1 segmento por linha partilhada é seguro** face a
`stroke` por célula (ou, se não for sempre seguro, confirmar as condições em que é seguro e
implementar só nesse subconjunto, com fallback para o comportamento actual nos outros casos).

1. Escrever teste(s) que falhem primeiro:
   - Contagem de `FrameItem::Shape::Line` emitidos por um `grid()`/`table()` N×M com stroke uniforme
     — deve ser igual ao número de linhas de grelha únicas (`(N-1) linhas horizontais interiores +
     2 bordas externas horizontais) × largura + equivalente vertical`, não 4× o número de células.
   - Se a Fase A confirmar que strokes diferentes por célula têm de continuar a desenhar segmentos
     separados: teste específico para esse caso, confirmando que o comportamento antigo (segmentos
     não colapsados) se mantém só onde é necessário.
2. Implementar a correcção no ponto identificado pela Fase A.
3. Suíte completa verde, discriminada por crate.
4. Recompilar `05-tables.typ` (fonte actual) e confirmar visualmente que a grelha continua idêntica
   em aparência (nenhuma linha a menos, nenhuma linha deslocada) e que a contagem de operadores `S`
   caiu para uma ordem de grandeza próxima da do vanilla (não precisa ser exactamente 371 — o
   cristalino pode ter uma estratégia de agrupamento diferente — mas não deve continuar em milhares
   para uma tabela deste tamanho).
5. `cargo run -- .` — zero violations.

## Fase C — Regressão

Mesma exigência de P886/P887 — correr o benchmark completo dos 7 cenários e comparar com a baseline
de P887 (`timings-*-p887.json` se existir; senão a de P886). Atenção especial a `05-tables` — espera-
se que fique **mais rápido** que o estado pós-P887 (menos operadores a escrever/comprimir), não mais
lento; se subir, é sinal de que a correcção introduziu overhead novo em vez de eliminar redundância,
e vale investigar antes de fechar.

## Resultado esperado

- Header de linhagem actualizado no(s) ficheiro(s) tocado(s).
- Testes novos cobrindo contagem de segmentos (e, se aplicável, o caso de stroke por célula
  divergente).
- Relatório do passo com: veredicto da Fase A (se e como `stroke` por célula limita a colapsagem),
  contagem de operadores `S` antes/depois, números do benchmark completo da Fase C.
- Se a Fase A concluir que não vale a pena (por exemplo, complexidade desproporcional ao ganho, ou
  risco de quebrar `stroke` por célula), registar essa conclusão explicitamente e fechar o passo como
  "avaliado, não implementado" em vez de forçar uma implementação frágil.
