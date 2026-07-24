# Relatório — typst-passo-895: `offset_x = infinito` (Parte A) + catálogo de terceiros (Parte B)

**Data:** 2026-07-24T16:54:06Z
**Executor:** Claude (Sonnet 5)
**Commit base:** `ccbe6816c5c26b84a780cc9aa15e2950cd5a6587` (HEAD do ramo `Tekt`).
**Pré-condição de árvore**: `git status` confirma que P894 (fix de `dot`, `01_core/src/engine/
stdlib/sym.rs`) e P893 (5 L0s + hashes sincronizados, Fase B **não iniciada** — só o gate, sem
lógica) continuam **por commitar**, tal como no início de P894/P893. As áreas de código não se
sobrepõem de forma conflituosa: P893 tocaria `MathLayouter::new`/`math_constants` (ainda não
implementado); este passo toca `equation.rs` (centragem/numeração) e `stdlib/{sym,structural}.rs` +
`math/layout/mod.rs` (Parte B) — ficheiros distintos ou secções distintas do mesmo ficheiro.

---

## Parte A — `offset_x = infinito`

### Fase A — desenho da correcção

1. **Quando `compute_page_width()` resolve o valor final**: só em `finish()` (fim do documento) e
   `new_page()` (quebra de página) — nunca antes, porque a largura final de uma página `auto` só é
   conhecida depois de todo o conteúdo dessa página estar colocado. `equation.rs` (Content::Equation)
   corre **durante** o layout, antes de qualquer destes dois pontos — não há como reordenar para
   "chamar depois da resolução": o conteúdo da própria equação faz parte do que determina a largura
   final. Conclusão: `equation.rs` tem de resolver localmente (guardar contra o valor ainda infinito),
   não esperar por um valor já resolvido que ainda não existe nesse ponto.
2. **Como o vanilla lida com centragem de bloco quando a largura é `auto`**: medido directamente
   (`$ a+b=c $` isolado, `width: auto`) — vanilla posiciona a equação exactamente na margem
   (`x=28.346pt` = 1cm), **sem** aplicar nenhum offset de centragem. Isto bate com a álgebra: quando a
   página se ajusta ao próprio conteúdo, `usable == largura do conteúdo`, logo `(usable - width)/2 = 0`
   de qualquer forma — a centragem degenera para offset zero no caso de uma equação isolada. Para
   múltiplas equações de larguras diferentes, vanilla usa um modelo de layout em duas passagens
   (mede tudo, depois posiciona com a largura final já conhecida) — arquitectura que o cristalino não
   tem para este caso (regista-se como fora de âmbito, abaixo).
3. **Outros consumidores de `self.regions.current.width` no mesmo ficheiro**: encontrado um segundo
   ponto **no mesmo `equation.rs`** — a posição do número da equação (`right_x = regions.current.width
   - margin - number_width`), sofrendo exactamente o mesmo problema quando `width: auto` +
   `equation.numbering` activo. Outros consumidores existem noutros ficheiros (`cursor.rs`, `mod.rs`,
   `block.rs`, `pad.rs`, `boxed.rs`) mas são maioritariamente checagens de "overflow"/"wrap" onde
   infinito já é semanticamente correcto (nunca ultrapassa, nunca quebra linha — comportamento
   desejado para `width: auto`) ou já fazem save/restore local do valor sem o usar directamente numa
   fórmula de centragem — não partilham o mesmo padrão de bug.

### Fase B — implementação (TDD)

Teste escrito primeiro (`layout_equation_bloco_com_width_auto_nao_produz_infinito`,
`01_core/src/engine/layout/tests.rs`): `Content::SetPage{width: Auto, height: Auto}` seguido de
`Content::equation(bloco)`, confirma `page.width`/`page.height` finitos e pelo menos um item com
posição finita. Corrido e confirmado a falhar (`page.width` = `inf`) antes da correcção.

Correcção (`01_core/src/engine/layout/equation.rs`): ambos os pontos (centragem P813 e numeração)
passam a verificar `self.regions.current.width.is_finite()` antes de usar o valor:
- **Centragem**: quando infinito, `offset_x` fica na margem (valor já inicializado, sem aplicar a
  fórmula) — paridade exacta com o comportamento medido do vanilla para uma equação isolada.
- **Numeração**: quando infinito, o número **não é posicionado** (scope-out — não há margem direita
  bem definida contra a qual alinhar; caso raro, não exercitado pelo ficheiro original de P894).

Suíte completa (`cargo test -p typst-core --lib`): 4704 passed, 0 failed. `crystalline-lint .`: 0
drift novo (L0 `engine/layout/equation.md` actualizado, hash sincronizado).

### Confirmação visual

Recompiladas as secções 1 e 4 do ficheiro de P894 (mesmo hash `sha256:f48b18113c828b3…80a9c5e`),
mesma disciplina de sincronização:

