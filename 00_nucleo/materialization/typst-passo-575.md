---
# P575 — O código órfão de L1 era mesmo a causa da diferença de linhas?

> **Passo:** 575
> **Data:** 2026-07-05
> **Foco:** P574 confirmou que P568 não muda a separação de palavras nem a posição do documento árabe de referência. Mas atribuiu a diferença de contagem de linhas (4 em P569, 2 agora) ao código órfão de L1 descartado em P572, sem testar isso directamente — só testou que P568 não era a causa, o que é diferente. Este passo faz o teste que falta, ou decide que não vale a pena, dado que o critério principal (palavras separadas) já está confirmado.
> **Tipo:** Verificação directa, pequena.
> **Tamanho:** XS.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P569 (relatório original com 4 linhas), P572 (onde o código órfão foi descartado), P574 (onde a atribuição foi feita sem teste directo).

---

## Verificação

```bash
git log --oneline | grep -i "P569\|P572" | tail -5
git checkout <commit-de-P569-antes-de-P572>
cargo build --release --bin typst
./target/release/typst /tmp/p574-longo.typ /tmp/p575-com-orfao.pdf
pdftotext /tmp/p575-com-orfao.pdf -
git checkout <commit-actual>
cargo build --release --bin typst
```

Confirmar se o estado com o código órfão presente produz mesmo 4 linhas, como o relatório de P569 registou.

---

## Critério de fecho

- [ ] Testado directamente com o código órfão presente.
- [ ] Confirmado se produz 4 linhas (confirma a atribuição de P574) ou outra coisa (a atribuição estava errada, e a causa real ainda não foi encontrada).
- [ ] Se confirmar: fechar sem mais acção, o critério principal já estava satisfeito.
- [ ] Se não confirmar: registar a causa real como pergunta em aberto, sem urgência, dado que não afecta o critério de palavras separadas.

---

## Relatório de execução

`00_nucleo/diagnosticos/paridade-producao-p575.md` — resultado do teste directo, curto.
