---
# P694 — Implementar o módulo `sys`

> **Passo:** 694
> **Data:** 2026-07-10
> **Foco:** P689 confirmou que `sys` (módulo builtin com `sys.version`, `sys.inputs`, etc.) está ausente do scope global do cristalino, bloqueando o `import` de `oxifmt` (dependência de `cetz`) logo nas primeiras linhas (`sys.version >= version(0,11,0)`). Este é o próximo bloqueio real confirmado da cadeia de validação de pacotes.
> **Tipo:** Sonda mínima + Implementação.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P689 (onde `sys` foi confirmado em falta), P678 (mapa geral de pacotes), toda a cadeia de correcções de `str`/`version` já feita (P684, P689-693), reaproveitável já que `sys.version` é do tipo `version`.

---

## Sonda mínima

### Confirmar todos os campos de `sys` que o vanilla expõe

```bash
cat > /tmp/p694-sys.typ <<'EOF'
#sys.version
#type(sys.version)
#sys.inputs
#type(sys.inputs)
EOF
lab/typst-original/target/release/typst compile /tmp/p694-sys.typ /tmp/p694-vanilla.pdf
pdftotext /tmp/p694-vanilla.pdf -
```

Confirmar a lista completa de campos de `sys` (documentação oficial, não só os dois óbvios) — pode haver mais além de `version`/`inputs`.

```bash
web_search "typst sys module reference version inputs"
```

### Confirmar como `--input` é passado ao compilador, e se o cristalino já tem esse mecanismo de CLI

```bash
lab/typst-original/target/release/typst compile --input chave=valor /tmp/p694-sys.typ /tmp/p694-input.pdf
```

Confirmar se o cristalino já aceita `--input` na CLI (mesmo que `sys.inputs` não esteja ligado ainda) — se não aceitar, isso é parte do trabalho deste passo também.

### Critério de fecho da sonda mínima

- [ ] Lista completa de campos de `sys` confirmada via documentação.
- [ ] Comportamento de `sys.version` confirmado (deve ser um valor `version`, reaproveitando o trabalho já feito).
- [ ] Comportamento de `sys.inputs` e da flag `--input` confirmados.

---

## Implementação

Adicionar o módulo `sys` ao scope global, com `sys.version` (o valor `version` correspondente à versão do cristalino — decidir que valor usar, dado que o cristalino não é literalmente uma versão do Typst oficial; provavelmente reportar a versão do Typst com que tem paridade, com nota clara) e `sys.inputs` (dicionário populado a partir de `--input` da CLI, vazio por defeito).

### Critério de fecho da implementação

- [ ] `sys.version` acessível, do tipo `version`.
- [ ] `sys.inputs` acessível, populável via `--input` na CLI.
- [ ] Outros campos confirmados pela sonda, implementados.

---

## Validação

```bash
./target/release/typst /tmp/p694-sys.typ /tmp/p694-depois.pdf
pdftotext /tmp/p694-depois.pdf -
```

### Confirmar que `oxifmt` avança

```bash
cat > /tmp/p694-oxifmt.typ <<'EOF'
#import "@preview/oxifmt:1.0.0"
EOF
./target/release/typst /tmp/p694-oxifmt.typ /tmp/p694-oxifmt.pdf
echo "Exit code: $?"
```

### Confirmar `cetz` de novo, com o documento e padrão correctos de P688

```bash
cat > /tmp/p694-cetz.typ <<'EOF'
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
EOF
./target/release/typst /tmp/p694-cetz.typ /tmp/p694-cetz.pdf
echo "Exit code: $?"
mutool draw -o /tmp/p694-cetz.png -r 150 /tmp/p694-cetz.pdf 2>/dev/null
```

Se produzir PDF: comparar visualmente com a imagem do vanilla já descrita em P688. Se falhar: registar o próximo bloqueio com honestidade — P688 já apontou o plugin WASM (`cetz_core.wasm`) como candidato provável, ainda não confirmado.

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Sonda mínima completa, lista de campos de `sys` confirmada.
- [ ] `sys.version` e `sys.inputs` implementados e testados.
- [ ] Import de `oxifmt` confirmado a funcionar.
- [ ] `cetz` re-testado com o documento correcto — PDF completo comparado visualmente, ou próximo bloqueio registado.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p694.md`, com hash do commit.
