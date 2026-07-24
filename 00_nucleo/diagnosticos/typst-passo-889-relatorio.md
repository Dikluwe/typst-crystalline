# Relatório — typst-passo-889: `04-math` diagnóstico combinado (divergência visual + 18.48× mais lento)

**Data:** 2026-07-24T11:49:22Z
**Executor:** Claude (Sonnet 5)
**Commit base:** `326ced44899862bbe1190386b5834f200bb4115d` (HEAD do ramo `Tekt`, após P888)
**Working tree:** limpa, exceto `00_nucleo/materialization/typst-passo-889.md` (não commitado, este
próprio passo). Trabalho de P888 já estava commitado quando este passo começou — nada a decidir
sobre árvore não commitada.

**Passo puramente diagnóstico** — nenhum código foi alterado.

---

## 0. Confirmação da fonte usada (erro de P885 a não repetir)

Perguntado ao dono do projecto se havia um `.typ` mais rico em mente (fração, radical, gregas
maiúsculas, delimitadores grandes) antes de prosseguir — resposta: usar o `.typ` simples actual,
mantendo consistência com o benchmark de P872–P888.

**`04-math.typ` usado** (`/tmp/p889/04-math.typ`, copiado do benchmark, hash
`sha256:f48b18113c828b37050167c0102ef7ec6c1537faaef471c07c08fc75580a9c5e`):

```typst
#for i in range(100) {
  $ sum_(i=0)^n i^2 = alpha + beta $
}
```

**Os dois PDFs deste relatório foram gerados no mesmo momento, a partir deste ficheiro exacto**:
```
$ /home/dikluwe/.../lab/typst-original/target/release/typst compile /tmp/p889/04-math.typ /tmp/p889/vanilla.pdf
$ /home/dikluwe/.../target/release/typst /tmp/p889/04-math.typ /tmp/p889/cristalino.pdf
```
(2026-07-24T11:38:25Z, binários recompilados no início deste passo a partir do commit acima).

---

## 1. Parte 1 — Divergência visual

### 1.1 Veredicto: sem glifos ausentes; achado 1 original (P885) continua descartado

Extração de texto e render a 150dpi dos dois PDFs (mesmo momento, mesma fonte, ver secção 0)
mostram os mesmos símbolos nos dois: `∑`, `i`, `n`, `=`, `0`, `2`, `α`, `β`, `+`. Nenhum carácter em
falta, nenhum tofu, nenhum símbolo trocado. **Confirmado visualmente** (não só por extração de
texto, que já se sabia não bastar para matemática desde P885) — a hipótese original do achado 1
(fracção/segunda equação em falta) continua sem fundamento com o `.typ` actual, tal como o
relatório de seguimento de P885 já tinha concluído por outra via (fonte vanilla desactualizada).

### 1.2 Anomalia real confirmada: espaçamento extra — **duas causas distintas**, não uma

Visualmente confirmado (render 150dpi, não só texto extraído): o cristalino insere espaço extra em
dois pontos específicos da equação, ambos ausentes no vanilla:

| Local | Vanilla | Cristalino |
|---|---|---|
| Expoente de `i` | `i²` (colado) | `i  ²` (gap visível) |
| Sub-índice do somatório | `i=0` (colado) | `i  = 0` (gap visível, dos dois lados de `=`) |

Nenhum gap em `α + β` (ambos idênticos, espaço normal de operador binário) — a anomalia não é geral
a toda a equação, é localizada a estes dois pontos.

#### 1.2.a `i=0` (sub-índice do somatório) — causa confirmada

`01_core/src/engine/math/layout/spacing.rs` já **documenta esta limitação no próprio cabeçalho**
(linhas 9-14, não é descoberta nova deste passo, é confirmação de um scope-out já registado):

> "Fora de escopo (registado, não silencioso): a condição 'unless in script size' de cada regra
> vanilla (...)"

`i=0` dentro de `sum_(i=0)^n` é uma sequência matemática (`MathIdent("i")` [Alphabetic],
`MathText("=")` [Relation], `0` [Normal]) processada por `compute_gaps`/`spacing_between`
(`spacing.rs:91-121`). A regra `(Relation, _) => THICK` e `(_, Relation) => THICK` insere espaço
grosso **dos dois lados** de `=` — exactamente o padrão visual observado (`i  = 0`). O vanilla
suprime (ou reduz) esta regra quando o conteúdo está em "script size" (dentro de um sub-índice/
super-índice); o cristalino não tem essa condição implementada — o comentário do próprio ficheiro já
avisava disto antes deste passo. **Nível de confiança: alto** — a regra e o efeito batem
exactamente, e a causa já estava documentada no código, não é inferência nova.

