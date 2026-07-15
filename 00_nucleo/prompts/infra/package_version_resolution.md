# Prompt L0 — `infra/package_version_resolution` — Resolução de versão implícita de pacotes

Hash do Código: caeadbbd

**Camada**: L3
**Criado em**: 2026-07-15 (Passo 764)
**Arquivos gerados**: `03_infra/src/package_downloader.rs` (futuro), alterações em `03_infra/src/world.rs`
**ADRs referência**: ADR-0107 (paridade com a linguagem), ADR-0108 (medir antes de decidir), ADR-0109 (atomização L1/L3)

---

## Contexto

A premissa inicial do relatório de pendências era que `#import "@preview/nome"` (sem versão) funcionava no vanilla e falhava no cristalino. A sonda de P764 refutou essa premissa: no vanilla CLI 0.15.0, a sintaxe de `import` **exige** versão; a única resolução implícita de versão observada está no comando `typst init` (templates) e em qualquer outro ponto futuro que use `VersionlessPackageSpec`.

Este L0 define como resolver uma versão quando o utilizador não a especifica, nos pontos onde isso é semanticamente válido.

## Objetivo

Especificar o comportamento de resolução de versão implícita de pacotes, alinhado ao vanilla CLI 0.15.0, de forma a evitar divergências de linguagem.

## Decisões arquiteturais (medidas em P764)

### 1. Fonte da "versão mais recente"

- **Namespace `@preview`**: consulta remota ao índice `https://packages.typst.org/preview/index.json`.
  - O vanilla não procura primeiro na cache local para determinar a versão mais recente; vai directamente ao índice remoto.
  - Se o índice remoto não estiver acessível, a resolução falha.
- **Outros namespaces (locais)**: procurar apenas na **data dir** (não na cache), conforme comentário do vanilla em `typst-kit/src/packages.rs`: "We only search in the data directory and not the cache directory, because the latter is not intended for storage of local packages."

### 2. Critério de ordenação semver

- O tipo `PackageVersion` do vanilla é composto apenas por `major`, `minor` e `patch` (`u32` cada).
- A ordenação é a ordem lexical dos três componentes numéricos (major → minor → patch), equivalente a `Ord` derivado da estrutura.
- Versões com sufixos de pré-lançamento (ex.: `0.2.0-beta`) **não são representáveis** em `PackageVersion`:
  - O parser rejeita `"0.2.0-beta"` com "version number has unexpected fourth component: `beta`".
  - Em namespaces locais, entradas de directório com nomes malformados são silenciosamente ignoradas por `latest_version`.
- Conclusão: não há tratamento especial de pré-lançamentos; eles são simplesmente ignorados.

### 3. Mensagem de erro para versão malformada na cache

- Não há mensagem de erro específica.
- O vanilla ignora entradas cujo nome de directório não parseie como `PackageVersion`.
- Replicar este comportamento: ignorar silenciosamente, sem alertar o utilizador.

### 4. Cache de resolução dentro da mesma compilação

- **Decisão de dono**: cache em memória do índice remoto e das resoluções de `latest_version` durante a vida do `SystemWorld`.
- O vanilla usa `once_cell::sync::OnceCell` para guardar o índice em memória no `UniversePackages`.
- Replicar: o índice é descarregado no máximo uma vez por instância de `SystemWorld`.

### 5. Ponto de entrada no código

- `03_infra/src/world.rs`, método `SystemWorld::resolve_package`.
- Quando um `PackageSpec` sem versão chegar ao ponto de resolução (se algum dia existir no parser), a versão é preenchida antes de procurar em disco ou descarregar.
- Até que o parser de import aceite versões omitidas, este L0 aplica-se principalmente a futuros comandos como `init` ou `template`.

## Restrições estruturais

- A lógica de resolução remota (download do `index.json`) fica em L3, partilhando cliente HTTP com `infra/package_downloader.md`.
- L1 expõe apenas tipos puros (`PackageSpec`, `PackageVersion`, `VersionlessPackageSpec`) e um trait de resolução.
- Não se altera o parser de import para aceitar versão omitida sem um passo L0 dedicado à sintaxe.

## Critérios de verificação

```
Dado typst init @preview/cetz num sistema com cache local contendo 0.2.2 e 0.5.2
Quando o índice remoto indica que 0.5.2 é a versão mais recente
Então a versão resolvida é 0.5.2

Dado typst init @local/testpkg numa data dir com 0.1.0, 0.2.0 e 0.2.0-beta
Quando se pede a versão mais recente
Então a versão resolvida é 0.2.0 (0.2.0-beta é ignorada)

Dado um namespace preview sem conectividade de rede
Quando se tenta resolver a versão mais recente
Então a operação falha com erro de rede ou índice não encontrado
```
