# Relatório — typst-passo-891: espaçamento anómalo em `04-math` (Fase A, Partes A e B)

**Data:** 2026-07-24T13:39:22Z
**Executor:** Claude (Sonnet 5)
**Commit base:** `326ced44899862bbe1190386b5834f200bb4115d` (HEAD do ramo `Tekt`, após P888)
**Working tree no início:** trabalho de P890 (`font_metrics.rs`, `font_metrics.md`) ainda por
commitar, completo e testado (suíte verde, benchmark confirmado em `typst-passo-890-relatorio.md`).
**Decisão registada**: as áreas de código não se sobrepõem na maior parte (`spacing.rs` é
completamente novo território) — **excepto** que a correcção da Parte B deste passo (achado de
`math_kern`) volta a tocar `font_metrics.rs`, o mesmo ficheiro que P890 já alterou. Decisão: construir
por cima do trabalho de P890 (não reverter), já que está completo e testado — não misturar os dois
diffs de forma confusa (as secções deste passo em `font_metrics.rs` ficam claramente marcadas com
`P891` nos comentários, distintas das de P890).

Este relatório cobre a Fase A de ambas as partes (instrumentação da Parte B já revertida). Nenhum
código de produção foi alterado ainda — só os L0s, como o gate exige antes da Fase B.

---

## Parte A — `i=0`: thick space em torno de `=` dentro de sub-índice

### Fase A, ponto 1 — comportamento exacto do vanilla (lido, não presumido)

`lab/typst-original/crates/typst-library/src/math/ir/process.rs:277-319`, função `spacing`:

```rust
let script = |f: &MathItem| f.size().is_some_and(|s| s <= MathSize::Script);

match (l.rclass(), r.lclass()) {
    (Relation, Relation) => {}
    (Relation, _) if !script(l) => l.set_rspace(Some(THICK)),
    (_, Relation) if !script(r) => r.set_lspace(Some(THICK)),
    ...
}
```

**Resposta confirmada**: a condição "unless in script size" **suprime o espaço por completo**
(não reduz para um valor menor) — quando o item está em `MathSize::Script` ou mais fundo
(`ScriptScript`), simplesmente não se chama `set_rspace`/`set_lspace`, ficando sem espaço extra
nenhum. A verificação é feita **por lado, independentemente** (`!script(l)` e `!script(r)` são
condições separadas) — o vanilla tem um `MathSize` discreto por item, propagado individualmente.

### Fase A, ponto 2 — como o cristalino sabe (ou não) que está em script size

Confirmado por leitura de `01_core/src/engine/math/layout/mod.rs:684-702` (`layout_sequence`):
`compute_gaps(&filtered, style.size.val())` é chamado com **um único `TextStyle` para toda a
sequência** — todos os nós processados numa mesma chamada partilham o mesmo estilo (e portanto o
mesmo "tamanho"/profundidade de script), ao contrário do vanilla que tem tamanho discreto por item.
`attach.rs:31-34` já constrói um `script_style` com `size: style.size *
self.constants.script_percent_scale_down` para o conteúdo de sub/sup — mas este `script_style`
**não carrega nenhuma marca explícita de "estou em script size"** hoje; `compute_gaps` não tem
acesso a essa informação.

**Adaptação decidida** (dado que cristalino processa a sequência inteira com um estilo partilhado,
não precisa da granularidade por-item do vanilla para o caso observado): adicionar
`TextStyle::math_script: bool` (mesmo padrão do `.math` de P784), colocado a `true` por `attach.rs`
ao construir `script_style`. `compute_gaps` passa a receber esse booleano e, quando verdadeiro,
suprime todas as regras (paridade com o comportamento confirmado do vanilla para sequências
inteiras em script size — o caso de `i=0`). Registado como scope-out residual (não silencioso): uma
sequência com tamanhos MISTOS ao mesmo nível (que o vanilla resolveria por item) não é tratada — não
é um caso observado nos benchmarks actuais.

### Gate do L0

`00_nucleo/prompts/engine/math/layout/spacing.md` documentava a condição "unless in script size"
como "fora de escopo" (P772y) — corrigido: nova secção `P891` substitui essa entrada, documenta o
comportamento vanilla confirmado, a adaptação decidida, e a assinatura nova de `compute_gaps`
(`nodes, size_pt, in_script`).

---

## Parte B — `i²`: gap antes do expoente

### Fase A — instrumentação (feita, confirmada, revertida)

