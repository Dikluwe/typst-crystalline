---
# P763f — Correcção real: transformação de coordenadas no renderizador de paths

> **Passo:** 763f
> **Data:** 2026-07-15
> **Foco:** P763e confirmou, duas vezes de forma independente (P763c e P763e, mesma metodologia, mesmo resultado AE=10725 e mesmas coordenadas), que o cristalino não aplica a translação local de grupo que o vanilla aplica (`transform="1 0 0 1 tx ty"`), usando em vez disso só um flip-Y global de página (`1 0 0 -1 0 841.89`), e que o eixo Y dentro do canvas do `cetz` está invertido — o círculo desenhado em `(0,0)` aparece no ponto final da linha, não no inicial. P763d chegou à conclusão errada de que isto estava corrigido porque comparou PDFs directamente com `compare -metric AE` em vez de rasterizar primeiro — esse erro metodológico está registado e não deve repetir-se neste passo nem em nenhum outro que use `compare`.
> **Tipo:** Implementação directa. A causa já está identificada com coordenadas exactas; não é mais sonda de localização, é correcção.
> **Tamanho:** L — mexe no renderizador de paths/transformações de grupo, caminho partilhado.
> **ADR-0108 EM VIGOR.** **Regra obrigatória de medição deste passo**: qualquer validação com `compare -metric AE` neste passo, e em qualquer passo futuro que o reutilize, tem de rasterizar com `mutool draw -r 300` antes de comparar. Comparação directa de PDF está proibida como método de validação de paridade visual a partir deste passo — reportar sempre os dois números (directo e rasterizado) se houver dúvida, nunca só um.
> **Dependências:** P763c (causa identificada), P763e (causa confirmada duas vezes, erro de P763d isolado e explicado).

---

## Localização exacta do código

```bash
grep -rn "1 0 0 -1\|flip.*y\|page_height\|Transform::" 01_core/src/engine/layout/*.rs 03_infra/src/*.rs 2>/dev/null | grep -iv test | head -40
grep -rn "fn.*group\|fn.*place\|local_transform\|group_transform" 01_core/src/engine/layout/*.rs 2>/dev/null | head -40
```

Confirmar:
1. Onde o flip-Y de página é aplicado (deve ser só uma vez, no nível mais externo do documento — como o vanilla faz).
2. Onde a translação local de um grupo posicionado (`place`, e por extensão o mecanismo interno que `cetz` usa para posicionar cada elemento do canvas) deveria compor com esse flip, e por que está a ser omitida ou a anular-se de forma errada.
3. Se há uma segunda inversão de sinal aplicada dentro do escopo do grupo (explicaria por que o círculo aparece no ponto final da linha em vez do inicial — um sinal duplicado ou trocado nalguma componente Y da composição de transformações).

## Implementação

Corrigir a composição de transformações para que:
- O flip-Y global de página continue a aplicar-se uma vez, no nível do documento (isto não muda — é o que já funciona para conteúdo simples, confirmado pelo baseline AE=241 de `#rect()` em P763b, que não tinha este bug).
- A translação local de grupo/`place` seja aplicada correctamente dentro desse referencial, replicando `transform="1 0 0 1 tx ty"` do vanilla — sem inversão adicional de Y dentro do grupo.

Não alterar o comportamento de conteúdo fora de grupos posicionados — o objectivo é só corrigir a composição, não o flip de página em si.

---

## Validação — com a regra de medição deste passo

```bash
cat > /tmp/p763f-cetz-original.typ <<'EOF'
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
EOF

lab/typst-original/target/release/typst compile /tmp/p763f-cetz-original.typ /tmp/p763f-vanilla.pdf
./target/release/typst compile /tmp/p763f-cetz-original.typ /tmp/p763f-cristalino.pdf

# SEMPRE rasterizar antes de comparar — nunca só o método directo
mutool draw -o /tmp/p763f-vanilla.png -r 300 /tmp/p763f-vanilla.pdf
mutool draw -o /tmp/p763f-cristalino.png -r 300 /tmp/p763f-cristalino.pdf
compare -metric AE /tmp/p763f-vanilla.png /tmp/p763f-cristalino.png -highlight-color red -lowlight-color none /tmp/p763f-diffmap.png

# Registar também o valor directo, só para constar — nunca usar como critério de fecho
compare -metric AE /tmp/p763f-vanilla.pdf /tmp/p763f-cristalino.pdf /tmp/p763f-direct.png 2>&1 || true
```

Confirmação por coordenadas, não só pela métrica de pixel:

```bash
mutool trace /tmp/p763f-vanilla.pdf > /tmp/p763f-trace-vanilla.txt
mutool trace /tmp/p763f-cristalino.pdf > /tmp/p763f-trace-cristalino.txt
diff /tmp/p763f-trace-vanilla.txt /tmp/p763f-trace-cristalino.txt
```

Esperado: sem diferença nas coordenadas de `line`/`circle`; círculo a coincidir com o ponto **inicial** da linha (não o final).

### Regressão nos casos já validados

Repetir os testes de baseline de P763b/P763c que já batiam (rasterizado, não directo):

```bash
cat > /tmp/p763f-rect.typ <<'EOF'
#rect(width: 2cm, height: 2cm)
EOF
lab/typst-original/target/release/typst compile /tmp/p763f-rect.typ /tmp/p763f-rect-vanilla.pdf
./target/release/typst compile /tmp/p763f-rect.typ /tmp/p763f-rect-cristalino.pdf
mutool draw -o /tmp/p763f-rect-vanilla.png -r 300 /tmp/p763f-rect-vanilla.pdf
mutool draw -o /tmp/p763f-rect-cristalino.png -r 300 /tmp/p763f-rect-cristalino.pdf
compare -metric AE /tmp/p763f-rect-vanilla.png /tmp/p763f-rect-cristalino.png /tmp/p763f-rect-diff.png
```

Esperado: AE≈241 (mesmo baseline de sempre) — confirmar que a correcção não mexeu no caminho de conteúdo simples.

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Código exacto da composição de transformações localizado por leitura directa.
- [ ] Translação local de grupo corrigida, sem inversão adicional de Y.
- [ ] Documento `cetz` original (idêntico byte a byte a P763c/P763e) validado com `mutool draw` → `compare`, **não** comparação directa de PDF.
- [ ] Coordenadas confirmadas via `mutool trace` — círculo no ponto inicial da linha, não no final.
- [ ] Baseline de `#rect()` sem regressão (AE≈241, rasterizado).
- [ ] Comparação directa de PDF registada só como nota, nunca como critério de fecho.
- [ ] `cargo test --workspace` verde.
- [ ] `crystalline-lint .` zero violações.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p763f.md`, com o AE final (rasterizado) e o diff de coordenadas.

---

## Próximo passo

Se AE final ficar no patamar do baseline: fechar definitivamente a linha `cetz`/download de pacotes (P763–P763f).
Verificação de metodologia recomendada, fora do âmbito deste passo mas registada para não esquecer: confirmar se P765b e P766 usaram alguma comparação visual (`compare -metric AE`) na validação — pela leitura dos relatórios, ambos validaram por `repr()`/saída de texto e testes unitários, não por diff de imagem, pelo que o erro metodológico de P763d (específico a comparação de PDF) provavelmente não os afecta. Confirmar isto explicitamente antes de dar como certo.
