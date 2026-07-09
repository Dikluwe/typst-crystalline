# Relatório de Paridade — P646

**Passo:** 646  
**Data:** 2026-07-09  
**Foco:** Chave vazia em ficheiro `.yaml`/`.yml` de bibliografia deve produzir erro, tal como em `.bib`.  
**Dependências:** P644 (correcção original), P645 (onde o gap aparente foi encontrado).

---

## 1. Sonda

### 1.1 Caminhos de código

`.bib` e `.yaml`/`.yml` passam por caminhos separados em `parse_bibliography` (`01_core/src/rules/eval/bibliography.rs:55`):

- `.bib` → `crate::rules::eval::bibtex::parse_bibtex(content)` (parser BibTeX custom).
- `.yaml`/`.yml` → `hayagriva::io::from_yaml_str(content)` → iteração sobre `library` → `hay_entry_to_bib_entry`.

Apesar de caminhos separados, **ambos** verificam chave vazia:

- `.bib`: parser BibTeX rejeita `"@article{,"` com `esperado key da entrada`.
- `.yaml`/`.yml`: `hay_entry_to_bib_entry` (`bibliography.rs:146`) rejeita `key.is_empty()` com `bibliography contains entry with empty key`.

### 1.2 Comportamento do vanilla

```bash
cat > /tmp/p646-chave-vazia.yaml <<'EOF'
"":
  type: Article
  title: Sem chave
  author: Alguém
  date: 2024
EOF
cat > /tmp/p646-chave-vazia.typ <<'EOF'
#bibliography("p646-chave-vazia.yaml")
EOF
lab/typst-original/target/release/typst compile /tmp/p646-chave-vazia.typ /tmp/p646-vanilla.pdf
```

Saída do vanilla:

```text
error: bibliography contains entry with empty key
  ┌─ p646-chave-vazia.typ:1:14
  │
1 │ #bibliography("p646-chave-vazia.yaml")
  │              ^^^^^^^^^^^^^^^^^^^^^^^^
```

O vanilla rejeita a chave vazia em `.yaml` com a mesma mensagem.

### 1.3 Causa real do comportamento observado em P645

A sonda revelou que o código do cristalino **já rejeita** chave vazia em `.yaml`. O comando de P645 parecia ter sucesso porque o binário `target/release/typst` estava desactualizado (pré-P644). Após recompilar com `cargo build --release --bin typst`, o comportamento correcto foi confirmado:

```text
p646-chave-vazia.typ:<detached>: error: bibliography contains entry with empty key
```

---

## 2. Implementação

Não foi necessária alteração de código de produção: a verificação de chave vazia em `.yaml`/`.yml` já estava implementada por P644 em `hay_entry_to_bib_entry`.

Para garantir cobertura do caminho completo (leitura do ficheiro + parse YAML + verificação de chave vazia), adicionou-se um teste unitário em `01_core/src/rules/eval/bibliography.rs`:

- `p646_yaml_chave_vazia_via_load_bib_entries_produz_erro`: usa `MockWorldFs` e `load_bib_entries_from_path` para testar o fluxo completo, não apenas `parse_bibliography` directamente.

---

## 3. Validação

### 3.1 Testes automáticos

```bash
cargo test --workspace
```

Resultado: **todos os testes passaram**.

```bash
crystalline-lint .
```

Resultado: **✓ No violations found**.

### 3.2 CLI — chave vazia em `.yaml`

```bash
./target/release/typst p646-chave-vazia.typ p646-cristalino.pdf
```

Saída:

```text
p646-chave-vazia.typ:<detached>: error: bibliography contains entry with empty key
```

O comando falha com a mensagem esperada.

### 3.3 CLI — `.bib` continua a funcionar

Repetido o teste de P644 com `.bib` de chave vazia: continua a falhar com `esperado key da entrada`, sem regressão.

### 3.4 CLI — `.yaml` válido continua a funcionar

Ficheiro `.yaml` com chave preenchida produz PDF sem erros.

---

## 4. Decisão

- A verificação de chave vazia para `.yaml`/`.yml` já estava presente; a aparência de falha em P645 era artefacto de binário desactualizado.
- Adicionou-se teste de caminho completo para evitar regressão futura.
- `.bib` e `.yaml`/`.yml` agora têm cobertura equivalente para chave vazia.

---

## 5. Estado de fecho

- [x] Sonda completa; causa confirmada.
- [x] Chave vazia em `.yaml`/`.yml` produz erro.
- [x] `.bib` continua a funcionar sem regressão.
- [x] `.yaml`/`.yml` válidos sem regressão.
- [x] Sem regressão em `cargo test --workspace`.
- [x] `crystalline-lint .` limpo.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p646.md`.

---

## 6. Hash do commit

`ef3ca21a9`
