---
# P525 — MVP de Variation Fonts: coordenadas de eixo no shaper + embutir VF completa

> **Passo:** 525
> **Data:** 2026-07-01
> **Foco:** Implementar o MVP de suporte a Variation Fonts. **Sonda obrigatória sobre caching de `Face` antes de tocar `set_variations`** — é a diferença entre um fix correcto e um bug de contaminação entre pesos diferentes no mesmo documento, que é exactamente o cenário que este passo se propõe a testar. Três sub-tarefas de implementação seguem-se à sonda: (1) mapear `weight`/`stretch`/`style` para eixos OpenType; (2) passar coordenadas ao rustybuzz de forma segura face ao mecanismo de cache real; (3) validar com Ubuntu Sans, incluindo o caso Oblique que a versão anterior deste passo deixava por testar.
> **Tipo:** Sonda obrigatória + Implementação.
> **Tamanho:** M (~60 min — inclui a sonda de caching que a versão anterior não tinha).
> **ADR-0108 EM VIGOR** — medir antes de decidir; **não assumir que "criar Face + set_variations + shape" é seguro na pipeline real sem confirmar se há cache de Face por identidade de fonte.**
> **ADR-0109 EM VIGOR** — atomização.
> **ADR-0107 EM VIGOR** — `weight`/`stretch`/`style` são semântica de linguagem.
> **Dependências:** P524 (sonda VF: `shaper.rs:97` sem `set_variations`, `shaper.rs:162` com `FontVariant::default()`, oxifont-subset já preserva VF).
> **Trilha:** 7 — Variation Fonts (MVP).

---

## Contexto

P524 confirmou: o subsetter preserva `fvar`/`gvar`/`avar`/`HVAR` sem problemas; o bottleneck é o shaper, que cria o `Face` sem `set_variations` e ignora o `FontVariant` real. Isso corresponde à localização exacta (`shaper.rs:97` e `:162`).

O que P524 não verificou — e é pré-condição para este passo — é **como** o `Face` é gerido no resto da pipeline: criado de novo a cada chamada de shaping, ou cacheado por identidade de fonte para evitar reparsear os bytes a cada run de texto. Dado que o projecto tem optimização de performance como preocupação explícita (P507/P518), uma cache é plausível — e se existir, chaveada só pela fonte (não pela combinação fonte+variante), `set_variations` muda estado partilhado: um documento com pesos diferentes no mesmo texto (exactamente o que a Sub-tarefa 3 testa) pode fazer um peso "vazar" para outro.

---

## Sub-tarefa 0 — Sonda obrigatória: gestão do `Face` na pipeline real

### 0.1 — Localizar todos os sites de criação/uso de `Face`

```bash
grep -n "rustybuzz::Face\|Face::from_slice\|struct.*FaceCache\|HashMap.*Face\|cache" 03_infra/src/shaper.rs
```

**Perguntas a responder, com `file:line`:**

- `Face::from_slice` é chamado uma vez por run de texto (nova instância sempre), ou existe uma estrutura de cache (`HashMap<FontSlotIdx, Face>` ou equivalente) reutilizada entre chamadas de `shape_text`?
- Se há cache: a chave inclui a variante (`FontVariant`/coordenadas de eixo), ou só a identidade da fonte (path/slot index)?
- Se a chave não incluir variante: `set_variations` num `Face` cacheado por fonte-apenas contamina chamadas subsequentes com peso diferente, até a próxima vez que esse slot for recarregado.

### 0.2 — Decisão de arquitectura, condicional ao resultado

**Se não há cache (Face sempre recriado por chamada):** a implementação directa da Sub-tarefa 2 (criar `Face`, chamar `set_variations`, `shape`) é segura. Prosseguir sem alteração adicional.

**Se há cache chaveada só por fonte:** a chave da cache precisa de incluir a variante, ou `set_variations` tem de ser chamado a cada `shape_text` mesmo com `Face` cacheado (reaplicar a variante correcta antes de cada shape, nunca confiar no estado deixado pela chamada anterior). A segunda opção é mais barata (não invalida a cache de bytes/parse da fonte, só reaplica coordenadas) e deve ser preferida se a API do rustybuzz permitir chamar `set_variations` num `Face` já existente sem custo de reparse.

### Critério de fecho

- [ ] Mecanismo de gestão do `Face` confirmado com `file:line` (cache ou não; se cache, chave usada).
- [ ] Decisão de arquitectura tomada e documentada antes de qualquer edição da Sub-tarefa 2.

---

## Sub-tarefa 1 — Mapear `FontVariant` → coordenadas de eixo OpenType

(Sem alterações relevantes face à versão anterior — mapeamento `weight→wght`, `stretch→wdth`, `style→ital/slnt` mantém-se.)

