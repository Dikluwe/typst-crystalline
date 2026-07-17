---
# P673 — A `FaceCache` de `shaped_width` persiste entre chamadas, ou é recriada a cada vez?

> **Passo:** 673
> **Data:** 2026-07-10
> **Foco:** P672 diz ter criado "uma `FaceCache` local dentro de `shaped_width`", sem esclarecer se essa cache vive durante o documento inteiro (como a de `shape_document`) ou é recriada a cada chamada da função — o que a tornaria inútil, sem nenhum ganho, apesar de aparentar estar corrigida. Dado o tamanho do ganho já confirmado no caminho principal, vale a pena garantir que o caminho de layout (`shaped_width`, usado na decisão de quebra de linha para árabe/devanágari) recebeu o mesmo benefício de facto, não só na aparência.
> **Tipo:** Verificação directa. Correcção se confirmado o problema.
> **Tamanho:** S.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P672 (onde a ambiguidade apareceu), P591/P622/P623 (onde `shaped_width` é usada para árabe/devanágari).

---

## Verificação

### Confirmar o âmbito exacto da `FaceCache` em `shaped_width`

```bash
grep -n "fn shaped_width\|FaceCache" 01_core/src/rules/layout/metrics.rs 03_infra/src/font_metrics.rs 03_infra/src/shaper.rs | head -20
```

Confirmar: a `FaceCache` é um parâmetro local dentro do corpo de `shaped_width` (recriada a cada chamada), ou é passada de fora, persistindo ao longo do documento (por exemplo, como campo do `Layouter`, semelhante a como P657/P659 já guardam outras caches)?

### Medir directamente com um documento que force muitas chamadas a `shaped_width`

```bash
python3 -c "
for i in range(2000):
    print('الكتاب على الطاولة يوم جميل')
" > /tmp/p673-arabe-repetido.typ
sed -i '1i #set text(lang: \"ar\", dir: rtl, font: \"DejaVu Sans\", size: 20pt)' /tmp/p673-arabe-repetido.typ
./target/release/typst /tmp/p673-arabe-repetido.typ /tmp/p673.pdf --timings-json /tmp/p673-timings.json
cat /tmp/p673-timings.json
```

Confirmar o `layout_ms` — se a cache não estiver a persistir, este número deve ser proporcional ao número de chamadas a `shaped_width`, com o mesmo custo repetido de `Face::parse` já identificado por P672 para o caminho de `shape_document`.

### Critério de fecho da verificação

- [ ] Âmbito exacto da `FaceCache` de `shaped_width` confirmado — persiste ou é recriada.
- [ ] Medido `layout_ms` com um documento que force muitas chamadas, para confirmar na prática, não só pela leitura do código.

---

## Decisão

Se a cache não persistir: corrigir para ter o mesmo âmbito de vida da cache de `shape_document` — por exemplo, um campo no `Layouter` que persiste ao longo de todo o documento, passado a `shaped_width` em vez de criado dentro dela.

Se já persistir correctamente: confirmar com os números medidos, não deixar como suposição.

---

## Implementação, se necessário

```rust
// Esboço, a confirmar contra a estrutura real:
pub(super) struct Layouter {
    // ...
    face_cache: FaceCache,  // persiste ao longo do documento
}
```

`shaped_width` passa a receber a `FaceCache` do `Layouter`, em vez de criar a sua própria.

---

## Validação

```bash
./target/release/typst /tmp/p673-arabe-repetido.typ /tmp/p673-depois.pdf --timings-json /tmp/p673-depois-timings.json
cat /tmp/p673-depois-timings.json
```

Comparar `layout_ms` antes e depois, se alguma correcção for feita.

```bash
cargo test --workspace
crystalline-lint .
```

Repetir os testes de RTL/devanágari (P590-592, P622-623) sem regressão.

---

## Critério de fecho do passo

- [ ] Âmbito da `FaceCache` de `shaped_width` confirmado com números, não só leitura de código.
- [ ] Se não persistir: corrigida para persistir ao longo do documento.
- [ ] `layout_ms` medido antes e depois, se houver correcção.
- [ ] Testes de RTL/devanágari sem regressão.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p673.md`, com hash do commit.