Tentativa inicial de instrumentar `math_kern()` directamente em `attach.rs`/`metrics.rs` —
**bloqueada**: ambos os ficheiros são L1 (`@layer L1`), e `CLAUDE.md` proíbe I/O (`eprintln!`
conta) em L1 de forma absoluta, mesmo temporariamente. Instrumentação movida para L3
(`03_infra/src/font_metrics.rs`), onde I/O é permitido: adicionado temporariamente um override de
`math_kern` em `impl FontMetrics for FallbackFontMetrics<'_>` que imprime e depois devolve
exactamente o mesmo `MathGlyphKern::default()` que o comportamento actual (sem override) já produz
— confirma se este código é de facto o caminho activo, sem mudar o resultado observável durante o
teste.

Corrido com `$ i^2 $` isolado:
```
P891-DEBUG FallbackFontMetrics::math_kern (via trait default) chamado para c='𝑖' (U+1D456)
```

**Confirma diretamente**: `FallbackFontMetrics::math_kern` é chamado (via o default do trait, já
que não havia override antes desta instrumentação) para a base `𝑖`, devolvendo kern zero.
**Revertido** imediatamente a seguir (`grep -rn "P891-DEBUG"` — 0 ocorrências antes de prosseguir).

### Causa exacta

**Sem sequer precisar da instrumentação para confirmar isto** (mecanismo estático, sem ambiguidade
de despacho): `01_core/src/engine/layout/metrics.rs`, trait `FontMetrics`, método `math_kern` tem
um default:
```rust
fn math_kern(&self, c: char) -> MathGlyphKern {
    let _ = c;
    MathGlyphKern::default()
}
```
`grep -rn "fn math_kern"` em todo o `01_core`/`03_infra` mostra **só duas** implementações reais:
o próprio default (acima) e `FontBookMetrics::math_kern` (`font_metrics.rs:349-405`, variante de
face única, usada em testes isolados). **`FallbackFontMetrics` — a variante multi-fonte
efectivamente usada em produção (confirmado o caminho em P890) — nunca sobrepôs `math_kern`**,
herdando sempre o default (kern zero incondicional). A instrumentação confirma que este é o caminho
realmente exercitado, não só uma possibilidade teórica.

`attach.rs:51-52` usa `base_kern.top_right`/etc. para aproximar o expoente de uma base itálica
inclinada — com kern sempre zero, essa aproximação nunca acontece, produzindo o gap confirmado em
P889 (`i  ²`).

### Achado colateral, fora de âmbito deste passo

O trait `FontMetrics` tem **mais três** métodos math-específicos com default e sem override em
`FallbackFontMetrics`: `math_constants` (usa `MathConstants::fallback()`, não a tabela MATH real da
fonte activa — afecta TODAS as proporções da equação, não só um kern pontual), `vertical_glyph_
variants` e `vertical_glyph_assembly` (delimitadores extensíveis podem não crescer). **Não
instrumentados nem corrigidos neste passo** — só `math_kern` foi confirmado como causa do sintoma
medido em P889; os outros três ficam registados para um passo dedicado futuro, dado o impacto
potencial ser maior (proporções globais da equação, não um gap pontual).

### Correcção proposta

`FallbackFontMetrics::math_kern` ganha implementação real — mas precisa de saber **qual face
activa** cobre `c` (a variante multi-fonte tem uma cadeia de candidatos, não uma face fixa como
`FontBookMetrics`). Isto requer `style: &TextStyle` — ausente da assinatura actual do trait.
Correcção: `math_kern(&self, c: char, style: &TextStyle) -> MathGlyphKern` (default inalterado,
ignora os dois parâmetros). `FallbackFontMetrics` resolve a face via
`resolve_primary_with_math_fallback` + `covering` (mesmo mecanismo de P890) e lê a tabela MATH
dessa face — lógica de leitura extraída de `FontBookMetrics::math_kern` para uma função livre
partilhada (`math_kern_from_face`). `attach.rs` (único consumidor de produção) passa `style` no
call site. `impl FontMetrics for &dyn FontMetrics` (wrapper P858) não reencaminha `math_kern` nem
antes nem depois deste passo — fora de âmbito (caminho de produção usa o tipo concreto, confirmado
em P890).

### Gate do L0

Três L0s actualizados:
- `00_nucleo/prompts/infra/font_metrics.md` — nova secção `P891` com a causa completa e a correcção
  proposta (incluindo o achado colateral dos três métodos não corrigidos).
- `00_nucleo/prompts/engine/math/layout/attach.md` — assinatura nova do call site
  (`math_kern(c, style)`) e a causa.
