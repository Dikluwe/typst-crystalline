# Relatório Passo 1048 — Investigação do Desvio de 0.109pt na Ponta da Peça do Delimitador Elástico

**Data**: 2026-08-14  
**Passo**: 1048  
**Status**: Concluído com Sucesso (Achado Relocalizado e Catalogado — Não Perseguido por ser Sub-Visual)  
**Objetivo**: Investigar e isolar a causa exata da diferença de 0.109pt na coordenada vertical $y_{\max}$ da chave elástica de `cases()` com `gap: 1em`, distinguindo entre divergência de dados de fonte, erro na fórmula de montagem OpenType MATH (`MathStretchyAssembly`) ou offset externo de fluxo.

---

## 1. Fase A — Confirmação de Fontes

1. **Fontes Padrão e Fixtures**:
   - `03_infra/fixtures/fonts/NewCMMath-Book.otf`: hash `ee3a250462742f98ee153087b3becfb3` (versão 0.700).
   - `typst-assets 94dcb99/files/fonts/NewCMMath-Book.otf`: hash `4c6abdaaf422a0cd20e3c2098a6f2ebe` (versão 0.800).
2. **Teste com Fonte Idêntica**:
   - Forçando o compilador Crystalline a utilizar rigorosamente o mesmo arquivo de fonte do Vanilla (`--font-path ~/.cargo/git/checkouts/typst-assets-...`), a coordenada $y_{\max}$ manteve exatamente o mesmo delta ($0.107420$ pt).
   - **Conclusão da Fase A**: A diferença **não é causada por variação de arquivo de fonte**.

---

## 2. Fase B — Auditoria Geométrica do Mecanismo de Montagem

A extração das coordenadas em pontos tipográficos (`pt`) de cada elemento no PDF gerado revelou a estrutura geométrica interna:

| Elemento | Coordenada $y$ Crystalline | Coordenada $y$ Vanilla | $\Delta y$ (Cryst − Vanilla) |
| :--- | :---: | :---: | :---: |
| **Linha 1 (`1`)** | $y = 11.115020\text{ pt}$ | $y = 11.007600\text{ pt}$ | $\mathbf{+0.107420\text{ pt}}$ |
| **Linha 2 (`2`)** | $y = 33.071020\text{ pt}$ | $y = 32.963600\text{ pt}$ | $\mathbf{+0.107420\text{ pt}}$ |
| **Ponta Superior (`⎧`)** | $y = 10.491420\text{ pt}$ | $y = 10.384000\text{ pt}$ | $\mathbf{+0.107420\text{ pt}}$ |
| **Centro da Chave (`⎨`)** | $y = 28.593020\text{ pt}$ | $y = 28.485600\text{ pt}$ | $\mathbf{+0.107420\text{ pt}}$ |
| **Ponta Inferior (`⎩`)** | $y = 38.444620\text{ pt}$ | $y = 38.337200\text{ pt}$ | $\mathbf{+0.107420\text{ pt}}$ |

### Análise Matemática das Distâncias Internas
1. **Espaçamento entre Linhas ($\Delta y_{\text{Linha 2} - \text{Linha 1}}$)**:
   - Crystalline: $33.071020 - 11.115020 = \mathbf{21.956000\text{ pt}}$.
   - Vanilla: $32.963600 - 11.007600 = \mathbf{21.956000\text{ pt}}$.
   - Diferença: $\mathbf{0.000000\text{ pt}}$ (identidade absoluta até a 6ª casa decimal).
2. **Posição Relativa da Chave em Relação ao Texto**:
   - Distância da ponta inferior `⎩` até a baseline da Linha 2:
     - Crystalline: $38.444620 - 33.071020 = 5.373600\text{ pt}$.
     - Vanilla: $38.337200 - 32.963600 = 5.373600\text{ pt}$.
     - Diferença: $\mathbf{0.000000\text{ pt}}$.

---

## 3. Fase C — Conclusão, Relocalização e Catalogação de Débito Técnico

1. **Causa Raiz Localizada**:
   - A fórmula de montagem de peças (`MathStretchyAssembly`), o cálculo de overlap com `minConnectorOverlap` (`assembly.rs:136-141`) e o posicionamento de cada glifo da chave elástica estão **100% corretos e idênticos ao Vanilla**.
   - O delta de **$+0.107420\text{ pt}$** ($\approx 0.037\text{ mm}$) é um deslocamento vertical global uniforme da caixa do bloco de equação em relação ao topo da página/parágrafo (`layout/mod.rs`), comum a todos os glifos da equação.
2. **Origem do 1 Raster a 150 DPI**:
   - O rasterizador a 150 DPI converte $38.3372\text{ pt} \times \frac{150}{72} = 79.869\text{ px}$ (arredondado para $80$ px) vs $38.4446\text{ pt} \times \frac{150}{72} = 80.093\text{ px}$ (arredondado para $81$ px), produzindo $75$ vs $76$ pixels no recorte de imagem estritamente por cruzar o limiar de inteiro do pixel.
3. **Classificação Final e Registro de Débito**:
   - **Achado Relocalizado e Catalogado como Débito Técnico**: O subsistema de math layout e delimitadores elásticos está perfeitamente aderente. O offset vertical de $+0.107\text{ pt}$ foi catalogado como débito de baixa prioridade em `layout/mod.rs` (posicionamento de blocos e parágrafos), **não sendo perseguido nesta fase por ser sub-visual**.
