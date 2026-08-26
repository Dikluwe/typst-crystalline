# ADR-0130 — Referências canônicas de Prompts L0

**Estado:** vigente — decisão homologada originalmente no Passo 1062

**Camada**: L0 — Governança e Rastreabilidade Documental
**Criado em**: 2026-08-17 (Passo 1062)
**Gate**: `ADR-0127` — Homologação da Opção (b) pelo dono (caminhos completos explícitos)

---

## 1. Princípio da Identificação Única por Caminho Completo

Devido à estrutura modular do compilador Typst, múltiplos submódulos em camadas distintas compartilham intencionalmente nomes de arquivos folha idênticos para manter a correspondência 1:1 com a arquitetura Rust (ex.: 7 arquivos `mod.md` correspondendo aos submódulos `mod.rs`, além de `layout.md`, `heading.md`, `footnote.md`, `bibliography.md`, etc., presentes em `compiler/`, `entities/` e `infra/`).

Para eliminar qualquer ambiguidade nas especificações L0 e na rastreabilidade com o código L1:

> [!IMPORTANT]
> **Regra de Desambiguação de Prompts (P1062 / ADR-0127)**:
> Toda referência ou citação a outro prompt L0 em qualquer arquivo `.md` deve utilizar **obrigatoriamente o caminho completo relativo a `00_nucleo/prompts/`**.
>
> * **Correto**: `compiler/layout.md`, `entities/layout.md`, `infra/layout.md`, `entities/ast/mod.md`, `compiler/eval/mod.md`
> * **Incorreto**: `layout.md`, `mod.md`, `heading.md`

Esta regra segue o mesmo princípio de explicitação e resiliência a fatiamento estabelecido no Passo 1054 ("preferir explicitação canônica a remissões curtas ambíguas").

---

## 2. Padrão de Anotação nos Arquivos-Fonte Rust (L1)

Todo arquivo de código-fonte Rust gerado ou mantido a partir de um prompt L0 deve declarar a anotação de proveniência no topo do módulo utilizando o caminho completo relativo à raiz do repositório:

```rust
//! @prompt 00_nucleo/prompts/<subpasta>/<nome_do_prompt>.md
//! @prompt-hash <hash_crc32_ou_sha>
```

Exemplos canônicos:
- `01_core/src/contracts/world.rs` $\to$ `//! @prompt 00_nucleo/prompts/contracts/world.md`
- `01_core/src/compiler/layout/mod.rs` $\to$ `//! @prompt 00_nucleo/prompts/compiler/layout.md`
- `01_core/src/entities/ast/code.rs` $\to$ `//! @prompt 00_nucleo/prompts/entities/ast/code.md`

---

## 3. Inventário de Nomes Folha Compartilhados (Referência P1060)

O inventário do Passo 1060 catalogou 35 nomes de arquivos repetidos entre pastas, totalizando 83 arquivos. Ao citar qualquer um destes arquivos, a atenção ao caminho completo é crítica:

1. **7 ocorrências**: `mod.md` (`compiler/`, `compiler/eval/`, `entities/`, `entities/ast/`, `entities/types/`, `infra/`, `contracts/`).
2. **3 ocorrências** (8 nomes, 24 arquivos):
   - `layout.md` (`compiler/`, `entities/`, `infra/`)
   - `heading.md` (`compiler/layout/`, `entities/`, `infra/`)
   - `footnote.md` (`compiler/layout/`, `entities/`, `infra/`)
   - `bibliography.md` (`compiler/layout/`, `entities/`, `infra/`)
   - `table.md` (`compiler/layout/`, `entities/`, `infra/`)
   - `label.md` (`compiler/`, `entities/`, `infra/`)
   - `color.md` (`compiler/`, `entities/`, `infra/`)
   - `_comum.md` (`compiler/`, `entities/`, `infra/`)
3. **2 ocorrências** (26 nomes, 52 arquivos):
   - `page.md`, `math.md`, `stroke.md`, `image.md`, `state.md`, `counter.md`, `context.md`, `shape.md`, `text.md`, `align.md`, etc.