- `00_nucleo/prompts/engine/layout.md` — nova secção `P891` documentando a mudança de assinatura do
  trait (`metrics.rs`), motivo, e a nota sobre `&dyn FontMetrics` não reencaminhar (inalterado).

---

## Fase B — Parte A: `i=0` (TDD, implementação, suíte, visual)

**Data:** 2026-07-24T14:26:02Z. **Commit base:** `326ced448` (mesmo HEAD; trabalho ainda não
commitado — working tree tinha só o backlog pré-existente + este passo).

### L0 adicional descoberto durante a implementação

Adicionar `TextStyle::math_script` exigiu um **quinto** L0 além dos quatro já editados na Fase A:
`00_nucleo/prompts/entities/layout_types.md` (documenta cada campo novo de `TextStyle` como secção
numerada, precedente `## P784`/`## P836`). Nova secção `## P891 — campo TextStyle::math_script: bool`
adicionada seguindo o mesmo template, antes de tocar código — mesmo gate, aplicado de novo assim que
a necessidade apareceu.

Grep confirmou (precedente de P784) que dois sites de construção não-spread de `TextStyle` precisam
do campo explícito: `entities/style_chain.rs::From<&StyleChain>` (`math_script: false`) e
`engine/layout/text.rs::resolve_effective_style` (`math_script: layouter.style.math_script`, herda
sem lógica). Ambos os L0s dessas duas unidades (`entities/style_chain.md`, `engine/
atomizacao_elementos.md`) também ganharam a secção espelho (`### math_script: false sempre (P891)`,
`## §16 — P891`), mesmo padrão das secções P784 já existentes nesses ficheiros.

### TDD

1. Assinatura de `compute_gaps` mudou para `(nodes, size_pt, in_script: bool)` com `in_script`
   ignorado (`let _ = in_script`) — mudança puramente mecânica, todos os 6 call sites de teste
   pré-existentes actualizados para passar `false`; suite de `spacing.rs` (26 testes) confirmada
   verde **sem mudança de comportamento** antes de escrever qualquer lógica nova.
2. Dois testes novos escritos **antes** da implementação:
   `p891_in_script_suprime_thick_de_relation` (`i=0`: par Alphabetic/Relation e Relation/Normal, `10pt` →
   esperado `[0.0, 0.0]`) e `p891_in_script_suprime_medium_de_binary` (par Binary). Corridos e
   **confirmados a falhar** (`left: [2.77…, 2.77…], right: [0.0, 0.0]` e `left: [2.22…, 2.22…]`) —
   valores batem exactamente com `THICK`/`MEDIUM` não suprimidos, confirmando que o teste exercita o
   caminho certo antes do fix.
3. Implementado: `compute_gaps` retorna `vec![0.0; n-1]` cedo quando `in_script` — suprime por
   completo, paridade `process.rs::spacing()` vanilla (não reduz, per Fase A). `attach.rs` marca
   `math_script: true` no `script_style` construído para sub/sup. `mod.rs::layout_sequence` passa
   `style.math_script` como terceiro argumento.
4. Suite `spacing.rs`: 29/29 verde (26 antigos + 3 novos, incluindo um terceiro teste de regressão
   `p891_fora_de_script_continua_normal` confirmando que `in_script=false` continua a produzir THICK
   normalmente). Suite completa `typst-core` (`cargo test -p typst-core --lib`): **4701 passed, 0
   failed**. Build de todo o workspace: limpo.

### Confirmação visual

Binário release recompilado (`cargo build --release -p typst-wiring --bin typst`). `04-math.typ`
recriado em scratch com o **mesmo hash exacto** de P889 (`sha256:f48b18113c828b37…80a9c5e`),
recompilado e traçado via `mutool trace`:

| Par | x glifo esquerdo | adv (pt) | x esperado sem gap | x glifo direito | Gap extra |
|---|---|---|---|---|---|
| `𝑖` → `=` | 270.533 | 2.6565 (.345×7.7) | 273.190 | 273.190 | **0.0pt** |
| `=` → `0` | 273.190 | 4.235 (.550×7.7) | 277.425 | 277.425 | **0.0pt** |

Ambos os pares dentro do sub-índice (`i=0`) mostram gap extra **zero** — antes da correcção, o
mesmo par produziria `THICK × 7.7 ≈ 2.14pt` de cada lado (confirmado pelo teste unitário com os
mesmos valores). Confirmado sem quebrar o caso normal (fora de script): suite `spacing.rs` mantém os
testes pré-existentes (`a_igual_b_produz_thick_dos_dois_lados` etc.) inalterados e verdes.

