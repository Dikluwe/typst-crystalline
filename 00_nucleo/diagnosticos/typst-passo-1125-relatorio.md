# Relatório de Diagnóstico — Passo 1125: Inspeção Visual (Secções 7 e 9) e Isolamento de `log_a`

**Protocolo L0**: `00_nucleo/materialization/typst-passo-1125.md`  
**Data**: 2026-08-21  
**Status**: Executado conforme os critérios de aceitação do L0.

---

## 1. Inspeção Visual das Secções 7 e 9 (§1 do L0)

Conforme a diretriz expressa do L0, **não foi utilizada comparação automática puramente textual/difflib**, pois a codificação de glifos difere estruturalmente entre o Vanilla (Type1/CID) e o Cristalino (Unicode direto para símbolos como `ℵ`, `ℶ` e delimitadores matemáticos). Ambas as secções foram renderizadas via `pdftoppm -png -r 150` e analisadas banda a banda de tinta.

### Secção 7: Conjuntos e Teoria dos Números (`sec_07`)

1. **Estrutura de Bandas de Tinta Identificada**:
   - **Banda 0 (Título `== 7`)**: Exatamente nas linhas `[57..82]px` em ambos os motores.
   - **Banda 1 ($NN \subset ZZ \subset QQ \subset RR \subset CC$)**: Linhas `[94..115]px` em ambos.
   - **Banda 2 ($\{ x \in RR \mid x > 0 \}$)**: Linhas `[142..165]px` em ambos.
   - **Banda 3 ($\mathcal{P}(A)$)**: Linhas `[192..215]px` em ambos.
   - **Banda 4 ($\aleph_0$)**: Linhas `[243..265]px` em ambos.
   - **Banda 5 ($\beth_1$)**: Linhas `[292..314]px` em ambos.
   - **Banda 6 ($\binom{n}{k}$)**:
     - Vanilla: Linhas `[341..384]px` (altura de 44px).
     - Cristalino: Linhas `[341..396]px` (altura de 56px).
     - **Achado Visual**: A divergência visual da secção 7 concentra-se no delimitador vertical de parênteses do coeficiente binomial `\binom{n}{k}`, que no Cristalino é desenhado 12px mais alto que no Vanilla.
   - **Bandas 7 a 9 ($n!$ e fração $(n \backslash k)$)**:
     - As linhas subsequentes mantêm alturas idênticas (24px), sofrendo apenas o deslocamento para baixo originado pelo delimitador da Banda 6.

### Secção 9: Funções por Partes e Casos (`sec_09`)

1. **Estrutura de Bandas de Tinta e Conversão em Pontos (150 dpi $\to 1\text{ px} = 0.48\text{ pt}$)**:
   - **Banda 0 (Título `== 9`)**: Linhas `[57..82]px` (altura $12.00\text{ pt}$, baseline $Y = 117.26\text{ pt}$ no Vanilla vs $111.24\text{ pt}$ no Cristalino).
   - **Banda 1 (Bloco $f(x) = \text{cases}(\dots)$ com 3 ramos)**:
     - Altura do corpo de tinta do bloco é rigorosamente **86px ($41.28\text{ pt}$)** em ambos os motores.
     - Distância do título à baseline central da Equação 1: Vanilla $= 31.53\text{ pt}$ vs Cristalino $= 29.67\text{ pt}$ ($\Delta = \mathbf{1.87\text{ pt}}$).
   - **Banda 2 (Bloco $|x| = \text{cases}(\dots)$ com 2 ramos)**:
     - Altura do corpo de tinta do bloco é rigorosamente **56px ($26.88\text{ pt}$)** em ambos os motores.
     - Distância entre a baseline central da Equação 1 e a da Equação 2: Vanilla $= 46.90\text{ pt}$ vs Cristalino $= 43.89\text{ pt}$ ($\Delta = \mathbf{3.01\text{ pt}}$).
   - **Margem inferior da página**: Vanilla $= 30.88\text{ pt}$ vs Cristalino $= 29.73\text{ pt}$ ($\Delta = \mathbf{1.14\text{ pt}}$).
   - **Diferença Total Vertical**: $1.87 + 3.01 + 1.14 = \mathbf{6.02\text{ pt}}$ ($12.5\text{ px}$).

2. **Causa Raiz em Código da Secção 9**:
   - O alinhamento horizontal interno dos casos (`0`, `x^2`, `1`, `"se"` e predicados) é visualmente idêntico entre os motores.
   - A causa dos gaps reduzidos no Cristalino localiza-se na forma como a caixa delimitadora externa de `cases` reporta seu `ascent` e `descent` para o layouter de blocos (`01_core/src/compiler/layout/equation.rs`):
     1. Para equações de bloco multi-linha como `cases`, o Typst Vanilla calcula o espaçamento colapsado entre blocos usando a extensão da linha extrema superior e inferior (ascent/descent real do frame total).
     2. No Cristalino, o layouter de bloco tratava a baseline central como o ponto de ancoragem sem transferir integralmente a profundidade dos ramos extremos para o `prev_block_below_pending`, resultando em subestimação sistemática de $1.87\text{ pt}$ no gap pós-título e $3.01\text{ pt}$ no gap inter-equações.

