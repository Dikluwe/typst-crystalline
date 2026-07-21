---
# P796 — `sys.version` display, `.at()`, e `--version` do CLI

> **Passo:** 796
> **Data:** 2026-07-20
> **Foco:** P786 confirmou três divergências em `utils::version_`: (1) `sys.version` renderiza como `version(0, 15, 0)` (representação de debug/construtor) no cristalino, quando o vanilla exibe `0.15.0` (formatação amigável); (2) `.at()` em `Version` é `unknown field`, quando o vanilla permite indexar componentes; (3) `typst --version` do CLI mostra `typst 0.1.0` (sem hash de commit), inconsistente com o próprio `sys.version` reportando `0.15.0` — os dois deveriam ser consistentes entre si e com o vanilla (`typst 0.15.0 (969087ec)`).
> **Tipo:** Sonda + Implementação directa.
> **Tamanho:** S/M.
> **ADR-0108 EM VIGOR** — confirmar o formato de exibição e a API de `.at()` antes de implementar.
> **Dependências:** P786 (achado, evidência em `temp/temp_p786/c_version_sys.typ`, `c_version_sys2.typ`, `--version`).

---

## Sonda — mecanismo exato do vanilla

```bash
grep -n "impl Display for Version\|fn at\b" lab/typst-original/crates/typst-library/src/foundations/version.rs 2>/dev/null
```

Confirmar:
1. O formato de `Display` para `Version` — provavelmente `major.minor.patch`, sem partes zero à direita se for esse o padrão do vanilla (confirmar, não assumir).
2. A assinatura de `.at(index)` — o que retorna para índices fora do alcance.

```bash
cat > /tmp/p796-version.typ <<'EOF'
#sys.version
#sys.version.at(0)
EOF
lab/typst-original/target/release/typst compile /tmp/p796-version.typ 2>&1
lab/typst-original/target/release/typst --version
```

---

## Decisão sobre o número de versão do CLI

O cristalino é um projeto próprio, não uma cópia do Typst — decidir explicitamente qual convenção seguir para `--version` do CLI: mostrar a própria versão do cristalino (ex: `typst-crystalline 0.1.0`) ou espelhar `0.15.0` para fins de paridade de teste. Esta decisão não é puramente técnica — registrar com justificativa, não assumir que "espelhar o vanilla" é automaticamente certo (o projeto pode ter uma identidade de versão própria por design).

---

## Implementação

1. Implementar `Display` para `Version` com o formato correto.
2. Implementar `.at()` como campo/método nativo.
3. Corrigir `--version` do CLI conforme a decisão tomada (item anterior) — se for para mostrar hash de commit, confirmar de onde vem essa informação no processo de build.

---

## Validação

```bash
./target/release/typst compile /tmp/p796-version.typ 2>&1
./target/release/typst --version
```

Confirmar formato correto e consistência entre `sys.version` e `--version`.

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Formato de `Display` de `Version` confirmado e implementado.
- [ ] `.at()` implementado.
- [ ] Decisão sobre convenção de `--version` do CLI registrada explicitamente, com justificativa.
- [ ] `sys.version` e `--version` consistentes entre si.
- [ ] `cargo test --workspace` verde, contagem da suíte mostrada.
- [ ] `crystalline-lint .` zero violações.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p796.md`, com comandos, saídas reais e nomes dos testes persistidos.

---

## Próximo passo

Ênfase/fontes (bold/italic sem efeito visual) — último item da lista de P786 §5.