#### 1.2.b `i²` (expoente) — causa candidata, não confirmada com a mesma força

`i^2` é um `MathAttach` (base `i`, `sup: 2`) — não passa por `compute_gaps`/`spacing_between`
(não há operador entre base e expoente; `attach.rs` não chama nenhuma função de `spacing.rs`). A
posição do expoente vem de `base_kern.top_right` (`attach.rs:219-227`), que depende de
`self.metrics.math_kern(c)` (`03_infra/src/font_metrics.rs:349-374`) — kerning específico do glifo
base, lido da tabela OpenType MATH (`MathGlyphInfo`/`kern_infos`) da fonte activa. Se esse lookup
falhar silenciosamente (glifo não encontrado na face activa, ou tabela MATH ausente na face
efectivamente usada), `base_kern` fica `MathGlyphKern::default()` (kern zero) — sem a aproximação
que normalmente "encaixa" o expoente mais perto de uma base itálica inclinada como `i`.

Confirmado que a base `i` chega a este ponto já convertida para o Unicode itálico matemático
(`\u{1D456}`, via `apply_math_style`/tabela de transformação em `math/layout/mod.rs:942-1057`) e
que **a fonte matemática primária (`NewComputerModernMath`) tem glifo directo para esse codepoint**
(confirmado via `fontTools`, secção 2.4 abaixo) — o que não descarta a hipótese de kern zero, mas
não a confirma de forma directa: não instrumentei `math_kern()` para ver se retorna `default()`
neste caso específico. **Nível de confiança: candidato razoável, não confirmado** — o que
refutaria: instrumentar `math_kern('\u{1D456}')` com a face realmente activa neste layout e
comparar com uma inspecção directa da tabela MATH da fonte via `fonttools`/`ttx` para esse glifo
específico (não feito neste passo — diagnóstico, não instrumentação de código).

**Duas causas distintas, mesma categoria de sintoma** (espaçamento visualmente incorrecto), mas
mecanismos de código diferentes (`spacing.rs` vs `attach.rs`) — não é um só achado.

### 1.3 Anomalia de `Section #i` (registada em P885/seguimento) — confirmado **não relacionada**

O relatório de seguimento de P885 (secção 5) registou `Section 0` → `Section    0` (texto comum,
não matemática) com o mesmo "padrão visual" de gap. **Confirmado, por leitura de código, que não
pode ser a mesma causa**: nem `spacing.rs` (`MathClass`/`compute_gaps`) nem `attach.rs`
(`MathGlyphKern`) são exercitados fora de conteúdo matemático — texto comum (`heading`, `#i`
interpolado) nunca passa por estas funções. A anomalia de `Section #i` tem de vir de um mecanismo
terceiro, não investigado neste passo (fora do escopo — P889 é sobre `04-math`). Fica registado
para quem decidir investigar essa anomalia separadamente: **não presumir a mesma causa**.

### 1.4 Localização candidata para uma correcção futura (sem implementar)

| Anomalia | Ficheiro | Correcção candidata |
|---|---|---|
| `i=0` (thick space em torno de `=` dentro do sub-índice) | `01_core/src/engine/math/layout/spacing.rs` | Implementar a condição "unless in script size" (suprimir ou reduzir `spacing_between` quando o nó está dentro de um `MathAttach`/script) |
| `i²` (gap antes do expoente) | `01_core/src/engine/math/layout/attach.rs` + `03_infra/src/font_metrics.rs::math_kern` | Confirmar por instrumentação se `math_kern` retorna default (zero) para a base itálica neste caso, antes de decidir a correcção |

---

## 2. Parte 2 — Tempo (18.48× mais lento)

### 2.1 Mesma causa de P873, confirmado (não é caminho de código diferente)

