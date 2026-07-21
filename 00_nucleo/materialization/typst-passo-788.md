---
# P788 — Referências e citações inválidas devem errar, não falhar em silêncio

> **Passo:** 788
> **Data:** 2026-07-20
> **Foco:** P786 confirmou três divergências em `model::reference`: (1) `@ref` para um heading sem `numbering` definido é aceito com exit 0 no cristalino (vazio no fluxo), enquanto o vanilla erra (`error: cannot reference heading without numbering`, com hint sugerindo `#set heading(numbering: "1.")`); (2) `@label-inexistente` compila com exit 0, produzindo "?" solto no documento, enquanto o vanilla erra (`error: label \`<label>\` does not exist in the document`); (3) mesmo no caso feliz (referência válida), o texto renderizado aparece no fundo da página em vez de no fluxo normal do texto, e o suplemento de idioma diverge ("Secção" vs "Section", possivelmente questão de configuração de idioma, a confirmar).
> **Tipo:** Sonda + Implementação directa.
> **Tamanho:** M/L — três causas possivelmente distintas (validação de numbering, validação de label, posicionamento no fluxo).
> **ADR-0108 EM VIGOR** — confirmar cada uma das três causas separadamente, não assumir que são a mesma.
> **Prioridade:** Alta — duas das três divergências são "aceita em silêncio quando deveria errar".
> **Dependências:** P786 (achado, evidência em `temp/temp_p786/a_reference.typ`, `a_reference_num.typ`, `a_reference_cite.typ`).

---

## Sonda — mecanismo exato do vanilla

```bash
grep -n "cannot reference\|does not exist in the document\|fn.*reference" lab/typst-original/crates/typst-library/src/model/reference.rs 2>/dev/null
```

Confirmar as duas mensagens de erro exatas, incluindo hints.

```bash
cat > /tmp/p788-noheading.typ <<'EOF'
= Sem numeração
@sem-numeracao
EOF
lab/typst-original/target/release/typst compile /tmp/p788-noheading.typ 2>&1

cat > /tmp/p788-badlabel.typ <<'EOF'
Isto cita @naoexiste1984.
EOF
lab/typst-original/target/release/typst compile /tmp/p788-badlabel.typ 2>&1
```

### Investigar o posicionamento no fluxo (achado 3, causa possivelmente distinta)

```bash
grep -n "fn layout.*ref\|Content::Ref" 01_core/src/rules/layout/*.rs 2>/dev/null
```

Confirmar por que a referência renderiza fora do fluxo normal de texto — pode ser o mesmo tipo de bug já visto em P772x (mecanismo de `layout_sub_frame`/decoração), ou algo específico de `Content::Ref`. Não assumir a causa sem ler o código.

```bash
cat > /tmp/p788-position.typ <<'EOF'
= Título <sec1>
#set heading(numbering: "1.")
Ver @sec1 no texto.
EOF
lab/typst-original/target/release/typst compile /tmp/p788-position.typ /tmp/p788-vanilla.pdf
./target/release/typst compile /tmp/p788-position.typ /tmp/p788-cristalino.pdf
mutool trace /tmp/p788-vanilla.pdf > /tmp/p788-trace-vanilla.txt
mutool trace /tmp/p788-cristalino.pdf > /tmp/p788-trace-cristalino.txt
diff /tmp/p788-trace-vanilla.txt /tmp/p788-trace-cristalino.txt
```

Confirmar a posição Y exata da referência no cristalino vs vanilla.

---

## Decisão de âmbito

Se as três causas forem independentes: decidir se cabem todas neste passo (tamanho M/L já prevê isso) ou se o posicionamento no fluxo precisa de passo separado (pode ser maior, dado ter aparecido em outras áreas antes — P772x).

---

## Implementação

1. Validar `numbering` do heading referenciado antes de resolver `@ref`; erro com hint se ausente.
2. Validar existência do label antes de resolver `@ref`/citação; erro se não existir.
3. Corrigir o posicionamento da referência no fluxo de texto, conforme a causa confirmada pela sonda.

---

## Validação

```bash
./target/release/typst compile /tmp/p788-noheading.typ 2>&1
./target/release/typst compile /tmp/p788-badlabel.typ 2>&1
```

Confirmar erros idênticos ao vanilla nos dois casos.

```bash
./target/release/typst compile /tmp/p788-position.typ /tmp/p788-cristalino-depois.pdf
mutool trace /tmp/p788-cristalino-depois.pdf > /tmp/p788-trace-depois.txt
diff /tmp/p788-trace-vanilla.txt /tmp/p788-trace-depois.txt
```

Confirmar posição corrigida.

```bash
# Não regressão — caso feliz continua funcionando
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Mensagens de erro exatas do vanilla confirmadas para os dois casos de validação.
- [ ] Causa do posicionamento fora do fluxo identificada por leitura de código.
- [ ] Referência a heading sem numbering erra corretamente.
- [ ] Referência/citação a label inexistente erra corretamente.
- [ ] Posicionamento no fluxo corrigido (ou registrado como passo dedicado maior, se a causa revelar isso).
- [ ] Suplemento de idioma ("Secção" vs "Section") investigado — confirmar se é bug ou configuração esperada.
- [ ] `cargo test --workspace` verde.
- [ ] `crystalline-lint .` zero violações.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p788.md`.

---

## Próximo passo

Próximo candidato: conflito de célula com header de tabela (grupo 7 de P786 §5), ou show rule por string não aplicada em silêncio (grupo 4).
