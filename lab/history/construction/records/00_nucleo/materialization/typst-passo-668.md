---
# P668 — Caminho single-font nunca instancia fontes variáveis

> **Passo:** 668
> **Data:** 2026-07-10
> **Foco:** P667 encontrou, e classificou como "fora do âmbito", que `build_cidfont` (o caminho de export usado quando o documento inteiro resolve numa única fonte) nunca instancia fontes variáveis, mesmo com Python/fontTools disponíveis e a funcionar. Um documento com um só peso não-default de uma fonte variável — o caso mais comum e mais simples de todos — continua a produzir contornos da instância default, em silêncio, sem erro nem aviso. É o bug original de P525, ainda presente no caminho mais provável de ser usado.
> **Tipo:** Sonda + Implementação. Prioridade alta.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P525 (bug original), P530 (correcção parcial, caminho multi-font), P666 (confirmação de que P525 estava fechado, sem ter testado o caminho single-font), P667 (onde este caso foi encontrado e adiado).

---

## Sonda

### Confirmar o alcance exacto do problema

```bash
export TYPST_CRYSTALLINE_PYTHON=lab/.venv/bin/python
cat > /tmp/p668-single-weight.typ <<'EOF'
#set text(font: "Ubuntu Sans", weight: 700, size: 40pt)
Só este peso, nada mais no documento.
EOF
./target/release/typst /tmp/p668-single-weight.typ /tmp/p668.pdf
pdffonts /tmp/p668.pdf
mutool draw -o /tmp/p668.png -r 150 /tmp/p668.pdf
```

Confirmar visualmente: o texto aparece em negrito (700) ou em regular (400), apesar do `weight: 700` pedido?

### Confirmar exactamente quando `build_cidfont` é usado em vez do caminho multi-font

```bash
grep -n "fn build_cidfont\|fn build_multifont\|build_cidfont(\|build_multifont(" 03_infra/src/export/builder.rs 03_infra/src/pipeline.rs | head -20
```

Confirmar a condição exacta que decide qual caminho é usado — provavelmente "documento resolve numa única fonte" — e confirmar se essa condição é comum (a maioria dos documentos simples cairia aqui) ou rara.

### Confirmar se a verificação de P667 (erro quando Python indisponível) também falha em avisar aqui

```bash
unset TYPST_CRYSTALLINE_PYTHON
./target/release/typst /tmp/p668-single-weight.typ /tmp/p668-sem-python.pdf
echo "Exit code: $?"
```

Se P667 só verificou o caminho multi-font, este teste pode continuar a passar silenciosamente mesmo sem Python — confirmar.

### Critério de fecho da sonda

- [ ] Confirmado visualmente que o caminho single-font produz contornos errados.
- [ ] Confirmado que esta é a condição mais comum, não um caso raro.
- [ ] Confirmado se a verificação de P667 (erro sem Python) também se aplica aqui, ou se este caminho continua a falhar em silêncio mesmo sem Python.

---

## Implementação

Aplicar a mesma instanciação já correcta no caminho multi-font (`03_infra/src/export/builder.rs:644`, confirmado por P666) ao caminho single-font (`build_cidfont`). Aplicar também a verificação de P667 (erro claro quando Python/fontTools indisponível) a este caminho.

### Critério de fecho da implementação

- [ ] `build_cidfont` instancia a fonte variável correctamente quando o peso/estilo não é default.
- [ ] Testado com o documento de peso único, confirmando visualmente o peso correcto.
- [ ] Verificação de P667 (erro sem Python) também se aplica a este caminho.
- [ ] Caminho multi-font (já correcto) sem regressão.

---

## Validação

```bash
export TYPST_CRYSTALLINE_PYTHON=lab/.venv/bin/python
./target/release/typst /tmp/p668-single-weight.typ /tmp/p668-depois.pdf
pdffonts /tmp/p668-depois.pdf
mutool draw -o /tmp/p668-depois.png -r 150 /tmp/p668-depois.pdf
```

Confirmar visualmente o peso correcto agora.

```bash
unset TYPST_CRYSTALLINE_PYTHON
./target/release/typst /tmp/p668-single-weight.typ /tmp/p668-erro.pdf
echo "Exit code: $?"
```

Confirmar erro claro, não PDF errado silencioso.

```bash
cargo test --workspace
crystalline-lint .
```

Repetir os testes de P666/P667 no caminho multi-font, confirmando sem regressão.

---

## Critério de fecho do passo

- [ ] Sonda completa, alcance confirmado.
- [ ] `build_cidfont` corrigido para instanciar correctamente.
- [ ] Verificação de Python/fontTools aplicada também a este caminho.
- [ ] Testado visualmente, peso correcto confirmado.
- [ ] Caminho multi-font sem regressão.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p668.md`, com hash do commit.
- [ ] Pendência de P525 — agora fica mesmo fechada, nos dois caminhos, não só num.
