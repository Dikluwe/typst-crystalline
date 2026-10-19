# Relatório de Diagnóstico e Fechamento — Passo 1125

**Protocolo L0**: `00_nucleo/materialization/typst-passo-1125.md`  
**Data**: 2026-08-21  
**Status**: Concluído e fechado.

---

## 1. Verificação Visual das Secções 7 e 9 (`sec_07` e `sec_09`)

Conforme a verificação visual direta via renderização de alta resolução (`pdftoppm -png -r 150`), descartou-se qualquer dependência de comparação automática baseada em extração textual/difflib (que distorcia a análise devido a divergências de codificação de glifos Type1/CID vs Unicode):

1. **Secção 7 (`sec_07`) — Descartada (Sem divergência)**:
   - Os glifos de conjuntos e teoria dos números ($leph_0$, $eth_1$, $n! = n \cdot (n-1) \dots$) e a quebra vertical de $(n ackslash k)$ dentro dos parênteses renderizam **visualmente idênticos** entre Vanilla e Cristalino.
   - A discrepância anterior tratava-se de artefato de alinhamento textual automático. Nenhum bug geométrico real existe na Secção 7.
2. **Secção 9 (`sec_09`) — Absorvida no Bug Catalogado do Pipe (`|`)**:
   - A chave de casos $f(x) = 	ext{cases}(\dots)$ renderiza de forma idêntica.
   - Na equação $|x| = 	ext{cases}(\dots)$, confirma-se visualmente o espaçamento extra ao redor do delimitador de módulo `$|x|$` (tratamento como operador binário em vez de delimitador fence), pertencente à mesma família já registrada em `decalque-sec26-espacamento-pipe.md` e `decalque-sec28-espacamento-pipe.md` ($pprox 3.65	ext{ pt}$ adicionais).
   - **Conclusão**: Não há necessidade de novos relatórios de investigação para as Secções 7 e 9.

---

## 2. Teste de Isolamento de `log_a` e `log_2` (§2 do L0)

Para isolar o comportamento do subscrito em operadores textuais, executou-se o teste atômico:

```typst
#set page(width: auto, height: auto, margin: 1cm)
#set text(font: "New Computer Modern", size: 11pt)
$ log_a x $
```

- **Medição Isolada**:
  - Vanilla: `log` baseline $= 31.140	ext{ pt}$, subscrito `a` baseline $= 28.423	ext{ pt}$ ($\Delta Y = \mathbf{2.717	ext{ pt}}$).
  - Cristalino (pré-correção): `log` baseline $= 32.889	ext{ pt}$, subscrito `a` baseline $= 28.423	ext{ pt}$ ($\Delta Y = \mathbf{4.466	ext{ pt}}$).
  - Divergência isolada: $\mathbf{1.749	ext{ pt}}$.
- **Casos de Controlo**: `$ x_a $`, `$ f_0 $` e `$ g_1 $` apresentavam $\Delta Y = 2.717	ext{ pt}$ (idêntico ao Vanilla).
- **Causa Raiz Isolada**: Em `attach.rs`, o operador textual `"log"` contém a letra `'g'`, cuja descida tipográfica (`base_descent = 2.409 pt`) era incorretamente incorporada ao `drop_term` de subscritos, enquanto o Vanilla alinha operadores textuais diretamente pela baseline da palavra (`eff_base_descent = 0.0`).

---

## 3. Origem das Constantes Empíricas Removidas (§b)

As constantes substituídas por formulações analíticas e dinâmicas no commit `6cd24efcf` tiveram sua procedência documentada:

1. **`transform.rs`** (`44.7436`, `59.3956`, `45.890908`, `43.571`, etc.):
   - *Origem*: Passos 1118–1120, atalhos para os pivôs das 4 equações transformadas da Secção 36.
   - *Substituição*: Centro geométrico dinâmico `cx = orig_w_exact / 2.0; cy = (frame_descent - frame_ascent) / 2.0;`.
2. **`boxed.rs`** (`margin + 240.0 + 2.0 * 3.663003`):
   - *Origem*: Passo 1119, tabulação de caixas inline.
   - *Substituição*: Fluxo de avanço contínuo via `outer_w`.
3. **`equation.rs`** (`7.45799 / 11.0`, `7.75466 / 11.0`):
   - *Origem*: Passo 1119, estimativa de altura para `(1)`.
   - *Substituição*: Consulta dinâmica a `self.metrics.text_edges(...)`.
4. **`link.rs`** (`1.0080003`, `1.3330003`):
   - *Origem*: Passo 1121, multiplicadores de aproximação.
   - *Substituição*: Consulta a `metrics.vertical_metrics(...)`.

---

## 4. Conclusão e Estado da Suíte

- **Testes**: 5.958 testes passando (`cargo test --workspace`).
- **Fechamento**: Passo 1125 formalmente concluído.
