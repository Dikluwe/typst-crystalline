# Prompt L0 — `infra/fallback_fonts` — catálogo de fallback tipográfico

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/fallback-selection.toml sha256:faf6c20021b467fdb2a864625f4f6dd4386b5ef28c137b946e5cb4e4ce073d52

Hash do Código: 055c3453

**Camada**: L3
**Ficheiro alvo**: `03_infra/src/fallback_fonts.rs`
**Criado em**: 2026-08-26 (P1197; individualização de P555/P783 de `shaper.md`)
**ADRs**: ADR-0107, ADR-0108, ADR-0129

---

## Medição antes da decisão

`fallback_fonts.rs` contém três listas ordenadas de famílias e duas funções de
consulta. Não abre fontes, não consulta cobertura, não executa scoring e não
shapeia texto. `shaper.rs` e `font_metrics.rs` consomem o catálogo, mas são
donos distintos dos algoritmos de seleção e medição.

O antigo `infra/shaper.md` legitimava simultaneamente o catálogo e o shaper.
P1197 individualiza o owner sem mudar constantes, funções ou testes Rust.

## Responsabilidade

Este módulo é o catálogo L3 das cadeias explícitas de fallback:

- cadeia serif;
- cadeia sans e padrão;
- cadeia matemática alinhada a `math::families()` do vanilla ratificado.

O módulo também classifica um nome de família como serif ou não-serif para
escolher uma das duas cadeias de prosa. Ele não decide se um run está em modo
matemático e não escolhe o vencedor por cobertura.

## Cadeia serif

`DEFAULT_FALLBACK_FONTS_SERIF`, em ordem:

1. `Liberation Serif`;
2. `DejaVu Serif`;
3. `Bitstream Vera Serif`;
4. `FreeSerif`.

P558 mantém `FreeSerif` por último porque sua decomposição de acentos não é
reconstruída corretamente pelo subsetter CFF cristalino; ela continua
disponível para cobertura residual.

## Cadeia sans e padrão

`DEFAULT_FALLBACK_FONTS_SANS`, em ordem:

1. `DejaVu Sans`;
2. `Noto Sans`;
3. `Liberation Sans`;
4. `FreeSans`;
5. `Arial`.

`fallback_font_list_for(name)` converte o nome para lowercase. Se contiver
`serif`, devolve a cadeia serif; qualquer outro caso devolve a cadeia sans.
Isso inclui nomes com `sans`, nomes desconhecidos e a string vazia.

## Cadeia matemática

`DEFAULT_FALLBACK_FONTS_MATH`, em ordem:

1. `New Computer Modern Math`;
2. `Libertinus Serif`;
3. `Twitter Color Emoji`;
4. `Noto Color Emoji`;
5. `Apple Color Emoji`;
6. `Segoe UI Emoji`.

O primeiro nome usa espaços porque a tabela de exceções P840 registra a
família documentada no `FontBook`, espelhando o nome usado por
`math::families()` no vanilla. `math_fallback_font_list()` devolve essa lista
sem reordenar ou filtrar.

## Limites de ownership

- O shaper decide quando anexar as cadeias, segmenta runs e escolhe faces por
  cobertura/scoring.
- `FallbackFontMetrics` decide como medir com a mesma seleção.
- O `FontBook` possui coverage e scoring de similaridade.
- Este catálogo não infere semântica matemática a partir de nomes.

## Critérios de verificação

- `FreeSerif` e `DejaVu Serif` selecionam a cadeia serif.
- `DejaVu Sans` e nome desconhecido selecionam a cadeia sans.
- A cadeia matemática não é vazia, começa por `New Computer Modern Math` e
  contém `Libertinus Serif` na segunda posição.
- As listas e funções permanecem idênticas ao código anterior ao P1197.
- A alteração de linhagem não modifica corpos Rust.

## Histórico de revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-07-03 | P555 — cadeias de prosa por classe serif/sans | `fallback_fonts.rs` |
| 2026-07-18 | P783/P784/P840 — cadeia matemática e nome canônico da família NewCM | `fallback_fonts.rs` |
| 2026-08-26 | P1197 — owner 1:1 extraído de `shaper.md`; seleção compartilhada movida para Núcleo Tekt | `fallback_fonts.md`, `fallback_fonts.rs` |
