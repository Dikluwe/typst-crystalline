---
# P745 — Confirmar rigorosamente se os 0,14% residuais de `cetz` são mesmo anti-aliasing

> **Passo:** 745
> **Data:** 2026-07-10
> **Foco:** Desde P731, todos os relatórios da cadeia `cetz` atribuem o diff de pixels residual (~0,14-0,15%) a "anti-aliasing", repetido sem nunca ter sido confirmado com metodologia própria — só assumido por ser a explicação mais óbvia. Este passo verifica isso a sério, com quatro testes independentes, antes de aceitar como definitivo.
> **Tipo:** Sonda directa, ampla. Correcção só se a sonda refutar a explicação de anti-aliasing.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.** Uma explicação repetida muitas vezes não se torna mais verdadeira por repetição — só por verificação directa.
> **Dependências:** P731-P744 (onde o número foi repetidamente observado e nunca verificado a fundo).

---

## Sonda

### Teste 1 — o diff diminui proporcionalmente com a resolução?

Anti-aliasing é um efeito de fronteira — o número de pixels afectados cresce com o perímetro dos traços (proporcional à resolução linear), enquanto o total de pixels cresce com a área (proporcional ao quadrado da resolução). Se for mesmo anti-aliasing, a percentagem de diff deve **diminuir** claramente ao aumentar a resolução.

```bash
cat > /tmp/p745-cetz.typ <<'EOF'
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
EOF

for dpi in 75 150 300 600; do
  ./target/release/typst /tmp/p745-cetz.typ /tmp/p745-cristalino.pdf
  lab/typst-original/target/release/typst compile /tmp/p745-cetz.typ /tmp/p745-vanilla.pdf
  mutool draw -o /tmp/p745-cristalino-$dpi.png -r $dpi /tmp/p745-cristalino.pdf
  mutool draw -o /tmp/p745-vanilla-$dpi.png -r $dpi /tmp/p745-vanilla.pdf
  echo "=== $dpi dpi ==="
  python3 /tmp/pngdiff.py /tmp/p745-vanilla-$dpi.png /tmp/p745-cristalino-$dpi.png 2>/dev/null || echo "usar o script já existente de P727/P731"
done
```

Se a percentagem cair de forma consistente e significativa com o aumento de resolução (75→150→300→600), confirma anti-aliasing. Se se mantiver estável ou aumentar, é sinal de erro real (posição, cor, ou geometria diferente).

### Teste 2 — visualizar o mapa de diferenças

```bash
python3 -c "
import struct, zlib

def read_png(path):
    with open(path, 'rb') as f:
        data = f.read()
    # usar o mesmo decoder já usado em pngdiff.py (P727)
    return data

# Gerar uma imagem que marca a vermelho só os pixels que diferem,
# preservando o resto em cinza claro — para inspecção visual directa.
"
```

Adaptar o script `pngdiff.py` já existente (usado desde P727) para, em vez de só contar, produzir um PNG de saída marcando os pixels diferentes. Visualizar esse PNG (`view` ou descrição) — confirmar se os pixels diferentes estão concentrados nas fronteiras dos traços (linha diagonal, contorno do círculo), ou espalhados/concentrados nalgum sítio específico (o que sugeriria um erro real, não ruído de rasterização).

### Teste 3 — o mesmo documento sem `cetz`, só formas nativas simples

```bash
cat > /tmp/p745-nativo.typ <<'EOF'
#line(start: (0pt, 0pt), end: (100pt, 50pt))
#circle(radius: 30pt)
EOF
./target/release/typst /tmp/p745-nativo.typ /tmp/p745-nativo-cristalino.pdf
lab/typst-original/target/release/typst compile /tmp/p745-nativo.typ /tmp/p745-nativo-vanilla.pdf
mutool draw -o /tmp/p745-nativo-cristalino.png -r 150 /tmp/p745-nativo-cristalino.pdf
mutool draw -o /tmp/p745-nativo-vanilla.png -r 150 /tmp/p745-nativo-vanilla.pdf
```

Comparar o diff percentual deste caso muito mais simples (sem WASM, sem `cetz`) com o do documento completo — se for da mesma ordem de grandeza, sugere que a causa é genérica (rasterização de qualquer traço), não específica de `cetz`/WASM.

### Teste 4 — mesmo motor de rasterização, confirmar que não há diferença de configuração

```bash
mutool draw -o /tmp/p745-v1.png -r 150 /tmp/p745-vanilla.pdf
mutool draw -o /tmp/p745-v2.png -r 150 /tmp/p745-vanilla.pdf
python3 /tmp/pngdiff.py /tmp/p745-v1.png /tmp/p745-v2.png
```

Rasterizar o **mesmo** PDF (vanilla) duas vezes e comparar — confirma se `mutool draw` é 100% determinístico (diff deve ser exactamente zero). Se não for zero, há uma fonte de ruído na própria ferramenta de comparação, não nos compiladores.

### Critério de fecho da sonda

- [ ] Teste 1: relação entre resolução e percentagem de diff confirmada.
- [ ] Teste 2: mapa visual de diferenças produzido e inspeccionado.
- [ ] Teste 3: diff de formas nativas simples comparado com o de `cetz`.
- [ ] Teste 4: determinismo do rasterizador confirmado.

---

## Decisão

Se os quatro testes confirmarem anti-aliasing (diff cai com resolução, concentrado em fronteiras, presente mesmo em formas nativas simples, rasterizador determinístico): a explicação fica finalmente confirmada, não só repetida — actualizar os relatórios/lista de controlo com esta confirmação definitiva, encerrando a questão.

Se algum teste refutar a explicação: investigar a causa real, com a mesma disciplina de sonda já usada no resto desta cadeia.

---

## Critério de fecho do passo

- [ ] Os quatro testes completos, com números e/ou imagens, não suposição.
- [ ] Conclusão registada com evidência directa.
- [ ] Se refutado: investigação da causa real, corrigida se possível.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p745.md`.
