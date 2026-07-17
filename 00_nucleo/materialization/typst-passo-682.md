---
# P682 — `version()` deve aceitar um array, além de três posicionais

> **Passo:** 682
> **Data:** 2026-07-10
> **Foco:** P681 encontrou que `cetz` (um pacote popular) falha logo na primeira linha porque chama `version((0,2,2))` com um único argumento array, mas o `version()` do cristalino exige três posicionais separados (`version(0, 2, 2)`). O vanilla aceita as duas formas. Isto bloqueia qualquer pacote real que use este padrão comum, incluindo o próprio exemplo escolhido para validar pacotes em P678/P681.
> **Tipo:** Sonda mínima + Implementação.
> **Tamanho:** S.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P681 (onde o problema foi encontrado, dentro do código de `cetz`), a linha P662-P664 (mesmo tipo de divergência de linguagem).

---

## Sonda mínima

### Confirmar todas as formas que o vanilla aceita

```bash
cat > /tmp/p682-version.typ <<'EOF'
#version(0, 2, 2)
#version((0, 2, 2))
#version(0, 2, 2, "beta")
#version((0, 2, 2, "beta"))
EOF
lab/typst-original/target/release/typst compile /tmp/p682-version.typ /tmp/p682-vanilla.pdf
pdftotext /tmp/p682-vanilla.pdf -
```

Confirmar exactamente que formas são aceites, e se o resultado (o objecto `version` produzido) é idêntico entre a forma posicional e a forma de array.

### Confirmar o estado actual do cristalino

```bash
./target/release/typst /tmp/p682-version.typ /tmp/p682-cristalino.pdf
echo "Exit code: $?"
```

### Critério de fecho da sonda mínima

- [ ] Todas as formas aceites pelo vanilla confirmadas.
- [ ] Confirmado se a forma de array e a forma posicional produzem o mesmo resultado.

---

## Implementação

Estender `version()` no cristalino para aceitar, além dos três (ou quatro, com `pre` opcional) argumentos posicionais já suportados, um único argumento array com os mesmos componentes.

### Critério de fecho da implementação

- [ ] `version((0, 2, 2))` funciona, produzindo o mesmo resultado que `version(0, 2, 2)`.
- [ ] `version((0, 2, 2, "beta"))` funciona, se confirmado pela sonda que o vanilla aceita este caso também.
- [ ] Forma posicional original sem regressão.
- [ ] Erros de aridade continuam claros para chamadas genuinamente inválidas (por exemplo, `version(0, 2)`, faltando um componente).

---

## Validação

```bash
./target/release/typst /tmp/p682-version.typ /tmp/p682-depois.pdf
pdftotext /tmp/p682-depois.pdf -
```

Comparar com o resultado do vanilla já obtido na sonda.

### Confirmar que `cetz` agora avança para lá da primeira linha

```bash
cat > /tmp/p682-cetz.typ <<'EOF'
#import "@preview/cetz:0.2.2": canvas, draw
#canvas({
  draw.line((0,0), (1,1))
})
EOF
./target/release/typst /tmp/p682-cetz.typ /tmp/p682-cetz.pdf
echo "Exit code: $?"
```

Confirmar se `cetz` agora funciona por completo, ou se avança para lá da linha 1 e encontra outro problema de linguagem mais adiante — se for o segundo caso, registar o próximo problema encontrado, não assumir que está tudo resolvido só porque a primeira falha desapareceu.

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Sonda mínima completa.
- [ ] `version()` aceita a forma de array, testada contra o vanilla.
- [ ] Forma posicional sem regressão.
- [ ] `cetz` testado de novo — funciona por completo, ou o próximo problema encontrado fica registado.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p682.md`, com hash do commit.
