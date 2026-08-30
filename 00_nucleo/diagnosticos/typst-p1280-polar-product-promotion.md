# Diagnóstico P1280 — promoção produtiva dos gradientes polares

## Veredito

`POLAR-PRODUCTIVELY-PRESERVED`.

Os seis pares Linear/Radial × Oklch/Hsl/Hsv foram promovidos da rota de
fallback para o servidor SVG adaptativo já certificado pelo P1279. A matriz
produtiva fechou 144/144 fixtures
polares nativas; Luma conservou fallback em
48/48 controles.

| Par | Produto | Determinismo | Classificação final |
|---|---:|---:|---|
| linear/oklch | 24/24 | 48/48 | Productively-Preserved |
| radial/oklch | 24/24 | 48/48 | Productively-Preserved |
| linear/hsl | 24/24 | 48/48 | Productively-Preserved |
| radial/hsl | 24/24 | 48/48 | Productively-Preserved |
| linear/hsv | 24/24 | 48/48 | Productively-Preserved |
| radial/hsv | 24/24 | 48/48 | Productively-Preserved |

## Evidência e ataques

- domínio inválido rejeitado: 56/56;
- recomposições inversa/repetida: 384/384;
- ataques P1280 rejeitados: 36/36;
- envelope numérico/raster/grafo/custo P1279 herdado por manifesto imutável: `8f44e3ac8da2b9f75ee075f0656a7917f8039c39734e1cef12951ca21916c1a3`.

O RED observou o fallback anterior em Linear/Oklch; o GREEN cobriu os seis
pares em fill e stroke. O delta produtivo limita-se ao predicado de
elegibilidade; o adaptador certificado não foi alterado.

## Limites

O veredito não afirma equivalência SVG geral. Linear/Radial × Luma e × CMYK
continuam `Unknown-generalization`; Conic e Tiling não foram promovidos.

## Proveniência

- baseline: upstream/main `a51e02804` ratificado;
- HEAD medido: `ad93af9e80213fc410ffeda1f4916b031b088474` com working tree não commitado;
- medição direta: `2026-08-29T21:35:26-03:00`;
- resultado produtivo reproduzido: `1f12d336b7196c8a7d986f4ee9a14c0da7ed6af6605710c0a394bfcd0f2e54ef`.

## Atestação

`EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO`. Os papéis, entradas e artefatos foram
segregados logicamente, mas compartilharam checkout e capacidade de leitura.
