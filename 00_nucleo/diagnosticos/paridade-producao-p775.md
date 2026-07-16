# P775 — Verificação por mapas de diferença e coordenadas: AE~87k nas orientações EXIF 2-8

**Data:** 2026-07-16  
**Commit de medição:** `b7fef82476dfd0d8cba8d23ee3a273b787faf6bc`  
**Ficheiros alterados:** nenhum (sonda de verificação).  
**Mapas de diferença:** `/tmp/p775-diffmaps/diffmap-orient{1..8}.png`

---

## Resumo

P774 reportou AE≈87000 para as orientações EXIF 2-8 e atribuiu a
"artefactos de recodificação JPEG". Este passo mediu directamente:

- Recompressão JPEG isolada (qualidade 95 → 95) introduz **AE=83**.
- As bounding boxes do rectângulo de referência batem entre vanilla e
  cristalino para **todas as 8 orientações**.
- O vanilla **não recodifica** JPEGs com orientação EXIF: mantém os bytes
  originais e aplica a transformação via matriz PDF (`exif_transform`).
- O cristalino **recodifica** JPEGs após a rotação (`image` crate,
  qualidade 95).

Conclusão: a orientação EXIF está geometricamente correcta; o AE elevado é um
efeito secundário da recodificação JPEG, não um bug de orientação. Não se
abre P776.

---

## Passo 0 — Sanity-check de recompressão JPEG

Medição isolada de uma segunda passagem JPEG:

```bash
convert /tmp/p774-base.png -quality 95 /tmp/p775-recomp-a.jpg
convert /tmp/p775-recomp-a.jpg -quality 95 /tmp/p775-recomp-b.jpg
compare -metric AE /tmp/p775-recomp-a.jpg /tmp/p775-recomp-b.jpg /tmp/p775-recomp-diff.png
```

**Resultado: AE = 83.**

Outros sanity-checks:

| Pipeline | AE |
|----------|-----|
| `convert` → `djpeg` → `cjpeg -quality 95` | 922 |
| `convert -quality 92` → `djpeg` → `cjpeg -quality 95` | 1044 |

Todos os sanity-checks ficam na casa das centenas/baixo milhar, **três ordens
de grandeza abaixo** dos ~87000 reportados.

---

## Passo 1 — Mapas de diferença

Gerados com:

```bash
compare -metric AE vanilla.png cristalino.png -highlight-color red -lowlight-color none diffmap.png
```

| Orientação | Transformação | AE |
|------------|---------------|-----|
| 1 | nenhuma | 0 |
| 2 | flip horizontal | 87026 |
| 3 | rotate 180° | 87208 |
| 4 | flip vertical | 87208 |
| 5 | flip horizontal + rotate 270° | 87022 |
| 6 | rotate 90° | 87204 |
| 7 | flip horizontal + rotate 90° | 87208 |
| 8 | rotate 270° | 87022 |

Os mapas mostram a área da imagem preenchida de forma sólida e bem definida
(sem difusão uniforme). Isso indica que **todos os pixels da imagem têm cores
ligeiramente diferentes**, não que haja desalinhamento ou conteúdo
geometricamente errado.

---

## Passo 2 — Bounding box do rectângulo de referência

Script próprio (reduz imagem a 10%, procura pixels com `R>180, G<100, B<100`):

```text
orient 1:  vanilla=(15,22,15,22)  cristalino=(15,22,15,22)
orient 2:  vanilla=(48,55,15,22)  cristalino=(48,55,15,22)
orient 3:  vanilla=(48,55,27,34)  cristalino=(48,55,27,34)
orient 4:  vanilla=(15,22,27,34)  cristalino=(15,22,27,34)
orient 5:  vanilla=(15,22,15,22)  cristalino=(15,22,15,22)
orient 6:  vanilla=(27,34,15,22)  cristalino=(27,34,15,22)
orient 7:  vanilla=(27,34,48,55)  cristalino=(27,34,48,55)
orient 8:  vanilla=(15,22,48,55)  cristalino=(15,22,48,55)
```

**Resultado:** as bounding boxes batem exactamente para todas as orientações.
A geometria (posição do rectângulo vermelho) está correcta.

---

## Causa identificada

Extração dos JPEGs embebidos nos PDFs (`pdfimages -list` / `pdfimages -j`):

| | Vanilla | Cristalino |
|---|---|---|
| Dimensões do JPEG embebido | 200×100 (original) | 100×200 (rodado) |
| Tamanho | 1040 B | 1923 B |
| Qualidade reportada | 92 | 95 |
| Subsampling | 1×1,1×1,1×1 | 1×1,1×1,1×1 |

O vanilla mantém o JPEG original e aplica a orientação EXIF como transformação
na matriz `cm` do PDF (`exif_transform` em
`lab/typst-original/crates/typst-pdf/src/image.rs`). O cristalino, pelo
contrário, descodifica o JPEG, aplica a rotação aos pixels e recodifica.

A diferença de cor em quase todos os pixels (AE~87k no render final, ~40k nos
JPEGs extraídos) vem desta recodificação, agravada por tabelas de quantização
diferentes entre o `convert` original e a crate `image`.

---

## Conclusão

- **Bug de orientação EXIF:** não confirmado. A geometria bate para as 8
  orientações.
- **Explicação de P774:** parcialmente correcta no sentido de que o AE vem de
  processamento JPEG, mas o valor não é explicado por "recompressão JPEG" em
  geral (sanity-check deu 83). A causa específica é que o vanilla **preserva
  o JPEG original** para orientações EXIF, enquanto o cristalino **recodifica
  os pixels**.
- **Próximo passo:** não se abre P776. A linha P769-P775 fecha. A
  preservação de JPEGs originais (em vez de recodificação) é um achado de
  fidelidade/performance separado, fora do âmbito de orientação EXIF.
