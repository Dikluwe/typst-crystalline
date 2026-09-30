# Archived pre-sanitation L0 — 00_nucleo/prompts/infra/system-world.md

This is historical evidence, not a materializable prompt or an approval.
Original SHA-256: `9ba59901676002202545736c870fb0455b14b502bf3c69ebde6eda8b91405df6`.
The original text below is preserved without rewriting; dates, line references,
working-tree inventories, proposals and old parity claims retain their original scope.
Dead or unpinned references are archival only and cannot prove current behavior.

---

# Prompt L0 — SystemWorld: fachada e composição
Hash do Código: 4e7e5ef0

**Camada:** L3
**Ficheiro alvo:** `03_infra/src/world.rs`
**Atualizado em:** 2026-09-26

## Medição anterior à decisão

O consumer vigente antes da extração possui 1303 linhas: SourceSlot em
`03_infra/src/world.rs:37-71`, bases de pacotes em `:76-100`, estado e
construtores em `:103-275`, fontes em `:293-364` e `:418-469`, registro
em `:368-404`, paths em `:472-586`, bytes em `:589-622`, entrypoint em
`:625-652`, World em `:655-870` e testes em `:873-1303`.
SHA-256 do L0 anterior: a0399075d4714a6a825c1de6063d24a45a5ce6a0353d34155e20d7cbf6997af2.
A atomização é mecânica L3 (ADR-0109). Não altera semântica, contratos
públicos, defaults, ordem de pipeline nem compatibilidade (ADR-0127).

## Fachada proprietária

Possui SystemWorld, seus campos privados, SystemWorldError e implementações
Display/Error; `new`, `for_eval`, `with_inputs` e `with_plugin_host`.
Mantém o caminho público `typst_infra::world::SystemWorld` e todas as
assinaturas e builders públicos existentes, mesmo quando seus impls passam
a residir nos filhos. Não criar nova representação de estado neste lote.

- `new` aceita root Into<PathBuf> e main AsRef<Path>, canonicaliza main,
  instala FileId(1), lê eager e conserva erros atuais; próximo ID é 2.
- `for_eval` instala source vazio materializado em root/<input-expression>
  sem ler/escrever esse path; imports mantêm base root.
- Ambos inicializam mapas, locks, book/library, inputs vazios, plugin None,
  caches vazios e downloader padrão obtido da política de pacotes existente.
- `with_inputs` converte Strings para SysInputs em L3; `with_plugin_host`
  instala Arc sem mudar default. Não adicionar I/O à camada L1.

## Descendentes com ownership 1:1

Cada filho permanece privado; usa estado da fachada por descendência.
Helpers entre irmãos usam apenas pub(super), nunca pub(crate) ou API nova.

| Consumer | Prompt proprietário |
|---|---|
| world/cache.rs | 00_nucleo/prompts/infra/system-world/cache.md |
| world/paths.rs | 00_nucleo/prompts/infra/system-world/paths.md |
| world/packages.rs | 00_nucleo/prompts/infra/system-world/packages.md |
| world/fonts.rs | 00_nucleo/prompts/infra/system-world/fonts.md |
| world/world_impl.rs | 00_nucleo/prompts/infra/system-world/world_impl.md |
| world/tests.rs | 00_nucleo/prompts/infra/system-world/tests.md |

As obrigações P450/P515/P681/P686/P694/P699/P753/P754/P772b/P827/P839/
P876/P927/P938/P1137/P1141/P1146/P1410 são distribuídas por responsabilidade
nos owners acima, sem alterar sua implementação. A injeção customizada de
CA e seu pin passam ao owner packages; a fachada não possui essa política.

## Verificação

Preservar source main, IDs, imports, leitura de assets e mensagens de erro.
Caracterizações adicionais podem ser escritas primeiro no módulo de testes
inline atual e depois transferidas ao consumer de testes. Gate estrutural
RED→GREEN: descendentes existem e a fachada não contém implementações de
cache, resolução, fontes ou World. Testes de comportamento devem continuar
passando antes e depois da extração; não inventar mudança semântica para
produzir um RED. Build e lint validam API pública e ownership/linhagem.
