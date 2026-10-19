# Relatório de Paridade de Produção — Passo 667

| Campo | Valor |
|-------|-------|
| Passo | P667 |
| Foco | Comportamento sem Python/fontTools para fontes variáveis |
| Data | 2026-07-10 |
| Autor | IA (Kimi Code CLI) sob direção do utilizador |
| Status | Corrigido — erro claro antes de gerar PDF errado |
| Commit de medição inicial | `c9cb5e73b7fa6d13095f659473353abc16434731` |

---

## Sonda

### Objetivo

P666 confirmou que a instanciação estática de fontes variáveis funciona quando `TYPST_CRYSTALLINE_PYTHON` aponta para um Python com `fontTools`. P667 verifica o que acontece quando essa dependência falta: produz-se um aviso claro, ou gera-se um PDF visualmente errado de forma silenciosa?

### Documento de teste

```bash
cat > /tmp/p667-vf-sem-python.typ <<'EOF'
#set text(font: "Ubuntu Sans", size: 40pt)
Hello world.

#set text(weight: 700)
Bold hello.
EOF
```

### Cenário 1 — sem `TYPST_CRYSTALLINE_PYTHON` e `python3` sem fontTools

```bash
unset TYPST_CRYSTALLINE_PYTHON
./target/release/typst /tmp/p667-vf-sem-python.typ /tmp/p667.pdf 2>&1
echo "Exit code: $?"
```

Resultado **antes da correcção**:

- Exit code `0`.
- Aviso via `eprintln!`: `Aviso: falha ao instanciar fonte variável para ...; a usar instância default.`
- PDF gerado com duas fontes CID TrueType, mas o texto a `weight: 700` renderiza com contornos de Regular (regressão P525).

Resultado **após a correcção**:

```text
/tmp/p667-vf-sem-python.typ:<detached>: error: fonte variável 'ubuntu sans' requer instanciação, mas Python/fontTools não está disponível
Exit code: 1
```

Nenhum PDF é gerado; o utilizador recebe um erro claro.

### Cenário 2 — Python definido, mas sem `fontTools`

```bash
python3 -m venv /tmp/p667-no-fonttools
export TYPST_CRYSTALLINE_PYTHON=/tmp/p667-no-fonttools/bin/python
./target/release/typst /tmp/p667-vf-sem-python.typ /tmp/p667-b.pdf 2>&1
echo "Exit code: $?"
```

Resultado **antes da correcção**: mesmo comportamento do cenário 1 — aviso no stderr, PDF errado, exit 0.

Resultado **após a correcção**:

```text
/tmp/p667-vf-sem-python.typ:<detached>: error: fonte variável 'ubuntu sans' requer instanciação, mas Python/fontTools não está disponível
Exit code: 1
```

### Cenário 3 — fonte variável só com instância default

```bash
cat > /tmp/p667-default-only.typ <<'EOF'
#set text(font: "Ubuntu Sans", size: 40pt)
Default only.
EOF
unset TYPST_CRYSTALLINE_PYTHON
./target/release/typst /tmp/p667-default-only.typ /tmp/p667-default.pdf 2>&1
echo "Exit code: $?"
```

Resultado: exit 0, PDF gerado correctamente. Como não há variações de eixo não-default, não é necessário instanciar.

### Cenário 4 — com `fontTools` disponível

```bash
export TYPST_CRYSTALLINE_PYTHON=lab/.venv/bin/python
./target/release/typst /tmp/p667-vf-sem-python.typ /tmp/p667-ok.pdf 2>&1
echo "Exit code: $?"
```

Resultado: exit 0, PDF gerado com pesos visuais distintos.

---

## Implementação

### Ficheiros alterados

- `03_infra/src/font_variant.rs`
  - Adicionado `python_for_instancer()` — helper privado que resolve o interpretador Python a usar (`TYPST_CRYSTALLINE_PYTHON` ou `python3`).
  - Adicionado `variable_font_instancer_available()` — verifica se o Python configurado consegue importar `fontTools`.
  - `instantiate_variable_font` passa a usar `python_for_instancer()` para evitar duplicação.

- `03_infra/src/pipeline.rs`
  - Após `resolve_fonts`, antes de chamar o export, verifica-se se alguma fonte resolvida é variável **e** requer eixos não-default.
  - Se sim e `variable_font_instancer_available()` for `false`, devolve-se um `SourceDiagnostic::error` claro e não se gera o PDF.

### Porque erro em vez de aviso?

O PDF gerado sem instanciação tem contornos errados — é uma regressão de linguagem visível. Um aviso deixaria o utilizador com um artefacto incorrecto; um erro impede isso e indica imediatamente a causa (Python/fontTools em falta) e a solução (definir `TYPST_CRYSTALLINE_PYTHON` ou instalar fontTools).

---

## Nota sobre o caminho single-font

Durante a sonda verificou-se que, mesmo **com** Python/fontTools, o caminho single-font (`build_cidfont`, quando o documento resolve numa única fonte) **não instancia** a fonte variável. Isso significa que um documento com apenas `#set text(font: "Ubuntu Sans", weight: 700)` continua a renderizar contornos de Regular. Este problema está fora do âmbito de P667 (que é sobre a dependência Python), mas deve ser tratado num passo futuro.

---

## Sanity checks

| Check | Comando | Resultado |
|-------|---------|-----------|
| Testes workspace | `cargo test --workspace` | 606 passed; 0 failed; 5 ignored |
| Linter | `crystalline-lint .` | ✓ No violations found |

---

## Tabela final de classificação

| Item | Resultado |
|------|-----------|
| Comportamento sem `TYPST_CRYSTALLINE_PYTHON` confirmado | ✅ Erro claro |
| Comportamento com Python sem `fontTools` confirmado | ✅ Erro claro |
| Código de tratamento de erro revisto | ✅ `font_variant.rs` + `pipeline.rs` |
| Falha silenciosa corrigida | ✅ PDF errado já não é gerado |
| Sem regressão | ✅ 606 tests passed |
| Linter limpo | ✅ |
| Relatório com hash do commit | ✅ |

---

## Reprodução

```bash
# Criar venv sem fontTools
python3 -m venv /tmp/p667-no-fonttools

# Documento que usa VF com peso não-default
cat > /tmp/p667-vf-sem-python.typ <<'EOF'
#set text(font: "Ubuntu Sans", size: 40pt)
Hello world.

#set text(weight: 700)
Bold hello.
EOF

# Deve falhar com erro claro
export TYPST_CRYSTALLINE_PYTHON=/tmp/p667-no-fonttools/bin/python
./target/release/typst /tmp/p667-vf-sem-python.typ /tmp/p667.pdf

# Com fontTools deve funcionar
export TYPST_CRYSTALLINE_PYTHON=lab/.venv/bin/python
./target/release/typst /tmp/p667-vf-sem-python.typ /tmp/p667-ok.pdf

# Sanity checks
cargo test --workspace
crystalline-lint .
```

---

## Linhagem

- L0: `00_nucleo/prompts/infra/font_variant.md`, `00_nucleo/prompts/infra/pipeline.md`
- Código: `03_infra/src/font_variant.rs`, `03_infra/src/pipeline.rs`
- ADR-0107: paridade é com a linguagem — o PDF sem instanciação viola a semântica `weight:`.
- ADR-0108: medir antes de decidir; o comportamento foi medido nos dois cenários antes da correcção.
