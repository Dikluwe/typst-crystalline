---
# P777 — Correcção do color space de imagens JPEG (`/ICCBased` vs `/DeviceRGB`)

> **Passo:** 777
> **Data:** 2026-07-16
> **Foco:** P776 fechou a paridade geométrica de orientação EXIF (matrizes `cm` idênticas a 3 casas decimais), deixando um resíduo de AE 0-195 atribuído a color space: o vanilla embebe o JPEG com perfil ICC (`/ICCBased`), o cristalino usa `/DeviceRGB`. Não há mais desvio geométrico conhecido na linha de imagem (P769-P776); este passo resolve o item de cor que ficou registado, para não deixar pendência conhecida.
> **Tipo:** Sonda + Implementação directa.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR** — confirmar a origem do perfil ICC do vanilla antes de replicar (pode vir do próprio ficheiro JPEG, de um perfil sRGB embutido por omissão, ou de outra fonte).
> **ADR-0107** — diferença de color space é candidata a "diferença de implementação aceitável" só se o resultado observável (cor renderizada) for equivalente; confirmar isso, não assumir pela pequena magnitude do AE.
> **Dependências:** P776 (achado e medição do resíduo, commit `bd5ae1321c8962b7a9af733bedc1953a5725a709`).

---

## Sonda — de onde vem o `/ICCBased` do vanilla

### Confirmar se o perfil vem do ficheiro de origem ou é sintetizado

```bash
exiftool -ICC_Profile /tmp/p774-base.png 2>/dev/null || echo "sem perfil ICC embutido"
```

Se a imagem de teste não tiver perfil ICC próprio, o vanilla está a **sintetizar** um (provavelmente sRGB por omissão) em vez de usar `/DeviceRGB` directo. Confirmar por leitura de código:

```bash
grep -n "ICCBased\|icc_profile\|ColorSpace\|sRGB" lab/typst-original/crates/typst-pdf/src/image.rs lab/typst-original/crates/typst-pdf/src/*.rs 2>/dev/null | head -30
```

Confirmar:
1. O vanilla sempre usa `/ICCBased`, mesmo para imagens sem perfil próprio (sintetizando sRGB), ou só quando a imagem já tem um perfil embutido?
2. Se sintetiza, qual é o perfil exacto embutido no PDF (extrair os bytes do stream ICC do PDF do vanilla e identificar, ex: `sRGB IEC61966-2.1` é o mais comum).
3. Se há alguma condição em que o vanilla usa `/DeviceRGB` puro (ex: imagens em escala de cinza, ou CMYK) — não assumir que é sempre `/ICCBased`.

### Confirmar se a cor renderizada realmente diverge, ou só o mecanismo de PDF

```bash
mutool draw -o /tmp/p777-vanilla.png -r 300 /tmp/p774-vanilla-orient1.pdf
mutool draw -o /tmp/p777-cristalino.png -r 300 /tmp/p774-cristalino-orient1.pdf
compare -metric AE /tmp/p777-vanilla.png /tmp/p777-cristalino.png -highlight-color red -lowlight-color none /tmp/p777-diffmap.png
```

Se AE já está em 0-195 (baseline), confirmar que essa pequena diferença é mesmo de cor (inspeccionar o mapa — deve ser difuso, cobrindo a imagem toda com intensidade baixa, não concentrado). Extrair valores RGB de um pixel conhecido em cada PNG e comparar diretamente:

```bash
python3 -c "
from PIL import Image
img_v = Image.open('/tmp/p777-vanilla.png').convert('RGB')
img_c = Image.open('/tmp/p777-cristalino.png').convert('RGB')
print('vanilla:', img_v.getpixel((100, 50)))
print('cristalino:', img_c.getpixel((100, 50)))
"
```

---

## Implementação

Conforme o resultado da sonda:

- **Se o vanilla sintetiza sRGB por omissão**: embutir o mesmo perfil ICC sRGB no exportador PDF (`03_infra/src/export/stream.rs` ou onde o color space da imagem é declarado), como `/ICCBased` referenciando o stream do perfil, em vez de `/DeviceRGB` directo.
- **Se o vanilla só usa `/ICCBased` quando a imagem de origem já tem perfil embutido**: extrair e propagar o perfil ICC do ficheiro original, sem sintetizar nada quando não existir (nesse caso, `/DeviceRGB` seria o comportamento correcto para imagens sem perfil, e o resíduo de P776 pode ter outra causa a confirmar).

Não implementar a opção mais simples sem confirmar qual das duas é o comportamento real do vanilla.

---

## Validação

```bash
cargo test --workspace
```

Repetir a validação de P776 (as 8 orientações, `validate.py` ou equivalente) e confirmar que o AE desce de 0-195 para 0 em todos os casos, ou fica explicado por um resíduo ainda menor com causa identificada.

```bash
crystalline-lint .
```

Confirmar com `pdfimages`/inspecção do PDF que o color space declarado agora bate com o vanilla (`/ICCBased` com o mesmo perfil, ou `/DeviceRGB` nos dois, conforme o que a sonda confirmar ser o comportamento correcto).

---

## Critério de fecho do passo

- [ ] Origem do `/ICCBased` do vanilla confirmada por leitura de código (sintetizado vs propagado do ficheiro).
- [ ] Perfil ICC exacto identificado, se sintetizado.
- [ ] Cor renderizada comparada pixel a pixel (não só a métrica AE) antes e depois.
- [ ] Implementação replicando o comportamento confirmado, não a opção mais simples por suposição.
- [ ] AE final em 0 (ou resíduo residual explicado com nova causa, não a mesma).
- [ ] `cargo test --workspace` verde.
- [ ] `crystalline-lint .` zero violações.
- [ ] L0 de `image`/exportador actualizado com a lógica de color space, antes do código.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p777.md`.

---

## Próximo passo

Com este passo, a linha de trabalho de imagem completa (P769-P777) fecha sem pendências conhecidas. Retomar a varredura da stdlib em P772a (lote 3).
