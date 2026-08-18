# L0 — Passo 1069: Criação de `export/pdf_defaults.rs` e Migração das Duplicações

**Gate**: `ADR-0127` — refactor estrutural, zero mudança de comportamento numérico
(confirmado no P1068: valores precisos e arredondados mantidos como constantes
separadas e nomeadas, não unificados). **Requer confirmação do dono antes de
executar**, mesmo assim — toca 2 arquivos existentes (`stream.rs`, `builder.rs`)
em múltiplos pontos e cria 2 arquivos novos.

**Base**: P1068 (auditoria + desenho, 4 candidatos com destino decidido).

---

## 1. Nucleação — prompt antes de código (per Manifesto, Princípio I)

Antes do arquivo `.rs`, criar o prompt L0 correspondente:
`00_nucleo/prompts/infra/export/pdf_defaults.md` — camada L3, alvo
`03_infra/src/export/pdf_defaults.rs`, documentando os 3 grupos de constantes
(`BEZIER_CIRCLE_KAPPA`, `FAUX_BOLD_K`, os 4 valores de A4) com a mesma citação de
proveniência já levantada no P1068 (ISO 32000-1, `lab/typst-original/.../text.rs`,
ISO 216).

**Confirmar o nome do diretório antes de criar** — `infra/export/` já tem
histórico de nomes colidentes nesta conversa (P1060: `mod.md` ocorre em
`infra/export/mod.md` e `infra/export/gradients/mod.md`); confirmar que
`pdf_defaults.md` não colide com nada existente antes de gravar.

`FontDescriptorMetrics::default()` (decisão do P1068 — `impl Default`, não
`pub const`) também precisa do seu próprio prompt, ou pode ser coberto pelo mesmo
`pdf_defaults.md` se a convenção do projecto permitir um prompt cobrir mais que um
item relacionado — confirmar contra outro prompt L0 já existente com padrão
semelhante antes de decidir, não inventar.

## 2. Criação do módulo

`03_infra/src/export/pdf_defaults.rs`, com o conteúdo já especificado no P1068:

```rust
//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/pdf_defaults.md
//! @prompt-hash <hash real, calculado após o prompt existir>
//! @layer L3
//! @updated 2026-08-17

pub const BEZIER_CIRCLE_KAPPA: f64 = 0.552_284_749_831;
pub const FAUX_BOLD_K: f64 = 0.04;
pub const A4_DEFAULT_WIDTH: f64 = 595.28;
pub const A4_DEFAULT_HEIGHT: f64 = 841.89;
pub const A4_FALLBACK_WIDTH_ROUNDED: f64 = 595.0;
pub const A4_FALLBACK_HEIGHT_ROUNDED: f64 = 842.0;
```

`FontDescriptorMetrics::default()` implementado em `export/fonts.rs` (decisão do
P1068 — mais perto do tipo, não em `pdf_defaults.rs`), não neste arquivo.

## 3. Migração dos pontos de uso — não reescrever lógica, só substituir literal por nome

| Local actual | Substituir por |
|---|---|
| `stream.rs:1245` (`const KAPPA`) | `pdf_defaults::BEZIER_CIRCLE_KAPPA` |
| `stream.rs:1458` (`const K`) | `pdf_defaults::BEZIER_CIRCLE_KAPPA` |
| `stream.rs:1597` (`const KAPPA`) | `pdf_defaults::BEZIER_CIRCLE_KAPPA` |
| `stream.rs:242` (`const FAUX_BOLD_K`) | `pdf_defaults::FAUX_BOLD_K` |
| `stream.rs:397` (`const FAUX_BOLD_K`) | `pdf_defaults::FAUX_BOLD_K` |
| `builder.rs:658-659` (`595.28, 841.89`) | `pdf_defaults::A4_DEFAULT_WIDTH/HEIGHT` |
| `builder.rs:1682` (`unwrap_or((595.0, 842.0))`) | `pdf_defaults::A4_FALLBACK_WIDTH_ROUNDED/HEIGHT_ROUNDED` |
| `builder.rs:2042, 2113` (`unwrap_or(842.0)`) | `pdf_defaults::A4_FALLBACK_HEIGHT_ROUNDED` |
| `builder.rs:1090-1094` | `FontDescriptorMetrics::default()` |
| `builder.rs:1578-1582` | `FontDescriptorMetrics::default()` |

Remover as declarações `const` locais duplicadas depois de cada ponto migrado
(não deixar `const KAPPA` local morto ao lado do import).

## 4. Verificação

1. `grep -rn "0.552_284_749_831\|0\.552284749831" 03_infra/src/export/` — deve
   aparecer só uma vez, em `pdf_defaults.rs`.
2. `grep -rn "const KAPPA\|const K:\|const FAUX_BOLD_K" 03_infra/src/export/stream.rs`
   — vazio depois da migração.
3. `cargo test --workspace` — 100% pass.
4. **Confirmar zero mudança de bytes de saída** — gerar o mesmo PDF de um
   documento do corpus canónico antes e depois desta migração, `diff` binário ou
   hash. Como os valores não mudaram (só o nome/localização), o PDF gerado deve
   ser byte-idêntico — esta é a prova mais forte de "zero comportamento alterado"
   deste passo, mais directa que reler o código.
5. `crystalline-lint .` — 0 erros.

## Critério de conclusão

- Prompt `pdf_defaults.md` criado e citado correctamente nos 2 arquivos `.rs`
  afectados (novo + `fonts.rs`).
- 10 pontos de uso migrados (tabela §3), zero `const` local duplicado restante.
- PDF de saída byte-idêntico antes/depois (item 4 da verificação) — não só "os
  testes passam", que não prova ausência de mudança se nenhum teste comparar
  bytes exactos.
- `crystalline-lint .` — 0 erros.

---

## Nota — estado das duas frentes de "hardcode" desta conversa

Com o P1069, a segunda frente (expansão de módulos de constantes) fecha o
domínio `export/`. `stdlib/text/` já foi dispensado no P1068 (sem candidatos).
Nenhuma pendência nova conhecida nesta linha depois deste passo.
