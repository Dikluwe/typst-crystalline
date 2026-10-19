# Passo 977 — variantes de script `ssty` (`.st`/`.sts`) não aplicadas em matemática

**Precede este passo**: achado de P975 (Fase A da pendência P952 §6.4) —
o vanilla aplica a feature OpenType `ssty` ao texto math em tamanho de
script (`MathSize::Script → feat("ssty", 1)`, `ScriptScript →
feat("ssty", 2)` — `lab/typst-original/crates/typst-library/src/text/
mod.rs:1457-1460`). Em NewCMMath-Book a feature é AlternateSubst
(GSUB type 3): `u1D45B → [u1D45B.st, u1D45B.sts]` (1094 glifos cobertos;
índice 0 = Script `.st`, índice 1 = ScriptScript `.sts`). O cristalino
escala o glifo base geometricamente — glifos de script mais estreitos e
com traço mais fino que o vanilla. **Medição exacta** (`$ K_n $`, sub n a
7.7pt): cristalino 4.62pt = advance do glifo base (600du); vanilla
5.44pt = advance de `u1D45B.st` (706du). É o driver dominante da
pendência de medianas horizontais de P952 (acumula por glifo de script
da esquerda para a direita — o padrão de gradiente medido nas secções
4/25/28).

**Pré-condição de árvore**: `git status`. Confirmar P975/P976 presentes.

---

## Fase A — mapear os pontos de injecção (sem escrever código de produção)

1. Confirmar onde o advance de um glifo de script é lido no layout
   (`FallbackFontMetrics::advance` e irmãos `text_ink_bounds*`): a
   substituição ssty tem de acontecer **na métrica** (o layout posiciona
   com o advance do `.st`) **e no render** (o glifo desenhado tem de ser
   o `.st`) — se só um lado mudar, layout e render divergem internamente
   (lição de P772o: medir e desenhar têm de usar a mesma instância).
2. Confirmar o leitor de GSUB disponível: ttf-parser 0.25 expõe
   `gsub::AlternateSubstitution` (`ttf-parser-0.25.1/src/tables/gsub.rs`)
   — verificar que chega para resolver `feature ssty → lookup →
   AlternateSet[gid base] → alternate[level−1]` com o upem correcto.
3. Confirmar o caminho do render: os itens math são `FrameItem::Text`
   (não passam pelo shaper) e o export resolve char→gid por mapa
   por-fonte (`builder.rs`, `per_font_char_to_gid`) — o mapa é
   style-cego. Desenhar como o gid `.st` entra no subset (widths,
   ToUnicode, `glyph_to_nominal`) — provavelmente: na construção do
   mapa, para chars usados com `style.math && math_size ∈ {Script,
   ScriptScript}`, registar o gid substituído; na emissão, seleccionar
   pelo `math_size` do item. Medir o blast radius antes de implementar
   (quantos sítios em `builder.rs`/`stream.rs` tocam gid de texto).
4. Confirmar se `text_ink_bounds` (ascent/descent das caixas math) deve
   usar a bbox do `.st` (provavelmente sim — o vanilla mede o fragmento
   do glifo substituído) — verificar com um caso medido (altura de
   subscrito de `f` — o `.st` pode ter métricas de tinta diferentes).
5. Verificar a interacção com P975 (IC no advance): o glifo `.st` tem a
   sua própria entrada de IC na tabela MATH? (medir `u1D45B.st` na
   coverage de `MathItalicsCorrectionInfo`) — a ordem dos dois termos
   importa para o total.

## Fase B — Implementação

Protocolo de dois agentes (pipeline de fontes, afecta todos os glifos de
script do documento — métricas + subsetting + ToUnicode). TDD com testes
de integração de fonte real (lição P972: `FixedMetrics` não vê glifos
`.st`). Gate ADR-0127: as assinaturas do trait `FontMetrics` não mudam
(`style` já carrega `math_size`, P945) — se o desenho da Fase A exigir
mudança de assinatura, **parar e confirmar** antes de implementar.

1. Testes: subscrito de `K_n` mede 5.44pt (advance do `.st`) a 11pt;
   scriptscript usa `.sts`; texto de corpo inalterado; prosa inalterada.
2. Implementar métrica + render + subsetting.
3. Suíte completa verde, discriminada por crate.

## Fase C — Revalidação

1. Medir os casos da Fase A de P975 (`$ K_n $`, `$ sum_(k=1)^n k $`) —
   larguras de script iguais ao vanilla.
2. `compare.py` nas secções 4/25/28 — as medianas devem cair para o nível
   do ruído de emparelhamento (a pendência de P952 §6.4 fecha aqui).
3. Benchmark completo, 7 cenários, `depois/antes`, zero regressão
   (atenção ao custo da leitura GSUB — cachear a resolução ssty por
   (face, gid, nível)).

## Resultado esperado

- Glifos de script math renderizados como variantes `.st`/`.sts`, com
  advances e tintas correspondentes — paridade tipográfica com o vanilla.
- Pendência P952 §6.4 fechada por correcção (não só por prova).
- Benchmark sem regressão.
