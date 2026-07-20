# Prompt L0 — `infra/embedded_fonts` — Fontes Embutidas via `typst-assets`
Hash do Código: 5af91e27

**Camada**: L3  
**Criado em**: 2026-07-14 (Passo 753)  
**Arquivos gerados**: `03_infra/src/embedded_fonts.rs` (novo), alterações em `03_infra/src/world.rs`, `03_infra/Cargo.toml`, `04_wiring/src/main.rs`  
**ADRs referência**: ADR-0019 (`ttf-parser` → L3 exclusivo), ADR-0022 (`FontInfo` — L1 recebe apenas campos primitivos), ADR-0107 (paridade com a linguagem), ADR-0108 (medir antes de decidir)

---

## Contexto

O vanilla 0.15.0 embute um conjunto de fontes redistribuíveis via a crate
`typst-assets`. O CLI do vanilla usa estas fontes como base, o que garante que
`Libertinus Serif` (fonte por defeito de `text`) e as fontes de math/code estejam
sempre disponíveis independentemente do sistema anfitrião.

Até **P753**, o cristalino dependia exclusivamente de fontes do sistema
(`fontdb`) e de paths de projecto (`--font-path`). Como `Libertinus Serif` não é
instalada por defeito na maioria das distribuições Linux, o cristalino usava
`Liberation Serif` como fonte por defeito. Isto introduzia uma divergência visual
e um resíduo de paginação em relação ao vanilla.

## Objetivo

Tornar o conjunto de fontes embutidas do cristalino idêntico ao do vanilla CLI,
garantindo que a fonte por defeito `Libertinus Serif` esteja sempre disponível,
sem que fontes especializadas de math/code (que podem conter glifos de cobertura
parcial ou alternativos para scripts como devanágari) sejam escolhidas como
fallback para texto normal.

## Restrições Estruturais

- `typst-assets` é uma dependência de L3 — fornece bytes de fonte em tempo de
  compilação/build.
- L1 continua a receber apenas `FontBook`/`FontInfo` (metadados primitivos) e
  `Font(Vec<u8>)` opaco.
- O carregamento de fontes embutidas deve ser **aditivo**: fontes do sistema e
  fontes de projecto (`--font-path`) continuam a funcionar exactamente como hoje.
- As fontes embutidas são divididas em dois grupos:
  - **Fontes de texto**: `Libertinus Serif*` e `NewCM10*`. Colocadas no início do
    `FontBook`, antes das fontes do sistema, para servirem de fallback primário.
  - **Fontes de math/code**: `NewCMMath*` e `DejaVu Sans Mono*`. Colocadas no
    **fim** do `FontBook`, depois das fontes do sistema, para não competirem no
    fallback carácter-a-carácter de texto normal. Continuam disponíveis para
    uso explícito (ex.: `#set text(font: "DejaVu Sans Mono")`) e para math.
- A ordem final no `FontBook` é: **texto embutido → sistema → math/code embutido
  → projecto**. Projectos podem sobrepor-se adicionando fontes no fim.
- Não alterar a trait `World` nem a assinatura dos métodos `book()`/`font()`.

## Instrução

1. Adicionar `typst-assets` às dependências de `03_infra/Cargo.toml`, com a
   feature `fonts`, usando a mesma revisão git que o vanilla 0.15.0
   (`https://github.com/typst/typst-assets?rev=c0ae970`).

2. Criar `03_infra/src/embedded_fonts.rs` com função pública:
   ```rust
   pub fn load_embedded_fonts() -> EmbeddedFontSets
   ```
   onde `EmbeddedFontSets` contém quatro vectores separados:
   - `text_slots` / `text_book`: fontes de texto (`Libertinus Serif*`, `NewCM10*`).
   - `math_code_slots` / `math_code_book`: fontes de math/code (`NewCMMath*`,
     `DejaVu Sans Mono*`).
   - Itera sobre `typst_assets::fonts()`.
   - Para cada blob de bytes:
     - Extrai `FontInfo` via `font_info_from_bytes`.
     - Classifica a fonte pelo nome da família:
       - Se começar com `Libertinus Serif` ou `NewCM10` → grupo texto.
       - Se começar com `NewCMMath` ou `DejaVu Sans Mono` → grupo math/code.
     - Cria um `FontSlot` cujo carregamento lazy devolve `Some(Font::from_data(bytes))`.
   - Retorna os dois conjuntos separados.

3. Expor em `03_infra/src/world.rs` um novo builder em `SystemWorld`:
   ```rust
   impl SystemWorld {
       /// Constrói um SystemWorld incluindo as fontes embutidas do vanilla.
       pub fn with_embedded_fonts(mut self) -> Self { ... }
   }
   ```
   - Chama `crate::embedded_fonts::load_embedded_fonts()`.
   - Associa os slots resultantes ao mundo da mesma forma que `with_fonts`.

4. Alterar `with_fonts_and_system` para combinar texto embutido + sistema +
   math/code embutido + projecto:
   ```rust
   pub fn with_fonts_and_system(mut self, font_paths: &[PathBuf]) -> Self
   ```
   - Carrega o grupo texto das embutidas primeiro.
   - Depois carrega sistema via `fontdb::load_system_fonts`.
   - Depois adiciona o grupo math/code das embutidas.
   - Finalmente adiciona fontes de projecto via `discover_fonts`.
   - Preserva a ordem: texto embutido → sistema → math/code embutido → projecto.