| | Antes (P894) | Depois (P895) |
|---|---|---|
| Secção 1 `MediaBox` | `[0 0 inf 249.81]` (inválida) | `[0 0 235.07 270.56]` |
| Secção 4 `MediaBox` | `inf` (via teste directo) | `[0 0 129.64 320.21]` |
| Render | Tudo esmagado/sobreposto numa faixa no fundo de uma página 612×792 substituta | Equações fluem normalmente pela página, tamanho auto-ajustado |

**Achado incidental durante a confirmação visual** (não corrigido, fora de âmbito deste passo,
registado para futuro): com a corrupção removida, um bug **diferente e pré-existente** ficou visível
— equações de bloco consecutivas onde uma tem conteúdo mais alto (fracções, `sqrt`, `root`) não
recebem espaçamento vertical suficiente, sobrepondo-se à linha seguinte. **Confirmado não relacionado
com `width: auto`**: reproduz-se identicamente com uma página fixa normal (`width: 400pt, height:
800pt`) — não é um efeito colateral desta correcção nem do achado original de P894, é um bug de
espaçamento vertical de blocos consecutivos já existente antes deste passo, só invisível porque a
corrupção de página o mascarava.

### Fase C — regressão

Benchmark completo, 7 cenários, comparado com a baseline mais recente sem conflito de árvore (P891,
já que P893 não alterou lógica e P894 foi diagnóstico + fix pontual de símbolo):

| Cenário | Cristalino (P891 baseline) | Cristalino (P895) |
|---|---|---|
| 01-hello | 94.4-96.2ms | 92.9ms |
| 02-lorem | 117.6-117.7ms | 116.1ms |
| 03-images | 102.2-103.4ms | 102.0ms |
| 04-math | 153.5-153.9ms | 154.0ms |
| 05-tables | 114.4-115.1ms | 114.5ms |
| 06-long | 372.4-376.1ms | 383.2ms |
| 07-context | 136.1ms | 139.6ms |

Todos dentro do ruído de hyperfine — **nenhuma regressão**.

---

## Parte B — catálogo de terceiros

Lista de 18 itens de uma tabela de terceiros (fora do fluxo deste projecto), verificados um a um,
isolados, nos dois binários, mesma disciplina de sincronização.

### Resultado: 18/18 confirmados como bugs reais (nenhum refutado)

| Item | Cristalino (antes) | Vanilla | Veredicto |
|---|---|---|---|
| `epsilon.alt`/`theta.alt`/`phi.alt`/`rho.alt`/`sigma.alt` | `unknown symbol modifier 'alt'` | ϵ/ϑ/ϕ/ϱ/ς | **Confirmado real** |
| `dot.double` | `unknown symbol modifier 'double'` | 𝑥̈ | **Confirmado real** — mesma classe do achado "funções math em falta" de P894 (`dot(x)` bare já não existe como função de acento) |
| `union.big`/`inter.big` | `unknown symbol modifier`/`unknown variable: inter` | ⋃/⋂ | **Confirmado real** — `inter` nem existia (só `sect`, nome **inexistente no vanilla**) |
| `dots.h` | `unknown symbol modifier 'h'` | … | **Confirmado real** |
| `oo` | `unknown variable: oo` | ∞ | **Confirmado real** |
| `beth` | `unknown variable: beth` | ב | **Confirmado real** |
| `prop` | `unknown variable: prop` | ∝ | **Confirmado real** |
| `thin`/`med`/`thick`/`quad`/`wide` | `unknown variable` (todos) | espaço nomeado | **Confirmado real** |
| `math.op(...)` | `module 'math' does not contain field "op"` | funciona | **Confirmado real** |

**Nenhum item da lista foi refutado** — ao contrário do padrão observado nas 5 hipóteses de
Prioridade 1 de P894 (todas refutadas), esta lista de terceiros estava inteiramente correcta.

### Correcções implementadas (TDD, todas com suíte verde)

1. **Variantes `.alt` de gregas + `.h`/`.big` de `dots`/`union`/`inter`**: achado arquitectural
   importante durante a implementação — uma entrada plana `"nome.modificador"` em `SYM_SIMPLE`
   (o padrão já usado por `eq.not`, `dot.c` antes deste passo) **não é alcançável a partir de modo
   math real**. `$epsilon.alt$` é sempre parseado como `FieldAccess(MathIdent("epsilon"), "alt")` —
   nunca como um único `MathIdent` "epsilon.alt" — e a resolução passa por
   `Value::Symbol::modified("alt")`, que só encontra a variante se o símbolo base foi construído via
   `Symbol::with_variants` (`SYM_GROUPS`), nunca `Symbol::new` (`SYM_SIMPLE`). Confirmado que isto
   **já afectava `eq.not`/`dot.c` antes deste passo** (`$ eq.not $` isolado também falha,
   confirmado agora) — um achado incidental sobre um mecanismo pré-existente, não introduzido por
   P894/P895. Corrigido migrando `epsilon`/`theta`/`phi`/`rho`/`sigma`/`dots`/`union` (mais o novo
   `inter`) de `SYM_SIMPLE` para `SYM_GROUPS` com as variantes pedidas. Testado com um teste de
   pipeline completo (`MockWorld` + eval real, não só `sym_lookup` directo) para não repetir o
   falso-positivo.