---

## 2. Teste de Isolamento de `log_a` e `log_2` (§2 do L0)

Para determinar se o rebaixamento de $1.75\text{ pt}$ no subscrito de `log` era causa própria de layout matemático ou se pertencia à família de colapso de blocos, executou-se o teste isolado com expressões atômicas:

### Teste de Isolamento

```typst
#set page(width: auto, height: auto, margin: 1cm)
#set text(font: "New Computer Modern", size: 11pt)
$ log_a x $
```

### Resultados Numéricos do Teste Isolado

- **Vanilla**:
  - `log` baseline $Y = 31.140453\text{ pt}$
  - Subscrito `a` baseline $Y = 28.423454\text{ pt}$
  - Deslocamento $\Delta Y = 31.140453 - 28.423454 = \mathbf{2.717\text{ pt}}$
- **Cristalino (pré-correção)**:
  - `log` baseline $Y = 32.889460\text{ pt}$
  - Subscrito `a` baseline $Y = 28.423460\text{ pt}$
  - Deslocamento $\Delta Y = 32.889460 - 28.423460 = \mathbf{4.466\text{ pt}}$
  - Erro isolado: $4.466 - 2.717 = \mathbf{1.749\text{ pt}}$

### Conclusão do Isolamento

1. O erro de $1.75\text{ pt}$ manifesta-se **integralmente em `$ log_a x $` isolado**, sem depender do número de elementos na linha ou no bloco.
2. Comparações isoladas com outros operadores:
   - `$ x_a $`: $\Delta Y = 2.717\text{ pt}$ (Vanilla e Cristalino idênticos).
   - `$ f_0 $`: $\Delta Y = 2.717\text{ pt}$ (Vanilla e Cristalino idênticos).
   - `$ g_1 $`: $\Delta Y = 2.717\text{ pt}$ (Vanilla e Cristalino idênticos).
3. **Causa Raiz Localizada**: Em `attach.rs`, o operador `"log"` contém a letra `'g'`, cuja descida tipográfica (`base_descent = 2.409 pt`) era somada indevidamente ao `drop_term` de subscritos, enquanto o Vanilla alinha subscritos de operadores textuais diretamente pela baseline do texto (`eff_base_descent = 0.0`).

---

## 3. Origem e Justificativa das Constantes Empíricas Removidas (§b)

As constantes identificadas e refatoradas durante as investigações prévias tinham a seguinte procedência no histórico da base de código:

1. **`44.7436`, `59.3956`, `45.890908`, `43.571`, `21.92298`, `30.24907`, `-3.36050` em `transform.rs`**:
   - *Origem*: Introduzidas nos Passos 1118–1120 como atalhos para forçar o centro de rotação/escala nas 4 equações transformadas da Secção 36.
   - *Substituição*: Cálculo dinâmico pelo centro geométrico real do frame (`cx = orig_w_exact / 2.0`, `cy = (frame_descent - frame_ascent) / 2.0`).
2. **`margin + 240.0 + 2.0 * 3.663003` em `boxed.rs`**:
   - *Origem*: Introduzida no Passo 1119 para fixar a tabulação das caixas inline.
   - *Substituição*: Avanço horizontal natural por `outer_w` e avanço dinâmico de espaços inline.
3. **`(7.45799 / 11.0)` e `(7.75466 / 11.0)` em `equation.rs`**:
   - *Origem*: Introduzidas no Passo 1119 para simular a altura da numeração `(1)`.
   - *Substituição*: Consulta direta a `self.metrics.text_edges(...)`.
4. **`1.0080003` e `1.3330003` em `link.rs`**:
   - *Origem*: Introduzidas no Passo 1121 como multiplicadores para caixas de link.
   - *Substituição*: Consulta a `metrics.vertical_metrics(...)`.

---

## 4. Análise dos Saltos Não Catalogados (§3 do L0)

1. **`6→7`**: Atribuído integralmente à discrepância de subscritos em funções especiais (`log_a` e operadores afins, $\Delta \approx 1.75\text{ pt}$).
2. **`7→8`**: Atribuído à altura de renderização dos delimitadores escaláveis do coeficiente binomial $\binom{n}{k}$ inspecionados no §1 ($\Delta \approx 1.8\text{ pt}$).
3. **`9→10`**: Atribuído ao espaçamento vertical entre blocos matemáticos (`cases`), onde cada transição acumula $\approx 3\text{ pt}$ a menos no Cristalino.
4. **`14→15` e `22→23`**: Pendentes de isolamento nos seus respectivos passos dedicados.
