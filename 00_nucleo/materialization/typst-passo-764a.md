---
# P764a — Sonda condicional + implementação: resolução de versão implícita

> **Passo:** 764a
> **Data:** 2026-07-15
> **Foco:** L0 fechado em P764 (`00_nucleo/prompts/infra/package_version_resolution.md`, hash `caeadbbd`). A sonda de P764 **refutou a premissa original** — o vanilla 0.15.0 exige versão em `#import`, não existe resolução implícita nesse caminho. A única resolução implícita real observada foi em `typst init @preview/nome` (sem versão). Antes de implementar qualquer coisa, este passo confirma se o comando `init` sequer existe no cristalino — se não existir, a implementação não tem onde ser chamada, e o passo fecha como backlog sem código.
> **Tipo:** Sonda condicional. Implementação só prossegue se a sonda confirmar um ponto de invocação real.
> **Tamanho:** S.
> **ADR-0108 EM VIGOR** — não implementar algo sem uso real confirmado.
> **Dependências:** P764 (L0 e sonda fechados, hash `caeadbbd`). P763a, se a decisão de cache/dados partilhados exigir a mesma infraestrutura de directórios.

---

## Sonda — existe `typst init` no cristalino?

```bash
./target/release/typst init --help 2>&1
grep -rn "\"init\"\|Init\b" 02_shell/src/*.rs 04_wiring/src/main.rs 2>/dev/null
```

### Caso A — `init` não existe no cristalino

Não há ponto de invocação real para `VersionlessPackageSpec`. Não implementar nada agora — registar como item de backlog no L0 já escrito, sem código. Fechar o passo aqui.

### Caso B — `init` existe no cristalino

Prosseguir para a implementação abaixo.

---

## Implementação (só se Caso B)

### Função de enumeração e ordenação

```rust
// 01_core/src/rules/... (caminho a confirmar contra a estrutura real de resolve_package)
fn latest_version(cached: &[PackageVersion]) -> Option<PackageVersion> {
    cached.iter().max() // ordem lexical de (major, minor, patch), já é o Ord derivado
}
```

Confirmado pela sonda de P764: `PackageVersion` só tem `major.minor.patch` (`u32`); versões malformadas (`0.2.0-beta`) são rejeitadas na leitura do directório, não entram na lista — sem mensagem de erro, comportamento silencioso (paridade com o vanilla, regra 4 do handoff).

### Ligação

Só ao caminho que constrói `VersionlessPackageSpec` (dentro do comando `init`, conforme confirmado na sonda), não ao resolvedor geral de `import` — isso continuaria a exigir versão explícita, replicando o vanilla.

---

## Validação (só se Caso B)

```bash
cargo test --workspace
crystalline-lint .
./target/release/typst init @preview/cetz /tmp/p764a-init-test
```

Esperado: resolve para a versão mais alta disponível (local ou remota, conforme decisão do L0), mesmo comportamento observado no vanilla por P764.

---

## Critério de fecho do passo

- [ ] Sonda de existência de `init` executada e documentada.
- [ ] Se Caso A: decisão registada explicitamente como backlog, sem código, sem abrir excepção à regra de ouro.
- [ ] Se Caso B: função de enumeração/ordenação implementada, ligada só ao caminho de `init`, testada.
- [ ] `cargo test --workspace` verde (ambos os casos).
- [ ] `crystalline-lint .` zero violações (ambos os casos).
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p764a.md`.

---

## Próximo passo

Se Caso A: nenhum — item fica em backlog até `init` ser priorizado por outro passo.
Se Caso B: nenhuma dependência adicional identificada; funcionalidade fecha aqui.