```rust
impl FontVariant {
    pub fn to_axis_variations(&self) -> Vec<rustybuzz::Variation> {
        let mut vars = Vec::new();
        let wght_value = match self.weight {
            FontWeight::Thin => 100.0,
            FontWeight::ExtraLight => 200.0,
            FontWeight::Light => 300.0,
            FontWeight::Regular => 400.0,
            FontWeight::Medium => 500.0,
            FontWeight::SemiBold => 600.0,
            FontWeight::Bold => 700.0,
            FontWeight::ExtraBold => 800.0,
            FontWeight::Black => 900.0,
        };
        vars.push(rustybuzz::Variation { tag: rustybuzz::Tag::from_bytes(b"wght"), value: wght_value });

        let wdth_value = match self.stretch {
            FontStretch::UltraCondensed => 50.0,
            FontStretch::ExtraCondensed => 62.5,
            FontStretch::Condensed => 75.0,
            FontStretch::SemiCondensed => 87.5,
            FontStretch::Normal => 100.0,
            FontStretch::SemiExpanded => 112.5,
            FontStretch::Expanded => 125.0,
            FontStretch::ExtraExpanded => 150.0,
            FontStretch::UltraExpanded => 200.0,
        };
        if wdth_value != 100.0 {
            vars.push(rustybuzz::Variation { tag: rustybuzz::Tag::from_bytes(b"wdth"), value: wdth_value });
        }

        match self.style {
            FontStyle::Italic => {
                vars.push(rustybuzz::Variation { tag: rustybuzz::Tag::from_bytes(b"ital"), value: 1.0 });
            }
            FontStyle::Oblique(angle) => {
                let slant = -(angle.to_degrees() as f32);
                vars.push(rustybuzz::Variation { tag: rustybuzz::Tag::from_bytes(b"slnt"), value: slant });
            }
            FontStyle::Normal => {}
        }
        vars
    }
}
```

**Sonda adicional para este passo:** confirmar se `text(style:)` no cristalino sequer aceita um ângulo numérico para Oblique, ou só `"normal"`/`"italic"`:

```bash
grep -n "Oblique\|style.*angle\|FontStyle::" 01_core/src/entities/style.rs 01_core/src/engine/stdlib/text.rs
```

Se `Oblique(angle)` não é alcançável a partir da linguagem Typst neste momento (só `Normal`/`Italic` expostos), o ramo `slnt` no `to_axis_variations` é código morto — manter por completude não é problema, mas **não pode ser apresentado como testado** se não houver forma de o exercitar via `#set text(style:)`. Registar esse facto explicitamente no critério de fecho, não deixar implícito.

### Critério de fecho

- [ ] `to_axis_variations` implementado.
- [ ] Confirmado se `Oblique(angle)` é alcançável via linguagem Typst — se não for, ramo `slnt` marcado como não-testável neste passo, não como "testado".
- [ ] Testes unitários de mapeamento (weight→wght, stretch→wdth, style→ital) passam.

---

## Sub-tarefa 2 — Passar coordenadas ao rustybuzz, de acordo com a decisão da Sub-tarefa 0

### 2.1 — Implementação (ramificada pela Sub-tarefa 0.2)

```rust
// Se Sub-tarefa 0 confirmou "sem cache" ou "cache já reaplica variações a cada shape":
let mut face = rustybuzz::Face::from_slice(font_data, index)?;
let axis_vars = variant.to_axis_variations();
if !axis_vars.is_empty() {
    face.set_variations(&axis_vars);
}
```

**Se a Sub-tarefa 0 revelou cache chaveada só por fonte:** a chamada a `set_variations` tem de acontecer em **todas** as invocações de shaping que usam essa entrada de cache, não só na primeira, e antes de cada `shape()` — nunca assumir que o estado de variação de uma chamada anterior é válido para a actual.

### 2.2 — `resolve_candidates` com `FontVariant` real

```rust
// Antes: let variant = FontVariant::default();
let variant = text_style.variant(); // ou equivalente — confirmar o nome real do campo/método
```

### 2.3 — Teste de integração que passa pela pipeline real, não por chamada isolada ao rustybuzz

O teste da versão anterior deste passo (Sub-tarefa 2.5) criava um `rustybuzz::Face` directamente, isolado da pipeline do `shaper.rs`. Isso valida a API do rustybuzz mas **não** valida se o mecanismo de cache do cristalino (confirmado ou não na Sub-tarefa 0) preserva a variação correcta ao longo de várias chamadas com pesos diferentes — que é o risco real identificado.

