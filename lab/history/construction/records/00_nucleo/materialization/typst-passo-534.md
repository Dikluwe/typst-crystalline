---
# P534 — Fallback de fonte por carácter (multi-script)

> **Passo:** 534
> **Data:** 2026-07-02
> **Foco:** P531 Grupo 3.3 confirmou que um documento com latim, CJK, emoji e árabe na mesma linha perde os caracteres não-latinos — o cristalino usa sempre uma única fonte (Helvetica), em vez de trocar de fonte por carácter como o vanilla faz. Este passo corrige a selecção de fonte por script/cobertura de glifo. A parte de fontes de cor para emoji (COLR/CPAL) fica fora deste passo — é maior e tem uma causa diferente (renderização de glifo a cores, não só escolha de fonte).
> **Tipo:** Sonda + Implementação.
> **Tamanho:** L.
> **ADR-0108 EM VIGOR.** **ADR-0114 EM VIGOR** — sonda antes de qualquer edição.
> **Dependências:** P515 (fontdb, onde a alegação original de "fallback por carácter" foi escrita — precisa de correcção), P531 (Grupo 3.3, onde o problema foi confirmado), P484 (bidi_runs — mecanismo parecido de dividir texto em segmentos, reutilizável aqui como referência de padrão).

---

## Contexto e correcção de uma afirmação

O handoff descreve P515 como tendo fechado "fontdb + font fallback: font fallback por carácter". P531 mostra que isto não está correcto: um documento com texto misto perde os caracteres não cobertos pela fonte escolhida. A primeira coisa a fazer neste passo é confirmar o que P515 implementou de facto, para perceber se é preciso construir do zero ou só completar algo que ficou parcial.

Isto é o mesmo tipo de situação já registada duas vezes antes neste projecto: uma linha escrita como fechada que, sondada de novo, não está.

---

## Sonda 1 — O que existe hoje

```bash
grep -rn "fallback\|resolve_font\|select_pattern\|has_char\|has_glyph" 03_infra/src/pipeline.rs 03_infra/src/fonts.rs --include="*.rs"
```

Perguntas, com `file:line`:

1. Existe alguma função que verifica se uma fonte cobre um carácter específico (`has_char`, `has_glyph`, ou equivalente da crate `fontdb`)?
2. Se existe, é chamada em algum ponto da pipeline, ou fica sem uso?
3. Quando um documento pede uma fonte e essa fonte não cobre um carácter, o que acontece hoje: mantém a fonte pedida e o carácter falha (glifo em falta), troca para uma fonte default fixa (Helvetica) para o documento inteiro, ou já tenta trocar por segmento (mesmo que mal)?
4. `resolve_font` recebe o texto completo do documento, ou já trabalha por segmentos menores (parágrafo, palavra, run de shaping)?

### Critério de fecho da sonda 1

- [ ] Confirmado o que P515 implementou de facto — não assumir pelo nome do passo.
- [ ] Localizado o ponto exacto onde a fonte é escolhida hoje, e se há verificação de cobertura de carácter.

---

## Sonda 2 — Como o vanilla faz isto (referência)

```bash
grep -rn "fallback\|fontique\|script.*font\|coverage" lab/typst-original/crates/typst-layout/src/ --include="*.rs" | head -20
```

O objectivo não é copiar o vanilla, mas confirmar o mecanismo geral: o vanilla segmenta o texto por script (semelhante ao que P484 já fez para bidi — dividir por trecho de direcção), e para cada segmento escolhe a primeira fonte da lista que cobre todos os caracteres desse segmento.

### Critério de fecho da sonda 2

- [ ] Confirmado o mecanismo geral do vanilla (segmentação + escolha por cobertura), para servir de referência de desenho.

---

## Sonda 3 — Impacto na divisão de `FrameItem::Text`/`BT...ET`

Ligado à nota deixada no fecho de P533: se este passo divide texto por fonte, cada segmento pode virar um `FrameItem::Text` separado, e cada um gera o seu próprio bloco `BT...ET` no PDF.

```bash
grep -n "BT\|ET" 03_infra/src/export/stream.rs | head -20
```

