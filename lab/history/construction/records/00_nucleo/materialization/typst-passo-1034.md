# Passo 1034 — Três defaults da linguagem ausentes: `bibliography.style`, `figure.numbering`, língua por defeito

**Tipo**: Investigar → gate (ADR-0127, categoria 2/3) → corrigir. Três achados do P1031
agrupados por serem, estruturalmente, a mesma classe de bug: **um valor por defeito da
linguagem nunca foi ligado**, produzindo saída errada sem erro sempre que o utilizador
não define o parâmetro explicitamente — que é o caso mais comum.
**Pré-condição**: `git status` limpo. HEAD ≥ Passo 1033.

---

## Achado 2 — `bibliography.style` sem default `"ieee"`

**Medição do P1031** (`A @netwok B @netwok C @other D @netwok` +
`#bibliography("works.bib")`, sem `style`):
- Vanilla: `A [1] B [1] C [2] D [1]` + `[1] J. Doe, At what cost. Fake Press, 2020.`
- Cristalino: `A [1] B ibid. C [2] D [1] Doe, op. cit.` + formato CSL diferente com
  retrolinks.
- **Com `style: "ieee"` explícito, os dois são byte-idênticos.**

O caminho CSL real já funciona — o defeito é só a ausência do default. Confirmar em
`compiler/layout/bibliography.rs` (ou onde o campo `style` é lido) que, sem valor
explícito, cai num fallback local em vez de assumir `"ieee"`.

## Achado 5 — `figure.numbering` sem default

**Medição do P1031**: vanilla `Figure 1: Uma coisa`; cristalino `Uma coisa` (sem número).
Vanilla usa `#[default(Some(NumberingPattern::from_str("1")))]`. Confirmar o campo
equivalente em `FigureElem` cristalino e por que fica `None` por defeito.

## Achado 6 — língua por defeito não é `en`

**Medição do P1031**: vanilla produz `Figure`/`Contents`; cristalino produz
`Figura`/`Índice`. Com `#set text(lang: "en")` explícito, coincidem. Vanilla usa
`#[default(Lang::ENGLISH)]`. Confirmar o default actual do cristalino (parece estar a
usar português como fallback, o que é um dado interessante em si — confirmar se é
coincidência de ambiente de build ou algo codificado).

---

## Fase A — Confirmar os três defaults por leitura, antes de mexer

Para cada um: localizar o campo, confirmar o valor por defeito actual (`None`/outro
idioma/fallback local) e o ponto exacto onde devia estar `Some("ieee")`/
`Some(NumberingPattern::from_str("1"))`/`Lang::ENGLISH` (ou equivalente cristalino).

## Fase B — Gate (ADR-0127, categoria 2/3)

Mudança de output visual/textual por defeito em qualquer documento que não defina
`style`/`numbering`/`lang` explicitamente — o caso mais comum, logo alto impacto.

```
Dado #bibliography("x.bib") sem style
Quando renderizado
Então usa "ieee" por defeito, batendo byte-a-byte com vanilla no caso já confirmado

Dado #figure(...) sem numbering
Quando renderizado
Então numera com padrão "1" por defeito

Dado documento sem #set text(lang:)
Quando qualquer texto gerado pela linguagem aparece (Figure/Contents/etc.)
Então usa inglês por defeito
```

Não-regressão: todos os testes existentes que já definem `style`/`numbering`/`lang`
explicitamente — devem continuar inalterados (só o caso sem valor explícito muda).

## Fase C — Implementar e validar

```
crystalline-lint .
cargo test --workspace
```
Decalque contra corpus onde aplicável — mudança de output em qualquer documento com
bibliografia, figura, ou texto que dependa de idioma sem overrides explícitos.

---

## Resultado esperado

Os três defaults aplicados, batendo com vanilla nos casos base (sem override). Documentos
existentes que já definem os três parâmetros explicitamente ficam inalterados.
