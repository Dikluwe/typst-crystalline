# Passo 890 — fallback de fonte matemática paga scan caro apesar de `math_fallback_font_list()` já existir

**Precede este passo**: `typst-passo-889-relatorio.md`, secções 2.1–2.5. Ler antes de começar —
os números e a localização candidata já estão lá, este prompt não repete a medição, só formaliza
a correcção.

**Pré-condição de árvore**: confirmar `git status`. P889 foi diagnóstico puro, não deve ter deixado
nada por commitar, mas confirmar de qualquer forma antes de editar.

---

## Sintoma confirmado (P889)

Qualquer identificador matemático de letra única (`i`, `n`, auto-itálico) ou letra grega (`alpha`,
`beta`) dispara um custo fixo de **~5s** (User 0.67s / System 4.34s, RSS 7GB, 1.78M minor page
faults, 78.55% do tempo em `read()`), pago **uma vez por compilação**, não por ocorrência —
confirmado com `range(1)` a `range(100)` no mesmo `.typ`, tempo idêntico nos quatro. O custo é ler
repetidamente `.ttc` CJK grandes (`NotoSansCJK-*.ttc`, `NotoSerifCJK-*.ttc`, 20-27MB cada, 21+21+11+11
aberturas medidas).

**O que torna isto absurdo, não só lento**: `NewComputerModernMath-Regular.otf` — primeira fonte da
cadeia `DEFAULT_FALLBACK_FONTS_MATH` — já tem glifo directo para os quatro codepoints testados
(confirmado via `fontTools`, P889 secção 2.4). A resposta certa está na primeira posição da lista
certa. O scan pelas dezenas de fontes do sistema, incluindo `.ttc` CJK, não deveria acontecer.

**Vanilla, mesmo `.typ`**: User 0.21s / System 0.07s. Sem custo fixo equivalente.

## Onde procurar (P889 já localizou os candidatos, não confirmou qual falha)

Dois mecanismos já existem no código e deveriam evitar isto:

- `03_infra/src/shaper.rs` — `try_shape`/`shaped_width` (~linhas 185-235/360-410): quando
  `style.math` é verdadeiro, adiciona `math_fallback_font_list()` como primárias adicionais **antes**
  do scan global (comentário "P783/P784" já no código).
- `03_infra/src/font_metrics.rs` — `text_ink_bounds` (linhas ~1019-1057) e `covering` (linhas
  ~734-770, comentário "P784"/"P880"): `covering()` só cai no scan caro
  (`self.world.candidates_for_char(c)`, linha ~754) se a lista `primary` falhar a cobrir o carácter.

P889 não confirmou, por leitura de código isolada, se `primary` de facto contém
`NewComputerModernMath` no momento em que o shaper resolve `𝑖`/`α`, nem em que ponto exacto a cadeia
falha caso não contenha.

## Fase A — Instrumentação temporária (obrigatória antes de corrigir)

1. Instrumentar `covering()` (e/ou `try_shape`/`shaped_width`, conforme o caminho real percorrido
   por este caso — confirmar qual dos dois é exercitado primeiro para conteúdo matemático, não
   presumir) para imprimir, na compilação de `04-math.typ`:
   - Se `style.math` está de facto `true` neste ponto.
   - O conteúdo exacto de `primary`/`math_fallback_font_list()` no momento da chamada.
   - Se `NewComputerModernMath` está nessa lista.
   - Se `primary.covers(c)` (ou equivalente) retorna `true`/`false` para `𝑖`/`α` **antes** de cair no
     scan global.
2. Rodar com a instrumentação e confirmar exactamente qual das hipóteses é real:
   - `style.math` não está `true` neste ponto (a informação de "estamos em modo math" perdeu-se
     algures entre o parser/layout e o shaper), ou
   - `style.math` está correcto, mas `math_fallback_font_list()` não inclui
     `NewComputerModernMath` (lista errada/incompleta), ou
   - a lista está correcta, mas `covers(c)` retorna `false` incorrectamente para este glifo
     específico (falso negativo na verificação de cobertura, apesar do glifo existir — testar
     directamente contra o `fontTools` já usado em P889 para confirmar que é falso negativo, não
     falta real), ou
   - outra causa, a determinar pela instrumentação.
3. **Reverter a instrumentação antes de avançar para a Fase B** — não deixar `println!`/`eprintln!`
   de diagnóstico no código final.

## Fase B — Implementação (TDD, per `CLAUDE.md`)

Só depois da Fase A identificar a causa exacta.

1. Escrever teste(s) que falhem primeiro. Nível sugerido: teste de unidade sobre a função
   identificada na Fase A, confirmando que, para `style.math = true` e um carácter coberto por
   `NewComputerModernMath`, a função devolve essa fonte **sem** consultar `candidates_for_char`
   (pode precisar de um mock/spy sobre `World` para confirmar que o método caro não foi chamado —
   não só que o resultado final está certo, já que o resultado final já estava certo antes, só
   lento).
2. Implementar a correcção no ponto exacto confirmado pela Fase A.
3. Suíte completa verde, discriminada por crate.
4. Medir `04-math.typ` de novo com `/usr/bin/time -v` e `strace -f -c` (mesmo método de P889) —
   confirmar que o tempo cai para a ordem de grandeza do vanilla (P889 estimou "provavelmente
   < 100ms", não é um número garantido, medir de facto) e que a contagem de aberturas dos `.ttc`
   CJK cai a zero (ou próximo) para este caso.
5. Confirmar visualmente que `04-math.typ` continua a renderizar `𝑖`, `α`, `β`, `𝑛` correctamente
   depois da correcção — mudar a cadeia de resolução de fonte é o tipo de correcção que pode trocar
   o glifo certo por um visualmente diferente (mesmo Unicode, fonte diferente, forma diferente).
6. `cargo run -- .` — zero violations.

## Fase C — Regressão

Esta correcção mexe em `shaper.rs`/`font_metrics.rs`, os dois ficheiros centrais de toda a frente
de performance de P872–P888. Correr o benchmark completo dos 7 cenários — atenção a `03-images`
(fallback de fonte também é relevante lá, per handoff pós-P884) e a qualquer cenário com texto CJK,
já que a correcção mexe directamente na lógica de fallback/cobertura que também serve esses casos.
Não fechar só com `04-math` melhorando — confirmar que nada mais regrediu.

## Resultado esperado

- Header de linhagem actualizado no(s) ficheiro(s) tocado(s).
- Instrumentação de diagnóstico removida antes do commit final.
- Teste(s) novo(s) confirmando que o caminho rápido é tomado (não só que o resultado final está
  certo).
- Relatório com: causa exacta confirmada na Fase A, tempo antes/depois, contagem de aberturas de
  `.ttc` antes/depois, confirmação visual de que os glifos continuam correctos, benchmark completo
  da Fase C.
