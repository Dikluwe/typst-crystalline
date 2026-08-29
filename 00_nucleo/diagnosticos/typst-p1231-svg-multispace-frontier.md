# P1231 — fronteira Linear/Radial multi-space SVG

**Veredito:** `REJECTED`.

As nove combinações candidatas foram medidas contra o vanilla ratificado
`a51e02804`. A tentativa de engenharia com uma malha uniforme de 64
subdivisões por intervalo preservou os controles sRGB, mas falhou 54 de 88
fixtures numéricas. Nenhuma candidata passou integralmente: cada uma teve ao
menos três falhas. Luma foi a divergência mais severa, com erro de cor máximo
`0.721348211` e erro de alpha máximo `0.6`.

Por isso a tentativa densa foi retirada. O algoritmo adaptativo P1229 e sua
whitelist exata foram preservados. Continuam `Unknown`, com fallback
`gradient-color-space` observável:

- Radial/Oklab;
- Linear e Radial/Oklch;
- Radial/LinearRgb;
- Linear e Radial/Luma;
- Linear e Radial/Hsl;
- Linear/Hsv.

Os controles continuam `Preserved`: Linear/Radial sRGB, Linear/Oklab,
Linear/LinearRgb e Radial/Hsv. CMYK continua `Unknown-ADR0097`. A disposição
das nove candidatas passou, mas o verificador final rejeitou o passo porque o
controle protegido R01 também reprovou: 19/30 fixtures raster e 14/30 fixtures
numéricas sRGB/P1229. Isso ficou como contradição de predecessor/orçamento
não resolvida, não como regressão causada pelo alargamento P1231, que foi
retirado. Não há certificado de aceitação.

A medição partiu do HEAD `697eaf31e8ce6aaa4eef7d61d7808e377005c3c5`
em working tree não commitada, inicialmente em
`2026-08-26T22:35:46-03:00`. O binário do diagnóstico denso teve SHA-256
`3ebd5363c88a9393a072c25104549b4f075349d13a846537f6f03080fbc80373` e
a evidência numérica bruta SHA-256
`395d07f27d2df92b2312d55535a4badbdd1a114bb753ab16086ff8adaf4c3a34`.

EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO.
