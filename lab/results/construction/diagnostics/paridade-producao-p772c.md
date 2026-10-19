# Paridade de Produção — P772c: `typst_syntax::package`

**Data:** 2026-07-16T12:40:21-03:00  
**Commit base:** `d258e2ed7c7eb60bab0acd703ea4be8134a24101`  
**Estado da working tree:** existem modificações não commitadas além deste passo (ver `git diff HEAD --stat` no final).  
**Prompt L0 afetado:** `00_nucleo/prompts/entities/package-spec.md` → hash `b38b66ac`

---

## 1. Lista de itens classificados

Fonte da lacuna:

```bash
awk -F'\t' '$1=="lacuna-inventario" && $5 ~ /typst_syntax::package/' \
  00_nucleo/diagnosticos/lente-lista-B-2026-07-15.txt | cut -f5 | sort
```

| # | Item | Classificação ADR-0107 | Justificação |
|---|------|------------------------|--------------|
| 1 | `typst_syntax::package::PackageInfo` | **mecânica / diverge** | Dados do manifesto `typst.toml` ([package]). O cristalino parseia o manifesto via `toml::Value` em L3 (`SystemWorld::load_package_entrypoint`), sem struct tipificado. Não afecta a semântica da linguagem. |
| 2 | `typst_syntax::package::PackageManifest` | **mecânica / diverge** | Struct tipificado do manifesto com `serde`. O cristalino usa `toml::Value` em L3 (ADR-0005 proíbe `serde` em L1). |
| 3 | `typst_syntax::package::parse_name` | **semântica / paridade** | O cristalino tem parsing equivalente em `PackageSpec::from_str` (L1), usando `is_ident` do lexer. |
| 4 | `typst_syntax::package::parse_namespace` | **semântica / paridade** | Idem — coberto por `PackageSpec::from_str`. |
| 5 | `typst_syntax::package::parse_version` | **semântica / paridade** | Idem — coberto por `PackageVersion::from_str`. |
| 6 | `typst_syntax::package::TemplateInfo` | **mecânica / diverge** | Secção `[template]` do manifesto. Não implementada no cristalino (fora de escopo até nova funcionalidade de templates). |
| 7 | `typst_syntax::package::ToolInfo` | **mecânica / diverge** | Secção `[tool]` do manifesto. Não aplicável ao compilador. |
| 8 | `typst_syntax::package::UnknownFields` | **mecânica / diverge** | Mapa de campos desconhecidos do manifesto. O cristalino não o expõe. |

**Conclusão da classificação:** os únicos itens com semântica da linguagem são `parse_name`, `parse_namespace` e `parse_version`, que já estão replicados no cristalino. Os restantes são mecanismos de manifesto/serde que divergem de propósito (ADR-0005, ADR-0107).

---

## 2. Testes de parsing de especificações malformadas

Executámos a bateria do passo:

```bash
for spec in "@preview" "@preview/" "@preview/nome:" "@preview/nome:abc" \
            "preview/nome:1.0.0" "@preview/nome:1.0.0.0"; do
  cat > /tmp/p772c-test.typ <<EOF
#import "$spec"
EOF
  lab/typst-original/target/release/typst compile /tmp/p772c-test.typ 2>&1 | head -3
  ./target/release/typst /tmp/p772c-test.typ 2>&1 | head -3
done
```

### Resultados

| Especificação | Vanilla | Cristalino | Observável diverge? |
|---------------|---------|------------|---------------------|
| `@preview` | `package specification is missing name` | `package specification is missing name` | Não (formato de saída differe, mensagem idêntica) |
| `@preview/` | `package specification is missing name` | `package specification is missing name` | Não |
| `@preview/nome:` | `package specification is missing version` | `package specification is missing version` | Não |
| `@preview/nome:abc` | `` `abc` is not a valid major version `` | `` `abc` is not a valid major version `` | Não |
| `preview/nome:1.0.0` | `file not found` | `include: ficheiro não encontrado: /tmp/preview/nome:1.0.0` | Semântica igual (tratado como path relativo); cristalino mostra `<detached>` no path — é um resquício de formatação, não de package parsing |
| `@preview/nome:1.0.0.0` | `version number has unexpected fourth component: `0`` | `version number has unexpected fourth component: `0`` | Não |

### Análise

As **mensagens de erro de parsing** são idênticas entre vanilla e cristalino. A única diferença visível é o formato de apresentação (pretty-print vs gcc/clang `path:linha:coluna`), o que é mecânica de L2 conforme ADR-0045/ADR-0050.

O caso `preview/nome:1.0.0` (sem `@`) não é um erro de package spec em nenhum dos dois: ambos tratam a string como path relativo e falham com "file not found". O cristalino mostra `<detached>` porque o erro de I/O vem sem span resolvível — isto é um detalhe de formatação de diagnósticos, não de parsing de `PackageSpec`.

---

## 3. Bugs reais

**Nenhum bug real encontrado** no parsing de `PackageSpec`/`PackageVersion`. As mensagens de erro já estavam alinhadas com o vanilla.

Como prevenção de regressão, endurecemos os testes unitários em `01_core/src/entities/package_spec.rs` para verificar as **mensagens exactas** de erro nos casos de parsing malformado:

- `package_spec_from_str_err_sem_arroba`
- `package_spec_from_str_err_namespace_vazio`
- `package_spec_from_str_err_namespace_invalido`
- `package_spec_from_str_err_nome_vazio`
- `package_spec_from_str_err_nome_invalido`
- `package_spec_from_str_err_sem_versao`
- `package_version_from_str_err_non_numeric`
- `package_version_from_str_err_too_few_parts`
- `package_version_from_str_err_too_many_parts`

---

## 4. Validação

```bash
cargo test --workspace
crystalline-lint .
```

- `cargo test --workspace`: todos os testes passaram.
- `crystalline-lint .`: zero violações (apenas warning pré-existente V7 sobre `package_version_resolution.md` órfão).

---

## 5. Estado do `git diff HEAD --stat`

```text
 00_nucleo/0.15.0.typ                     | 463 -------------------------------
 00_nucleo/testing/fontes-padrao-teste.md |  74 -----
 01_core/src/entities/package_spec.rs     |  39 ++-
 3 files changed, 34 insertions(+), 542 deletions(-)
```

Nota: os ficheiros `00_nucleo/0.15.0.typ` e `00_nucleo/testing/fontes-padrao-teste.md` já estavam modificados na working tree antes deste passo; não foram alterados por P772c.

---

## 6. Critério de fecho

- [x] Os 8 itens de `typst_syntax::package` classificados item a item.
- [x] Casos de parsing malformado testados com mensagem exacta comparada.
- [x] Confirmado que não há sobreposição não resolvida com P763/P764 (resolução/download de pacotes permanece inalterada).
- [x] Nenhum bug real a corrigir; testes de mensagem exacta adicionados como prevenção.
- [x] `cargo test --workspace` verde.
- [x] `crystalline-lint .` zero violações.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p772c.md`.