`/usr/bin/time -v` no `04-math.typ` actual: **User 0.67s / System 4.34s** (elapsed 5.02s), RSS
máxima **7.07 GB**, 1.78M minor page faults — mesma assinatura de P873 (dominância de tempo de
sistema sobre CPU, RSS anormalmente alta). `strace -f -c`: 78,55% do tempo em `read()` (3021
chamadas). Contagem de aberturas dos ficheiros `.ttc` CJK grandes:

```
21 NotoSansCJK-Regular.ttc
21 NotoSansCJK-Bold.ttc
11 NotoSerifCJK-Regular.ttc
11 NotoSerifCJK-Bold.ttc
```

**Números idênticos aos medidos em P873** (antes de qualquer correcção de P874–P888). Confirma:
nenhuma das correcções de P874–P888 (subsetting CFF, default de stroke, fusão de segmentos de
tabela) tocou este mecanismo — é a mesma causa, não um caminho de código diferente para `04-math`.

### 2.2 Achado novo: o custo é **fixo** (não escala com o número de equações)

Testado `range(1)`, `range(10)`, `range(50)`, `range(100)` no mesmo `.typ`:

| N equações | Tempo real | System |
|---|---|---|
| 1 | 4.98s | 4.31s |
| 10 | 4.99s | 4.37s |
| 50 | 5.03s | 4.37s |
| 100 | 4.99s | 4.29s |

**O tempo não muda com o número de equações** — é um custo fixo, de arranque, pago uma vez por
compilação (não por equação). Isto **refuta directamente** a hipótese de "estrutura de dados O(n)
repetida por equação" levantada no prompt do passo (ponto 3 da Parte 2) — não há degradação alguma
com mais repetições porque o resultado da resolução de fallback é cacheado após a primeira vez
(`FaceCache`/`FontSlot::OnceLock`, já confirmado em P873). O custo de ~5s acontece **uma vez**,
na primeira ocorrência de um símbolo que dispare o fallback caro.

### 2.3 Símbolo exacto isolado: qualquer letra itálica matemática ou grega — não é geral a "matemática"

Testado cada símbolo da equação isoladamente (`$ x $` para cada `x`):

| Símbolo | Tempo |
|---|---|
| `i` (identificador de letra única) | **4.94s** |
| `n` (idem) | **4.97s** |
| `alpha` (α) | **5.02s** |
| `beta` (β) | **4.98s** |
| `0` (dígito) | 0.11s |
| `2` (dígito) | 0.11s |
| `=` | 0.11s |
| `+` | 0.12s |
| `sum` (∑) sozinho | 0.11s |

**Isolamento limpo**: qualquer identificador de letra única (auto-itálico) ou letra grega dispara o
custo fixo de ~5s; dígitos, operadores ASCII e o próprio símbolo `sum` sozinho não disparam nada.
Isto significa que praticamente qualquer equação realista (que quase sempre tem pelo menos uma
variável ou letra grega) paga este custo — não é um caso de nicho.

### 2.4 O custo é desnecessário: a fonte primária já tem o glifo

Confirmado via `fontTools` que `NewComputerModernMath-Regular.otf` (primeira da cadeia
`DEFAULT_FALLBACK_FONTS_MATH`, `fallback_fonts.rs`) **já tem glifo directo** para os quatro
codepoints testados:

```
italic i (U+1D456) -> FOUND
alpha (U+03B1)     -> FOUND
beta (U+03B2)      -> FOUND
italic n (U+1D45B) -> FOUND
```

Ou seja: a resposta certa está disponível na **primeira** fonte candidata da cadeia dedicada — o
scan caro pelas dezenas de fontes do sistema (incluindo os `.ttc` CJK de 20-27MB) não deveria ser
necessário. Isto não é "faltava cobertura, teve de procurar" — é "a cobertura já estava ali, e
mesmo assim procurou em todo o `FontBook`".

### 2.5 Onde a cadeia rápida deveria intervir — não confirmado por que falha

Dois mecanismos já existentes no código deveriam evitar o scan caro:

- `03_infra/src/shaper.rs` (`try_shape`/`shaped_width`, ambos linhas ~185-235/360-410): quando
  `style.math` é verdadeiro, adiciona `math_fallback_font_list()` como primárias adicionais **antes**
  do scan global (comentário próprio, "P783/P784").