5. No CLI (`04_wiring/src/main.rs`), manter a chamada a
   `SystemWorld::new(...).with_fonts_and_system(&font_paths)`. A alteração é
   transparente para o CLI: `with_fonts_and_system` passa a incluir embutidas.

## Critérios de Verificação

```
Dado um SystemWorld construído com with_fonts_and_system(&[])
Quando book().select("Libertinus Serif", &FontVariant::default()) é chamado
Então retorna Some(idx)

Dado um SystemWorld construído com with_fonts_and_system(&[])
Quando font(idx) é chamado para o índice de "Libertinus Serif"
Então retorna Some(Font) com bytes não vazios

Dado um SystemWorld construído com new(root, main).with_fonts(paths)
Quando book().len() é consultado
Então comportamento é idêntico ao pré-P753 (apenas paths fornecidos, sem embutidas)

Dado um documento vazio de texto "X" compilado com o CLI cristalino
Quando a fonte embutida no PDF é inspeccionada
Então a família é "Libertinus Serif" (e não "Liberation Serif")

Dado um documento devanagari "नमस्ते संसार" compilado com o CLI cristalino
Quando a fonte embutida no PDF é inspeccionada
Então a família NÃO é "NewCMMath-Regular" nem "NewCMMath-Book"
E os glifos renderizados são reconhecíveis como devanagari
```

## Resultado Esperado

- `03_infra/src/embedded_fonts.rs` criado.
- `03_infra/Cargo.toml` com `typst-assets` (feature `fonts`).
- `SystemWorld::with_embedded_fonts` disponível.
- `SystemWorld::with_fonts_and_system` carrega texto embutido + sistema +
  math/code embutido + projecto.
- Fonte por defeito do cristalino (`01_core/src/entities/style_chain.rs`) passa a
  ser `Libertinus Serif`.
- Fallback para scripts não latinos (devanágari, árabe, CJK) continua a usar
  fontes do sistema especializadas em vez de fontes math/code embutidas.
- `crystalline-lint .` com zero violations.

## §P784 — nomes de família reais das fontes "New Computer Modern *"

`embedded_font_group()` classifica cada fonte embutida em `"text"` ou
`"math_code"` por substring do nome de família. **P783** assumiu (sem
verificar por leitura directa da tabela `name`) que o nome de
`NewCMMath-*.otf` era `"New Computer Modern Math"` (com espaços — a
correcção anterior de P783, substituindo o `"newcmmath"` ainda mais
errado). **Ambas estavam erradas.**

Medido por `fontTools`/`ttf_parser` (nameID 1 `FAMILY`) directamente nos
ficheiros embutidos via `typst-assets` (`c0ae970`, o mesmo pinned em
`Cargo.lock`):

| Ficheiro | Família real (nameID 1) |
|---|---|
| `LibertinusSerif-Regular.otf` | `"Libertinus Serif"` (com espaços) |
| `NewCMMath-Regular.otf` | `"NewComputerModernMath"` (**sem** espaços) |
| `NewCM10-Regular.otf` | `"NewComputerModern10"` (**sem** espaços) |

A inconsistência é dos próprios ficheiros de fonte upstream, não de
extracção do cristalino (`font_info_from_bytes`, `03_infra/src/fonts.rs`,
lê o nameID correcto — só o *literal usado para comparar* em
`embedded_font_group` estava errado).

**Consequência prática de ambos os bugs**: (1) `NewCM10` (nome real
`"NewComputerModern10"`) nunca correspondia a `starts_with("newcm10")`
nem a `starts_with("newcomputermodern10")` até esta correcção — caía no
`else` "math_code" por acidente, apesar de a docstring de
`EmbeddedFontSets` dizer explicitamente que devia estar em `"text"`. (2)
`NewCMMath` nunca correspondia a `contains("new computer modern math")`
(com espaços) — o teste `p754_newcm_math_is_not_in_text_group` passava
sempre, mas **vacuamente** (nunca encontrava a string em lado nenhum, nem
sequer em `math_code`), não porque a classificação estivesse correcta.
Reforçado com uma asserção positiva (confirma presença em `math_code`, não
só ausência em `text`) — ver `embedded_fonts.rs`.

Corrigido: `embedded_font_group` compara agora contra os nomes reais
(`"newcomputermodernmath"`, `"newcomputermodern10"`, ambos sem espaços,
comparação já `to_lowercase()`). Efeito colateral descoberto e corrigido
na mesma investigação, não scope creep: `NewCM10` passa a ir para `"text"`
correctamente. Ver `infra/shaper.md` §P783/P784 para o impacto a jusante
(resolução de candidatos por nome em `shaper.rs`, o bug que motivou esta
investigação — verificação visual real com glifo `⨿`/U+2A3F).

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-07-14 | Criação — P753: fontes embutidas para paridade com vanilla | `embedded_fonts.md` |
| 2026-07-14 | P754: separar texto e math/code no FontBook para não quebrar fallback de scripts não latinos | `embedded_fonts.md`, `embedded_fonts.rs`, `world.rs` |
| 2026-07-17 | P784: nomes de família reais das fontes "New Computer Modern *" são sem espaços (`NewComputerModernMath`/`NewComputerModern10`) — P783 tinha assumido "New Computer Modern Math" (com espaços), nunca verificado por leitura directa. `NewCM10` estava a ser classificado incorrectamente em math_code por este erro | `embedded_fonts.md`, `embedded_fonts.rs`, `fallback_fonts.rs` |
