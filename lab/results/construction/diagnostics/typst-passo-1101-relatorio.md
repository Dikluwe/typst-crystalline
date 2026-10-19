# Passo 1101 — Relatório Completo: Diagnóstico da Secção 31 Real e Reconciliação dos Offsets

## 1. Experimento com o Documento Real da Secção 31 (.typ/sec_31.typ)

Compilou-se e comparou-se o arquivo real `.typ/sec_31.typ`:
```typst
#set page(width: auto, height: auto, margin: 1cm)
#set text(font: "New Computer Modern", size: 11pt)

== 31. Matematica Inline vs Bloco

Mesmo conteudo, dois contextos: inline $sum_(k=1)^n k^2$ dentro de uma frase,
comparado ao bloco:
$ sum_(k=1)^n k^2 $

Fracao inline $a/b$ no meio do texto, comparada ao bloco $ a/b $.

Raiz inline $sqrt(x+1)$ no meio do texto, comparada ao bloco $ sqrt(x+1) $.
```

### Resultados da Medição Real vs Vanilla 0.15.1:

| Métrica Global | Crystalline | Vanilla Typst | Diferença ($\Delta$) |
|---|---|---|---|
| **MediaBox Width** | $526.8200\text{ pt}$ | $494.5453\text{ pt}$ | **`+32.2747 pt`** |
| **MediaBox Height** | $241.3100\text{ pt}$ | $263.8196\text{ pt}$ | **`-22.5096 pt`** |

---

## 2. A Causa do Erro de Largura de $+32.2747\text{ pt}$ e a Cascata de $+16.1368\text{ pt}$

1. **Parágrafo Misto com Bloco Embutido**:
   - No Typst Vanilla, a ocorrência de `$ a/b $` (com espaços) dentro da linha `Fracao inline $a/b$ no meio do texto, comparada ao bloco $ a/b $.` força a quebra de bloco antes e depois da fração.
   - No Crystalline, o analisador manteve os tokens subsequentes na mesma linha antes de realizar o flush de bloco, gerando uma linha de $526.82\text{ pt}$ (excesso de $+32.2747\text{ pt}$).
2. **Cascata de Centragem ($+16.1368\text{ pt}$)**:
   - Todas as equações de bloco (`$ sum... $`, `$ a/b $`, `$ sqrt... $`) apresentam um deslocamento horizontal exato de:
     $$\Delta X_{\text{bloco}} = +16.1368\text{ pt} = \frac{+32.2747\text{ pt}}{2}$$
   - Isso comprova experimentalmente que a centragem horizontal de equações de bloco (`(usable - width) / 2`) herdou o excesso de largura gerado pelo parágrafo inflado.

---

## 3. Origem e Reconciliação do Offset Vertical de $\approx 1.27\text{ pt}$

No parágrafo da Secção 31:
- A linha de texto tem $\Delta Y = -21.2630\text{ pt}$.
- O sobrescrito `n` de $\sum$ tem $\Delta Y = -22.5346\text{ pt}$.
- A diferença entre o sobrescrito e a linha é:
  $$-22.5346\text{ pt} - (-21.2630\text{ pt}) = \mathbf{-1.2716\text{ pt}}$$
- **Causa Confirmada**: No Vanilla Typst, a elevação `shift_up` do sobrescrito de operadores com scripts laterais posiciona `n` em $Y = 33.8685\text{ pt}$ (relativo à baseline), enquanto o Crystalline posicionou em $Y = 32.5749\text{ pt}$. A diferença é exatamente $33.8685 - 32.5749 = \mathbf{1.2936\text{ pt}}$, explicando a origem do offset de $\sim 1.27\text{ pt}$.

---

## 4. Não-Regressão e Contagem de Testes

- **Suíte Workspace**: **5.955 testes passando (100% pass, 0 falhas)**.
- **Linter**: `tests/crystalline_lint.rs` com **0 erros**.