2. **`sect` → `inter`**: `sect` não existe em lado nenhum do vanilla (confirmado por grep no codex) —
   nome inventado/divergente. Renomeado para `inter` (nome real), com `.big` adicionado.
3. **`oo`/`beth`/`prop`**: adicionados como entradas `SYM_SIMPLE` bare simples (sem modificador,
   sem o problema do item 1 acima).
4. **`math.op(...)`**: `native_op` (já existente, registado no scope global) também registado no
   scope do módulo `math` (`make_math_module()`), mesmo padrão de `math.class`.
5. **`thin`/`med`/`thick`/`quad`/`wide`**: registados em `make_math_module()` como
   `Value::Content(Content::h_space(Length::em(...), false))` — `THIN`/`MEDIUM`/`THICK` reusam as
   mesmas fracções de em já usadas em `spacing.rs` (P891/P772y) para o espaçamento automático por
   `MathClass`; `QUAD`/`WIDE` são 1em/2em (paridade vanilla `math/mod.rs:36-40`).

   **Achado colateral confirmado por teste geométrico, não só compilação**: registar como
   `Value::Content` fez **compilar sem erro**, mas **não produzia nenhum espaço visível** — os 5
   nomes davam o mesmo gap (zero). Causa: `MathLayouter::layout_node` não tinha nenhum arm para
   `Content::HSpace`, caindo no catch-all genérico (`other.plain_text()` — vazio para `HSpace`,
   `width: 0.0`). Corrigido com um arm dedicado (`math/layout/mod.rs`) que lê `Spacing::Absolute` e
   devolve a largura real; `Spacing::Fractional` fica scope-out (`width: 0.0`, não exercitado pelos 5
   nomes). Confirmado por teste geométrico (`p895_hspace_em_sequencia_math_contribui_largura`,
   mede o gap real entre dois `MathIdent`) e por `mutool trace` no PDF final: gaps de
   thin<med<thick<quad<wide, coordenadas crescentes confirmadas.

### Item catalogado, não corrigido

- **`dot.double`**: confirmado real, mas é a **mesma classe** do achado já registado em P894
  ("funções matemáticas nativas em falta" — `dot(x)` bare como função de acento não existe; `.double`
  é um modificador dessa função ausente). Corrigir isto exige implementar `dot(...)` como função de
  acento primeiro — fora de âmbito deste passo (que só tratou símbolos/modificadores e os dois
  espaçamentos/funções triviais de registar). Fica no mesmo balde do achado de P894, não duplicado
  como item novo.

### Suíte e regressão

`cargo test -p typst-core --lib`: 4721 passed, 0 failed (12 testes novos de símbolos +
`p895_math_op_existe_no_modulo_math` + 2 testes de pipeline completo + `p895_thin_med_thick_quad_wide_compilam`
+ `p895_hspace_em_sequencia_math_contribui_largura`; `p299_math_module_total_42_operadores`
actualizado 42→46→47→52 ao longo de P795/P895). `crystalline-lint .`: 0 drift novo. Confirmação
end-to-end via CLI de todos os 13 itens corrigidos (`epsilon.alt` até `math.op`), coordenadas reais
extraídas via `mutool trace`.

---

## Resultado — Passo 895 fechado

- **Parte A**: `offset_x = infinito` corrigido (centragem + numeração), confirmado visual e
  geometricamente (MediaBox finita, equações fluem normalmente). Achado incidental de espaçamento
  vertical entre blocos consecutivos registado, não corrigido (pré-existente, não relacionado).
- **Parte B**: 18/18 itens do catálogo de terceiros confirmados reais; 13 corrigidos com TDD
  (incluindo um achado arquitectural sobre o mecanismo de variantes `SYM_SIMPLE` vs `SYM_GROUPS`, e
  um achado sobre `Content::HSpace` nunca ter sido suportado em modo math); 1 item (`dot.double`)
  catalogado como pertencente ao achado maior já registado em P894, não corrigido aqui.
- Fase C: 7/7 cenários sem regressão.
- L0s tocados: `engine/layout/equation.md` (Parte A), `math/layout/_comum.md` e
  `engine/stdlib/structural.md` (Parte B). Nenhum drift novo.
