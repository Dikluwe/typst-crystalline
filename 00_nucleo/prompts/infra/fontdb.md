# Prompt L0 — `infra/fontdb` — Descoberta automática de fontes do sistema

**Camada**: L3  
**Criado em**: 2026-06-30  
**Atualizado em**: 2026-07-10  
**Arquivos gerados**: `03_infra/src/fontdb.rs` (novo), alterações em `03_infra/src/world.rs`, `03_infra/Cargo.toml`  
**ADR referência**: ADR-0020 (ativação), ADR-0019, ADR-0022, ADR-0108  

---

## Contexto

O `SystemWorld` actual carrega fontes apenas via paths explícitos (`with_fonts(paths)`), usando `discover_fonts` + `FontSlot` + `OnceLock`. Isto funciona para testes e para projectos que distribuem fontes, mas não descobre automaticamente as fontes instaladas no sistema operativo.

A ADR-0020 adiou a integração de `fontdb` até o CLI precisar de descoberta automática sem `--font-path`. O Passo 515 (Trilha 5) activa essa condição.

## Restrições Estruturais

- `fontdb` é uma dependência de L3 — faz I/O de sistema e leitura de variáveis de ambiente.
- L1 continua a receber apenas `FontBook`/`FontInfo` (metadados primitivos) e `Font(Vec<u8>)` opaco.
- A integração deve ser **aditiva**: `SystemWorld::with_fonts(paths)` continua a funcionar exactamente como hoje.
- Não alterar a trait `World` nem a assinatura dos métodos `book()`/`font()`.
- `fontdb::Database` pode ser mantido vivo em `SystemWorld` para garantir que os bytes das fontes (memória-mapeada ou do file system) permaneçam válidos enquanto o `World` existir.

## Instrução

1. Adicionar `fontdb` às dependências de `03_infra/Cargo.toml` (versão `0.21` ou compatível com `ttf-parser 0.25` / `rustybuzz 0.20`).

2. Criar `03_infra/src/fontdb.rs` com função pública:
   ```rust
   pub fn load_system_fonts() -> (Vec<FontSlot>, FontBook)
   ```
   - Inicializa `fontdb::Database::new()`.
   - Chama `db.load_system_fonts()`.
   - Itera `db.faces()`; para cada face:
     - Obtém o caminho do ficheiro via `face.source.path()`.
     - Usa `face.index` (índice da face na colecção).
     - Cria um `FontSlot::new(path, index)`.
     - Extrai `FontInfo` via `font_info_from_bytes` (reutilizar `crate::fonts::font_info_from_bytes`), **usando `db.with_face_data(face.id, |data, index| font_info_from_bytes(data, index))`** para reutilizar os bytes já carregados pelo `fontdb` em vez de reler o ficheiro do disco. ~~Faces que falhem a extrair `FontInfo` são mantidas como slots~~ **(revogado em P839)**: faces sem `FontInfo` extraível **não** entram nos slots nem no `FontBook` — os dois ficam sempre emparelhados por índice, como no vanilla (`typst-kit/src/fonts.rs:176-189`, `filter_map`). A redacção original ("mantidas como slots, o FontBook ignora-as") codificava o desalinhamento medido no achado #28/I4 de P831.
   - Retorna os slots e o `FontBook` populado.

3. Expor em `03_infra/src/world.rs` um novo builder em `SystemWorld`:
   ```rust
   impl SystemWorld {
       /// Constrói um SystemWorld com descoberta automática de fontes do sistema.
       /// Preserva a possibilidade de adicionar fontes de projecto via `with_fonts`.
       pub fn with_system_fonts(root: impl Into<PathBuf>, main: impl AsRef<Path>)
           -> Result<Self, SystemWorldError>;
   }
   ```
   - Inicializa `SystemWorld::new(root, main)` (sem fontes).
   - Chama `crate::fontdb::load_system_fonts()`.
   - Associa os slots resultantes ao mundo da mesma forma que `with_fonts`.

4. Opcionalmente, adicionar composição:
   ```rust
   pub fn with_system_and_project_fonts(mut self, project_paths: &[PathBuf]) -> Self
   ```
   - Carrega fontes do sistema e depois adiciona as fontes de projecto via `discover_fonts`, evitando duplicados por caminho+index.

5. No CLI (`04_wiring` / `02_shell`), activar `with_system_fonts` por defeito quando nenhum `--font-path` for fornecido. Este prompt **não** cobre a alteração da CLI — apenas a infraestrutura L3.

## Critérios de Verificação

```
Dado um SystemWorld construído com with_system_fonts
Quando book().len() é consultado
Então retorna > 0 em sistema com fontes instaladas

Dado um SystemWorld construído com with_system_fonts
Quando font(idx) é chamado para um índice válido do FontBook
Então retorna Some(Font) com bytes não vazios

Dado um SystemWorld construído com new(root, main).with_fonts(paths)
Quando book().len() é consultado
Então comportamento é idêntico ao pré-P515 (apenas paths fornecidos)

Dado load_system_fonts() num sistema sem fontes
Quando o par (slots, book) é inspeccionado
Então slots e book estão vazios (não panic)
```

## Resultado Esperado

- `03_infra/src/fontdb.rs` criado.
- `03_infra/Cargo.toml` com `fontdb`.
- `SystemWorld::with_system_fonts` disponível.
- Testes unitários em `world.rs` ou `fontdb.rs` verificando pelo menos um slot não vazio quando há fontes no sistema.
- `crystalline-lint .` com zero violations.

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-06-30 | Criação — ativação de ADR-0020 para P515 | `fontdb.md` |
| 2026-07-10 | P674 — elimina leitura duplicada de fontes do sistema usando `db.with_face_data` | `fontdb.md`, `03_infra/src/fontdb.rs` |
| 2026-07-22 | P839 — revoga o "faces sem info mantidas como slots": slot e entrada no book inseridos juntos (índices alinhados), replicando o `filter_map` do vanilla; latente registado por P838, achado #28/I4 de P831 | `fontdb.md`, `03_infra/src/fontdb.rs` |