- `03_infra/src/font_metrics.rs::text_ink_bounds` (linhas 1019-1057) e `::covering` (linhas 734-770):
  mesmo padrão (comentário "P784"/"P880") — `covering()` só cai no scan caro
  (`self.world.candidates_for_char(c)`, linha 754) se a lista `primary` (que devia já conter
  `NewComputerModernMath`) falhar a cobrir o carácter.

**Não confirmei, por leitura de código isolada, por que estas duas optimizações já existentes não
impedem o scan caro para este caso específico** — precisaria de instrumentação (não feita, fora do
escopo de um passo de diagnóstico) para ver, em tempo de execução, se `primary` realmente contém
`NewComputerModernMath` no momento em que `covering()`/o shaper resolve `𝑖`/`α`, e se não, em que
ponto exacto a cadeia falha. **Isto é a lacuna mais importante a fechar antes de corrigir** — não
presumir qual das duas funções (ou uma terceira, ainda não encontrada) é a culpada sem confirmar.

### 2.6 Vanilla não degrada

Vanilla no mesmo `.typ`: User 0.21s / System 0.07s — sem qualquer sinal de custo fixo grande. A
"curva" do vanilla é plana/rápida para o mesmo conjunto de símbolos — reforça que o problema é
específico da estratégia de resolução de fallback do cristalino, não inerente a processar itálicos
matemáticos ou letras gregas.

---

## 3. Parte 1 e Parte 2 partilham causa raiz? — Não, confirmado independentes

| | Parte 1 (espaçamento) | Parte 2 (tempo) |
|---|---|---|
| Mecanismo | `spacing.rs` (regras `MathClass` sobre sequência) / `attach.rs` (`MathGlyphKern`, tabela OpenType MATH da face já resolvida) | `shaper.rs`/`font_metrics.rs` (resolução de **qual fonte** usar, fallback entre famílias) |
| Preocupação | Geometria/tipografia entre glifos **já resolvidos** | Selecção de fonte **antes** de desenhar qualquer glifo |
| Evidência de independência | Nenhuma das duas funções de espaçamento chama código de resolução de fonte; nenhuma trata de qual `.ttc`/família usar | O scan caro (`candidates_for_char`) não faz nada relacionado com posicionamento/kerning — só decide qual face cobre o carácter |

Confirmado por leitura directa do código (não suposição): são três mecanismos genuinamente
distintos (spacing.rs, attach.rs, shaper.rs/font_metrics.rs), cada um com o seu próprio ficheiro,
sem chamadas cruzadas entre eles. Corrigir um não deve afectar os outros.

**Ponto 4 da Parte 2 do prompt** ("se a Parte 1 encontrar glifo ausente, verificar se buscá-lo
contribui para o tempo") não se aplica directamente — a Parte 1 não encontrou glifo ausente (a
busca de fonte, por mais cara que seja, **sempre encontra** um glifo válido; o resultado visual é
espaçamento incorrecto, não ausência de glifo). O elo que existe é mais fraco do que "a mesma
causa": ambos os problemas acontecem porque o conteúdo é matemático e envolve letras/gregas, mas
divergem completamente a partir daí em mecanismos de código não relacionados.

---

## 4. Recomendação

**Dois passos de correcção separados, não um**:

1. **`typst-passo-890` (prioridade alta — tempo)**: instrumentar (temporariamente, revertendo antes
   de fechar) `covering()`/`try_shape`/`shaped_width` para confirmar exactamente por que o scan caro
   ainda dispara apesar das optimizações P783/P784/P880 já existentes, e corrigir. Prioridade alta
   porque afecta **qualquer** documento com uma variável matemática ou letra grega — não é caso de
   nicho, e o ganho potencial é enorme (de ~5s fixos para provavelmente < 100ms, a avaliar depois de
   confirmar a causa exacta).
2. **`typst-passo-891` (prioridade menor — visual)**: duas correcções independentes dentro do mesmo
   passo (ou dois passos, à discrição de quem planear):
   - `spacing.rs`: implementar "unless in script size" (causa confirmada com alta confiança).
   - `attach.rs`/`font_metrics.rs::math_kern`: confirmar por instrumentação se o kern zera, antes de
     decidir a correcção (causa candidata, não confirmada).

Sugiro tratar o tempo primeiro (afecta todo documento matemático, ganho maior) e o visual depois
(afecta a qualidade tipográfica, mas o documento já renderiza correctamente, só com espaçamento
subóptimo).
