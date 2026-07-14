# Prompt L0 — `infra/embedded_fonts` — Fontes Embutidas via `typst-assets`
Hash do Código: 78852948

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
garantindo que a fonte por defeito `Libertinus Serif` esteja sempre disponível.

## Restrições Estruturais

- `typst-assets` é uma dependência de L3 — fornece bytes de fonte em tempo de
  compilação/build.
- L1 continua a receber apenas `FontBook`/`FontInfo` (metadados primitivos) e
  `Font(Vec<u8>)` opaco.
- O carregamento de fontes embutidas deve ser **aditivo**: fontes do sistema e
  fontes de projecto (`--font-path`) continuam a funcionar exactamente como hoje.
- A ordem de prioridade no `FontBook` é: **embutidas primeiro**, depois sistema,
  depois projecto. Isto assegura que o vanilla-like set é preferido por defeito,
  mas projectos podem sobrepôr-se ao adicionar fontes com o mesmo nome depois.
- Não alterar a trait `World` nem a assinatura dos métodos `book()`/`font()`.

## Instrução

1. Adicionar `typst-assets` às dependências de `03_infra/Cargo.toml`, com a
   feature `fonts`, usando a mesma revisão git que o vanilla 0.15.0
   (`https://github.com/typst/typst-assets?rev=c0ae970`).

2. Criar `03_infra/src/embedded_fonts.rs` com função pública:
   ```rust
   pub fn load_embedded_fonts() -> (Vec<FontSlot>, FontBook)
   ```
   - Itera sobre `typst_assets::fonts()`.
   - Para cada blob de bytes:
     - Cria um `FontSlot` cujo carregamento lazy devolve `Some(Font::from_data(bytes))`.
       Como os bytes vêm de `typst_assets` (compilados no binário), não há path no
       disco; o `FontSlot` pode armazenar os bytes directamente em `Arc<[u8]>` ou
       usar um mecanismo equivalente que preserve a interface pública de
       `FontSlot`.
     - Extrai `FontInfo` via `font_info_from_bytes` (reutilizar
       `crate::fonts::font_info_from_bytes`).
   - Retorna os slots e o `FontBook` populado.

3. Expor em `03_infra/src/world.rs` um novo builder em `SystemWorld`:
   ```rust
   impl SystemWorld {
       /// Constrói um SystemWorld incluindo as fontes embutidas do vanilla.
       pub fn with_embedded_fonts(mut self) -> Self { ... }
   }
   ```
   - Chama `crate::embedded_fonts::load_embedded_fonts()`.
   - Associa os slots resultantes ao mundo da mesma forma que `with_fonts`.

4. Alterar `with_fonts_and_system` para combinar embutidas + sistema + projecto:
   ```rust
   pub fn with_fonts_and_system(mut self, font_paths: &[PathBuf]) -> Self
   ```
   - Carrega embutidas primeiro.
   - Depois carrega sistema via `fontdb::load_system_fonts`.
   - Finalmente adiciona fontes de projecto via `discover_fonts`.
   - Preserva a ordem: embutidas, sistema, projecto.

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
```

## Resultado Esperado

- `03_infra/src/embedded_fonts.rs` criado.
- `03_infra/Cargo.toml` com `typst-assets` (feature `fonts`).
- `SystemWorld::with_embedded_fonts` disponível.
- `SystemWorld::with_fonts_and_system` carrega embutidas + sistema + projecto.
- Fonte por defeito do cristalino (`01_core/src/entities/style_chain.rs`) passa a
  ser `Libertinus Serif`.
- `crystalline-lint .` com zero violations.

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-07-14 | Criação — P753: fontes embutidas para paridade com vanilla | `embedded_fonts.md` |