---

## Fase B — Parte B: `i²` (TDD, implementação, suíte, visual)

### Mudança mecânica de assinatura primeiro

`FontMetrics::math_kern` mudou de `fn(&self, c: char)` para `fn(&self, c: char, style: &TextStyle)`
— default do trait actualizado (ignora ambos), `FontBookMetrics::math_kern` actualizado para aceitar
(e ignorar) `style`, `attach.rs`/`tests.rs` (um teste unitário `fixed_metrics_math_kern_vazio`
chamava `math_kern('f')` directamente) actualizados para passar o argumento novo. Suite completa
(`cargo test --workspace`) confirmada verde **antes** de qualquer lógica nova — mudança mecânica,
sem alteração de comportamento.

### TDD

Extraída a lógica de leitura da tabela MATH de `FontBookMetrics::math_kern` para uma função livre
partilhada `math_kern_from_face(face: &ttf_parser::Face, c: char) -> MathGlyphKern` (byte-idêntica ao
código antigo, só movida). `FontBookMetrics::math_kern` passa a delegar nela.

Teste novo escrito **antes** da implementação de `FallbackFontMetrics::math_kern`:
`p891_fallback_font_metrics_math_kern_le_tabela_math_real`. Localiza a fonte `NewCMMath` real
embutida via `typst_assets::fonts()` **por cobertura** (tabela MATH presente + kern não-trivial para
`𝐷`, U+1D437 — as 3 variantes embutidas Regular/Bold/Book não expõem nome PostScript decodificável
nas plataformas que `ttf_parser` lê por omissão, confirmado por instrumentação temporária depois
revertida). O valor esperado (`kern.bottom_right = -80` design units) é calculado **directamente via
ttf_parser no próprio teste**, não hardcoded — não fica frágil a uma actualização do asset.

Corrido antes da implementação: **falhou** exactamente como esperado (`left: None, right:
Some(-80.0)` — `FallbackFontMetrics` ainda herdava o default do trait). Implementado
`FallbackFontMetrics::math_kern`: resolve a face candidata via `resolve_primary_with_math_fallback` +
`covering` (mesmo mecanismo de P890, `text_ink_bounds`), chama `math_kern_from_face` na face
resolvida. Teste corrido de novo: **passou**.

Suite completa (`cargo test --workspace`): **todas as 9 test-binaries verdes** (`typst-core` 4701,
`typst-infra` 734 — 733 + o novo, `typst-shell` 41/2/37/2, doc-tests 0/3 ignored). `crystalline-lint
.`: 0 drift novo (só o V7 pré-existente, não relacionado).

### Confirmação visual — achado importante (medido, não presumido)

Binário release recompilado com as duas correcções. Repetido o traçado de `04-math.typ`
(`sum_(i=0)^n i^2 = alpha + beta`):

`𝑖` (base, U+1D456) em `attach.rs`: x=281.005 (trm=11, adv=.345→3.795pt) → `²` em x=284.800. Gap
extra = **0.0pt**, idêntico ao valor pré-correcção (kern zero em ambos os casos).

**Isto não é uma falha da correcção** — verificado directamente: `kerns.get(gid)` para `𝑖`
(U+1D456) devolve `None` (sem registo de kern nenhum, não só quadrantes vazios) nas 3 variantes
embutidas de `NewCMMath` (confirmado por instrumentação temporária). A tabela MATH real desta fonte
**não tem kern definido para este glifo específico** — kern zero é o valor **correcto**, não um
efeito residual do bug.

**Confirmado que a correcção tem efeito real** onde a fonte tem dados: recompilado `$ D_2 $`
isolado (D tem `bottom_right` kern = -80 design units, confirmado no teste unitário). Traçado:
D em x=291.736 (trm=11, adv=.828→9.108pt) → sem kern esperava-se `2` em x=300.844; medido x=299.964.
Diferença = **-0.880pt**, batendo exactamente com `-80 unidades × 11pt ÷ 1000 upem = -0.88pt`
(assumindo upem=1000, confirmado pela fonte). **A correcção move visivelmente o sub-índice**, quando
a fonte fornece o dado.