```rust
#[test]
fn shaper_pipeline_mixed_weights_nao_contamina() {
    // Chamar a função pública real do shaper.rs (não rustybuzz directamente),
    // com uma sequência que alterna pesos, replicando o cenário da Sub-tarefa 3:
    // 700 → 100 → 700 novamente, confirmando que o terceiro shape produz
    // o MESMO resultado que o primeiro (não herda o peso 100 intermédio).
    let font_data = load_fixture_vf();

    let glyphs_bold_1 = shape_via_pipeline(&font_data, "Hello", FontWeight::Bold);
    let glyphs_thin    = shape_via_pipeline(&font_data, "Hello", FontWeight::Thin);
    let glyphs_bold_2 = shape_via_pipeline(&font_data, "Hello", FontWeight::Bold);

    let width_bold_1 = glyphs_bold_1.iter().map(|g| g.x_advance).sum::<i32>();
    let width_thin   = glyphs_thin.iter().map(|g| g.x_advance).sum::<i32>();
    let width_bold_2 = glyphs_bold_2.iter().map(|g| g.x_advance).sum::<i32>();

    assert!(width_bold_1 > width_thin, "bold deve ser mais largo que thin");
    assert_eq!(width_bold_1, width_bold_2,
        "duas chamadas com o mesmo peso, intercaladas por um peso diferente, \
         devem produzir o mesmo resultado — falha aqui indica contaminação de estado \
         num Face cacheado sem reaplicar variações");
}
```

Este teste, não o da versão anterior, é o que realmente cobre o risco identificado na Sub-tarefa 0.

### 2.4 — Fonte de teste em fixture, não caminho de sistema

Seguir o padrão já estabelecido em P523: copiar Ubuntu Sans (ou outra VF confirmada por P524) para `03_infra/fixtures/fonts/`, com licença anotada, e usar `env!("CARGO_MANIFEST_DIR")` no teste — não `/usr/share/fonts/...` hardcoded. Skip gracioso se a fixture faltar, não `.unwrap()` num caminho de sistema.

### Critério de fecho

- [ ] Implementação de `set_variations` segue a decisão da Sub-tarefa 0 (não a versão "ingénua" que assume ausência de cache sem verificar).
- [ ] `resolve_candidates` usa `FontVariant` real.
- [ ] Teste `shaper_pipeline_mixed_weights_nao_contamina` passa, usando a pipeline real do `shaper.rs`, não uma chamada isolada ao rustybuzz.
- [ ] Fixture de fonte VF no repositório, teste com skip gracioso.

---

## Sub-tarefa 3 — Validação empírica com Ubuntu Sans, incluindo Oblique se alcançável

### 3.1 — Documento de teste

```typst
#set text(font: "Ubuntu Sans", size: 40pt)
Hello world.

#set text(weight: 700)
Bold hello.

#set text(weight: 100)
Thin hello.

#set text(weight: 400, stretch: 75%)
Condensed hello.

#set text(weight: 400, style: "italic")
Italic hello.
```

**Se a Sub-tarefa 1 confirmar que `Oblique(angle)` é alcançável via linguagem** (não apenas `"italic"` binário), adicionar uma secção ao documento de teste que a exercite. **Se não for alcançável**, registar explicitamente no relatório que o ramo `slnt` do mapeamento não tem validação empírica neste passo — não deixar essa lacuna implícita.

### 3.2–3.6 — (mantidos da versão anterior: compilação, critérios de validação, comparação vanilla, inspecção de content stream)

Critérios de validação, comparação com vanilla, e inspecção do `TJ` seguem exactamente como na versão anterior deste passo — essas partes estavam correctas.

### Critério de fecho adicional

- [ ] Estado do ramo `slnt`/Oblique explicitamente registado: testado, ou confirmado não-alcançável pela linguagem neste momento.

---

## Sub-tarefa 4 — Bateria de paridade e sanity check

(Sem alterações — corpus P490–P520, corpus CFF de P523, benchmark P518, linter.)

---

## Sub-tarefa 5 — Documentação e fecho de Trilha 7 (MVP)

(Sem alterações relevantes — actualizar handoff, documentar limitações do MVP, incluindo agora explicitamente a decisão tomada na Sub-tarefa 0 sobre gestão de `Face`/cache, para que uma futura optimização de performance não reintroduza a contaminação por engano.)

---

## Critério de fecho do passo

- [ ] Sub-tarefa 0: mecanismo de gestão do `Face` confirmado antes de qualquer edição.
- [ ] Sub-tarefa 1: mapeamento implementado; estado do ramo Oblique/slnt registado com honestidade.
- [ ] Sub-tarefa 2: implementação segue a decisão de arquitectura da Sub-tarefa 0; teste de pipeline real (não chamada isolada ao rustybuzz) confirma ausência de contaminação entre pesos.
- [ ] Sub-tarefa 3: validação empírica; Oblique testado se alcançável, ou lacuna registada se não.
- [ ] Sub-tarefa 4: sem regressão em corpus/benchmark/linter.
- [ ] Sub-tarefa 5: documentação actualizada, incluindo a decisão sobre `Face`/cache.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p525.md`, com a Sub-tarefa 0 documentada separadamente das restantes (é a que decide se o resto do passo está a resolver o problema certo).

---

## Próximo passo

- **Se Sub-tarefa 0 revelar contaminação real em produção actual** (mesmo antes deste fix, se já houver algum uso de variantes por outro caminho): tratar como bug prioritário, não só como risco teórico evitado.
- Caso contrário, seguir para as opções do handoff (Lookahead, publicação, optimização, subsetting VF optimizado).
