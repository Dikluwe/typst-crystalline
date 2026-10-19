---
# P759 — Sonda: vale a pena mudar de layout greedy para Knuth-Plass?

> **Passo:** 759
> **Data:** 2026-07-14
> **Foco:** P756/P758 confirmaram que o layout greedy do cristalino produz quebras de linha diferentes do Knuth-Plass do vanilla, em qualquer texto (não só CJK — greedy vs óptimo é uma diferença fundamental de algoritmo, presente em todo o documento). Antes de decidir implementar Knuth-Plass, esta sonda mapeia o alcance real da mudança e avalia se "greedy, mas com resultado visualmente aceitável" é uma divergência mecânica aceitável (ADR-0107), ou se as diferenças são grandes o suficiente para justificar uma reescrita do motor de quebra de linha.
> **Tipo:** Sonda directa, ampla. Sem implementação neste passo.
> **Tamanho:** M para a sonda; a implementação, se avançar, é provavelmente XL — o maior item de mudança de mecanismo central desta conversa.
> **ADR-0108 EM VIGOR.** **ADR-0114 EM VIGOR** — mudança no motor de layout central, usado por todo e qualquer texto, não só CJK.
> **Dependências:** P756/P758 (onde a diferença de algoritmo foi confirmada como a causa residual).

---

## Sonda

### Confirmar o mecanismo exacto do Knuth-Plass no vanilla

```bash
grep -rn "fn.*knuth\|fn.*optimal.*break\|struct.*Breakpoint" lab/typst-original/crates/typst-layout/src/inline/*.rs 2>/dev/null | head -30
```

Confirmar a implementação real — é Knuth-Plass completo (com "badness"/"demerits", múltiplas passagens, considera parágrafo inteiro), ou uma variante mais simples?

### Medir a magnitude real da diferença em texto latino comum, não só CJK

```bash
cat > /tmp/p759-latim.typ <<'EOF'
#set page(width: 300pt, margin: 20pt)
Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.
EOF
lab/typst-original/target/release/typst compile /tmp/p759-latim.typ /tmp/p759-vanilla.pdf
pdftotext /tmp/p759-vanilla.pdf -
./target/release/typst /tmp/p759-latim.typ /tmp/p759-cristalino.pdf
pdftotext /tmp/p759-cristalino.pdf -
```

Comparar as quebras de linha, linha a linha — confirmar se a diferença é subtil (uma palavra a mais/menos ocasionalmente) ou substancial (parágrafos com forma visual muito diferente, "rios" de espaço branco desiguais).

### Medir com um documento maior, representativo de uso real

```bash
cat > /tmp/p759-grande.typ <<'EOF'
#set page(width: 350pt, margin: 40pt)
#lorem(200)
EOF
lab/typst-original/target/release/typst compile /tmp/p759-grande.typ /tmp/p759-grande-vanilla.pdf
./target/release/typst /tmp/p759-grande.typ /tmp/p759-grande-cristalino.pdf
mutool draw -o /tmp/p759-grande-vanilla.png -r 150 /tmp/p759-grande-vanilla.pdf
mutool draw -o /tmp/p759-grande-cristalino.png -r 150 /tmp/p759-grande-cristalino.pdf
python3 /tmp/pngdiff.py /tmp/p759-grande-vanilla.png /tmp/p759-grande-cristalino.png
```

Medir a percentagem de diferença de pixels num documento de texto corrido substancial — comparar com os valores residuais já vistos noutros contextos desta conversa (0,14% para `cetz`), para ter uma referência de escala.

### Confirmar o esforço de implementação

```bash
wc -l lab/typst-original/crates/typst-layout/src/inline/linebreak.rs
```

Estimar o tamanho do algoritmo no vanilla, como indicador do esforço envolvido em replicá-lo (não é garantia directa, mas dá uma ordem de grandeza).

### Critério de fecho da sonda

- [ ] Mecanismo exacto do Knuth-Plass do vanilla confirmado.
- [ ] Magnitude da diferença medida em texto latino comum, não só CJK.
- [ ] Diferença de pixels medida num documento representativo, comparável com outros valores já vistos nesta conversa.
- [ ] Esforço de implementação estimado.

---

## Decisão

Com base na magnitude medida:

- Se a diferença for pequena (comparável aos ~0,1-0,2% já aceites como anti-aliasing/mecânica noutros contextos): registar como divergência mecânica aceitável (ADR-0107), não implementar Knuth-Plass — o benefício não justificaria o esforço de reescrever o motor de layout central.
- Se a diferença for substancial (forma visual claramente diferente, não só posições de quebra ligeiramente distintas): propor a divisão do trabalho em passos menores, seguindo o padrão já usado para outras funcionalidades grandes desta conversa.

---

## Critério de fecho do passo

- [ ] Sonda completa, com medições reais, não estimativas.
- [ ] Magnitude da diferença registada com números concretos.
- [ ] Decisão tomada com base na medição — implementar ou aceitar como divergência mecânica.
- [ ] Se decidido implementar: proposta de divisão em passos menores.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p759.md`.
