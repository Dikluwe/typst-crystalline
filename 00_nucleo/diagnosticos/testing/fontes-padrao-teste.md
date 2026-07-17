# Fontes padrão para testes de comparação

**Data:** 2026-07-06  
**Estado:** Em vigor  
**Aplica-se a:** todos os testes de algoritmo que comparam directamente o cristalino com o vanilla (posições, larguras, quebras de linha, etc.).  
**ADR base:** `00_nucleo/adr/adr-paridade-defeitos-testes.md` — Paridade de definições por defeito nos testes de comparação.

---

## Fonte neutra para testes de algoritmo

```typst
#set text(font: "DejaVu Sans")
```

**Fonte escolhida:** `DejaVu Sans`

### Porquê esta fonte

1. **Disponível em ambos os ambientes.**
   - Confirmada no sistema operativo de teste (`fc-list : family`) com as variantes `DejaVu Sans`, `DejaVu Sans Condensed` e `DejaVu Sans Light`.
   - Confirmada no vanilla (`lab/typst-original/target/release/typst fonts`) com `DejaVu Sans` e variantes conhecidas.
   - O cristalino carrega fontes do sistema via `fontdb::load_system_fonts()`; `DejaVu Sans` é portanto resolvida no cristalino quando o mesmo sistema de fontes está presente.

2. **Sem problemas conhecidos nesta sequência.**
   - `FreeSerif` foi descartada devido a bug de decomposição de acentos observado em P558.
   - `DejaVu Sans` já foi usada noutros testes desta sequência sem registo de anomalias.

3. **Cobertura ampla.**
   - Inclui suporte para latim, alfabeto árabe básico e pontuação comum, o que cobre a maioria dos testes de algoritmo de layout sem forçar uma fonte específica por script.

### Alternativas consideradas

| Fonte | Disponibilidade | Motivo de não escolha |
|-------|-----------------|----------------------|
| `Liberation Sans` | Sistema + vanilla | Usada como fonte por defeito do cristalino (P558); escolhê-la para testes de algoritmo criaria confusão com testes de produção. |
| `Libertinus Serif` | Vanilla (default) | Não garantida no cristalino via sistema; além disso, é a fonte default do vanilla, logo inadequada para neutralizar comparações. |
| `Noto Sans Arabic` | Sistema + vanilla | Boa para árabe, mas a medição P589 mostrou que o cristalino ainda quebra linhas de forma diferente do vanilla para texto árabe devido a shaping; portanto não resolve sozinha testes RTL. |

## Outras definições sensíveis a neutralizar

Para testes de algoritmo, além da fonte, devem ser explícitas sempre que a comparação depender delas:

- `text(size: ...)` — tamanho de letra.
- `text(dir: ..., lang: ...)` — direcção e língua; o vanilla e o cristalino podem inferir valores diferentes para o mesmo texto.
- `page(margin: ..., width: ..., height: ...)` — geometria da página e margens.
- `par(justify: ..., leading: ...)` — quando relevante para a medição.

O helper `documento_algoritmo` garante apenas a fonte; o teste deve adicionar as restantes definições conforme necessário.

## Helper de teste

Em `01_core/src/engine/layout/tests.rs`:

```rust
/// Fonte neutra para testes de algoritmo.
pub(crate) const FONTE_NEUTRA_TESTE: &str = "DejaVu Sans";

/// Helper que envolve um documento de teste de algoritmo com a fonte neutra.
pub(crate) fn documento_algoritmo(conteudo: &str) -> String {
    format!("#set text(font: \"{}\")\n{}", FONTE_NEUTRA_TESTE, conteudo)
}
```

Uso:

```rust
let doc = layout_typst(&documento_algoritmo("Texto a comparar"));
```

## Regra de ouro

- **Testes de algoritmo:** usar sempre `documento_algoritmo` (ou equivalente) para garantir a mesma fonte nos dois lados.
- **Testes de produção:** usar valores por defeito reais de cada lado e registar no relatório quando uma diferença é atribuível a uma definição por defeito diferente.
