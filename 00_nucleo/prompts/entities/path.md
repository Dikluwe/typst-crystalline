# Prompt L0 — `entities/path` — caminho virtual enraizado
Hash do Código: 26ff79df

**Camada:** L1  
**Ficheiro alvo:** `01_core/src/entities/path.rs`  
**Criado em:** 2026-08-24 (P1141)  
**ADRs:** ADR-0024, ADR-0107, ADR-0108, ADR-0127

## Medição antes da decisão

No vanilla ratificado `a51e02804`, `foundations/path.rs:134-224` define o tipo
público `path`, seu constructor contextual, `repr` e `PathOrStr`;
`typst-syntax/src/path.rs:16-60,72-81,189-357` define `RootedPath` como raiz
(`Project | Package(PackageSpec)`) + path virtual normalizado. As sondas P1141
mediram `type(path("x")) == path`, `path(path("x")) == path("x")`, igualdade
após normalizar `.`/`..`, `repr(path("a")) == "path(\"/a\")"`, rejeição de
barra invertida/escape e retenção da base através de chamada cross-file. Ver
`typst-passo-1141.md` para comandos, outputs e proveniência.

O cristalino tem apenas `FileId` opaco e `geometry::PathItem`; nenhum deles
representa esse contrato de linguagem. `SystemWorld` já conhece roots físicos,
mas o valor não pode conter `PathBuf` nem realizar I/O em L1.

## Decisão

Materializar em L1 um domínio lexical, portátil e fechado:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VirtualRoot { Project, Package(PackageSpec) }

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VirtualPath(EcoString); // forma absoluta normalizada, começa por '/'

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RootedPath { root: VirtualRoot, vpath: VirtualPath }

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PathOrStr { Path(RootedPath), Str(EcoString) }
```

Campos permanecem privados. Construtores/accessors públicos mínimos:
`VirtualPath::new`, `join`, `parent`, `get_with_slash`; `RootedPath::new`,
`root`, `vpath`; `PathOrStr` conversões de path/string. `EcoString` é permitido
em L1 pelo ADR-0024; nenhum tipo físico (`Path`, `PathBuf`) aparece no contrato.

Normalização é lexical: slash separa segmentos; `.` desaparece; `..` remove o
segmento anterior; ultrapassar a raiz retorna `PathError::Escapes`; qualquer
backslash retorna `PathError::Backslash`. Não consultar filesystem. Path
absoluto começa na raiz do chamador; relativo é unido ao parent do ficheiro
chamador pela resolução contextual do `World`.

`RootedPath` inclui a raiz na igualdade/hash. Dois paths de roots de pacote ou
projeto diferentes não são iguais mesmo quando o `repr` textual coincide.
`repr` é `path(<repr da vpath absoluta>)`; por paridade, não expõe a spec do
pacote. Essa diferença entre identidade e representação é intencional.

`PathOrStr::Path` nunca é re-resolvido. `PathOrStr::Str` é resolvido uma única
vez no `FileId` do span/chamador e então convertido em `RootedPath` antes de I/O.

## Fronteira L1/L3

L1 possui identidade virtual, normalização e erros de domínio. O trait `World`
traduz `(FileId, str)` em `RootedPath` e materializa `RootedPath` em recurso
físico; L3 possui root real, caches, canonicalização defensiva e I/O. O interner
global do vanilla não é copiado; mecânica de IDs/caches é livre (ADR-0107).

## Critérios de aceitação

- `a/./b/../c` normaliza para `/a/c`; `./x == x` no mesmo root.
- `..` além da raiz e `\\` falham com as classes/mensagens medidas.
- projeto e pacote preservam identidades distintas.
- path construído num ficheiro mantém root/vpath quando consumido noutro.
- zero I/O, relógio, ambiente, estado global ou `PathBuf` em L1.
- `geometry::PathItem` não é importado nem convertido.

## Gate

Tipo e métodos públicos novos: ADR-0127 exige confirmação antes de código.