**Comparação directa vanilla vs cristalino para `04-math.typ`** (ambos recompilados no mesmo
momento, mesmo `.typ`, mesmo hash sha256 de P889): vanilla usa `NewCMMath-Book` (não `-Regular` —
diferença de variante de peso na selecção da fonte math por omissão entre vanilla e cristalino,
**não investigada aqui, fora de âmbito**); para o par `𝑖`→`²`, vanilla também produz gap extra
**0.0pt** (x=281.2113→285.00633, diferença=3.79503pt = advance exacto de `𝑖`, `.345×11`). **Vanilla
e cristalino produzem o mesmo resultado (zero) para este par específico, com e sem a correcção** —
o sintoma visual original de P889 (`i²` colado no vanilla vs `i  ²` com gap no cristalino) **não se
reproduz** nesta comparação fresca, mesmo binário/fonte/momento. Duas explicações possíveis,
nenhuma confirmada aqui (fora de âmbito deste passo): (a) a medição visual original de P889 pode ter
sido influenciada pela mesma classe de erro metodológico já documentado nesse relatório para os
achados 1/4 (comparação de builds/momentos não sincronizados); ou (b) a causa real do "gap" original
é um mecanismo diferente — `italic_correction` (campo per-glifo da tabela MATH, distinto de
`kern_infos`, que `attach.rs` **nunca lê**, confirmado por leitura directa do código — não é um dos
3 achados colaterais já registados em P891/Fase A). **Registado para investigação futura dedicada,
não decidido aqui**: esta correcção (Parte B) está correcta e confirmada para o mecanismo que
implementa (`math_kern`/`kern_infos`), mas não fecha por si só o sintoma visual original de `i²` —
que pode exigir `italic_correction`, achado novo, fora do âmbito desta passo.

---

## Fase C — Regressão (benchmark completo, 7 cenários)

Mesmo protocolo de P890 (`hyperfine --warmup 2 -N`, mesmos `.typ` de `/tmp/p872-bench/`, binários
recompilados neste passo). Comparação com a linha de base de P890 (mesma tabela):

| Cenário | Cristalino (P890) | Cristalino (P891) | Δ |
|---|---|---|---|
| 01-hello | 94.9ms | 94.4ms | ruído |
| 02-lorem | 119.6ms | 117.6ms | ruído |
| 03-images | 103.5ms | 103.4ms | ruído |
| 04-math | 153.7ms | 153.9ms | ruído |
| 05-tables | 115.4ms | 115.1ms | ruído |
| 06-long | 371.9ms | 372.4ms | ruído |
| 07-context | 136.5ms | 136.1ms | ruído |

**Nenhuma regressão** — todos os 7 cenários dentro do ruído de hyperfine face a P890, confirmando a
expectativa do próprio prompt (correcções de espaçamento/kern não deviam ter impacto mensurável de
tempo).

---

## Resultado — Passo 891 fechado

- **Parte A (`i=0`)**: implementada, testada (29/29 `spacing.rs`, 4701/4701 `typst-core`),
  confirmada visualmente (gap extra 0.0pt dentro do sub-índice, sem regressão fora de script).
- **Parte B (`math_kern`)**: mecanismo corrigido, testado (novo teste com fonte MATH real,
  ground-truth calculado no próprio teste), confirmado visualmente com efeito real onde a fonte tem
  dados (`D_2`, -0.88pt). **Não fecha** o sintoma visual original de `i²` de P889 — esse sintoma não
  se reproduziu numa comparação fresca vanilla-vs-cristalino, e a causa pode ser `italic_correction`
  (achado novo, não implementado, fora de âmbito).
- Suite completa (`cargo test --workspace`): verde, 0 falhas.
- `crystalline-lint .`: 0 drift novo.
- Fase C: 7/7 cenários sem regressão.
- 5 L0s tocados no total neste passo: `spacing.md`, `attach.md`, `font_metrics.md`, `layout.md`,
  `entities/layout_types.md` (+ secções espelho em `entities/style_chain.md` e `engine/
  atomizacao_elementos.md`, ambos já geridos pela mesma disciplina de L0 por precedente P784).
- Diffs de Parte A e Parte B **não se sobrepõem** excepto em `attach.rs` (Parte A não o toca; Parte B
  só muda o call site de `math_kern`) — cada secção deste relatório referencia exactamente os
  ficheiros/testes da sua parte, sem misturar.
- **Achado novo registado para passo futuro dedicado**: `italic_correction` (campo MATH per-glifo,
  candidato real para o "gap" de `i²` que este passo não fechou) — juntar-se aos 3 achados
  colaterais já registados (`math_constants`, `vertical_glyph_variants`, `vertical_glyph_assembly`)
  como candidatos a uma futura revisão dedicada de cobertura `FontMetrics`.
