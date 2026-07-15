# Relatório de Paridade — P764a

> **Passo:** 764a  
> **Data:** 2026-07-15T15:36:14-03:00  
> **Commit base:** `5469d8d1e12f0bb77732b22b6dd1844312b3f282`  
> **L0:** `00_nucleo/prompts/infra/package_version_resolution.md` (hash `caeadbbd`)

---

## Resumo

Sonda condicional para confirmar a existência de `typst init` no cristalino. A sonda de P764 refutou a premissa original — o vanilla 0.15.0 só tem resolução implícita de versão no comando `typst init @preview/nome`. Se o comando não existir no cristalino, não há ponto de invocação real para a funcionalidade.

## Estado do critério de fecho

- [x] Sonda de existência de `init` executada e documentada.
- [x] Caso A confirmado: `typst init` não existe no cristalino.
- [x] Decisão registada explicitamente como backlog, sem código, sem abrir excepção à regra de ouro.
- [x] `cargo test --workspace` verde.
- [x] `crystalline-lint .` — zero erros; warning V7 esperado no L0 de `package_version_resolution.md` (órfão porque não há código que o referencie).

## Evidência da sonda

```bash
./target/release/typst init --help 2>&1
# Usage: typst [OPTIONS] <INPUT> [OUTPUT]
# error: unexpected argument 'init' found

grep -rn '"init"\|Init\b' 02_shell/src/*.rs 04_wiring/src/main.rs 2>/dev/null
# sem resultados
```

## Decisão

Não implementar `VersionlessPackageSpec` nem resolução implícita neste passo. O item fica em backlog até o comando `init` ser priorizado por outro passo. O L0 `package_version_resolution.md` permanece válido e será reutilizado quando houver ponto de invocação real.

## Próximo passo

Nenhuma dependência adicional identificada; funcionalidade fecha aqui até `init` ser priorizado.