Confirmar se blocos `BT...ET` consecutivos do mesmo `Tf` (mesma fonte) podem ser fundidos num só, ou se cada `FrameItem` gera sempre o seu próprio bloco independentemente da fonte ser igual à do item anterior. Se puder ser fundido, isso reduz o impacto do problema descrito em P533 — não é preciso resolver agora, mas vale confirmar se a divisão por fonte deste passo vai piorar um problema já conhecido ou não.

### Critério de fecho da sonda 3

- [ ] Confirmado se blocos `BT...ET` consecutivos da mesma fonte podem ser fundidos.
- [ ] Decisão registada: fundir agora (se for simples) ou deixar para depois, com nota explícita de que o problema fica maior com este passo.

---

## Desenho da solução (MVP — sem cor)

Este passo cobre: escolher a fonte estática/VF correcta por segmento de texto, com base em script e cobertura de carácter. Não cobre: renderizar glifos a cores (emoji COLR/CPAL) — isso fica registado como item separado, porque é um mecanismo de renderização diferente (não é "escolher a fonte certa", é "desenhar um glifo com múltiplas camadas de cor"), maior, e sem relação directa com o resto deste passo.

### Passos de implementação, uma vez confirmadas as sondas

1. Segmentar o texto de cada `FrameItem::Text` de entrada por: mudança de script (latim → CJK → árabe, etc., usando a mesma lógica de classificação Unicode já usada em `bidi_runs` de P484, ou uma equivalente para script em vez de direcção) e por cobertura de carácter na fonte pedida.
2. Para cada segmento, se a fonte pedida pelo documento não cobrir todos os caracteres do segmento, procurar na lista de fontes disponíveis (fontdb) a primeira que cobre.
3. Se nenhuma cobrir, manter o comportamento actual (glifo em falta ou `.notdef`) — não inventar um novo fallback silencioso sem aviso.
4. Cada segmento resultante mantém a lógica já existente de shaping/subsetting/export, só muda a fonte usada nesse segmento.

### Critério de fecho do desenho

- [ ] Mecanismo de segmentação por script definido, reaproveitando o padrão de `bidi_runs` onde fizer sentido.
- [ ] Mecanismo de escolha de fonte por cobertura definido.
- [ ] Caso "nenhuma fonte cobre" tratado sem regressão silenciosa.

---

## Validação

Documento de teste (o mesmo de P531, para comparação directa):

```typst
#set text(size: 20pt)
Hello 你好 مرحبا
```

(Sem emoji neste teste — cor fica fora do MVP. Emoji entram num teste separado, só para confirmar que o comportamento actual — sem cor, mas com o carácter certo se a fonte tiver o glifo em preto-e-branco — não piora.)

```bash
./target/release/typst /tmp/test-mixed.typ /tmp/mixed-pos-fix.pdf
pdftotext /tmp/mixed-pos-fix.pdf -
pdffonts /tmp/mixed-pos-fix.pdf
```

Esperado: texto latino, CJK, e árabe todos extraíveis correctamente, com fontes diferentes listadas em `pdffonts` para os segmentos que precisaram.

Comparar com vanilla 0.15.0 para o mesmo documento.

```bash
cargo test --workspace
crystalline-lint .
python3 tools/perf/benchmark-p507.py
```

O benchmark aqui importa mais que no costume — este passo adiciona verificação de cobertura por carácter, que pode ter custo. Confirmar que documentos sem texto misto (a maioria) não ficam mais lentos.

---

## Critério de fecho do passo

- [ ] Sondas 1–3 completas antes de qualquer código.
- [ ] Afirmação de P515 corrigida no handoff, reflectindo o que estava de facto implementado antes deste passo.
- [ ] Segmentação por script e escolha por cobertura implementadas.
- [ ] Documento de teste latim+CJK+árabe funciona sem perda de caracteres.
- [ ] Fontes de cor (emoji) explicitamente fora deste passo, registadas como item separado, não escondidas.
- [ ] Sem regressão de performance em documentos sem texto misto.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p534.md`.

---

## Próximo passo

P535 — Bookmarks PDF (`/Outlines`). Mais pequeno que este, isolado à exportação, não toca no shaper.
